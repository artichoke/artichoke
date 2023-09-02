#![allow(clippy::missing_panics_doc)]

use alloc::borrow::Cow;
use alloc::boxed::Box;
use alloc::collections::TryReserveError;
use alloc::string::String;
use alloc::vec::{IntoIter, Vec};
use core::borrow::{Borrow, BorrowMut};
use core::fmt;
use core::ops::{Deref, DerefMut};
use core::slice::{Iter, IterMut};
#[cfg(feature = "std")]
use std::io::{self, IoSlice, Write};

use raw_parts::RawParts;

/// Ensure the given `Vec` can be used safely by C code as a string buffer.
///
/// mruby C code assumes that all string buffers it allocates have at least one
/// extra byte trailing the requested capacity AND that said byte is the NUL
/// byte (`b'\0'` or `0`).
///
/// This function MUST be called by all APIs which may modify the inner `Vec`.
///
/// This function produces a stronger guarantee than that provided by mruby: the
/// first AND last bytes of the spare capacity trailing the `Vec` will be the
/// NUL byte.
fn ensure_nul_terminated(vec: &mut Vec<u8>) -> Result<(), TryReserveError> {
    const NUL_BYTE: u8 = 0;

    let spare_capacity = vec.spare_capacity_mut();
    // If the vec has spare capacity, set the first and last bytes to NUL.
    //
    // See:
    //
    // - https://github.com/artichoke/artichoke/pull/1976#discussion_r932782264
    // - https://github.com/artichoke/artichoke/blob/16c869a9ad29acfe143bfcc011917ef442ccac54/artichoke-backend/vendor/mruby/src/string.c#L36-L38
    match spare_capacity {
        [] => {}
        [next] => {
            next.write(NUL_BYTE);
            return Ok(());
        }
        [head, .., tail] => {
            head.write(NUL_BYTE);
            tail.write(NUL_BYTE);
            return Ok(());
        }
    }
    // Else `vec.len == vec.capacity`, so reserve an extra byte.
    vec.try_reserve_exact(1)?;
    let spare_capacity = vec.spare_capacity_mut();
    match spare_capacity {
        [] => unreachable!("Vec should have spare capacity"),
        [next] => {
            next.write(NUL_BYTE);
        }
        [head, .., tail] => {
            head.write(NUL_BYTE);
            tail.write(NUL_BYTE);
        }
    }
    Ok(())
}

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
/// In addition to the guarantees of the underlying `Vec`, `Buf` is guaranteed
/// to have a NUL-terminated allocation. All `Buf`s will have spare capacity.
/// The first and last bytes of that spare capacity will be the NUL byte.
///
/// `Buf` does not expose any APIs, such as mutable access to the underlying
/// `Vec`, that allow violating this invariant. This variant is even upheld by
/// unsafe APIs such as [`set_len`].
///
/// ```
/// # #[cfg(feature = "nul-terminated")]
/// # {
/// use scolapasta_strbuf::Buf;
///
/// let buf = Buf::new();
/// assert_eq!(buf.capacity(), 1);
///
/// let mut inner = buf.into_inner();
/// let spare = inner.spare_capacity_mut();
/// assert!(!spare.is_empty());
/// assert_eq!(unsafe { spare.first().unwrap().assume_init() }, 0);
/// assert_eq!(unsafe { spare.last().unwrap().assume_init() }, 0);
/// # }
/// ```
///
/// [`Vec<u8>`]: Vec
/// [`String`]: https://ruby-doc.org/3.2.0/String.html
/// [`Index`]: core::ops::Index
/// [guarantees as a `Vec`]: https://doc.rust-lang.org/std/vec/struct.Vec.html#guarantees
/// [vec-docs]: mod@alloc::vec
/// [`set_len`]: Self::set_len
#[repr(transparent)]
#[derive(Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct Buf {
    inner: Vec<u8>,
}

impl Buf {
    /// Consume this buffer and return its inner [`Vec<u8>`].
    ///
    /// # Examples
    ///
    /// ```
    /// use scolapasta_strbuf::Buf;
    ///
    /// let buf = Buf::from(b"abc");
    /// let vec: Vec<u8> = buf.into_inner();
    /// assert_eq!(vec, b"abc");
    /// ```
    ///
    /// [`Vec<u8>`]: Vec
    #[inline]
    #[must_use]
    pub fn into_inner(self) -> Vec<u8> {
        self.inner
    }
}

impl Default for Buf {
    #[inline]
    fn default() -> Self {
        Self::new()
    }
}

impl Clone for Buf {
    #[inline]
    fn clone(&self) -> Self {
        let vec = self.inner.clone();
        Self::from(vec)
    }
}

impl From<Vec<u8>> for Buf {
    #[inline]
    fn from(mut vec: Vec<u8>) -> Self {
        ensure_nul_terminated(&mut vec).expect("alloc failure");
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
        // SAFETY: the mutable reference given out is a slice, NOT the
        // underlying `Vec`, so the allocation cannot change size.
        &mut self.inner
    }
}

impl FromIterator<u8> for Buf {
    #[inline]
    fn from_iter<T>(iter: T) -> Self
    where
        T: IntoIterator<Item = u8>,
    {
        let inner = iter.into_iter().collect::<Vec<u8>>();
        Self::from(inner)
    }
}

