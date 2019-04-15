use mruby_sys::*;
use std::convert::TryFrom;

use crate::value::{types, ConvertError, TryValue, Value};

type Int = u16;

fn type_error(type_tag: types::Ruby) -> ConvertError<types::Ruby, types::Rust> {
    ConvertError {
        from: type_tag,
        to: types::Rust::UnsignedInt,
    }
}

impl TryValue for Int {
    type Error = ConvertError<types::Rust, types::Ruby>;

    fn try_value(&self, _mrb: *mut mrb_state) -> Result<Value, Self::Error> {
        Value::try_from(*self)
    }
}

impl TryFrom<Int> for Value {
    type Error = ConvertError<types::Rust, types::Ruby>;

    fn try_from(value: Int) -> Result<Self, Self::Error> {
        let value = i64::from(value);
        Ok(Self(unsafe { mrb_sys_fixnum_value(value) }))
    }
}

impl TryFrom<Value> for Int {
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

impl TryFrom<Option<Int>> for Value {
    type Error = ConvertError<types::Rust, types::Ruby>;

    fn try_from(value: Option<Int>) -> Result<Self, Self::Error> {
        match value {
            Some(value) => Self::try_from(value),
            None => Ok(Self(unsafe { mrb_sys_nil_value() })),
        }
    }
}

impl TryFrom<Value> for Option<Int> {
    type Error = ConvertError<types::Ruby, types::Rust>;

    fn try_from(value: Value) -> Result<Self, Self::Error> {
        match value.ruby_type() {
            types::Ruby::Nil => Ok(None),
            _ => Int::try_from(value).map(Some),
        }
    }
}

#[cfg(test)]
mod tests {
    use mruby_sys::*;

    use super::*;
    use crate::value::*;

    const MAX: i64 = Int::max_value() as i64;
    const MIN: i64 = Int::min_value() as i64;
    const ZERO: Int = 0;

    #[test]
    fn try_value() {
        unsafe {
            let mrb = mrb_open();

            let value = ZERO.try_value(mrb).expect("convert");
            assert_eq!(value.ruby_type(), types::Ruby::Fixnum);
            assert_eq!(mrb_sys_fixnum_to_cint(value.0), 0);

            let value = Int::max_value().try_value(mrb).expect("convert");
            assert_eq!(value.ruby_type(), types::Ruby::Fixnum);
            assert_eq!(mrb_sys_fixnum_to_cint(value.0), MAX);

            let value = Int::min_value().try_value(mrb).expect("convert");
            assert_eq!(value.ruby_type(), types::Ruby::Fixnum);
            assert_eq!(mrb_sys_fixnum_to_cint(value.0), MIN);

            mrb_close(mrb);
        }
    }

    #[test]
    fn value_from_zero() {
        unsafe {
            let value = Value::try_from(ZERO).expect("convert");
            assert_eq!(value.ruby_type(), types::Ruby::Fixnum);
            assert_eq!(mrb_sys_fixnum_to_cint(value.0), 0);
        }
    }

    #[test]
    fn value_from_max() {
        unsafe {
            let value = Value::try_from(Int::max_value()).expect("convert");
            assert_eq!(mrb_sys_fixnum_to_cint(value.0), MAX);
        }
    }

    #[test]
    fn value_from_min() {
        unsafe {
            let value = Value::try_from(Int::min_value()).expect("convert");
            assert_eq!(mrb_sys_fixnum_to_cint(value.0), MIN);
        }
    }

    #[test]
    fn int_from_zero_value() {
        unsafe {
            let value = Int::try_from(Value(mrb_sys_fixnum_value(0))).expect("convert");
            assert_eq!(value, ZERO);
        }
    }

    #[test]
    fn int_from_max_value() {
        unsafe {
            let value = Int::try_from(Value(mrb_sys_fixnum_value(MAX))).expect("convert");
            assert_eq!(value, Int::max_value());
        }
    }

    #[test]
    fn int_from_min_value() {
        unsafe {
            let value = Int::try_from(Value(mrb_sys_fixnum_value(MIN))).expect("convert");
            assert_eq!(value, Int::min_value());
        }
    }

    #[test]
    fn err_from_nil_value() {
        unsafe {
            let err = Int::try_from(Value(mrb_sys_nil_value()));
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
            let err = Int::try_from(Value(mrb_sys_true_value()));
            let expected = ConvertError {
                from: types::Ruby::Bool,
                to: types::Rust::UnsignedInt,
            };
            assert_eq!(err, Err(expected));
        }
    }

