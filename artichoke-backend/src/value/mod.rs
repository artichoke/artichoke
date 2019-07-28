use log::{error, trace, warn};
use std::convert::TryFrom;
use std::ffi::c_void;
use std::fmt;
use std::mem;
use std::rc::Rc;

use crate::convert::{Convert, TryConvert};
use crate::exception::{LastError, MrbExceptionHandler};
use crate::gc::MrbGarbageCollection;
use crate::sys;
use crate::ArtichokeError;
use crate::Mrb;

pub mod types;

/// Max argument count for function calls including initialize.
///
/// Defined in `vm.c`.
pub const MRB_FUNCALL_ARGC_MAX: usize = 16;

struct ProtectArgs {
    slf: sys::mrb_value,
    func_sym: u32,
    args: Vec<sys::mrb_value>,
}

struct ProtectArgsWithBlock {
    slf: sys::mrb_value,
    func_sym: u32,
    args: Vec<sys::mrb_value>,
    block: sys::mrb_value,
}

impl ProtectArgs {
    fn new(slf: sys::mrb_value, func_sym: u32, args: Vec<sys::mrb_value>) -> Self {
        Self {
            slf,
            func_sym,
            args,
        }
    }

    fn with_block(self, block: sys::mrb_value) -> ProtectArgsWithBlock {
        ProtectArgsWithBlock {
            slf: self.slf,
            func_sym: self.func_sym,
            args: self.args,
            block,
        }
    }
}

#[allow(clippy::module_name_repetitions)]
pub trait ValueLike
where
    Self: Sized,
{
    fn inner(&self) -> sys::mrb_value;

    fn interp(&self) -> &Mrb;

    fn funcall<T, M, A>(&self, func: M, args: A) -> Result<T, ArtichokeError>
    where
        T: TryConvert<Value, From = types::Ruby, To = types::Rust>,
        M: AsRef<str>,
        A: AsRef<[Value]>,
    {
        unsafe extern "C" fn run_protected(
            mrb: *mut sys::mrb_state,
            data: sys::mrb_value,
        ) -> sys::mrb_value {
            let ptr = sys::mrb_sys_cptr_ptr(data);
            let args = Rc::from_raw(ptr as *const ProtectArgs);

            let value = sys::mrb_funcall_argv(
                mrb,
                args.slf,
                args.func_sym,
                // This will always unwrap because we've already checked that we
                // have fewer than `MRB_FUNCALL_ARGC_MAX` args, which is less
                // than i64 max value.
                i64::try_from(args.args.len()).unwrap_or_default(),
                args.args.as_ptr(),
            );
            sys::mrb_sys_raise_current_exception(mrb);
            value
        }
        // Ensure the borrow is out of scope by the time we eval code since
        // Rust-backed files and types may need to mutably borrow the `Mrb` to
        // get access to the underlying `MrbState`.
        let (mrb, _ctx) = {
            let borrow = self.interp().borrow();
            (borrow.mrb, borrow.ctx)
        };

        let _arena = self.interp().create_arena_savepoint();

        let args = args.as_ref().iter().map(Value::inner).collect::<Vec<_>>();
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
            "Calling {}#{} with {} args",
            types::Ruby::from(self.inner()),
            func.as_ref(),
            args.len()
        );
        let args = Rc::new(ProtectArgs::new(
            self.inner(),
            self.interp().borrow_mut().sym_intern(func.as_ref()),
            args,
        ));
        let value = unsafe {
            let data = sys::mrb_sys_cptr_value(mrb, Rc::into_raw(args) as *mut c_void);
            let mut state = <mem::MaybeUninit<sys::mrb_bool>>::uninit();

            let value = sys::mrb_protect(mrb, Some(run_protected), data, state.as_mut_ptr());
            if state.assume_init() != 0 {
                (*mrb).exc = sys::mrb_sys_obj_ptr(value);
            }
            value
        };
        let value = Value::new(self.interp(), value);

        match self.interp().last_error() {
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
                Err(ArtichokeError::UnreachableValue(value.inner().tt))
            }
            LastError::None => unsafe {
                T::try_convert(self.interp(), value).map_err(ArtichokeError::ConvertToRust)
            },
        }
    }

    fn funcall_with_block<T, M, A>(
        &self,
        func: M,
        args: A,
        block: Value,
    ) -> Result<T, ArtichokeError>
    where
        T: TryConvert<Value, From = types::Ruby, To = types::Rust>,
        M: AsRef<str>,
        A: AsRef<[Value]>,
    {
        unsafe extern "C" fn run_protected(
            mrb: *mut sys::mrb_state,
            data: sys::mrb_value,
        ) -> sys::mrb_value {
            let ptr = sys::mrb_sys_cptr_ptr(data);
            let args = Rc::from_raw(ptr as *const ProtectArgsWithBlock);

            let value = sys::mrb_funcall_with_block(
                mrb,
                args.slf,
                args.func_sym,
                // This will always unwrap because we've already checked that we
                // have fewer than `MRB_FUNCALL_ARGC_MAX` args, which is less
                // than i64 max value.
                i64::try_from(args.args.len()).unwrap_or_default(),
                args.args.as_ptr(),
                args.block,
            );
            sys::mrb_sys_raise_current_exception(mrb);
            value
        }
        // Ensure the borrow is out of scope by the time we eval code since
        // Rust-backed files and types may need to mutably borrow the `Mrb` to
        // get access to the underlying `MrbState`.
        let (mrb, _ctx) = {
            let borrow = self.interp().borrow();
            (borrow.mrb, borrow.ctx)
        };

        let _arena = self.interp().create_arena_savepoint();

        let args = args.as_ref().iter().map(Value::inner).collect::<Vec<_>>();
        if args.len() > MRB_FUNCALL_ARGC_MAX {
            warn!(
                "Too many args supplied to funcall_with_block: given {}, max {}.",
                args.len(),
                MRB_FUNCALL_ARGC_MAX
            );
            return Err(ArtichokeError::TooManyArgs {
                given: args.len(),
                max: MRB_FUNCALL_ARGC_MAX,
            });
        }
        trace!(
            "Calling {}#{} with {} args and block",
            types::Ruby::from(self.inner()),
            func.as_ref(),
            args.len()
        );
        let args = Rc::new(
            ProtectArgs::new(
                self.inner(),
                self.interp().borrow_mut().sym_intern(func.as_ref()),
                args,
            )
            .with_block(block.inner()),
        );
        let value = unsafe {
            let data = sys::mrb_sys_cptr_value(mrb, Rc::into_raw(args) as *mut c_void);
            let mut state = <mem::MaybeUninit<sys::mrb_bool>>::uninit();

            let value = sys::mrb_protect(mrb, Some(run_protected), data, state.as_mut_ptr());
            if state.assume_init() != 0 {
                (*mrb).exc = sys::mrb_sys_obj_ptr(value);
            }
            value
        };
        let value = Value::new(self.interp(), value);

        match self.interp().last_error() {
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
                Err(ArtichokeError::UnreachableValue(value.inner().tt))
            }
            LastError::None => unsafe {
                T::try_convert(self.interp(), value).map_err(ArtichokeError::ConvertToRust)
            },
        }
    }

    fn respond_to(&self, method: &str) -> Result<bool, ArtichokeError> {
        let method = Value::convert(self.interp(), method);
        self.funcall::<bool, _, _>("respond_to?", &[method])
    }
}