impl Extend<u8> for Buf {
    #[inline]
    fn extend<I: IntoIterator<Item = u8>>(&mut self, iter: I) {
        self.inner.extend(iter);
        ensure_nul_terminated(&mut self.inner).expect("alloc failure");
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
    /// The buffer will allocate one byte to maintain its NUL termination
    /// invariant.
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
        let inner = Vec::with_capacity(1);
        Self::from(inner)
    }

    /// Constructs a new, empty `Buf` with at least the specified capacity.
    ///
    /// The buffer will be able to hold at least `capacity` bytes without
    /// reallocating. This method is allowed to allocate for more elements than
    /// `capacity`. If `capacity` is 0, the buffer will allocate 1 byte to
    /// maintain its NUL termination invariant.
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
        let capacity = capacity.checked_add(1).expect("capacity overflow");
        let inner = Vec::with_capacity(capacity);
        Self::from(inner)
    }

    /// Creates a `Buf` directly from a pointer, a capacity, and a length.
    ///
    /// Reconstructing the buffer may cause a reallocation to maintain the
    /// buffer's NUL termination invariant.
    ///
    /// # Safety
    ///
    /// This is highly unsafe, due to the number of invariants that aren't
    /// checked.
    ///
    /// Refer to the safety documentation for [`Vec::from_raw_parts`] for more
    /// details.
    ///
    /// In addition to the safety invariants of `Vec`, `Buf` has the additional
    /// requirement that callers ensure the spare capacity of the allocation
    /// referred to by `ptr` is NUL terminated at offset `length` and `capacity`.
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
        // SAFETY: Callers ensure that the raw parts safety invariants are
        // upheld.
        let inner = unsafe { raw_parts.into_vec() };
        Self::from(inner)
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
    /// # #[cfg(feature = "nul-terminated")]
    /// # {
    /// use scolapasta_strbuf::Buf;
    ///
    /// let mut buf = Buf::with_capacity(10);
    /// buf.push_byte(b'!');
    /// assert_eq!(buf.capacity(), 11);
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
        ensure_nul_terminated(&mut self.inner).expect("alloc failure");
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
        ensure_nul_terminated(&mut self.inner).expect("alloc failure");
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
        let additional = additional.checked_add(1).unwrap_or(additional);
        self.inner.try_reserve(additional)?;
        ensure_nul_terminated(&mut self.inner)?;
        Ok(())
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
        let additional = additional.checked_add(1).unwrap_or(additional);
        self.inner.try_reserve_exact(additional)?;
        ensure_nul_terminated(&mut self.inner)?;
        Ok(())
    }

    /// Shrinks the capacity of the buffer as much as possible while maintaining
    /// its NUL termination invariant.
    ///
    /// It will drop down as close as possible to the length but the allocator
    /// may still inform the buffer that there is space for a few more bytes.
    ///
    /// # Examples
    ///
    /// ```
    /// # #[cfg(feature = "nul-terminated")]
    /// # {
    /// use scolapasta_strbuf::Buf;
    ///
    /// let mut buf = Buf::with_capacity(10);
    /// buf.extend(b"123");
    /// assert_eq!(buf.capacity(), 11);
    /// buf.shrink_to(4);
    /// assert!(buf.capacity() >= 4);
    /// buf.shrink_to_fit();
    /// assert!(buf.capacity() >= 4);
    /// # }
    /// ```
    #[inline]
    pub fn shrink_to_fit(&mut self) {
        self.inner.shrink_to_fit();
        ensure_nul_terminated(&mut self.inner).expect("alloc failure");
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
    /// # #[cfg(feature = "nul-terminated")]
    /// # {
    /// use scolapasta_strbuf::Buf;
    ///
    /// let mut buf = Buf::with_capacity(10);
    /// buf.extend(b"123");
    /// assert_eq!(buf.capacity(), 11);
    /// buf.shrink_to(4);
    /// assert!(buf.capacity() >= 4);
    /// buf.shrink_to(0);
    /// assert!(buf.capacity() >= 4);
    /// # }
    /// ```
    #[inline]
    pub fn shrink_to(&mut self, min_capacity: usize) {
        self.inner.shrink_to(min_capacity);
        ensure_nul_terminated(&mut self.inner).expect("alloc failure");
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
    /// # #[cfg(feature = "nul-terminated")]
    /// # {
    /// use scolapasta_strbuf::Buf;
    ///
    /// let mut buf = Buf::with_capacity(10);
    /// buf.extend(b"123");
    ///
    /// assert_eq!(buf.capacity(), 11);
    /// let slice = buf.into_boxed_slice();
    /// assert_eq!(slice.into_vec().capacity(), 3);
    /// # }
    /// ```
    #[inline]
    #[must_use]
    pub fn into_boxed_slice(self) -> Box<[u8]> {
        self.inner.into_boxed_slice()
    }

    /// Shorten the buffer, keeping the first `len` bytes and dropping the rest.
    ///
    /// If `len` is greater than the buffer's current length, this has no
    /// effect.
    ///
    /// Note that this method has no effect on the allocated capacity of the
    /// buffer.
    ///
    /// # Examples
    ///
    /// Truncating a five byte buffer to two bytes:
    ///
    /// ```
    /// use scolapasta_strbuf::Buf;
    ///
    /// let mut buf = Buf::from(b"12345");
    /// buf.truncate(2);
    /// assert_eq!(buf, b"12");
    /// ```
    ///
    /// No truncation occurs when `len` is greater than the buffer's current
    /// length:
    ///
    /// ```
    /// use scolapasta_strbuf::Buf;
    ///
    /// let mut buf = Buf::from(b"123");
    /// buf.truncate(8);
    /// assert_eq!(buf, b"123");
    /// ```
    ///
    /// Truncating when `len == 0` is equivalent to calling the [`clear`]
    /// method.
    ///
    /// ```
    /// use scolapasta_strbuf::Buf;
    ///
    /// let mut buf = Buf::from(b"123");
    /// buf.truncate(0);
    /// assert_eq!(buf, b"");
    /// ```
    ///
    /// [`clear`]: Self::clear
    #[inline]
    pub fn truncate(&mut self, len: usize) {
        self.inner.truncate(len);
        ensure_nul_terminated(&mut self.inner).expect("alloc failure");
    }

    /// Extract a slice containing the entire buffer.
    ///
    /// Equivalent to `&buf[..]`.
    #[inline]
    #[must_use]
    pub fn as_slice(&self) -> &[u8] {
        self.inner.as_slice()
    }

    /// Extract a mutable slice containing the entire buffer.
    ///
    /// Equivalent to `&mut buf[..]`.
    #[inline]
    #[must_use]
    pub fn as_mut_slice(&mut self) -> &mut [u8] {
        self.inner.as_mut_slice()
    }

    /// Return a raw pointer to the buffer's inner vec, or a dangling raw
    /// pointer valid for zero sized reads if the buffer didn't allocate.
    ///
    /// The caller must ensure correct use of the pointer. See [`Vec::as_ptr`]
    /// for more details.
    ///
    /// Callers must also ensure that the NUL termination invariant of the
    /// buffer is maintained is the returned pointer is used for writes.
    #[inline]
    #[must_use]
    pub fn as_ptr(&self) -> *const u8 {
        self.inner.as_ptr()
    }

    /// Return an unsafe mutable pointer to the buffer's inner vec, or a
    /// dangling raw pointer valid for zero sized reads if the buffer didn't
    /// allocate.
    ///
    /// The caller must ensure correct use of the pointer. See [`Vec::as_mut_ptr`]
    /// for more details.
    ///
    /// Callers must also ensure that the NUL termination invariant of the
    /// buffer is maintained is the returned pointer is used for writes.
    #[inline]
    #[must_use]
    pub fn as_mut_ptr(&mut self) -> *mut u8 {
        self.inner.as_mut_ptr()
    }

    /// Force the length of the buffer to `new_len`.
    ///
    /// This is a low-level operation that maintains none of the normal
    /// invariants of the type. Normally changing the length of a vector is done
    /// using one of the safe operations instead, such as [`truncate`],
    /// [`resize`], [`extend`], or [`clear`].
    ///
    /// [`truncate`]: Self::truncate
    /// [`resize`]: Self::resize
    /// [`extend`]: Self::extend
    /// [`clear`]: Self::clear
    ///
    /// # Safety
    ///
    /// - `new_len` must be less than or equal to [`capacity()`].
    /// - The elements at `old_len..new_len` must be initialized.
    ///
    /// [`capacity()`]: Self::capacity
    #[inline]
    pub unsafe fn set_len(&mut self, new_len: usize) {
        // SAFETY: Caller has guaranteed the safety invariants of `Vec::set_len`
        // are upheld.
        unsafe {
            self.inner.set_len(new_len);
        }
        ensure_nul_terminated(&mut self.inner).expect("alloc failure");
    }

    /// Insert a byte at position `index` within the buffer, shifting all
    /// elements after it to the right.
    ///
    /// # Panics
    ///
    /// Panics if `index > len`.
    ///
    /// # Examples
    ///
    /// ```
    /// use scolapasta_strbuf::Buf;
    ///
    /// let mut buf = Buf::from(b"123");
    /// buf.insert(1, b'4');
    /// assert_eq!(buf, b"1423");
    /// buf.insert(4, b'5');
    /// assert_eq!(buf, b"14235");
    /// ```
    #[inline]
    pub fn insert(&mut self, index: usize, element: u8) {
        self.inner.insert(index, element);
        ensure_nul_terminated(&mut self.inner).expect("alloc failure");
    }

    /// Remove and return the byte at position `index` within the buffer,
    /// shifting all bytes after it to the left.
    ///
    /// **Note**: Because this shifts over the remaining bytes, it has a
    /// worst-case performance of *O*(*n*).
    ///
    /// # Panics
    ///
    /// Panics if `index` is out of bounds.
    ///
    /// # Examples
    ///
    /// ```
    /// use scolapasta_strbuf::Buf;
    ///
    /// let mut buf = Buf::from(b"123");
    /// assert_eq!(buf.remove(1), b'2');
    /// assert_eq!(buf, b"13");
    /// ```
    #[inline]
    #[track_caller]
    pub fn remove(&mut self, index: usize) -> u8 {
        let removed = self.inner.remove(index);
        ensure_nul_terminated(&mut self.inner).expect("alloc failure");
        removed
    }

    /// Retain only the bytes specified by the predicate.
    ///
    /// In other words, remove all bytes `b` for which `f(&b)` returns `false`.
    /// This method operates in place, visiting each byte exactly once in the
    /// original order, and preserves the order of the retained bytes.
    ///
    /// # Examples
    ///
    /// ```
    /// use scolapasta_strbuf::Buf;
    ///
    /// let mut buf = Buf::from(b"abc, 123!");
    /// buf.retain(|&b| b.is_ascii_alphanumeric());
    /// assert_eq!(buf, b"abc123");
    /// ```
    ///
    /// Because the bytes are visited exactly once in the original order,
    /// external state may be used to decide which elements to keep.
    ///
    /// ```
    /// use scolapasta_strbuf::Buf;
    ///
    /// let mut buf = Buf::from(b"abc, 123!");
    /// let mut seen_space = false;
    /// buf.retain(|&b| {
    ///     if seen_space {
    ///         true
    ///     } else {
    ///         seen_space = b.is_ascii_whitespace();
    ///         false
    ///     }
    /// });
    /// assert_eq!(buf, b"123!");
    /// ```
    #[inline]
    pub fn retain<F>(&mut self, f: F)
    where
        F: FnMut(&u8) -> bool,
    {
        self.inner.retain(f);
        ensure_nul_terminated(&mut self.inner).expect("alloc failure");
    }

    /// Remove the last byte from the buffer and return it, or [`None`] if the
    /// buffer is empty.
    ///
    /// # Examples
    ///
    /// ```
    /// use scolapasta_strbuf::Buf;
    ///
    /// let mut buf = Buf::from(b"abc, 123!");
    /// assert_eq!(buf.pop_byte(), Some(b'!'));
    /// assert_eq!(buf, "abc, 123");
    /// ```
    #[inline]
    pub fn pop_byte(&mut self) -> Option<u8> {
        let popped = self.inner.pop();
        ensure_nul_terminated(&mut self.inner).expect("alloc failure");
        popped
    }

    /// Clear the buffer, removing all bytes.
    ///
    /// This method sets the length of the buffer to zero. Note that this method
    /// has no effect on the allocated capacity of the buffer.
    ///
    /// # Examples
    ///
    /// ```
    /// use scolapasta_strbuf::Buf;
    ///
    /// let mut buf = Buf::from(b"abc, 123!");
    /// let capacity = buf.capacity();
    ///
    /// buf.clear();
    ///
    /// assert!(buf.is_empty());
    /// assert_eq!(buf.capacity(), capacity);
    /// ```
    #[inline]
    pub fn clear(&mut self) {
        self.inner.clear();
        ensure_nul_terminated(&mut self.inner).expect("alloc failure");
    }

    /// Return the number of bytes in the buffer, also referred to as its
    /// 'length' or 'bytesize'.
    ///
    /// # Examples
    ///
    /// ```
    /// use scolapasta_strbuf::Buf;
    ///
    /// let buf = Buf::from(b"abc");
    /// assert_eq!(buf.len(), 3);
    /// ```
    #[inline]
    #[must_use]
    pub fn len(&self) -> usize {
        self.inner.len()
    }

    /// Return `true` if the buffer has empty byte content.
    ///
    /// # Examples
    ///
    /// ```
    /// use scolapasta_strbuf::Buf;
    ///
    /// let mut buf = Buf::new();
    /// assert!(buf.is_empty());
    ///
    /// buf.push_byte(b'!');
    /// assert!(!buf.is_empty());
    /// ```
    #[inline]
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.inner.is_empty()
    }

    /// Resize the `Buf` in-place so that `len` is equal to `new_len`.
    ///
    /// If `new_len` is greater than `len`, the `Vec` is extended by the
    /// difference, with each additional slot filled with `value`. If `new_len`
    /// is less than `len`, the `Buf` is simply truncated.
    ///
    /// # Examples
    ///
    /// ```
    /// use scolapasta_strbuf::Buf;
    ///
    /// let mut buf = Buf::from(b"hello");
    /// buf.resize(8, b'!');
    /// assert_eq!(buf, b"hello!!!");
    ///
    /// let mut buf = Buf::from("wxyz");
    /// buf.resize(2, b'.');
    /// assert_eq!(buf, b"wx");
    /// ```
    #[inline]
    pub fn resize(&mut self, new_len: usize, value: u8) {
        self.inner.resize(new_len, value);
        ensure_nul_terminated(&mut self.inner).expect("alloc failure");
    }

    /// Copy and append all bytes in the given slice to the `Buf`.
    ///
    /// Iterate over the slice `other`, copy each byte, and then append
    /// it to this `Buf`. The `other` slice is traversed in-order.
    ///
    /// # Examples
    ///
    /// ```
    /// use scolapasta_strbuf::Buf;
    ///
    /// let mut buf = Buf::from(b"h");
    /// buf.extend_from_slice(b"ello world");
    /// assert_eq!(buf, b"hello world");
    /// ```
    #[inline]
    pub fn extend_from_slice(&mut self, other: &[u8]) {
        self.inner.extend_from_slice(other);
        ensure_nul_terminated(&mut self.inner).expect("alloc failure");
    }
}

