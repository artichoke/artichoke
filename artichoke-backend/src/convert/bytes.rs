use std::convert::TryFrom;
use std::ffi::{CStr, OsStr, OsString};
use std::slice;

use crate::convert::UnboxRubyError;
use crate::core::{ConvertMut, TryConvert, TryConvertMut};
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
        let mrb = self.0.borrow().mrb;
        // Ruby strings contain raw bytes, so we can convert from a &[u8] to a
        // `char *` and `size_t`.
        let raw = value.as_ptr() as *const i8;
        let len = value.len();
        // `mrb_str_new` copies the `char *` to the mruby heap so we do not have
        // to worry about the lifetime of the slice passed into this converter.
        let string = unsafe { sys::mrb_str_new(mrb, raw, len) };
        Value::new(self, string)
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

impl TryConvert<Value, Vec<u8>> for Artichoke {
    type Error = Exception;

    fn try_convert(&self, value: Value) -> Result<Vec<u8>, Self::Error> {
        TryConvert::<_, &[u8]>::try_convert(self, value).map(<[_]>::to_vec)
    }
}

impl<'a> TryConvert<Value, &'a [u8]> for Artichoke {
    type Error = Exception;

    fn try_convert(&self, value: Value) -> Result<&'a [u8], Self::Error> {
        let mrb = self.0.borrow().mrb;
        match value.ruby_type() {
            Ruby::Symbol => {
                // mruby does not expose an API to get the raw byte contents of a
                // `Symbol`. For non-literal symbols and non-ASCII symbols,
                // `sys::mrb_sys_symbol_name` round trips through a `String`
                // `mrb_value` to turn a `char *` wrapped in quotes into a
                // `char *`.
                let bytes = unsafe { sys::mrb_sys_symbol_name(mrb, value.inner()) };
                let slice = unsafe { CStr::from_ptr(bytes) };
                Ok(slice.to_bytes())
            }
            Ruby::String => {
                let bytes = value.inner();
                let raw = unsafe { sys::mrb_string_value_ptr(mrb, bytes) as *const u8 };
                let len = unsafe { sys::mrb_string_value_len(mrb, bytes) };
                let len =
                    usize::try_from(len).map_err(|_| UnboxRubyError::new(&value, Rust::Bytes))?;
                // We can return a borrowed slice because the memory is stored
                // on the mruby heap. As long as `value` is reachable, this
                // slice points to valid memory.
                let slice = unsafe { slice::from_raw_parts(raw, len) };
                Ok(slice)
            }
            _ => Err(Exception::from(UnboxRubyError::new(&value, Rust::Bytes))),
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
        let result = value.try_into::<Vec<u8>>(&interp);
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
        let len = value
            .funcall::<usize>(&mut interp, "length", &[], None)
            .unwrap();
        if len != bytes.len() {
            return false;
        }
        let empty = value
            .funcall::<bool>(&mut interp, "empty?", &[], None)
            .unwrap();
        if empty != bytes.is_empty() {
            return false;
        }
        let zero = interp.convert(0);
        let first = value
            .funcall::<Option<&[u8]>>(&mut interp, "[]", &[zero], None)
            .unwrap();
        if first != bytes.get(0..1) {
            return false;
        }
        let recovered: Vec<u8> = interp.try_convert(value).unwrap();
        if recovered != bytes {
            return false;
        }
        // Owned converter
        let value = interp.convert_mut(bytes.to_vec());
        let len = value
            .funcall::<usize>(&mut interp, "length", &[], None)
            .unwrap();
        if len != bytes.len() {
            return false;
        }
        let empty = value
            .funcall::<bool>(&mut interp, "empty?", &[], None)
            .unwrap();
        if empty != bytes.is_empty() {
            return false;
        }
        let zero = interp.convert(0);
        let first = value
            .funcall::<Option<&[u8]>>(&mut interp, "[]", &[zero], None)
            .unwrap();
        if first != bytes.get(0..1) {
            return false;
        }
        let recovered: Vec<u8> = interp.try_convert(value).unwrap();
        if recovered != bytes {
            return false;
        }
        true
    }

    #[quickcheck]
    fn roundtrip(bytes: Vec<u8>) -> bool {
        let mut interp = crate::interpreter().unwrap();
        let value = interp.convert_mut(bytes.as_slice());
        let value = value.try_into::<Vec<u8>>(&interp).unwrap();
        value == bytes
    }

    #[quickcheck]
    fn roundtrip_err(b: bool) -> bool {
        let interp = crate::interpreter().unwrap();
        let value = interp.convert(b);
        let value = value.try_into::<Vec<u8>>(&interp);
        value.is_err()
    }
}
