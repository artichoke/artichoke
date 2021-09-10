use std::borrow::Cow;
use std::convert::TryFrom;
use std::ffi::{OsStr, OsString};
use std::slice;

use crate::convert::UnboxRubyError;
use crate::core::{TryConvertMut, Value as _};
use crate::error::Error;
use crate::platform_string::{os_str_to_bytes, os_string_to_bytes};
use crate::sys;
use crate::types::{Ruby, Rust};
use crate::value::Value;
use crate::Artichoke;

impl TryConvertMut<Vec<u8>, Value> for Artichoke {
    type Error = Error;

    fn try_convert_mut(&mut self, value: Vec<u8>) -> Result<Value, Self::Error> {
        self.try_convert_mut(value.as_slice())
    }
}

impl TryConvertMut<&[u8], Value> for Artichoke {
    type Error = Error;

    fn try_convert_mut(&mut self, value: &[u8]) -> Result<Value, Self::Error> {
        // Ruby strings contain raw bytes, so we can convert from a &[u8] to a
        // `char *` and `size_t`.
        let raw = value.as_ptr().cast::<i8>();
        let len = value.len();
        // `mrb_str_new` copies the `char *` to the mruby heap so we do not have
        // to worry about the lifetime of the slice passed into this converter.
        let string = unsafe { self.with_ffi_boundary(|mrb| sys::mrb_str_new(mrb, raw, len))? };
        Ok(string.into())
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

    fn try_convert_mut(&mut self, value: Value) -> Result<Vec<u8>, Self::Error> {
        TryConvertMut::<_, &[u8]>::try_convert_mut(self, value).map(<[_]>::to_vec)
    }
}

impl<'a> TryConvertMut<Value, &'a [u8]> for Artichoke {
    type Error = Error;

    fn try_convert_mut(&mut self, value: Value) -> Result<&'a [u8], Self::Error> {
        if let Ruby::String = value.ruby_type() {
            let bytes = value.inner();
            unsafe {
                self.with_ffi_boundary(|mrb| {
                    let raw = sys::mrb_string_value_ptr(mrb, bytes).cast::<u8>();
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
            Err(UnboxRubyError::new(&value, Rust::Bytes).into())
        }
    }
}

#[cfg(test)]
mod tests {
    use bstr::ByteSlice;
    use quickcheck::quickcheck;

    use crate::test::prelude::*;

    #[test]
    fn fail_convert() {
        let mut interp = interpreter().unwrap();
        // get a Ruby value that can't be converted to a primitive type.
        let value = interp.eval(b"Object.new").unwrap();
        let result = value.try_convert_into_mut::<Vec<u8>>(&mut interp);
        assert!(result.is_err());
    }

    #[test]
    fn convert_with_trailing_nul() {
        let mut interp = interpreter().unwrap();
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
            let mut interp = interpreter().unwrap();
            let value = interp.try_convert_mut(bytes).unwrap();
            value.ruby_type() == Ruby::String
        }

        #[allow(clippy::needless_pass_by_value)]
        fn bytestring_borrowed(bytes: Vec<u8>) -> bool {
            let mut interp = interpreter().unwrap();
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

        #[allow(clippy::needless_pass_by_value)]
        fn bytestring_owned(bytes: Vec<u8>) -> bool {
            let mut interp = interpreter().unwrap();
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

        #[allow(clippy::needless_pass_by_value)]
        fn roundtrip(bytes: Vec<u8>) -> bool {
            let mut interp = interpreter().unwrap();
            let value = interp.try_convert_mut(bytes.as_slice()).unwrap();
            let value = value.try_convert_into_mut::<Vec<u8>>(&mut interp).unwrap();
            value == bytes
        }

        fn roundtrip_err(b: bool) -> bool {
            let mut interp = interpreter().unwrap();
            let value = interp.convert(b);
            let value = value.try_convert_into_mut::<Vec<u8>>(&mut interp);
            value.is_err()
        }
    }
}
