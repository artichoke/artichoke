#![warn(clippy::all)]
#![warn(clippy::pedantic)]
#![warn(clippy::cargo)]
#![cfg_attr(test, allow(clippy::non_ascii_literal))]
#![allow(unknown_lints)]
// TODO: warn on missing docs once crate is API-complete.
// #![warn(missing_docs)]
#![warn(missing_debug_implementations)]
#![warn(missing_copy_implementations)]
#![warn(rust_2018_idioms)]
#![warn(rust_2021_compatibility)]
#![warn(trivial_casts, trivial_numeric_casts)]
#![warn(unused_qualifications)]
#![warn(variant_size_differences)]
// Enable feature callouts in generated documentation:
// https://doc.rust-lang.org/beta/unstable-book/language-features/doc-cfg.html
//
// This approach is borrowed from tokio.
#![cfg_attr(docsrs, feature(doc_cfg))]
#![cfg_attr(docsrs, feature(doc_alias))]

//! A String object holds and manipulates an arbitrary sequence of bytes,
//! typically representing characters.

#![no_std]

// Ensure code blocks in `README.md` compile
#[cfg(doctest)]
#[doc = include_str!("../README.md")]
mod readme {}

extern crate alloc;
#[cfg(feature = "std")]
extern crate std;

use alloc::boxed::Box;
use alloc::vec::Vec;
use core::cmp::Ordering;
use core::fmt::{self};
use core::hash::{Hash, Hasher};
use core::ops::Range;
use core::slice::SliceIndex;
use core::str;

use bstr::ByteSlice;
#[doc(inline)]
#[cfg(feature = "casecmp")]
#[cfg_attr(feature = "docsrs", doc(cfg(feature = "casecmp")))]
pub use focaccia::CaseFold;
#[doc(inline)]
pub use raw_parts::RawParts;

mod center;
mod chars;
mod codepoints;
mod enc;
mod encoding;
mod eq;
mod impls;
mod inspect;
mod iter;
mod ord;

pub use center::{Center, CenterError};
pub use chars::Chars;
pub use codepoints::{Codepoints, CodepointsError, InvalidCodepointError};
use enc::EncodedString;
pub use encoding::{Encoding, InvalidEncodingError};
pub use inspect::Inspect;
pub use iter::{Bytes, IntoIter, Iter, IterMut};
pub use ord::OrdError;

#[derive(Default, Clone)]
pub struct String {
    inner: EncodedString,
}

impl fmt::Debug for String {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("String")
            .field("buf", &self.inner.as_slice().as_bstr())
            .field("encoding", &self.inner.encoding())
            .finish()
    }
}

impl Hash for String {
    fn hash<H: Hasher>(&self, hasher: &mut H) {
        // A `String`'s hash only depends on its byte contents.
        //
        // ```
        // [3.0.2] > s = "abc"
        // => "abc"
        // [3.0.2] > t = s.dup.force_encoding(Encoding::ASCII)
        // => "abc"
        // [3.0.2] > s.hash
        // => 3398383793005079442
        // [3.0.2] > t.hash
        // => 3398383793005079442
        // ```
        self.inner.as_slice().hash(hasher);
    }
}

impl PartialEq for String {
    fn eq(&self, other: &Self) -> bool {
        // Equality only depends on each `String`'s byte contents.
        //
        // ```
        // [3.0.2] > s = "abc"
        // => "abc"
        // [3.0.2] > t = s.dup.force_encoding(Encoding::ASCII)
        // => "abc"
        // [3.0.2] > s == t
        // => true
        // ```
        *self.inner.as_slice() == *other.inner.as_slice()
    }
}

impl Eq for String {}

impl PartialOrd for String {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.inner.as_slice().partial_cmp(other.inner.as_slice())
    }
}

impl Ord for String {
    fn cmp(&self, other: &Self) -> Ordering {
        self.inner.as_slice().cmp(other.inner.as_slice())
    }
}

// Constructors
impl String {
    /// Constructs a new, empty `String`.
    ///
    /// The `String` is [conventionally UTF-8].
    ///
    /// The string will not allocate until bytes are pushed onto it.
    ///
    /// # Examples
    ///
    /// ```
    /// use spinoso_string::{Encoding, String};
    ///
    /// let S: String = String::new();
    /// assert_eq!(S.encoding(), Encoding::Utf8);
    /// ```
    ///
    /// [conventionally UTF-8]: crate::Encoding::Utf8
    #[inline]
    #[must_use]
    pub fn new() -> Self {
        let buf = Vec::new();
        Self {
            inner: EncodedString::new(buf, Encoding::Utf8),
        }
    }

    /// Constructs a new, empty `String` with the specified capacity.
    ///
    /// The `String` is [conventionally UTF-8].
    ///
    /// The string will be able to hold exactly `capacity` bytes without
    /// reallocating. If `capacity` is 0, the string will not allocate.
    ///
    /// It is important to note that although the returned string has the
    /// capacity specified, the string will have a zero length. For an
    /// explanation of the difference between length and capacity, see
    /// *[Capacity and reallocation]*.
    ///
    /// # Examples
    ///
    /// Encoding, capacity, and length:
    ///
    /// ```
    /// use spinoso_string::{Encoding, String};
    ///
    /// let s = String::with_capacity(10);
    /// assert_eq!(s.encoding(), Encoding::Utf8);
    /// assert_eq!(s.capacity(), 10);
    /// assert_eq!(s.len(), 0);
    /// ```
    ///
    /// Allocation:
    ///
    /// ```
    /// use spinoso_string::{Encoding, String};
    ///
    /// let mut s = String::with_capacity(10);
    ///
    /// for ch in 'a'..='j' {
    ///     s.push_byte(ch as u8);
    /// }
    /// // 10 elements have been inserted without reallocating.
    /// assert_eq!(s.capacity(), 10);
    /// assert_eq!(s.len(), 10);
    /// ```
    ///
    /// [conventionally UTF-8]: crate::Encoding::Utf8
    /// [Capacity and reallocation]: https://doc.rust-lang.org/std/vec/struct.Vec.html#capacity-and-reallocation
    #[inline]
    #[must_use]
    pub fn with_capacity(capacity: usize) -> Self {
        let buf = Vec::with_capacity(capacity);
        Self {
            inner: EncodedString::new(buf, Encoding::Utf8),
        }
    }

    /// Constructs a new, empty `String` with the specified capacity and
    /// encoding.
    ///
    /// The string will be able to hold exactly `capacity` bytes without
    /// reallocating. If `capacity` is 0, the string will not allocate.
    ///
    /// It is important to note that although the returned string has the
    /// capacity specified, the string will have a zero length. For an
    /// explanation of the difference between length and capacity, see
    /// *[Capacity and reallocation]*.
    ///
    /// # Examples
    ///
    /// Encoding, capacity, and length:
    ///
    /// ```
    /// use spinoso_string::{Encoding, String};
    ///
    /// let s = String::with_capacity(10);
    /// assert_eq!(s.encoding(), Encoding::Utf8);
    /// assert_eq!(s.capacity(), 10);
    /// assert_eq!(s.len(), 0);
    /// ```
    ///
    /// Allocation:
    ///
    /// ```
    /// use spinoso_string::{Encoding, String};
    ///
    /// let mut s = String::with_capacity_and_encoding(10, Encoding::Binary);
    /// assert_eq!(s.encoding(), Encoding::Binary);
    ///
    /// for ch in 'a'..='j' {
    ///     s.push_byte(ch as u8);
    /// }
    /// // 10 elements have been inserted without reallocating.
    /// assert_eq!(s.capacity(), 10);
    /// assert_eq!(s.len(), 10);
    /// ```
    ///
    /// [Capacity and reallocation]: https://doc.rust-lang.org/std/vec/struct.Vec.html#capacity-and-reallocation
    #[inline]
    #[must_use]
    pub fn with_capacity_and_encoding(capacity: usize, encoding: Encoding) -> Self {
        let buf = Vec::with_capacity(capacity);
        Self {
            inner: EncodedString::new(buf, encoding),
        }
    }

    #[inline]
    #[must_use]
    pub fn with_bytes_and_encoding(buf: Vec<u8>, encoding: Encoding) -> Self {
        Self {
            inner: EncodedString::new(buf, encoding),
        }
    }

    #[inline]
    #[must_use]
    pub fn utf8(buf: Vec<u8>) -> Self {
        Self::with_bytes_and_encoding(buf, Encoding::Utf8)
    }

    #[inline]
    #[must_use]
    pub fn ascii(buf: Vec<u8>) -> Self {
        Self::with_bytes_and_encoding(buf, Encoding::Ascii)
    }

    #[inline]
    #[must_use]
    pub fn binary(buf: Vec<u8>) -> Self {
        Self::with_bytes_and_encoding(buf, Encoding::Binary)
    }
}

// Core data structure manipulation
impl String {
    /// Returns the [`Encoding`] of this `String`.
    ///
    /// # Examples
    ///
    /// ```
    /// use spinoso_string::{Encoding, String};
    ///
    /// let s = String::utf8(b"xyz".to_vec());
    /// assert_eq!(s.encoding(), Encoding::Utf8);
    /// ```
    #[inline]
    #[must_use]
    pub fn encoding(&self) -> Encoding {
        self.inner.encoding()
    }

