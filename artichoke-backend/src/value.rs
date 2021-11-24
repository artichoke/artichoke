use std::borrow::Cow;
use std::error;
use std::fmt;
use std::ptr;

use artichoke_core::convert::{Convert, ConvertMut, TryConvert, TryConvertMut};
use artichoke_core::intern::Intern;
use artichoke_core::value::Value as ValueCore;

use crate::convert::BoxUnboxVmValue;
use crate::core::ClassRegistry;
use crate::error::{Error, RubyException};
use crate::exception_handler;
use crate::extn::core::exception::{ArgumentError, Fatal};
use crate::extn::core::symbol::Symbol;
use crate::gc::MrbGarbageCollection;
use crate::sys::{self, protect};
use crate::types::{self, Ruby};
use crate::Artichoke;

/// Max argument count for function calls including initialize and yield.
pub const MRB_FUNCALL_ARGC_MAX: usize = 16;

/// Boxed Ruby value in the [`Artichoke`] interpreter.
#[derive(Default, Debug, Clone, Copy)]
pub struct Value(sys::mrb_value);

impl From<Value> for sys::mrb_value {
    /// Extract the inner [`sys::mrb_value`] from this [`Value`].
    fn from(value: Value) -> Self {
        value.0
    }
}

impl From<sys::mrb_value> for Value {
    /// Construct a new [`Value`] from a [`sys::mrb_value`].
    fn from(value: sys::mrb_value) -> Self {
        Self(value)
    }
}

impl From<Option<sys::mrb_value>> for Value {
    fn from(value: Option<sys::mrb_value>) -> Self {
        if let Some(value) = value {
            Self::from(value)
        } else {
            Self::nil()
        }
    }
}

impl From<Option<Value>> for Value {
    fn from(value: Option<Value>) -> Self {
        value.unwrap_or_else(Value::nil)
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
    /// Create a new, empty Ruby value.
    ///
    /// Alias for `Value::default`.
    #[inline]
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Create a `nil` Ruby Value.
    #[inline]
    #[must_use]
    pub fn nil() -> Self {
        Self::default()
    }

    /// The [`sys::mrb_value`] that this [`Value`] wraps.
    // TODO(GH-251): make `Value::inner` pub(crate).
    #[inline]
    #[must_use]
    pub const fn inner(&self) -> sys::mrb_value {
        self.0
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
        let value = self.inner();
        let is_dead = unsafe { interp.with_ffi_boundary(|mrb| sys::mrb_sys_value_is_dead(mrb, value)) };
        is_dead.unwrap_or_default()
    }

    pub fn is_range(&self, interp: &mut Artichoke, len: i64) -> Result<Option<protect::Range>, Error> {
        let mut arena = interp.create_arena_savepoint()?;
        let result = unsafe {
            arena
                .interp()
                .with_ffi_boundary(|mrb| protect::is_range(mrb, self.inner(), len))?
        };
        match result {
            Ok(range) => Ok(range),
            Err(exception) => {
                let exception = Self::from(exception);
                Err(exception_handler::last_error(&mut arena, exception)?)
            }
        }
    }
}

impl ValueCore for Value {
    type Artichoke = Artichoke;
    type Arg = Self;
    type Value = Self;
    type Block = Self;
    type Error = Error;

