use alloc::vec;
use core::iter::FusedIterator;
use core::slice;

/// Immutable [`String`] byte slice iterator.
///
/// This struct is created by the [`iter`] method on a Spinoso [`String`]. See
/// its documentation for more.
///
/// # Examples
///
/// ```
/// # use spinoso_string::String;
/// let s = String::utf8(b"Artichoke Ruby".to_vec());
///
/// let mut checksum: u32 = 0;
/// for &byte in s.iter() {
///     checksum += byte as u32;
/// }
/// assert_eq!(checksum, 1372);
/// ```
///
/// [`String`]: crate::String
/// [`iter`]: crate::String::iter
#[derive(Debug, Clone)]
pub struct Iter<'a>(pub(crate) slice::Iter<'a, u8>);

impl<'a> Iter<'a> {
    /// Views the underlying data as a subslice of the original data.
    ///
    /// This has the same lifetime as the original slice, and so the iterator
    /// can continue to be used while this exists.
    ///
    /// # Examples
    ///
    /// ```
    /// # use spinoso_string::String;
    /// let s = String::utf8(b"Artichoke Ruby".to_vec());
    ///
    /// // Then, we get the iterator:
    /// let mut iter = s.iter();
    /// assert_eq!(iter.as_slice(), b"Artichoke Ruby");
    ///
    /// // Next, we move to the second element of the slice:
    /// iter.next();
    /// // Now `as_slice` returns "rtichoke Ruby":
    /// assert_eq!(iter.as_slice(), b"rtichoke Ruby");
    /// ```
    #[inline]
    #[must_use]
    pub fn as_slice(&self) -> &[u8] {
        self.0.as_slice()
    }
}

impl<'a> AsRef<[u8]> for Iter<'a> {
    fn as_ref(&self) -> &[u8] {
        self.as_slice()
    }
}

impl<'a> Iterator for Iter<'a> {
    type Item = &'a u8;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        self.0.next()
    }

    #[inline]
    fn nth(&mut self, n: usize) -> Option<Self::Item> {
        self.0.nth(n)
    }

    #[inline]
    fn count(self) -> usize {
        self.0.count()
    }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        self.0.size_hint()
    }

    #[inline]
    fn last(self) -> Option<Self::Item> {
        self.0.last()
    }
}

impl<'a> DoubleEndedIterator for Iter<'a> {
    #[inline]
    fn next_back(&mut self) -> Option<Self::Item> {
        self.0.next_back()
    }

    #[inline]
    fn nth_back(&mut self, n: usize) -> Option<Self::Item> {
        self.0.nth_back(n)
    }
}

impl<'a> FusedIterator for Iter<'a> {}

impl<'a> ExactSizeIterator for Iter<'a> {}

/// Mutable [`String`] byte iterator.
///
/// This struct is created by the [`iter_mut`] method on a Spinoso [`String`].
/// See its documentation for more.
///
/// # Examples
///
/// ```
/// # use spinoso_string::String;
/// let mut s = String::utf8(b"Artichoke Ruby".to_vec());
///
/// for byte in s.iter_mut() {
///     *byte = b'z';
/// }
/// assert_eq!(s, "zzzzzzzzzzzzzz");
/// ```
///
/// [`String`]: crate::String
/// [`iter_mut`]: crate::String::iter_mut
#[derive(Debug)]
#[allow(clippy::module_name_repetitions)]
pub struct IterMut<'a>(pub(crate) slice::IterMut<'a, u8>);

impl<'a> IterMut<'a> {
    /// Views the underlying data as a subslice of the original data.
    ///
    /// To avoid creating `&mut` references that alias, this is forced to consume
    /// the iterator.
    ///
    /// # Examples
    ///
    /// ```
    /// # use spinoso_string::String;
    /// let mut s = String::utf8(b"Artichoke Ruby".to_vec());
    /// {
    ///     let mut iter = s.iter_mut();
    ///     iter.next();
    ///     assert_eq!(iter.into_slice(), b"rtichoke Ruby");
    /// }
    /// {
    ///     let mut iter = s.iter_mut();
    ///     *iter.next().unwrap() = b'a';
    ///     *iter.nth(9).unwrap() = b'r';
    /// }
    /// assert_eq!(s, &b"artichoke ruby"[..]);
    /// ```
    #[inline]
    #[must_use]
    pub fn into_slice(self) -> &'a mut [u8] {
        self.0.into_slice()
    }
}

impl<'a> Iterator for IterMut<'a> {
    type Item = &'a mut u8;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        self.0.next()
    }

    #[inline]
    fn nth(&mut self, n: usize) -> Option<Self::Item> {
        self.0.nth(n)
    }

    #[inline]
    fn count(self) -> usize {
        self.0.count()
    }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        self.0.size_hint()
    }

    #[inline]
    fn last(self) -> Option<Self::Item> {
        self.0.last()
    }
}

impl<'a> DoubleEndedIterator for IterMut<'a> {
    #[inline]
    fn next_back(&mut self) -> Option<Self::Item> {
        self.0.next_back()
    }

    #[inline]
    fn nth_back(&mut self, n: usize) -> Option<Self::Item> {
        self.0.nth_back(n)
    }
}

impl<'a> FusedIterator for IterMut<'a> {}

impl<'a> ExactSizeIterator for IterMut<'a> {}