    /// Shortens the string, keeping the first `len` bytes and dropping the
    /// rest.
    ///
    /// If `len` is greater than the string's current length, this has no
    /// effect.
    ///
    /// Note that this method has no effect on the allocated capacity
    /// of the string.
    ///
    /// # Examples
    ///
    /// Truncating a five byte to two elements:
    ///
    /// ```
    /// use spinoso_string::String;
    ///
    /// let mut s = String::from("12345");
    /// s.truncate(2);
    /// assert_eq!(*s, *b"12");
    /// ```
    ///
    /// No truncation occurs when `len` is greater than the string's current
    /// length:
    ///
    /// ```
    /// use spinoso_string::String;
    ///
    /// let mut s = String::from("12345");
    /// s.truncate(10);
    /// assert_eq!(*s, *b"12345");
    /// ```
    ///
    /// Truncating when `len == 0` is equivalent to calling the [`clear`]
    /// method.
    ///
    /// ```
    /// use spinoso_string::String;
    ///
    /// let mut s = String::from("12345");
    /// s.truncate(0);
    /// assert_eq!(*s, *b"");
    /// ```
    ///
    /// [`clear`]: Self::clear
    #[inline]
    pub fn truncate(&mut self, len: usize) {
        self.inner.truncate(len);
    }

    /// Extracts a slice containing the entire byte string.
    ///
    /// Equivalent to `&s[..]`.
    #[inline]
    #[must_use]
    pub fn as_slice(&self) -> &[u8] {
        self.inner.as_slice()
    }

    /// Extracts a mutable slice containing the entire byte string.
    ///
    /// Equivalent to `&mut s[..]`.
    #[inline]
    #[must_use]
    pub fn as_mut_slice(&mut self) -> &mut [u8] {
        self.inner.as_mut_slice()
    }

    /// Returns a raw pointer to the string's buffer.
    ///
    /// The caller must ensure that the string outlives the pointer this
    /// function returns, or else it will end up pointing to garbage. Modifying
    /// the string may cause its buffer to be reallocated, which would also make
    /// any pointers to it invalid.
    ///
    /// The caller must also ensure that the memory the pointer
    /// (non-transitively) points to is never written to (except inside an
    /// `UnsafeCell`) using this pointer or any pointer derived from it. If you
    /// need to mutate the contents of the slice, use [`as_mut_ptr`].
    ///
    /// # Examples
    ///
    /// ```
    /// use spinoso_string::String;
    ///
    /// let s = String::utf8(b"xyz".to_vec());
    /// let s_ptr = s.as_ptr();
    ///
    /// unsafe {
    ///     for i in 0..s.len() {
    ///         assert_eq!(*s_ptr.add(i), b'x' + (i as u8));
    ///     }
    /// }
    /// ```
    ///
    /// [`as_mut_ptr`]: Self::as_mut_ptr
    #[inline]
    #[must_use]
    pub fn as_ptr(&self) -> *const u8 {
        self.inner.as_ptr()
    }

    /// Returns an unsafe mutable pointer to the string's buffer.
    ///
    /// The caller must ensure that the string outlives the pointer this
    /// function returns, or else it will end up pointing to garbage. Modifying
    /// the string may cause its buffer to be reallocated, which would also make
    /// any pointers to it invalid.
    ///
    /// # Examples
    ///
    /// ```
    /// use spinoso_string::String;
    ///
    /// // Allocate string big enough for 3 bytes.
    /// let size = 3;
    /// let mut s = String::with_capacity(size);
    /// let s_ptr = s.as_mut_ptr();
    ///
    /// // Initialize elements via raw pointer writes, then set length.
    /// unsafe {
    ///     for i in 0..size {
    ///         *s_ptr.add(i) = b'x' + (i as u8);
    ///     }
    ///     s.set_len(size);
    /// }
    /// assert_eq!(&*s, b"xyz");
    /// ```
    #[inline]
    #[must_use]
    pub fn as_mut_ptr(&mut self) -> *mut u8 {
        self.inner.as_mut_ptr()
    }

    /// Forces the length of the string to `new_len`.
    ///
    /// This is a low-level operation that maintains none of the normal
    /// invariants of the type. Normally changing the length of a string is done
    /// using one of the safe operations instead, such as [`truncate`],
    /// [`extend`], or [`clear`].
    ///
    /// This function can change the return value of [`String::is_valid_encoding`].
    ///
    /// # Safety
    ///
    /// - `new_len` must be less than or equal to [`capacity()`].
    /// - The elements at `old_len..new_len` must be initialized.
    ///
    /// [`truncate`]: Self::truncate
    /// [`extend`]: Extend::extend
    /// [`clear`]: Self::clear
    /// [`capacity()`]: Self::capacity
    #[inline]
    pub unsafe fn set_len(&mut self, new_len: usize) {
        self.inner.set_len(new_len);
    }

    /// Creates a `String` directly from the raw components of another string.
    ///
    /// # Safety
    ///
    /// This is highly unsafe, due to the number of invariants that aren't
    /// checked:
    ///
    /// - `ptr` needs to have been previously allocated via `String` (at least,
    ///   it's highly likely to be incorrect if it wasn't).
    /// - `length` needs to be less than or equal to `capacity`.
    /// - `capacity` needs to be the `capacity` that the pointer was allocated
    ///   with.
    ///
    /// Violating these may cause problems like corrupting the allocator's
    /// internal data structures.
    ///
    /// The ownership of `ptr` is effectively transferred to the `String` which
    /// may then deallocate, reallocate or change the contents of memory pointed
    /// to by the pointer at will. Ensure that nothing else uses the pointer
    /// after calling this function.
    #[must_use]
    pub unsafe fn from_raw_parts(raw_parts: RawParts<u8>) -> Self {
        Self::utf8(RawParts::into_vec(raw_parts))
    }

    /// Creates a `String` directly from the raw components of another string
    /// with the specified encoding.
    ///
    /// # Safety
    ///
    /// This is highly unsafe, due to the number of invariants that aren't
    /// checked:
    ///
    /// - `ptr` needs to have been previously allocated via `String` (at least,
    ///   it's highly likely to be incorrect if it wasn't).
    /// - `length` needs to be less than or equal to `capacity`.
    /// - `capacity` needs to be the `capacity` that the pointer was allocated
    ///   with.
    ///
    /// Violating these may cause problems like corrupting the allocator's
    /// internal data structures.
    ///
    /// The ownership of `ptr` is effectively transferred to the `String` which
    /// may then deallocate, reallocate or change the contents of memory pointed
    /// to by the pointer at will. Ensure that nothing else uses the pointer
    /// after calling this function.
    #[must_use]
    pub unsafe fn from_raw_parts_with_encoding(raw_parts: RawParts<u8>, encoding: Encoding) -> Self {
        Self::with_bytes_and_encoding(RawParts::into_vec(raw_parts), encoding)
    }

    /// Decomposes a `String` into its raw components.
    ///
    /// Returns the raw pointer to the underlying data, the length of the string
    /// (in bytes), and the allocated capacity of the data (in bytes).  These
    /// are the same arguments in the same order as the arguments to
    /// [`from_raw_parts`].
    ///
    /// After calling this function, the caller is responsible for the memory
    /// previously managed by the `String`. The only way to do this is to
    /// convert the raw pointer, length, and capacity back into a `String` with
    /// the [`from_raw_parts`] function, allowing the destructor to perform the
    /// cleanup.
    ///
    /// [`from_raw_parts`]: String::from_raw_parts
    #[must_use]
    pub fn into_raw_parts(self) -> RawParts<u8> {
        RawParts::from_vec(self.inner.into_vec())
    }

    /// Converts self into a vector without clones or allocation.
    ///
    /// This method consumes this `String` and returns its inner [`Vec<u8>`]
    /// buffer.
    ///
    /// # Examples
    ///
    /// ```
    /// use spinoso_string::String;
    ///
    /// let s = String::from("hello");
    /// let buf = s.into_vec();
    /// // `s` cannot be used anymore because it has been converted into `buf`.
    ///
    /// assert_eq!(buf, b"hello".to_vec());
    /// ```
    /// [`Vec<u8>`]: alloc::vec::Vec
    #[inline]
    #[must_use]
    pub fn into_vec(self) -> Vec<u8> {
        self.inner.into_vec()
    }

    /// Converts the vector into `Box<[u8]>`.
    ///
    /// Note that this will drop any excess capacity.
    ///
    /// # Examples
    ///
    /// ```
    /// use spinoso_string::String;
    ///
    /// let s = String::from("abc");
    /// let slice = s.into_boxed_slice();
    /// ```
    ///
    /// Any excess capacity is removed:
    ///
    /// ```
    /// use spinoso_string::String;
    ///
    /// let mut s = String::with_capacity(10);
    /// s.extend_from_slice(&[b'a', b'b', b'c']);
    ///
    /// assert_eq!(s.capacity(), 10);
    /// let slice = s.into_boxed_slice();
    /// assert_eq!(slice.into_vec().capacity(), 3);
    /// ```
    ///
    /// [`Box<u8>`]: alloc::boxed::Box
    #[inline]
    #[must_use]
    pub fn into_boxed_slice(self) -> Box<[u8]> {
        self.inner.into_vec().into_boxed_slice()
    }

