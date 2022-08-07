use core::num::NonZeroU32;

use crate::error::{InvalidRadixError, InvalidRadixErrorKind};
use crate::subject::IntegerString;

// Create a lookup table from each byte value (of which the ASCII range is
// relevant) to the maximum minimum radix the character is valid for.
#[allow(clippy::cast_possible_truncation)]
const fn radix_table() -> [u32; 256] {
    // `u32::MAX` is used as a sentinel such that no valid radix can use this
    // byte.
    let mut table = [u32::MAX; 256];
    let mut idx = 0_usize;
    loop {
        if idx >= table.len() {
            return table;
        }
        let byte = idx as u8;
        if byte >= b'0' && byte <= b'9' {
            table[idx] = (byte - b'0' + 1) as u32;
        } else if byte >= b'A' && byte <= b'Z' {
            table[idx] = (byte - b'A' + 11) as u32;
        } else if byte >= b'a' && byte <= b'z' {
            table[idx] = (byte - b'a' + 11) as u32;
        }
        idx += 1;
    }
}

pub static RADIX_TABLE: [u32; 256] = radix_table();

/// A checked container for the radix to use when converting a string to an
/// integer.
///
/// This type enforces that its value is in the range 2 and 36 inclusive, which
/// is required by [`i64::from_str_radix`].
///
/// See [`parse`] for more details on how to use this type.
///
/// [`parse`]: crate::parse
#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct Radix(NonZeroU32);

impl Default for Radix {
    fn default() -> Self {
        // SAFETY: Constant `10` is non-zero and between 2 and 36.
        unsafe { Self::new_unchecked(10) }
    }
}

impl From<Radix> for u32 {
    fn from(radix: Radix) -> Self {
        radix.as_u32()
    }
}

impl Radix {
    /// Construct a new `Radix`.
    ///
    /// `radix` must be non-zero and between 2 and 36 inclusive; otherwise
    /// [`None`] is returned.
    ///
    /// # Examples
    ///
    /// ```
    /// # use scolapasta_int_parse::Radix;
    /// let radix = Radix::new(16);
    /// assert!(matches!(radix, Some(radix) if radix.as_u32() == 16));
    ///
    /// let invalid_radix = Radix::new(512);
    /// assert_eq!(invalid_radix, None);
    /// ```
    #[must_use]
    pub fn new(radix: u32) -> Option<Self> {
        let radix = NonZeroU32::new(radix)?;
        if (2..=36).contains(&radix.get()) {
            Some(Self(radix))
        } else {
            None
        }
    }

    /// Construct a new `Radix` without checking the value.
    ///
    /// # Safety
    ///
    /// The given radix must not be zero. The given radix must be between 2 and
    /// 36 inclusive.
    ///
    /// # Examples
    ///
    /// ```
    /// # use scolapasta_int_parse::Radix;
    /// let radix = unsafe { Radix::new_unchecked(16) };
    /// assert_eq!(radix.as_u32(), 16);
    /// ```
    #[must_use]
    pub const unsafe fn new_unchecked(radix: u32) -> Self {
        Self(NonZeroU32::new_unchecked(radix))
    }

