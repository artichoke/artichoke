use arrayvec::ArrayVec;

use crate::convert::Convert;
use crate::extn::core::exception::RubyException;
use crate::gc::MrbGarbageCollection;
use crate::sys;
use crate::value::Value;
use crate::Artichoke;

#[derive(Clone)]
pub struct InlineBuffer {
    dynamic: Option<Vec<sys::mrb_value>>,
    inline: ArrayVec<[sys::mrb_value; 8]>,
}

impl Default for InlineBuffer {
    fn default() -> Self {
        Self {
            dynamic: None,
            inline: ArrayVec::new(),
        }
    }
}

impl From<Vec<sys::mrb_value>> for InlineBuffer {
    fn from(values: Vec<sys::mrb_value>) -> Self {
        let mut inline = ArrayVec::new();
        if values.len() < inline.capacity() {
            for elem in values {
                unsafe { inline.push_unchecked(elem) };
            }
            Self {
                dynamic: None,
                inline,
            }
        } else {
            Self {
                dynamic: Some(values),
                inline,
            }
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
        let mut inline = ArrayVec::new();
        if values.len() < inline.capacity() {
            for elem in values {
                unsafe { inline.push_unchecked(*elem) };
            }
            Self {
                dynamic: None,
                inline,
            }
        } else {
            Self {
                dynamic: Some(values.to_vec()),
                inline,
            }
        }
    }
}

impl<'a> From<&'a [Value]> for InlineBuffer {
    fn from(values: &'a [Value]) -> Self {
        let mut inline = ArrayVec::new();
        if values.len() < inline.capacity() {
            for elem in values {
                unsafe { inline.push_unchecked(elem.inner()) };
            }
            Self {
                dynamic: None,
                inline,
            }
        } else {
            Self {
                dynamic: Some(values.iter().map(Value::inner).collect()),
                inline,
            }
        }
    }
}

impl InlineBuffer {
    pub fn with_capacity(capacity: usize) -> Self {
        let mut buffer = Self::default();
        if capacity > buffer.inline.capacity() {
            buffer.dynamic = Some(Vec::with_capacity(capacity));
        }
        buffer
    }

    pub fn as_vec(&self, interp: &Artichoke) -> Vec<Value> {
        if let Some(ref buffer) = self.dynamic {
            buffer
                .iter()
                .map(|value| Value::new(interp, *value))
                .collect()
        } else {
            self.inline
                .iter()
                .map(|value| Value::new(interp, *value))
                .collect()
        }
    }

    pub fn as_ptr(&self) -> *const sys::mrb_value {
        if let Some(ref buffer) = self.dynamic {
            buffer.as_ptr()
        } else {
            self.inline.as_ptr()
        }
    }

    pub fn as_mut_ptr(&mut self) -> *mut sys::mrb_value {
        if let Some(ref mut buffer) = self.dynamic {
            buffer.as_mut_ptr()
        } else {
            self.inline.as_mut_ptr()
        }
    }

    pub unsafe fn set_len(&mut self, len: usize) {
        if let Some(ref mut buffer) = self.dynamic {
            buffer.set_len(len);
        } else {
            self.inline.set_len(len);
        }
    }

    pub fn clear(&mut self) {
        self.inline.clear();
        if let Some(ref mut buffer) = self.dynamic {
            buffer.clear();
        }
    }

    pub fn gc_mark(&self, interp: &Artichoke) {
        if let Some(ref buffer) = self.dynamic {
            for element in buffer {
                interp.mark_value(&Value::new(interp, *element));
            }
        } else {
            for element in &self.inline {
                interp.mark_value(&Value::new(interp, *element));
            }
        }
    }

    pub fn real_children(&self) -> usize {
        if let Some(ref buffer) = self.dynamic {
            buffer.len()
        } else {
            self.inline.len()
        }
    }

    pub fn len(&self) -> usize {
        if let Some(ref buffer) = self.dynamic {
            buffer.len()
        } else {
            self.inline.len()
        }
    }

    pub fn is_empty(&self) -> bool {
        if let Some(ref buffer) = self.dynamic {
            buffer.is_empty()
        } else {
            self.inline.is_empty()
        }
    }

