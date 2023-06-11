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
//! # Examples
//!
//! Index into arrays:
//!
//! ```
//! # fn example() -> Option<()> {
//! let data = [1, 2, 3, 4, 5];
//!
//! // Positive offset
//! let offset = 2;
//! let index = scolapasta_aref::offset_to_index(offset, data.len())?;
//! assert_eq!(index, 2);
//! assert_eq!(data[index], 3);
//!
//! // Negative offset
//! let offset = -3;
//! let index = scolapasta_aref::offset_to_index(offset, data.len())?;
//! assert_eq!(index, 2);
//! assert_eq!(data[index], 3);
//!
//! // Out-of-bounds offset
//! let offset = 10;
//! let index = scolapasta_aref::offset_to_index(offset, data.len())?;
//! assert_eq!(index, 10);
//!
//! // Out-of-bounds negative offset
//! let offset = -10;
//! let index = scolapasta_aref::offset_to_index(offset, data.len());
//! assert_eq!(index, None);
//! # Some(())
//! # }
//! # example().unwrap()
//! ```
//!
//! Index into strings:
//!
//! ```
//! # fn example() -> Option<()> {
//! let data = "Hello, World!";
//!
//! // Positive offset
//! let offset = 7;
//! let index = scolapasta_aref::offset_to_index(offset, data.len())?;
//! assert_eq!(index, 7);
//! assert_eq!(&data[index..], "World!");
//!
//! // Negative offset
//! let offset = -6;
//! let index = scolapasta_aref::offset_to_index(offset, data.len())?;
//! assert_eq!(index, 7);
//! assert_eq!(&data[index..], "World!");
//!
//! // Out-of-bounds offset
//! let offset = 20;
//! let index = scolapasta_aref::offset_to_index(offset, data.len())?;
//! assert_eq!(index, 20);
//!
//! // Out-of-bounds negative offset
//! let offset = -20;
//! let index = scolapasta_aref::offset_to_index(offset, data.len());
//! assert_eq!(index, None);
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
/// container. The returned index may be out of range since this routine can be
/// used to calculate indexes beyond the length of the container during
/// assignment (for example, `Array#[]=` may perform length-extension upon an
/// out-of-bounds index).
///
/// # Examples
///
/// ```
/// # fn example() -> Option<()> {
/// let data = "ABC, 123, XYZ";
///
/// let offset = 6;
/// let index = scolapasta_aref::offset_to_index(offset, data.len())?;
/// assert_eq!(index, 6);
/// assert_eq!(&data[index..], "23, XYZ");
///
/// let offset = 55;
/// let index = scolapasta_aref::offset_to_index(offset, data.len())?;
/// assert_eq!(index, 55);
///
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
    match usize::try_from(index) {
        Ok(index) => Some(index),
        Err(_) => index
            .checked_neg()
            .and_then(|index| usize::try_from(index).ok())
            .and_then(|index| len.checked_sub(index)),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_zero_index() {
        // Test case: index = 0, len = 0
        assert_eq!(offset_to_index(0_i64, 0_usize), Some(0_usize));

        // Test case: index = 0, len = 1
        assert_eq!(offset_to_index(0_i64, 1_usize), Some(0_usize));

        // Test case: index = 0, len = 10
        assert_eq!(offset_to_index(0, 10), Some(0_usize));

        // Test case: index = 0, len = usize::MAX
        assert_eq!(offset_to_index(0_i64, usize::MAX), Some(0_usize));
    }

    #[test]
    fn test_positive_index() {
        // Test case: index = 1, len = 0
        assert_eq!(offset_to_index(1_i64, 0_usize), Some(1_usize));

        // Test case: index = 1, len = 1
        assert_eq!(offset_to_index(1_i64, 1_usize), Some(1_usize));

        // Test case: index = 1, len = 2
        assert_eq!(offset_to_index(1_i64, 2_usize), Some(1_usize));

        // Test case: index = 1, len = usize::MAX
        assert_eq!(offset_to_index(1_i64, usize::MAX), Some(1_usize));

        // Test case: index = 15, len = 10
        assert_eq!(offset_to_index(15, 10), Some(15_usize));

        // Test case: index = 123, len = 0
        assert_eq!(offset_to_index(123_i64, 0_usize), Some(123_usize));

        // Test case: index = 123, len = 1
        assert_eq!(offset_to_index(123_i64, 1_usize), Some(123_usize));

        // Test case: index = 123, len = 123
        assert_eq!(offset_to_index(123_i64, 123_usize), Some(123_usize));

        // Test case: index = 123, len = 123
        assert_eq!(offset_to_index(123_i64, 124_usize), Some(123_usize));

        // Test case: index = 123, len = 123
        assert_eq!(offset_to_index(123_i64, 500_usize), Some(123_usize));

        // Test case: index = 123, len = usize::MAX
        assert_eq!(offset_to_index(123_i64, usize::MAX), Some(123_usize));

        // Test case: index = i64::MAX, len = 5
        #[cfg(target_pointer_width = "64")]
        assert_eq!(offset_to_index(i64::MAX, 5), Some(usize::try_from(i64::MAX).unwrap()));

        // Test case: index = i64::MAX, len = usize::MAX
        #[cfg(target_pointer_width = "64")]
        assert_eq!(
            offset_to_index(i64::MAX, usize::MAX),
            Some(usize::try_from(i64::MAX).unwrap())
        );

        // Test case: index = 100, len = 1000
        assert_eq!(offset_to_index(100_i64, 1000_usize), Some(100_usize));

        // Test case: index = 500, len = 500
        assert_eq!(offset_to_index(500_i64, 500_usize), Some(500_usize));

        // Test case: index = 999, len = 100
        assert_eq!(offset_to_index(999_i64, 100_usize), Some(999_usize));
    }

    #[test]
    fn test_negative_index() {
        // Test case: index = -1, len = 0
        assert_eq!(offset_to_index(-1_i64, 0_usize), None);

        // Test case: index = -1, len = 1
        assert_eq!(offset_to_index(-1_i64, 1_usize), Some(0_usize));

        // Test case: index = -1, len = 2
        assert_eq!(offset_to_index(-1_i64, 2_usize), Some(1_usize));

        // Test case: index = -1, len = 10
        assert_eq!(offset_to_index(-1_i64, 10_usize), Some(9_usize));

        // Test case: index = -1, len = 245
        assert_eq!(offset_to_index(-1_i64, 245_usize), Some(244_usize));

        // Test case: index = -10, len = 0
        assert_eq!(offset_to_index(-10_i64, 0_usize), None);

        // Test case: index = -10, len = 1
        assert_eq!(offset_to_index(-10_i64, 1_usize), None);

        // Test case: index = -10, len = 2
        assert_eq!(offset_to_index(-10_i64, 2_usize), None);

        // Test case: index = -10, len = 10
        assert_eq!(offset_to_index(-10_i64, 10_usize), Some(0_usize));

        // Test case: index = -10, len = 245
        assert_eq!(offset_to_index(-10_i64, 245_usize), Some(235_usize));

        // Test case: index = -123, len = 0
        assert_eq!(offset_to_index(-123_i64, 0_usize), None);

        // Test case: index = -123, len = 1
        assert_eq!(offset_to_index(-123_i64, 1_usize), None);

        // Test case: index = -123, len = 2
        assert_eq!(offset_to_index(-123_i64, 2_usize), None);

        // Test case: index = -123, len = 10
        assert_eq!(offset_to_index(-123_i64, 10_usize), None);

        // Test case: index = -123, len = 245
        assert_eq!(offset_to_index(-123_i64, 245_usize), Some(122_usize));

        // Test case: index = i64::MIN, len = 0
        assert_eq!(offset_to_index(i64::MIN, 0_usize), None);

        // Test case: index = i64::MIN, len = 1
        assert_eq!(offset_to_index(i64::MIN, 1_usize), None);

        // Test case: index = i64::MIN, len = 2
        assert_eq!(offset_to_index(i64::MIN, 2_usize), None);

        // Test case: index = i64::MIN, len = 10
        assert_eq!(offset_to_index(i64::MIN, 10_usize), None);

        // Test case: index = i64::MIN, len = 245
        assert_eq!(offset_to_index(i64::MIN, 245_usize), None);
    }

    #[test]
    fn test_out_of_bounds_positive_offset() {
        // Test case: Offset greater than or equal to length
        //
        // ```
        // [3.2.2] > a = [1,2,3,4,5]
        // => [1, 2, 3, 4, 5]
        // [3.2.2] > a[10]
        // => nil
        // [3.2.2] > a[10] = 'a'
        // => "a"
        // [3.2.2] > a
        // => [1, 2, 3, 4, 5, nil, nil, nil, nil, nil, "a"]
        // ```
        assert_eq!(offset_to_index(10, 5), Some(10_usize));
    }

    #[test]
    fn test_positive_offset_equal_to_length() {
        // ```
        // [3.2.2] > a = [1,2,3,4,5]
        // => [1, 2, 3, 4, 5]
        // [3.2.2] > a[5]
        // => nil
        // [3.2.2] > a[5, 0]
        // => []
        // [3.2.2] > a[5] = 'a'
        // => "a"
        // [3.2.2] > a
        // => [1, 2, 3, 4, 5, "a"]
        // ```
        assert_eq!(offset_to_index(5, 5), Some(5_usize));
    }

    #[test]
    fn test_negative_offset_of_magnitude_length() {
        // Test case: Offset equal to negative length
        //
        // ```
        // [3.2.2] > a = [1,2,3,4,5]
        // => [1, 2, 3, 4, 5]
        // [3.2.2] > a[-5]
        // => 1
        // [3.2.2] > a[-5] = 'a'
        // => "a"
        // [3.2.2] > a
        // => ["a", 2, 3, 4, 5]
        // ```
        assert_eq!(offset_to_index(-5, 5), Some(0));

        assert_eq!(offset_to_index(-10, 10), Some(0_usize));
    }

    #[test]
    fn test_invalid_negative_offset() {
        // Test case: Offset less than negative length
        //
        // ```
        // [3.2.2] > a = [1,2,3,4,5]
        // => [1, 2, 3, 4, 5]
        // [3.2.2] > a[-10]
        // => nil
        // [3.2.2] > a[-10] = 'a'
        // (irb):5:in `<main>': index -10 too small for array; minimum: -5 (IndexError)
        // ```
        assert_eq!(offset_to_index(-10, 5), None);
    }

    #[test]
    fn test_edge_cases() {
        // Test case: Length is zero
        assert_eq!(offset_to_index(0, 0), Some(0_usize));

        // Test case: Offset is the minimum `i64` value
        assert_eq!(offset_to_index(i64::MIN, 10), None);

        // Test case: Offset is the maximum `i64` value
        #[cfg(target_pointer_width = "64")]
        assert_eq!(offset_to_index(i64::MAX, 10), Some(usize::try_from(i64::MAX).unwrap()));

        // Test case: index = 0, len = usize::MAX
        assert_eq!(offset_to_index(0_i64, usize::MAX), Some(0_usize));

        // Test case: index = 1, len = usize::MAX
        assert_eq!(offset_to_index(1_i64, usize::MAX), Some(1_usize));

        // Test case: index = -1, len = usize::MAX
        assert_eq!(offset_to_index(-1_i64, usize::MAX), Some(usize::MAX - 1));

        // Test case: index = 10, len = usize::MAX
        assert_eq!(offset_to_index(10, usize::MAX), Some(10_usize));

        // Test case: index = i64::MAX, len = usize::MAX
        #[cfg(target_pointer_width = "64")]
        assert_eq!(
            offset_to_index(i64::MAX, usize::MAX),
            Some(usize::try_from(i64::MAX).unwrap())
        );
    }
}