/// Implementation of useful extension methods from [`bstr::ByteVec`].
///
/// [`bstr::ByteVec`]: https://docs.rs/bstr/latest/bstr/trait.ByteVec.html
impl Buf {
    /// Append the given byte to the end of this buffer.
    ///
    /// Note that this is equivalent to the generic [`Vec::push`] method. This
    /// method is provided to permit callers to explicitly differentiate
    /// between pushing bytes, codepoints and strings.
    ///
    /// # Examples
    ///
    /// ```
    /// use scolapasta_strbuf::Buf;
    ///
    /// let mut buf = Buf::from(b"abc");
    /// buf.push_byte(b'\xF0');
    /// buf.push_byte(b'\x9F');
    /// buf.push_byte(b'\xA6');
    /// buf.push_byte(b'\x80');
    /// assert_eq!(buf, "abc🦀");
    /// ```
    #[inline]
    pub fn push_byte(&mut self, byte: u8) {
        self.inner.push(byte);
        ensure_nul_terminated(&mut self.inner).expect("alloc failure");
    }

    /// Append the given [`char`] to the end of the buffer.
    ///
    /// The given `char` is encoded to its UTF-8 byte sequence which is appended
    /// to the buffer.
    ///
    /// # Examples
    ///
    /// ```
    /// use scolapasta_strbuf::Buf;
    ///
    /// let mut buf = Buf::from(b"abc");
    /// buf.push_char('🦀');
    /// assert_eq!(buf, "abc🦀");
    /// ```
    #[inline]
    pub fn push_char(&mut self, ch: char) {
        let mut buf = [0; 4];
        let s = ch.encode_utf8(&mut buf[..]);
        self.push_str(s);
    }

