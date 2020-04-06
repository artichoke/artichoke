use std::iter::FromIterator;

use crate::convert::{RustBackedValue, UnboxRubyError};
use crate::exception::Exception;
use crate::extn::core::array::{Array, InlineBuffer};
use crate::types::{Int, Ruby, Rust};
use crate::value::Value;
use crate::{Artichoke, Convert, ConvertMut, TryConvert};

impl ConvertMut<&[Value], Value> for Artichoke {
    fn convert_mut(&mut self, value: &[Value]) -> Value {
        let ary = Array::new(InlineBuffer::from(value));
        ary.try_into_ruby(self, None).expect("Array into Value")
    }
}

impl ConvertMut<Vec<Value>, Value> for Artichoke {
    fn convert_mut(&mut self, value: Vec<Value>) -> Value {
        let ary = Array::new(InlineBuffer::from(value));
        ary.try_into_ruby(self, None).expect("Array into Value")
    }
}

impl ConvertMut<&[Option<Value>], Value> for Artichoke {
    fn convert_mut(&mut self, value: &[Option<Value>]) -> Value {
        let ary = Array::new(InlineBuffer::from_iter(value));
        ary.try_into_ruby(self, None).expect("Array into Value")
    }
}

impl ConvertMut<Vec<Vec<u8>>, Value> for Artichoke {
    fn convert_mut(&mut self, value: Vec<Vec<u8>>) -> Value {
        let iter = value.into_iter().map(|item| self.convert_mut(item));
        let ary = Array::new(InlineBuffer::from_iter(iter));
        ary.try_into_ruby(self, None).expect("Array into Value")
    }
}

impl ConvertMut<Vec<&[u8]>, Value> for Artichoke {
    fn convert_mut(&mut self, value: Vec<&[u8]>) -> Value {
        let iter = value.into_iter().map(|item| self.convert_mut(item));
        let ary = Array::new(InlineBuffer::from_iter(iter));
        ary.try_into_ruby(self, None).expect("Array into Value")
    }
}

impl ConvertMut<&[Vec<u8>], Value> for Artichoke {
    fn convert_mut(&mut self, value: &[Vec<u8>]) -> Value {
        let iter = value.iter().map(|item| self.convert_mut(item.as_slice()));
        let ary = Array::new(InlineBuffer::from_iter(iter));
        ary.try_into_ruby(self, None).expect("Array into Value")
    }
}

impl ConvertMut<&[&[u8]], Value> for Artichoke {
    fn convert_mut(&mut self, value: &[&[u8]]) -> Value {
        let iter = value.iter().copied().map(|item| self.convert_mut(item));
        let ary = Array::new(InlineBuffer::from_iter(iter));
        ary.try_into_ruby(self, None).expect("Array into Value")
    }
}

impl ConvertMut<Vec<String>, Value> for Artichoke {
    fn convert_mut(&mut self, value: Vec<String>) -> Value {
        let iter = value.into_iter().map(|item| self.convert_mut(item));
        let ary = Array::new(InlineBuffer::from_iter(iter));
        ary.try_into_ruby(self, None).expect("Array into Value")
    }
}

impl ConvertMut<Vec<&str>, Value> for Artichoke {
    fn convert_mut(&mut self, value: Vec<&str>) -> Value {
        let iter = value.into_iter().map(|item| self.convert_mut(item));
        let ary = Array::new(InlineBuffer::from_iter(iter));
        ary.try_into_ruby(self, None).expect("Array into Value")
    }
}

impl ConvertMut<&[String], Value> for Artichoke {
    fn convert_mut(&mut self, value: &[String]) -> Value {
        let iter = value.iter().map(|item| self.convert_mut(item.as_str()));
        let ary = Array::new(InlineBuffer::from_iter(iter));
        ary.try_into_ruby(self, None).expect("Array into Value")
    }
}

