//! [`Regexp#named_captures`](https://ruby-doc.org/core-2.6.3/Regexp.html#method-i-named_captures)

use crate::convert::{FromMrb, RustBackedValue};
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
    let regex = (*borrow.regex).as_ref().ok_or(Error::Fatal)?;
    // Use a Vec of key-value pairs because insertion order matters for spec
    // compliance.
    let mut map = vec![];
    for (name, index) in regex.capture_names() {
        map.push((
            name,
            Value::from_mrb(
                interp,
                index.iter().map(|idx| i64::from(*idx)).collect::<Vec<_>>(),
            ),
        ));
    }
    Ok(Value::from_mrb(interp, map))
}
