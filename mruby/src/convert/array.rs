use mruby_sys::*;
use std::convert::TryFrom;

use crate::convert::fixnum::Int;
use crate::convert::{Error, RubyToRustError, RustToRubyError, TryRuby};
use crate::value::{Ruby, Rust, Value};

impl TryRuby<Vec<Int>> for Value {
    type RubyConvertError = RustToRubyError;

    fn try_ruby_convert(
        mrb: *mut mrb_state,
        value: Vec<Int>,
    ) -> Result<Self, Self::RubyConvertError> {
        let size = i64::try_from(value.len()).map_err(|_| Error {
            from: Rust::Vec,
            to: Ruby::Array,
        })?;
        let array = unsafe { mrb_ary_new_capa(mrb, size) };
        for (i, item) in value.iter().enumerate() {
            let idx = i64::try_from(i).map_err(|_| Error {
                from: Rust::Vec,
                to: Ruby::Array,
            })?;
            let ary_item = Self::try_ruby_convert(mrb, *item)?;
            let inner = ary_item.inner();
            unsafe { mrb_ary_set(mrb, array, idx, inner) };
        }
        Ok(Self::new(array))
    }
}

impl TryRuby<Value> for Vec<Int> {
    type RubyConvertError = RubyToRustError;

    fn try_ruby_convert(mrb: *mut mrb_state, value: Value) -> Result<Self, Self::RubyConvertError> {
        match value.ruby_type() {
            Ruby::Array => {
                let inner = value.inner();
                let len = unsafe { mrb_sys_ary_len(inner) };
                let cap = usize::try_from(len).map_err(|_| Error {
                    from: Ruby::Array,
                    to: Rust::Vec,
                })?;
                let mut vec = Self::with_capacity(cap);
                for i in 0..cap {
                    let idx = i64::try_from(i).map_err(|_| Error {
                        from: Ruby::Array,
                        to: Rust::Vec,
                    })?;
                    let item = Value::new(unsafe { mrb_ary_ref(mrb, inner, idx) });
                    vec.push(Int::try_ruby_convert(mrb, item)?);
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

#[cfg(test)]
mod tests {
    use mruby_sys::*;

    use super::*;

    #[test]
    fn try_value_from_vec() {
        unsafe {
            let mrb = mrb_open();

            let value = vec![-100, 0, 100];
            let value = Value::try_ruby_convert(mrb, value).expect("convert");
            let to_s = value.to_s(mrb);
            assert_eq!(value.ruby_type(), Ruby::Array);
            assert_eq!(&to_s, "[-100, 0, 100]");

            mrb_close(mrb);
        }
    }

    #[test]
    fn try_value_from_empty_vec() {
        unsafe {
            let mrb = mrb_open();

            let value = vec![];
            let value = Value::try_ruby_convert(mrb, value).expect("convert");
            let to_s = value.to_s(mrb);
            assert_eq!(value.ruby_type(), Ruby::Array);
            assert_eq!(&to_s, "[]");

            mrb_close(mrb);
        }
    }

    #[test]
    fn vec_from_value() {
        unsafe {
            let mrb = mrb_open();
            let context = mrbc_context_new(mrb);

            let code = "[-100, 0, 100]";
            let value = mrb_load_nstring_cxt(mrb, code.as_ptr() as *const i8, code.len(), context);
            let vec = <Vec<i64>>::try_ruby_convert(mrb, Value::new(value)).expect("convert");
            assert_eq!(vec, vec![-100, 0, 100]);

            mrb_close(mrb);
        }
    }

    #[test]
    fn empty_vec_from_value() {
        unsafe {
            let mrb = mrb_open();
            let context = mrbc_context_new(mrb);

            let code = "[]";
            let value = mrb_load_nstring_cxt(mrb, code.as_ptr() as *const i8, code.len(), context);
            let vec = <Vec<i64>>::try_ruby_convert(mrb, Value::new(value)).expect("convert");
            assert_eq!(vec, vec![]);

            mrb_close(mrb);
        }
    }
}
