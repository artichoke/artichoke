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

//! Functions for converting numeric immediates to integer or "fixnum"
//! immediates.

#![no_std]

use core::time::Duration;

/// The maximum possible value that a fixnum can represent, 63 bits of an
/// [`i64`].
///
/// # C Declaration
/// ```c
/// /** Maximum possible value that a fixnum can represent. */
/// #define RUBY_FIXNUM_MAX  (LONG_MAX / 2)
/// ```
pub const RUBY_FIXNUM_MAX: i64 = i64::MAX / 2;

/// The minimum possible value that a fixnum can represent, 63 bits of an
/// [`i64`].
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
    fn to_fix(self) -> Option<i64>;

    /// Test whether a fixable numeric value is in range.
    fn is_fixable(self) -> bool {
        self.to_fix().is_some()
    }
}

impl Fixable for i8 {
    fn to_fix(self) -> Option<i64> {
        Some(self.into())
    }

    fn is_fixable(self) -> bool {
        true
    }
}

impl Fixable for i16 {
    fn to_fix(self) -> Option<i64> {
        Some(self.into())
    }

    fn is_fixable(self) -> bool {
        true
    }
}

impl Fixable for i32 {
    fn to_fix(self) -> Option<i64> {
        Some(self.into())
    }

    fn is_fixable(self) -> bool {
        true
    }
}

impl Fixable for i64 {
    fn to_fix(self) -> Option<i64> {
        if self > RUBY_FIXNUM_MAX {
            return None;
        }
        if self < RUBY_FIXNUM_MIN {
            return None;
        }
        Some(self)
    }
}

impl Fixable for i128 {
    fn to_fix(self) -> Option<i64> {
        let x = i64::try_from(self).ok()?;
        x.to_fix()
    }
}

impl Fixable for u8 {
    fn to_fix(self) -> Option<i64> {
        Some(self.into())
    }

    fn is_fixable(self) -> bool {
        true
    }
}

impl Fixable for u16 {
    fn to_fix(self) -> Option<i64> {
        Some(self.into())
    }

    fn is_fixable(self) -> bool {
        true
    }
}

impl Fixable for u32 {
    fn to_fix(self) -> Option<i64> {
        Some(self.into())
    }

    fn is_fixable(self) -> bool {
        true
    }
}

impl Fixable for u64 {
    fn to_fix(self) -> Option<i64> {
        let x = i64::try_from(self).ok()?;
        if x > RUBY_FIXNUM_MAX {
            return None;
        }
        // no need to check the min bound since `u64::MIN` is zero.
        Some(x)
    }
}

impl Fixable for u128 {
    fn to_fix(self) -> Option<i64> {
        let x = i64::try_from(self).ok()?;
        if x > RUBY_FIXNUM_MAX {
            return None;
        }
        // no need to check the min bound since `u128::MIN` is zero.
        Some(x)
    }
}

