use crate::convert::{Convert, RustBackedValue, TryConvert};
use crate::extn::core::array::{Array, InlineBuffer};
use crate::sys;
use crate::types::{Int, Ruby, Rust};
use crate::value::Value;
use crate::{Artichoke, ArtichokeError};

impl Convert<&[Value], Value> for Artichoke {
    fn convert(&self, value: &[Value]) -> Value {
        let ary = Array::new(InlineBuffer::from(value));
        ary.try_into_ruby(self, None).expect("Array into Value")
    }
}

impl Convert<Vec<Value>, Value> for Artichoke {
    fn convert(&self, value: Vec<Value>) -> Value {
        let ary = Array::new(InlineBuffer::from(value));
        ary.try_into_ruby(self, None).expect("Array into Value")
    }
}

impl Convert<&[Option<Value>], Value> for Artichoke {
    fn convert(&self, value: &[Option<Value>]) -> Value {
        let buf = value
            .iter()
            .map(|item| {
                item.as_ref()
                    .map_or_else(|| unsafe { sys::mrb_sys_nil_value() }, Value::inner)
            })
            .collect::<Vec<_>>();
        let ary = Array::new(InlineBuffer::from(buf));
        ary.try_into_ruby(self, None).expect("Array into Value")
    }
}

impl Convert<Vec<Vec<u8>>, Value> for Artichoke {
    fn convert(&self, value: Vec<Vec<u8>>) -> Value {
        let buf = value
            .into_iter()
            .map(|item| self.convert(item).inner())
            .collect::<Vec<_>>();
        let ary = Array::new(InlineBuffer::from(buf));
        ary.try_into_ruby(self, None).expect("Array into Value")
    }
}

impl Convert<Vec<&[u8]>, Value> for Artichoke {
    fn convert(&self, value: Vec<&[u8]>) -> Value {
        let buf = value
            .into_iter()
            .map(|item| self.convert(item).inner())
            .collect::<Vec<_>>();
        let ary = Array::new(InlineBuffer::from(buf));
        ary.try_into_ruby(self, None).expect("Array into Value")
    }
}

impl Convert<Vec<String>, Value> for Artichoke {
    fn convert(&self, value: Vec<String>) -> Value {
        let buf = value
            .into_iter()
            .map(|item| self.convert(item).inner())
            .collect::<Vec<_>>();
        let ary = Array::new(InlineBuffer::from(buf));
        ary.try_into_ruby(self, None).expect("Array into Value")
    }
}

impl Convert<Vec<&str>, Value> for Artichoke {
    fn convert(&self, value: Vec<&str>) -> Value {
        let buf = value
            .into_iter()
            .map(|item| self.convert(item).inner())
            .collect::<Vec<_>>();
        let ary = Array::new(InlineBuffer::from(buf));
        ary.try_into_ruby(self, None).expect("Array into Value")
    }
}

impl Convert<Vec<Int>, Value> for Artichoke {
    fn convert(&self, value: Vec<Int>) -> Value {
        let buf = value
            .into_iter()
            .map(|item| self.convert(item).inner())
            .collect::<Vec<_>>();
        let ary = Array::new(InlineBuffer::from(buf));
        ary.try_into_ruby(self, None).expect("Array into Value")
    }
}

impl Convert<&[Int], Value> for Artichoke {
    fn convert(&self, value: &[Int]) -> Value {
        let buf = value
            .iter()
            .copied()
            .map(|item| self.convert(item).inner())
            .collect::<Vec<_>>();
        let ary = Array::new(InlineBuffer::from(buf));
        ary.try_into_ruby(self, None).expect("Array into Value")
    }
}

impl Convert<Vec<Option<Vec<u8>>>, Value> for Artichoke {
    fn convert(&self, value: Vec<Option<Vec<u8>>>) -> Value {
        let buf = value
            .into_iter()
            .map(|item| self.convert(item).inner())
            .collect::<Vec<_>>();
        let ary = Array::new(InlineBuffer::from(buf));
        ary.try_into_ruby(self, None).expect("Array into Value")
    }
}

impl Convert<&[Option<&[u8]>], Value> for Artichoke {
    fn convert(&self, value: &[Option<&[u8]>]) -> Value {
        let buf = value
            .iter()
            .copied()
            .map(|item| self.convert(item).inner())
            .collect::<Vec<_>>();
        let ary = Array::new(InlineBuffer::from(buf));
        ary.try_into_ruby(self, None).expect("Array into Value")
    }
}

impl Convert<Vec<Option<&[u8]>>, Value> for Artichoke {
    fn convert(&self, value: Vec<Option<&[u8]>>) -> Value {
        let buf = value
            .into_iter()
            .map(|item| self.convert(item).inner())
            .collect::<Vec<_>>();
        let ary = Array::new(InlineBuffer::from(buf));
        ary.try_into_ruby(self, None).expect("Array into Value")
    }
}