impl ConvertMut<&[&str], Value> for Artichoke {
    fn convert_mut(&mut self, value: &[&str]) -> Value {
        let iter = value.iter().copied().map(|item| self.convert_mut(item));
        let ary = Array::new(InlineBuffer::from_iter(iter));
        ary.try_into_ruby(self, None).expect("Array into Value")
    }
}

impl ConvertMut<Vec<Int>, Value> for Artichoke {
    fn convert_mut(&mut self, value: Vec<Int>) -> Value {
        let iter = value.into_iter().map(|item| self.convert(item));
        let ary = Array::new(InlineBuffer::from_iter(iter));
        ary.try_into_ruby(self, None).expect("Array into Value")
    }
}

impl ConvertMut<&[Int], Value> for Artichoke {
    fn convert_mut(&mut self, value: &[Int]) -> Value {
        let iter = value.iter().copied().map(|item| self.convert(item));
        let ary = Array::new(InlineBuffer::from_iter(iter));
        ary.try_into_ruby(self, None).expect("Array into Value")
    }
}

impl ConvertMut<&[Option<Vec<u8>>], Value> for Artichoke {
    fn convert_mut(&mut self, value: &[Option<Vec<u8>>]) -> Value {
        let iter = value.iter().map(|item| self.convert_mut(item.as_deref()));
        let ary = Array::new(InlineBuffer::from_iter(iter));
        ary.try_into_ruby(self, None).expect("Array into Value")
    }
}

impl ConvertMut<Vec<Option<Vec<u8>>>, Value> for Artichoke {
    fn convert_mut(&mut self, value: Vec<Option<Vec<u8>>>) -> Value {
        let iter = value.into_iter().map(|item| self.convert_mut(item));
        let ary = Array::new(InlineBuffer::from_iter(iter));
        ary.try_into_ruby(self, None).expect("Array into Value")
    }
}

impl ConvertMut<&[Option<&[u8]>], Value> for Artichoke {
    fn convert_mut(&mut self, value: &[Option<&[u8]>]) -> Value {
        let iter = value.iter().copied().map(|item| self.convert_mut(item));
        let ary = Array::new(InlineBuffer::from_iter(iter));
        ary.try_into_ruby(self, None).expect("Array into Value")
    }
}

impl ConvertMut<Vec<Option<&[u8]>>, Value> for Artichoke {
    fn convert_mut(&mut self, value: Vec<Option<&[u8]>>) -> Value {
        let iter = value.into_iter().map(|item| self.convert_mut(item));
        let ary = Array::new(InlineBuffer::from_iter(iter));
        ary.try_into_ruby(self, None).expect("Array into Value")
    }
}

impl ConvertMut<Vec<Vec<Option<Vec<u8>>>>, Value> for Artichoke {
    fn convert_mut(&mut self, value: Vec<Vec<Option<Vec<u8>>>>) -> Value {
        let iter = value.into_iter().map(|item| self.convert_mut(item));
        let ary = Array::new(InlineBuffer::from_iter(iter));
        ary.try_into_ruby(self, None).expect("Array into Value")
    }
}

impl ConvertMut<Vec<Vec<Option<&[u8]>>>, Value> for Artichoke {
    fn convert_mut(&mut self, value: Vec<Vec<Option<&[u8]>>>) -> Value {
        let iter = value.into_iter().map(|item| self.convert_mut(item));
        let ary = Array::new(InlineBuffer::from_iter(iter));
        ary.try_into_ruby(self, None).expect("Array into Value")
    }
}

impl ConvertMut<&[Option<&str>], Value> for Artichoke {
    fn convert_mut(&mut self, value: &[Option<&str>]) -> Value {
        let iter = value.iter().copied().map(|item| self.convert_mut(item));
        let ary = Array::new(InlineBuffer::from_iter(iter));
        ary.try_into_ruby(self, None).expect("Array into Value")
    }
}

