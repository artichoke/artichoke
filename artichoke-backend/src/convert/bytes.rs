use std::borrow::Cow;
use std::convert::TryFrom;
use std::ffi::{OsStr, OsString};
use std::slice;

use crate::convert::UnboxRubyError;
use crate::core::{ConvertMut, TryConvertMut};
use crate::exception::Exception;
use crate::ffi;
use crate::sys;
use crate::types::{Ruby, Rust};
use crate::value::Value;
use crate::Artichoke;

impl ConvertMut<Vec<u8>, Value> for Artichoke {
    fn convert_mut(&mut self, value: Vec<u8>) -> Value {
        self.convert_mut(value.as_slice())
    }
}

impl ConvertMut<&[u8], Value> for Artichoke {
    fn convert_mut(&mut self, value: &[u8]) -> Value {
        // Ruby strings contain raw bytes, so we can convert from a &[u8] to a
        // `char *` and `size_t`.
        let raw = value.as_ptr() as *const i8;
        let len = value.len();
        // `mrb_str_new` copies the `char *` to the mruby heap so we do not have
        // to worry about the lifetime of the slice passed into this converter.
        let string = unsafe { self.with_ffi_boundary(|mrb| sys::mrb_str_new(mrb, raw, len)) };
        Value::from(string.unwrap())
    }
}

impl<'a> ConvertMut<Cow<'a, [u8]>, Value> for Artichoke {
    fn convert_mut(&mut self, value: Cow<'a, [u8]>) -> Value {
        match value {
            Cow::Borrowed(bytes) => self.convert_mut(bytes),
            Cow::Owned(bytes) => self.convert_mut(bytes),
        }
    }
}

impl TryConvertMut<OsString, Value> for Artichoke {
    type Error = Exception;

    fn try_convert_mut(&mut self, value: OsString) -> Result<Value, Self::Error> {
        let bytes = ffi::os_str_to_bytes(&*value)?;
        Ok(self.convert_mut(bytes))
    }
}

impl TryConvertMut<&OsStr, Value> for Artichoke {
    type Error = Exception;

    fn try_convert_mut(&mut self, value: &OsStr) -> Result<Value, Self::Error> {
        let bytes = ffi::os_str_to_bytes(value)?;
        Ok(self.convert_mut(bytes))
    }
}

impl<'a> TryConvertMut<Cow<'a, OsStr>, Value> for Artichoke {
    type Error = Exception;

    fn try_convert_mut(&mut self, value: Cow<'a, OsStr>) -> Result<Value, Self::Error> {
        match value {
            Cow::Borrowed(value) => {
                let bytes = ffi::os_str_to_bytes(value)?;
                Ok(self.convert_mut(bytes))
            }
            Cow::Owned(value) => {
                let bytes = ffi::os_string_to_bytes(value)?;
                Ok(self.convert_mut(bytes))
            }
        }
    }
}

impl TryConvertMut<Value, Vec<u8>> for Artichoke {
    type Error = Exception;

    fn try_convert_mut(&mut self, value: Value) -> Result<Vec<u8>, Self::Error> {
        TryConvertMut::<_, &[u8]>::try_convert_mut(self, value).map(<[_]>::to_vec)
    }
}

impl<'a> TryConvertMut<Value, &'a [u8]> for Artichoke {
    type Error = Exception;

    fn try_convert_mut(&mut self, value: Value) -> Result<&'a [u8], Self::Error> {
        if let Ruby::String = value.ruby_type() {
            let bytes = value.inner();
            unsafe {
                self.with_ffi_boundary(|mrb| {
                    let raw = sys::mrb_string_value_ptr(mrb, bytes) as *const u8;
                    let len = sys::mrb_string_value_len(mrb, bytes);
                    if let Ok(len) = usize::try_from(len) {
                        // We can return a borrowed slice because the memory is
                        // stored on the mruby heap. As long as `value` is
                        // reachable, this slice points to valid memory.
                        Ok(slice::from_raw_parts(raw, len))
                    } else {
                        Err(UnboxRubyError::new(&value, Rust::Bytes).into())
                    }
                })?
            }
        } else {
            Err(Exception::from(UnboxRubyError::new(&value, Rust::Bytes)))
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
        let result = value.try_into_mut::<Vec<u8>>(&mut interp);
        assert!(result.is_err());
    }

    #[quickcheck]
    fn convert_to_vec(bytes: Vec<u8>) -> bool {
        let mut interp = crate::interpreter().unwrap();
        let value = interp.convert_mut(bytes);
        value.ruby_type() == Ruby::String
    }

    #[quickcheck]
    fn bytestring(bytes: Vec<u8>) -> bool {
        let mut interp = crate::interpreter().unwrap();
        // Borrowed converter
        let value = interp.convert_mut(bytes.as_slice());
        let len = value.funcall(&mut interp, "length", &[], None).unwrap();
        let len = len.try_into::<usize>(&interp).unwrap();
        if len != bytes.len() {
            return false;
        }
        let empty = value.funcall(&mut interp, "empty?", &[], None).unwrap();
        let empty = empty.try_into::<bool>(&interp).unwrap();
        if empty != bytes.is_empty() {
            return false;
        }
        let zero = interp.convert(0);
        let first = value.funcall(&mut interp, "[]", &[zero], None).unwrap();
        let first = first.try_into_mut::<Option<&[u8]>>(&mut interp).unwrap();
        if first != bytes.get(0..1) {
            return false;
        }
        let recovered: Vec<u8> = interp.try_convert_mut(value).unwrap();
        if recovered != bytes {
            return false;
        }
        // Owned converter
        let value = interp.convert_mut(bytes.to_vec());
        let len = value.funcall(&mut interp, "length", &[], None).unwrap();
        let len = len.try_into::<usize>(&interp).unwrap();
        if len != bytes.len() {
            return false;
        }
        let empty = value.funcall(&mut interp, "empty?", &[], None).unwrap();
        let empty = empty.try_into::<bool>(&interp).unwrap();
        if empty != bytes.is_empty() {
            return false;
        }
        let zero = interp.convert(0);
        let first = value.funcall(&mut interp, "[]", &[zero], None).unwrap();
        let first = first.try_into_mut::<Option<&[u8]>>(&mut interp).unwrap();
        if first != bytes.get(0..1) {
            return false;
        }
        let recovered: Vec<u8> = interp.try_convert_mut(value).unwrap();
        if recovered != bytes {
            return false;
        }
        true
    }

    #[quickcheck]
    fn roundtrip(bytes: Vec<u8>) -> bool {
        let mut interp = crate::interpreter().unwrap();
        let value = interp.convert_mut(bytes.as_slice());
        let value = value.try_into_mut::<Vec<u8>>(&mut interp).unwrap();
        value == bytes
    }

    #[quickcheck]
    fn roundtrip_err(b: bool) -> bool {
        let mut interp = crate::interpreter().unwrap();
        let value = interp.convert(b);
        let value = value.try_into_mut::<Vec<u8>>(&mut interp);
        value.is_err()
    }
}
