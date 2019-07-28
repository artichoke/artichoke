//! [`Regexp#options`](https://ruby-doc.org/core-2.6.3/Regexp.html#method-i-options)

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
    let opts = i64::from(borrow.literal_options.flags().bits()) | borrow.encoding.flags();
    Ok(Value::from_mrb(interp, opts))
}
