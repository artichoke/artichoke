use onig::{Regex, Syntax};
use std::cmp::{self, Ordering};
use std::collections::HashMap;
use std::convert::TryFrom;
use std::fmt;
use std::rc::Rc;
use std::str;

use crate::extn::core::matchdata::MatchData;
use crate::extn::core::regexp::{self, Config, Encoding, Regexp, RegexpType};
use crate::extn::prelude::*;

use super::{HashOfStringToArrayOfInt, NilableString};

#[derive(Clone)]
pub struct Onig {
    literal: Config,
    derived: Config,
    encoding: Encoding,
    regex: Rc<Regex>,
}

impl Onig {
    pub fn new(
        interp: &Artichoke,
        literal: Config,
        derived: Config,
        encoding: Encoding,
    ) -> Result<Self, Exception> {
        let pattern = str::from_utf8(derived.pattern.as_slice()).map_err(|_| {
            ArgumentError::new(
                interp,
                "Oniguruma-backed Regexp only supports UTF-8 patterns",
            )
        })?;
        let regex = Regex::with_options(pattern, derived.options.flags(), Syntax::ruby()).map_err(
            |err| {
                if literal.options.literal {
                    Exception::from(SyntaxError::new(interp, err.description().to_owned()))
                } else {
                    Exception::from(RegexpError::new(interp, err.description().to_owned()))
                }
            },
        )?;
        let regexp = Self {
            literal,
            derived,
            encoding,
            regex: Rc::new(regex),
        };
        Ok(regexp)
    }
}

impl fmt::Debug for Onig {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "/{}/{}{}",
            String::from_utf8_lossy(self.literal.pattern.as_slice()).replace("/", r"\/"),
            self.literal.options.modifier_string(),
            self.encoding.string()
        )
    }
}

impl fmt::Display for Onig {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}",
            String::from_utf8_lossy(self.derived.pattern.as_slice())
        )
    }
}

impl RegexpType for Onig {
    fn box_clone(&self) -> Box<dyn RegexpType> {
        Box::new(self.clone())
    }

    fn captures(
        &self,
        interp: &Artichoke,
        haystack: &[u8],
    ) -> Result<Option<Vec<NilableString>>, Exception> {
        let haystack = str::from_utf8(haystack).map_err(|_| {
            ArgumentError::new(
                interp,
                "Oniguruma-backed Regexp only supports UTF-8 haystacks",
            )
        })?;
        let result = self.regex.captures(haystack).map(|captures| {
            captures
                .iter()
                .map(|capture| capture.map(str::as_bytes).map(<[u8]>::to_vec))
                .collect()
        });
        Ok(result)
    }

    fn capture_indexes_for_name(
        &self,
        interp: &Artichoke,
        name: &[u8],
    ) -> Result<Option<Vec<usize>>, Exception> {
        let _ = interp;
        let mut result = None;
        self.regex.foreach_name(|group, group_indexes| {
            if name == group.as_bytes() {
                let mut indexes = Vec::with_capacity(group_indexes.len());
                for index in group_indexes {
                    if let Ok(index) = usize::try_from(*index) {
                        indexes.push(index);
                    }
                }
                result = Some(indexes);
                false
            } else {
                true
            }
        });
        Ok(result)
    }

    fn captures_len(
        &self,
        interp: &Artichoke,
        haystack: Option<&[u8]>,
    ) -> Result<usize, Exception> {
        let result = if let Some(haystack) = haystack {
            let haystack = str::from_utf8(haystack).map_err(|_| {
                ArgumentError::new(
                    interp,
                    "Oniguruma-backed Regexp only supports UTF-8 haystacks",
                )
            })?;
            self.regex
                .captures(haystack)
                .map(|captures| captures.len())
                .unwrap_or_default()
        } else {
            self.regex.captures_len()
        };
        Ok(result)
    }

