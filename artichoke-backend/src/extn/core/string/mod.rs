use core::ops::Deref;
use std::ffi::c_void;
use std::os::raw::c_char;

use artichoke_core::value::Value as _;
use spinoso_exception::TypeError;
#[doc(inline)]
use spinoso_string::{Encoding, RawParts, String};

use crate::convert::{BoxUnboxVmValue, UnboxedValueGuard};
use crate::error::Error;
use crate::sys;
use crate::types::Ruby;
use crate::value::Value;
use crate::Artichoke;

mod ffi;
pub mod mruby;
pub mod trampoline;

const ENCODING_FLAG_BITPOS: usize = 5;

impl BoxUnboxVmValue for String {
    type Unboxed = Self;
    type Guarded = String;

    const RUBY_TYPE: &'static str = "String";

    #[allow(clippy::cast_possible_truncation)]
    #[allow(clippy::cast_sign_loss)]
    unsafe fn unbox_from_value<'a>(
        value: &'a mut Value,
        interp: &mut Artichoke,
    ) -> Result<UnboxedValueGuard<'a, Self::Guarded>, Error> {
        let _ = interp;

        // Make sure we have a String otherwise extraction will fail.
        // This check is critical to the safety of accessing the `value` union.
        if value.ruby_type() != Ruby::String {
            let mut message = std::string::String::from("uninitialized ");
            message.push_str(Self::RUBY_TYPE);
            return Err(TypeError::from(message).into());
        }

        // Safety:
        //
        // The above check on the data type ensures the `value` union holds an
        // `RString` in the `p` variant.
        let value = value.inner();
        let string = sys::mrb_sys_basic_ptr(value).cast::<sys::RString>();

        let ptr = (*string).as_.heap.ptr;
        let length = (*string).as_.heap.len as usize;
        let capacity = (*string).as_.heap.aux.capa as usize;

        // the encoding flag is 4 bits wide.
        let flags = string.as_ref().unwrap().flags();
        let encoding_flag = flags & (0b1111 << ENCODING_FLAG_BITPOS);
        let encoding = (encoding_flag >> ENCODING_FLAG_BITPOS) as u8;
        let encoding = Encoding::try_from_flag(encoding).map_err(|_| TypeError::with_message("Unknown encoding"))?;

        let s = String::from_raw_parts_with_encoding(
            RawParts {
                ptr: ptr.cast::<u8>(),
                length,
                capacity,
            },
            encoding,
        );
        let s = UnboxedValueGuard::new(s);

        Ok(s)
    }

    fn alloc_value(value: Self::Unboxed, interp: &mut Artichoke) -> Result<Value, Error> {
        let encoding = value.encoding();
        let RawParts { ptr, length, capacity } = String::into_raw_parts(value);
        let value = unsafe {
            interp.with_ffi_boundary(|mrb| {
                sys::mrb_sys_alloc_rstring(
                    mrb,
                    ptr.cast::<c_char>(),
                    length as sys::mrb_int,
                    capacity as sys::mrb_int,
                )
            })?
        };
        let string = unsafe { sys::mrb_sys_basic_ptr(value).cast::<sys::RString>() };
        unsafe {
            let flags = string.as_ref().unwrap().flags();
            let encoding_bits = encoding.to_flag();
            let flags_with_zeroed_encoding = flags & !(0b1111 << ENCODING_FLAG_BITPOS);
            let flags_with_encoding = flags_with_zeroed_encoding | (u32::from(encoding_bits) << ENCODING_FLAG_BITPOS);
            string.as_mut().unwrap().set_flags(flags_with_encoding);
        }
        Ok(interp.protect(value.into()))
    }

    fn box_into_value(value: Self::Unboxed, into: Value, interp: &mut Artichoke) -> Result<Value, Error> {
        // Make sure we have an String otherwise boxing will produce undefined
        // behavior.
        //
        // This check is critical to the memory safety of future runs of the
        // garbage collector.
        assert_eq!(
            into.ruby_type(),
            Ruby::String,
            "Tried to box String into {:?} value",
            into.ruby_type()
        );

        let encoding = value.encoding();
        let RawParts { ptr, length, capacity } = String::into_raw_parts(value);
        let string = unsafe {
            sys::mrb_sys_repack_into_rstring(
                ptr.cast::<c_char>(),
                length as sys::mrb_int,
                capacity as sys::mrb_int,
                into.inner(),
            )
        };
        unsafe {
            let flags = string.as_ref().unwrap().flags();
            let encoding_bits = encoding.to_flag();
            let flags_with_zeroed_encoding = flags & !(0b1111 << ENCODING_FLAG_BITPOS);
            let flags_with_encoding = flags_with_zeroed_encoding | (u32::from(encoding_bits) << ENCODING_FLAG_BITPOS);
            string.as_mut().unwrap().set_flags(flags_with_encoding);
        }

        Ok(interp.protect(into))
    }

    fn free(data: *mut c_void) {
        // this function is never called. `String` is freed directly in the VM
        // by calling `mrb_gc_free_str` which is defined in
        // `extn/core/string/ffi.rs`.
        //
        // `String` should not have a destructor registered in the class
        // registry.
        let _ = data;
    }
}

impl<'a> Deref for UnboxedValueGuard<'a, String> {
    type Target = String;

    fn deref(&self) -> &Self::Target {
        self.as_inner_ref()
    }
}

#[cfg(test)]
mod tests {
    use crate::test::prelude::*;

    const SUBJECT: &str = "String";
    const FUNCTIONAL_TEST: &[u8] = include_bytes!("string_test.rb");

    #[test]
    #[cfg(feature = "core-regexp")]
    fn functional() {
        let mut interp = interpreter();
        let result = interp.eval(FUNCTIONAL_TEST);
        unwrap_or_panic_with_backtrace(&mut interp, SUBJECT, result);
        let result = interp.eval(b"spec");
        unwrap_or_panic_with_backtrace(&mut interp, SUBJECT, result);
    }

    #[test]
    fn modifying_and_repacking_encoding_zeroes_old_encoding_flags() {
        let mut interp = interpreter();
        // Modify the encoding of a binary string in place to be UTF-8 by
        // pushing a UTF-8 string into an empty binary string.
        //
        // Test for the newly taken UTF-8 encoding by ensuring that the char
        // length of the string is 1.
        let test = "be = ''.b ; be << '😀' ; raise 'unexpected encoding' unless be.length == 1";
        let result = interp.eval(test.as_bytes());
        unwrap_or_panic_with_backtrace(&mut interp, SUBJECT, result);
    }
}
