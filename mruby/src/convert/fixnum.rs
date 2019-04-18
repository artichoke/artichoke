use mruby_sys::*;

use crate::convert::{Error, TryFromMrb};
use crate::value::{Ruby, Rust, Value};

pub type Int = i64;

impl TryFromMrb<Int> for Value {
    type From = Rust;
    type To = Ruby;

    unsafe fn try_from_mrb(
        _mrb: *mut mrb_state,
        value: Int,
    ) -> Result<Self, Error<Self::From, Self::To>> {
        Ok(Self::new(mrb_sys_fixnum_value(value)))
    }
}

impl TryFromMrb<Value> for Int {
    type From = Ruby;
    type To = Rust;

    unsafe fn try_from_mrb(
        _mrb: *mut mrb_state,
        value: Value,
    ) -> Result<Self, Error<Self::From, Self::To>> {
        match value.ruby_type() {
            Ruby::Fixnum => {
                let inner = value.inner();
                Ok(mrb_sys_fixnum_to_cint(inner))
            }
            type_tag => Err(Error {
                from: type_tag,
                to: Rust::SignedInt,
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
    fn convert_to_fixnum(i: Int) -> bool {
        unsafe {
            let mrb = Mrb::new().expect("mrb init");
            let value = Value::try_from_mrb(mrb.inner().unwrap(), i).expect("convert");
            value.ruby_type() == Ruby::Fixnum
        }
    }

    #[quickcheck]
    fn fixnum_with_value(i: Int) -> bool {
        unsafe {
            let mrb = Mrb::new().expect("mrb init");
            let value = Value::try_from_mrb(mrb.inner().unwrap(), i).expect("convert");
            let inner = value.inner();
            let cint = mrb_sys_fixnum_to_cint(inner);
            cint == i
        }
    }

    #[quickcheck]
    fn roundtrip(i: Int) -> bool {
        unsafe {
            let mrb = Mrb::new().expect("mrb init");
            let value = Value::try_from_mrb(mrb.inner().unwrap(), i).expect("convert");
            let value = Int::try_from_mrb(mrb.inner().unwrap(), value).expect("convert");
            value == i
        }
    }

    #[quickcheck]
    fn roundtrip_err(b: bool) -> bool {
        unsafe {
            let mrb = Mrb::new().expect("mrb init");
            let value = Value::try_from_mrb(mrb.inner().unwrap(), b).expect("convert");
            let value = Int::try_from_mrb(mrb.inner().unwrap(), value);
            let expected = Err(Error {
                from: Ruby::Bool,
                to: Rust::SignedInt,
            });
            value == expected
        }
    }
}
