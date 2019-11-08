use once_cell::sync::OnceCell;
use std::collections::HashMap;
use std::fmt;

use crate::extn::core::exception::RubyException;
use crate::extn::core::regexp::{Config, Encoding, Regexp, RegexpType};
use crate::types::Int;
use crate::value::{Block, Value};
use crate::Artichoke;

pub struct Lazy {
    literal: Config,
    encoding: Encoding,
    regexp: OnceCell<Regexp>,
}

impl Lazy {
    pub fn new(literal: Config) -> Self {
        Self {
            literal,
            encoding: Encoding::default(),
            regexp: OnceCell::new(),
        }
    }

    pub fn regexp(&self, interp: &Artichoke) -> &Regexp {
        self.regexp.get_or_init(|| {
            Regexp::new(
                interp,
                self.literal.clone(),
                self.literal.clone(),
                Encoding::default(),
            )
            .expect("Lazy Regexp did not parse")
        })
    }
}

impl fmt::Debug for Lazy {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "/{}/{}{}",
            String::from_utf8_lossy(self.literal.pattern.as_slice()).replace("/", r"\/"),
            self.literal.options.modifier_string(),
            Encoding::default().string()
        )
    }
}

impl fmt::Display for Lazy {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}",
            String::from_utf8_lossy(self.literal.pattern.as_slice())
        )
    }
}

impl Clone for Lazy {
    fn clone(&self) -> Self {
        Self::new(self.literal.clone())
    }
}

impl RegexpType for Lazy {
    fn box_clone(&self) -> Box<dyn RegexpType> {
        Box::new(self.clone())
    }

    fn captures(
        &self,
        interp: &Artichoke,
        haystack: &[u8],
    ) -> Result<Option<Vec<Option<Vec<u8>>>>, Box<dyn RubyException>> {
        self.regexp(interp).inner().captures(interp, haystack)
    }

    fn capture_indexes_for_name(
        &self,
        interp: &Artichoke,
        name: &[u8],
    ) -> Result<Option<Vec<usize>>, Box<dyn RubyException>> {
        self.regexp(interp)
            .inner()
            .capture_indexes_for_name(interp, name)
    }

    fn captures_len(
        &self,
        interp: &Artichoke,
        haystack: Option<&[u8]>,
    ) -> Result<usize, Box<dyn RubyException>> {
        self.regexp(interp).inner().captures_len(interp, haystack)
    }

    fn capture0<'a>(
        &self,
        interp: &Artichoke,
        haystack: &'a [u8],
    ) -> Result<Option<&'a [u8]>, Box<dyn RubyException>> {
        self.regexp(interp).inner().capture0(interp, haystack)
    }

    fn debug(&self) -> String {
        format!("{:?}", self)
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

    fn inspect(&self, interp: &Artichoke) -> Vec<u8> {
        self.regexp(interp).inner().inspect(interp)
    }

    fn string(&self, interp: &Artichoke) -> &[u8] {
        self.regexp(interp).inner().string(interp)
    }

    fn case_match(
        &self,
        interp: &Artichoke,
        pattern: &[u8],
    ) -> Result<bool, Box<dyn RubyException>> {
        self.regexp(interp).inner().case_match(interp, pattern)
    }

    fn is_match(
        &self,
        interp: &Artichoke,
        pattern: &[u8],
        pos: Option<Int>,
    ) -> Result<bool, Box<dyn RubyException>> {
        self.regexp(interp).inner().is_match(interp, pattern, pos)
    }

    fn match_(
        &self,
        interp: &Artichoke,
        pattern: &[u8],
        pos: Option<Int>,
        block: Option<Block>,
    ) -> Result<Value, Box<dyn RubyException>> {
        self.regexp(interp)
            .inner()
            .match_(interp, pattern, pos, block)
    }

    fn match_operator(
        &self,
        interp: &Artichoke,
        pattern: &[u8],
    ) -> Result<Option<Int>, Box<dyn RubyException>> {
        self.regexp(interp).inner().match_operator(interp, pattern)
    }

    fn named_captures(
        &self,
        interp: &Artichoke,
    ) -> Result<Vec<(Vec<u8>, Vec<Int>)>, Box<dyn RubyException>> {
        self.regexp(interp).inner().named_captures(interp)
    }

    fn named_captures_for_haystack(
        &self,
        interp: &Artichoke,
        haystack: &[u8],
    ) -> Result<Option<HashMap<Vec<u8>, Option<Vec<u8>>>>, Box<dyn RubyException>> {
        self.regexp(interp)
            .inner()
            .named_captures_for_haystack(interp, haystack)
    }

    fn names(&self, interp: &Artichoke) -> Vec<Vec<u8>> {
        self.regexp(interp).inner().names(interp)
    }

    fn pos(
        &self,
        interp: &Artichoke,
        haystack: &[u8],
        at: usize,
    ) -> Result<Option<(usize, usize)>, Box<dyn RubyException>> {
        self.regexp(interp).inner().pos(interp, haystack, at)
    }

    fn scan(
        &self,
        interp: &Artichoke,
        haystack: Value,
        block: Option<Block>,
    ) -> Result<Value, Box<dyn RubyException>> {
        self.regexp(interp).inner().scan(interp, haystack, block)
    }
}
