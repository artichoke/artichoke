use crate::sys;
use crate::types::{Float, Ruby, Rust};
use crate::value::Value;
use crate::{Artichoke, ArtichokeError, ConvertMut, TryConvert};

// TODO: when ,mruby is gone, float conversion should not allocate.
impl ConvertMut<Float, Value> for Artichoke {
    fn convert_mut(&mut self, value: Float) -> Value {
        let mrb = self.0.borrow().mrb;
        let float = unsafe { sys::mrb_sys_float_value(mrb, value) };
        Value::new(self, float)
    }
}

impl TryConvert<Value, Float> for Artichoke {
    fn try_convert(&self, value: Value) -> Result<Float, ArtichokeError> {
        match value.ruby_type() {
            Ruby::Float => {
                let value = value.inner();
                Ok(unsafe { sys::mrb_sys_float_to_cdouble(value) })
            }
            type_tag => Err(ArtichokeError::ConvertToRust {
                from: type_tag,
                to: Rust::Float,
            }),
        }
    }
}

#[cfg(test)]
mod tests {
    use quickcheck_macros::quickcheck;

    use crate::test::prelude::*;

    #[test]
    fn fail_convert() {
        let mut interp = crate::interpreter().expect("init");
        // get a mrb_value that can't be converted to a primitive type.
        let value = interp.eval(b"Object.new").expect("eval");
        let expected = Err(ArtichokeError::ConvertToRust {
            from: Ruby::Object,
            to: Rust::Float,
        });
        let result = value.try_into::<Float>();
        assert_eq!(result, expected);
    }

    #[quickcheck]
    fn convert_to_float(f: Float) -> bool {
        let mut interp = crate::interpreter().expect("init");
        let value = interp.convert_mut(f);
        value.ruby_type() == Ruby::Float
    }

    #[quickcheck]
    fn float_with_value(f: Float) -> bool {
        let mut interp = crate::interpreter().expect("init");
        let value = interp.convert_mut(f);
        let inner = value.inner();
        let cdouble = unsafe { sys::mrb_sys_float_to_cdouble(inner) };
        (cdouble - f).abs() < std::f64::EPSILON
    }

    #[quickcheck]
    fn roundtrip(f: Float) -> bool {
        let mut interp = crate::interpreter().expect("init");
        let value = interp.convert_mut(f);
        let value = value.try_into::<Float>().expect("convert");
        (value - f).abs() < std::f64::EPSILON
    }

    #[quickcheck]
    fn roundtrip_err(b: bool) -> bool {
        let interp = crate::interpreter().expect("init");
        let value = interp.convert(b);
        let value = value.try_into::<Float>();
        let expected = Err(ArtichokeError::ConvertToRust {
            from: Ruby::Bool,
            to: Rust::Float,
        });
        value == expected
    }
}
