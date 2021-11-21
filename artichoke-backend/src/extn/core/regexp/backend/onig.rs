use core::mem::size_of;
use std::collections::HashMap;
use std::fmt;
use std::num::NonZeroUsize;
use std::rc::Rc;
use std::str;

use onig::{Regex, Syntax};

use super::{NameToCaptureLocations, NilableString};
use crate::extn::core::matchdata::MatchData;
use crate::extn::core::regexp::{self, Config, Encoding, Regexp, RegexpType, Scan, Source};
use crate::extn::prelude::*;

// The Oniguruma `Regexp` backend requires that `u32` can be widened to `usize`
// losslessly.
//
// This const-evaluated expression ensures that `usize` is always at least as
// wide as `usize`.
const _: () = [()][!(size_of::<usize>() >= size_of::<u32>()) as usize];

#[derive(Debug, Clone)]
pub struct Onig {
    source: Source,
    config: Config,
    encoding: Encoding,
    regex: Rc<Regex>,
}

impl Onig {
    pub fn new(source: Source, config: Config, encoding: Encoding) -> Result<Self, Error> {
        let pattern = str::from_utf8(config.pattern())
            .map_err(|_| ArgumentError::with_message("Oniguruma backend for Regexp only supports UTF-8 patterns"))?;
        let regex = match Regex::with_options(pattern, config.options().into(), Syntax::ruby()) {
            Ok(regex) => regex,
            Err(err) if source.is_literal() => return Err(SyntaxError::from(err.description().to_owned()).into()),
            Err(err) => return Err(RegexpError::from(err.description().to_owned()).into()),
        };
        let regexp = Self {
            source,
            config,
            encoding,
            regex: Rc::new(regex),
        };
        Ok(regexp)
    }
}

impl fmt::Display for Onig {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let pattern = self.config.pattern();
        format_unicode_debug_into(f, pattern).map_err(WriteError::into_inner)
    }
}

impl RegexpType for Onig {
    fn box_clone(&self) -> Box<dyn RegexpType> {
        Box::new(self.clone())
    }

    fn captures(&self, haystack: &[u8]) -> Result<Option<Vec<NilableString>>, Error> {
        let haystack = str::from_utf8(haystack)
            .map_err(|_| ArgumentError::with_message("Oniguruma backend for Regexp only supports UTF-8 haystacks"))?;
        if let Some(captures) = self.regex.captures(haystack) {
            let mut result = Vec::with_capacity(captures.len());
            for capture in captures.iter() {
                if let Some(capture) = capture {
                    result.push(Some(capture.into()));
                } else {
                    result.push(None);
                }
            }
            Ok(Some(result))
        } else {
            Ok(None)
        }
    }

    fn capture_indexes_for_name(&self, name: &[u8]) -> Result<Option<Vec<usize>>, Error> {
        let mut result = None;
        self.regex.foreach_name(|group, group_indexes| {
            if name != group.as_bytes() {
                // Continue searching through named captures.
                return true;
            }
            let mut indexes = Vec::with_capacity(group_indexes.len());
            for &index in group_indexes {
                indexes.push(index as usize);
            }
            result = Some(indexes);
            false
        });
        Ok(result)
    }