    /// Returns the number of bytes the string can hold without reallocating.
    ///
    /// # Examples
    ///
    /// ```
    /// use spinoso_string::String;
    ///
    /// let s = String::with_capacity(10);
    /// assert_eq!(s.capacity(), 10);
    /// ```
    #[inline]
    #[must_use]
    pub fn capacity(&self) -> usize {
        self.inner.capacity()
    }

    /// Clears the string, removing all bytes.
    ///
    /// Note that this method has no effect on the allocated capacity or the
    /// encoding of the string.
    ///
    /// # Examples
    ///
    /// ```
    /// use spinoso_string::String;
    ///
    /// let mut s = String::from("abc");
    /// s.clear();
    /// assert!(s.is_empty());
    /// ```
    #[inline]
    pub fn clear(&mut self) {
        self.inner.clear();
    }

    /// Returns true if the vector contains no bytes.
    ///
    /// # Examples
    ///
    /// ```
    /// use spinoso_string::String;
    ///
    /// let mut s = String::new();
    /// assert!(s.is_empty());
    ///
    /// s.push_char('x');
    /// assert!(!s.is_empty());
    /// ```
    #[inline]
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.inner.is_empty()
    }

    /// Returns the number of bytes in the string, also referred to as its
    /// "length" or "bytesize".
    ///
    /// See also [`bytesize`].
    ///
    /// # Examples
    ///
    /// ```
    /// use spinoso_string::String;
    ///
    /// let s = String::from("xyz");
    /// assert_eq!(s.len(), 3);
    /// ```
    ///
    /// [`bytesize`]: Self::bytesize
    #[inline]
    #[must_use]
    pub fn len(&self) -> usize {
        self.inner.len()
    }
}

// Core iterators
impl String {
    /// Returns an iterator over this string's underlying byte slice.
    ///
    /// # Examples
    ///
    /// ```
    /// use spinoso_string::String;
    ///
    /// let s = String::from("abc");
    /// let mut iterator = s.iter();
    ///
    /// assert_eq!(iterator.next(), Some(&b'a'));
    /// assert_eq!(iterator.next(), Some(&b'b'));
    /// assert_eq!(iterator.next(), Some(&b'c'));
    /// assert_eq!(iterator.next(), None);
    /// ```
    #[inline]
    #[must_use]
    pub fn iter(&self) -> Iter<'_> {
        self.inner.iter()
    }

    /// Returns an iterator that allows modifying this string's underlying byte
    /// slice.
    ///
    /// # Examples
    ///
    /// ```
    /// use spinoso_string::String;
    ///
    /// let mut s = String::from("abc");
    ///
    /// for byte in s.iter_mut() {
    ///     *byte = b'x';
    /// }
    ///
    /// assert_eq!(s, "xxx");
    /// ```
    #[inline]
    #[must_use]
    pub fn iter_mut(&mut self) -> IterMut<'_> {
        self.inner.iter_mut()
    }

    /// Returns an iterator over the bytes in this byte string.
    ///
    /// # Examples
    ///
    /// ```
    /// use spinoso_string::String;
    ///
    /// let s = String::utf8(b"foobar".to_vec());
    /// let bytes: Vec<u8> = s.bytes().collect();
    /// assert_eq!(bytes, s);
    /// ```
    #[inline]
    #[must_use]
    pub fn bytes(&self) -> Bytes<'_> {
        self.inner.bytes()
    }
}

// Additional IntoIterator iterator
impl IntoIterator for String {
    type Item = u8;
    type IntoIter = IntoIter;

    /// Returns an iterator that moves over the remaining bytes of a slice
    ///
    /// # Examples
    ///
    /// ```
    /// use spinoso_string::String;
    ///
    /// let s = String::from("abc");
    ///
    /// let mut iterator = s.into_iter();
    ///
    /// assert_eq!(iterator.next(), Some(b'a'));
    /// assert_eq!(iterator.next(), Some(b'b'));
    /// assert_eq!(iterator.next(), Some(b'c'));
    /// assert_eq!(iterator.next(), None);
    /// ```

    #[inline]
    #[must_use]
    fn into_iter(self) -> Self::IntoIter {
        self.inner.into_iter()
    }
}

// Memory management
impl String {
    /// Reserves capacity for at least `additional` more bytes to be inserted in
    /// the given `String`. The string may reserve more space to avoid frequent
    /// reallocations. After calling `reserve`, capacity will be greater than or
    /// equal to `self.len() + additional`. Does nothing if capacity is already
    /// sufficient.
    ///
    /// # Panics
    ///
    /// Panics if the new capacity exceeds [`isize::MAX`] bytes.
    ///
    /// # Examples
    ///
    /// ```
    /// use spinoso_string::String;
    ///
    /// let mut s = String::from("x");
    /// s.reserve(10);
    /// assert!(s.capacity() >= 11);
    /// ```
    #[inline]
    pub fn reserve(&mut self, additional: usize) {
        self.inner.reserve(additional);
    }

    /// Tries to reserve capacity for at least `additional` more elements to be
    /// inserted in the `String`. The collection may reserve more space
    /// to avoid frequent reallocations.
    /// After calling `try_reserve`, capacity will be greater than or equal to
    /// `self.len() + additional`.
    /// Does nothing if capacity is already sufficient.
    ///
    /// # Errors
    ///
    /// If the capacity overflows, or the allocator reports a failure, then an
    /// error is returned.
    ///
    /// # Examples
    ///
    /// ```
    /// use spinoso_string::String;
    /// let mut str = String::from("x");
    /// str.try_reserve(10).expect("why is this OOMing?");
    /// assert!(str.capacity() >= 11);
    /// ```
    #[inline]
    pub fn try_reserve(&mut self, additional: usize) -> Result<(), alloc::collections::TryReserveError> {
        self.inner.try_reserve(additional)
    }

    /// Reserves the minimum capacity for exactly `additional` more bytes to be
    /// inserted in the given `String`. After calling `reserve_exact`, capacity
    /// will be greater than or equal to `self.len() + additional`. Does nothing
    /// if the capacity is already sufficient.
    ///
    /// Note that the allocator may give the string more space than it requests.
    /// Therefore, capacity can not be relied upon to be precisely minimal.
    /// Prefer `reserve` if future insertions are expected.
    ///
    /// # Panics
    ///
    /// Panics if the new capacity overflows [`usize`].
    ///
    /// # Examples
    ///
    /// ```
    /// use spinoso_string::String;
    ///
    /// let mut s = String::from("x");
    /// s.reserve_exact(10);
    /// assert!(s.capacity() >= 11);
    /// ```
    #[inline]
    pub fn reserve_exact(&mut self, additional: usize) {
        self.inner.reserve_exact(additional);
    }

    /// Tries to reserve the minimum capacity for exactly `additional`
    /// elements to be inserted in the `String`.
    /// After calling `try_reserve_exact`, capacity will be greater
    /// than or equal to `self.len() + additional` if it returns Ok(()).
    /// Does nothing if the capacity is already sufficient.
    ///
    /// Note that the allocator may give the collection more space than
    /// it requests. Therefore, capacity can not be relied upon to be
    /// precisely minimal.
    /// Prefer `reserve` if future insertions are expected.
    ///
    /// # Errors
    ///
    /// If the capacity overflows, or the allocator reports a failure, then an
    /// error is returned.
    ///
    /// # Examples
    ///
    /// ```
    /// use spinoso_string::String;
    /// let mut str = String::from("x");
    /// str.try_reserve_exact(10).expect("why is this OOMing?");
    /// assert!(str.capacity() >= 11);
    /// ```
    #[inline]
    pub fn try_reserve_exact(&mut self, additional: usize) -> Result<(), alloc::collections::TryReserveError> {
        self.inner.try_reserve_exact(additional)
    }

    /// Shrinks the capacity of the vector as much as possible.
    ///
    /// It will drop down as close as possible to the length but the allocator
    /// may still inform the string that there is space for a few more bytes.
    ///
    /// # Examples
    ///
    /// ```
    /// use spinoso_string::String;
    ///
    /// let mut s = String::with_capacity(10);
    /// s.extend_from_slice(b"abc");
    /// assert_eq!(s.capacity(), 10);
    /// s.shrink_to_fit();
    /// assert!(s.capacity() >= 3);
    /// ```
    #[inline]
    pub fn shrink_to_fit(&mut self) {
        self.inner.shrink_to_fit();
    }

    /// Shrinks the capacity of the vector with a lower bound.
    ///
    /// The capacity will remain at least as large as both the length and the
    /// supplied value.
    ///
    /// If the current capacity is less than the lower limit, this is a no-op.
    ///
    /// # Examples
    ///
    /// ```
    /// use spinoso_string::String;
    ///
    /// let mut s = String::with_capacity(10);
    /// s.extend_from_slice(b"abc");
    /// assert_eq!(s.capacity(), 10);
    /// s.shrink_to(5);
    /// assert!(s.capacity() >= 5);
    /// ```
    #[inline]
    pub fn shrink_to(&mut self, min_capacity: usize) {
        self.inner.shrink_to(min_capacity);
    }
}

