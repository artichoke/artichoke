use std::fmt;
use std::ptr;

use crate::convert::{Convert, ConvertMut, TryConvert};
use crate::exception::Exception;
use crate::exception_handler;
use crate::extn::core::exception::{Fatal, TypeError};
use crate::gc::MrbGarbageCollection;
use crate::sys::{self, protect};
use crate::types::{self, Int, Ruby};
use crate::{Artichoke, ArtichokeError, Intern, ValueLike};

/// Max argument count for function calls including initialize and yield.
pub const MRB_FUNCALL_ARGC_MAX: usize = 16;

/// Wrapper around a [`sys::mrb_value`].
pub struct Value {
    interp: Artichoke,
    value: sys::mrb_value,
}

impl Value {
    /// Construct a new [`Value`] from an interpreter and [`sys::mrb_value`].
    #[must_use]
    pub fn new(interp: &Artichoke, value: sys::mrb_value) -> Self {
        Self {
            interp: interp.clone(),
            value,
        }
    }

    /// The [`sys::mrb_value`] that this [`Value`] wraps.
    // TODO: make Value::inner pub(crate), GH-251.
    #[inline]
    #[must_use]
    pub fn inner(&self) -> sys::mrb_value {
        self.value
    }

    /// Return this values [Rust-mapped type tag](Ruby).
    #[must_use]
    pub fn ruby_type(&self) -> Ruby {
        types::ruby_from_mrb_value(self.value)
    }

    #[must_use]
    pub fn pretty_name<'a>(&self) -> &'a str {
        if let Ok(true) = Self::new(&self.interp, self.value).try_into::<bool>() {
            "true"
        } else if let Ok(false) = Self::new(&self.interp, self.value).try_into::<bool>() {
            "false"
        } else if let Ok(None) = Self::new(&self.interp, self.value).try_into::<Option<Self>>() {
            "nil"
        } else if let Ruby::Data | Ruby::Object = self.ruby_type() {
            self.funcall::<Self>("class", &[], None)
                .and_then(|class| class.funcall::<&'a str>("name", &[], None))
                .unwrap_or_default()
        } else {
            self.ruby_type().class_name()
        }
    }

    /// Some type tags like [`MRB_TT_UNDEF`](sys::mrb_vtype::MRB_TT_UNDEF) are
    /// internal to the mruby VM and manipulating them with the [`sys`] API is
    /// unspecified and may result in a segfault.
    ///
    /// After extracting a [`sys::mrb_value`] from the interpreter, check to see
    /// if the value is [unreachable](Ruby::Unreachable) and propagate an
    /// [`ArtichokeError::UnreachableValue`](crate::ArtichokeError::UnreachableValue) error.
    ///
    /// See: <https://github.com/mruby/mruby/issues/4460>
    #[must_use]
    pub fn is_unreachable(&self) -> bool {
        self.ruby_type() == Ruby::Unreachable
    }

    /// Prevent this value from being garbage collected.
    ///
    /// Calls [`sys::mrb_gc_protect`] on this value which adds it to the GC
    /// arena. This object will remain in the arena until
    /// [`ArenaIndex::restore`](crate::gc::ArenaIndex::restore) restores the
    /// arena to an index before this call to protect.
    pub fn protect(&self) {
        let mrb = self.interp.0.borrow().mrb;
        unsafe { sys::mrb_gc_protect(mrb, self.value) }
    }

    /// Return whether this object is unreachable by any GC roots.
    #[must_use]
    pub fn is_dead(&self) -> bool {
        let mrb = self.interp.0.borrow().mrb;
        unsafe { sys::mrb_sys_value_is_dead(mrb, self.value) }
    }

    /// Generate a debug representation of self.
    ///
    /// Format:
    ///
    /// ```ruby
    /// "#{self.class.name}<#{self.inspect}>"
    /// ```
    ///
    /// This function can never fail.
    #[must_use]
    pub fn to_s_debug(&self) -> String {
        let inspect = self.inspect();
        format!(
            "{}<{}>",
            self.ruby_type().class_name(),
            String::from_utf8_lossy(&inspect)
        )
    }

    pub fn implicitly_convert_to_int(&self) -> Result<Int, Exception> {
        let int = if let Ok(int) = self.clone().try_into::<Int>() {
            int
        } else {
            let pretty_name = self.pretty_name();
            if let Ok(maybe_int) = self.funcall::<Self>("to_int", &[], None) {
                let gives_pretty_name = maybe_int.pretty_name();
                if let Ok(int) = maybe_int.try_into::<Int>() {
                    int
                } else {
                    return Err(Exception::from(TypeError::new(
                        &self.interp,
                        format!(
                            "can't convert {} to Integer ({}#to_int gives {})",
                            pretty_name, pretty_name, gives_pretty_name
                        ),
                    )));
                }
            } else {
                return Err(Exception::from(TypeError::new(
                    &self.interp,
                    format!("no implicit conversion of {} into Integer", pretty_name),
                )));
            }
        };
        Ok(int)
    }
}

impl ValueLike for Value {
    type Artichoke = Artichoke;
    type Arg = Self;
    type Block = Self;
    type Error = Exception;

