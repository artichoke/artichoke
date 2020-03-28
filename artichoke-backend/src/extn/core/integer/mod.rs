use std::convert::TryFrom;
use std::mem;

use crate::extn::core::numeric::Numeric;
use crate::extn::prelude::*;

pub mod mruby;
pub mod trampoline;

#[derive(Debug)]
pub struct Integer;

pub fn chr(
    interp: &mut Artichoke,
    value: Int,
    encoding: Option<Value>,
) -> Result<Vec<u8>, Exception> {
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
                Ok(vec![chr])
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

pub fn element_reference(interp: &Artichoke, value: Int, bit: Value) -> Result<Int, Exception> {
    let _ = interp;
    let bit = bit.implicitly_convert_to_int()?;
    if let Ok(bit) = u32::try_from(bit) {
        Ok(value.checked_shr(bit).map_or(0, |v| v & 1))
    } else {
        Ok(0)
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Quotient {
    Int(Int),
    Float(Float),
}

pub fn div(interp: &mut Artichoke, value: Int, denominator: Value) -> Result<Quotient, Exception> {
    match denominator.ruby_type() {
        Ruby::Fixnum => {
            let denominator = denominator.try_into::<Int>()?;
            if denominator == 0 {
                Err(Exception::from(ZeroDivisionError::new(
                    interp,
                    "divided by 0",
                )))
            } else {
                Ok(Quotient::Int(value / denominator))
            }
        }
        Ruby::Float => {
            let denominator = denominator.try_into::<Float>()?;
            #[allow(clippy::cast_precision_loss)]
            Ok(Quotient::Float(value as Float / denominator))
        }
        _ => {
            let borrow = interp.0.borrow();
            let numeric = borrow
                .class_spec::<Numeric>()
                .ok_or_else(|| NotDefinedError::class("Numeric"))?;
            let numeric = numeric
                .value(interp)
                .ok_or_else(|| NotDefinedError::class("Numeric"))?;
            drop(borrow);
            if let Ok(true) = denominator.funcall("is_a?", &[numeric], None) {
                if denominator.respond_to("to_f")? {
                    let coerced = denominator.funcall::<Value>("to_f", &[], None)?;
                    if let Ruby::Float = coerced.ruby_type() {
                        let denom = coerced.try_into::<Float>()?;
                        #[allow(clippy::cast_precision_loss)]
                        Ok(Quotient::Float(value as Float / denom))
                    } else {
                        let mut message = String::from("can't convert ");
                        message.push_str(denominator.pretty_name());
                        message.push_str(" into Float (");
                        message.push_str(denominator.pretty_name());
                        message.push_str("#to_f gives ");
                        message.push_str(coerced.pretty_name());
                        message.push(')');
                        Err(Exception::from(TypeError::new(interp, message)))
                    }
                } else {
                    let mut message = String::from("can't convert ");
                    message.push_str(denominator.pretty_name());
                    message.push_str(" into Float");
                    Err(Exception::from(TypeError::new(interp, message)))
                }
            } else {
                let mut message = String::from(denominator.pretty_name());
                message.push_str(" can't be coerced into Integer");
                Err(Exception::from(TypeError::new(interp, message)))
            }
        }
    }
}

#[must_use]
pub const fn size(interp: &Artichoke, value: Int) -> usize {
    let _ = interp;
    let _ = value;
    mem::size_of::<Int>()
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
