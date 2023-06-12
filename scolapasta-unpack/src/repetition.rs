use core::num::{IntErrorKind, NonZeroUsize};
use core::str;

use crate::RangeError;

// NOTE: The unpack amount must be the last modifier:
//
// ```console
// [3.2.2] > "1111111111111111".unpack('s*_')
// <internal:pack>:20: warning: unknown unpack directive '_' in 's*_'
// => [12593, 12593, 12593, 12593, 12593, 12593, 12593, 12593]
// [3.2.2] > "1111111111111111".unpack('s_*')
// => [12593, 12593, 12593, 12593, 12593, 12593, 12593, 12593]
// ```

/// Represents various repetition directives used in a format string.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Repetition {
    /// Specifies the number of times to repeat the preceding directive.
    Repeat(NonZeroUsize),

    /// Consume to end.
    ///
    /// An asterisk (`*`) will use up all remaining elements.
    ConsumeToEnd,

    /// The amount to unpack is consumed.
    Finished,
}

impl Default for Repetition {
    fn default() -> Self {
        Self::Repeat(NonZeroUsize::MIN)
    }
}

impl Repetition {
    pub fn next_from_format_bytes(format: &mut &[u8]) -> Result<Self, RangeError> {
        match *format {
            [] => return Ok(Self::default()),
            [b'*', tail @ ..] => {
                *format = tail;
                return Ok(Self::ConsumeToEnd);
            }
            [byte, ..] if byte.is_ascii_digit() => {}
            _ => return Ok(Self::default()),
        }
        // Trim leading zeros:
        //
        // ```console
        // [3.2.2] > "11111111111111111111111111111111".unpack('h010')
        // => ["1313131313"]
        // [3.2.2] > "11111111111111111111111111111111".unpack('h010').first.length
        // => 10
        // ```
        while let Some((&b'0', tail)) = format.split_first() {
            *format = tail;
        }

        // Find where the ASCII digits end.
        let end_pos = format
            .iter()
            .position(|b| !b.is_ascii_digit())
            .unwrap_or_else(|| format.len());

        let (number, tail) = format.split_at(end_pos);
        *format = tail;
        let number = str::from_utf8(number).expect("slice is only ASCII digits");

        let count = match usize::from_str_radix(number, 10) {
            Ok(count) => count,
            // we had some numbers, but they must have all been zero and stripped above.
            Err(err) if matches!(err.kind(), IntErrorKind::Empty | IntErrorKind::Zero) => return Ok(Self::Finished),
            Err(err) if *err.kind() == IntErrorKind::PosOverflow => {
                // ```
                // <internal:pack>:20:in `unpack': pack length too big (RangeError)
                // ```
                return Err(RangeError::with_message("pack length too big"));
            }
            Err(err) if *err.kind() == IntErrorKind::InvalidDigit => {
                unreachable!("unexpected digits. Got {number}, expected 0-9 digits");
            }
            Err(err) if *err.kind() == IntErrorKind::NegOverflow => {
                unreachable!("parsing into usize does not permit negative values");
            }
            // handle non-exhaustive case for this enum.
            Err(_) => return Ok(Self::Finished),
        };
        match NonZeroUsize::new(count) {
            Some(count) => Ok(Self::Repeat(count)),
            None => Ok(Self::Finished),
        }
    }