    pub(crate) fn try_base_from_str_and_i64(
        subject: IntegerString<'_>,
        num: i64,
    ) -> Result<Option<u32>, InvalidRadixError> {
        match i32::try_from(num) {
            // ```
            // [3.1.2] > Integer "123", 0
            // => 123
            // [3.1.2] > Integer "0123", 0
            // => 83
            // [3.1.2] > Integer "0x123", 0
            // => 291
            // ```
            Ok(0) => Ok(None),
            // ```
            // [3.1.2] > Integer "123", 1
            // (irb):31:in `Integer': invalid radix 1 (ArgumentError)
            //         from (irb):31:in `<main>'
            //         from /usr/local/var/rbenv/versions/3.1.2/lib/ruby/gems/3.1.0/gems/irb-1.4.1/exe/irb:11:in `<top (required)>'
            //         from /usr/local/var/rbenv/versions/3.1.2/bin/irb:25:in `load'
            //         from /usr/local/var/rbenv/versions/3.1.2/bin/irb:25:in `<main>'
            // [3.1.2] > Integer "0x123", 1
            // (irb):32:in `Integer': invalid radix 1 (ArgumentError)
            //         from (irb):32:in `<main>'
            //         from /usr/local/var/rbenv/versions/3.1.2/lib/ruby/gems/3.1.0/gems/irb-1.4.1/exe/irb:11:in `<top (required)>'
            //         from /usr/local/var/rbenv/versions/3.1.2/bin/irb:25:in `load'
            //         from /usr/local/var/rbenv/versions/3.1.2/bin/irb:25:in `<main>'
            // ```
            Ok(1) => Err(InvalidRadixErrorKind::Invalid(num).into()),
            // Octal and base literals ignore negative radixes, but only if they
            // are in range of `i32`.
            //
            // ```
            // [3.1.2] > Integer "0123", -1
            // => 83
            // [3.1.2] > Integer "0123", -2000
            // => 83
            // [3.1.2] > Integer "0x123", -1
            // => 291
            // [3.1.2] > Integer "0x123", -(2 ** 31)
            // => 291
            // [3.1.2] > Integer "0d123", -(2 ** 39)
            // (irb):81:in `Integer': integer -549755813888 too small to convert to `int' (RangeError)
            //         from (irb):81:in `<main>'
            //         from /usr/local/var/rbenv/versions/3.1.2/lib/ruby/gems/3.1.0/gems/irb-1.4.1/exe/irb:11:in `<top (required)>'
            //         from /usr/local/var/rbenv/versions/3.1.2/bin/irb:25:in `load'
            //         from /usr/local/var/rbenv/versions/3.1.2/bin/irb:25:in `<main>'
            // ```
            Ok(_) if num < 0 && matches!(subject.as_bytes().first(), Some(&b'0')) => Ok(None),
            // ```
            // [3.1.2] > Integer "123", -(2 ** 31)
            // (irb):63:in `Integer': invalid radix -2147483648 (ArgumentError)
            //         from (irb):63:in `<main>'
            //         from /usr/local/var/rbenv/versions/3.1.2/lib/ruby/gems/3.1.0/gems/irb-1.4.1/exe/irb:11:in `<top (required)>'
            //         from /usr/local/var/rbenv/versions/3.1.2/bin/irb:25:in `load'
            //         from /usr/local/var/rbenv/versions/3.1.2/bin/irb:25:in `<main>'
            // ```
            Ok(i32::MIN) => Err(InvalidRadixErrorKind::Invalid(num).into()),
            Ok(radix) => {
                // ```
                // [3.1.2] > Integer "123", -(2**21)
                // (irb):46:in `Integer': invalid radix 2097152 (ArgumentError)
                //         from (irb):46:in `<main>'
                //         from /usr/local/var/rbenv/versions/3.1.2/lib/ruby/gems/3.1.0/gems/irb-1.4.1/exe/irb:11:in `<top (required)>'
                //         from /usr/local/var/rbenv/versions/3.1.2/bin/irb:25:in `load'
                //         from /usr/local/var/rbenv/versions/3.1.2/bin/irb:25:in `<main>'
                // ```
                let radix = match u32::try_from(radix) {
                    Ok(radix) => radix,
                    // ```
                    // [3.1.2] > Integer "111", -2
                    // => 7
                    // [3.1.2] > Integer "123", -36
                    // => 1371
                    // ```
                    Err(_) if (-36..=-2).contains(&radix) => (-radix).try_into().expect("radix is in range for u32"),
                    // ```
                    // [3.1.2] > Integer "123", -(2 ** 21)
                    // (irb):67:in `Integer': invalid radix 2097152 (ArgumentError)
                    //         from (irb):67:in `<main>'
                    //         from /usr/local/var/rbenv/versions/3.1.2/lib/ruby/gems/3.1.0/gems/irb-1.4.1/exe/irb:11:in `<top (required)>'
                    //         from /usr/local/var/rbenv/versions/3.1.2/bin/irb:25:in `load'
                    //         from /usr/local/var/rbenv/versions/3.1.2/bin/irb:25:in `<main>'
                    // [3.1.2] > 2 ** 21
                    // => 2097152
                    // ```
                    Err(_) => {
                        // Unchecked negation is safe because we checked for
                        // `i32::MAX` above.
                        let num = -num;
                        return Err(InvalidRadixErrorKind::Invalid(num).into());
                    }
                };
                if let Some(radix) = Radix::new(radix) {
                    Ok(Some(radix.as_u32()))
                } else {
                    // ```
                    // [3.1.2] > Integer "123", 49
                    // (irb):83:in `Integer': invalid radix 49 (ArgumentError)
                    //         from (irb):83:in `<main>'
                    //         from /usr/local/var/rbenv/versions/3.1.2/lib/ruby/gems/3.1.0/gems/irb-1.4.1/exe/irb:11:in `<top (required)>'
                    //         from /usr/local/var/rbenv/versions/3.1.2/bin/irb:25:in `load'
                    //         from /usr/local/var/rbenv/versions/3.1.2/bin/irb:25:in `<main>'
                    // ```
                    Err(InvalidRadixErrorKind::Invalid(num).into())
                }
            }
            // ```
            // [3.1.2] > Integer "123", (2 ** 32 + 1)
            // (irb):34:in `Integer': integer 4294967297 too big to convert to `int' (RangeError)
            //         from (irb):34:in `<main>'
            //         from /usr/local/var/rbenv/versions/3.1.2/lib/ruby/gems/3.1.0/gems/irb-1.4.1/exe/irb:11:in `<top (required)>'
            //         from /usr/local/var/rbenv/versions/3.1.2/bin/irb:25:in `load'
            //         from /usr/local/var/rbenv/versions/3.1.2/bin/irb:25:in `<main>'
            // ```
            Err(_) if num > i32::MAX.into() => Err(InvalidRadixErrorKind::TooBig(num).into()),
            // ```
            // [3.1.2] > Integer "123", -(2 ** 32 + 1)
            // (irb):33:in `Integer': integer -4294967297 too small to convert to `int' (RangeError)
            //         from (irb):33:in `<main>'
            //         from /usr/local/var/rbenv/versions/3.1.2/lib/ruby/gems/3.1.0/gems/irb-1.4.1/exe/irb:11:in `<top (required)>'
            //         from /usr/local/var/rbenv/versions/3.1.2/bin/irb:25:in `load'
            //         from /usr/local/var/rbenv/versions/3.1.2/bin/irb:25:in `<main>'
            // ```
            Err(_) if num < i32::MIN.into() => Err(InvalidRadixErrorKind::TooSmall(num).into()),
            Err(_) => unreachable!("all cases covered"),
        }
    }

