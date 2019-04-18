use crate::convert::fixnum::Int;

mrb_nilable_impl!(bool as nilable_bool);
mrb_nilable_impl!(Vec<u8> as nilable_byte_string);
// TODO: nilable float
mrb_nilable_impl!(Int as nilable_fixnum);
mrb_nilable_impl!(String as nilable_string);