    fn captures_len(&self, haystack: Option<&[u8]>) -> Result<usize, Error> {
        let result = if let Some(haystack) = haystack {
            let haystack = str::from_utf8(haystack).map_err(|_| {
                ArgumentError::with_message("Oniguruma backend for Regexp only supports UTF-8 haystacks")
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

    fn capture0<'a>(&self, haystack: &'a [u8]) -> Result<Option<&'a [u8]>, Error> {
        let haystack = str::from_utf8(haystack)
            .map_err(|_| ArgumentError::with_message("Oniguruma backend for Regexp only supports UTF-8 haystacks"))?;
        let result = self
            .regex
            .captures(haystack)
            .and_then(|captures| captures.at(0))
            .map(str::as_bytes);
        Ok(result)
    }

    fn debug(&self) -> String {
        let mut debug = String::from("/");
        let mut pattern = String::new();
        // Explicitly suppress this error because `debug` is infallible and
        // cannot panic.
        //
        // In practice this error will never be triggered since the only
        // fallible call in `format_unicode_debug_into` is to `write!` which
        // never `panic!`s for a `String` formatter, which we are using here.
        let _ = format_unicode_debug_into(&mut pattern, self.source.pattern());
        debug.push_str(pattern.replace("/", r"\/").as_str());
        debug.push('/');
        debug.push_str(self.source.options().as_display_modifier());
        debug.push_str(self.encoding.as_modifier_str());
        debug
    }

    fn source(&self) -> &Source {
        &self.source
    }

    fn config(&self) -> &Config {
        &self.config
    }

    fn encoding(&self) -> &Encoding {
        &self.encoding
    }

    fn inspect(&self) -> Vec<u8> {
        // pattern length + 2x '/' + mix + encoding
        let mut inspect = Vec::with_capacity(self.source.pattern().len() + 2 + 4);
        inspect.push(b'/');
        if let Ok(pat) = str::from_utf8(self.source.pattern()) {
            inspect.extend_from_slice(pat.replace("/", r"\/").as_bytes());
        } else {
            inspect.extend_from_slice(self.source.pattern());
        }
        inspect.push(b'/');
        inspect.extend_from_slice(self.source.options().as_display_modifier().as_bytes());
        inspect.extend_from_slice(self.encoding.as_modifier_str().as_bytes());
        inspect
    }

    fn string(&self) -> &[u8] {
        self.config.pattern()
    }

    fn case_match(&self, interp: &mut Artichoke, haystack: &[u8]) -> Result<bool, Error> {
        let haystack = str::from_utf8(haystack)
            .map_err(|_| ArgumentError::with_message("Oniguruma backend for Regexp only supports UTF-8 haystacks"))?;
        regexp::clear_capture_globals(interp)?;
        if let Some(captures) = self.regex.captures(haystack) {
            interp.set_active_regexp_globals(captures.len())?;
            let value = interp.try_convert_mut(captures.at(0))?;
            interp.set_global_variable(regexp::LAST_MATCHED_STRING, &value)?;

            for group in 0..captures.len() {
                let value = interp.try_convert_mut(captures.at(group))?;
                let group = unsafe { NonZeroUsize::new_unchecked(1 + group) };
                interp.set_global_variable(regexp::nth_match_group(group), &value)?;
            }

            if let Some(match_pos) = captures.pos(0) {
                let pre_match = interp.try_convert_mut(&haystack[..match_pos.0])?;
                let post_match = interp.try_convert_mut(&haystack[match_pos.1..])?;
                interp.set_global_variable(regexp::STRING_LEFT_OF_MATCH, &pre_match)?;
                interp.set_global_variable(regexp::STRING_RIGHT_OF_MATCH, &post_match)?;
            }
            let matchdata = MatchData::new(haystack.into(), Regexp::from(self.box_clone()), ..);
            let matchdata = MatchData::alloc_value(matchdata, interp)?;
            interp.set_global_variable(regexp::LAST_MATCH, &matchdata)?;
            Ok(true)
        } else {
            interp.unset_global_variable(regexp::STRING_LEFT_OF_MATCH)?;
            interp.unset_global_variable(regexp::STRING_RIGHT_OF_MATCH)?;
            Ok(false)
        }
    }

    fn is_match(&self, haystack: &[u8], pos: Option<i64>) -> Result<bool, Error> {
        let haystack = str::from_utf8(haystack)
            .map_err(|_| ArgumentError::with_message("Oniguruma backend for Regexp only supports UTF-8 haystacks"))?;
        let haystack_char_len = haystack.chars().count();
        let pos = pos.unwrap_or_default();
        let pos = if let Ok(pos) = usize::try_from(pos) {
            pos
        } else {
            let pos = pos
                .checked_neg()
                .and_then(|pos| usize::try_from(pos).ok())
                .and_then(|pos| haystack_char_len.checked_sub(pos));
            if let Some(pos) = pos {
                pos
            } else {
                return Ok(false);
            }
        };
        let offset = haystack.chars().take(pos).map(char::len_utf8).sum();
        if let Some(haystack) = haystack.get(offset..) {
            Ok(self.regex.find(haystack).is_some())
        } else {
            Ok(false)
        }
    }

    fn match_(
        &self,
        interp: &mut Artichoke,
        haystack: &[u8],
        pos: Option<i64>,
        block: Option<Block>,
    ) -> Result<Value, Error> {
        let haystack = str::from_utf8(haystack)
            .map_err(|_| ArgumentError::with_message("Oniguruma backend for Regexp only supports UTF-8 haystacks"))?;
        regexp::clear_capture_globals(interp)?;
        let haystack_char_len = haystack.chars().count();
        let pos = pos.unwrap_or_default();
        let pos = if let Ok(pos) = usize::try_from(pos) {
            pos
        } else {
            let pos = pos
                .checked_neg()
                .and_then(|pos| usize::try_from(pos).ok())
                .and_then(|pos| haystack_char_len.checked_sub(pos));
            if let Some(pos) = pos {
                pos
            } else {
                return Ok(Value::nil());
            }
        };
        let offset = haystack.chars().take(pos).map(char::len_utf8).sum();
        let target = if let Some(haystack) = haystack.get(offset..) {
            haystack
        } else {
            interp.unset_global_variable(regexp::LAST_MATCH)?;
            interp.unset_global_variable(regexp::STRING_LEFT_OF_MATCH)?;
            interp.unset_global_variable(regexp::STRING_RIGHT_OF_MATCH)?;
            return Ok(Value::nil());
        };

        if let Some(captures) = self.regex.captures(target) {
            interp.set_active_regexp_globals(captures.len())?;

            let value = interp.try_convert_mut(captures.at(0))?;
            interp.set_global_variable(regexp::LAST_MATCHED_STRING, &value)?;
            for group in 0..captures.len() {
                let value = interp.try_convert_mut(captures.at(group))?;
                let group = unsafe { NonZeroUsize::new_unchecked(1 + group) };
                interp.set_global_variable(regexp::nth_match_group(group), &value)?;
            }

            let mut matchdata = MatchData::new(haystack.into(), Regexp::from(self.box_clone()), ..);
            if let Some(match_pos) = captures.pos(0) {
                let pre_match = interp.try_convert_mut(&target[..match_pos.0])?;
                let post_match = interp.try_convert_mut(&target[match_pos.1..])?;
                interp.set_global_variable(regexp::STRING_LEFT_OF_MATCH, &pre_match)?;
                interp.set_global_variable(regexp::STRING_RIGHT_OF_MATCH, &post_match)?;
                matchdata.set_region(offset + match_pos.0..offset + match_pos.1);
            }
            let data = MatchData::alloc_value(matchdata, interp)?;
            interp.set_global_variable(regexp::LAST_MATCH, &data)?;
            if let Some(block) = block {
                let result = block.yield_arg(interp, &data)?;
                Ok(result)
            } else {
                Ok(data)
            }
        } else {
            interp.unset_global_variable(regexp::LAST_MATCH)?;
            interp.unset_global_variable(regexp::STRING_LEFT_OF_MATCH)?;
            interp.unset_global_variable(regexp::STRING_RIGHT_OF_MATCH)?;
            Ok(Value::nil())
        }
    }

    fn match_operator(&self, interp: &mut Artichoke, haystack: &[u8]) -> Result<Option<usize>, Error> {
        let haystack = str::from_utf8(haystack)
            .map_err(|_| ArgumentError::with_message("Oniguruma backend for Regexp only supports UTF-8 haystacks"))?;
        regexp::clear_capture_globals(interp)?;
        if let Some(captures) = self.regex.captures(haystack) {
            interp.set_active_regexp_globals(captures.len())?;

            let value = interp.try_convert_mut(captures.at(0))?;
            interp.set_global_variable(regexp::LAST_MATCHED_STRING, &value)?;
            for group in 0..captures.len() {
                let value = interp.try_convert_mut(captures.at(group))?;
                let group = unsafe { NonZeroUsize::new_unchecked(1 + group) };
                interp.set_global_variable(regexp::nth_match_group(group), &value)?;
            }

            let matchdata = MatchData::new(haystack.into(), Regexp::from(self.box_clone()), ..);
            let data = MatchData::alloc_value(matchdata, interp)?;
            interp.set_global_variable(regexp::LAST_MATCH, &data)?;
            if let Some(match_pos) = captures.pos(0) {
                let pre_match = interp.try_convert_mut(&haystack[..match_pos.0])?;
                let post_match = interp.try_convert_mut(&haystack[match_pos.1..])?;
                interp.set_global_variable(regexp::STRING_LEFT_OF_MATCH, &pre_match)?;
                interp.set_global_variable(regexp::STRING_RIGHT_OF_MATCH, &post_match)?;
                let pos = match_pos.0;
                Ok(Some(pos))
            } else {
                Ok(Some(0))
            }
        } else {
            interp.unset_global_variable(regexp::LAST_MATCH)?;
            interp.unset_global_variable(regexp::STRING_LEFT_OF_MATCH)?;
            interp.unset_global_variable(regexp::STRING_RIGHT_OF_MATCH)?;
            Ok(None)
        }
    }

    fn named_captures(&self) -> Result<NameToCaptureLocations, Error> {
        // Use a Vec of key-value pairs because insertion order matters for spec
        // compliance.
        let mut map = vec![];
        self.regex.foreach_name(|group, group_indexes| {
            let mut converted = Vec::with_capacity(group_indexes.len());
            for &index in group_indexes {
                converted.push(index as usize);
            }
            map.push((group.into(), converted));
            true
        });
        Ok(map)
    }

    fn named_captures_for_haystack(&self, haystack: &[u8]) -> Result<Option<HashMap<Vec<u8>, NilableString>>, Error> {
        let haystack = str::from_utf8(haystack)
            .map_err(|_| ArgumentError::with_message("Oniguruma backend for Regexp only supports UTF-8 haystacks"))?;
        if let Some(captures) = self.regex.captures(haystack) {
            let mut map = HashMap::with_capacity(captures.len());
            self.regex.foreach_name(|group, group_indexes| {
                for &index in group_indexes.iter().rev() {
                    if let Some(capture) = captures.at(index as usize) {
                        map.insert(group.into(), Some(capture.into()));
                        return true;
                    }
                }
                map.insert(group.into(), None);
                true
            });
            Ok(Some(map))
        } else {
            Ok(None)
        }
    }

    fn names(&self) -> Vec<Vec<u8>> {
        let mut names = vec![];
        let mut capture_names = vec![];
        self.regex.foreach_name(|group, group_indexes| {
            capture_names.push((group.as_bytes().to_vec(), group_indexes.to_vec()));
            true
        });
        capture_names.sort_by(|left, right| {
            let left = left.1.iter().min().copied().unwrap_or(u32::MAX);
            let right = right.1.iter().min().copied().unwrap_or(u32::MAX);
            left.cmp(&right)
        });
        for (name, _) in capture_names {
            if !names.contains(&name) {
                names.push(name);
            }
        }
        names
    }

    fn pos(&self, haystack: &[u8], at: usize) -> Result<Option<(usize, usize)>, Error> {
        let haystack = str::from_utf8(haystack)
            .map_err(|_| ArgumentError::with_message("Oniguruma backend for Regexp only supports UTF-8 haystacks"))?;
        let pos = self.regex.captures(haystack).and_then(|captures| captures.pos(at));
        Ok(pos)
    }

    fn scan(&self, interp: &mut Artichoke, haystack: &[u8], block: Option<Block>) -> Result<Scan, Error> {
        let haystack = str::from_utf8(haystack)
            .map_err(|_| ArgumentError::with_message("Oniguruma backend for Regexp only supports UTF-8 haystacks"))?;
        regexp::clear_capture_globals(interp)?;
        let mut matchdata = MatchData::new(haystack.into(), Regexp::from(self.box_clone()), ..);

        let len = NonZeroUsize::new(self.regex.captures_len());
        if let Some(block) = block {
            if let Some(len) = len {
                interp.set_active_regexp_globals(len.get())?;

                let mut iter = self.regex.captures_iter(haystack).peekable();
                if iter.peek().is_none() {
                    interp.unset_global_variable(regexp::LAST_MATCH)?;
                    return Ok(Scan::Haystack);
                }
                for captures in iter {
                    let fullcapture = interp.try_convert_mut(captures.at(0))?;
                    interp.set_global_variable(regexp::LAST_MATCHED_STRING, &fullcapture)?;

                    let mut groups = Vec::with_capacity(len.get());
                    for group in 1..=len.get() {
                        let capture = captures.at(group);
                        groups.push(capture);
                        let capture = interp.try_convert_mut(capture)?;
                        let group = unsafe { NonZeroUsize::new_unchecked(group) };
                        interp.set_global_variable(regexp::nth_match_group(group), &capture)?;
                    }

                    let matched = interp.try_convert_mut(groups)?;
                    if let Some(pos) = captures.pos(0) {
                        matchdata.set_region(pos.0..pos.1);
                    }
                    let data = MatchData::alloc_value(matchdata.clone(), interp)?;
                    interp.set_global_variable(regexp::LAST_MATCH, &data)?;
                    block.yield_arg(interp, &matched)?;
                    interp.set_global_variable(regexp::LAST_MATCH, &data)?;
                }
            } else {
                let mut iter = self.regex.find_iter(haystack).peekable();
                if iter.peek().is_none() {
                    interp.unset_global_variable(regexp::LAST_MATCH)?;
                    return Ok(Scan::Haystack);
                }
                for pos in iter {
                    let scanned = &haystack[pos.0..pos.1];
                    let matched = interp.try_convert_mut(scanned)?;
                    matchdata.set_region(pos.0..pos.1);
                    let data = MatchData::alloc_value(matchdata.clone(), interp)?;
                    interp.set_global_variable(regexp::LAST_MATCH, &data)?;
                    block.yield_arg(interp, &matched)?;
                    interp.set_global_variable(regexp::LAST_MATCH, &data)?;
                }
            }
            Ok(Scan::Haystack)
        } else {
            let mut last_pos = (0, 0);
            if let Some(len) = len {
                interp.set_active_regexp_globals(len.get())?;

                let mut collected = vec![];
                let mut iter = self.regex.captures_iter(haystack).peekable();
                if iter.peek().is_none() {
                    interp.unset_global_variable(regexp::LAST_MATCH)?;
                    return Ok(Scan::Collected(Vec::new()));
                }
                for captures in iter {
                    let mut groups = Vec::with_capacity(len.get());
                    for group in 1..=len.get() {
                        groups.push(captures.at(group).map(str::as_bytes).map(Vec::from));
                    }

                    if let Some(pos) = captures.pos(0) {
                        last_pos = pos;
                    }
                    collected.push(groups);
                }
                matchdata.set_region(last_pos.0..last_pos.1);
                let data = MatchData::alloc_value(matchdata, interp)?;
                interp.set_global_variable(regexp::LAST_MATCH, &data)?;

                let mut iter = collected.iter().enumerate();
                if let Some((_, fullcapture)) = iter.next() {
                    let fullcapture = interp.try_convert_mut(fullcapture.as_slice())?;
                    interp.set_global_variable(regexp::LAST_MATCHED_STRING, &fullcapture)?;
                }
                for (group, capture) in iter {
                    let capture = interp.try_convert_mut(capture.as_slice())?;
                    let group = unsafe { NonZeroUsize::new_unchecked(group) };
                    interp.set_global_variable(regexp::nth_match_group(group), &capture)?;
                }
                Ok(Scan::Collected(collected))
            } else {
                let mut collected = vec![];
                let mut iter = self.regex.find_iter(haystack).peekable();
                if iter.peek().is_none() {
                    interp.unset_global_variable(regexp::LAST_MATCH)?;
                    return Ok(Scan::Patterns(Vec::new()));
                }
                for pos in iter {
                    let scanned = &haystack[pos.0..pos.1];
                    last_pos = pos;
                    collected.push(Vec::from(scanned.as_bytes()));
                }
                matchdata.set_region(last_pos.0..last_pos.1);
                let data = MatchData::alloc_value(matchdata, interp)?;
                interp.set_global_variable(regexp::LAST_MATCH, &data)?;

                let last_matched = collected.last().map(Vec::as_slice);
                let last_matched = interp.try_convert_mut(last_matched)?;
                interp.set_global_variable(regexp::LAST_MATCHED_STRING, &last_matched)?;
                Ok(Scan::Patterns(collected))
            }
        }
    }
}
