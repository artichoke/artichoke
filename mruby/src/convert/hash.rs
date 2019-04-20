use mruby_sys::*;
use std::convert::TryFrom;

use crate::convert::fixnum::Int;
use crate::convert::Error;
use crate::{Ruby, Rust, TryFromMrb, Value};

// bail out implementation for mixed-type collections

// We can't implement `PartialEq` or `Hash` on Value because accessing these
// methods on self requires an `mrb_state`. Instead, convert a Hash `Value` to
// a sequence of key-value pairs.
impl TryFromMrb<Vec<(Value, Value)>> for Value {
    type From = Rust;
    type To = Ruby;

    unsafe fn try_from_mrb(
        mrb: *mut mrb_state,
        value: Vec<(Self, Self)>,
    ) -> Result<Self, Error<Self::From, Self::To>> {
        let size = Int::try_from(value.len()).map_err(|_| Error {
            from: Rust::Map,
            to: Ruby::Hash,
        })?;
        let hash = mrb_hash_new_capa(mrb, size);
        for (key, value) in value {
            mrb_hash_set(mrb, hash, key.inner(), value.inner());
        }
        Ok(Self::new(hash))
    }
}

#[allow(clippy::use_self)]
impl TryFromMrb<Value> for Vec<(Value, Value)> {
    type From = Ruby;
    type To = Rust;

    unsafe fn try_from_mrb(
        mrb: *mut mrb_state,
        value: Value,
    ) -> Result<Self, Error<Self::From, Self::To>> {
        match value.ruby_type() {
            Ruby::Hash => {
                let inner = value.inner();
                let keys = <Vec<Value>>::try_from_mrb(mrb, Value::new(mrb_hash_keys(mrb, inner)));
                let keys = keys.map_err(|_| Error {
                    from: Ruby::Hash,
                    to: Rust::Map,
                })?;
                let mut kv_pairs = Self::with_capacity(keys.len());
                for key in keys {
                    let value = mrb_hash_get(mrb, inner, key.inner());
                    let value = Value::new(value);
                    kv_pairs.push((key, value));
                }
                Ok(kv_pairs)
            }
            type_tag => Err(Error {
                from: type_tag,
                to: Rust::Map,
            }),
        }
    }
}

#[cfg(test)]
mod value {
    mod tests {
        use crate::*;

        #[test]
        fn roundtrip_kv() {
            unsafe {
                let mrb = Mrb::new().expect("mrb init");

                let mut map = vec![];
                let key = Value::try_from_mrb(mrb.inner().unwrap(), 1).expect("convert");
                let value = Value::try_from_mrb(mrb.inner().unwrap(), 2).expect("convert");
                map.push((key, value));
                let key = Value::try_from_mrb(mrb.inner().unwrap(), 100).expect("convert");
                let value = Value::try_from_mrb(mrb.inner().unwrap(), 1000).expect("convert");
                map.push((key, value));

                let value = Value::try_from_mrb(mrb.inner().unwrap(), map).expect("convert");

                assert_eq!("{1=>2, 100=>1000}", value.to_s(mrb.inner().unwrap()));

                let mut kv_pairs = <Vec<(Value, Value)>>::try_from_mrb(mrb.inner().unwrap(), value)
                    .expect("convert");
                let mut rt = vec![];

                let (key, value) = kv_pairs.pop().expect("index");
                let key = Int::try_from_mrb(mrb.inner().unwrap(), key).expect("convert");
                let value = Int::try_from_mrb(mrb.inner().unwrap(), value).expect("convert");
                rt.push((key, value));

                let (key, value) = kv_pairs.pop().expect("index");
                let key = Int::try_from_mrb(mrb.inner().unwrap(), key).expect("convert");
                let value = Int::try_from_mrb(mrb.inner().unwrap(), value).expect("convert");
                rt.push((key, value));

                rt.sort();
                assert_eq!(rt, vec![(1, 2), (100, 1000)]);
            }
        }
    }
}
