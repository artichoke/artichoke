use std::mem;

use crate::extn::core::array::{backend, ArrayType};
use crate::extn::prelude::*;
use crate::gc::MrbGarbageCollection;

#[derive(Debug, Clone)]
pub struct Two(Value, Value);

impl Two {
    pub fn new(first: Value, second: Value) -> Self {
        Self(first, second)
    }
}

impl ArrayType for Two {
    fn box_clone(&self) -> Box<dyn ArrayType> {
        Box::new(self.clone())
    }

    fn gc_mark(&self, interp: &Artichoke) {
        interp.mark_value(&self.0);
        interp.mark_value(&self.1);
    }

    fn real_children(&self) -> usize {
        2
    }

    fn len(&self) -> usize {
        2
    }

    fn is_empty(&self) -> bool {
        false
    }

    fn get(&self, interp: &Artichoke, index: usize) -> Result<Value, Exception> {
        if index == 0 {
            Ok(self.0.clone())
        } else if index == 1 {
            Ok(self.1.clone())
        } else {
            Ok(interp.convert(None::<Value>))
        }
    }

    fn slice(
        &self,
        interp: &Artichoke,
        start: usize,
        len: usize,
    ) -> Result<Box<dyn ArrayType>, Exception> {
        let _ = interp;
        if start == 0 && len == 1 {
            Ok(backend::fixed::one(self.0.clone()))
        } else if start == 0 && len > 1 {
            Ok(self.box_clone())
        } else if start == 1 && len > 0 {
            Ok(backend::fixed::one(self.1.clone()))
        } else {
            Ok(backend::fixed::empty())
        }
    }

    fn set(
        &mut self,
        interp: &Artichoke,
        index: usize,
        elem: Value,
        realloc: &mut Option<Vec<Box<dyn ArrayType>>>,
    ) -> Result<(), Exception> {
        let _ = interp;
        if index == 0 {
            self.0 = elem;
        } else if index == 1 {
            self.1 = elem;
        } else if index == 2 {
            let buffer = vec![self.0.clone(), self.1.clone(), elem];
            let buffer: Box<dyn ArrayType> = Box::new(backend::buffer::Buffer::from(buffer));
            let alloc = vec![buffer];
            *realloc = Some(alloc);
        } else {
            let alloc = vec![
                self.box_clone(),
                backend::fixed::hole(index - 2),
                backend::fixed::one(elem),
            ];
            *realloc = Some(alloc);
        }
        Ok(())
    }

    fn set_with_drain(
        &mut self,
        interp: &Artichoke,
        start: usize,
        drain: usize,
        with: Value,
        realloc: &mut Option<Vec<Box<dyn ArrayType>>>,
    ) -> Result<usize, Exception> {
        let _ = interp;
        let drained = if start == 0 && drain == 0 {
            let buffer = vec![with, self.0.clone(), self.1.clone()];
            let buffer: Box<dyn ArrayType> = Box::new(backend::buffer::Buffer::from(buffer));
            let alloc = vec![buffer];
            *realloc = Some(alloc);
            0
        } else if start == 0 && drain == 1 {
            self.0 = with;
            1
        } else if start == 0 {
            let alloc = vec![backend::fixed::one(with)];
            *realloc = Some(alloc);
            2
        } else if start == 1 && drain == 0 {
            let buffer = vec![self.0.clone(), with, self.1.clone()];
            let buffer: Box<dyn ArrayType> = Box::new(backend::buffer::Buffer::from(buffer));
            let alloc = vec![buffer];
            *realloc = Some(alloc);
            0
        } else if start == 1 {
            self.1 = with;
            1
        } else if start == 2 {
            let buffer = vec![with, self.0.clone(), self.1.clone()];
            let buffer: Box<dyn ArrayType> = Box::new(backend::buffer::Buffer::from(buffer));
            let alloc = vec![buffer];
            *realloc = Some(alloc);
            0
        } else {
            let alloc = vec![
                self.box_clone(),
                backend::fixed::hole(start - 2),
                backend::fixed::one(with),
            ];
            *realloc = Some(alloc);
            0
        };
        Ok(drained)
    }

    fn set_slice(
        &mut self,
        interp: &Artichoke,
        start: usize,
        drain: usize,
        with: Box<dyn ArrayType>,
        realloc: &mut Option<Vec<Box<dyn ArrayType>>>,
    ) -> Result<usize, Exception> {
        let _ = interp;
        let (alloc, drained) = if start == 0 && drain == 0 {
            let alloc = vec![with, self.box_clone()];
            (alloc, 0)
        } else if start == 0 && drain == 1 {
            let alloc = vec![with, backend::fixed::one(self.1.clone())];
            (alloc, 1)
        } else if start == 0 {
            let alloc = vec![with];
            (alloc, 2)
        } else if start == 1 && drain == 0 {
            let alloc = vec![
                backend::fixed::one(self.0.clone()),
                with,
                backend::fixed::one(self.1.clone()),
            ];
            (alloc, 0)
        } else if start == 1 {
            let alloc = vec![backend::fixed::one(self.0.clone()), with];
            (alloc, 1)
        } else if start == 2 {
            let alloc = vec![self.box_clone(), with];
            (alloc, 0)
        } else {
            let alloc = vec![self.box_clone(), backend::fixed::hole(start - 2), with];
            (alloc, 0)
        };
        *realloc = Some(alloc);
        Ok(drained)
    }

    fn concat(
        &mut self,
        interp: &Artichoke,
        other: Box<dyn ArrayType>,
        realloc: &mut Option<Vec<Box<dyn ArrayType>>>,
    ) -> Result<(), Exception> {
        if other.len() == 0 {
            return Ok(());
        } else if other.len() < backend::buffer::BUFFER_INLINE_MAX - 1 {
            let mut buffer = Vec::with_capacity(2 + other.len());
            buffer.push(self.0.clone());
            buffer.push(self.1.clone());
            for idx in 0..other.len() {
                buffer.push(other.get(interp, idx)?);
            }
            let buffer: Box<dyn ArrayType> = Box::new(backend::buffer::Buffer::from(buffer));
            *realloc = Some(vec![buffer]);
        } else {
            *realloc = Some(vec![self.box_clone(), other]);
        }
        Ok(())
    }

    fn pop(
        &mut self,
        interp: &Artichoke,
        realloc: &mut Option<Vec<Box<dyn ArrayType>>>,
    ) -> Result<Value, Exception> {
        let _ = interp;
        *realloc = Some(vec![backend::fixed::one(self.0.clone())]);
        Ok(self.1.clone())
    }

    fn reverse(&mut self, interp: &Artichoke) -> Result<(), Exception> {
        let _ = interp;
        mem::swap(&mut self.0, &mut self.1);
        Ok(())
    }
}
