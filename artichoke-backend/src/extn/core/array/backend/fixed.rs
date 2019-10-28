use std::num::NonZeroUsize;

use crate::extn::core::array::ArrayType;
use crate::value::Value;

mod empty;
mod hole;
mod one;
mod two;

pub fn empty() -> Box<dyn ArrayType> {
    Box::new(empty::Empty::new())
}

pub fn hole(len: usize) -> Box<dyn ArrayType> {
    if let Some(len) = NonZeroUsize::new(len) {
        Box::new(hole::Hole::new(len))
    } else {
        Box::new(empty::Empty::new())
    }
}

pub fn one(elem: Value) -> Box<dyn ArrayType> {
    Box::new(one::One::new(elem))
}

pub fn two(first: Value, second: Value) -> Box<dyn ArrayType> {
    Box::new(two::Two::new(first, second))
}
