use std::convert::TryFrom;
use std::ffi::CStr;
use std::slice;

use crate::convert::{Convert, TryConvert};
use crate::sys;
use crate::types::{Ruby, Rust};
use crate::value::Value;
use crate::{Artichoke, ArtichokeError};

impl Convert<Vec<u8>, Value> for Artichoke {
    fn convert(&self, value: Vec<u8>) -> Value {
        self.convert(value.as_slice())
    }
}

impl Convert<&[u8], Value> for Artichoke {
    fn convert(&self, value: &[u8]) -> Value {
        let mrb = self.0.borrow().mrb;
        // Ruby strings contain raw bytes, so we can convert from a &[u8] to a
        // `char *` and `size_t`.
        let raw = value.as_ptr() as *const i8;
        let len = value.len();
        // `mrb_str_new` copies the `char *` to the mruby heap so we do not have
        // to worry about the lifetime of the slice passed into this converter.
        Value::new(self, unsafe { sys::mrb_str_new(mrb, raw, len) })
    }
}

impl TryConvert<Value, Vec<u8>> for Artichoke {
    fn try_convert(&self, value: Value) -> Result<Vec<u8>, ArtichokeError> {
        TryConvert::<_, &[u8]>::try_convert(self, value).map(<[_]>::to_vec)
    }
}

impl<'a> TryConvert<Value, &'a [u8]> for Artichoke {
    fn try_convert(&self, value: Value) -> Result<&'a [u8], ArtichokeError> {
        let mrb = self.0.borrow().mrb;
        match value.ruby_type() {
            Ruby::Symbol => {
                // mruby does not expose an API to get the raw byte contents of a
                // `Symbol`. For non-literal symbols and non-ASCII symbols,
                // `sys::mrb_sys_symbol_name` round trips through a `String`
                // `mrb_value` to turn a `char *` wrapped in quotes into a
                // `char *`.
                let bytes = unsafe { sys::mrb_sys_symbol_name(mrb, value.inner()) };
                let slice = unsafe { CStr::from_ptr(bytes) };
                Ok(slice.to_bytes())
            }
            Ruby::String => {
                let bytes = value.inner();
                let raw = unsafe { sys::mrb_string_value_ptr(mrb, bytes) as *const u8 };
                let len = unsafe { sys::mrb_string_value_len(mrb, bytes) };
                let len = usize::try_from(len).map_err(|_| ArtichokeError::ConvertToRust {
                    from: Ruby::String,
                    to: Rust::Bytes,
                })?;
                // We can return a borrowed slice because the memory is stored
                // on the mruby heap. As long as `value` is reachable, this
                // slice points to valid memory.
                let slice = unsafe { slice::from_raw_parts(raw, len) };
                Ok(slice)
            }
            type_tag => Err(ArtichokeError::ConvertToRust {
                from: type_tag,
                to: Rust::Bytes,
            }),
        }
    }
}

#[cfg(test)]
// Convert<Vec<u8>> is implemented in terms of Convert<&[u8]> so only implement
// the tests for Vec<u8> to exercise both code paths.
mod tests {
    use quickcheck_macros::quickcheck;
    use std::convert::TryFrom;

    use crate::test::prelude::*;

    #[test]
    fn fail_convert() {
        let interp = crate::interpreter().expect("init");
        // get a mrb_value that can't be converted to a primitive type.
        let value = interp.eval(b"Object.new").expect("eval");
        let expected = Err(ArtichokeError::ConvertToRust {
            from: Ruby::Object,
            to: Rust::Bytes,
        });
        let result = value.try_into::<Vec<u8>>();
        assert_eq!(result, expected);
    }

    #[allow(clippy::needless_pass_by_value)]
    #[quickcheck]
    fn convert_to_vec(v: Vec<u8>) -> bool {
        let interp = crate::interpreter().expect("init");
        let value = interp.convert(v.clone());
        value.ruby_type() == Ruby::String
    }

    #[allow(clippy::needless_pass_by_value)]
    #[quickcheck]
    fn vec_with_value(v: Vec<u8>) -> bool {
        let interp = crate::interpreter().expect("init");
        let mrb = interp.0.borrow().mrb;
        let value = interp.convert(v.clone());
        let inner = value.inner();
        let len = unsafe { sys::mrb_string_value_len(mrb, inner) };
        let len = usize::try_from(len).expect("usize");
        v.len() == len
    }

    #[allow(clippy::needless_pass_by_value)]
    #[quickcheck]
    fn roundtrip(v: Vec<u8>) -> bool {
        let interp = crate::interpreter().expect("init");
        let value = interp.convert(v.clone());
        let value = value.try_into::<Vec<u8>>().expect("convert");
        value == v
    }

    #[quickcheck]
    fn roundtrip_err(b: bool) -> bool {
        let interp = crate::interpreter().expect("init");
        let value = interp.convert(b);
        let value = value.try_into::<Vec<u8>>();
        let expected = Err(ArtichokeError::ConvertToRust {
            from: Ruby::Bool,
            to: Rust::Bytes,
        });
        value == expected
    }
}
