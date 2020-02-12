use std::convert::TryFrom;
use std::ffi::CStr;
use std::slice;

use crate::convert::UnboxRubyError;
use crate::exception::Exception;
use crate::sys;
use crate::types::{Ruby, Rust};
use crate::value::Value;
use crate::{Artichoke, ConvertMut, TryConvert};

impl ConvertMut<Vec<u8>, Value> for Artichoke {
    fn convert_mut(&mut self, value: Vec<u8>) -> Value {
        self.convert_mut(value.as_slice())
    }
}

impl ConvertMut<&[u8], Value> for Artichoke {
    fn convert_mut(&mut self, value: &[u8]) -> Value {
        let mrb = self.0.borrow().mrb;
        // Ruby strings contain raw bytes, so we can convert from a &[u8] to a
        // `char *` and `size_t`.
        let raw = value.as_ptr() as *const i8;
        let len = value.len();
        // `mrb_str_new` copies the `char *` to the mruby heap so we do not have
        // to worry about the lifetime of the slice passed into this converter.
        let string = unsafe { sys::mrb_str_new(mrb, raw, len) };
        Value::new(self, string)
    }
}

impl TryConvert<Value, Vec<u8>> for Artichoke {
    type Error = Exception;

    fn try_convert(&self, value: Value) -> Result<Vec<u8>, Self::Error> {
        TryConvert::<_, &[u8]>::try_convert(self, value).map(<[_]>::to_vec)
    }
}

impl<'a> TryConvert<Value, &'a [u8]> for Artichoke {
    type Error = Exception;

    fn try_convert(&self, value: Value) -> Result<&'a [u8], Self::Error> {
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
                let len =
                    usize::try_from(len).map_err(|_| UnboxRubyError::new(&value, Rust::Bytes))?;
                // We can return a borrowed slice because the memory is stored
                // on the mruby heap. As long as `value` is reachable, this
                // slice points to valid memory.
                let slice = unsafe { slice::from_raw_parts(raw, len) };
                Ok(slice)
            }
            _ => Err(Exception::from(UnboxRubyError::new(&value, Rust::Bytes))),
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
        let mut interp = crate::interpreter().expect("init");
        // get a mrb_value that can't be converted to a primitive type.
        let value = interp.eval(b"Object.new").expect("eval");
        let result = value.try_into::<Vec<u8>>();
        assert!(result.is_err());
    }

    #[allow(clippy::needless_pass_by_value)]
    #[quickcheck]
    fn convert_to_vec(v: Vec<u8>) -> bool {
        let mut interp = crate::interpreter().expect("init");
        let value = interp.convert_mut(v.clone());
        value.ruby_type() == Ruby::String
    }

    #[allow(clippy::needless_pass_by_value)]
    #[quickcheck]
    fn vec_with_value(v: Vec<u8>) -> bool {
        let mut interp = crate::interpreter().expect("init");
        let mrb = interp.0.borrow().mrb;
        let value = interp.convert_mut(v.clone());
        let inner = value.inner();
        let len = unsafe { sys::mrb_string_value_len(mrb, inner) };
        let len = usize::try_from(len).expect("usize");
        v.len() == len
    }

    #[allow(clippy::needless_pass_by_value)]
    #[quickcheck]
    fn roundtrip(v: Vec<u8>) -> bool {
        let mut interp = crate::interpreter().expect("init");
        let value = interp.convert_mut(v.clone());
        let value = value.try_into::<Vec<u8>>().expect("convert");
        value == v
    }

    #[quickcheck]
    fn roundtrip_err(b: bool) -> bool {
        let interp = crate::interpreter().expect("init");
        let value = interp.convert(b);
        let value = value.try_into::<Vec<u8>>();
        value.is_err()
    }
}
