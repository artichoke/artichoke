//! An implementation of [`MatchData`][matchdata] for all [`Regexp`] backends.
//!
//! `MatchData` is mostly implemented in Rust with some methods implemented in
//! Ruby. `MatchData` lazily computes matches by delegating to its underlying
//! [`Regexp`] instance on access.
//!
//! `MatchData` passes all non-skipped [ruby/spec][rubyspec]s.
//!
//! [matchdata]: https://ruby-doc.org/core-3.1.2/MatchData.html
//! [rubyspec]: https://github.com/ruby/spec

use std::collections::HashMap;
use std::fmt::Write as _;
use std::ops::{Bound, RangeBounds};
use std::str;

use bstr::BString;
use scolapasta_string_escape::format_debug_escape_into;

use crate::convert::{implicitly_convert_to_int, implicitly_convert_to_string};
use crate::extn::core::regexp::backend::NilableString;
use crate::extn::core::regexp::Regexp;
use crate::extn::core::symbol::Symbol;
use crate::extn::prelude::*;
use crate::fmt::WriteError;

mod boxing;
pub(in crate::extn) mod mruby;
pub(super) mod trampoline;

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub struct Region {
    start: Bound<usize>,
    end: Bound<usize>,
}

impl Region {
    fn from_range<R>(bounds: R) -> Self
    where
        R: RangeBounds<usize>,
    {
        let start = bounds.start_bound().cloned();
        let end = bounds.end_bound().cloned();
        Region { start, end }
    }

    fn offset(&self) -> usize {
        match self.start {
            Bound::Included(bound) => bound,
            Bound::Excluded(bound) => bound.checked_sub(1).unwrap_or_default(),
            Bound::Unbounded => 0,
        }
    }
}

impl RangeBounds<usize> for Region {
    fn start_bound(&self) -> Bound<&usize> {
        // TODO: Use `self.start.as_ref()` when upstream `std` stabilizes:
        // https://github.com/rust-lang/rust/issues/80996
        match self.start {
            Bound::Included(ref bound) => Bound::Included(bound),
            Bound::Excluded(ref bound) => Bound::Excluded(bound),
            Bound::Unbounded => Bound::Unbounded,
        }
    }