    pub fn next(self) -> Option<Self> {
        let count = match self {
            Self::Repeat(count) => count.get(),
            // No updates.
            Self::ConsumeToEnd => return Some(self),
            // Once the repetition is finished, it is fully consumed.
            Self::Finished => return None,
        };
        let count = match count.checked_sub(1) {
            None | Some(0) => return None,
            Some(count) => count,
        };
        let count = NonZeroUsize::new(count).expect("count is nonzero");
        Some(Self::Repeat(count))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default() {
        let default_repetition = Repetition::default();
        let expected_repetition = Repetition::Repeat(NonZeroUsize::new(1).unwrap());
        assert_eq!(default_repetition, expected_repetition);
    }

    #[test]
    fn test_next_from_format_bytes_empty() {
        // Empty format should yield the default repetition.
        let mut format = &b""[..];
        assert_eq!(
            Repetition::next_from_format_bytes(&mut format),
            Ok(Repetition::default())
        );
        assert_eq!(format, &b""[..]);
    }

    #[test]
    fn test_next_from_format_bytes_consume_to_end() {
        // Format with '*' should yield ConsumeToEnd.
        let mut format = &b"*"[..];
        assert_eq!(
            Repetition::next_from_format_bytes(&mut format),
            Ok(Repetition::ConsumeToEnd)
        );
        assert_eq!(format, &b""[..]);
    }

    #[test]
    fn test_next_from_format_bytes_zero() {
        // Format with zero should yield Finished repetition.
        let mut format = &b"0"[..];
        assert_eq!(
            Repetition::next_from_format_bytes(&mut format),
            Ok(Repetition::Finished)
        );
        assert_eq!(format, &b""[..]);
    }

    #[test]
    fn test_next_from_format_bytes_zeroes() {
        // Format with only zeroes should yield Finished repetition and preserve
        // the remaining format.
        let mut format = &b"0000000Z"[..];
        assert_eq!(
            Repetition::next_from_format_bytes(&mut format),
            Ok(Repetition::Finished)
        );
        assert_eq!(format, &b"Z"[..]);
    }

    #[test]
    fn test_next_from_format_bytes_leading_zeros() {
        // Format with leading zeros should ignore them.
        let mut format = &b"00012345"[..];
        assert_eq!(
            Repetition::next_from_format_bytes(&mut format),
            Ok(Repetition::Repeat(NonZeroUsize::new(12345).unwrap()))
        );
        assert_eq!(format, &b""[..]);
    }

    #[test]
    fn test_next_from_format_bytes_non_digit() {
        // Format with non-digit character should yield the default repetition.
        let mut format = &b"abc"[..];
        assert_eq!(
            Repetition::next_from_format_bytes(&mut format),
            Ok(Repetition::default())
        );
        assert_eq!(format, &b"abc"[..]);
    }

    #[test]
    fn test_next_from_format_bytes_remaining_specifiers() {
        // Format leaves next specifiers.
        let mut format = &b"12345Z"[..];
        assert_eq!(
            Repetition::next_from_format_bytes(&mut format),
            Ok(Repetition::Repeat(NonZeroUsize::new(12345).unwrap()))
        );
        assert_eq!(format, &b"Z"[..]);
    }

    #[test]
    fn test_next_from_format_bytes_large_number() {
        // Format with large number should yield RangeError.
        let mut format = &b"123456789012345678901234567890"[..];
        assert_eq!(
            Repetition::next_from_format_bytes(&mut format),
            Err(RangeError::with_message("pack length too big"))
        );
        assert_eq!(format, &b""[..]);
    }

    #[test]
    fn test_next_repeat() {
        // Repeat variant with count of 3 should return 2 more repetitions.
        let repetition = Repetition::Repeat(NonZeroUsize::new(3).unwrap());
        let repetition = repetition.next();
        assert_eq!(repetition, Some(Repetition::Repeat(NonZeroUsize::new(2).unwrap())));
        let repetition = repetition.unwrap().next();
        assert_eq!(repetition, Some(Repetition::Repeat(NonZeroUsize::new(1).unwrap())));
        let repetition = repetition.unwrap().next();
        assert_eq!(repetition, None);
    }

    #[test]
    fn test_next_consume_to_end() {
        // ConsumeToEnd variant should always return itself.
        let repetition = Repetition::ConsumeToEnd;
        let repetition = repetition.next();
        assert_eq!(repetition, Some(Repetition::ConsumeToEnd));
        let repetition = repetition.unwrap().next();
        assert_eq!(repetition, Some(Repetition::ConsumeToEnd));
        let repetition = repetition.unwrap().next();
        assert_eq!(repetition, Some(Repetition::ConsumeToEnd));
    }

    #[test]
    fn test_next_finished() {
        // Finished variant should always return None.
        let repetition = Repetition::Finished;
        assert_eq!(repetition.next(), None);
    }
}
