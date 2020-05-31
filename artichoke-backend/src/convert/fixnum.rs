use std::convert::TryFrom;

use crate::convert::{BoxIntoRubyError, UnboxRubyError};
use crate::core::{Convert, TryConvert};
use crate::exception::Exception;
use crate::sys;
use crate::types::{Int, Ruby, Rust};
use crate::value::Value;
use crate::Artichoke;

impl Convert<u8, Value> for Artichoke {
    #[inline]
    fn convert(&self, value: u8) -> Value {
        self.convert(Int::from(value))
    }
}

impl Convert<u16, Value> for Artichoke {
    #[inline]
    fn convert(&self, value: u16) -> Value {
        self.convert(Int::from(value))
    }
}

impl Convert<u32, Value> for Artichoke {
    #[inline]
    fn convert(&self, value: u32) -> Value {
        self.convert(Int::from(value))
    }
}

impl TryConvert<u64, Value> for Artichoke {
    type Error = Exception;

    fn try_convert(&self, value: u64) -> Result<Value, Self::Error> {
        if let Ok(value) = Int::try_from(value) {
            let fixnum = unsafe { sys::mrb_sys_fixnum_value(value) };
            Ok(Value::from(fixnum))
        } else {
            Err(Exception::from(BoxIntoRubyError::new(
                Rust::UnsignedInt,
                Ruby::Fixnum,
            )))
        }
    }
}

impl Convert<i8, Value> for Artichoke {
    #[inline]
    fn convert(&self, value: i8) -> Value {
        self.convert(Int::from(value))
    }
}

impl Convert<i16, Value> for Artichoke {
    #[inline]
    fn convert(&self, value: i16) -> Value {
        self.convert(Int::from(value))
    }
}

impl Convert<i32, Value> for Artichoke {
    #[inline]
    fn convert(&self, value: i32) -> Value {
        self.convert(Int::from(value))
    }
}

/// Converter for Artichoke native integer type.
///
/// The `Int` type alias must be `i64`.
///
/// ```
/// # use std::any::TypeId;
/// # use std::mem;
/// # use artichoke_backend::types::Int;
/// assert_eq!(mem::size_of::<i64>(), mem::size_of::<Int>());
/// assert_eq!(i64::min_value(), Int::min_value());
/// assert_eq!(i64::max_value(), Int::max_value());
/// assert_eq!(TypeId::of::<i64>(), TypeId::of::<Int>());
/// ```
impl Convert<Int, Value> for Artichoke {
    #[inline]
    fn convert(&self, value: Int) -> Value {
        let fixnum = unsafe { sys::mrb_sys_fixnum_value(value) };
        Value::from(fixnum)
    }
}

impl TryConvert<Value, Int> for Artichoke {
    type Error = Exception;

    fn try_convert(&self, value: Value) -> Result<Int, Self::Error> {
        if let Ruby::Fixnum = value.ruby_type() {
            let inner = value.inner();
            Ok(unsafe { sys::mrb_sys_fixnum_to_cint(inner) })
        } else {
            Err(Exception::from(UnboxRubyError::new(
                &value,
                Rust::SignedInt,
            )))
        }
    }
}

impl TryConvert<Value, u32> for Artichoke {
    type Error = Exception;

    fn try_convert(&self, value: Value) -> Result<u32, Self::Error> {
        if let Ruby::Fixnum = value.ruby_type() {
            let inner = value.inner();
            let num = unsafe { sys::mrb_sys_fixnum_to_cint(inner) };
            let num =
                u32::try_from(num).map_err(|_| UnboxRubyError::new(&value, Rust::UnsignedInt))?;
            Ok(num)
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
        if let Ruby::Fixnum = value.ruby_type() {
            let inner = value.inner();
            let num = unsafe { sys::mrb_sys_fixnum_to_cint(inner) };
            let num =
                usize::try_from(num).map_err(|_| UnboxRubyError::new(&value, Rust::UnsignedInt))?;
            Ok(num)
        } else {
            Err(Exception::from(UnboxRubyError::new(
                &value,
                Rust::SignedInt,
            )))
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
        let result = value.try_into::<Int>(&interp);
        assert!(result.is_err());
    }

    #[quickcheck]
    fn convert_to_fixnum(i: Int) -> bool {
        let interp = crate::interpreter().unwrap();
        let value = interp.convert(i);
        value.ruby_type() == Ruby::Fixnum
    }

    #[quickcheck]
    fn fixnum_with_value(i: Int) -> bool {
        let interp = crate::interpreter().unwrap();
        let value = interp.convert(i);
        let inner = value.inner();
        let cint = unsafe { sys::mrb_sys_fixnum_to_cint(inner) };
        cint == i
    }

    #[quickcheck]
    fn roundtrip(i: Int) -> bool {
        let interp = crate::interpreter().unwrap();
        let value = interp.convert(i);
        let value = value.try_into::<Int>(&interp).unwrap();
        value == i
    }

    #[quickcheck]
    fn roundtrip_err(b: bool) -> bool {
        let interp = crate::interpreter().unwrap();
        let value = interp.convert(b);
        let value = value.try_into::<Int>(&interp);
        value.is_err()
    }

    #[test]
    fn fixnum_to_usize() {
        let interp = crate::interpreter().unwrap();
        let value = Convert::<_, Value>::convert(&interp, 100);
        let value = value.try_into::<usize>(&interp).unwrap();
        assert_eq!(100, value);
        let value = Convert::<_, Value>::convert(&interp, -100);
        let value = value.try_into::<usize>(&interp);
        assert!(value.is_err());
    }
}
