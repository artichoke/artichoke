use mruby_sys::*;
use std::convert::TryFrom;

use crate::value::{types, ConvertError, TryValue, Value};

fn type_error(type_tag: &types::Ruby) -> ConvertError<types::Ruby, types::Rust> {
    ConvertError {
        from: type_tag.clone(),
        to: types::Rust::Vec,
    }
}

fn convert_error(type_tag: &types::Ruby) -> ConvertError<types::Rust, types::Ruby> {
    ConvertError {
        from: types::Rust::Vec,
        to: type_tag.clone(),
    }
}

impl<T> TryValue for Vec<T>
where
    T: TryValue<Error = ConvertError<types::Rust, types::Ruby>>,
{
    type Error = T::Error;

    fn try_value(&self, mrb: *mut mrb_state) -> Result<Value, Self::Error> {
        let size = i64::try_from(self.len()).map_err(|_| convert_error(&types::Ruby::Array))?;
        let array = unsafe { mrb_ary_new_capa(mrb, size) };
        for (i, item) in self.iter().enumerate() {
            let i = i64::try_from(i).map_err(|_| convert_error(&types::Ruby::Array))?;
            let item = item.try_value(mrb)?;
            unsafe { mrb_ary_set(mrb, array, i, item.0) };
        }
        Ok(Value(array))
    }
}

impl<T> TryValue for Option<Vec<T>>
where
    T: TryValue<Error = ConvertError<types::Rust, types::Ruby>>,
{
    type Error = T::Error;

    fn try_value(&self, mrb: *mut mrb_state) -> Result<Value, Self::Error> {
        match self {
            Some(value) => Ok(value.try_value(mrb)?),
            None => Ok(Value(unsafe { mrb_sys_nil_value() })),
        }
    }
}

impl TryFrom<Value> for Vec<u8> {
    type Error = ConvertError<types::Ruby, types::Rust>;

    fn try_from(value: Value) -> Result<Self, Self::Error> {
        match value.ruby_type() {
            type_tag @ types::Ruby::Array => {
                let len = unsafe { mrb_sys_ary_len(value.0) };
                let cap = usize::try_from(len).map_err(|_| type_error(&type_tag))?;
                let mut vec = Self::with_capacity(cap);
                for i in 0..cap {
                    let idx = i64::try_from(i).map_err(|_| type_error(&type_tag))?;
                    let null: *const mrb_state = std::ptr::null();
                    let item = Value(unsafe { mrb_ary_ref(null as *mut mrb_state, value.0, idx) });
                    let item_type = item.ruby_type();
                    let value = u8::try_from(item).map_err(|_| type_error(&item_type))?;
                    vec.push(value);
                }
                Ok(vec)
            }
            type_tag => Err(type_error(&type_tag)),
        }
    }
}

impl TryFrom<Value> for Option<Vec<u8>> {
    type Error = ConvertError<types::Ruby, types::Rust>;

    fn try_from(value: Value) -> Result<Self, Self::Error> {
        match value.ruby_type() {
            types::Ruby::Nil => Ok(None),
            _ => <Vec<u8>>::try_from(value).map(Some),
        }
    }
}

impl TryFrom<Value> for Vec<u16> {
    type Error = ConvertError<types::Ruby, types::Rust>;

    fn try_from(value: Value) -> Result<Self, Self::Error> {
        match value.ruby_type() {
            type_tag @ types::Ruby::Array => {
                let len = unsafe { mrb_sys_ary_len(value.0) };
                let cap = usize::try_from(len).map_err(|_| type_error(&type_tag))?;
                let mut vec = Self::with_capacity(cap);
                for i in 0..cap {
                    let idx = i64::try_from(i).map_err(|_| type_error(&type_tag))?;
                    let null: *const mrb_state = std::ptr::null();
                    let item = Value(unsafe { mrb_ary_ref(null as *mut mrb_state, value.0, idx) });
                    let item_type = item.ruby_type();
                    let value = u16::try_from(item).map_err(|_| type_error(&item_type))?;
                    vec.push(value);
                }
                Ok(vec)
            }
            type_tag => Err(type_error(&type_tag)),
        }
    }
}

impl TryFrom<Value> for Option<Vec<u16>> {
    type Error = ConvertError<types::Ruby, types::Rust>;

