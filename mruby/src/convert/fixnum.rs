use mruby_sys::*;

use crate::convert::{Error, TryFromMrb};
use crate::value::{Ruby, Rust, Value};

pub type Int = i64;

impl TryFromMrb<Int> for Value {
    type From = Rust;
    type To = Ruby;

    fn try_from_mrb(_mrb: *mut mrb_state, value: Int) -> Result<Self, Error<Self::From, Self::To>> {
        Ok(Self::new(unsafe { mrb_sys_fixnum_value(value) }))
    }
}

impl TryFromMrb<Value> for Int {
    type From = Ruby;
    type To = Rust;

    fn try_from_mrb(
        _mrb: *mut mrb_state,
        value: Value,
    ) -> Result<Self, Error<Self::From, Self::To>> {
        match value.ruby_type() {
            Ruby::Fixnum => {
                let inner = value.inner();
                let int = unsafe { mrb_sys_fixnum_to_cint(inner) };
                Ok(int)
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

    #[quickcheck]
    fn convert_to_fixnum(i: Int) -> bool {
        let mrb = unsafe { mrb_open() };
        let value = Value::try_from_mrb(mrb, i).expect("convert");
        unsafe { mrb_close(mrb) };
        value.ruby_type() == Ruby::Fixnum
    }

    #[quickcheck]
    fn fixnum_with_value(i: Int) -> bool {
        let mrb = unsafe { mrb_open() };
        let value = Value::try_from_mrb(mrb, i).expect("convert");
        let inner = value.inner();
        let cint = unsafe { mrb_sys_fixnum_to_cint(inner) };
        unsafe { mrb_close(mrb) };
        cint == i
    }

    #[quickcheck]
    fn roundtrip(i: Int) -> bool {
        let mrb = unsafe { mrb_open() };
        let value = Value::try_from_mrb(mrb, i).expect("convert");
        let value = Int::try_from_mrb(mrb, value).expect("convert");
        unsafe { mrb_close(mrb) };
        value == i
    }

    #[quickcheck]
    fn roundtrip_err(b: bool) -> bool {
        let mrb = unsafe { mrb_open() };
        let value = Value::try_from_mrb(mrb, b).expect("convert");
        let value = Int::try_from_mrb(mrb, value);
        unsafe { mrb_close(mrb) };
        let expected = Err(Error {
            from: Ruby::Bool,
            to: Rust::SignedInt,
        });
        value == expected
    }
}
