//! Encoding represents a character encoding usable in Ruby
//!
//! This module implements the [`Encoding`] class from Ruby Core.
//!
//! You can use this class in your application by accessing it directly. As a
//! Core class, it is globally available:
//!
//! ```ruby
//! Encoding.list
//! ```
//!
//! [`Encoding`]: https://ruby-doc.org/3.1.2/Encoding.html

mod backend;
pub(in crate::extn) mod mruby;
pub(super) mod trampoline;

use crate::convert::{Immediate, UnboxedValueGuard};
use crate::extn::prelude::*;
use artichoke_core::encoding::Encoding as _;
pub use backend::{replica::Encoding as ReplicaEncoding, spinoso::Encoding as SpinosoEncoding, Encoding};
use bstr::ByteVec;
use core::ffi::c_void;

impl BoxUnboxVmValue for Encoding {
    type Unboxed = Self;
    type Guarded = Immediate<Self::Unboxed>;

    const RUBY_TYPE: &'static str = "Encoding";

    #[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
    unsafe fn unbox_from_value<'a>(
        value: &'a mut Value,
        interp: &mut Artichoke,
    ) -> Result<UnboxedValueGuard<'a, Self::Guarded>, Error> {
        if value.ruby_type() != Ruby::Encoding {
            let mut message = String::from("uninitialized ");
            message.push_str(Self::RUBY_TYPE);
            return Err(TypeError::from(message).into());
        }

        let value = value.inner().value.i;
        let value: u8 = if let 0..=255 = value {
            // SAFETY: 0..=255 will always be able to be cast into a `u8`
            value as u8
        } else {
            let mut message = String::from("uninitialized ");
            message.push_str(Self::RUBY_TYPE);
            return Err(TypeError::from(message).into());
        };

        // TODO: Maybe we just use try_from_flag instead of storing encodings
        // on state.
        let state = interp.state.as_deref_mut().ok_or_else(InterpreterExtractError::new)?;
        let encoding = state.encodings.get(&value).ok_or_else(|| {
            let mut message = String::from("uninitialized ");
            message.push_str(Self::RUBY_TYPE);
            TypeError::from(message)
        })?;

        Ok(UnboxedValueGuard::new(Immediate::new(encoding.clone())))
    }

    fn alloc_value(value: Self::Unboxed, interp: &mut Artichoke) -> Result<Value, Error> {
        // Ensure the Encoding class is around bound.
        let _rclass = {
            let state = interp.state.as_deref_mut().ok_or_else(InterpreterExtractError::new)?;
            let encoding_class = state
                .classes
                .get::<Self>()
                .ok_or_else(|| NotDefinedError::class(Self::RUBY_TYPE))?;

            let rclass = encoding_class.rclass();
            unsafe { interp.with_ffi_boundary(|mrb| rclass.resolve(mrb)) }?
                .ok_or_else(|| NotDefinedError::class(Self::RUBY_TYPE))?
        };

        // Create the encoding type
        let encoding_flag: u8 = value.clone().into();
        let obj = unsafe { sys::mrb_sys_new_encoding(i64::from(encoding_flag)) };
        let encoding = Value::from(obj);

        // Generate and assign all the constants for this encoding
        for alias in value.aliases() {
            // TODO: Better way to unwrap Vec<u8> into &str
            interp.define_class_constant::<Self>(&alias.into_string().unwrap(), encoding)?;
        }

        // We should now be able to register this Encoding as being in use.
        interp.def_encoding(value)?;
        //state.encodings.insert(encoding_flag, value);
        Ok(encoding)
    }

    fn box_into_value(value: Self::Unboxed, into: Value, interp: &mut Artichoke) -> Result<Value, Error> {
        let _ = value;
        let _ = into;
        let _ = interp;
        Err(Fatal::from("Encodings are immutable and cannot be reinitialized").into())
    }

    fn free(data: *mut c_void) {
        // this function is never called. `Symbol` is `Copy`/immediate and does
        // not have a destructor registered in the class registry.
        let _ = data;
    }
}
#[cfg(test)]
mod tests {
    use crate::test::prelude::*;

    const SUBJECT: &str = "Encoding";
    const FUNCTIONAL_TEST: &[u8] = include_bytes!("encoding_test.rb");

    #[test]
    fn functional() {
        let mut interp = interpreter();
        let result = interp.eval(FUNCTIONAL_TEST);
        unwrap_or_panic_with_backtrace(&mut interp, SUBJECT, result);
        let result = interp.eval(b"spec");
        unwrap_or_panic_with_backtrace(&mut interp, SUBJECT, result);
    }
}