    fn try_from(value: Value) -> Result<Self, Self::Error> {
        match value.ruby_type() {
            types::Ruby::Nil => Ok(None),
            _ => <Vec<u16>>::try_from(value).map(Some),
        }
    }
}

#[cfg(test)]
mod tests {
    use mruby_sys::*;

    use crate::value::vec::*;
    use crate::value::*;

    #[test]
    fn vec_u8() {
        unsafe {
            let mrb = mrb_open();
            let vec: Vec<u8> = [u8::min_value(), 100, u8::max_value()].to_vec();
            let value = vec.try_value(mrb).expect("convert");
            let roundtrip = <Vec<u8>>::try_from(value).expect("convert");
            assert_eq!(&roundtrip, &[u8::min_value(), 100, u8::max_value()]);
        }
    }

    #[test]
    fn vec_u8_empty() {
        unsafe {
            let mrb = mrb_open();
            let vec: Vec<u8> = [].to_vec();
            let value = vec.try_value(mrb).expect("convert");
            let roundtrip = <Vec<u8>>::try_from(value).expect("convert");
            assert_eq!(&roundtrip, &[]);
        }
    }

    #[test]
    fn option_vec_u8() {
        unsafe {
            let mrb = mrb_open();
            let vec: Vec<u8> = [u8::min_value(), 100, u8::max_value()].to_vec();
            let value = Some(vec).try_value(mrb).expect("convert");
            let roundtrip = <Vec<u8>>::try_from(value).expect("convert");
            assert_eq!(&roundtrip, &[u8::min_value(), 100, u8::max_value()]);
        }
    }

    #[test]
    fn option_vec_u8_empty() {
        unsafe {
            let mrb = mrb_open();
            let vec: Vec<u8> = [].to_vec();
            let value = Some(vec).try_value(mrb).expect("convert");
            let roundtrip = <Vec<u8>>::try_from(value).expect("convert");
            assert_eq!(&roundtrip, &[]);
        }
    }

    #[test]
    fn option_vec_u8_none() {
        unsafe {
            let mrb = mrb_open();
            let value = (None as Option<Vec<u8>>).try_value(mrb).expect("convert");
            let roundtrip = <Option<Vec<u8>>>::try_from(value).expect("convert");
            assert_eq!(roundtrip.is_none(), true);
        }
    }

    #[test]
    fn vec_u16() {
        unsafe {
            let mrb = mrb_open();
            let vec: Vec<u16> = [u16::min_value(), 100, u16::max_value()].to_vec();
            let value = vec.try_value(mrb).expect("convert");
            let roundtrip = <Vec<u16>>::try_from(value).expect("convert");
            assert_eq!(&roundtrip, &[u16::min_value(), 100, u16::max_value()]);
        }
    }

    #[test]
    fn vec_u16_empty() {
        unsafe {
            let mrb = mrb_open();
            let vec: Vec<u16> = [].to_vec();
            let value = vec.try_value(mrb).expect("convert");
            let roundtrip = <Vec<u16>>::try_from(value).expect("convert");
            assert_eq!(&roundtrip, &[]);
        }
    }

    #[test]
    fn option_vec_u16() {
        unsafe {
            let mrb = mrb_open();
            let vec: Vec<u16> = [u16::min_value(), 100, u16::max_value()].to_vec();
            let value = Some(vec).try_value(mrb).expect("convert");
            let roundtrip = <Vec<u16>>::try_from(value).expect("convert");
            assert_eq!(&roundtrip, &[u16::min_value(), 100, u16::max_value()]);
        }
    }

    #[test]
    fn option_vec_u16_empty() {
        unsafe {
            let mrb = mrb_open();
            let vec: Vec<u16> = [].to_vec();
            let value = Some(vec).try_value(mrb).expect("convert");
            let roundtrip = <Vec<u16>>::try_from(value).expect("convert");
            assert_eq!(&roundtrip, &[]);
        }
    }

    #[test]
    fn option_vec_u16_none() {
        unsafe {
            let mrb = mrb_open();
            let value = (None as Option<Vec<u16>>).try_value(mrb).expect("convert");
            let roundtrip = <Option<Vec<u16>>>::try_from(value).expect("convert");
            assert_eq!(roundtrip.is_none(), true);
        }
    }
}