impl Convert<Vec<Vec<Option<&[u8]>>>, Value> for Artichoke {
    fn convert(&self, value: Vec<Vec<Option<&[u8]>>>) -> Value {
        let buf = value
            .into_iter()
            .map(|item| self.convert(item).inner())
            .collect::<Vec<_>>();
        let ary = Array::new(InlineBuffer::from(buf));
        ary.try_into_ruby(self, None).expect("Array into Value")
    }
}

impl Convert<&[Option<&str>], Value> for Artichoke {
    fn convert(&self, value: &[Option<&str>]) -> Value {
        let buf = value
            .iter()
            .copied()
            .map(|item| self.convert(item).inner())
            .collect::<Vec<_>>();
        let ary = Array::new(InlineBuffer::from(buf));
        ary.try_into_ruby(self, None).expect("Array into Value")
    }
}

impl Convert<Vec<Option<&str>>, Value> for Artichoke {
    fn convert(&self, value: Vec<Option<&str>>) -> Value {
        let buf = value
            .into_iter()
            .map(|item| self.convert(item).inner())
            .collect::<Vec<_>>();
        let ary = Array::new(InlineBuffer::from(buf));
        ary.try_into_ruby(self, None).expect("Array into Value")
    }
}

impl Convert<Vec<Vec<Option<&str>>>, Value> for Artichoke {
    fn convert(&self, value: Vec<Vec<Option<&str>>>) -> Value {
        let buf = value
            .into_iter()
            .map(|item| self.convert(item).inner())
            .collect::<Vec<_>>();
        let ary = Array::new(InlineBuffer::from(buf));
        ary.try_into_ruby(self, None).expect("Array into Value")
    }
}

impl TryConvert<Value, Vec<Value>> for Artichoke {
    fn try_convert(&self, value: Value) -> Result<Vec<Value>, ArtichokeError> {
        match value.ruby_type() {
            Ruby::Array => {
                unreachable!("mrb_array implementation is obsoleted by extn::core::array")
            }
            Ruby::Data => {
                let array = unsafe { Array::try_from_ruby(self, &value)? };
                let borrow = array.borrow();
                Ok(borrow.as_vec(self))
            }
            type_tag => Err(ArtichokeError::ConvertToRust {
                from: type_tag,
                to: Rust::Vec,
            }),
        }
    }
}

impl TryConvert<Value, Vec<Vec<u8>>> for Artichoke {
    fn try_convert(&self, value: Value) -> Result<Vec<Vec<u8>>, ArtichokeError> {
        match value.ruby_type() {
            Ruby::Array => {
                unreachable!("mrb_array implementation is obsoleted by extn::core::array")
            }
            Ruby::Data => {
                let array = unsafe { Array::try_from_ruby(self, &value)? };
                let borrow = array.borrow();
                let array = borrow.as_vec(self);
                let mut buf = Vec::with_capacity(array.len());
                for elem in array {
                    buf.push(self.try_convert(elem)?);
                }
                Ok(buf)
            }
            type_tag => Err(ArtichokeError::ConvertToRust {
                from: type_tag,
                to: Rust::Vec,
            }),
        }
    }
}

impl TryConvert<Value, Vec<Option<Vec<u8>>>> for Artichoke {
    fn try_convert(&self, value: Value) -> Result<Vec<Option<Vec<u8>>>, ArtichokeError> {
        match value.ruby_type() {
            Ruby::Array => {
                unreachable!("mrb_array implementation is obsoleted by extn::core::array")
            }
            Ruby::Data => {
                let array = unsafe { Array::try_from_ruby(self, &value)? };
                let borrow = array.borrow();
                let array = borrow.as_vec(self);
                let mut buf = Vec::with_capacity(array.len());
                for elem in array {
                    buf.push(self.try_convert(elem)?);
                }
                Ok(buf)
            }
            type_tag => Err(ArtichokeError::ConvertToRust {
                from: type_tag,
                to: Rust::Vec,
            }),
        }
    }
}

impl<'a> TryConvert<Value, Vec<&'a [u8]>> for Artichoke {
    fn try_convert(&self, value: Value) -> Result<Vec<&'a [u8]>, ArtichokeError> {
        match value.ruby_type() {
            Ruby::Array => {
                unreachable!("mrb_array implementation is obsoleted by extn::core::array")
            }
            Ruby::Data => {
                let array = unsafe { Array::try_from_ruby(self, &value)? };
                let borrow = array.borrow();
                let array = borrow.as_vec(self);
                let mut buf = Vec::with_capacity(array.len());
                for elem in array {
                    buf.push(self.try_convert(elem)?);
                }
                Ok(buf)
            }
            type_tag => Err(ArtichokeError::ConvertToRust {
                from: type_tag,
                to: Rust::Vec,
            }),
        }
    }
}

