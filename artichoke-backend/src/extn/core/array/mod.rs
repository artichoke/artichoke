use std::convert::TryFrom;
use std::ffi::c_void;
use std::fmt::Write;
use std::ops::Deref;
use std::slice;

use spinoso_array::Array as SpinosoArray;

use crate::convert::{implicitly_convert_to_int, implicitly_convert_to_string, UnboxedValueGuard};
use crate::extn::prelude::*;
use crate::io::IoError;

pub mod args;
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

impl Extend<sys::mrb_value> for Array {
    fn extend<T>(&mut self, iter: T)
    where
        T: IntoIterator<Item = sys::mrb_value>,
    {
        self.0.extend(iter.into_iter());
    }
}

impl Extend<Value> for Array {
    fn extend<T>(&mut self, iter: T)
    where
        T: IntoIterator<Item = Value>,
    {
        self.0.extend(iter.into_iter().map(|value| value.inner()));
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
    pub fn capacity(&self) -> usize {
        self.0.capacity()
    }

    #[must_use]
    pub fn as_mut_ptr(&mut self) -> *mut sys::mrb_value {
        self.0.as_mut_ptr()
    }

    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    pub fn clear(&mut self) {
        self.0.clear();
    }

    pub fn initialize(
        interp: &mut Artichoke,
        first: Option<Value>,
        second: Option<Value>,
        block: Option<Block>,
    ) -> Result<Self, Error> {
        let vector = match (first, second, block) {
            (Some(mut array_or_len), default, None) => {
                if let Ok(len) = array_or_len.try_convert_into::<i64>(interp) {
                    let len = usize::try_from(len).map_err(|_| ArgumentError::with_message("negative array size"))?;
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
                            let name = interp.inspect_type_name_for_value(array_or_len);
                            message.push_str(name);
                            message.push_str(" to Array (");
                            message.push_str(name);
                            message.push_str("#to_ary gives ");
                            message.push_str(interp.inspect_type_name_for_value(other));
                            return Err(TypeError::from(message).into());
                        }
                    } else {
                        let len = implicitly_convert_to_int(interp, array_or_len)?;
                        let len =
                            usize::try_from(len).map_err(|_| ArgumentError::with_message("negative array size"))?;
                        let default = default.unwrap_or_else(Value::nil);
                        SpinosoArray::with_len_and_default(len, default.inner())
                    }
                }
            }
            (Some(mut array_or_len), default, Some(block)) => {
                if let Ok(len) = array_or_len.try_convert_into::<i64>(interp) {
                    let len = usize::try_from(len).map_err(|_| ArgumentError::with_message("negative array size"))?;
                    if default.is_some() {
                        interp.warn(b"warning: block supersedes default value argument")?;
                    }
                    let mut buffer = SpinosoArray::with_capacity(len);
                    for idx in 0..len {
                        let idx = i64::try_from(idx)
                            .map_err(|_| RangeError::with_message("bignum too big to convert into `long'"))?;
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
                            let name = interp.inspect_type_name_for_value(array_or_len);
                            message.push_str(name);
                            message.push_str(" to Array (");
                            message.push_str(name);
                            message.push_str("#to_ary gives ");
                            message.push_str(interp.inspect_type_name_for_value(other));
                            return Err(TypeError::from(message).into());
                        }
                    } else {
                        let len = implicitly_convert_to_int(interp, array_or_len)?;
                        let len =
                            usize::try_from(len).map_err(|_| ArgumentError::with_message("negative array size"))?;
                        if default.is_some() {
                            interp.warn(b"warning: block supersedes default value argument")?;
                        }
                        let mut buffer = SpinosoArray::with_capacity(len);
                        for idx in 0..len {
                            let idx = i64::try_from(idx)
                                .map_err(|_| RangeError::with_message("bignum too big to convert into `long'"))?;
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

    pub fn repeat(&self, n: usize) -> Result<Self, ArgumentError> {
        if let Some(repeated) = self.0.repeat(n) {
            Ok(Array::from(repeated))
        } else {
            Err(ArgumentError::with_message("argument too big"))
        }
    }

    pub fn join(&self, interp: &mut Artichoke, sep: &[u8]) -> Result<Vec<u8>, Error> {
        fn flatten(interp: &mut Artichoke, mut value: Value, out: &mut Vec<Vec<u8>>) -> Result<(), Error> {
            match value.ruby_type() {
                Ruby::Array => {
                    let ary = unsafe { Array::unbox_from_value(&mut value, interp)? };
                    out.reserve(ary.len());
                    for elem in ary.iter() {
                        flatten(interp, elem, out)?;
                    }
                }
                Ruby::Fixnum => {
                    let mut buf = Vec::new();
                    let int = unsafe { sys::mrb_sys_fixnum_to_cint(value.inner()) };
                    itoa::write(&mut buf, int).map_err(IoError::from)?;
                    out.push(buf);
                }
                Ruby::Float => {
                    let float = unsafe { sys::mrb_sys_float_to_cdouble(value.inner()) };
                    let mut buf = String::new();
                    write!(&mut buf, "{}", float).map_err(WriteError::from)?;
                    out.push(buf.into_bytes());
                }
                _ => {
                    // Safety:
                    //
                    // `s` is converted to an owned byte vec immediately before
                    // any intervening operaitons on the VM which may cause a
                    // garbage collection of the `RString` that backs `value`.
                    if let Ok(s) = unsafe { implicitly_convert_to_string(interp, &mut value) } {
                        out.push(s.to_vec());
                    } else {
                        out.push(value.to_s(interp));
                    }
                }
            }
            Ok(())
        }

        let mut vec = Vec::with_capacity(self.len());
        for elem in self {
            flatten(interp, elem, &mut vec)?;
        }

        Ok(bstr::join(sep, vec))
    }

    fn element_reference(
        &self,
        interp: &mut Artichoke,
        index: Value,
        len: Option<Value>,
    ) -> Result<Option<Value>, Error> {
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
    ) -> Result<Value, Error> {
        let (start, drain, mut elem) = args::element_assignment(interp, first, second, third, self.0.len())?;

        if let Some(drain) = drain {
            if let Ok(other) = unsafe { Self::unbox_from_value(&mut elem, interp) } {
                self.0.set_slice(start, drain, other.0.as_slice());
            } else if elem.respond_to(interp, "to_ary")? {
                let mut other = elem.funcall(interp, "to_ary", &[], None)?;
                if let Ok(other) = unsafe { Self::unbox_from_value(&mut other, interp) } {
                    self.0.set_slice(start, drain, other.0.as_slice());
                } else {
                    let mut message = String::from("can't convert ");
                    let name = interp.inspect_type_name_for_value(elem);
                    message.push_str(name);
                    message.push_str(" to Array (");
                    message.push_str(name);
                    message.push_str("#to_ary gives ");
                    message.push_str(interp.inspect_type_name_for_value(other));
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

    pub fn pop(&mut self) -> Option<Value> {
        self.0.pop().map(Value::from)
    }

    pub fn push(&mut self, elem: Value) {
        self.0.push(elem.inner());
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

    /// Creates an `Array<T>` directly from the raw components of another array.
    ///
    /// # Safety
    ///
    /// This is highly unsafe, due to the number of invariants that aren't
    /// checked:
    ///
    /// - `ptr` needs to have been previously allocated via `Array<T>` (at
    ///   least, it's highly likely to be incorrect if it wasn't).
    /// - `T` needs to have the same size and alignment as what `ptr` was
    ///   allocated with. (`T` having a less strict alignment is not sufficient,
    ///   the alignment really needs to be equal to satisfy the `dealloc`
    ///   requirement that memory must be allocated and deallocated with the
    ///   same layout.)
    /// - `length` needs to be less than or equal to `capacity`.
    /// - `capacity` needs to be the `capacity` that the pointer was allocated
    ///   with.
    ///
    /// Violating these may cause problems like corrupting the allocator's
    /// internal data structures.
    ///
    /// The ownership of `ptr` is effectively transferred to the `Array<T>`
    /// which may then deallocate, reallocate or change the contents of memory
    /// pointed to by the pointer at will. Ensure that nothing else uses the
    /// pointer after calling this function.
    #[must_use]
    pub unsafe fn from_raw_parts(ptr: *mut sys::mrb_value, length: usize, capacity: usize) -> Self {
        Self(SpinosoArray::from_raw_parts(ptr, length, capacity))
    }

    /// Decomposes an `Array<T>` into its raw components.
    ///
    /// Returns the raw pointer to the underlying data, the length of the array
    /// (in elements), and the allocated capacity of the data (in elements).
    /// These are the same arguments in the same order as the arguments to
    /// [`from_raw_parts`].
    ///
    /// After calling this function, the caller is responsible for the memory
    /// previously managed by the `Array`. The only way to do this is to convert
    /// the raw pointer, length, and capacity back into a `Array` with the
    /// [`from_raw_parts`] function, allowing the destructor to perform the
    /// cleanup.
    ///
    /// [`from_raw_parts`]: Array::from_raw_parts
    #[must_use]
    pub fn into_raw_parts(self) -> (*mut sys::mrb_value, usize, usize) {
        self.0.into_raw_parts()
    }
}

impl BoxUnboxVmValue for Array {
    type Unboxed = Self;
    type Guarded = Array;

    const RUBY_TYPE: &'static str = "Array";

    #[allow(clippy::cast_possible_truncation)]
    #[allow(clippy::cast_sign_loss)]
    unsafe fn unbox_from_value<'a>(
        value: &'a mut Value,
        interp: &mut Artichoke,
    ) -> Result<UnboxedValueGuard<'a, Self::Guarded>, Error> {
        let _ = interp;

        // Make sure we have an Array otherwise extraction will fail.
        // This check is critical to the safety of accessing the `value` union.
        if value.ruby_type() != Ruby::Array {
            let mut message = String::from("uninitialized ");
            message.push_str(Self::RUBY_TYPE);
            return Err(TypeError::from(message).into());
        }

        // Safety:
        //
        // The above check on the data type ensures the `value` union holds an
        // `RArray` in the `p` variant.
        let value = value.inner();
        let ary = sys::mrb_sys_basic_ptr(value).cast::<sys::RArray>();

        let ptr = (*ary).as_.heap.ptr;
        let len = (*ary).as_.heap.len as usize;
        let capacity = (*ary).as_.heap.aux.capa as usize;
        let array = Array::from_raw_parts(ptr, len, capacity);

        Ok(UnboxedValueGuard::new(array))
    }

    fn alloc_value(value: Self::Unboxed, interp: &mut Artichoke) -> Result<Value, Error> {
        let (ptr, len, capacity) = Array::into_raw_parts(value);
        let value = unsafe {
            interp.with_ffi_boundary(|mrb| {
                // Overflow Safety:
                //
                // `Array` is backed by a `Vec` which can at most allocate
                // `isize::MAX` bytes.
                //
                // `mrb_value` is not a ZST, so in practice, `len` and
                // `capacity` will never overflow `mrb_int`, which is an `i64`
                // on 64-bit targets.
                //
                // On 32-bit targets, `usize` is `u32` which will never overflow
                // `i64`. Artichoke unconditionally compiles mruby with `-DMRB_INT64`.
                let len = sys::mrb_int::try_from(len)
                    .expect("Length of an `Array` cannot exceed isize::MAX == i64::MAX == mrb_int::MAX");
                let capa = sys::mrb_int::try_from(capacity)
                    .expect("Capacity of an `Array` cannot exceed isize::MAX == i64::MAX == mrb_int::MAX");
                sys::mrb_sys_alloc_rarray(mrb, ptr, len, capa)
            })?
        };
        Ok(interp.protect(value.into()))
    }

    fn box_into_value(value: Self::Unboxed, into: Value, interp: &mut Artichoke) -> Result<Value, Error> {
        // Make sure we have an Array otherwise boxing will produce undefined
        // behavior.
        //
        // This check is critical to the memory safety of future runs of the
        // garbage collector.
        if into.ruby_type() != Ruby::Array {
            panic!("Tried to box Array into {:?} value", into.ruby_type());
        }

        let (ptr, len, capacity) = Array::into_raw_parts(value);
        unsafe {
            sys::mrb_sys_repack_into_rarray(ptr, len as sys::mrb_int, capacity as sys::mrb_int, into.inner());
        }

        Ok(interp.protect(into))
    }

    fn free(data: *mut c_void) {
        // this function is never called. `Array` is freed directly in the VM by
        // calling `mrb_ary_artichoke_free`.
        //
        // Array should not have a destructor registered in the class registry.
        let _ = data;
    }
}

impl<'a> AsRef<Array> for UnboxedValueGuard<'a, Array> {
    fn as_ref(&self) -> &Array {
        self.as_inner_ref()
    }
}

impl<'a> Deref for UnboxedValueGuard<'a, Array> {
    type Target = Array;

    fn deref(&self) -> &Self::Target {
        self.as_inner_ref()
    }
}

#[cfg(test)]
mod tests {
    use crate::test::prelude::*;
    use bstr::ByteSlice;

    const SUBJECT: &str = "Array";
    const FUNCTIONAL_TEST: &[u8] = include_bytes!("array_functional_test.rb");

    #[test]
    fn functional() {
        let mut interp = interpreter().unwrap();
        interp.eval(FUNCTIONAL_TEST).unwrap();
        let result = interp.eval(b"spec");
        if let Err(exc) = result {
            let backtrace = exc.vm_backtrace(&mut interp);
            let backtrace = bstr::join("\n", backtrace.unwrap_or_default());
            panic!(
                "{} tests failed with message: {:?} and backtrace:\n{:?}",
                SUBJECT,
                exc.message().as_bstr(),
                backtrace.as_bstr()
            );
        }
    }
}
