use std::borrow::Cow;
use std::cell::RefCell;
use std::iter;

use crate::convert::Convert;
use crate::extn::core::array::{backend, ArrayType};
use crate::extn::core::exception::RubyException;
use crate::gc::MrbGarbageCollection;
use crate::value::Value;
use crate::Artichoke;

pub const BUFFER_INLINE_MAX: usize = 128;

#[derive(Debug, Clone)]
pub struct Buffer(RefCell<Vec<Value>>);

impl Buffer {
    pub fn with_capacity(capacity: usize) -> Self {
        let buffer = Vec::with_capacity(capacity);
        Self(RefCell::new(buffer))
    }
}

impl From<Vec<Value>> for Buffer {
    fn from(values: Vec<Value>) -> Self {
        Self(RefCell::new(values))
    }
}

impl<'a> From<&'a [Value]> for Buffer {
    fn from(values: &'a [Value]) -> Self {
        let mut buffer = Vec::with_capacity(values.len());
        for value in values {
            buffer.push(value.clone());
        }
        Self(RefCell::new(buffer))
    }
}

impl ArrayType for Buffer {
    fn box_clone(&self) -> Box<dyn ArrayType> {
        Box::new(self.clone())
    }

    fn gc_mark(&self, interp: &Artichoke) {
        let borrow = self.0.borrow();
        for element in borrow.iter() {
            interp.mark_value(element);
        }
    }

    fn real_children(&self) -> usize {
        self.0.borrow().len()
    }

    fn len(&self) -> usize {
        self.0.borrow().len()
    }

    fn is_empty(&self) -> bool {
        self.0.borrow().is_empty()
    }

    fn get(&self, interp: &Artichoke, index: usize) -> Result<Value, Box<dyn RubyException>> {
        Ok(interp.convert(self.0.borrow().get(index)))
    }

    fn slice(
        &self,
        interp: &Artichoke,
        start: usize,
        len: usize,
    ) -> Result<Box<dyn ArrayType>, Box<dyn RubyException>> {
        let _ = interp;
        if start < self.0.borrow().len() {
            let borrow = self.0.borrow();
            let iter = borrow.iter().skip(start).take(len);
            let mut buffer = vec![];
            for elem in iter {
                buffer.push(elem.clone());
            }
            Ok(Box::new(Self::from(buffer)))
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
        let buflen = self.0.borrow().len();
        if index < buflen {
            let mut borrow = self.0.borrow_mut();
            borrow[index] = elem;
        } else if index <= BUFFER_INLINE_MAX {
            let mut borrow = self.0.borrow_mut();
            for _ in buflen..index {
                borrow.push(interp.convert(None::<Value>));
            }
            borrow.push(elem);
        } else {
            let alloc = vec![
                self.box_clone(),
                backend::fixed::hole(index - buflen),
                backend::fixed::one(elem),
            ];
            *realloc = Some(alloc);
        }
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
        let mut borrow = self.0.borrow_mut();
        let buflen = borrow.len();
        if start < buflen {
            let remaining = buflen - start;
            let after = remaining.checked_sub(drain).unwrap_or_default();
            borrow.splice(start..buflen - after, iter::once(with));
            Ok(remaining - after)
        } else if start <= BUFFER_INLINE_MAX {
            for _ in buflen..start {
                borrow.push(interp.convert(None::<Value>));
            }
            borrow.push(with);
            Ok(0)
        } else {
            let alloc = vec![
                self.box_clone(),
                backend::fixed::hole(start - buflen),
                backend::fixed::one(with),
            ];
            *realloc = Some(alloc);
            Ok(0)
        }
    }

    fn set_slice(
        &self,
        interp: &Artichoke,
        start: usize,
        drain: usize,
        with: Box<dyn ArrayType>,
        realloc: &mut Option<Vec<Box<dyn ArrayType>>>,
    ) -> Result<usize, Box<dyn RubyException>> {
        let mut borrow = self.0.borrow_mut();
        let buflen = borrow.len();
        if start < buflen {
            let remaining = buflen - start;
            let after = remaining.checked_sub(drain).unwrap_or_default();
            let newlen = start + after + with.len();
            if newlen <= BUFFER_INLINE_MAX {
                if newlen > buflen {
                    borrow.reserve(newlen - buflen);
                }
                let mut insert = Vec::with_capacity(with.len());
                for idx in 0..with.len() {
                    insert.push(with.get(interp, idx)?);
                }
                borrow.splice(start..buflen - after, insert);
            } else {
                let mut alloc = Vec::with_capacity(3);
                if start > 0 {
                    let buffer: Box<dyn ArrayType> = Box::new(Self::from(&borrow[..start]));
                    alloc.push(buffer);
                }
                alloc.push(with);
                if after > 0 {
                    let buffer: Box<dyn ArrayType> =
                        Box::new(Self::from(&borrow[buflen - after..]));
                    alloc.push(buffer);
                }
                *realloc = Some(alloc);
            }
            Ok(remaining - after)
        } else if start + with.len() <= BUFFER_INLINE_MAX {
            let newlen = start + with.len();
            if newlen > buflen {
                borrow.reserve(newlen - buflen);
            }
            for _ in buflen..start {
                borrow.push(interp.convert(None::<Value>));
            }
            for idx in 0..with.len() {
                borrow.push(with.get(interp, idx)?);
            }
            Ok(0)
        } else {
            let alloc = vec![self.box_clone(), backend::fixed::hole(start - buflen), with];
            *realloc = Some(alloc);
            Ok(0)
        }
    }

    fn concat(
        &self,
        interp: &Artichoke,
        other: Box<dyn ArrayType>,
        realloc: &mut Option<Vec<Box<dyn ArrayType>>>,
    ) -> Result<(), Box<dyn RubyException>> {
        let mut borrow = self.0.borrow_mut();
        if borrow.len() + other.len() <= BUFFER_INLINE_MAX {
            if other.len() > borrow.len() {
                borrow.reserve(other.len());
            }
            for idx in 0..other.len() {
                borrow.push(other.get(interp, idx)?);
            }
        } else {
            let alloc = vec![self.box_clone(), other];
            *realloc = Some(alloc);
        }
        Ok(())
    }

    fn pop(
        &self,
        interp: &Artichoke,
        realloc: &mut Option<Vec<Box<dyn ArrayType>>>,
    ) -> Result<Value, Box<dyn RubyException>> {
        let _ = realloc;
        let mut borrow = self.0.borrow_mut();
        Ok(interp.convert(borrow.pop()))
    }

    fn reverse(&self, interp: &Artichoke) -> Result<Box<dyn ArrayType>, Box<dyn RubyException>> {
        let _ = interp;
        let borrow = self.0.borrow();
        let mut buffer = borrow.clone();
        buffer.reverse();
        Ok(Box::new(Self::from(buffer)))
    }
}
