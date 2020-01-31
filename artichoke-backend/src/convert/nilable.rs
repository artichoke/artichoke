//! Converters for nilable primitive Ruby types. Excludes collection types
//! Array and Hash.

use std::collections::HashMap;

use crate::convert::{Convert, TryConvert};
use crate::sys;
use crate::types::{Float, Int, Ruby};
use crate::value::Value;
use crate::{Artichoke, ArtichokeError};

// bail out implementation for mixed-type collections
impl Convert<Option<Value>, Value> for Artichoke {
    fn convert(&self, value: Option<Value>) -> Value {
        if let Some(value) = value {
            value
        } else {
            Value::new(self, unsafe { sys::mrb_sys_nil_value() })
        }
    }
}

impl Convert<Option<&Value>, Value> for Artichoke {
    fn convert(&self, value: Option<&Value>) -> Value {
        self.convert(value.cloned())
    }
}

impl Convert<Value, Option<Value>> for Artichoke {
    fn convert(&self, value: Value) -> Option<Value> {
        if let Ruby::Nil = value.ruby_type() {
            None
        } else {
            Some(value)
        }
    }
}

macro_rules! option_to_ruby {
    ($elem:ty) => {
        impl<'a> Convert<Option<$elem>, Value> for Artichoke {
            fn convert(&self, value: Option<$elem>) -> Value {
                if let Some(value) = value {
                    self.convert(value)
                } else {
                    Value::new(self, unsafe { sys::mrb_sys_nil_value() })
                }
            }
        }
    };
}

macro_rules! ruby_to_option {
    ($elem:ty) => {
        impl<'a> TryConvert<Value, Option<$elem>> for Artichoke {
            fn try_convert(&self, value: Value) -> Result<Option<$elem>, ArtichokeError> {
                if let Some(value) = self.convert(value) {
                    self.try_convert(value).map(Some)
                } else {
                    Ok(None)
                }
            }
        }
    };
}

