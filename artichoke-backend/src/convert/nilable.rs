//! Converters for nilable primitive Ruby types.
//!
//! Excludes collection types Array and Hash.

use crate::core::{Convert, ConvertMut, TryConvert, TryConvertMut, Value as _};
use crate::error::Error;
use crate::types::Int;
use crate::value::Value;
use crate::Artichoke;

impl Convert<Option<Value>, Value> for Artichoke {
    fn convert(&self, value: Option<Value>) -> Value {
        Value::from(value)
    }
}

impl Convert<Option<Int>, Value> for Artichoke {
    fn convert(&self, value: Option<Int>) -> Value {
        if let Some(value) = value {
            self.convert(value)
        } else {
            Value::nil()
        }
    }
}

impl ConvertMut<Option<Vec<u8>>, Value> for Artichoke {
    fn convert_mut(&mut self, value: Option<Vec<u8>>) -> Value {
        self.convert_mut(value.as_deref())
    }
}

impl ConvertMut<Option<&[u8]>, Value> for Artichoke {
    fn convert_mut(&mut self, value: Option<&[u8]>) -> Value {
        if let Some(value) = value {
            self.convert_mut(value)
        } else {
            Value::nil()
        }
    }
}

impl ConvertMut<Option<String>, Value> for Artichoke {
    fn convert_mut(&mut self, value: Option<String>) -> Value {
        self.convert_mut(value.as_deref())
    }
}

impl ConvertMut<Option<&str>, Value> for Artichoke {
    fn convert_mut(&mut self, value: Option<&str>) -> Value {
        if let Some(value) = value {
            self.convert_mut(value)
        } else {
            Value::nil()
        }
    }
}

impl Convert<Value, Option<Value>> for Artichoke {
    fn convert(&self, value: Value) -> Option<Value> {
        if value.is_nil() {
            None
        } else {
            Some(value)
        }
    }
}

impl TryConvertMut<Value, Option<Vec<u8>>> for Artichoke {
    type Error = Error;

    fn try_convert_mut(&mut self, value: Value) -> Result<Option<Vec<u8>>, Self::Error> {
        if value.is_nil() {
            Ok(None)
        } else {
            self.try_convert_mut(value).map(Some)
        }
    }
}

impl<'a> TryConvertMut<Value, Option<&'a [u8]>> for Artichoke {
    type Error = Error;

    fn try_convert_mut(&mut self, value: Value) -> Result<Option<&'a [u8]>, Self::Error> {
        if value.is_nil() {
            Ok(None)
        } else {
            self.try_convert_mut(value).map(Some)
        }
    }
}

impl TryConvertMut<Value, Option<String>> for Artichoke {
    type Error = Error;

    fn try_convert_mut(&mut self, value: Value) -> Result<Option<String>, Self::Error> {
        if value.is_nil() {
            Ok(None)
        } else {
            self.try_convert_mut(value).map(Some)
        }
    }
}

impl<'a> TryConvertMut<Value, Option<&'a str>> for Artichoke {
    type Error = Error;

    fn try_convert_mut(&mut self, value: Value) -> Result<Option<&'a str>, Self::Error> {
        if value.is_nil() {
            Ok(None)
        } else {
            self.try_convert_mut(value).map(Some)
        }
    }
}

impl TryConvert<Value, Option<Int>> for Artichoke {
    type Error = Error;

    fn try_convert(&self, value: Value) -> Result<Option<Int>, Self::Error> {
        if value.is_nil() {
            Ok(None)
        } else {
            self.try_convert(value).map(Some)
        }
    }
}
