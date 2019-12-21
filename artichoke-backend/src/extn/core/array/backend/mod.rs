use std::any::Any;

use crate::extn::core::exception::RubyException;
use crate::value::Value;
use crate::Artichoke;

pub mod aggregate;
pub mod buffer;
pub mod fixed;
pub mod integer_range;
pub mod repeated;

pub trait ArrayType: Any {
    fn box_clone(&self) -> Box<dyn ArrayType>;

    fn gc_mark(&self, interp: &Artichoke);

    fn real_children(&self) -> usize;

    fn len(&self) -> usize;

    fn is_empty(&self) -> bool;

    fn get(&self, interp: &Artichoke, index: usize) -> Result<Value, Box<dyn RubyException>>;

    fn slice(
        &self,
        interp: &Artichoke,
        start: usize,
        len: usize,
    ) -> Result<Box<dyn ArrayType>, Box<dyn RubyException>>;

    fn set(
        &mut self,
        interp: &Artichoke,
        index: usize,
        elem: Value,
        realloc: &mut Option<Vec<Box<dyn ArrayType>>>,
    ) -> Result<(), Box<dyn RubyException>>;

    fn set_with_drain(
        &mut self,
        interp: &Artichoke,
        start: usize,
        drain: usize,
        with: Value,
        realloc: &mut Option<Vec<Box<dyn ArrayType>>>,
    ) -> Result<usize, Box<dyn RubyException>>;

    fn set_slice(
        &mut self,
        interp: &Artichoke,
        start: usize,
        drain: usize,
        with: Box<dyn ArrayType>,
        realloc: &mut Option<Vec<Box<dyn ArrayType>>>,
    ) -> Result<usize, Box<dyn RubyException>>;

    fn concat(
        &mut self,
        interp: &Artichoke,
        other: Box<dyn ArrayType>,
        realloc: &mut Option<Vec<Box<dyn ArrayType>>>,
    ) -> Result<(), Box<dyn RubyException>>;

    fn pop(
        &mut self,
        interp: &Artichoke,
        realloc: &mut Option<Vec<Box<dyn ArrayType>>>,
    ) -> Result<Value, Box<dyn RubyException>>;

    fn reverse(&mut self, interp: &Artichoke) -> Result<(), Box<dyn RubyException>>;
}

#[allow(clippy::missing_safety_doc)]
mod internal {
    downcast!(dyn super::ArrayType);
}
