use std::cmp;
use std::num::NonZeroUsize;

use crate::extn::core::array::{backend, ArrayType};
use crate::extn::prelude::*;
use crate::gc::MrbGarbageCollection;
use crate::value;

#[derive(Debug, Clone)]
pub struct Value(value::Value, NonZeroUsize);

impl Value {
    pub fn new(value: value::Value, len: NonZeroUsize) -> Self {
        Self(value, len)
    }
}

impl ArrayType for Value {
    fn box_clone(&self) -> Box<dyn ArrayType> {
        Box::new(self.clone())
    }

    fn gc_mark(&self, interp: &Artichoke) {
        interp.mark_value(&self.0);
    }

    fn real_children(&self) -> usize {
        1
    }

    fn len(&self) -> usize {
        self.1.get()
    }

    fn is_empty(&self) -> bool {
        false
    }

    fn get(&self, interp: &Artichoke, index: usize) -> Result<value::Value, Exception> {
        if index < self.1.get() {
            Ok(self.0.clone())
        } else {
            Ok(interp.convert(None::<value::Value>))
        }
    }

    fn slice(
        &self,
        interp: &Artichoke,
        start: usize,
        len: usize,
    ) -> Result<Box<dyn ArrayType>, Exception> {
        let _ = interp;
        if start < self.1.get() {
            let remaining = self.1.get() - start;
            let len = cmp::min(remaining, len);
            Ok(backend::repeated::value(self.0.clone(), len))
        } else {
            Ok(backend::fixed::empty())
        }
    }

    fn set(
        &mut self,
        interp: &Artichoke,
        index: usize,
        elem: value::Value,
        realloc: &mut Option<Vec<Box<dyn ArrayType>>>,
    ) -> Result<(), Exception> {
        let _ = interp;
        let alloc = if index == 0 {
            vec![
                backend::fixed::one(elem),
                backend::repeated::value(self.0.clone(), self.1.get() - 1),
            ]
        } else if index == self.1.get() {
            vec![self.box_clone(), backend::fixed::one(elem)]
        } else if index == self.1.get() - 1 {
            vec![
                backend::repeated::value(self.0.clone(), self.1.get() - 1),
                backend::fixed::one(elem),
            ]
        } else if index < self.1.get() {
            let before = index;
            let after = self.1.get() - 1;
            vec![
                backend::repeated::value(self.0.clone(), before),
                backend::fixed::one(elem),
                backend::repeated::value(self.0.clone(), after),
            ]
        } else {
            vec![
                self.box_clone(),
                backend::fixed::hole(index - self.1.get()),
                backend::fixed::one(elem),
            ]
        };
        *realloc = Some(alloc);
        Ok(())
    }

    fn set_with_drain(
        &mut self,
        interp: &Artichoke,
        start: usize,
        drain: usize,
        with: value::Value,
        realloc: &mut Option<Vec<Box<dyn ArrayType>>>,
    ) -> Result<usize, Exception> {
        let _ = interp;
        let (alloc, drained) = match self.1.get() {
            idx if start < idx => {
                let before = start;
                let remaining = idx - start;
                let after = remaining.checked_sub(drain).unwrap_or_default();
                let mut alloc = Vec::with_capacity(3);
                if before > 0 {
                    alloc.push(backend::repeated::value(self.0.clone(), before));
                }
                alloc.push(backend::fixed::one(with));
                if after > 0 {
                    alloc.push(backend::repeated::value(self.0.clone(), after));
                }
                (alloc, remaining - after)
            }
            idx if start == idx => {
                let alloc = vec![self.box_clone(), backend::fixed::one(with)];
                (alloc, 0)
            }
            idx => {
                let alloc = vec![
                    self.box_clone(),
                    backend::fixed::hole(start - idx),
                    backend::fixed::one(with),
                ];
                (alloc, 0)
            }
        };
        *realloc = Some(alloc);
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
        let (alloc, drained) = match self.1.get() {
            idx if start < idx => {
                let before = start;
                let remaining = self.1.get() - start;
                let after = remaining.checked_sub(drain).unwrap_or_default();
                let mut alloc = Vec::with_capacity(3);
                if before > 0 {
                    alloc.push(backend::repeated::value(self.0.clone(), before));
                }
                alloc.push(with);
                if after > 0 {
                    alloc.push(backend::repeated::value(self.0.clone(), after));
                }
                (alloc, remaining - after)
            }
            idx if start == idx => {
                let alloc = vec![self.box_clone(), with];
                (alloc, 0)
            }
            idx => {
                let alloc = vec![self.box_clone(), backend::fixed::hole(start - idx), with];
                (alloc, 0)
            }
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
        let _ = interp;
        *realloc = Some(vec![self.box_clone(), other]);
        Ok(())
    }

    fn pop(
        &mut self,
        interp: &Artichoke,
        realloc: &mut Option<Vec<Box<dyn ArrayType>>>,
    ) -> Result<value::Value, Exception> {
        let _ = interp;
        if let Some(len) = NonZeroUsize::new(self.1.get() - 1) {
            self.1 = len;
        } else {
            *realloc = Some(vec![backend::fixed::empty()]);
        }
        Ok(self.0.clone())
    }

    fn reverse(&mut self, interp: &Artichoke) -> Result<(), Exception> {
        let _ = interp;
        Ok(())
    }
}