impl<'a> TryConvert<Value, Vec<Option<&'a [u8]>>> for Artichoke {
    fn try_convert(&self, value: Value) -> Result<Vec<Option<&'a [u8]>>, ArtichokeError> {
        match value.ruby_type() {
            Ruby::Array => {
                unreachable!("mrb_array implementation is obsoleted by extn::core::array")
            }
            Ruby::Data => {
                let array = unsafe { Array::try_from_ruby(self, &value)? };
                let borrow = array.borrow();
                let array = borrow.as_vec(self);
                let mut buf = Vec::with_capacity(array.len());
                for elem in array {
                    buf.push(self.try_convert(elem)?);
                }
                Ok(buf)
            }
            type_tag => Err(ArtichokeError::ConvertToRust {
                from: type_tag,
                to: Rust::Vec,
            }),
        }
    }
}

impl TryConvert<Value, Vec<String>> for Artichoke {
    fn try_convert(&self, value: Value) -> Result<Vec<String>, ArtichokeError> {
        match value.ruby_type() {
            Ruby::Array => {
                unreachable!("mrb_array implementation is obsoleted by extn::core::array")
            }
            Ruby::Data => {
                let array = unsafe { Array::try_from_ruby(self, &value)? };
                let borrow = array.borrow();
                let array = borrow.as_vec(self);
                let mut buf = Vec::with_capacity(array.len());
                for elem in array {
                    buf.push(self.try_convert(elem)?);
                }
                Ok(buf)
            }
            type_tag => Err(ArtichokeError::ConvertToRust {
                from: type_tag,
                to: Rust::Vec,
            }),
        }
    }
}

impl TryConvert<Value, Vec<Option<String>>> for Artichoke {
    fn try_convert(&self, value: Value) -> Result<Vec<Option<String>>, ArtichokeError> {
        match value.ruby_type() {
            Ruby::Array => {
                unreachable!("mrb_array implementation is obsoleted by extn::core::array")
            }
            Ruby::Data => {
                let array = unsafe { Array::try_from_ruby(self, &value)? };
                let borrow = array.borrow();
                let array = borrow.as_vec(self);
                let mut buf = Vec::with_capacity(array.len());
                for elem in array {
                    buf.push(self.try_convert(elem)?);
                }
                Ok(buf)
            }
            type_tag => Err(ArtichokeError::ConvertToRust {
                from: type_tag,
                to: Rust::Vec,
            }),
        }
    }
}

impl<'a> TryConvert<Value, Vec<&'a str>> for Artichoke {
    fn try_convert(&self, value: Value) -> Result<Vec<&'a str>, ArtichokeError> {
        match value.ruby_type() {
            Ruby::Array => {
                unreachable!("mrb_array implementation is obsoleted by extn::core::array")
            }
            Ruby::Data => {
                let array = unsafe { Array::try_from_ruby(self, &value)? };
                let borrow = array.borrow();
                let array = borrow.as_vec(self);
                let mut buf = Vec::with_capacity(array.len());
                for elem in array {
                    buf.push(self.try_convert(elem)?);
                }
                Ok(buf)
            }
            type_tag => Err(ArtichokeError::ConvertToRust {
                from: type_tag,
                to: Rust::Vec,
            }),
        }
    }
}

impl<'a> TryConvert<Value, Vec<Option<&'a str>>> for Artichoke {
    fn try_convert(&self, value: Value) -> Result<Vec<Option<&'a str>>, ArtichokeError> {
        match value.ruby_type() {
            Ruby::Array => {
                unreachable!("mrb_array implementation is obsoleted by extn::core::array")
            }
            Ruby::Data => {
                let array = unsafe { Array::try_from_ruby(self, &value)? };
                let borrow = array.borrow();
                let array = borrow.as_vec(self);
                let mut buf = Vec::with_capacity(array.len());
                for elem in array {
                    buf.push(self.try_convert(elem)?);
                }
                Ok(buf)
            }
            type_tag => Err(ArtichokeError::ConvertToRust {
                from: type_tag,
                to: Rust::Vec,
            }),
        }
    }
}

impl TryConvert<Value, Vec<Int>> for Artichoke {
    fn try_convert(&self, value: Value) -> Result<Vec<Int>, ArtichokeError> {
        match value.ruby_type() {
            Ruby::Array => {
                unreachable!("mrb_array implementation is obsoleted by extn::core::array")
            }
            Ruby::Data => {
                let array = unsafe { Array::try_from_ruby(self, &value)? };
                let borrow = array.borrow();
                let array = borrow.as_vec(self);
                let mut buf = Vec::with_capacity(array.len());
                for elem in array {
                    buf.push(self.try_convert(elem)?);
                }
                Ok(buf)
            }
            type_tag => Err(ArtichokeError::ConvertToRust {
                from: type_tag,
                to: Rust::Vec,
            }),
        }
    }
}
