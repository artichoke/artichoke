use crate::convert::{Convert, Error, TryConvert};
use crate::types::{Ruby, Rust};
use crate::value::Value;
use crate::Artichoke;

impl Convert<Option<Vec<u8>>> for Value {
    type From = Rust;
    type To = Ruby;

    fn convert(interp: &Artichoke, value: Option<Vec<u8>>) -> Self {
        if let Some(value) = value {
            Self::convert(interp, value)
        } else {
            Self::convert(interp, None::<Self>)
        }
    }
}

impl Convert<Option<&[u8]>> for Value {
    type From = Rust;
    type To = Ruby;

    fn convert(interp: &Artichoke, value: Option<&[u8]>) -> Self {
        if let Some(value) = value {
            Self::convert(interp, value)
        } else {
            Self::convert(interp, None::<Self>)
        }
    }
}

#[allow(clippy::use_self)]
// https://github.com/rust-lang/rust-clippy/issues/4143
impl TryConvert<Value> for Option<Vec<u8>> {
    type From = Ruby;
    type To = Rust;

    unsafe fn try_convert(
        interp: &Artichoke,
        value: Value,
    ) -> Result<Self, Error<Self::From, Self::To>> {
        let value = <Option<Value>>::try_convert(interp, value)?;
        if let Some(item) = value {
            Ok(Some(<Vec<u8>>::try_convert(interp, item)?))
        } else {
            Ok(None)
        }
    }
}

#[cfg(test)]
// Convert<Option<Vec<u8>>> is implemented in terms of Convert<Option<&[u8]>> so
// only implement the tests for Vec<u8> to exercise both code paths.
mod tests {
    use quickcheck_macros::quickcheck;

    use crate::convert::{Convert, TryConvert};
    use crate::eval::Eval;
    use crate::sys;
    use crate::types::Ruby;
    use crate::value::Value;

    #[test]
    fn fail_convert() {
        let interp = crate::interpreter().expect("init");
        // get a mrb_value that can't be converted to a primitive type.
        let value = interp.eval("Object.new").expect("eval");
        let result = unsafe { <Option<Vec<u8>>>::try_convert(&interp, value) }.map(|_| ());
        assert_eq!(result.map_err(|e| e.from), Err(Ruby::Object));
    }

    #[allow(clippy::needless_pass_by_value)]
    #[quickcheck]
    fn convert_to_value(v: Option<Vec<u8>>) -> bool {
        let interp = crate::interpreter().expect("init");
        let value = Value::convert(&interp, v.clone());
        if let Some(v) = v {
            let value = unsafe { <Vec<u8>>::try_convert(&interp, value) }.expect("convert");
            value == v
        } else {
            unsafe { sys::mrb_sys_value_is_nil(value.inner()) }
        }
    }

    #[allow(clippy::needless_pass_by_value)]
    #[quickcheck]
    fn roundtrip(v: Option<Vec<u8>>) -> bool {
        let interp = crate::interpreter().expect("init");
        let value = Value::convert(&interp, v.clone());
        let value = unsafe { <Option<Vec<u8>>>::try_convert(&interp, value) }.expect("convert");
        value == v
    }
}
