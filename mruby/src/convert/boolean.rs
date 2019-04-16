use mruby_sys::*;

use crate::convert::{Error, RubyToRustError, RustToRubyError, TryRuby};
use crate::value::{Ruby, Rust, Value};

impl TryRuby<bool> for Value {
    type RubyConvertError = RustToRubyError;

    fn try_ruby_convert(_mrb: *mut mrb_state, value: bool) -> Result<Self, Self::RubyConvertError> {
        if value {
            Ok(Self::new(unsafe { mrb_sys_true_value() }))
        } else {
            Ok(Self::new(unsafe { mrb_sys_false_value() }))
        }
    }
}

impl TryRuby<Value> for bool {
    type RubyConvertError = RubyToRustError;

    fn try_ruby_convert(
        _mrb: *mut mrb_state,
        value: Value,
    ) -> Result<Self, Self::RubyConvertError> {
        match value.ruby_type() {
            Ruby::Bool => {
                let inner = value.inner();
                if unsafe { mrb_sys_value_is_true(inner) } {
                    Ok(true)
                } else if unsafe { mrb_sys_value_is_false(inner) } {
                    Ok(false)
                } else {
                    // This should be unreachable
                    Err(Error {
                        from: Ruby::Bool,
                        to: Rust::Bool,
                    })
                }
            }
            type_tag => Err(Error {
                from: type_tag,
                to: Rust::Bool,
            }),
        }
    }
}

#[cfg(test)]
mod tests {
    use mruby_sys::*;

    use super::*;

    #[test]
    fn value_from_true_bool() {
        unsafe {
            let mrb = mrb_open();

            let value = Value::try_ruby_convert(mrb, true).expect("convert");
            assert_eq!(value.ruby_type(), Ruby::Bool);
            assert_eq!(mrb_sys_value_is_nil(value.inner()), false);
            assert_eq!(mrb_sys_value_is_false(value.inner()), false);
            assert_eq!(mrb_sys_value_is_true(value.inner()), true);

            mrb_close(mrb);
        }
    }

    #[test]
    fn value_from_false_bool() {
        unsafe {
            let mrb = mrb_open();

            let value = Value::try_ruby_convert(mrb, false).expect("convert");
            assert_eq!(value.ruby_type(), Ruby::Bool);
            assert_eq!(mrb_sys_value_is_nil(value.inner()), false);
            assert_eq!(mrb_sys_value_is_false(value.inner()), true);
            assert_eq!(mrb_sys_value_is_true(value.inner()), false);

            mrb_close(mrb);
        }
    }

    #[test]
    fn bool_from_true_value() {
        unsafe {
            let mrb = mrb_open();

            let value =
                bool::try_ruby_convert(mrb, Value::new(mrb_sys_true_value())).expect("convert");
            assert_eq!(value, true);

            mrb_close(mrb);
        }
    }

    #[test]
    fn bool_from_false_value() {
        unsafe {
            let mrb = mrb_open();

            let value =
                bool::try_ruby_convert(mrb, Value::new(mrb_sys_false_value())).expect("convert");
            assert_eq!(value, false);

            mrb_close(mrb);
        }
    }

    #[test]
    fn err_from_nil_value() {
        unsafe {
            let mrb = mrb_open();

            let err = bool::try_ruby_convert(mrb, Value::new(mrb_sys_nil_value()));
            let expected = Error {
                from: Ruby::Nil,
                to: Rust::Bool,
            };
            assert_eq!(err, Err(expected));

            mrb_close(mrb);
        }
    }

    #[test]
    fn err_from_fixnum_value() {
        unsafe {
            let mrb = mrb_open();

            let err = bool::try_ruby_convert(mrb, Value::new(mrb_sys_fixnum_value(17)));
            let expected = Error {
                from: Ruby::Fixnum,
                to: Rust::Bool,
            };
            assert_eq!(err, Err(expected));

            mrb_close(mrb);
        }
    }
}
