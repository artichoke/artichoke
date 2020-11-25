use std::iter::FromIterator;

use crate::convert::{BoxUnboxVmValue, UnboxRubyError};
use crate::core::{Convert, ConvertMut, TryConvert, TryConvertMut, Value as _};
use crate::error::Error;
use crate::extn::core::array::Array;
use crate::types::{Int, Ruby, Rust};
use crate::value::Value;
use crate::Artichoke;

impl TryConvertMut<&[Value], Value> for Artichoke {
    type Error = Error;

    fn try_convert_mut(&mut self, value: &[Value]) -> Result<Value, Self::Error> {
        let ary = Array::from(value);
        Array::alloc_value(ary, self)
    }
}

impl TryConvertMut<Vec<Value>, Value> for Artichoke {
    type Error = Error;

    fn try_convert_mut(&mut self, value: Vec<Value>) -> Result<Value, Self::Error> {
        let ary = Array::from(value);
        Array::alloc_value(ary, self)
    }
}

impl TryConvertMut<&[Option<Value>], Value> for Artichoke {
    type Error = Error;

    fn try_convert_mut(&mut self, value: &[Option<Value>]) -> Result<Value, Self::Error> {
        let ary = Array::from_iter(value);
        Array::alloc_value(ary, self)
    }
}

impl TryConvertMut<Vec<Vec<u8>>, Value> for Artichoke {
    type Error = Error;

    fn try_convert_mut(&mut self, value: Vec<Vec<u8>>) -> Result<Value, Self::Error> {
        let iter = value.into_iter().map(|item| self.convert_mut(item));
        let ary = Array::from_iter(iter);
        Array::alloc_value(ary, self)
    }
}

impl TryConvertMut<Vec<&[u8]>, Value> for Artichoke {
    type Error = Error;

    fn try_convert_mut(&mut self, value: Vec<&[u8]>) -> Result<Value, Self::Error> {
        let iter = value.into_iter().map(|item| self.convert_mut(item));
        let ary = Array::from_iter(iter);
        Array::alloc_value(ary, self)
    }
}

impl TryConvertMut<&[Vec<u8>], Value> for Artichoke {
    type Error = Error;

    fn try_convert_mut(&mut self, value: &[Vec<u8>]) -> Result<Value, Self::Error> {
        let iter = value.iter().map(|item| self.convert_mut(item.as_slice()));
        let ary = Array::from_iter(iter);
        Array::alloc_value(ary, self)
    }
}

impl TryConvertMut<&[&[u8]], Value> for Artichoke {
    type Error = Error;

    fn try_convert_mut(&mut self, value: &[&[u8]]) -> Result<Value, Self::Error> {
        let iter = value.iter().copied().map(|item| self.convert_mut(item));
        let ary = Array::from_iter(iter);
        Array::alloc_value(ary, self)
    }
}

impl TryConvertMut<Vec<String>, Value> for Artichoke {
    type Error = Error;

    fn try_convert_mut(&mut self, value: Vec<String>) -> Result<Value, Self::Error> {
        let iter = value.into_iter().map(|item| self.convert_mut(item));
        let ary = Array::from_iter(iter);
        Array::alloc_value(ary, self)
    }
}

impl TryConvertMut<Vec<&str>, Value> for Artichoke {
    type Error = Error;

    fn try_convert_mut(&mut self, value: Vec<&str>) -> Result<Value, Self::Error> {
        let iter = value.into_iter().map(|item| self.convert_mut(item));
        let ary = Array::from_iter(iter);
        Array::alloc_value(ary, self)
    }
}

impl TryConvertMut<&[String], Value> for Artichoke {
    type Error = Error;

    fn try_convert_mut(&mut self, value: &[String]) -> Result<Value, Self::Error> {
        let iter = value.iter().map(|item| self.convert_mut(item.as_str()));
        let ary = Array::from_iter(iter);
        Array::alloc_value(ary, self)
    }
}

impl TryConvertMut<&[&str], Value> for Artichoke {
    type Error = Error;

    fn try_convert_mut(&mut self, value: &[&str]) -> Result<Value, Self::Error> {
        let iter = value.iter().copied().map(|item| self.convert_mut(item));
        let ary = Array::from_iter(iter);
        Array::alloc_value(ary, self)
    }
}

impl TryConvertMut<Vec<Int>, Value> for Artichoke {
    type Error = Error;

    fn try_convert_mut(&mut self, value: Vec<Int>) -> Result<Value, Self::Error> {
        let iter = value.into_iter().map(|item| self.convert(item));
        let ary = Array::from_iter(iter);
        Array::alloc_value(ary, self)
    }
}

impl TryConvertMut<&[Int], Value> for Artichoke {
    type Error = Error;

