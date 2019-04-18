use mruby_sys::*;
use std::ffi::{CStr, CString};

use crate::convert::{Error, TryFromMrb};
use crate::value::{Ruby, Rust, Value};

// TODO: document encoding assumptions - can only convert UTF-8 data to a Rust
// String.
//
// TODO: Document danger associated with lifetimes.
// If the mrb_value lives longer than the `String` or `&str` the mrb_value may
// point to garbage.

impl TryFromMrb<String> for Value {
    type From = Rust;
    type To = Ruby;

    unsafe fn try_from_mrb(
        mrb: *mut mrb_state,
        value: String,
    ) -> Result<Self, Error<Self::From, Self::To>> {
        Self::try_from_mrb(mrb, value.as_str())
    }
}
impl TryFromMrb<&str> for Value {
    type From = Rust;
    type To = Ruby;

    unsafe fn try_from_mrb(
        mrb: *mut mrb_state,
        value: &str,
    ) -> Result<Self, Error<Self::From, Self::To>> {
        // mruby has the API `mrb_str_new` which takes a char* and size_t but
        // Rust `CString` does not support &str that contain NUL interior bytes.
        // To create a Ruby String that has NULs, use `TryFromMrb<&[u8]>` or
        // `TryFromMrb<Vec<u8>>`.
        match CString::new(value) {
            Ok(cstr) => {
                let ptr = cstr.as_ptr();
                Ok(Self::new(mrb_str_new_cstr(mrb, ptr)))
            }
            Err(_) => Err(Error {
                from: Rust::String,
                to: Ruby::String,
            }),
        }
    }
}

impl TryFromMrb<Value> for String {
    type From = Ruby;
    type To = Rust;

    unsafe fn try_from_mrb(
        mrb: *mut mrb_state,
        value: Value,
    ) -> Result<Self, Error<Self::From, Self::To>> {
        match value.ruby_type() {
            Ruby::String => {
                let mut value = value.inner();
                let cstr = mrb_string_value_cstr(mrb, &mut value);
                match CStr::from_ptr(cstr).to_str() {
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
            unsafe {
                let mrb = mrb_open();
                let value = Value::try_from_mrb(mrb, s.clone());
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
                mrb_close(mrb);
                good
            }
        }

        #[allow(clippy::needless_pass_by_value)]
        #[quickcheck]
        fn string_with_value(s: String) -> bool {
            unsafe {
                let mrb = mrb_open();
                let value = Value::try_from_mrb(mrb, s.clone());
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
                mrb_close(mrb);
                good
            }
        }

        #[allow(clippy::needless_pass_by_value)]
        #[quickcheck]
        fn roundtrip(s: String) -> bool {
            unsafe {
                let mrb = mrb_open();
                let value = Value::try_from_mrb(mrb, s.clone());
                let good = match value {
                    Ok(value) => {
                        let value = String::try_from_mrb(mrb, value).expect("convert");
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
                mrb_close(mrb);
                good
            }
        }

        #[quickcheck]
        fn roundtrip_err(b: bool) -> bool {
            unsafe {
                let mrb = mrb_open();
                let value = Value::try_from_mrb(mrb, b).expect("convert");
                let value = String::try_from_mrb(mrb, value);
                mrb_close(mrb);
                let expected = Err(Error {
                    from: Ruby::Bool,
                    to: Rust::String,
                });
                value == expected
            }
        }
    }

    mod str {
        use super::*;

        #[allow(clippy::needless_pass_by_value)]
        #[quickcheck]
        fn convert_to_str(s: String) -> bool {
            unsafe {
                let s = s.as_str();
                let mrb = mrb_open();
                let value = Value::try_from_mrb(mrb, s);
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
                mrb_close(mrb);
                good
            }
        }

        #[allow(clippy::needless_pass_by_value)]
        #[quickcheck]
        fn str_with_value(s: String) -> bool {
            unsafe {
                let s = s.as_str();
                let mrb = mrb_open();
                let value = Value::try_from_mrb(mrb, s);
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
                mrb_close(mrb);
                good
            }
        }
    }
}
