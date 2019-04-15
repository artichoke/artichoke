use mruby_sys::*;
use std::convert::TryFrom;

use crate::value::{types, ConvertError, Value};

fn type_error(type_tag: types::Ruby) -> ConvertError<types::Ruby, types::Rust> {
    ConvertError {
        from: type_tag,
        to: types::Rust::SignedInt,
    }
}

impl TryFrom<i64> for Value {
    type Error = ConvertError<types::Rust, types::Ruby>;

    fn try_from(value: i64) -> Result<Self, Self::Error> {
        Ok(Self(unsafe { mrb_sys_fixnum_value(value) }))
    }
}

impl TryFrom<Value> for i64 {
    type Error = ConvertError<types::Ruby, types::Rust>;

    fn try_from(value: Value) -> Result<Self, Self::Error> {
        match value.ruby_type() {
            type_tag @ types::Ruby::Fixnum => {
                let value = unsafe { mrb_sys_fixnum_to_cint(value.0) };
                Self::try_from(value).map_err(|_| type_error(type_tag))
            }
            type_tag => Err(type_error(type_tag)),
        }
    }
}

impl TryFrom<Option<i64>> for Value {
    type Error = ConvertError<types::Rust, types::Ruby>;

    fn try_from(value: Option<i64>) -> Result<Self, Self::Error> {
        match value {
            Some(value) => Ok(Self(unsafe { mrb_sys_fixnum_value(value) })),
            None => Ok(Self(unsafe { mrb_sys_nil_value() })),
        }
    }
}

impl TryFrom<Value> for Option<i64> {
    type Error = ConvertError<types::Ruby, types::Rust>;

    fn try_from(value: Value) -> Result<Self, Self::Error> {
        match value.ruby_type() {
            type_tag @ types::Ruby::Fixnum => {
                let value = unsafe { mrb_sys_fixnum_to_cint(value.0) };
                i64::try_from(value)
                    .map(Some)
                    .map_err(|_| type_error(type_tag))
            }
            types::Ruby::Nil => Ok(None),
            type_tag => Err(type_error(type_tag)),
        }
    }
}

#[cfg(test)]
mod tests {
    use mruby_sys::*;

    use crate::value::i64::*;
    use crate::value::*;

    #[test]
    fn value_from_zero() {
        unsafe {
            let value = Value::try_from(0_i64).expect("convert");
            assert_eq!(value.ruby_type(), types::Ruby::Fixnum);
            assert_eq!(mrb_sys_fixnum_to_cint(value.0), 0);
        }
    }

    #[test]
    fn value_from_max() {
        unsafe {
            let value = Value::try_from(i64::max_value()).expect("convert");
            assert_eq!(mrb_sys_fixnum_to_cint(value.0), 9_223_372_036_854_775_807);
        }
    }

    #[test]
    fn value_from_min() {
        unsafe {
            let value = Value::try_from(i64::min_value()).expect("convert");
            assert_eq!(mrb_sys_fixnum_to_cint(value.0), -9_223_372_036_854_775_808);
        }
    }

    #[test]
    fn i64_from_zero_value() {
        unsafe {
            let value = i64::try_from(Value(mrb_sys_fixnum_value(0))).expect("convert");
            assert_eq!(value, 0_i64);
        }
    }

    #[test]
    fn i64_from_max_value() {
        unsafe {
            let value = i64::try_from(Value(mrb_sys_fixnum_value(9_223_372_036_854_775_807)))
                .expect("convert");
            assert_eq!(value, i64::max_value());
        }
    }

    #[test]
    fn i64_from_min_value() {
        unsafe {
            let value = i64::try_from(Value(mrb_sys_fixnum_value(-9_223_372_036_854_775_808)))
                .expect("convert");
            assert_eq!(value, i64::min_value());
        }
    }

    #[test]
    fn err_from_nil_value() {
        unsafe {
            let err = i64::try_from(Value(mrb_sys_nil_value()));
            let expected = ConvertError {
                from: types::Ruby::Nil,
                to: types::Rust::SignedInt,
            };
            assert_eq!(err, Err(expected));
        }
    }

    #[test]
    fn err_from_bool_value() {
        unsafe {
            let err = i64::try_from(Value(mrb_sys_true_value()));
            let expected = ConvertError {
                from: types::Ruby::Bool,
                to: types::Rust::SignedInt,
            };
            assert_eq!(err, Err(expected));
        }
    }

    #[test]
    fn value_from_some_zero() {
        unsafe {
            let value = Value::try_from(Some(0_i64)).expect("convert");
            assert_eq!(value.ruby_type(), types::Ruby::Fixnum);
            assert_eq!(mrb_sys_fixnum_to_cint(value.0), 0);
        }
    }

    #[test]
    fn value_from_some_max_i64() {
        unsafe {
            let value = Value::try_from(Some(i64::max_value())).expect("convert");
            assert_eq!(value.ruby_type(), types::Ruby::Fixnum);
            assert_eq!(mrb_sys_fixnum_to_cint(value.0), 9_223_372_036_854_775_807);
        }
    }

    #[test]
    fn value_from_some_min_i64() {
        unsafe {
            let value = Value::try_from(Some(i64::min_value())).expect("convert");
            assert_eq!(value.ruby_type(), types::Ruby::Fixnum);
            assert_eq!(mrb_sys_fixnum_to_cint(value.0), -9_223_372_036_854_775_808);
        }
    }

    #[test]
    fn value_from_none() {
        unsafe {
            let value = Value::try_from(None as Option<i64>).expect("convert");
            assert_eq!(value.ruby_type(), types::Ruby::Nil);
            assert_eq!(mrb_sys_value_is_nil(value.0), true);
        }
    }

    #[test]
    fn option_some_i64_from_zero() {
        unsafe {
            let value = Option::<i64>::try_from(Value(mrb_sys_fixnum_value(0))).expect("convert");
            assert_eq!(value, Some(0_i64));
        }
    }

    #[test]
    fn option_some_i64_from_max_i64() {
        unsafe {
            let value =
                Option::<i64>::try_from(Value(mrb_sys_fixnum_value(9_223_372_036_854_775_807)))
                    .expect("convert");
            assert_eq!(value, Some(i64::max_value()));
        }
    }

    #[test]
    fn option_some_i64_from_min_i64() {
        unsafe {
            let value =
                Option::<i64>::try_from(Value(mrb_sys_fixnum_value(-9_223_372_036_854_775_808)))
                    .expect("convert");
            assert_eq!(value, Some(i64::min_value()));
        }
    }

    #[test]
    fn option_none_from_nil_value() {
        unsafe {
            let value = Option::<i64>::try_from(Value(mrb_sys_nil_value())).expect("convert");
            assert_eq!(value, None);
        }
    }

    #[test]
    fn option_err_from_bool_value() {
        unsafe {
            let err = Option::<i64>::try_from(Value(mrb_sys_true_value()));
            let expected = ConvertError {
                from: types::Ruby::Bool,
                to: types::Rust::SignedInt,
            };
            assert_eq!(err, Err(expected));
        }
    }
}
