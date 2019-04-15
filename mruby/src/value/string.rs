use mruby_sys::*;
use std::convert::TryFrom;
use std::ffi::{CStr, CString};

use crate::value::{types, ConvertError, TryValue, Value};

fn type_error(type_tag: types::Ruby) -> ConvertError<types::Ruby, types::Rust> {
    ConvertError {
        from: type_tag,
        to: types::Rust::String,
    }
}

fn convert_error() -> ConvertError<types::Rust, types::Ruby> {
    ConvertError {
        from: types::Rust::String,
        to: types::Ruby::String,
    }
}

impl TryValue for String {
    type Error = ConvertError<types::Rust, types::Ruby>;

    fn try_value(&self, mrb: *mut mrb_state) -> Result<Value, Self::Error> {
        unsafe {
            let s: &str = self;
            match CString::new(s) {
                Ok(cstr) => Ok(Value(mrb_str_new_cstr(mrb, cstr.as_ptr()))),
                Err(_) => Err(convert_error()),
            }
        }
    }
}

impl TryFrom<Value> for String {
    type Error = ConvertError<types::Ruby, types::Rust>;

    fn try_from(value: Value) -> Result<Self, Self::Error> {
        match value.ruby_type() {
            type_tag @ types::Ruby::String => unsafe {
                // TODO: handle null pointer
                let mrb = mrb_open();
                let mut value = value.0;
                let cstr = mrb_string_value_cstr(mrb, &mut value);
                let result = match CStr::from_ptr(cstr).to_str() {
                    Ok(string) => Ok(string.to_owned()),
                    Err(_) => Err(type_error(type_tag)),
                };
                mrb_close(mrb);
                result
            },
            type_tag => Err(type_error(type_tag)),
        }
    }
}

impl TryValue for Option<String> {
    type Error = ConvertError<types::Rust, types::Ruby>;

    fn try_value(&self, mrb: *mut mrb_state) -> Result<Value, Self::Error> {
        match self {
            Some(value) => Ok(value.try_value(mrb)?),
            None => Ok(Value(unsafe { mrb_sys_nil_value() })),
        }
    }
}

impl TryFrom<Value> for Option<String> {
    type Error = ConvertError<types::Ruby, types::Rust>;

    fn try_from(value: Value) -> Result<Self, Self::Error> {
        match value.ruby_type() {
            types::Ruby::Nil => Ok(None),
            _ => Ok(Some(String::try_from(value)?)),
        }
    }
}

#[cfg(test)]
mod tests {
    use mruby_sys::*;

    use crate::value::string::*;
    use crate::value::*;

    #[test]
    fn value_from_string() {
        unsafe {
            let mrb = mrb_open();
            let value = "foo".to_owned().try_value(mrb).expect("convert");
            assert_eq!(value.ruby_type(), types::Ruby::String);
            assert_eq!(mrb_sys_value_is_nil(value.0), false);
            assert_eq!(value.to_s(), "foo".to_owned());
            mrb_close(mrb);
        }
    }

    #[test]
    fn value_from_empty_string() {
        unsafe {
            let mrb = mrb_open();
            let value = "".to_owned().try_value(mrb).expect("convert");
            assert_eq!(value.ruby_type(), types::Ruby::String);
            assert_eq!(mrb_sys_value_is_nil(value.0), false);
            assert_eq!(value.to_s(), "".to_owned());
            mrb_close(mrb);
        }
    }

    #[test]
    fn string_from_string_value() {
        unsafe {
            let mrb = mrb_open();
            let cstr = "foo";
            let cstr_raw = CString::new(cstr).unwrap();
            let rb_value = mrb_str_new_cstr(mrb, cstr_raw.as_ptr());
            let value = String::try_from(Value(rb_value)).expect("convert");
            assert_eq!(value, "foo".to_owned());
        }
    }

    #[test]
    fn string_from_empty_string_value() {
        unsafe {
            let mrb = mrb_open();
            let cstr = "";
            let cstr_raw = CString::new(cstr).unwrap();
            let rb_value = mrb_str_new_cstr(mrb, cstr_raw.as_ptr());
            let value = String::try_from(Value(rb_value)).expect("convert");
            assert_eq!(value, "".to_owned());
        }
    }

    #[test]
    fn err_from_nil_value() {
        unsafe {
            let err = String::try_from(Value(mrb_sys_nil_value()));
            let expected = ConvertError {
                from: types::Ruby::Nil,
                to: types::Rust::String,
            };
            assert_eq!(err, Err(expected));
        }
    }

    #[test]
    fn err_from_fixnum_value() {
        unsafe {
            let err = String::try_from(Value(mrb_sys_fixnum_value(17)));
            let expected = ConvertError {
                from: types::Ruby::Fixnum,
                to: types::Rust::String,
            };
            assert_eq!(err, Err(expected));
        }
    }

    #[test]
    fn value_from_some_string() {
        unsafe {
            let mrb = mrb_open();
            let value = Some("foo".to_owned()).try_value(mrb, ).expect("convert");
            assert_eq!(value.ruby_type(), types::Ruby::String);
            assert_eq!(mrb_sys_value_is_nil(value.0), false);
            assert_eq!(value.to_s(), "foo".to_owned());
            mrb_close(mrb);
        }
    }

    #[test]
    fn value_from_some_empty_string() {
        unsafe {
            let mrb = mrb_open();
            let value = Some("".to_owned()).try_value(mrb).expect("convert");
            assert_eq!(value.ruby_type(), types::Ruby::String);
            assert_eq!(mrb_sys_value_is_nil(value.0), false);
            assert_eq!(value.to_s(), "".to_owned());
            mrb_close(mrb);
        }
    }

    #[test]
    fn value_from_none() {
        unsafe {
            let mrb = mrb_open();
            let value = (None as Option<String>).try_value(mrb).expect("convert");
            assert_eq!(value.ruby_type(), types::Ruby::Nil);
            assert_eq!(mrb_sys_value_is_nil(value.0), true);
            mrb_close(mrb);
        }
    }

    #[test]
    fn option_some_bool_from_string_value() {
        unsafe {
            let mrb = mrb_open();
            let cstr = "foo";
            let cstr_raw = CString::new(cstr).unwrap();
            let rb_value = mrb_str_new_cstr(mrb, cstr_raw.as_ptr());
            let value = Option::<String>::try_from(Value(rb_value)).expect("convert");
            assert_eq!(value, Some("foo".to_owned()));
        }
    }

    #[test]
    fn option_some_bool_from_empty_string_value() {
        unsafe {
            let mrb = mrb_open();
            let cstr = "";
            let cstr_raw = CString::new(cstr).unwrap();
            let rb_value = mrb_str_new_cstr(mrb, cstr_raw.as_ptr());
            let value = Option::<String>::try_from(Value(rb_value)).expect("convert");
            assert_eq!(value, Some("".to_owned()));
        }
    }

    #[test]
    fn option_none_from_nil_value() {
        unsafe {
            let value = Option::<String>::try_from(Value(mrb_sys_nil_value())).expect("convert");
            assert_eq!(value, None);
        }
    }

    #[test]
    fn option_err_from_fixnum_value() {
        unsafe {
            let err = Option::<String>::try_from(Value(mrb_sys_fixnum_value(17)));
            let expected = ConvertError {
                from: types::Ruby::Fixnum,
                to: types::Rust::String,
            };
            assert_eq!(err, Err(expected));
        }
    }
}
