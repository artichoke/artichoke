use mruby_sys::*;
use std::ffi::CStr;

use crate::interpreter::MrbApi;

mod types;

pub use self::types::*;

// We can't impl `fmt::Debug` because `mrb_sys_value_debug_str` requires a
// `mrb_state` interpreter, which we can't store on the `Value` because we
// construct it from Rust native types.
pub struct Value(mrb_value);

impl Value {
    pub fn new(inner: mrb_value) -> Self {
        Self(inner)
    }

    pub fn inner(&self) -> mrb_value {
        self.0
    }

    pub fn ruby_type(&self) -> types::Ruby {
        types::Ruby::from(self.0)
    }

    pub unsafe fn to_s(&self, api: &MrbApi) -> String {
        let inner = self.inner();
        // `mrb_str_to_str` is defined in object.h. This function has
        // specialized to_s implementations for String, Fixnum, Class, and
        // Module. For all other type tags, it calls `to_s` in the
        // mrb interpreter.
        let to_s = mrb_str_to_str(api.mrb(), inner);
        let cstr = mrb_str_to_cstr(api.mrb(), to_s);
        CStr::from_ptr(cstr)
            .to_str()
            .unwrap_or_else(|_| "<unknown>")
            .to_owned()
    }

    pub unsafe fn to_s_debug(&self, api: &MrbApi) -> String {
        let inner = self.inner();
        let debug = mrb_sys_value_debug_str(api.mrb(), inner);
        let cstr = mrb_str_to_cstr(api.mrb(), debug);
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
            let api = interp.borrow_mut();

            let value = Value::try_from_mrb(&api, true).expect("convert");
            let string = value.to_s(&api);
            assert_eq!(string, "true");
        }
    }

    #[test]
    fn debug_true() {
        unsafe {
            let interp = Interpreter::create().expect("mrb init");
            let api = interp.borrow_mut();

            let value = Value::try_from_mrb(&api, true).expect("convert");
            let debug = value.to_s_debug(&api);
            assert_eq!(debug, "Boolean<true>");
        }
    }

    #[test]
    fn to_s_false() {
        unsafe {
            let interp = Interpreter::create().expect("mrb init");
            let api = interp.borrow_mut();

            let value = Value::try_from_mrb(&api, false).expect("convert");
            let string = value.to_s(&api);
            assert_eq!(string, "false");
        }
    }

    #[test]
    fn debug_false() {
        unsafe {
            let interp = Interpreter::create().expect("mrb init");
            let api = interp.borrow_mut();

            let value = Value::try_from_mrb(&api, false).expect("convert");
            let debug = value.to_s_debug(&api);
            assert_eq!(debug, "Boolean<false>");
        }
    }

    #[test]
    fn to_s_nil() {
        unsafe {
            let interp = Interpreter::create().expect("mrb init");
            let api = interp.borrow_mut();

            let value = Value::try_from_mrb(&api, None as Option<Value>).expect("convert");
            let string = value.to_s(&api);
            assert_eq!(string, "");
        }
    }

    #[test]
    fn debug_nil() {
        unsafe {
            let interp = Interpreter::create().expect("mrb init");
            let api = interp.borrow_mut();

            let value = Value::try_from_mrb(&api, None as Option<Value>).expect("convert");
            let debug = value.to_s_debug(&api);
            assert_eq!(debug, "NilClass<nil>");
        }
    }

    #[test]
    fn to_s_fixnum() {
        unsafe {
            let interp = Interpreter::create().expect("mrb init");
            let api = interp.borrow_mut();

            let value = Value::try_from_mrb(&api, 255).expect("convert");
            let string = value.to_s(&api);
            assert_eq!(string, "255");
        }
    }

    #[test]
    fn debug_fixnum() {
        unsafe {
            let interp = Interpreter::create().expect("mrb init");
            let api = interp.borrow_mut();

            let value = Value::try_from_mrb(&api, 255).expect("convert");
            let debug = value.to_s_debug(&api);
            assert_eq!(debug, "Fixnum<255>");
        }
    }

    #[test]
    fn to_s_string() {
        unsafe {
            let interp = Interpreter::create().expect("mrb init");
            let api = interp.borrow_mut();

            let value = Value::try_from_mrb(&api, "interstate").expect("convert");
            let string = value.to_s(&api);
            assert_eq!(string, "interstate");
        }
    }

    #[test]
    fn debug_string() {
        unsafe {
            let interp = Interpreter::create().expect("mrb init");
            let api = interp.borrow_mut();

            let value = Value::try_from_mrb(&api, "interstate").expect("convert");
            let debug = value.to_s_debug(&api);
            assert_eq!(debug, r#"String<"interstate">"#);
        }
    }

    #[test]
    fn to_s_empty_string() {
        unsafe {
            let interp = Interpreter::create().expect("mrb init");
            let api = interp.borrow_mut();

            let value = Value::try_from_mrb(&api, "").expect("convert");
            let string = value.to_s(&api);
            assert_eq!(string, "");
        }
    }

    #[test]
    fn debug_empty_string() {
        unsafe {
            let interp = Interpreter::create().expect("mrb init");
            let api = interp.borrow_mut();

            let value = Value::try_from_mrb(&api, "").expect("convert");
            let debug = value.to_s_debug(&api);
            assert_eq!(debug, r#"String<"">"#);
        }
    }
}