// Indexing
impl String {
    /// Returns a reference to a byte or sub-byteslice depending on the type of
    /// index.
    ///
    /// - If given a position, returns a reference to the byte at that position
    ///   or [`None`] if out of bounds.
    /// - If given a range, returns the subslice corresponding to that range, or
    ///   [`None`] if out of bounds.
    ///
    /// # Examples
    ///
    /// ```
    /// use spinoso_string::String;
    ///
    /// let s = String::from("abc");
    /// assert_eq!(s.get(1), Some(&b'b'));
    /// assert_eq!(s.get(0..2), Some(&b"ab"[..]));
    /// assert_eq!(s.get(3), None);
    /// assert_eq!(s.get(0..4), None);
    /// ```
    #[inline]
    #[must_use]
    pub fn get<I>(&self, index: I) -> Option<&I::Output>
    where
        I: SliceIndex<[u8]>,
    {
        self.inner.get(index)
    }

    /// Returns a mutable reference to a byte or sub-byteslice depending on the
    /// type of index (see [`get`]) or [`None`] if the index is out of bounds.
    ///
    /// # Examples
    ///
    /// ```
    /// use spinoso_string::String;
    ///
    /// let mut s = String::from("abc");
    ///
    /// if let Some(byte) = s.get_mut(1) {
    ///     *byte = b'x';
    /// }
    /// assert_eq!(s, "axc");
    /// ```
    ///
    /// [`get`]: Self::get
    #[inline]
    #[must_use]
    pub fn get_mut<I>(&mut self, index: I) -> Option<&mut I::Output>
    where
        I: SliceIndex<[u8]>,
    {
        self.inner.get_mut(index)
    }

    /// Returns a reference to a byte or sub-byteslice, without doing bounds
    /// checking.
    ///
    /// For a safe alternative see [`get`].
    ///
    /// # Safety
    ///
    /// Calling this method with an out-of-bounds index is *[undefined
    /// behavior]* even if the resulting reference is not used.
    ///
    /// # Examples
    ///
    /// ```
    /// use spinoso_string::String;
    ///
    /// let s = String::from("abc");
    ///
    /// unsafe {
    ///     assert_eq!(s.get_unchecked(1), &b'b');
    /// }
    /// ```
    ///
    /// [`get`]: Self::get
    /// [undefined behavior]: https://doc.rust-lang.org/reference/behavior-considered-undefined.html
    #[inline]
    #[must_use]
    pub unsafe fn get_unchecked<I>(&self, index: I) -> &I::Output
    where
        I: SliceIndex<[u8]>,
    {
        self.inner.get_unchecked(index)
    }

    /// Returns a mutable reference to a byte or sub-byteslice, without doing
    /// bounds checking.
    ///
    /// For a safe alternative see [`get_mut`].
    ///
    /// # Safety
    ///
    /// Calling this method with an out-of-bounds index is *[undefined
    /// behavior]* even if the resulting reference is not used.
    ///
    /// # Examples
    ///
    /// ```
    /// use spinoso_string::String;
    ///
    /// let mut s = String::from("abc");
    ///
    /// unsafe {
    ///     let byte = s.get_unchecked_mut(1);
    ///     *byte = b'x';
    /// }
    /// assert_eq!(s, "axc");
    /// ```
    ///
    /// [`get_mut`]: Self::get_mut
    /// [undefined behavior]: https://doc.rust-lang.org/reference/behavior-considered-undefined.html
    #[inline]
    #[must_use]
    pub unsafe fn get_unchecked_mut<I>(&mut self, index: I) -> &mut I::Output
    where
        I: SliceIndex<[u8]>,
    {
        self.inner.get_unchecked_mut(index)
    }
}

// Pushing and popping bytes, codepoints, and strings.
impl String {
    /// Appends a given byte onto the end of this `String`.
    ///
    /// The given byte is not required to be a valid byte given this `String`'s
    /// [encoding] because encodings are only conventional.
    ///
    /// # Examples
    ///
    /// ```
    /// use spinoso_string::String;
    ///
    /// let mut s = String::utf8(b"UTF-8?".to_vec());
    /// s.push_byte(0xFF);
    /// assert_eq!(s, &b"UTF-8?\xFF"[..]);
    /// ```
    ///
    /// [encoding]: crate::Encoding
    #[inline]
    pub fn push_byte(&mut self, byte: u8) {
        self.inner.push_byte(byte);
    }

    /// Try to append a given Unicode codepoint onto the end of this `String`.
    ///
    /// This API is encoding-aware. For [UTF-8] strings, the given integer is
    /// converted to a [`char`] before appending to this `String` using
    /// [`push_char`]. For [ASCII] and [binary] strings, the given integer is
    /// converted to a byte before appending to this `String` using
    /// [`push_byte`].
    ///
    /// This function can be used to implement the Ruby method [`String#<<`] for
    /// [`Integer`][ruby-integer] arguments.
    ///
    /// # Errors
    ///
    /// If this `String` is [conventionally UTF-8] and the given codepoint is
    /// not a valid [`char`], an error is returned.
    ///
    /// If this `String` has [ASCII] or [binary] encoding and the given
    /// codepoint is not a valid byte, an error is returned.
    ///
    /// # Examples
    ///
    /// For [UTF-8] strings, the given codepoint is converted to a Unicode scalar
    /// value before appending:
    ///
    /// ```
    /// use spinoso_string::String;
    ///
    /// # fn example() -> Result<(), spinoso_string::InvalidCodepointError> {
    /// let mut s = String::utf8(b"".to_vec());
    /// s.try_push_codepoint(b'a' as i64)?;
    /// assert_eq!(s, "a");
    /// assert!(s.try_push_codepoint(0xD83F).is_err());
    /// assert!(s.try_push_codepoint(-1).is_err());
    /// # Ok(())
    /// # }
    /// # example().unwrap();
    /// ```
    ///
    /// For [ASCII] and [binary] strings, the given codepoint must be a valid
    /// byte:
    ///
    /// ```
    /// use spinoso_string::String;
    ///
    /// # fn example() -> Result<(), spinoso_string::InvalidCodepointError> {
    /// let mut s = String::binary(b"".to_vec());
    /// s.try_push_codepoint(b'a' as i64)?;
    /// assert_eq!(s, "a");
    /// assert!(s.try_push_codepoint(1024).is_err());
    /// assert!(s.try_push_codepoint(-1).is_err());
    /// # Ok(())
    /// # }
    /// # example().unwrap();
    /// ```
    ///
    /// [UTF-8]: crate::Encoding::Utf8
    /// [ASCII]: crate::Encoding::Ascii
    /// [binary]: crate::Encoding::Binary
    /// [`push_char`]: Self::push_char
    /// [`push_byte`]: Self::push_byte
    /// [`String#<<`]: https://ruby-doc.org/core-2.6.3/String.html#method-i-3C-3C
    /// [ruby-integer]: https://ruby-doc.org/core-2.6.3/Integer.html
    /// [conventionally UTF-8]: crate::Encoding::Utf8
    #[inline]
    pub fn try_push_codepoint(&mut self, codepoint: i64) -> Result<(), InvalidCodepointError> {
        self.inner.try_push_codepoint(codepoint)
    }

    /// Appends a given [`char`] onto the end of this `String`.
    ///
    /// The given char is UTF-8 encoded and the UTF-8 bytes are appended to the
    /// end of this `String`.
    ///
    /// # Examples
    ///
    /// ```
    /// use spinoso_string::String;
    ///
    /// let mut s = String::from("<3");
    /// s.push_char('💎');
    /// assert_eq!(s, &b"<3\xF0\x9F\x92\x8E"[..]); // "<3💎"
    /// ```
    #[inline]
    pub fn push_char(&mut self, ch: char) {
        self.inner.push_char(ch);
    }

    /// Appends a given string slice onto the end of this `String`.
    ///
    /// # Examples
    ///
    /// ```
    /// use spinoso_string::String;
    ///
    /// let mut s = String::utf8(b"spinoso".to_vec());
    /// s.push_str("-string");
    /// assert_eq!(s, "spinoso-string");
    /// ```
    #[inline]
    pub fn push_str(&mut self, s: &str) {
        self.inner.push_str(s);
    }

    /// Copies and appends all bytes in a slice to the `String`.
    ///
    /// Iterates over the slice `other`, copies each element, and then appends
    /// it to this `String`. The other byte slice is traversed in-order.
    ///
    /// # Examples
    ///
    /// ```
    /// use spinoso_string::String;
    ///
    /// let mut s = String::from("a");
    /// s.extend_from_slice(&b"bc"[..]);
    /// assert_eq!(s, "abc");
    /// ```
    #[inline]
    pub fn extend_from_slice(&mut self, other: &[u8]) {
        self.inner.extend_from_slice(other);
    }
}

// Ruby APIs
impl String {
    /// Appends the given bytes to this `String`.
    ///
    /// See also [`Extend`].
    ///
    /// This function can be used to implement the Ruby method [`String#<<`] for
    /// [`String`][ruby-string] arguments.
    ///
    /// # Examples
    ///
    /// ```
    /// use spinoso_string::String;
    ///
    /// let mut s = String::ascii(b"abc".to_vec());
    /// s.concat(", easy as 123");
    /// assert_eq!(s, "abc, easy as 123");
    /// ```
    ///
    /// [`String#<<`]: https://ruby-doc.org/core-2.6.3/String.html#method-i-3C-3C
    /// [ruby-string]: https://ruby-doc.org/core-2.6.3/String.html
    #[inline]
    pub fn concat<T: AsRef<[u8]>>(&mut self, other: T) {
        let other = other.as_ref();
        self.inner.extend_from_slice(other);
    }