/// An iterator that moves out of a string.
///
/// This struct is created by the `into_iter` method on `String` (provided by
/// the [`IntoIterator`] trait).
///
/// # Examples
///
/// ```
/// use spinoso_string::String;
///
/// let s = String::from("hello");
/// let iter: spinoso_string::IntoIter = s.into_iter();
/// ```
#[derive(Debug)]
#[allow(clippy::module_name_repetitions)]
pub struct IntoIter(pub(crate) vec::IntoIter<u8>);

impl IntoIter {
    /// Returns the remaining bytes of this iterator as a slice.
    ///
    /// # Examples
    ///
    /// ```
    /// use spinoso_string::String;
    ///
    /// let s = String::from("abc");
    /// let mut into_iter = s.into_iter();
    /// assert_eq!(into_iter.as_slice(), &[b'a', b'b', b'c']);
    /// let _ = into_iter.next().unwrap();
    /// assert_eq!(into_iter.as_slice(), &[b'b', b'c']);
    /// ```
    #[inline]
    #[must_use]
    pub fn as_slice(&self) -> &[u8] {
        self.0.as_slice()
    }

    /// Returns the remaining bytes of this iterator as a mutable slice.
    ///
    /// # Examples
    ///
    /// ```
    /// use spinoso_string::String;
    ///
    /// let s = String::from("abc");
    /// let mut into_iter = s.into_iter();
    /// assert_eq!(into_iter.as_slice(), &[b'a', b'b', b'c']);
    /// into_iter.as_mut_slice()[2] = b'z';
    /// assert_eq!(into_iter.next(), Some(b'a'));
    /// assert_eq!(into_iter.next(), Some(b'b'));
    /// assert_eq!(into_iter.next(), Some(b'z'));
    /// assert_eq!(into_iter.next(), None);
    /// ```
    #[inline]
    #[must_use]
    pub fn as_mut_slice(&mut self) -> &mut [u8] {
        self.0.as_mut_slice()
    }
}

impl AsRef<[u8]> for IntoIter {
    #[inline]
    fn as_ref(&self) -> &[u8] {
        self.as_slice()
    }
}

impl Iterator for IntoIter {
    type Item = u8;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        self.0.next()
    }

    #[inline]
    fn nth(&mut self, n: usize) -> Option<Self::Item> {
        self.0.nth(n)
    }

    #[inline]
    fn count(self) -> usize {
        self.0.count()
    }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        self.0.size_hint()
    }

    #[inline]
    fn last(self) -> Option<Self::Item> {
        self.0.last()
    }
}

impl DoubleEndedIterator for IntoIter {
    #[inline]
    fn next_back(&mut self) -> Option<Self::Item> {
        self.0.next_back()
    }

    #[inline]
    fn nth_back(&mut self, n: usize) -> Option<Self::Item> {
        self.0.nth_back(n)
    }
}

impl FusedIterator for IntoIter {}

impl ExactSizeIterator for IntoIter {}

/// Immutable [`String`] byte iterator.
///
/// This struct is created by the [`bytes`] method on a Spinoso [`String`]. See
/// its documentation for more.
///
/// # Examples
///
/// ```
/// # use spinoso_string::String;
/// let s = String::utf8(b"Artichoke Ruby".to_vec());
///
/// let mut checksum: u32 = 0;
/// for byte in s.bytes() {
///     checksum += byte as u32;
/// }
/// assert_eq!(checksum, 1372);
/// ```
///
/// [`String`]: crate::String
/// [`bytes`]: crate::String::bytes
#[derive(Debug, Clone)]
pub struct Bytes<'a>(pub(crate) slice::Iter<'a, u8>);

impl<'a> From<&'a [u8]> for Bytes<'a> {
    #[inline]
    fn from(bytes: &'a [u8]) -> Self {
        Self(bytes.iter())
    }
}

impl<'a> Bytes<'a> {
    /// Views the underlying data as a subslice of the original data.
    ///
    /// This has the same lifetime as the original slice, and so the iterator
    /// can continue to be used while this exists.
    ///
    /// # Examples
    ///
    /// ```
    /// # use spinoso_string::String;
    /// let s = String::utf8(b"Artichoke Ruby".to_vec());
    ///
    /// // Then, we get the iterator:
    /// let mut iter = s.bytes();
    /// assert_eq!(iter.as_slice(), b"Artichoke Ruby");
    ///
    /// // Next, we move to the second element of the slice:
    /// iter.next();
    /// // Now `as_slice` returns "rtichoke Ruby":
    /// assert_eq!(iter.as_slice(), b"rtichoke Ruby");
    /// ```
    #[inline]
    #[must_use]
    pub fn as_slice(&self) -> &[u8] {
        self.0.as_slice()
    }
}

impl<'a> Iterator for Bytes<'a> {
    type Item = u8;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        self.0.next().copied()
    }

    #[inline]
    fn nth(&mut self, n: usize) -> Option<Self::Item> {
        self.0.nth(n).copied()
    }

    #[inline]
    fn count(self) -> usize {
        self.0.count()
    }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        self.0.size_hint()
    }

    #[inline]
    fn last(self) -> Option<Self::Item> {
        self.0.last().copied()
    }
}

impl<'a> DoubleEndedIterator for Bytes<'a> {
    #[inline]
    fn next_back(&mut self) -> Option<Self::Item> {
        self.0.next_back().copied()
    }

    #[inline]
    fn nth_back(&mut self, n: usize) -> Option<Self::Item> {
        self.0.nth_back(n).copied()
    }
}

impl<'a> FusedIterator for Bytes<'a> {}

impl<'a> ExactSizeIterator for Bytes<'a> {}