    fn try_convert_mut(&mut self, value: &[Int]) -> Result<Value, Self::Error> {
        let iter = value.iter().copied().map(|item| self.convert(item));
        let ary = Array::from_iter(iter);
        Array::alloc_value(ary, self)
    }
}

impl TryConvertMut<&[Option<Vec<u8>>], Value> for Artichoke {
    type Error = Error;

    fn try_convert_mut(&mut self, value: &[Option<Vec<u8>>]) -> Result<Value, Self::Error> {
        let iter = value.iter().map(|item| self.convert_mut(item.as_deref()));
        let ary = Array::from_iter(iter);
        Array::alloc_value(ary, self)
    }
}

impl TryConvertMut<Vec<Option<Vec<u8>>>, Value> for Artichoke {
    type Error = Error;

    fn try_convert_mut(&mut self, value: Vec<Option<Vec<u8>>>) -> Result<Value, Self::Error> {
        let iter = value.into_iter().map(|item| self.convert_mut(item));
        let ary = Array::from_iter(iter);
        Array::alloc_value(ary, self)
    }
}

impl TryConvertMut<&[Option<&[u8]>], Value> for Artichoke {
    type Error = Error;

    fn try_convert_mut(&mut self, value: &[Option<&[u8]>]) -> Result<Value, Self::Error> {
        let iter = value.iter().copied().map(|item| self.convert_mut(item));
        let ary = Array::from_iter(iter);
        Array::alloc_value(ary, self)
    }
}

impl TryConvertMut<Vec<Option<&[u8]>>, Value> for Artichoke {
    type Error = Error;

    fn try_convert_mut(&mut self, value: Vec<Option<&[u8]>>) -> Result<Value, Self::Error> {
        let iter = value.into_iter().map(|item| self.convert_mut(item));
        let ary = Array::from_iter(iter);
        Array::alloc_value(ary, self)
    }
}

impl TryConvertMut<Vec<Vec<Option<Vec<u8>>>>, Value> for Artichoke {
    type Error = Error;

    fn try_convert_mut(&mut self, value: Vec<Vec<Option<Vec<u8>>>>) -> Result<Value, Self::Error> {
        let ary = value
            .into_iter()
            .map(|item| self.try_convert_mut(item))
            .collect::<Result<Array, _>>()?;
        Array::alloc_value(ary, self)
    }
}

impl TryConvertMut<Vec<Vec<Option<&[u8]>>>, Value> for Artichoke {
    type Error = Error;

    fn try_convert_mut(&mut self, value: Vec<Vec<Option<&[u8]>>>) -> Result<Value, Self::Error> {
        let ary = value
            .into_iter()
            .map(|item| self.try_convert_mut(item))
            .collect::<Result<Array, _>>()?;
        Array::alloc_value(ary, self)
    }
}

impl TryConvertMut<&[Option<&str>], Value> for Artichoke {
    type Error = Error;

    fn try_convert_mut(&mut self, value: &[Option<&str>]) -> Result<Value, Self::Error> {
        let iter = value.iter().copied().map(|item| self.convert_mut(item));
        let ary = Array::from_iter(iter);
        Array::alloc_value(ary, self)
    }
}

impl TryConvertMut<Vec<Option<&str>>, Value> for Artichoke {
    type Error = Error;

    fn try_convert_mut(&mut self, value: Vec<Option<&str>>) -> Result<Value, Self::Error> {
        let iter = value.into_iter().map(|item| self.convert_mut(item));
        let ary = Array::from_iter(iter);
        Array::alloc_value(ary, self)
    }
}

impl TryConvertMut<Vec<Vec<Option<&str>>>, Value> for Artichoke {
    type Error = Error;

    fn try_convert_mut(&mut self, value: Vec<Vec<Option<&str>>>) -> Result<Value, Self::Error> {
        let ary = value
            .into_iter()
            .map(|item| self.try_convert_mut(item))
            .collect::<Result<Array, _>>()?;
        Array::alloc_value(ary, self)
    }
}

impl TryConvertMut<Value, Vec<Value>> for Artichoke {
    type Error = Error;

    fn try_convert_mut(&mut self, mut value: Value) -> Result<Vec<Value>, Self::Error> {
        if let Ruby::Array = value.ruby_type() {
            let array = unsafe { Array::unbox_from_value(&mut value, self)? };
            Ok(array.iter().collect())
        } else {
            Err(UnboxRubyError::new(&value, Rust::Vec).into())
        }
    }
}

impl TryConvertMut<Value, Vec<Vec<u8>>> for Artichoke {
    type Error = Error;

    fn try_convert_mut(&mut self, mut value: Value) -> Result<Vec<Vec<u8>>, Self::Error> {
        if let Ruby::Array = value.ruby_type() {
            let array = unsafe { Array::unbox_from_value(&mut value, self)? };
            array
                .iter()
                .map(|elem| self.try_convert_mut(elem))
                .collect()
        } else {
            Err(UnboxRubyError::new(&value, Rust::Vec).into())
        }
    }
}

