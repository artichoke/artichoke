use crate::convert::fixnum::Int;
use crate::convert::float::Float;
use crate::convert::{Error, FromMrb, TryFromMrb};
use crate::interpreter::Mrb;
use crate::value::types::{Ruby, Rust};
use crate::value::Value;

impl FromMrb<Option<Vec<bool>>> for Value {
    type From = Rust;
    type To = Ruby;

    fn from_mrb(interp: &Mrb, value: Option<Vec<bool>>) -> Self {
        if let Some(value) = value {
            Self::from_mrb(interp, value)
        } else {
            Self::from_mrb(interp, None::<Self>)
        }
    }
}

#[allow(clippy::use_self)]
// https://github.com/rust-lang/rust-clippy/issues/4143
impl TryFromMrb<Value> for Option<Vec<bool>> {
    type From = Ruby;
    type To = Rust;

    unsafe fn try_from_mrb(
        interp: &Mrb,
        value: Value,
    ) -> Result<Self, Error<Self::From, Self::To>> {
        let value = <Option<Value>>::try_from_mrb(interp, value)?;
        if let Some(item) = value {
            Ok(Some(<Vec<bool>>::try_from_mrb(interp, item)?))
        } else {
            Ok(None)
        }
    }
}

impl FromMrb<Option<Vec<Vec<u8>>>> for Value {
    type From = Rust;
    type To = Ruby;

    fn from_mrb(interp: &Mrb, value: Option<Vec<Vec<u8>>>) -> Self {
        if let Some(value) = value {
            Self::from_mrb(interp, value)
        } else {
            Self::from_mrb(interp, None::<Self>)
        }
    }
}

#[allow(clippy::use_self)]
// https://github.com/rust-lang/rust-clippy/issues/4143
impl TryFromMrb<Value> for Option<Vec<Vec<u8>>> {
    type From = Ruby;
    type To = Rust;

    unsafe fn try_from_mrb(
        interp: &Mrb,
        value: Value,
    ) -> Result<Self, Error<Self::From, Self::To>> {
        let value = <Option<Value>>::try_from_mrb(interp, value)?;
        if let Some(item) = value {
            Ok(Some(<Vec<Vec<u8>>>::try_from_mrb(interp, item)?))
        } else {
            Ok(None)
        }
    }
}

impl FromMrb<Option<Vec<Int>>> for Value {
    type From = Rust;
    type To = Ruby;

    fn from_mrb(interp: &Mrb, value: Option<Vec<Int>>) -> Self {
        if let Some(value) = value {
            Self::from_mrb(interp, value)
        } else {
            Self::from_mrb(interp, None::<Self>)
        }
    }
}

#[allow(clippy::use_self)]
// https://github.com/rust-lang/rust-clippy/issues/4143
impl TryFromMrb<Value> for Option<Vec<Int>> {
    type From = Ruby;
    type To = Rust;

    unsafe fn try_from_mrb(
        interp: &Mrb,
        value: Value,
    ) -> Result<Self, Error<Self::From, Self::To>> {
        let value = <Option<Value>>::try_from_mrb(interp, value)?;
        if let Some(item) = value {
            Ok(Some(<Vec<Int>>::try_from_mrb(interp, item)?))
        } else {
            Ok(None)
        }
    }
}

impl FromMrb<Option<Vec<Float>>> for Value {
    type From = Rust;
    type To = Ruby;

    fn from_mrb(interp: &Mrb, value: Option<Vec<Float>>) -> Self {
        if let Some(value) = value {
            Self::from_mrb(interp, value)
        } else {
            Self::from_mrb(interp, None::<Self>)
        }
    }
}

#[allow(clippy::use_self)]
// https://github.com/rust-lang/rust-clippy/issues/4143
impl TryFromMrb<Value> for Option<Vec<Float>> {
    type From = Ruby;
    type To = Rust;

    unsafe fn try_from_mrb(
        interp: &Mrb,
        value: Value,
    ) -> Result<Self, Error<Self::From, Self::To>> {
        let value = <Option<Value>>::try_from_mrb(interp, value)?;
        if let Some(item) = value {
            Ok(Some(<Vec<Float>>::try_from_mrb(interp, item)?))
        } else {
            Ok(None)
        }
    }
}

impl FromMrb<Option<Vec<String>>> for Value {
    type From = Rust;
    type To = Ruby;

    fn from_mrb(interp: &Mrb, value: Option<Vec<String>>) -> Self {
        if let Some(value) = value {
            Self::from_mrb(interp, value)
        } else {
            Self::from_mrb(interp, None::<Self>)
        }
    }
}

#[allow(clippy::use_self)]
// https://github.com/rust-lang/rust-clippy/issues/4143
impl TryFromMrb<Value> for Option<Vec<String>> {
    type From = Ruby;
    type To = Rust;

    unsafe fn try_from_mrb(
        interp: &Mrb,
        value: Value,
    ) -> Result<Self, Error<Self::From, Self::To>> {
        let value = <Option<Value>>::try_from_mrb(interp, value)?;
        if let Some(item) = value {
            Ok(Some(<Vec<String>>::try_from_mrb(interp, item)?))
        } else {
            Ok(None)
        }
    }
}

impl FromMrb<Option<Vec<&str>>> for Value {
    type From = Rust;
    type To = Ruby;

    fn from_mrb(interp: &Mrb, value: Option<Vec<&str>>) -> Self {
        if let Some(value) = value {
            Self::from_mrb(interp, value)
        } else {
            Self::from_mrb(interp, None::<Self>)
        }
    }
}

#[cfg(test)]
mod tests {
    use quickcheck_macros::quickcheck;

    use crate::convert::fixnum::Int;
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
        let result = unsafe { <Option<Vec<Int>>>::try_from_mrb(&interp, value) }.map(|_| ());
        assert_eq!(result.map_err(|e| e.from), Err(Ruby::Object));
    }

    #[quickcheck]
    #[allow(clippy::needless_pass_by_value)]
    fn convert_to_value(v: Option<Vec<Int>>) -> bool {
        let interp = Interpreter::create().expect("mrb init");
        let value = Value::from_mrb(&interp, v.clone());
        if let Some(v) = v {
            let value = unsafe { <Vec<Int>>::try_from_mrb(&interp, value) }.expect("convert");
            value == v
        } else {
            unsafe { sys::mrb_sys_value_is_nil(value.inner()) }
        }
    }

    #[quickcheck]
    #[allow(clippy::needless_pass_by_value)]
    fn roundtrip(v: Option<Vec<Int>>) -> bool {
        let interp = Interpreter::create().expect("mrb init");
        let value = Value::from_mrb(&interp, v.clone());
        let value = unsafe { <Option<Vec<Int>>>::try_from_mrb(&interp, value) }.expect("convert");
        value == v
    }
}
