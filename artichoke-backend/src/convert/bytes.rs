use core::mem;
use std::borrow::Cow;
use std::ffi::{OsStr, OsString};

use spinoso_string::String;

use crate::convert::BoxUnboxVmValue;
use crate::core::TryConvertMut;
use crate::error::Error;
use crate::platform_string::{os_str_to_bytes, os_string_to_bytes};
use crate::value::Value;
use crate::Artichoke;

impl TryConvertMut<Vec<u8>, Value> for Artichoke {
    type Error = Error;

    fn try_convert_mut(&mut self, value: Vec<u8>) -> Result<Value, Self::Error> {
        let s = String::utf8(value);
        let value = String::alloc_value(s, self)?;
        Ok(self.protect(value))
    }
}

impl TryConvertMut<&[u8], Value> for Artichoke {
    type Error = Error;

    fn try_convert_mut(&mut self, value: &[u8]) -> Result<Value, Self::Error> {
        self.try_convert_mut(value.to_vec())
    }
}

impl<'a> TryConvertMut<Cow<'a, [u8]>, Value> for Artichoke {
    type Error = Error;

    fn try_convert_mut(&mut self, value: Cow<'a, [u8]>) -> Result<Value, Self::Error> {
        match value {
            Cow::Borrowed(bytes) => self.try_convert_mut(bytes),
            Cow::Owned(bytes) => self.try_convert_mut(bytes),
        }
    }
}

impl TryConvertMut<OsString, Value> for Artichoke {
    type Error = Error;

    fn try_convert_mut(&mut self, value: OsString) -> Result<Value, Self::Error> {
        let bytes = os_string_to_bytes(value)?;
        self.try_convert_mut(bytes)
    }
}

impl TryConvertMut<&OsStr, Value> for Artichoke {
    type Error = Error;

    fn try_convert_mut(&mut self, value: &OsStr) -> Result<Value, Self::Error> {
        let bytes = os_str_to_bytes(value)?;
        self.try_convert_mut(bytes)
    }
}

impl<'a> TryConvertMut<Cow<'a, OsStr>, Value> for Artichoke {
    type Error = Error;

    fn try_convert_mut(&mut self, value: Cow<'a, OsStr>) -> Result<Value, Self::Error> {
        match value {
            Cow::Borrowed(value) => {
                let bytes = os_str_to_bytes(value)?;
                self.try_convert_mut(bytes)
            }
            Cow::Owned(value) => {
                let bytes = os_string_to_bytes(value)?;
                self.try_convert_mut(bytes)
            }
        }
    }
}

impl TryConvertMut<Value, Vec<u8>> for Artichoke {
    type Error = Error;

    fn try_convert_mut(&mut self, mut value: Value) -> Result<Vec<u8>, Self::Error> {
        let s = unsafe { String::unbox_from_value(&mut value, self)? };
        Ok(s.clone().into_vec())
    }
}

impl<'a> TryConvertMut<Value, &'a [u8]> for Artichoke {
    type Error = Error;

    fn try_convert_mut(&mut self, mut value: Value) -> Result<&'a [u8], Self::Error> {
        self.protect(value);
        let s = unsafe { String::unbox_from_value(&mut value, self)? };
        // Safety
        //
        // This transmute modifies the lifetime of the byte slice pulled out of
        // the boxed `String`. This is only safe if there are no garbage
        // collections that reclaim `value`, which is enforced for at least this
        // entry from an mruby trampoline by the call to `protect` above.
        let slice = unsafe { mem::transmute(s.as_slice()) };
        Ok(slice)
    }
}

#[cfg(test)]
mod tests {
    use bstr::ByteSlice;
    use quickcheck::quickcheck;

    use crate::test::prelude::*;

    #[test]
    fn fail_convert() {
        let mut interp = interpreter();
        // get a Ruby value that can't be converted to a primitive type.
        let value = interp.eval(b"Object.new").unwrap();
        let result = value.try_convert_into_mut::<Vec<u8>>(&mut interp);
        assert!(result.is_err());
    }