    fn capture0<'a>(
        &self,
        interp: &Artichoke,
        haystack: &'a [u8],
    ) -> Result<Option<&'a [u8]>, Exception> {
        let haystack = str::from_utf8(haystack).map_err(|_| {
            ArgumentError::new(
                interp,
                "Oniguruma-backed Regexp only supports UTF-8 haystacks",
            )
        })?;
        let result = self
            .regex
            .captures(haystack)
            .and_then(|captures| captures.at(0))
            .map(str::as_bytes);
        Ok(result)
    }

    fn debug(&self) -> String {
        format!("{:?}", self)
    }

    fn literal_config(&self) -> &Config {
        &self.literal
    }

    fn derived_config(&self) -> &Config {
        &self.derived
    }

    fn encoding(&self) -> &Encoding {
        &self.encoding
    }

    fn inspect(&self, interp: &Artichoke) -> Vec<u8> {
        let _ = interp;
        // pattern length + 2x '/' + mix + encoding
        let mut inspect = Vec::with_capacity(self.literal.pattern.len() + 2 + 4);
        inspect.push(b'/');
        if let Ok(pat) = str::from_utf8(self.literal.pattern.as_slice()) {
            inspect.extend(pat.replace("/", r"\/").as_bytes());
        } else {
            inspect.extend(self.literal.pattern.iter());
        }
        inspect.push(b'/');
        inspect.extend(self.literal.options.modifier_string().as_bytes());
        inspect.extend(self.encoding.string().as_bytes());
        inspect
    }

    fn string(&self, interp: &Artichoke) -> &[u8] {
        let _ = interp;
        self.derived.pattern.as_slice()
    }

    fn case_match(&self, interp: &mut Artichoke, pattern: &[u8]) -> Result<bool, Exception> {
        let pattern = str::from_utf8(pattern).map_err(|_| {
            ArgumentError::new(
                interp,
                "Oniguruma-backed Regexp only supports UTF-8 patterns",
            )
        })?;
        let mrb = interp.0.borrow().mrb;
        if let Some(captures) = self.regex.captures(pattern) {
            let globals_to_set = cmp::max(interp.0.borrow().active_regexp_globals, captures.len());
            let sym = interp.intern_symbol(regexp::LAST_MATCHED_STRING);
            let value = interp.convert_mut(captures.at(0));
            unsafe {
                sys::mrb_gv_set(mrb, sym, value.inner());
            }
            for group in 1..=globals_to_set {
                let sym = interp.intern_symbol(regexp::nth_match_group(group));
                let value = interp.convert_mut(captures.at(group));
                unsafe {
                    sys::mrb_gv_set(mrb, sym, value.inner());
                }
            }
            interp.0.borrow_mut().active_regexp_globals = captures.len();

            if let Some(match_pos) = captures.pos(0) {
                let pre_match = interp.convert_mut(&pattern[..match_pos.0]);
                let post_match = interp.convert_mut(&pattern[match_pos.1..]);
                let pre_match_sym = interp.intern_symbol(regexp::STRING_LEFT_OF_MATCH);
                let post_match_sym = interp.intern_symbol(regexp::STRING_RIGHT_OF_MATCH);
                unsafe {
                    sys::mrb_gv_set(mrb, pre_match_sym, pre_match.inner());
                    sys::mrb_gv_set(mrb, post_match_sym, post_match.inner());
                }
            }
            let matchdata = MatchData::new(
                pattern.as_bytes().to_vec(),
                Regexp::from(self.box_clone()),
                0,
                pattern.len(),
            );
            let matchdata = matchdata.try_into_ruby(&interp, None).map_err(|_| {
                Fatal::new(interp, "Could not create Ruby Value from Rust MatchData")
            })?;
            let matchdata_sym = interp.intern_symbol(regexp::LAST_MATCH);
            unsafe {
                sys::mrb_gv_set(mrb, matchdata_sym, matchdata.inner());
            }
            Ok(true)
        } else {
            let pre_match_sym = interp.intern_symbol(regexp::STRING_LEFT_OF_MATCH);
            let post_match_sym = interp.intern_symbol(regexp::STRING_RIGHT_OF_MATCH);
            let nil = interp.convert(None::<Value>).inner();
            unsafe {
                sys::mrb_gv_set(mrb, pre_match_sym, nil);
                sys::mrb_gv_set(mrb, post_match_sym, nil);
            }
            Ok(false)
        }
    }

    fn is_match(
        &self,
        interp: &Artichoke,
        pattern: &[u8],
        pos: Option<Int>,
    ) -> Result<bool, Exception> {
        let pattern = str::from_utf8(pattern).map_err(|_| {
            ArgumentError::new(
                interp,
                "Oniguruma-backed Regexp only supports UTF-8 patterns",
            )
        })?;
        let pattern_char_len = pattern.chars().count();
        let pos = pos.unwrap_or_default();
        let pos = if pos < 0 {
            let pos = usize::try_from(-pos).map_err(|_| {
                Fatal::new(interp, "Expected positive position to convert to usize")
            })?;
            if let Some(pos) = pattern_char_len.checked_sub(pos) {
                pos
            } else {
                return Ok(false);
            }
        } else {
            usize::try_from(pos)
                .map_err(|_| Fatal::new(interp, "Expected positive position to convert to usize"))?
        };
        // onig will panic if pos is beyond the end of string
        if pos > pattern_char_len {
            return Ok(false);
        }
        let byte_offset = pattern.chars().take(pos).map(char::len_utf8).sum();

        let match_target = &pattern[byte_offset..];
        Ok(self.regex.find(match_target).is_some())
    }

    fn match_(
        &self,
        interp: &mut Artichoke,
        pattern: &[u8],
        pos: Option<Int>,
        block: Option<Block>,
    ) -> Result<Value, Exception> {
        let mrb = interp.0.borrow().mrb;
        let pattern = str::from_utf8(pattern).map_err(|_| {
            ArgumentError::new(
                interp,
                "Oniguruma-backed Regexp only supports UTF-8 patterns",
            )
        })?;
        let pattern_char_len = pattern.chars().count();
        let pos = pos.unwrap_or_default();
        let pos = if pos < 0 {
            let pos = usize::try_from(-pos).map_err(|_| {
                Fatal::new(interp, "Expected positive position to convert to usize")
            })?;
            if let Some(pos) = pattern_char_len.checked_sub(pos) {
                pos
            } else {
                return Ok(interp.convert(None::<Value>));
            }
        } else {
            usize::try_from(pos)
                .map_err(|_| Fatal::new(interp, "Expected positive position to convert to usize"))?
        };
        // onig will panic if pos is beyond the end of string
        if pos > pattern_char_len {
            return Ok(interp.convert(None::<Value>));
        }
        let byte_offset = pattern.chars().take(pos).map(char::len_utf8).sum();

        let match_target = &pattern[byte_offset..];
        if let Some(captures) = self.regex.captures(match_target) {
            let globals_to_set = cmp::max(interp.0.borrow().active_regexp_globals, captures.len());
            let sym = interp.intern_symbol(regexp::LAST_MATCHED_STRING);
            let value = interp.convert_mut(captures.at(0));
            unsafe {
                sys::mrb_gv_set(mrb, sym, value.inner());
            }
            for group in 1..=globals_to_set {
                let sym = interp.intern_symbol(regexp::nth_match_group(group));
                let value = interp.convert_mut(captures.at(group));
                unsafe {
                    sys::mrb_gv_set(mrb, sym, value.inner());
                }
            }
            interp.0.borrow_mut().active_regexp_globals = captures.len();

            let mut matchdata = MatchData::new(
                pattern.as_bytes().to_vec(),
                Regexp::from(self.box_clone()),
                0,
                pattern.len(),
            );
            if let Some(match_pos) = captures.pos(0) {
                let pre_match = interp.convert_mut(&match_target[..match_pos.0]);
                let post_match = interp.convert_mut(&match_target[match_pos.1..]);
                let pre_match_sym = interp.intern_symbol(regexp::STRING_LEFT_OF_MATCH);
                let post_match_sym = interp.intern_symbol(regexp::STRING_RIGHT_OF_MATCH);
                unsafe {
                    sys::mrb_gv_set(mrb, pre_match_sym, pre_match.inner());
                    sys::mrb_gv_set(mrb, post_match_sym, post_match.inner());
                }
                matchdata.set_region(byte_offset + match_pos.0, byte_offset + match_pos.1);
            }
            let data = matchdata.try_into_ruby(interp, None).map_err(|_| {
                Fatal::new(
                    interp,
                    "Failed to initialize Ruby MatchData Value with Rust MatchData",
                )
            })?;
            let matchdata_sym = interp.intern_symbol(regexp::LAST_MATCH);
            unsafe {
                sys::mrb_gv_set(mrb, matchdata_sym, data.inner());
            }
            if let Some(block) = block {
                let result = block.yield_arg(interp, &data)?;
                Ok(result)
            } else {
                Ok(data)
            }
        } else {
            let last_match_sym = interp.intern_symbol(regexp::LAST_MATCH);
            let pre_match_sym = interp.intern_symbol(regexp::STRING_LEFT_OF_MATCH);
            let post_match_sym = interp.intern_symbol(regexp::STRING_RIGHT_OF_MATCH);
            let nil = interp.convert(None::<Value>).inner();
            unsafe {
                sys::mrb_gv_set(mrb, last_match_sym, nil);
                sys::mrb_gv_set(mrb, pre_match_sym, nil);
                sys::mrb_gv_set(mrb, post_match_sym, nil);
            }
            Ok(interp.convert(None::<Value>))
        }
    }

    fn match_operator(
        &self,
        interp: &mut Artichoke,
        pattern: &[u8],
    ) -> Result<Option<Int>, Exception> {
        let mrb = interp.0.borrow().mrb;
        let pattern = str::from_utf8(pattern).map_err(|_| {
            ArgumentError::new(
                interp,
                "Oniguruma-backed Regexp only supports UTF-8 patterns",
            )
        })?;
        if let Some(captures) = self.regex.captures(pattern) {
            let globals_to_set = cmp::max(interp.0.borrow().active_regexp_globals, captures.len());
            let sym = interp.intern_symbol(regexp::LAST_MATCHED_STRING);
            let value = interp.convert_mut(captures.at(0));
            unsafe {
                sys::mrb_gv_set(mrb, sym, value.inner());
            }
            for group in 1..=globals_to_set {
                let sym = interp.intern_symbol(regexp::nth_match_group(group));
                let value = interp.convert_mut(captures.at(group));
                unsafe {
                    sys::mrb_gv_set(mrb, sym, value.inner());
                }
            }
            interp.0.borrow_mut().active_regexp_globals = captures.len();

            let matchdata = MatchData::new(
                pattern.as_bytes().to_vec(),
                Regexp::from(self.box_clone()),
                0,
                pattern.len(),
            );
            let matchdata = matchdata.try_into_ruby(interp, None).map_err(|_| {
                Fatal::new(
                    interp,
                    "Failed to initialize Ruby MatchData Value with Rust MatchData",
                )
            })?;
            let matchdata_sym = interp.intern_symbol(regexp::LAST_MATCH);
            unsafe {
                sys::mrb_gv_set(mrb, matchdata_sym, matchdata.inner());
            }
            if let Some(match_pos) = captures.pos(0) {
                let pre_match = interp.convert_mut(&pattern[..match_pos.0]);
                let post_match = interp.convert_mut(&pattern[match_pos.1..]);
                let pre_match_sym = interp.intern_symbol(regexp::STRING_LEFT_OF_MATCH);
                let post_match_sym = interp.intern_symbol(regexp::STRING_RIGHT_OF_MATCH);
                unsafe {
                    sys::mrb_gv_set(mrb, pre_match_sym, pre_match.inner());
                    sys::mrb_gv_set(mrb, post_match_sym, post_match.inner());
                }
                let pos = Int::try_from(match_pos.0).map_err(|_| {
                    Fatal::new(interp, "Match position does not fit in Integer max")
                })?;
                Ok(Some(pos))
            } else {
                Ok(Some(0))
            }
        } else {
            let last_match_sym = interp.intern_symbol(regexp::LAST_MATCH);
            let pre_match_sym = interp.intern_symbol(regexp::STRING_LEFT_OF_MATCH);
            let post_match_sym = interp.intern_symbol(regexp::STRING_RIGHT_OF_MATCH);
            let nil = interp.convert(None::<Value>).inner();
            unsafe {
                sys::mrb_gv_set(mrb, last_match_sym, nil);
                sys::mrb_gv_set(mrb, pre_match_sym, nil);
                sys::mrb_gv_set(mrb, post_match_sym, nil);
            }
            Ok(None)
        }
    }

    fn named_captures(&self, interp: &Artichoke) -> Result<HashOfStringToArrayOfInt, Exception> {
        // Use a Vec of key-value pairs because insertion order matters for spec
        // compliance.
        let mut map = vec![];
        let mut fatal = false;
        self.regex.foreach_name(|group, group_indexes| {
            let mut indexes = vec![];
            for idx in group_indexes {
                if let Ok(idx) = Int::try_from(*idx) {
                    indexes.push(idx);
                } else {
                    fatal = true;
                    break;
                }
            }
            map.push((group.as_bytes().to_owned(), indexes));
            !fatal
        });
        if fatal {
            Err(Exception::from(Fatal::new(
                interp,
                "Regexp#named_captures group index does not fit in Integer max",
            )))
        } else {
            Ok(map)
        }
    }

    fn named_captures_for_haystack(
        &self,
        interp: &Artichoke,
        haystack: &[u8],
    ) -> Result<Option<HashMap<Vec<u8>, NilableString>>, Exception> {
        let haystack = str::from_utf8(haystack).map_err(|_| {
            ArgumentError::new(
                interp,
                "Oniguruma-backed Regexp only supports UTF-8 haystacks",
            )
        })?;
        if let Some(captures) = self.regex.captures(haystack) {
            let mut map = HashMap::with_capacity(captures.len());
            self.regex.foreach_name(|group, group_indexes| {
                let capture = group_indexes.iter().rev().copied().find_map(|index| {
                    let index = usize::try_from(index).unwrap_or_default();
                    captures.at(index)
                });
                if let Some(capture) = capture {
                    map.insert(group.as_bytes().to_vec(), Some(capture.as_bytes().to_vec()));
                } else {
                    map.insert(group.as_bytes().to_vec(), None);
                }
                true
            });
            Ok(Some(map))
        } else {
            Ok(None)
        }
    }

    fn names(&self, interp: &Artichoke) -> Vec<Vec<u8>> {
        let _ = interp;
        let mut names = vec![];
        let mut capture_names = vec![];
        self.regex.foreach_name(|group, group_indexes| {
            capture_names.push((group.as_bytes().to_owned(), group_indexes.to_vec()));
            true
        });
        capture_names.sort_by(|left, right| {
            let left = left.1.iter().copied().fold(u32::max_value(), u32::min);
            let right = right.1.iter().copied().fold(u32::max_value(), u32::min);
            left.partial_cmp(&right).unwrap_or(Ordering::Equal)
        });
        for (name, _) in capture_names {
            if !names.contains(&name) {
                names.push(name);
            }
        }
        names
    }

    fn pos(
        &self,
        interp: &Artichoke,
        haystack: &[u8],
        at: usize,
    ) -> Result<Option<(usize, usize)>, Exception> {
        let haystack = str::from_utf8(haystack).map_err(|_| {
            ArgumentError::new(
                interp,
                "Oniguruma-backed Regexp only supports UTF-8 haystacks",
            )
        })?;
        let pos = self
            .regex
            .captures(haystack)
            .and_then(|captures| captures.pos(at));
        Ok(pos)
    }

    fn scan(
        &self,
        interp: &mut Artichoke,
        value: Value,
        block: Option<Block>,
    ) -> Result<Value, Exception> {
        let haystack = if let Ok(haystack) = value.clone().try_into::<&[u8]>() {
            haystack
        } else {
            return Err(Exception::from(ArgumentError::new(
                interp,
                "Regexp scan expected String haystack",
            )));
        };
        let haystack = str::from_utf8(haystack).map_err(|_| {
            ArgumentError::new(
                interp,
                "Oniguruma-backed Regexp only supports UTF-8 haystacks",
            )
        })?;
        let mrb = interp.0.borrow().mrb;
        let last_match_sym = interp.intern_symbol(regexp::LAST_MATCH);
        let mut matchdata = MatchData::new(
            haystack.as_bytes().to_vec(),
            Regexp::from(self.box_clone()),
            0,
            haystack.len(),
        );

        let len = self.regex.captures_len();
        if let Some(block) = block {
            if len > 0 {
                // zero old globals
                let globals = interp.0.borrow().active_regexp_globals;
                let nil = interp.convert(None::<Value>).inner();
                for group in 1..=globals {
                    let sym = interp.intern_symbol(regexp::nth_match_group(group));
                    unsafe {
                        sys::mrb_gv_set(mrb, sym, nil);
                    }
                }
                interp.0.borrow_mut().active_regexp_globals = len;
                let mut iter = self.regex.captures_iter(haystack).peekable();
                if iter.peek().is_none() {
                    unsafe {
                        sys::mrb_gv_set(mrb, last_match_sym, nil);
                    }
                    return Ok(value);
                }
                for captures in iter {
                    let fullmatch = interp.intern_symbol(regexp::LAST_MATCHED_STRING);
                    let fullcapture = interp.convert_mut(captures.at(0));
                    unsafe {
                        sys::mrb_gv_set(mrb, fullmatch, fullcapture.inner());
                    }
                    let mut groups = vec![];
                    for group in 1..=len {
                        let sym = interp.intern_symbol(regexp::nth_match_group(group));
                        let capture = interp.convert_mut(captures.at(group));
                        groups.push(captures.at(group));
                        unsafe {
                            sys::mrb_gv_set(mrb, sym, capture.inner());
                        }
                    }

                    let matched = interp.convert_mut(groups);
                    if let Some(pos) = captures.pos(0) {
                        matchdata.set_region(pos.0, pos.1);
                    }
                    let data = matchdata.clone().try_into_ruby(interp, None).map_err(|_| {
                        Fatal::new(interp, "Failed to convert MatchData to Ruby Value")
                    })?;
                    unsafe {
                        sys::mrb_gv_set(mrb, last_match_sym, data.inner());
                    }
                    let _ = block.yield_arg::<Value>(interp, &matched)?;
                    unsafe {
                        sys::mrb_gv_set(mrb, last_match_sym, data.inner());
                    }
                }
            } else {
                let mut iter = self.regex.find_iter(haystack).peekable();
                if iter.peek().is_none() {
                    unsafe {
                        sys::mrb_gv_set(mrb, last_match_sym, interp.convert(None::<Value>).inner());
                    }
                    return Ok(value);
                }
                for pos in iter {
                    let scanned = &haystack[pos.0..pos.1];
                    let matched = interp.convert_mut(scanned);
                    matchdata.set_region(pos.0, pos.1);
                    let data = matchdata.clone().try_into_ruby(interp, None).map_err(|_| {
                        Fatal::new(interp, "Failed to convert MatchData to Ruby Value")
                    })?;
                    unsafe {
                        sys::mrb_gv_set(mrb, last_match_sym, data.inner());
                    }
                    let _ = block.yield_arg::<Value>(interp, &matched)?;
                    unsafe {
                        sys::mrb_gv_set(mrb, last_match_sym, data.inner());
                    }
                }
            }
            Ok(value)
        } else {
            let mut last_pos = (0, 0);
            if len > 0 {
                let mut collected = vec![];
                // zero old globals
                let globals = interp.0.borrow().active_regexp_globals;
                let nil = interp.convert(None::<Value>).inner();
                for group in 1..=globals {
                    let sym = interp.intern_symbol(regexp::nth_match_group(group));
                    unsafe {
                        sys::mrb_gv_set(mrb, sym, nil);
                    }
                }
                interp.0.borrow_mut().active_regexp_globals = len;
                let mut iter = self.regex.captures_iter(haystack).peekable();
                if iter.peek().is_none() {
                    unsafe {
                        sys::mrb_gv_set(mrb, last_match_sym, nil);
                    }
                    return Ok(interp.convert_mut(&[] as &[Value]));
                }
                for captures in iter {
                    let mut groups = vec![];
                    for group in 1..=len {
                        groups.push(captures.at(group));
                    }

                    if let Some(pos) = captures.pos(0) {
                        last_pos = pos;
                    }
                    collected.push(groups);
                }
                matchdata.set_region(last_pos.0, last_pos.1);
                let data = matchdata
                    .try_into_ruby(interp, None)
                    .map_err(|_| Fatal::new(interp, "Failed to convert MatchData to Ruby Value"))?;
                unsafe {
                    sys::mrb_gv_set(mrb, last_match_sym, data.inner());
                }
                let mut iter = collected.iter().enumerate();
                if let Some((_, fullcapture)) = iter.next() {
                    let fullmatch = interp.intern_symbol(regexp::LAST_MATCHED_STRING);
                    let fullcapture = interp.convert_mut(fullcapture.as_slice());
                    unsafe {
                        sys::mrb_gv_set(mrb, fullmatch, fullcapture.inner());
                    }
                }
                for (group, capture) in iter {
                    let sym = interp.intern_symbol(regexp::nth_match_group(group));
                    let capture = interp.convert_mut(capture.as_slice());
                    unsafe {
                        sys::mrb_gv_set(mrb, sym, capture.inner());
                    }
                }
                Ok(interp.convert_mut(collected))
            } else {
                let mut collected = vec![];
                let mut iter = self.regex.find_iter(haystack).peekable();
                if iter.peek().is_none() {
                    unsafe {
                        sys::mrb_gv_set(mrb, last_match_sym, interp.convert(None::<Value>).inner());
                    }
                    return Ok(interp.convert_mut(&[] as &[Value]));
                }
                for pos in iter {
                    let scanned = &haystack[pos.0..pos.1];
                    last_pos = pos;
                    collected.push(scanned);
                }
                matchdata.set_region(last_pos.0, last_pos.1);
                let data = matchdata
                    .try_into_ruby(interp, None)
                    .map_err(|_| Fatal::new(interp, "Failed to convert MatchData to Ruby Value"))?;
                unsafe {
                    sys::mrb_gv_set(mrb, last_match_sym, data.inner());
                }
                if let Some(fullcapture) = collected.last().copied() {
                    let fullmatch = interp.intern_symbol(regexp::LAST_MATCHED_STRING);
                    let fullcapture = interp.convert_mut(fullcapture);
                    unsafe {
                        sys::mrb_gv_set(mrb, fullmatch, fullcapture.inner());
                    }
                }
                Ok(interp.convert_mut(collected))
            }
        }
    }
}
