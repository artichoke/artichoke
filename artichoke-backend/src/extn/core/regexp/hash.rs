//! [`Regexp#hash`](https://ruby-doc.org/core-2.6.3/Regexp.html#method-i-hash)

use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

use crate::convert::{Convert, RustBackedValue};
use crate::extn::core::exception::{Fatal, RubyException};
use crate::extn::core::regexp::Regexp;
use crate::types::Int;
use crate::value::Value;
use crate::Artichoke;

pub fn method(interp: &Artichoke, value: &Value) -> Result<Value, Box<dyn RubyException>> {
    let value = unsafe { Regexp::try_from_ruby(interp, value) }.map_err(|_| {
        Fatal::new(
            interp,
            "Unable to extract Rust Regexp from Ruby Regexp receiver",
        )
    })?;
    let borrow = value.borrow();
    let mut s = DefaultHasher::new();
    borrow.hash(&mut s);
    let hash = s.finish();
    #[allow(clippy::cast_possible_wrap)]
    Ok(interp.convert(hash as Int))
}
