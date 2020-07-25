use spinoso_array::SmallArray as SpinosoArray;
use std::convert::TryFrom;
use std::iter::FromIterator;
use std::slice;

use crate::extn::prelude::*;

pub mod args;
mod boxing;
mod ffi;
pub mod mruby;
pub mod trampoline;

/// Contiguous growable vector that implement the [Ruby `Array`] API for
/// [`sys::mrb_value`] elements.
///
/// `Array` is a growable vector with potentially heap-allocated contents.
/// `Array` implements the small vector optimization using
/// [`spinoso_array::SmallArray`]. This type can be passed by pointer over FFI
/// to the underlying mruby VM (this is why it stores `sys::mrb_value` rather
/// than [`Value`].
///
/// `Array` implements [`BoxUnboxVmValue`] which enables it to be serialized to
/// a mruby value and unboxed to the Rust `Array` type.
#[derive(Default, Debug, Clone)]
pub struct Array(SpinosoArray<sys::mrb_value>);

impl From<SpinosoArray<sys::mrb_value>> for Array {
    fn from(buffer: SpinosoArray<sys::mrb_value>) -> Self {
        Self(buffer)
    }
}

impl From<Vec<sys::mrb_value>> for Array {
    fn from(values: Vec<sys::mrb_value>) -> Self {
        Self(values.into())
    }
}

impl From<Vec<Value>> for Array {
    fn from(values: Vec<Value>) -> Self {
        Self(values.iter().map(Value::inner).collect())
    }
}

impl<'a> From<&'a [sys::mrb_value]> for Array {
    fn from(values: &'a [sys::mrb_value]) -> Self {
        Self(values.into())
    }
}

impl<'a> From<&'a [Value]> for Array {
    fn from(values: &'a [Value]) -> Self {
        Self(values.iter().map(Value::inner).collect())
    }
}

impl FromIterator<sys::mrb_value> for Array {
    fn from_iter<I>(iter: I) -> Self
    where
        I: IntoIterator<Item = sys::mrb_value>,
    {
        Self(iter.into_iter().collect())
    }
}

impl FromIterator<Value> for Array {
    fn from_iter<I>(iter: I) -> Self
    where
        I: IntoIterator<Item = Value>,
    {
        Self(iter.into_iter().map(|value| value.inner()).collect())
    }
}

impl FromIterator<Option<Value>> for Array {
    fn from_iter<I>(iter: I) -> Self
    where
        I: IntoIterator<Item = Option<Value>>,
    {
        let array = iter
            .into_iter()
            .map(|value| value.unwrap_or_default().inner())
            .collect();
        Self(array)
    }
}

impl<'a> FromIterator<&'a Option<Value>> for Array {
    fn from_iter<I>(iter: I) -> Self
    where
        I: IntoIterator<Item = &'a Option<Value>>,
    {
        let array = iter
            .into_iter()
            .map(|value| value.unwrap_or_default().inner())
            .collect();
        Self(array)
    }
}

#[derive(Debug)]
pub struct Iter<'a>(slice::Iter<'a, sys::mrb_value>);

impl<'a> Iterator for Iter<'a> {
    type Item = Value;

    fn next(&mut self) -> Option<Self::Item> {
        self.0.next().copied().map(Value::from)
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
        Self(SpinosoArray::new())
    }

    #[must_use]
    pub fn with_capacity(capacity: usize) -> Self {
        Self(SpinosoArray::with_capacity(capacity))
    }

