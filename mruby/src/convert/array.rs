use crate::convert::fixnum::Int;
use crate::convert::float::Float;

mrb_array_impl!(bool as bool);
mrb_array_impl!(Option<bool> as nilable_bool);
mrb_array_impl!(Vec<bool> as bool_array);
mrb_array_impl!(Vec<Option<bool>> as nilable_bool_array);

mrb_array_impl!(Vec<u8> as bytes);
mrb_array_impl!(Option<Vec<u8>> as nilable_bytes);
mrb_array_impl!(Vec<Vec<u8>> as bytes_array);
mrb_array_impl!(Vec<Option<Vec<u8>>> as nilable_bytes_array);

mrb_array_impl!(Float as float);
mrb_array_impl!(Option<Float> as nilable_float);
mrb_array_impl!(Vec<Float> as float_array);
mrb_array_impl!(Vec<Option<Float>> as nilable_float_array);

mrb_array_impl!(Int as fixnum);
mrb_array_impl!(Option<Int> as nilable_fixnum);
mrb_array_impl!(Vec<Int> as fixnum_array);
mrb_array_impl!(Vec<Option<Int>> as nilable_fixnum_array);

mrb_array_impl!(String as string);
mrb_array_impl!(Option<String> as nilable_string);
mrb_array_impl!(Vec<String> as string_array);
mrb_array_impl!(Vec<Option<String>> as nilable_string_array);