impl Fixable for f32 {
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
/// FIXABLE can be applied to any numeric type. See the implementersof the
/// [`Fixable`] trait for more details on which numeric types are fixable.
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
            assert!(x.is_fixable(), "{x} should be fixable");
            assert!(RB_FIXABLE(x), "{x} should be fixable");
        }
    }

    #[test]
    fn all_i16_are_fixable() {
        for x in i16::MIN..=i16::MAX {
            assert!(x.is_fixable(), "{x} should be fixable");
            assert!(RB_FIXABLE(x), "{x} should be fixable");
        }
    }

    #[test]
    fn all_neg_i32_are_fixable() {
        for x in i32::MIN..0 {
            assert!(x.is_fixable(), "{x} should be fixable");
            assert!(RB_FIXABLE(x), "{x} should be fixable");
        }
    }

    #[test]
    fn zero_i32_is_fixable() {
        assert!(0_i32.is_fixable(), "0 should be fixable");
        assert!(RB_FIXABLE(0_i32), "0 should be fixable");
    }

    #[test]
    fn all_pos_i32_are_fixable() {
        for x in 1..=i32::MAX {
            assert!(x.is_fixable(), "{x} should be fixable");
            assert!(RB_FIXABLE(x), "{x} should be fixable");
        }
    }

    #[test]
    fn all_u8_are_fixable() {
        for x in u8::MIN..=u8::MAX {
            assert!(x.is_fixable(), "{x} should be fixable");
            assert!(RB_FIXABLE(x), "{x} should be fixable");
        }
    }

    #[test]
    fn all_u16_are_fixable() {
        for x in u16::MIN..=u16::MAX {
            assert!(x.is_fixable(), "{x} should be fixable");
            assert!(RB_FIXABLE(x), "{x} should be fixable");
        }
    }

    #[test]
    fn all_neg_u32_are_fixable() {
        for x in u32::MIN..0 {
            assert!(x.is_fixable(), "{x} should be fixable");
            assert!(RB_FIXABLE(x), "{x} should be fixable");
        }
    }

    #[test]
    fn zero_u32_is_fixable() {
        assert!(0_u32.is_fixable(), "0 should be fixable");
        assert!(RB_FIXABLE(0_u32), "0 should be fixable");
    }

    #[test]
    fn all_pos_u32_are_fixable() {
        for x in 1..=u32::MAX {
            assert!(x.is_fixable(), "{x} should be fixable");
            assert!(RB_FIXABLE(x), "{x} should be fixable");
        }
    }

    #[test]
    fn all_i8_fix_to_self() {
        for x in i8::MIN..=i8::MAX {
            assert_eq!(x.to_fix(), Some(x.into()), "{x} should be its own fixnum");
        }
    }

    #[test]
    fn all_i16_fix_to_self() {
        for x in i16::MIN..=i16::MAX {
            assert_eq!(x.to_fix(), Some(x.into()), "{x} should be its own fixnum");
        }
    }

    #[test]
    fn all_neg_i32_fix_to_self() {
        for x in i32::MIN..0 {
            assert_eq!(x.to_fix(), Some(x.into()), "{x} should be its own fixnum");
        }
    }

    #[test]
    fn zero_i32_fixes_to_self() {
        assert_eq!(0_i32.to_fix(), Some(0), "0 should be its own fixnum");
    }

    #[test]
    fn all_pos_i32_fix_to_self() {
        for x in 1..=i32::MAX {
            assert_eq!(x.to_fix(), Some(x.into()), "{x} should be its own fixnum");
        }
    }

    #[test]
    fn i64_to_fix() {
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
    fn i128_to_fix() {
        let test_cases = [
            (i128::MIN, None),
            (i64::MIN.into(), None),
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
    fn all_u8_fix_to_self() {
        for x in u8::MIN..=u8::MAX {
            assert_eq!(x.to_fix(), Some(x.into()), "{x} should be its own fixnum");
        }
    }

    #[test]
    fn all_u16_fix_to_self() {
        for x in u16::MIN..=u16::MAX {
            assert_eq!(x.to_fix(), Some(x.into()), "{x} should be its own fixnum");
        }
    }

    #[test]
    fn all_neg_u32_fix_to_self() {
        for x in u32::MIN..0 {
            assert_eq!(x.to_fix(), Some(x.into()), "{x} should be its own fixnum");
        }
    }

    #[test]
    fn zero_u32_fixes_to_self() {
        assert_eq!(0_u32.to_fix(), Some(0), "0 should be its own fixnum");
    }

    #[test]
    fn all_pos_u32_fix_to_self() {
        for x in 1..=u32::MAX {
            assert_eq!(x.to_fix(), Some(x.into()), "{x} should be its own fixnum");
        }
    }

    #[test]
    fn u64_to_fix() {
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
    fn u128_to_fix() {
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
    fn f32_to_fix() {
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
    fn f64_to_fix() {
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
}
