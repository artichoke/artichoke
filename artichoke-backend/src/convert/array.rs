use std::collections::HashMap;
use std::convert::TryFrom;

use crate::convert::float::Float;
use crate::convert::{Convert, TryConvert};
use crate::sys;
use crate::types::{Int, Ruby, Rust};
use crate::value::{Value, ValueLike};
use crate::{Artichoke, ArtichokeError};

// bail out implementation for mixed-type collections
impl Convert<Vec<Value>, Value> for Artichoke {
    fn convert(&self, value: Vec<Value>) -> Value {
        let mrb = self.0.borrow().mrb;
        let capa = Int::try_from(value.len()).unwrap_or_default();
        let array = unsafe { sys::mrb_ary_new_capa(mrb, capa) };

        for (idx, item) in value.iter().enumerate() {
            let idx = Int::try_from(idx).unwrap_or_default();
            let item = item.inner();
            unsafe {
                sys::mrb_ary_set(mrb, array, idx, item);
            }
        }
        Value::new(self, array)
    }
}

impl TryConvert<Value, Vec<Value>> for Artichoke {
    fn try_convert(&self, value: Value) -> Result<Vec<Value>, ArtichokeError> {
        let mrb = self.0.borrow().mrb;
        match value.ruby_type() {
            Ruby::Array => {
                let array = value.inner();
                let size = unsafe { sys::mrb_sys_ary_len(array) };
                let capa = usize::try_from(size).map_err(|_| ArtichokeError::ConvertToRust {
                    from: Ruby::Array,
                    to: Rust::Vec,
                })?;
                let mut elems = <Vec<Value>>::with_capacity(capa);
                for idx in 0..size {
                    let elem = Value::new(self, unsafe { sys::mrb_ary_ref(mrb, array, idx) });
                    elems.push(elem);
                }
                Ok(elems)
            }
            type_tag => Err(ArtichokeError::ConvertToRust {
                from: type_tag,
                to: Rust::Vec,
            }),
        }
    }
}

macro_rules! array_to_ruby {
    ($elem:ty) => {
        impl<'a> Convert<Vec<$elem>, Value> for Artichoke {
            fn convert(&self, value: Vec<$elem>) -> Value {
                let elems = value
                    .into_iter()
                    .map(|elem| self.convert(elem))
                    .collect::<Vec<Value>>();
                self.convert(elems)
            }
        }
    };
}

macro_rules! ruby_to_array {
    ($elem:ty) => {
        impl<'a> TryConvert<Value, Vec<$elem>> for Artichoke {
            fn try_convert(&self, value: Value) -> Result<Vec<$elem>, ArtichokeError> {
                let elems: Vec<Value> = self.try_convert(value)?;
                let mut vec = <Vec<$elem>>::with_capacity(elems.len());
                for elem in elems.into_iter() {
                    let elem = elem.try_into::<$elem>()?;
                    vec.push(elem);
                }
                Ok(vec)
            }
        }
    };
}

