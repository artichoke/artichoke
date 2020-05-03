use std::error;
use std::fmt;
use std::ptr;

use crate::class_registry::ClassRegistry;
use crate::core::{Convert, ConvertMut, Intern, TryConvert, Value as ValueCore};
use crate::exception::{Exception, RubyException};
use crate::exception_handler;
use crate::extn::core::exception::{ArgumentError, Fatal, TypeError};
use crate::ffi::InterpreterExtractError;
use crate::gc::MrbGarbageCollection;
use crate::sys::{self, protect};
use crate::types::{self, Int, Ruby};
use crate::Artichoke;

/// Max argument count for function calls including initialize and yield.
pub const MRB_FUNCALL_ARGC_MAX: usize = 16;

/// Boxed Ruby value in the [`Artichoke`] interpreter.
#[derive(Clone, Copy)]
pub struct Value(sys::mrb_value);

impl fmt::Debug for Value {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Value")
            .field("type", &self.ruby_type())
            .finish()
    }
}

impl PartialEq for Value {
    fn eq(&self, other: &Self) -> bool {
        let this = unsafe { sys::mrb_sys_basic_ptr(self.inner()) };
        let other = unsafe { sys::mrb_sys_basic_ptr(other.inner()) };
        ptr::eq(this, other)
    }
}

impl Value {
    /// Construct a new [`Value`] from an interpreter and [`sys::mrb_value`].
    #[must_use]
    pub fn new(interp: &Artichoke, value: sys::mrb_value) -> Self {
        let _ = interp;
        Self(value)
    }

    /// The [`sys::mrb_value`] that this [`Value`] wraps.
    // TODO(GH-251): make `Value::inner` pub(crate).
    #[inline]
    #[must_use]
    pub fn inner(&self) -> sys::mrb_value {
        self.0
    }

    /// Return this values [Rust-mapped type tag](Ruby).
    #[inline]
    #[must_use]
    pub fn ruby_type(&self) -> Ruby {
        types::ruby_from_mrb_value(self.inner())
    }

    #[must_use]
    pub fn pretty_name<'a>(&self, interp: &mut Artichoke) -> &'a str {
        let _ = interp;
        match self.try_into(interp) {
            Ok(Some(true)) => "true",
            Ok(Some(false)) => "false",
            Ok(None) => "nil",
            Err(_) => {
                if let Ruby::Data | Ruby::Object = self.ruby_type() {
                    self.funcall::<Self>(interp, "class", &[], None)
                        .and_then(|class| class.funcall::<Value>(interp, "name", &[], None))
                        .and_then(|class| class.try_into_mut(interp))
                        .unwrap_or_default()
                } else {
                    self.ruby_type().class_name()
                }
            }
        }
    }

    /// Whether a value is an interpreter-only variant not exposed to Ruby.
    ///
    /// Some type tags like [`MRB_TT_UNDEF`](sys::mrb_vtype::MRB_TT_UNDEF) are
    /// internal to the mruby VM and manipulating them with the [`sys`] API is
    /// unspecified and may result in a segfault.
    ///
    /// After extracting a [`sys::mrb_value`] from the interpreter, check to see
    /// if the value is [unreachable](Ruby::Unreachable) a [`Fatal`] exception.
    ///
    /// See: [mruby#4460](https://github.com/mruby/mruby/issues/4460).
    #[must_use]
    #[inline]
    pub fn is_unreachable(&self) -> bool {
        matches!(self.ruby_type(), Ruby::Unreachable)
    }

    /// Return whether this object is unreachable by any GC roots.
    #[must_use]
    pub fn is_dead(&self, interp: &mut Artichoke) -> bool {
        unsafe {
            let mrb = interp.mrb.as_mut();
            sys::mrb_sys_value_is_dead(mrb, self.inner())
        }
    }

