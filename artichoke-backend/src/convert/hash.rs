use std::collections::HashMap;
use std::convert::TryFrom;

use crate::convert::{BoxUnboxVmValue, UnboxRubyError};
use crate::core::{ConvertMut, TryConvertMut, Value as _};
use crate::error::Error;
use crate::extn::core::array::Array;
use crate::sys;
use crate::types::{Int, Ruby, Rust};
use crate::value::Value;
use crate::Artichoke;

// TODO(GH-28): implement `PartialEq`, `Eq`, and `Hash` on `Value`.
// TODO(GH-29): implement `Convert<HashMap<Value, Value>>`.

impl ConvertMut<Vec<(Value, Value)>, Value> for Artichoke {
    fn convert_mut(&mut self, value: Vec<(Value, Value)>) -> Value {
        let capa = Int::try_from(value.len()).unwrap_or_default();
        let hash = unsafe { self.with_ffi_boundary(|mrb| sys::mrb_hash_new_capa(mrb, capa)) };
        let hash = self.protect(Value::from(hash.unwrap()));
        for (key, val) in value {
            let key = key.inner();
            let val = val.inner();
            let _ = unsafe { self.with_ffi_boundary(|mrb| sys::mrb_hash_set(mrb, hash.inner(), key, val)) };
        }
        hash
    }
}

impl TryConvertMut<Vec<(Vec<u8>, Vec<Int>)>, Value> for Artichoke {
    type Error = Error;

    fn try_convert_mut(&mut self, value: Vec<(Vec<u8>, Vec<Int>)>) -> Result<Value, Self::Error> {
        let capa = Int::try_from(value.len()).unwrap_or_default();
        let hash = unsafe { self.with_ffi_boundary(|mrb| sys::mrb_hash_new_capa(mrb, capa)) };
        let hash = self.protect(Value::from(hash.unwrap()));
        for (key, val) in value {
            let key = self.try_convert_mut(key)?;
            let val = self.try_convert_mut(val)?;
            let _ = unsafe {
                self.with_ffi_boundary(|mrb| sys::mrb_hash_set(mrb, hash.inner(), key.inner(), val.inner()))
            };
        }
        Ok(hash)
    }
}

impl ConvertMut<HashMap<Vec<u8>, Vec<u8>>, Value> for Artichoke {
    fn convert_mut(&mut self, value: HashMap<Vec<u8>, Vec<u8>>) -> Value {
        let capa = Int::try_from(value.len()).unwrap_or_default();
        let hash = unsafe { self.with_ffi_boundary(|mrb| sys::mrb_hash_new_capa(mrb, capa)) };
        let hash = self.protect(Value::from(hash.unwrap()));
        for (key, val) in value {
            let key = self.convert_mut(key);
            let val = self.convert_mut(val);
            let _ = unsafe {
                self.with_ffi_boundary(|mrb| sys::mrb_hash_set(mrb, hash.inner(), key.inner(), val.inner()))
            };
        }
        hash
    }
}

impl ConvertMut<Option<HashMap<Vec<u8>, Option<Vec<u8>>>>, Value> for Artichoke {
    fn convert_mut(&mut self, value: Option<HashMap<Vec<u8>, Option<Vec<u8>>>>) -> Value {
        if let Some(value) = value {
            let capa = Int::try_from(value.len()).unwrap_or_default();
            let hash = unsafe { self.with_ffi_boundary(|mrb| sys::mrb_hash_new_capa(mrb, capa)) };
            let hash = self.protect(Value::from(hash.unwrap()));
            for (key, val) in value {
                let key = self.convert_mut(key);
                let val = self.convert_mut(val);
                let _ = unsafe {
                    self.with_ffi_boundary(|mrb| sys::mrb_hash_set(mrb, hash.inner(), key.inner(), val.inner()))
                };
            }
            hash
        } else {
            Value::nil()
        }
    }
}

impl TryConvertMut<Value, Vec<(Value, Value)>> for Artichoke {
    type Error = Error;

    fn try_convert_mut(&mut self, value: Value) -> Result<Vec<(Value, Value)>, Self::Error> {
        if let Ruby::Hash = value.ruby_type() {
            let hash = value.inner();
            let keys = unsafe { self.with_ffi_boundary(|mrb| sys::mrb_hash_keys(mrb, hash))? };

            let mut keys = self.protect(Value::from(keys));
            let array = unsafe { Array::unbox_from_value(&mut keys, self) }?;

            let mut pairs = Vec::with_capacity(array.len());
            for key in &*array {
                let value = unsafe { self.with_ffi_boundary(|mrb| sys::mrb_hash_get(mrb, hash, key.inner()))? };
                pairs.push((key, self.protect(Value::from(value))))
            }
            Ok(pairs)
        } else {
            Err(UnboxRubyError::new(&value, Rust::Map).into())
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
        let mut interp = interpreter().unwrap();
        let value = interp.convert_mut(hash.clone());
        let len = value.funcall(&mut interp, "length", &[], None).unwrap();
        let len = len.try_into::<usize>(&interp).unwrap();
        if len != hash.len() {
            return false;
        }
        let recovered = value.try_into_mut::<Vec<(Value, Value)>>(&mut interp).unwrap();
        if recovered.len() != hash.len() {
            return false;
        }
        for (key, val) in recovered {
            let key = key.try_into_mut::<Vec<u8>>(&mut interp).unwrap();
            let val = val.try_into_mut::<Vec<u8>>(&mut interp).unwrap();
            match hash.get(&key) {
                Some(retrieved) if retrieved == &val => {}
                _ => return false,
            }
        }
        true
    }
}
