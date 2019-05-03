use crate::convert::fixnum::Int;
use crate::convert::float::Float;
use crate::convert::Error;
use crate::interpreter::Mrb;
use crate::sys;
use crate::{Ruby, Rust, TryFromMrb, Value};

mrb_nilable_impl!(bool as bool);
mrb_nilable_impl!(Vec<u8> as bytes);
mrb_nilable_impl!(Int as fixnum);
mrb_nilable_impl!(Float as float with eq = |a: Float, b: Float| {
    (a - b).abs() < std::f64::EPSILON
});
mrb_nilable_impl!(String as string);

// bail out implementation for mixed-type collections
impl TryFromMrb<Option<Value>> for Value {
    type From = Rust;
    type To = Ruby;

    unsafe fn try_from_mrb(
        _mrb: &Mrb,
        value: Option<Self>,
    ) -> Result<Self, Error<Self::From, Self::To>> {
        match value {
            Some(value) => Ok(value),
            None => Ok(Self::new(sys::mrb_sys_nil_value())),
        }
    }
}

impl TryFromMrb<Value> for Option<Value> {
    type From = Ruby;
    type To = Rust;

    unsafe fn try_from_mrb(
        _mrb: &Mrb,
        value: Value,
    ) -> std::result::Result<Self, Error<Self::From, Self::To>> {
        match value.ruby_type() {
            Ruby::Nil => Ok(None),
            _ => Ok(Some(value)),
        }
    }
}
