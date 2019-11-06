//! [`Regexp#match?`](https://ruby-doc.org/core-2.6.3/Regexp.html#method-i-match-3F)

use artichoke_core::value::Value as ValueLike;
use std::convert::TryFrom;

use crate::convert::{Convert, RustBackedValue};
use crate::extn::core::exception::{Fatal, RubyException, TypeError};
use crate::extn::core::regexp::{Backend, Regexp};
use crate::types::Int;
use crate::value::Value;
use crate::Artichoke;

#[derive(Debug, Clone, Copy)]
pub struct Args<'a> {
    pub pattern: Option<&'a str>,
    pub pos: Option<Int>,
}

impl<'a> Args<'a> {
    pub fn extract(
        interp: &Artichoke,
        pattern: Value,
        pos: Option<Value>,
    ) -> Result<Self, Box<dyn RubyException>> {
        let pattern = if let Ok(pattern) = pattern.try_into::<Option<&str>>(interp) {
            pattern
        } else if let Ok(pattern) = pattern.funcall::<Option<&str>>(interp, "to_str", &[], None) {
            pattern
        } else {
            return Err(Box::new(TypeError::new(
                interp,
                "no implicit conversion into String",
            )));
        };
        let pos = if let Some(pos) = pos {
            if let Ok(pos) = pos.try_into::<Int>(interp) {
                Some(pos)
            } else if let Ok(pos) = pos.funcall::<Int>(interp, "to_int", &[], None) {
                Some(pos)
            } else {
                return Err(Box::new(TypeError::new(
                    interp,
                    "no implicit conversion into Integer",
                )));
            }
        } else {
            None
        };
        Ok(Self { pattern, pos })
    }
}

pub fn method(
    interp: &Artichoke,
    args: Args,
    value: &Value,
) -> Result<Value, Box<dyn RubyException>> {
    let data = unsafe { Regexp::try_from_ruby(interp, value) }.map_err(|_| {
        Fatal::new(
            interp,
            "Unable to extract Rust Regexp from Ruby Regexp receiver",
        )
    })?;
    let pattern = if let Some(pattern) = args.pattern {
        pattern
    } else {
        return Ok(interp.convert(false));
    };
    let pattern_char_len = pattern.chars().count();
    let pos = args.pos.unwrap_or_default();
    let pos = if pos < 0 {
        let strlen = Int::try_from(pattern_char_len)
            .map_err(|_| Fatal::new(interp, "Pattern length greater than Integer max"))?;
        let pos = strlen + pos; // this can never wrap since char len is at most `usize`.
        if pos < 0 {
            return Ok(interp.convert(false));
        }
        usize::try_from(pos)
            .map_err(|_| Fatal::new(interp, "Expected positive position to convert to usize"))?
    } else {
        usize::try_from(pos)
            .map_err(|_| Fatal::new(interp, "Expected positive position to convert to usize"))?
    };
    // onig will panic if pos is beyond the end of string
    if pos > pattern_char_len {
        return Ok(interp.convert(false));
    }
    let byte_offset = pattern.chars().take(pos).collect::<String>().len();

    let match_target = &pattern[byte_offset..];
    let borrow = data.borrow();
    let regex = (*borrow.regex)
        .as_ref()
        .ok_or_else(|| Fatal::new(interp, "Uninitialized Regexp"))?;
    match regex {
        Backend::Onig(regex) => Ok(interp.convert(regex.find(match_target).is_some())),
        Backend::Rust(_) => unimplemented!("Rust-backed Regexp"),
    }
}
