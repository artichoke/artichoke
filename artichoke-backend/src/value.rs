use std::convert::TryFrom;
use std::ffi::c_void;
use std::fmt;
use std::mem;

use crate::convert::{Convert, TryConvert};
use crate::exception::{ExceptionHandler, LastError};
use crate::extn::core::exception::{RubyException, TypeError};
use crate::gc::MrbGarbageCollection;
use crate::sys;
use crate::types::{self, Int, Ruby};
use crate::{Artichoke, ArtichokeError};

pub(crate) use artichoke_core::value::Value as ValueLike;

/// Max argument count for function calls including initialize and yield.
pub const MRB_FUNCALL_ARGC_MAX: usize = 16;

// `Protect` must be `Copy` because the call to a function in the
// `mrb_funcall...` family can unwind with `longjmp` which does not allow Rust
// to run destructors.
#[derive(Clone, Copy)]
struct Protect<'a> {
    slf: sys::mrb_value,
    func_sym: u32,
    args: &'a [sys::mrb_value],
    block: Option<sys::mrb_value>,
}

impl<'a> Protect<'a> {
    fn new(slf: sys::mrb_value, func_sym: u32, args: &'a [sys::mrb_value]) -> Self {
        Self {
            slf,
            func_sym,
            args,
            block: None,
        }
    }

    fn with_block(self, block: sys::mrb_value) -> Self {
        Self {
            slf: self.slf,
            func_sym: self.func_sym,
            args: self.args,
            block: Some(block),
        }
    }

    unsafe extern "C" fn run(mrb: *mut sys::mrb_state, data: sys::mrb_value) -> sys::mrb_value {
        let ptr = sys::mrb_sys_cptr_ptr(data);
        // `protect` must be `Copy` because the call to a function in the
        // `mrb_funcall...` family can unwind with `longjmp` which does not
        // allow Rust to run destructors.
        let protect = Box::from_raw(ptr as *mut Self);

        // Pull all of the args out of the `Box` so we can free the
        // heap-allocated `Box`.
        let slf = protect.slf;
        let func_sym = protect.func_sym;
        let args = protect.args;
        // This will always unwrap because we've already checked that we
        // have fewer than `MRB_FUNCALL_ARGC_MAX` args, which is less than
        // i64 max value.
        let argslen = Int::try_from(args.len()).unwrap_or_default();
        let block = protect.block;

        // Drop the `Box` to ensure it is freed.
        drop(protect);

        if let Some(block) = block {
            sys::mrb_funcall_with_block(mrb, slf, func_sym, argslen, args.as_ptr(), block)
        } else {
            sys::mrb_funcall_argv(mrb, slf, func_sym, argslen, args.as_ptr())
        }
    }
}

/// Wrapper around a [`sys::mrb_value`].
pub struct Value {
    interp: Artichoke,
    value: sys::mrb_value,
}

impl Value {
    /// Construct a new [`Value`] from an interpreter and [`sys::mrb_value`].
    pub fn new(interp: &Artichoke, value: sys::mrb_value) -> Self {
        Self {
            interp: interp.clone(),
            value,
        }
    }

    /// The [`sys::mrb_value`] that this [`Value`] wraps.
    // TODO: make Value::inner pub(crate), GH-251.
    #[inline]
    pub fn inner(&self) -> sys::mrb_value {
        self.value
    }

    /// Return this values [Rust-mapped type tag](Ruby).
    pub fn ruby_type(&self) -> Ruby {
        types::ruby_from_mrb_value(self.value)
    }

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
    pub fn to_s_debug(&self) -> String {
        let inspected_str = String::from_utf8_lossy(&self.inspect()).to_string();
        format!("{}<{}>", self.ruby_type().class_name(), inspected_str)
    }

