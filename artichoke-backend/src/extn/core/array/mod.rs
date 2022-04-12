use std::ffi::c_void;
use std::fmt::Write as _;
use std::ops::Deref;

use crate::convert::{implicitly_convert_to_int, implicitly_convert_to_string, UnboxedValueGuard};
use crate::extn::prelude::*;

pub mod args;
mod ffi;
pub mod mruby;
pub mod trampoline;
mod wrapper;

#[doc(inline)]
pub use wrapper::{Array, RawParts};

pub fn initialize(
    interp: &mut Artichoke,
    first: Option<Value>,
    second: Option<Value>,
    block: Option<Block>,
) -> Result<Array, Error> {
    let ary = match (first, second, block) {
        (Some(mut array_or_len), default, None) => {
            if let Ok(len) = array_or_len.try_convert_into::<i64>(interp) {
                let len = usize::try_from(len).map_err(|_| ArgumentError::with_message("negative array size"))?;
                let default = default.unwrap_or_else(Value::nil);
                Array::with_len_and_default(len, default)
            } else {
                let unboxed = unsafe { Array::unbox_from_value(&mut array_or_len, interp) };
                if let Ok(ary) = unboxed {
                    ary.clone()
                } else if array_or_len.respond_to(interp, "to_ary")? {
                    let mut other = array_or_len.funcall(interp, "to_ary", &[], None)?;
                    let unboxed = unsafe { Array::unbox_from_value(&mut other, interp) };
                    if let Ok(other) = unboxed {
                        other.clone()
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
                    let len = usize::try_from(len).map_err(|_| ArgumentError::with_message("negative array size"))?;
                    let default = default.unwrap_or_else(Value::nil);
                    Array::with_len_and_default(len, default)
                }
            }
        }
        (Some(mut array_or_len), default, Some(block)) => {
            if let Ok(len) = array_or_len.try_convert_into::<i64>(interp) {
                let len = usize::try_from(len).map_err(|_| ArgumentError::with_message("negative array size"))?;
                if default.is_some() {
                    interp.warn(b"warning: block supersedes default value argument")?;
                }
                let mut buffer = Array::with_capacity(len);
                for idx in 0..len {
                    let idx = i64::try_from(idx)
                        .map_err(|_| RangeError::with_message("bignum too big to convert into `long'"))?;
                    let idx = interp.convert(idx);
                    let elem = block.yield_arg(interp, &idx)?;
                    buffer.push(elem);
                }
                buffer
            } else {
                let unboxed = unsafe { Array::unbox_from_value(&mut array_or_len, interp) };
                if let Ok(ary) = unboxed {
                    ary.clone()
                } else if array_or_len.respond_to(interp, "to_ary")? {
                    let mut other = array_or_len.funcall(interp, "to_ary", &[], None)?;
                    let unboxed = unsafe { Array::unbox_from_value(&mut other, interp) };
                    if let Ok(other) = unboxed {
                        other.clone()
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
                    let len = usize::try_from(len).map_err(|_| ArgumentError::with_message("negative array size"))?;
                    if default.is_some() {
                        interp.warn(b"warning: block supersedes default value argument")?;
                    }
                    let mut buffer = Array::with_capacity(len);
                    for idx in 0..len {
                        let idx = i64::try_from(idx)
                            .map_err(|_| RangeError::with_message("bignum too big to convert into `long'"))?;
                        let idx = interp.convert(idx);
                        let elem = block.yield_arg(interp, &idx)?;
                        buffer.push(elem);
                    }
                    buffer
                }
            }
        }
        (None, None, _) => Array::new(),
        (None, Some(_), _) => {
            let err_msg = "default cannot be set if first arg is missing in Array#initialize";
            return Err(Fatal::from(err_msg).into());
        }
    };
    Ok(ary)
}

pub fn repeat(ary: &Array, n: usize) -> Result<Array, ArgumentError> {
    ary.repeat(n)
        .ok_or_else(|| ArgumentError::with_message("argument too big"))
}

pub fn join(interp: &mut Artichoke, ary: &Array, sep: &[u8]) -> Result<Vec<u8>, Error> {
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
                let mut buf = String::new();
                let int = unsafe { sys::mrb_sys_fixnum_to_cint(value.inner()) };
                write!(&mut buf, "{}", int).map_err(WriteError::from)?;
                out.push(buf.into_bytes());
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
                // any intervening operations on the VM which may cause a
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

    let mut vec = Vec::with_capacity(ary.len());
    for elem in ary {
        flatten(interp, elem, &mut vec)?;
    }

    Ok(bstr::join(sep, vec))
}

fn aref(interp: &mut Artichoke, ary: &Array, index: Value, len: Option<Value>) -> Result<Option<Value>, Error> {
    let (index, len) = match args::element_reference(interp, index, len, ary.len())? {
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
            .and_then(|index| ary.len().checked_sub(index));
        if let Some(start) = idx {
            start
        } else {
            return Ok(None);
        }
    };
    if start > ary.len() {
        return Ok(None);
    }
    if let Some(len) = len {
        let result = ary.slice(start, len);
        let result = Array::alloc_value(result.into(), interp)?;
        Ok(Some(result))
    } else {
        Ok(ary.get(start))
    }
}

fn aset(
    interp: &mut Artichoke,
    ary: &mut Array,
    first: Value,
    second: Value,
    third: Option<Value>,
) -> Result<Value, Error> {
    let (start, drain, mut elem) = args::element_assignment(interp, first, second, third, ary.len())?;

    if let Some(drain) = drain {
        if let Ok(other) = unsafe { Array::unbox_from_value(&mut elem, interp) } {
            ary.set_slice(start, drain, other.as_slice());
        } else if elem.respond_to(interp, "to_ary")? {
            let mut other = elem.funcall(interp, "to_ary", &[], None)?;
            if let Ok(other) = unsafe { Array::unbox_from_value(&mut other, interp) } {
                ary.set_slice(start, drain, other.as_slice());
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
            ary.set_with_drain(start, drain, elem);
        }
    } else {
        ary.set(start, elem);
    }

    Ok(elem)
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
        let length = (*ary).as_.heap.len as usize;
        let capacity = (*ary).as_.heap.aux.capa as usize;
        let array = Array::from_raw_parts(RawParts { ptr, length, capacity });

        Ok(UnboxedValueGuard::new(array))
    }

    fn alloc_value(value: Self::Unboxed, interp: &mut Artichoke) -> Result<Value, Error> {
        let RawParts { ptr, length, capacity } = Array::into_raw_parts(value);
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
                let length = sys::mrb_int::try_from(length)
                    .expect("Length of an `Array` cannot exceed isize::MAX == i64::MAX == mrb_int::MAX");
                let capa = sys::mrb_int::try_from(capacity)
                    .expect("Capacity of an `Array` cannot exceed isize::MAX == i64::MAX == mrb_int::MAX");
                sys::mrb_sys_alloc_rarray(mrb, ptr, length, capa)
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
        assert_eq!(
            into.ruby_type(),
            Ruby::Array,
            "Tried to box Array into {:?} value",
            into.ruby_type()
        );

        let RawParts { ptr, length, capacity } = Array::into_raw_parts(value);
        unsafe {
            sys::mrb_sys_repack_into_rarray(ptr, length as sys::mrb_int, capacity as sys::mrb_int, into.inner());
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

impl<'a> Deref for UnboxedValueGuard<'a, Array> {
    type Target = Array;

    fn deref(&self) -> &Self::Target {
        self.as_inner_ref()
    }
}

#[cfg(test)]
mod tests {
    use crate::test::prelude::*;

    const SUBJECT: &str = "Array";
    const FUNCTIONAL_TEST: &[u8] = include_bytes!("array_functional_test.rb");

    #[test]
    fn functional() {
        let mut interp = interpreter();
        let result = interp.eval(FUNCTIONAL_TEST);
        unwrap_or_panic_with_backtrace(&mut interp, SUBJECT, result);
        let result = interp.eval(b"spec");
        unwrap_or_panic_with_backtrace(&mut interp, SUBJECT, result);
    }
}
