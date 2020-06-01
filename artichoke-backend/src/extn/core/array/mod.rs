use std::convert::TryFrom;
use std::iter::FromIterator;

use crate::extn::prelude::*;

pub mod args;
mod ffi;
mod inline_buffer;
pub mod mruby;
pub mod trampoline;

use inline_buffer::InlineBuffer;

#[derive(Default, Debug, Clone)]
pub struct Array(InlineBuffer);

impl From<InlineBuffer> for Array {
    fn from(buffer: InlineBuffer) -> Self {
        Self(buffer)
    }
}

impl From<Vec<sys::mrb_value>> for Array {
    fn from(values: Vec<sys::mrb_value>) -> Self {
        Self(InlineBuffer::from(values))
    }
}

impl From<Vec<Value>> for Array {
    fn from(values: Vec<Value>) -> Self {
        Self(InlineBuffer::from(values))
    }
}

impl<'a> From<&'a [sys::mrb_value]> for Array {
    fn from(values: &'a [sys::mrb_value]) -> Self {
        Self(InlineBuffer::from(values))
    }
}

impl<'a> From<&'a [Value]> for Array {
    fn from(values: &'a [Value]) -> Self {
        Self(InlineBuffer::from(values))
    }
}

impl FromIterator<sys::mrb_value> for Array {
    fn from_iter<I>(iter: I) -> Self
    where
        I: IntoIterator<Item = sys::mrb_value>,
    {
        Self(InlineBuffer::from_iter(iter.into_iter()))
    }
}

impl FromIterator<Value> for Array {
    fn from_iter<I>(iter: I) -> Self
    where
        I: IntoIterator<Item = Value>,
    {
        Self(InlineBuffer::from_iter(iter.into_iter()))
    }
}

impl FromIterator<Option<Value>> for Array {
    fn from_iter<I>(iter: I) -> Self
    where
        I: IntoIterator<Item = Option<Value>>,
    {
        Self(InlineBuffer::from_iter(iter.into_iter()))
    }
}

impl<'a> FromIterator<&'a Option<Value>> for Array {
    fn from_iter<I>(iter: I) -> Self
    where
        I: IntoIterator<Item = &'a Option<Value>>,
    {
        Self(InlineBuffer::from_iter(iter.into_iter()))
    }
}

#[derive(Debug)]
pub struct Iter<'a>(inline_buffer::Iter<'a>);

impl<'a> Iterator for Iter<'a> {
    type Item = Value;

    fn next(&mut self) -> Option<Self::Item> {
        self.0.next()
    }
}

impl<'a> IntoIterator for &'a Array {
    type Item = Value;
    type IntoIter = Iter<'a>;

    fn into_iter(self) -> Self::IntoIter {
        Iter(self.0.iter())
    }
}

impl Array {
    #[must_use]
    pub fn new() -> Self {
        Self(InlineBuffer::new())
    }

    #[must_use]
    pub fn with_capacity(capacity: usize) -> Self {
        Self(InlineBuffer::with_capacity(capacity))
    }

    #[must_use]
    pub fn assoc(one: Value, two: Value) -> Self {
        let mut buffer = Self::with_capacity(2);
        buffer.push(one);
        buffer.push(two);
        buffer
    }

