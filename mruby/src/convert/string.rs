use mruby_sys::*;
use std::ffi::{CStr, CString};

use crate::convert::{Error, RubyToRustError, RustToRubyError, TryRuby};
use crate::value::{Ruby, Rust, Value};

impl TryRuby<String> for Value {
    type RubyConvertError = RustToRubyError;

    fn try_ruby_convert(
        mrb: *mut mrb_state,
        value: String,
    ) -> Result<Self, Self::RubyConvertError> {
        let value: &str = &value;
        Self::try_ruby_convert(mrb, value)
    }
}
impl TryRuby<&str> for Value {
    type RubyConvertError = RustToRubyError;

    fn try_ruby_convert(mrb: *mut mrb_state, value: &str) -> Result<Self, Self::RubyConvertError> {
        match CString::new(value) {
            Ok(cstr) => Ok(Self::new(unsafe { mrb_str_new_cstr(mrb, cstr.as_ptr()) })),
            Err(_) => Err(Error {
                from: Rust::String,
                to: Ruby::String,
            }),
        }
    }
}

impl TryRuby<Value> for String {
    type RubyConvertError = RubyToRustError;

    fn try_ruby_convert(mrb: *mut mrb_state, value: Value) -> Result<Self, Self::RubyConvertError> {
        match value.ruby_type() {
            Ruby::String => {
                let mut value = value.inner();
                let cstr = unsafe { mrb_string_value_cstr(mrb, &mut value) };
                match unsafe { CStr::from_ptr(cstr) }.to_str() {
                    Ok(string) => Ok(string.to_owned()),
                    Err(_) => Err(Error {
                        from: Ruby::String,
                        to: Rust::String,
                    }),
                }
            }
            type_tag => Err(Error {
                from: type_tag,
                to: Rust::String,
            }),
        }
    }
}

#[cfg(test)]
mod tests {
    use mruby_sys::*;

    use super::*;

    #[test]
    fn try_value_from_string() {
        unsafe {
            let mrb = mrb_open();

            let value = "rust-mruby-bindings".to_owned();
            let value = Value::try_ruby_convert(mrb, value).expect("convert");
            let to_s = value.to_s(mrb);
            assert_eq!(value.ruby_type(), Ruby::String);
            assert_eq!(&to_s, "rust-mruby-bindings");

            mrb_close(mrb);
        }
    }

    #[test]
    fn try_value_from_empty_string() {
        unsafe {
            let mrb = mrb_open();

            let value = "".to_owned();
            let value = Value::try_ruby_convert(mrb, value).expect("convert");
            let to_s = value.to_s(mrb);
            assert_eq!(value.ruby_type(), Ruby::String);
            assert_eq!(&to_s, "");

            mrb_close(mrb);
        }
    }

    #[test]
    fn try_value_from_str() {
        unsafe {
            let mrb = mrb_open();

            let value = "rust-mruby-bindings";
            let value = Value::try_ruby_convert(mrb, value).expect("convert");
            let to_s = value.to_s(mrb);
            assert_eq!(value.ruby_type(), Ruby::String);
            assert_eq!(&to_s, "rust-mruby-bindings");

            mrb_close(mrb);
        }
    }

    #[test]
    fn try_value_from_empty_str() {
        unsafe {
            let mrb = mrb_open();

            let value = "";
            let value = Value::try_ruby_convert(mrb, value).expect("convert");
            let to_s = value.to_s(mrb);
            assert_eq!(value.ruby_type(), Ruby::String);
            assert_eq!(&to_s, "");

            mrb_close(mrb);
        }
    }

    #[test]
    fn string_from_value() {
        unsafe {
            let mrb = mrb_open();
            let context = mrbc_context_new(mrb);

            let code = "'rust-mruby-bindings'";
            let value = mrb_load_nstring_cxt(mrb, code.as_ptr() as *const i8, code.len(), context);
            let vec = String::try_ruby_convert(mrb, Value::new(value)).expect("convert");
            assert_eq!(vec, "rust-mruby-bindings".to_owned());

            mrb_close(mrb);
        }
    }

    #[test]
    fn string_from_empty_value() {
        unsafe {
            let mrb = mrb_open();
            let context = mrbc_context_new(mrb);

            let code = "''";
            let value = mrb_load_nstring_cxt(mrb, code.as_ptr() as *const i8, code.len(), context);
            let vec = String::try_ruby_convert(mrb, Value::new(value)).expect("convert");
            assert_eq!(vec, "".to_owned());

            mrb_close(mrb);
        }
    }
}