    #[test]
    fn convert_with_trailing_nul() {
        let mut interp = interpreter();
        let bytes: &[u8] = &[0];
        let value = interp.try_convert_mut(bytes).unwrap();
        let retrieved_bytes = value.try_convert_into_mut::<&[u8]>(&mut interp).unwrap();
        assert_eq!(bytes.as_bstr(), retrieved_bytes.as_bstr());

        let len = value.funcall(&mut interp, "bytesize", &[], None).unwrap();
        let len = len.try_convert_into::<usize>(&interp).unwrap();
        assert_eq!(len, 1);

        let empty = value.funcall(&mut interp, "empty?", &[], None).unwrap();
        let empty = empty.try_convert_into::<bool>(&interp).unwrap();
        assert!(!empty);

        let zero = interp.convert(0);
        let one = interp.convert(1);

        let str_bytes = value.funcall(&mut interp, "bytes", &[], None).unwrap();
        let first = str_bytes.funcall(&mut interp, "[]", &[zero], None).unwrap();
        let first = first.try_convert_into::<i64>(&interp).unwrap();
        assert_eq!(first, 0_i64);

        let slice = value.funcall(&mut interp, "byteslice", &[zero, one], None).unwrap();
        let slice = slice.try_convert_into_mut::<Option<&[u8]>>(&mut interp).unwrap();
        let expected: Option<&[u8]> = Some(&[0]);
        assert_eq!(slice, expected);
    }

    quickcheck! {
        fn convert_to_vec(bytes: Vec<u8>) -> bool {
            let mut interp = interpreter();
            let value = interp.try_convert_mut(bytes).unwrap();
            value.ruby_type() == Ruby::String
        }

        fn byte_string_borrowed(bytes: Vec<u8>) -> bool {
            let mut interp = interpreter();
            // Borrowed converter
            let value = interp.try_convert_mut(bytes.as_slice()).unwrap();
            let len = value.funcall(&mut interp, "bytesize", &[], None).unwrap();
            let len = len.try_convert_into::<usize>(&interp).unwrap();
            if len != bytes.len() {
                return false;
            }

            let empty = value.funcall(&mut interp, "empty?", &[], None).unwrap();
            let empty = empty.try_convert_into::<bool>(&interp).unwrap();
            if empty != bytes.is_empty() {
                return false;
            }

            let zero = interp.convert(0);
            let one = interp.convert(1);

            let str_bytes = value.funcall(&mut interp, "bytes", &[], None).unwrap();
            let first = str_bytes.funcall(&mut interp, "[]", &[zero], None).unwrap();
            let first = first.try_convert_into::<Option<i64>>(&interp).unwrap();
            if first != bytes.get(0).copied().map(i64::from) {
                return false;
            }

            let slice = value.funcall(&mut interp, "byteslice", &[zero, one], None).unwrap();
            let slice = slice.try_convert_into_mut::<Option<&[u8]>>(&mut interp).unwrap();
            if slice.unwrap_or_default() != bytes.get(0..1).unwrap_or_default() {
                return false;
            }

            let recovered: Vec<u8> = interp.try_convert_mut(value).unwrap();
            if recovered != bytes {
                return false;
            }
            true
        }

        fn byte_string_owned(bytes: Vec<u8>) -> bool {
            let mut interp = interpreter();
            // Owned converter
            let value = interp.try_convert_mut(bytes.clone()).unwrap();
            let len = value.funcall(&mut interp, "bytesize", &[], None).unwrap();
            let len = len.try_convert_into::<usize>(&interp).unwrap();
            if len != bytes.len() {
                return false;
            }

            let empty = value.funcall(&mut interp, "empty?", &[], None).unwrap();
            let empty = empty.try_convert_into::<bool>(&interp).unwrap();
            if empty != bytes.is_empty() {
                return false;
            }

            let zero = interp.convert(0);
            let one = interp.convert(1);

            let str_bytes = value.funcall(&mut interp, "bytes", &[], None).unwrap();
            let first = str_bytes.funcall(&mut interp, "[]", &[zero], None).unwrap();
            let first = first.try_convert_into::<Option<i64>>(&interp).unwrap();
            if first != bytes.get(0).copied().map(i64::from) {
                return false;
            }

            let slice = value.funcall(&mut interp, "byteslice", &[zero, one], None).unwrap();
            let slice = slice.try_convert_into_mut::<Option<&[u8]>>(&mut interp).unwrap();
            if slice.unwrap_or_default() != bytes.get(0..1).unwrap_or_default() {
                return false;
            }

            let recovered: Vec<u8> = interp.try_convert_mut(value).unwrap();
            if recovered != bytes {
                return false;
            }
            true
        }

        fn roundtrip(bytes: Vec<u8>) -> bool {
            let mut interp = interpreter();
            let value = interp.try_convert_mut(bytes.as_slice()).unwrap();
            let value = value.try_convert_into_mut::<Vec<u8>>(&mut interp).unwrap();
            value == bytes
        }

        fn roundtrip_err(b: bool) -> bool {
            let mut interp = interpreter();
            let value = interp.convert(b);
            let value = value.try_convert_into_mut::<Vec<u8>>(&mut interp);
            value.is_err()
        }
    }
}
