use mruby_sys::*;
use std::ffi::{CStr, CString};

use crate::convert::{Error, RubyToRustError, RustToRubyError, TryRuby};
use crate::value::{Ruby, Rust, Value};

impl TryRuby<String> for Value {
    type RubyConvertError = RustToRubyError;

    fn try_ruby_convert(
        mrb: *mut mrb_state,
        value: String,
    ) -> Result<Self, Self::RubyConvertError> {
        let value: &str = &value;
        Self::try_ruby_convert(mrb, value)
    }
}
impl TryRuby<&str> for Value {
    type RubyConvertError = RustToRubyError;

    fn try_ruby_convert(mrb: *mut mrb_state, value: &str) -> Result<Self, Self::RubyConvertError> {
        // mruby has the API `mrb_str_new` which takes a char* and size_t but
        // Rust `CString` does not support &str that contain NUL interior bytes.
        match CString::new(value) {
            Ok(cstr) => Ok(Self::new(unsafe { mrb_str_new_cstr(mrb, cstr.as_ptr()) })),
            Err(_) => Err(Error {
                from: Rust::String,
                to: Ruby::String,
            }),
        }
    }
}

impl TryRuby<Value> for String {
    type RubyConvertError = RubyToRustError;

    fn try_ruby_convert(mrb: *mut mrb_state, value: Value) -> Result<Self, Self::RubyConvertError> {
        match value.ruby_type() {
            Ruby::String => {
                let mut value = value.inner();
                let cstr = unsafe { mrb_string_value_cstr(mrb, &mut value) };
                match unsafe { CStr::from_ptr(cstr) }.to_str() {
                    Ok(string) => Ok(string.to_owned()),
                    Err(_) => Err(Error {
                        from: Ruby::String,
                        to: Rust::String,
                    }),
                }
            }
            type_tag => Err(Error {
                from: type_tag,
                to: Rust::String,
            }),
        }
    }
}

#[cfg(test)]
mod tests {
    use mruby_sys::*;
    use quickcheck_macros::quickcheck;

    use super::*;

    mod string {
        use super::*;

        #[allow(clippy::needless_pass_by_value)]
        #[quickcheck]
        fn convert_to_string(s: String) -> bool {
            let mrb = unsafe { mrb_open() };
            let value = Value::try_ruby_convert(mrb, s.clone());
            let good = match value {
                Ok(value) => value.ruby_type() == Ruby::String,
                Err(err) => {
                    let expected = Error {
                        from: Rust::String,
                        to: Ruby::String,
                    };
                    s.contains('\u{0}') && err == expected
                }
            };
            unsafe { mrb_close(mrb) };
            good
        }

        #[allow(clippy::needless_pass_by_value)]
        #[quickcheck]
        fn string_with_value(s: String) -> bool {
            let mrb = unsafe { mrb_open() };
            let value = Value::try_ruby_convert(mrb, s.clone());
            let good = match value {
                Ok(value) => {
                    let to_s = value.to_s(mrb);
                    to_s == s
                }
                Err(err) => {
                    let expected = Error {
                        from: Rust::String,
                        to: Ruby::String,
                    };
                    s.contains('\u{0}') && err == expected
                }
            };
            unsafe { mrb_close(mrb) };
            good
        }

        #[allow(clippy::needless_pass_by_value)]
        #[quickcheck]
        fn roundtrip(s: String) -> bool {
            let mrb = unsafe { mrb_open() };
            let value = Value::try_ruby_convert(mrb, s.clone());
            let good = match value {
                Ok(value) => {
                    let value = String::try_ruby_convert(mrb, value).expect("convert");
                    value == s
                }
                Err(err) => {
                    let expected = Error {
                        from: Rust::String,
                        to: Ruby::String,
                    };
                    s.contains('\u{0}') && err == expected
                }
            };
            unsafe { mrb_close(mrb) };
            good
        }

        #[quickcheck]
        fn roundtrip_err(b: bool) -> bool {
            let mrb = unsafe { mrb_open() };
            let value = Value::try_ruby_convert(mrb, b).expect("convert");
            let value = String::try_ruby_convert(mrb, value);
            unsafe { mrb_close(mrb) };
            let expected = Err(Error {
                from: Ruby::Bool,
                to: Rust::String,
            });
            value == expected
        }
    }

    mod str {
        use super::*;

        #[allow(clippy::needless_pass_by_value)]
        #[quickcheck]
        fn convert_to_str(s: String) -> bool {
            let s: &str = &s;
            let mrb = unsafe { mrb_open() };
            let value = Value::try_ruby_convert(mrb, s);
            let good = match value {
                Ok(value) => value.ruby_type() == Ruby::String,
                Err(err) => {
                    let expected = Error {
                        from: Rust::String,
                        to: Ruby::String,
                    };
                    s.contains('\u{0}') && err == expected
                }
            };
            unsafe { mrb_close(mrb) };
            good
        }

        #[allow(clippy::needless_pass_by_value)]
        #[quickcheck]
        fn str_with_value(s: String) -> bool {
            let s: &str = &s;
            let mrb = unsafe { mrb_open() };
            let value = Value::try_ruby_convert(mrb, s);
            let good = match value {
                Ok(value) => {
                    let to_s = value.to_s(mrb);
                    to_s == s
                }
                Err(err) => {
                    let expected = Error {
                        from: Rust::String,
                        to: Ruby::String,
                    };
                    s.contains('\u{0}') && err == expected
                }
            };
            unsafe { mrb_close(mrb) };
            good
        }
    }
}
