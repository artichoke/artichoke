use std::ffi::CStr;

use crate::interpreter::Mrb;
use crate::sys;

mod types;

pub use self::types::*;

// We can't impl `fmt::Debug` because `mrb_sys_value_debug_str` requires a
// `mrb_state` interpreter, which we can't store on the `Value` because we
// construct it from Rust native types.
pub struct Value(sys::mrb_value);

impl Value {
    pub fn new(inner: sys::mrb_value) -> Self {
        Self(inner)
    }

    pub fn inner(&self) -> sys::mrb_value {
        self.0
    }

    pub fn ruby_type(&self) -> types::Ruby {
        types::Ruby::from(self.0)
    }

    pub unsafe fn to_s(&self, mrb: &Mrb) -> String {
        let inner = self.inner();
        // `mrb_str_to_str` is defined in object.h. This function has
        // specialized to_s implementations for String, Fixnum, Class, and
        // Module. For all other type tags, it calls `to_s` in the
        // mrb interpreter.
        let to_s = sys::mrb_str_to_str(mrb.borrow().mrb, inner);
        let cstr = sys::mrb_str_to_cstr(mrb.borrow().mrb, to_s);
        CStr::from_ptr(cstr)
            .to_str()
            .unwrap_or_else(|_| "<unknown>")
            .to_owned()
    }

    pub unsafe fn to_s_debug(&self, mrb: &Mrb) -> String {
        let inner = self.inner();
        let debug = sys::mrb_sys_value_debug_str(mrb.borrow().mrb, inner);
        let cstr = sys::mrb_str_to_cstr(mrb.borrow().mrb, debug);
        let string = CStr::from_ptr(cstr).to_string_lossy();
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
            let string = value.to_s(&interp);
            assert_eq!(string, "true");
        }
    }

    #[test]
    fn debug_true() {
        unsafe {
            let interp = Interpreter::create().expect("mrb init");

            let value = Value::try_from_mrb(&interp, true).expect("convert");
            let debug = value.to_s_debug(&interp);
            assert_eq!(debug, "Boolean<true>");
        }
    }

    #[test]
    fn to_s_false() {
        unsafe {
            let interp = Interpreter::create().expect("mrb init");

            let value = Value::try_from_mrb(&interp, false).expect("convert");
            let string = value.to_s(&interp);
            assert_eq!(string, "false");
        }
    }

    #[test]
    fn debug_false() {
        unsafe {
            let interp = Interpreter::create().expect("mrb init");

            let value = Value::try_from_mrb(&interp, false).expect("convert");
            let debug = value.to_s_debug(&interp);
            assert_eq!(debug, "Boolean<false>");
        }
    }

    #[test]
    fn to_s_nil() {
        unsafe {
            let interp = Interpreter::create().expect("mrb init");

            let value = Value::try_from_mrb(&interp, None::<Value>).expect("convert");
            let string = value.to_s(&interp);
            assert_eq!(string, "");
        }
    }

    #[test]
    fn debug_nil() {
        unsafe {
            let interp = Interpreter::create().expect("mrb init");

            let value = Value::try_from_mrb(&interp, None::<Value>).expect("convert");
            let debug = value.to_s_debug(&interp);
            assert_eq!(debug, "NilClass<nil>");
        }
    }

    #[test]
    fn to_s_fixnum() {
        unsafe {
            let interp = Interpreter::create().expect("mrb init");

            let value = Value::try_from_mrb(&interp, 255).expect("convert");
            let string = value.to_s(&interp);
            assert_eq!(string, "255");
        }
    }

    #[test]
    fn debug_fixnum() {
        unsafe {
            let interp = Interpreter::create().expect("mrb init");

            let value = Value::try_from_mrb(&interp, 255).expect("convert");
            let debug = value.to_s_debug(&interp);
            assert_eq!(debug, "Fixnum<255>");
        }
    }

    #[test]
    fn to_s_string() {
        unsafe {
            let interp = Interpreter::create().expect("mrb init");

            let value = Value::try_from_mrb(&interp, "interstate").expect("convert");
            let string = value.to_s(&interp);
            assert_eq!(string, "interstate");
        }
    }

    #[test]
    fn debug_string() {
        unsafe {
            let interp = Interpreter::create().expect("mrb init");

            let value = Value::try_from_mrb(&interp, "interstate").expect("convert");
            let debug = value.to_s_debug(&interp);
            assert_eq!(debug, r#"String<"interstate">"#);
        }
    }

    #[test]
    fn to_s_empty_string() {
        unsafe {
            let interp = Interpreter::create().expect("mrb init");

            let value = Value::try_from_mrb(&interp, "").expect("convert");
            let string = value.to_s(&interp);
            assert_eq!(string, "");
        }
    }

    #[test]
    fn debug_empty_string() {
        unsafe {
            let interp = Interpreter::create().expect("mrb init");

            let value = Value::try_from_mrb(&interp, "").expect("convert");
            let debug = value.to_s_debug(&interp);
            assert_eq!(debug, r#"String<"">"#);
        }
    }
}
