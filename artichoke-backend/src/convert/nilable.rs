//! Converters for nilable primitive Ruby types. Excludes collection types
//! Array and Hash.

use crate::convert::Convert;
use crate::sys;
use crate::value::types::{Ruby, Rust};
use crate::value::Value;
use crate::Mrb;

mod array;
mod boolean;
mod bytes;
mod fixnum;
mod float;
mod string;

pub use self::array::*;
pub use self::boolean::*;
pub use self::bytes::*;
pub use self::fixnum::*;
pub use self::float::*;
pub use self::string::*;

// bail out implementation for mixed-type collections
impl Convert<Option<Value>> for Value {
    type From = Rust;
    type To = Ruby;

    fn convert(interp: &Mrb, value: Option<Self>) -> Self {
        match value {
            Some(value) => value,
            None => Self::new(interp, unsafe { sys::mrb_sys_nil_value() }),
        }
    }
}

impl Convert<Value> for Option<Value> {
    type From = Ruby;
    type To = Rust;

    fn convert(_interp: &Mrb, value: Value) -> Self {
        match value.ruby_type() {
            Ruby::Nil => None,
            _ => Some(value),
        }
    }
}