    /// Append the given slice to the end of this buffer.
    ///
    /// This method accepts any type that be converted to a `&[u8]`. This
    /// includes, but is not limited to, `&str`, `&Buf`, and of course, `&[u8]`
    /// itself.
    ///
    /// # Examples
    ///
    /// ```
    /// use scolapasta_strbuf::Buf;
    ///
    /// let mut buf = Buf::from(b"abc");
    /// buf.push_str("🦀");
    /// assert_eq!(buf, "abc🦀");
    ///
    /// buf.push_str(b"\xF0\x9F\xA6\x80");
    /// assert_eq!(buf, "abc🦀🦀");
    /// ```
    #[inline]
    pub fn push_str<B: AsRef<[u8]>>(&mut self, bytes: B) {
        self.extend_from_slice(bytes.as_ref());
    }
}

impl fmt::Write for Buf {
    #[inline]
    fn write_str(&mut self, s: &str) -> fmt::Result {
        self.push_str(s);
        Ok(())
    }

    #[inline]
    fn write_char(&mut self, c: char) -> fmt::Result {
        self.push_char(c);
        Ok(())
    }
}

#[cfg(feature = "std")]
impl Write for Buf {
    #[inline]
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        let result = self.inner.write(buf);
        ensure_nul_terminated(&mut self.inner).expect("alloc failure");
        result
    }

    #[inline]
    fn write_vectored(&mut self, bufs: &[IoSlice<'_>]) -> io::Result<usize> {
        let result = self.inner.write_vectored(bufs);
        ensure_nul_terminated(&mut self.inner).expect("alloc failure");
        result
    }

    #[inline]
    fn write_all(&mut self, buf: &[u8]) -> io::Result<()> {
        let result = self.inner.write_all(buf);
        ensure_nul_terminated(&mut self.inner).expect("alloc failure");
        result
    }

    #[inline]
    fn flush(&mut self) -> io::Result<()> {
        let result = self.inner.flush();
        ensure_nul_terminated(&mut self.inner).expect("alloc failure");
        result
    }
}

