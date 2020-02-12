use std::convert::TryFrom;

use crate::convert::{BoxIntoRubyError, UnboxRubyError};
use crate::exception::Exception;
use crate::sys;
use crate::types::{Int, Ruby, Rust};
use crate::value::Value;
use crate::{Artichoke, Convert, TryConvert};

impl Convert<u8, Value> for Artichoke {
    fn convert(&self, value: u8) -> Value {
        self.convert(Int::from(value))
    }
}

impl Convert<u16, Value> for Artichoke {
    fn convert(&self, value: u16) -> Value {
        self.convert(Int::from(value))
    }
}

#[cfg(not(target_arch = "wasm32"))]
impl Convert<u32, Value> for Artichoke {
    fn convert(&self, value: u32) -> Value {
        self.convert(Int::from(value))
    }
}

#[cfg(target_arch = "wasm32")]
impl TryConvert<u32, Value> for Artichoke {
    type Error = Exception;

    fn try_convert(&self, value: u32) -> Result<Value, Self::Error> {
        let value = Int::try_from(value)
            .map_err(|_| BoxIntoRubyError::new(Rust::UnsignedInt, Ruby::Fixnum))?;
        let fixnum = unsafe { sys::mrb_sys_fixnum_value(value) };
        Ok(Value::new(self, fixnum))
    }
}

impl TryConvert<u64, Value> for Artichoke {
    type Error = Exception;

    fn try_convert(&self, value: u64) -> Result<Value, Self::Error> {
        let value = Int::try_from(value)
            .map_err(|_| BoxIntoRubyError::new(Rust::UnsignedInt, Ruby::Fixnum))?;
        let fixnum = unsafe { sys::mrb_sys_fixnum_value(value) };
        Ok(Value::new(self, fixnum))
    }
}

impl Convert<i8, Value> for Artichoke {
    fn convert(&self, value: i8) -> Value {
        self.convert(Int::from(value))
    }
}

impl Convert<i16, Value> for Artichoke {
    fn convert(&self, value: i16) -> Value {
        self.convert(Int::from(value))
    }
}

impl Convert<i32, Value> for Artichoke {
    fn convert(&self, value: i32) -> Value {
        self.convert(Int::from(value))
    }
}

#[cfg(not(target_arch = "wasm32"))]
impl Convert<i64, Value> for Artichoke {
    fn convert(&self, value: i64) -> Value {
        let fixnum = unsafe { sys::mrb_sys_fixnum_value(value) };
        Value::new(self, fixnum)
    }
}

#[cfg(target_arch = "wasm32")]
impl TryConvert<i64, Value> for Artichoke {
    type Error = Exception;

    fn try_convert(&self, value: i64) -> Result<Value, Self::Error> {
        let value = Int::try_from(value)
            .map_err(|_| BoxIntoRubyError::new(Rust::SignedInt, Ruby::Fixnum))?;
        let fixnum = unsafe { sys::mrb_sys_fixnum_value(value) };
        Ok(Value::new(self, fixnum))
    }
}

impl TryConvert<Value, Int> for Artichoke {
    type Error = Exception;

    fn try_convert(&self, value: Value) -> Result<Int, Self::Error> {
        if let Ruby::Fixnum = value.ruby_type() {
            let value = value.inner();
            Ok(unsafe { sys::mrb_sys_fixnum_to_cint(value) })
        } else {
            Err(Exception::from(UnboxRubyError::new(
                &value,
                Rust::SignedInt,
            )))
        }
    }
}

impl TryConvert<Value, usize> for Artichoke {
    type Error = Exception;

    fn try_convert(&self, value: Value) -> Result<usize, Self::Error> {
        let err = UnboxRubyError::new(&value, Rust::UnsignedInt);
        let int = TryConvert::<_, Int>::try_convert(self, value)?;
        let int = usize::try_from(int).map_err(|_| err)?;
        Ok(int)
    }
}

#[cfg(test)]
mod tests {
    use quickcheck_macros::quickcheck;

    use crate::test::prelude::*;

    #[test]
    fn fail_convert() {
        let mut interp = crate::interpreter().expect("init");
        // get a mrb_value that can't be converted to a primitive type.
        let value = interp.eval(b"Object.new").expect("eval");
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
        let value = Convert::<_, Value>::convert(&interp, 100);
        let value = value.try_into::<usize>();
        let expected = Ok(100);
        assert_eq!(value, expected);
        let value = Convert::<_, Value>::convert(&interp, -100);
        let value = value.try_into::<usize>();
        let expected = Err(ArtichokeError::ConvertToRust {
            from: Ruby::Fixnum,
            to: Rust::UnsignedInt,
        });
        assert_eq!(value, expected);
    }
}