    /// Returns true for a string which has only ASCII characters.
    ///
    /// ASCII is an encoding that defines 128 codepoints. A byte corresponds to
    /// an ASCII codepoint if and only if it is in the inclusive range
    /// `[0, 127]`.
    ///
    /// This function ignores this `String`'s [encoding].
    ///
    /// # Examples
    ///
    /// ```
    /// use spinoso_string::String;
    ///
    /// let s = String::utf8("abc".as_bytes().to_vec());
    /// assert!(s.is_ascii_only());
    /// let s = String::utf8("abc\u{6666}".as_bytes().to_vec());
    /// assert!(!s.is_ascii_only());
    /// ```
    ///
    /// [encoding]: crate::Encoding
    #[inline]
    #[must_use]
    pub fn is_ascii_only(&self) -> bool {
        self.inner.is_ascii_only()
    }

    /// Change the [encoding] of this `String` to [`Encoding::Binary`].
    ///
    /// This function can be used to implement the Ruby method [`String#b`].
    ///
    /// # Examples
    ///
    /// ```
    /// use spinoso_string::{Encoding, String};
    ///
    /// let mut s = String::utf8(b"xyz".to_vec());
    /// assert_eq!(s.encoding(), Encoding::Utf8);
    /// s.make_binary();
    /// assert_eq!(s.encoding(), Encoding::Binary);
    /// ```
    ///
    /// [encoding]: crate::Encoding
    /// [`String#b`]: https://ruby-doc.org/core-2.6.3/String.html#method-i-b
    #[inline]
    pub fn make_binary(&mut self) {
        let old = self.inner.as_slice().to_vec();
        self.inner = EncodedString::new(old, Encoding::Binary);
    }

    /// Returns the length of this `String` in bytes.
    ///
    /// `bytesize` is an [`Encoding`]-oblivious API and is equivalent to
    /// [`String::len`].
    ///
    /// This function can be used to implement the Ruby method
    /// [`String#bytesize`].
    ///
    /// # Examples
    ///
    /// ```
    /// use spinoso_string::String;
    ///
    /// let s = String::utf8("💎".as_bytes().to_vec());
    /// assert_eq!(s.bytesize(), 4);
    /// assert_eq!(s.bytesize(), s.len());
    /// ```
    ///
    /// [`String#bytesize`]: https://ruby-doc.org/core-2.6.3/String.html#method-i-bytesize
    #[inline]
    #[must_use]
    pub fn bytesize(&self) -> usize {
        self.len()
    }

    /// Modify this `String` to have the first character converted to uppercase
    /// and the remainder to lowercase.
    #[inline]
    pub fn make_capitalized(&mut self) {
        self.inner.make_capitalized();
    }

    /// Modify this `String` to have all characters converted to lowercase.
    #[inline]
    pub fn make_lowercase(&mut self) {
        self.inner.make_lowercase();
    }

    /// Modify this `String` to have the all characters converted to uppercase.
    #[inline]
    pub fn make_uppercase(&mut self) {
        self.inner.make_uppercase();
    }

    #[inline]
    #[must_use]
    #[cfg(feature = "casecmp")]
    #[cfg_attr(feature = "docsrs", doc(cfg(feature = "casecmp")))]
    pub fn ascii_casecmp(&self, other: &[u8]) -> Ordering {
        focaccia::ascii_casecmp(self.as_slice(), other)
    }

    #[inline]
    #[must_use]
    #[cfg(feature = "casecmp")]
    #[cfg_attr(feature = "docsrs", doc(cfg(feature = "casecmp")))]
    pub fn unicode_casecmp(&self, other: &String, options: CaseFold) -> Option<bool> {
        let left = self;
        let right = other;
        // If both `String`s are conventionally UTF-8, they must be case
        // compared using the given case folding strategy. This requires the
        // `String`s be well-formed UTF-8.
        if let (Encoding::Utf8, Encoding::Utf8) = (self.encoding(), other.encoding()) {
            if let (Ok(left), Ok(right)) = (str::from_utf8(left), str::from_utf8(right)) {
                // Both slices are UTF-8, compare with the given Unicode case
                // folding scheme.
                Some(options.case_eq(left, right))
            } else {
                // At least one `String` contains invalid UTF-8 bytes.
                None
            }
        } else {
            // At least one slice is not conventionally UTF-8, so fallback to
            // ASCII comparator.
            Some(focaccia::ascii_case_eq(left, right))
        }
    }

    /// Centers this `String` in width with the given padding.
    ///
    /// This function returns an iterator that yields [`u8`].
    ///
    /// If width is greater than the length of this `String`, the returned
    /// iterator yields a byte sequence of length `width` with the byte content
    /// of this `String` centered and padded with the given padding; otherwise,
    /// yields the original bytes.
    ///
    /// If the given padding is [`None`], the `String` is padded with an ASCII
    /// space.
    ///
    /// # Errors
    ///
    /// If given an empty padding byte string, this function returns an error.
    /// This error is returned regardless of whether the `String` would be
    /// centered with the given
    ///
    /// # Examples
    ///
    /// ```
    /// use spinoso_string::String;
    /// # fn example() -> Result<(), spinoso_string::CenterError> {
    /// let s = String::from("hello");
    ///
    /// assert_eq!(s.center(4, None)?.collect::<Vec<_>>(), b"hello");
    /// assert_eq!(s.center(20, None)?.collect::<Vec<_>>(), b"       hello        ");
    /// assert_eq!(s.center(20, Some(&b"123"[..]))?.collect::<Vec<_>>(), b"1231231hello12312312");
    /// # Ok(())
    /// # }
    /// # example().unwrap();
    /// ```
    ///
    /// This iterator is [encoding-aware]. [Conventionally UTF-8] strings are
    /// iterated by UTF-8 byte sequences.
    ///
    /// ```
    /// use spinoso_string::String;
    /// # fn example() -> Result<(), spinoso_string::CenterError> {
    /// let s = String::from("💎");
    ///
    /// assert_eq!(s.center(3, None)?.collect::<Vec<_>>(), " 💎 ".as_bytes());
    /// # Ok(())
    /// # }
    /// # example().unwrap();
    /// ```
    ///
    /// [`center`]: crate::String::center
    /// [encoding-aware]: crate::Encoding
    /// [Conventionally UTF-8]: crate::Encoding::Utf8
    #[inline]
    pub fn center<'a, 'b>(&'a self, width: usize, padding: Option<&'b [u8]>) -> Result<Center<'a, 'b>, CenterError> {
        let padding = match padding {
            None => b" ",
            Some(p) if p.is_empty() => return Err(CenterError::ZeroWidthPadding),
            Some(p) => p,
        };
        let padding_width = width.saturating_sub(self.char_len());
        Ok(Center::with_chars_width_and_padding(
            self.chars(),
            padding_width,
            padding,
        ))
    }

    /// Modifies this `String` in-place with the given record separator removed
    /// from the end of str (if given).
    ///
    /// If `separator` is [`None`] (i.e. `separator` has not been changed from
    /// the default Ruby record separator), then `chomp` also removes carriage
    /// return characters (that is it will remove `\n`, `\r`, and `\r\n`). If
    /// `separator` is an empty string, it will remove all trailing newlines
    /// from the string.
    ///
    /// A [`None`] separator does not mean that `chomp` is passed a `nil`
    /// separator. For `str.chomp nil`, MRI returns `str.dup`. For
    /// `str.chomp! nil`, MRI makes no changes to the receiver and returns
    /// `nil`.
    ///
    /// This function returns `true` if self is modified, `false` otherwise.
    ///
    /// # Examples
    ///
    /// ```
    /// use spinoso_string::String;
    ///
    /// let mut s = String::utf8(b"This is a paragraph.\r\n\n\n".to_vec());
    /// let modified = s.chomp(None::<&[u8]>);
    /// assert!(modified);
    /// assert_eq!(s, "This is a paragraph.\r\n\n");
    ///
    /// let mut s = String::utf8(b"This is a paragraph.\r\n\n\n".to_vec());
    /// let modified = s.chomp(Some(""));
    /// assert!(modified);
    /// assert_eq!(s, "This is a paragraph.");
    ///
    /// let mut s = String::utf8(b"hello\r\n\r\r\n".to_vec());
    /// let modified = s.chomp(None::<&[u8]>);
    /// assert!(modified);
    /// assert_eq!(s, "hello\r\n\r");
    ///
    /// let mut s = String::utf8(b"hello\r\n\r\r\n".to_vec());
    /// let modified = s.chomp(Some(""));
    /// assert!(modified);
    /// assert_eq!(s, "hello\r\n\r");
    ///
    /// let mut s = String::utf8(b"This is a paragraph.".to_vec());
    /// let modified = s.chomp(Some("."));
    /// assert!(modified);
    /// assert_eq!(s, "This is a paragraph");
    ///
    /// let mut s = String::utf8(b"This is a paragraph.".to_vec());
    /// let modified = s.chomp(Some("abc"));
    /// assert!(!modified);
    /// assert_eq!(s, "This is a paragraph.");
    /// ```
    #[inline]
    #[must_use]
    #[allow(clippy::needless_pass_by_value)]
    pub fn chomp<T: AsRef<[u8]>>(&mut self, separator: Option<T>) -> bool {
        // convert to a concrete type and delegate to a single `chomp` impl
        // to minimize code duplication when monomorphizing.
        let separator = separator.as_ref().map(AsRef::as_ref);
        chomp(self, separator)
    }

