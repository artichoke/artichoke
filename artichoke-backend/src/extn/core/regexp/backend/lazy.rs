use once_cell::sync::OnceCell;
use std::collections::HashMap;
use std::fmt;

use crate::extn::core::regexp::{Config, Encoding, Regexp, RegexpType, Scan};
use crate::extn::prelude::*;

use super::{NameToCaptureLocations, NilableString};

#[derive(Default, Debug)]
pub struct Lazy {
    literal: Config,
    encoding: Encoding,
    regexp: OnceCell<Regexp>,
}

impl From<Config> for Lazy {
    fn from(config: Config) -> Self {
        Self {
            literal: config,
            encoding: Encoding::default(),
            regexp: OnceCell::new(),
        }
    }
}

impl Lazy {
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    pub fn regexp(&self) -> Result<&Regexp, Exception> {
        self.regexp.get_or_try_init(|| {
            Regexp::new(self.literal.clone(), self.literal.clone(), self.encoding)
        })
    }
}

impl fmt::Display for Lazy {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        string::format_unicode_debug_into(f, self.literal.pattern.as_slice())
            .map_err(string::WriteError::into_inner)
    }
}

impl Clone for Lazy {
    fn clone(&self) -> Self {
        self.literal.clone().into()
    }
}

impl RegexpType for Lazy {
    fn box_clone(&self) -> Box<dyn RegexpType> {
        Box::new(self.clone())
    }

    fn captures(&self, haystack: &[u8]) -> Result<Option<Vec<NilableString>>, Exception> {
        self.regexp()?.inner().captures(haystack)
    }

    fn capture_indexes_for_name(&self, name: &[u8]) -> Result<Option<Vec<usize>>, Exception> {
        self.regexp()?.inner().capture_indexes_for_name(name)
    }

    fn captures_len(&self, haystack: Option<&[u8]>) -> Result<usize, Exception> {
        self.regexp()?.inner().captures_len(haystack)
    }

    fn capture0<'a>(&self, haystack: &'a [u8]) -> Result<Option<&'a [u8]>, Exception> {
        self.regexp()?.inner().capture0(haystack)
    }

    fn debug(&self) -> String {
        let mut debug = String::from("/");
        let mut pattern = String::new();
        // Explicitly supress this error because `debug` is infallible and
        // cannot panic.
        //
        // In practice this error will never be triggered since the only
        // fallible call in `string::format_unicode_debug_into` is to `write!` which never
        // `panic!`s for a `String` formatter, which we are using here.
        let _ = string::format_unicode_debug_into(&mut pattern, self.literal.pattern.as_slice());
        debug.push_str(pattern.replace("/", r"\/").as_str());
        debug.push('/');
        debug.push_str(self.literal.options.as_display_modifier());
        debug.push_str(self.encoding.modifier_string());
        debug
    }

    fn literal_config(&self) -> &Config {
        &self.literal
    }

    fn derived_config(&self) -> &Config {
        &self.literal
    }

    fn encoding(&self) -> &Encoding {
        &self.encoding
    }

    fn inspect(&self) -> Vec<u8> {
        self.regexp()
            .map(|regexp| regexp.inner().inspect())
            .unwrap_or_default()
    }

    fn string(&self) -> &[u8] {
        self.regexp()
            .map(|regexp| regexp.inner().string())
            .unwrap_or_default()
    }

    fn case_match(&self, interp: &mut Artichoke, haystack: &[u8]) -> Result<bool, Exception> {
        self.regexp()?.inner().case_match(interp, haystack)
    }

    fn is_match(&self, haystack: &[u8], pos: Option<Int>) -> Result<bool, Exception> {
        self.regexp()?.inner().is_match(haystack, pos)
    }

    fn match_(
        &self,
        interp: &mut Artichoke,
        haystack: &[u8],
        pos: Option<Int>,
        block: Option<Block>,
    ) -> Result<Value, Exception> {
        self.regexp()?.inner().match_(interp, haystack, pos, block)
    }

    fn match_operator(
        &self,
        interp: &mut Artichoke,
        haystack: &[u8],
    ) -> Result<Option<usize>, Exception> {
        self.regexp()?.inner().match_operator(interp, haystack)
    }

    fn named_captures(&self) -> Result<NameToCaptureLocations, Exception> {
        self.regexp()?.inner().named_captures()
    }

    fn named_captures_for_haystack(
        &self,
        haystack: &[u8],
    ) -> Result<Option<HashMap<Vec<u8>, NilableString>>, Exception> {
        self.regexp()?.inner().named_captures_for_haystack(haystack)
    }

    fn names(&self) -> Vec<Vec<u8>> {
        self.regexp()
            .map(|regexp| regexp.inner().names())
            .unwrap_or_default()
    }

    fn pos(&self, haystack: &[u8], at: usize) -> Result<Option<(usize, usize)>, Exception> {
        self.regexp()?.inner().pos(haystack, at)
    }

    fn scan(
        &self,
        interp: &mut Artichoke,
        haystack: &[u8],
        block: Option<Block>,
    ) -> Result<Scan, Exception> {
        self.regexp()?.inner().scan(interp, haystack, block)
    }
}
