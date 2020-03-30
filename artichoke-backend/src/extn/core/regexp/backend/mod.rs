use std::collections::HashMap;

use crate::extn::core::regexp::{Config, Encoding};
use crate::extn::prelude::*;

pub mod lazy;
pub mod onig;
pub mod regex;

pub type NilableString = Option<Vec<u8>>;
pub type NameToCaptureLocations = Vec<(Vec<u8>, Vec<usize>)>;

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub enum Scan {
    Collected(Vec<Vec<Option<Vec<u8>>>>),
    Patterns(Vec<Vec<u8>>),
    Haystack,
}

pub trait RegexpType {
    fn box_clone(&self) -> Box<dyn RegexpType>;

    fn debug(&self) -> String;

    fn literal_config(&self) -> &Config;

    fn derived_config(&self) -> &Config;

    fn encoding(&self) -> &Encoding;

    fn inspect(&self, interp: &mut Artichoke) -> Vec<u8>;

    fn string(&self, interp: &mut Artichoke) -> &[u8];

    fn captures(
        &self,
        interp: &mut Artichoke,
        haystack: &[u8],
    ) -> Result<Option<Vec<NilableString>>, Exception>;

    fn capture_indexes_for_name(
        &self,
        interp: &mut Artichoke,
        name: &[u8],
    ) -> Result<Option<Vec<usize>>, Exception>;

    fn captures_len(
        &self,
        interp: &mut Artichoke,
        haystack: Option<&[u8]>,
    ) -> Result<usize, Exception>;

    fn capture0<'a>(
        &self,
        interp: &mut Artichoke,
        haystack: &'a [u8],
    ) -> Result<Option<&'a [u8]>, Exception>;

    fn case_match(&self, interp: &mut Artichoke, haystack: &[u8]) -> Result<bool, Exception>;

    fn is_match(
        &self,
        interp: &mut Artichoke,
        haystack: &[u8],
        pos: Option<Int>,
    ) -> Result<bool, Exception>;

    fn match_(
        &self,
        interp: &mut Artichoke,
        haystack: &[u8],
        pos: Option<Int>,
        block: Option<Block>,
    ) -> Result<Value, Exception>;

    fn match_operator(
        &self,
        interp: &mut Artichoke,
        haystack: &[u8],
    ) -> Result<Option<usize>, Exception>;

    fn named_captures(&self, interp: &mut Artichoke) -> Result<NameToCaptureLocations, Exception>;

    fn named_captures_for_haystack(
        &self,
        interp: &mut Artichoke,
        haystack: &[u8],
    ) -> Result<Option<HashMap<Vec<u8>, NilableString>>, Exception>;

    fn names(&self, interp: &mut Artichoke) -> Vec<Vec<u8>>;

    fn pos(
        &self,
        interp: &mut Artichoke,
        haystack: &[u8],
        at: usize,
    ) -> Result<Option<(usize, usize)>, Exception>;

    fn scan(
        &self,
        interp: &mut Artichoke,
        haystack: &[u8],
        block: Option<Block>,
    ) -> Result<Scan, Exception>;
}
