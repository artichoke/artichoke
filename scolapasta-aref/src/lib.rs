#![warn(clippy::all)]
#![warn(clippy::pedantic)]
#![warn(clippy::cargo)]
#![allow(unknown_lints)]
#![allow(clippy::manual_let_else)]
#![warn(missing_docs)]
#![warn(missing_debug_implementations)]
#![warn(missing_copy_implementations)]
#![warn(rust_2018_idioms)]
#![warn(rust_2021_compatibility)]
#![warn(trivial_casts, trivial_numeric_casts)]
#![warn(unused_qualifications)]
#![warn(variant_size_differences)]
#![forbid(unsafe_code)]
// Enable feature callouts in generated documentation:
// https://doc.rust-lang.org/beta/unstable-book/language-features/doc-cfg.html
//
// This approach is borrowed from tokio.
#![cfg_attr(docsrs, feature(doc_cfg))]
#![cfg_attr(docsrs, feature(doc_alias))]

//! Functions for working with Ruby containers that respond to `#[]` or "aref".
//!
//! Convert offsets to `usize` indexes like this:
//!
//! ```
//! # fn example() -> Option<()> {
//! let data = "ABC, 123, XYZ";
//! let offset = -5;
//! let index = scolapasta_aref::offset_to_index(offset, data.len())?;
//! assert_eq!(index, 8);
//! assert_eq!(&data[index..], ", XYZ");
//! # Some(())
//! # }
//! # example().unwrap()
//! ```

#![no_std]

// Ensure code blocks in `README.md` compile
#[cfg(doctest)]
#[doc = include_str!("../README.md")]
mod readme {}

/// Convert a signed aref offset to a `usize` index into the underlying container.
///
/// Negative indexes are interpreted as indexing from the end of the container
/// as long as their magnitude is less than the given length.
///
/// Callers must still check whether the returned index is in bounds for the
/// container.
///
/// # Examples
///
/// ```
/// # fn example() -> Option<()> {
/// let data = "ABC, 123, XYZ";
/// let offset = 6;
/// let index = scolapasta_aref::offset_to_index(offset, data.len())?;
/// assert_eq!(index, 6);
/// assert_eq!(&data[index..], "23, XYZ");
///
/// let data = "ABC, 123, XYZ";
/// let offset = -5;
/// let index = scolapasta_aref::offset_to_index(offset, data.len())?;
/// assert_eq!(index, 8);
/// assert_eq!(&data[index..], ", XYZ");
///
/// let offset = -44;
/// let index = scolapasta_aref::offset_to_index(offset, data.len());
/// assert_eq!(index, None);
/// # Some(())
/// # }
/// # example().unwrap()
/// ```
#[must_use]
pub fn offset_to_index(index: i64, len: usize) -> Option<usize> {
    // Here's an example of this behavior from `String`. All containers that
    // respond to `#[]` ("aref") behave similarly.
    //
    // ```
    // [3.0.1] > s = "abc"
    // => "abc"
    //
    // [3.0.1] > s[-2]
    // => "b"
    // [3.0.1] > s[-3]
    // => "a"
    // [3.0.1] > s[-4]
    // => nil
    //
    // [3.0.1] > s[-2, 10]
    // => "bc"
    // [3.0.1] > s[-3, 10]
    // => "abc"
    // [3.0.1] > s[-4, 10]
    //
    // [3.0.2] > s.byteslice(-2, 10)
    // => "bc"
    // [3.0.2] > s.byteslice(-3, 10)
    // => "abc"
    // [3.0.2] > s.byteslice(-4, 10)
    // => nil
    // => nil
    // ```
    if let Ok(index) = usize::try_from(index) {
        Some(index)
    } else {
        index
            .checked_neg()
            .and_then(|index| usize::try_from(index).ok())
            .and_then(|index| len.checked_sub(index))
    }
}

#[cfg(test)]
mod tests {
    use super::offset_to_index;

    #[test]
    fn zero_index() {
        let test_cases = [
            (0_i64, 0_usize, Some(0_usize)),
            (0, 1, Some(0)),
            (0, usize::MAX, Some(0)),
        ];
        for (index, len, expected) in test_cases {
            assert_eq!(
                offset_to_index(index, len),
                expected,
                "unexpected result for index {index}, len {len}"
            );
        }
    }

    #[test]
    fn positive_index() {
        let test_cases = [
            (1_i64, 0_usize, Some(1_usize)),
            (1, 1, Some(1)),
            (1, usize::MAX, Some(1)),
            (123, 0, Some(123)),
            (123, 1, Some(123)),
            (123, 123, Some(123)),
            (123, usize::MAX, Some(123)),
            (i64::MAX, usize::MAX, Some(i64::MAX.try_into().unwrap())),
        ];
        for (index, len, expected) in test_cases {
            assert_eq!(
                offset_to_index(index, len),
                expected,
                "unexpected result for index {index}, len {len}"
            );
        }
    }

    #[test]
    fn negative_index() {
        let test_cases = [
            (-1_i64, 0_usize, None),
            (-1, 1, Some(0)),
            (-1, 2, Some(1)),
            (-1, 10, Some(9)),
            (-1, 245, Some(244)),
            (-10, 0, None),
            (-10, 1, None),
            (-10, 2, None),
            (-10, 10, Some(0)),
            (-10, 245, Some(235)),
            (-123, 0, None),
            (-123, 1, None),
            (-123, 2, None),
            (-123, 10, None),
            (-123, 245, Some(122)),
            (i64::MIN, 0, None),
            (i64::MIN, 1, None),
            (i64::MIN, 2, None),
            (i64::MIN, 10, None),
            (i64::MIN, 245, None),
        ];
        for (index, len, expected) in test_cases {
            assert_eq!(
                offset_to_index(index, len),
                expected,
                "unexpected result for index {index}, len {len}"
            );
        }
    }
}
