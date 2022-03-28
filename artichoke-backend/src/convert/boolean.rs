use crate::convert::UnboxRubyError;
use crate::core::{Convert, TryConvert, Value as _};
use crate::error::Error;
use crate::sys;
use crate::types::{Ruby, Rust};
use crate::value::Value;
use crate::Artichoke;

impl Convert<bool, Value> for Artichoke {
    fn convert(&self, value: bool) -> Value {
        // Safety
        //
        // Boolean Ruby Values do not need to be protected because they are
        // immediates and do not live on the mruby heap.
        if value {
            Value::from(unsafe { sys::mrb_sys_true_value() })
        } else {
            Value::from(unsafe { sys::mrb_sys_false_value() })
        }
    }
}

impl Convert<Option<bool>, Value> for Artichoke {
    fn convert(&self, value: Option<bool>) -> Value {
        if let Some(value) = value {
            self.convert(value)
        } else {
            Value::nil()
        }
    }
}

impl TryConvert<Value, bool> for Artichoke {
    type Error = Error;

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
                Err(UnboxRubyError::new(&value, Rust::Bool).into())
            }
        } else {
            Err(UnboxRubyError::new(&value, Rust::Bool).into())
        }
    }
}

impl TryConvert<Value, Option<bool>> for Artichoke {
    type Error = Error;

    fn try_convert(&self, value: Value) -> Result<Option<bool>, Self::Error> {
        if value.is_nil() {
            Ok(None)
        } else {
            Ok(Some(self.try_convert(value)?))
        }
    }
}

#[cfg(test)]
mod tests {
    use quickcheck::quickcheck;

    use crate::test::prelude::*;

    #[test]
    fn fail_convert() {
        let mut interp = interpreter();
        // get a Ruby value that can't be converted to a primitive type.
        let value = interp.eval(b"Object.new").unwrap();
        let result = value.try_convert_into::<bool>(&interp);
        assert!(result.is_err());
    }

    quickcheck! {
        fn convert_to_bool(b: bool) -> bool {
            let interp = interpreter();
            let value = interp.convert(b);
            value.ruby_type() == Ruby::Bool
        }

        fn convert_to_nilable_bool(b: Option<bool>) -> bool {
            let interp = interpreter();
            let value = interp.convert(b);
            if b.is_some() {
                value.ruby_type() == Ruby::Bool
            } else {
                value.ruby_type() == Ruby::Nil
            }
        }

        fn bool_with_value(b: bool) -> bool {
            let interp = interpreter();
            let value = interp.convert(b);
            let value = value.inner();
            if b {
                if unsafe { sys::mrb_sys_value_is_false(value) } {
                    return false;
                }
                if !unsafe { sys::mrb_sys_value_is_true(value) } {
                    return false;
                }
            } else {
                if !unsafe { sys::mrb_sys_value_is_false(value) } {
                    return false;
                }
                if unsafe { sys::mrb_sys_value_is_true(value) } {
                    return false;
                }
            }
            if unsafe { sys::mrb_sys_value_is_nil(value) } {
                return false;
            }
            true
        }

        fn nilable_bool_with_value(b: Option<bool>) -> bool {
            let interp = interpreter();
            let value = interp.convert(b);
            let value = value.inner();
            match b {
                Some(true) => {
                    if unsafe { sys::mrb_sys_value_is_false(value) } {
                        return false;
                    }
                    if !unsafe { sys::mrb_sys_value_is_true(value) } {
                        return false;
                    }
                    if unsafe { sys::mrb_sys_value_is_nil(value) } {
                        return false;
                    }
                }
                Some(false) => {
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
                None => {
                    if unsafe { sys::mrb_sys_value_is_false(value) } {
                        return false;
                    }
                    if unsafe { sys::mrb_sys_value_is_true(value) } {
                        return false;
                    }
                    if !unsafe { sys::mrb_sys_value_is_nil(value) } {
                        return false;
                    }
                }
            }
            true
        }

        fn roundtrip(b: bool) -> bool {
            let interp = interpreter();
            let value = interp.convert(b);
            let value = value.try_convert_into::<bool>(&interp).unwrap();
            value == b
        }

        fn nilable_roundtrip(b: Option<bool>) -> bool {
            let interp = interpreter();
            let value = interp.convert(b);
            let value = value.try_convert_into::<Option<bool>>(&interp).unwrap();
            value == b
        }

        fn roundtrip_err(i: i64) -> bool {
            let interp = interpreter();
            let value = interp.convert(i);
            let value = value.try_convert_into::<bool>(&interp);
            value.is_err()
        }
    }
}
