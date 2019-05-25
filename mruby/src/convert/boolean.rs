use crate::convert::{Error, FromMrb, TryFromMrb};
use crate::interpreter::Mrb;
use crate::sys;
use crate::value::types::{Ruby, Rust};
use crate::value::Value;

impl FromMrb<bool> for Value {
    type From = Rust;
    type To = Ruby;

    fn from_mrb(interp: &Mrb, value: bool) -> Self {
        if value {
            Self::new(interp, unsafe { sys::mrb_sys_true_value() })
        } else {
            Self::new(interp, unsafe { sys::mrb_sys_false_value() })
        }
    }
}

impl TryFromMrb<Value> for bool {
    type From = Ruby;
    type To = Rust;

    unsafe fn try_from_mrb(
        _interp: &Mrb,
        value: Value,
    ) -> Result<Self, Error<Self::From, Self::To>> {
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

    use crate::convert::{Error, FromMrb, TryFromMrb};
    use crate::eval::MrbEval;
    use crate::interpreter::Interpreter;
    use crate::sys;
    use crate::value::types::{Ruby, Rust};
    use crate::value::Value;

    #[test]
    fn fail_convert() {
        let interp = Interpreter::create().expect("mrb init");
        // get a mrb_value that can't be converted to a primitive type.
        let value = interp.eval("Object.new").expect("eval");
        let expected = Error {
            from: Ruby::Object,
            to: Rust::Bool,
        };
        let result = unsafe { <bool>::try_from_mrb(&interp, value) }.map(|_| ());
        assert_eq!(result, Err(expected));
    }

    #[quickcheck]
    fn convert_to_bool(b: bool) -> bool {
        let interp = Interpreter::create().expect("mrb init");
        let value = Value::from_mrb(&interp, b);
        value.ruby_type() == Ruby::Bool
    }

    #[quickcheck]
    fn bool_with_value(b: bool) -> bool {
        let interp = Interpreter::create().expect("mrb init");
        let value = Value::from_mrb(&interp, b);
        let inner = value.inner();
        let is_false = unsafe { sys::mrb_sys_value_is_false(inner) };
        let is_true = unsafe { sys::mrb_sys_value_is_true(inner) };
        let is_nil = unsafe { sys::mrb_sys_value_is_nil(inner) };
        if b {
            is_true && !is_nil
        } else {
            is_false && !is_nil
        }
    }

    #[quickcheck]
    fn roundtrip(b: bool) -> bool {
        let interp = Interpreter::create().expect("mrb init");
        let value = Value::from_mrb(&interp, b);
        let value = unsafe { bool::try_from_mrb(&interp, value) }.expect("convert");
        value == b
    }

    #[quickcheck]
    fn roundtrip_err(i: i64) -> bool {
        let interp = Interpreter::create().expect("mrb init");
        let value = Value::from_mrb(&interp, i);
        let value = unsafe { bool::try_from_mrb(&interp, value) };
        let expected = Err(Error {
            from: Ruby::Fixnum,
            to: Rust::Bool,
        });
        value == expected
    }
}
