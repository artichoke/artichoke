use crate::convert::{Error, TryFromMrb};
use crate::interpreter::Mrb;
use crate::sys;
use crate::value::types::{Ruby, Rust};
use crate::value::Value;

pub type Float = f64;

impl TryFromMrb<Float> for Value {
    type From = Rust;
    type To = Ruby;

    unsafe fn try_from_mrb(mrb: &Mrb, value: Float) -> Result<Self, Error<Self::From, Self::To>> {
        Ok(Self::new(sys::mrb_sys_float_value(mrb.borrow().mrb, value)))
    }
}

impl TryFromMrb<Value> for Float {
    type From = Ruby;
    type To = Rust;

    unsafe fn try_from_mrb(_mrb: &Mrb, value: Value) -> Result<Self, Error<Self::From, Self::To>> {
        match value.ruby_type() {
            Ruby::Float => Ok(sys::mrb_sys_float_to_cdouble(value.inner())),
            type_tag => Err(Error {
                from: type_tag,
                to: Rust::Float,
            }),
        }
    }
}

#[cfg(test)]
mod tests {
    use quickcheck_macros::quickcheck;

    use crate::convert::*;
    use crate::interpreter::*;
    use crate::sys;
    use crate::value::types::*;
    use crate::value::*;

    #[quickcheck]
    fn convert_to_float(f: Float) -> bool {
        unsafe {
            let interp = Interpreter::create().expect("mrb init");
            let value = Value::try_from_mrb(&interp, f).expect("convert");
            value.ruby_type() == Ruby::Float
        }
    }

    #[quickcheck]
    fn float_with_value(f: Float) -> bool {
        unsafe {
            let interp = Interpreter::create().expect("mrb init");
            let value = Value::try_from_mrb(&interp, f).expect("convert");
            let inner = value.inner();
            let cdouble = sys::mrb_sys_float_to_cdouble(inner);
            (cdouble - f).abs() < std::f64::EPSILON
        }
    }

    #[quickcheck]
    fn roundtrip(f: Float) -> bool {
        unsafe {
            let interp = Interpreter::create().expect("mrb init");
            let value = Value::try_from_mrb(&interp, f).expect("convert");
            let value = Float::try_from_mrb(&interp, value).expect("convert");
            (value - f).abs() < std::f64::EPSILON
        }
    }

    #[quickcheck]
    fn roundtrip_err(b: bool) -> bool {
        unsafe {
            let interp = Interpreter::create().expect("mrb init");
            let value = Value::try_from_mrb(&interp, b).expect("convert");
            let value = Float::try_from_mrb(&interp, value);
            let expected = Err(Error {
                from: Ruby::Bool,
                to: Rust::Float,
            });
            value == expected
        }
    }
}
