//! Converters for nilable primitive Ruby types. Excludes collection types
//! Array and Hash.

use crate::convert::{Convert, TryConvert};
use crate::sys;
use crate::types::{Int, Ruby};
use crate::value::Value;
use crate::{Artichoke, ArtichokeError};

impl Convert<Option<Value>, Value> for Artichoke {
    fn convert(&self, value: Option<Value>) -> Value {
        if let Some(value) = value {
            value
        } else {
            Value::new(self, unsafe { sys::mrb_sys_nil_value() })
        }
    }
}

impl Convert<&Option<Value>, Value> for Artichoke {
    fn convert(&self, value: &Option<Value>) -> Value {
        if let Some(value) = value {
            value.clone()
        } else {
            Value::new(self, unsafe { sys::mrb_sys_nil_value() })
        }
    }
}

impl Convert<Option<Int>, Value> for Artichoke {
    fn convert(&self, value: Option<Int>) -> Value {
        if let Some(value) = value {
            self.convert(value)
        } else {
            Value::new(self, unsafe { sys::mrb_sys_nil_value() })
        }
    }
}

impl Convert<Option<Vec<u8>>, Value> for Artichoke {
    fn convert(&self, value: Option<Vec<u8>>) -> Value {
        if let Some(value) = value {
            self.convert(value)
        } else {
            Value::new(self, unsafe { sys::mrb_sys_nil_value() })
        }
    }
}

impl Convert<Option<&[u8]>, Value> for Artichoke {
    fn convert(&self, value: Option<&[u8]>) -> Value {
        if let Some(value) = value {
            self.convert(value)
        } else {
            Value::new(self, unsafe { sys::mrb_sys_nil_value() })
        }
    }
}

impl Convert<Option<&str>, Value> for Artichoke {
    fn convert(&self, value: Option<&str>) -> Value {
        if let Some(value) = value {
            self.convert(value)
        } else {
            Value::new(self, unsafe { sys::mrb_sys_nil_value() })
        }
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

impl<'a> TryConvert<Value, Option<bool>> for Artichoke {
    fn try_convert(&self, value: Value) -> Result<Option<bool>, ArtichokeError> {
        if let Ruby::Nil = value.ruby_type() {
            Ok(None)
        } else {
            self.try_convert(value).map(Some)
        }
    }
}

impl<'a> TryConvert<Value, Option<Vec<u8>>> for Artichoke {
    fn try_convert(&self, value: Value) -> Result<Option<Vec<u8>>, ArtichokeError> {
        if let Ruby::Nil = value.ruby_type() {
            Ok(None)
        } else {
            self.try_convert(value).map(Some)
        }
    }
}

impl<'a> TryConvert<Value, Option<&'a [u8]>> for Artichoke {
    fn try_convert(&self, value: Value) -> Result<Option<&'a [u8]>, ArtichokeError> {
        if let Ruby::Nil = value.ruby_type() {
            Ok(None)
        } else {
            self.try_convert(value).map(Some)
        }
    }
}

impl<'a> TryConvert<Value, Option<String>> for Artichoke {
    fn try_convert(&self, value: Value) -> Result<Option<String>, ArtichokeError> {
        if let Ruby::Nil = value.ruby_type() {
            Ok(None)
        } else {
            self.try_convert(value).map(Some)
        }
    }
}

impl<'a> TryConvert<Value, Option<&'a str>> for Artichoke {
    fn try_convert(&self, value: Value) -> Result<Option<&'a str>, ArtichokeError> {
        if let Ruby::Nil = value.ruby_type() {
            Ok(None)
        } else {
            self.try_convert(value).map(Some)
        }
    }
}

impl<'a> TryConvert<Value, Option<Int>> for Artichoke {
    fn try_convert(&self, value: Value) -> Result<Option<Int>, ArtichokeError> {
        if let Ruby::Nil = value.ruby_type() {
            Ok(None)
        } else {
            self.try_convert(value).map(Some)
        }
    }
}
