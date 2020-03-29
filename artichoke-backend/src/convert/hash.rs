use std::collections::HashMap;
use std::convert::TryFrom;

use crate::convert::{RustBackedValue, UnboxRubyError};
use crate::exception::Exception;
use crate::extn::core::array::Array;
use crate::sys;
use crate::types::{Int, Ruby, Rust};
use crate::value::Value;
use crate::{Artichoke, ConvertMut, TryConvert};

// TODO: implement `PartialEq`, `Eq`, and `Hash` on `Value`, see GH-159.
// TODO: implement `Convert<HashMap<Value, Value>>`, see GH-160.

impl ConvertMut<Vec<(Value, Value)>, Value> for Artichoke {
    fn convert_mut(&mut self, value: Vec<(Value, Value)>) -> Value {
        let mrb = self.0.borrow().mrb;
        let capa = Int::try_from(value.len()).unwrap_or_default();
        let hash = unsafe { sys::mrb_hash_new_capa(mrb, capa) };
        for (key, val) in value {
            let key = key.inner();
            let val = val.inner();
            unsafe { sys::mrb_hash_set(mrb, hash, key, val) };
        }
        Value::new(self, hash)
    }
}

impl ConvertMut<Vec<(Vec<u8>, Vec<Int>)>, Value> for Artichoke {
    fn convert_mut(&mut self, value: Vec<(Vec<u8>, Vec<Int>)>) -> Value {
        let mrb = self.0.borrow().mrb;
        let capa = Int::try_from(value.len()).unwrap_or_default();
        let hash = unsafe { sys::mrb_hash_new_capa(mrb, capa) };
        for (key, val) in value {
            let key = self.convert_mut(key).inner();
            let val = self.convert_mut(val).inner();
            unsafe { sys::mrb_hash_set(mrb, hash, key, val) };
        }
        Value::new(self, hash)
    }
}

impl ConvertMut<HashMap<Vec<u8>, Vec<u8>>, Value> for Artichoke {
    fn convert_mut(&mut self, value: HashMap<Vec<u8>, Vec<u8>>) -> Value {
        let mrb = self.0.borrow().mrb;
        let capa = Int::try_from(value.len()).unwrap_or_default();
        let hash = unsafe { sys::mrb_hash_new_capa(mrb, capa) };
        for (key, val) in value {
            let key = self.convert_mut(key).inner();
            let val = self.convert_mut(val).inner();
            unsafe { sys::mrb_hash_set(mrb, hash, key, val) };
        }
        Value::new(self, hash)
    }
}

impl ConvertMut<Option<HashMap<Vec<u8>, Option<Vec<u8>>>>, Value> for Artichoke {
    fn convert_mut(&mut self, value: Option<HashMap<Vec<u8>, Option<Vec<u8>>>>) -> Value {
        if let Some(value) = value {
            let mrb = self.0.borrow().mrb;
            let capa = Int::try_from(value.len()).unwrap_or_default();
            let hash = unsafe { sys::mrb_hash_new_capa(mrb, capa) };
            for (key, val) in value {
                let key = self.convert_mut(key).inner();
                let val = self.convert_mut(val).inner();
                unsafe { sys::mrb_hash_set(mrb, hash, key, val) };
            }
            Value::new(self, hash)
        } else {
            Value::new(self, unsafe { sys::mrb_sys_nil_value() })
        }
    }
}

impl TryConvert<Value, Vec<(Value, Value)>> for Artichoke {
    type Error = Exception;

    fn try_convert(&self, value: Value) -> Result<Vec<(Value, Value)>, Self::Error> {
        if let Ruby::Hash = value.ruby_type() {
            let mrb = self.0.borrow().mrb;
            let hash = value.inner();
            let keys = unsafe { sys::mrb_hash_keys(mrb, hash) };

            let keys = Value::new(self, keys);
            let array = unsafe { Array::try_from_ruby(self, &keys) }?;
            let borrow = array.borrow();

            let pairs = borrow
                .as_vec(self)
                .into_iter()
                .map(|key| {
                    let value = unsafe { sys::mrb_hash_get(mrb, hash, key.inner()) };
                    (key, Value::new(self, value))
                })
                .collect::<Vec<_>>();
            Ok(pairs)
        } else {
            Err(Exception::from(UnboxRubyError::new(&value, Rust::Map)))
        }
    }
}

#[cfg(test)]
mod tests {
    use quickcheck_macros::quickcheck;
    use std::collections::HashMap;

    use crate::test::prelude::*;

    #[quickcheck]
    fn roundtrip_kv(hash: HashMap<Vec<u8>, Vec<u8>>) -> bool {
        let mut interp = crate::interpreter().unwrap();
        let value = interp.convert_mut(hash.clone());
        let len = value.funcall::<usize>("length", &[], None).unwrap();
        if len != hash.len() {
            return false;
        }
        let recovered = value.try_into::<Vec<(Value, Value)>>().unwrap();
        if recovered.len() != hash.len() {
            return false;
        }
        for (key, val) in recovered {
            let key = key.try_into::<Vec<u8>>().unwrap();
            let val = val.try_into::<Vec<u8>>().unwrap();
            match hash.get(&key) {
                Some(retrieved) if retrieved == &val => {}
                _ => return false,
            }
        }
        true
    }
}
