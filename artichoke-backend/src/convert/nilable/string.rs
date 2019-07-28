use crate::convert::{Convert, Error, TryConvert};
use crate::value::types::{Ruby, Rust};
use crate::value::Value;
use crate::Mrb;

impl Convert<Option<String>> for Value {
    type From = Rust;
    type To = Ruby;

    fn convert(interp: &Mrb, value: Option<String>) -> Self {
        Self::convert(interp, value.as_ref().map(String::as_str))
    }
}

impl Convert<Option<&str>> for Value {
    type From = Rust;
    type To = Ruby;

    fn convert(interp: &Mrb, value: Option<&str>) -> Self {
        if let Some(value) = value {
            Self::convert(interp, value)
        } else {
            Self::convert(interp, None::<Self>)
        }
    }
}

#[allow(clippy::use_self)]
// https://github.com/rust-lang/rust-clippy/issues/4143
impl TryConvert<Value> for Option<String> {
    type From = Ruby;
    type To = Rust;

    unsafe fn try_convert(interp: &Mrb, value: Value) -> Result<Self, Error<Self::From, Self::To>> {
        let value = <Option<Value>>::try_convert(interp, value)?;
        if let Some(item) = value {
            Ok(Some(String::try_convert(interp, item)?))
        } else {
            Ok(None)
        }
    }
}

#[cfg(test)]
// Convert<Option<String>> is implemented in terms of Convert<Option<&str>> so
// only implement the tests for String to exercise both code paths.
mod tests {
    use quickcheck_macros::quickcheck;

    use crate::convert::{Convert, TryConvert};
    use crate::eval::Eval;
    use crate::sys;
    use crate::value::types::Ruby;
    use crate::value::Value;

    #[test]
    fn fail_convert() {
        let interp = crate::interpreter().expect("mrb init");
        // get a mrb_value that can't be converted to a primitive type.
        let value = interp.eval("Object.new").expect("eval");
        let result = unsafe { <Option<String>>::try_convert(&interp, value) }.map(|_| ());
        assert_eq!(result.map_err(|e| e.from), Err(Ruby::Object));
    }

    #[allow(clippy::needless_pass_by_value)]
    #[quickcheck]
    fn convert_to_value(v: Option<String>) -> bool {
        let interp = crate::interpreter().expect("mrb init");
        let value = Value::convert(&interp, v.clone());
        if let Some(v) = v {
            let value = unsafe { String::try_convert(&interp, value) }.expect("convert");
            value == v
        } else {
            unsafe { sys::mrb_sys_value_is_nil(value.inner()) }
        }
    }

    #[allow(clippy::needless_pass_by_value)]
    #[quickcheck]
    fn roundtrip(v: Option<String>) -> bool {
        let interp = crate::interpreter().expect("mrb init");
        let value = Value::convert(&interp, v.clone());
        let value = unsafe { <Option<String>>::try_convert(&interp, value) }.expect("convert");
        value == v
    }
}