    /// Modifies this `String` in-place and removes the last character.
    ///
    /// This method returns a [`bool`] that indicates if this string was modified.
    ///
    /// If the string ends with `\r\n`, both characters are removed. When
    /// applying `chop` to an empty string, the string remains empty.
    ///
    /// [`String::chomp`] is often a safer alternative, as it leaves the string
    /// unchanged if it doesn't end in a record separator.
    ///
    /// # Examples
    ///
    /// ```
    /// use spinoso_string::String;
    ///
    /// let mut s = String::utf8(b"This is a paragraph.\r\n".to_vec());
    /// let modified = s.chop();
    /// assert!(modified);
    /// assert_eq!(s, "This is a paragraph.");
    ///
    /// let mut s = String::utf8(b"This is a paragraph.".to_vec());
    /// let modified = s.chop();
    /// assert!(modified);
    /// assert_eq!(s, "This is a paragraph");
    ///
    /// let mut s = String::utf8(b"".to_vec());
    /// let modified = s.chop();
    /// assert!(!modified);
    /// assert_eq!(s, "");
    ///
    /// let mut s = String::utf8(b"x".to_vec());
    /// let modified = s.chop();
    /// assert!(modified);
    /// assert_eq!(s, "");
    /// ```
    #[inline]
    #[must_use]
    pub fn chop(&mut self) -> bool {
        if self.is_empty() {
            return false;
        }
        let bytes_to_remove = if self.inner.as_slice().ends_with(b"\r\n") {
            2
        } else if let Encoding::Utf8 = self.encoding() {
            let (ch, size) = bstr::decode_last_utf8(&self.as_slice());
            if ch.is_some() {
                size
            } else {
                1
            }
        } else {
            // `buf` is checked to be non-empty above.
            1
        };
        // This subtraction is guaranteed to not panic because we have validated
        // that we're removing a subslice of `buf`.
        self.truncate(self.len() - bytes_to_remove);
        true
    }

    /// Returns a one-character string at the beginning of the string.
    ///
    /// # Examples
    ///
    /// [Conventionally UTF-8] `String`s perform a partial UTF-8 decode to
    /// compute the first character.
    ///
    /// ```
    /// use spinoso_string::String;
    ///
    /// let s = String::utf8(b"abcde".to_vec());
    /// assert_eq!(s.chr(), &b"a"[..]);
    ///
    /// let s = String::utf8(b"".to_vec());
    /// assert_eq!(s.chr(), &[]);
    ///
    /// let s = String::utf8("🦀spinoso💎".as_bytes().to_vec());
    /// assert_eq!(s.chr(), &b"\xF0\x9F\xA6\x80"[..]);
    ///
    /// let s = String::utf8(b"\xFFspinoso".to_vec());
    /// assert_eq!(s.chr(), &b"\xFF"[..]);
    /// ```
    ///
    /// For [ASCII] and [binary] `String`s this function returns a slice of the
    /// first byte or the empty slice if the `String` is empty.
    ///
    /// ```
    /// use spinoso_string::String;
    ///
    /// let s = String::binary(b"abcde".to_vec());
    /// assert_eq!(s.chr(), &b"a"[..]);
    ///
    /// let s = String::binary(b"".to_vec());
    /// assert_eq!(s.chr(), &[]);
    ///
    /// let s = String::binary("🦀spinoso💎".as_bytes().to_vec());
    /// assert_eq!(s.chr(), &b"\xF0"[..]);
    ///
    /// let s = String::binary(b"\xFFspinoso".to_vec());
    /// assert_eq!(s.chr(), &b"\xFF"[..]);
    /// ```
    ///
    /// [Conventionally UTF-8]: Encoding::Utf8
    /// [ASCII]: crate::Encoding::Ascii
    /// [binary]: crate::Encoding::Binary
    #[inline]
    #[must_use]
    pub fn chr(&self) -> &[u8] {
        self.inner.chr()
    }

    /// Returns the index of the first occurrence of the given substring in this
    /// `String`.
    ///
    /// Returns [`None`] if not found. If the second parameter is present, it
    /// specifies the position in the string to begin the search.
    ///
    /// This function can be used to implement [`String#index`].
    ///
    /// # Examples
    ///
    /// ```
    /// use spinoso_string::String;
    ///
    /// let s = String::from("hello");
    /// assert_eq!(s.index("e", None), Some(1));
    /// assert_eq!(s.index("lo", None), Some(3));
    /// assert_eq!(s.index("a", None), None);
    /// assert_eq!(s.index("l", Some(3)), Some(3));
    /// ```
    ///
    /// [`String#index`]: https://ruby-doc.org/core-2.6.3/String.html#method-i-index
    #[inline]
    #[must_use]
    pub fn index<T: AsRef<[u8]>>(&self, needle: T, offset: Option<usize>) -> Option<usize> {
        fn inner(buf: &[u8], needle: &[u8], offset: Option<usize>) -> Option<usize> {
            if let Some(offset) = offset {
                let buf = buf.get(offset..)?;
                let index = buf.find(needle)?;
                // This addition is guaranteed not to overflow because the result is
                // a valid index of the underlying `Vec`.
                //
                // `self.buf.len() < isize::MAX` because `self.buf` is a `Vec` and
                // `Vec` documents `isize::MAX` as its maximum allocation size.
                Some(index + offset)
            } else {
                buf.find(needle)
            }
        }
        // convert to a concrete type and delegate to a single `index` impl
        // to minimize code duplication when monomorphizing.
        let needle = needle.as_ref();
        inner(self.inner.as_slice(), needle, offset)
    }

    #[inline]
    #[must_use]
    pub fn rindex<T: AsRef<[u8]>>(&self, needle: T, offset: Option<usize>) -> Option<usize> {
        fn inner(buf: &[u8], needle: &[u8], offset: Option<usize>) -> Option<usize> {
            if let Some(offset) = offset {
                let end = buf.len().checked_sub(offset).unwrap_or_default();
                let buf = buf.get(..end)?;
                buf.rfind(needle)
            } else {
                buf.rfind(needle)
            }
        }
        // convert to a concrete type and delegate to a single `rindex` impl
        // to minimize code duplication when monomorphizing.
        let needle = needle.as_ref();
        inner(self.inner.as_slice(), needle, offset)
    }

    /// Returns an iterator that yields a debug representation of the `String`.
    ///
    /// This iterator produces [`char`] sequences like `"spinoso"` and
    /// `"invalid-\xFF-utf8"`.
    ///
    /// This function can be used to implement the Ruby method
    /// [`String#inspect`].
    ///
    /// [`String#inspect`]: https://ruby-doc.org/core-2.6.3/String.html#method-i-inspect:
    #[inline]
    pub fn inspect(&self) -> Inspect<'_> {
        Inspect::from(self.as_slice())
    }

    /// Returns the Integer ordinal of a one-character string.
    ///
    /// # Errors
    ///
    /// If this `String` is empty, an error is returned.
    ///
    /// If this `String` is [conventionally UTF-8] and the string contents begin
    /// with an invalid UTF-8 byte sequence, an error is returned.
    ///
    /// [conventionally UTF-8]: crate::Encoding::Utf8
    #[inline]
    pub fn ord(&self) -> Result<u32, OrdError> {
        self.inner.ord()
    }
}

