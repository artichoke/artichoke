use artichoke_core::value::Value as _;
use std::num::NonZeroUsize;

use crate::extn::core::array::{backend, ArrayType};
use crate::value::Value;

mod value;

pub fn value(value: Value, count: usize) -> Box<dyn ArrayType> {
    if value.is_nil() {
        backend::fixed::hole(count)
    } else if let Some(len) = NonZeroUsize::new(count) {
        if len.get() == 1 {
            backend::fixed::one(value)
        } else if len.get() == 2 {
            backend::fixed::two(value.clone(), value)
        } else {
            Box::new(value::Value::new(value, len))
        }
    } else {
        backend::fixed::empty()
    }
}
