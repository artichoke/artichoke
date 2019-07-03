#![allow(warnings)]
use std::cmp;
use std::mem;

use crate::convert::{FromMrb, RustBackedValue, TryFromMrb};
use crate::extn::core::matchdata::MatchData;
use crate::extn::core::regexp::{syntax, Encoding, Options, Regexp};
use crate::gc::MrbGarbageCollection;
use crate::interpreter::{Mrb, MrbApi};
use crate::sys;
use crate::value::{Value, ValueLike};

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub enum Error {
    Fatal,
    WrongType,
}

#[derive(Debug)]
pub struct Args {
    pub regexp: Option<Regexp>,
    pub block: Option<Value>,
}

impl Args {
    const ARGSPEC: &'static [u8] = b"o&\0";

    pub unsafe fn extract(interp: &Mrb) -> Result<Self, Error> {
        let pattern = mem::uninitialized::<sys::mrb_value>();
        let block = mem::uninitialized::<sys::mrb_value>();
        sys::mrb_get_args(
            interp.borrow().mrb,
            Self::ARGSPEC.as_ptr() as *const i8,
            &pattern,
            &block,
        );
        let regexp = if let Ok(regexp) = Regexp::try_from_ruby(interp, &Value::new(interp, pattern))
        {
            Some(regexp.borrow().clone())
        } else if let Some(ref pattern) = Value::new(interp, pattern)
            .funcall::<Option<String>, _, _>("to_str", &[])
            .map_err(|_| Error::WrongType)?
        {
            Regexp::new(
                syntax::escape(pattern),
                syntax::escape(pattern),
                Options::default(),
                Options::default(),
                Encoding::default(),
            )
        } else {
            None
        };
        let block = if sys::mrb_sys_value_is_nil(block) {
            None
        } else {
            Some(Value::new(interp, block))
        };
        Ok(Self { regexp, block })
    }
}

pub fn method(interp: &Mrb, args: Args, value: Value) -> Result<Value, Error> {
    let mrb = interp.borrow().mrb;
    let regexp = args.regexp.ok_or(Error::WrongType)?;
    let s = value.itself().map_err(|_| Error::Fatal)?;
    let s = unsafe { String::try_from_mrb(&interp, s) }.map_err(|_| Error::WrongType)?;

    let gc_was_enabled = interp.disable_gc();

    let last_match_sym = interp.borrow_mut().sym_intern("$~");
    let data = MatchData::new(s.as_str(), regexp.clone(), 0, s.len());
    let data = unsafe { data.try_into_ruby(interp, None) }.map_err(|_| Error::Fatal)?;
    unsafe { sys::mrb_gv_set(mrb, last_match_sym, data.inner()) };
    let matchdata = unsafe { MatchData::try_from_ruby(interp, &data) }.map_err(|_| Error::Fatal)?;

    let mut was_match = false;
    let mut collected = vec![];
    let len = regexp.regex.captures_len();

    if len > 0 {
        for captures in regexp.regex.captures_iter(s.as_str()) {
            was_match = true;
            let mut groups = vec![];
            for index in 1..=len {
                groups.push(captures.at(index));
            }
            let matched = Value::from_mrb(interp, groups);
            if let Some(pos) = captures.pos(0) {
                matchdata.borrow_mut().set_region(pos.0, pos.1);
            }
            if let Some(ref block) = args.block {
                unsafe {
                    sys::mrb_yield(mrb, block.inner(), matched.inner());
                    sys::mrb_gv_set(mrb, last_match_sym, data.inner());
                }
            } else {
                collected.push(matched);
            }
        }
    } else {
        for pos in regexp.regex.find_iter(s.as_str()) {
            was_match = true;
            let scanned = &s[pos.0..pos.1];
            let matched = Value::from_mrb(interp, scanned);
            matchdata.borrow_mut().set_region(pos.0, pos.1);
            if let Some(ref block) = args.block {
                unsafe {
                    sys::mrb_yield(mrb, block.inner(), matched.inner());
                    sys::mrb_gv_set(mrb, last_match_sym, data.inner());
                }
            } else {
                collected.push(matched);
            }
        }
    }
    if !was_match {
        unsafe {
            sys::mrb_gv_set(mrb, last_match_sym, interp.nil().inner());
        }
    }
    let result = if args.block.is_some() {
        value
    } else {
        Value::from_mrb(interp, collected)
    };
    if gc_was_enabled {
        interp.enable_gc();
    }
    Ok(result)
}
