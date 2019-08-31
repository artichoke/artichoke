use log::trace;
use std::convert::TryFrom;
use std::mem;

use crate::convert::{Convert, TryConvert};
use crate::def::{ClassLike, Define};
use crate::eval::Eval;
use crate::extn::core::error::{ArgumentError, RangeError, RubyException, RuntimeError};
use crate::sys;
use crate::types::Int;
use crate::value::Value;
use crate::{Artichoke, ArtichokeError};

pub fn init(interp: &Artichoke) -> Result<(), ArtichokeError> {
    if interp.borrow().class_spec::<Integer>().is_some() {
        return Ok(());
    }

    let integer = interp
        .borrow_mut()
        .def_class::<Integer>("Integer", None, None);

    integer
        .borrow_mut()
        .add_method("chr", Integer::chr, sys::mrb_args_opt(1));

    integer
        .borrow_mut()
        .add_method("size", Integer::size, sys::mrb_args_none());

    integer
        .borrow()
        .define(interp)
        .map_err(|_| ArtichokeError::New)?;

    interp.eval(include_str!("integer.rb"))?;

    trace!("Patched Integer onto interpreter");

    Ok(())
}

pub struct Integer;

impl Integer {
    pub unsafe extern "C" fn chr(mrb: *mut sys::mrb_state, slf: sys::mrb_value) -> sys::mrb_value {
        let interp = unwrap_interpreter!(mrb);
        let mut encoding = <mem::MaybeUninit<sys::mrb_value>>::uninit();
        let argc = sys::mrb_get_args(mrb, b"|o\0".as_ptr() as *const i8, encoding.as_mut_ptr());
        match argc {
            0 => {
                // When no encoding is supplied, MRI assumes the encoding is
                // either ASCII or ASCII-8BIT.
                //
                // - `Integer`s from 0..127 result in a `String` with ASCII
                //   encoding.
                // - `Integer`s from 128..256 result in a `String` with binary
                //   (ASCII-8BIT) encoding.
                // - All other integers raise a `RangeError`.
                //
                // ```txt
                // [2.6.3] > [0.chr, 0.chr.encoding]
                // => ["\x00", #<Encoding:US-ASCII>]
                // [2.6.3] > [127.chr, 127.chr.encoding]
                // => ["\x7F", #<Encoding:US-ASCII>]
                // [2.6.3] > [128.chr, 128.chr.encoding]
                // => ["\x80", #<Encoding:ASCII-8BIT>]
                // [2.6.3] > [255.chr, 255.chr.encoding]
                // => ["\xFF", #<Encoding:ASCII-8BIT>]
                // [2.6.3] > [256.chr, 256.chr.encoding]
                // Traceback (most recent call last):
                //         5: from /usr/local/var/rbenv/versions/2.6.3/bin/irb:23:in `<main>'
                //         4: from /usr/local/var/rbenv/versions/2.6.3/bin/irb:23:in `load'
                //         3: from /usr/local/var/rbenv/versions/2.6.3/lib/ruby/gems/2.6.0/gems/irb-1.0.0/exe/irb:11:in `<top (required)>'
                //         2: from (irb):9
                //         1: from (irb):9:in `chr'
                // RangeError (256 out of char range)
                // ```
                if let Ok(chr) = Int::try_convert(&interp, Value::new(&interp, slf)) {
                    match u8::try_from(chr) {
                        Ok(chr @ 0..=127) | Ok(chr @ 128..=255) => {
                            // ASCII encoding | Binary/ASCII-8BIT encoding
                            // Without `Encoding` support, these two arms are the same
                            Value::convert(&interp, vec![chr]).inner()
                        }
                        _ => {
                            let value = Value::new(&interp, slf);
                            RangeError::raisef(interp, "%S out of char range", vec![value])
                        }
                    }
                } else {
                    let value = Value::new(&interp, slf);
                    RangeError::raisef(interp, "%S out of char range", vec![value])
                }
            }
            1 => {
                let encoding = encoding.assume_init();
                let encoding = Value::new(&interp, encoding);
                unimplemented!(
                    "Integer#chr not implemented with explicit encoding argument {:?}",
                    encoding
                )
            }
            n => {
                let argc = Value::convert(&interp, n);
                ArgumentError::raisef(
                    interp,
                    "wrong number of arguments (given %i, expected 0..1)",
                    vec![argc],
                )
            }
        }
    }

    pub unsafe extern "C" fn size(mrb: *mut sys::mrb_state, slf: sys::mrb_value) -> sys::mrb_value {
        let interp = unwrap_interpreter!(mrb);
        let result = Int::try_convert(&interp, Value::new(&interp, slf));
        if result.is_ok() {
            if let Ok(size) = Int::try_from(mem::size_of::<Int>()) {
                Value::convert(&interp, size).inner()
            } else {
                RuntimeError::raise(interp, "fatal Integer#size error")
            }
        } else {
            RuntimeError::raise(interp, "fatal Integer#size error")
        }
    }
}