    pub fn is_range(
        &self,
        interp: &mut Artichoke,
        len: Int,
    ) -> Result<Option<protect::Range>, Exception> {
        let _arena = interp.create_arena_savepoint();
        let result =
            unsafe { interp.with_ffi_boundary(|mrb| protect::is_range(mrb, self.inner(), len))? };
        match result {
            Ok(range) => Ok(range),
            Err(exception) => {
                let exception = Value::new(interp, exception);
                Err(exception_handler::last_error(interp, exception)?)
            }
        }
    }

    pub fn implicitly_convert_to_int(&self, interp: &mut Artichoke) -> Result<Int, TypeError> {
        let int = if let Ok(int) = self.try_into::<Option<Int>>(interp) {
            if let Some(int) = int {
                int
            } else {
                return Err(TypeError::new(
                    interp,
                    "no implicit conversion from nil to integer",
                ));
            }
        } else if let Ok(true) = self.respond_to(interp, "to_int") {
            if let Ok(maybe) = self.funcall::<Self>(interp, "to_int", &[], None) {
                if let Ok(int) = maybe.try_into::<Int>(interp) {
                    int
                } else {
                    let mut message = String::from("can't convert ");
                    message.push_str(self.pretty_name(interp));
                    message.push_str(" to Integer (");
                    message.push_str(self.pretty_name(interp));
                    message.push_str("#to_int gives ");
                    message.push_str(maybe.pretty_name(interp));
                    message.push(')');
                    return Err(TypeError::new(interp, message));
                }
            } else {
                let mut message = String::from("no implicit conversion of ");
                message.push_str(self.pretty_name(interp));
                message.push_str(" into Integer");
                return Err(TypeError::new(interp, message));
            }
        } else {
            let mut message = String::from("no implicit conversion of ");
            message.push_str(self.pretty_name(interp));
            message.push_str(" into Integer");
            return Err(TypeError::new(interp, message));
        };
        Ok(int)
    }

    pub fn implicitly_convert_to_string(&self, interp: &mut Artichoke) -> Result<&[u8], TypeError> {
        let string = if let Ok(string) = self.try_into_mut::<&[u8]>(interp) {
            string
        } else if let Ok(true) = self.respond_to(interp, "to_str") {
            if let Ok(maybe) = self.funcall::<Self>(interp, "to_str", &[], None) {
                if let Ok(string) = maybe.try_into_mut::<&[u8]>(interp) {
                    string
                } else {
                    let mut message = String::from("can't convert ");
                    message.push_str(self.pretty_name(interp));
                    message.push_str(" to String (");
                    message.push_str(self.pretty_name(interp));
                    message.push_str("#to_str gives ");
                    message.push_str(maybe.pretty_name(interp));
                    message.push(')');
                    return Err(TypeError::new(interp, message));
                }
            } else {
                let mut message = String::from("no implicit conversion of ");
                message.push_str(self.pretty_name(interp));
                message.push_str(" into String");
                return Err(TypeError::new(interp, message));
            }
        } else {
            let mut message = String::from("no implicit conversion of ");
            message.push_str(self.pretty_name(interp));
            message.push_str(" into String");
            return Err(TypeError::new(interp, message));
        };
        Ok(string)
    }

    #[inline]
    pub fn implicitly_convert_to_nilable_string(
        &self,
        interp: &mut Artichoke,
    ) -> Result<Option<&[u8]>, TypeError> {
        if self.is_nil() {
            Ok(None)
        } else {
            self.implicitly_convert_to_string(interp).map(Some)
        }
    }
}

impl ValueCore for Value {
    type Artichoke = Artichoke;
    type Arg = Self;
    type Block = Self;
    type Error = Exception;

