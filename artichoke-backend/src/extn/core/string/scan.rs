use std::cmp;
use std::mem;

use crate::convert::{Convert, RustBackedValue, TryConvert};
use crate::extn::core::matchdata::MatchData;
use crate::extn::core::regexp::enc::Encoding;
use crate::extn::core::regexp::opts::Options;
use crate::extn::core::regexp::{syntax, Backend, Regexp};
use crate::sys;
use crate::value::{Value, ValueLike};
use crate::Artichoke;

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

    pub unsafe fn extract(interp: &Artichoke) -> Result<Self, Error> {
        let mut pattern = <mem::MaybeUninit<sys::mrb_value>>::uninit();
        let mut block = <mem::MaybeUninit<sys::mrb_value>>::uninit();
        sys::mrb_get_args(
            interp.0.borrow().mrb,
            Self::ARGSPEC.as_ptr() as *const i8,
            pattern.as_mut_ptr(),
            block.as_mut_ptr(),
        );
        let pattern = pattern.assume_init();
        let block = block.assume_init();
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

pub fn method(interp: &Artichoke, args: Args, value: Value) -> Result<Value, Error> {
    let mrb = interp.0.borrow().mrb;
    let regexp = args.regexp.ok_or(Error::WrongType)?;
    let s = value.itself().map_err(|_| Error::Fatal)?;
    let s = interp.try_convert(s).map_err(|_| Error::WrongType)?;

    let last_match_sym = interp.0.borrow_mut().sym_intern("$~");
    let data = MatchData::new(s, regexp.clone(), 0, s.len());
    let data = unsafe { data.try_into_ruby(interp, None) }.map_err(|_| Error::Fatal)?;
    unsafe { sys::mrb_gv_set(mrb, last_match_sym, data.inner()) };
    let matchdata = unsafe { MatchData::try_from_ruby(interp, &data) }.map_err(|_| Error::Fatal)?;

    let mut was_match = false;
    let mut collected = vec![];
    let regex = (*regexp.regex).as_ref().ok_or(Error::Fatal)?;
    match regex {
        Backend::Onig(regex) => {
            let len = regex.captures_len();

            if len > 0 {
                for captures in regex.captures_iter(s) {
                    was_match = true;
                    let mut groups = vec![];
                    let num_regexp_globals_to_set = {
                        let num_previously_set_globals =
                            interp.0.borrow().num_set_regexp_capture_globals;
                        cmp::max(num_previously_set_globals, captures.len())
                    };
                    for group in 0..num_regexp_globals_to_set {
                        let sym = if group == 0 {
                            interp.0.borrow_mut().sym_intern("$&")
                        } else {
                            interp.0.borrow_mut().sym_intern(&format!("${}", group))
                        };

                        let capture = captures.at(group);
                        if group > 0 {
                            groups.push(captures.at(group));
                        }
                        unsafe {
                            sys::mrb_gv_set(mrb, sym, interp.convert(capture).inner());
                        }
                    }
                    interp.0.borrow_mut().num_set_regexp_capture_globals = captures.len();

                    let matched = interp.convert(groups);
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
                for pos in regex.find_iter(s) {
                    was_match = true;
                    let scanned = &s[pos.0..pos.1];
                    let matched = interp.convert(scanned);
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
        }
        Backend::Rust(_) => unimplemented!("Rust-backed Regexp"),
    };
    if !was_match {
        unsafe {
            sys::mrb_gv_set(mrb, last_match_sym, sys::mrb_sys_nil_value());
        }
    }
    let result = if args.block.is_some() {
        value
    } else {
        interp.convert(collected)
    };
    Ok(result)
}
