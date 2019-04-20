use crate::convert::fixnum::Int;
use crate::convert::float::Float;

mrb_nilable_impl!(bool as nilable_bool);
mrb_nilable_impl!(Vec<u8> as nilable_byte_string);
mrb_nilable_impl!(Int as nilable_fixnum);
mrb_nilable_impl!(Float as nilable_float with eq = |a: Float, b: Float| (a - b).abs() < std::f64::EPSILON);
mrb_nilable_impl!(String as nilable_string);
