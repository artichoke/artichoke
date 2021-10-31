// This module is forked from `regex-syntax` crate @ 26f7318e.
//
// https://github.com/rust-lang/regex/blob/26f7318e2895eae56e95a260e81e2d48b90e7c25/regex-syntax/src/lib.rs
//
// MIT License
// Copyright (c) 2014 The Rust Project Developers

#![allow(clippy::match_same_arms)]

//! Helpers for parsing Regexp patterns.

/// Escapes all regular expression meta characters in `text`.
///
/// The string returned may be safely used as a literal in a regular expression.
#[must_use]
pub fn escape(text: &str) -> String {
    let mut quoted = String::new();
    escape_into(text, &mut quoted);
    quoted
}

/// Escapes all meta characters in `text` and writes the result into `buf`.
///
/// This will append escape characters into the given buffer. The characters
/// that are appended are safe to use as a literal in a regular expression.
pub fn escape_into(text: &str, buf: &mut String) {
    buf.reserve(text.len());
    for c in text.chars() {
        match c {
            c if is_meta_character(c) => {
                buf.push('\\');
                buf.push(c);
            }
            c if is_non_printable_character(c) => {
                if let Some(escape) = is_non_supported_non_printable_character(c) {
                    buf.push_str(escape);
                } else {
                    for part in c.escape_default() {
                        buf.push(part);
                    }
                }
            }
            c => buf.push(c),
        }
    }
}

/// Returns true if the given character has significance in a regex.
#[must_use]
pub fn is_meta_character(c: char) -> bool {
    match c {
        '\\' | '.' | '+' | '*' | '?' | '(' | ')' | '|' | '[' | ']' | '{' | '}' | '^' | '$' | '#' | '&' | '-' | '~' => {
            true
        }
        // This match arm differs from `regex-syntax` by including '/'.
        // Ruby uses '/' to mark `Regexp` literals in source code.
        '/' => true,
        // This match arm differs from `regex-syntax` by including ' ' (an ASCII
        // space character). Ruby always escapes ' ' in calls to `Regexp::escape`.
        ' ' => true,
        _ => false,
    }
}

/// Returns true if the given character is non-printable and needs to be quoted.
#[must_use]
pub const fn is_non_printable_character(c: char) -> bool {
    matches!(
        c,
        '\n' | '\r' | '\t' |
        // form feed aka "\f"
        '\u{C}'
    )
}

/// Returns `Some(_)` if the given character is non-printable and Rust does not
/// support the escape sequence.
#[must_use]
pub const fn is_non_supported_non_printable_character(c: char) -> Option<&'static str> {
    match c {
        // form feed aka "\f"
        '\u{C}' => Some(r"\f"),
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn escape_meta() {
        assert_eq!(
            escape(r"\.+*?()|[]{}^$#&-~"),
            r"\\\.\+\*\?\(\)\|\[\]\{\}\^\$\#\&\-\~".to_string()
        );
    }
}
