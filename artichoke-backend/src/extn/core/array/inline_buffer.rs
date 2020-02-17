use smallvec::SmallVec;
use std::cmp;
use std::fmt;
use std::iter::{self, FromIterator};

use crate::extn::core::array::ArrayType;
use crate::extn::prelude::*;
use crate::gc::MrbGarbageCollection;

const INLINE_CAPACITY: usize = 8;

#[derive(Clone, Default)]
pub struct InlineBuffer(SmallVec<[sys::mrb_value; INLINE_CAPACITY]>);

impl From<Vec<sys::mrb_value>> for InlineBuffer {
    fn from(values: Vec<sys::mrb_value>) -> Self {
        Self(SmallVec::from_vec(values))
    }
}

impl From<Vec<Value>> for InlineBuffer {
    fn from(values: Vec<Value>) -> Self {
        Self(SmallVec::from_iter(values.iter().map(Value::inner)))
    }
}

impl<'a> From<&'a [sys::mrb_value]> for InlineBuffer {
    fn from(values: &'a [sys::mrb_value]) -> Self {
        Self(SmallVec::from_slice(values))
    }
}

impl<'a> From<&'a [Value]> for InlineBuffer {
    fn from(values: &'a [Value]) -> Self {
        Self(SmallVec::from_iter(values.iter().map(Value::inner)))
    }
}

impl FromIterator<Value> for InlineBuffer {
    fn from_iter<I: IntoIterator<Item = Value>>(iter: I) -> Self {
        Self(SmallVec::from_iter(
            iter.into_iter().map(|elem| elem.inner()),
        ))
    }
}

impl FromIterator<Option<Value>> for InlineBuffer {
    fn from_iter<I: IntoIterator<Item = Option<Value>>>(iter: I) -> Self {
        Self(SmallVec::from_iter(iter.into_iter().map(|elem| {
            elem.map_or_else(|| unsafe { sys::mrb_sys_nil_value() }, |elem| elem.inner())
        })))
    }
}

impl<'a> FromIterator<&'a Option<Value>> for InlineBuffer {
    fn from_iter<I: IntoIterator<Item = &'a Option<Value>>>(iter: I) -> Self {
        Self(SmallVec::from_iter(iter.into_iter().map(|elem| {
            if let Some(elem) = elem {
                elem.inner()
            } else {
                unsafe { sys::mrb_sys_nil_value() }
            }
        })))
    }
}

impl fmt::Debug for InlineBuffer {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_list()
            .entries(iter::repeat("Value").take(self.len()))
            .finish()
    }
}

impl ArrayType for InlineBuffer {
    fn box_clone(&self) -> Box<dyn ArrayType> {
        Box::new(self.clone())
    }

