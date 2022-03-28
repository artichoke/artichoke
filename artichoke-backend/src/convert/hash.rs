use std::collections::HashMap;

use crate::convert::{BoxUnboxVmValue, UnboxRubyError};
use crate::core::{TryConvertMut, Value as _};
use crate::error::Error;
use crate::extn::core::array::Array;
use crate::sys;
use crate::types::{Ruby, Rust};
use crate::value::Value;
use crate::Artichoke;

impl TryConvertMut<Vec<(Value, Value)>, Value> for Artichoke {
    type Error = Error;

    fn try_convert_mut(&mut self, value: Vec<(Value, Value)>) -> Result<Value, Self::Error> {
        let capa = sys::mrb_int::try_from(value.len()).unwrap_or_default();

        let hash = unsafe { self.with_ffi_boundary(|mrb| sys::mrb_hash_new_capa(mrb, capa))? };
        let hash = self.protect(Value::from(hash));

        for (key, val) in value {
            let key = key.inner();
            let val = val.inner();
            unsafe {
                self.with_ffi_boundary(|mrb| sys::mrb_hash_set(mrb, hash.inner(), key, val))?;
            }
        }
        Ok(hash)
    }
}

impl TryConvertMut<Vec<(Vec<u8>, Vec<i64>)>, Value> for Artichoke {
    type Error = Error;

    fn try_convert_mut(&mut self, value: Vec<(Vec<u8>, Vec<i64>)>) -> Result<Value, Self::Error> {
        let capa = sys::mrb_int::try_from(value.len()).unwrap_or_default();

        let hash = unsafe { self.with_ffi_boundary(|mrb| sys::mrb_hash_new_capa(mrb, capa))? };
        let hash = self.protect(Value::from(hash));

        for (key, val) in value {
            let key = self.try_convert_mut(key)?;
            let val = self.try_convert_mut(val)?;
            unsafe {
                self.with_ffi_boundary(|mrb| sys::mrb_hash_set(mrb, hash.inner(), key.inner(), val.inner()))?;
            }
        }
        Ok(hash)
    }
}

impl TryConvertMut<HashMap<Vec<u8>, Vec<u8>>, Value> for Artichoke {
    type Error = Error;

    fn try_convert_mut(&mut self, value: HashMap<Vec<u8>, Vec<u8>>) -> Result<Value, Self::Error> {
        let capa = sys::mrb_int::try_from(value.len()).unwrap_or_default();

        let hash = unsafe { self.with_ffi_boundary(|mrb| sys::mrb_hash_new_capa(mrb, capa))? };
        let hash = self.protect(Value::from(hash));

        for (key, val) in value {
            let key = self.try_convert_mut(key)?;
            let val = self.try_convert_mut(val)?;
            unsafe {
                self.with_ffi_boundary(|mrb| sys::mrb_hash_set(mrb, hash.inner(), key.inner(), val.inner()))?;
            }
        }
        Ok(hash)
    }
}

impl TryConvertMut<Option<HashMap<Vec<u8>, Option<Vec<u8>>>>, Value> for Artichoke {
    type Error = Error;

    fn try_convert_mut(&mut self, value: Option<HashMap<Vec<u8>, Option<Vec<u8>>>>) -> Result<Value, Self::Error> {
        if let Some(value) = value {
            let capa = sys::mrb_int::try_from(value.len()).unwrap_or_default();

            let hash = unsafe { self.with_ffi_boundary(|mrb| sys::mrb_hash_new_capa(mrb, capa))? };
            let hash = self.protect(Value::from(hash));

            for (key, val) in value {
                let key = self.try_convert_mut(key)?;
                let val = self.try_convert_mut(val)?;
                unsafe {
                    self.with_ffi_boundary(|mrb| sys::mrb_hash_set(mrb, hash.inner(), key.inner(), val.inner()))?;
                }
            }
            Ok(hash)
        } else {
            Ok(Value::nil())
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
                pairs.push((key, self.protect(Value::from(value))));
            }
            Ok(pairs)
        } else {
            Err(UnboxRubyError::new(&value, Rust::Map).into())
        }
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use quickcheck::quickcheck;

    use crate::test::prelude::*;

    quickcheck! {
        #[allow(clippy::needless_pass_by_value)]
        fn roundtrip_kv(hash: HashMap<Vec<u8>, Vec<u8>>) -> bool {
            let mut interp = interpreter();
            let value = interp.try_convert_mut(hash.clone()).unwrap();
            let len = value.funcall(&mut interp, "length", &[], None).unwrap();
            let len = len.try_convert_into::<usize>(&interp).unwrap();
            if len != hash.len() {
                return false;
            }
            let recovered = value.try_convert_into_mut::<Vec<(Value, Value)>>(&mut interp).unwrap();
            if recovered.len() != hash.len() {
                return false;
            }
            for (key, val) in recovered {
                let key = key.try_convert_into_mut::<Vec<u8>>(&mut interp).unwrap();
                let val = val.try_convert_into_mut::<Vec<u8>>(&mut interp).unwrap();
                match hash.get(&key) {
                    Some(retrieved) if retrieved == &val => {}
                    _ => return false,
                }
            }
            true
        }
    }
}
