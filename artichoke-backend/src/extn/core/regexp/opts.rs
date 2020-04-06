//! Parse options parameter to `Regexp#initialize` and `Regexp::compile`.

use onig::RegexOptions;

use crate::extn::core::regexp;
use crate::extn::prelude::*;

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Hash)]
pub struct Options {
    pub multiline: bool,
    pub ignore_case: bool,
    pub extended: bool,
    pub literal: bool,
}

impl Options {
    #[must_use]
    pub fn ignore_case() -> Self {
        let mut opts = Self::default();
        opts.ignore_case = true;
        opts
    }

    #[must_use]
    pub fn flags(self) -> RegexOptions {
        let mut bits = RegexOptions::REGEX_OPTION_NONE;
        if self.multiline {
            bits |= RegexOptions::REGEX_OPTION_MULTILINE;
        }
        if self.ignore_case {
            bits |= RegexOptions::REGEX_OPTION_IGNORECASE;
        }
        if self.extended {
            bits |= RegexOptions::REGEX_OPTION_EXTEND;
        }
        bits
    }

    // TODO: This function should return a &'static str but linking fails under
    // the wasm32-unknown-emscripten build if this function does not return an
    // owned String.
    #[must_use]
    pub fn modifier_string(self) -> String {
        match (self.multiline, self.ignore_case, self.extended) {
            (true, true, true) => "mix",
            (true, true, false) => "mi",
            (true, false, true) => "mx",
            (true, false, false) => "m",
            (false, true, true) => "ix",
            (false, true, false) => "i",
            (false, false, true) => "x",
            (false, false, false) => "",
        }
        .to_owned()
    }

    // TODO: This function should return a &'static str but linking fails under
    // the wasm32-unknown-emscripten build if this function does not return an
    // owned String.
    #[must_use]
    fn onig_string(self) -> String {
        match (self.multiline, self.ignore_case, self.extended) {
            (true, true, true) => "mix",
            (true, true, false) => "mi-x",
            (true, false, true) => "mx-i",
            (true, false, false) => "m-ix",
            (false, true, true) => "ix-m",
            (false, true, false) => "i-mx",
            (false, false, true) => "x-mi",
            (false, false, false) => "-mix",
        }
        .to_owned()
    }
}

#[must_use]
pub fn parse(interp: &mut Artichoke, value: &Value) -> Options {
    // If options is an Integer, it should be one or more of the constants
    // Regexp::EXTENDED, Regexp::IGNORECASE, and Regexp::MULTILINE, logically
    // or-ed together. Otherwise, if options is not nil or false, the regexp
    // will be case insensitive.
    if let Ok(options) = value.try_into::<Int>(interp) {
        Options {
            multiline: options & regexp::MULTILINE > 0,
            ignore_case: options & regexp::IGNORECASE > 0,
            extended: options & regexp::EXTENDED > 0,
            literal: options & regexp::LITERAL > 0,
        }
    } else if let Ok(options) = value.try_into::<Option<bool>>(interp) {
        match options {
            Some(false) | None => Options::default(),
            _ => Options::ignore_case(),
        }
    } else if let Ok(options) = value.try_into::<Option<&str>>(interp) {
        if let Some(options) = options {
            Options {
                multiline: options.contains('m'),
                ignore_case: options.contains('i'),
                extended: options.contains('x'),
                literal: false,
            }
        } else {
            Options::default()
        }
    } else {
        Options::ignore_case()
    }
}

// TODO(GH-26): Add tests for `parse_pattern`.
#[must_use]
pub fn parse_pattern(pattern: &[u8], mut opts: Options) -> (Vec<u8>, Options) {
    let orig_opts = opts;
    let mut chars = pattern.iter().copied();
    let mut enabled = true;
    let mut pat_buf = Vec::new();
    let mut pointer = 0;
    match chars.next() {
        None => {
            pat_buf.extend(b"(?".to_vec());
            pat_buf.extend(opts.onig_string().into_bytes());
            pat_buf.push(b':');
            pat_buf.push(b')');
            return (pat_buf, opts);
        }
        Some(token) if token != b'(' => {
            pat_buf.extend(b"(?".to_vec());
            pat_buf.extend(opts.onig_string().into_bytes());
            pat_buf.push(b':');
            pat_buf.extend(pattern);
            pat_buf.push(b')');
            return (pat_buf, opts);
        }
        _ => (),
    };
    pointer += 1;
    match chars.next() {
        None => {
            pat_buf.extend(b"(?".to_vec());
            pat_buf.extend(opts.onig_string().into_bytes());
            pat_buf.push(b':');
            pat_buf.extend(pattern);
            pat_buf.push(b')');
            return (pat_buf, opts);
        }
        Some(token) if token != b'?' => {
            pat_buf.extend(b"(?".to_vec());
            pat_buf.extend(opts.onig_string().into_bytes());
            pat_buf.push(b':');
            pat_buf.extend(pattern);
            pat_buf.push(b')');
            return (pat_buf, opts);
        }
        _ => (),
    };
    pointer += 1;
    for token in chars {
        pointer += 1;
        match token {
            b'-' => enabled = false,
            b'i' => {
                opts.ignore_case = enabled;
            }
            b'm' => {
                opts.multiline = enabled;
            }
            b'x' => {
                opts.extended = enabled;
            }
            b':' => break,
            _ => {
                pat_buf.extend(b"(?".to_vec());
                pat_buf.extend(opts.onig_string().into_bytes());
                pat_buf.push(b':');
                pat_buf.extend(pattern);
                pat_buf.push(b')');
                return (pat_buf, opts);
            }
        }
    }
    let mut chars = pattern[pointer..].iter().copied();
    let mut nest = 1;
    while let Some(token) = chars.next() {
        if token == b'(' {
            nest += 1;
        } else if token == b')' {
            nest -= 1;
            if nest == 0 && chars.next().is_some() {
                pat_buf.extend(b"(?".to_vec());
                pat_buf.extend(orig_opts.onig_string().into_bytes());
                pat_buf.push(b':');
                pat_buf.extend(pattern);
                pat_buf.push(b')');
                return (pat_buf, orig_opts);
            }
            break;
        }
    }
    pat_buf.extend(b"(?".to_vec());
    pat_buf.extend(opts.onig_string().into_bytes());
    pat_buf.push(b':');
    pat_buf.extend(&pattern[pointer..]);
    (pat_buf, opts)
}
