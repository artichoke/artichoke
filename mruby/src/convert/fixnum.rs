use crate::convert::{Error, FromMrb, TryFromMrb};
use crate::interpreter::Mrb;
use crate::sys;
use crate::value::types::{Ruby, Rust};
use crate::value::Value;

pub type Int = i64;

impl FromMrb<Int> for Value {
    type From = Rust;
    type To = Ruby;

    fn from_mrb(interp: &Mrb, value: Int) -> Self {
        Self::new(interp, unsafe { sys::mrb_sys_fixnum_value(value) })
    }
}

impl TryFromMrb<Value> for Int {
    type From = Ruby;
    type To = Rust;

    unsafe fn try_from_mrb(
        _interp: &Mrb,
        value: Value,
    ) -> Result<Self, Error<Self::From, Self::To>> {
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

    use crate::convert::fixnum::Int;
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
            to: Rust::SignedInt,
        };
        let result = unsafe { Int::try_from_mrb(&interp, value) }.map(|_| ());
        assert_eq!(result, Err(expected));
    }

    #[quickcheck]
    fn convert_to_fixnum(i: Int) -> bool {
        let interp = Interpreter::create().expect("mrb init");
        let value = Value::from_mrb(&interp, i);
        value.ruby_type() == Ruby::Fixnum
    }

    #[quickcheck]
    fn fixnum_with_value(i: Int) -> bool {
        let interp = Interpreter::create().expect("mrb init");
        let value = Value::from_mrb(&interp, i);
        let inner = value.inner();
        let cint = unsafe { sys::mrb_sys_fixnum_to_cint(inner) };
        cint == i
    }

    #[quickcheck]
    fn roundtrip(i: Int) -> bool {
        let interp = Interpreter::create().expect("mrb init");
        let value = Value::from_mrb(&interp, i);
        let value = unsafe { Int::try_from_mrb(&interp, value) }.expect("convert");
        value == i
    }

    #[quickcheck]
    fn roundtrip_err(b: bool) -> bool {
        let interp = Interpreter::create().expect("mrb init");
        let value = Value::from_mrb(&interp, b);
        let value = unsafe { Int::try_from_mrb(&interp, value) };
        let expected = Err(Error {
            from: Ruby::Bool,
            to: Rust::SignedInt,
        });
        value == expected
    }
}
