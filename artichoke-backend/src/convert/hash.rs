use std::collections::HashMap;
use std::convert::TryFrom;
use std::hash::BuildHasher;

use crate::convert::float::Float;
use crate::convert::{Convert, TryConvert};
#[cfg(feature = "artichoke-array")]
use crate::extn::core::array;
use crate::sys;
use crate::types::{Int, Ruby, Rust};
use crate::value::{Value, ValueLike};
use crate::{Artichoke, ArtichokeError};

// TODO: implement `PartialEq`, `Eq`, and `Hash` on `Value`, see GH-159.
// TODO: implement `Convert<HashMap<Value, Value>>`, see GH-160.

// bail out implementation for mixed-type collections
impl Convert<Vec<(Value, Value)>, Value> for Artichoke {
    fn convert(&self, value: Vec<(Value, Value)>) -> Value {
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

impl TryConvert<Value, Vec<(Value, Value)>> for Artichoke {
    fn try_convert(&self, value: Value) -> Result<Vec<(Value, Value)>, ArtichokeError> {
        let mrb = self.0.borrow().mrb;
        match value.ruby_type() {
            Ruby::Hash => {
                let hash = value.inner();
                let size = unsafe { sys::mrb_hash_size(mrb, hash) };
                let keys = unsafe { sys::mrb_hash_keys(mrb, hash) };
                let cap = usize::try_from(size).map_err(|_| ArtichokeError::ConvertToRust {
                    from: Ruby::Hash,
                    to: Rust::Map,
                })?;
                let mut pairs = <Vec<(Value, Value)>>::with_capacity(cap);
                for idx in 0..size {
                    // Doing a `hash[key]` access is guaranteed to succeed since
                    // we're iterating over the keys in the hash.
                    #[cfg(feature = "artichoke-array")]
                    let key = unsafe { array::ffi::artichoke_ary_ref(mrb, keys, idx) };
                    #[cfg(not(feature = "artichoke-array"))]
                    let key = unsafe { sys::mrb_ary_ref(mrb, keys, idx) };
                    let value = unsafe { sys::mrb_hash_get(mrb, hash, key) };
                    pairs.push((Value::new(self, key), Value::new(self, value)));
                }
                Ok(pairs)
            }
            type_tag => Err(ArtichokeError::ConvertToRust {
                from: type_tag,
                to: Rust::Map,
            }),
        }
    }
}

macro_rules! hash_to_ruby {
    ($key:ty => $value:ty) => {
        impl<'a> Convert<Vec<($key, $value)>, Value> for Artichoke {
            fn convert(&self, value: Vec<($key, $value)>) -> Value {
                let pairs = value
                    .into_iter()
                    .map(|(key, value)| {
                        let key: Value = self.convert(key);
                        let value: Value = self.convert(value);
                        (key, value)
                    })
                    .collect::<Vec<_>>();
                let result: Value = self.convert(pairs);
                result
            }
        }

        impl<'a> Convert<HashMap<$key, $value>, Value> for Artichoke {
            fn convert(&self, value: HashMap<$key, $value>) -> Value {
                let pairs = value.into_iter().collect::<Vec<($key, $value)>>();
                let result: Value = self.convert(pairs);
                result
            }
        }
    };
    (no_hash_map | $key:ty => $value:ty) => {
        impl<'a> Convert<Vec<($key, $value)>, Value> for Artichoke {
            fn convert(&self, value: Vec<($key, $value)>) -> Value {
                let pairs = value
                    .into_iter()
                    .map(|(key, value)| {
                        let key: Value = self.convert(key);
                        let value: Value = self.convert(value);
                        (key, value)
                    })
                    .collect::<Vec<_>>();
                let result: Value = self.convert(pairs);
                result
            }
        }
    };
}

