use std::convert::TryFrom;

use crate::convert::{Convert, Error, TryConvert};
use crate::sys;
use crate::value::types::{Ruby, Rust};
use crate::value::Value;
use crate::Artichoke;

#[cfg(not(target_arch = "wasm32"))]
pub type Int = i64;
#[cfg(target_arch = "wasm32")]
pub type Int = i32;

impl Convert<Int> for Value {
    type From = Rust;
    type To = Ruby;

    fn convert(interp: &Artichoke, value: Int) -> Self {
        Self::new(interp, unsafe { sys::mrb_sys_fixnum_value(value) })
    }
}

impl Convert<u8> for Value {
    type From = Rust;
    type To = Ruby;

    fn convert(interp: &Artichoke, value: u8) -> Self {
        Self::convert(interp, Int::from(value))
    }
}

impl Convert<u16> for Value {
    type From = Rust;
    type To = Ruby;

    fn convert(interp: &Artichoke, value: u16) -> Self {
        Self::convert(interp, Int::from(value))
    }
}

impl Convert<u32> for Value {
    type From = Rust;
    type To = Ruby;

    fn convert(interp: &Artichoke, value: u32) -> Self {
        Self::convert(interp, Int::from(value))
    }
}

impl Convert<i8> for Value {
    type From = Rust;
    type To = Ruby;

    fn convert(interp: &Artichoke, value: i8) -> Self {
        Self::convert(interp, Int::from(value))
    }
}

impl Convert<i16> for Value {
    type From = Rust;
    type To = Ruby;

    fn convert(interp: &Artichoke, value: i16) -> Self {
        Self::convert(interp, Int::from(value))
    }
}

#[cfg(not(target_arch = "wasm32"))]
impl Convert<i32> for Value {
    type From = Rust;
    type To = Ruby;

    fn convert(interp: &Artichoke, value: i32) -> Self {
        Self::convert(interp, Int::from(value))
    }
}

impl TryConvert<Value> for Int {
    type From = Ruby;
    type To = Rust;

    unsafe fn try_convert(
        _interp: &Artichoke,
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

impl TryConvert<Value> for usize {
    type From = Ruby;
    type To = Rust;

    unsafe fn try_convert(
        interp: &Artichoke,
        value: Value,
    ) -> Result<Self, Error<Self::From, Self::To>> {
        if let Ok(result) = Int::try_convert(interp, value) {
            if let Ok(result) = Self::try_from(result) {
                return Ok(result);
            }
        }
        Err(Error {
            from: Ruby::Fixnum,
            to: Rust::UnsignedInt,
        })
    }
}

#[cfg(test)]
mod tests {
    use quickcheck_macros::quickcheck;

    use crate::convert::fixnum::Int;
    use crate::convert::{Convert, Error, TryConvert};
    use crate::eval::Eval;
    use crate::sys;
    use crate::value::types::{Ruby, Rust};
    use crate::value::Value;

    #[test]
    fn fail_convert() {
        let interp = crate::interpreter().expect("init");
        // get a mrb_value that can't be converted to a primitive type.
        let value = interp.eval("Object.new").expect("eval");
        let expected = Error {
            from: Ruby::Object,
            to: Rust::SignedInt,
        };
        let result = unsafe { Int::try_convert(&interp, value) }.map(|_| ());
        assert_eq!(result, Err(expected));
    }

    #[quickcheck]
    fn convert_to_fixnum(i: Int) -> bool {
        let interp = crate::interpreter().expect("init");
        let value = Value::convert(&interp, i);
        value.ruby_type() == Ruby::Fixnum
    }

    #[quickcheck]
    fn fixnum_with_value(i: Int) -> bool {
        let interp = crate::interpreter().expect("init");
        let value = Value::convert(&interp, i);
        let inner = value.inner();
        let cint = unsafe { sys::mrb_sys_fixnum_to_cint(inner) };
        cint == i
    }

    #[quickcheck]
    fn roundtrip(i: Int) -> bool {
        let interp = crate::interpreter().expect("init");
        let value = Value::convert(&interp, i);
        let value = unsafe { Int::try_convert(&interp, value) }.expect("convert");
        value == i
    }

    #[quickcheck]
    fn roundtrip_err(b: bool) -> bool {
        let interp = crate::interpreter().expect("init");
        let value = Value::convert(&interp, b);
        let value = unsafe { Int::try_convert(&interp, value) };
        let expected = Err(Error {
            from: Ruby::Bool,
            to: Rust::SignedInt,
        });
        value == expected
    }

    #[test]
    fn fixnum_to_usize() {
        let interp = crate::interpreter().expect("init");
        let value = Value::convert(&interp, 100);
        let value = unsafe { usize::try_convert(&interp, value) };
        let expected = Ok(100);
        assert_eq!(value, expected);
        let value = Value::convert(&interp, -100);
        let value = unsafe { usize::try_convert(&interp, value) };
        let expected = Err(Error {
            from: Ruby::Fixnum,
            to: Rust::UnsignedInt,
        });
        assert_eq!(value, expected);
    }
}
