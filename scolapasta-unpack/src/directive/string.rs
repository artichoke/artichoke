use core::fmt::{self, Write as _};

/// String directives
///
/// Consists of various string directives.
///
/// The corresponding characters for each directive are as follows:
///
/// | String Directive | Returns | Meaning                                                          |
/// |------------------|---------|------------------------------------------------------------------|
/// | `A`              | String  | arbitrary binary string (remove trailing nulls and ASCII spaces) |
/// | `a`              | String  | arbitrary binary string                                          |
/// | `Z`              | String  | null-terminated string                                           |
/// | `B`              | String  | bit string (MSB first)                                           |
/// | `b`              | String  | bit string (LSB first)                                           |
/// | `H`              | String  | hex string (high nibble first)                                   |
/// | `h`              | String  | hex string (low nibble first)                                    |
/// | `u`              | String  | UU-encoded string                                                |
/// | `M`              | String  | quoted-printable, MIME encoding (see RFC2045)                    |
/// | `m`              | String  | base64 encoded string (RFC 2045) (default)                       |
/// | `m0`             | String  | base64 encoded string (RFC 4648) if followed by 0                |
/// | `P`              | String  | pointer to a structure (fixed-length string)                     |
/// | `p`              | String  | pointer to a null-terminated string                              |
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Directive {
    /// Arbitrary binary string, remove trailing nulls and ASCII spaces (`A`)
    ArbitraryBinaryTrimmed,

    /// Arbitrary binary string (`a`)
    ArbitraryBinary,

    /// Null-terminated string (`Z`)
    NullTerminated,

    /// Bit string, MSB first (`B`)
    BitStringMsbFirst,

    /// Bit string, LSB first (`b`)
    BitStringLsbFirst,

    /// Hex string, high nibble first (`H`)
    HexStringHighNibbleFirst,

    /// Hex string, low nibble first (`h`)
    HexStringLowNibbleFirst,

    /// UU-encoded string (`U`)
    UuEncoded,

    /// Quoted-printable, MIME encoding - see RFC2045 (`M`)
    QuotedPrintable,

    /// Base64 encoded string - RFC 2045 default, RFC 4648 if followed by 0 (`m`)
    Base64Encoded,

    /// Pointer to a structure, fixed-length string (`P`)
    StructurePointer,

    /// Pointer to a null-terminated string (`p`)
    NullTerminatedStringPointer,
}

impl TryFrom<u8> for Directive {
    type Error = ();

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            b'A' => Ok(Self::ArbitraryBinaryTrimmed),
            b'a' => Ok(Self::ArbitraryBinary),
            b'Z' => Ok(Self::NullTerminated),
            b'B' => Ok(Self::BitStringMsbFirst),
            b'b' => Ok(Self::BitStringLsbFirst),
            b'H' => Ok(Self::HexStringHighNibbleFirst),
            b'h' => Ok(Self::HexStringLowNibbleFirst),
            b'u' => Ok(Self::UuEncoded),
            b'M' => Ok(Self::QuotedPrintable),
            b'm' => Ok(Self::Base64Encoded),
            b'P' => Ok(Self::StructurePointer),
            b'p' => Ok(Self::NullTerminatedStringPointer),
            _ => Err(()),
        }
    }
}

impl fmt::Display for Directive {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let directive_char = match self {
            Self::ArbitraryBinaryTrimmed => 'A',
            Self::ArbitraryBinary => 'a',
            Self::NullTerminated => 'Z',
            Self::BitStringMsbFirst => 'B',
            Self::BitStringLsbFirst => 'b',
            Self::HexStringHighNibbleFirst => 'H',
            Self::HexStringLowNibbleFirst => 'h',
            Self::UuEncoded => 'u',
            Self::QuotedPrintable => 'M',
            Self::Base64Encoded => 'm',
            Self::StructurePointer => 'P',
            Self::NullTerminatedStringPointer => 'p',
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
        assert_eq!(Directive::try_from(b'A'), Ok(Directive::ArbitraryBinaryTrimmed));
        assert_eq!(Directive::try_from(b'a'), Ok(Directive::ArbitraryBinary));
        assert_eq!(Directive::try_from(b'Z'), Ok(Directive::NullTerminated));
        assert_eq!(Directive::try_from(b'B'), Ok(Directive::BitStringMsbFirst));
        assert_eq!(Directive::try_from(b'b'), Ok(Directive::BitStringLsbFirst));
        assert_eq!(Directive::try_from(b'H'), Ok(Directive::HexStringHighNibbleFirst));
        assert_eq!(Directive::try_from(b'h'), Ok(Directive::HexStringLowNibbleFirst));
        assert_eq!(Directive::try_from(b'u'), Ok(Directive::UuEncoded));
        assert_eq!(Directive::try_from(b'M'), Ok(Directive::QuotedPrintable));
        assert_eq!(Directive::try_from(b'm'), Ok(Directive::Base64Encoded));
        assert_eq!(Directive::try_from(b'P'), Ok(Directive::StructurePointer));
        assert_eq!(Directive::try_from(b'p'), Ok(Directive::NullTerminatedStringPointer));
    }

    #[test]
    fn test_directive_try_from_invalid() {
        // Negative tests for non-directive characters
        assert_eq!(Directive::try_from(b'C'), Err(()));
        assert_eq!(Directive::try_from(b'Y'), Err(()));
        assert_eq!(Directive::try_from(b'Q'), Err(()));

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
            Directive::ArbitraryBinaryTrimmed,
            Directive::ArbitraryBinary,
            Directive::NullTerminated,
            Directive::BitStringMsbFirst,
            Directive::BitStringLsbFirst,
            Directive::HexStringHighNibbleFirst,
            Directive::HexStringLowNibbleFirst,
            Directive::UuEncoded,
            Directive::QuotedPrintable,
            Directive::Base64Encoded,
            Directive::StructurePointer,
            Directive::NullTerminatedStringPointer,
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
