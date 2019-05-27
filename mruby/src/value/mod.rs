use log::{trace, warn};
use std::convert::TryFrom;
use std::rc::Rc;

use crate::convert::{FromMrb, TryFromMrb};
use crate::gc::GarbageCollection;
use crate::interpreter::{Mrb, MrbApi};
use crate::sys;
use crate::MrbError;

pub mod types;

#[allow(clippy::module_name_repetitions)]
pub trait ValueLike
where
    Self: Sized,
{
    // defined in vm.c
    const MRB_FUNCALL_ARGC_MAX: usize = 16;

    fn inner(&self) -> sys::mrb_value;

    fn interp(&self) -> &Mrb;

    fn funcall<T, M, A>(&self, method: M, args: A) -> Result<T, MrbError>
    where
        T: TryFromMrb<Value, From = types::Ruby, To = types::Rust>,
        M: AsRef<str>,
        A: AsRef<[Value]>,
    {
        let arena = self.interp().create_arena_savepoint();
        let args = args.as_ref().iter().map(Value::inner).collect::<Vec<_>>();
        if args.len() > Self::MRB_FUNCALL_ARGC_MAX {
            warn!(
                "Too many args supplied to funcall: given {}, max {}.",
                args.len(),
                Self::MRB_FUNCALL_ARGC_MAX
            );
            return Err(MrbError::TooManyArgs {
                given: args.len(),
                max: Self::MRB_FUNCALL_ARGC_MAX,
            });
        }
        let method = method.as_ref();
        trace!("Calling {}#{}", types::Ruby::from(self.inner()), method);
        // Scope the borrow so because we might require a borrow_mut in Rust
        // code we call into via the Ruby VM.
        let mrb = { self.interp().borrow().mrb };
        // This conversion will never fail because MRB_FUNCALL_ARGC_MAX is less
        // than `std::i64::MAX`.
        let size = i64::try_from(args.len()).expect("Unreachable");
        let value = unsafe {
            let sym = sys::mrb_intern(mrb, method.as_ptr() as *const i8, method.len());
            let value = sys::mrb_funcall_argv(mrb, self.inner(), sym, size, args.as_ptr());
            let value = Value::new(self.interp(), value);
            T::try_from_mrb(self.interp(), value).map_err(MrbError::ConvertToRust)
        };

        if let Some(backtrace) = self.interp().current_exception() {
            warn!("runtime error with exception backtrace: {}", backtrace);
            return Err(MrbError::Exec(backtrace));
        }

        arena.restore();
        value
    }
}

// We can't impl `fmt::Debug` because `mrb_sys_value_debug_str` requires a
// `mrb_state` interpreter, which we can't store on the `Value` because we
// construct it from Rust native types.
pub struct Value {
    interp: Mrb,
    value: sys::mrb_value,
}

impl Value {
    pub fn new(interp: &Mrb, value: sys::mrb_value) -> Self {
        Self {
            interp: Rc::clone(interp),
            value,
        }
    }

    pub fn inner(&self) -> sys::mrb_value {
        self.value
    }

    pub fn ruby_type(&self) -> types::Ruby {
        types::Ruby::from(self.value)
    }

    /// Some types like [`sys::mrb_vtype::MRB_TT_UNDEF`] are internal to the
    /// mruby VM and manipulating them with the [`sys`] API is unspecified and
    /// may result in a segfault.
    ///
    /// After extracting a [`sys::mrb_value`] from the interpreter, check to see
    /// if the value is [unreachable](types::Ruby::Unreachable) and propagate an
    /// [`MrbError::UnreachableValue`](crate::MrbError::UnreachableValue) error.
    ///
    /// See: <https://github.com/mruby/mruby/issues/4460>
    pub fn is_unreachable(&self) -> bool {
        self.ruby_type() == types::Ruby::Unreachable
    }

    pub fn is_dead(&self) -> bool {
        unsafe { sys::mrb_sys_value_is_dead(self.interp.borrow().mrb, self.value) }
    }