// Encoding-aware APIs.
impl String {
    /// Returns an iterator over the chars of a `String`.
    ///
    /// This function is encoding-aware. `String`s with [UTF-8 encoding] are
    /// only [conventionally UTF-8]. This iterator yields `&[u8]` byte slices
    /// that correspond to either a valid UTF-8 byte sequence or a single
    /// invalid UTF-8 byte. For [ASCII encoded] and [binary encoded] strings,
    /// this iterator yields slices of single bytes.
    ///
    /// For UTF-8 encoded strings, the yielded byte slices can be parsed into
    /// [`char`]s with [`str::from_utf8`] and [`str::chars`].
    ///
    /// # Examples
    ///
    /// Iterating over the characters of a conventionally UTF-8 string:
    ///
    /// ```
    /// use core::str;
    /// use spinoso_string::String;
    ///
    /// let s = String::utf8(b"ab\xF0\x9F\x92\x8E\xFF".to_vec());
    /// let mut chars = s.chars();
    /// assert_eq!(chars.next(), Some(&b"a"[..]));
    /// assert_eq!(chars.next().map(str::from_utf8), Some(Ok("b")));
    /// assert_eq!(chars.next(), Some(&[0xF0, 0x9F, 0x92, 0x8E][..]));
    /// assert_eq!(chars.next(), Some(&b"\xFF"[..]));
    /// assert_eq!(chars.next(), None);
    /// ```
    ///
    /// Iterating over the characters of a binary string:
    ///
    /// ```
    /// use spinoso_string::String;
    ///
    /// let s = String::binary("💎".as_bytes().to_vec());
    /// let mut chars = s.chars();
    /// assert_eq!(chars.next(), Some(&[0xF0][..]));
    /// assert_eq!(chars.next(), Some(&[0x9F][..]));
    /// assert_eq!(chars.next(), Some(&[0x92][..]));
    /// assert_eq!(chars.next(), Some(&[0x8E][..]));
    /// assert_eq!(chars.next(), None);
    /// ```
    ///
    /// [UTF-8 encoding]: crate::Encoding::Utf8
    /// [conventionally UTF-8]: crate::Encoding::Utf8
    /// [ASCII encoded]: crate::Encoding::Ascii
    /// [binary encoded]: crate::Encoding::Binary
    /// [`str::from_utf8`]: core::str::from_utf8
    #[inline]
    #[must_use]
    pub fn chars(&self) -> Chars<'_> {
        Chars::from(self)
    }

    /// Returns an iterator over the `u32` codepoints of a `String`.
    ///
    /// This function is encoding-aware. `String`s with [UTF-8 encoding] are
    /// only [conventionally UTF-8]. This function only returns `Ok` for
    /// `String`s with UTF-8 encoding if the underlying bytes in the `String`
    /// are valid UTF-8. For UTF-8 `String`s, this iterator yields the `u32`
    /// values of the [`char`]s in the byte string. For [ASCII encoded] and
    /// [binary encoded] strings, this iterator yields slices of single bytes.
    ///
    /// For UTF-8 encoded strings, the yielded byte slices can be parsed into
    /// [`char`]s with `.into()`.
    ///
    /// # Errors
    ///
    /// This function requires the `String` contents to be well-formed with
    /// respect to its encoding. This function will return an error if the
    /// `String` has UTF-8 encoding and contains invalid UTF-8 byte sequences.
    ///
    /// # Examples
    ///
    /// Iterating over the codepoints of a conventionally UTF-8 string:
    ///
    /// ```
    /// use spinoso_string::{CodepointsError, String};
    ///
    /// # fn example() -> Result<(), spinoso_string::CodepointsError> {
    /// let s = String::utf8(b"ab\xF0\x9F\x92\x8E\xFF".to_vec());
    /// assert!(matches!(s.codepoints(), Err(CodepointsError::InvalidUtf8Codepoint)));
    ///
    /// let s = String::utf8("💎".as_bytes().to_vec());
    /// let mut codepoints = s.codepoints()?;
    /// assert_eq!(codepoints.next(), Some(u32::from('💎')));
    /// assert_eq!(codepoints.next(), None);
    /// # Ok(())
    /// # }
    /// # example().unwrap();
    /// ```
    ///
    /// Iterating over the codepoints of a binary string:
    ///
    /// ```
    /// use spinoso_string::String;
    ///
    /// # fn example() -> Result<(), spinoso_string::CodepointsError> {
    /// let s = String::binary("💎".as_bytes().to_vec());
    /// let mut codepoints = s.codepoints()?;
    /// assert_eq!(codepoints.next(), Some(0xF0));
    /// assert_eq!(codepoints.next(), Some(0x9F));
    /// assert_eq!(codepoints.next(), Some(0x92));
    /// assert_eq!(codepoints.next(), Some(0x8E));
    /// assert_eq!(codepoints.next(), None);
    /// # Ok(())
    /// # }
    /// # example().unwrap();
    /// ```
    ///
    /// [UTF-8 encoding]: crate::Encoding::Utf8
    /// [conventionally UTF-8]: crate::Encoding::Utf8
    /// [ASCII encoded]: crate::Encoding::Ascii
    /// [binary encoded]: crate::Encoding::Binary
    /// [`str::from_utf8`]: core::str::from_utf8
    #[inline]
    pub fn codepoints(&self) -> Result<Codepoints<'_>, CodepointsError> {
        Codepoints::try_from(self)
    }

    /// Returns the character length of this `String`.
    ///
    /// This function is encoding-aware. For `String`s with [UTF-8 encoding],
    /// multi-byte Unicode characters are length 1 and invalid UTF-8 bytes are
    /// length 1. For `String`s with [ASCII encoding] or [binary encoding],
    /// this function is equivalent to [`len`] and [`bytesize`].
    ///
    /// # Examples
    ///
    /// ```
    /// use spinoso_string::String;
    ///
    /// let s = String::utf8(b"abc\xF0\x9F\x92\x8E\xFF".to_vec()); // "abc💎\xFF"
    /// assert_eq!(s.char_len(), 5);
    ///
    /// let b = String::binary(b"abc\xF0\x9F\x92\x8E\xFF".to_vec()); // "abc💎\xFF"
    /// assert_eq!(b.char_len(), 8);
    /// ```
    ///
    /// [UTF-8 encoding]: crate::Encoding::Utf8
    /// [ASCII encoding]: crate::Encoding::Ascii
    /// [binary encoding]: crate::Encoding::Binary
    /// [`len`]: Self::len
    /// [`bytesize`]: Self::bytesize
    #[inline]
    #[must_use]
    pub fn char_len(&self) -> usize {
        self.inner.char_len()
    }

    /// Returns the `index`'th character in the string.
    ///
    /// This function is encoding-aware. For `String`s with [UTF-8 encoding],
    /// multi-byte Unicode characters are length 1 and invalid UTF-8 bytes are
    /// length 1. For `String`s with [ASCII encoding] or [binary encoding],
    /// this function is equivalent to [`get`] with a range of length 1.
    ///
    /// # Examples
    ///
    /// ```
    /// use spinoso_string::String;
    ///
    /// let s = String::utf8(b"abc\xF0\x9F\x92\x8E\xFF".to_vec()); // "abc💎\xFF"
    /// assert_eq!(s.get_char(0), Some(&b"a"[..]));
    /// assert_eq!(s.get_char(1), Some(&b"b"[..]));
    /// assert_eq!(s.get_char(2), Some(&b"c"[..]));
    /// assert_eq!(s.get_char(3), Some("💎".as_bytes()));
    /// assert_eq!(s.get_char(4), Some(&b"\xFF"[..]));
    /// assert_eq!(s.get_char(5), None);
    ///
    /// let b = String::binary(b"abc\xF0\x9F\x92\x8E\xFF".to_vec()); // "abc💎\xFF"
    /// assert_eq!(b.get_char(0), Some(&b"a"[..]));
    /// assert_eq!(b.get_char(1), Some(&b"b"[..]));
    /// assert_eq!(b.get_char(2), Some(&b"c"[..]));
    /// assert_eq!(b.get_char(3), Some(&b"\xF0"[..]));
    /// assert_eq!(b.get_char(4), Some(&b"\x9F"[..]));
    /// assert_eq!(b.get_char(5), Some(&b"\x92"[..]));
    /// assert_eq!(b.get_char(6), Some(&b"\x8E"[..]));
    /// assert_eq!(b.get_char(7), Some(&b"\xFF"[..]));
    /// assert_eq!(b.get_char(8), None);
    /// ```
    ///
    /// [UTF-8 encoding]: crate::Encoding::Utf8
    /// [ASCII encoding]: crate::Encoding::Ascii
    /// [binary encoding]: crate::Encoding::Binary
    /// [`get`]: Self::get
    #[inline]
    #[must_use]
    pub fn get_char(&self, index: usize) -> Option<&'_ [u8]> {
        // `Vec` has a max allocation size of `isize::MAX`. For a `Vec<u8>` like
        // the one in `String` where the `size_of::<u8>() == 1`, the max length
        // is `isize::MAX`. This checked add short circuits with `None` if we
        // are given `usize::MAX` as an index, which we could never slice.
        index.checked_add(1)?;

        self.inner.get_char(index)
    }

    /// Returns a substring of characters in the string.
    ///
    /// This function is encoding-aware. For `String`s with [UTF-8 encoding],
    /// multi-byte Unicode characters are length 1 and invalid UTF-8 bytes are
    /// length 1. For `String`s with [ASCII encoding] or [binary encoding],
    /// this function is equivalent to [`get`] with a range.
    ///
    /// # Examples
    ///
    /// ```
    /// use core::ops::Range;
    /// use spinoso_string::String;
    ///
    /// let s = String::utf8(b"abc\xF0\x9F\x92\x8E\xFF".to_vec()); // "abc💎\xFF"
    /// assert_eq!(s.get_char_slice(Range { start:  0, end:  1 }), Some(&b"a"[..]));
    /// assert_eq!(s.get_char_slice(Range { start:  0, end:  3 }), Some(&b"abc"[..]));
    /// assert_eq!(s.get_char_slice(Range { start:  0, end:  4 }), Some("abc💎".as_bytes()));
    /// assert_eq!(s.get_char_slice(Range { start:  0, end:  5 }), Some(&b"abc\xF0\x9F\x92\x8E\xFF"[..]));
    /// assert_eq!(s.get_char_slice(Range { start:  3, end: 10 }), Some(&b"\xF0\x9F\x92\x8E\xFF"[..]));
    /// assert_eq!(s.get_char_slice(Range { start:  4, end: 10 }), Some(&b"\xFF"[..]));
    /// assert_eq!(s.get_char_slice(Range { start: 10, end: 15 }), None);
    /// assert_eq!(s.get_char_slice(Range { start: 15, end: 10 }), None);
    /// assert_eq!(s.get_char_slice(Range { start: 15, end:  1 }), None);
    /// assert_eq!(s.get_char_slice(Range { start:  4, end:  1 }), Some(&b""[..]));
    /// ```
    ///
    /// [UTF-8 encoding]: crate::Encoding::Utf8
    /// [ASCII encoding]: crate::Encoding::Ascii
    /// [binary encoding]: crate::Encoding::Binary
    /// [`get`]: Self::get
    #[inline]
    #[must_use]
    pub fn get_char_slice(&self, range: Range<usize>) -> Option<&'_ [u8]> {
        let Range { start, end } = range;

        // Fast path the lookup if the end of the range is before the start.
        if end < start {
            // Yes, these types of ranges are allowed and they return `""`.
            //
            // ```
            // [3.0.1] > "aaa"[1..0]
            // => ""
            // [3.0.1] > "aaa"[2..0]
            // => ""
            // [3.0.1] > "aaa"[2..1]
            // => ""
            // [3.0.1] > "aaa"[3..0]
            // => ""
            // [3.0.1] > "💎🦀😅"[2..1]
            // => ""
            // [3.0.1] > "💎🦀😅"[3..0]
            // => ""
            // ```
            //
            // but only if `start` is within the string.
            //
            // ```
            // [3.0.1] > "aaa"[10..4]
            // => nil
            // [3.0.1] > "aaa"[10..4]
            // => nil
            // [3.0.1] > "aaa"[10..0]
            // => nil
            // [3.0.1] > "💎🦀😅"[10..4]
            // => nil
            // [3.0.1] > "💎🦀😅"[10..0]
            // => nil
            // [3.0.1] > "💎🦀😅"[6..0]
            // => nil
            // [3.0.1] > "💎🦀😅"[4..0]
            // => nil
            // ```
            //
            // attempt to short-circuit with a cheap len retrieval
            if start > self.len() || start > self.char_len() {
                return None;
            }
            return Some(&[]);
        }

        // If the start of the range is beyond the character count of the
        // string, the whole lookup must fail.
        //
        // Slice lookups where the start is just beyond the last character index
        // always return an empty slice.
        //
        // ```
        // [3.0.1] > "aaa"[10, 0]
        // => nil
        // [3.0.1] > "aaa"[10, 7]
        // => nil
        // [3.0.1] > "aaa"[3, 7]
        // => ""
        // [3.0.1] > "🦀💎"[2, 0]
        // => ""
        // [3.0.1] > "🦀💎"[3, 1]
        // => nil
        // [3.0.1] > "🦀💎"[2, 1]
        // => ""
        // ```
        //
        // Fast path rejection for indexes beyond bytesize, which is cheap to
        // retrieve.
        if start > self.len() {
            return None;
        }
        match self.char_len() {
            char_length if start > char_length => return None,
            char_length if start == char_length => return Some(&[]),
            _ => {}
        }

        // The span is guaranteed to at least partially overlap now.
        match end - start {
            // Empty substrings are present in all strings, even empty ones.
            //
            // ```
            // [3.0.1] > "aaa"[""]
            // => ""
            // [3.0.1] > ""[""]
            // => ""
            // [3.0.1] > ""[0, 0]
            // => ""
            // [3.0.1] > "aaa"[0, 0]
            // => ""
            // [3.0.1] > "aaa"[2, 0]
            // => ""
            // [3.0.1] > "🦀💎"[1, 0]
            // => ""
            // [3.0.1] > "🦀💎"[2, 0]
            // => ""
            // ```
            0 => return Some(&[]),
            // Delegate to the specialized single char lookup, which allows the
            // remainder of this routine to fall back to the general case of
            // multi-character spans.
            //
            // ```
            // [3.0.1] > "abc"[2, 1]
            // => "c"
            // [3.0.1] > "🦀💎"[1, 1]
            // => "💎"
            // ```
            1 => return self.get_char(start),
            _ => {}
        }

        self.inner.get_char_slice(range)
    }

    /// Returns true for a `String` which is encoded correctly.
    ///
    /// For this method to return true, `String`s with [conventionally UTF-8]
    /// must be well-formed UTF-8; [ASCII]-encoded `String`s must only contain
    /// bytes in the range `0..=127`; [binary]-encoded `String`s may contain any
    /// byte sequence.
    ///
    /// This method is suitable for implementing the Ruby method
    /// [`String#valid_encoding?`].
    ///
    /// # Examples
    ///
    /// ```
    /// use spinoso_string::{Encoding, String};
    ///
    /// let s = String::utf8(b"xyz".to_vec());
    /// assert!(s.is_valid_encoding());
    /// let s = String::utf8("🚀".to_string().into_bytes());
    /// assert!(s.is_valid_encoding());
    /// let s = String::utf8(b"abc\xFF\xFExyz".to_vec());
    /// assert!(!s.is_valid_encoding());
    ///
    /// let s = String::ascii(b"xyz".to_vec());
    /// assert!(s.is_valid_encoding());
    /// let s = String::ascii("🚀".to_string().into_bytes());
    /// assert!(!s.is_valid_encoding());
    /// let s = String::ascii(b"abc\xFF\xFExyz".to_vec());
    /// assert!(!s.is_valid_encoding());
    ///
    /// let s = String::binary(b"xyz".to_vec());
    /// assert!(s.is_valid_encoding());
    /// let s = String::binary("🚀".to_string().into_bytes());
    /// assert!(s.is_valid_encoding());
    /// let s = String::binary(b"abc\xFF\xFExyz".to_vec());
    /// assert!(s.is_valid_encoding());
    /// ```
    ///
    /// [conventionally UTF-8]: crate::Encoding::Utf8
    /// [ASCII]: crate::Encoding::Ascii
    /// [binary]: crate::Encoding::Binary
    /// [`String#valid_encoding?`]: https://ruby-doc.org/core-3.0.0/String.html#method-i-valid_encoding-3F
    #[inline]
    #[must_use]
    pub fn is_valid_encoding(&self) -> bool {
        self.inner.is_valid_encoding()
    }
}