    #[test]
    fn err_from_too_large_fixnum_value() {
        unsafe {
            let err = Int::try_from(Value(mrb_sys_fixnum_value(i64::max_value())));
            let expected = ConvertError {
                from: types::Ruby::Fixnum,
                to: types::Rust::UnsignedInt,
            };
            assert_eq!(err, Err(expected));
        }
    }

    #[test]
    fn err_from_negative_fixnum_value() {
        unsafe {
            let err = Int::try_from(Value(mrb_sys_fixnum_value(-1)));
            let expected = ConvertError {
                from: types::Ruby::Fixnum,
                to: types::Rust::UnsignedInt,
            };
            assert_eq!(err, Err(expected));
        }
    }

    #[test]
    fn err_from_too_small_fixnum_value() {
        unsafe {
            let err = Int::try_from(Value(mrb_sys_fixnum_value(i64::min_value())));
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
            let value = Value::try_from(Some(ZERO)).expect("convert");
            assert_eq!(value.ruby_type(), types::Ruby::Fixnum);
            assert_eq!(mrb_sys_fixnum_to_cint(value.0), 0);
        }
    }

    #[test]
    fn value_from_some_max() {
        unsafe {
            let value = Value::try_from(Some(Int::max_value())).expect("convert");
            assert_eq!(value.ruby_type(), types::Ruby::Fixnum);
            assert_eq!(mrb_sys_fixnum_to_cint(value.0), MAX);
        }
    }

    #[test]
    fn value_from_some_min() {
        unsafe {
            let value = Value::try_from(Some(Int::min_value())).expect("convert");
            assert_eq!(value.ruby_type(), types::Ruby::Fixnum);
            assert_eq!(mrb_sys_fixnum_to_cint(value.0), MIN);
        }
    }

    #[test]
    fn value_from_none() {
        unsafe {
            let value = Value::try_from(None as Option<Int>).expect("convert");
            assert_eq!(value.ruby_type(), types::Ruby::Nil);
            assert_eq!(mrb_sys_value_is_nil(value.0), true);
        }
    }

    #[test]
    fn some_int_from_zero_value() {
        unsafe {
            let value = Option::<Int>::try_from(Value(mrb_sys_fixnum_value(0))).expect("convert");
            assert_eq!(value, Some(ZERO));
        }
    }

    #[test]
    fn some_int_from_max_value() {
        unsafe {
            let value = Option::<Int>::try_from(Value(mrb_sys_fixnum_value(MAX))).expect("convert");
            assert_eq!(value, Some(Int::max_value()));
        }
    }

    #[test]
    fn some_int_from_min_value() {
        unsafe {
            let value = Option::<Int>::try_from(Value(mrb_sys_fixnum_value(MIN))).expect("convert");
            assert_eq!(value, Some(Int::min_value()));
        }
    }

    #[test]
    fn none_from_nil_value() {
        unsafe {
            let value = Option::<Int>::try_from(Value(mrb_sys_nil_value())).expect("convert");
            assert_eq!(value, None);
        }
    }

    #[test]
    fn option_err_from_bool_value() {
        unsafe {
            let err = Option::<Int>::try_from(Value(mrb_sys_true_value()));
            let expected = ConvertError {
                from: types::Ruby::Bool,
                to: types::Rust::UnsignedInt,
            };
            assert_eq!(err, Err(expected));
        }
    }

    #[test]
    fn option_err_from_too_large_fixnum_value() {
        unsafe {
            let err = Option::<Int>::try_from(Value(mrb_sys_fixnum_value(i64::max_value())));
            let expected = ConvertError {
                from: types::Ruby::Fixnum,
                to: types::Rust::UnsignedInt,
            };
            assert_eq!(err, Err(expected));
        }
    }

    #[test]
    fn option_err_from_negative_fixnum_value() {
        unsafe {
            let err = Option::<Int>::try_from(Value(mrb_sys_fixnum_value(-1)));
            let expected = ConvertError {
                from: types::Ruby::Fixnum,
                to: types::Rust::UnsignedInt,
            };
            assert_eq!(err, Err(expected));
        }
    }

    #[test]
    fn option_err_from_too_small_fixnum_value() {
        unsafe {
            let err = Option::<Int>::try_from(Value(mrb_sys_fixnum_value(i64::min_value())));
            let expected = ConvertError {
                from: types::Ruby::Fixnum,
                to: types::Rust::UnsignedInt,
            };
            assert_eq!(err, Err(expected));
        }
    }
}
