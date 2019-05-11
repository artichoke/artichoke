use std::ffi::CStr;

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
    pub fn new(interp: Mrb, value: sys::mrb_value) -> Self {
        Self { interp, value }
    }

    pub fn inner(&self) -> sys::mrb_value {
        self.value
    }

    pub fn ruby_type(&self) -> types::Ruby {
        types::Ruby::from(self.value)
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
        let cstr = unsafe { sys::mrb_str_to_cstr(mrb, to_s) };
        arena.restore();
        unsafe { CStr::from_ptr(cstr) }
            .to_str()
            .unwrap_or_else(|_| "<unknown>")
            .to_owned()
    }

    pub fn to_s_debug(&self) -> String {
        let arena = self.interp.create_arena_savepoint();
        let mrb = { self.interp.borrow().mrb };
        let debug = unsafe { sys::mrb_sys_value_debug_str(mrb, self.value) };
        let cstr = unsafe { sys::mrb_str_to_cstr(mrb, debug) };
        let string = unsafe { CStr::from_ptr(cstr) }.to_string_lossy();
        arena.restore();
        format!("{}<{}>", self.ruby_type().class_name(), string)
    }
}

#[cfg(test)]
mod tests {
    use crate::convert::*;
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
}