    fn funcall<T>(
        &self,
        interp: &mut Self::Artichoke,
        func: &str,
        args: &[Self::Arg],
        block: Option<Self::Block>,
    ) -> Result<T, Self::Error>
    where
        Self::Artichoke: TryConvert<Self, T, Error = Self::Error>,
    {
        let _arena = interp.create_arena_savepoint();
        if args.len() > MRB_FUNCALL_ARGC_MAX {
            let err = ArgCountError::new(args);
            warn!("{}", err);
            return Err(err.into());
        }
        let args = args.iter().map(Self::inner).collect::<Vec<_>>();
        trace!(
            "Calling {}#{} with {} args{}",
            self.ruby_type(),
            func,
            args.len(),
            if block.is_some() { " and block" } else { "" }
        );
        let func = interp.intern_symbol(func.as_bytes().to_vec());
        let result = unsafe {
            interp.with_ffi_boundary(|mrb| {
                protect::funcall(
                    mrb,
                    self.inner(),
                    func,
                    args.as_slice(),
                    block.as_ref().map(Self::inner),
                )
            })?
        };
        match result {
            Ok(value) => {
                let value = Self::new(interp, value);
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
                    value.try_into::<T>(interp)
                }
            }
            Err(exception) => {
                let exception = Self::new(interp, exception);
                Err(exception_handler::last_error(interp, exception)?)
            }
        }
    }

    fn freeze(&mut self, interp: &mut Self::Artichoke) -> Result<(), Self::Error> {
        let _ = self.funcall::<Self>(interp, "freeze", &[], None)?;
        Ok(())
    }

    fn is_frozen(&self, interp: &mut Self::Artichoke) -> bool {
        unsafe {
            let mrb = interp.mrb.as_mut();
            sys::mrb_sys_obj_frozen(mrb, self.inner())
        }
    }

    fn inspect(&self, interp: &mut Self::Artichoke) -> Vec<u8> {
        if let Ok(display) = self.funcall::<Value>(interp, "inspect", &[], None) {
            display.try_into_mut(interp).unwrap_or_default()
        } else {
            Vec::new()
        }
    }

    fn is_nil(&self) -> bool {
        matches!(self.ruby_type(), Ruby::Nil)
    }

    fn respond_to(&self, interp: &mut Self::Artichoke, method: &str) -> Result<bool, Self::Error> {
        let method = interp.convert_mut(method);
        self.funcall::<bool>(interp, "respond_to?", &[method], None)
    }

    fn to_s(&self, interp: &mut Self::Artichoke) -> Vec<u8> {
        if let Ok(display) = self.funcall::<Value>(interp, "to_s", &[], None) {
            display.try_into_mut(interp).unwrap_or_default()
        } else {
            Vec::new()
        }
    }
}

impl Convert<Value, Value> for Artichoke {
    fn convert(&self, value: Value) -> Value {
        value
    }
}

impl ConvertMut<Value, Value> for Artichoke {
    fn convert_mut(&mut self, value: Value) -> Value {
        value
    }
}

#[derive(Clone, Copy)]
pub struct Block(sys::mrb_value);

impl fmt::Debug for Block {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "proc")
    }
}

impl Block {
    #[must_use]
    pub fn new(block: sys::mrb_value) -> Option<Self> {
        if let Ruby::Nil = types::ruby_from_mrb_value(block) {
            None
        } else {
            Some(Self(block))
        }
    }

    #[inline]
    #[must_use]
    pub fn inner(&self) -> sys::mrb_value {
        self.0
    }

    pub fn yield_arg<T>(&self, interp: &mut Artichoke, arg: &Value) -> Result<T, Exception>
    where
        Artichoke: TryConvert<Value, T, Error = Exception>,
    {
        let _arena = interp.create_arena_savepoint();

        let result = unsafe {
            interp.with_ffi_boundary(|mrb| protect::block_yield(mrb, self.inner(), arg.inner()))?
        };
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
                    value.try_into::<T>(interp)
                }
            }
            Err(exception) => {
                let exception = Value::new(interp, exception);
                Err(exception_handler::last_error(interp, exception)?)
            }
        }
    }
}

/// Argument count exceeds maximum allowed by the VM.
#[derive(Debug, Clone, Copy)]
pub struct ArgCountError {
    /// Number of arguments given.
    pub given: usize,
    /// Maximum number of arguments supported.
    pub max: usize,
}

impl ArgCountError {
    pub fn new<T>(args: T) -> Self
    where
        T: AsRef<[Value]>,
    {
        Self {
            given: args.as_ref().len(),
            max: MRB_FUNCALL_ARGC_MAX,
        }
    }
}

