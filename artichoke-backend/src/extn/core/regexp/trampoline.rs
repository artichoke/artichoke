use crate::extn::core::regexp::Regexp;
use crate::extn::prelude::*;

pub fn initialize(
    interp: &mut Artichoke,
    pattern: Value,
    options: Option<Value>,
    encoding: Option<Value>,
    into: Option<Value>,
) -> Result<Value, Exception> {
    Regexp::initialize(interp, pattern, options, encoding, into)
}

pub fn escape(interp: &mut Artichoke, pattern: Value) -> Result<Value, Exception> {
    Regexp::escape(interp, pattern)
}

pub fn union(interp: &mut Artichoke, patterns: &[Value]) -> Result<Value, Exception> {
    Regexp::union(interp, patterns)
}

pub fn is_match(
    interp: &mut Artichoke,
    regexp: Value,
    pattern: Value,
    pos: Option<Value>,
) -> Result<Value, Exception> {
    let regexp = unsafe { Regexp::try_from_ruby(interp, &regexp) }.map_err(|err| {
        if let ArtichokeError::UninitializedValue("Regexp") = err {
            Exception::from(TypeError::new(interp, "uninitialized Regexp"))
        } else {
            Exception::from(Fatal::new(
                interp,
                "Unable to extract Rust Regexp from Ruby Regexp receiver",
            ))
        }
    })?;
    let borrow = regexp.borrow();
    borrow.is_match(interp, pattern, pos)
}

pub fn match_(
    interp: &mut Artichoke,
    regexp: Value,
    pattern: Value,
    pos: Option<Value>,
    block: Option<Block>,
) -> Result<Value, Exception> {
    let regexp = unsafe { Regexp::try_from_ruby(interp, &regexp) }.map_err(|err| {
        if let ArtichokeError::UninitializedValue("Regexp") = err {
            Exception::from(TypeError::new(interp, "uninitialized Regexp"))
        } else {
            Exception::from(Fatal::new(
                interp,
                "Unable to extract Rust Regexp from Ruby Regexp receiver",
            ))
        }
    })?;
    let borrow = regexp.borrow();
    borrow.match_(interp, pattern, pos, block)
}

pub fn eql(interp: &mut Artichoke, regexp: Value, other: Value) -> Result<Value, Exception> {
    let regexp = unsafe { Regexp::try_from_ruby(interp, &regexp) }.map_err(|err| {
        if let ArtichokeError::UninitializedValue("Regexp") = err {
            Exception::from(TypeError::new(interp, "uninitialized Regexp"))
        } else {
            Exception::from(Fatal::new(
                interp,
                "Unable to extract Rust Regexp from Ruby Regexp receiver",
            ))
        }
    })?;
    let borrow = regexp.borrow();
    borrow.eql(interp, other)
}

pub fn case_compare(
    interp: &mut Artichoke,
    regexp: Value,
    other: Value,
) -> Result<Value, Exception> {
    let regexp = unsafe { Regexp::try_from_ruby(interp, &regexp) }.map_err(|err| {
        if let ArtichokeError::UninitializedValue("Regexp") = err {
            Exception::from(TypeError::new(interp, "uninitialized Regexp"))
        } else {
            Exception::from(Fatal::new(
                interp,
                "Unable to extract Rust Regexp from Ruby Regexp receiver",
            ))
        }
    })?;
    let borrow = regexp.borrow();
    borrow.case_compare(interp, other)
}

pub fn match_operator(
    interp: &mut Artichoke,
    regexp: Value,
    pattern: Value,
) -> Result<Value, Exception> {
    let regexp = unsafe { Regexp::try_from_ruby(interp, &regexp) }.map_err(|err| {
        if let ArtichokeError::UninitializedValue("Regexp") = err {
            Exception::from(TypeError::new(interp, "uninitialized Regexp"))
        } else {
            Exception::from(Fatal::new(
                interp,
                "Unable to extract Rust Regexp from Ruby Regexp receiver",
            ))
        }
    })?;
    let borrow = regexp.borrow();
    borrow.match_operator(interp, pattern)
}

pub fn is_casefold(interp: &mut Artichoke, regexp: Value) -> Result<Value, Exception> {
    let regexp = unsafe { Regexp::try_from_ruby(interp, &regexp) }.map_err(|err| {
        if let ArtichokeError::UninitializedValue("Regexp") = err {
            Exception::from(TypeError::new(interp, "uninitialized Regexp"))
        } else {
            Exception::from(Fatal::new(
                interp,
                "Unable to extract Rust Regexp from Ruby Regexp receiver",
            ))
        }
    })?;
    let borrow = regexp.borrow();
    borrow.is_casefold(interp)
}

