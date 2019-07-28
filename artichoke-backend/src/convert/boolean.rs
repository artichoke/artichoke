use crate::convert::{Convert, Error, TryConvert};
use crate::sys;
use crate::value::types::{Ruby, Rust};
use crate::value::Value;
use crate::Artichoke;

impl Convert<bool> for Value {
    type From = Rust;
    type To = Ruby;

    fn convert(interp: &Artichoke, value: bool) -> Self {
        if value {
            Self::new(interp, unsafe { sys::mrb_sys_true_value() })
        } else {
            Self::new(interp, unsafe { sys::mrb_sys_false_value() })
        }
    }
}

impl TryConvert<Value> for bool {
    type From = Ruby;
    type To = Rust;

    unsafe fn try_convert(
        _interp: &Artichoke,
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

    use crate::convert::{Convert, Error, TryConvert};
    use crate::eval::Eval;
    use crate::sys;
    use crate::value::types::{Ruby, Rust};
    use crate::value::Value;

    #[test]
    fn fail_convert() {
        let interp = crate::interpreter().expect("init");
        // get a mrb_value that can't be converted to a primitive type.
        let value = interp.eval("Object.new").expect("eval");
        let expected = Error {
            from: Ruby::Object,
            to: Rust::Bool,
        };
        let result = unsafe { <bool>::try_convert(&interp, value) }.map(|_| ());
        assert_eq!(result, Err(expected));
    }

    #[quickcheck]
    fn convert_to_bool(b: bool) -> bool {
        let interp = crate::interpreter().expect("init");
        let value = Value::convert(&interp, b);
        value.ruby_type() == Ruby::Bool
    }

    #[quickcheck]
    fn bool_with_value(b: bool) -> bool {
        let interp = crate::interpreter().expect("init");
        let value = Value::convert(&interp, b);
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
        let interp = crate::interpreter().expect("init");
        let value = Value::convert(&interp, b);
        let value = unsafe { bool::try_convert(&interp, value) }.expect("convert");
        value == b
    }

    #[quickcheck]
    fn roundtrip_err(i: i64) -> bool {
        let interp = crate::interpreter().expect("init");
        let value = Value::convert(&interp, i);
        let value = unsafe { bool::try_convert(&interp, value) };
        let expected = Err(Error {
            from: Ruby::Fixnum,
            to: Rust::Bool,
        });
        value == expected
    }
}
