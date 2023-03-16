#![warn(clippy::all)]
#![warn(clippy::pedantic)]
#![warn(clippy::cargo)]
#![allow(unknown_lints)]
#![allow(clippy::manual_let_else)]
// to use value receivers for primitives like `f64::is_nan` does in `std`.
#![allow(clippy::wrong_self_convention)]
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

//! Functions for converting numeric immediates to integer or "fixnum"
//! immediates.
//!
//! Fixnums have range of a 63-bit unsigned int and are returned as a native
//! representation [`i64`].
//!
//! # Usage
//!
//! Check whether a numeric value is able to be converted to an in-range
//! "fixnum":
//!
//! ```
//! use scolapasta_fixable::RB_FIXABLE;
//!
//! assert!(RB_FIXABLE(23_u8));
//! assert!(RB_FIXABLE(u16::MIN));
//! assert!(RB_FIXABLE(i32::MAX));
//! assert!(RB_FIXABLE(1024_u64));
//! assert!(RB_FIXABLE(1024_i64));
//! assert!(RB_FIXABLE(1.0_f32));
//! assert!(RB_FIXABLE(-9000.27_f64));
//! ```
//!
//! This crate also exports a [`Fixable`] trait which provides methods on
//! numeric types to check if they are fixable and to do a fallible conversion
//! to an [`i64`] fixnum.
//!
//! ```
//! use scolapasta_fixable::Fixable;
//!
//! assert!(23_u8.is_fixable());
//! assert_eq!(23_u8.to_fix(), Some(23_i64));
//! assert!(-9000.27_f64.is_fixable());
//! assert_eq!(-9000.27_f64.to_fix(), Some(-9000_i64));
//! ```
//!
//! Some numeric types, such as [`u64`], [`i128`], and [`f64`] have values that
//! exceed fixnum range. Conversions on values of these types which are outside
//! the 63-bit int range will fail:
//!
//! ```rust
//! use scolapasta_fixable::Fixable;
//!
//! assert_eq!(u64::MAX.to_fix(), None);
//! assert_eq!(i128::MIN.to_fix(), None);
//! assert_eq!(4_611_686_018_427_387_904.0_f64.to_fix(), None);
//! assert_eq!(f64::INFINITY, None);
//! assert_eq!(f64::NAN, None);
//! ```
//!
//! For non-integer fixable types, the fractional part is discarded when converting
//! to fixnum, i.e. converting to fixnum rounds to zero.
//!
//! # Panics
//!
//! All routines in this crate are implemented with checked operations and will
//! never panic.

#![no_std]

// Ensure code blocks in `README.md` compile
#[cfg(doctest)]
#[doc = include_str!("../README.md")]
mod readme {}

use core::time::Duration;

/// The maximum possible value that a fixnum can represent, 63 bits of an
/// [`i64`].
///
/// # C Declaration
///
/// ```c
/// /** Maximum possible value that a fixnum can represent. */
/// #define RUBY_FIXNUM_MAX  (LONG_MAX / 2)
/// ```
pub const RUBY_FIXNUM_MAX: i64 = i64::MAX / 2;

pub(crate) const RUBY_FIXNUM_MAX_U64: u64 = RUBY_FIXNUM_MAX as u64;
pub(crate) const RUBY_FIXNUM_MAX_U128: u128 = RUBY_FIXNUM_MAX as u128;

/// The minimum possible value that a fixnum can represent, 63 bits of an
/// [`i64`].
///
/// # C Declaration
///
/// ```c
/// /** Minimum possible value that a fixnum can represent. */
/// #define RUBY_FIXNUM_MIN  (LONG_MIN / 2)
/// ```
pub const RUBY_FIXNUM_MIN: i64 = i64::MIN / 2;

mod private {
    pub trait Sealed {}

    impl Sealed for i8 {}
    impl Sealed for i16 {}
    impl Sealed for i32 {}
    impl Sealed for i64 {}
    impl Sealed for i128 {}

    impl Sealed for u8 {}
    impl Sealed for u16 {}
    impl Sealed for u32 {}
    impl Sealed for u64 {}
    impl Sealed for u128 {}

    impl Sealed for f32 {}
    impl Sealed for f64 {}
}

/// Marker trait for numeric values which can be converted to a "fixnum", or
/// Integer, representation.
///
/// A numeric value is fixable if its integral portion can fit within 63 bits of
/// an [`i64`].
///
/// See [`RUBY_FIXNUM_MIN`] and [`RUBY_FIXNUM_MAX`] for more details on the range
/// of values yielded by implementers of this trait.
///
/// This trait is sealed and cannot be implmented outside of this crate.
pub trait Fixable: private::Sealed + Sized {
    /// Convert a fixable numeric value to its integral part.
    ///
    /// This method returns [`None`] if `self` is out of range.
    #[must_use]
    fn to_fix(self) -> Option<i64>;

