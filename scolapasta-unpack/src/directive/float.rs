use core::fmt::{self, Write as _};

/// Float directives
///
/// Consists of various float directives.
///
/// The corresponding characters for each directive are as follows:
///
/// | Float Directive | Returns  | Meaning                                                           |
/// |-----------------|----------|-------------------------------------------------------------------|
/// | `D`, `d`        | Float    | double-precision, native format                                   |
/// | `F`, `f`        | Float    | single-precision, native format                                   |
/// | `E`             | Float    | double-precision, little-endian byte order                        |
/// | `e`             | Float    | single-precision, little-endian byte order                        |
/// | `G`             | Float    | double-precision, network (big-endian) byte order                 |
/// | `g`             | Float    | single-precision, network (big-endian) byte order                 |
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Directive {
    /// Double-precision, native format (`D`, `d`)
    DoubleNativeEndian,

    /// Single-precision, native format (`F`, `f`)
    SingleNativeEndian,

    /// Double-precision, little-endian byte order (`E`)
    DoubleLittleEndian,

    /// Single-precision, little-endian byte order (`e`)
    SingleLittleEndian,

    /// Double-precision, network (big-endian) byte order (`G`)
    DoubleBigEndian,

    /// Single-precision, network (big-endian) byte order (`g`)
    SingleBigEndian,
}

impl TryFrom<u8> for Directive {
    type Error = ();

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            b'D' | b'd' => Ok(Self::DoubleNativeEndian),
            b'F' | b'f' => Ok(Self::SingleNativeEndian),
            b'E' => Ok(Self::DoubleLittleEndian),
            b'e' => Ok(Self::SingleLittleEndian),
            b'G' => Ok(Self::DoubleBigEndian),
            b'g' => Ok(Self::SingleBigEndian),
            _ => Err(()),
        }
    }
}

impl fmt::Display for Directive {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let directive_char = match self {
            Self::DoubleNativeEndian => 'D',
            Self::SingleNativeEndian => 'F',
            Self::DoubleLittleEndian => 'E',
            Self::SingleLittleEndian => 'e',
            Self::DoubleBigEndian => 'G',
            Self::SingleBigEndian => 'g',
        };
        f.write_char(directive_char)
    }
}

#[cfg(test)]
mod tests {
    use std::fmt::Write;

    use super::*;

    #[test]
    fn test_directive_try_from_valid() {
        // Positive tests for valid directives
        assert_eq!(Directive::try_from(b'D'), Ok(Directive::DoubleNativeEndian));
        assert_eq!(Directive::try_from(b'd'), Ok(Directive::DoubleNativeEndian));
        assert_eq!(Directive::try_from(b'F'), Ok(Directive::SingleNativeEndian));
        assert_eq!(Directive::try_from(b'f'), Ok(Directive::SingleNativeEndian));
        assert_eq!(Directive::try_from(b'E'), Ok(Directive::DoubleLittleEndian));
        assert_eq!(Directive::try_from(b'e'), Ok(Directive::SingleLittleEndian));
        assert_eq!(Directive::try_from(b'G'), Ok(Directive::DoubleBigEndian));
        assert_eq!(Directive::try_from(b'g'), Ok(Directive::SingleBigEndian));
    }

    #[test]
    fn test_directive_try_from_invalid() {
        // Negative tests for non-directive characters
        assert_eq!(Directive::try_from(b'A'), Err(()));
        assert_eq!(Directive::try_from(b'Z'), Err(()));
        assert_eq!(Directive::try_from(b' '), Err(()));
        assert_eq!(Directive::try_from(b'!'), Err(()));

        // Negative tests for ASCII control characters
        assert_eq!(Directive::try_from(0x00), Err(())); // NUL
        assert_eq!(Directive::try_from(0x07), Err(())); // BEL
        assert_eq!(Directive::try_from(0x1B), Err(())); // ESC
        assert_eq!(Directive::try_from(0x7F), Err(())); // DEL

        // Negative tests for bytes outside the ASCII range
        assert_eq!(Directive::try_from(0x80), Err(()));
        assert_eq!(Directive::try_from(0xA5), Err(()));
        assert_eq!(Directive::try_from(0xFF), Err(()));
    }

    #[test]
    fn test_directive_display_not_empty() {
        let test_cases = [
            Directive::DoubleNativeEndian,
            Directive::SingleNativeEndian,
            Directive::DoubleLittleEndian,
            Directive::SingleLittleEndian,
            Directive::DoubleBigEndian,
            Directive::SingleBigEndian,
        ];

        for directive in test_cases {
            let mut buf = String::new();
            write!(&mut buf, "{directive}").unwrap();
            assert!(
                !buf.is_empty(),
                "Formatted string is empty for directive: {directive:?}",
            );
        }
    }
}
