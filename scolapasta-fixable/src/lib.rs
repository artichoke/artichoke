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

    impl Sealed for u8 {}
    impl Sealed for u16 {}
    impl Sealed for u32 {}
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
            assert!(RB_FIXABLE(x), "{x} should be fixable");
        }
    }

    #[test]
    fn all_i16_are_fixable() {
        for x in i16::MIN..=i16::MAX {
            assert!(RB_FIXABLE(x), "{x} should be fixable");
        }
    }

    #[test]
    fn all_neg_i32_are_fixable() {
        for x in i32::MIN..0 {
            assert!(RB_FIXABLE(x), "{x} should be fixable");
        }
    }

    #[test]
    fn zero_i32_is_fixable() {
        assert!(RB_FIXABLE(0_i32), "0 should be fixable");
    }

    #[test]
    fn all_pos_i32_are_fixable() {
        for x in 1..=i32::MAX {
            assert!(RB_FIXABLE(x), "{x} should be fixable");
        }
    }

    #[test]
    fn all_u8_are_fixable() {
        for x in u8::MIN..=u8::MAX {
            assert!(RB_FIXABLE(x), "{x} should be fixable");
        }
    }

    #[test]
    fn all_u16_are_fixable() {
        for x in u16::MIN..=u16::MAX {
            assert!(RB_FIXABLE(x), "{x} should be fixable");
        }
    }

    #[test]
    fn all_neg_u32_are_fixable() {
        for x in u32::MIN..0 {
            assert!(RB_FIXABLE(x), "{x} should be fixable");
        }
    }

    #[test]
    fn zero_u32_is_fixable() {
        assert!(RB_FIXABLE(0_u32), "0 should be fixable");
    }

    #[test]
    fn all_pos_u32_are_fixable() {
        for x in 1..=u32::MAX {
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
}
