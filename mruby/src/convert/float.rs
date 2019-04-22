use mruby_sys::*;

use crate::convert::{Error, TryFromMrb};
use crate::interpreter::MrbApi;
use crate::value::{Ruby, Rust, Value};

pub type Float = f64;

impl TryFromMrb<Float> for Value {
    type From = Rust;
    type To = Ruby;

    unsafe fn try_from_mrb(
        api: &MrbApi,
        value: Float,
    ) -> Result<Self, Error<Self::From, Self::To>> {
        Ok(Self::new(mrb_sys_float_value(api.mrb(), value)))
    }
}

impl TryFromMrb<Value> for Float {
    type From = Ruby;
    type To = Rust;

    unsafe fn try_from_mrb(
        _api: &MrbApi,
        value: Value,
    ) -> Result<Self, Error<Self::From, Self::To>> {
        match value.ruby_type() {
            Ruby::Float => {
                let inner = value.inner();
                Ok(mrb_sys_float_to_cdouble(inner))
            }
            type_tag => Err(Error {
                from: type_tag,
                to: Rust::Float,
            }),
        }
    }
}

#[cfg(test)]
mod tests {
    use mruby_sys::*;
    use quickcheck_macros::quickcheck;

    use super::*;
    use crate::interpreter::*;

    #[quickcheck]
    fn convert_to_float(f: Float) -> bool {
        unsafe {
            let interp = Interpreter::create().expect("mrb init");
            let api = interp.borrow_mut();
            let value = Value::try_from_mrb(&api, f).expect("convert");
            value.ruby_type() == Ruby::Float
        }
    }

    #[quickcheck]
    fn float_with_value(f: Float) -> bool {
        unsafe {
            let interp = Interpreter::create().expect("mrb init");
            let api = interp.borrow_mut();
            let value = Value::try_from_mrb(&api, f).expect("convert");
            let inner = value.inner();
            let cdouble = mrb_sys_float_to_cdouble(inner);
            (cdouble - f).abs() < std::f64::EPSILON
        }
    }

    #[quickcheck]
    fn roundtrip(f: Float) -> bool {
        unsafe {
            let interp = Interpreter::create().expect("mrb init");
            let api = interp.borrow_mut();
            let value = Value::try_from_mrb(&api, f).expect("convert");
            let value = Float::try_from_mrb(&api, value).expect("convert");
            (value - f).abs() < std::f64::EPSILON
        }
    }

    #[quickcheck]
    fn roundtrip_err(b: bool) -> bool {
        unsafe {
            let interp = Interpreter::create().expect("mrb init");
            let api = interp.borrow_mut();
            let value = Value::try_from_mrb(&api, b).expect("convert");
            let value = Float::try_from_mrb(&api, value);
            let expected = Err(Error {
                from: Ruby::Bool,
                to: Rust::Float,
            });
            value == expected
        }
    }
}
