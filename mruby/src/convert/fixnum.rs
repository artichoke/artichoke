use mruby_sys::*;

use crate::convert::{Error, RubyToRustError, RustToRubyError, TryRuby};
use crate::value::{Ruby, Rust, Value};

pub type Int = i64;

impl TryRuby<Int> for Value {
    type RubyConvertError = RustToRubyError;

    fn try_ruby_convert(_mrb: *mut mrb_state, value: Int) -> Result<Self, Self::RubyConvertError> {
        Ok(Self::new(unsafe { mrb_sys_fixnum_value(value) }))
    }
}

impl TryRuby<Value> for Int {
    type RubyConvertError = RubyToRustError;

    fn try_ruby_convert(
        _mrb: *mut mrb_state,
        value: Value,
    ) -> Result<Self, Self::RubyConvertError> {
        match value.ruby_type() {
            Ruby::Fixnum => {
                let inner = value.inner();
                let int = unsafe { mrb_sys_fixnum_to_cint(inner) };
                Ok(int)
            }
            type_tag => Err(Error {
                from: type_tag,
                to: Rust::SignedInt,
            }),
        }
    }
}

#[cfg(test)]
mod tests {
    use mruby_sys::*;

    use super::*;

    const MAX: i64 = Int::max_value() as i64;
    const MIN: i64 = Int::min_value() as i64;
    const ZERO: Int = 0;

    #[test]
    fn try_value() {
        unsafe {
            let mrb = mrb_open();

            let value = Value::try_ruby_convert(mrb, ZERO).expect("convert");
            assert_eq!(value.ruby_type(), Ruby::Fixnum);
            assert_eq!(mrb_sys_fixnum_to_cint(value.inner()), 0);

            let value = Value::try_ruby_convert(mrb, Int::max_value()).expect("convert");
            assert_eq!(value.ruby_type(), Ruby::Fixnum);
            assert_eq!(mrb_sys_fixnum_to_cint(value.inner()), MAX);

            let value = Value::try_ruby_convert(mrb, Int::min_value()).expect("convert");
            assert_eq!(value.ruby_type(), Ruby::Fixnum);
            assert_eq!(mrb_sys_fixnum_to_cint(value.inner()), MIN);

            mrb_close(mrb);
        }
    }

    #[test]
    fn value_from_zero() {
        unsafe {
            let mrb = mrb_open();

            let value = Value::try_ruby_convert(mrb, ZERO).expect("convert");
            assert_eq!(value.ruby_type(), Ruby::Fixnum);
            assert_eq!(mrb_sys_fixnum_to_cint(value.inner()), 0);

            mrb_close(mrb);
        }
    }

    #[test]
    fn value_from_max() {
        unsafe {
            let mrb = mrb_open();

            let value = Value::try_ruby_convert(mrb, Int::max_value()).expect("convert");
            assert_eq!(value.ruby_type(), Ruby::Fixnum);
            assert_eq!(mrb_sys_fixnum_to_cint(value.inner()), MAX);

            mrb_close(mrb);
        }
    }

    #[test]
    fn value_from_min() {
        unsafe {
            let mrb = mrb_open();

            let value = Value::try_ruby_convert(mrb, Int::min_value()).expect("convert");
            assert_eq!(value.ruby_type(), Ruby::Fixnum);
            assert_eq!(mrb_sys_fixnum_to_cint(value.inner()), MIN);

            mrb_close(mrb);
        }
    }

    #[test]
    fn int_from_zero_value() {
        unsafe {
            let mrb = mrb_open();

            let value =
                Int::try_ruby_convert(mrb, Value::new(mrb_sys_fixnum_value(0))).expect("convert");
            assert_eq!(value, ZERO);

            mrb_close(mrb);
        }
    }

    #[test]
    fn int_from_max_value() {
        unsafe {
            let mrb = mrb_open();

            let value =
                Int::try_ruby_convert(mrb, Value::new(mrb_sys_fixnum_value(MAX))).expect("convert");
            assert_eq!(value, Int::max_value());

            mrb_close(mrb);
        }
    }

    #[test]
    fn int_from_min_value() {
        unsafe {
            let mrb = mrb_open();

            let value =
                Int::try_ruby_convert(mrb, Value::new(mrb_sys_fixnum_value(MIN))).expect("convert");
            assert_eq!(value, Int::min_value());

            mrb_close(mrb);
        }
    }

    #[test]
    fn err_from_nil_value() {
        unsafe {
            let mrb = mrb_open();

            let err = Int::try_ruby_convert(mrb, Value::new(mrb_sys_nil_value()));
            let expected = Error {
                from: Ruby::Nil,
                to: Rust::SignedInt,
            };
            assert_eq!(err, Err(expected));

            mrb_close(mrb);
        }
    }

    #[test]
    fn err_from_bool_value() {
        unsafe {
            let mrb = mrb_open();

            let err = Int::try_ruby_convert(mrb, Value::new(mrb_sys_true_value()));
            let expected = Error {
                from: Ruby::Bool,
                to: Rust::SignedInt,
            };
            assert_eq!(err, Err(expected));

            mrb_close(mrb);
        }
    }
}
