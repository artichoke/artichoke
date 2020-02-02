use std::collections::HashMap;

use crate::extn::core::regexp::{Config, Encoding};
use crate::extn::prelude::*;

pub mod lazy;
pub mod onig;
// See GH-490: Add `regex::Binary` implementation of `RegexType`.
// pub mod regex;
// pub mod regex_binary;
pub mod regex_utf8;

type NilableString = Option<Vec<u8>>;
type HashOfStringToArrayOfInt = Vec<(Vec<u8>, Vec<Int>)>;

pub trait RegexpType {
    fn box_clone(&self) -> Box<dyn RegexpType>;

    fn debug(&self) -> String;

    fn literal_config(&self) -> &Config;

    fn derived_config(&self) -> &Config;

    fn encoding(&self) -> &Encoding;

    fn inspect(&self, interp: &Artichoke) -> Vec<u8>;

    fn string(&self, interp: &Artichoke) -> &[u8];

    fn captures(
        &self,
        interp: &Artichoke,
        haystack: &[u8],
    ) -> Result<Option<Vec<NilableString>>, Exception>;

    fn capture_indexes_for_name(
        &self,
        interp: &Artichoke,
        name: &[u8],
    ) -> Result<Option<Vec<usize>>, Exception>;

    fn captures_len(&self, interp: &Artichoke, haystack: Option<&[u8]>)
        -> Result<usize, Exception>;

    fn capture0<'a>(
        &self,
        interp: &Artichoke,
        haystack: &'a [u8],
    ) -> Result<Option<&'a [u8]>, Exception>;

    fn case_match(&self, interp: &mut Artichoke, pattern: &[u8]) -> Result<bool, Exception>;

    fn is_match(
        &self,
        interp: &Artichoke,
        pattern: &[u8],
        pos: Option<Int>,
    ) -> Result<bool, Exception>;

    fn match_(
        &self,
        interp: &mut Artichoke,
        pattern: &[u8],
        pos: Option<Int>,
        block: Option<Block>,
    ) -> Result<Value, Exception>;

    fn match_operator(
        &self,
        interp: &mut Artichoke,
        pattern: &[u8],
    ) -> Result<Option<Int>, Exception>;

    fn named_captures(&self, interp: &Artichoke) -> Result<HashOfStringToArrayOfInt, Exception>;

    fn named_captures_for_haystack(
        &self,
        interp: &Artichoke,
        haystack: &[u8],
    ) -> Result<Option<HashMap<Vec<u8>, NilableString>>, Exception>;

    fn names(&self, interp: &Artichoke) -> Vec<Vec<u8>>;

    fn pos(
        &self,
        interp: &Artichoke,
        haystack: &[u8],
        at: usize,
    ) -> Result<Option<(usize, usize)>, Exception>;

    fn scan(
        &self,
        interp: &mut Artichoke,
        haystack: Value,
        block: Option<Block>,
    ) -> Result<Value, Exception>;
}
