use mruby_sys::*;

use crate::convert::fixnum::Int;
use crate::convert::{RubyToRustError, RustToRubyError, TryRuby};
use crate::value::{Ruby, Value};

impl TryRuby<Option<Int>> for Value {
    type RubyConvertError = RustToRubyError;

    fn try_ruby_convert(
        mrb: *mut mrb_state,
        value: Option<i64>,
    ) -> Result<Self, Self::RubyConvertError> {
        match value {
            Some(value) => Self::try_ruby_convert(mrb, value),
            None => Ok(Self::new(unsafe { mrb_sys_nil_value() })),
        }
    }
}

impl TryRuby<Value> for Option<Int> {
    type RubyConvertError = RubyToRustError;

    fn try_ruby_convert(mrb: *mut mrb_state, value: Value) -> Result<Self, Self::RubyConvertError> {
        match value.ruby_type() {
            Ruby::Nil => Ok(None),
            _ => Int::try_ruby_convert(mrb, value).map(Some),
        }
    }
}

#[cfg(test)]
mod tests {
    use mruby_sys::*;

    use super::*;

    #[test]
    fn try_value_from_some() {
        unsafe {
            let mrb = mrb_open();

            let value = Some(100);
            let value = Value::try_ruby_convert(mrb, value).expect("convert");
            let to_s = value.to_s(mrb);
            assert_eq!(value.ruby_type(), Ruby::Fixnum);
            assert_eq!(&to_s, "100");

            mrb_close(mrb);
        }
    }

    #[test]
    fn try_value_from_none() {
        unsafe {
            let mrb = mrb_open();

            let value = None as Option<i64>;
            let value = Value::try_ruby_convert(mrb, value).expect("convert");
            let to_s = value.to_s(mrb);
            assert_eq!(value.ruby_type(), Ruby::Nil);
            assert_eq!(&to_s, "");

            mrb_close(mrb);
        }
    }

    #[test]
    fn some_from_value() {
        unsafe {
            let mrb = mrb_open();
            let context = mrbc_context_new(mrb);

            let code = "100";
            let value = mrb_load_nstring_cxt(mrb, code.as_ptr() as *const i8, code.len(), context);
            let vec = <Option<i64>>::try_ruby_convert(mrb, Value::new(value)).expect("convert");
            assert_eq!(vec, Some(100));

            mrb_close(mrb);
        }
    }

    #[test]
    fn none_from_nil_value() {
        unsafe {
            let mrb = mrb_open();
            let context = mrbc_context_new(mrb);

            let code = "nil";
            let value = mrb_load_nstring_cxt(mrb, code.as_ptr() as *const i8, code.len(), context);
            let vec = <Option<i64>>::try_ruby_convert(mrb, Value::new(value)).expect("convert");
            assert_eq!(vec, None);

            mrb_close(mrb);
        }
    }
}
