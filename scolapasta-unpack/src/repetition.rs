use core::num::{IntErrorKind, NonZeroUsize};
use core::str;

use crate::RangeError;

/// Represents various  directives used in a format string.
///
/// The unpack amount must be the last modifier:
///
/// ```console
/// [3.2.2] > "1111111111111111".unpack('s*_')
/// <internal:pack>:20: warning: unknown unpack directive '_' in 's*_'
/// => [12593, 12593, 12593, 12593, 12593, 12593, 12593, 12593]
/// [3.2.2] > "1111111111111111".unpack('s_*')
/// => [12593, 12593, 12593, 12593, 12593, 12593, 12593, 12593]
/// ```
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
            [byte, ..] if !byte.is_ascii_digit() => return Ok(Self::default()),
            _ => {}
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

        match usize::from_str_radix(number, 10) {
            Ok(count) => match NonZeroUsize::new(count) {
                Some(count) => Ok(Self::Repeat(count)),
                None => Ok(Self::Finished),
            },
            // we had some numbers, but they must have all been zero and stripped above.
            Err(err) if matches!(err.kind(), IntErrorKind::Empty | IntErrorKind::Zero) => Ok(Self::Finished),
            Err(err) if *err.kind() == IntErrorKind::PosOverflow => {
                // ```
                // <internal:pack>:20:in `unpack': pack length too big (RangeError)
                // ```
                Err(RangeError::with_message("pack length too big"))
            }
            Err(err) if *err.kind() == IntErrorKind::InvalidDigit => {
                unreachable!("unexpected digits. Got {number}, expected 0-9 digits");
            }
            Err(err) if *err.kind() == IntErrorKind::NegOverflow => {
                unreachable!("parsing into usize does not permit negative values");
            }
            // handle non-exhaustive case for this enum.
            Err(_) => Ok(Self::Finished),
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
