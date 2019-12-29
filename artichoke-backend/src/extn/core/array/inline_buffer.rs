use smallvec::SmallVec;
use std::cmp;
use std::iter::FromIterator;

use crate::convert::Convert;
use crate::extn::core::array::ArrayType;
use crate::extn::core::exception::RubyException;
use crate::gc::MrbGarbageCollection;
use crate::sys;
use crate::value::Value;
use crate::Artichoke;

const INLINE_CAPACITY: usize = 8;

#[derive(Clone, Default)]
pub struct InlineBuffer(SmallVec<[sys::mrb_value; INLINE_CAPACITY]>);

impl From<Vec<sys::mrb_value>> for InlineBuffer {
    #[must_use]
    fn from(values: Vec<sys::mrb_value>) -> Self {
        Self(SmallVec::from_vec(values))
    }
}

impl From<Vec<Value>> for InlineBuffer {
    #[must_use]
    fn from(values: Vec<Value>) -> Self {
        Self::from(values.as_slice())
    }
}

impl<'a> From<&'a [sys::mrb_value]> for InlineBuffer {
    #[must_use]
    fn from(values: &'a [sys::mrb_value]) -> Self {
        Self(SmallVec::from_slice(values))
    }
}

impl<'a> From<&'a [Value]> for InlineBuffer {
    #[must_use]
    fn from(values: &'a [Value]) -> Self {
        Self(SmallVec::from_iter(values.iter().map(Value::inner)))
    }
}

impl ArrayType for InlineBuffer {
    #[must_use]
    fn box_clone(&self) -> Box<dyn ArrayType> {
        Box::new(self.clone())
    }

    fn gc_mark(&self, interp: &Artichoke) {
        for elem in self.0.iter().copied() {
            let value = Value::new(interp, elem);
            interp.mark_value(&value);
        }
    }

    #[must_use]
    fn real_children(&self) -> usize {
        self.0.len()
    }

    #[must_use]
    fn len(&self) -> usize {
        self.0.len()
    }

    #[must_use]
    fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    fn get(&self, interp: &Artichoke, index: usize) -> Result<Value, Box<dyn RubyException>> {
        Self::get(self, interp, index)
    }

    fn slice(
        &self,
        interp: &Artichoke,
        start: usize,
        len: usize,
    ) -> Result<Box<dyn ArrayType>, Box<dyn RubyException>> {
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
    ) -> Result<(), Box<dyn RubyException>> {
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
    ) -> Result<usize, Box<dyn RubyException>> {
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
    ) -> Result<usize, Box<dyn RubyException>> {
        let _ = realloc;
        if let Ok(buffer) = with.downcast_ref::<Self>() {
            Self::set_slice(self, interp, start, drain, buffer)
        } else {
            unimplemented!("Set slice on InlineBuffer with other ArrayType");
        }
    }

    fn concat(
        &mut self,
        interp: &Artichoke,
        other: Box<dyn ArrayType>,
        realloc: &mut Option<Vec<Box<dyn ArrayType>>>,
    ) -> Result<(), Box<dyn RubyException>> {
        let _ = realloc;
        if let Ok(buffer) = other.downcast_ref::<Self>() {
            Self::concat(self, interp, buffer)
        } else {
            unimplemented!("Set slice on InlineBuffer with other ArrayType");
        }
    }

    fn pop(
        &mut self,
        interp: &Artichoke,
        realloc: &mut Option<Vec<Box<dyn ArrayType>>>,
    ) -> Result<Value, Box<dyn RubyException>> {
        let _ = realloc;
        Self::pop(self, interp)
    }

    fn reverse(&mut self, interp: &Artichoke) -> Result<(), Box<dyn RubyException>> {
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

    pub fn get(&self, interp: &Artichoke, index: usize) -> Result<Value, Box<dyn RubyException>> {
        let elem = self.0.get(index);
        let elem = elem.copied().map(|elem| Value::new(interp, elem));
        Ok(interp.convert(elem))
    }

    pub fn slice(
        &self,
        interp: &Artichoke,
        start: usize,
        len: usize,
    ) -> Result<Self, Box<dyn RubyException>> {
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

    pub fn set(
        &mut self,
        interp: &Artichoke,
        index: usize,
        elem: Value,
    ) -> Result<(), Box<dyn RubyException>> {
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
    ) -> Result<usize, Box<dyn RubyException>> {
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
    ) -> Result<usize, Box<dyn RubyException>> {
        let _ = interp;
        let buflen = self.len();
        let drained = cmp::min(buflen.checked_sub(start).unwrap_or_default(), drain);
        if start > buflen {
            let nil = interp.convert(None::<Value>).inner();
            for _ in buflen..start {
                self.0.push(nil);
            }
            self.0.extend_from_slice(with.0.as_slice());
        } else if start == buflen {
            self.0.extend_from_slice(with.0.as_slice());
        } else if drain == 0 {
            self.0.insert_from_slice(start, with.0.as_slice());
        } else {
            let split_at = cmp::min(cmp::min(buflen - start, drain), with.0.len());
            let overwrite = &with.0[..split_at];
            let insert = &with.0[split_at..];
            let drain = drain.checked_sub(with.0.len());
            self.0[start..start + split_at].copy_from_slice(overwrite);
            self.0.insert_from_slice(start + split_at, insert);
            if let Some(drain) = drain {
                let end_of_insert = start + with.0.len();
                let end_of_drain = cmp::min(end_of_insert + drain, self.0.len());
                self.0.drain(end_of_insert..end_of_drain);
            }
        }
        Ok(drained)
    }

    pub fn concat(
        &mut self,
        interp: &Artichoke,
        other: &Self,
    ) -> Result<(), Box<dyn RubyException>> {
        let _ = interp;
        self.0.extend_from_slice(other.0.as_slice());
        Ok(())
    }

    pub fn pop(&mut self, interp: &Artichoke) -> Result<Value, Box<dyn RubyException>> {
        let value = self.0.pop();
        Ok(interp.convert(value.map(|value| Value::new(interp, value))))
    }

    pub fn reverse(&mut self, interp: &Artichoke) -> Result<(), Box<dyn RubyException>> {
        let _ = interp;
        if let Some(mut right) = self.0.len().checked_sub(1) {
            let mut left = 0;
            while left < right {
                self.0.swap(left, right);
                left += 1;
                right -= 1;
            }
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use artichoke_core::eval::Eval;
    use artichoke_core::value::Value;

    #[test]
    fn integration_test() {
        let interp = crate::interpreter().unwrap();
        let _ = interp
            .eval(&include_bytes!("inline_buffer_test.rb")[..])
            .unwrap();
        let result = interp.eval(b"spec");
        let result = result.and_then(Value::try_into::<bool>);
        assert_eq!(Ok(true), result);
    }
}
