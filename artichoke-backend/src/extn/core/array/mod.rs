use artichoke_core::value::Value as _;
use artichoke_core::warn::Warn;
use std::convert::TryFrom;

use crate::convert::{Convert, RustBackedValue};
use crate::extn::core::exception::{
    ArgumentError, Fatal, RangeError, RubyException, RuntimeError, TypeError,
};
use crate::types::Int;
use crate::value::{Block, Value};
use crate::Artichoke;

pub mod args;
pub mod backend;
mod ffi;
mod inline_buffer;
pub mod mruby;
pub mod trampoline;

pub use backend::ArrayType;
pub use inline_buffer::InlineBuffer;

pub struct Array(InlineBuffer);

impl Clone for Array {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}

impl Array {
    pub fn new(ary: InlineBuffer) -> Self {
        Self(ary)
    }

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
        interp: &Artichoke,
        first: Option<Value>,
        second: Option<Value>,
        block: Option<Block>,
        into: Value,
    ) -> Result<Value, Box<dyn RubyException>> {
        let result = if let Some(first) = first {
            if let Ok(ary) = unsafe { Self::try_from_ruby(interp, &first) } {
                ary.borrow().0.clone()
            } else if let Ok(len) = first.clone().try_into::<Int>() {
                let len = usize::try_from(len)
                    .map_err(|_| ArgumentError::new(interp, "negative array size"))?;
                if let Some(block) = block {
                    if second.is_some() {
                        interp
                            .warn(&b"warning: block supersedes default value argument"[..])
                            .map_err(|_| Fatal::new(interp, "Could not emit warning"))?;
                    }
                    let mut buffer = Vec::with_capacity(len);
                    for idx in 0..len {
                        let idx = Int::try_from(idx).map_err(|_| {
                            RangeError::new(interp, "bignum too big to convert into `long'")
                        })?;
                        let idx = interp.convert(idx);
                        // TODO: propagate exceptions from block call.
                        let elem = block.yield_arg(interp, &idx).map_err(|_| {
                            RuntimeError::new(interp, "exception during Array#initialize block")
                        })?;
                        buffer.push(elem);
                    }
                    InlineBuffer::from(buffer)
                } else if let Some(_default) = second {
                    // backend::repeated::value(default, len)
                    panic!();
                } else {
                    // backend::fixed::hole(len)
                    panic!();
                }
            } else if let Ok(true) = first.respond_to("to_ary") {
                let ruby_type = first.pretty_name();
                if let Ok(other) = first.funcall("to_ary", &[], None) {
                    if let Ok(other) = unsafe { Self::try_from_ruby(interp, &other) } {
                        other.borrow().0.clone()
                    } else {
                        return Err(Box::new(TypeError::new(
                            interp,
                            format!(
                            "can't convert {classname} to Array ({classname}#to_ary gives {gives})",
                            classname = ruby_type,
                            gives = other.pretty_name()
                        ),
                        )));
                    }
                } else {
                    // TODO: propagate exceptions thrown by `value#to_a`.
                    return Err(Box::new(Fatal::new(
                        interp,
                        "Error calling #to_a even though it exists",
                    )));
                }
            } else if let Ok(len) = first.funcall::<Int>("to_int", &[], None) {
                let len = usize::try_from(len)
                    .map_err(|_| ArgumentError::new(interp, "negative array size"))?;
                if let Some(block) = block {
                    if second.is_some() {
                        interp
                            .warn(&b"warning: block supersedes default value argument"[..])
                            .map_err(|_| Fatal::new(interp, "Could not emit warning"))?;
                    }
                    let mut buffer = Vec::with_capacity(len);
                    for idx in 0..len {
                        let idx = Int::try_from(idx).map_err(|_| {
                            RangeError::new(interp, "bignum too big to convert into `long'")
                        })?;
                        let idx = interp.convert(idx);
                        // TODO: propagate exceptions from block call.
                        let elem = block.yield_arg(interp, &idx).map_err(|_| {
                            RuntimeError::new(interp, "exception during Array#initialize block")
                        })?;
                        buffer.push(elem);
                    }
                    InlineBuffer::from(buffer)
                } else if let Some(_default) = second {
                    // backend::repeated::value(default, len)
                    panic!();
                } else {
                    // backend::fixed::hole(len)
                    panic!();
                }
            } else {
                return Err(Box::new(TypeError::new(
                    interp,
                    format!(
                        "no implicit conversion of {} into Integer",
                        first.pretty_name()
                    ),
                )));
            }
        } else if second.is_some() {
            return Err(Box::new(Fatal::new(
                interp,
                "default cannot be set if first arg is missing in Array#initialize",
            )));
        } else {
            InlineBuffer::default()
        };
        let result = Self(result);
        let result = unsafe { result.try_into_ruby(interp, Some(into.inner())) }
            .map_err(|_| Fatal::new(interp, "Unable to initialize Ruby Array from Rust Array"))?;
        Ok(result)
    }

    fn element_reference(
        &self,
        interp: &Artichoke,
        index: Value,
        len: Option<Value>,
    ) -> Result<Value, Box<dyn RubyException>> {
        let (index, len) = match args::element_reference(interp, index, len, self.0.len())? {
            args::ElementReference::Empty => return Ok(interp.convert(None::<Value>)),
            args::ElementReference::Index(index) => (index, None),
            args::ElementReference::StartLen(index, len) => (index, Some(len)),
        };
        let start = if index < 0 {
            // Positive Int must be usize
            let idx = usize::try_from(-index)
                .map_err(|_| Fatal::new(interp, "Expected positive index to convert to usize"))?;
            if let Some(index) = self.0.len().checked_sub(idx) {
                index
            } else {
                return Ok(interp.convert(None::<Value>));
            }
        } else {
            usize::try_from(index)
                .map_err(|_| Fatal::new(interp, "Expected positive index to convert to usize"))?
        };
        if let Some(len) = len {
            let result = self.0.slice(interp, start, len)?;
            let result = Self(result);
            let result = unsafe { result.try_into_ruby(interp, None) }.map_err(|_| {
                Fatal::new(interp, "Unable to initialize Ruby Array from Rust Array")
            })?;
            Ok(result)
        } else {
            self.0.get(interp, start)
        }
    }

    fn element_assignment(
        &mut self,
        interp: &Artichoke,
        first: Value,
        second: Value,
        third: Option<Value>,
    ) -> Result<Value, Box<dyn RubyException>> {
        let (start, drain, elem) =
            args::element_assignment(interp, first, second, third, self.0.len())?;
        if let Some(drain) = drain {
            if let Ok(other) = unsafe { Self::try_from_ruby(interp, &elem) } {
                self.0.set_slice(interp, start, drain, &other.borrow().0)?;
            } else if let Ok(true) = elem.respond_to("to_ary") {
                let ruby_type = elem.pretty_name();
                if let Ok(other) = elem.funcall("to_ary", &[], None) {
                    if let Ok(other) = unsafe { Self::try_from_ruby(interp, &other) } {
                        self.0.set_slice(interp, start, drain, &other.borrow().0)?;
                    } else {
                        return Err(Box::new(TypeError::new(
                            interp,
                            format!(
                            "can't convert {classname} to Array ({classname}#to_ary gives {gives})",
                            classname = ruby_type,
                            gives = other.pretty_name()
                        ),
                        )));
                    }
                } else {
                    // TODO: propagate exceptions thrown by `value#to_a`.
                    return Err(Box::new(Fatal::new(
                        interp,
                        "Error calling #to_a even though it exists",
                    )));
                }
            } else {
                self.0.set_with_drain(interp, start, drain, elem.clone())?;
            }
        } else {
            self.0.set(interp, start, elem.clone())?;
        }
        Ok(elem)
    }

    pub fn get(&self, interp: &Artichoke, index: usize) -> Result<Value, Box<dyn RubyException>> {
        self.0.get(interp, index)
    }

    pub fn slice(
        &self,
        interp: &Artichoke,
        start: usize,
        len: usize,
    ) -> Result<InlineBuffer, Box<dyn RubyException>> {
        self.0.slice(interp, start, len)
    }

    pub fn set(
        &mut self,
        interp: &Artichoke,
        index: usize,
        elem: Value,
    ) -> Result<(), Box<dyn RubyException>> {
        self.0.set(interp, index, elem)?;
        Ok(())
    }

    fn set_with_drain(
        &mut self,
        interp: &Artichoke,
        start: usize,
        drain: usize,
        with: Value,
    ) -> Result<(), Box<dyn RubyException>> {
        self.0.set_with_drain(interp, start, drain, with)?;
        Ok(())
    }

    fn set_slice(
        &mut self,
        interp: &Artichoke,
        start: usize,
        drain: usize,
        with: &InlineBuffer,
    ) -> Result<(), Box<dyn RubyException>> {
        self.0.set_slice(interp, start, drain, with)?;
        Ok(())
    }

    pub fn concat(
        &mut self,
        interp: &Artichoke,
        other: Value,
    ) -> Result<(), Box<dyn RubyException>> {
        if let Ok(other) = unsafe { Self::try_from_ruby(interp, &other) } {
            self.0.concat(interp, &other.borrow().0)?;
        } else if let Ok(true) = other.respond_to("to_ary") {
            let ruby_type = other.pretty_name();
            if let Ok(other) = other.funcall("to_ary", &[], None) {
                if let Ok(other) = unsafe { Self::try_from_ruby(interp, &other) } {
                    self.0.concat(interp, &other.borrow().0)?;
                } else {
                    return Err(Box::new(TypeError::new(
                        interp,
                        format!(
                            "can't convert {classname} to Array ({classname}#to_ary gives {gives})",
                            classname = ruby_type,
                            gives = other.pretty_name()
                        ),
                    )));
                }
            } else {
                // TODO: propagate exceptions thrown by `value#to_a`.
                return Err(Box::new(Fatal::new(
                    interp,
                    "Error calling #to_a even though it exists",
                )));
            }
        } else {
            return Err(Box::new(TypeError::new(
                interp,
                format!(
                    "no implicit conversion of {classname} into Array",
                    classname = other.pretty_name(),
                ),
            )));
        };
        Ok(())
    }

    pub fn len(&self, interp: &Artichoke) -> Result<Value, Box<dyn RubyException>> {
        let len = Int::try_from(self.0.len())
            .map_err(|_| Fatal::new(interp, "Array length does not fit in Integer max"))?;
        Ok(interp.convert(len))
    }

    pub fn len_usize(&self) -> usize {
        self.0.len()
    }

    pub fn pop(&mut self, interp: &Artichoke) -> Result<Value, Box<dyn RubyException>> {
        let popped = self.0.pop(interp)?;
        Ok(popped)
    }

    pub fn reverse(&mut self, interp: &Artichoke) -> Result<(), Box<dyn RubyException>> {
        self.0.reverse(interp)?;
        Ok(())
    }
}

impl RustBackedValue for Array {
    fn ruby_type_name() -> &'static str {
        "Array"
    }
}
