use crate::convert::fixnum::Int;
use crate::convert::float::Float;
use crate::convert::FromMrb;
use crate::interpreter::Mrb;
use crate::sys;
use crate::value::types::{Ruby, Rust};
use crate::value::Value;

mrb_nilable_impl!(bool as bool);
mrb_nilable_impl!(Vec<u8> as bytes);
mrb_nilable_impl!(Int as fixnum);
mrb_nilable_impl!(Float as float with eq = |a: Float, b: Float| {
    (a - b).abs() < std::f64::EPSILON
});
mrb_nilable_impl!(String as string);

// bail out implementation for mixed-type collections
impl FromMrb<Option<Value>> for Value {
    type From = Rust;
    type To = Ruby;

    fn from_mrb(interp: &Mrb, value: Option<Self>) -> Self {
        match value {
            Some(value) => value,
            None => Self::new(interp, unsafe { sys::mrb_sys_nil_value() }),
        }
    }
}

impl FromMrb<Value> for Option<Value> {
    type From = Ruby;
    type To = Rust;

    fn from_mrb(_interp: &Mrb, value: Value) -> Self {
        match value.ruby_type() {
            Ruby::Nil => None,
            _ => Some(value),
        }
    }
}
