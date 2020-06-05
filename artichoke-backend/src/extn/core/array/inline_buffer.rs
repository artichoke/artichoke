use smallvec::SmallVec;
use std::cmp;
use std::iter::FromIterator;
use std::slice;

use crate::extn::prelude::*;

const INLINE_CAPACITY: usize = 8;

#[derive(Default, Debug, Clone)]
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

impl FromIterator<sys::mrb_value> for InlineBuffer {
    fn from_iter<I>(iter: I) -> Self
    where
        I: IntoIterator<Item = sys::mrb_value>,
    {
        Self(SmallVec::from_iter(iter.into_iter()))
    }
}

impl FromIterator<Value> for InlineBuffer {
    fn from_iter<I>(iter: I) -> Self
    where
        I: IntoIterator<Item = Value>,
    {
        Self(SmallVec::from_iter(
            iter.into_iter().map(|elem| elem.inner()),
        ))
    }
}

impl FromIterator<Option<Value>> for InlineBuffer {
    fn from_iter<I>(iter: I) -> Self
    where
        I: IntoIterator<Item = Option<Value>>,
    {
        Self(SmallVec::from_iter(
            iter.into_iter()
                .map(|elem| elem.unwrap_or_else(Value::nil).inner()),
        ))
    }
}

impl<'a> FromIterator<&'a Option<Value>> for InlineBuffer {
    fn from_iter<I>(iter: I) -> Self
    where
        I: IntoIterator<Item = &'a Option<Value>>,
    {
        Self(SmallVec::from_iter(
            iter.into_iter()
                .map(|elem| elem.unwrap_or_else(Value::nil).inner()),
        ))
    }
}

#[derive(Debug)]
pub struct Iter<'a>(slice::Iter<'a, sys::mrb_value>);

impl<'a> Iterator for Iter<'a> {
    type Item = Value;

    fn next(&mut self) -> Option<Self::Item> {
        let value = self.0.next()?;
        Some(Value::from(*value))
    }
}

impl<'a> IntoIterator for &'a InlineBuffer {
    type Item = Value;
    type IntoIter = Iter<'a>;

    fn into_iter(self) -> Self::IntoIter {
        Iter(self.0.iter())
    }
}

impl InlineBuffer {
    #[must_use]
    pub fn new() -> Self {
        Self(SmallVec::new())
    }

    #[must_use]
    pub fn with_capacity(capacity: usize) -> Self {
        Self(SmallVec::with_capacity(capacity))
    }

    #[must_use]
    pub fn iter(&self) -> Iter<'_> {
        self.into_iter()
    }

    #[must_use]
    pub fn as_slice(&self) -> &[sys::mrb_value] {
        self.0.as_slice()
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

    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    pub fn get(&self, index: usize) -> Option<Value> {
        let elem = self.0.get(index);
        elem.copied().map(Value::from)
    }

    pub fn slice(&self, start: usize, len: usize) -> Self {
        if self.is_empty() {
            return Self::default();
        }
        if let Some(slice) = self.0.get(start..start + len) {
            Self::from(slice)
        } else if let Some(slice) = self.0.get(start..self.0.len()) {
            Self::from(slice)
        } else {
            Self::default()
        }
    }

    pub fn set(&mut self, index: usize, elem: Value) {
        if let Some(cell) = self.0.get_mut(index) {
            *cell = elem.inner();
        } else {
            let nil = Value::nil().inner();
            let buflen = self.len();
            // index is *at least* buflen, so this calculation never underflows
            // and ensures we allocate an additional slot.
            self.0.reserve(index + 1 - buflen);
            for _ in buflen..index {
                self.0.push(nil);
            }
            self.0.push(elem.inner());
        }
    }

    pub fn set_with_drain(&mut self, start: usize, drain: usize, elem: Value) -> usize {
        let buflen = self.len();
        let drained = cmp::min(buflen.checked_sub(start).unwrap_or_default(), drain);

        if let Some(cell) = self.0.get_mut(start) {
            match drain {
                0 => self.0.insert(start, elem.inner()),
                1 => *cell = elem.inner(),
                _ => {
                    *cell = elem.inner();
                    let drain_end_idx = cmp::min(start + drain, buflen);
                    self.0.drain(start + 1..drain_end_idx);
                }
            }
        } else {
            let nil = Value::nil().inner();
            // start is *at least* buflen, so this calculation never underflows
            // and ensures we allocate an additional slot.
            self.0.reserve(start + 1 - buflen);
            for _ in buflen..start {
                self.0.push(nil);
            }
            self.0.push(elem.inner());
        }

        drained
    }

    pub fn set_slice(&mut self, start: usize, drain: usize, src: &[sys::mrb_value]) -> usize {
        let buflen = self.0.len();
        let drained = cmp::min(buflen.checked_sub(start).unwrap_or_default(), drain);

        if src.len() <= drain {
            if let Some(slice) = self.0.get_mut(start..start + src.len()) {
                slice.copy_from_slice(src);
                if drained - src.len() > 0 {
                    self.0.drain(start + src.len()..start + drained);
                }
            } else if let Some(slice) = self.0.get_mut(start..start + drained) {
                let (left, right) = src.split_at(drained);
                slice.copy_from_slice(left);
                self.0.extend_from_slice(right);
            } else {
                let nil = Value::nil().inner();
                for _ in buflen..start {
                    self.0.push(nil);
                }
                self.0.extend_from_slice(src);
            }
        } else if let Some(slice) = self.0.get_mut(start..start + drain) {
            let (left, right) = src.split_at(drain);
            slice.copy_from_slice(left);
            self.0.insert_from_slice(start + drain, right);
        } else if let Some(slice) = self.0.get_mut(start..start + drained) {
            let (left, right) = src.split_at(drained);
            slice.copy_from_slice(left);
            self.0.extend_from_slice(right);
        } else {
            let nil = Value::nil().inner();
            for _ in buflen..start {
                self.0.push(nil);
            }
            self.0.extend_from_slice(src);
        }

        drained
    }

    pub fn concat<T>(&mut self, other: T)
    where
        T: IntoIterator<Item = Value>,
    {
        for elem in other {
            self.0.push(elem.inner());
        }
    }

    pub fn pop(&mut self) -> Option<Value> {
        self.0.pop().map(Value::from)
    }

    pub fn push(&mut self, elem: Value) {
        self.0.push(elem.inner());
    }

    pub fn reverse(&mut self) {
        self.0.reverse();
    }
}

#[cfg(test)]
mod tests {
    use bstr::ByteSlice;

    use crate::test::prelude::*;

    #[test]
    fn integration_test() {
        let mut interp = crate::interpreter().unwrap();
        let _ = interp
            .eval(&include_bytes!("inline_buffer_test.rb")[..])
            .unwrap();
        let result = interp.eval(b"spec");
        if let Err(exc) = result {
            let backtrace = exc.vm_backtrace(&mut interp);
            let backtrace = bstr::join("\n", backtrace.unwrap_or_default());
            panic!(
                "InlineBuffer tests failed with backtrace:\n{:?}",
                backtrace.as_bstr()
            );
        }
    }
}
