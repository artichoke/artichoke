use crate::convert::{Error, TryFromMrb};
use crate::interpreter::Mrb;
use crate::sys;
use crate::value::types::{Ruby, Rust};
use crate::value::Value;

impl TryFromMrb<bool> for Value {
    type From = Rust;
    type To = Ruby;

    unsafe fn try_from_mrb(_mrb: &Mrb, value: bool) -> Result<Self, Error<Self::From, Self::To>> {
        if value {
            Ok(Self::new(sys::mrb_sys_true_value()))
        } else {
            Ok(Self::new(sys::mrb_sys_false_value()))
        }
    }
}

impl TryFromMrb<Value> for bool {
    type From = Ruby;
    type To = Rust;

    unsafe fn try_from_mrb(_mrb: &Mrb, value: Value) -> Result<Self, Error<Self::From, Self::To>> {
        match value.ruby_type() {
            Ruby::Bool => {
                let inner = value.inner();
                if sys::mrb_sys_value_is_true(inner) {
                    Ok(true)
                } else if sys::mrb_sys_value_is_false(inner) {
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
    use quickcheck_macros::quickcheck;

    use crate::convert::*;
    use crate::interpreter::*;
    use crate::sys;
    use crate::value::types::*;
    use crate::value::*;

    #[quickcheck]
    fn convert_to_bool(b: bool) -> bool {
        unsafe {
            let interp = Interpreter::create().expect("mrb init");
            let value = Value::try_from_mrb(&interp, b).expect("convert");
            value.ruby_type() == Ruby::Bool
        }
    }

    #[quickcheck]
    fn bool_with_value(b: bool) -> bool {
        unsafe {
            let interp = Interpreter::create().expect("mrb init");
            let value = Value::try_from_mrb(&interp, b).expect("convert");
            let inner = value.inner();
            let is_false = sys::mrb_sys_value_is_false(inner);
            let is_true = sys::mrb_sys_value_is_true(inner);
            let is_nil = sys::mrb_sys_value_is_nil(inner);
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
            let interp = Interpreter::create().expect("mrb init");
            let value = Value::try_from_mrb(&interp, b).expect("convert");
            let value = bool::try_from_mrb(&interp, value).expect("convert");
            value == b
        }
    }

    #[quickcheck]
    fn roundtrip_err(i: i64) -> bool {
        unsafe {
            let interp = Interpreter::create().expect("mrb init");
            let value = Value::try_from_mrb(&interp, i).expect("convert");
            let value = bool::try_from_mrb(&interp, value);
            let expected = Err(Error {
                from: Ruby::Fixnum,
                to: Rust::Bool,
            });
            value == expected
        }
    }
}
