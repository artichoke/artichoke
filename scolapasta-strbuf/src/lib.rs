#![warn(clippy::all)]
#![warn(clippy::pedantic)]
#![warn(clippy::cargo)]
#![allow(unknown_lints)]
#![allow(clippy::manual_let_else)]
#![allow(clippy::module_name_repetitions)]
#![warn(missing_docs)]
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

//! A contiguous growable byte string, written as `Buf`, short for 'buffer'.
//!
//! Buffers have *O*(1) indexing, amortized *O*(1) push (to the end) and *O*(1)
//! pop (from the end).
//!
//! Buffers ensure they never allocate more than `isize::MAX` bytes.
//!
//! # Examples
//!
//! You can explicitly create a [`Buf`] with [`Buf::new`]:
//!
//! ```
//! use scolapasta_strbuf::Buf;
//!
//! let buf = Buf::new();
//! ```
//!
//! You can [`push_byte`] bytes into the end of a buffer (which will grow the
//! buffer as needed):
//!
//! ```
//! use scolapasta_strbuf::Buf;
//!
//! let mut buf = Buf::from(b"12");
//!
//! buf.push_byte(b'3');
//! assert_eq!(buf, b"123");
//! ```
//!
//! Popping bytes works in much the same way:
//!
//! ```
//! use scolapasta_strbuf::Buf;
//!
//! let mut buf = Buf::from(b"12");
//!
//! let alpha_two = buf.pop_byte();
//! assert_eq!(alpha_two, Some(b'2'));
//! ```
//!
//! Buffers also support indexing (through the [`Index`] and [`IndexMut`]
//! traits):
//!
//! ```
//! use scolapasta_strbuf::Buf;
//!
//! let mut buf = Buf::from(b"123");
//! let three = buf[2];
//! buf[1] = b'!';
//! ```
//!
//! # Crate features
//!
//! - **std**: Enabled by default. Implement [`std::io::Write`] for `Buf`. If
//!   this feature is disabled, this crate only depends on [`alloc`].
//! - **nul-terminated**: Use an alternate byte buffer backend that ensures
//!   byte content is always followed by a NUL byte in the buffer's spare
//!   capacity. This feature can be used to ensure `Buf`s are FFI compatible
//!   with C code that expects byte content to be NUL terminated.
//!
#![cfg_attr(
    not(feature = "std"),
    doc = "[`std::io::Write`]: https://doc.rust-lang.org/std/io/trait.Write.html"
)]
//! [`push_byte`]: Buf::push_byte
//! [`Index`]: core::ops::Index
//! [`IndexMut`]: core::ops::IndexMut

#![no_std]

// Ensure code blocks in `README.md` compile
#[cfg(doctest)]
#[doc = include_str!("../README.md")]
mod readme {}

extern crate alloc;
#[cfg(feature = "std")]
extern crate std;

macro_rules! impl_partial_eq {
    ($lhs:ty, $rhs:ty) => {
        impl<'a, 'b> PartialEq<$rhs> for $lhs {
            #[inline]
            fn eq(&self, other: &$rhs) -> bool {
                let other: &[u8] = other.as_ref();
                PartialEq::eq(self.as_slice(), other)
            }
        }

        impl<'a, 'b> PartialEq<$lhs> for $rhs {
            #[inline]
            fn eq(&self, other: &$lhs) -> bool {
                let this: &[u8] = self.as_ref();
                PartialEq::eq(this, other.as_slice())
            }
        }
    };
}

macro_rules! impl_partial_eq_array {
    ($lhs:ty, $rhs:ty) => {
        impl<'a, 'b, const N: usize> PartialEq<$rhs> for $lhs {
            #[inline]
            fn eq(&self, other: &$rhs) -> bool {
                let other: &[u8] = other.as_ref();
                PartialEq::eq(self.as_slice(), other)
            }
        }

        impl<'a, 'b, const N: usize> PartialEq<$lhs> for $rhs {
            #[inline]
            fn eq(&self, other: &$lhs) -> bool {
                let this: &[u8] = self.as_ref();
                PartialEq::eq(this, other.as_slice())
            }
        }
    };
}

pub use raw_parts::RawParts;

mod nul_terminated_vec;
mod vec;

#[cfg(feature = "nul-terminated")]
use nul_terminated_vec as imp;
#[cfg(not(feature = "nul-terminated"))]
use vec as imp;

pub use imp::Buf;
