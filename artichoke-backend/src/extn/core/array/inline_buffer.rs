use arrayvec::ArrayVec;
use std::cmp;
use std::iter;

use crate::convert::Convert;
use crate::extn::core::array::ArrayType;
use crate::extn::core::exception::RubyException;
use crate::gc::MrbGarbageCollection;
use crate::sys;
use crate::value::Value;
use crate::Artichoke;

const INLINE_CAPACITY: usize = 8;

#[derive(Clone)]
pub enum InlineBuffer {
    Dynamic(Vec<sys::mrb_value>),
    Inline(ArrayVec<[sys::mrb_value; INLINE_CAPACITY]>),
}

impl Default for InlineBuffer {
    fn default() -> Self {
        Self::Inline(ArrayVec::new())
    }
}

impl From<Vec<sys::mrb_value>> for InlineBuffer {
    fn from(values: Vec<sys::mrb_value>) -> Self {
        if values.len() <= INLINE_CAPACITY {
            let mut inline = ArrayVec::new();
            inline.extend(values);
            Self::Inline(inline)
        } else {
            Self::Dynamic(values)
        }
    }
}

impl From<Vec<Value>> for InlineBuffer {
    fn from(values: Vec<Value>) -> Self {
        Self::from(values.as_slice())
    }
}

impl<'a> From<&'a [sys::mrb_value]> for InlineBuffer {
    fn from(values: &'a [sys::mrb_value]) -> Self {
        if values.len() <= INLINE_CAPACITY {
            let mut inline = ArrayVec::new();
            inline.extend(values.iter().copied());
            Self::Inline(inline)
        } else {
            Self::Dynamic(values.to_vec())
        }
    }
}

impl<'a> From<&'a [Value]> for InlineBuffer {
    fn from(values: &'a [Value]) -> Self {
        if values.len() <= INLINE_CAPACITY {
            let mut inline = ArrayVec::new();
            inline.extend(values.iter().map(Value::inner));
            Self::Inline(inline)
        } else {
            Self::Dynamic(values.iter().map(Value::inner).collect())
        }
    }
}

impl ArrayType for InlineBuffer {
    fn box_clone(&self) -> Box<dyn ArrayType> {
        Box::new(self.clone())
    }

    fn gc_mark(&self, interp: &Artichoke) {
        match self {
            Self::Dynamic(buffer) => {
                for element in buffer {
                    interp.mark_value(&Value::new(interp, *element));
                }
            }
            Self::Inline(buffer) => {
                for element in buffer {
                    interp.mark_value(&Value::new(interp, *element));
                }
            }
        }
    }

    fn real_children(&self) -> usize {
        match self {
            Self::Dynamic(buffer) => buffer.len(),
            Self::Inline(buffer) => buffer.len(),
        }
    }

    fn len(&self) -> usize {
        match self {
            Self::Dynamic(buffer) => buffer.len(),
            Self::Inline(buffer) => buffer.len(),
        }
    }