#[cfg(test)]
#[allow(clippy::undocumented_unsafe_blocks)]
mod tests {
    use alloc::string::String;
    use alloc::vec::Vec;

    use quickcheck::quickcheck;
    use raw_parts::RawParts;

    use super::{ensure_nul_terminated, Buf};

    #[must_use]
    fn is_nul_terminated(bytes: &mut Vec<u8>) -> bool {
        let spare_capacity = bytes.spare_capacity_mut();
        if spare_capacity.is_empty() {
            return false;
        }

        let first = unsafe { spare_capacity.first().unwrap().assume_init() };
        if first != 0 {
            return false;
        }

        let last = unsafe { spare_capacity.last().unwrap().assume_init() };
        if last != 0 {
            return false;
        }
        true
    }

    #[test]
    fn default_is_new() {
        assert_eq!(Buf::default(), Buf::new());
    }

    #[test]
    fn extra_capa_is_not_included_in_len() {
        let buf = Buf::new();
        assert!(buf.is_empty());
        assert_eq!(buf.len(), 0);

        let buf = Buf::with_capacity(0);
        assert!(buf.is_empty());
        assert_eq!(buf.len(), 0);

        let buf = Buf::with_capacity(100);
        assert!(buf.is_empty());
        assert_eq!(buf.len(), 0);
    }

