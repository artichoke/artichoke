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
        unsafe {
            // `mrb_str_to_str` is defined in object.h. This function has
            // specialized to_s implementations for String, Fixnum, Class, and
            // Module. For all other type tags, it calls `to_s` in the
            // mrb interpreter.
            let to_s = mrb_str_to_str(mrb, self.0);
            let to_s_str = mrb_str_to_cstr(mrb, to_s);
            let string = CStr::from_ptr(to_s_str)
                .to_str()
                .unwrap_or_else(|_| "<unknown>");
            string.to_owned()
        }
    }

    #[allow(dead_code)]
    fn to_s_debug(&self, mrb: *mut mrb_state) -> String {
        let debug = unsafe {
            let debug = mrb_sys_value_debug_str(mrb, self.0);
            let debug_str = mrb_str_to_cstr(mrb, debug);
            let string = CStr::from_ptr(debug_str)
                .to_str()
                .unwrap_or_else(|_| "<unknown>");
            let owned = string.to_owned();
            mrb_close(mrb);
            owned
        };
        format!("{}<{}>", self.ruby_type().class_name(), debug)
    }
}

trait TryValue {
    type Error;

    fn try_value(&self, mrb: *mut mrb_state) -> Result<Value, Self::Error>;
}

#[allow(dead_code)]
#[derive(Clone, Debug, Eq, PartialEq)]
struct ConvertError<F, T> {
    from: F,
    to: T,
}

#[cfg(__nope__)]
mod tests {
    use crate::value::*;
    use std::convert::TryFrom;

    #[test]
    fn to_s_true() {
        let value = Value::try_from(true).expect("convert");
        let string = value.to_s();
        assert_eq!(string, "true");
    }

    #[test]
    fn debug_true() {
        let value = Value::try_from(true).expect("convert");
        let debug = format!("{:?}", value);
        assert_eq!(debug, "Bool<true>");
    }

    #[test]
    fn to_s_false() {
        let value = Value::try_from(false).expect("convert");
        let string = value.to_s();
        assert_eq!(string, "false");
    }

    #[test]
    fn debug_false() {
        let value = Value::try_from(false).expect("convert");
        let debug = format!("{:?}", value);
        assert_eq!(debug, "Bool<false>");
    }

    #[test]
    fn to_s_nil() {
        let value = Value::try_from(None as Option<bool>).expect("convert");
        let string = value.to_s();
        assert_eq!(string, "");
    }

    #[test]
    fn debug_nil() {
        let value = Value::try_from(None as Option<bool>).expect("convert");
        let debug = format!("{:?}", value);
        assert_eq!(debug, "Nil<nil>");
    }

    #[test]
    fn to_s_unsigned_fixnum() {
        let value = Value::try_from(255_u64).expect("convert");
        let string = value.to_s();
        assert_eq!(string, "255");
    }

    #[test]
    fn debug_unsigned_fixnum() {
        let value = Value::try_from(255_u64).expect("convert");
        let debug = format!("{:?}", value);
        assert_eq!(debug, "Fixnum<255>");
    }
}