    /// Test whether a fixable numeric value is in range.
    #[must_use]
    fn is_fixable(self) -> bool {
        self.to_fix().is_some()
    }
}

impl Fixable for i8 {
    /// Convert an [`i8`] to a fixnum.
    ///
    /// This method on [`i8`] is infallible and will always return `Some(self)`
    /// since `i8` is always in range of fixnum.
    fn to_fix(self) -> Option<i64> {
        Some(self.into())
    }

    /// Test whether an [`i8`] value is in range of fixnum.
    ///
    /// This method on [`i8`] will always return `true` since `i8` is always in
    /// range of fixnum.
    fn is_fixable(self) -> bool {
        true
    }
}

impl Fixable for i16 {
    /// Convert an [`i16`] to a fixnum.
    ///
    /// This method on [`i16`] is infallible and will always return `Some(self)`
    /// since `i16` is always in range of fixnum.
    fn to_fix(self) -> Option<i64> {
        Some(self.into())
    }

    /// Test whether an [`i16`] value is in range of fixnum.
    ///
    /// This method on [`i16`] will always return `true` since `i16` is always in
    /// range of fixnum.
    fn is_fixable(self) -> bool {
        true
    }
}

impl Fixable for i32 {
    /// Convert an [`i32`] to a fixnum.
    ///
    /// This method on [`i32`] is infallible and will always return `Some(self)`
    /// since `i32` is always in range of fixnum.
    fn to_fix(self) -> Option<i64> {
        Some(self.into())
    }

    /// Test whether an [`i32`] value is in range of fixnum.
    ///
    /// This method on [`i32`] will always return `true` since `i32` is always in
    /// range of fixnum.
    fn is_fixable(self) -> bool {
        true
    }
}

impl Fixable for i64 {
    /// Convert an [`i64`] to a fixnum if it is less than or equal to
    /// [`RUBY_FIXNUM_MAX`] and greater than or equal to [`RUBY_FIXNUM_MIN`] in
    /// magnitude.
    ///
    /// This method returns [`None`] if the receiver is greater than
    /// [`RUBY_FIXNUM_MAX`] or less than [`RUBY_FIXNUM_MIN`].
    fn to_fix(self) -> Option<i64> {
        if self > RUBY_FIXNUM_MAX {
            return None;
        }
        if self < RUBY_FIXNUM_MIN {
            return None;
        }
        Some(self)
    }

    /// Test whether an [`i64`] value is in range of fixnum.
    ///
    /// This method returns `false` if the receiver is greater than
    /// [`RUBY_FIXNUM_MAX`] or less than [`RUBY_FIXNUM_MAX`].
    fn is_fixable(self) -> bool {
        (RUBY_FIXNUM_MIN..=RUBY_FIXNUM_MAX).contains(&self)
    }
}

impl Fixable for i128 {
    /// Convert an [`i128`] to a fixnum if it is less than or equal to
    /// [`RUBY_FIXNUM_MAX`] and greater than or equal to [`RUBY_FIXNUM_MIN`] in
    /// magnitude.
    ///
    /// This method returns [`None`] if the receiver is greater than
    /// [`RUBY_FIXNUM_MAX`] or less than [`RUBY_FIXNUM_MIN`].
    fn to_fix(self) -> Option<i64> {
        let x = i64::try_from(self).ok()?;
        x.to_fix()
    }

    /// Test whether an [`i128`] value is in range of fixnum.
    ///
    /// This method returns `false` if the receiver is greater than
    /// [`RUBY_FIXNUM_MAX`] or less than [`RUBY_FIXNUM_MAX`].
    fn is_fixable(self) -> bool {
        (RUBY_FIXNUM_MIN.into()..=RUBY_FIXNUM_MAX.into()).contains(&self)
    }
}

impl Fixable for u8 {
    /// Convert a [`u8`] to a fixnum.
    ///
    /// This method on [`u8`] is infallible and will always return `Some(self)`
    /// since `u8` is always in range of fixnum.
    fn to_fix(self) -> Option<i64> {
        Some(self.into())
    }

    /// Test whether a [`u8`] value is in range of fixnum.
    ///
    /// This method on [`u8`] will always return `true` since `u8` is always in
    /// range of fixnum.
    fn is_fixable(self) -> bool {
        true
    }
}

impl Fixable for u16 {
    /// Convert a [`u16`] to a fixnum.
    ///
    /// This method on [`u16`] is infallible and will always return `Some(self)`
    /// since `u16` is always in range of fixnum.
    fn to_fix(self) -> Option<i64> {
        Some(self.into())
    }