    #[test]
    fn clone_is_equal() {
        let buf = Buf::from("abc");
        assert_eq!(buf, buf.clone());
    }

    #[test]
    fn try_reserve_overflow_is_err() {
        let mut buf = Buf::new();
        assert!(buf.try_reserve(usize::MAX).is_err());
        assert!(buf.is_empty());
        assert_eq!(buf.len(), 0);
    }

    #[test]
    fn try_reserve_exact_overflow_is_err() {
        let mut buf = Buf::new();
        assert!(buf.try_reserve_exact(usize::MAX).is_err());
        assert!(buf.is_empty());
        assert_eq!(buf.len(), 0);
    }

    #[test]
    fn try_reserve_zero_is_ok() {
        let mut buf = Buf::new();
        assert!(buf.try_reserve(0).is_ok());
        assert_eq!(buf.capacity(), 1);
        assert!(buf.is_empty());
        assert_eq!(buf.len(), 0);
    }

    #[test]
    fn try_reserve_exact_zero_is_ok() {
        let mut buf = Buf::new();
        assert!(buf.try_reserve_exact(0).is_ok());
        assert_eq!(buf.capacity(), 1);
        assert!(buf.is_empty());
        assert_eq!(buf.len(), 0);
    }

    #[test]
    fn test_ensure_nul_terminated_default() {
        let buf = Buf::default();
        let mut bytes = buf.into_inner();
        assert!(is_nul_terminated(&mut bytes));
    }

    #[test]
    fn test_ensure_nul_terminated_new() {
        let buf = Buf::new();
        let mut bytes = buf.into_inner();
        assert!(is_nul_terminated(&mut bytes));
    }

    #[test]
    fn test_ensure_nul_terminated_with_capacity() {
        let capacities = [0_usize, 1, 2, 3, 4, 19, 280, 499, 1024, 4096, 4099];
        for capa in capacities {
            let buf = Buf::with_capacity(capa);
            let mut bytes = buf.into_inner();
            assert!(is_nul_terminated(&mut bytes), "failed for capacity {capa}");
        }
    }