    /// Extract the `Radix` as the underlying [`u32`].
    ///
    /// # Examples
    ///
    /// ```
    /// # use scolapasta_int_parse::Radix;
    /// # fn example() -> Option<()> {
    /// for base in 2..=36 {
    ///     let radix = Radix::new(base)?;
    ///     assert_eq!(radix.as_u32(), base);
    /// }
    /// # Some(())
    /// # }
    /// # example().unwrap();
    /// ```
    #[inline]
    #[must_use]
    pub const fn as_u32(self) -> u32 {
        self.0.get()
    }
}

#[cfg(test)]
mod tests {
    use alloc::string::String;
    use core::fmt::Write as _;

    use super::{Radix, RADIX_TABLE};
    use crate::error::InvalidRadixExceptionKind;

    #[test]
    fn default_is_radix_10() {
        let default = Radix::default();
        let base10 = Radix::new(10).unwrap();
        assert_eq!(default, base10);
    }

    #[test]
    fn radix_new_validates_radix_is_nonzero() {
        let radix = Radix::new(0);
        assert_eq!(radix, None);
    }

    #[test]
    fn radix_new_parses_valid_radixes() {
        for r in 2..=36 {
            let radix = Radix::new(r);
            let radix = radix.unwrap();
            assert_eq!(radix.as_u32(), r, "unexpected value for test case {r}");
        }
    }

    #[test]
    fn radix_new_rejects_too_large_radixes() {
        for r in 37..=525 {
            let radix = Radix::new(r);
            assert_eq!(radix, None, "unexpected value for test case {r}");
        }
    }

    #[test]
    fn radix_new_unchecked_valid_radixes() {
        for r in 2..=36 {
            let radix = unsafe { Radix::new_unchecked(r) };
            assert_eq!(radix.as_u32(), r, "unexpected value for test case {r}");
        }
    }

    #[test]
    fn radix_table_is_valid() {
        let test_cases = [
            (b'0', 1_u32),
            (b'1', 2),
            (b'2', 3),
            (b'3', 4),
            (b'4', 5),
            (b'5', 6),
            (b'6', 7),
            (b'7', 8),
            (b'8', 9),
            (b'9', 10),
            (b'A', 11),
            (b'B', 12),
            (b'C', 13),
            (b'D', 14),
            (b'E', 15),
            (b'F', 16),
            (b'G', 17),
            (b'H', 18),
            (b'I', 19),
            (b'J', 20),
            (b'K', 21),
            (b'L', 22),
            (b'M', 23),
            (b'N', 24),
            (b'O', 25),
            (b'P', 26),
            (b'Q', 27),
            (b'R', 28),
            (b'S', 29),
            (b'T', 30),
            (b'U', 31),
            (b'V', 32),
            (b'W', 33),
            (b'X', 34),
            (b'Y', 35),
            (b'Z', 36),
            (b'a', 11),
            (b'b', 12),
            (b'c', 13),
            (b'd', 14),
            (b'e', 15),
            (b'f', 16),
            (b'g', 17),
            (b'h', 18),
            (b'i', 19),
            (b'j', 20),
            (b'k', 21),
            (b'l', 22),
            (b'm', 23),
            (b'n', 24),
            (b'o', 25),
            (b'p', 26),
            (b'q', 27),
            (b'r', 28),
            (b's', 29),
            (b't', 30),
            (b'u', 31),
            (b'v', 32),
            (b'w', 33),
            (b'x', 34),
            (b'y', 35),
            (b'z', 36),
        ];
        for (byte, radix) in test_cases {
            assert_eq!(
                RADIX_TABLE[usize::from(byte)],
                radix,
                "unexpected value for test case ({byte}, {radix})"
            );
        }
    }

