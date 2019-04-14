use mruby_sys::*;
use std::convert::TryFrom;

use crate::value::{types, ConvertError, Value};

fn type_error(type_tag: types::Ruby) -> ConvertError<types::Ruby, types::Rust> {
    ConvertError {
        from: type_tag,
        to: types::Rust::UnsignedInt,
    }
}

fn convert_error() -> ConvertError<types::Rust, types::Ruby> {
    ConvertError {
        from: types::Rust::UnsignedInt,
        to: types::Ruby::Fixnum,
    }
}

impl TryFrom<u64> for Value {
    type Error = ConvertError<types::Rust, types::Ruby>;

    fn try_from(value: u64) -> Result<Self, Self::Error> {
        let value = i64::try_from(value).map_err(|_| convert_error())?;
        Ok(Self(unsafe { mrb_sys_fixnum_value(value) }))
    }
}

impl TryFrom<Value> for u64 {
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

impl TryFrom<Option<u64>> for Value {
    type Error = ConvertError<types::Rust, types::Ruby>;

    fn try_from(value: Option<u64>) -> Result<Self, Self::Error> {
        match value {
            Some(value) => {
                let value = i64::try_from(value).map_err(|_| convert_error())?;
                Ok(Self(unsafe { mrb_sys_fixnum_value(value) }))
            }
            None => Ok(Self(unsafe { mrb_sys_nil_value() })),
        }
    }
}

impl TryFrom<Value> for Option<u64> {
    type Error = ConvertError<types::Ruby, types::Rust>;

