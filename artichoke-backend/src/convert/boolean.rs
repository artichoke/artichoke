use crate::convert::UnboxRubyError;
use crate::core::{Convert, TryConvert};
use crate::exception::Exception;
use crate::sys;
use crate::types::{Ruby, Rust};
use crate::value::Value;
use crate::Artichoke;

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
    type Error = Exception;

    fn try_convert(&self, value: Value) -> Result<bool, Self::Error> {
        if let Ruby::Bool = value.ruby_type() {
            let inner = value.inner();
            if unsafe { sys::mrb_sys_value_is_true(inner) } {
                Ok(true)
            } else if unsafe { sys::mrb_sys_value_is_false(inner) } {
                Ok(false)
            } else {
                // This branch is unreachable because `Ruby::Bool` typed values
                // are guaranteed to be either true or false.
                Err(Exception::from(UnboxRubyError::new(&value, Rust::Bool)))
            }
        } else {
            Err(Exception::from(UnboxRubyError::new(&value, Rust::Bool)))
        }
    }
}

#[cfg(test)]
mod tests {
    use quickcheck_macros::quickcheck;

    use crate::test::prelude::*;

    #[test]
    fn fail_convert() {
        let mut interp = crate::interpreter().unwrap();
        // get a Ruby value that can't be converted to a primitive type.
        let value = interp.eval(b"Object.new").unwrap();
        let result = value.try_into::<bool>(&interp);
        assert!(result.is_err());
    }

    #[quickcheck]
    fn convert_to_bool(b: bool) -> bool {
        let interp = crate::interpreter().unwrap();
        let value = interp.convert(b);
        value.ruby_type() == Ruby::Bool
    }

    #[quickcheck]
    fn bool_with_value(b: bool) -> bool {
        let interp = crate::interpreter().unwrap();
        let value = interp.convert(b);
        let value = value.inner();
        if b {
            if unsafe { sys::mrb_sys_value_is_false(value) } {
                return false;
            }
            if !unsafe { sys::mrb_sys_value_is_true(value) } {
                return false;
            }
            if unsafe { sys::mrb_sys_value_is_nil(value) } {
                return false;
            }
        } else {
            if !unsafe { sys::mrb_sys_value_is_false(value) } {
                return false;
            }
            if unsafe { sys::mrb_sys_value_is_true(value) } {
                return false;
            }
            if unsafe { sys::mrb_sys_value_is_nil(value) } {
                return false;
            }
        }
        true
    }

    #[quickcheck]
    fn roundtrip(b: bool) -> bool {
        let interp = crate::interpreter().unwrap();
        let value = interp.convert(b);
        let value = value.try_into::<bool>(&interp).unwrap();
        value == b
    }

    #[quickcheck]
    fn roundtrip_err(i: i64) -> bool {
        let interp = crate::interpreter().unwrap();
        let value = interp.convert(i);
        let value = value.try_into::<bool>(&interp);
        value.is_err()
    }
}