// Primitives
array_to_ruby!(bool);
array_to_ruby!(Vec<u8>);
array_to_ruby!(Int);
array_to_ruby!(Float);
array_to_ruby!(String);
array_to_ruby!(&'a str);

// Optional primitives
array_to_ruby!(Option<Value>);
array_to_ruby!(Option<bool>);
array_to_ruby!(Option<Vec<u8>>);
array_to_ruby!(Option<Int>);
array_to_ruby!(Option<Float>);
array_to_ruby!(Option<String>);
array_to_ruby!(Option<&'a str>);

// Array of primitives
array_to_ruby!(Vec<Value>);
array_to_ruby!(Vec<bool>);
array_to_ruby!(Vec<Vec<u8>>);
array_to_ruby!(Vec<Int>);
array_to_ruby!(Vec<Float>);
array_to_ruby!(Vec<String>);
array_to_ruby!(Vec<&'a str>);

// Array of optional primitives
array_to_ruby!(Vec<Option<Value>>);
array_to_ruby!(Vec<Option<bool>>);
array_to_ruby!(Vec<Option<Vec<u8>>>);
array_to_ruby!(Vec<Option<Int>>);
array_to_ruby!(Vec<Option<Float>>);
array_to_ruby!(Vec<Option<String>>);
array_to_ruby!(Vec<Option<&'a str>>);

// Hash of primitive keys to values
array_to_ruby!(HashMap<bool, Value>);
array_to_ruby!(HashMap<bool, bool>);
array_to_ruby!(HashMap<bool, Vec<u8>>);
array_to_ruby!(HashMap<bool, Int>);
array_to_ruby!(HashMap<bool, Float>);
array_to_ruby!(HashMap<bool, String>);
array_to_ruby!(HashMap<bool, &'a str>);
array_to_ruby!(HashMap<Vec<u8>, Value>);
array_to_ruby!(HashMap<Vec<u8>, bool>);
array_to_ruby!(HashMap<Vec<u8>, Vec<u8>>);
array_to_ruby!(HashMap<Vec<u8>, Int>);
array_to_ruby!(HashMap<Vec<u8>, Float>);
array_to_ruby!(HashMap<Vec<u8>, String>);
array_to_ruby!(HashMap<Vec<u8>, &'a str>);
array_to_ruby!(HashMap<Int, Value>);
array_to_ruby!(HashMap<Int, bool>);
array_to_ruby!(HashMap<Int, Vec<u8>>);
array_to_ruby!(HashMap<Int, Int>);
array_to_ruby!(HashMap<Int, Float>);
array_to_ruby!(HashMap<Int, String>);
array_to_ruby!(HashMap<Int, &'a str>);
array_to_ruby!(HashMap<String, Value>);
array_to_ruby!(HashMap<String, bool>);
array_to_ruby!(HashMap<String, Vec<u8>>);
array_to_ruby!(HashMap<String, Int>);
array_to_ruby!(HashMap<String, Float>);
array_to_ruby!(HashMap<String, String>);
array_to_ruby!(HashMap<String, &'a str>);
array_to_ruby!(HashMap<&'a str, Value>);
array_to_ruby!(HashMap<&'a str, bool>);
array_to_ruby!(HashMap<&'a str, Vec<u8>>);
array_to_ruby!(HashMap<&'a str, Int>);
array_to_ruby!(HashMap<&'a str, Float>);
array_to_ruby!(HashMap<&'a str, String>);
array_to_ruby!(HashMap<&'a str, &'a str>);

// Hash of optional keys to values
array_to_ruby!(HashMap<Option<bool>, Value>);
array_to_ruby!(HashMap<Option<bool>, bool>);
array_to_ruby!(HashMap<Option<bool>, Vec<u8>>);
array_to_ruby!(HashMap<Option<bool>, Int>);
array_to_ruby!(HashMap<Option<bool>, Float>);
array_to_ruby!(HashMap<Option<bool>, String>);
array_to_ruby!(HashMap<Option<bool>, &'a str>);
array_to_ruby!(HashMap<Option<Vec<u8>>, Value>);
array_to_ruby!(HashMap<Option<Vec<u8>>, bool>);
array_to_ruby!(HashMap<Option<Vec<u8>>, Vec<u8>>);
array_to_ruby!(HashMap<Option<Vec<u8>>, Int>);
array_to_ruby!(HashMap<Option<Vec<u8>>, Float>);
array_to_ruby!(HashMap<Option<Vec<u8>>, String>);
array_to_ruby!(HashMap<Option<Vec<u8>>, &'a str>);
array_to_ruby!(HashMap<Option<Int>, Value>);
array_to_ruby!(HashMap<Option<Int>, bool>);
array_to_ruby!(HashMap<Option<Int>, Vec<u8>>);
array_to_ruby!(HashMap<Option<Int>, Int>);
array_to_ruby!(HashMap<Option<Int>, Float>);
array_to_ruby!(HashMap<Option<Int>, String>);
array_to_ruby!(HashMap<Option<Int>, &'a str>);
array_to_ruby!(HashMap<Option<String>, Value>);
array_to_ruby!(HashMap<Option<String>, bool>);
array_to_ruby!(HashMap<Option<String>, Vec<u8>>);
array_to_ruby!(HashMap<Option<String>, Int>);
array_to_ruby!(HashMap<Option<String>, Float>);
array_to_ruby!(HashMap<Option<String>, String>);
array_to_ruby!(HashMap<Option<String>, &'a str>);
array_to_ruby!(HashMap<Option<&'a str>, Value>);
array_to_ruby!(HashMap<Option<&'a str>, bool>);
array_to_ruby!(HashMap<Option<&'a str>, Vec<u8>>);
array_to_ruby!(HashMap<Option<&'a str>, Int>);
array_to_ruby!(HashMap<Option<&'a str>, Float>);
array_to_ruby!(HashMap<Option<&'a str>, String>);
array_to_ruby!(HashMap<Option<&'a str>, &'a str>);

// Hash of primitive keys to optional values
array_to_ruby!(HashMap<bool, Option<Value>>);
array_to_ruby!(HashMap<bool, Option<bool>>);
array_to_ruby!(HashMap<bool, Option<Vec<u8>>>);
array_to_ruby!(HashMap<bool, Option<Int>>);
array_to_ruby!(HashMap<bool, Option<Float>>);
array_to_ruby!(HashMap<bool, Option<String>>);
array_to_ruby!(HashMap<bool, Option<&'a str>>);
array_to_ruby!(HashMap<Vec<u8>, Option<Value>>);
array_to_ruby!(HashMap<Vec<u8>, Option<bool>>);
array_to_ruby!(HashMap<Vec<u8>, Option<Vec<u8>>>);
array_to_ruby!(HashMap<Vec<u8>, Option<Int>>);
array_to_ruby!(HashMap<Vec<u8>, Option<Float>>);
array_to_ruby!(HashMap<Vec<u8>, Option<String>>);
array_to_ruby!(HashMap<Vec<u8>, Option<&'a str>>);
array_to_ruby!(HashMap<Int, Option<Value>>);
array_to_ruby!(HashMap<Int, Option<bool>>);
array_to_ruby!(HashMap<Int, Option<Vec<u8>>>);
array_to_ruby!(HashMap<Int, Option<Int>>);
array_to_ruby!(HashMap<Int, Option<Float>>);
array_to_ruby!(HashMap<Int, Option<String>>);
array_to_ruby!(HashMap<Int, Option<&'a str>>);
array_to_ruby!(HashMap<String, Option<Value>>);
array_to_ruby!(HashMap<String, Option<bool>>);
array_to_ruby!(HashMap<String, Option<Vec<u8>>>);
array_to_ruby!(HashMap<String, Option<Int>>);
array_to_ruby!(HashMap<String, Option<Float>>);
array_to_ruby!(HashMap<String, Option<String>>);
array_to_ruby!(HashMap<String, Option<&'a str>>);
array_to_ruby!(HashMap<&'a str, Option<Value>>);
array_to_ruby!(HashMap<&'a str, Option<bool>>);
array_to_ruby!(HashMap<&'a str, Option<Vec<u8>>>);
array_to_ruby!(HashMap<&'a str, Option<Int>>);
array_to_ruby!(HashMap<&'a str, Option<Float>>);
array_to_ruby!(HashMap<&'a str, Option<String>>);
array_to_ruby!(HashMap<&'a str, Option<&'a str>>);

// Hash of primitive optional keys to optional values
array_to_ruby!(HashMap<Option<bool>, Option<Value>>);
array_to_ruby!(HashMap<Option<bool>, Option<bool>>);
array_to_ruby!(HashMap<Option<bool>, Option<Vec<u8>>>);
array_to_ruby!(HashMap<Option<bool>, Option<Int>>);
array_to_ruby!(HashMap<Option<bool>, Option<Float>>);
array_to_ruby!(HashMap<Option<bool>, Option<String>>);
array_to_ruby!(HashMap<Option<bool>, Option<&'a str>>);
array_to_ruby!(HashMap<Option<Vec<u8>>, Option<Value>>);
array_to_ruby!(HashMap<Option<Vec<u8>>, Option<bool>>);
array_to_ruby!(HashMap<Option<Vec<u8>>, Option<Vec<u8>>>);
array_to_ruby!(HashMap<Option<Vec<u8>>, Option<Int>>);
array_to_ruby!(HashMap<Option<Vec<u8>>, Option<Float>>);
array_to_ruby!(HashMap<Option<Vec<u8>>, Option<String>>);
array_to_ruby!(HashMap<Option<Vec<u8>>, Option<&'a str>>);
array_to_ruby!(HashMap<Option<Int>, Option<Value>>);
array_to_ruby!(HashMap<Option<Int>, Option<bool>>);
array_to_ruby!(HashMap<Option<Int>, Option<Vec<u8>>>);
array_to_ruby!(HashMap<Option<Int>, Option<Int>>);
array_to_ruby!(HashMap<Option<Int>, Option<Float>>);
array_to_ruby!(HashMap<Option<Int>, Option<String>>);
array_to_ruby!(HashMap<Option<Int>, Option<&'a str>>);
array_to_ruby!(HashMap<Option<String>, Option<Value>>);
array_to_ruby!(HashMap<Option<String>, Option<bool>>);
array_to_ruby!(HashMap<Option<String>, Option<Vec<u8>>>);
array_to_ruby!(HashMap<Option<String>, Option<Int>>);
array_to_ruby!(HashMap<Option<String>, Option<Float>>);
array_to_ruby!(HashMap<Option<String>, Option<String>>);
array_to_ruby!(HashMap<Option<String>, Option<&'a str>>);
array_to_ruby!(HashMap<Option<&'a str>, Option<Value>>);
array_to_ruby!(HashMap<Option<&'a str>, Option<bool>>);
array_to_ruby!(HashMap<Option<&'a str>, Option<Vec<u8>>>);
array_to_ruby!(HashMap<Option<&'a str>, Option<Int>>);
array_to_ruby!(HashMap<Option<&'a str>, Option<Float>>);
array_to_ruby!(HashMap<Option<&'a str>, Option<String>>);
array_to_ruby!(HashMap<Option<&'a str>, Option<&'a str>>);

// Primitives
ruby_to_array!(bool);
ruby_to_array!(Vec<u8>);
ruby_to_array!(Int);
ruby_to_array!(Float);
ruby_to_array!(String);
ruby_to_array!(&'a str);

// Optional primitives
ruby_to_array!(Option<Value>);
ruby_to_array!(Option<bool>);
ruby_to_array!(Option<Vec<u8>>);
ruby_to_array!(Option<Int>);
ruby_to_array!(Option<Float>);
ruby_to_array!(Option<String>);
ruby_to_array!(Option<&'a str>);

// Array of primitives
ruby_to_array!(Vec<Value>);
ruby_to_array!(Vec<bool>);
ruby_to_array!(Vec<Vec<u8>>);
ruby_to_array!(Vec<Int>);
ruby_to_array!(Vec<Float>);
ruby_to_array!(Vec<String>);
ruby_to_array!(Vec<&'a str>);

// Array of optional primitives
ruby_to_array!(Vec<Option<Value>>);
ruby_to_array!(Vec<Option<bool>>);
ruby_to_array!(Vec<Option<Vec<u8>>>);
ruby_to_array!(Vec<Option<Int>>);
ruby_to_array!(Vec<Option<Float>>);
ruby_to_array!(Vec<Option<String>>);
ruby_to_array!(Vec<Option<&'a str>>);

// Hash of primitive keys to values
ruby_to_array!(HashMap<bool, Value>);
ruby_to_array!(HashMap<bool, bool>);
ruby_to_array!(HashMap<bool, Vec<u8>>);
ruby_to_array!(HashMap<bool, Int>);
ruby_to_array!(HashMap<bool, Float>);
ruby_to_array!(HashMap<bool, String>);
ruby_to_array!(HashMap<bool, &'a str>);
ruby_to_array!(HashMap<Vec<u8>, Value>);
ruby_to_array!(HashMap<Vec<u8>, bool>);
ruby_to_array!(HashMap<Vec<u8>, Vec<u8>>);
ruby_to_array!(HashMap<Vec<u8>, Int>);
ruby_to_array!(HashMap<Vec<u8>, Float>);
ruby_to_array!(HashMap<Vec<u8>, String>);
ruby_to_array!(HashMap<Vec<u8>, &'a str>);
ruby_to_array!(HashMap<Int, Value>);
ruby_to_array!(HashMap<Int, bool>);
ruby_to_array!(HashMap<Int, Vec<u8>>);
ruby_to_array!(HashMap<Int, Int>);
ruby_to_array!(HashMap<Int, Float>);
ruby_to_array!(HashMap<Int, String>);
ruby_to_array!(HashMap<Int, &'a str>);
ruby_to_array!(HashMap<String, Value>);
ruby_to_array!(HashMap<String, bool>);
ruby_to_array!(HashMap<String, Vec<u8>>);
ruby_to_array!(HashMap<String, Int>);
ruby_to_array!(HashMap<String, Float>);
ruby_to_array!(HashMap<String, String>);
ruby_to_array!(HashMap<String, &'a str>);
ruby_to_array!(HashMap<&'a str, Value>);
ruby_to_array!(HashMap<&'a str, bool>);
ruby_to_array!(HashMap<&'a str, Vec<u8>>);
ruby_to_array!(HashMap<&'a str, Int>);
ruby_to_array!(HashMap<&'a str, Float>);
ruby_to_array!(HashMap<&'a str, String>);
ruby_to_array!(HashMap<&'a str, &'a str>);

// Hash of optional keys to values
ruby_to_array!(HashMap<Option<bool>, Value>);
ruby_to_array!(HashMap<Option<bool>, bool>);
ruby_to_array!(HashMap<Option<bool>, Vec<u8>>);
ruby_to_array!(HashMap<Option<bool>, Int>);
ruby_to_array!(HashMap<Option<bool>, Float>);
ruby_to_array!(HashMap<Option<bool>, String>);
ruby_to_array!(HashMap<Option<bool>, &'a str>);
ruby_to_array!(HashMap<Option<Vec<u8>>, Value>);
ruby_to_array!(HashMap<Option<Vec<u8>>, bool>);
ruby_to_array!(HashMap<Option<Vec<u8>>, Vec<u8>>);
ruby_to_array!(HashMap<Option<Vec<u8>>, Int>);
ruby_to_array!(HashMap<Option<Vec<u8>>, Float>);
ruby_to_array!(HashMap<Option<Vec<u8>>, String>);
ruby_to_array!(HashMap<Option<Vec<u8>>, &'a str>);
ruby_to_array!(HashMap<Option<Int>, Value>);
ruby_to_array!(HashMap<Option<Int>, bool>);
ruby_to_array!(HashMap<Option<Int>, Vec<u8>>);
ruby_to_array!(HashMap<Option<Int>, Int>);
ruby_to_array!(HashMap<Option<Int>, Float>);
ruby_to_array!(HashMap<Option<Int>, String>);
ruby_to_array!(HashMap<Option<Int>, &'a str>);
ruby_to_array!(HashMap<Option<String>, Value>);
ruby_to_array!(HashMap<Option<String>, bool>);
ruby_to_array!(HashMap<Option<String>, Vec<u8>>);
ruby_to_array!(HashMap<Option<String>, Int>);
ruby_to_array!(HashMap<Option<String>, Float>);
ruby_to_array!(HashMap<Option<String>, String>);
ruby_to_array!(HashMap<Option<String>, &'a str>);
ruby_to_array!(HashMap<Option<&'a str>, Value>);
ruby_to_array!(HashMap<Option<&'a str>, bool>);
ruby_to_array!(HashMap<Option<&'a str>, Vec<u8>>);
ruby_to_array!(HashMap<Option<&'a str>, Int>);
ruby_to_array!(HashMap<Option<&'a str>, Float>);
ruby_to_array!(HashMap<Option<&'a str>, String>);
ruby_to_array!(HashMap<Option<&'a str>, &'a str>);

// Hash of primitive keys to optional values
ruby_to_array!(HashMap<bool, Option<Value>>);
ruby_to_array!(HashMap<bool, Option<bool>>);
ruby_to_array!(HashMap<bool, Option<Vec<u8>>>);
ruby_to_array!(HashMap<bool, Option<Int>>);
ruby_to_array!(HashMap<bool, Option<Float>>);
ruby_to_array!(HashMap<bool, Option<String>>);
ruby_to_array!(HashMap<bool, Option<&'a str>>);
ruby_to_array!(HashMap<Vec<u8>, Option<Value>>);
ruby_to_array!(HashMap<Vec<u8>, Option<bool>>);
ruby_to_array!(HashMap<Vec<u8>, Option<Vec<u8>>>);
ruby_to_array!(HashMap<Vec<u8>, Option<Int>>);
ruby_to_array!(HashMap<Vec<u8>, Option<Float>>);
ruby_to_array!(HashMap<Vec<u8>, Option<String>>);
ruby_to_array!(HashMap<Vec<u8>, Option<&'a str>>);
ruby_to_array!(HashMap<Int, Option<Value>>);
ruby_to_array!(HashMap<Int, Option<bool>>);
ruby_to_array!(HashMap<Int, Option<Vec<u8>>>);
ruby_to_array!(HashMap<Int, Option<Int>>);
ruby_to_array!(HashMap<Int, Option<Float>>);
ruby_to_array!(HashMap<Int, Option<String>>);
ruby_to_array!(HashMap<Int, Option<&'a str>>);
ruby_to_array!(HashMap<String, Option<Value>>);
ruby_to_array!(HashMap<String, Option<bool>>);
ruby_to_array!(HashMap<String, Option<Vec<u8>>>);
ruby_to_array!(HashMap<String, Option<Int>>);
ruby_to_array!(HashMap<String, Option<Float>>);
ruby_to_array!(HashMap<String, Option<String>>);
ruby_to_array!(HashMap<String, Option<&'a str>>);
ruby_to_array!(HashMap<&'a str, Option<Value>>);
ruby_to_array!(HashMap<&'a str, Option<bool>>);
ruby_to_array!(HashMap<&'a str, Option<Vec<u8>>>);
ruby_to_array!(HashMap<&'a str, Option<Int>>);
ruby_to_array!(HashMap<&'a str, Option<Float>>);
ruby_to_array!(HashMap<&'a str, Option<String>>);
ruby_to_array!(HashMap<&'a str, Option<&'a str>>);

// Hash of primitive optional keys to optional values
ruby_to_array!(HashMap<Option<bool>, Option<Value>>);
ruby_to_array!(HashMap<Option<bool>, Option<bool>>);
ruby_to_array!(HashMap<Option<bool>, Option<Vec<u8>>>);
ruby_to_array!(HashMap<Option<bool>, Option<Int>>);
ruby_to_array!(HashMap<Option<bool>, Option<Float>>);
ruby_to_array!(HashMap<Option<bool>, Option<String>>);
ruby_to_array!(HashMap<Option<bool>, Option<&'a str>>);
ruby_to_array!(HashMap<Option<Vec<u8>>, Option<Value>>);
ruby_to_array!(HashMap<Option<Vec<u8>>, Option<bool>>);
ruby_to_array!(HashMap<Option<Vec<u8>>, Option<Vec<u8>>>);
ruby_to_array!(HashMap<Option<Vec<u8>>, Option<Int>>);
ruby_to_array!(HashMap<Option<Vec<u8>>, Option<Float>>);
ruby_to_array!(HashMap<Option<Vec<u8>>, Option<String>>);
ruby_to_array!(HashMap<Option<Vec<u8>>, Option<&'a str>>);
ruby_to_array!(HashMap<Option<Int>, Option<Value>>);
ruby_to_array!(HashMap<Option<Int>, Option<bool>>);
ruby_to_array!(HashMap<Option<Int>, Option<Vec<u8>>>);
ruby_to_array!(HashMap<Option<Int>, Option<Int>>);
ruby_to_array!(HashMap<Option<Int>, Option<Float>>);
ruby_to_array!(HashMap<Option<Int>, Option<String>>);
ruby_to_array!(HashMap<Option<Int>, Option<&'a str>>);
ruby_to_array!(HashMap<Option<String>, Option<Value>>);
ruby_to_array!(HashMap<Option<String>, Option<bool>>);
ruby_to_array!(HashMap<Option<String>, Option<Vec<u8>>>);
ruby_to_array!(HashMap<Option<String>, Option<Int>>);
ruby_to_array!(HashMap<Option<String>, Option<Float>>);
ruby_to_array!(HashMap<Option<String>, Option<String>>);
ruby_to_array!(HashMap<Option<String>, Option<&'a str>>);
ruby_to_array!(HashMap<Option<&'a str>, Option<Value>>);
ruby_to_array!(HashMap<Option<&'a str>, Option<bool>>);
ruby_to_array!(HashMap<Option<&'a str>, Option<Vec<u8>>>);
ruby_to_array!(HashMap<Option<&'a str>, Option<Int>>);
ruby_to_array!(HashMap<Option<&'a str>, Option<Float>>);
ruby_to_array!(HashMap<Option<&'a str>, Option<String>>);
ruby_to_array!(HashMap<Option<&'a str>, Option<&'a str>>);