/// Wrapper around a [`sys::mrb_value`].
pub struct Value {
    interp: Mrb,
    value: sys::mrb_value,
}

impl Value {
    /// Construct a new [`Value`] from an interpreter and [`sys::mrb_value`].
    pub fn new(interp: &Mrb, value: sys::mrb_value) -> Self {
        Self {
            interp: Rc::clone(interp),
            value,
        }
    }

    /// The [`sys::mrb_value`] that this [`Value`] wraps.
    pub fn inner(&self) -> sys::mrb_value {
        self.value
    }

    /// Return this values [Rust-mapped type tag](types::Ruby).
    pub fn ruby_type(&self) -> types::Ruby {
        types::Ruby::from(self.value)
    }

    /// Some type tags like [`MRB_TT_UNDEF`](sys::mrb_vtype::MRB_TT_UNDEF) are
    /// internal to the mruby VM and manipulating them with the [`sys`] API is
    /// unspecified and may result in a segfault.
    ///
    /// After extracting a [`sys::mrb_value`] from the interpreter, check to see
    /// if the value is [unreachable](types::Ruby::Unreachable) and propagate an
    /// [`ArtichokeError::UnreachableValue`](crate::ArtichokeError::UnreachableValue) error.
    ///
    /// See: <https://github.com/mruby/mruby/issues/4460>
    pub fn is_unreachable(&self) -> bool {
        self.ruby_type() == types::Ruby::Unreachable
    }

    /// Prevent this value from being garbage collected.
    ///
    /// Calls [`sys::mrb_gc_protect`] on this value which adds it to the GC
    /// arena. This object will remain in the arena until
    /// [`ArenaIndex::restore`](crate::gc::ArenaIndex::restore) restores the
    /// arena to an index before this call to protect.
    pub fn protect(&self) {
        unsafe { sys::mrb_gc_protect(self.interp.borrow().mrb, self.value) }
    }

    /// Return whether this object is unreachable by any GC roots.
    pub fn is_dead(&self) -> bool {
        unsafe { sys::mrb_sys_value_is_dead(self.interp.borrow().mrb, self.value) }
    }