    /// Test whether a [`u16`] value is in range of fixnum.
    ///
    /// This method on [`u16`] will always return `true` since `u16` is always in
    /// range of fixnum.
    fn is_fixable(self) -> bool {
        true
    }
}

impl Fixable for u32 {
    /// Convert a [`u32`] to a fixnum.
    ///
    /// This method on [`u32`] is infallible and will always return `Some(self)`
    /// since `u32` is always in range of fixnum.
    fn to_fix(self) -> Option<i64> {
        Some(self.into())
    }

    /// Test whether a [`u32`] value is in range of fixnum.
    ///
    /// This method on [`u32`] will always return `true` since `u32` is always in
    /// range of fixnum.
    fn is_fixable(self) -> bool {
        true
    }
}

impl Fixable for u64 {
    /// Convert a [`u64`] to a fixnum if it is less than or equal to
    /// [`RUBY_FIXNUM_MAX`] in magnitude.
    ///
    /// This method returns [`None`] if the receiver is greater than
    /// [`RUBY_FIXNUM_MAX`].
    fn to_fix(self) -> Option<i64> {
        let x = i64::try_from(self).ok()?;
        if x > RUBY_FIXNUM_MAX {
            return None;
        }
        // no need to check the min bound since `u64::MIN` is zero.
        Some(x)
    }

    /// Test whether a [`u64`] value is in range of fixnum.
    ///
    /// This method returns `false` if the receiver is greater than
    /// [`RUBY_FIXNUM_MAX`].
    fn is_fixable(self) -> bool {
        (..=RUBY_FIXNUM_MAX_U64).contains(&self)
    }
}

impl Fixable for u128 {
    /// Convert a [`u128`] to a fixnum if it is less than or equal to
    /// [`RUBY_FIXNUM_MAX`] in magnitude.
    ///
    /// This method returns [`None`] if the receiver is greater than
    /// [`RUBY_FIXNUM_MAX`].
    fn to_fix(self) -> Option<i64> {
        let x = i64::try_from(self).ok()?;
        if x > RUBY_FIXNUM_MAX {
            return None;
        }
        // no need to check the min bound since `u128::MIN` is zero.
        Some(x)
    }

    /// Test whether a [`u128`] value is in range of fixnum.
    ///
    /// This method returns `false` if the receiver is greater than
    /// [`RUBY_FIXNUM_MAX`].
    fn is_fixable(self) -> bool {
        (..=RUBY_FIXNUM_MAX_U128).contains(&self)
    }
}

impl Fixable for f32 {
    /// Convert an [`f32`] to a fixnum if it is less than or equal to
    /// [`RUBY_FIXNUM_MAX`] and greater than or equal to [`RUBY_FIXNUM_MIN`] in
    /// magnitude.
    ///
    /// This method returns [`None`] if the receiver is greater than
    /// [`RUBY_FIXNUM_MAX`] or less than [`RUBY_FIXNUM_MIN`].
    ///
    /// This function discards the fractional part of the float, i.e. truncates.
    ///
    /// [NaN](f32::NAN) and infinities return [`None`].
    ///
    /// # Implementation Notes
    ///
    /// Conversion is implemented using checked operations and will never panic.
    ///
    /// This conversion is implemented using [`Duration::try_from_secs_f32`] and
    /// extracting the the integral portion of the float via [`Duration::as_secs`].
    fn to_fix(self) -> Option<i64> {
        if let Ok(d) = Duration::try_from_secs_f32(self) {
            let x = d.as_secs();
            return x.to_fix();
        }
        if let Ok(d) = Duration::try_from_secs_f32(-self) {
            let x = d.as_secs();
            let x = i64::try_from(x).ok()?;
            let x = x.checked_neg()?;
            return x.to_fix();
        }
        None
    }
}

impl Fixable for f64 {
    /// Convert an [`f64`] to a fixnum if it is less than or equal to
    /// [`RUBY_FIXNUM_MAX`] and greater than or equal to [`RUBY_FIXNUM_MIN`] in
    /// magnitude.
    ///
    /// This method returns [`None`] if the receiver is greater than
    /// [`RUBY_FIXNUM_MAX`] or less than [`RUBY_FIXNUM_MIN`].
    ///
    /// This function discards the fractional part of the float, i.e. truncates.
    ///
    /// [NaN](f64::NAN) and infinities return [`None`].
    ///
    /// # Implementation Notes
    ///
    /// Conversion is implemented using checked operations and will never panic.
    ///
    /// This conversion is implemented using [`Duration::try_from_secs_f32`] and
    /// extracting the the integral portion of the float via [`Duration::as_secs`].
    fn to_fix(self) -> Option<i64> {
        if let Ok(d) = Duration::try_from_secs_f64(self) {
            let x = d.as_secs();
            return x.to_fix();
        }
        if let Ok(d) = Duration::try_from_secs_f64(-self) {
            let x = d.as_secs();
            let x = i64::try_from(x).ok()?;
            let x = x.checked_neg()?;
            return x.to_fix();
        }
        None
    }
}

