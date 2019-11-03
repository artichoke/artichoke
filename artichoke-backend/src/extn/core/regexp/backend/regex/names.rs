//! [`Regexp#names`](https://ruby-doc.org/core-2.6.3/Regexp.html#method-i-names)

use crate::convert::{Convert, RustBackedValue};
use crate::extn::core::regexp::Regexp;
use crate::value::Value;
use crate::Artichoke;

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub enum Error {
    Fatal,
}

pub fn method(interp: &Artichoke, value: &Value) -> Result<Value, Error> {
    let data = unsafe { Regexp::try_from_ruby(interp, value) }.map_err(|_| Error::Fatal)?;
    let borrow = data.borrow();
    let mut names = vec![];
    let regex = (*borrow.regex).as_ref().ok_or(Error::Fatal)?;
    let capture_names = regex.capture_names().collect::<Vec<_>>();
    for name in capture_names {
        if let Some(name) = name {
            if !names.contains(&name) {
                names.push(name);
            }
        }
    }
    Ok(Value::convert(&interp, names))
}
