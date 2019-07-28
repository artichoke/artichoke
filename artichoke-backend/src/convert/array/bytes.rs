use crate::convert::{Convert, Error, TryConvert};
use crate::value::types::{Ruby, Rust};
use crate::value::Value;
use crate::Mrb;

impl Convert<Vec<Vec<u8>>> for Value {
    type From = Rust;
    type To = Ruby;

    fn from_mrb(interp: &Mrb, value: Vec<Vec<u8>>) -> Self {
        Self::from_mrb(interp, value.iter().map(Vec::as_slice).collect::<Vec<_>>())
    }
}

impl Convert<Vec<&[u8]>> for Value {
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
impl TryConvert<Value> for Vec<Vec<u8>> {
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
impl Convert<Vec<Option<Vec<u8>>>> for Value {
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
impl Convert<Vec<Option<&[u8]>>> for Value {
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
impl TryConvert<Value> for Vec<Option<Vec<u8>>> {
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
// Convert<Vec<Vec<u8>>> is implemented in terms of Convert<Vec<&[u8]>> so only
// implement the tests for Vec<u8> to exercise both code paths.
mod tests {
    use quickcheck_macros::quickcheck;
    use std::convert::TryFrom;

    use crate::convert::{Convert, Error, TryConvert};
    use crate::eval::MrbEval;
    use crate::sys;
    use crate::value::types::{Ruby, Rust};
    use crate::value::Value;

    #[test]
    fn fail_convert() {
        let interp = crate::interpreter().expect("mrb init");
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
        let interp = crate::interpreter().expect("mrb init");
        let value = Value::from_mrb(&interp, v.clone());
        let inner = value.inner();
        let size = i64::try_from(v.len()).expect("vec size");
        unsafe { sys::mrb_sys_ary_len(inner) == size }
    }

    #[allow(clippy::needless_pass_by_value)]
    #[quickcheck]
    fn roundtrip(v: Vec<Vec<u8>>) -> bool {
        let interp = crate::interpreter().expect("mrb init");
        let value = Value::from_mrb(&interp, v.clone());
        unsafe { <Vec<Vec<u8>>>::try_from_mrb(&interp, value) == Ok(v) }
    }
}