    pub fn get(&self, interp: &Artichoke, index: usize) -> Result<Value, Box<dyn RubyException>> {
        if let Some(ref buffer) = self.dynamic {
            Ok(interp.convert(buffer.get(index).map(|elem| Value::new(interp, *elem))))
        } else {
            let elem = self.inline.get(index).map(|elem| Value::new(interp, *elem));
            Ok(interp.convert(elem))
        }
    }

    pub fn slice(
        &self,
        interp: &Artichoke,
        start: usize,
        len: usize,
    ) -> Result<Self, Box<dyn RubyException>> {
        let _ = interp;
        if start < self.len() {
            if let Some(ref buffer) = self.dynamic {
                let iter = buffer.iter().skip(start).take(len);
                Ok(Self::from(iter.cloned().collect::<Vec<_>>()))
            } else {
                let iter = self.inline.iter().skip(start).take(len);
                Ok(Self::from(iter.cloned().collect::<Vec<_>>()))
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
        if index < buflen {
            if let Some(ref mut buffer) = self.dynamic {
                buffer[index] = elem.inner();
            } else {
                self.inline[index] = elem.inner();
            }
        } else if let Some(ref mut buffer) = self.dynamic {
            buffer.extend(vec![unsafe { sys::mrb_sys_nil_value() }; index - buflen]);
            buffer.push(elem.inner());
        } else if index < self.inline.capacity() {
            for _ in buflen..index {
                unsafe {
                    self.inline.push_unchecked(sys::mrb_sys_nil_value());
                }
            }
            unsafe {
                self.inline.push_unchecked(elem.inner());
            }
        } else {
            let mut buffer = Vec::from(self.inline.as_slice());
            buffer.extend(vec![unsafe { sys::mrb_sys_nil_value() }; index - buflen]);
            buffer.push(elem.inner());
            self.dynamic = Some(buffer);
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
        let drained = std::cmp::min(buflen.checked_sub(start).unwrap_or_default(), drain);
        if start > buflen {
            set_with_drain_sparse(self, start, with);
        } else if (buflen + 1).checked_sub(drain).unwrap_or_default() < self.inline.capacity() {
            set_with_drain_to_inline(self, start, drain, with);
        } else if let Some(ref mut buffer) = self.dynamic {
            buffer.push(with.inner());
        } else {
            let mut buffer = self.inline.as_slice().to_vec();
            let nil = unsafe { sys::mrb_sys_nil_value() };
            for _ in buflen..start {
                buffer.push(nil);
            }
            buffer.push(with.inner());
            self.dynamic = Some(buffer);
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
        let drained = std::cmp::min(buflen.checked_sub(start).unwrap_or_default(), drain);
        let newlen = start
            + buflen
                .checked_sub(start)
                .and_then(|tail| tail.checked_sub(drain))
                .unwrap_or_default()
            + with.len();
        if start > buflen {
            set_slice_with_drain_sparse(self, start, with);
        } else if newlen < self.inline.capacity() {
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
        if self.len() + other.len() < self.inline.capacity() {
            concat_to_inline(self, other);
        } else {
            concat_to_dynamic(self, other);
        }
        Ok(())
    }

    pub fn pop(&mut self, interp: &Artichoke) -> Result<Value, Box<dyn RubyException>> {
        let value = if let Some(ref mut buffer) = self.dynamic {
            buffer.pop()
        } else {
            self.inline.pop()
        };
        Ok(interp.convert(value.map(|value| Value::new(interp, value))))
    }

    pub fn reverse(&mut self, interp: &Artichoke) -> Result<(), Box<dyn RubyException>> {
        let _ = interp;
        if let Some(ref mut buffer) = self.dynamic {
            buffer.reverse();
        } else if !self.inline.is_empty() {
            let mut left = 0;
            let mut right = self.inline.len() - 1;
            while left < right {
                self.inline.swap(left, right);
                left += 1;
                right -= 1;
            }
        }
        Ok(())
    }
}

fn set_with_drain_sparse(ary: &mut InlineBuffer, start: usize, elem: Value) {
    let nil = unsafe { sys::mrb_sys_nil_value() };
    let buflen = ary.len();
    if start < ary.inline.capacity() {
        if let Some(ref buffer) = ary.dynamic {
            for idx in 0..buflen {
                ary.inline[idx] = unsafe { *buffer.get_unchecked(idx) };
            }
            for idx in buflen..start {
                ary.inline[idx] = nil;
            }
            ary.inline.insert(start, elem.inner());
            ary.dynamic = None;
        } else {
            for idx in buflen..start {
                ary.inline[idx] = nil;
            }
            ary.inline.insert(start, elem.inner());
        }
        unsafe {
            ary.inline.set_len(start + 1);
        }
    } else if let Some(ref mut buffer) = ary.dynamic {
        for _ in buflen..start {
            buffer.push(nil);
        }
        buffer.push(elem.inner());
    } else {
        let mut buffer = Vec::with_capacity(start + 1);
        for elem in &ary.inline {
            buffer.push(*elem);
        }
        for _ in buflen..start {
            buffer.push(nil);
        }
        buffer.push(elem.inner());
        ary.dynamic = Some(buffer);
    }
}

fn set_with_drain_to_inline(ary: &mut InlineBuffer, start: usize, drain: usize, elem: Value) {
    let buflen = ary.len();
    if let Some(ref buffer) = ary.dynamic {
        for idx in 0..start {
            ary.inline[idx] = unsafe { *buffer.get_unchecked(idx) };
        }
        ary.inline.insert(start, elem.inner());
        if start + drain > ary.inline.len() {
            unsafe {
                ary.inline.set_len(start + 1);
            }
        } else {
            for _ in 0..drain {
                ary.inline.remove(start + 1);
            }
            unsafe {
                ary.inline.set_len(buflen - drain + 1);
            }
        }
        ary.dynamic = None;
    } else {
        ary.inline.insert(start, elem.inner());
        if start + drain > ary.inline.len() {
            unsafe {
                ary.inline.set_len(start + 1);
            }
        } else {
            for _ in 0..drain {
                ary.inline.remove(start + 1);
            }
            unsafe {
                ary.inline.set_len(buflen - drain + 1);
            }
        }
    }
}

fn set_slice_with_drain_sparse(ary: &mut InlineBuffer, start: usize, with: &InlineBuffer) {
    let buflen = ary.len();
    let nil = unsafe { sys::mrb_sys_nil_value() };
    if let Some(ref mut buffer) = ary.dynamic {
        for _ in buflen..start {
            buffer.push(nil);
        }
        if let Some(ref other) = with.dynamic {
            buffer.extend(other.clone());
        } else {
            for elem in &with.inline {
                buffer.push(*elem);
            }
        }
    } else if start + with.len() < ary.inline.capacity() {
        for _ in buflen..start {
            unsafe {
                ary.inline.push_unchecked(nil);
            }
        }
        if let Some(ref other) = with.dynamic {
            for elem in other {
                unsafe {
                    ary.inline.push_unchecked(*elem);
                }
            }
        } else {
            for elem in &with.inline {
                unsafe {
                    ary.inline.push_unchecked(*elem);
                }
            }
        }
    }
}

fn set_slice_with_drain_to_inline(
    ary: &mut InlineBuffer,
    start: usize,
    drain: usize,
    with: &InlineBuffer,
) {
    if let Some(ref mut buffer) = ary.dynamic {
        ary.inline.clear();
        for elem in buffer.drain(0..start) {
            unsafe {
                ary.inline.push_unchecked(elem);
            }
        }
        buffer.drain(0..drain);
        if let Some(ref other) = with.dynamic {
            for elem in other {
                unsafe {
                    ary.inline.push_unchecked(*elem);
                }
            }
        } else {
            for elem in &with.inline {
                unsafe {
                    ary.inline.push_unchecked(*elem);
                }
            }
        }
        for elem in buffer.drain(..) {
            unsafe {
                ary.inline.push_unchecked(elem);
            }
        }
        ary.dynamic = None;
    } else if start + drain > ary.inline.len() {
        unsafe {
            ary.inline.set_len(start);
        }
        if let Some(ref buffer) = with.dynamic {
            for elem in buffer {
                unsafe {
                    ary.inline.push_unchecked(*elem);
                }
            }
        } else {
            for elem in &with.inline {
                unsafe {
                    ary.inline.push_unchecked(*elem);
                }
            }
        }
    } else if drain >= with.len() {
        if let Some(ref buffer) = with.dynamic {
            for (idx, elem) in buffer.iter().enumerate() {
                ary.inline[start + idx] = *elem;
            }
        } else {
            for (idx, elem) in with.inline.iter().enumerate() {
                ary.inline[start + idx] = *elem;
            }
        }
        for _ in with.len()..drain {
            ary.inline.remove(start + with.len());
        }
    } else if let Some(ref buffer) = with.dynamic {
        for idx in 0..drain {
            ary.inline[start + idx] = unsafe { *buffer.get_unchecked(idx) };
        }
        for idx in (drain..with.len()).rev() {
            ary.inline
                .insert(start + drain, unsafe { *buffer.get_unchecked(idx) });
        }
    } else {
        for idx in 0..drain {
            ary.inline[start + idx] = unsafe { *with.inline.get_unchecked(idx) };
        }
        for idx in (drain..with.len()).rev() {
            ary.inline
                .insert(start + drain, unsafe { *with.inline.get_unchecked(idx) });
        }
    }
}

fn set_slice_with_drain_to_dynamic(
    ary: &mut InlineBuffer,
    start: usize,
    drain: usize,
    with: &InlineBuffer,
) {
    if let Some(ref mut buffer) = ary.dynamic {
        if let Some(ref other) = with.dynamic {
            buffer.splice(start..start + drain, other.clone());
        } else {
            buffer.splice(start..start + drain, with.inline.as_slice().to_vec());
        }
    } else {
        let mut buffer = ary.inline.as_slice().to_vec();
        if let Some(ref other) = with.dynamic {
            buffer.splice(start..start + drain, other.clone());
        } else {
            buffer.splice(start..start + drain, with.inline.as_slice().to_vec());
        }
        ary.dynamic = Some(buffer);
    }
}

fn concat_to_inline(ary: &mut InlineBuffer, other: &InlineBuffer) {
    match (ary.dynamic.as_mut(), other.dynamic.as_ref()) {
        (Some(buf), Some(otherbuf)) => {
            unsafe {
                ary.inline.set_len(0);
            }
            for elem in buf {
                unsafe {
                    ary.inline.push_unchecked(*elem);
                }
            }
            for elem in otherbuf {
                unsafe {
                    ary.inline.push_unchecked(*elem);
                }
            }
            ary.dynamic = None;
        }
        (Some(buf), None) => {
            unsafe {
                ary.inline.set_len(0);
            }
            for elem in buf {
                unsafe {
                    ary.inline.push_unchecked(*elem);
                }
            }
            for elem in &other.inline {
                unsafe {
                    ary.inline.push_unchecked(*elem);
                }
            }
            ary.dynamic = None;
        }
        (None, Some(otherbuf)) => {
            for elem in otherbuf {
                unsafe {
                    ary.inline.push_unchecked(*elem);
                }
            }
        }
        (None, None) => {
            for elem in &other.inline {
                unsafe {
                    ary.inline.push_unchecked(*elem);
                }
            }
        }
    };
}

fn concat_to_dynamic(ary: &mut InlineBuffer, other: &InlineBuffer) {
    match (ary.dynamic.as_mut(), other.dynamic.as_ref()) {
        (Some(buf), Some(otherbuf)) => {
            buf.extend(otherbuf.clone());
        }
        (Some(buf), None) => {
            for elem in &other.inline {
                buf.push(*elem);
            }
        }
        (None, Some(otherbuf)) => {
            let mut buf = ary.inline.as_slice().to_vec();
            buf.reserve(otherbuf.len());
            for elem in otherbuf {
                buf.push(*elem);
            }
            ary.dynamic = Some(buf);
        }
        (None, None) => {
            let mut buf = Vec::with_capacity(ary.len() + other.len());
            for elem in &ary.inline {
                buf.push(*elem);
            }
            for elem in &other.inline {
                buf.push(*elem);
            }
            ary.dynamic = Some(buf);
        }
    };
}
