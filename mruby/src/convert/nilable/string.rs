use crate::convert::{Error, FromMrb, TryFromMrb};
use crate::interpreter::Mrb;
use crate::value::types::{Ruby, Rust};
use crate::value::Value;

impl FromMrb<Option<String>> for Value {
    type From = Rust;
    type To = Ruby;

    fn from_mrb(interp: &Mrb, value: Option<String>) -> Self {
        Self::from_mrb(interp, value.as_ref().map(String::as_str))
    }
}

impl FromMrb<Option<&str>> for Value {
    type From = Rust;
    type To = Ruby;

    fn from_mrb(interp: &Mrb, value: Option<&str>) -> Self {
        if let Some(value) = value {
            Self::from_mrb(interp, value)
        } else {
            Self::from_mrb(interp, None::<Self>)
        }
    }
}

#[allow(clippy::use_self)]
// https://github.com/rust-lang/rust-clippy/issues/4143
impl TryFromMrb<Value> for Option<String> {
    type From = Ruby;
    type To = Rust;

    unsafe fn try_from_mrb(
        interp: &Mrb,
        value: Value,
    ) -> Result<Self, Error<Self::From, Self::To>> {
        let value = <Option<Value>>::try_from_mrb(interp, value)?;
        if let Some(item) = value {
            Ok(Some(String::try_from_mrb(interp, item)?))
        } else {
            Ok(None)
        }
    }
}

#[cfg(test)]
// FromMrb<Option<String>> is implemented in terms of FromMrb<Option<&str>> so
// only implement the tests for String to exercise both code paths.
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
        let result = unsafe { <Option<String>>::try_from_mrb(&interp, value) }.map(|_| ());
        assert_eq!(result.map_err(|e| e.from), Err(Ruby::Object));
    }

    #[allow(clippy::needless_pass_by_value)]
    #[quickcheck]
    fn convert_to_value(v: Option<String>) -> bool {
        let interp = Interpreter::create().expect("mrb init");
        let value = Value::from_mrb(&interp, v.clone());
        if let Some(v) = v {
            let value = unsafe { String::try_from_mrb(&interp, value) }.expect("convert");
            value == v
        } else {
            unsafe { sys::mrb_sys_value_is_nil(value.inner()) }
        }
    }

    #[allow(clippy::needless_pass_by_value)]
    #[quickcheck]
    fn roundtrip(v: Option<String>) -> bool {
        let interp = Interpreter::create().expect("mrb init");
        let value = Value::from_mrb(&interp, v.clone());
        let value = unsafe { <Option<String>>::try_from_mrb(&interp, value) }.expect("convert");
        value == v
    }
}
