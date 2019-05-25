use crate::convert::{Error, FromMrb, TryFromMrb};
use crate::interpreter::Mrb;
use crate::value::types::{Ruby, Rust};
use crate::value::Value;

impl FromMrb<Vec<Vec<u8>>> for Value {
    type From = Rust;
    type To = Ruby;

    fn from_mrb(interp: &Mrb, value: Vec<Vec<u8>>) -> Self {
        Self::from_mrb(interp, value.iter().map(Vec::as_slice).collect::<Vec<_>>())
    }
}

impl FromMrb<Vec<&[u8]>> for Value {
    type From = Rust;
    type To = Ruby;

    fn from_mrb(interp: &Mrb, value: Vec<&[u8]>) -> Self {
        let mut values = Vec::with_capacity(value.len());
        for item in value {
            values.push(Self::from_mrb(interp, item));
        }
        Self::from_mrb(interp, values)
    }
}

#[allow(clippy::use_self)]
// https://github.com/rust-lang/rust-clippy/issues/4143
impl TryFromMrb<Value> for Vec<Vec<u8>> {
    type From = Ruby;
    type To = Rust;

    unsafe fn try_from_mrb(
        interp: &Mrb,
        value: Value,
    ) -> Result<Self, Error<Self::From, Self::To>> {
        let values = <Vec<Value>>::try_from_mrb(interp, value)?;
        let mut vec = Self::with_capacity(values.len());
        for item in values {
            vec.push(<Vec<u8>>::try_from_mrb(interp, item)?);
        }
        Ok(vec)
    }
}

#[allow(clippy::use_self)]
// https://github.com/rust-lang/rust-clippy/issues/4143
impl FromMrb<Vec<Option<Vec<u8>>>> for Value {
    type From = Rust;
    type To = Ruby;

    fn from_mrb(interp: &Mrb, value: Vec<Option<Vec<u8>>>) -> Self {
        let mut values = Vec::with_capacity(value.len());
        for item in value {
            values.push(Self::from_mrb(interp, item));
        }
        Self::from_mrb(interp, values)
    }
}

#[allow(clippy::use_self)]
// https://github.com/rust-lang/rust-clippy/issues/4143
impl FromMrb<Vec<Option<&[u8]>>> for Value {
    type From = Rust;
    type To = Ruby;

    fn from_mrb(interp: &Mrb, value: Vec<Option<&[u8]>>) -> Self {
        let mut values = Vec::with_capacity(value.len());
        for item in value {
            values.push(Self::from_mrb(interp, item));
        }
        Self::from_mrb(interp, values)
    }
}

#[allow(clippy::use_self)]
// https://github.com/rust-lang/rust-clippy/issues/4143
impl TryFromMrb<Value> for Vec<Option<Vec<u8>>> {
    type From = Ruby;
    type To = Rust;

    unsafe fn try_from_mrb(
        interp: &Mrb,
        value: Value,
    ) -> Result<Self, Error<Self::From, Self::To>> {
        let values = <Vec<Value>>::try_from_mrb(interp, value)?;
        let mut vec = Self::with_capacity(values.len());
        for item in values {
            vec.push(<Option<Vec<u8>>>::try_from_mrb(interp, item)?);
        }
        Ok(vec)
    }
}

#[cfg(test)]
// FromMrb<Vec<Vec<u8>>> is implemented in terms of FromMrb<Vec<&[u8]>> so only
// implement the tests for Vec<u8> to exercise both code paths.
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
            to: Rust::Vec,
        };
        let result = unsafe { <Vec<Vec<u8>>>::try_from_mrb(&interp, value) }.map(|_| ());
        assert_eq!(result, Err(expected));
    }

    #[allow(clippy::needless_pass_by_value)]
    #[quickcheck]
    fn convert_to_value(v: Vec<Vec<u8>>) -> bool {
        let interp = Interpreter::create().expect("mrb init");
        let value = Value::from_mrb(&interp, v.clone());
        let inner = value.inner();
        let size = i64::try_from(v.len()).expect("vec size");
        unsafe { sys::mrb_sys_ary_len(inner) == size }
    }

    #[allow(clippy::needless_pass_by_value)]
    #[quickcheck]
    fn roundtrip(v: Vec<Vec<u8>>) -> bool {
        let interp = Interpreter::create().expect("mrb init");
        let value = Value::from_mrb(&interp, v.clone());
        unsafe { <Vec<Vec<u8>>>::try_from_mrb(&interp, value) == Ok(v) }
    }
}