impl TryConvertMut<Value, Vec<Option<Vec<u8>>>> for Artichoke {
    type Error = Error;

    fn try_convert_mut(&mut self, mut value: Value) -> Result<Vec<Option<Vec<u8>>>, Self::Error> {
        if let Ruby::Array = value.ruby_type() {
            let array = unsafe { Array::unbox_from_value(&mut value, self)? };
            array
                .iter()
                .map(|elem| self.try_convert_mut(elem))
                .collect()
        } else {
            Err(UnboxRubyError::new(&value, Rust::Vec).into())
        }
    }
}

impl<'a> TryConvertMut<Value, Vec<&'a [u8]>> for Artichoke {
    type Error = Error;

    fn try_convert_mut(&mut self, mut value: Value) -> Result<Vec<&'a [u8]>, Self::Error> {
        if let Ruby::Array = value.ruby_type() {
            let array = unsafe { Array::unbox_from_value(&mut value, self)? };
            array
                .iter()
                .map(|elem| self.try_convert_mut(elem))
                .collect()
        } else {
            Err(UnboxRubyError::new(&value, Rust::Vec).into())
        }
    }
}

impl<'a> TryConvertMut<Value, Vec<Option<&'a [u8]>>> for Artichoke {
    type Error = Error;

    fn try_convert_mut(&mut self, mut value: Value) -> Result<Vec<Option<&'a [u8]>>, Self::Error> {
        if let Ruby::Array = value.ruby_type() {
            let array = unsafe { Array::unbox_from_value(&mut value, self)? };
            array
                .iter()
                .map(|elem| self.try_convert_mut(elem))
                .collect()
        } else {
            Err(UnboxRubyError::new(&value, Rust::Vec).into())
        }
    }
}

impl TryConvertMut<Value, Vec<String>> for Artichoke {
    type Error = Error;

    fn try_convert_mut(&mut self, mut value: Value) -> Result<Vec<String>, Self::Error> {
        if let Ruby::Array = value.ruby_type() {
            let array = unsafe { Array::unbox_from_value(&mut value, self)? };
            array
                .iter()
                .map(|elem| self.try_convert_mut(elem))
                .collect()
        } else {
            Err(UnboxRubyError::new(&value, Rust::Vec).into())
        }
    }
}

impl TryConvertMut<Value, Vec<Option<String>>> for Artichoke {
    type Error = Error;

    fn try_convert_mut(&mut self, mut value: Value) -> Result<Vec<Option<String>>, Self::Error> {
        if let Ruby::Array = value.ruby_type() {
            let array = unsafe { Array::unbox_from_value(&mut value, self)? };
            array
                .iter()
                .map(|elem| self.try_convert_mut(elem))
                .collect()
        } else {
            Err(UnboxRubyError::new(&value, Rust::Vec).into())
        }
    }
}

impl<'a> TryConvertMut<Value, Vec<&'a str>> for Artichoke {
    type Error = Error;

    fn try_convert_mut(&mut self, mut value: Value) -> Result<Vec<&'a str>, Self::Error> {
        if let Ruby::Array = value.ruby_type() {
            let array = unsafe { Array::unbox_from_value(&mut value, self)? };
            array
                .iter()
                .map(|elem| self.try_convert_mut(elem))
                .collect()
        } else {
            Err(UnboxRubyError::new(&value, Rust::Vec).into())
        }
    }
}

impl<'a> TryConvertMut<Value, Vec<Option<&'a str>>> for Artichoke {
    type Error = Error;

    fn try_convert_mut(&mut self, mut value: Value) -> Result<Vec<Option<&'a str>>, Self::Error> {
        if let Ruby::Array = value.ruby_type() {
            let array = unsafe { Array::unbox_from_value(&mut value, self)? };
            array
                .iter()
                .map(|elem| self.try_convert_mut(elem))
                .collect()
        } else {
            Err(UnboxRubyError::new(&value, Rust::Vec).into())
        }
    }
}

impl TryConvertMut<Value, Vec<Int>> for Artichoke {
    type Error = Error;

    fn try_convert_mut(&mut self, mut value: Value) -> Result<Vec<Int>, Self::Error> {
        if let Ruby::Array = value.ruby_type() {
            let array = unsafe { Array::unbox_from_value(&mut value, self)? };
            array.iter().map(|elem| self.try_convert(elem)).collect()
        } else {
            Err(UnboxRubyError::new(&value, Rust::Vec).into())
        }
    }
}

#[cfg(test)]
mod tests {
    use quickcheck_macros::quickcheck;

    use crate::test::prelude::*;

