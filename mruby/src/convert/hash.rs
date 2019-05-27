use std::collections::HashMap;
use std::convert::TryFrom;
use std::hash::BuildHasher;

use crate::convert::fixnum::Int;
use crate::convert::float::Float;
use crate::convert::{Error, FromMrb, TryFromMrb};
use crate::interpreter::Mrb;
use crate::sys;
use crate::value::types::{Ruby, Rust};
use crate::value::Value;

// TODO: The following comment is no longer valid since `Value`s own a pointer
// to the interpreter.
// We can't implement `PartialEq` or `Hash` on Value because accessing these
// methods on self requires an `mrb_state`. Instead, convert a Hash `Value` to
// a sequence of key-value pairs.

// bail out implementation for mixed-type collections
impl FromMrb<Vec<(Value, Value)>> for Value {
    type From = Rust;
    type To = Ruby;

    fn from_mrb(interp: &Mrb, value: Vec<(Self, Self)>) -> Self {
        // We can initalize a `Hash` with a known capacity using
        // `sys::mrb_hash_new_capa`, but doing so requires converting from
        // `usize` to `i64` which is fallible. To simplify the code and make
        // `Vec<(Value, Value)>` easier to work with, use an infallible `Hash`
        // constructor.
        let hash = unsafe { sys::mrb_hash_new(interp.borrow().mrb) };
        for (key, val) in value {
            unsafe { sys::mrb_hash_set(interp.borrow().mrb, hash, key.inner(), val.inner()) };
        }
        Self::new(interp, hash)
    }
}

impl FromMrb<Vec<(Option<Value>, Value)>> for Value {
    type From = Rust;
    type To = Ruby;

    fn from_mrb(interp: &Mrb, value: Vec<(Option<Self>, Self)>) -> Self {
        // We can initalize a `Hash` with a known capacity using
        // `sys::mrb_hash_new_capa`, but doing so requires converting from
        // `usize` to `i64` which is fallible. To simplify the code and make
        // `Vec<(Value, Value)>` easier to work with, use an infallible `Hash`
        // constructor.
        let hash = unsafe { sys::mrb_hash_new(interp.borrow().mrb) };
        for (key, val) in value {
            unsafe {
                sys::mrb_hash_set(
                    interp.borrow().mrb,
                    hash,
                    Value::from_mrb(interp, key).inner(),
                    val.inner(),
                )
            };
        }
        Self::new(interp, hash)
    }
}

impl FromMrb<Vec<(Value, Option<Value>)>> for Value {
    type From = Rust;
    type To = Ruby;

    fn from_mrb(interp: &Mrb, value: Vec<(Self, Option<Self>)>) -> Self {
        // We can initalize a `Hash` with a known capacity using
        // `sys::mrb_hash_new_capa`, but doing so requires converting from
        // `usize` to `i64` which is fallible. To simplify the code and make
        // `Vec<(Value, Value)>` easier to work with, use an infallible `Hash`
        // constructor.
        let hash = unsafe { sys::mrb_hash_new(interp.borrow().mrb) };
        for (key, val) in value {
            unsafe {
                sys::mrb_hash_set(
                    interp.borrow().mrb,
                    hash,
                    key.inner(),
                    Value::from_mrb(interp, val).inner(),
                )
            };
        }
        Self::new(interp, hash)
    }
}

impl FromMrb<Vec<(Option<Value>, Option<Value>)>> for Value {
    type From = Rust;
    type To = Ruby;

    fn from_mrb(interp: &Mrb, value: Vec<(Option<Self>, Option<Self>)>) -> Self {
        // We can initalize a `Hash` with a known capacity using
        // `sys::mrb_hash_new_capa`, but doing so requires converting from
        // `usize` to `i64` which is fallible. To simplify the code and make
        // `Vec<(Value, Value)>` easier to work with, use an infallible `Hash`
        // constructor.
        let hash = unsafe { sys::mrb_hash_new(interp.borrow().mrb) };
        for (key, val) in value {
            unsafe {
                sys::mrb_hash_set(
                    interp.borrow().mrb,
                    hash,
                    Value::from_mrb(interp, key).inner(),
                    Value::from_mrb(interp, val).inner(),
                )
            };
        }
        Self::new(interp, hash)
    }
}

impl TryFromMrb<Value> for Vec<(Value, Value)> {
    type From = Ruby;
    type To = Rust;

    unsafe fn try_from_mrb(
        interp: &Mrb,
        value: Value,
    ) -> Result<Self, Error<Self::From, Self::To>> {
        match value.ruby_type() {
            Ruby::Hash => {
                let hash = value.inner();
                let size = sys::mrb_hash_size(interp.borrow().mrb, hash);
                let keys = sys::mrb_hash_keys(interp.borrow().mrb, hash);
                let cap = usize::try_from(size).map_err(|_| Error {
                    from: Ruby::Hash,
                    to: Rust::Map,
                })?;
                let mut pairs = Self::with_capacity(cap);
                for idx in 0..size {
                    // Doing a `hash[key]` access is guaranteed to succeed since
                    // we're iterating over the keys in the hash.
                    let key = sys::mrb_ary_ref(interp.borrow().mrb, keys, idx);
                    let value = sys::mrb_hash_get(interp.borrow().mrb, hash, key);
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
        impl<S: BuildHasher + Default> TryFromMrb<Value> for HashMap<$key, $value, S> {
            type From = Ruby;
            type To = Rust;

            unsafe fn try_from_mrb(
                interp: &Mrb,
                value: Value,
            ) -> Result<Self, Error<Self::From, Self::To>> {
                let pairs = <Vec<(Value, Value)>>::try_from_mrb(interp, value)?;
                let mut hash = Self::default();
                for (key, value) in pairs.into_iter() {
                    let key = <$key>::try_from_mrb(interp, key)?;
                    let value = <$value>::try_from_mrb(&interp, value)?;
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

#[cfg(test)]
mod value {
    mod tests {
        use std::collections::HashMap;

        use crate::convert::{FromMrb, TryFromMrb};
        use crate::interpreter::Interpreter;
        use crate::value::Value;

        #[test]
        fn roundtrip_kv() {
            let interp = Interpreter::create().expect("mrb init");

            let map = vec![
                (Value::from_mrb(&interp, 1), Value::from_mrb(&interp, 2)),
                (Value::from_mrb(&interp, 7), Value::from_mrb(&interp, 8)),
            ];

            let value = Value::from_mrb(&interp, map);
            assert_eq!("{1=>2, 7=>8}", value.to_s());

            let pairs =
                unsafe { <Vec<(Value, Value)>>::try_from_mrb(&interp, value) }.expect("convert");
            let map = pairs
                .into_iter()
                .map(|(key, value)| {
                    let key = unsafe { i64::try_from_mrb(&interp, key) }.expect("convert");
                    let value = unsafe { i64::try_from_mrb(&interp, value) }.expect("convert");
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
