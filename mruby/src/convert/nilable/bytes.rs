use crate::convert::{Error, FromMrb, TryFromMrb};
use crate::interpreter::Mrb;
use crate::value::types::{Ruby, Rust};
use crate::value::Value;

impl FromMrb<Option<Vec<u8>>> for Value {
    type From = Rust;
    type To = Ruby;

    fn from_mrb(interp: &Mrb, value: Option<Vec<u8>>) -> Self {
        if let Some(value) = value {
            Self::from_mrb(interp, value)
        } else {
            Self::from_mrb(interp, None::<Self>)
        }
    }
}

impl FromMrb<Option<&[u8]>> for Value {
    type From = Rust;
    type To = Ruby;

    fn from_mrb(interp: &Mrb, value: Option<&[u8]>) -> Self {
        if let Some(value) = value {
            Self::from_mrb(interp, value)
        } else {
            Self::from_mrb(interp, None::<Self>)
        }
    }
}

#[allow(clippy::use_self)]
// https://github.com/rust-lang/rust-clippy/issues/4143
impl TryFromMrb<Value> for Option<Vec<u8>> {
    type From = Ruby;
    type To = Rust;

    unsafe fn try_from_mrb(
        interp: &Mrb,
        value: Value,
    ) -> Result<Self, Error<Self::From, Self::To>> {
        let value = <Option<Value>>::try_from_mrb(interp, value)?;
        if let Some(item) = value {
            Ok(Some(<Vec<u8>>::try_from_mrb(interp, item)?))
        } else {
            Ok(None)
        }
    }
}

#[cfg(test)]
// FromMrb<Option<Vec<u8>>> is implemented in terms of FromMrb<Option<&[u8]>> so
// only implement the tests for Vec<u8> to exercise both code paths.
mod tests {
    use quickcheck_macros::quickcheck;

    use crate::convert::{FromMrb, TryFromMrb};
    use crate::eval::MrbEval;
    use crate::interpreter::Interpreter;
    use crate::sys;
    use crate::value::types::Ruby;
    use crate::value::Value;

    #[test]
    fn fail_convert() {
        let interp = Interpreter::create().expect("mrb init");
        // get a mrb_value that can't be converted to a primitive type.
        let value = interp.eval("Object.new").expect("eval");
        let result = unsafe { <Option<Vec<u8>>>::try_from_mrb(&interp, value) }.map(|_| ());
        assert_eq!(result.map_err(|e| e.from), Err(Ruby::Object));
    }

    #[allow(clippy::needless_pass_by_value)]
    #[quickcheck]
    fn convert_to_value(v: Option<Vec<u8>>) -> bool {
        let interp = Interpreter::create().expect("mrb init");
        let value = Value::from_mrb(&interp, v.clone());
        if let Some(v) = v {
            let value = unsafe { <Vec<u8>>::try_from_mrb(&interp, value) }.expect("convert");
            value == v
        } else {
            unsafe { sys::mrb_sys_value_is_nil(value.inner()) }
        }
    }

    #[allow(clippy::needless_pass_by_value)]
    #[quickcheck]
    fn roundtrip(v: Option<Vec<u8>>) -> bool {
        let interp = Interpreter::create().expect("mrb init");
        let value = Value::from_mrb(&interp, v.clone());
        let value = unsafe { <Option<Vec<u8>>>::try_from_mrb(&interp, value) }.expect("convert");
        value == v
    }
}
