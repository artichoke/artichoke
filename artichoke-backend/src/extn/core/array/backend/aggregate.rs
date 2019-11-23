use crate::convert::Convert;
use crate::extn::core::array::{backend, ArrayType};
use crate::extn::core::exception::{RangeError, RubyException};
use crate::value::Value;
use crate::Artichoke;

#[derive(Default)]
pub struct Aggregate(Vec<Box<dyn ArrayType>>);

impl Aggregate {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_parts(parts: Vec<Box<dyn ArrayType>>) -> Self {
        Self(parts.into_iter().filter(|part| !part.is_empty()).collect())
    }

    pub fn into_parts(self) -> Vec<Box<dyn ArrayType>> {
        self.0
    }

    pub fn parts(&self) -> Vec<Box<dyn ArrayType>> {
        self.0.iter().map(|part| part.box_clone()).collect()
    }
}

impl Clone for Aggregate {
    fn clone(&self) -> Self {
        Self(self.parts())
    }
}

impl ArrayType for Aggregate {
    fn box_clone(&self) -> Box<dyn ArrayType> {
        Box::new(self.clone())
    }

    fn gc_mark(&self, interp: &Artichoke) {
        for part in &self.0 {
            part.gc_mark(interp);
        }
    }

    fn real_children(&self) -> usize {
        let mut real_children = 0_usize;
        for part in &self.0 {
            real_children =
                if let Some(real_children) = real_children.checked_add(part.real_children()) {
                    real_children
                } else {
                    return usize::max_value();
                };
        }
        real_children
    }

    fn len(&self) -> usize {
        let mut len = 0_usize;
        for part in &self.0 {
            len = if let Some(len) = len.checked_add(part.len()) {
                len
            } else {
                return usize::max_value();
            };
        }
        len
    }

    fn is_empty(&self) -> bool {
        false
    }

    fn get(&self, interp: &Artichoke, index: usize) -> Result<Value, Box<dyn RubyException>> {
        let mut base = 0;
        for part in &self.0 {
            let idx = index - base;
            if idx < part.len() {
                return part.get(interp, idx);
            }
            base = base
                .checked_add(part.len())
                .ok_or_else(|| RangeError::new(interp, "array too big"))?;
        }
        Ok(interp.convert(None::<Value>))
    }

    fn slice(
        &self,
        interp: &Artichoke,
        start: usize,
        len: usize,
    ) -> Result<Box<dyn ArrayType>, Box<dyn RubyException>> {
        let mut base = 0;
        let mut iter = self.0.iter();
        while let Some(part) = iter.next() {
            let idx = start - base;
            if idx < part.len() {
                let slice = part.slice(interp, idx, len)?;
                if slice.len() == len {
                    return Ok(slice);
                }
                let mut collected = slice.len();
                let mut slice_parts = vec![slice];
                for part in iter {
                    let slice = part.slice(interp, 0, len - collected)?;
                    collected += slice.len();
                    slice_parts.push(slice);
                    if collected == len {
                        return Ok(Box::new(Self::with_parts(slice_parts)));
                    }
                }
                return Ok(Box::new(Self::with_parts(slice_parts)));
            }
            base = base
                .checked_add(part.len())
                .ok_or_else(|| RangeError::new(interp, "array too big"))?;
        }
        Ok(backend::fixed::empty())
    }

    fn set(
        &mut self,
        interp: &Artichoke,
        index: usize,
        elem: Value,
        realloc: &mut Option<Vec<Box<dyn ArrayType>>>,
    ) -> Result<(), Box<dyn RubyException>> {
        let _ = realloc;
        let mut base = 0;
        for (chunk, part) in self.0.iter_mut().enumerate() {
            let idx = index - base;
            if idx < part.len() {
                let mut realloc = None;
                part.set(interp, idx, elem, &mut realloc)?;
                if let Some(reallocated_parts) = realloc {
                    let reallocated_parts = reallocated_parts
                        .into_iter()
                        .filter(|part| !part.is_empty());
                    self.0.splice(chunk..=chunk, reallocated_parts);
                }
                return Ok(());
            }
            base = base
                .checked_add(part.len())
                .ok_or_else(|| RangeError::new(interp, "array too big"))?;
        }
        if index > base {
            self.0.push(backend::fixed::hole(index - base));
        }
        self.0.push(backend::fixed::one(elem));
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
        let _ = realloc;
        let mut base = 0;
        let mut iter = self.0.iter_mut().enumerate();
        while let Some((chunk, part)) = iter.next() {
            let idx = start - base;
            if idx < part.len() {
                let replace_start = chunk;
                let mut replace_end = chunk;
                let mut reallocated_parts = None::<Vec<Box<dyn ArrayType>>>;
                let mut drained = 0;
                let mut realloc = None;

                drained += part.set_with_drain(interp, idx, drain, with, &mut realloc)?;
                if let Some(realloc) = realloc {
                    if let Some(ref mut reallocated_parts) = reallocated_parts {
                        reallocated_parts.extend(realloc);
                    } else {
                        reallocated_parts = Some(realloc);
                    }
                }
                if drained < drain {
                    for (chunk, part) in iter {
                        replace_end = chunk;
                        let mut realloc = None;
                        drained += part.set_slice(
                            interp,
                            0,
                            drain - drained,
                            backend::fixed::empty(),
                            &mut realloc,
                        )?;
                        if let Some(realloc) = realloc {
                            if let Some(ref mut reallocated_parts) = reallocated_parts {
                                reallocated_parts.extend(realloc);
                            } else {
                                reallocated_parts = Some(realloc);
                            }
                        }
                        if drained == drain {
                            break;
                        }
                    }
                }
                if let Some(reallocated_parts) = reallocated_parts {
                    let reallocated_parts = reallocated_parts
                        .into_iter()
                        .filter(|part| !part.is_empty());
                    self.0
                        .splice(replace_start..=replace_end, reallocated_parts);
                }
                return Ok(drained);
            }
            base = base
                .checked_add(part.len())
                .ok_or_else(|| RangeError::new(interp, "array too big"))?;
        }
        if start > base {
            self.0.push(backend::fixed::hole(start - base));
        }
        self.0.push(backend::fixed::one(with));
        Ok(0)
    }

