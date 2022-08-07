use alloc::string::String;

use crate::error::ArgumentError;
use crate::subject::IntegerString;

#[must_use]
#[derive(Default, Debug, Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub enum Sign {
    #[default]
    Positive,
    Negative,
}

#[must_use]
#[derive(Debug, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub enum State<'a> {
    Initial(IntegerString<'a>),
    Sign(IntegerString<'a>, Sign),
    Accumulate(IntegerString<'a>, String),
}

impl<'a> State<'a> {
    #[inline]
    pub fn new(subject: IntegerString<'a>) -> Self {
        Self::Initial(subject)
    }

    pub fn set_sign(self, sign: Sign) -> Result<Self, ArgumentError<'a>> {
        match self {
            Self::Sign(subject, _) | Self::Accumulate(subject, _) => Err(subject.into()),
            Self::Initial(subject) => Ok(Self::Sign(subject, sign)),
        }
    }

    pub fn collect_digit(self, digit: u8) -> Self {
        // For `i64::MIN` and `i64::MAX` in base 10:
        //
        // ```
        // [3.1.2] > 2 ** 64 - 1
        // => 18446744073709551615
        // [3.1.2] > (2 ** 64 - 1).to_s.length
        // => 20
        // [3.1.2] > -(2 ** 64)
        // => -18446744073709551616
        // [3.1.2] > (-(2 ** 64 - 1)).to_s.length
        // => 21
        // ```
        //
        // In bases below 10, the string repr for large numbers will be longer,
        // but pre-allocating for these uncommon cases seems wasteful. The
        // `String` will reallocate if it needs to in these pathological cases.
        const MAX_REPR_LENGTH: usize = 21;

        match self {
            Self::Initial(arg) => {
                let mut digits = String::with_capacity(MAX_REPR_LENGTH);
                digits.push(char::from(digit));
                Self::Accumulate(arg, digits)
            }
            Self::Sign(arg, sign) => {
                let mut digits = String::with_capacity(MAX_REPR_LENGTH);
                if let Sign::Negative = sign {
                    digits.push('-');
                }
                digits.push(char::from(digit));
                Self::Accumulate(arg, digits)
            }
            Self::Accumulate(arg, mut digits) => {
                digits.push(char::from(digit));
                Self::Accumulate(arg, digits)
            }
        }
    }

    pub fn into_numeric_string(self) -> Result<String, ArgumentError<'a>> {
        match self {
            Self::Accumulate(_, digits) => Ok(digits),
            Self::Initial(subject) | Self::Sign(subject, _) => Err(subject.into()),
        }
    }
}