    #[must_use]
    pub fn assoc(one: Value, two: Value) -> Self {
        Self(SpinosoArray::assoc(one.inner(), two.inner()))
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

    /// Mark all elements in the `Array` as reachable to the garbage collector.
    ///
    /// This method ensures that the contents of the conained
    /// [`sys::mrb_value`]s do not get deallocated while this `Array` is alive
    /// in the mruby VM.
    pub fn gc_mark(&self, interp: &mut Artichoke) {
        for elem in self {
            interp.mark_value(&elem);
        }
    }

    /// The count of [`sys::mrb_value`]s in this `Array`.
    ///
    /// This method allows for `Array`s with holes or other virtualized
    /// elements. `Array` does not store virtual elements so this method always
    /// returns the array's length.
    #[must_use]
    fn real_children(&self) -> usize {
        self.0.len()
    }

    pub fn initialize(
        interp: &mut Artichoke,
        first: Option<Value>,
        second: Option<Value>,
        block: Option<Block>,
    ) -> Result<Self, Exception> {
        let vector = match (first, second, block) {
            (Some(mut array_or_len), default, None) => {
                if let Ok(len) = array_or_len.try_into::<Int>(interp) {
                    let len = usize::try_from(len)
                        .map_err(|_| ArgumentError::from("negative array size"))?;
                    let default = default.unwrap_or_else(Value::nil);
                    SpinosoArray::with_len_and_default(len, default.inner())
                } else {
                    let unboxed = unsafe { Self::unbox_from_value(&mut array_or_len, interp) };
                    if let Ok(ary) = unboxed {
                        ary.0.clone()
                    } else if array_or_len.respond_to(interp, "to_ary")? {
                        let mut other = array_or_len.funcall(interp, "to_ary", &[], None)?;
                        let unboxed = unsafe { Self::unbox_from_value(&mut other, interp) };
                        if let Ok(other) = unboxed {
                            other.0.clone()
                        } else {
                            let mut message = String::from("can't convert ");
                            message.push_str(array_or_len.pretty_name(interp));
                            message.push_str(" to Array (");
                            message.push_str(array_or_len.pretty_name(interp));
                            message.push_str("#to_ary gives ");
                            message.push_str(other.pretty_name(interp));
                            return Err(TypeError::from(message).into());
                        }
                    } else {
                        let len = array_or_len.implicitly_convert_to_int(interp)?;
                        let len = usize::try_from(len)
                            .map_err(|_| ArgumentError::from("negative array size"))?;
                        let default = default.unwrap_or_else(Value::nil);
                        SpinosoArray::with_len_and_default(len, default.inner())
                    }
                }
            }
            (Some(mut array_or_len), default, Some(block)) => {
                if let Ok(len) = array_or_len.try_into::<Int>(interp) {
                    let len = usize::try_from(len)
                        .map_err(|_| ArgumentError::from("negative array size"))?;
                    if default.is_some() {
                        interp.warn(b"warning: block supersedes default value argument")?;
                    }
                    let mut buffer = SpinosoArray::with_capacity(len);
                    for idx in 0..len {
                        let idx = Int::try_from(idx).map_err(|_| {
                            RangeError::from("bignum too big to convert into `long'")
                        })?;
                        let idx = interp.convert(idx);
                        let elem = block.yield_arg(interp, &idx)?;
                        buffer.push(elem.inner());
                    }
                    buffer
                } else {
                    let unboxed = unsafe { Self::unbox_from_value(&mut array_or_len, interp) };
                    if let Ok(ary) = unboxed {
                        ary.0.clone()
                    } else if array_or_len.respond_to(interp, "to_ary")? {
                        let mut other = array_or_len.funcall(interp, "to_ary", &[], None)?;
                        let unboxed = unsafe { Self::unbox_from_value(&mut other, interp) };
                        if let Ok(other) = unboxed {
                            other.0.clone()
                        } else {
                            let mut message = String::from("can't convert ");
                            message.push_str(array_or_len.pretty_name(interp));
                            message.push_str(" to Array (");
                            message.push_str(array_or_len.pretty_name(interp));
                            message.push_str("#to_ary gives ");
                            message.push_str(other.pretty_name(interp));
                            return Err(TypeError::from(message).into());
                        }
                    } else {
                        let len = array_or_len.implicitly_convert_to_int(interp)?;
                        let len = usize::try_from(len)
                            .map_err(|_| ArgumentError::from("negative array size"))?;
                        if default.is_some() {
                            interp.warn(b"warning: block supersedes default value argument")?;
                        }
                        let mut buffer = SpinosoArray::with_capacity(len);
                        for idx in 0..len {
                            let idx = Int::try_from(idx).map_err(|_| {
                                RangeError::from("bignum too big to convert into `long'")
                            })?;
                            let idx = interp.convert(idx);
                            let elem = block.yield_arg(interp, &idx)?;
                            buffer.push(elem.inner());
                        }
                        buffer
                    }
                }
            }
            (None, None, _) => SpinosoArray::new(),
            (None, Some(_), _) => {
                let err_msg = "default cannot be set if first arg is missing in Array#initialize";
                return Err(Fatal::from(err_msg).into());
            }
        };
        Ok(Self(vector))
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
            let result = Self(result.into());
            let result = Self::alloc_value(result, interp)?;
            Ok(Some(result))
        } else {
            Ok(self.0.get(start).copied().map(Value::from))
        }
    }