impl ConvertMut<Vec<Option<&str>>, Value> for Artichoke {
    fn convert_mut(&mut self, value: Vec<Option<&str>>) -> Value {
        let iter = value.into_iter().map(|item| self.convert_mut(item));
        let ary = Array::new(InlineBuffer::from_iter(iter));
        ary.try_into_ruby(self, None).expect("Array into Value")
    }
}

impl ConvertMut<Vec<Vec<Option<&str>>>, Value> for Artichoke {
    fn convert_mut(&mut self, value: Vec<Vec<Option<&str>>>) -> Value {
        let iter = value.into_iter().map(|item| self.convert_mut(item));
        let ary = Array::new(InlineBuffer::from_iter(iter));
        ary.try_into_ruby(self, None).expect("Array into Value")
    }
}

impl TryConvert<Value, Vec<Value>> for Artichoke {
    type Error = Exception;

    fn try_convert(&self, value: Value) -> Result<Vec<Value>, Self::Error> {
        if let Ruby::Data = value.ruby_type() {
            let array = unsafe { Array::try_from_ruby(self, &value) }?;
            let borrow = array.borrow();
            Ok(borrow.as_vec(self))
        } else {
            Err(Exception::from(UnboxRubyError::new(&value, Rust::Vec)))
        }
    }
}

impl TryConvert<Value, Vec<Vec<u8>>> for Artichoke {
    type Error = Exception;

    fn try_convert(&self, value: Value) -> Result<Vec<Vec<u8>>, Self::Error> {
        if let Ruby::Data = value.ruby_type() {
            let array = unsafe { Array::try_from_ruby(self, &value) }?;
            let borrow = array.borrow();
            let array = borrow.as_vec(self);
            let mut buf = Vec::with_capacity(array.len());
            for elem in array {
                buf.push(self.try_convert(elem)?);
            }
            Ok(buf)
        } else {
            Err(Exception::from(UnboxRubyError::new(&value, Rust::Vec)))
        }
    }
}

impl TryConvert<Value, Vec<Option<Vec<u8>>>> for Artichoke {
    type Error = Exception;

    fn try_convert(&self, value: Value) -> Result<Vec<Option<Vec<u8>>>, Self::Error> {
        if let Ruby::Data = value.ruby_type() {
            let array = unsafe { Array::try_from_ruby(self, &value) }?;
            let borrow = array.borrow();
            let array = borrow.as_vec(self);
            let mut buf = Vec::with_capacity(array.len());
            for elem in array {
                buf.push(self.try_convert(elem)?);
            }
            Ok(buf)
        } else {
            Err(Exception::from(UnboxRubyError::new(&value, Rust::Vec)))
        }
    }
}

impl<'a> TryConvert<Value, Vec<&'a [u8]>> for Artichoke {
    type Error = Exception;

    fn try_convert(&self, value: Value) -> Result<Vec<&'a [u8]>, Self::Error> {
        if let Ruby::Data = value.ruby_type() {
            let array = unsafe { Array::try_from_ruby(self, &value) }?;
            let borrow = array.borrow();
            let array = borrow.as_vec(self);
            let mut buf = Vec::with_capacity(array.len());
            for elem in array {
                buf.push(self.try_convert(elem)?);
            }
            Ok(buf)
        } else {
            Err(Exception::from(UnboxRubyError::new(&value, Rust::Vec)))
        }
    }
}

impl<'a> TryConvert<Value, Vec<Option<&'a [u8]>>> for Artichoke {
    type Error = Exception;

    fn try_convert(&self, value: Value) -> Result<Vec<Option<&'a [u8]>>, Self::Error> {
        if let Ruby::Data = value.ruby_type() {
            let array = unsafe { Array::try_from_ruby(self, &value) }?;
            let borrow = array.borrow();
            let array = borrow.as_vec(self);
            let mut buf = Vec::with_capacity(array.len());
            for elem in array {
                buf.push(self.try_convert(elem)?);
            }
            Ok(buf)
        } else {
            Err(Exception::from(UnboxRubyError::new(&value, Rust::Vec)))
        }
    }
}

