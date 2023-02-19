use std::borrow::Cow;
use std::str;

use crate::convert::UnboxRubyError;
use crate::core::TryConvertMut;
use crate::error::Error;
use crate::types::Rust;
use crate::value::Value;
use crate::Artichoke;

impl TryConvertMut<String, Value> for Artichoke {
    type Error = Error;

    fn try_convert_mut(&mut self, value: String) -> Result<Value, Self::Error> {
        // Ruby `String`s are just bytes, so get a pointer to the underlying
        // `&[u8]` infallibly and convert that to a `Value`.
        self.try_convert_mut(value.into_bytes())
    }
}

impl TryConvertMut<&str, Value> for Artichoke {
    type Error = Error;

    fn try_convert_mut(&mut self, value: &str) -> Result<Value, Self::Error> {
        // Ruby `String`s are just bytes, so get a pointer to the underlying
        // `&[u8]` infallibly and convert that to a `Value`.
        self.try_convert_mut(value.as_bytes())
    }
}

impl<'a> TryConvertMut<Cow<'a, str>, Value> for Artichoke {
    type Error = Error;

    fn try_convert_mut(&mut self, value: Cow<'a, str>) -> Result<Value, Self::Error> {
        match value {
            Cow::Borrowed(string) => self.try_convert_mut(string),
            Cow::Owned(string) => self.try_convert_mut(string),
        }
    }
}

impl TryConvertMut<Value, String> for Artichoke {
    type Error = Error;

    fn try_convert_mut(&mut self, value: Value) -> Result<String, Self::Error> {
        let bytes = self.try_convert_mut(value)?;
        // This converter requires that the bytes be valid UTF-8 data. If the
        // `Value` contains binary data, use the `Vec<u8>` or `&[u8]` converter.
        let string = String::from_utf8(bytes).map_err(|_| UnboxRubyError::new(&value, Rust::String))?;
        Ok(string)
    }
}

impl<'a> TryConvertMut<Value, &'a str> for Artichoke {
    type Error = Error;

    fn try_convert_mut(&mut self, value: Value) -> Result<&'a str, Self::Error> {
        let bytes = self.try_convert_mut(value)?;
        // This converter requires that the bytes be valid UTF-8 data. If the
        // `Value` contains binary data, use the `Vec<u8>` or `&[u8]` converter.
        let string = str::from_utf8(bytes).map_err(|_| UnboxRubyError::new(&value, Rust::String))?;
        Ok(string)
    }
}

#[cfg(test)]
mod tests {
    use quickcheck::quickcheck;

    use crate::test::prelude::*;

    #[test]
    fn fail_convert() {
        let mut interp = interpreter();
        // get a mrb_value that can't be converted to a primitive type.
        let value = interp.eval(b"Object.new").unwrap();
        let result = value.try_convert_into_mut::<String>(&mut interp);
        assert!(result.is_err());
    }

    quickcheck! {
        fn convert_to_string(s: String) -> bool {
            let mut interp = interpreter();
            let value = interp.try_convert_mut(s.clone()).unwrap();
            let string: Vec<u8> = interp.try_convert_mut(value).unwrap();
            s.as_bytes() == string
        }

        fn string_with_value(s: String) -> bool {
            let mut interp = interpreter();
            let value = interp.try_convert_mut(s.clone()).unwrap();
            value.to_s(&mut interp) == s.as_bytes()
        }

        #[cfg(feature = "core-regexp")]
        fn utf8string_borrowed(string: String) -> bool {
            let mut interp = interpreter();
            // Borrowed converter
            let value = interp.try_convert_mut(string.as_str()).unwrap();
            let len = value
                .funcall(&mut interp, "length", &[], None)
                .and_then(|value| value.try_convert_into::<usize>(&interp))
                .unwrap();
            if len != string.chars().count() {
                return false;
            }
            let zero = interp.convert(0);
            let first = value
                .funcall(&mut interp, "[]", &[zero], None)
                .and_then(|value| value.try_convert_into_mut::<Option<String>>(&mut interp))
                .unwrap();
            let mut iter = string.chars();
            if let Some(ch) = iter.next() {
                if first != Some(ch.to_string()) {
                    return false;
                }
            } else if first.is_some() {
                return false;
            }
            let recovered: String = interp.try_convert_mut(value).unwrap();
            if recovered != string {
                return false;
            }
            true
        }

        #[cfg(feature = "core-regexp")]
        fn utf8string_owned(string: String) -> bool {
            let mut interp = interpreter();
            // Owned converter
            let value = interp.try_convert_mut(string.clone()).unwrap();
            let len = value
                .funcall(&mut interp, "length", &[], None)
                .and_then(|value| value.try_convert_into::<usize>(&interp))
                .unwrap();
            if len != string.chars().count() {
                return false;
            }
            let zero = interp.convert(0);
            let first = value
                .funcall(&mut interp, "[]", &[zero], None)
                .and_then(|value| value.try_convert_into_mut::<Option<String>>(&mut interp))
                .unwrap();
            let mut iter = string.chars();
            if let Some(ch) = iter.next() {
                if first != Some(ch.to_string()) {
                    return false;
                }
            } else if first.is_some() {
                return false;
            }
            let recovered: String = interp.try_convert_mut(value).unwrap();
            if recovered != string {
                return false;
            }
            true
        }

        fn roundtrip(s: String) -> bool {
            let mut interp = interpreter();
            let value = interp.try_convert_mut(s.clone()).unwrap();
            let value = value.try_convert_into_mut::<String>(&mut interp).unwrap();
            value == s
        }

        fn roundtrip_err(b: bool) -> bool {
            let mut interp = interpreter();
            let value = interp.convert(b);
            let result = value.try_convert_into_mut::<String>(&mut interp);
            result.is_err()
        }
    }
}
