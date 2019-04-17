use mruby_sys::*;

use crate::convert::{Error, RubyToRustError, RustToRubyError, TryRuby};
use crate::value::{Ruby, Rust, Value};

pub type Int = i64;

impl TryRuby<Int> for Value {
    type RubyConvertError = RustToRubyError;

    fn try_ruby_convert(_mrb: *mut mrb_state, value: Int) -> Result<Self, Self::RubyConvertError> {
        Ok(Self::new(unsafe { mrb_sys_fixnum_value(value) }))
    }
}

impl TryRuby<Value> for Int {
    type RubyConvertError = RubyToRustError;

    fn try_ruby_convert(
        _mrb: *mut mrb_state,
        value: Value,
    ) -> Result<Self, Self::RubyConvertError> {
        match value.ruby_type() {
            Ruby::Fixnum => {
                let inner = value.inner();
                let int = unsafe { mrb_sys_fixnum_to_cint(inner) };
                Ok(int)
            }
            type_tag => Err(Error {
                from: type_tag,
                to: Rust::SignedInt,
            }),
        }
    }
}

#[cfg(test)]
mod tests {
    use mruby_sys::*;
    use quickcheck_macros::quickcheck;

    use super::*;

    #[quickcheck]
    fn convert_to_fixnum(i: Int) -> bool {
        let mrb = unsafe { mrb_open() };
        let value = Value::try_ruby_convert(mrb, i).expect("convert");
        unsafe { mrb_close(mrb) };
        value.ruby_type() == Ruby::Fixnum
    }

    #[quickcheck]
    fn fixnum_with_value(i: Int) -> bool {
        let mrb = unsafe { mrb_open() };
        let value = Value::try_ruby_convert(mrb, i).expect("convert");
        let inner = value.inner();
        let cint = unsafe { mrb_sys_fixnum_to_cint(inner) };
        unsafe { mrb_close(mrb) };
        cint == i
    }

    #[quickcheck]
    fn roundtrip(i: Int) -> bool {
        let mrb = unsafe { mrb_open() };
        let value = Value::try_ruby_convert(mrb, i).expect("convert");
        let value = Int::try_ruby_convert(mrb, value).expect("convert");
        unsafe { mrb_close(mrb) };
        value == i
    }

    #[quickcheck]
    fn roundtrip_err(b: bool) -> bool {
        let mrb = unsafe { mrb_open() };
        let value = Value::try_ruby_convert(mrb, b).expect("convert");
        let value = Int::try_ruby_convert(mrb, value);
        unsafe { mrb_close(mrb) };
        let expected = Err(Error {
            from: Ruby::Bool,
            to: Rust::SignedInt,
        });
        value == expected
    }
}