    #[test]
    fn fail_convert() {
        let mut interp = interpreter().unwrap();
        // get a Ruby value that can't be converted to a primitive type.
        let value = interp.eval(b"Object.new").unwrap();
        let result = value.try_into_mut::<Vec<Value>>(&mut interp);
        assert!(result.is_err());
    }

    #[quickcheck]
    fn arr_int(arr: Vec<Int>) -> bool {
        let mut interp = interpreter().unwrap();
        // Borrowed converter
        let value = interp.try_convert_mut(arr.as_slice()).unwrap();
        let len = value.funcall(&mut interp, "length", &[], None).unwrap();
        let len = len.try_into::<usize>(&interp).unwrap();
        if len != arr.len() {
            return false;
        }
        let empty = value.funcall(&mut interp, "empty?", &[], None).unwrap();
        let empty = empty.try_into::<bool>(&interp).unwrap();
        if empty != arr.is_empty() {
            return false;
        }
        let recovered: Vec<Int> = interp.try_convert_mut(value).unwrap();
        if recovered != arr {
            return false;
        }
        // Owned converter
        let value = interp.try_convert_mut(arr.to_vec()).unwrap();
        let len = value.funcall(&mut interp, "length", &[], None).unwrap();
        let len = len.try_into::<usize>(&interp).unwrap();
        if len != arr.len() {
            return false;
        }
        let empty = value.funcall(&mut interp, "empty?", &[], None).unwrap();
        let empty = empty.try_into::<bool>(&interp).unwrap();
        if empty != arr.is_empty() {
            return false;
        }
        let recovered: Vec<Int> = interp.try_convert_mut(value).unwrap();
        if recovered != arr {
            return false;
        }
        true
    }

    #[quickcheck]
    fn arr_utf8(arr: Vec<String>) -> bool {
        let mut interp = interpreter().unwrap();
        // Borrowed converter
        let value = interp.try_convert_mut(arr.as_slice()).unwrap();
        let len = value.funcall(&mut interp, "length", &[], None).unwrap();
        let len = len.try_into::<usize>(&interp).unwrap();
        if len != arr.len() {
            return false;
        }
        let empty = value.funcall(&mut interp, "empty?", &[], None).unwrap();
        let empty = empty.try_into::<bool>(&interp).unwrap();
        if empty != arr.is_empty() {
            return false;
        }
        let recovered: Vec<String> = interp.try_convert_mut(value).unwrap();
        if recovered != arr {
            return false;
        }
        // Owned converter
        let value = interp.try_convert_mut(arr.to_vec()).unwrap();
        let len = value.funcall(&mut interp, "length", &[], None).unwrap();
        let len = len.try_into::<usize>(&interp).unwrap();
        if len != arr.len() {
            return false;
        }
        let empty = value.funcall(&mut interp, "empty?", &[], None).unwrap();
        let empty = empty.try_into::<bool>(&interp).unwrap();
        if empty != arr.is_empty() {
            return false;
        }
        let recovered: Vec<String> = interp.try_convert_mut(value).unwrap();
        if recovered != arr {
            return false;
        }
        true
    }

    #[quickcheck]
    fn arr_nilable_bstr(arr: Vec<Option<Vec<u8>>>) -> bool {
        let mut interp = interpreter().unwrap();
        // Borrowed converter
        let value = interp.try_convert_mut(arr.as_slice()).unwrap();
        let len = value.funcall(&mut interp, "length", &[], None).unwrap();
        let len = len.try_into::<usize>(&interp).unwrap();
        if len != arr.len() {
            return false;
        }
        let empty = value.funcall(&mut interp, "empty?", &[], None).unwrap();
        let empty = empty.try_into::<bool>(&interp).unwrap();
        if empty != arr.is_empty() {
            return false;
        }
        let recovered: Vec<Option<Vec<u8>>> = interp.try_convert_mut(value).unwrap();
        if recovered != arr {
            return false;
        }
        // Owned converter
        let value = interp.try_convert_mut(arr.to_vec()).unwrap();
        let len = value.funcall(&mut interp, "length", &[], None).unwrap();
        let len = len.try_into::<usize>(&interp).unwrap();
        if len != arr.len() {
            return false;
        }
        let empty = value.funcall(&mut interp, "empty?", &[], None).unwrap();
        let empty = empty.try_into::<bool>(&interp).unwrap();
        if empty != arr.is_empty() {
            return false;
        }
        let recovered: Vec<Option<Vec<u8>>> = interp.try_convert_mut(value).unwrap();
        if recovered != arr {
            return false;
        }
        true
    }

    #[quickcheck]
    fn roundtrip_err(i: i64) -> bool {
        let mut interp = interpreter().unwrap();
        let value = interp.convert(i);
        let value = value.try_into_mut::<Vec<Value>>(&mut interp);
        value.is_err()
    }
}
