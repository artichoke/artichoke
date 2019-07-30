//! Parse options parameter to `Regexp#initialize` and `Regexp::compile`.

use onig::RegexOptions;

use crate::convert::Int;
use crate::extn::core::regexp::Regexp;
use crate::value::Value;

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Hash)]
pub struct Options {
    pub multiline: bool,
    pub ignore_case: bool,
    pub extended: bool,
}

impl Options {
    pub fn ignore_case() -> Self {
        let mut opts = Self::default();
        opts.ignore_case = true;
        opts
    }

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

    pub fn modifier_string(self) -> &'static str {
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
    }

    fn onig_string(self) -> &'static str {
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
    }
}

pub fn parse(value: &Value) -> Options {
    // If options is an Integer, it should be one or more of the constants
    // Regexp::EXTENDED, Regexp::IGNORECASE, and Regexp::MULTILINE, logically
    // or-ed together. Otherwise, if options is not nil or false, the regexp
    // will be case insensitive.
    if let Ok(options) = value.itself::<Int>() {
        // Only deal with Regexp opts
        let options = options & Regexp::ALL_REGEXP_OPTS;
        Options {
            multiline: options & Regexp::MULTILINE > 0,
            ignore_case: options & Regexp::IGNORECASE > 0,
            extended: options & Regexp::EXTENDED > 0,
        }
    } else if let Ok(options) = value.itself::<Option<bool>>() {
        match options {
            Some(false) | None => Options::default(),
            _ => Options::ignore_case(),
        }
    } else if let Ok(options) = value.itself::<Option<String>>() {
        if let Some(options) = options {
            Options {
                multiline: options.contains('m'),
                ignore_case: options.contains('i'),
                extended: options.contains('x'),
            }
        } else {
            Options::default()
        }
    } else {
        Options::ignore_case()
    }
}

// TODO: Add tests for this parse_pattern, see GH-157.
pub fn parse_pattern(pattern: &str, mut opts: Options) -> (String, Options) {
    let orig_opts = opts;
    let mut chars = pattern.chars();
    let mut enabled = true;
    let mut pat_buf = String::new();
    let mut pointer = 0;
    match chars.next() {
        None => {
            pat_buf.push_str("(?");
            pat_buf.push_str(opts.onig_string());
            pat_buf.push(':');
            pat_buf.push(')');
            return (pat_buf, opts);
        }
        Some(token) if token != '(' => {
            pat_buf.push_str("(?");
            pat_buf.push_str(opts.onig_string());
            pat_buf.push(':');
            pat_buf.push_str(pattern);
            pat_buf.push(')');
            return (pat_buf, opts);
        }
        _ => (),
    };
    pointer += 1;
    match chars.next() {
        None => {
            pat_buf.push_str("(?");
            pat_buf.push_str(opts.onig_string());
            pat_buf.push(':');
            pat_buf.push_str(pattern);
            pat_buf.push(')');
            return (pat_buf, opts);
        }
        Some(token) if token != '?' => {
            pat_buf.push_str("(?");
            pat_buf.push_str(opts.onig_string());
            pat_buf.push(':');
            pat_buf.push_str(pattern);
            pat_buf.push(')');
            return (pat_buf, opts);
        }
        _ => (),
    };
    pointer += 1;
    for token in chars {
        pointer += 1;
        match token {
            '-' => enabled = false,
            'i' => {
                opts.ignore_case = enabled;
            }
            'm' => {
                opts.multiline = enabled;
            }
            'x' => {
                opts.extended = enabled;
            }
            ':' => break,
            _ => {
                pat_buf.push_str("(?");
                pat_buf.push_str(opts.onig_string());
                pat_buf.push(':');
                pat_buf.push_str(pattern);
                pat_buf.push(')');
                return (pat_buf, opts);
            }
        }
    }
    let mut chars = pattern[pointer..].chars();
    let mut token = chars.next();
    let mut nest = 1;
    while token.is_some() {
        match token {
            Some(token) if token == '(' => nest += 1,
            Some(token) if token == ')' => {
                nest -= 1;
                if nest == 0 && chars.next().is_some() {
                    return (
                        format!("(?{}:{})", orig_opts.onig_string(), pattern),
                        orig_opts,
                    );
                }
                break;
            }
            _ => (),
        }
        token = chars.next();
    }

    (
        format!("(?{}:{}", opts.onig_string(), &pattern[pointer..]),
        opts,
    )
}