impl fmt::Display for ArgCountError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Too many arguments for function call: ")?;
        write!(
            f,
            "gave {} arguments, but Artichoke only supports a maximum of {} arguments",
            self.given, self.max
        )
    }
}

impl error::Error for ArgCountError {}

impl RubyException for ArgCountError {
    fn message(&self) -> &[u8] {
        &b"Too many arguments"[..]
    }

    fn name(&self) -> String {
        String::from("ArgumentError")
    }

    fn vm_backtrace(&self, interp: &mut Artichoke) -> Option<Vec<Vec<u8>>> {
        let _ = interp;
        None
    }

    fn as_mrb_value(&self, interp: &mut Artichoke) -> Option<sys::mrb_value> {
        let message = interp.convert_mut(self.to_string());
        let value = interp
            .new_instance::<ArgumentError>(&[message])
            .ok()
            .flatten()?;
        Some(value.inner())
    }
}

impl From<ArgCountError> for Exception {
    fn from(exception: ArgCountError) -> Self {
        Self::from(Box::<dyn RubyException>::from(exception))
    }
}

impl From<Box<ArgCountError>> for Exception {
    fn from(exception: Box<ArgCountError>) -> Self {
        Self::from(Box::<dyn RubyException>::from(exception))
    }
}

#[allow(clippy::use_self)]
impl From<ArgCountError> for Box<dyn RubyException> {
    fn from(exception: ArgCountError) -> Box<dyn RubyException> {
        Box::new(exception)
    }
}

#[allow(clippy::use_self)]
impl From<Box<ArgCountError>> for Box<dyn RubyException> {
    fn from(exception: Box<ArgCountError>) -> Box<dyn RubyException> {
        exception
    }
}

#[cfg(test)]
mod tests {
    use crate::gc::MrbGarbageCollection;
    use crate::test::prelude::*;

    #[test]
    fn to_s_true() {
        let mut interp = crate::interpreter().unwrap();

        let value = interp.convert(true);
        let string = value.to_s(&mut interp);
        assert_eq!(string, b"true");
    }

    #[test]
    fn inspect_true() {
        let mut interp = crate::interpreter().unwrap();

        let value = interp.convert(true);
        let debug = value.inspect(&mut interp);
        assert_eq!(debug, b"true");
    }

    #[test]
    fn to_s_false() {
        let mut interp = crate::interpreter().unwrap();

        let value = interp.convert(false);
        let string = value.to_s(&mut interp);
        assert_eq!(string, b"false");
    }

    #[test]
    fn inspect_false() {
        let mut interp = crate::interpreter().unwrap();

        let value = interp.convert(false);
        let debug = value.inspect(&mut interp);
        assert_eq!(debug, b"false");
    }

    #[test]
    fn to_s_nil() {
        let mut interp = crate::interpreter().unwrap();

        let value = interp.convert(None::<Value>);
        let string = value.to_s(&mut interp);
        assert_eq!(string, b"");
    }

    #[test]
    fn inspect_nil() {
        let mut interp = crate::interpreter().unwrap();

        let value = interp.convert(None::<Value>);
        let debug = value.inspect(&mut interp);
        assert_eq!(debug, b"nil");
    }

    #[test]
    fn to_s_fixnum() {
        let mut interp = crate::interpreter().unwrap();

        let value = Convert::<_, Value>::convert(&interp, 255);
        let string = value.to_s(&mut interp);
        assert_eq!(string, b"255");
    }

    #[test]
    fn inspect_fixnum() {
        let mut interp = crate::interpreter().unwrap();

        let value = Convert::<_, Value>::convert(&interp, 255);
        let debug = value.inspect(&mut interp);
        assert_eq!(debug, b"255");
    }

    #[test]
    fn to_s_string() {
        let mut interp = crate::interpreter().unwrap();

        let value = interp.convert_mut("interstate");
        let string = value.to_s(&mut interp);
        assert_eq!(string, b"interstate");
    }