    pub fn to_s(&self) -> String {
        let arena = self.interp.create_arena_savepoint();
        let mrb = { self.interp.borrow().mrb };
        // `mrb_str_to_str` is defined in object.h. This function has
        // specialized to_s implementations for String, Fixnum, Class, and
        // Module. For all other type tags, it calls `to_s` in the
        // mrb interpreter.
        let to_s = unsafe { sys::mrb_str_to_str(mrb, self.value) };
        let to_s = unsafe { String::try_from_mrb(&self.interp, Self::new(&self.interp, to_s)) }
            .unwrap_or_else(|_| "<unknown>".to_owned());
        arena.restore();
        to_s
    }

    pub fn to_s_debug(&self) -> String {
        format!("{}<{}>", self.ruby_type().class_name(), self.inspect())
    }

    pub fn inspect(&self) -> String {
        self.funcall::<String, _, _>("inspect", &[])
            .unwrap_or_else(|_| "<unknown>".to_owned())
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

impl FromMrb<Value> for Value {
    type From = types::Ruby;
    type To = types::Rust;

    fn from_mrb(_interp: &Mrb, value: Self) -> Self {
        value
    }
}

#[cfg(test)]
mod tests {
    use crate::convert::*;
    use crate::eval::MrbEval;
    use crate::interpreter::*;
    use crate::value::*;

    #[test]
    fn to_s_true() {
        unsafe {
            let interp = Interpreter::create().expect("mrb init");

            let value = Value::try_from_mrb(&interp, true).expect("convert");
            let string = value.to_s();
            assert_eq!(string, "true");
        }
    }

    #[test]
    fn debug_true() {
        unsafe {
            let interp = Interpreter::create().expect("mrb init");

            let value = Value::try_from_mrb(&interp, true).expect("convert");
            let debug = value.to_s_debug();
            assert_eq!(debug, "Boolean<true>");
        }
    }

    #[test]
    fn inspect_true() {
        unsafe {
            let interp = Interpreter::create().expect("mrb init");

            let value = Value::try_from_mrb(&interp, true).expect("convert");
            let debug = value.inspect();
            assert_eq!(debug, "true");
        }
    }

    #[test]
    fn to_s_false() {
        unsafe {
            let interp = Interpreter::create().expect("mrb init");

            let value = Value::try_from_mrb(&interp, false).expect("convert");
            let string = value.to_s();
            assert_eq!(string, "false");
        }
    }

    #[test]
    fn debug_false() {
        unsafe {
            let interp = Interpreter::create().expect("mrb init");

            let value = Value::try_from_mrb(&interp, false).expect("convert");
            let debug = value.to_s_debug();
            assert_eq!(debug, "Boolean<false>");
        }
    }

    #[test]
    fn inspect_false() {
        unsafe {
            let interp = Interpreter::create().expect("mrb init");

            let value = Value::try_from_mrb(&interp, false).expect("convert");
            let debug = value.inspect();
            assert_eq!(debug, "false");
        }
    }

    #[test]
    fn to_s_nil() {
        unsafe {
            let interp = Interpreter::create().expect("mrb init");

            let value = Value::try_from_mrb(&interp, None::<Value>).expect("convert");
            let string = value.to_s();
            assert_eq!(string, "");
        }
    }

    #[test]
    fn debug_nil() {
        unsafe {
            let interp = Interpreter::create().expect("mrb init");

            let value = Value::try_from_mrb(&interp, None::<Value>).expect("convert");
            let debug = value.to_s_debug();
            assert_eq!(debug, "NilClass<nil>");
        }
    }

    #[test]
    fn inspect_nil() {
        unsafe {
            let interp = Interpreter::create().expect("mrb init");

            let value = Value::try_from_mrb(&interp, None::<Value>).expect("convert");
            let debug = value.inspect();
            assert_eq!(debug, "nil");
        }
    }

    #[test]
    fn to_s_fixnum() {
        unsafe {
            let interp = Interpreter::create().expect("mrb init");

            let value = Value::try_from_mrb(&interp, 255).expect("convert");
            let string = value.to_s();
            assert_eq!(string, "255");
        }
    }

    #[test]
    fn debug_fixnum() {
        unsafe {
            let interp = Interpreter::create().expect("mrb init");

            let value = Value::try_from_mrb(&interp, 255).expect("convert");
            let debug = value.to_s_debug();
            assert_eq!(debug, "Fixnum<255>");
        }
    }

    #[test]
    fn inspect_fixnum() {
        unsafe {
            let interp = Interpreter::create().expect("mrb init");

            let value = Value::try_from_mrb(&interp, 255).expect("convert");
            let debug = value.inspect();
            assert_eq!(debug, "255");
        }
    }

    #[test]
    fn to_s_string() {
        unsafe {
            let interp = Interpreter::create().expect("mrb init");

            let value = Value::try_from_mrb(&interp, "interstate").expect("convert");
            let string = value.to_s();
            assert_eq!(string, "interstate");
        }
    }

    #[test]
    fn debug_string() {
        unsafe {
            let interp = Interpreter::create().expect("mrb init");

            let value = Value::try_from_mrb(&interp, "interstate").expect("convert");
            let debug = value.to_s_debug();
            assert_eq!(debug, r#"String<"interstate">"#);
        }
    }

    #[test]
    fn inspect_string() {
        unsafe {
            let interp = Interpreter::create().expect("mrb init");

            let value = Value::try_from_mrb(&interp, "interstate").expect("convert");
            let debug = value.inspect();
            assert_eq!(debug, r#""interstate""#);
        }
    }

    #[test]
    fn to_s_empty_string() {
        unsafe {
            let interp = Interpreter::create().expect("mrb init");

            let value = Value::try_from_mrb(&interp, "").expect("convert");
            let string = value.to_s();
            assert_eq!(string, "");
        }
    }

    #[test]
    fn debug_empty_string() {
        unsafe {
            let interp = Interpreter::create().expect("mrb init");

            let value = Value::try_from_mrb(&interp, "").expect("convert");
            let debug = value.to_s_debug();
            assert_eq!(debug, r#"String<"">"#);
        }
    }

    #[test]
    fn inspect_empty_string() {
        unsafe {
            let interp = Interpreter::create().expect("mrb init");

            let value = Value::try_from_mrb(&interp, "").expect("convert");
            let debug = value.inspect();
            assert_eq!(debug, r#""""#);
        }
    }

    #[test]
    fn is_dead() {
        let interp = Interpreter::create().expect("mrb init");
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
        let interp = Interpreter::create().expect("mrb init");
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
        let fixnum = interp.fixnum(99);
        assert!(!fixnum.is_dead());
    }

    #[test]
    fn funcall() {
        let interp = Interpreter::create().expect("mrb init");
        let nil = interp.nil();
        assert!(nil.funcall::<bool, _, _>("nil?", &[]).expect("nil?"));
        let s = interp.string("foo");
        assert!(!s.funcall::<bool, _, _>("nil?", &[]).expect("nil?"));
        let delim = interp.string("");
        let split = s
            .funcall::<Vec<String>, _, _>("split", &[delim])
            .expect("split");
        assert_eq!(split, vec!["f".to_owned(), "o".to_owned(), "o".to_owned()])
    }

    #[test]
    fn funcall_different_types() {
        let interp = Interpreter::create().expect("mrb init");
        let nil = interp.nil();
        let s = interp.string("foo");
        let eql = nil.funcall::<bool, _, _>("==", &[s]);
        assert_eq!(eql, Ok(false));
    }

    #[test]
    fn funcall_type_error() {
        let interp = Interpreter::create().expect("mrb init");
        let nil = interp.nil();
        let s = interp.string("foo");
        let result = s.funcall::<String, _, _>("+", &[nil]);
        assert_eq!(
            result,
            Err(MrbError::Exec("TypeError: expected String".to_owned()))
        );
    }

    #[test]
    fn funcall_method_not_exists() {
        let interp = Interpreter::create().expect("mrb init");
        let nil = interp.nil();
        let s = interp.string("foo");
        let result = nil.funcall::<bool, _, _>("garbage_method_name", &[s]);
        assert_eq!(
            result,
            Err(MrbError::Exec(
                "NoMethodError: undefined method 'garbage_method_name'".to_owned()
            ))
        );
    }
}
