use crate::convert::fixnum::Int;
use crate::convert::float::Float;

mrb_array_impl!(bool as array_of_bool);
mrb_array_impl!(Option<bool> as array_of_nilable_bool);
mrb_array_impl!(Vec<bool> as array_of_array_of_bool);
mrb_array_impl!(Vec<Option<bool>> as array_of_array_of_nilable_bool);

mrb_array_impl!(Vec<u8> as array_of_byte_strings);
mrb_array_impl!(Option<Vec<u8>> as array_of_nilable_byte_strings);
mrb_array_impl!(Vec<Vec<u8>> as array_of_array_of_byte_strings);
mrb_array_impl!(Vec<Option<Vec<u8>>> as array_of_array_of_nilable_byte_strings);

mrb_array_impl!(Float as array_of_float);
mrb_array_impl!(Option<Float> as array_of_nilable_float);
mrb_array_impl!(Vec<Float> as array_of_array_of_float);
mrb_array_impl!(Vec<Option<Float>> as array_of_array_of_nilable_float);

mrb_array_impl!(Int as array_of_fixnum);
mrb_array_impl!(Option<Int> as array_of_nilable_fixnum);
mrb_array_impl!(Vec<Int> as array_of_array_of_fixnum);
mrb_array_impl!(Vec<Option<Int>> as array_of_array_of_nilable_fixnum);

mrb_array_impl!(String as array_of_string);
mrb_array_impl!(Option<String> as array_of_nilable_string);
mrb_array_impl!(Vec<String> as array_of_array_of_string);
mrb_array_impl!(Vec<Option<String>> as array_of_array_of_nilable_string);
