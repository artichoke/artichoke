use std::convert::TryFrom;

use crate::value::types::{Ruby, Rust};
use crate::convert::{Error, TryFromMrb};
use crate::interpreter::Mrb;
use crate::sys;
use crate::value::Value;

// TODO: Document danger associated with lifetimes.
// If the mrb_value lives longer than the `Vec<u8>` or `&[u8]` the mrb_value
// may point to garbage.

impl TryFromMrb<Vec<u8>> for Value {
    type From = Rust;
    type To = Ruby;

    unsafe fn try_from_mrb(mrb: &Mrb, value: Vec<u8>) -> Result<Self, Error<Self::From, Self::To>> {
        Self::try_from_mrb(mrb, value.as_slice())
    }
}

impl TryFromMrb<&[u8]> for Value {
    type From = Rust;
    type To = Ruby;

    unsafe fn try_from_mrb(mrb: &Mrb, value: &[u8]) -> Result<Self, Error<Self::From, Self::To>> {
        // mruby strings contain raw bytes, so we can convert from a &[u8] to a
        // `char *` and `size_t`.
        let raw = value.as_ptr() as *const i8;
        let len = value.len();
        Ok(Self::new(sys::mrb_str_new(mrb.borrow().mrb, raw, len)))
    }
}

impl TryFromMrb<Value> for Vec<u8> {
    type From = Ruby;
    type To = Rust;

    unsafe fn try_from_mrb(mrb: &Mrb, value: Value) -> Result<Self, Error<Self::From, Self::To>> {
        match value.ruby_type() {
            Ruby::String => {
                let value = value.inner();
                let raw = sys::mrb_string_value_ptr(mrb.borrow().mrb, value) as *const u8;
                let len = sys::mrb_string_value_len(mrb.borrow().mrb, value);
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
    use quickcheck_macros::quickcheck;

    use crate::convert::*;
    use crate::interpreter::*;
    use crate::sys;
    use crate::value::*;

    mod vec {
        use std::convert::TryFrom;

        use super::*;

        #[allow(clippy::needless_pass_by_value)]
        #[quickcheck]
        fn convert_to_vec(v: Vec<u8>) -> bool {
            unsafe {
                let interp = Interpreter::create().expect("mrb init");
                let value = Value::try_from_mrb(&interp, v.clone()).expect("convert");
                value.ruby_type() == Ruby::String
            }
        }

        #[allow(clippy::needless_pass_by_value)]
        #[quickcheck]
        fn vec_with_value(v: Vec<u8>) -> bool {
            unsafe {
                let interp = Interpreter::create().expect("mrb init");
                let value = Value::try_from_mrb(&interp, v.clone()).expect("convert");
                let inner = value.inner();
                let len = sys::mrb_string_value_len(interp.borrow().mrb, inner);
                let len = usize::try_from(len).expect("usize");
                v.len() == len
            }
        }

        #[allow(clippy::needless_pass_by_value)]
        #[quickcheck]
        fn roundtrip(v: Vec<u8>) -> bool {
            unsafe {
                let interp = Interpreter::create().expect("mrb init");
                let value = Value::try_from_mrb(&interp, v.clone()).expect("convert");
                let value = <Vec<u8>>::try_from_mrb(&interp, value).expect("convert");
                value == v
            }
        }

        #[quickcheck]
        fn roundtrip_err(b: bool) -> bool {
            unsafe {
                let interp = Interpreter::create().expect("mrb init");
                let value = Value::try_from_mrb(&interp, b).expect("convert");
                let value = <Vec<u8>>::try_from_mrb(&interp, value);
                let expected = Err(Error {
                    from: Ruby::Bool,
                    to: Rust::Bytes,
                });
                value == expected
            }
        }
    }

    mod slice {
        use std::convert::TryFrom;

        use super::*;

        #[allow(clippy::needless_pass_by_value)]
        #[quickcheck]
        fn convert_to_slice(v: Vec<u8>) -> bool {
            unsafe {
                let interp = Interpreter::create().expect("mrb init");
                let v = v.as_slice();
                let value = Value::try_from_mrb(&interp, v).expect("convert");
                value.ruby_type() == Ruby::String
            }
        }

        #[allow(clippy::needless_pass_by_value)]
        #[quickcheck]
        fn slice_with_value(v: Vec<u8>) -> bool {
            unsafe {
                let interp = Interpreter::create().expect("mrb init");
                let v = v.as_slice();
                let value = Value::try_from_mrb(&interp, v).expect("convert");
                let inner = value.inner();
                let len = sys::mrb_string_value_len(interp.borrow().mrb, inner);
                let len = usize::try_from(len).expect("usize");
                v.len() == len
            }
        }
    }
}
