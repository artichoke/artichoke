use std::collections::HashMap;
use std::convert::TryFrom;
use std::hash::BuildHasher;

use crate::convert::fixnum::Int;
use crate::convert::float::Float;
use crate::convert::{Convert, Error, TryConvert};
use crate::sys;
use crate::value::types::{Ruby, Rust};
use crate::value::Value;
use crate::Artichoke;

// TODO: implement `PartialEq`, `Eq`, and `Hash` on `Value`, see GH-159.
// TODO: implement `Convert<HashMap<Value, Value>>`, see GH-160.

// bail out implementation for mixed-type collections
impl Convert<Vec<(Value, Value)>> for Value {
    type From = Rust;
    type To = Ruby;

    fn convert(interp: &Artichoke, value: Vec<(Self, Self)>) -> Self {
        let mrb = interp.borrow().mrb;
        let hash =
            unsafe { sys::mrb_hash_new_capa(mrb, i64::try_from(value.len()).unwrap_or_default()) };
        for (key, val) in value {
            unsafe { sys::mrb_hash_set(mrb, hash, key.inner(), val.inner()) };
        }
        Self::new(interp, hash)
    }
}

impl Convert<Vec<(Option<Value>, Value)>> for Value {
    type From = Rust;
    type To = Ruby;

    fn convert(interp: &Artichoke, value: Vec<(Option<Self>, Self)>) -> Self {
        let pairs = value
            .into_iter()
            .map(|(key, value)| {
                let key = Self::convert(&interp, key);
                let value = Self::convert(&interp, value);
                (key, value)
            })
            .collect::<Vec<(Self, Self)>>();
        Self::convert(interp, pairs)
    }
}

impl Convert<Vec<(Value, Option<Value>)>> for Value {
    type From = Rust;
    type To = Ruby;

    fn convert(interp: &Artichoke, value: Vec<(Self, Option<Self>)>) -> Self {
        let pairs = value
            .into_iter()
            .map(|(key, value)| {
                let key = Self::convert(&interp, key);
                let value = Self::convert(&interp, value);
                (key, value)
            })
            .collect::<Vec<(Self, Self)>>();
        Self::convert(interp, pairs)
    }
}

impl Convert<Vec<(Option<Value>, Option<Value>)>> for Value {
    type From = Rust;
    type To = Ruby;

    fn convert(interp: &Artichoke, value: Vec<(Option<Self>, Option<Self>)>) -> Self {
        let pairs = value
            .into_iter()
            .map(|(key, value)| {
                let key = Self::convert(&interp, key);
                let value = Self::convert(&interp, value);
                (key, value)
            })
            .collect::<Vec<(Self, Self)>>();
        Self::convert(interp, pairs)
    }
}

impl TryConvert<Value> for Vec<(Value, Value)> {
    type From = Ruby;
    type To = Rust;

    unsafe fn try_convert(
        interp: &Artichoke,
        value: Value,
    ) -> Result<Self, Error<Self::From, Self::To>> {
        let mrb = interp.borrow().mrb;
        match value.ruby_type() {
            Ruby::Hash => {
                let hash = value.inner();
                let size = sys::mrb_hash_size(mrb, hash);
                let keys = sys::mrb_hash_keys(mrb, hash);
                let cap = usize::try_from(size).map_err(|_| Error {
                    from: Ruby::Hash,
                    to: Rust::Map,
                })?;
                let mut pairs = Self::with_capacity(cap);
                for idx in 0..size {
                    // Doing a `hash[key]` access is guaranteed to succeed since
                    // we're iterating over the keys in the hash.
                    let key = sys::mrb_ary_ref(mrb, keys, idx);
                    let value = sys::mrb_hash_get(mrb, hash, key);
                    pairs.push((Value::new(interp, key), Value::new(interp, value)));
                }
                Ok(pairs)
            }
            type_tag => Err(Error {
                from: type_tag,
                to: Rust::Map,
            }),
        }
    }
}

macro_rules! hash_converter {
    ($key:ty => $value:ty) => {
        #[allow(clippy::use_self)]
        impl Convert<Vec<($key, $value)>> for Value {
            type From = Rust;
            type To = Ruby;

            fn convert(interp: &Artichoke, value: Vec<($key, $value)>) -> Self {
                let pairs = value
                    .into_iter()
                    .map(|(key, value)| {
                        let key = Self::convert(&interp, key);
                        let value = Self::convert(&interp, value);
                        (key, value)
                    })
                    .collect::<Vec<(Self, Self)>>();
                Self::convert(interp, pairs)
            }
        }

        #[allow(clippy::use_self)]
        impl Convert<HashMap<$key, $value>> for Value {
            type From = Rust;
            type To = Ruby;

            fn convert(interp: &Artichoke, value: HashMap<$key, $value>) -> Self {
                let pairs = value.into_iter().collect::<Vec<($key, $value)>>();
                Self::convert(interp, pairs)
            }
        }

        impl<S: BuildHasher + Default> TryConvert<Value> for HashMap<$key, $value, S> {
            type From = Ruby;
            type To = Rust;

            unsafe fn try_convert(
                interp: &Artichoke,
                value: Value,
            ) -> Result<Self, Error<Self::From, Self::To>> {
                let pairs = <Vec<(Value, Value)>>::try_convert(interp, value)?;
                let mut hash = Self::default();
                for (key, value) in pairs.into_iter() {
                    let key = <$key>::try_convert(interp, key)?;
                    let value = <$value>::try_convert(&interp, value)?;
                    hash.insert(key, value);
                }
                Ok(hash)
            }
        }
    };
}

