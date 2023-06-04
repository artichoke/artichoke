use alloc::borrow::Cow;
use alloc::boxed::Box;
use alloc::collections::TryReserveError;
use alloc::string::String;
use alloc::vec::{IntoIter, Vec};
use core::borrow::{Borrow, BorrowMut};
#[cfg(feature = "std")]
use core::fmt::Arguments;
use core::ops::{Deref, DerefMut};
use core::slice::{Iter, IterMut};
#[cfg(feature = "std")]
use std::io::{self, IoSlice, Write};

use bstr::ByteVec;
use raw_parts::RawParts;

/// A contiguous growable byte string, written as `Buf`, short for 'buffer'.
///
/// This buffer is a transparent wrapper around [`Vec<u8>`] with a minimized API
/// sufficient for implementing the Ruby [`String`] type.
///
/// This buffer does not assume any encoding. Encoding is a higher-level concept
/// that should be built on top of `Buf`.
///
/// # Examples
///
/// ```
/// use scolapasta_strbuf::Buf;
///
/// let mut buf = Buf::new();
/// buf.push_byte(b'a');
/// buf.push_byte(b'z');
///
/// assert_eq!(buf.len(), 2);
/// assert_eq!(buf[0], b'a');
///
/// assert_eq!(buf.pop_byte(), Some(b'z'));
/// assert_eq!(buf.len(), 1);
///
/// buf[0] = b'!';
/// assert_eq!(buf[0], b'!');
///
/// buf.extend(b"excite!!!");
///
/// for byte in &buf {
///     println!("{byte}");
/// }
/// assert_eq!(buf, b"!excite!!!");
/// ```
///
/// # Indexing
///
/// The `Buf` type allows to access values by index, because it implements the
/// [`Index`] trait. An example will be more explicit:
///
/// ```
/// use scolapasta_strbuf::Buf;
///
/// let buf = Buf::from(b"scolapasta-strbuf");
/// println!("{}", buf[1]); // it will display 'c'
/// ```
///
/// However be careful: if you try to access an index which isn't in the `Buf`,
/// your software will panic! You cannot do this:
///
/// ```should_panic
/// use scolapasta_strbuf::Buf;
///
/// let buf = Buf::from(b"scolapasta-strbuf");
/// println!("{}", buf[100]); // it will panic!
/// ```
///
/// # Capacity and reallocation
///
/// The capacity of a buffer is the amount of space allocated for any future
/// bytes that will be added onto the buffer. This is not to be confused with
/// the _length_ of a buffer, which specifies the number of actual bytes within
/// the buffer. If a buffer's length exceeds its capacity, its capacity will
/// automatically be increased, but its contents will have to be reallocated.
///
/// For example, a buffer with capacity 10 and length 0 would be an empty buffer
/// with space for 10 more bytes. Pushing 10 or fewer bytes into the buffer will
/// not change its capacity or cause reallocation to occur. However, if the
/// buffer's length is increased to 11, it will have to reallocate, which can be
/// slow. For this reason, it is recommended to use `Buf::with_capacity`
/// whenever possible to specify how big the buffer is expected to get.
///
/// # Guarantees
///
/// `Buf` is guaranteed to be a `repr(transparent)` wrapper around a `Vec<u8>`,
/// which means it shares all the same [guarantees as a `Vec`]. See the upstream
/// documentation in [`std`][vec-docs] for more details.
///
/// [`Vec<u8>`]: Vec
/// [`String`]: https://ruby-doc.org/3.2.0/String.html
/// [`Index`]: core::ops::Index
/// [guarantees as a `Vec`]: https://doc.rust-lang.org/std/vec/struct.Vec.html#guarantees
/// [vec-docs]: alloc::vec
#[repr(transparent)]
#[derive(Default, Debug, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct Buf {
    inner: Vec<u8>,
}

impl Buf {
    #[inline]
    #[must_use]
    pub fn into_inner(self) -> Vec<u8> {
        self.inner
    }
}

impl From<Vec<u8>> for Buf {
    #[inline]
    fn from(vec: Vec<u8>) -> Self {
        Self { inner: vec }
    }
}

