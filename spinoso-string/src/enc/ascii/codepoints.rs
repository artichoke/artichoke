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