    /// Call `#to_s` on this [`Value`].
    ///
    /// This function can never fail.
    pub fn to_s(&self) -> String {
        self.funcall::<String, _, _>("to_s", &[])
            .unwrap_or_else(|_| "<unknown>".to_owned())
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
        format!("{}<{}>", self.ruby_type().class_name(), self.inspect())
    }

    /// Call `#inspect` on this [`Value`].
    ///
    /// This function can never fail.
    pub fn inspect(&self) -> String {
        self.funcall::<String, _, _>("inspect", &[])
            .unwrap_or_else(|_| "<unknown>".to_owned())
    }

    /// Consume `self` and try to convert `self` to type `T`.
    ///
    /// If you do not want to consume this [`Value`], use [`Value::itself`].
    pub fn try_into<T>(self) -> Result<T, ArtichokeError>
    where
        T: TryConvert<Self, From = types::Ruby, To = types::Rust>,
    {
        let interp = Rc::clone(&self.interp);
        unsafe { T::try_convert(&interp, self) }.map_err(ArtichokeError::ConvertToRust)
    }

    /// Call `#itself` on this [`Value`] and try to convert the result to type
    /// `T`.
    ///
    /// If you want to consume this [`Value`], use [`Value::try_into`].
    pub fn itself<T>(&self) -> Result<T, ArtichokeError>
    where
        T: TryConvert<Self, From = types::Ruby, To = types::Rust>,
    {
        self.clone().try_into::<T>()
    }

    /// Call `#freeze` on this [`Value`] and consume `self`.
    pub fn freeze(self) -> Result<Self, ArtichokeError> {
        let frozen = self.funcall::<Self, _, _>("freeze", &[])?;
        frozen.protect();
        Ok(frozen)
    }
}

impl ValueLike for Value {
    fn inner(&self) -> sys::mrb_value {
        self.value
    }

    fn interp(&self) -> &Mrb {
        &self.interp
    }
}

impl Convert<Value> for Value {
    type From = types::Ruby;
    type To = types::Rust;

    fn convert(_interp: &Mrb, value: Self) -> Self {
        value
    }
}

impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.to_s())
    }
}

impl fmt::Debug for Value {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.to_s_debug())
    }
}