    fn funcall(
        &self,
        interp: &mut Self::Artichoke,
        func: &str,
        args: &[Self::Arg],
        block: Option<Self::Block>,
    ) -> Result<Self::Value, Self::Error> {
        if self.is_dead(interp) {
            return Err(Fatal::from("Value receiver for function call is dead. This indicates a bug in the mruby garbage collector. Please leave a comment at https://github.com/artichoke/artichoke/issues/1336.").into());
        }
        if let Ok(arg_count_error) = ArgCountError::try_from(args) {
            warn!("{}", arg_count_error);
            return Err(arg_count_error.into());
        }
        let args = args.iter().map(Self::inner).collect::<Vec<_>>();
        trace!(
            "Calling {}#{} with {} args{}",
            self.ruby_type(),
            func,
            args.len(),
            if block.is_some() { " and block" } else { "" }
        );
        let func = interp.intern_string(func.to_string())?;
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
                let value = Self::from(value);
                if value.is_unreachable() {
                    // Unreachable values are internal to the mruby interpreter
                    // and interacting with them via the C API is unspecified
                    // and may result in a segfault.
                    //
                    // See: https://github.com/mruby/mruby/issues/4460
                    Err(Fatal::from("Unreachable Ruby value").into())
                } else {
                    Ok(interp.protect(value))
                }
            }
            Err(exception) => {
                let exception = interp.protect(Self::from(exception));
                Err(exception_handler::last_error(interp, exception)?)
            }
        }
    }

    fn freeze(&mut self, interp: &mut Self::Artichoke) -> Result<(), Self::Error> {
        self.funcall(interp, "freeze", &[], None)?;
        Ok(())
    }

    fn is_frozen(&self, interp: &mut Self::Artichoke) -> bool {
        let value = self.inner();
        let is_frozen = unsafe { interp.with_ffi_boundary(|mrb| sys::mrb_sys_obj_frozen(mrb, value)) };
        is_frozen.unwrap_or_default()
    }

    fn inspect(&self, interp: &mut Self::Artichoke) -> Vec<u8> {
        if let Ok(display) = self.funcall(interp, "inspect", &[], None) {
            display.try_convert_into_mut(interp).unwrap_or_default()
        } else {
            Vec::new()
        }
    }

    fn is_nil(&self) -> bool {
        matches!(self.ruby_type(), Ruby::Nil)
    }

    fn respond_to(&self, interp: &mut Self::Artichoke, method: &str) -> Result<bool, Self::Error> {
        let method = interp.intern_string(String::from(method))?;
        let method = Symbol::new(method);
        let method = Symbol::alloc_value(method, interp)?;
        let respond_to = self.funcall(interp, "respond_to?", &[method], None)?;
        interp.try_convert(respond_to)
    }

    fn to_s(&self, interp: &mut Self::Artichoke) -> Vec<u8> {
        if let Ok(display) = self.funcall(interp, "to_s", &[], None) {
            display.try_convert_into_mut(interp).unwrap_or_default()
        } else {
            Vec::new()
        }
    }

    fn ruby_type(&self) -> Ruby {
        types::ruby_from_mrb_value(self.inner())
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

/// Argument count exceeds maximum allowed by the VM.
#[derive(Default, Debug, Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct ArgCountError {
    /// Number of arguments given.
    pub given: usize,
    /// Maximum number of arguments supported.
    pub max: usize,
}

impl TryFrom<Vec<Value>> for ArgCountError {
    type Error = ();

    fn try_from(args: Vec<Value>) -> Result<Self, Self::Error> {
        if args.len() > MRB_FUNCALL_ARGC_MAX {
            Ok(Self {
                given: args.len(),
                max: MRB_FUNCALL_ARGC_MAX,
            })
        } else {
            Err(())
        }
    }
}

impl TryFrom<Vec<sys::mrb_value>> for ArgCountError {
    type Error = ();

    fn try_from(args: Vec<sys::mrb_value>) -> Result<Self, Self::Error> {
        if args.len() > MRB_FUNCALL_ARGC_MAX {
            Ok(Self {
                given: args.len(),
                max: MRB_FUNCALL_ARGC_MAX,
            })
        } else {
            Err(())
        }
    }
}

impl TryFrom<&[Value]> for ArgCountError {
    type Error = ();

    fn try_from(args: &[Value]) -> Result<Self, Self::Error> {
        if args.len() > MRB_FUNCALL_ARGC_MAX {
            Ok(Self {
                given: args.len(),
                max: MRB_FUNCALL_ARGC_MAX,
            })
        } else {
            Err(())
        }
    }
}

impl TryFrom<&[sys::mrb_value]> for ArgCountError {
    type Error = ();

    fn try_from(args: &[sys::mrb_value]) -> Result<Self, Self::Error> {
        if args.len() > MRB_FUNCALL_ARGC_MAX {
            Ok(Self {
                given: args.len(),
                max: MRB_FUNCALL_ARGC_MAX,
            })
        } else {
            Err(())
        }
    }
}

impl ArgCountError {
    /// Constructs a new, empty `ArgCountError`.
    #[must_use]
    pub fn new() -> Self {
        Self {
            given: 0,
            max: MRB_FUNCALL_ARGC_MAX,
        }
    }
}

impl fmt::Display for ArgCountError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("Too many arguments for function call: ")?;
        write!(
            f,
            "gave {} arguments, but Artichoke only supports a maximum of {} arguments",
            self.given, self.max
        )
    }
}

impl error::Error for ArgCountError {}

impl RubyException for ArgCountError {
    fn message(&self) -> Cow<'_, [u8]> {
        Cow::Borrowed(b"Too many arguments")
    }

    fn name(&self) -> Cow<'_, str> {
        "ArgumentError".into()
    }

    fn vm_backtrace(&self, interp: &mut Artichoke) -> Option<Vec<Vec<u8>>> {
        let _ = interp;
        None
    }

    fn as_mrb_value(&self, interp: &mut Artichoke) -> Option<sys::mrb_value> {
        let message = interp.try_convert_mut(self.to_string()).ok()?;
        let value = interp.new_instance::<ArgumentError>(&[message]).ok().flatten()?;
        Some(value.inner())
    }
}

