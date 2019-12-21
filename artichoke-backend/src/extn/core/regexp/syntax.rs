// This module is forked from Rust regex crate @ 172898a.
//
// https://github.com/rust-lang/regex/blob/172898a4fda4fd6a2d1be9fc7b8a0ea971c84ca6/regex-syntax/src/lib.rs
//
// MIT Licence
// Copyright (c) 2014 The Rust Project Developers

//! Helpers for parsing Regexp patterns.

/// Escapes all regular expression meta characters in `text`.
///
/// The string returned may be safely used as a literal in a regular
/// expression.
#[must_use]
pub fn escape(text: &str) -> String {
    let mut quoted = String::with_capacity(text.len());
    escape_into(text, &mut quoted);
    quoted
}

/// Escapes all meta characters in `text` and writes the result into `buf`.
///
/// This will append escape characters into the given buffer. The characters
/// that are appended are safe to use as a literal in a regular expression.
pub fn escape_into(text: &str, buf: &mut String) {
    for c in text.chars() {
        if is_meta_character(c) {
            buf.push('\\');
            buf.push(c);
        } else if let Some(escape) = is_non_supported_non_printable_character(c) {
            buf.push_str(escape.as_str());
        } else if is_non_printable_character(c) {
            for c in c.escape_default() {
                buf.push(c);
            }
        } else if c == ' ' {
            buf.push('\\');
            buf.push(' ');
        } else {
            buf.push(c);
        }
    }
}

/// Returns true if the given character has significance in a regex.
///
/// These are the only characters that are allowed to be escaped, with one
/// exception: an ASCII space character may be escaped when extended mode (with
/// the `x` flag) is enabld. In particular, `is_meta_character(' ')` returns
/// `false`.
///
/// Note that the set of characters for which this function returns `true` or
/// `false` is fixed and won't change in a semver compatible release.
#[must_use]
pub fn is_meta_character(c: char) -> bool {
    match c {
        '\\' | '/' | '.' | '+' | '*' | '?' | '(' | ')' | '|' | '[' | ']' | '{' | '}' | '^'
        | '$' | '#' | '&' | '-' | '~' => true,
        _ => false,
    }
}

/// Returns true if the given character is non-printable and needs to be quoted.
#[must_use]
pub fn is_non_printable_character(c: char) -> bool {
    let form_feed = 0x0C as char; // "\f"
    match c {
        '\n' | '\r' | '\t' => true,
        c if c == form_feed => true,
        _ => false,
    }
}

/// Returns true if the given character is non-printable and Rust does not support
/// the escape sequence.
#[must_use]
pub fn is_non_supported_non_printable_character(c: char) -> Option<String> {
    let form_feed = 0x0C as char; // "\f"
    match c {
        c if c == form_feed => Some("\\f".to_owned()),
        _ => None,
    }
}