    pub fn implicitly_convert_to_int(&self) -> Result<Int, Box<dyn RubyException>> {
        let int = if let Ok(int) = self.clone().try_into::<Int>() {
            int
        } else {
            let pretty_name = self.pretty_name();
            if let Ok(maybe_int) = self.funcall::<Self>("to_int", &[], None) {
                let gives_pretty_name = maybe_int.pretty_name();
                if let Ok(int) = maybe_int.try_into::<Int>() {
                    int
                } else {
                    return Err(Box::new(TypeError::new(
                        &self.interp,
                        format!(
                            "can't convert {} to Integer ({}#to_int gives {})",
                            pretty_name, pretty_name, gives_pretty_name
                        ),
                    )));
                }
            } else {
                return Err(Box::new(TypeError::new(
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

    fn funcall<T>(
        &self,
        func: &str,
        args: &[Self::Arg],
        block: Option<Self::Block>,
    ) -> Result<T, ArtichokeError>
    where
        Self::Artichoke: TryConvert<Self, T>,
    {
        // Ensure the borrow is out of scope by the time we eval code since
        // Rust-backed files and types may need to mutably borrow the `Artichoke` to
        // get access to the underlying `ArtichokeState`.
        let (mrb, _ctx) = {
            let borrow = self.interp.0.borrow();
            (borrow.mrb, borrow.ctx)
        };

        let _arena = self.interp.create_arena_savepoint();

        let args = args.as_ref().iter().map(Self::inner).collect::<Vec<_>>();
        if args.len() > MRB_FUNCALL_ARGC_MAX {
            warn!(
                "Too many args supplied to funcall: given {}, max {}.",
                args.len(),
                MRB_FUNCALL_ARGC_MAX
            );
            return Err(ArtichokeError::TooManyArgs {
                given: args.len(),
                max: MRB_FUNCALL_ARGC_MAX,
            });
        }
        trace!(
            "Calling {}#{} with {} args{}",
            types::ruby_from_mrb_value(self.inner()),
            func,
            args.len(),
            if block.is_some() { " and block" } else { "" }
        );
        let func = self
            .interp
            .0
            .borrow_mut()
            .sym_intern(func.as_bytes().to_vec());
        let mut protect = Protect::new(self.inner(), func, args.as_ref());
        if let Some(block) = block {
            protect = protect.with_block(block.inner());
        }
        let value = unsafe {
            let data =
                sys::mrb_sys_cptr_value(mrb, Box::into_raw(Box::new(protect)) as *mut c_void);
            let mut state = mem::MaybeUninit::<sys::mrb_bool>::uninit();

            let value = sys::mrb_protect(mrb, Some(Protect::run), data, state.as_mut_ptr());
            if state.assume_init() != 0 {
                (*mrb).exc = sys::mrb_sys_obj_ptr(value);
            }
            value
        };
        let value = Self::new(&self.interp, value);

        match self.interp.last_error() {
            LastError::Some(exception) => {
                warn!("runtime error with exception backtrace: {}", exception);
                Err(ArtichokeError::Exec(exception.to_string()))
            }
            LastError::UnableToExtract(err) => {
                error!("failed to extract exception after runtime error: {}", err);
                Err(err)
            }
            LastError::None if value.is_unreachable() => {
                // Unreachable values are internal to the mruby interpreter and
                // interacting with them via the C API is unspecified and may
                // result in a segfault.
                //
                // See: https://github.com/mruby/mruby/issues/4460
                Err(ArtichokeError::UnreachableValue)
            }
            LastError::None => self.interp.try_convert(value),
        }
    }

    fn unchecked_funcall(
        &self,
        func: &str,
        args: &[Self::Arg],
        block: Option<Self::Block>,
    ) -> Result<Self, ArtichokeError> {
        // Ensure the borrow is out of scope by the time we eval code since
        // Rust-backed files and types may need to mutably borrow the `Artichoke` to
        // get access to the underlying `ArtichokeState`.
        let (mrb, _ctx) = {
            let borrow = self.interp.0.borrow();
            (borrow.mrb, borrow.ctx)
        };

        let arena = self.interp.create_arena_savepoint();

        let args = args.as_ref().iter().map(Self::inner).collect::<Vec<_>>();
        if args.len() > MRB_FUNCALL_ARGC_MAX {
            warn!(
                "Too many args supplied to funcall: given {}, max {}.",
                args.len(),
                MRB_FUNCALL_ARGC_MAX
            );
            return Err(ArtichokeError::TooManyArgs {
                given: args.len(),
                max: MRB_FUNCALL_ARGC_MAX,
            });
        }
        trace!(
            "Calling {}#{} with {} args{}",
            types::ruby_from_mrb_value(self.inner()),
            func,
            args.len(),
            if block.is_some() { " and block" } else { "" }
        );
        let func = self
            .interp
            .0
            .borrow_mut()
            .sym_intern(func.as_bytes().to_vec());
        let mut protect = Protect::new(self.inner(), func, args.as_ref());
        if let Some(block) = block {
            protect = protect.with_block(block.inner());
        }
        let value = unsafe {
            let data =
                sys::mrb_sys_cptr_value(mrb, Box::into_raw(Box::new(protect)) as *mut c_void);
            let mut state = mem::MaybeUninit::<sys::mrb_bool>::uninit();

            let value = sys::mrb_protect(mrb, Some(Protect::run), data, state.as_mut_ptr());
            if state.assume_init() != 0 {
                // drop all bindings to heap-allocated objects because we are
                // about to unwind with longjmp.
                drop(arena);
                drop(args);
                (*mrb).exc = sys::mrb_sys_obj_ptr(value);
                sys::mrb_sys_raise_current_exception(mrb);
                unreachable!("mrb_raise will unwind the stack with longjmp");
            }
            value
        };
        Ok(Self::new(&self.interp, value))
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

    fn freeze(&mut self) -> Result<(), ArtichokeError> {
        self.funcall::<Self>("freeze", &[], None)?;
        Ok(())
    }

    fn is_frozen(&self) -> bool {
        let mrb = self.interp.0.borrow().mrb;
        let inner = self.inner();
        unsafe { sys::mrb_sys_obj_frozen(mrb, inner) }
    }

    fn inspect(&self) -> Vec<u8> {
        self.funcall::<Vec<u8>>("inspect", &[], None)
            .unwrap_or_else(|_| Vec::new())
    }

    fn is_nil(&self) -> bool {
        unsafe { sys::mrb_sys_value_is_nil(self.inner()) }
    }

    fn respond_to(&self, method: &str) -> Result<bool, ArtichokeError> {
        let method = self.interp.convert(method);
        self.funcall::<bool>("respond_to?", &[method], None)
    }

    fn to_s(&self) -> Vec<u8> {
        self.funcall::<Vec<u8>>("to_s", &[], None)
            .unwrap_or_else(|_| Vec::new())
    }
}

impl Convert<Value, Value> for Artichoke {
    fn convert(&self, value: Value) -> Value {
        value
    }
}

impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let string_repr = String::from_utf8_lossy(&self.to_s()).to_string();
        write!(f, "{}", string_repr)
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
        std::ptr::eq(unsafe { sys::mrb_sys_basic_ptr(self.inner()) }, unsafe {
            sys::mrb_sys_basic_ptr(other.inner())
        })
    }
}

#[derive(Clone, Copy)]
pub struct Block {
    value: sys::mrb_value,
}

impl Block {
    /// Construct a new [`Value`] from an interpreter and [`sys::mrb_value`].
    pub fn new(block: sys::mrb_value) -> Option<Self> {
        if unsafe { sys::mrb_sys_value_is_nil(block) } {
            None
        } else {
            Some(Self { value: block })
        }
    }

    pub fn yield_arg(&self, interp: &Artichoke, arg: &Value) -> Result<Value, ArtichokeError> {
        // Ensure the borrow is out of scope by the time we eval code since
        // Rust-backed files and types may need to mutably borrow the `Artichoke` to
        // get access to the underlying `ArtichokeState`.
        let mrb = interp.0.borrow().mrb;

        let _arena = interp.create_arena_savepoint();

        let value = unsafe { sys::mrb_yield(mrb, self.value, arg.inner()) };
        let value = Value::new(interp, value);

        match interp.last_error() {
            LastError::Some(exception) => {
                warn!("runtime error with exception backtrace: {}", exception);
                Err(ArtichokeError::Exec(exception.to_string()))
            }
            LastError::UnableToExtract(err) => {
                error!("failed to extract exception after runtime error: {}", err);
                Err(err)
            }
            LastError::None if value.is_unreachable() => {
                // Unreachable values are internal to the mruby interpreter and
                // interacting with them via the C API is unspecified and may
                // result in a segfault.
                //
                // See: https://github.com/mruby/mruby/issues/4460
                Err(ArtichokeError::UnreachableValue)
            }
            LastError::None => Ok(value),
        }
    }
}

#[cfg(test)]
mod tests {
    use artichoke_core::eval::Eval;

    use crate::convert::Convert;
    use crate::gc::MrbGarbageCollection;
    use crate::value::{Value, ValueLike};
    use crate::ArtichokeError;

    #[test]
    fn to_s_true() {
        let interp = crate::interpreter().expect("init");

        let value = interp.convert(true);
        let string = value.to_s();
        assert_eq!(string, "true");
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
        assert_eq!(debug, "true");
    }

    #[test]
    fn to_s_false() {
        let interp = crate::interpreter().expect("init");

        let value = interp.convert(false);
        let string = value.to_s();
        assert_eq!(string, "false");
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
        assert_eq!(debug, "false");
    }

    #[test]
    fn to_s_nil() {
        let interp = crate::interpreter().expect("init");

        let value = interp.convert(None::<Value>);
        let string = value.to_s();
        assert_eq!(string, "");
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
        assert_eq!(debug, "nil");
    }

    #[test]
    fn to_s_fixnum() {
        let interp = crate::interpreter().expect("init");

        let value: Value = interp.convert(255);
        let string = value.to_s();
        assert_eq!(string, "255");
    }

    #[test]
    fn debug_fixnum() {
        let interp = crate::interpreter().expect("init");

        let value: Value = interp.convert(255);
        let debug = value.to_s_debug();
        assert_eq!(debug, "Fixnum<255>");
    }

    #[test]
    fn inspect_fixnum() {
        let interp = crate::interpreter().expect("init");

        let value: Value = interp.convert(255);
        let debug = value.inspect();
        assert_eq!(debug, "255");
    }

    #[test]
    fn to_s_string() {
        let interp = crate::interpreter().expect("init");

        let value = interp.convert("interstate");
        let string = value.to_s();
        assert_eq!(string, "interstate");
    }

    #[test]
    fn debug_string() {
        let interp = crate::interpreter().expect("init");

        let value = interp.convert("interstate");
        let debug = value.to_s_debug();
        assert_eq!(debug, r#"String<"interstate">"#);
    }

    #[test]
    fn inspect_string() {
        let interp = crate::interpreter().expect("init");

        let value = interp.convert("interstate");
        let debug = value.inspect();
        assert_eq!(debug, r#""interstate""#);
    }

    #[test]
    fn to_s_empty_string() {
        let interp = crate::interpreter().expect("init");

        let value = interp.convert("");
        let string = value.to_s();
        assert_eq!(string, "");
    }

    #[test]
    fn debug_empty_string() {
        let interp = crate::interpreter().expect("init");

        let value = interp.convert("");
        let debug = value.to_s_debug();
        assert_eq!(debug, r#"String<"">"#);
    }

    #[test]
    fn inspect_empty_string() {
        let interp = crate::interpreter().expect("init");

        let value = interp.convert("");
        let debug = value.inspect();
        assert_eq!(debug, r#""""#);
    }

    #[test]
    fn is_dead() {
        let interp = crate::interpreter().expect("init");
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
        let interp = crate::interpreter().expect("init");
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
        let fixnum: Value = interp.convert(99);
        assert!(!fixnum.is_dead());
    }

    #[test]
    fn funcall() {
        let interp = crate::interpreter().expect("init");
        let nil = interp.convert(None::<Value>);
        assert!(nil.funcall::<bool>("nil?", &[], None).expect("nil?"));
        let s = interp.convert("foo");
        assert!(!s.funcall::<bool>("nil?", &[], None).expect("nil?"));
        let delim = interp.convert("");
        let split = s
            .funcall::<Vec<&str>>("split", &[delim], None)
            .expect("split");
        assert_eq!(split, vec!["f", "o", "o"])
    }

    #[test]
    fn funcall_different_types() {
        let interp = crate::interpreter().expect("init");
        let nil = interp.convert(None::<Value>);
        let s = interp.convert("foo");
        let eql = nil.funcall::<bool>("==", &[s], None);
        assert_eq!(eql, Ok(false));
    }

    #[test]
    fn funcall_type_error() {
        let interp = crate::interpreter().expect("init");
        let nil = interp.convert(None::<Value>);
        let s = interp.convert("foo");
        let result = s.funcall::<String>("+", &[nil], None);
        assert_eq!(
            result,
            Err(ArtichokeError::Exec(
                "TypeError: nil cannot be converted to String".to_owned()
            ))
        );
    }

    #[test]
    fn funcall_method_not_exists() {
        let interp = crate::interpreter().expect("init");
        let nil = interp.convert(None::<Value>);
        let s = interp.convert("foo");
        let result = nil.funcall::<bool>("garbage_method_name", &[s], None);
        assert_eq!(
            result,
            Err(ArtichokeError::Exec(
                "NoMethodError: undefined method 'garbage_method_name'".to_owned()
            ))
        );
    }
}
