//! Converters for nilable primitive Ruby types. Excludes collection types
//! Array and Hash.

use crate::convert::FromMrb;
use crate::interpreter::Mrb;
use crate::sys;
use crate::value::types::{Ruby, Rust};
use crate::value::Value;

mod boolean;
mod bytes;
mod fixnum;
mod float;
mod string;

pub use self::boolean::*;
pub use self::bytes::*;
pub use self::fixnum::*;
pub use self::float::*;
pub use self::string::*;

// bail out implementation for mixed-type collections
impl FromMrb<Option<Value>> for Value {
    type From = Rust;
    type To = Ruby;

    fn from_mrb(interp: &Mrb, value: Option<Self>) -> Self {
        match value {
            Some(value) => value,
            None => Self::new(interp, unsafe { sys::mrb_sys_nil_value() }),
        }
    }
}

impl FromMrb<Value> for Option<Value> {
    type From = Ruby;
    type To = Rust;

    fn from_mrb(_interp: &Mrb, value: Value) -> Self {
        match value.ruby_type() {
            Ruby::Nil => None,
            _ => Some(value),
        }
    }
}