    fn set_slice(
        &mut self,
        interp: &Artichoke,
        start: usize,
        drain: usize,
        with: Box<dyn ArrayType>,
        realloc: &mut Option<Vec<Box<dyn ArrayType>>>,
    ) -> Result<usize, Box<dyn RubyException>> {
        let _ = realloc;
        let mut base = 0;
        let mut iter = self.0.iter_mut().enumerate();
        while let Some((chunk, part)) = iter.next() {
            let idx = start - base;
            if idx < part.len() {
                let replace_start = chunk;
                let mut replace_end = chunk;
                let mut reallocated_parts = None::<Vec<Box<dyn ArrayType>>>;
                let mut drained = 0;
                let mut realloc = None;

                drained += part.set_slice(interp, idx, drain, with, &mut realloc)?;
                if let Some(realloc) = realloc {
                    if let Some(ref mut reallocated_parts) = reallocated_parts {
                        reallocated_parts.extend(realloc);
                    } else {
                        reallocated_parts = Some(realloc);
                    }
                }
                if drained < drain {
                    for (chunk, part) in iter {
                        replace_end = chunk;
                        let mut realloc = None;
                        drained += part.set_slice(
                            interp,
                            0,
                            drain - drained,
                            backend::fixed::empty(),
                            &mut realloc,
                        )?;
                        if let Some(realloc) = realloc {
                            if let Some(ref mut reallocated_parts) = reallocated_parts {
                                reallocated_parts.extend(realloc);
                            } else {
                                reallocated_parts = Some(realloc);
                            }
                        }
                        if drained == drain {
                            break;
                        }
                    }
                }
                if let Some(reallocated_parts) = reallocated_parts {
                    let reallocated_parts = reallocated_parts
                        .into_iter()
                        .filter(|part| !part.is_empty());
                    self.0
                        .splice(replace_start..=replace_end, reallocated_parts);
                }
                return Ok(drained);
            }
            base = base
                .checked_add(part.len())
                .ok_or_else(|| RangeError::new(interp, "array too big"))?;
        }
        if start > base {
            self.0.push(backend::fixed::hole(start - base));
        }
        self.0.push(with);
        Ok(0)
    }

    fn concat(
        &mut self,
        interp: &Artichoke,
        other: Box<dyn ArrayType>,
        realloc: &mut Option<Vec<Box<dyn ArrayType>>>,
    ) -> Result<(), Box<dyn RubyException>> {
        let _ = interp;
        let _ = realloc;
        if let Ok(other) = other.downcast_ref::<Self>() {
            self.0.extend(other.parts());
        } else {
            self.0.push(other);
        }
        Ok(())
    }

    fn pop(
        &mut self,
        interp: &Artichoke,
        realloc: &mut Option<Vec<Box<dyn ArrayType>>>,
    ) -> Result<Value, Box<dyn RubyException>> {
        let _ = realloc;
        if let Some(last) = self.0.last_mut() {
            let mut realloc = None;
            let popped = last.pop(interp, &mut realloc)?;
            if let Some(realloc) = realloc {
                let reallocated_parts = realloc.into_iter().filter(|part| !part.is_empty());
                self.0.pop();
                self.0.extend(reallocated_parts);
            }
            Ok(popped)
        } else {
            Ok(interp.convert(None::<Value>))
        }
    }

    fn reverse(&mut self, interp: &Artichoke) -> Result<(), Box<dyn RubyException>> {
        self.0.reverse();
        for part in self.0.iter_mut() {
            part.reverse(interp)?;
        }
        Ok(())
    }
}