    fn gc_mark(&self, interp: &Artichoke) {
        for elem in self.0.iter().copied() {
            let value = Value::new(interp, elem);
            interp.mark_value(&value);
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

    fn get(&self, interp: &Artichoke, index: usize) -> Result<Option<Value>, Exception> {
        Self::get(self, interp, index)
    }

    fn slice(
        &self,
        interp: &Artichoke,
        start: usize,
        len: usize,
    ) -> Result<Box<dyn ArrayType>, Exception> {
        match Self::slice(self, interp, start, len) {
            Ok(slice) => Ok(Box::new(slice)),
            Err(err) => Err(err),
        }
    }

    fn set(
        &mut self,
        interp: &Artichoke,
        index: usize,
        elem: Value,
        realloc: &mut Option<Vec<Box<dyn ArrayType>>>,
    ) -> Result<(), Exception> {
        let _ = realloc;
        Self::set(self, interp, index, elem)
    }

    fn set_with_drain(
        &mut self,
        interp: &Artichoke,
        start: usize,
        drain: usize,
        with: Value,
        realloc: &mut Option<Vec<Box<dyn ArrayType>>>,
    ) -> Result<usize, Exception> {
        let _ = realloc;
        Self::set_with_drain(self, interp, start, drain, with)
    }

    fn set_slice(
        &mut self,
        interp: &Artichoke,
        start: usize,
        drain: usize,
        with: Box<dyn ArrayType>,
        realloc: &mut Option<Vec<Box<dyn ArrayType>>>,
    ) -> Result<usize, Exception> {
        let _ = realloc;
        if let Ok(buffer) = with.downcast_ref::<Self>() {
            Self::set_slice(self, interp, start, drain, buffer)
        } else {
            Err(Exception::from(Fatal::new(
                interp,
                "set slice on InlineBuffer with unknown ArrayType",
            )))
        }
    }

    fn concat(
        &mut self,
        interp: &Artichoke,
        other: Box<dyn ArrayType>,
        realloc: &mut Option<Vec<Box<dyn ArrayType>>>,
    ) -> Result<(), Exception> {
        let _ = realloc;
        if let Ok(buffer) = other.downcast_ref::<Self>() {
            Self::concat(self, interp, buffer)
        } else {
            Err(Exception::from(Fatal::new(
                interp,
                "concat on InlineBuffer with unknown ArrayType",
            )))
        }
    }

    fn pop(
        &mut self,
        interp: &Artichoke,
        realloc: &mut Option<Vec<Box<dyn ArrayType>>>,
    ) -> Result<Value, Exception> {
        let _ = realloc;
        Self::pop(self, interp)
    }

    fn reverse(&mut self, interp: &Artichoke) -> Result<(), Exception> {
        Self::reverse(self, interp)
    }
}

impl InlineBuffer {
    #[must_use]
    pub fn with_capacity(capacity: usize) -> Self {
        Self(SmallVec::with_capacity(capacity))
    }

    #[must_use]
    pub fn as_vec(&self, interp: &Artichoke) -> Vec<Value> {
        self.0
            .iter()
            .copied()
            .map(|value| Value::new(interp, value))
            .collect()
    }

    #[must_use]
    pub fn as_ptr(&self) -> *const sys::mrb_value {
        self.0.as_ptr()
    }

    pub fn as_mut_ptr(&mut self) -> *mut sys::mrb_value {
        self.0.as_mut_ptr()
    }

    /// Set the vector's length without dropping or moving out elements
    ///
    /// This method is unsafe because it changes the notion of the number of
    /// "valid" elements in the vector. Use with care.
    ///
    /// # Safety
    ///
    /// - `new_len` must be less than or equal to capacity().
    /// - The elements at `old_len..new_len` must be initialized.
    ///
    /// # Examples
    ///
    /// This method is primarily used when mutating an `InlineBuffer` via a raw
    /// pointer passed via FFI into the mruby VM.
    ///
    /// See the `ARRAY_PTR` macro in C.
    pub unsafe fn set_len(&mut self, new_len: usize) {
        self.0.set_len(new_len);
    }

    pub fn clear(&mut self) {
        self.0.clear();
    }

    pub fn get(&self, interp: &Artichoke, index: usize) -> Result<Option<Value>, Exception> {
        let elem = self.0.get(index);
        Ok(elem.copied().map(|elem| Value::new(interp, elem)))
    }

    pub fn slice(&self, interp: &Artichoke, start: usize, len: usize) -> Result<Self, Exception> {
        let _ = interp;
        if self.is_empty() {
            return Ok(Self::default());
        }
        let buflen = self.len();
        if start < buflen {
            Ok(Self::from(&self.0[start..cmp::min(start + len, buflen)]))
        } else {
            Ok(Self::default())
        }
    }

    pub fn set(&mut self, interp: &Artichoke, index: usize, elem: Value) -> Result<(), Exception> {
        let _ = interp;
        let buflen = self.len();
        match index {
            idx if idx < buflen => self.0[index] = elem.inner(),
            idx if idx == buflen => self.0.push(elem.inner()),
            idx => {
                let nil = interp.convert(None::<Value>).inner();
                self.0.reserve(idx + 1 - buflen);
                for _ in buflen..idx {
                    self.0.push(nil);
                }
                self.0.push(elem.inner());
            }
        }
        Ok(())
    }

    pub fn set_with_drain(
        &mut self,
        interp: &Artichoke,
        start: usize,
        drain: usize,
        with: Value,
    ) -> Result<usize, Exception> {
        let _ = interp;
        let buflen = self.len();
        let drained = cmp::min(buflen.checked_sub(start).unwrap_or_default(), drain);
        match start {
            idx if idx > buflen => {
                let nil = interp.convert(None::<Value>).inner();
                self.0.reserve(start + 1 - buflen);
                for _ in buflen..start {
                    self.0.push(nil);
                }
                self.0.push(with.inner());
            }
            idx if idx == buflen => self.0.push(with.inner()),
            idx if drain == 0 => self.0.insert(idx, with.inner()),
            idx if drain == 1 => self.0[idx] = with.inner(),
            idx => {
                self.0[idx] = with.inner();
                self.0.drain(idx + 1..cmp::min(idx + drain, buflen));
            }
        }
        Ok(drained)
    }

    pub fn set_slice(
        &mut self,
        interp: &Artichoke,
        start: usize,
        drain: usize,
        with: &Self,
    ) -> Result<usize, Exception> {
        let _ = interp;
        let buflen = self.len();
        let withlen = with.len();
        let drained = cmp::min(buflen.checked_sub(start).unwrap_or_default(), drain);
        match start {
            idx if idx > buflen => {
                let nil = interp.convert(None::<Value>).inner();
                for _ in buflen..idx {
                    self.0.push(nil);
                }
                self.0.extend_from_slice(with.0.as_slice());
            }
            idx if idx == buflen => self.0.extend_from_slice(with.0.as_slice()),
            idx => match cmp::min(buflen - idx, drain) {
                0 => self.0.insert_from_slice(start, with.0.as_slice()),
                to_drain if to_drain == withlen => {
                    let end = start + withlen;
                    self.0[start..end].copy_from_slice(with.0.as_slice());
                }
                to_drain if to_drain > withlen => {
                    let end = start + with.len();
                    let remaining_drain = to_drain - with.len();
                    self.0[start..end].copy_from_slice(with.0.as_slice());
                    self.0.drain(end..end + remaining_drain);
                }
                to_drain => {
                    let (overwrite, insert) = with.0.split_at(to_drain);
                    let overwrite_until = start + overwrite.len();
                    self.0[start..overwrite_until].copy_from_slice(overwrite);
                    self.0.insert_from_slice(overwrite_until, insert);
                }
            },
        }
        Ok(drained)
    }

    pub fn concat(&mut self, interp: &Artichoke, other: &Self) -> Result<(), Exception> {
        let _ = interp;
        self.0.extend_from_slice(other.0.as_slice());
        Ok(())
    }

    pub fn pop(&mut self, interp: &Artichoke) -> Result<Value, Exception> {
        let value = self.0.pop();
        Ok(interp.convert(value.map(|value| Value::new(interp, value))))
    }

    pub fn reverse(&mut self, interp: &Artichoke) -> Result<(), Exception> {
        let _ = interp;
        self.0.reverse();
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use crate::test::prelude::*;

    #[test]
    fn integration_test() {
        let mut interp = crate::interpreter().unwrap();
        let _ = interp
            .eval(&include_bytes!("inline_buffer_test.rb")[..])
            .unwrap();
        let result = interp.eval(b"spec");
        let result = result.unwrap().try_into::<bool>().unwrap();
        assert!(result);
    }
}