    fn try_from(value: Value) -> Result<Self, Self::Error> {
        match value.ruby_type() {
            type_tag @ types::Ruby::Fixnum => {
                let value = unsafe { mrb_sys_fixnum_to_cint(value.0) };
                u64::try_from(value)
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

    use crate::value::u64::*;
    use crate::value::*;

    #[test]
    fn value_from_zero() {
        unsafe {
            let value = Value::try_from(0_u64).expect("convert");
            assert_eq!(value.ruby_type(), types::Ruby::Fixnum);
            assert_eq!(mrb_sys_fixnum_to_cint(value.0), 0);
        }
    }

    #[test]
    fn err_from_max_u64() {
        let value = Value::try_from(u64::max_value());
        let expected = ConvertError {
            from: types::Rust::UnsignedInt,
            to: types::Ruby::Fixnum,
        };
        assert_eq!(value.err(), Some(expected));
    }

    #[test]
    fn value_from_min_u64() {
        unsafe {
            let value = Value::try_from(u64::min_value()).expect("convert");
            assert_eq!(mrb_sys_fixnum_to_cint(value.0), 0);
        }
    }

    #[test]
    fn u64_from_zero_value() {
        unsafe {
            let value = u64::try_from(Value(mrb_sys_fixnum_value(0))).expect("convert");
            assert_eq!(value, 0_u64);
        }
    }

    #[test]
    fn u64_from_min_value() {
        unsafe {
            let value = u64::try_from(Value(mrb_sys_fixnum_value(0))).expect("convert");
            assert_eq!(value, u64::min_value());
        }
    }

    #[test]
    fn err_from_nil_value() {
        unsafe {
            let err = u64::try_from(Value(mrb_sys_nil_value()));
            let expected = ConvertError {
                from: types::Ruby::Nil,
                to: types::Rust::UnsignedInt,
            };
            assert_eq!(err, Err(expected));
        }
    }

    #[test]
    fn err_from_bool_value() {
        unsafe {
            let err = u64::try_from(Value(mrb_sys_true_value()));
            let expected = ConvertError {
                from: types::Ruby::Bool,
                to: types::Rust::UnsignedInt,
            };
            assert_eq!(err, Err(expected));
        }
    }

    #[test]
    fn u64_from_max_fixnum() {
        unsafe {
            let value =
                u64::try_from(Value(mrb_sys_fixnum_value(i64::max_value()))).expect("convert");
            assert_eq!(value, 9_223_372_036_854_775_807);
        }
    }

    #[test]
    fn err_from_negative_fixnum() {
        unsafe {
            let err = u64::try_from(Value(mrb_sys_fixnum_value(-1)));
            let expected = ConvertError {
                from: types::Ruby::Fixnum,
                to: types::Rust::UnsignedInt,
            };
            assert_eq!(err, Err(expected));
        }
    }

    #[test]
    fn err_from_too_small_fixnum() {
        unsafe {
            let err = u64::try_from(Value(mrb_sys_fixnum_value(i64::min_value())));
            let expected = ConvertError {
                from: types::Ruby::Fixnum,
                to: types::Rust::UnsignedInt,
            };
            assert_eq!(err, Err(expected));
        }
    }

    #[test]
    fn value_from_some_zero() {
        unsafe {
            let value = Value::try_from(Some(0_u64)).expect("convert");
            assert_eq!(value.ruby_type(), types::Ruby::Fixnum);
            assert_eq!(mrb_sys_fixnum_to_cint(value.0), 0);
        }
    }

    #[test]
    fn value_from_some_max_u64() {
        let value = Value::try_from(Some(u64::max_value()));
        let expected = ConvertError {
            from: types::Rust::UnsignedInt,
            to: types::Ruby::Fixnum,
        };
        assert_eq!(value.err(), Some(expected));
    }

    #[test]
    fn value_from_some_min_u64() {
        unsafe {
            let value = Value::try_from(Some(u64::min_value())).expect("convert");
            assert_eq!(value.ruby_type(), types::Ruby::Fixnum);
            assert_eq!(mrb_sys_fixnum_to_cint(value.0), 0);
        }
    }

    #[test]
    fn value_from_none() {
        unsafe {
            let value = Value::try_from(None as Option<u64>).expect("convert");
            assert_eq!(value.ruby_type(), types::Ruby::Nil);
            assert_eq!(mrb_sys_value_is_nil(value.0), true);
        }
    }

    #[test]
    fn option_some_u64_from_zero() {
        unsafe {
            let value = Option::<u64>::try_from(Value(mrb_sys_fixnum_value(0))).expect("convert");
            assert_eq!(value, Some(0_u64));
        }
    }

    #[test]
    fn option_some_u64_from_min_u64() {
        unsafe {
            let value = Option::<u64>::try_from(Value(mrb_sys_fixnum_value(0))).expect("convert");
            assert_eq!(value, Some(u64::min_value()));
        }
    }

    #[test]
    fn option_none_from_nil_value() {
        unsafe {
            let value = Option::<u64>::try_from(Value(mrb_sys_nil_value())).expect("convert");
            assert_eq!(value, None);
        }
    }

    #[test]
    fn option_err_from_bool_value() {
        unsafe {
            let err = Option::<u64>::try_from(Value(mrb_sys_true_value()));
            let expected = ConvertError {
                from: types::Ruby::Bool,
                to: types::Rust::UnsignedInt,
            };
            assert_eq!(err, Err(expected));
        }
    }

    #[test]
    fn option_u64_from_max_fixnum() {
        unsafe {
            let value = Option::<u64>::try_from(Value(mrb_sys_fixnum_value(i64::max_value())))
                .expect("convert");
            assert_eq!(value, Some(9_223_372_036_854_775_807));
        }
    }

    #[test]
    fn option_err_from_negative_fixnum() {
        unsafe {
            let err = Option::<u64>::try_from(Value(mrb_sys_fixnum_value(-1)));
            let expected = ConvertError {
                from: types::Ruby::Fixnum,
                to: types::Rust::UnsignedInt,
            };
            assert_eq!(err, Err(expected));
        }
    }

    #[test]
    fn option_err_from_too_small_fixnum() {
        unsafe {
            let err = Option::<u64>::try_from(Value(mrb_sys_fixnum_value(i64::min_value())));
            let expected = ConvertError {
                from: types::Ruby::Fixnum,
                to: types::Rust::UnsignedInt,
            };
            assert_eq!(err, Err(expected));
        }
    }
}