// Primitives
option_to_ruby!(bool);
option_to_ruby!(Vec<u8>);
option_to_ruby!(&'a [u8]);
option_to_ruby!(Int);
option_to_ruby!(Float);
option_to_ruby!(String);
option_to_ruby!(&'a str);

// Array of primitives
option_to_ruby!(Vec<Value>);
option_to_ruby!(Vec<bool>);
option_to_ruby!(Vec<Vec<u8>>);
option_to_ruby!(Vec<&'a [u8]>);
option_to_ruby!(Vec<Int>);
option_to_ruby!(Vec<Float>);
option_to_ruby!(Vec<String>);
option_to_ruby!(Vec<&'a str>);

// Array of optional primitives
option_to_ruby!(Vec<Option<Value>>);
option_to_ruby!(Vec<Option<bool>>);
option_to_ruby!(Vec<Option<Vec<u8>>>);
option_to_ruby!(Vec<Option<Int>>);
option_to_ruby!(Vec<Option<Float>>);
option_to_ruby!(Vec<Option<String>>);
option_to_ruby!(Vec<Option<&'a str>>);

option_to_ruby!(HashMap<Vec<u8>, Option<Vec<u8>>>);

#[cfg(feature = "artichoke-all-converters")]
mod optional {
    use super::*;

    // Hash of primitive keys to values
    option_to_ruby!(HashMap<bool, Value>);
    option_to_ruby!(HashMap<bool, bool>);
    option_to_ruby!(HashMap<bool, Vec<u8>>);
    option_to_ruby!(HashMap<bool, Int>);
    option_to_ruby!(HashMap<bool, Float>);
    option_to_ruby!(HashMap<bool, String>);
    option_to_ruby!(HashMap<bool, &'a str>);
    option_to_ruby!(HashMap<Vec<u8>, Value>);
    option_to_ruby!(HashMap<Vec<u8>, bool>);
    option_to_ruby!(HashMap<Vec<u8>, Vec<u8>>);
    option_to_ruby!(HashMap<Vec<u8>, Int>);
    option_to_ruby!(HashMap<Vec<u8>, Float>);
    option_to_ruby!(HashMap<Vec<u8>, String>);
    option_to_ruby!(HashMap<Vec<u8>, &'a str>);
    option_to_ruby!(HashMap<Int, Value>);
    option_to_ruby!(HashMap<Int, bool>);
    option_to_ruby!(HashMap<Int, Vec<u8>>);
    option_to_ruby!(HashMap<Int, Int>);
    option_to_ruby!(HashMap<Int, Float>);
    option_to_ruby!(HashMap<Int, String>);
    option_to_ruby!(HashMap<Int, &'a str>);
    option_to_ruby!(HashMap<String, Value>);
    option_to_ruby!(HashMap<String, bool>);
    option_to_ruby!(HashMap<String, Vec<u8>>);
    option_to_ruby!(HashMap<String, Int>);
    option_to_ruby!(HashMap<String, Float>);
    option_to_ruby!(HashMap<String, String>);
    option_to_ruby!(HashMap<String, &'a str>);
    option_to_ruby!(HashMap<&'a str, Value>);
    option_to_ruby!(HashMap<&'a str, bool>);
    option_to_ruby!(HashMap<&'a str, Vec<u8>>);
    option_to_ruby!(HashMap<&'a str, Int>);
    option_to_ruby!(HashMap<&'a str, Float>);
    option_to_ruby!(HashMap<&'a str, String>);
    option_to_ruby!(HashMap<&'a str, &'a str>);

    // Hash of optional keys to values
    option_to_ruby!(HashMap<Option<bool>, Value>);
    option_to_ruby!(HashMap<Option<bool>, bool>);
    option_to_ruby!(HashMap<Option<bool>, Vec<u8>>);
    option_to_ruby!(HashMap<Option<bool>, Int>);
    option_to_ruby!(HashMap<Option<bool>, Float>);
    option_to_ruby!(HashMap<Option<bool>, String>);
    option_to_ruby!(HashMap<Option<bool>, &'a str>);
    option_to_ruby!(HashMap<Option<Vec<u8>>, Value>);
    option_to_ruby!(HashMap<Option<Vec<u8>>, bool>);
    option_to_ruby!(HashMap<Option<Vec<u8>>, Vec<u8>>);
    option_to_ruby!(HashMap<Option<Vec<u8>>, Int>);
    option_to_ruby!(HashMap<Option<Vec<u8>>, Float>);
    option_to_ruby!(HashMap<Option<Vec<u8>>, String>);
    option_to_ruby!(HashMap<Option<Vec<u8>>, &'a str>);
    option_to_ruby!(HashMap<Option<Int>, Value>);
    option_to_ruby!(HashMap<Option<Int>, bool>);
    option_to_ruby!(HashMap<Option<Int>, Vec<u8>>);
    option_to_ruby!(HashMap<Option<Int>, Int>);
    option_to_ruby!(HashMap<Option<Int>, Float>);
    option_to_ruby!(HashMap<Option<Int>, String>);
    option_to_ruby!(HashMap<Option<Int>, &'a str>);
    option_to_ruby!(HashMap<Option<String>, Value>);
    option_to_ruby!(HashMap<Option<String>, bool>);
    option_to_ruby!(HashMap<Option<String>, Vec<u8>>);
    option_to_ruby!(HashMap<Option<String>, Int>);
    option_to_ruby!(HashMap<Option<String>, Float>);
    option_to_ruby!(HashMap<Option<String>, String>);
    option_to_ruby!(HashMap<Option<String>, &'a str>);
    option_to_ruby!(HashMap<Option<&'a str>, Value>);
    option_to_ruby!(HashMap<Option<&'a str>, bool>);
    option_to_ruby!(HashMap<Option<&'a str>, Vec<u8>>);
    option_to_ruby!(HashMap<Option<&'a str>, Int>);
    option_to_ruby!(HashMap<Option<&'a str>, Float>);
    option_to_ruby!(HashMap<Option<&'a str>, String>);
    option_to_ruby!(HashMap<Option<&'a str>, &'a str>);

    // Hash of primitive keys to optional values
    option_to_ruby!(HashMap<bool, Option<Value>>);
    option_to_ruby!(HashMap<bool, Option<bool>>);
    option_to_ruby!(HashMap<bool, Option<Vec<u8>>>);
    option_to_ruby!(HashMap<bool, Option<Int>>);
    option_to_ruby!(HashMap<bool, Option<Float>>);
    option_to_ruby!(HashMap<bool, Option<String>>);
    option_to_ruby!(HashMap<bool, Option<&'a str>>);
    option_to_ruby!(HashMap<Vec<u8>, Option<Value>>);
    option_to_ruby!(HashMap<Vec<u8>, Option<bool>>);
    option_to_ruby!(HashMap<Vec<u8>, Option<Int>>);
    option_to_ruby!(HashMap<Vec<u8>, Option<Float>>);
    option_to_ruby!(HashMap<Vec<u8>, Option<String>>);
    option_to_ruby!(HashMap<Vec<u8>, Option<&'a str>>);
    option_to_ruby!(HashMap<Int, Option<Value>>);
    option_to_ruby!(HashMap<Int, Option<bool>>);
    option_to_ruby!(HashMap<Int, Option<Vec<u8>>>);
    option_to_ruby!(HashMap<Int, Option<Int>>);
    option_to_ruby!(HashMap<Int, Option<Float>>);
    option_to_ruby!(HashMap<Int, Option<String>>);
    option_to_ruby!(HashMap<Int, Option<&'a str>>);
    option_to_ruby!(HashMap<String, Option<Value>>);
    option_to_ruby!(HashMap<String, Option<bool>>);
    option_to_ruby!(HashMap<String, Option<Vec<u8>>>);
    option_to_ruby!(HashMap<String, Option<Int>>);
    option_to_ruby!(HashMap<String, Option<Float>>);
    option_to_ruby!(HashMap<String, Option<String>>);
    option_to_ruby!(HashMap<String, Option<&'a str>>);
    option_to_ruby!(HashMap<&'a str, Option<Value>>);
    option_to_ruby!(HashMap<&'a str, Option<bool>>);
    option_to_ruby!(HashMap<&'a str, Option<Vec<u8>>>);
    option_to_ruby!(HashMap<&'a str, Option<Int>>);
    option_to_ruby!(HashMap<&'a str, Option<Float>>);
    option_to_ruby!(HashMap<&'a str, Option<String>>);
    option_to_ruby!(HashMap<&'a str, Option<&'a str>>);

    // Hash of primitive optional keys to optional values
    option_to_ruby!(HashMap<Option<bool>, Option<Value>>);
    option_to_ruby!(HashMap<Option<bool>, Option<bool>>);
    option_to_ruby!(HashMap<Option<bool>, Option<Vec<u8>>>);
    option_to_ruby!(HashMap<Option<bool>, Option<Int>>);
    option_to_ruby!(HashMap<Option<bool>, Option<Float>>);
    option_to_ruby!(HashMap<Option<bool>, Option<String>>);
    option_to_ruby!(HashMap<Option<bool>, Option<&'a str>>);
    option_to_ruby!(HashMap<Option<Vec<u8>>, Option<Value>>);
    option_to_ruby!(HashMap<Option<Vec<u8>>, Option<bool>>);
    option_to_ruby!(HashMap<Option<Vec<u8>>, Option<Vec<u8>>>);
    option_to_ruby!(HashMap<Option<Vec<u8>>, Option<Int>>);
    option_to_ruby!(HashMap<Option<Vec<u8>>, Option<Float>>);
    option_to_ruby!(HashMap<Option<Vec<u8>>, Option<String>>);
    option_to_ruby!(HashMap<Option<Vec<u8>>, Option<&'a str>>);
    option_to_ruby!(HashMap<Option<Int>, Option<Value>>);
    option_to_ruby!(HashMap<Option<Int>, Option<bool>>);
    option_to_ruby!(HashMap<Option<Int>, Option<Vec<u8>>>);
    option_to_ruby!(HashMap<Option<Int>, Option<Int>>);
    option_to_ruby!(HashMap<Option<Int>, Option<Float>>);
    option_to_ruby!(HashMap<Option<Int>, Option<String>>);
    option_to_ruby!(HashMap<Option<Int>, Option<&'a str>>);
    option_to_ruby!(HashMap<Option<String>, Option<Value>>);
    option_to_ruby!(HashMap<Option<String>, Option<bool>>);
    option_to_ruby!(HashMap<Option<String>, Option<Vec<u8>>>);
    option_to_ruby!(HashMap<Option<String>, Option<Int>>);
    option_to_ruby!(HashMap<Option<String>, Option<Float>>);
    option_to_ruby!(HashMap<Option<String>, Option<String>>);
    option_to_ruby!(HashMap<Option<String>, Option<&'a str>>);
    option_to_ruby!(HashMap<Option<&'a str>, Option<Value>>);
    option_to_ruby!(HashMap<Option<&'a str>, Option<bool>>);
    option_to_ruby!(HashMap<Option<&'a str>, Option<Vec<u8>>>);
    option_to_ruby!(HashMap<Option<&'a str>, Option<Int>>);
    option_to_ruby!(HashMap<Option<&'a str>, Option<Float>>);
    option_to_ruby!(HashMap<Option<&'a str>, Option<String>>);
    option_to_ruby!(HashMap<Option<&'a str>, Option<&'a str>>);

    // Hash of primitive keys to values
    ruby_to_option!(HashMap<bool, Value>);
    ruby_to_option!(HashMap<bool, bool>);
    ruby_to_option!(HashMap<bool, Vec<u8>>);
    ruby_to_option!(HashMap<bool, Int>);
    ruby_to_option!(HashMap<bool, Float>);
    ruby_to_option!(HashMap<bool, String>);
    ruby_to_option!(HashMap<bool, &'a str>);
    ruby_to_option!(HashMap<Vec<u8>, Value>);
    ruby_to_option!(HashMap<Vec<u8>, bool>);
    ruby_to_option!(HashMap<Vec<u8>, Vec<u8>>);
    ruby_to_option!(HashMap<Vec<u8>, Int>);
    ruby_to_option!(HashMap<Vec<u8>, Float>);
    ruby_to_option!(HashMap<Vec<u8>, String>);
    ruby_to_option!(HashMap<Vec<u8>, &'a str>);
    ruby_to_option!(HashMap<Int, Value>);
    ruby_to_option!(HashMap<Int, bool>);
    ruby_to_option!(HashMap<Int, Vec<u8>>);
    ruby_to_option!(HashMap<Int, Int>);
    ruby_to_option!(HashMap<Int, Float>);
    ruby_to_option!(HashMap<Int, String>);
    ruby_to_option!(HashMap<Int, &'a str>);
    ruby_to_option!(HashMap<String, Value>);
    ruby_to_option!(HashMap<String, bool>);
    ruby_to_option!(HashMap<String, Vec<u8>>);
    ruby_to_option!(HashMap<String, Int>);
    ruby_to_option!(HashMap<String, Float>);
    ruby_to_option!(HashMap<String, String>);
    ruby_to_option!(HashMap<String, &'a str>);
    ruby_to_option!(HashMap<&'a str, Value>);
    ruby_to_option!(HashMap<&'a str, bool>);
    ruby_to_option!(HashMap<&'a str, Vec<u8>>);
    ruby_to_option!(HashMap<&'a str, Int>);
    ruby_to_option!(HashMap<&'a str, Float>);
    ruby_to_option!(HashMap<&'a str, String>);
    ruby_to_option!(HashMap<&'a str, &'a str>);

    // Hash of optional keys to values
    ruby_to_option!(HashMap<Option<bool>, Value>);
    ruby_to_option!(HashMap<Option<bool>, bool>);
    ruby_to_option!(HashMap<Option<bool>, Vec<u8>>);
    ruby_to_option!(HashMap<Option<bool>, Int>);
    ruby_to_option!(HashMap<Option<bool>, Float>);
    ruby_to_option!(HashMap<Option<bool>, String>);
    ruby_to_option!(HashMap<Option<bool>, &'a str>);
    ruby_to_option!(HashMap<Option<Vec<u8>>, Value>);
    ruby_to_option!(HashMap<Option<Vec<u8>>, bool>);
    ruby_to_option!(HashMap<Option<Vec<u8>>, Vec<u8>>);
    ruby_to_option!(HashMap<Option<Vec<u8>>, Int>);
    ruby_to_option!(HashMap<Option<Vec<u8>>, Float>);
    ruby_to_option!(HashMap<Option<Vec<u8>>, String>);
    ruby_to_option!(HashMap<Option<Vec<u8>>, &'a str>);
    ruby_to_option!(HashMap<Option<Int>, Value>);
    ruby_to_option!(HashMap<Option<Int>, bool>);
    ruby_to_option!(HashMap<Option<Int>, Vec<u8>>);
    ruby_to_option!(HashMap<Option<Int>, Int>);
    ruby_to_option!(HashMap<Option<Int>, Float>);
    ruby_to_option!(HashMap<Option<Int>, String>);
    ruby_to_option!(HashMap<Option<Int>, &'a str>);
    ruby_to_option!(HashMap<Option<String>, Value>);
    ruby_to_option!(HashMap<Option<String>, bool>);
    ruby_to_option!(HashMap<Option<String>, Vec<u8>>);
    ruby_to_option!(HashMap<Option<String>, Int>);
    ruby_to_option!(HashMap<Option<String>, Float>);
    ruby_to_option!(HashMap<Option<String>, String>);
    ruby_to_option!(HashMap<Option<String>, &'a str>);
    ruby_to_option!(HashMap<Option<&'a str>, Value>);
    ruby_to_option!(HashMap<Option<&'a str>, bool>);
    ruby_to_option!(HashMap<Option<&'a str>, Vec<u8>>);
    ruby_to_option!(HashMap<Option<&'a str>, Int>);
    ruby_to_option!(HashMap<Option<&'a str>, Float>);
    ruby_to_option!(HashMap<Option<&'a str>, String>);
    ruby_to_option!(HashMap<Option<&'a str>, &'a str>);

    // Hash of primitive keys to optional values
    ruby_to_option!(HashMap<bool, Option<Value>>);
    ruby_to_option!(HashMap<bool, Option<bool>>);
    ruby_to_option!(HashMap<bool, Option<Vec<u8>>>);
    ruby_to_option!(HashMap<bool, Option<Int>>);
    ruby_to_option!(HashMap<bool, Option<Float>>);
    ruby_to_option!(HashMap<bool, Option<String>>);
    ruby_to_option!(HashMap<bool, Option<&'a str>>);
    ruby_to_option!(HashMap<Vec<u8>, Option<Value>>);
    ruby_to_option!(HashMap<Vec<u8>, Option<bool>>);
    ruby_to_option!(HashMap<Vec<u8>, Option<Int>>);
    ruby_to_option!(HashMap<Vec<u8>, Option<Float>>);
    ruby_to_option!(HashMap<Vec<u8>, Option<String>>);
    ruby_to_option!(HashMap<Vec<u8>, Option<&'a str>>);
    ruby_to_option!(HashMap<Int, Option<Value>>);
    ruby_to_option!(HashMap<Int, Option<bool>>);
    ruby_to_option!(HashMap<Int, Option<Vec<u8>>>);
    ruby_to_option!(HashMap<Int, Option<Int>>);
    ruby_to_option!(HashMap<Int, Option<Float>>);
    ruby_to_option!(HashMap<Int, Option<String>>);
    ruby_to_option!(HashMap<Int, Option<&'a str>>);
    ruby_to_option!(HashMap<String, Option<Value>>);
    ruby_to_option!(HashMap<String, Option<bool>>);
    ruby_to_option!(HashMap<String, Option<Vec<u8>>>);
    ruby_to_option!(HashMap<String, Option<Int>>);
    ruby_to_option!(HashMap<String, Option<Float>>);
    ruby_to_option!(HashMap<String, Option<String>>);
    ruby_to_option!(HashMap<String, Option<&'a str>>);
    ruby_to_option!(HashMap<&'a str, Option<Value>>);
    ruby_to_option!(HashMap<&'a str, Option<bool>>);
    ruby_to_option!(HashMap<&'a str, Option<Vec<u8>>>);
    ruby_to_option!(HashMap<&'a str, Option<Int>>);
    ruby_to_option!(HashMap<&'a str, Option<Float>>);
    ruby_to_option!(HashMap<&'a str, Option<String>>);
    ruby_to_option!(HashMap<&'a str, Option<&'a str>>);

    // Hash of primitive optional keys to optional values
    ruby_to_option!(HashMap<Option<bool>, Option<Value>>);
    ruby_to_option!(HashMap<Option<bool>, Option<bool>>);
    ruby_to_option!(HashMap<Option<bool>, Option<Vec<u8>>>);
    ruby_to_option!(HashMap<Option<bool>, Option<Int>>);
    ruby_to_option!(HashMap<Option<bool>, Option<Float>>);
    ruby_to_option!(HashMap<Option<bool>, Option<String>>);
    ruby_to_option!(HashMap<Option<bool>, Option<&'a str>>);
    ruby_to_option!(HashMap<Option<Vec<u8>>, Option<Value>>);
    ruby_to_option!(HashMap<Option<Vec<u8>>, Option<bool>>);
    ruby_to_option!(HashMap<Option<Vec<u8>>, Option<Vec<u8>>>);
    ruby_to_option!(HashMap<Option<Vec<u8>>, Option<Int>>);
    ruby_to_option!(HashMap<Option<Vec<u8>>, Option<Float>>);
    ruby_to_option!(HashMap<Option<Vec<u8>>, Option<String>>);
    ruby_to_option!(HashMap<Option<Vec<u8>>, Option<&'a str>>);
    ruby_to_option!(HashMap<Option<Int>, Option<Value>>);
    ruby_to_option!(HashMap<Option<Int>, Option<bool>>);
    ruby_to_option!(HashMap<Option<Int>, Option<Vec<u8>>>);
    ruby_to_option!(HashMap<Option<Int>, Option<Int>>);
    ruby_to_option!(HashMap<Option<Int>, Option<Float>>);
    ruby_to_option!(HashMap<Option<Int>, Option<String>>);
    ruby_to_option!(HashMap<Option<Int>, Option<&'a str>>);
    ruby_to_option!(HashMap<Option<String>, Option<Value>>);
    ruby_to_option!(HashMap<Option<String>, Option<bool>>);
    ruby_to_option!(HashMap<Option<String>, Option<Vec<u8>>>);
    ruby_to_option!(HashMap<Option<String>, Option<Int>>);
    ruby_to_option!(HashMap<Option<String>, Option<Float>>);
    ruby_to_option!(HashMap<Option<String>, Option<String>>);
    ruby_to_option!(HashMap<Option<String>, Option<&'a str>>);
    ruby_to_option!(HashMap<Option<&'a str>, Option<Value>>);
    ruby_to_option!(HashMap<Option<&'a str>, Option<bool>>);
    ruby_to_option!(HashMap<Option<&'a str>, Option<Vec<u8>>>);
    ruby_to_option!(HashMap<Option<&'a str>, Option<Int>>);
    ruby_to_option!(HashMap<Option<&'a str>, Option<Float>>);
    ruby_to_option!(HashMap<Option<&'a str>, Option<String>>);
    ruby_to_option!(HashMap<Option<&'a str>, Option<&'a str>>);
}

// Primitives
ruby_to_option!(bool);
ruby_to_option!(Vec<u8>);
ruby_to_option!(&'a [u8]);
ruby_to_option!(Int);
ruby_to_option!(Float);
ruby_to_option!(String);
ruby_to_option!(&'a str);

// Array of primitives
ruby_to_option!(Vec<Value>);
ruby_to_option!(Vec<bool>);
ruby_to_option!(Vec<Vec<u8>>);
ruby_to_option!(Vec<&'a [u8]>);
ruby_to_option!(Vec<Int>);
ruby_to_option!(Vec<Float>);
ruby_to_option!(Vec<String>);
ruby_to_option!(Vec<&'a str>);

// Array of optional primitives
ruby_to_option!(Vec<Option<Value>>);
ruby_to_option!(Vec<Option<bool>>);
ruby_to_option!(Vec<Option<Vec<u8>>>);
ruby_to_option!(Vec<Option<Int>>);
ruby_to_option!(Vec<Option<Float>>);
ruby_to_option!(Vec<Option<String>>);
ruby_to_option!(Vec<Option<&'a str>>);

ruby_to_option!(HashMap<Vec<u8>, Option<Vec<u8>>>);
