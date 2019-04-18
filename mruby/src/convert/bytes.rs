use mruby_sys::*;
use std::convert::TryFrom;

use crate::convert::{Error, TryFromMrb};
use crate::value::{Ruby, Rust, Value};

// TODO: Document danger associated with lifetimes.
// If the mrb_value lives longer than the `Vec<u8>` or `&[u8]` the mrb_value
// may point to garbage.

impl TryFromMrb<Vec<u8>> for Value {
    type From = Rust;
    type To = Ruby;

    unsafe fn try_from_mrb(
        mrb: *mut mrb_state,
        value: Vec<u8>,
    ) -> Result<Self, Error<Self::From, Self::To>> {
        Self::try_from_mrb(mrb, value.as_slice())
    }
}
impl TryFromMrb<&[u8]> for Value {
    type From = Rust;
    type To = Ruby;

    unsafe fn try_from_mrb(
        mrb: *mut mrb_state,
        value: &[u8],
    ) -> Result<Self, Error<Self::From, Self::To>> {
        // mruby strings contain raw bytes, so we can convert from a &[u8] to a
        // `char *` and `size_t`.
        let raw = value.as_ptr() as *const i8;
        let len = value.len();
        Ok(Self::new(mrb_str_new(mrb, raw, len)))
    }
}

impl TryFromMrb<Value> for Vec<u8> {
    type From = Ruby;
    type To = Rust;

    unsafe fn try_from_mrb(
        mrb: *mut mrb_state,
        value: Value,
    ) -> Result<Self, Error<Self::From, Self::To>> {
        match value.ruby_type() {
            Ruby::String => {
                let value = value.inner();
                let raw = mrb_string_value_ptr(mrb, value) as *const u8;
                let len = mrb_string_value_len(mrb, value);
                let len = usize::try_from(len).map_err(|_| Error {
                    from: Ruby::String,
                    to: Rust::Bytes,
                })?;
                let slice = std::slice::from_raw_parts(raw, len);
                Ok(slice.to_vec())
            }
            type_tag => Err(Error {
                from: type_tag,
                to: Rust::Bytes,
            }),
        }
    }
}

#[cfg(test)]
mod tests {
    use mruby_sys::*;
    use quickcheck_macros::quickcheck;

    use super::*;

    mod vec {
        use super::*;

        #[allow(clippy::needless_pass_by_value)]
        #[quickcheck]
        fn convert_to_vec(v: Vec<u8>) -> bool {
            unsafe {
                let mrb = mrb_open();
                let value = Value::try_from_mrb(mrb, v.clone()).expect("convert");
                let good = value.ruby_type() == Ruby::String;
                mrb_close(mrb);
                good
            }
        }

        #[allow(clippy::needless_pass_by_value)]
        #[quickcheck]
        fn vec_with_value(v: Vec<u8>) -> bool {
            unsafe {
                let mrb = mrb_open();
                let value = Value::try_from_mrb(mrb, v.clone()).expect("convert");
                let inner = value.inner();
                let len = mrb_string_value_len(mrb, inner);
                let len = usize::try_from(len).expect("usize");
                let good = v.len() == len;
                mrb_close(mrb);
                good
            }
        }

        #[allow(clippy::needless_pass_by_value)]
        #[quickcheck]
        fn roundtrip(v: Vec<u8>) -> bool {
            unsafe {
                let mrb = mrb_open();
                let value = Value::try_from_mrb(mrb, v.clone()).expect("convert");
                let value = <Vec<u8>>::try_from_mrb(mrb, value).expect("convert");
                let good = value == v;
                mrb_close(mrb);
                good
            }
        }

        #[quickcheck]
        fn roundtrip_err(b: bool) -> bool {
            unsafe {
                let mrb = mrb_open();
                let value = Value::try_from_mrb(mrb, b).expect("convert");
                let value = <Vec<u8>>::try_from_mrb(mrb, value);
                mrb_close(mrb);
                let expected = Err(Error {
                    from: Ruby::Bool,
                    to: Rust::Bytes,
                });
                value == expected
            }
        }
    }

    mod slice {
        use super::*;

        #[allow(clippy::needless_pass_by_value)]
        #[quickcheck]
        fn convert_to_slice(v: Vec<u8>) -> bool {
            unsafe {
                let v = v.as_slice();
                let mrb = mrb_open();
                let value = Value::try_from_mrb(mrb, v).expect("convert");
                let good = value.ruby_type() == Ruby::String;
                mrb_close(mrb);
                good
            }
        }

        #[allow(clippy::needless_pass_by_value)]
        #[quickcheck]
        fn slice_with_value(v: Vec<u8>) -> bool {
            unsafe {
                let v = v.as_slice();
                let mrb = mrb_open();
                let value = Value::try_from_mrb(mrb, v).expect("convert");
                let inner = value.inner();
                let len = mrb_string_value_len(mrb, inner);
                let len = usize::try_from(len).expect("usize");
                let good = v.len() == len;
                mrb_close(mrb);
                good
            }
        }
    }
}
