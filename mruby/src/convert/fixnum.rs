use crate::convert::{Error, TryFromMrb};
use crate::interpreter::Mrb;
use crate::sys;
use crate::value::{Ruby, Rust, Value};

pub type Int = i64;

impl TryFromMrb<Int> for Value {
    type From = Rust;
    type To = Ruby;

    unsafe fn try_from_mrb(_mrb: &Mrb, value: Int) -> Result<Self, Error<Self::From, Self::To>> {
        Ok(Self::new(sys::mrb_sys_fixnum_value(value)))
    }
}

impl TryFromMrb<Value> for Int {
    type From = Ruby;
    type To = Rust;

    unsafe fn try_from_mrb(_mrb: &Mrb, value: Value) -> Result<Self, Error<Self::From, Self::To>> {
        match value.ruby_type() {
            Ruby::Fixnum => Ok(sys::mrb_sys_fixnum_to_cint(value.inner())),
            type_tag => Err(Error {
                from: type_tag,
                to: Rust::SignedInt,
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
    use crate::value::*;

    #[quickcheck]
    fn convert_to_fixnum(i: Int) -> bool {
        unsafe {
            let interp = Interpreter::create().expect("mrb init");
            let value = Value::try_from_mrb(&interp, i).expect("convert");
            value.ruby_type() == Ruby::Fixnum
        }
    }

    #[quickcheck]
    fn fixnum_with_value(i: Int) -> bool {
        unsafe {
            let interp = Interpreter::create().expect("mrb init");
            let value = Value::try_from_mrb(&interp, i).expect("convert");
            let inner = value.inner();
            let cint = sys::mrb_sys_fixnum_to_cint(inner);
            cint == i
        }
    }

    #[quickcheck]
    fn roundtrip(i: Int) -> bool {
        unsafe {
            let interp = Interpreter::create().expect("mrb init");
            let value = Value::try_from_mrb(&interp, i).expect("convert");
            let value = Int::try_from_mrb(&interp, value).expect("convert");
            value == i
        }
    }

    #[quickcheck]
    fn roundtrip_err(b: bool) -> bool {
        unsafe {
            let interp = Interpreter::create().expect("mrb init");
            let value = Value::try_from_mrb(&interp, b).expect("convert");
            let value = Int::try_from_mrb(&interp, value);
            let expected = Err(Error {
                from: Ruby::Bool,
                to: Rust::SignedInt,
            });
            value == expected
        }
    }
}
