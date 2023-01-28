//! Converters for nilable primitive Ruby types.
//!
//! Excludes collection types Array and Hash.

use crate::core::{Convert, TryConvert, TryConvertMut, Value as _};
use crate::error::Error;
use crate::value::Value;
use crate::Artichoke;

impl Convert<Option<Value>, Value> for Artichoke {
    fn convert(&self, value: Option<Value>) -> Value {
        Value::from(value)
    }
}

impl Convert<Option<i64>, Value> for Artichoke {
    fn convert(&self, value: Option<i64>) -> Value {
        if let Some(value) = value {
            self.convert(value)
        } else {
            Value::nil()
        }
    }
}

impl TryConvert<Option<usize>, Value> for Artichoke {
    type Error = Error;

    fn try_convert(&self, value: Option<usize>) -> Result<Value, Self::Error> {
        if let Some(value) = value {
            self.try_convert(value)
        } else {
            Ok(Value::nil())
        }
    }
}

impl TryConvertMut<Option<Vec<u8>>, Value> for Artichoke {
    type Error = Error;

    fn try_convert_mut(&mut self, value: Option<Vec<u8>>) -> Result<Value, Self::Error> {
        self.try_convert_mut(value.as_deref())
    }
}

impl TryConvertMut<Option<&[u8]>, Value> for Artichoke {
    type Error = Error;

    fn try_convert_mut(&mut self, value: Option<&[u8]>) -> Result<Value, Self::Error> {
        if let Some(value) = value {
            self.try_convert_mut(value)
        } else {
            Ok(Value::nil())
        }
    }
}

impl TryConvertMut<Option<String>, Value> for Artichoke {
    type Error = Error;

    fn try_convert_mut(&mut self, value: Option<String>) -> Result<Value, Self::Error> {
        self.try_convert_mut(value.as_deref())
    }
}

impl TryConvertMut<Option<&str>, Value> for Artichoke {
    type Error = Error;

    fn try_convert_mut(&mut self, value: Option<&str>) -> Result<Value, Self::Error> {
        if let Some(value) = value {
            self.try_convert_mut(value)
        } else {
            Ok(Value::nil())
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

impl TryConvert<Value, Option<i64>> for Artichoke {
    type Error = Error;

    fn try_convert(&self, value: Value) -> Result<Option<i64>, Self::Error> {
        if value.is_nil() {
            Ok(None)
        } else {
            self.try_convert(value).map(Some)
        }
    }
}