    #[test]
    fn non_ascii_alphanumeric_are_invalid() {
        for byte in 0..=u8::MAX {
            if byte.is_ascii_alphanumeric() {
                continue;
            }
            // no valid radix can use this byte
            assert_eq!(
                RADIX_TABLE[usize::from(byte)],
                u32::MAX,
                "unexpected value for test case '{byte}'"
            );
        }
    }

    #[test]
    fn from_base_zero() {
        let subject = "123".try_into().unwrap();
        let result = Radix::try_base_from_str_and_i64(subject, 0);
        assert_eq!(result.unwrap(), None);

        let subject = "0123".try_into().unwrap();
        let result = Radix::try_base_from_str_and_i64(subject, 0);
        assert_eq!(result.unwrap(), None);

        let subject = "0x123".try_into().unwrap();
        let result = Radix::try_base_from_str_and_i64(subject, 0);
        assert_eq!(result.unwrap(), None);
    }

    #[test]
    fn from_base_one_err() {
        let subject = "123".try_into().unwrap();
        let err = Radix::try_base_from_str_and_i64(subject, 1).unwrap_err();
        assert_eq!(err.exception_kind(), InvalidRadixExceptionKind::ArgumentError);

        let subject = "0123".try_into().unwrap();
        let err = Radix::try_base_from_str_and_i64(subject, 1).unwrap_err();
        assert_eq!(err.exception_kind(), InvalidRadixExceptionKind::ArgumentError);

        let subject = "0x123".try_into().unwrap();
        let err = Radix::try_base_from_str_and_i64(subject, 1).unwrap_err();
        assert_eq!(err.exception_kind(), InvalidRadixExceptionKind::ArgumentError);
    }

    #[test]
    fn from_base_i32_min_no_prefix_err() {
        let subject = "123".try_into().unwrap();
        let err = Radix::try_base_from_str_and_i64(subject, i32::MIN.into()).unwrap_err();
        assert_eq!(err.exception_kind(), InvalidRadixExceptionKind::ArgumentError);
    }

    #[test]
    fn from_base_i32_min_with_prefix_ignored() {
        let subject = "0123".try_into().unwrap();
        let result = Radix::try_base_from_str_and_i64(subject, i32::MIN.into());
        assert_eq!(result.unwrap(), None);

        let subject = "0x123".try_into().unwrap();
        let result = Radix::try_base_from_str_and_i64(subject, i32::MIN.into());
        assert_eq!(result.unwrap(), None);
    }

    #[test]
    fn from_base_negative_out_of_i32_range_min_with_prefix_err() {
        let subject = "0d123".try_into().unwrap();
        let err = Radix::try_base_from_str_and_i64(subject, -(2_i64.pow(39))).unwrap_err();

        let mut buf = String::new();
        write!(&mut buf, "{}", err).unwrap();
        assert_eq!(&*buf, "integer -549755813888 too small to convert to `int'");
        assert_eq!(err.exception_kind(), InvalidRadixExceptionKind::RangeError);
    }

    #[test]
    fn from_base_negative_out_of_range_err() {
        let subject = "123".try_into().unwrap();
        let err = Radix::try_base_from_str_and_i64(subject, -(2_i64.pow(21))).unwrap_err();

        let mut buf = String::new();
        write!(&mut buf, "{}", err).unwrap();
        assert_eq!(&*buf, "invalid radix 2097152");
        assert_eq!(err.exception_kind(), InvalidRadixExceptionKind::ArgumentError);

        let subject = "123".try_into().unwrap();
        let err = Radix::try_base_from_str_and_i64(subject, -(2_i64.pow(31))).unwrap_err();

        let mut buf = String::new();
        write!(&mut buf, "{}", err).unwrap();
        assert_eq!(&*buf, "invalid radix -2147483648");
        assert_eq!(err.exception_kind(), InvalidRadixExceptionKind::ArgumentError);
    }