    fn funcall<T>(
        &self,
        func: &str,
        args: &[Self::Arg],
        block: Option<Self::Block>,
    ) -> Result<T, Self::Error>
    where
        Self::Artichoke: TryConvert<Self, T>,
    {
        // Ensure the borrow is out of scope by the time we eval code since
        // Rust-backed files and types may need to mutably borrow the `Artichoke` to
        // get access to the underlying `ArtichokeState`.
        let mrb = self.interp.0.borrow().mrb;

        let _arena = self.interp.create_arena_savepoint();

        let args = args.iter().map(Self::inner).collect::<Vec<_>>();
        if args.len() > MRB_FUNCALL_ARGC_MAX {
            warn!(
                "Too many args supplied to funcall: given {}, max {}.",
                args.len(),
                MRB_FUNCALL_ARGC_MAX
            );
            let err = ArtichokeError::TooManyArgs {
                given: args.len(),
                max: MRB_FUNCALL_ARGC_MAX,
            };
            return Err(Exception::from(Fatal::new(&self.interp, err.to_string())));
        }
        trace!(
            "Calling {}#{} with {} args{}",
            types::ruby_from_mrb_value(self.inner()),
            func,
            args.len(),
            if block.is_some() { " and block" } else { "" }
        );
        let func = {
            // This is a hack until Value properly supports interpreter
            // mutability.
            let mut interp = self.interp.clone();
            interp.intern_symbol(func.as_bytes().to_vec())
        };
        let result = unsafe {
            protect::funcall(
                mrb,
                self.inner(),
                func,
                args.as_slice(),
                block.as_ref().map(Self::inner),
            )
        };
        match result {
            Ok(value) => {
                let value = Self::new(&self.interp, value);
                if value.is_unreachable() {
                    // Unreachable values are internal to the mruby interpreter
                    // and interacting with them via the C API is unspecified
                    // and may result in a segfault.
                    //
                    // See: https://github.com/mruby/mruby/issues/4460
                    Err(Exception::from(Fatal::new(
                        &self.interp,
                        "Unreachable Ruby value",
                    )))
                } else {
                    let value = value.try_into::<T>().map_err(|err| {
                        TypeError::new(&self.interp, format!("Type conversion failed: {}", err))
                    })?;
                    Ok(value)
                }
            }
            Err(exception) => {
                let exception = Self::new(&self.interp, exception);
                Err(exception_handler::last_error(&self.interp, exception)?)
            }
        }
    }

    fn try_into<T>(self) -> Result<T, ArtichokeError>
    where
        Self::Artichoke: TryConvert<Self, T>,
    {
        // We must clone interp out of self because try_convert consumes self.
        let interp = self.interp.clone();
        interp.try_convert(self)
    }

    fn itself<T>(&self) -> Result<T, ArtichokeError>
    where
        Self::Artichoke: TryConvert<Self, T>,
    {
        self.clone().try_into::<T>()
    }

    fn freeze(&mut self) -> Result<(), Self::Error> {
        let _ = self.funcall::<Self>("freeze", &[], None)?;
        Ok(())
    }

    fn is_frozen(&self) -> bool {
        let mrb = self.interp.0.borrow().mrb;
        let inner = self.inner();
        unsafe { sys::mrb_sys_obj_frozen(mrb, inner) }
    }

    fn inspect(&self) -> Vec<u8> {
        self.funcall::<Vec<u8>>("inspect", &[], None)
            .unwrap_or_default()
    }

    fn is_nil(&self) -> bool {
        unsafe { sys::mrb_sys_value_is_nil(self.inner()) }
    }

    fn respond_to(&self, method: &str) -> Result<bool, Self::Error> {
        // This is a hack until Value properly supports interpreter mutability.
        let mut interp = self.interp.clone();
        let method = interp.convert_mut(method);
        self.funcall::<bool>("respond_to?", &[method], None)
    }

    fn to_s(&self) -> Vec<u8> {
        self.funcall::<Vec<u8>>("to_s", &[], None)
            .unwrap_or_default()
    }
}

impl Convert<Value, Value> for Artichoke {
    fn convert(&self, value: Value) -> Value {
        value
    }
}

impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let string_repr = self.to_s();
        write!(f, "{}", String::from_utf8_lossy(string_repr.as_slice()))
    }
}