pub fn is_fixed_encoding(interp: &mut Artichoke, regexp: Value) -> Result<Value, Exception> {
    let regexp = unsafe { Regexp::try_from_ruby(interp, &regexp) }.map_err(|err| {
        if let ArtichokeError::UninitializedValue("Regexp") = err {
            Exception::from(TypeError::new(interp, "uninitialized Regexp"))
        } else {
            Exception::from(Fatal::new(
                interp,
                "Unable to extract Rust Regexp from Ruby Regexp receiver",
            ))
        }
    })?;
    let borrow = regexp.borrow();
    borrow.is_fixed_encoding(interp)
}

pub fn hash(interp: &mut Artichoke, regexp: Value) -> Result<Value, Exception> {
    let regexp = unsafe { Regexp::try_from_ruby(interp, &regexp) }.map_err(|err| {
        if let ArtichokeError::UninitializedValue("Regexp") = err {
            Exception::from(TypeError::new(interp, "uninitialized Regexp"))
        } else {
            Exception::from(Fatal::new(
                interp,
                "Unable to extract Rust Regexp from Ruby Regexp receiver",
            ))
        }
    })?;
    let borrow = regexp.borrow();
    borrow.hash(interp)
}

pub fn inspect(interp: &mut Artichoke, regexp: Value) -> Result<Value, Exception> {
    let regexp = unsafe { Regexp::try_from_ruby(interp, &regexp) }.map_err(|err| {
        if let ArtichokeError::UninitializedValue("Regexp") = err {
            Exception::from(TypeError::new(interp, "uninitialized Regexp"))
        } else {
            Exception::from(Fatal::new(
                interp,
                "Unable to extract Rust Regexp from Ruby Regexp receiver",
            ))
        }
    })?;
    let borrow = regexp.borrow();
    borrow.inspect(interp)
}

pub fn named_captures(interp: &mut Artichoke, regexp: Value) -> Result<Value, Exception> {
    let regexp = unsafe { Regexp::try_from_ruby(interp, &regexp) }.map_err(|err| {
        if let ArtichokeError::UninitializedValue("Regexp") = err {
            Exception::from(TypeError::new(interp, "uninitialized Regexp"))
        } else {
            Exception::from(Fatal::new(
                interp,
                "Unable to extract Rust Regexp from Ruby Regexp receiver",
            ))
        }
    })?;
    let borrow = regexp.borrow();
    borrow.named_captures(interp)
}

pub fn names(interp: &mut Artichoke, regexp: Value) -> Result<Value, Exception> {
    let regexp = unsafe { Regexp::try_from_ruby(interp, &regexp) }.map_err(|err| {
        if let ArtichokeError::UninitializedValue("Regexp") = err {
            Exception::from(TypeError::new(interp, "uninitialized Regexp"))
        } else {
            Exception::from(Fatal::new(
                interp,
                "Unable to extract Rust Regexp from Ruby Regexp receiver",
            ))
        }
    })?;
    let borrow = regexp.borrow();
    borrow.names(interp)
}

pub fn options(interp: &mut Artichoke, regexp: Value) -> Result<Value, Exception> {
    let regexp = unsafe { Regexp::try_from_ruby(interp, &regexp) }.map_err(|err| {
        if let ArtichokeError::UninitializedValue("Regexp") = err {
            Exception::from(TypeError::new(interp, "uninitialized Regexp"))
        } else {
            Exception::from(Fatal::new(
                interp,
                "Unable to extract Rust Regexp from Ruby Regexp receiver",
            ))
        }
    })?;
    let borrow = regexp.borrow();
    borrow.options(interp)
}

pub fn source(interp: &mut Artichoke, regexp: Value) -> Result<Value, Exception> {
    let regexp = unsafe { Regexp::try_from_ruby(interp, &regexp) }.map_err(|err| {
        if let ArtichokeError::UninitializedValue("Regexp") = err {
            Exception::from(TypeError::new(interp, "uninitialized Regexp"))
        } else {
            Exception::from(Fatal::new(
                interp,
                "Unable to extract Rust Regexp from Ruby Regexp receiver",
            ))
        }
    })?;
    let borrow = regexp.borrow();
    borrow.source(interp)
}

pub fn to_s(interp: &mut Artichoke, regexp: Value) -> Result<Value, Exception> {
    let regexp = unsafe { Regexp::try_from_ruby(interp, &regexp) }.map_err(|err| {
        if let ArtichokeError::UninitializedValue("Regexp") = err {
            Exception::from(TypeError::new(interp, "uninitialized Regexp"))
        } else {
            Exception::from(Fatal::new(
                interp,
                "Unable to extract Rust Regexp from Ruby Regexp receiver",
            ))
        }
    })?;
    let borrow = regexp.borrow();
    borrow.string(interp)
}
