use std::convert::TryFrom;

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
