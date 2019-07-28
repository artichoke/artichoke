//! [`Regexp#hash`](https://ruby-doc.org/core-2.6.3/Regexp.html#method-i-hash)

use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

use crate::convert::{Convert, RustBackedValue};
use crate::extn::core::regexp::Regexp;
use crate::value::Value;
use crate::Mrb;

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub enum Error {
    Fatal,
}

pub fn method(interp: &Mrb, value: &Value) -> Result<Value, Error> {
    let data = unsafe { Regexp::try_from_ruby(interp, value) }.map_err(|_| Error::Fatal)?;
    let borrow = data.borrow();
    let mut s = DefaultHasher::new();
    borrow.hash(&mut s);
    let hash = s.finish();
    #[allow(clippy::cast_possible_wrap)]
    Ok(Value::from_mrb(interp, hash as i64))
}
