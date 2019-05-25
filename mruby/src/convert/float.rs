use crate::convert::{Error, FromMrb, TryFromMrb};
use crate::interpreter::Mrb;
use crate::sys;
use crate::value::types::{Ruby, Rust};
use crate::value::Value;

pub type Float = f64;

impl FromMrb<Float> for Value {
    type From = Rust;
    type To = Ruby;

    fn from_mrb(interp: &Mrb, value: Float) -> Self {
        Self::new(interp, unsafe {
            sys::mrb_sys_float_value(interp.borrow().mrb, value)
        })
    }
}

impl TryFromMrb<Value> for Float {
    type From = Ruby;
    type To = Rust;

    unsafe fn try_from_mrb(
        _interp: &Mrb,
        value: Value,
    ) -> Result<Self, Error<Self::From, Self::To>> {
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

    use crate::convert::float::Float;
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
            to: Rust::Float,
        };
        let result = unsafe { Float::try_from_mrb(&interp, value) }.map(|_| ());
        assert_eq!(result, Err(expected));
    }

    #[quickcheck]
    fn convert_to_float(f: Float) -> bool {
        let interp = Interpreter::create().expect("mrb init");
        let value = Value::from_mrb(&interp, f);
        value.ruby_type() == Ruby::Float
    }

    #[quickcheck]
    fn float_with_value(f: Float) -> bool {
        let interp = Interpreter::create().expect("mrb init");
        let value = Value::from_mrb(&interp, f);
        let inner = value.inner();
        let cdouble = unsafe { sys::mrb_sys_float_to_cdouble(inner) };
        (cdouble - f).abs() < std::f64::EPSILON
    }

    #[quickcheck]
    fn roundtrip(f: Float) -> bool {
        let interp = Interpreter::create().expect("mrb init");
        let value = Value::from_mrb(&interp, f);
        let value = unsafe { Float::try_from_mrb(&interp, value) }.expect("convert");
        (value - f).abs() < std::f64::EPSILON
    }

    #[quickcheck]
    fn roundtrip_err(b: bool) -> bool {
        let interp = Interpreter::create().expect("mrb init");
        let value = Value::from_mrb(&interp, b);
        let value = unsafe { Float::try_from_mrb(&interp, value) };
        let expected = Err(Error {
            from: Ruby::Bool,
            to: Rust::Float,
        });
        value == expected
    }
}
