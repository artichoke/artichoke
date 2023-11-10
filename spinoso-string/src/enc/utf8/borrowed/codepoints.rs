use core::str::Chars;

use super::Utf8Str;
use crate::CodepointsError;

#[derive(Debug, Clone)]
pub struct Codepoints<'a> {
    inner: Chars<'a>,
}

impl<'a> TryFrom<&'a Utf8Str> for Codepoints<'a> {
    type Error = CodepointsError;

    #[inline]
    fn try_from(s: &'a Utf8Str) -> Result<Self, Self::Error> {
        match simdutf8::basic::from_utf8(s.as_bytes()) {
            Ok(s) => Ok(Self { inner: s.chars() }),
            // ```
            // [3.2.2] > s = "abc\xFFxyz"
            // => "abc\xFFxyz"
            // [3.2.2] > s.encoding
            // => #<Encoding:UTF-8>
            // [3.2.2] > s.codepoints
            // (irb):5:in `codepoints': invalid byte sequence in UTF-8 (ArgumentError)
            //         from (irb):5:in `<main>'
            //         from /usr/local/var/rbenv/versions/3.2.2/lib/ruby/gems/3.2.0/gems/irb-1.6.2/exe/irb:11:in `<top (required)>'
            //         from /usr/local/var/rbenv/versions/3.2.2/bin/irb:25:in `load'
            //         from /usr/local/var/rbenv/versions/3.2.2/bin/irb:25:in `<main>'
            // ```
            Err(_) => Err(CodepointsError::invalid_utf8_codepoint()),
        }
    }
}

impl<'a> Iterator for Codepoints<'a> {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        self.inner.next().map(u32::from)
    }
}

impl<'a> Default for Codepoints<'a> {
    #[inline]
    fn default() -> Self {
        Self { inner: "".chars() }
    }
}

#[cfg(test)]
mod tests {
    use alloc::vec::Vec;

    use super::*;

    #[test]
    fn test_valid_utf8() {
        let s = Utf8Str::new("hello💎");
        let codepoints = Codepoints::try_from(s).unwrap();
        assert_eq!(codepoints.collect::<Vec<_>>(), &[104, 101, 108, 108, 111, 128_142]);
    }

    #[test]
    fn test_invalid_utf8() {
        let s = Utf8Str::new(b"hello\xFF");
        let err = Codepoints::try_from(s).unwrap_err();
        assert_eq!(err, CodepointsError::invalid_utf8_codepoint());
    }
}
