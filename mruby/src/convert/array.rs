use crate::convert::fixnum::Int;

// primitive ruby types
mrb_vec_impl!(bool as array_of_bool);
mrb_vec_impl!(Vec<bool> as array_of_array_of_bool);

mrb_vec_impl!(Vec<u8> as array_of_bytes);
mrb_vec_impl!(Vec<Vec<u8>> as array_of_array_of_bytes);

mrb_vec_impl!(Int as array_of_fixnum);
mrb_vec_impl!(Vec<Int> as array_of_array_of_fixnum);

mrb_vec_impl!(String as array_of_string);
mrb_vec_impl!(Vec<String> as array_of_array_of_string);

// nilable types
//mrb_vec_impl!(Option<bool> as array_of_nilable_bool);
//mrb_vec_impl!(Vec<Option<bool>> as array_of_array_of_nilable_bool);

//mrb_vec_impl!(Option<Vec<u8>> as array_of_nilable_bytes);
//mrb_vec_impl!(Option<Vec<Vec<u8>>> as array_of_array_of_nilable_bytes);

mrb_vec_impl!(Option<Int> as array_of_nilable_fixnum);
mrb_vec_impl!(Vec<Option<Int>> as array_of_array_of_nilable_fixnum);

//mrb_vec_impl!(Option<String> as array_of_nilable_string);
//mrb_vec_impl!(Vec<Option<String>> as array_of_array_of_nilable_string);