impl TryConvert<Value, Vec<String>> for Artichoke {
    type Error = Exception;

    fn try_convert(&self, value: Value) -> Result<Vec<String>, Self::Error> {
        if let Ruby::Data = value.ruby_type() {
            let array = unsafe { Array::try_from_ruby(self, &value) }?;
            let borrow = array.borrow();
            let array = borrow.as_vec(self);
            let mut buf = Vec::with_capacity(array.len());
            for elem in array {
                buf.push(self.try_convert(elem)?);
            }
            Ok(buf)
        } else {
            Err(Exception::from(UnboxRubyError::new(&value, Rust::Vec)))
        }
    }
}

impl TryConvert<Value, Vec<Option<String>>> for Artichoke {
    type Error = Exception;

    fn try_convert(&self, value: Value) -> Result<Vec<Option<String>>, Self::Error> {
        if let Ruby::Data = value.ruby_type() {
            let array = unsafe { Array::try_from_ruby(self, &value) }?;
            let borrow = array.borrow();
            let array = borrow.as_vec(self);
            let mut buf = Vec::with_capacity(array.len());
            for elem in array {
                buf.push(self.try_convert(elem)?);
            }
            Ok(buf)
        } else {
            Err(Exception::from(UnboxRubyError::new(&value, Rust::Vec)))
        }
    }
}

impl<'a> TryConvert<Value, Vec<&'a str>> for Artichoke {
    type Error = Exception;

    fn try_convert(&self, value: Value) -> Result<Vec<&'a str>, Self::Error> {
        if let Ruby::Data = value.ruby_type() {
            let array = unsafe { Array::try_from_ruby(self, &value) }?;
            let borrow = array.borrow();
            let array = borrow.as_vec(self);
            let mut buf = Vec::with_capacity(array.len());
            for elem in array {
                buf.push(self.try_convert(elem)?);
            }
            Ok(buf)
        } else {
            Err(Exception::from(UnboxRubyError::new(&value, Rust::Vec)))
        }
    }
}

impl<'a> TryConvert<Value, Vec<Option<&'a str>>> for Artichoke {
    type Error = Exception;

    fn try_convert(&self, value: Value) -> Result<Vec<Option<&'a str>>, Self::Error> {
        if let Ruby::Data = value.ruby_type() {
            let array = unsafe { Array::try_from_ruby(self, &value) }?;
            let borrow = array.borrow();
            let array = borrow.as_vec(self);
            let mut buf = Vec::with_capacity(array.len());
            for elem in array {
                buf.push(self.try_convert(elem)?);
            }
            Ok(buf)
        } else {
            Err(Exception::from(UnboxRubyError::new(&value, Rust::Vec)))
        }
    }
}

impl TryConvert<Value, Vec<Int>> for Artichoke {
    type Error = Exception;

    fn try_convert(&self, value: Value) -> Result<Vec<Int>, Self::Error> {
        if let Ruby::Data = value.ruby_type() {
            let array = unsafe { Array::try_from_ruby(self, &value) }?;
            let borrow = array.borrow();
            let array = borrow.as_vec(self);
            let mut buf = Vec::with_capacity(array.len());
            for elem in array {
                buf.push(self.try_convert(elem)?);
            }
            Ok(buf)
        } else {
            Err(Exception::from(UnboxRubyError::new(&value, Rust::Vec)))
        }
    }
}

#[cfg(test)]
mod tests {
    use quickcheck_macros::quickcheck;

    use crate::test::prelude::*;

    #[test]
    fn fail_convert() {
        let mut interp = crate::interpreter().unwrap();
        // get a Ruby value that can't be converted to a primitive type.
        let value = interp.eval(b"Object.new").unwrap();
        let result = value.try_into::<Vec<Value>>(&interp);
        assert!(result.is_err());
    }

