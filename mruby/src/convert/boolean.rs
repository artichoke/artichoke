use mruby_sys::*;

use crate::convert::{Error, TryFromMrb};
use crate::value::{Ruby, Rust, Value};

impl TryFromMrb<bool> for Value {
    type From = Rust;
    type To = Ruby;

    unsafe fn try_from_mrb(
        _mrb: *mut mrb_state,
        value: bool,
    ) -> Result<Self, Error<Self::From, Self::To>> {
        if value {
            Ok(Self::new(mrb_sys_true_value()))
        } else {
            Ok(Self::new(mrb_sys_false_value()))
        }
    }
}

impl TryFromMrb<Value> for bool {
    type From = Ruby;
    type To = Rust;

    unsafe fn try_from_mrb(
        _mrb: *mut mrb_state,
        value: Value,
    ) -> Result<Self, Error<Self::From, Self::To>> {
        match value.ruby_type() {
            Ruby::Bool => {
                let inner = value.inner();
                if mrb_sys_value_is_true(inner) {
                    Ok(true)
                } else if mrb_sys_value_is_false(inner) {
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
    use crate::interpreter::*;

    #[quickcheck]
    fn convert_to_bool(b: bool) -> bool {
        unsafe {
            let mrb = Mrb::new().expect("mrb init");
            let value = Value::try_from_mrb(mrb.inner().unwrap(), b).expect("convert");
            value.ruby_type() == Ruby::Bool
        }
    }

    #[quickcheck]
    fn bool_with_value(b: bool) -> bool {
        unsafe {
            let mrb = Mrb::new().expect("mrb init");
            let value = Value::try_from_mrb(mrb.inner().unwrap(), b).expect("convert");
            let inner = value.inner();
            let is_false = mrb_sys_value_is_false(inner);
            let is_true = mrb_sys_value_is_true(inner);
            let is_nil = mrb_sys_value_is_nil(inner);
            if b {
                is_true && !is_nil
            } else {
                is_false && !is_nil
            }
        }
    }

    #[quickcheck]
    fn roundtrip(b: bool) -> bool {
        unsafe {
            let mrb = Mrb::new().expect("mrb init");
            let value = Value::try_from_mrb(mrb.inner().unwrap(), b).expect("convert");
            let value = bool::try_from_mrb(mrb.inner().unwrap(), value).expect("convert");
            value == b
        }
    }

    #[quickcheck]
    fn roundtrip_err(i: i64) -> bool {
        unsafe {
            let mrb = Mrb::new().expect("mrb init");
            let value = Value::try_from_mrb(mrb.inner().unwrap(), i).expect("convert");
            let value = bool::try_from_mrb(mrb.inner().unwrap(), value);
            let expected = Err(Error {
                from: Ruby::Fixnum,
                to: Rust::Bool,
            });
            value == expected
        }
    }
}