impl Clone for Value {
    fn clone(&self) -> Self {
        if self.ruby_type() == types::Ruby::Data {
            panic!("Cannot safely clone a Value with type tag Ruby::Data.");
        }
        Self {
            interp: Rc::clone(&self.interp),
            value: self.value,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::convert::Convert;
    use crate::eval::MrbEval;
    use crate::gc::MrbGarbageCollection;
    use crate::value::{Value, ValueLike};
    use crate::ArtichokeError;

    #[test]
    fn to_s_true() {
        let interp = crate::interpreter().expect("mrb init");

        let value = Value::convert(&interp, true);
        let string = value.to_s();
        assert_eq!(string, "true");
    }

    #[test]
    fn debug_true() {
        let interp = crate::interpreter().expect("mrb init");

        let value = Value::convert(&interp, true);
        let debug = value.to_s_debug();
        assert_eq!(debug, "Boolean<true>");
    }

    #[test]
    fn inspect_true() {
        let interp = crate::interpreter().expect("mrb init");

        let value = Value::convert(&interp, true);
        let debug = value.inspect();
        assert_eq!(debug, "true");
    }

    #[test]
    fn to_s_false() {
        let interp = crate::interpreter().expect("mrb init");

        let value = Value::convert(&interp, false);
        let string = value.to_s();
        assert_eq!(string, "false");
    }

    #[test]
    fn debug_false() {
        let interp = crate::interpreter().expect("mrb init");

        let value = Value::convert(&interp, false);
        let debug = value.to_s_debug();
        assert_eq!(debug, "Boolean<false>");
    }

    #[test]
    fn inspect_false() {
        let interp = crate::interpreter().expect("mrb init");

        let value = Value::convert(&interp, false);
        let debug = value.inspect();
        assert_eq!(debug, "false");
    }

    #[test]
    fn to_s_nil() {
        let interp = crate::interpreter().expect("mrb init");

        let value = Value::convert(&interp, None::<Value>);
        let string = value.to_s();
        assert_eq!(string, "");
    }

    #[test]
    fn debug_nil() {
        let interp = crate::interpreter().expect("mrb init");

        let value = Value::convert(&interp, None::<Value>);
        let debug = value.to_s_debug();
        assert_eq!(debug, "NilClass<nil>");
    }

    #[test]
    fn inspect_nil() {
        let interp = crate::interpreter().expect("mrb init");

        let value = Value::convert(&interp, None::<Value>);
        let debug = value.inspect();
        assert_eq!(debug, "nil");
    }

    #[test]
    fn to_s_fixnum() {
        let interp = crate::interpreter().expect("mrb init");

        let value = Value::convert(&interp, 255);
        let string = value.to_s();
        assert_eq!(string, "255");
    }

    #[test]
    fn debug_fixnum() {
        let interp = crate::interpreter().expect("mrb init");

        let value = Value::convert(&interp, 255);
        let debug = value.to_s_debug();
        assert_eq!(debug, "Fixnum<255>");
    }

    #[test]
    fn inspect_fixnum() {
        let interp = crate::interpreter().expect("mrb init");

        let value = Value::convert(&interp, 255);
        let debug = value.inspect();
        assert_eq!(debug, "255");
    }

    #[test]
    fn to_s_string() {
        let interp = crate::interpreter().expect("mrb init");

        let value = Value::convert(&interp, "interstate");
        let string = value.to_s();
        assert_eq!(string, "interstate");
    }

    #[test]
    fn debug_string() {
        let interp = crate::interpreter().expect("mrb init");

        let value = Value::convert(&interp, "interstate");
        let debug = value.to_s_debug();
        assert_eq!(debug, r#"String<"interstate">"#);
    }

    #[test]
    fn inspect_string() {
        let interp = crate::interpreter().expect("mrb init");

        let value = Value::convert(&interp, "interstate");
        let debug = value.inspect();
        assert_eq!(debug, r#""interstate""#);
    }

    #[test]
    fn to_s_empty_string() {
        let interp = crate::interpreter().expect("mrb init");

        let value = Value::convert(&interp, "");
        let string = value.to_s();
        assert_eq!(string, "");
    }

    #[test]
    fn debug_empty_string() {
        let interp = crate::interpreter().expect("mrb init");

        let value = Value::convert(&interp, "");
        let debug = value.to_s_debug();
        assert_eq!(debug, r#"String<"">"#);
    }

    #[test]
    fn inspect_empty_string() {
        let interp = crate::interpreter().expect("mrb init");

        let value = Value::convert(&interp, "");
        let debug = value.inspect();
        assert_eq!(debug, r#""""#);
    }

    #[test]
    fn is_dead() {
        let interp = crate::interpreter().expect("mrb init");
        let arena = interp.create_arena_savepoint();
        let live = interp.eval("'dead'").expect("value");
        assert!(!live.is_dead());
        let dead = live;
        let live = interp.eval("'live'").expect("value");
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
        let interp = crate::interpreter().expect("mrb init");
        let arena = interp.create_arena_savepoint();
        let live = interp.eval("27").expect("value");
        assert!(!live.is_dead());
        let immediate = live;
        let live = interp.eval("64").expect("value");
        arena.restore();
        interp.full_gc();
        // immediate objects are never dead
        assert!(!immediate.is_dead());
        // the result of the most recent eval is always live even after a full
        // garbage collection
        assert!(!live.is_dead());
        // Fixnums are immediate even if they are created directly without an
        // interpreter.
        let fixnum = Value::convert(&interp, 99);
        assert!(!fixnum.is_dead());
    }

    #[test]
    fn funcall() {
        let interp = crate::interpreter().expect("mrb init");
        let nil = Value::convert(&interp, None::<Value>);
        assert!(nil.funcall::<bool, _, _>("nil?", &[]).expect("nil?"));
        let s = Value::convert(&interp, "foo");
        assert!(!s.funcall::<bool, _, _>("nil?", &[]).expect("nil?"));
        let delim = Value::convert(&interp, "");
        let split = s
            .funcall::<Vec<String>, _, _>("split", &[delim])
            .expect("split");
        assert_eq!(split, vec!["f".to_owned(), "o".to_owned(), "o".to_owned()])
    }

    #[test]
    fn funcall_different_types() {
        let interp = crate::interpreter().expect("mrb init");
        let nil = Value::convert(&interp, None::<Value>);
        let s = Value::convert(&interp, "foo");
        let eql = nil.funcall::<bool, _, _>("==", &[s]);
        assert_eq!(eql, Ok(false));
    }

    #[test]
    fn funcall_type_error() {
        let interp = crate::interpreter().expect("mrb init");
        let nil = Value::convert(&interp, None::<Value>);
        let s = Value::convert(&interp, "foo");
        let result = s.funcall::<String, _, _>("+", &[nil]);
        assert_eq!(
            result,
            Err(ArtichokeError::Exec(
                "TypeError: expected String".to_owned()
            ))
        );
    }

    #[test]
    fn funcall_method_not_exists() {
        let interp = crate::interpreter().expect("mrb init");
        let nil = Value::convert(&interp, None::<Value>);
        let s = Value::convert(&interp, "foo");
        let result = nil.funcall::<bool, _, _>("garbage_method_name", &[s]);
        assert_eq!(
            result,
            Err(ArtichokeError::Exec(
                "NoMethodError: undefined method 'garbage_method_name'".to_owned()
            ))
        );
    }
}
