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
    use quickcheck_macros::quickcheck;

    use super::*;

    #[quickcheck]
    fn convert_to_bool(b: bool) -> bool {
        let mrb = unsafe { mrb_open() };
        let value = Value::try_ruby_convert(mrb, b).expect("convert");
        unsafe { mrb_close(mrb) };
        value.ruby_type() == Ruby::Bool
    }

    #[quickcheck]
    fn bool_with_value(b: bool) -> bool {
        let mrb = unsafe { mrb_open() };
        let value = Value::try_ruby_convert(mrb, b).expect("convert");
        let inner = value.inner();
        let is_false = unsafe { mrb_sys_value_is_false(inner) };
        let is_true = unsafe { mrb_sys_value_is_true(inner) };
        let is_nil = unsafe { mrb_sys_value_is_nil(inner) };
        unsafe { mrb_close(mrb) };
        if b {
            is_true && !is_nil
        } else {
            is_false && !is_nil
        }
    }

    #[quickcheck]
    fn roundtrip(b: bool) -> bool {
        let mrb = unsafe { mrb_open() };
        let value = Value::try_ruby_convert(mrb, b).expect("convert");
        let value = bool::try_ruby_convert(mrb, value).expect("convert");
        unsafe { mrb_close(mrb) };
        value == b
    }

    #[quickcheck]
    fn roundtrip_err(i: i64) -> bool {
        let mrb = unsafe { mrb_open() };
        let value = Value::try_ruby_convert(mrb, i).expect("convert");
        let value = bool::try_ruby_convert(mrb, value);
        unsafe { mrb_close(mrb) };
        let expected = Err(Error {
            from: Ruby::Fixnum,
            to: Rust::Bool,
        });
        value == expected
    }
}