impl From<ArgCountError> for Error {
    fn from(exception: ArgCountError) -> Self {
        Self::from(Box::<dyn RubyException>::from(exception))
    }
}

impl From<Box<ArgCountError>> for Error {
    fn from(exception: Box<ArgCountError>) -> Self {
        Self::from(Box::<dyn RubyException>::from(exception))
    }
}

impl From<ArgCountError> for Box<dyn RubyException> {
    fn from(exception: ArgCountError) -> Box<dyn RubyException> {
        Box::new(exception)
    }
}

impl From<Box<ArgCountError>> for Box<dyn RubyException> {
    fn from(exception: Box<ArgCountError>) -> Box<dyn RubyException> {
        exception
    }
}

#[cfg(test)]
mod tests {
    use bstr::ByteSlice;

    use crate::gc::MrbGarbageCollection;
    use crate::test::prelude::*;

    #[test]
    fn to_s_true() {
        let mut interp = interpreter().unwrap();

        let value = interp.convert(true);
        let string = value.to_s(&mut interp);
        assert_eq!(string.as_bstr(), b"true".as_bstr());
    }

    #[test]
    fn inspect_true() {
        let mut interp = interpreter().unwrap();

        let value = interp.convert(true);
        let debug = value.inspect(&mut interp);
        assert_eq!(debug.as_bstr(), b"true".as_bstr());
    }

    #[test]
    fn to_s_false() {
        let mut interp = interpreter().unwrap();

        let value = interp.convert(false);
        let string = value.to_s(&mut interp);
        assert_eq!(string.as_bstr(), b"false".as_bstr());
    }

    #[test]
    fn inspect_false() {
        let mut interp = interpreter().unwrap();

        let value = interp.convert(false);
        let debug = value.inspect(&mut interp);
        assert_eq!(debug.as_bstr(), b"false".as_bstr());
    }

    #[test]
    fn to_s_nil() {
        let mut interp = interpreter().unwrap();

        let value = Value::nil();
        let string = value.to_s(&mut interp);
        assert_eq!(string.as_bstr(), b"".as_bstr());
    }

    #[test]
    fn inspect_nil() {
        let mut interp = interpreter().unwrap();

        let value = Value::nil();
        let debug = value.inspect(&mut interp);
        assert_eq!(debug.as_bstr(), b"nil".as_bstr());
    }

    #[test]
    fn to_s_fixnum() {
        let mut interp = interpreter().unwrap();

        let value = Convert::<_, Value>::convert(&*interp, 255);
        let string = value.to_s(&mut interp);
        assert_eq!(string.as_bstr(), b"255".as_bstr());
    }

    #[test]
    fn inspect_fixnum() {
        let mut interp = interpreter().unwrap();

        let value = Convert::<_, Value>::convert(&*interp, 255);
        let debug = value.inspect(&mut interp);
        assert_eq!(debug.as_bstr(), b"255".as_bstr());
    }

    #[test]
    fn to_s_string() {
        let mut interp = interpreter().unwrap();

        let value = interp.try_convert_mut("interstate").unwrap();
        let string = value.to_s(&mut interp);
        assert_eq!(string.as_bstr(), b"interstate".as_bstr());
    }

