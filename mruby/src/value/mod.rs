use std::rc::Rc;

use crate::convert::TryFromMrb;
use crate::gc::GarbageCollection;
use crate::interpreter::Mrb;
use crate::sys;

pub mod types;

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
        let to_s = unsafe { String::try_from_mrb(&self.interp, Value::new(&self.interp, to_s)) }
            .unwrap_or_else(|_| "<unknown>".to_owned());
        arena.restore();
        to_s
    }

    pub fn to_s_debug(&self) -> String {
        format!("{}<{}>", self.ruby_type().class_name(), self.inspect())
    }

    pub fn inspect(&self) -> String {
        let arena = self.interp.create_arena_savepoint();
        let mrb = { self.interp.borrow().mrb };
        let debug = unsafe { sys::mrb_sys_value_debug_str(mrb, self.value) };
        let debug = unsafe { String::try_from_mrb(&self.interp, Value::new(&self.interp, debug)) }
            .unwrap_or_else(|_| "<unknown>".to_owned());
        arena.restore();
        debug
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
}