    fn element_assignment(
        &mut self,
        interp: &mut Artichoke,
        first: Value,
        second: Value,
        third: Option<Value>,
    ) -> Result<Value, Exception> {
        let (start, drain, mut elem) =
            args::element_assignment(interp, first, second, third, self.0.len())?;

        if let Some(drain) = drain {
            if let Ok(other) = unsafe { Self::unbox_from_value(&mut elem, interp) } {
                self.0.set_slice(start, drain, other.0.as_slice());
            } else if elem.respond_to(interp, "to_ary")? {
                let mut other = elem.funcall(interp, "to_ary", &[], None)?;
                if let Ok(other) = unsafe { Self::unbox_from_value(&mut other, interp) } {
                    self.0.set_slice(start, drain, other.0.as_slice());
                } else {
                    let mut message = String::from("can't convert ");
                    message.push_str(elem.pretty_name(interp));
                    message.push_str(" to Array (");
                    message.push_str(elem.pretty_name(interp));
                    message.push_str("#to_ary gives ");
                    message.push_str(other.pretty_name(interp));
                    return Err(TypeError::from(message).into());
                }
            } else {
                self.0.set_with_drain(start, drain, elem.inner());
            }
        } else {
            self.0.set(start, elem.inner());
        }

        Ok(elem)
    }

    #[must_use]
    pub fn get(&self, index: usize) -> Option<Value> {
        self.0.get(index).copied().map(Value::from)
    }

    pub fn set(&mut self, index: usize, elem: Value) {
        self.0.set(index, elem.inner());
    }

    fn set_with_drain(&mut self, start: usize, drain: usize, elem: Value) -> usize {
        self.0.set_with_drain(start, drain, elem.inner())
    }

    fn set_slice(&mut self, start: usize, drain: usize, src: &[sys::mrb_value]) -> usize {
        self.0.set_slice(start, drain, src)
    }

    pub fn concat(&mut self, interp: &mut Artichoke, mut other: Value) -> Result<(), Exception> {
        if let Ok(other) = unsafe { Self::unbox_from_value(&mut other, interp) } {
            self.0.concat(other.0.as_slice());
        } else if other.respond_to(interp, "to_ary")? {
            let mut arr = other.funcall(interp, "to_ary", &[], None)?;
            if let Ok(other) = unsafe { Self::unbox_from_value(&mut arr, interp) } {
                self.0.concat(other.0.as_slice());
            } else {
                let mut message = String::from("can't convert ");
                message.push_str(other.pretty_name(interp));
                message.push_str(" to Array (");
                message.push_str(other.pretty_name(interp));
                message.push_str("#to_ary gives ");
                message.push_str(other.pretty_name(interp));
                return Err(TypeError::from(message).into());
            }
        } else {
            let mut message = String::from("no implicit conversion of ");
            message.push_str(other.pretty_name(interp));
            message.push_str(" into Array");
            return Err(TypeError::from(message).into());
        };
        Ok(())
    }

    pub fn pop(&mut self) -> Option<Value> {
        self.0.pop().map(Value::from)
    }

    pub fn push(&mut self, elem: Value) {
        self.0.push(elem.inner())
    }

    pub fn reverse(&mut self) {
        self.0.reverse();
    }

    pub fn shift(&mut self) -> Option<Value> {
        self.0.shift().map(Value::from)
    }

    pub fn shift_n(&mut self, count: usize) -> Self {
        Self(self.0.shift_n(count))
    }
}
