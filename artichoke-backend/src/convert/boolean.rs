use crate::convert::{Convert, TryConvert};
use crate::sys;
use crate::types::{Ruby, Rust};
use crate::value::Value;
use crate::{Artichoke, ArtichokeError};

impl Convert<bool, Value> for Artichoke {
    fn convert(&self, value: bool) -> Value {
        if value {
            Value::new(self, unsafe { sys::mrb_sys_true_value() })
        } else {
            Value::new(self, unsafe { sys::mrb_sys_false_value() })
        }
    }
}

impl TryConvert<Value, bool> for Artichoke {
    fn try_convert(&self, value: Value) -> Result<bool, ArtichokeError> {
        match value.ruby_type() {
            Ruby::Bool => {
                let value = value.inner();
                if unsafe { sys::mrb_sys_value_is_true(value) } {
                    Ok(true)
                } else if unsafe { sys::mrb_sys_value_is_false(value) } {
                    Ok(false)
                } else {
                    // This should be unreachable
                    Err(ArtichokeError::ConvertToRust {
                        from: Ruby::Bool,
                        to: Rust::Bool,
                    })
                }
            }
            type_tag => Err(ArtichokeError::ConvertToRust {
                from: type_tag,
                to: Rust::Bool,
            }),
        }
    }
}

#[cfg(test)]
mod tests {
    use artichoke_core::eval::Eval;
    use quickcheck_macros::quickcheck;

    use crate::convert::Convert;
    use crate::sys;
    use crate::types::{Ruby, Rust};
    use crate::value::ValueLike;
    use crate::ArtichokeError;

    #[test]
    fn fail_convert() {
        let interp = crate::interpreter().expect("init");
        // get a mrb_value that can't be converted to a primitive type.
        let value = interp.eval(b"Object.new").expect("eval");
        let expected = Err(ArtichokeError::ConvertToRust {
            from: Ruby::Object,
            to: Rust::Bool,
        });
        let result = value.try_into::<bool>();
        assert_eq!(result, expected);
    }

    #[quickcheck]
    fn convert_to_bool(b: bool) -> bool {
        let interp = crate::interpreter().expect("init");
        let value = interp.convert(b);
        value.ruby_type() == Ruby::Bool
    }

    #[quickcheck]
    fn bool_with_value(b: bool) -> bool {
        let interp = crate::interpreter().expect("init");
        let value = interp.convert(b);
        let value = value.inner();
        if b {
            !unsafe { sys::mrb_sys_value_is_false(value) }
                && unsafe { sys::mrb_sys_value_is_true(value) }
                && !unsafe { sys::mrb_sys_value_is_nil(value) }
        } else {
            !unsafe { sys::mrb_sys_value_is_true(value) }
                && unsafe { sys::mrb_sys_value_is_false(value) }
                && !unsafe { sys::mrb_sys_value_is_nil(value) }
        }
    }

    #[quickcheck]
    fn roundtrip(b: bool) -> bool {
        let interp = crate::interpreter().expect("init");
        let value = interp.convert(b);
        let value = value.try_into::<bool>().expect("convert");
        value == b
    }

    #[quickcheck]
    fn roundtrip_err(i: i64) -> bool {
        let interp = crate::interpreter().expect("init");
        let value = interp.convert(i);
        let value = value.try_into::<bool>();
        let expected = Err(ArtichokeError::ConvertToRust {
            from: Ruby::Fixnum,
            to: Rust::Bool,
        });
        value == expected
    }
}
