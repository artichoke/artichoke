use core::fmt::{self, Write as _};

/// Miscellaneous directives
///
/// Consists of various miscellaneous directives.
///
/// The corresponding characters for each directive are as follows:
///
/// | Directive | Returns | Meaning                                         |
/// |-----------|---------|-------------------------------------------------|
/// | `@`       | ---     | skip to the offset given by the length argument |
/// | `X`       | ---     | skip backward one byte                          |
/// | `x`       | ---     | skip forward one byte                           |
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Directive {
    /// Skip to offset (`@`)
    SkipToOffset,

    /// Skip backward (`X`)
    SkipBackward,

    /// Skip forward (`x`)
    SkipForward,
}

impl TryFrom<u8> for Directive {
    type Error = ();

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            b'@' => Ok(Self::SkipToOffset),
            b'X' => Ok(Self::SkipBackward),
            b'x' => Ok(Self::SkipForward),
            _ => Err(()),
        }
    }
}

impl fmt::Display for Directive {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let directive_char = match self {
            Self::SkipToOffset => '@',
            Self::SkipBackward => 'X',
            Self::SkipForward => 'x',
        };
        f.write_char(directive_char)
    }
}

#[cfg(test)]
mod tests {
    use core::fmt::Write;

    use super::*;

    #[test]
    fn test_directive_try_from_valid() {
        // Positive tests for valid directives
        assert_eq!(Directive::try_from(b'@'), Ok(Directive::SkipToOffset));
        assert_eq!(Directive::try_from(b'X'), Ok(Directive::SkipBackward));
        assert_eq!(Directive::try_from(b'x'), Ok(Directive::SkipForward));
    }

    #[test]
    fn test_directive_try_from_invalid() {
        // Negative tests for non-directive characters
        assert_eq!(Directive::try_from(b'A'), Err(()));
        assert_eq!(Directive::try_from(b'Y'), Err(()));
        assert_eq!(Directive::try_from(b'Z'), Err(()));

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
        let test_cases = [Directive::SkipToOffset, Directive::SkipBackward, Directive::SkipForward];

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