    #[test]
    fn from_base_negative_abs_is_valid() {
        let subject = "111".try_into().unwrap();
        let result = Radix::try_base_from_str_and_i64(subject, -2);
        assert_eq!(result.unwrap(), Some(2));

        let subject = "111".try_into().unwrap();
        let result = Radix::try_base_from_str_and_i64(subject, -10);
        assert_eq!(result.unwrap(), Some(10));

        let subject = "111".try_into().unwrap();
        let result = Radix::try_base_from_str_and_i64(subject, -36);
        assert_eq!(result.unwrap(), Some(36));
    }

    #[test]
    fn from_base_negative_abs_is_invalid_err() {
        let subject = "111".try_into().unwrap();
        let err = Radix::try_base_from_str_and_i64(subject, -500).unwrap_err();

        let mut buf = String::new();
        write!(&mut buf, "{}", err).unwrap();
        assert_eq!(&*buf, "invalid radix 500");
        assert_eq!(err.exception_kind(), InvalidRadixExceptionKind::ArgumentError);

        let subject = "111".try_into().unwrap();
        let err = Radix::try_base_from_str_and_i64(subject, -49).unwrap_err();

        let mut buf = String::new();
        write!(&mut buf, "{}", err).unwrap();
        assert_eq!(&*buf, "invalid radix 49");
        assert_eq!(err.exception_kind(), InvalidRadixExceptionKind::ArgumentError);
    }

    #[test]
    fn from_base_positive_is_valid() {
        let subject = "111".try_into().unwrap();
        let result = Radix::try_base_from_str_and_i64(subject, 2);
        assert_eq!(result.unwrap(), Some(2));

        let subject = "111".try_into().unwrap();
        let result = Radix::try_base_from_str_and_i64(subject, 10);
        assert_eq!(result.unwrap(), Some(10));

        let subject = "111".try_into().unwrap();
        let result = Radix::try_base_from_str_and_i64(subject, 36);
        assert_eq!(result.unwrap(), Some(36));
    }

    #[test]
    fn from_base_positive_is_invalid_err() {
        let subject = "111".try_into().unwrap();
        let err = Radix::try_base_from_str_and_i64(subject, 500).unwrap_err();

        let mut buf = String::new();
        write!(&mut buf, "{}", err).unwrap();
        assert_eq!(&*buf, "invalid radix 500");
        assert_eq!(err.exception_kind(), InvalidRadixExceptionKind::ArgumentError);

        let subject = "111".try_into().unwrap();
        let err = Radix::try_base_from_str_and_i64(subject, 49).unwrap_err();

        let mut buf = String::new();
        write!(&mut buf, "{}", err).unwrap();
        assert_eq!(&*buf, "invalid radix 49");
        assert_eq!(err.exception_kind(), InvalidRadixExceptionKind::ArgumentError);
    }

    #[test]
    fn from_base_too_big_i32() {
        let subject = "111".try_into().unwrap();
        let err = Radix::try_base_from_str_and_i64(subject, i64::from(i32::MAX) + 1_i64).unwrap_err();

        let mut buf = String::new();
        write!(&mut buf, "{}", err).unwrap();
        assert_eq!(&*buf, "integer 2147483648 too big to convert to `int'");
        assert_eq!(err.exception_kind(), InvalidRadixExceptionKind::RangeError);

        let subject = "111".try_into().unwrap();
        let err = Radix::try_base_from_str_and_i64(subject, 2_i64.pow(32) + 1_i64).unwrap_err();

        let mut buf = String::new();
        write!(&mut buf, "{}", err).unwrap();
        assert_eq!(&*buf, "integer 4294967297 too big to convert to `int'");
        assert_eq!(err.exception_kind(), InvalidRadixExceptionKind::RangeError);
    }

    #[test]
    fn from_base_too_small_i32() {
        let subject = "111".try_into().unwrap();
        let err = Radix::try_base_from_str_and_i64(subject, i64::from(i32::MIN) - 1_i64).unwrap_err();

        let mut buf = String::new();
        write!(&mut buf, "{}", err).unwrap();
        assert_eq!(&*buf, "integer -2147483649 too small to convert to `int'");
        assert_eq!(err.exception_kind(), InvalidRadixExceptionKind::RangeError);

        let subject = "111".try_into().unwrap();
        let err = Radix::try_base_from_str_and_i64(subject, -(2_i64).pow(32) - 1_i64).unwrap_err();

        let mut buf = String::new();
        write!(&mut buf, "{}", err).unwrap();
        assert_eq!(&*buf, "integer -4294967297 too small to convert to `int'");
        assert_eq!(err.exception_kind(), InvalidRadixExceptionKind::RangeError);
    }
}