macro_rules! hash_impl {
    ($key:ty) => {
        // non nilable
        hash_converter!($key => bool);
        hash_converter!($key => Vec<u8>);
        hash_converter!($key => Int);
        hash_converter!($key => Float);
        hash_converter!($key => String);
        hash_converter!($key => Option<bool>);
        hash_converter!($key => Option<Vec<u8>>);
        hash_converter!($key => Option<Int>);
        hash_converter!($key => Option<Float>);
        hash_converter!($key => Option<String>);
        hash_converter!($key => Vec<bool>);
        hash_converter!($key => Vec<Vec<u8>>);
        hash_converter!($key => Vec<Int>);
        hash_converter!($key => Vec<Float>);
        hash_converter!($key => Vec<String>);
        hash_converter!($key => Vec<Option<bool>>);
        hash_converter!($key => Vec<Option<Vec<u8>>>);
        hash_converter!($key => Vec<Option<Int>>);
        hash_converter!($key => Vec<Option<Float>>);
        hash_converter!($key => Vec<Option<String>>);

        // nilable
        hash_converter!(Option<$key> => bool);
        hash_converter!(Option<$key> => Vec<u8>);
        hash_converter!(Option<$key> => Int);
        hash_converter!(Option<$key> => Float);
        hash_converter!(Option<$key> => String);
        hash_converter!(Option<$key> => Option<bool>);
        hash_converter!(Option<$key> => Option<Vec<u8>>);
        hash_converter!(Option<$key> => Option<Int>);
        hash_converter!(Option<$key> => Option<Float>);
        hash_converter!(Option<$key> => Option<String>);
        hash_converter!(Option<$key> => Vec<bool>);
        hash_converter!(Option<$key> => Vec<Vec<u8>>);
        hash_converter!(Option<$key> => Vec<Int>);
        hash_converter!(Option<$key> => Vec<Float>);
        hash_converter!(Option<$key> => Vec<String>);
        hash_converter!(Option<$key> => Vec<Option<bool>>);
        hash_converter!(Option<$key> => Vec<Option<Vec<u8>>>);
        hash_converter!(Option<$key> => Vec<Option<Int>>);
        hash_converter!(Option<$key> => Vec<Option<Float>>);
        hash_converter!(Option<$key> => Vec<Option<String>>);

        // nested hash
        hash_converter!($key => Vec<(Value, Value)>);
        hash_converter!(Option<$key> => Vec<(Value, Value)>);

        // bail out
        hash_converter!($key => Value);
        hash_converter!($key => Option<Value>);
        hash_converter!(Option<$key> => Value);
        hash_converter!(Option<$key> => Option<Value>);
    };
}

// Primitive keys except for `f64` because `f64` is not hashable.
hash_impl!(bool);
hash_impl!(Vec<u8>);
hash_impl!(Int);
hash_impl!(String);

#[allow(clippy::use_self)]
impl Convert<Vec<(&str, Value)>> for Value {
    type From = Rust;
    type To = Ruby;

    fn convert(interp: &Artichoke, value: Vec<(&str, Value)>) -> Self {
        let pairs = value
            .into_iter()
            .map(|(key, value)| {
                let key = Self::convert(&interp, key);
                let value = Self::convert(&interp, value);
                (key, value)
            })
            .collect::<Vec<(Self, Self)>>();
        Self::convert(interp, pairs)
    }
}

impl Convert<HashMap<&str, Self>> for Value {
    type From = Rust;
    type To = Ruby;

    fn convert(interp: &Artichoke, value: HashMap<&str, Self>) -> Self {
        let pairs = value.into_iter().collect::<Vec<(&str, Self)>>();
        Self::convert(interp, pairs)
    }
}

#[cfg(test)]
mod value {
    mod tests {
        use std::collections::HashMap;

        use crate::convert::{Convert, TryConvert};
        use crate::value::Value;

        #[test]
        fn roundtrip_kv() {
            let interp = crate::interpreter().expect("mrb init");

            let map = vec![
                (Value::convert(&interp, 1), Value::convert(&interp, 2)),
                (Value::convert(&interp, 7), Value::convert(&interp, 8)),
            ];

            let value = Value::convert(&interp, map);
            assert_eq!("{1=>2, 7=>8}", value.to_s());

            let pairs =
                unsafe { <Vec<(Value, Value)>>::try_convert(&interp, value) }.expect("convert");
            let map = pairs
                .into_iter()
                .map(|(key, value)| {
                    let key = unsafe { i64::try_convert(&interp, key) }.expect("convert");
                    let value = unsafe { i64::try_convert(&interp, value) }.expect("convert");
                    (key, value)
                })
                .collect::<HashMap<_, _>>();
            let mut expected = HashMap::new();
            expected.insert(1, 2);
            expected.insert(7, 8);

            assert_eq!(map, expected);
        }
    }
}