    #[test]
    fn inspect_string() {
        let mut interp = crate::interpreter().unwrap();

        let value = interp.convert_mut("interstate");
        let debug = value.inspect(&mut interp);
        assert_eq!(debug, br#""interstate""#);
    }

    #[test]
    fn to_s_empty_string() {
        let mut interp = crate::interpreter().unwrap();

        let value = interp.convert_mut("");
        let string = value.to_s(&mut interp);
        assert_eq!(string, b"");
    }

    #[test]
    fn inspect_empty_string() {
        let mut interp = crate::interpreter().unwrap();

        let value = interp.convert_mut("");
        let debug = value.inspect(&mut interp);
        assert_eq!(debug, br#""""#);
    }

    #[test]
    fn is_dead() {
        let mut interp = crate::interpreter().unwrap();
        let arena = interp.create_arena_savepoint();
        let live = interp.eval(b"'dead'").unwrap();
        assert!(!live.is_dead(&mut interp));
        let dead = live;
        let live = interp.eval(b"'live'").unwrap();
        arena.restore();
        interp.full_gc();
        // unreachable objects are dead after a full garbage collection
        assert!(dead.is_dead(&mut interp));
        // the result of the most recent eval is always live even after a full
        // garbage collection
        assert!(!live.is_dead(&mut interp));
    }

    #[test]
    fn immediate_is_dead() {
        let mut interp = crate::interpreter().unwrap();
        let arena = interp.create_arena_savepoint();
        let live = interp.eval(b"27").unwrap();
        assert!(!live.is_dead(&mut interp));
        let immediate = live;
        let live = interp.eval(b"64").unwrap();
        arena.restore();
        interp.full_gc();
        // immediate objects are never dead
        assert!(!immediate.is_dead(&mut interp));
        // the result of the most recent eval is always live even after a full
        // garbage collection
        assert!(!live.is_dead(&mut interp));
        // Fixnums are immediate even if they are created directly without an
        // interpreter.
        let fixnum = Convert::<_, Value>::convert(&interp, 99);
        assert!(!fixnum.is_dead(&mut interp));
    }

    #[test]
    fn funcall() {
        let mut interp = crate::interpreter().unwrap();
        let nil = interp.convert(None::<Value>);
        let nil_is_nil = nil.funcall::<bool>(&mut interp, "nil?", &[], None).unwrap();
        assert!(nil_is_nil);
        let s = interp.convert_mut("foo");
        let string_is_nil = s.funcall::<bool>(&mut interp, "nil?", &[], None).unwrap();
        assert!(!string_is_nil);
        let delim = interp.convert_mut("");
        let split = s
            .funcall::<Value>(&mut interp, "split", &[delim], None)
            .unwrap();
        let split: Vec<&str> = interp.try_convert_mut(split).unwrap();
        assert_eq!(split, vec!["f", "o", "o"])
    }

    #[test]
    fn funcall_different_types() {
        let mut interp = crate::interpreter().unwrap();
        let nil = interp.convert(None::<Value>);
        let s = interp.convert_mut("foo");
        let eql = nil.funcall::<bool>(&mut interp, "==", &[s], None).unwrap();
        assert!(!eql);
    }

    #[test]
    fn funcall_type_error() {
        let mut interp = crate::interpreter().unwrap();
        let nil = interp.convert(None::<Value>);
        let s = interp.convert_mut("foo");
        let err = s
            .funcall::<String>(&mut interp, "+", &[nil], None)
            .unwrap_err();
        assert_eq!("TypeError", err.name().as_str());
        assert_eq!(&b"nil cannot be converted to String"[..], err.message());
    }

    #[test]
    fn funcall_method_not_exists() {
        let mut interp = crate::interpreter().unwrap();
        let nil = interp.convert(None::<Value>);
        let s = interp.convert_mut("foo");
        let err = nil
            .funcall::<bool>(&mut interp, "garbage_method_name", &[s], None)
            .unwrap_err();
        assert_eq!("NoMethodError", err.name().as_str());
        assert_eq!(
            &b"undefined method 'garbage_method_name'"[..],
            err.message()
        );
    }
}