macro_rules! ruby_to_hash {
    ($key:ty => $value:ty) => {
        impl<'a> TryConvert<Value, Vec<($key, $value)>> for Artichoke {
            fn try_convert(&self, value: Value) -> Result<Vec<($key, $value)>, ArtichokeError> {
                let pairs = value.try_into::<Vec<(Value, Value)>>()?;
                let mut vec = vec![];
                for (key, value) in pairs.into_iter() {
                    let key = key.try_into::<$key>()?;
                    let value = value.try_into::<$value>()?;
                    vec.push((key, value));
                }
                Ok(vec)
            }
        }

        impl<'a, S: BuildHasher + Default> TryConvert<Value, HashMap<$key, $value, S>>
            for Artichoke
        {
            fn try_convert(
                &self,
                value: Value,
            ) -> Result<HashMap<$key, $value, S>, ArtichokeError> {
                let pairs = value.try_into::<Vec<($key, $value)>>()?;
                let mut hash = HashMap::default();
                for (key, value) in pairs {
                    hash.insert(key, value);
                }
                Ok(hash)
            }
        }
    };
    (no_hash_map | $key:ty => $value:ty) => {
        impl<'a> TryConvert<Value, Vec<($key, $value)>> for Artichoke {
            fn try_convert(&self, value: Value) -> Result<Vec<($key, $value)>, ArtichokeError> {
                let pairs = value.try_into::<Vec<(Value, Value)>>()?;
                let mut vec = vec![];
                for (key, value) in pairs.into_iter() {
                    let key = key.try_into::<$key>()?;
                    let value = value.try_into::<$value>()?;
                    vec.push((key, value));
                }
                Ok(vec)
            }
        }
    };
}

