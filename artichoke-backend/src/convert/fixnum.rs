use std::convert::TryFrom;

use crate::convert::{Convert, TryConvert};
use crate::sys;
use crate::types::{Int, Ruby, Rust};
use crate::value::Value;
use crate::{Artichoke, ArtichokeError};

impl Convert<u8, Value> for Artichoke {
    fn convert(&self, value: u8) -> Value {
        let value = Int::from(value);
        Value::new(self, unsafe { sys::mrb_sys_fixnum_value(value) })
    }
}

impl Convert<u16, Value> for Artichoke {
    fn convert(&self, value: u16) -> Value {
        let value = Int::from(value);
        Value::new(self, unsafe { sys::mrb_sys_fixnum_value(value) })
    }
}

#[cfg(not(target_arch = "wasm32"))]
impl Convert<u32, Value> for Artichoke {
    fn convert(&self, value: u32) -> Value {
        let value = Int::from(value);
        Value::new(self, unsafe { sys::mrb_sys_fixnum_value(value) })
    }
}

#[cfg(target_arch = "wasm32")]
impl TryConvert<u32, Value> for Artichoke {
    fn try_convert(&self, value: u32) -> Result<Value, ArtichokeError> {
        let value = Int::try_from(value).map_err(|_| ArtichokeError::ConvertToRuby {
            from: Rust::UnsignedInt,
            to: Ruby::Fixnum,
        })?;
        Ok(Value::new(self, unsafe {
            sys::mrb_sys_fixnum_value(value)
        }))
    }
}

impl TryConvert<u64, Value> for Artichoke {
    fn try_convert(&self, value: u64) -> Result<Value, ArtichokeError> {
        let value = Int::try_from(value).map_err(|_| ArtichokeError::ConvertToRuby {
            from: Rust::UnsignedInt,
            to: Ruby::Fixnum,
        })?;
        Ok(Value::new(self, unsafe {
            sys::mrb_sys_fixnum_value(value)
        }))
    }
}

impl Convert<i8, Value> for Artichoke {
    fn convert(&self, value: i8) -> Value {
        let value = Int::from(value);
        Value::new(self, unsafe { sys::mrb_sys_fixnum_value(value) })
    }
}

impl Convert<i16, Value> for Artichoke {
    fn convert(&self, value: i16) -> Value {
        let value = Int::from(value);
        Value::new(self, unsafe { sys::mrb_sys_fixnum_value(value) })
    }
}

impl Convert<i32, Value> for Artichoke {
    fn convert(&self, value: i32) -> Value {
        let value = Int::from(value);
        Value::new(self, unsafe { sys::mrb_sys_fixnum_value(value) })
    }
}

#[cfg(not(target_arch = "wasm32"))]
impl Convert<i64, Value> for Artichoke {
    fn convert(&self, value: i64) -> Value {
        Value::new(self, unsafe { sys::mrb_sys_fixnum_value(value) })
    }
}

#[cfg(target_arch = "wasm32")]
impl TryConvert<i64, Value> for Artichoke {
    fn try_convert(&self, value: i64) -> Result<Value, ArtichokeError> {
        let value = Int::try_from(value).map_err(|_| ArtichokeError::ConvertToRuby {
            from: Rust::UnsignedInt,
            to: Ruby::Fixnum,
        })?;
        Ok(Value::new(self, unsafe {
            sys::mrb_sys_fixnum_value(value)
        }))
    }
}

impl TryConvert<Value, Int> for Artichoke {
    fn try_convert(&self, value: Value) -> Result<Int, ArtichokeError> {
        match value.ruby_type() {
            Ruby::Fixnum => {
                let value = value.inner();
                Ok(unsafe { sys::mrb_sys_fixnum_to_cint(value) })
            }
            type_tag => Err(ArtichokeError::ConvertToRust {
                from: type_tag,
                to: Rust::SignedInt,
            }),
        }
    }
}

impl TryConvert<Value, usize> for Artichoke {
    fn try_convert(&self, value: Value) -> Result<usize, ArtichokeError> {
        let value: Int = self
            .try_convert(value)
            .map_err(|_| ArtichokeError::ConvertToRust {
                from: Ruby::Fixnum,
                to: Rust::UnsignedInt,
            })?;
        usize::try_from(value).map_err(|_| ArtichokeError::ConvertToRust {
            from: Ruby::Fixnum,
            to: Rust::UnsignedInt,
        })
    }
}

#[cfg(test)]
mod tests {
    use quickcheck_macros::quickcheck;

    use crate::convert::Convert;
    use crate::eval::Eval;
    use crate::sys;
    use crate::types::{Int, Ruby, Rust};
    use crate::value::Value;
    use crate::ArtichokeError;

    #[test]
    fn fail_convert() {
        let interp = crate::interpreter().expect("init");
        // get a mrb_value that can't be converted to a primitive type.
        let value = interp.eval("Object.new").expect("eval");
        let expected = Err(ArtichokeError::ConvertToRust {
            from: Ruby::Object,
            to: Rust::SignedInt,
        });
        let result = value.try_into::<Int>();
        assert_eq!(result, expected);
    }

    #[quickcheck]
    fn convert_to_fixnum(i: Int) -> bool {
        let interp = crate::interpreter().expect("init");
        let value = interp.convert(i);
        value.ruby_type() == Ruby::Fixnum
    }

    #[quickcheck]
    fn fixnum_with_value(i: Int) -> bool {
        let interp = crate::interpreter().expect("init");
        let value = interp.convert(i);
        let inner = value.inner();
        let cint = unsafe { sys::mrb_sys_fixnum_to_cint(inner) };
        cint == i
    }

    #[quickcheck]
    fn roundtrip(i: Int) -> bool {
        let interp = crate::interpreter().expect("init");
        let value = interp.convert(i);
        let value = value.try_into::<Int>().expect("convert");
        value == i
    }

    #[quickcheck]
    fn roundtrip_err(b: bool) -> bool {
        let interp = crate::interpreter().expect("init");
        let value = interp.convert(b);
        let value = value.try_into::<Int>();
        let expected = Err(ArtichokeError::ConvertToRust {
            from: Ruby::Bool,
            to: Rust::SignedInt,
        });
        value == expected
    }

    #[test]
    fn fixnum_to_usize() {
        let interp = crate::interpreter().expect("init");
        let value: Value = interp.convert(100);
        let value = value.try_into::<usize>();
        let expected = Ok(100);
        assert_eq!(value, expected);
        let value: Value = interp.convert(-100);
        let value = value.try_into::<usize>();
        let expected = Err(ArtichokeError::ConvertToRust {
            from: Ruby::Fixnum,
            to: Rust::UnsignedInt,
        });
        assert_eq!(value, expected);
    }
}
