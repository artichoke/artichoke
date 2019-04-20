use crate::convert::fixnum::Int;
use crate::convert::float::Float;

mrb_nilable_impl!(bool as bool);
mrb_nilable_impl!(Vec<u8> as bytes);
mrb_nilable_impl!(Int as fixnum);
mrb_nilable_impl!(Float as float with eq = |a: Float, b: Float| {
    (a - b).abs() < std::f64::EPSILON
});
mrb_nilable_impl!(String as string);