impl fmt::Debug for Value {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.to_s_debug())
    }
}

impl Clone for Value {
    fn clone(&self) -> Self {
        Self {
            interp: self.interp.clone(),
            value: self.value,
        }
    }
}

impl PartialEq for Value {
    fn eq(&self, other: &Self) -> bool {
        let this = unsafe { sys::mrb_sys_basic_ptr(self.inner()) };
        let other = unsafe { sys::mrb_sys_basic_ptr(other.inner()) };
        ptr::eq(this, other)
    }
}

#[derive(Clone, Copy)]
pub struct Block {
    value: sys::mrb_value,
}

impl Block {
    /// Construct a new [`Value`] from an interpreter and [`sys::mrb_value`].
    #[must_use]
    pub fn new(block: sys::mrb_value) -> Option<Self> {
        if unsafe { sys::mrb_sys_value_is_nil(block) } {
            None
        } else {
            Some(Self { value: block })
        }
    }

    pub fn yield_arg<T>(&self, interp: &Artichoke, arg: &Value) -> Result<T, Exception>
    where
        Artichoke: TryConvert<Value, T>,
    {
        // Ensure the borrow is out of scope by the time we eval code since
        // Rust-backed files and types may need to mutably borrow the `Artichoke` to
        // get access to the underlying `ArtichokeState`.
        let mrb = interp.0.borrow().mrb;

        let _arena = interp.create_arena_savepoint();

        let result = unsafe { protect::block_yield(mrb, self.value, arg.inner()) };
        match result {
            Ok(value) => {
                let value = Value::new(interp, value);
                if value.is_unreachable() {
                    // Unreachable values are internal to the mruby interpreter
                    // and interacting with them via the C API is unspecified
                    // and may result in a segfault.
                    //
                    // See: https://github.com/mruby/mruby/issues/4460
                    Err(Exception::from(Fatal::new(
                        interp,
                        "Unreachable Ruby value",
                    )))
                } else {
                    let value = value.try_into::<T>().map_err(|err| {
                        TypeError::new(interp, format!("Type conversion failed: {}", err))
                    })?;
                    Ok(value)
                }
            }
            Err(exception) => {
                let exception = Value::new(interp, exception);
                Err(exception_handler::last_error(interp, exception)?)
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::gc::MrbGarbageCollection;
    use crate::test::prelude::*;

    #[test]
    fn to_s_true() {
        let interp = crate::interpreter().expect("init");

        let value = interp.convert(true);
        let string = value.to_s();
        assert_eq!(string, b"true");
    }

    #[test]
    fn debug_true() {
        let interp = crate::interpreter().expect("init");

        let value = interp.convert(true);
        let debug = value.to_s_debug();
        assert_eq!(debug, "Boolean<true>");
    }

    #[test]
    fn inspect_true() {
        let interp = crate::interpreter().expect("init");

        let value = interp.convert(true);
        let debug = value.inspect();
        assert_eq!(debug, b"true");
    }

    #[test]
    fn to_s_false() {
        let interp = crate::interpreter().expect("init");

        let value = interp.convert(false);
        let string = value.to_s();
        assert_eq!(string, b"false");
    }

    #[test]
    fn debug_false() {
        let interp = crate::interpreter().expect("init");

        let value = interp.convert(false);
        let debug = value.to_s_debug();
        assert_eq!(debug, "Boolean<false>");
    }

    #[test]
    fn inspect_false() {
        let interp = crate::interpreter().expect("init");

        let value = interp.convert(false);
        let debug = value.inspect();
        assert_eq!(debug, b"false");
    }

    #[test]
    fn to_s_nil() {
        let interp = crate::interpreter().expect("init");

        let value = interp.convert(None::<Value>);
        let string = value.to_s();
        assert_eq!(string, b"");
    }

    #[test]
    fn debug_nil() {
        let interp = crate::interpreter().expect("init");

        let value = interp.convert(None::<Value>);
        let debug = value.to_s_debug();
        assert_eq!(debug, "NilClass<nil>");
    }

    #[test]
    fn inspect_nil() {
        let interp = crate::interpreter().expect("init");

        let value = interp.convert(None::<Value>);
        let debug = value.inspect();
        assert_eq!(debug, b"nil");
    }

    #[test]
    fn to_s_fixnum() {
        let interp = crate::interpreter().expect("init");

        let value = Convert::<_, Value>::convert(&interp, 255);
        let string = value.to_s();
        assert_eq!(string, b"255");
    }

    #[test]
    fn debug_fixnum() {
        let interp = crate::interpreter().expect("init");

        let value = Convert::<_, Value>::convert(&interp, 255);
        let debug = value.to_s_debug();
        assert_eq!(debug, "Fixnum<255>");
    }

    #[test]
    fn inspect_fixnum() {
        let interp = crate::interpreter().expect("init");

        let value = Convert::<_, Value>::convert(&interp, 255);
        let debug = value.inspect();
        assert_eq!(debug, b"255");
    }

    #[test]
    fn to_s_string() {
        let mut interp = crate::interpreter().expect("init");

        let value = interp.convert_mut("interstate");
        let string = value.to_s();
        assert_eq!(string, b"interstate");
    }

    #[test]
    fn debug_string() {
        let mut interp = crate::interpreter().expect("init");

        let value = interp.convert_mut("interstate");
        let debug = value.to_s_debug();
        assert_eq!(debug, r#"String<"interstate">"#);
    }

    #[test]
    fn inspect_string() {
        let mut interp = crate::interpreter().expect("init");

        let value = interp.convert_mut("interstate");
        let debug = value.inspect();
        assert_eq!(debug, br#""interstate""#);
    }

    #[test]
    fn to_s_empty_string() {
        let mut interp = crate::interpreter().expect("init");

        let value = interp.convert_mut("");
        let string = value.to_s();
        assert_eq!(string, b"");
    }

    #[test]
    fn debug_empty_string() {
        let mut interp = crate::interpreter().expect("init");

        let value = interp.convert_mut("");
        let debug = value.to_s_debug();
        assert_eq!(debug, r#"String<"">"#);
    }

    #[test]
    fn inspect_empty_string() {
        let mut interp = crate::interpreter().expect("init");

        let value = interp.convert_mut("");
        let debug = value.inspect();
        assert_eq!(debug, br#""""#);
    }

    #[test]
    fn is_dead() {
        let mut interp = crate::interpreter().expect("init");
        let arena = interp.create_arena_savepoint();
        let live = interp.eval(b"'dead'").expect("value");
        assert!(!live.is_dead());
        let dead = live;
        let live = interp.eval(b"'live'").expect("value");
        arena.restore();
        interp.full_gc();
        // unreachable objects are dead after a full garbage collection
        assert!(dead.is_dead());
        // the result of the most recent eval is always live even after a full
        // garbage collection
        assert!(!live.is_dead());
    }

    #[test]
    fn immediate_is_dead() {
        let mut interp = crate::interpreter().expect("init");
        let arena = interp.create_arena_savepoint();
        let live = interp.eval(b"27").expect("value");
        assert!(!live.is_dead());
        let immediate = live;
        let live = interp.eval(b"64").expect("value");
        arena.restore();
        interp.full_gc();
        // immediate objects are never dead
        assert!(!immediate.is_dead());
        // the result of the most recent eval is always live even after a full
        // garbage collection
        assert!(!live.is_dead());
        // Fixnums are immediate even if they are created directly without an
        // interpreter.
        let fixnum = Convert::<_, Value>::convert(&interp, 99);
        assert!(!fixnum.is_dead());
    }

    #[test]
    fn funcall() {
        let mut interp = crate::interpreter().expect("init");
        let nil = interp.convert(None::<Value>);
        assert!(nil.funcall::<bool>("nil?", &[], None).expect("nil?"));
        let s = interp.convert_mut("foo");
        assert!(!s.funcall::<bool>("nil?", &[], None).expect("nil?"));
        let delim = interp.convert_mut("");
        let split = s
            .funcall::<Vec<&str>>("split", &[delim], None)
            .expect("split");
        assert_eq!(split, vec!["f", "o", "o"])
    }

    #[test]
    fn funcall_different_types() {
        let mut interp = crate::interpreter().expect("init");
        let nil = interp.convert(None::<Value>);
        let s = interp.convert_mut("foo");
        let eql = nil.funcall::<bool>("==", &[s], None).unwrap();
        assert!(!eql);
    }

    #[test]
    fn funcall_type_error() {
        let mut interp = crate::interpreter().expect("init");
        let nil = interp.convert(None::<Value>);
        let s = interp.convert_mut("foo");
        let err = s.funcall::<String>("+", &[nil], None).unwrap_err();
        assert_eq!("TypeError", err.name().as_str());
        assert_eq!(&b"nil cannot be converted to String"[..], err.message());
    }

    #[test]
    fn funcall_method_not_exists() {
        let mut interp = crate::interpreter().expect("init");
        let nil = interp.convert(None::<Value>);
        let s = interp.convert_mut("foo");
        let err = nil
            .funcall::<bool>("garbage_method_name", &[s], None)
            .unwrap_err();
        assert_eq!("NoMethodError", err.name().as_str());
        assert_eq!(
            &b"undefined method 'garbage_method_name'"[..],
            err.message()
        );
    }
}