    #[quickcheck]
    fn arr_int(arr: Vec<Int>) -> bool {
        let mut interp = crate::interpreter().unwrap();
        // Borrowed converter
        let value = interp.convert_mut(arr.as_slice());
        let len = value
            .funcall::<usize>(&mut interp, "length", &[], None)
            .unwrap();
        if len != arr.len() {
            return false;
        }
        let empty = value
            .funcall::<bool>(&mut interp, "empty?", &[], None)
            .unwrap();
        if empty != arr.is_empty() {
            return false;
        }
        let recovered: Vec<Int> = interp.try_convert(value).unwrap();
        if recovered != arr {
            return false;
        }
        // Owned converter
        let value = interp.convert_mut(arr.to_vec());
        let len = value
            .funcall::<usize>(&mut interp, "length", &[], None)
            .unwrap();
        if len != arr.len() {
            return false;
        }
        let empty = value
            .funcall::<bool>(&mut interp, "empty?", &[], None)
            .unwrap();
        if empty != arr.is_empty() {
            return false;
        }
        let recovered: Vec<Int> = interp.try_convert(value).unwrap();
        if recovered != arr {
            return false;
        }
        true
    }

    #[quickcheck]
    fn arr_utf8(arr: Vec<String>) -> bool {
        let mut interp = crate::interpreter().unwrap();
        // Borrowed converter
        let value = interp.convert_mut(arr.as_slice());
        let len = value
            .funcall::<usize>(&mut interp, "length", &[], None)
            .unwrap();
        if len != arr.len() {
            return false;
        }
        let empty = value
            .funcall::<bool>(&mut interp, "empty?", &[], None)
            .unwrap();
        if empty != arr.is_empty() {
            return false;
        }
        let recovered: Vec<String> = interp.try_convert(value).unwrap();
        if recovered != arr {
            return false;
        }
        // Owned converter
        let value = interp.convert_mut(arr.to_vec());
        let len = value
            .funcall::<usize>(&mut interp, "length", &[], None)
            .unwrap();
        if len != arr.len() {
            return false;
        }
        let empty = value
            .funcall::<bool>(&mut interp, "empty?", &[], None)
            .unwrap();
        if empty != arr.is_empty() {
            return false;
        }
        let recovered: Vec<String> = interp.try_convert(value).unwrap();
        if recovered != arr {
            return false;
        }
        true
    }

    #[quickcheck]
    fn arr_nilable_bstr(arr: Vec<Option<Vec<u8>>>) -> bool {
        let mut interp = crate::interpreter().unwrap();
        // Borrowed converter
        let value = interp.convert_mut(arr.as_slice());
        let len = value
            .funcall::<usize>(&mut interp, "length", &[], None)
            .unwrap();
        if len != arr.len() {
            return false;
        }
        let empty = value
            .funcall::<bool>(&mut interp, "empty?", &[], None)
            .unwrap();
        if empty != arr.is_empty() {
            return false;
        }
        let recovered: Vec<Option<Vec<u8>>> = interp.try_convert(value).unwrap();
        if recovered != arr {
            return false;
        }
        // Owned converter
        let value = interp.convert_mut(arr.to_vec());
        let len = value
            .funcall::<usize>(&mut interp, "length", &[], None)
            .unwrap();
        if len != arr.len() {
            return false;
        }
        let empty = value
            .funcall::<bool>(&mut interp, "empty?", &[], None)
            .unwrap();
        if empty != arr.is_empty() {
            return false;
        }
        let recovered: Vec<Option<Vec<u8>>> = interp.try_convert(value).unwrap();
        if recovered != arr {
            return false;
        }
        true
    }

    #[quickcheck]
    fn roundtrip_err(i: i64) -> bool {
        let interp = crate::interpreter().unwrap();
        let value = interp.convert(i);
        let value = value.try_into::<Vec<Value>>(&interp);
        value.is_err()
    }
}
