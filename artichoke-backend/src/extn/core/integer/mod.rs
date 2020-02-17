use std::convert::TryFrom;
use std::mem;

use crate::extn::core::float::Float;
use crate::extn::prelude::*;
use crate::types;

pub mod mruby;
pub mod trampoline;

#[derive(Debug)]
pub struct Integer;

pub fn chr(
    interp: &mut Artichoke,
    value: Int,
    encoding: Option<Value>,
) -> Result<Value, Exception> {
    if let Some(encoding) = encoding {
        let mut message = b"encoding parameter of Integer#chr (given ".to_vec();
        message.extend(encoding.inspect());
        message.extend(b") not supported");
        Err(Exception::from(NotImplementedError::new_raw(
            &interp, message,
        )))
    } else {
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
        match u8::try_from(value) {
            // ASCII encoding | Binary/ASCII-8BIT encoding
            // Without `Encoding` support, these two arms are the same
            Ok(chr @ 0..=127) | Ok(chr @ 128..=255) => {
                // Create a single byte `String` from the character given by
                // `self`.
                Ok(interp.convert_mut(&[chr][..]))
            }
            _ => {
                let mut message = String::new();
                string::format_int_into(&mut message, value)?;
                message.push_str(" out of char range");
                Err(Exception::from(RangeError::new(&interp, message)))
            }
        }
    }
}

fn element_reference(interp: &Artichoke, value: Int, bit: Value) -> Result<Value, Exception> {
    let bit = bit.implicitly_convert_to_int()?;
    let result = if let Ok(bit) = u32::try_from(bit) {
        value.checked_shr(bit).map_or(0, |v| v & 1)
    } else {
        0
    };
    Ok(interp.convert(result))
}

pub fn div(interp: &mut Artichoke, value: Int, denominator: Value) -> Result<Value, Exception> {
    match denominator.ruby_type() {
        Ruby::Fixnum => {
            let denominator = denominator.try_into::<Int>()?;
            if denominator == 0 {
                Err(Exception::from(ZeroDivisionError::new(
                    interp,
                    "divided by 0",
                )))
            } else {
                Ok(interp.convert(value / denominator))
            }
        }
        Ruby::Float => {
            let denominator = denominator.try_into::<types::Float>()?;
            if denominator == 0.0 {
                match value {
                    x if x > 0 => Ok(interp.convert_mut(Float::INFINITY)),
                    x if x < 0 => Ok(interp.convert_mut(Float::NEG_INFINITY)),
                    _ => Ok(interp.convert_mut(Float::NAN)),
                }
            } else {
                #[allow(clippy::cast_precision_loss)]
                Ok(interp.convert_mut(value as types::Float / denominator))
            }
        }
        _ => {
            let mut message = String::from(denominator.pretty_name());
            message.push_str(" can't be coerced into Integer");
            Err(Exception::from(TypeError::new(interp, message)))
        }
    }
}

fn size(interp: &Artichoke, value: Int) -> Result<Value, Fatal> {
    let _ = value;
    let size = mem::size_of::<Int>();
    let size = Int::try_from(size)
        .map_err(|_| Fatal::new(interp, "size of Integer exceeds Integer max value"))?;
    Ok(interp.convert(size))
}

#[cfg(test)]
mod tests {
    use quickcheck_macros::quickcheck;

    use crate::test::prelude::*;

    #[quickcheck]
    fn integer_division_vm_opcode(x: Int, y: Int) -> bool {
        let mut interp = crate::interpreter().expect("init");
        let mut result = true;
        match (x, y) {
            (0, 0) => result &= interp.eval(b"0 / 0").is_err(),
            (x, 0) | (0, x) => {
                let expr = format!("{} / 0", x).into_bytes();
                result &= interp.eval(expr.as_slice()).is_err();
                let expr = format!("0 / {}", x).into_bytes();
                let division = interp
                    .eval(expr.as_slice())
                    .unwrap()
                    .try_into::<Int>()
                    .unwrap();
                result &= division == 0
            }
            (x, y) => {
                let expr = format!("{} / {}", x, y).into_bytes();
                let division = interp
                    .eval(expr.as_slice())
                    .unwrap()
                    .try_into::<Int>()
                    .unwrap();
                result &= division == x / y
            }
        }
        result
    }

    #[quickcheck]
    fn integer_division_send(x: Int, y: Int) -> bool {
        let mut interp = crate::interpreter().expect("init");
        let mut result = true;
        match (x, y) {
            (0, 0) => result &= interp.eval(b"0.send('/', 0)").is_err(),
            (x, 0) | (0, x) => {
                let expr = format!("{}.send('/', 0)", x).into_bytes();
                result &= interp.eval(expr.as_slice()).is_err();
                let expr = format!("0.send('/', {})", x).into_bytes();
                let division = interp
                    .eval(expr.as_slice())
                    .unwrap()
                    .try_into::<Int>()
                    .unwrap();
                result &= division == 0
            }
            (x, y) => {
                let expr = format!("{}.send('/', {})", x, y).into_bytes();
                let division = interp
                    .eval(expr.as_slice())
                    .unwrap()
                    .try_into::<Int>()
                    .unwrap();
                result &= division == x / y
            }
        }
        result
    }
}
