use core::slice;

#[derive(Debug, Clone)]
pub struct Codepoints<'a> {
    inner: slice::Iter<'a, u8>,
}

impl<'a> Codepoints<'a> {
    #[inline]
    pub fn new(bytes: &'a [u8]) -> Self {
        Self { inner: bytes.iter() }
    }
}

impl<'a> Iterator for Codepoints<'a> {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        self.inner.next().map(|&b| u32::from(b))
    }
}

impl<'a> Default for Codepoints<'a> {
    #[inline]
    fn default() -> Self {
        Self::new(b"")
    }
}

#[cfg(test)]
mod tests {
    use alloc::vec::Vec;

    use super::*;
    use crate::enc::ascii::AsciiString;

    #[test]
    fn test_valid_ascii() {
        let s = AsciiString::from("abc");
        let codepoints = Codepoints::new(&s);
        assert_eq!(codepoints.collect::<Vec<_>>(), &[97, 98, 99]);
    }

    #[test]
    fn test_utf8_interpreted_as_bytes() {
        let s = AsciiString::from("abc💎");
        let codepoints = Codepoints::new(&s);
        assert_eq!(codepoints.collect::<Vec<_>>(), &[97, 98, 99, 240, 159, 146, 142]);
    }

    #[test]
    fn test_invalid_utf8_interpreted_as_bytes() {
        let s = AsciiString::from(b"abc\xFF");
        let codepoints = Codepoints::new(&s);
        assert_eq!(codepoints.collect::<Vec<_>>(), &[97, 98, 99, 255]);
    }
}