    fn is_empty(&self) -> bool {
        match self {
            Self::Dynamic(buffer) => buffer.is_empty(),
            Self::Inline(buffer) => buffer.is_empty(),
        }
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
    pub fn with_capacity(capacity: usize) -> Self {
        if capacity <= INLINE_CAPACITY {
            Self::Inline(ArrayVec::new())
        } else {
            Self::Dynamic(Vec::with_capacity(capacity))
        }
    }

    pub fn as_vec(&self, interp: &Artichoke) -> Vec<Value> {
        match self {
            Self::Dynamic(buffer) => buffer
                .iter()
                .copied()
                .map(|value| Value::new(interp, value))
                .collect(),
            Self::Inline(buffer) => buffer
                .iter()
                .copied()
                .map(|value| Value::new(interp, value))
                .collect(),
        }
    }

    pub fn as_ptr(&self) -> *const sys::mrb_value {
        match self {
            Self::Dynamic(buffer) => buffer.as_ptr(),
            Self::Inline(buffer) => buffer.as_ptr(),
        }
    }

    pub fn as_mut_ptr(&mut self) -> *mut sys::mrb_value {
        match self {
            Self::Dynamic(buffer) => buffer.as_mut_ptr(),
            Self::Inline(buffer) => buffer.as_mut_ptr(),
        }
    }

    pub unsafe fn set_len(&mut self, len: usize) {
        match self {
            Self::Dynamic(buffer) => buffer.set_len(len),
            Self::Inline(buffer) => buffer.set_len(len),
        }
    }

    pub fn clear(&mut self) {
        match self {
            Self::Dynamic(buffer) => buffer.clear(),
            Self::Inline(buffer) => buffer.clear(),
        }
    }

    pub fn get(&self, interp: &Artichoke, index: usize) -> Result<Value, Box<dyn RubyException>> {
        let elem = match self {
            Self::Dynamic(buffer) => buffer.get(index),
            Self::Inline(buffer) => buffer.get(index),
        };
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
            match self {
                Self::Dynamic(buffer) => {
                    Ok(Self::from(&buffer[start..cmp::min(start + len, buflen)]))
                }
                Self::Inline(buffer) => {
                    Ok(Self::from(&buffer[start..cmp::min(start + len, buflen)]))
                }
            }
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
        match self {
            Self::Dynamic(ref mut buffer) => {
                if index < buflen {
                    buffer[index] = elem.inner();
                } else if index == buflen {
                    buffer.push(elem.inner());
                } else {
                    buffer.reserve(index + 1 - buflen);
                    let nil = interp.convert(None::<Value>).inner();
                    for _ in buflen..index {
                        buffer.push(nil);
                    }
                    buffer.push(elem.inner());
                }
            }
            Self::Inline(ref mut buffer) => {
                if index < buflen {
                    buffer[index] = elem.inner();
                } else if index == buflen {
                    buffer.push(elem.inner());
                } else if index < buffer.capacity() {
                    let nil = interp.convert(None::<Value>).inner();
                    for _ in buflen..index {
                        buffer.push(nil);
                    }
                    buffer.push(elem.inner());
                } else {
                    let mut dynamic = Vec::with_capacity(index + 1);
                    let nil = interp.convert(None::<Value>).inner();
                    dynamic.extend(buffer.drain(..));
                    for _ in buflen..index {
                        dynamic.push(nil);
                    }
                    dynamic.push(elem.inner());
                    *self = Self::Dynamic(dynamic);
                }
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
        if start > buflen {
            set_with_drain_sparse(interp, self, start, with);
        } else if start == buflen {
            match self {
                Self::Dynamic(ref mut buffer) => buffer.push(with.inner()),
                Self::Inline(ref mut buffer) if buffer.remaining_capacity() > 0 => {
                    buffer.push(with.inner())
                }
                Self::Inline(ref mut buffer) => {
                    let mut dynamic = Vec::with_capacity(INLINE_CAPACITY + 1);
                    dynamic.extend(buffer.drain(..));
                    dynamic.push(with.inner());
                    *self = Self::Dynamic(dynamic);
                }
            }
        } else {
            let newlen = (buflen + 1).checked_sub(drained).unwrap_or_default();
            match self {
                Self::Dynamic(ref mut buffer) if newlen <= INLINE_CAPACITY => {
                    let mut inline = ArrayVec::new();
                    if start < buffer.len() {
                        inline.extend(buffer.drain(..start));
                    } else {
                        inline.extend(buffer.drain(..));
                    }
                    inline.push(with.inner());
                    if drain < buffer.len() {
                        inline.extend(buffer.drain(drain..));
                    }
                    *self = Self::Inline(inline);
                }
                Self::Dynamic(ref mut buffer) => {
                    let tail_start_idx = cmp::min(start + drain, buflen);
                    buffer.splice(start..tail_start_idx, iter::once(with.inner()));
                }
                Self::Inline(ref mut buffer) if newlen <= INLINE_CAPACITY => {
                    let mut inline = ArrayVec::new();
                    if start < buffer.len() {
                        inline.extend(buffer.drain(..start));
                    } else {
                        inline.extend(buffer.drain(..));
                    }
                    inline.push(with.inner());
                    if drain < buffer.len() {
                        inline.extend(buffer.drain(drain..));
                    }
                    *self = Self::Inline(inline);
                }
                Self::Inline(_) => {
                    // This branch is unreachable because an inline can only be
                    // promoted to a dynamic if:
                    //
                    // - start == INLINE_CAPACITY, handled by above branch
                    // - start > INLINE_CAPACITY, handled by the sparse branch
                    unreachable!("Inline variant promoted to Dynamic");
                }
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
        let newlen = buflen - drained + with.len();
        if start > buflen {
            set_slice_with_drain_sparse(interp, self, start, with);
        } else if newlen <= INLINE_CAPACITY {
            set_slice_with_drain_to_inline(self, start, drain, with);
        } else {
            set_slice_with_drain_to_dynamic(self, start, drain, with);
        }
        Ok(drained)
    }

    pub fn concat(
        &mut self,
        interp: &Artichoke,
        other: &Self,
    ) -> Result<(), Box<dyn RubyException>> {
        let _ = interp;
        if self.len() + other.len() <= INLINE_CAPACITY {
            concat_to_inline(self, other);
        } else {
            concat_to_dynamic(self, other);
        }
        Ok(())
    }

    pub fn pop(&mut self, interp: &Artichoke) -> Result<Value, Box<dyn RubyException>> {
        let value = match self {
            // The `Dynamic` variant does NOT realloc into an `Inline` if len
            // can fit within `INLINE_CAPACITY`.
            Self::Dynamic(buffer) => buffer.pop(),
            Self::Inline(buffer) => buffer.pop(),
        };
        Ok(interp.convert(value.map(|value| Value::new(interp, value))))
    }

    pub fn reverse(&mut self, interp: &Artichoke) -> Result<(), Box<dyn RubyException>> {
        let _ = interp;
        match self {
            Self::Dynamic(ref mut buffer) => {
                buffer.reverse();
            }
            Self::Inline(ref mut buffer) if buffer.is_empty() => {}
            Self::Inline(ref mut buffer) => {
                let mut left = 0;
                let mut right = buffer.len() - 1;
                while left < right {
                    buffer.swap(left, right);
                    left += 1;
                    right -= 1;
                }
            }
        }
        Ok(())
    }
}

fn set_with_drain_sparse(interp: &Artichoke, ary: &mut InlineBuffer, start: usize, elem: Value) {
    let nil = interp.convert(None::<Value>).inner();
    let buflen = ary.len();
    if start < INLINE_CAPACITY {
        match ary {
            InlineBuffer::Dynamic(buffer) => {
                let mut inline = ArrayVec::new();
                inline.extend(buffer.drain(..));
                for _ in buflen..start {
                    inline.push(nil);
                }
                inline.push(elem.inner());
                *ary = InlineBuffer::Inline(inline);
            }
            InlineBuffer::Inline(ref mut buffer) => {
                for _ in buflen..start {
                    buffer.push(nil);
                }
                buffer.push(elem.inner());
            }
        }
    } else {
        match ary {
            InlineBuffer::Dynamic(ref mut buffer) => {
                buffer.reserve(start + 1 - buflen);
                for _ in buflen..start {
                    buffer.push(nil);
                }
                buffer.push(elem.inner());
            }
            InlineBuffer::Inline(buffer) => {
                let mut dynamic = Vec::with_capacity(start + 1);
                dynamic.extend(buffer.drain(..));
                for _ in buflen..start {
                    dynamic.push(nil);
                }
                dynamic.push(elem.inner());
                *ary = InlineBuffer::Dynamic(dynamic);
            }
        }
    }
}

fn set_slice_with_drain_sparse(
    interp: &Artichoke,
    ary: &mut InlineBuffer,
    start: usize,
    with: &InlineBuffer,
) {
    let buflen = ary.len();
    let nil = interp.convert(None::<Value>).inner();
    match ary {
        InlineBuffer::Dynamic(ref mut buffer) => {
            for _ in buflen..start {
                buffer.push(nil);
            }
            match with {
                InlineBuffer::Dynamic(with) => {
                    buffer.extend_from_slice(with.as_slice());
                }
                InlineBuffer::Inline(with) => {
                    buffer.extend_from_slice(with.as_slice());
                }
            }
        }
        InlineBuffer::Inline(ref mut buffer) if start < INLINE_CAPACITY => {
            for _ in buflen..start {
                buffer.push(nil);
            }
            match with {
                InlineBuffer::Dynamic(with) => {
                    if buffer.try_extend_from_slice(with.as_slice()).is_err() {
                        let mut dynamic = buffer.as_slice().to_vec();
                        dynamic.extend_from_slice(with.as_slice());
                        *ary = InlineBuffer::Dynamic(dynamic);
                    }
                }
                InlineBuffer::Inline(with) => {
                    if buffer.try_extend_from_slice(with.as_slice()).is_err() {
                        let mut dynamic = buffer.as_slice().to_vec();
                        dynamic.extend_from_slice(with.as_slice());
                        *ary = InlineBuffer::Dynamic(dynamic);
                    }
                }
            }
        }
        InlineBuffer::Inline(ref buffer) => {
            let mut dynamic = buffer.as_slice().to_vec();
            for _ in buflen..start {
                dynamic.push(nil);
            }
            match with {
                InlineBuffer::Dynamic(with) => {
                    dynamic.extend_from_slice(with.as_slice());
                }
                InlineBuffer::Inline(with) => {
                    dynamic.extend_from_slice(with.as_slice());
                }
            }
            *ary = InlineBuffer::Dynamic(dynamic);
        }
    }
}

fn set_slice_with_drain_to_inline(
    ary: &mut InlineBuffer,
    start: usize,
    drain: usize,
    with: &InlineBuffer,
) {
    match ary {
        InlineBuffer::Dynamic(ref mut buffer) => {
            let mut inline = ArrayVec::new();
            if start < buffer.len() {
                inline.extend(buffer.drain(..start));
            } else {
                inline.extend(buffer.drain(..));
            }
            if drain < buffer.len() {
                buffer.drain(..drain);
            } else {
                buffer.clear();
            }
            match with {
                InlineBuffer::Dynamic(with) => {
                    if inline.try_extend_from_slice(with.as_slice()).is_err() {
                        let mut dynamic = inline.as_slice().to_vec();
                        dynamic.extend_from_slice(with.as_slice());
                        dynamic.append(buffer);
                        *ary = InlineBuffer::Dynamic(dynamic);
                    } else {
                        inline.extend(buffer.iter().copied());
                        *ary = InlineBuffer::Inline(inline);
                    }
                }
                InlineBuffer::Inline(with) => {
                    if inline.try_extend_from_slice(with.as_slice()).is_err() {
                        let mut dynamic = inline.as_slice().to_vec();
                        dynamic.extend_from_slice(with.as_slice());
                        dynamic.append(buffer);
                        *ary = InlineBuffer::Dynamic(dynamic);
                    } else {
                        inline.extend(buffer.iter().copied());
                        *ary = InlineBuffer::Inline(inline);
                    }
                }
            }
        }
        InlineBuffer::Inline(ref mut buffer) => {
            let mut inline = ArrayVec::new();
            if start < buffer.len() {
                inline.extend(buffer.drain(..start));
            } else {
                inline.extend(buffer.drain(..));
            }
            if drain < buffer.len() {
                buffer.drain(..drain);
            } else {
                buffer.clear();
            }
            match with {
                InlineBuffer::Dynamic(with) => {
                    if inline.try_extend_from_slice(with.as_slice()).is_err() {
                        let mut dynamic = inline.as_slice().to_vec();
                        dynamic.extend_from_slice(with.as_slice());
                        dynamic.extend_from_slice(buffer.as_slice());
                        *ary = InlineBuffer::Dynamic(dynamic);
                    } else {
                        inline.extend(buffer.iter().copied());
                        *ary = InlineBuffer::Inline(inline);
                    }
                }
                InlineBuffer::Inline(with) => {
                    if inline.try_extend_from_slice(with.as_slice()).is_err() {
                        let mut dynamic = inline.as_slice().to_vec();
                        dynamic.extend_from_slice(with.as_slice());
                        dynamic.extend_from_slice(buffer.as_slice());
                        *ary = InlineBuffer::Dynamic(dynamic);
                    } else {
                        inline.extend(buffer.iter().copied());
                        *ary = InlineBuffer::Inline(inline);
                    }
                }
            }
        }
    }
}

fn set_slice_with_drain_to_dynamic(
    ary: &mut InlineBuffer,
    start: usize,
    drain: usize,
    with: &InlineBuffer,
) {
    let tail_start_idx = cmp::min(start + drain, ary.len());
    match ary {
        InlineBuffer::Dynamic(ref mut buffer) => match with {
            InlineBuffer::Dynamic(with) => {
                buffer.splice(start..tail_start_idx, with.iter().copied());
            }
            InlineBuffer::Inline(with) => {
                buffer.splice(start..tail_start_idx, with.as_slice().iter().copied());
            }
        },
        InlineBuffer::Inline(buffer) => {
            let mut dynamic = buffer.as_slice().to_vec();
            match with {
                InlineBuffer::Dynamic(with) => {
                    dynamic.splice(start..tail_start_idx, with.iter().copied());
                }
                InlineBuffer::Inline(with) => {
                    dynamic.splice(start..tail_start_idx, with.as_slice().iter().copied());
                }
            }
            *ary = InlineBuffer::Dynamic(dynamic);
        }
    }
}

fn concat_to_inline(ary: &mut InlineBuffer, other: &InlineBuffer) {
    let mut inline = ArrayVec::new();
    match ary {
        InlineBuffer::Dynamic(buffer) => inline.extend(buffer.drain(..)),
        InlineBuffer::Inline(buffer) => inline.extend(buffer.drain(..)),
    }
    match other {
        InlineBuffer::Dynamic(buffer) => inline.extend(buffer.as_slice().iter().copied()),
        InlineBuffer::Inline(buffer) => inline.extend(buffer.as_slice().iter().copied()),
    }
    *ary = InlineBuffer::Inline(inline);
}

fn concat_to_dynamic(ary: &mut InlineBuffer, other: &InlineBuffer) {
    let mut dynamic = Vec::with_capacity(ary.len() + other.len());
    match ary {
        InlineBuffer::Dynamic(buffer) => dynamic.extend_from_slice(buffer.as_slice()),
        InlineBuffer::Inline(buffer) => dynamic.extend_from_slice(buffer.as_slice()),
    }
    match other {
        InlineBuffer::Dynamic(buffer) => dynamic.extend_from_slice(buffer.as_slice()),
        InlineBuffer::Inline(buffer) => dynamic.extend_from_slice(buffer.as_slice()),
    }
    *ary = InlineBuffer::Dynamic(dynamic);
}

#[cfg(test)]
mod tests {
    use artichoke_core::eval::Eval;
    use artichoke_core::value::Value;

    #[test]
    fn integration_test() {
        let interp = crate::interpreter().unwrap();
        interp
            .eval(&include_bytes!("inline_buffer_test.rb")[..])
            .unwrap();
        let result = interp.eval(b"spec");
        let result = result.and_then(Value::try_into::<bool>);
        assert_eq!(Ok(true), result);
    }
}