impl<'a> From<&'a [u8]> for Buf {
    #[inline]
    fn from(s: &'a [u8]) -> Self {
        let vec = s.to_vec();
        Self::from(vec)
    }
}

impl<'a> From<&'a mut [u8]> for Buf {
    #[inline]
    fn from(s: &'a mut [u8]) -> Self {
        let vec = s.to_vec();
        Self::from(vec)
    }
}

impl<const N: usize> From<[u8; N]> for Buf {
    #[inline]
    fn from(s: [u8; N]) -> Self {
        let vec = Vec::from(s);
        Self::from(vec)
    }
}

impl<'a, const N: usize> From<&'a [u8; N]> for Buf {
    #[inline]
    fn from(s: &'a [u8; N]) -> Self {
        let vec = s.to_vec();
        Self::from(vec)
    }
}

impl<'a, const N: usize> From<&'a mut [u8; N]> for Buf {
    #[inline]
    fn from(s: &'a mut [u8; N]) -> Self {
        let vec = s.to_vec();
        Self::from(vec)
    }
}

impl<'a> From<Cow<'a, [u8]>> for Buf {
    #[inline]
    fn from(s: Cow<'a, [u8]>) -> Self {
        let vec = s.into_owned();
        Self::from(vec)
    }
}

impl From<String> for Buf {
    #[inline]
    fn from(s: String) -> Self {
        let vec = s.into_bytes();
        Self::from(vec)
    }
}

impl<'a> From<&'a str> for Buf {
    #[inline]
    fn from(s: &'a str) -> Self {
        let vec = s.as_bytes().to_vec();
        Self::from(vec)
    }
}

impl<'a> From<&'a mut str> for Buf {
    #[inline]
    fn from(s: &'a mut str) -> Self {
        let vec = s.as_bytes().to_vec();
        Self::from(vec)
    }
}

impl<'a> From<Cow<'a, str>> for Buf {
    #[inline]
    fn from(s: Cow<'a, str>) -> Self {
        let vec = s.into_owned().into_bytes();
        Self::from(vec)
    }
}

impl From<Buf> for Vec<u8> {
    #[inline]
    fn from(buf: Buf) -> Self {
        buf.inner
    }
}

impl<const N: usize> TryFrom<Buf> for [u8; N] {
    type Error = Buf;

    #[inline]
    fn try_from(buf: Buf) -> Result<Self, Self::Error> {
        match buf.into_inner().try_into() {
            Ok(array) => Ok(array),
            Err(vec) => Err(vec.into()),
        }
    }
}

impl<'a> From<Buf> for Cow<'a, [u8]> {
    #[inline]
    fn from(buf: Buf) -> Self {
        Cow::Owned(buf.into())
    }
}

impl AsRef<[u8]> for Buf {
    #[inline]
    fn as_ref(&self) -> &[u8] {
        self.inner.as_ref()
    }
}

impl AsMut<[u8]> for Buf {
    #[inline]
    fn as_mut(&mut self) -> &mut [u8] {
        self.inner.as_mut()
    }
}

impl Borrow<[u8]> for Buf {
    fn borrow(&self) -> &[u8] {
        self
    }
}

impl BorrowMut<[u8]> for Buf {
    fn borrow_mut(&mut self) -> &mut [u8] {
        self
    }
}

impl Deref for Buf {
    type Target = [u8];

    #[inline]
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl DerefMut for Buf {
    #[inline]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl FromIterator<u8> for Buf {
    #[inline]
    fn from_iter<T>(iter: T) -> Self
    where
        T: IntoIterator<Item = u8>,
    {
        let inner = iter.into_iter().collect();
        Self { inner }
    }
}

impl Extend<u8> for Buf {
    #[inline]
    fn extend<I: IntoIterator<Item = u8>>(&mut self, iter: I) {
        self.inner.extend(iter.into_iter());
    }
}

impl<'a> Extend<&'a u8> for Buf {
    #[inline]
    fn extend<I: IntoIterator<Item = &'a u8>>(&mut self, iter: I) {
        self.inner.extend(iter.into_iter().copied());
    }
}