    quickcheck! {
        fn test_ensure_nul_terminated(bytes: Vec<u8>) -> bool {
            let mut bytes = bytes;
            ensure_nul_terminated(&mut bytes).unwrap();
            is_nul_terminated(&mut bytes)
        }

        fn test_ensure_nul_terminated_after_shrink(bytes: Vec<u8>) -> bool {
            let mut bytes = bytes;
            bytes.shrink_to_fit();
            ensure_nul_terminated(&mut bytes).unwrap();
            is_nul_terminated(&mut bytes)
        }

        fn test_ensure_nul_terminated_from_vec(bytes: Vec<u8>) -> bool {
            let buf = Buf::from(bytes);
            let mut bytes = buf.into_inner();
            is_nul_terminated(&mut bytes)
        }

        fn test_ensure_nul_terminated_from_buf(bytes: Vec<u8>) -> bool {
            let buf = Buf::from(bytes);
            let mut bytes = Vec::from(buf);
            is_nul_terminated(&mut bytes)
        }

        fn test_ensure_nul_terminated_after_clone(bytes: Vec<u8>) -> bool {
            let buf = Buf::from(bytes);
            #[allow(clippy::redundant_clone)]
            let buf = buf.clone();
            let mut bytes = buf.into_inner();
            is_nul_terminated(&mut bytes)
        }

        fn test_ensure_nul_terminated_from_iterator(bytes: Vec<u8>) -> bool {
            #[allow(clippy::from_iter_instead_of_collect)]
            let buf = Buf::from_iter(bytes.into_iter());
            let mut bytes = buf.into_inner();
            is_nul_terminated(&mut bytes)
        }

        fn test_ensure_nul_terminated_collect(bytes: Vec<u8>) -> bool {
            let buf = bytes.into_iter().collect::<Buf>();
            let mut bytes = buf.into_inner();
            is_nul_terminated(&mut bytes)
        }

        fn test_ensure_nul_terminated_after_extend(bytes: Vec<u8>, extend: Vec<u8>) -> bool {
            let mut buf = Buf::from(bytes);
            buf.extend(extend.into_iter());
            let mut bytes = buf.into_inner();
            is_nul_terminated(&mut bytes)
        }

        fn test_ensure_nul_terminated_from_raw_parts(bytes: Vec<u8>) -> bool {
            let raw_parts = RawParts::from_vec(bytes);
            let buf = unsafe { Buf::from_raw_parts(raw_parts) };
            let mut bytes = buf.into_inner();
            is_nul_terminated(&mut bytes)
        }

        fn test_ensure_nul_terminated_raw_parts_round_trip(bytes: Vec<u8>) -> bool {
            let buf = Buf::from(bytes);
            let raw_parts = buf.into_raw_parts();
            let buf = unsafe { Buf::from_raw_parts(raw_parts) };
            let mut bytes = buf.into_inner();
            is_nul_terminated(&mut bytes)
        }

        fn test_ensure_nul_terminated_reserve(bytes: Vec<u8>) -> bool {
            let additional = [0_usize, 1, 2, 3, 4, 19, 280, 499, 1024, 4096, 4099];
            for reserve in additional {
                let mut buf = Buf::from(bytes.clone());
                buf.reserve(reserve);
                let mut bytes = buf.into_inner();
                if !is_nul_terminated(&mut bytes) {
                    return false;
                }
            }
            true
        }

        fn test_ensure_nul_terminated_reserve_exact(bytes: Vec<u8>) -> bool {
            let additional = [0_usize, 1, 2, 3, 4, 19, 280, 499, 1024, 4096, 4099];
            for reserve in additional {
                let mut buf = Buf::from(bytes.clone());
                buf.reserve_exact(reserve);
                let mut bytes = buf.into_inner();
                if !is_nul_terminated(&mut bytes) {
                    return false;
                }
            }
            true
        }

        fn test_ensure_nul_terminated_try_reserve(bytes: Vec<u8>) -> bool {
            let additional = [0_usize, 1, 2, 3, 4, 19, 280, 499, 1024, 4096, 4099];
            for reserve in additional {
                let mut buf = Buf::from(bytes.clone());
                buf.try_reserve(reserve).unwrap();
                let mut bytes = buf.into_inner();
                if !is_nul_terminated(&mut bytes) {
                    return false;
                }
            }
            true
        }

        fn test_ensure_nul_terminated_try_reserve_exact(bytes: Vec<u8>) -> bool {
            let additional = [0_usize, 1, 2, 3, 4, 19, 280, 499, 1024, 4096, 4099];
            for reserve in additional {
                let mut buf = Buf::from(bytes.clone());
                buf.try_reserve_exact(reserve).unwrap();
                let mut bytes = buf.into_inner();
                if !is_nul_terminated(&mut bytes) {
                    return false;
                }
            }
            true
        }

        fn test_ensure_nul_terminated_shrink_to_fit(bytes: Vec<u8>) -> bool {
            let mut buf = Buf::from(bytes);
            buf.shrink_to_fit();
            let mut bytes = buf.into_inner();
            is_nul_terminated(&mut bytes)
        }

        fn test_ensure_nul_terminated_shrink_to(bytes: Vec<u8>, shrink_to: usize) -> bool {
            let mut buf = Buf::from(bytes);
            buf.shrink_to(shrink_to);
            let mut bytes = buf.into_inner();
            is_nul_terminated(&mut bytes)
        }

        fn test_ensure_nul_terminated_truncate(bytes: Vec<u8>, truncate_to: usize) -> bool {
            let mut buf = Buf::from(bytes);
            buf.truncate(truncate_to);
            let mut bytes = buf.into_inner();
            is_nul_terminated(&mut bytes)
        }

        fn test_ensure_nul_terminated_set_len(bytes: Vec<u8>) -> bool {
            let mut buf = Buf::from(bytes);
            unsafe {
                buf.set_len(0);
            }
            let mut bytes = buf.into_inner();
            is_nul_terminated(&mut bytes)
        }

        fn test_ensure_nul_terminated_insert_first(bytes: Vec<u8>) -> bool {
            if bytes.is_empty() {
                return true;
            }
            let mut buf = Buf::from(bytes);
            buf.insert(0, u8::MAX);
            let mut bytes = buf.into_inner();
            is_nul_terminated(&mut bytes)
        }

        fn test_ensure_nul_terminated_insert_past_end(bytes: Vec<u8>) -> bool {
            let mut buf = Buf::from(bytes);
            buf.insert(buf.len(), u8::MAX);
            let mut bytes = buf.into_inner();
            is_nul_terminated(&mut bytes)
        }

        fn test_ensure_nul_terminated_insert_last(bytes: Vec<u8>) -> bool {
            if bytes.is_empty() {
                return true;
            }
            let mut buf = Buf::from(bytes);
            buf.insert(buf.len() - 1, u8::MAX);
            let mut bytes = buf.into_inner();
            is_nul_terminated(&mut bytes)
        }

        fn test_ensure_nul_terminated_insert_interior(bytes: Vec<u8>) -> bool {
            if bytes.len() < 2 {
                return true;
            }
            let mut buf = Buf::from(bytes);
            buf.insert(buf.len() - 2, u8::MAX);
            let mut bytes = buf.into_inner();
            is_nul_terminated(&mut bytes)
        }

        fn test_ensure_nul_terminated_remove_first(bytes: Vec<u8>) -> bool {
            if bytes.is_empty() {
                return true;
            }
            let mut buf = Buf::from(bytes);
            buf.remove(0);
            let mut bytes = buf.into_inner();
            is_nul_terminated(&mut bytes)
        }

        fn test_ensure_nul_terminated_remove_last(bytes: Vec<u8>) -> bool {
            if bytes.is_empty() {
                return true;
            }
            let mut buf = Buf::from(bytes);
            buf.remove(buf.len() - 1);
            let mut bytes = buf.into_inner();
            is_nul_terminated(&mut bytes)
        }

        fn test_ensure_nul_terminated_remove_interior(bytes: Vec<u8>) -> bool {
            if bytes.len() < 2 {
                return true;
            }
            let mut buf = Buf::from(bytes);
            buf.remove(buf.len() - 2);
            let mut bytes = buf.into_inner();
            is_nul_terminated(&mut bytes)
        }

        fn test_ensure_nul_terminated_retain_all(bytes: Vec<u8>) -> bool {
            let mut buf = Buf::from(bytes);
            buf.retain(|_| true);
            let mut bytes = buf.into_inner();
            is_nul_terminated(&mut bytes)
        }

        fn test_ensure_nul_terminated_retain_none(bytes: Vec<u8>) -> bool {
            let mut buf = Buf::from(bytes);
            buf.retain(|_| false);
            let mut bytes = buf.into_inner();
            is_nul_terminated(&mut bytes)
        }

        fn test_ensure_nul_terminated_retain_some(bytes: Vec<u8>) -> bool {
            let mut idx = 0_usize;
            let mut buf = Buf::from(bytes);
            buf.retain(|_| {
                idx += 1;
                idx % 2 == 0
            });
            let mut bytes = buf.into_inner();
            is_nul_terminated(&mut bytes)
        }

        fn test_ensure_nul_terminated_pop(bytes: Vec<u8>) -> bool {
            if bytes.is_empty() {
                return true;
            }
            let mut buf = Buf::from(bytes);
            buf.pop_byte();
            let mut bytes = buf.into_inner();
            is_nul_terminated(&mut bytes)
        }

        fn test_ensure_nul_terminated_clear(bytes: Vec<u8>) -> bool {
            let mut buf = Buf::from(bytes);
            buf.clear();
            let mut bytes = buf.into_inner();
            is_nul_terminated(&mut bytes)
        }

        fn test_ensure_nul_terminated_resize(bytes: Vec<u8>) -> bool {
            let lengths = [0_usize, 1, 2, 3, 4, 19, 280, 499, 1024, 4096, 4099];
            for len in lengths {
                let mut buf = Buf::from(bytes.clone());
                buf.resize(len, u8::MAX);
                let mut bytes = buf.into_inner();
                if !is_nul_terminated(&mut bytes) {
                    return false;
                }
            }
            true
        }

        fn test_ensure_nul_terminated_extend_from_slice(bytes: Vec<u8>, other: Vec<u8>) -> bool {
            let mut buf = Buf::from(bytes);
            buf.extend_from_slice(&other);
            let mut bytes = buf.into_inner();
            is_nul_terminated(&mut bytes)
        }

        fn test_ensure_nul_terminated_push_byte(bytes: Vec<u8>, pushed: u8) -> bool {
            let mut buf = Buf::from(bytes);
            buf.push_byte(pushed);
            let mut bytes = buf.into_inner();
            is_nul_terminated(&mut bytes)
        }

        fn test_ensure_nul_terminated_push_char(bytes: Vec<u8>, pushed: char) -> bool {
            let mut buf = Buf::from(bytes);
            buf.push_char(pushed);
            let mut bytes = buf.into_inner();
            is_nul_terminated(&mut bytes)
        }

        fn test_ensure_nul_terminated_push_str(bytes: Vec<u8>, pushed: String) -> bool {
            let mut buf = Buf::from(bytes);
            buf.push_str(pushed);
            let mut bytes = buf.into_inner();
            is_nul_terminated(&mut bytes)
        }

        #[cfg(feature = "std")]
        fn test_ensure_nul_terminated_write(bytes: Vec<u8>, data: Vec<u8>) -> bool {
            use std::io::Write;

            let mut buf = Buf::from(bytes);
            let _written = buf.write(&data).unwrap();
            let mut bytes = buf.into_inner();
            is_nul_terminated(&mut bytes)
        }

        #[cfg(feature = "std")]
        fn test_ensure_nul_terminated_flush(bytes: Vec<u8>, data: Vec<u8>) -> bool {
            use std::io::Write;

            let mut buf = Buf::from(bytes);
            buf.write_all(&data).unwrap();
            buf.flush().unwrap();
            let mut bytes = buf.into_inner();
            is_nul_terminated(&mut bytes)
        }

        #[cfg(feature = "std")]
        fn test_ensure_nul_terminated_write_vectored(bytes: Vec<u8>, data1: Vec<u8>, data2: Vec<u8>) -> bool {
            use std::io::{IoSlice, Write};

            let mut buf = Buf::from(bytes);
            let _written = buf.write_vectored(&[IoSlice::new(&data1), IoSlice::new(&data2)]).unwrap();
            let mut bytes = buf.into_inner();
            is_nul_terminated(&mut bytes)
        }

        #[cfg(feature = "std")]
        fn test_ensure_nul_terminated_write_all(bytes: Vec<u8>, data: Vec<u8>) -> bool {
            use std::io::Write;

            let mut buf = Buf::from(bytes);
            buf.write_all(&data).unwrap();
            let mut bytes = buf.into_inner();
            is_nul_terminated(&mut bytes)
        }

        #[cfg(feature = "std")]
        fn test_ensure_nul_terminated_write_fmt(bytes: Vec<u8>, data: String) -> bool {
            use std::io::Write;

            let mut buf = Buf::from(bytes);
            buf.write_fmt(format_args!("{data}")).unwrap();
            let mut bytes = buf.into_inner();
            is_nul_terminated(&mut bytes)
        }
    }
}
