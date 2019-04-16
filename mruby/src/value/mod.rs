use mruby_sys::*;

use std::ffi::CStr;

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

    #[allow(dead_code)]
    pub fn to_s(&self, mrb: *mut mrb_state) -> String {
        let inner = self.inner();
        // `mrb_str_to_str` is defined in object.h. This function has
        // specialized to_s implementations for String, Fixnum, Class, and
        // Module. For all other type tags, it calls `to_s` in the
        // mrb interpreter.
        let to_s = unsafe { mrb_str_to_str(mrb, inner) };
        let cstr = unsafe { mrb_str_to_cstr(mrb, to_s) };
        unsafe { CStr::from_ptr(cstr) }
            .to_str()
            .unwrap_or_else(|_| "<unknown>")
            .to_owned()
    }

    #[allow(dead_code)]
    fn to_s_debug(&self, mrb: *mut mrb_state) -> String {
        let inner = self.inner();
        let debug = unsafe { mrb_sys_value_debug_str(mrb, inner) };
        let cstr = unsafe { mrb_str_to_cstr(mrb, debug) };
        let string = unsafe { CStr::from_ptr(cstr) }
            .to_str()
            .unwrap_or_else(|_| "<unknown>");
        format!("{}<{}>", self.ruby_type().class_name(), string)
    }
}

#[cfg(test)]
mod tests {
    use mruby_sys::*;

    use crate::convert::*;
    use crate::value::*;

    #[test]
    fn to_s_true() {
        unsafe {
            let mrb = mrb_open();

            let value = Value::try_ruby_convert(mrb, true).expect("convert");
            let string = value.to_s(mrb);
            assert_eq!(string, "true");

            mrb_close(mrb);
        }
    }

    #[test]
    fn debug_true() {
        unsafe {
            let mrb = mrb_open();

            let value = Value::try_ruby_convert(mrb, true).expect("convert");
            let debug = value.to_s_debug(mrb);
            assert_eq!(debug, "Boolean<true>");

            mrb_close(mrb);
        }
    }

    #[test]
    fn to_s_false() {
        unsafe {
            let mrb = mrb_open();

            let value = Value::try_ruby_convert(mrb, false).expect("convert");
            let string = value.to_s(mrb);
            assert_eq!(string, "false");

            mrb_close(mrb);
        }
    }

    #[test]
    fn debug_false() {
        unsafe {
            let mrb = mrb_open();

            let value = Value::try_ruby_convert(mrb, false).expect("convert");
            let debug = value.to_s_debug(mrb);
            assert_eq!(debug, "Boolean<false>");

            mrb_close(mrb);
        }
    }

    #[test]
    fn to_s_nil() {
        unsafe {
            let mrb = mrb_open();

            let value = Value::try_ruby_convert(mrb, None as Option<i64>).expect("convert");
            let string = value.to_s(mrb);
            assert_eq!(string, "");

            mrb_close(mrb);
        }
    }

    #[test]
    fn debug_nil() {
        unsafe {
            let mrb = mrb_open();

            let value = Value::try_ruby_convert(mrb, None as Option<i64>).expect("convert");
            let debug = value.to_s_debug(mrb);
            assert_eq!(debug, "NilClass<nil>");

            mrb_close(mrb);
        }
    }

    #[test]
    fn to_s_fixnum() {
        unsafe {
            let mrb = mrb_open();

            let value = Value::try_ruby_convert(mrb, 255).expect("convert");
            let string = value.to_s(mrb);
            assert_eq!(string, "255");

            mrb_close(mrb);
        }
    }

    #[test]
    fn debug_fixnum() {
        unsafe {
            let mrb = mrb_open();

            let value = Value::try_ruby_convert(mrb, 255).expect("convert");
            let debug = value.to_s_debug(mrb);
            assert_eq!(debug, "Fixnum<255>");

            mrb_close(mrb);
        }
    }

    #[test]
    fn to_s_string() {
        unsafe {
            let mrb = mrb_open();

            let value = Value::try_ruby_convert(mrb, "interstate").expect("convert");
            let string = value.to_s(mrb);
            assert_eq!(string, "interstate");

            mrb_close(mrb);
        }
    }

    #[test]
    fn debug_string() {
        unsafe {
            let mrb = mrb_open();

            let value = Value::try_ruby_convert(mrb, "interstate").expect("convert");
            let debug = value.to_s_debug(mrb);
            assert_eq!(debug, r#"String<"interstate">"#);

            mrb_close(mrb);
        }
    }

    #[test]
    fn to_s_empty_string() {
        unsafe {
            let mrb = mrb_open();

            let value = Value::try_ruby_convert(mrb, "").expect("convert");
            let string = value.to_s(mrb);
            assert_eq!(string, "");

            mrb_close(mrb);
        }
    }

    #[test]
    fn debug_empty_string() {
        unsafe {
            let mrb = mrb_open();

            let value = Value::try_ruby_convert(mrb, "").expect("convert");
            let debug = value.to_s_debug(mrb);
            assert_eq!(debug, r#"String<"">"#);

            mrb_close(mrb);
        }
    }
}
