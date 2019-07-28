use crate::convert::float::Float;
use crate::convert::{Convert, Error, TryConvert};
use crate::value::types::{Ruby, Rust};
use crate::value::Value;
use crate::Artichoke;

impl Convert<Option<Float>> for Value {
    type From = Rust;
    type To = Ruby;

    fn convert(interp: &Artichoke, value: Option<Float>) -> Self {
        if let Some(value) = value {
            Self::convert(interp, value)
        } else {
            Self::convert(interp, None::<Self>)
        }
    }
}

#[allow(clippy::use_self)]
// https://github.com/rust-lang/rust-clippy/issues/4143
impl TryConvert<Value> for Option<Float> {
    type From = Ruby;
    type To = Rust;

    unsafe fn try_convert(
        interp: &Artichoke,
        value: Value,
    ) -> Result<Self, Error<Self::From, Self::To>> {
        let value = <Option<Value>>::try_convert(interp, value)?;
        if let Some(item) = value {
            Ok(Some(Float::try_convert(interp, item)?))
        } else {
            Ok(None)
        }
    }
}

#[cfg(test)]
mod tests {
    use quickcheck_macros::quickcheck;

    use crate::convert::float::Float;
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
        let result = unsafe { <Option<Float>>::try_convert(&interp, value) }.map(|_| ());
        assert_eq!(result.map_err(|e| e.from), Err(Ruby::Object));
    }

    #[quickcheck]
    fn convert_to_value(v: Option<Float>) -> bool {
        let interp = crate::interpreter().expect("mrb init");
        let value = Value::convert(&interp, v);
        if let Some(v) = v {
            let value = unsafe { Float::try_convert(&interp, value) }.expect("convert");
            (value - v).abs() < std::f64::EPSILON
        } else {
            unsafe { sys::mrb_sys_value_is_nil(value.inner()) }
        }
    }

    #[quickcheck]
    fn roundtrip(v: Option<Float>) -> bool {
        let interp = crate::interpreter().expect("mrb init");
        let value = Value::convert(&interp, v);
        let value = unsafe { <Option<Float>>::try_convert(&interp, value) }.expect("convert");
        match (value, v) {
            (Some(value), Some(v)) => (value - v).abs() < std::f64::EPSILON,
            (None, None) => true,
            _ => false,
        }
    }
}
