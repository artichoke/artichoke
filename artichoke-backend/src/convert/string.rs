use std::str;

use crate::convert::UnboxRubyError;
use crate::exception::Exception;
use crate::types::Rust;
use crate::value::Value;
use crate::{Artichoke, ConvertMut, TryConvert};

impl ConvertMut<String, Value> for Artichoke {
    fn convert_mut(&mut self, value: String) -> Value {
        // Ruby `String`s are just bytes, so get a pointer to the underlying
        // `&[u8]` infallibly and convert that to a `Value`.
        self.convert_mut(value.as_bytes())
    }
}

impl ConvertMut<&str, Value> for Artichoke {
    fn convert_mut(&mut self, value: &str) -> Value {
        // Ruby `String`s are just bytes, so get a pointer to the underlying
        // `&[u8]` infallibly and convert that to a `Value`.
        self.convert_mut(value.as_bytes())
    }
}

impl TryConvert<Value, String> for Artichoke {
    type Error = Exception;

    fn try_convert(&self, value: Value) -> Result<String, Self::Error> {
        TryConvert::<_, &str>::try_convert(self, value).map(String::from)
    }
}

impl<'a> TryConvert<Value, &'a str> for Artichoke {
    type Error = Exception;

    fn try_convert(&self, value: Value) -> Result<&'a str, Self::Error> {
        let err = UnboxRubyError::new(&value, Rust::String);
        let bytes = self.try_convert(value)?;
        // This converter requires that the bytes be valid UTF-8 data. If the
        // `Value` contains binary data, use the `Vec<u8>` or `&[u8]` converter.
        let string = str::from_utf8(bytes).map_err(|_| err)?;
        Ok(string)
    }
}

#[cfg(test)]
// Convert<String> is implemented in terms of Convert<&str> so only implement
// the tests for String to exercise both code paths.
mod tests {
    use quickcheck_macros::quickcheck;
    use std::convert::TryFrom;

    use crate::test::prelude::*;

    #[test]
    fn fail_convert() {
        let mut interp = crate::interpreter().expect("init");
        // get a mrb_value that can't be converted to a primitive type.
        let value = interp.eval(b"Object.new").expect("eval");
        let result = value.try_into::<String>();
        assert!(result.is_err());
    }

    #[allow(clippy::needless_pass_by_value)]
    #[quickcheck]
    fn convert_to_string(s: String) -> bool {
        let mut interp = crate::interpreter().expect("init");
        let mrb = interp.0.borrow().mrb;
        let value = interp.convert_mut(s.clone());
        let ptr = unsafe { sys::mrb_string_value_ptr(mrb, value.inner()) };
        let len = unsafe { sys::mrb_string_value_len(mrb, value.inner()) };
        let string =
            unsafe { std::slice::from_raw_parts(ptr as *const u8, usize::try_from(len).unwrap()) };
        s.as_bytes() == string
    }

    #[allow(clippy::needless_pass_by_value)]
    #[quickcheck]
    fn string_with_value(s: String) -> bool {
        let mut interp = crate::interpreter().expect("init");
        let value = interp.convert_mut(s.clone());
        value.to_s() == s.as_bytes()
    }

    #[quickcheck]
    fn utf8string(string: String) -> bool {
        let mut interp = crate::interpreter().unwrap();
        // Borrowed converter
        let value = interp.convert_mut(string.as_str());
        let len = value.funcall::<usize>("length", &[], None).unwrap();
        if len != string.chars().count() {
            return false;
        }
        let first = value
            .funcall::<Option<String>>("[]", &[interp.convert(0)], None)
            .unwrap();
        let mut iter = string.chars();
        if let Some(ch) = iter.next() {
            if first != Some(ch.to_string()) {
                return false;
            }
        } else {
            if first.is_some() {
                return false;
            }
        }
        let recovered: String = interp.try_convert(value).unwrap();
        if recovered != string {
            return false;
        }
        // Owned converter
        let value = interp.convert_mut(string.clone());
        let len = value.funcall::<usize>("length", &[], None).unwrap();
        if len != string.chars().count() {
            return false;
        }
        let first = value
            .funcall::<Option<String>>("[]", &[interp.convert(0)], None)
            .unwrap();
        let mut iter = string.chars();
        if let Some(ch) = iter.next() {
            if first != Some(ch.to_string()) {
                return false;
            }
        } else {
            if first.is_some() {
                return false;
            }
        }
        let recovered: String = interp.try_convert(value).unwrap();
        if recovered != string {
            return false;
        }
        true
    }

    #[allow(clippy::needless_pass_by_value)]
    #[quickcheck]
    fn roundtrip(s: String) -> bool {
        let mut interp = crate::interpreter().unwrap();
        let value = interp.convert_mut(s.clone());
        let value = value.try_into::<String>().unwrap();
        value == s
    }

    #[quickcheck]
    fn roundtrip_err(b: bool) -> bool {
        let interp = crate::interpreter().unwrap();
        let value = interp.convert(b);
        let result = value.try_into::<String>();
        result.is_err()
    }

    #[test]
    fn symbol_to_string() {
        let mut interp = crate::interpreter().unwrap();
        let value = interp.eval(b":sym").unwrap();
        let value = value.try_into::<String>().unwrap();
        assert_eq!(&value, "sym");
    }
}
