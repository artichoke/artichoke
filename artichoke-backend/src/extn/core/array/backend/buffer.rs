use std::iter;

use crate::convert::Convert;
use crate::extn::core::array::{backend, ArrayType};
use crate::extn::core::exception::RubyException;
use crate::gc::MrbGarbageCollection;
use crate::value::Value;
use crate::Artichoke;

pub const BUFFER_INLINE_MAX: usize = 128;

#[derive(Debug, Clone)]
pub struct Buffer(Vec<Value>);

impl Buffer {
    pub fn with_capacity(capacity: usize) -> Self {
        let buffer = Vec::with_capacity(capacity);
        Self(buffer)
    }
}

impl From<Vec<Value>> for Buffer {
    fn from(values: Vec<Value>) -> Self {
        Self(values)
    }
}

impl<'a> From<&'a [Value]> for Buffer {
    fn from(values: &'a [Value]) -> Self {
        Self(Vec::from(values))
    }
}

impl ArrayType for Buffer {
    fn box_clone(&self) -> Box<dyn ArrayType> {
        Box::new(self.clone())
    }

    fn gc_mark(&self, interp: &Artichoke) {
        for element in &self.0 {
            interp.mark_value(element);
        }
    }

    fn real_children(&self) -> usize {
        self.0.len()
    }

    fn len(&self) -> usize {
        self.0.len()
    }

    fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    fn get(&self, interp: &Artichoke, index: usize) -> Result<Value, Box<dyn RubyException>> {
        Ok(interp.convert(self.0.get(index)))
    }

    fn slice(
        &self,
        interp: &Artichoke,
        start: usize,
        len: usize,
    ) -> Result<Box<dyn ArrayType>, Box<dyn RubyException>> {
        let _ = interp;
        if start < self.0.len() {
            let iter = self.0.iter().skip(start).take(len);
            Ok(Box::new(Self(iter.cloned().collect())))
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
    ) -> Result<(), Box<dyn RubyException>> {
        let buflen = self.0.len();
        if index < buflen {
            self.0[index] = elem;
        } else if index <= BUFFER_INLINE_MAX {
            self.0.reserve(index - buflen);
            for _ in buflen..index {
                self.0.push(interp.convert(None::<Value>));
            }
            self.0.push(elem);
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
        &mut self,
        interp: &Artichoke,
        start: usize,
        drain: usize,
        with: Value,
        realloc: &mut Option<Vec<Box<dyn ArrayType>>>,
    ) -> Result<usize, Box<dyn RubyException>> {
        let buflen = self.0.len();
        if start < buflen {
            let remaining = buflen - start;
            let after = remaining.checked_sub(drain).unwrap_or_default();
            self.0.splice(start..buflen - after, iter::once(with));
            Ok(remaining - after)
        } else if start <= BUFFER_INLINE_MAX {
            self.0.reserve(start - buflen);
            for _ in buflen..start {
                self.0.push(interp.convert(None::<Value>));
            }
            self.0.push(with);
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
        &mut self,
        interp: &Artichoke,
        start: usize,
        drain: usize,
        with: Box<dyn ArrayType>,
        realloc: &mut Option<Vec<Box<dyn ArrayType>>>,
    ) -> Result<usize, Box<dyn RubyException>> {
        let buflen = self.0.len();
        if start < buflen {
            let remaining = buflen - start;
            let after = remaining.checked_sub(drain).unwrap_or_default();
            let newlen = start + after + with.len();
            if newlen <= BUFFER_INLINE_MAX {
                let insert = if let Ok(buffer) = with.downcast_ref::<Self>() {
                    buffer.0.clone()
                } else {
                    let mut insert = Vec::with_capacity(with.len());
                    for idx in 0..with.len() {
                        insert.push(with.get(interp, idx)?);
                    }
                    insert
                };
                self.0.splice(start..buflen - after, insert);
            } else {
                let mut alloc = Vec::with_capacity(3);
                if start > 0 {
                    let buffer: Box<dyn ArrayType> = Box::new(Self::from(&self.0[..start]));
                    alloc.push(buffer);
                }
                alloc.push(with);
                if after > 0 {
                    let buffer: Box<dyn ArrayType> =
                        Box::new(Self::from(&self.0[buflen - after..]));
                    alloc.push(buffer);
                }
                *realloc = Some(alloc);
            }
            Ok(remaining - after)
        } else if start + with.len() <= BUFFER_INLINE_MAX {
            let newlen = start + with.len();
            if newlen > buflen {
                self.0.reserve(newlen - buflen);
            }
            for _ in buflen..start {
                self.0.push(interp.convert(None::<Value>));
            }
            if let Ok(buffer) = with.downcast_ref::<Self>() {
                self.0.extend(buffer.0.clone());
            } else {
                for idx in 0..with.len() {
                    self.0.push(with.get(interp, idx)?);
                }
            }
            Ok(0)
        } else {
            let alloc = vec![self.box_clone(), backend::fixed::hole(start - buflen), with];
            *realloc = Some(alloc);
            Ok(0)
        }
    }

    fn concat(
        &mut self,
        interp: &Artichoke,
        other: Box<dyn ArrayType>,
        realloc: &mut Option<Vec<Box<dyn ArrayType>>>,
    ) -> Result<(), Box<dyn RubyException>> {
        if self.len() + other.len() <= BUFFER_INLINE_MAX {
            if let Ok(buffer) = other.downcast_ref::<Self>() {
                self.0.extend(buffer.0.clone());
            } else {
                self.0.reserve(other.len());
                for idx in 0..other.len() {
                    self.0.push(other.get(interp, idx)?);
                }
            }
        } else {
            let alloc = vec![self.box_clone(), other];
            *realloc = Some(alloc);
        }
        Ok(())
    }

    fn pop(
        &mut self,
        interp: &Artichoke,
        realloc: &mut Option<Vec<Box<dyn ArrayType>>>,
    ) -> Result<Value, Box<dyn RubyException>> {
        let _ = realloc;
        Ok(interp.convert(self.0.pop()))
    }

    fn reverse(&mut self, interp: &Artichoke) -> Result<(), Box<dyn RubyException>> {
        let _ = interp;
        self.0.reverse();
        Ok(())
    }
}
