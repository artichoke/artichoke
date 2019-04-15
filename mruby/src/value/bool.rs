use mruby_sys::*;
use std::convert::TryFrom;

use crate::value::{types, ConvertError, TryValue, Value};

fn type_error(type_tag: types::Ruby) -> ConvertError<types::Ruby, types::Rust> {
    ConvertError {
        from: type_tag,
        to: types::Rust::Bool,
    }
}

impl TryValue for bool {
    type Error = ConvertError<types::Rust, types::Ruby>;

    fn try_value(&self, _mrb: *mut mrb_state) -> Result<Value, Self::Error> {
        Value::try_from(*self)
    }
}

impl TryFrom<bool> for Value {
    type Error = ConvertError<types::Rust, types::Ruby>;

    fn try_from(value: bool) -> Result<Self, Self::Error> {
        if value {
            Ok(Self(unsafe { mrb_sys_true_value() }))
        } else {
            Ok(Self(unsafe { mrb_sys_false_value() }))
        }
    }
}

impl TryFrom<Value> for bool {
    type Error = ConvertError<types::Ruby, types::Rust>;

    fn try_from(value: Value) -> Result<Self, Self::Error> {
        match value.ruby_type() {
            type_tag @ types::Ruby::Bool => {
                if unsafe { mrb_sys_value_is_true(value.0) } {
                    Ok(true)
                } else if unsafe { mrb_sys_value_is_false(value.0) } {
                    Ok(false)
                } else {
                    Err(type_error(type_tag))
                }
            }
            type_tag => Err(type_error(type_tag)),
        }
    }
}

impl TryFrom<Option<bool>> for Value {
    type Error = ConvertError<types::Rust, types::Ruby>;

    fn try_from(value: Option<bool>) -> Result<Self, Self::Error> {
        match value {
            Some(value) => Value::try_from(value),
            None => Ok(Self(unsafe { mrb_sys_nil_value() })),
        }
    }
}

impl TryFrom<Value> for Option<bool> {
    type Error = ConvertError<types::Ruby, types::Rust>;

    fn try_from(value: Value) -> Result<Self, Self::Error> {
        match value.ruby_type() {
            types::Ruby::Nil => Ok(None),
            type_tag @ types::Ruby::Bool => {
                if unsafe { mrb_sys_value_is_true(value.0) } {
                    Ok(Some(true))
                } else if unsafe { mrb_sys_value_is_false(value.0) } {
                    Ok(Some(false))
                } else {
                    Err(type_error(type_tag))
                }
            }
            type_tag => Err(type_error(type_tag)),
        }
    }
}

#[cfg(test)]
mod tests {
    use mruby_sys::*;

    use crate::value::bool::*;
    use crate::value::*;

    #[test]
    fn value_from_true_bool() {
        unsafe {
            let value = Value::try_from(true).expect("convert");
            assert_eq!(value.ruby_type(), types::Ruby::Bool);
            assert_eq!(mrb_sys_value_is_nil(value.0), false);
            assert_eq!(mrb_sys_value_is_false(value.0), false);
            assert_eq!(mrb_sys_value_is_true(value.0), true);
        }
    }

    #[test]
    fn value_from_false_bool() {
        unsafe {
            let value = Value::try_from(false).expect("convert");
            assert_eq!(value.ruby_type(), types::Ruby::Bool);
            assert_eq!(mrb_sys_value_is_nil(value.0), false);
            assert_eq!(mrb_sys_value_is_false(value.0), true);
            assert_eq!(mrb_sys_value_is_true(value.0), false);
        }
    }

    #[test]
    fn bool_from_true_value() {
        unsafe {
            let value = bool::try_from(Value(mrb_sys_true_value())).expect("convert");
            assert_eq!(value, true);
        }
    }

    #[test]
    fn bool_from_false_value() {
        unsafe {
            let value = bool::try_from(Value(mrb_sys_false_value())).expect("convert");
            assert_eq!(value, false);
        }
    }

    #[test]
    fn err_from_nil_value() {
        unsafe {
            let err = bool::try_from(Value(mrb_sys_nil_value()));
            let expected = ConvertError {
                from: types::Ruby::Nil,
                to: types::Rust::Bool,
            };
            assert_eq!(err, Err(expected));
        }
    }

    #[test]
    fn err_from_fixnum_value() {
        unsafe {
            let err = bool::try_from(Value(mrb_sys_fixnum_value(17)));
            let expected = ConvertError {
                from: types::Ruby::Fixnum,
                to: types::Rust::Bool,
            };
            assert_eq!(err, Err(expected));
        }
    }

    #[test]
    fn value_from_some_true_bool() {
        unsafe {
            let value = Value::try_from(Some(true)).expect("convert");
            assert_eq!(value.ruby_type(), types::Ruby::Bool);
            assert_eq!(mrb_sys_value_is_nil(value.0), false);
            assert_eq!(mrb_sys_value_is_false(value.0), false);
            assert_eq!(mrb_sys_value_is_true(value.0), true);
        }
    }

    #[test]
    fn value_from_some_false_bool() {
        unsafe {
            let value = Value::try_from(Some(false)).expect("convert");
            assert_eq!(value.ruby_type(), types::Ruby::Bool);
            assert_eq!(mrb_sys_value_is_nil(value.0), false);
            assert_eq!(mrb_sys_value_is_false(value.0), true);
            assert_eq!(mrb_sys_value_is_true(value.0), false);
        }
    }

    #[test]
    fn value_from_none() {
        unsafe {
            let value = Value::try_from(None as Option<bool>).expect("convert");
            assert_eq!(value.ruby_type(), types::Ruby::Nil);
            assert_eq!(mrb_sys_value_is_nil(value.0), true);
            assert_eq!(mrb_sys_value_is_false(value.0), false);
            assert_eq!(mrb_sys_value_is_true(value.0), false);
        }
    }

    #[test]
    fn option_some_bool_from_true_value() {
        unsafe {
            let value = Option::<bool>::try_from(Value(mrb_sys_true_value())).expect("convert");
            assert_eq!(value, Some(true));
        }
    }

    #[test]
    fn option_some_bool_from_false_value() {
        unsafe {
            let value = Option::<bool>::try_from(Value(mrb_sys_false_value())).expect("convert");
            assert_eq!(value, Some(false));
        }
    }

    #[test]
    fn option_none_from_nil_value() {
        unsafe {
            let value = Option::<bool>::try_from(Value(mrb_sys_nil_value())).expect("convert");
            assert_eq!(value, None);
        }
    }

    #[test]
    fn option_err_from_fixnum_value() {
        unsafe {
            let err = Option::<bool>::try_from(Value(mrb_sys_fixnum_value(17)));
            let expected = ConvertError {
                from: types::Ruby::Fixnum,
                to: types::Rust::Bool,
            };
            assert_eq!(err, Err(expected));
        }
    }
}
