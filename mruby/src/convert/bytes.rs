use std::convert::TryFrom;

use crate::convert::{Error, FromMrb, TryFromMrb};
use crate::interpreter::Mrb;
use crate::sys;
use crate::value::types::{Ruby, Rust};
use crate::value::Value;

impl FromMrb<Vec<u8>> for Value {
    type From = Rust;
    type To = Ruby;

    fn from_mrb(interp: &Mrb, value: Vec<u8>) -> Self {
        Self::from_mrb(interp, value.as_slice())
    }
}

impl FromMrb<&[u8]> for Value {
    type From = Rust;
    type To = Ruby;

    fn from_mrb(interp: &Mrb, value: &[u8]) -> Self {
        // mruby strings contain raw bytes, so we can convert from a &[u8] to a
        // `char *` and `size_t`.
        let raw = value.as_ptr() as *const i8;
        let len = value.len();
        Self::new(interp, unsafe {
            sys::mrb_str_new(interp.borrow().mrb, raw, len)
        })
    }
}

impl TryFromMrb<Value> for Vec<u8> {
    type From = Ruby;
    type To = Rust;

    unsafe fn try_from_mrb(
        interp: &Mrb,
        value: Value,
    ) -> Result<Self, Error<Self::From, Self::To>> {
        match value.ruby_type() {
            Ruby::String => {
                let bytes = value.inner();
                let raw = sys::mrb_string_value_ptr(interp.borrow().mrb, bytes) as *const u8;
                let len = sys::mrb_string_value_len(interp.borrow().mrb, bytes);
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
// FromMrb<Vec<u8>> is implemented in terms of FromMrb<&[u8]> so only implement
// the tests for Vec<u8> to exercise both code paths.
mod tests {
    use quickcheck_macros::quickcheck;
    use std::convert::TryFrom;

    use crate::convert::{Error, FromMrb, TryFromMrb};
    use crate::eval::MrbEval;
    use crate::interpreter::Interpreter;
    use crate::sys;
    use crate::value::types::{Ruby, Rust};
    use crate::value::Value;

    #[test]
    fn fail_convert() {
        let interp = Interpreter::create().expect("mrb init");
        // get a mrb_value that can't be converted to a primitive type.
        let value = interp.eval("Object.new").expect("eval");
        let expected = Error {
            from: Ruby::Object,
            to: Rust::Bytes,
        };
        let result = unsafe { <Vec<u8>>::try_from_mrb(&interp, value) }.map(|_| ());
        assert_eq!(result, Err(expected));
    }

    #[allow(clippy::needless_pass_by_value)]
    #[quickcheck]
    fn convert_to_vec(v: Vec<u8>) -> bool {
        let interp = Interpreter::create().expect("mrb init");
        let value = Value::from_mrb(&interp, v.clone());
        value.ruby_type() == Ruby::String
    }

    #[allow(clippy::needless_pass_by_value)]
    #[quickcheck]
    fn vec_with_value(v: Vec<u8>) -> bool {
        let interp = Interpreter::create().expect("mrb init");
        let value = Value::from_mrb(&interp, v.clone());
        let inner = value.inner();
        let len = unsafe { sys::mrb_string_value_len(interp.borrow().mrb, inner) };
        let len = usize::try_from(len).expect("usize");
        v.len() == len
    }

    #[allow(clippy::needless_pass_by_value)]
    #[quickcheck]
    fn roundtrip(v: Vec<u8>) -> bool {
        let interp = Interpreter::create().expect("mrb init");
        let value = Value::from_mrb(&interp, v.clone());
        let value = unsafe { <Vec<u8>>::try_from_mrb(&interp, value) }.expect("convert");
        value == v
    }

    #[quickcheck]
    fn roundtrip_err(b: bool) -> bool {
        let interp = Interpreter::create().expect("mrb init");
        let value = Value::from_mrb(&interp, b);
        let value = unsafe { <Vec<u8>>::try_from_mrb(&interp, value) };
        let expected = Err(Error {
            from: Ruby::Bool,
            to: Rust::Bytes,
        });
        value == expected
    }
}