macro_rules! hash_impl {
    (Value) => {
        // non nilable
        hash_to_ruby!(no_hash_map | Value => bool);
        hash_to_ruby!(no_hash_map | Value => Vec<u8>);
        hash_to_ruby!(no_hash_map | Value => Int);
        hash_to_ruby!(no_hash_map | Value => Float);
        hash_to_ruby!(no_hash_map | Value => String);
        hash_to_ruby!(no_hash_map | Value => &'a str);
        hash_to_ruby!(no_hash_map | Value => Option<bool>);
        hash_to_ruby!(no_hash_map | Value => Option<Vec<u8>>);
        hash_to_ruby!(no_hash_map | Value => Option<Int>);
        hash_to_ruby!(no_hash_map | Value => Option<Float>);
        hash_to_ruby!(no_hash_map | Value => Option<String>);
        hash_to_ruby!(no_hash_map | Value => Option<&'a str>);
        hash_to_ruby!(no_hash_map | Value => Vec<bool>);
        hash_to_ruby!(no_hash_map | Value => Vec<Vec<u8>>);
        hash_to_ruby!(no_hash_map | Value => Vec<Int>);
        hash_to_ruby!(no_hash_map | Value => Vec<Float>);
        hash_to_ruby!(no_hash_map | Value => Vec<String>);
        hash_to_ruby!(no_hash_map | Value => Vec<&'a str>);
        hash_to_ruby!(no_hash_map | Value => Vec<Option<bool>>);
        hash_to_ruby!(no_hash_map | Value => Vec<Option<Vec<u8>>>);
        hash_to_ruby!(no_hash_map | Value => Vec<Option<Int>>);
        hash_to_ruby!(no_hash_map | Value => Vec<Option<Float>>);
        hash_to_ruby!(no_hash_map | Value => Vec<Option<String>>);
        hash_to_ruby!(no_hash_map | Value => Vec<Option<&'a str>>);

        // nilable
        hash_to_ruby!(no_hash_map | Option<Value> => bool);
        hash_to_ruby!(no_hash_map | Option<Value> => Vec<u8>);
        hash_to_ruby!(no_hash_map | Option<Value> => Int);
        hash_to_ruby!(no_hash_map | Option<Value> => Float);
        hash_to_ruby!(no_hash_map | Option<Value> => String);
        hash_to_ruby!(no_hash_map | Option<Value> => &'a str);
        hash_to_ruby!(no_hash_map | Option<Value> => Option<bool>);
        hash_to_ruby!(no_hash_map | Option<Value> => Option<Vec<u8>>);
        hash_to_ruby!(no_hash_map | Option<Value> => Option<Int>);
        hash_to_ruby!(no_hash_map | Option<Value> => Option<Float>);
        hash_to_ruby!(no_hash_map | Option<Value> => Option<String>);
        hash_to_ruby!(no_hash_map | Option<Value> => Option<&'a str>);
        hash_to_ruby!(no_hash_map | Option<Value> => Vec<bool>);
        hash_to_ruby!(no_hash_map | Option<Value> => Vec<Vec<u8>>);
        hash_to_ruby!(no_hash_map | Option<Value> => Vec<Int>);
        hash_to_ruby!(no_hash_map | Option<Value> => Vec<Float>);
        hash_to_ruby!(no_hash_map | Option<Value> => Vec<String>);
        hash_to_ruby!(no_hash_map | Option<Value> => Vec<&'a str>);
        hash_to_ruby!(no_hash_map | Option<Value> => Vec<Option<bool>>);
        hash_to_ruby!(no_hash_map | Option<Value> => Vec<Option<Vec<u8>>>);
        hash_to_ruby!(no_hash_map | Option<Value> => Vec<Option<Int>>);
        hash_to_ruby!(no_hash_map | Option<Value> => Vec<Option<Float>>);
        hash_to_ruby!(no_hash_map | Option<Value> => Vec<Option<String>>);
        hash_to_ruby!(no_hash_map | Option<Value> => Vec<Option<&'a str>>);

        // nested hash
        // already implemented by hand -> hash_to_ruby!(no_hash_map | Value => Vec<(Value, Value)>);
        hash_to_ruby!(no_hash_map | Option<Value> => Vec<(Value, Value)>);

        // bail out
        // already implemented by hand -> hash_to_ruby!(no_hash_map | $key => Value);
        hash_to_ruby!(no_hash_map | Value => Option<Value>);
        hash_to_ruby!(no_hash_map | Option<Value> => Value);
        hash_to_ruby!(no_hash_map | Option<Value> => Option<Value>);

        // non nilable
        ruby_to_hash!(no_hash_map | Value => bool);
        ruby_to_hash!(no_hash_map | Value => Vec<u8>);
        ruby_to_hash!(no_hash_map | Value => Int);
        ruby_to_hash!(no_hash_map | Value => Float);
        ruby_to_hash!(no_hash_map | Value => String);
        ruby_to_hash!(no_hash_map | Value => &'a str);
        ruby_to_hash!(no_hash_map | Value => Option<bool>);
        ruby_to_hash!(no_hash_map | Value => Option<Vec<u8>>);
        ruby_to_hash!(no_hash_map | Value => Option<Int>);
        ruby_to_hash!(no_hash_map | Value => Option<Float>);
        ruby_to_hash!(no_hash_map | Value => Option<String>);
        ruby_to_hash!(no_hash_map | Value => Option<&'a str>);
        ruby_to_hash!(no_hash_map | Value => Vec<bool>);
        ruby_to_hash!(no_hash_map | Value => Vec<Vec<u8>>);
        ruby_to_hash!(no_hash_map | Value => Vec<Int>);
        ruby_to_hash!(no_hash_map | Value => Vec<Float>);
        ruby_to_hash!(no_hash_map | Value => Vec<String>);
        ruby_to_hash!(no_hash_map | Value => Vec<&'a str>);
        ruby_to_hash!(no_hash_map | Value => Vec<Option<bool>>);
        ruby_to_hash!(no_hash_map | Value => Vec<Option<Vec<u8>>>);
        ruby_to_hash!(no_hash_map | Value => Vec<Option<Int>>);
        ruby_to_hash!(no_hash_map | Value => Vec<Option<Float>>);
        ruby_to_hash!(no_hash_map | Value => Vec<Option<String>>);
        ruby_to_hash!(no_hash_map | Value => Vec<Option<&'a str>>);

        // nilable
        ruby_to_hash!(no_hash_map | Option<Value> => bool);
        ruby_to_hash!(no_hash_map | Option<Value> => Vec<u8>);
        ruby_to_hash!(no_hash_map | Option<Value> => Int);
        ruby_to_hash!(no_hash_map | Option<Value> => Float);
        ruby_to_hash!(no_hash_map | Option<Value> => String);
        ruby_to_hash!(no_hash_map | Option<Value> => &'a str);
        ruby_to_hash!(no_hash_map | Option<Value> => Option<bool>);
        ruby_to_hash!(no_hash_map | Option<Value> => Option<Vec<u8>>);
        ruby_to_hash!(no_hash_map | Option<Value> => Option<Int>);
        ruby_to_hash!(no_hash_map | Option<Value> => Option<Float>);
        ruby_to_hash!(no_hash_map | Option<Value> => Option<String>);
        ruby_to_hash!(no_hash_map | Option<Value> => Option<&'a str>);
        ruby_to_hash!(no_hash_map | Option<Value> => Vec<bool>);
        ruby_to_hash!(no_hash_map | Option<Value> => Vec<Vec<u8>>);
        ruby_to_hash!(no_hash_map | Option<Value> => Vec<Int>);
        ruby_to_hash!(no_hash_map | Option<Value> => Vec<Float>);
        ruby_to_hash!(no_hash_map | Option<Value> => Vec<String>);
        ruby_to_hash!(no_hash_map | Option<Value> => Vec<&'a str>);
        ruby_to_hash!(no_hash_map | Option<Value> => Vec<Option<bool>>);
        ruby_to_hash!(no_hash_map | Option<Value> => Vec<Option<Vec<u8>>>);
        ruby_to_hash!(no_hash_map | Option<Value> => Vec<Option<Int>>);
        ruby_to_hash!(no_hash_map | Option<Value> => Vec<Option<Float>>);
        ruby_to_hash!(no_hash_map | Option<Value> => Vec<Option<String>>);
        ruby_to_hash!(no_hash_map | Option<Value> => Vec<Option<&'a str>>);

        // nested hash
        // already implemented by hand -> ruby_to_hash!(no_hash_map | Value => Vec<(Value, Value)>);
        ruby_to_hash!(no_hash_map | Option<Value> => Vec<(Value, Value)>);

        // bail out
        // already implemented by hand -> ruby_to_hash!(no_hash_map | $key => Value);
        ruby_to_hash!(no_hash_map | Value => Option<Value>);
        ruby_to_hash!(no_hash_map | Option<Value> => Value);
        ruby_to_hash!(no_hash_map | Option<Value> => Option<Value>);
    };
    (no_hash_map | $key:tt) => {
        // non nilable
        hash_to_ruby!(no_hash_map | $key => bool);
        hash_to_ruby!(no_hash_map | $key => Vec<u8>);
        hash_to_ruby!(no_hash_map | $key => Int);
        hash_to_ruby!(no_hash_map | $key => Float);
        hash_to_ruby!(no_hash_map | $key => String);
        hash_to_ruby!(no_hash_map | $key => &'a str);
        hash_to_ruby!(no_hash_map | $key => Option<bool>);
        hash_to_ruby!(no_hash_map | $key => Option<Vec<u8>>);
        hash_to_ruby!(no_hash_map | $key => Option<Int>);
        hash_to_ruby!(no_hash_map | $key => Option<Float>);
        hash_to_ruby!(no_hash_map | $key => Option<String>);
        hash_to_ruby!(no_hash_map | $key => Option<&'a str>);
        hash_to_ruby!(no_hash_map | $key => Vec<bool>);
        hash_to_ruby!(no_hash_map | $key => Vec<Vec<u8>>);
        hash_to_ruby!(no_hash_map | $key => Vec<Int>);
        hash_to_ruby!(no_hash_map | $key => Vec<Float>);
        hash_to_ruby!(no_hash_map | $key => Vec<String>);
        hash_to_ruby!(no_hash_map | $key => Vec<&'a str>);
        hash_to_ruby!(no_hash_map | $key => Vec<Option<bool>>);
        hash_to_ruby!(no_hash_map | $key => Vec<Option<Vec<u8>>>);
        hash_to_ruby!(no_hash_map | $key => Vec<Option<Int>>);
        hash_to_ruby!(no_hash_map | $key => Vec<Option<Float>>);
        hash_to_ruby!(no_hash_map | $key => Vec<Option<String>>);
        hash_to_ruby!(no_hash_map | $key => Vec<Option<&'a str>>);

        // nilable
        hash_to_ruby!(no_hash_map | Option<$key> => bool);
        hash_to_ruby!(no_hash_map | Option<$key> => Vec<u8>);
        hash_to_ruby!(no_hash_map | Option<$key> => Int);
        hash_to_ruby!(no_hash_map | Option<$key> => Float);
        hash_to_ruby!(no_hash_map | Option<$key> => String);
        hash_to_ruby!(no_hash_map | Option<$key> => &'a str);
        hash_to_ruby!(no_hash_map | Option<$key> => Option<bool>);
        hash_to_ruby!(no_hash_map | Option<$key> => Option<Vec<u8>>);
        hash_to_ruby!(no_hash_map | Option<$key> => Option<Int>);
        hash_to_ruby!(no_hash_map | Option<$key> => Option<Float>);
        hash_to_ruby!(no_hash_map | Option<$key> => Option<String>);
        hash_to_ruby!(no_hash_map | Option<$key> => Option<&'a str>);
        hash_to_ruby!(no_hash_map | Option<$key> => Vec<bool>);
        hash_to_ruby!(no_hash_map | Option<$key> => Vec<Vec<u8>>);
        hash_to_ruby!(no_hash_map | Option<$key> => Vec<Int>);
        hash_to_ruby!(no_hash_map | Option<$key> => Vec<Float>);
        hash_to_ruby!(no_hash_map | Option<$key> => Vec<String>);
        hash_to_ruby!(no_hash_map | Option<$key> => Vec<&'a str>);
        hash_to_ruby!(no_hash_map | Option<$key> => Vec<Option<bool>>);
        hash_to_ruby!(no_hash_map | Option<$key> => Vec<Option<Vec<u8>>>);
        hash_to_ruby!(no_hash_map | Option<$key> => Vec<Option<Int>>);
        hash_to_ruby!(no_hash_map | Option<$key> => Vec<Option<Float>>);
        hash_to_ruby!(no_hash_map | Option<$key> => Vec<Option<String>>);
        hash_to_ruby!(no_hash_map | Option<$key> => Vec<Option<&'a str>>);

        // nested hash
        hash_to_ruby!(no_hash_map | $key => Vec<(Value, Value)>);
        hash_to_ruby!(no_hash_map | Option<$key> => Vec<(Value, Value)>);

        // bail out
        // already implemented by hand -> hash_to_ruby!(no_hash_map | $key => Value);
        hash_to_ruby!(no_hash_map | $key => Option<Value>);
        hash_to_ruby!(no_hash_map | Option<$key> => Value);
        hash_to_ruby!(no_hash_map | Option<$key> => Option<Value>);

        // non nilable
        ruby_to_hash!(no_hash_map | $key => bool);
        ruby_to_hash!(no_hash_map | $key => Vec<u8>);
        ruby_to_hash!(no_hash_map | $key => Int);
        ruby_to_hash!(no_hash_map | $key => Float);
        ruby_to_hash!(no_hash_map | $key => String);
        ruby_to_hash!(no_hash_map | $key => &'a str);
        ruby_to_hash!(no_hash_map | $key => Option<bool>);
        ruby_to_hash!(no_hash_map | $key => Option<Vec<u8>>);
        ruby_to_hash!(no_hash_map | $key => Option<Int>);
        ruby_to_hash!(no_hash_map | $key => Option<Float>);
        ruby_to_hash!(no_hash_map | $key => Option<String>);
        ruby_to_hash!(no_hash_map | $key => Option<&'a str>);
        ruby_to_hash!(no_hash_map | $key => Vec<bool>);
        ruby_to_hash!(no_hash_map | $key => Vec<Vec<u8>>);
        ruby_to_hash!(no_hash_map | $key => Vec<Int>);
        ruby_to_hash!(no_hash_map | $key => Vec<Float>);
        ruby_to_hash!(no_hash_map | $key => Vec<String>);
        ruby_to_hash!(no_hash_map | $key => Vec<&'a str>);
        ruby_to_hash!(no_hash_map | $key => Vec<Option<bool>>);
        ruby_to_hash!(no_hash_map | $key => Vec<Option<Vec<u8>>>);
        ruby_to_hash!(no_hash_map | $key => Vec<Option<Int>>);
        ruby_to_hash!(no_hash_map | $key => Vec<Option<Float>>);
        ruby_to_hash!(no_hash_map | $key => Vec<Option<String>>);
        ruby_to_hash!(no_hash_map | $key => Vec<Option<&'a str>>);

        // nilable
        ruby_to_hash!(no_hash_map | Option<$key> => bool);
        ruby_to_hash!(no_hash_map | Option<$key> => Vec<u8>);
        ruby_to_hash!(no_hash_map | Option<$key> => Int);
        ruby_to_hash!(no_hash_map | Option<$key> => Float);
        ruby_to_hash!(no_hash_map | Option<$key> => String);
        ruby_to_hash!(no_hash_map | Option<$key> => &'a str);
        ruby_to_hash!(no_hash_map | Option<$key> => Option<bool>);
        ruby_to_hash!(no_hash_map | Option<$key> => Option<Vec<u8>>);
        ruby_to_hash!(no_hash_map | Option<$key> => Option<Int>);
        ruby_to_hash!(no_hash_map | Option<$key> => Option<Float>);
        ruby_to_hash!(no_hash_map | Option<$key> => Option<String>);
        ruby_to_hash!(no_hash_map | Option<$key> => Option<&'a str>);
        ruby_to_hash!(no_hash_map | Option<$key> => Vec<bool>);
        ruby_to_hash!(no_hash_map | Option<$key> => Vec<Vec<u8>>);
        ruby_to_hash!(no_hash_map | Option<$key> => Vec<Int>);
        ruby_to_hash!(no_hash_map | Option<$key> => Vec<Float>);
        ruby_to_hash!(no_hash_map | Option<$key> => Vec<String>);
        ruby_to_hash!(no_hash_map | Option<$key> => Vec<&'a str>);
        ruby_to_hash!(no_hash_map | Option<$key> => Vec<Option<bool>>);
        ruby_to_hash!(no_hash_map | Option<$key> => Vec<Option<Vec<u8>>>);
        ruby_to_hash!(no_hash_map | Option<$key> => Vec<Option<Int>>);
        ruby_to_hash!(no_hash_map | Option<$key> => Vec<Option<Float>>);
        ruby_to_hash!(no_hash_map | Option<$key> => Vec<Option<String>>);
        ruby_to_hash!(no_hash_map | Option<$key> => Vec<Option<&'a str>>);

        // nested hash
        ruby_to_hash!(no_hash_map | $key => Vec<(Value, Value)>);
        ruby_to_hash!(no_hash_map | Option<$key> => Vec<(Value, Value)>);

        // bail out
        // already implemented by hand -> ruby_to_hash!(no_hash_map | $key => Value);
        ruby_to_hash!(no_hash_map | $key => Option<Value>);
        ruby_to_hash!(no_hash_map | Option<$key> => Value);
        ruby_to_hash!(no_hash_map | Option<$key> => Option<Value>);
    };
    ($key:ty) => {
        // non nilable
        hash_to_ruby!($key => bool);
        hash_to_ruby!($key => Vec<u8>);
        hash_to_ruby!($key => Int);
        hash_to_ruby!($key => Float);
        hash_to_ruby!($key => String);
        hash_to_ruby!($key => &'a str);
        hash_to_ruby!($key => Option<bool>);
        hash_to_ruby!($key => Option<Vec<u8>>);
        hash_to_ruby!($key => Option<Int>);
        hash_to_ruby!($key => Option<Float>);
        hash_to_ruby!($key => Option<String>);
        hash_to_ruby!($key => Option<&'a str>);
        hash_to_ruby!($key => Vec<bool>);
        hash_to_ruby!($key => Vec<Vec<u8>>);
        hash_to_ruby!($key => Vec<Int>);
        hash_to_ruby!($key => Vec<Float>);
        hash_to_ruby!($key => Vec<String>);
        hash_to_ruby!($key => Vec<&'a str>);
        hash_to_ruby!($key => Vec<Option<bool>>);
        hash_to_ruby!($key => Vec<Option<Vec<u8>>>);
        hash_to_ruby!($key => Vec<Option<Int>>);
        hash_to_ruby!($key => Vec<Option<Float>>);
        hash_to_ruby!($key => Vec<Option<String>>);
        hash_to_ruby!($key => Vec<Option<&'a str>>);

        // nilable
        hash_to_ruby!(Option<$key> => bool);
        hash_to_ruby!(Option<$key> => Vec<u8>);
        hash_to_ruby!(Option<$key> => Int);
        hash_to_ruby!(Option<$key> => Float);
        hash_to_ruby!(Option<$key> => String);
        hash_to_ruby!(Option<$key> => &'a str);
        hash_to_ruby!(Option<$key> => Option<bool>);
        hash_to_ruby!(Option<$key> => Option<Vec<u8>>);
        hash_to_ruby!(Option<$key> => Option<Int>);
        hash_to_ruby!(Option<$key> => Option<Float>);
        hash_to_ruby!(Option<$key> => Option<String>);
        hash_to_ruby!(Option<$key> => Option<&'a str>);
        hash_to_ruby!(Option<$key> => Vec<bool>);
        hash_to_ruby!(Option<$key> => Vec<Vec<u8>>);
        hash_to_ruby!(Option<$key> => Vec<Int>);
        hash_to_ruby!(Option<$key> => Vec<Float>);
        hash_to_ruby!(Option<$key> => Vec<String>);
        hash_to_ruby!(Option<$key> => Vec<&'a str>);
        hash_to_ruby!(Option<$key> => Vec<Option<bool>>);
        hash_to_ruby!(Option<$key> => Vec<Option<Vec<u8>>>);
        hash_to_ruby!(Option<$key> => Vec<Option<Int>>);
        hash_to_ruby!(Option<$key> => Vec<Option<Float>>);
        hash_to_ruby!(Option<$key> => Vec<Option<String>>);
        hash_to_ruby!(Option<$key> => Vec<Option<&'a str>>);

        // nested hash
        hash_to_ruby!($key => Vec<(Value, Value)>);
        hash_to_ruby!(Option<$key> => Vec<(Value, Value)>);

        // bail out
        hash_to_ruby!($key => Value);
        hash_to_ruby!($key => Option<Value>);
        hash_to_ruby!(Option<$key> => Value);
        hash_to_ruby!(Option<$key> => Option<Value>);

        // non nilable
        ruby_to_hash!($key => bool);
        ruby_to_hash!($key => Vec<u8>);
        ruby_to_hash!($key => Int);
        ruby_to_hash!($key => Float);
        ruby_to_hash!($key => String);
        ruby_to_hash!($key => &'a str);
        ruby_to_hash!($key => Option<bool>);
        ruby_to_hash!($key => Option<Vec<u8>>);
        ruby_to_hash!($key => Option<Int>);
        ruby_to_hash!($key => Option<Float>);
        ruby_to_hash!($key => Option<String>);
        ruby_to_hash!($key => Option<&'a str>);
        ruby_to_hash!($key => Vec<bool>);
        ruby_to_hash!($key => Vec<Vec<u8>>);
        ruby_to_hash!($key => Vec<Int>);
        ruby_to_hash!($key => Vec<Float>);
        ruby_to_hash!($key => Vec<String>);
        ruby_to_hash!($key => Vec<&'a str>);
        ruby_to_hash!($key => Vec<Option<bool>>);
        ruby_to_hash!($key => Vec<Option<Vec<u8>>>);
        ruby_to_hash!($key => Vec<Option<Int>>);
        ruby_to_hash!($key => Vec<Option<Float>>);
        ruby_to_hash!($key => Vec<Option<String>>);
        ruby_to_hash!($key => Vec<Option<&'a str>>);

        // nilable
        ruby_to_hash!(Option<$key> => bool);
        ruby_to_hash!(Option<$key> => Vec<u8>);
        ruby_to_hash!(Option<$key> => Int);
        ruby_to_hash!(Option<$key> => Float);
        ruby_to_hash!(Option<$key> => String);
        ruby_to_hash!(Option<$key> => &'a str);
        ruby_to_hash!(Option<$key> => Option<bool>);
        ruby_to_hash!(Option<$key> => Option<Vec<u8>>);
        ruby_to_hash!(Option<$key> => Option<Int>);
        ruby_to_hash!(Option<$key> => Option<Float>);
        ruby_to_hash!(Option<$key> => Option<String>);
        ruby_to_hash!(Option<$key> => Option<&'a str>);
        ruby_to_hash!(Option<$key> => Vec<bool>);
        ruby_to_hash!(Option<$key> => Vec<Vec<u8>>);
        ruby_to_hash!(Option<$key> => Vec<Int>);
        ruby_to_hash!(Option<$key> => Vec<Float>);
        ruby_to_hash!(Option<$key> => Vec<String>);
        ruby_to_hash!(Option<$key> => Vec<&'a str>);
        ruby_to_hash!(Option<$key> => Vec<Option<bool>>);
        ruby_to_hash!(Option<$key> => Vec<Option<Vec<u8>>>);
        ruby_to_hash!(Option<$key> => Vec<Option<Int>>);
        ruby_to_hash!(Option<$key> => Vec<Option<Float>>);
        ruby_to_hash!(Option<$key> => Vec<Option<String>>);
        ruby_to_hash!(Option<$key> => Vec<Option<&'a str>>);

        // nested hash
        ruby_to_hash!($key => Vec<(Value, Value)>);
        ruby_to_hash!(Option<$key> => Vec<(Value, Value)>);

        // bail out
        ruby_to_hash!($key => Value);
        ruby_to_hash!($key => Option<Value>);
        ruby_to_hash!(Option<$key> => Value);
        ruby_to_hash!(Option<$key> => Option<Value>);
    };
}

hash_impl!(Value);
hash_impl!(bool);
hash_impl!(Vec<u8>);
hash_impl!(no_hash_map | Float);
hash_impl!(Int);
hash_impl!(String);
hash_impl!(&'a str);

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use crate::convert::Convert;
    use crate::types::Int;
    use crate::value::{Value, ValueLike};

    #[test]
    fn roundtrip_kv() {
        let interp = crate::interpreter().expect("init");

        let map = vec![
            (interp.convert(1), interp.convert(2)),
            (interp.convert(7), interp.convert(8)),
        ];

        let value: Value = interp.convert(map);
        assert_eq!("{1=>2, 7=>8}", value.to_s());

        let pairs = value.try_into::<Vec<(Value, Value)>>().expect("convert");
        let map = pairs
            .into_iter()
            .map(|(key, value)| {
                let key = key.try_into::<Int>().expect("convert");
                let value = value.try_into::<Int>().expect("convert");
                (key, value)
            })
            .collect::<HashMap<_, _>>();
        let mut expected = HashMap::new();
        expected.insert(1, 2);
        expected.insert(7, 8);

        assert_eq!(map, expected);
    }
}