/// Check whether the given numeric is in the range of fixnum.
///
/// `RB_FIXABLE` can be applied to any numeric type. See the implementers of the
/// [`Fixable`] trait for more details on which numeric types are fixable.
///
/// To convert the given numeric value to a fixnum instead, see
/// [`Fixable::to_fix`].
///
/// # Examples
///
/// ```
/// use scolapasta_fixable::RB_FIXABLE;
///
/// assert!(RB_FIXABLE(23_u8));
/// assert!(RB_FIXABLE(u16::MIN));
/// assert!(RB_FIXABLE(i32::MAX));
/// assert!(RB_FIXABLE(1024_u64));
/// assert!(RB_FIXABLE(1024_i64));
/// assert!(RB_FIXABLE(1.0_f32));
/// assert!(RB_FIXABLE(-9000.27_f64));
/// ```
///
/// # C Declaration
///
/// ```c
/// /**
///  * Checks if the passed value is in  range of fixnum, assuming it is a positive
///  * number.  Can sometimes be useful for C's unsigned integer types.
///  *
///  * @internal
///  *
///  * FIXABLE can be applied to anything, from double to intmax_t.  The problem is
///  * double.   On a  64bit system  RUBY_FIXNUM_MAX is  4,611,686,018,427,387,903,
///  * which is not representable by a double.  The nearest value that a double can
///  * represent  is   4,611,686,018,427,387,904,  which   is  not   fixable.   The
///  * seemingly-strange "< FIXNUM_MAX + 1" expression below is due to this.
///  */
/// #define RB_POSFIXABLE(_) ((_) <  RUBY_FIXNUM_MAX + 1)
///
/// /**
///  * Checks if the passed value is in  range of fixnum, assuming it is a negative
///  * number.  This is an implementation of #RB_FIXABLE.  Rarely used stand alone.
///  */
/// #define RB_NEGFIXABLE(_) ((_) >= RUBY_FIXNUM_MIN)
///
/// /** Checks if the passed value is in  range of fixnum */
/// #define RB_FIXABLE(_)    (RB_POSFIXABLE(_) && RB_NEGFIXABLE(_))
/// ```
#[must_use]
#[allow(non_snake_case)] // match MRI macro name
pub fn RB_FIXABLE<T: Fixable>(x: T) -> bool {
    x.is_fixable()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn all_i8_are_fixable() {
        for x in i8::MIN..=i8::MAX {
            assert_eq!(x.to_fix(), Some(x.into()), "{x} should be its own fixnum");
            assert!(x.is_fixable(), "{x} should be fixable");
            assert!(RB_FIXABLE(x), "{x} should be fixable");
        }
    }

    #[test]
    fn all_i16_are_fixable() {
        for x in i16::MIN..=i16::MAX {
            assert_eq!(x.to_fix(), Some(x.into()), "{x} should be its own fixnum");
            assert!(x.is_fixable(), "{x} should be fixable");
            assert!(RB_FIXABLE(x), "{x} should be fixable");
        }
    }

    #[test]
    fn all_i32_are_fixable_shard_0() {
        const N: i32 = 0;
        const LOWER: i32 = i32::MIN + N * (1 << 29);
        const UPPER: i32 = LOWER + (1 << 29);

        let mut c = 0;
        for x in LOWER..UPPER {
            assert_eq!(x.to_fix(), Some(x.into()), "{x} should be its own fixnum");
            assert!(x.is_fixable(), "{x} should be fixable");
            assert!(RB_FIXABLE(x), "{x} should be fixable");
            c += 1;
        }
        assert_eq!(c, 1 << 29);
    }

    #[test]
    fn all_i32_are_fixable_shard_1() {
        const N: i32 = 1;
        const LOWER: i32 = i32::MIN + N * (1 << 29);
        const UPPER: i32 = LOWER + (1 << 29);

        let mut c = 0;
        for x in LOWER..UPPER {
            assert_eq!(x.to_fix(), Some(x.into()), "{x} should be its own fixnum");
            assert!(x.is_fixable(), "{x} should be fixable");
            assert!(RB_FIXABLE(x), "{x} should be fixable");
            c += 1;
        }
        assert_eq!(c, 1 << 29);
    }

    #[test]
    fn all_i32_are_fixable_shard_2() {
        const N: i32 = 2;
        const LOWER: i32 = i32::MIN + N * (1 << 29);
        const UPPER: i32 = LOWER + (1 << 29);

        let mut c = 0;
        for x in LOWER..UPPER {
            assert_eq!(x.to_fix(), Some(x.into()), "{x} should be its own fixnum");
            assert!(x.is_fixable(), "{x} should be fixable");
            assert!(RB_FIXABLE(x), "{x} should be fixable");
            c += 1;
        }
        assert_eq!(c, 1 << 29);
    }

    #[test]
    fn all_i32_are_fixable_shard_3() {
        const N: i32 = 3;
        const LOWER: i32 = i32::MIN + N * (1 << 29);
        const UPPER: i32 = LOWER + (1 << 29);

        let mut c = 0;
        for x in LOWER..UPPER {
            assert_eq!(x.to_fix(), Some(x.into()), "{x} should be its own fixnum");
            assert!(x.is_fixable(), "{x} should be fixable");
            assert!(RB_FIXABLE(x), "{x} should be fixable");
            c += 1;
        }
        assert_eq!(c, 1 << 29);
    }

    #[test]
    fn all_i32_are_fixable_shard_4() {
        const N: i32 = 0;
        const LOWER: i32 = N * (1 << 29);
        const UPPER: i32 = LOWER + (1 << 29);

        let mut c = 0;
        for x in LOWER..UPPER {
            assert_eq!(x.to_fix(), Some(x.into()), "{x} should be its own fixnum");
            assert!(x.is_fixable(), "{x} should be fixable");
            assert!(RB_FIXABLE(x), "{x} should be fixable");
            c += 1;
        }
        assert_eq!(c, 1 << 29);
    }

    #[test]
    fn all_i32_are_fixable_shard_5() {
        const N: i32 = 1;
        const LOWER: i32 = N * (1 << 29);
        const UPPER: i32 = LOWER + (1 << 29);

        let mut c = 0;
        for x in LOWER..UPPER {
            assert_eq!(x.to_fix(), Some(x.into()), "{x} should be its own fixnum");
            assert!(x.is_fixable(), "{x} should be fixable");
            assert!(RB_FIXABLE(x), "{x} should be fixable");
            c += 1;
        }
        assert_eq!(c, 1 << 29);
    }

    #[test]
    fn all_i32_are_fixable_shard_6() {
        const N: i32 = 2;
        const LOWER: i32 = N * (1 << 29);
        const UPPER: i32 = LOWER + (1 << 29);

        let mut c = 0;
        for x in LOWER..UPPER {
            assert_eq!(x.to_fix(), Some(x.into()), "{x} should be its own fixnum");
            assert!(x.is_fixable(), "{x} should be fixable");
            assert!(RB_FIXABLE(x), "{x} should be fixable");
            c += 1;
        }
        assert_eq!(c, 1 << 29);
    }

    #[test]
    fn all_i32_are_fixable_shard_7() {
        const N: i32 = 3;
        const LOWER: i32 = N * (1 << 29);

        let mut c = 0;
        for x in LOWER..=i32::MAX {
            assert_eq!(x.to_fix(), Some(x.into()), "{x} should be its own fixnum");
            assert!(x.is_fixable(), "{x} should be fixable");
            assert!(RB_FIXABLE(x), "{x} should be fixable");
            c += 1;
        }
        assert_eq!(c, 1 << 29);
    }

    #[test]
    fn i64_are_fixable() {
        let test_cases = [
            (i64::MIN, None),
            (RUBY_FIXNUM_MIN - 1, None),
            (RUBY_FIXNUM_MIN, Some(RUBY_FIXNUM_MIN)),
            (RUBY_FIXNUM_MIN + 1, Some(RUBY_FIXNUM_MIN + 1)),
            // ```
            // >>> (-(2 ** 63 - 1)) >> 1
            // -4611686018427387904
            // ``
            (-4_611_686_018_427_387_904 - 1, None),
            (-4_611_686_018_427_387_904, Some(-4_611_686_018_427_387_904)),
            (-4_611_686_018_427_387_904 + 1, Some(-4_611_686_018_427_387_903)),
            (-1024, Some(-1024)),
            (-10, Some(-10)),
            (-1, Some(-1)),
            (0_i64, Some(0)),
            (1, Some(1)),
            (10, Some(10)),
            (1024, Some(1024)),
            // ```
            // >>> (2 ** 63 - 1) >> 1
            // 4611686018427387903
            // ```
            (4_611_686_018_427_387_903 - 1, Some(4_611_686_018_427_387_902)),
            (4_611_686_018_427_387_903, Some(4_611_686_018_427_387_903)),
            (4_611_686_018_427_387_903 + 1, None),
            (RUBY_FIXNUM_MAX - 1, Some(RUBY_FIXNUM_MAX - 1)),
            (RUBY_FIXNUM_MAX, Some(RUBY_FIXNUM_MAX)),
            (RUBY_FIXNUM_MAX + 1, None),
            (i64::MAX, None),
        ];
        for (x, fixed) in test_cases {
            assert_eq!(x.to_fix(), fixed, "{x} did not fix correctly");
            assert_eq!(x.is_fixable(), fixed.is_some(), "{x} did not is_fixable correctly");
            assert_eq!(RB_FIXABLE(x), fixed.is_some(), "{x} did not RB_FIXABLE correctly");
        }
    }

    #[test]
    fn i128_are_fixable() {
        let test_cases = [
            (i128::MIN, None),
            (i64::MIN.into(), None),
            (i128::from(RUBY_FIXNUM_MIN) - 1, None),
            (i128::from(RUBY_FIXNUM_MIN), Some(RUBY_FIXNUM_MIN)),
            (i128::from(RUBY_FIXNUM_MIN) + 1, Some(RUBY_FIXNUM_MIN + 1)),
            // ```
            // >>> (-(2 ** 63 - 1)) >> 1
            // -4611686018427387904
            // ``
            (-4_611_686_018_427_387_904 - 1, None),
            (-4_611_686_018_427_387_904, Some(-4_611_686_018_427_387_904)),
            (-4_611_686_018_427_387_904 + 1, Some(-4_611_686_018_427_387_903)),
            (-1024, Some(-1024)),
            (-10, Some(-10)),
            (-1, Some(-1)),
            (0_i128, Some(0)),
            (1, Some(1)),
            (10, Some(10)),
            (1024, Some(1024)),
            // ```
            // >>> (2 ** 63 - 1) >> 1
            // 4611686018427387903
            // ```
            (4_611_686_018_427_387_903 - 1, Some(4_611_686_018_427_387_902)),
            (4_611_686_018_427_387_903, Some(4_611_686_018_427_387_903)),
            (4_611_686_018_427_387_903 + 1, None),
            (i128::from(RUBY_FIXNUM_MAX) - 1, Some(RUBY_FIXNUM_MAX - 1)),
            (i128::from(RUBY_FIXNUM_MAX), Some(RUBY_FIXNUM_MAX)),
            (i128::from(RUBY_FIXNUM_MAX) + 1, None),
            (i64::MAX.into(), None),
            (i128::MAX, None),
        ];
        for (x, fixed) in test_cases {
            assert_eq!(x.to_fix(), fixed, "{x} did not fix correctly");
            assert_eq!(x.is_fixable(), fixed.is_some(), "{x} did not is_fixable correctly");
            assert_eq!(RB_FIXABLE(x), fixed.is_some(), "{x} did not RB_FIXABLE correctly");
        }
    }

    #[test]
    fn all_u8_are_fixable() {
        for x in u8::MIN..=u8::MAX {
            assert_eq!(x.to_fix(), Some(x.into()), "{x} should be its own fixnum");
            assert!(x.is_fixable(), "{x} should be fixable");
            assert!(RB_FIXABLE(x), "{x} should be fixable");
        }
    }

    #[test]
    fn all_u16_are_fixable() {
        for x in u16::MIN..=u16::MAX {
            assert_eq!(x.to_fix(), Some(x.into()), "{x} should be its own fixnum");
            assert!(x.is_fixable(), "{x} should be fixable");
            assert!(RB_FIXABLE(x), "{x} should be fixable");
        }
    }

    #[test]
    fn all_u32_are_fixable_shard_0() {
        const N: u32 = 0;
        const LOWER: u32 = N * (1 << 29);
        const UPPER: u32 = LOWER + (1 << 29);

        let mut c = 0;
        for x in LOWER..UPPER {
            assert_eq!(x.to_fix(), Some(x.into()), "{x} should be its own fixnum");
            assert!(x.is_fixable(), "{x} should be fixable");
            assert!(RB_FIXABLE(x), "{x} should be fixable");
            c += 1;
        }
        assert_eq!(c, 1 << 29);
    }

    #[test]
    fn all_u32_are_fixable_shard_1() {
        const N: u32 = 1;
        const LOWER: u32 = N * (1 << 29);
        const UPPER: u32 = LOWER + (1 << 29);

        let mut c = 0;
        for x in LOWER..UPPER {
            assert_eq!(x.to_fix(), Some(x.into()), "{x} should be its own fixnum");
            assert!(x.is_fixable(), "{x} should be fixable");
            assert!(RB_FIXABLE(x), "{x} should be fixable");
            c += 1;
        }
        assert_eq!(c, 1 << 29);
    }

    #[test]
    fn all_u32_are_fixable_shard_2() {
        const N: u32 = 2;
        const LOWER: u32 = N * (1 << 29);
        const UPPER: u32 = LOWER + (1 << 29);

        let mut c = 0;
        for x in LOWER..UPPER {
            assert_eq!(x.to_fix(), Some(x.into()), "{x} should be its own fixnum");
            assert!(x.is_fixable(), "{x} should be fixable");
            assert!(RB_FIXABLE(x), "{x} should be fixable");
            c += 1;
        }
        assert_eq!(c, 1 << 29);
    }

    #[test]
    fn all_u32_are_fixable_shard_3() {
        const N: u32 = 3;
        const LOWER: u32 = N * (1 << 29);
        const UPPER: u32 = LOWER + (1 << 29);

        let mut c = 0;
        for x in LOWER..UPPER {
            assert_eq!(x.to_fix(), Some(x.into()), "{x} should be its own fixnum");
            assert!(x.is_fixable(), "{x} should be fixable");
            assert!(RB_FIXABLE(x), "{x} should be fixable");
            c += 1;
        }
        assert_eq!(c, 1 << 29);
    }

    #[test]
    fn all_u32_are_fixable_shard_4() {
        const N: u32 = 4;
        const LOWER: u32 = N * (1 << 29);
        const UPPER: u32 = LOWER + (1 << 29);

        let mut c = 0;
        for x in LOWER..UPPER {
            assert_eq!(x.to_fix(), Some(x.into()), "{x} should be its own fixnum");
            assert!(x.is_fixable(), "{x} should be fixable");
            assert!(RB_FIXABLE(x), "{x} should be fixable");
            c += 1;
        }
        assert_eq!(c, 1 << 29);
    }

    #[test]
    fn all_u32_are_fixable_shard_5() {
        const N: u32 = 5;
        const LOWER: u32 = N * (1 << 29);
        const UPPER: u32 = LOWER + (1 << 29);

        let mut c = 0;
        for x in LOWER..UPPER {
            assert_eq!(x.to_fix(), Some(x.into()), "{x} should be its own fixnum");
            assert!(x.is_fixable(), "{x} should be fixable");
            assert!(RB_FIXABLE(x), "{x} should be fixable");
            c += 1;
        }
        assert_eq!(c, 1 << 29);
    }

    #[test]
    fn all_u32_are_fixable_shard_6() {
        const N: u32 = 6;
        const LOWER: u32 = N * (1 << 29);
        const UPPER: u32 = LOWER + (1 << 29);

        let mut c = 0;
        for x in LOWER..UPPER {
            assert_eq!(x.to_fix(), Some(x.into()), "{x} should be its own fixnum");
            assert!(x.is_fixable(), "{x} should be fixable");
            assert!(RB_FIXABLE(x), "{x} should be fixable");
            c += 1;
        }
        assert_eq!(c, 1 << 29);
    }

    #[test]
    fn all_u32_are_fixable_shard_7() {
        const N: u32 = 7;
        const LOWER: u32 = N * (1 << 29);

        let mut c = 0;
        for x in LOWER..=u32::MAX {
            assert_eq!(x.to_fix(), Some(x.into()), "{x} should be its own fixnum");
            assert!(x.is_fixable(), "{x} should be fixable");
            assert!(RB_FIXABLE(x), "{x} should be fixable");
            c += 1;
        }
        assert_eq!(c, 1 << 29);
    }

    #[test]
    fn u64_are_fixable() {
        let test_cases = [
            (u64::MIN, Some(0)),
            (0_u64, Some(0)),
            (1, Some(1)),
            (10, Some(10)),
            (1024, Some(1024)),
            // ```
            // >>> (2 ** 63 - 1) >> 1
            // 4611686018427387903
            // ```
            (4_611_686_018_427_387_903 - 1, Some(4_611_686_018_427_387_902)),
            (4_611_686_018_427_387_903, Some(4_611_686_018_427_387_903)),
            (4_611_686_018_427_387_903 + 1, None),
            (u64::try_from(RUBY_FIXNUM_MAX).unwrap() - 1, Some(RUBY_FIXNUM_MAX - 1)),
            (u64::try_from(RUBY_FIXNUM_MAX).unwrap(), Some(RUBY_FIXNUM_MAX)),
            (u64::try_from(RUBY_FIXNUM_MAX).unwrap() + 1, None),
            (i64::MAX.try_into().unwrap(), None),
            (u64::MAX, None),
        ];
        for (x, fixed) in test_cases {
            assert_eq!(x.to_fix(), fixed, "{x} did not fix correctly");
            assert_eq!(x.is_fixable(), fixed.is_some(), "{x} did not is_fixable correctly");
            assert_eq!(RB_FIXABLE(x), fixed.is_some(), "{x} did not RB_FIXABLE correctly");
        }
    }

    #[test]
    fn u128_are_fixable() {
        let test_cases = [
            (u128::MIN, Some(0)),
            (0_u128, Some(0)),
            (1, Some(1)),
            (10, Some(10)),
            (1024, Some(1024)),
            // ```
            // >>> (2 ** 63 - 1) >> 1
            // 4611686018427387903
            // ```
            (4_611_686_018_427_387_903 - 1, Some(4_611_686_018_427_387_902)),
            (4_611_686_018_427_387_903, Some(4_611_686_018_427_387_903)),
            (4_611_686_018_427_387_903 + 1, None),
            (u128::try_from(RUBY_FIXNUM_MAX).unwrap() - 1, Some(RUBY_FIXNUM_MAX - 1)),
            (u128::try_from(RUBY_FIXNUM_MAX).unwrap(), Some(RUBY_FIXNUM_MAX)),
            (u128::try_from(RUBY_FIXNUM_MAX).unwrap() + 1, None),
            (i64::MAX.try_into().unwrap(), None),
            (u128::MAX, None),
        ];
        for (x, fixed) in test_cases {
            assert_eq!(x.to_fix(), fixed, "{x} did not fix correctly");
            assert_eq!(x.is_fixable(), fixed.is_some(), "{x} did not is_fixable correctly");
            assert_eq!(RB_FIXABLE(x), fixed.is_some(), "{x} did not RB_FIXABLE correctly");
        }
    }

    #[test]
    fn f32_are_fixable() {
        let test_cases = [
            (f32::NEG_INFINITY, None),
            (f32::MIN, None),
            (i64::MIN as _, None),
            (-4_612_686_018_427_388_000.0, None),
            (-1024.0, Some(-1024)),
            (-10.0, Some(-10)),
            (-1.9, Some(-1)),
            (-1.7, Some(-1)),
            (-1.5, Some(-1)),
            (-1.2, Some(-1)),
            (-1.1, Some(-1)),
            (-1.0, Some(-1)),
            (-0.0_f32, Some(0)),
            (0.0_f32, Some(0)),
            (1.0, Some(1)),
            (1.1, Some(1)),
            (1.2, Some(1)),
            (1.5, Some(1)),
            (1.7, Some(1)),
            (1.9, Some(1)),
            (10.0, Some(10)),
            (1024.0, Some(1024)),
            (4_611_686_018_427_387_904.0, None),
            (i64::MAX as _, None),
            (f32::MAX, None),
            (f32::INFINITY, None),
            (f32::MIN_POSITIVE, Some(0)),
            (f32::EPSILON, Some(0)),
            (f32::NAN, None),
        ];
        for (x, fixed) in test_cases {
            assert_eq!(x.to_fix(), fixed, "{x} did not fix correctly");
            assert_eq!(x.is_fixable(), fixed.is_some(), "{x} did not is_fixable correctly");
            assert_eq!(RB_FIXABLE(x), fixed.is_some(), "{x} did not RB_FIXABLE correctly");
        }
    }

    #[test]
    fn f64_are_fixable() {
        let test_cases = [
            (f64::NEG_INFINITY, None),
            (f64::MIN, None),
            (i64::MIN as _, None),
            (-4_612_686_018_427_388_000.0, None),
            (-1024.0, Some(-1024)),
            (-10.0, Some(-10)),
            (-1.9, Some(-1)),
            (-1.7, Some(-1)),
            (-1.5, Some(-1)),
            (-1.2, Some(-1)),
            (-1.1, Some(-1)),
            (-1.0, Some(-1)),
            (-0.0_f64, Some(0)),
            (0.0_f64, Some(0)),
            (1.0, Some(1)),
            (1.1, Some(1)),
            (1.2, Some(1)),
            (1.5, Some(1)),
            (1.7, Some(1)),
            (1.9, Some(1)),
            (10.0, Some(10)),
            (1024.0, Some(1024)),
            (4_611_686_018_427_387_904.0, None),
            (i64::MAX as _, None),
            (f64::MAX, None),
            (f64::INFINITY, None),
            (f64::MIN_POSITIVE, Some(0)),
            (f64::EPSILON, Some(0)),
            (f64::NAN, None),
        ];
        for (x, fixed) in test_cases {
            assert_eq!(x.to_fix(), fixed, "{x} did not fix correctly");
            assert_eq!(x.is_fixable(), fixed.is_some(), "{x} did not is_fixable correctly");
            assert_eq!(RB_FIXABLE(x), fixed.is_some(), "{x} did not RB_FIXABLE correctly");
        }
    }

    #[test]
    fn casts_in_const_context_are_safe() {
        assert_eq!(RUBY_FIXNUM_MAX_U64, u64::try_from(RUBY_FIXNUM_MAX).unwrap());
        assert_eq!(RUBY_FIXNUM_MAX_U128, u128::try_from(RUBY_FIXNUM_MAX).unwrap());
    }
}
