use crate::convert::UnboxRubyError;
use crate::core::{ConvertMut, TryConvert};
use crate::exception::Exception;
use crate::sys;
use crate::types::{Fp, Ruby, Rust};
use crate::value::Value;
use crate::Artichoke;

// TODO: when ,mruby is gone, float conversion should not allocate.
impl ConvertMut<Fp, Value> for Artichoke {
    fn convert_mut(&mut self, value: Fp) -> Value {
        let float = unsafe { self.with_ffi_boundary(|mrb| sys::mrb_sys_float_value(mrb, value)) };
        Value::new(self, float.unwrap())
    }
}

impl TryConvert<Value, Fp> for Artichoke {
    type Error = Exception;

    fn try_convert(&self, value: Value) -> Result<Fp, Self::Error> {
        if let Ruby::Float = value.ruby_type() {
            let value = value.inner();
            Ok(unsafe { sys::mrb_sys_float_to_cdouble(value) })
        } else {
            Err(Exception::from(UnboxRubyError::new(&value, Rust::Float)))
        }
    }
}

#[cfg(test)]
mod tests {
    use quickcheck_macros::quickcheck;

    use crate::test::prelude::*;

    #[test]
    fn fail_convert() {
        let mut interp = crate::interpreter().unwrap();
        // get a Ruby Value that can't be converted to a primitive type.
        let value = interp.eval(b"Object.new").unwrap();
        let result = value.try_into::<Fp>(&interp);
        assert!(result.is_err());
    }

    #[quickcheck]
    fn convert_to_float(f: Fp) -> bool {
        let mut interp = crate::interpreter().unwrap();
        let value = interp.convert_mut(f);
        value.ruby_type() == Ruby::Float
    }

    #[quickcheck]
    fn float_with_value(f: Fp) -> bool {
        let mut interp = crate::interpreter().unwrap();
        let value = interp.convert_mut(f);
        let inner = value.inner();
        let cdouble = unsafe { sys::mrb_sys_float_to_cdouble(inner) };
        (cdouble - f).abs() < Fp::EPSILON
    }

    #[quickcheck]
    fn roundtrip(f: Fp) -> bool {
        let mut interp = crate::interpreter().unwrap();
        let value = interp.convert_mut(f);
        let value = value.try_into::<Fp>(&interp).unwrap();
        (value - f).abs() < Fp::EPSILON
    }

    #[quickcheck]
    fn roundtrip_err(b: bool) -> bool {
        let interp = crate::interpreter().unwrap();
        let value = interp.convert(b);
        let value = value.try_into::<Fp>(&interp);
        value.is_err()
    }
}