impl_partial_eq!(Buf, Vec<u8>);
impl_partial_eq!(Buf, &'a Vec<u8>);
impl_partial_eq!(Buf, [u8]);
impl_partial_eq!(Buf, &'a [u8]);
impl_partial_eq!(Buf, &'a mut [u8]);
impl_partial_eq!(Buf, String);
impl_partial_eq!(Buf, &'a String);
impl_partial_eq!(Buf, str);
impl_partial_eq!(Buf, &'a str);
impl_partial_eq!(Buf, &'a mut str);
impl_partial_eq_array!(Buf, [u8; N]);
impl_partial_eq_array!(Buf, &'a [u8; N]);
impl_partial_eq_array!(Buf, &'a mut [u8; N]);

impl IntoIterator for Buf {
    type Item = u8;
    type IntoIter = IntoIter<u8>;

    fn into_iter(self) -> Self::IntoIter {
        self.into_inner().into_iter()
    }
}

impl<'a> IntoIterator for &'a Buf {
    type Item = &'a u8;
    type IntoIter = Iter<'a, u8>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

impl<'a> IntoIterator for &'a mut Buf {
    type Item = &'a mut u8;
    type IntoIter = IterMut<'a, u8>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter_mut()
    }
}

/// Minimal [`Vec`] API.
impl Buf {
    /// Constructs a new, empty `Buf`.
    ///
    /// The buffer will not allocate until bytes are pushed into it.
    ///
    /// # Examples
    ///
    /// ```
    /// use scolapasta_strbuf::Buf;
    ///
    /// let mut buf = Buf::new();
    /// ```
    #[inline]
    #[must_use]
    pub fn new() -> Self {
        let inner = Vec::new();
        Self { inner }
    }

    /// Constructs a new, empty `Buf` with at least the specified capacity.
    ///
    /// The buffer will be able to hold at least `capacity` bytes without
    /// reallocating. This method is allowed to allocate for more elements than
    /// `capacity`. If `capacity` is 0, the buffer will not allocate.
    ///
    /// It is important to note that although the returned buffer has the
    /// minimum *capacity* specified, the vector will have a zero *length*. For
    /// an explanation of the difference between length and capacity, see
    /// *[Capacity and reallocation]*.
    ///
    /// If it is important to know the exact allocated capacity of a `Buf`,
    /// always use the [`capacity`] method after construction.
    ///
    /// [Capacity and reallocation]: #capacity-and-reallocation
    /// [`capacity`]: Self::capacity
    ///
    /// # Panics
    ///
    /// Panics if the new capacity exceeds `isize::MAX` bytes.
    ///
    /// # Examples
    ///
    /// ```
    /// use scolapasta_strbuf::Buf;
    ///
    /// let mut buf = Buf::with_capacity(26);
    ///
    /// // The buffer is empty, even though it has capacity for more
    /// assert_eq!(buf.len(), 0);
    /// assert!(buf.capacity() >= 26);
    ///
    /// // These are all done without reallocating...
    /// for ch in b'a'..=b'z' {
    ///     buf.push_byte(ch);
    /// }
    /// assert_eq!(buf.len(), 26);
    /// assert!(buf.capacity() >= 26);
    ///
    /// // ...but this may make the buffer reallocate
    /// buf.push_byte(b'!');
    /// assert_eq!(buf.len(), 27);
    /// assert!(buf.capacity() >= 27);
    /// ```
    #[inline]
    #[must_use]
    pub fn with_capacity(capacity: usize) -> Self {
        let inner = Vec::with_capacity(capacity);
        Self { inner }
    }

    /// Creates a `Buf` directly from a pointer, a capacity, and a length.
    ///
    /// # Safety
    ///
    /// This is highly unsafe, due to the number of invariants that aren't
    /// checked.
    ///
    /// Refer to the safety documentation for [`Vec::from_raw_parts`] for more
    /// details.
    ///
    /// # Examples
    ///
    /// ```
    /// use core::ptr;
    ///
    /// use raw_parts::RawParts;
    /// use scolapasta_strbuf::Buf;
    ///
    /// let buf = Buf::from(b"abcde");
    /// let RawParts { ptr, length, capacity } = buf.into_raw_parts();
    ///
    /// unsafe {
    ///     ptr::write(ptr, b'A');
    ///     ptr::write(ptr.add(1), b'B');
    ///
    ///     let raw_parts = RawParts { ptr, length, capacity };
    ///     let rebuilt = Buf::from_raw_parts(raw_parts);
    ///
    ///     assert_eq!(rebuilt, b"ABcde");
    /// }
    /// ```
    #[inline]
    #[must_use]
    pub unsafe fn from_raw_parts(raw_parts: RawParts<u8>) -> Self {
        let inner = raw_parts.into_vec();
        Self { inner }
    }

    /// Decomposes a `Buf` into its raw components.
    ///
    /// Returns the raw pointer to the underlying bytes, the length of the
    /// buffer (in bytes), and the allocated capacity of the data (in bytes).
    ///
    /// After calling this function, the caller is responsible for the memory
    /// previously managed by the `Buf`. The only way to do this is to convert
    /// the raw pointer, length, and capacity back into a `Buf` with the
    /// [`from_raw_parts`] function, allowing the destructor to perform the cleanup.
    ///
    /// [`from_raw_parts`]: Self::from_raw_parts
    ///
    /// # Examples
    ///
    /// ```
    /// use core::ptr;
    ///
    /// use raw_parts::RawParts;
    /// use scolapasta_strbuf::Buf;
    ///
    /// let buf = Buf::from(b"abcde");
    /// let RawParts { ptr, length, capacity } = buf.into_raw_parts();
    ///
    /// unsafe {
    ///     ptr::write(ptr, b'A');
    ///     ptr::write(ptr.add(1), b'B');
    ///
    ///     let raw_parts = RawParts { ptr, length, capacity };
    ///     let rebuilt = Buf::from_raw_parts(raw_parts);
    ///
    ///     assert_eq!(rebuilt, b"ABcde");
    /// }
    /// ```
    #[inline]
    #[must_use]
    pub fn into_raw_parts(self) -> RawParts<u8> {
        RawParts::from_vec(self.inner)
    }

    /// Returns the total number of bytes the buffer can hold without
    /// reallocating.
    ///
    /// # Examples
    ///
    /// ```
    /// # #[cfg(not(feature = "nul-terminated"))]
    /// # {
    /// use scolapasta_strbuf::Buf;
    ///
    /// let mut buf = Buf::with_capacity(10);
    /// buf.push_byte(b'!');
    /// assert_eq!(buf.capacity(), 10);
    /// # }
    /// ```
    #[inline]
    #[must_use]
    pub fn capacity(&self) -> usize {
        self.inner.capacity()
    }

    /// Reserves capacity for at least `additional` more bytes to be inserted in
    /// the given `Buf`.
    ///
    /// The buffer may reserve more space to speculatively avoid frequent
    /// reallocations. After calling `reserve`, capacity will be greater than or
    /// equal to `self.len() + additional`. Does nothing if capacity is already
    /// sufficient.
    ///
    /// # Panics
    ///
    /// Panics if the new capacity exceeds `isize::MAX` bytes.
    ///
    /// # Examples
    ///
    /// ```
    /// use scolapasta_strbuf::Buf;
    ///
    /// let mut buf = Buf::from(b"@");
    /// buf.reserve(10);
    /// assert!(buf.capacity() >= 11);
    /// ```
    #[inline]
    pub fn reserve(&mut self, additional: usize) {
        self.inner.reserve(additional);
    }

    /// Reserves the minimum capacity for at least `additional` more bytes to
    /// be inserted in the given `Buf`.
    ///
    /// Unlike [`reserve`], this will not deliberately over-allocate to
    /// speculatively avoid frequent allocations. After calling `reserve_exact`,
    /// capacity will be greater than or equal to `self.len() + additional`.
    /// Does nothing if the capacity is already sufficient.
    ///
    /// Note that the allocator may give the buffer more space than it requests.
    /// Therefore, capacity can not be relied upon to be precisely minimal.
    /// Prefer [`reserve`] if future insertions are expected.
    ///
    /// [`reserve`]: Self::reserve
    ///
    /// # Panics
    ///
    /// Panics if the new capacity exceeds `isize::MAX` bytes.
    ///
    /// # Examples
    ///
    /// ```
    /// use scolapasta_strbuf::Buf;
    ///
    /// let mut buf = Buf::from(b"@");
    /// buf.reserve_exact(10);
    /// assert!(buf.capacity() >= 11);
    /// ```
    #[inline]
    pub fn reserve_exact(&mut self, additional: usize) {
        self.inner.reserve_exact(additional);
    }

    /// Tries to reserve capacity for at least `additional` more bytes to be
    /// inserted in the given `Buf`.
    ///
    /// The buffer may reserve more space to speculatively avoid frequent
    /// reallocations. After calling `try_reserve`, capacity will be greater
    /// than or equal to `self.len() + additional` if it returns `Ok(())`. Does
    /// nothing if capacity is already sufficient. This method preserves the
    /// byte contents even if an error occurs.
    ///
    /// # Errors
    ///
    /// If the capacity overflows, or the allocator reports a failure, then an
    /// error is returned.
    #[inline]
    pub fn try_reserve(&mut self, additional: usize) -> Result<(), TryReserveError> {
        self.inner.try_reserve(additional)
    }

    /// Tries to reserve the minimum capacity for at least `additional`
    /// elements to be inserted in the given `Buf`.
    ///
    /// Unlike [`try_reserve`], this will not deliberately over-allocate to
    /// speculatively avoid frequent allocations. After calling
    /// `try_reserve_exact`, capacity will be greater than or equal to
    /// `self.len() + additional` if it returns `Ok(())`. Does nothing if the
    /// capacity is already sufficient.
    ///
    /// Note that the allocator may give the buffer more space than it requests.
    /// Therefore, capacity can not be relied upon to be precisely minimal.
    /// Prefer [`try_reserve`] if future insertions are expected.
    ///
    /// [`try_reserve`]: Self::try_reserve
    ///
    /// # Errors
    ///
    /// If the capacity overflows, or the allocator reports a failure, then an
    /// error is returned.
    #[inline]
    pub fn try_reserve_exact(&mut self, additional: usize) -> Result<(), TryReserveError> {
        self.inner.try_reserve_exact(additional)
    }

    /// Shrinks the capacity of the buffer as much as possible.
    ///
    /// It will drop down as close as possible to the length but the allocator
    /// may still inform the buffer that there is space for a few more bytes.
    ///
    /// # Examples
    ///
    /// ```
    /// # #[cfg(not(feature = "nul-terminated"))]
    /// # {
    /// use scolapasta_strbuf::Buf;
    ///
    /// let mut buf = Buf::with_capacity(10);
    /// buf.extend(b"123");
    /// assert_eq!(buf.capacity(), 10);
    /// buf.shrink_to(4);
    /// assert!(buf.capacity() >= 4);
    /// buf.shrink_to_fit();
    /// assert!(buf.capacity() >= 3);
    /// # }
    /// ```
    #[inline]
    pub fn shrink_to_fit(&mut self) {
        self.inner.shrink_to_fit();
    }

    /// Shrinks the capacity of the buffer with a lower bound.
    ///
    /// The capacity will remain at least as large as both the length and the
    /// supplied value.
    ///
    /// If the current capacity is less than the lower limit, this is a no-op.
    ///
    /// # Examples
    ///
    /// ```
    /// # #[cfg(not(feature = "nul-terminated"))]
    /// # {
    /// use scolapasta_strbuf::Buf;
    ///
    /// let mut buf = Buf::with_capacity(10);
    /// buf.extend(b"123");
    /// assert_eq!(buf.capacity(), 10);
    /// buf.shrink_to(4);
    /// assert!(buf.capacity() >= 4);
    /// buf.shrink_to(0);
    /// assert!(buf.capacity() >= 3);
    /// # }
    /// ```
    #[inline]
    pub fn shrink_to(&mut self, min_capacity: usize) {
        self.inner.shrink_to(min_capacity);
    }

    /// Converts the buffer into [`Box<[u8]>`][owned slice].
    ///
    /// If the buffer has excess capacity, its bytes will be moved into a
    /// newly-allocated buffer with exactly the right capacity.
    ///
    /// [owned slice]: Box
    ///
    /// # Examples
    ///
    /// ```
    /// use scolapasta_strbuf::Buf;
    ///
    /// let buf = Buf::from(b"123");
    ///
    /// let slice = buf.into_boxed_slice();
    /// ```
    ///
    /// Any excess capacity is removed:
    ///
    /// ```
    /// # #[cfg(not(feature = "nul-terminated"))]
    /// # {
    /// use scolapasta_strbuf::Buf;
    ///
    /// let mut buf = Buf::with_capacity(10);
    /// buf.extend(b"123");
    ///
    /// assert_eq!(buf.capacity(), 10);
    /// let slice = buf.into_boxed_slice();
    /// assert_eq!(slice.into_vec().capacity(), 3);
    /// # }
    /// ```
    #[inline]
    #[must_use]
    pub fn into_boxed_slice(self) -> Box<[u8]> {
        self.inner.into_boxed_slice()
    }

    #[inline]
    pub fn truncate(&mut self, len: usize) {
        self.inner.truncate(len);
    }

    #[inline]
    #[must_use]
    pub fn as_slice(&self) -> &[u8] {
        self.inner.as_slice()
    }

    #[inline]
    pub fn as_mut_slice(&mut self) -> &mut [u8] {
        self.inner.as_mut_slice()
    }

    #[inline]
    #[must_use]
    pub fn as_ptr(&self) -> *const u8 {
        self.inner.as_ptr()
    }

    #[inline]
    #[must_use]
    pub fn as_mut_ptr(&mut self) -> *mut u8 {
        self.inner.as_mut_ptr()
    }

    #[inline]
    pub unsafe fn set_len(&mut self, new_len: usize) {
        self.inner.set_len(new_len);
    }

    #[inline]
    pub fn insert(&mut self, index: usize, element: u8) {
        self.inner.insert(index, element);
    }

    #[inline]
    pub fn remove(&mut self, index: usize) -> u8 {
        self.inner.remove(index)
    }

    #[inline]
    pub fn retain<F>(&mut self, f: F)
    where
        F: FnMut(&u8) -> bool,
    {
        self.inner.retain(f);
    }

    #[inline]
    pub fn pop_byte(&mut self) -> Option<u8> {
        self.inner.pop()
    }

    #[inline]
    pub fn clear(&mut self) {
        self.inner.clear();
    }

    #[inline]
    #[must_use]
    pub fn len(&self) -> usize {
        self.inner.len()
    }

    #[inline]
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.inner.is_empty()
    }

    #[inline]
    pub fn resize(&mut self, new_len: usize, value: u8) {
        self.inner.resize(new_len, value);
    }

    #[inline]
    pub fn extend_from_slice(&mut self, other: &[u8]) {
        self.inner.extend_from_slice(other);
    }
}

/// Implementation of useful extension methods from [`bstr::ByteVec`].
impl Buf {
    #[inline]
    pub fn push_byte(&mut self, byte: u8) {
        self.inner.push_byte(byte);
    }

    #[inline]
    pub fn push_char(&mut self, ch: char) {
        self.inner.push_char(ch);
    }

    #[inline]
    pub fn push_str<B: AsRef<[u8]>>(&mut self, bytes: B) {
        self.inner.push_str(bytes);
    }
}

#[cfg(feature = "std")]
impl Write for Buf {
    #[inline]
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        self.inner.write(buf)
    }

    #[inline]
    fn flush(&mut self) -> io::Result<()> {
        self.inner.flush()
    }

    #[inline]
    fn write_vectored(&mut self, bufs: &[IoSlice<'_>]) -> io::Result<usize> {
        self.inner.write_vectored(bufs)
    }

    #[inline]
    fn write_all(&mut self, buf: &[u8]) -> io::Result<()> {
        self.inner.write_all(buf)
    }

    #[inline]
    fn write_fmt(&mut self, fmt: Arguments<'_>) -> io::Result<()> {
        self.inner.write_fmt(fmt)
    }
}
