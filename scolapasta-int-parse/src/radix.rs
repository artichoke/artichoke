use core::num::NonZeroU32;

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
    use super::{Radix, RADIX_TABLE};

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
}