    fn end_bound(&self) -> Bound<&usize> {
        // TODO: Use `self.end.as_ref()` when upstream `std` stabilizes:
        // https://github.com/rust-lang/rust/issues/80996
        match self.end {
            Bound::Included(ref bound) => Bound::Included(bound),
            Bound::Excluded(ref bound) => Bound::Excluded(bound),
            Bound::Unbounded => Bound::Unbounded,
        }
    }
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub enum Capture<'a> {
    GroupIndex(i64),
    GroupName(&'a [u8]),
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub enum CaptureExtract<'a> {
    GroupIndex(i64),
    GroupName(&'a [u8]),
    Symbol(Symbol),
}

impl<'a> TryConvertMut<&'a mut Value, CaptureExtract<'a>> for Artichoke {
    type Error = Error;

    fn try_convert_mut(&mut self, value: &'a mut Value) -> Result<CaptureExtract<'a>, Self::Error> {
        if let Ok(idx) = implicitly_convert_to_int(self, *value) {
            Ok(CaptureExtract::GroupIndex(idx))
        } else if let Ok(symbol) = unsafe { Symbol::unbox_from_value(value, self) } {
            let sym = symbol.id();
            Ok(CaptureExtract::Symbol(sym.into()))
        } else {
            let name = unsafe { implicitly_convert_to_string(self, value)? };
            Ok(CaptureExtract::GroupName(name))
        }
    }
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub enum CaptureAt<'a> {
    GroupIndex(i64),
    GroupName(&'a [u8]),
    StartLen(i64, i64),
}

#[derive(Debug, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub enum CaptureMatch {
    None,
    Single(Option<Vec<u8>>),
    Range(Vec<Option<Vec<u8>>>),
}

impl TryConvertMut<CaptureMatch, Value> for Artichoke {
    type Error = Error;

    fn try_convert_mut(&mut self, value: CaptureMatch) -> Result<Value, Self::Error> {
        match value {
            CaptureMatch::None => Ok(Value::nil()),
            CaptureMatch::Single(capture) => self.try_convert_mut(capture),
            CaptureMatch::Range(captures) => self.try_convert_mut(captures),
        }
    }
}

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub struct MatchData {
    haystack: BString,
    regexp: Regexp,
    region: Region,
}

impl MatchData {
    #[must_use]
    pub fn new<R>(haystack: Vec<u8>, regexp: Regexp, bounds: R) -> Self
    where
        R: RangeBounds<usize>,
    {
        let region = Region::from_range(bounds);
        Self {
            haystack: haystack.into(),
            regexp,
            region,
        }
    }

    pub fn set_region<R>(&mut self, bounds: R)
    where
        R: RangeBounds<usize>,
    {
        self.region = Region::from_range(bounds);
    }

    #[must_use]
    pub fn matched_region(&self) -> &[u8] {
        let matched = match (self.region.start, self.region.end) {
            (Bound::Included(start), Bound::Included(end)) => self.haystack.get(start..=end),
            (Bound::Included(start), Bound::Excluded(end)) => self.haystack.get(start..end),
            (Bound::Included(start), Bound::Unbounded) => self.haystack.get(start..),
            (Bound::Excluded(start), Bound::Included(end)) => self.haystack.get((start + 1)..=end),
            (Bound::Excluded(start), Bound::Excluded(end)) => self.haystack.get((start + 1)..end),
            (Bound::Excluded(start), Bound::Unbounded) => self.haystack.get(start + 1..),
            (Bound::Unbounded, Bound::Included(end)) => self.haystack.get(..=end),
            (Bound::Unbounded, Bound::Excluded(end)) => self.haystack.get(..end),
            (Bound::Unbounded, Bound::Unbounded) => self.haystack.get(..),
        };
        matched.unwrap_or_default()
    }

    #[inline]
    pub fn begin(&self, capture: Capture<'_>) -> Result<Option<usize>, Error> {
        if let Some([begin, _]) = self.offset(capture)? {
            Ok(Some(begin))
        } else {
            Ok(None)
        }
    }

    pub fn capture_at(&self, at: CaptureAt<'_>) -> Result<CaptureMatch, Error> {
        let haystack = self.matched_region();
        let captures = if let Some(captures) = self.regexp.inner().captures(haystack)? {
            captures
        } else {
            return Ok(CaptureMatch::None);
        };
        match at {
            CaptureAt::GroupIndex(index) => match aref::offset_to_index(index, captures.len()) {
                None => Ok(CaptureMatch::None),
                Some(idx) => {
                    if let Some(capture) = captures.into_iter().nth(idx) {
                        Ok(CaptureMatch::Single(capture))
                    } else {
                        Ok(CaptureMatch::None)
                    }
                }
            },
            CaptureAt::GroupName(name) => {
                let indexes = self.regexp.inner().capture_indexes_for_name(name)?;
                if let Some(indexes) = indexes {
                    let capture = indexes
                        .iter()
                        .copied()
                        .filter_map(|index| captures.get(index).and_then(Option::as_deref))
                        .last();
                    Ok(CaptureMatch::Single(capture.map(<[_]>::to_vec)))
                } else {
                    let mut message = String::from("undefined group name reference: \"");
                    format_debug_escape_into(&mut message, name).map_err(WriteError::from)?;
                    message.push('"');
                    Err(IndexError::from(message).into())
                }
            }
            CaptureAt::StartLen(start, len) => {
                if let Ok(len) = usize::try_from(len) {
                    let start = if let Some(start) = aref::offset_to_index(start, captures.len()) {
                        start
                    } else {
                        return Ok(CaptureMatch::None);
                    };
                    let matches = captures.into_iter().skip(start).take(len).collect::<Vec<_>>();
                    Ok(CaptureMatch::Range(matches))
                } else {
                    Ok(CaptureMatch::None)
                }
            }
        }
    }

    pub fn captures(&self) -> Result<Option<Vec<Option<Vec<u8>>>>, Error> {
        let haystack = self.matched_region();
        let captures = self.regexp.inner().captures(haystack)?;
        if let Some(mut captures) = captures {
            // Panic safety:
            //
            // All Regexp matches are guaranteed to have a zero capture group.
            captures.remove(0);
            Ok(Some(captures))
        } else {
            Ok(None)
        }
    }

    #[inline]
    pub fn end(&self, capture: Capture<'_>) -> Result<Option<usize>, Error> {
        if let Some([_, end]) = self.offset(capture)? {
            Ok(Some(end))
        } else {
            Ok(None)
        }
    }

    #[inline]
    pub fn len(&self) -> Result<usize, Error> {
        let haystack = self.matched_region();
        self.regexp.inner().captures_len(Some(haystack))
    }

    #[inline]
    pub fn named_captures(&self) -> Result<Option<HashMap<Vec<u8>, NilableString>>, Error> {
        let haystack = self.matched_region();
        self.regexp.inner().named_captures_for_haystack(haystack)
    }

    #[inline]
    #[must_use]
    pub fn names(&self) -> Vec<Vec<u8>> {
        self.regexp.names()
    }

    pub fn offset(&self, capture: Capture<'_>) -> Result<Option<[usize; 2]>, Error> {
        let haystack = self.matched_region();
        let index = match capture {
            Capture::GroupIndex(index) => {
                let captures_len = self.regexp.inner().captures_len(Some(haystack))?;
                match usize::try_from(index) {
                    Ok(idx) if idx < captures_len => idx,
                    _ => {
                        let mut message = String::new();
                        write!(&mut message, "index {} out of matches", index).map_err(WriteError::from)?;
                        return Err(IndexError::from(message).into());
                    }
                }
            }
            Capture::GroupName(name) => {
                let indexes = self.regexp.inner().capture_indexes_for_name(name)?;
                if let Some(index) = indexes.and_then(|indexes| indexes.last().copied()) {
                    index
                } else {
                    return Ok(None);
                }
            }
        };
        if let Some((begin, end)) = self.regexp.inner().pos(haystack, index)? {
            let begin = if let Some(Ok(haystack)) = haystack.get(..begin).map(str::from_utf8) {
                haystack.chars().count()
            } else {
                haystack.len()
            };
            let end = if let Some(Ok(haystack)) = haystack.get(..end).map(str::from_utf8) {
                haystack.chars().count()
            } else {
                haystack.len()
            };
            let offset = self.region.offset();
            Ok(Some([offset + begin, offset + end]))
        } else {
            Ok(None)
        }
    }

    #[must_use]
    pub fn pre(&self) -> &[u8] {
        let pre = match self.region.start {
            Bound::Included(start) => self.haystack.get(..start),
            Bound::Excluded(start) => self.haystack.get(..=start),
            Bound::Unbounded => return &[],
        };
        pre.unwrap_or_else(|| {
            // if start is out of range, the whole haystack is the pre match
            self.haystack.as_slice()
        })
    }

    #[must_use]
    pub fn post(&self) -> &[u8] {
        let post = match self.region.end {
            Bound::Included(end) => self.haystack.get(end + 1..),
            Bound::Excluded(end) => self.haystack.get(end..),
            Bound::Unbounded => return &[],
        };
        // if end is out of range, there is no post match
        post.unwrap_or_default()
    }

    #[inline]
    #[must_use]
    pub fn regexp(&self) -> &Regexp {
        &self.regexp
    }

    #[inline]
    #[must_use]
    pub fn string(&self) -> &[u8] {
        self.haystack.as_slice()
    }

    #[inline]
    pub fn to_a(&self) -> Result<Option<Vec<NilableString>>, Error> {
        let haystack = self.matched_region();
        self.regexp.inner().captures(haystack)
    }

    #[inline]
    pub fn to_s(&self) -> Result<Option<&[u8]>, Error> {
        let haystack = self.matched_region();
        self.regexp.inner().capture0(haystack)
    }
}
