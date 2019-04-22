use mruby_sys::*;
use std::convert::TryFrom;

use crate::convert::fixnum::Int;
use crate::convert::float::Float;
use crate::convert::Error;
use crate::interpreter::MrbApi;
use crate::{Ruby, Rust, TryFromMrb, Value};

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

// bail out implementation for mixed-type collections
impl TryFromMrb<Vec<Value>> for Value {
    type From = Rust;
    type To = Ruby;

    unsafe fn try_from_mrb(
        api: &MrbApi,
        value: Vec<Self>,
    ) -> Result<Self, Error<Self::From, Self::To>> {
        let size = Int::try_from(value.len()).map_err(|_| Error {
            from: Rust::Vec,
            to: Ruby::Array,
        })?;
        let array = mrb_ary_new_capa(api.mrb(), size);
        for (i, item) in value.into_iter().enumerate() {
            let idx = Int::try_from(i).map_err(|_| Error {
                from: Rust::Vec,
                to: Ruby::Array,
            })?;
            let inner = item.inner();
            mrb_ary_set(api.mrb(), array, idx, inner);
        }
        Ok(Self::new(array))
    }
}

impl TryFromMrb<Value> for Vec<Value> {
    type From = Ruby;
    type To = Rust;

    unsafe fn try_from_mrb(
        api: &MrbApi,
        value: Value,
    ) -> Result<Self, Error<Self::From, Self::To>> {
        match value.ruby_type() {
            Ruby::Array => {
                let inner = value.inner();
                let len = mrb_sys_ary_len(inner);
                let cap = usize::try_from(len).map_err(|_| Error {
                    from: Ruby::Array,
                    to: Rust::Vec,
                })?;
                let mut vec = Self::with_capacity(cap);
                for i in 0..cap {
                    let idx = Int::try_from(i).map_err(|_| Error {
                        from: Ruby::Array,
                        to: Rust::Vec,
                    })?;
                    let item = Value::new(mrb_ary_ref(api.mrb(), inner, idx));
                    vec.push(item);
                }
                Ok(vec)
            }
            type_tag => Err(Error {
                from: type_tag,
                to: Rust::Vec,
            }),
        }
    }
}