#[must_use]
fn chomp(string: &mut String, separator: Option<&[u8]>) -> bool {
    if string.is_empty() {
        return false;
    }
    match separator {
        Some(separator) if separator.is_empty() => {
            let original_len = string.len();
            let mut iter = string.bytes().rev().peekable();
            while let Some(&b'\n') = iter.peek() {
                iter.next();
                if let Some(&b'\r') = iter.peek() {
                    iter.next();
                }
            }
            let truncate_to = iter.count();
            string.inner.truncate(truncate_to);
            truncate_to != original_len
        }
        Some(separator) if string.inner.ends_with(separator) => {
            let original_len = string.len();
            // This subtraction is guaranteed not to panic because
            // `separator` is a substring of `buf`.
            let truncate_to_len = original_len - separator.len();
            string.inner.truncate(truncate_to_len);
            // Separator is non-empty and we are always truncating, so this
            // branch always modifies the buffer.
            true
        }
        Some(_) => false,
        None => {
            let original_len = string.len();
            let mut iter = string.bytes().rev().peekable();
            match iter.peek() {
                Some(&b'\n') => {
                    iter.next();
                    if let Some(&b'\r') = iter.peek() {
                        iter.next();
                    }
                }
                Some(b'\r') => {
                    iter.next();
                }
                Some(_) | None => {}
            };
            let truncate_to_len = iter.count();
            string.inner.truncate(truncate_to_len);
            truncate_to_len != original_len
        }
    }
}

#[cfg(test)]
#[allow(clippy::invisible_characters)]
mod tests {
    use crate::center::CenterError;
    use crate::String;

    #[test]
    fn center_returns_error_with_empty_padding() {
        let s = String::utf8(b"jumbo".to_vec());
        let center = s.center(10, Some(b""));
        assert!(matches!(center, Err(CenterError::ZeroWidthPadding)));

        let center = s.center(9, Some(b""));
        assert!(matches!(center, Err(CenterError::ZeroWidthPadding)));

        let center = s.center(1, Some(b""));
        assert!(matches!(center, Err(CenterError::ZeroWidthPadding)));

        let center = s.center(5, Some(b""));
        assert!(matches!(center, Err(CenterError::ZeroWidthPadding)));
    }

    #[test]
    fn strings_compare_equal_only_based_on_byte_content() {
        let utf8 = String::utf8(b"abc".to_vec());
        let ascii = String::ascii(b"abc".to_vec());
        let binary = String::binary(b"abc".to_vec());
        assert_eq!(utf8, ascii);
        assert_eq!(utf8, binary);
        assert_eq!(binary, ascii);
    }

    #[test]
    fn strings_compare_equal_only_based_on_byte_content_without_valid_encoding() {
        let utf8 = String::utf8(b"abc\xFE\xFF".to_vec());
        let ascii = String::ascii(b"abc\xFE\xFF".to_vec());
        let binary = String::binary(b"abc\xFE\xFF".to_vec());
        assert_eq!(utf8, ascii);
        assert_eq!(utf8, binary);
        assert_eq!(binary, ascii);
    }
}