    #[test]
    fn inspect_string() {
        let mut interp = interpreter().unwrap();

        let value = interp.try_convert_mut("interstate").unwrap();
        let debug = value.inspect(&mut interp);
        assert_eq!(debug.as_bstr(), br#""interstate""#.as_bstr());
    }

    #[test]
    fn to_s_empty_string() {
        let mut interp = interpreter().unwrap();

        let value = interp.try_convert_mut("").unwrap();
        let string = value.to_s(&mut interp);
        assert_eq!(string.as_bstr(), b"".as_bstr());
    }

    #[test]
    fn inspect_empty_string() {
        let mut interp = interpreter().unwrap();

        let value = interp.try_convert_mut("").unwrap();
        let debug = value.inspect(&mut interp);
        assert_eq!(debug.as_bstr(), br#""""#.as_bstr());
    }

    #[test]
    fn is_dead() {
        let mut interp = interpreter().unwrap();
        let mut arena = interp.create_arena_savepoint().unwrap();
        let live = arena.eval(b"'dead'").unwrap();
        assert!(!live.is_dead(&mut arena));

        let dead = live;
        let live = arena.eval(b"'live'").unwrap();
        arena.restore();
        interp.full_gc().unwrap();

        // unreachable objects are dead after a full garbage collection
        assert!(dead.is_dead(&mut interp));
        // the result of the most recent eval is always live even after a full
        // garbage collection
        assert!(!live.is_dead(&mut interp));
    }

    #[test]
    fn funcall_is_dead() {
        let mut interp = interpreter().unwrap();
        let mut arena = interp.create_arena_savepoint().unwrap();

        let dead = arena.eval(b"'dead'").unwrap();
        arena.eval(b"'live'").unwrap();
        arena.restore();
        interp.full_gc().unwrap();

        assert!(dead.is_dead(&mut interp));

        let error = dead.funcall(&mut interp, "nil?", &[], None).unwrap_err();
        assert_eq!(error.name().as_ref(), "fatal");
    }

    #[test]
    fn immediate_is_dead() {
        let mut interp = interpreter().unwrap();
        let mut arena = interp.create_arena_savepoint().unwrap();
        let live = arena.eval(b"27").unwrap();
        assert!(!live.is_dead(&mut arena));

        let immediate = live;
        let live = arena.eval(b"64").unwrap();
        arena.restore();
        interp.full_gc().unwrap();

        // immediate objects are never dead
        assert!(!immediate.is_dead(&mut interp));
        // the result of the most recent eval is always live even after a full
        // garbage collection
        assert!(!live.is_dead(&mut interp));

        // `Fixnum`s are immediate even if they are created directly without an
        // interpreter.
        let fixnum = Convert::<_, Value>::convert(&*interp, 99);
        assert!(!fixnum.is_dead(&mut interp));
    }

    #[test]
    fn funcall_nil_nil() {
        let mut interp = interpreter().unwrap();

        let nil = Value::nil();
        let result = nil
            .funcall(&mut interp, "nil?", &[], None)
            .and_then(|value| value.try_convert_into::<bool>(&interp));
        let nil_is_nil = unwrap_or_panic_with_backtrace(&mut interp, "Value::funcall", result);
        assert!(nil_is_nil);
    }

    #[test]
    fn funcall_string_nil() {
        let mut interp = interpreter().unwrap();

        let s = interp.try_convert_mut("foo").unwrap();
        let result = s
            .funcall(&mut interp, "nil?", &[], None)
            .and_then(|value| value.try_convert_into::<bool>(&interp));
        let string_is_nil = unwrap_or_panic_with_backtrace(&mut interp, "Value::funcall", result);
        assert!(!string_is_nil);
    }

    #[test]
    #[cfg(feature = "core-regexp")]
    fn funcall_string_split_regexp() {
        let mut interp = interpreter().unwrap();

        let s = interp.try_convert_mut("foo").unwrap();
        let delim = interp.try_convert_mut("").unwrap();
        let result = s
            .funcall(&mut interp, "split", &[delim], None)
            .and_then(|value| value.try_convert_into_mut::<Vec<&str>>(&mut interp));
        let split = unwrap_or_panic_with_backtrace(&mut interp, "Value::funcall", result);
        assert_eq!(split, vec!["f", "o", "o"]);
    }

    #[test]
    fn funcall_different_types() {
        let mut interp = interpreter().unwrap();
        let nil = Value::nil();
        let s = interp.try_convert_mut("foo").unwrap();
        let result = nil
            .funcall(&mut interp, "==", &[s], None)
            .and_then(|value| value.try_convert_into::<bool>(&interp));
        let eql = unwrap_or_panic_with_backtrace(&mut interp, "Value::funcall", result);
        assert!(!eql);
    }

    #[test]
    fn funcall_type_error() {
        let mut interp = interpreter().unwrap();
        let nil = Value::nil();
        let s = interp.try_convert_mut("foo").unwrap();
        let err = s
            .funcall(&mut interp, "+", &[nil], None)
            .and_then(|value| value.try_convert_into_mut::<String>(&mut interp))
            .unwrap_err();
        assert_eq!("TypeError", err.name().as_ref());
        assert_eq!(
            b"nil cannot be converted to String".as_bstr(),
            err.message().as_ref().as_bstr()
        );
    }

    #[test]
    fn funcall_method_not_exists() {
        let mut interp = interpreter().unwrap();
        let nil = Value::nil();
        let s = interp.try_convert_mut("foo").unwrap();
        let err = nil.funcall(&mut interp, "garbage_method_name", &[s], None).unwrap_err();
        assert_eq!("NoMethodError", err.name().as_ref());
        assert_eq!(
            b"undefined method 'garbage_method_name'".as_bstr(),
            err.message().as_ref().as_bstr()
        );
    }
}
