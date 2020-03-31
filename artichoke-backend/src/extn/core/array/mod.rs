use std::convert::TryFrom;

use crate::extn::prelude::*;

pub mod args;
pub mod backend;
mod ffi;
mod inline_buffer;
pub mod mruby;
pub mod trampoline;

pub use backend::ArrayType;
pub use inline_buffer::InlineBuffer;

#[derive(Debug, Clone)]
pub struct Array(InlineBuffer);

impl Array {
    #[must_use]
    pub fn new(ary: InlineBuffer) -> Self {
        Self(ary)
    }

    #[must_use]
    pub fn as_vec(&self, interp: &Artichoke) -> Vec<Value> {
        self.0.as_vec(interp)
    }

    fn gc_mark(&self, interp: &Artichoke) {
        self.0.gc_mark(interp)
    }

    fn real_children(&self) -> usize {
        self.0.real_children()
    }

    pub fn clear(&mut self) {
        self.0.clear();
    }

    pub fn initialize(
        interp: &mut Artichoke,
        first: Option<Value>,
        second: Option<Value>,
        block: Option<Block>,
        into: Value,
    ) -> Result<Value, Exception> {
        let result = if let Some(first) = first {
            if let Ok(ary) = unsafe { Self::try_from_ruby(interp, &first) } {
                ary.borrow().0.clone()
            } else if first.respond_to("to_ary")? {
                let other = first.funcall("to_ary", &[], None)?;
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
                        let elem = block.yield_arg::<Value>(interp, &idx)?;
                        buffer.push(elem.inner());
                    }
                    InlineBuffer::from(buffer)
                } else {
                    let default = second.unwrap_or_else(|| interp.convert(None::<Value>));
                    let mut buffer = Vec::with_capacity(len);
                    for _ in 0..len {
                        buffer.push(default.inner());
                    }
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
        let result = Self(result);
        let result = result.try_into_ruby(interp, Some(into.inner()))?;
        Ok(result)
    }

    fn element_reference(
        &self,
        interp: &mut Artichoke,
        index: Value,
        len: Option<Value>,
    ) -> Result<Value, Exception> {
        let (index, len) = match args::element_reference(interp, index, len, self.0.len())? {
            args::ElementReference::Empty => return Ok(interp.convert(None::<Value>)),
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
                return Ok(interp.convert(None::<Value>));
            }
        };
        if start > self.0.len() {
            return Ok(interp.convert(None::<Value>));
        }
        if let Some(len) = len {
            let result = self.0.slice(interp, start, len)?;
            let result = Self(result);
            let result = result.try_into_ruby(interp, None)?;
            Ok(result)
        } else {
            let result = self.0.get(interp, start)?;
            Ok(interp.convert(result))
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
                self.0.set_slice(interp, start, drain, &other.borrow().0)?;
            } else if elem.respond_to("to_ary")? {
                let other = elem.funcall("to_ary", &[], None)?;
                if let Ok(other) = unsafe { Self::try_from_ruby(interp, &other) } {
                    self.0.set_slice(interp, start, drain, &other.borrow().0)?;
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
                self.0.set_with_drain(interp, start, drain, elem.clone())?;
            }
        } else {
            self.0.set(interp, start, elem.clone())?;
        }
        Ok(elem)
    }

    pub fn get(&self, interp: &Artichoke, index: usize) -> Result<Value, Exception> {
        let result = self.0.get(interp, index)?;
        Ok(interp.convert(result))
    }

    pub fn slice(
        &self,
        interp: &Artichoke,
        start: usize,
        len: usize,
    ) -> Result<InlineBuffer, Exception> {
        self.0.slice(interp, start, len)
    }

    pub fn set(&mut self, interp: &Artichoke, index: usize, elem: Value) -> Result<(), Exception> {
        self.0.set(interp, index, elem)?;
        Ok(())
    }

    fn set_with_drain(
        &mut self,
        interp: &Artichoke,
        start: usize,
        drain: usize,
        with: Value,
    ) -> Result<(), Exception> {
        self.0.set_with_drain(interp, start, drain, with)?;
        Ok(())
    }

    fn set_slice(
        &mut self,
        interp: &Artichoke,
        start: usize,
        drain: usize,
        with: &InlineBuffer,
    ) -> Result<(), Exception> {
        self.0.set_slice(interp, start, drain, with)?;
        Ok(())
    }

    pub fn concat(&mut self, interp: &mut Artichoke, other: Value) -> Result<(), Exception> {
        if let Ok(other) = unsafe { Self::try_from_ruby(interp, &other) } {
            self.0.concat(interp, &other.borrow().0)?;
        } else if other.respond_to("to_ary")? {
            let arr = other.funcall("to_ary", &[], None)?;
            if let Ok(other) = unsafe { Self::try_from_ruby(interp, &arr) } {
                self.0.concat(interp, &other.borrow().0)?;
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

    #[must_use]
    pub fn len(&self) -> usize {
        self.0.len()
    }

    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    pub fn pop(&mut self, interp: &Artichoke) -> Result<Value, Exception> {
        let popped = self.0.pop(interp)?;
        Ok(popped)
    }

    pub fn reverse(&mut self, interp: &Artichoke) -> Result<(), Exception> {
        self.0.reverse(interp)?;
        Ok(())
    }
}

impl RustBackedValue for Array {
    fn ruby_type_name() -> &'static str {
        "Array"
    }
}
