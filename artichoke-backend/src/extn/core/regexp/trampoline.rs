use crate::convert::RustBackedValue;
use crate::extn::core::exception::{Fatal, RubyException};
use crate::extn::core::regexp::Regexp;
use crate::value::{Block, Value};
use crate::Artichoke;

pub fn initialize(
    interp: &Artichoke,
    pattern: Value,
    options: Option<Value>,
    encoding: Option<Value>,
    into: Option<Value>,
) -> Result<Value, Box<dyn RubyException>> {
    Regexp::initialize(interp, pattern, options, encoding, into)
}

pub fn escape(interp: &Artichoke, pattern: Value) -> Result<Value, Box<dyn RubyException>> {
    Regexp::escape(interp, pattern)
}

pub fn union(interp: &Artichoke, patterns: &[Value]) -> Result<Value, Box<dyn RubyException>> {
    Regexp::union(interp, patterns)
}

pub fn is_match(
    interp: &Artichoke,
    regexp: Value,
    pattern: Value,
    pos: Option<Value>,
) -> Result<Value, Box<dyn RubyException>> {
    let regexp = unsafe { Regexp::try_from_ruby(interp, &regexp) }.map_err(|_| {
        Fatal::new(
            interp,
            "Unable to extract Rust Regexp from Ruby Regexp receiver",
        )
    })?;
    let borrow = regexp.borrow();
    borrow.is_match(interp, pattern, pos)
}

pub fn match_(
    interp: &Artichoke,
    regexp: Value,
    pattern: Value,
    pos: Option<Value>,
    block: Option<Block>,
) -> Result<Value, Box<dyn RubyException>> {
    let regexp = unsafe { Regexp::try_from_ruby(interp, &regexp) }.map_err(|_| {
        Fatal::new(
            interp,
            "Unable to extract Rust Regexp from Ruby Regexp receiver",
        )
    })?;
    let borrow = regexp.borrow();
    borrow.match_(interp, pattern, pos, block)
}

pub fn eql(
    interp: &Artichoke,
    regexp: Value,
    other: Value,
) -> Result<Value, Box<dyn RubyException>> {
    let regexp = unsafe { Regexp::try_from_ruby(interp, &regexp) }.map_err(|_| {
        Fatal::new(
            interp,
            "Unable to extract Rust Regexp from Ruby Regexp receiver",
        )
    })?;
    let borrow = regexp.borrow();
    borrow.eql(interp, other)
}

pub fn case_compare(
    interp: &Artichoke,
    regexp: Value,
    other: Value,
) -> Result<Value, Box<dyn RubyException>> {
    let regexp = unsafe { Regexp::try_from_ruby(interp, &regexp) }.map_err(|_| {
        Fatal::new(
            interp,
            "Unable to extract Rust Regexp from Ruby Regexp receiver",
        )
    })?;
    let borrow = regexp.borrow();
    borrow.case_compare(interp, other)
}

pub fn match_operator(
    interp: &Artichoke,
    regexp: Value,
    pattern: Value,
) -> Result<Value, Box<dyn RubyException>> {
    let regexp = unsafe { Regexp::try_from_ruby(interp, &regexp) }.map_err(|_| {
        Fatal::new(
            interp,
            "Unable to extract Rust Regexp from Ruby Regexp receiver",
        )
    })?;
    let borrow = regexp.borrow();
    borrow.match_operator(interp, pattern)
}

pub fn is_casefold(interp: &Artichoke, regexp: Value) -> Result<Value, Box<dyn RubyException>> {
    let regexp = unsafe { Regexp::try_from_ruby(interp, &regexp) }.map_err(|_| {
        Fatal::new(
            interp,
            "Unable to extract Rust Regexp from Ruby Regexp receiver",
        )
    })?;
    let borrow = regexp.borrow();
    borrow.is_casefold(interp)
}

pub fn is_fixed_encoding(
    interp: &Artichoke,
    regexp: Value,
) -> Result<Value, Box<dyn RubyException>> {
    let regexp = unsafe { Regexp::try_from_ruby(interp, &regexp) }.map_err(|_| {
        Fatal::new(
            interp,
            "Unable to extract Rust Regexp from Ruby Regexp receiver",
        )
    })?;
    let borrow = regexp.borrow();
    borrow.is_fixed_encoding(interp)
}

pub fn hash(interp: &Artichoke, regexp: Value) -> Result<Value, Box<dyn RubyException>> {
    let regexp = unsafe { Regexp::try_from_ruby(interp, &regexp) }.map_err(|_| {
        Fatal::new(
            interp,
            "Unable to extract Rust Regexp from Ruby Regexp receiver",
        )
    })?;
    let borrow = regexp.borrow();
    borrow.hash(interp)
}

pub fn inspect(interp: &Artichoke, regexp: Value) -> Result<Value, Box<dyn RubyException>> {
    let regexp = unsafe { Regexp::try_from_ruby(interp, &regexp) }.map_err(|_| {
        Fatal::new(
            interp,
            "Unable to extract Rust Regexp from Ruby Regexp receiver",
        )
    })?;
    let borrow = regexp.borrow();
    borrow.inspect(interp)
}

pub fn named_captures(interp: &Artichoke, regexp: Value) -> Result<Value, Box<dyn RubyException>> {
    let regexp = unsafe { Regexp::try_from_ruby(interp, &regexp) }.map_err(|_| {
        Fatal::new(
            interp,
            "Unable to extract Rust Regexp from Ruby Regexp receiver",
        )
    })?;
    let borrow = regexp.borrow();
    borrow.named_captures(interp)
}

pub fn names(interp: &Artichoke, regexp: Value) -> Result<Value, Box<dyn RubyException>> {
    let regexp = unsafe { Regexp::try_from_ruby(interp, &regexp) }.map_err(|_| {
        Fatal::new(
            interp,
            "Unable to extract Rust Regexp from Ruby Regexp receiver",
        )
    })?;
    let borrow = regexp.borrow();
    borrow.names(interp)
}

pub fn options(interp: &Artichoke, regexp: Value) -> Result<Value, Box<dyn RubyException>> {
    let regexp = unsafe { Regexp::try_from_ruby(interp, &regexp) }.map_err(|_| {
        Fatal::new(
            interp,
            "Unable to extract Rust Regexp from Ruby Regexp receiver",
        )
    })?;
    let borrow = regexp.borrow();
    borrow.options(interp)
}

pub fn source(interp: &Artichoke, regexp: Value) -> Result<Value, Box<dyn RubyException>> {
    let regexp = unsafe { Regexp::try_from_ruby(interp, &regexp) }.map_err(|_| {
        Fatal::new(
            interp,
            "Unable to extract Rust Regexp from Ruby Regexp receiver",
        )
    })?;
    let borrow = regexp.borrow();
    borrow.source(interp)
}

pub fn to_s(interp: &Artichoke, regexp: Value) -> Result<Value, Box<dyn RubyException>> {
    let regexp = unsafe { Regexp::try_from_ruby(interp, &regexp) }.map_err(|_| {
        Fatal::new(
            interp,
            "Unable to extract Rust Regexp from Ruby Regexp receiver",
        )
    })?;
    let borrow = regexp.borrow();
    borrow.string(interp)
}
