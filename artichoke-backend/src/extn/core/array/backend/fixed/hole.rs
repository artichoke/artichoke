use std::num::NonZeroUsize;

use crate::convert::Convert;
use crate::extn::core::array::{backend, ArrayType};
use crate::extn::core::exception::RubyException;
use crate::value::Value;
use crate::Artichoke;

#[derive(Debug, Clone, Copy)]
pub struct Hole(NonZeroUsize);

impl Hole {
    pub fn new(len: NonZeroUsize) -> Self {
        Self(len)
    }
}

impl ArrayType for Hole {
    fn box_clone(&self) -> Box<dyn ArrayType> {
        Box::new(*self)
    }

    fn gc_mark(&self, interp: &Artichoke) {
        let _ = interp;
    }

    fn real_children(&self) -> usize {
        0
    }

    fn len(&self) -> usize {
        self.0.get()
    }

    fn is_empty(&self) -> bool {
        false
    }

    fn get(&self, interp: &Artichoke, index: usize) -> Result<Value, Box<dyn RubyException>> {
        let _ = index;
        Ok(interp.convert(None::<Value>))
    }

    fn slice(
        &self,
        interp: &Artichoke,
        start: usize,
        len: usize,
    ) -> Result<Box<dyn ArrayType>, Box<dyn RubyException>> {
        let _ = interp;
        if start < self.0.get() {
            if start + len < self.0.get() {
                Ok(backend::fixed::hole(len))
            } else {
                Ok(backend::fixed::hole(self.0.get() - start))
            }
        } else {
            Ok(backend::fixed::empty())
        }
    }

    fn set(
        &self,
        interp: &Artichoke,
        index: usize,
        elem: Value,
        realloc: &mut Option<Vec<Box<dyn ArrayType>>>,
    ) -> Result<(), Box<dyn RubyException>> {
        let _ = interp;
        let len = self.0.get();
        let alloc = if index == 0 && len == 1 {
            vec![backend::fixed::one(elem)]
        } else if index == 0 && len == 2 {
            vec![backend::fixed::two(elem, interp.convert(None::<Value>))]
        } else if index == 0 {
            vec![backend::fixed::one(elem), backend::fixed::hole(len - 1)]
        } else if index == 1 && len == 1 {
            vec![backend::fixed::two(elem, interp.convert(None::<Value>))]
        } else if index == 1 && len == 2 {
            vec![backend::fixed::two(interp.convert(None::<Value>), elem)]
        } else if index == len {
            vec![backend::fixed::hole(len), backend::fixed::one(elem)]
        } else if index == len - 1 {
            vec![backend::fixed::hole(len - 1), backend::fixed::one(elem)]
        } else if index < len {
            vec![
                backend::fixed::hole(index),
                backend::fixed::one(elem),
                backend::fixed::hole(len - index - 1),
            ]
        } else {
            vec![backend::fixed::hole(index), backend::fixed::one(elem)]
        };
        *realloc = Some(alloc);
        Ok(())
    }

    fn set_with_drain(
        &self,
        interp: &Artichoke,
        start: usize,
        drain: usize,
        with: Value,
        realloc: &mut Option<Vec<Box<dyn ArrayType>>>,
    ) -> Result<usize, Box<dyn RubyException>> {
        let _ = interp;
        let (alloc, drained) = if start < self.0.get() {
            let before = start;
            let remaining = self.0.get() - start;
            let after = remaining.checked_sub(drain).unwrap_or_default();
            let mut alloc = vec![];
            if before > 0 {
                alloc.push(backend::fixed::hole(before));
            }
            alloc.push(backend::fixed::one(with));
            if after > 0 {
                alloc.push(backend::fixed::hole(after));
            }
            (alloc, remaining - after)
        } else {
            let alloc = vec![backend::fixed::hole(start), backend::fixed::one(with)];
            (alloc, 0)
        };
        *realloc = Some(alloc);
        Ok(drained)
    }

    fn set_slice(
        &self,
        interp: &Artichoke,
        start: usize,
        drain: usize,
        with: Box<dyn ArrayType>,
        realloc: &mut Option<Vec<Box<dyn ArrayType>>>,
    ) -> Result<usize, Box<dyn RubyException>> {
        let _ = interp;
        let (alloc, drained) = if start < self.0.get() {
            let before = start;
            let remaining = self.0.get() - start;
            let after = remaining.checked_sub(drain).unwrap_or_default();
            let mut alloc = vec![];
            if before > 0 {
                alloc.push(backend::fixed::hole(before));
            }
            alloc.push(with);
            if after > 0 {
                alloc.push(backend::fixed::hole(after));
            }
            (alloc, remaining - after)
        } else {
            let alloc = vec![backend::fixed::hole(start), with];
            (alloc, 0)
        };
        *realloc = Some(alloc);
        Ok(drained)
    }

    fn concat(
        &self,
        interp: &Artichoke,
        other: Box<dyn ArrayType>,
        realloc: &mut Option<Vec<Box<dyn ArrayType>>>,
    ) -> Result<(), Box<dyn RubyException>> {
        let _ = interp;
        *realloc = Some(vec![Box::new(*self), other]);
        Ok(())
    }

    fn pop(
        &self,
        interp: &Artichoke,
        realloc: &mut Option<Vec<Box<dyn ArrayType>>>,
    ) -> Result<Value, Box<dyn RubyException>> {
        *realloc = Some(vec![backend::fixed::hole(self.0.get() - 1)]);
        Ok(interp.convert(None::<Value>))
    }

    fn reverse(&self, interp: &Artichoke) -> Result<Box<dyn ArrayType>, Box<dyn RubyException>> {
        let _ = interp;
        Ok(Box::new(*self))
    }
}