    #[must_use]
    pub fn iter(&self) -> Iter<'_> {
        self.into_iter()
    }

    #[must_use]
    pub fn len(&self) -> usize {
        self.0.len()
    }

    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    pub fn clear(&mut self) {
        self.0.clear();
    }

    pub fn gc_mark(&self, interp: &mut Artichoke) {
        for elem in &self.0 {
            interp.mark_value(&elem);
        }
    }

    fn real_children(&self) -> usize {
        self.0.len()
    }

    pub fn initialize(
        interp: &mut Artichoke,
        first: Option<Value>,
        second: Option<Value>,
        block: Option<Block>,
    ) -> Result<Self, Exception> {
        let result = if let Some(first) = first {
            if let Ok(ary) = unsafe { Self::try_from_ruby(interp, &first) } {
                ary.borrow().0.clone()
            } else if first.respond_to(interp, "to_ary")? {
                let other = first.funcall(interp, "to_ary", &[], None)?;
                if let Ok(other) = unsafe { Self::try_from_ruby(interp, &other) } {
                    other.borrow().0.clone()
                } else {
                    let mut message = String::from("can't convert ");
                    message.push_str(first.pretty_name(interp));
                    message.push_str(" to Array (");
                    message.push_str(first.pretty_name(interp));
                    message.push_str("#to_ary gives ");
                    message.push_str(other.pretty_name(interp));
                    return Err(Exception::from(TypeError::new(interp, message)));
                }
            } else {
                let len = first.implicitly_convert_to_int(interp)?;
                let len = usize::try_from(len)
                    .map_err(|_| ArgumentError::new(interp, "negative array size"))?;
                if let Some(block) = block {
                    if second.is_some() {
                        interp.warn(&b"warning: block supersedes default value argument"[..])?;
                    }
                    let mut buffer = Vec::with_capacity(len);
                    for idx in 0..len {
                        let idx = Int::try_from(idx).map_err(|_| {
                            RangeError::new(interp, "bignum too big to convert into `long'")
                        })?;
                        let idx = interp.convert(idx);
                        let elem = block.yield_arg(interp, &idx)?;
                        buffer.push(elem.inner());
                    }
                    InlineBuffer::from(buffer)
                } else {
                    let default = second.unwrap_or_else(Value::nil);
                    let buffer = vec![default; len];
                    InlineBuffer::from(buffer)
                }
            }
        } else if second.is_some() {
            return Err(Exception::from(Fatal::new(
                interp,
                "default cannot be set if first arg is missing in Array#initialize",
            )));
        } else {
            InlineBuffer::default()
        };
        Ok(Self::from(result))
    }

    fn element_reference(
        &self,
        interp: &mut Artichoke,
        index: Value,
        len: Option<Value>,
    ) -> Result<Option<Value>, Exception> {
        let (index, len) = match args::element_reference(interp, index, len, self.0.len())? {
            args::ElementReference::Empty => return Ok(None),
            args::ElementReference::Index(index) => (index, None),
            args::ElementReference::StartLen(index, len) => (index, Some(len)),
        };
        let start = if let Ok(start) = usize::try_from(index) {
            start
        } else {
            let idx = index
                .checked_neg()
                .and_then(|index| usize::try_from(index).ok())
                .and_then(|index| self.0.len().checked_sub(index));
            if let Some(start) = idx {
                start
            } else {
                return Ok(None);
            }
        };
        if start > self.0.len() {
            return Ok(None);
        }
        if let Some(len) = len {
            let result = self.0.slice(start, len);
            let result = Self(result);
            let result = result.try_into_ruby(interp, None)?;
            Ok(Some(result))
        } else {
            Ok(self.0.get(start))
        }
    }

    fn element_assignment(
        &mut self,
        interp: &mut Artichoke,
        first: Value,
        second: Value,
        third: Option<Value>,
    ) -> Result<Value, Exception> {
        let (start, drain, elem) =
            args::element_assignment(interp, first, second, third, self.0.len())?;

        if let Some(drain) = drain {
            if let Ok(other) = unsafe { Self::try_from_ruby(interp, &elem) } {
                self.0.set_slice(start, drain, &other.borrow().0.as_slice());
            } else if elem.respond_to(interp, "to_ary")? {
                let other = elem.funcall(interp, "to_ary", &[], None)?;
                if let Ok(other) = unsafe { Self::try_from_ruby(interp, &other) } {
                    self.0.set_slice(start, drain, &other.borrow().0.as_slice());
                } else {
                    let mut message = String::from("can't convert ");
                    message.push_str(elem.pretty_name(interp));
                    message.push_str(" to Array (");
                    message.push_str(elem.pretty_name(interp));
                    message.push_str("#to_ary gives ");
                    message.push_str(other.pretty_name(interp));
                    return Err(Exception::from(TypeError::new(interp, message)));
                }
            } else {
                self.0.set_with_drain(start, drain, elem);
            }
        } else {
            self.0.set(start, elem);
        }

        Ok(elem)
    }

    #[must_use]
    pub fn get(&self, index: usize) -> Option<Value> {
        self.0.get(index)
    }

    pub fn set(&mut self, index: usize, elem: Value) {
        self.0.set(index, elem);
    }

    fn set_with_drain(&mut self, start: usize, drain: usize, with: Value) -> usize {
        self.0.set_with_drain(start, drain, with)
    }

    fn set_slice(&mut self, start: usize, drain: usize, with: &[sys::mrb_value]) -> usize {
        self.0.set_slice(start, drain, with)
    }

    pub fn concat(&mut self, interp: &mut Artichoke, other: Value) -> Result<(), Exception> {
        if let Ok(other) = unsafe { Self::try_from_ruby(interp, &other) } {
            self.0.concat(&other.borrow().0);
        } else if other.respond_to(interp, "to_ary")? {
            let arr = other.funcall(interp, "to_ary", &[], None)?;
            if let Ok(other) = unsafe { Self::try_from_ruby(interp, &arr) } {
                self.0.concat(&other.borrow().0);
            } else {
                let mut message = String::from("can't convert ");
                message.push_str(other.pretty_name(interp));
                message.push_str(" to Array (");
                message.push_str(other.pretty_name(interp));
                message.push_str("#to_ary gives ");
                message.push_str(other.pretty_name(interp));
                return Err(Exception::from(TypeError::new(interp, message)));
            }
        } else {
            let mut message = String::from("no implicit conversion of ");
            message.push_str(other.pretty_name(interp));
            message.push_str(" into Array");
            return Err(Exception::from(TypeError::new(interp, message)));
        };
        Ok(())
    }

    pub fn pop(&mut self) -> Option<Value> {
        self.0.pop()
    }

    pub fn push(&mut self, elem: Value) {
        self.0.push(elem)
    }

    pub fn reverse(&mut self) {
        self.0.reverse();
    }
}

impl RustBackedValue for Array {
    fn ruby_type_name() -> &'static str {
        "Array"
    }
}
