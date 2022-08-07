use core::fmt::{self, Write as _};

use scolapasta_string_escape::format_debug_escape_into;

use crate::subject::IntegerString;

/// Sum type for all possible errors from this crate.
///
/// See [`ArgumentError`] and [`InvalidRadixError`] for more details.
#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub enum Error<'a> {
    /// An [`ArgumentError`].
    Argument(ArgumentError<'a>),
    /// An [`InvalidRadixError`].
    Radix(InvalidRadixError),
}

impl<'a> From<ArgumentError<'a>> for Error<'a> {
    fn from(err: ArgumentError<'a>) -> Self {
        Self::Argument(err)
    }
}

impl<'a> From<IntegerString<'a>> for Error<'a> {
    fn from(subject: IntegerString<'a>) -> Self {
        Self::Argument(subject.into())
    }
}

impl<'a> From<&'a [u8]> for Error<'a> {
    fn from(subject: &'a [u8]) -> Self {
        Self::Argument(subject.into())
    }
}

impl<'a> From<InvalidRadixError> for Error<'a> {
    fn from(err: InvalidRadixError) -> Self {
        Self::Radix(err)
    }
}

impl<'a> From<InvalidRadixErrorKind> for Error<'a> {
    fn from(err: InvalidRadixErrorKind) -> Self {
        Self::Radix(err.into())
    }
}

impl<'a> fmt::Display for Error<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Argument(err) => write!(f, "{}", err),
            Self::Radix(err) => write!(f, "{}", err),
        }
    }
}

#[cfg(feature = "std")]
impl<'a> std::error::Error for Error<'a> {}

/// Error that indicates the input to [`parse`] was invalid.
///
/// This error can be returned in the following circumstances:
///
/// - The input has non-ASCII bytes.
/// - The input contains a NUL byte.
/// - The input is the empty byte slice.
/// - The input only contains +/- signs.
/// - The given radix does not match a `0x`-style prefix.
/// - Invalid or duplicate +/- signs are in the input.
/// - Consecutive underscores are present in the input.
/// - Leading or trailing underscores are present in the input.
/// - The input contains ASCII alphanumeric bytes that are invalid for the
///   computed radix.
///
/// # Examples
///
/// ```
/// # use scolapasta_int_parse::Radix;
/// let result = scolapasta_int_parse::parse("0xBAD", Some(10));
/// let err = result.unwrap_err();
/// assert_eq!(err.to_string(), r#"invalid value for Integer(): "0xBAD""#);
/// ```
///
/// [`parse`]: crate::parse
/// [Ruby `ArgumentError` Exception class]: https://ruby-doc.org/core-3.1.2/ArgumentError.html
#[allow(clippy::module_name_repetitions)]
#[derive(Default, Debug, Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct ArgumentError<'a> {
    subject: &'a [u8],
}

impl<'a> ArgumentError<'a> {
    /// Return the subject of parsing that returned this argument error.
    ///
    /// # Examples
    ///
    /// ```
    /// # use scolapasta_int_parse::Error;
    /// let result = scolapasta_int_parse::parse("0xBAD", Some(10));
    /// let err = result.unwrap_err();
    /// assert!(matches!(err, Error::Argument(err) if err.subject() == "0xBAD".as_bytes()));
    /// ```
    #[must_use]
    pub const fn subject(self) -> &'a [u8] {
        self.subject
    }
}

impl<'a> From<IntegerString<'a>> for ArgumentError<'a> {
    fn from(subject: IntegerString<'a>) -> Self {
        let subject = subject.as_bytes();
        Self { subject }
    }
}

impl<'a> From<&'a [u8]> for ArgumentError<'a> {
    fn from(subject: &'a [u8]) -> Self {
        Self { subject }
    }
}

impl<'a> fmt::Display for ArgumentError<'a> {
    fn fmt(&self, mut f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(r#"invalid value for Integer(): ""#)?;
        // FIXME: this should actually be `String#inspect`, which is encoding
        // aware.
        format_debug_escape_into(&mut f, self.subject)?;
        f.write_char('"')?;
        Ok(())
    }
}

#[cfg(feature = "std")]
impl<'a> std::error::Error for ArgumentError<'a> {}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub enum InvalidRadixErrorKind {
    TooSmall(i64),
    TooBig(i64),
    Invalid(i64),
}

/// An enum describing which type of Ruby `Exception` and [`InvalidRadixError`]
/// should be mapped to.
///
/// If the given radix falls outside the range of an [`i32`], the error should
/// be mapped to a [`RangeError`].
///
/// If the given radix falls within the range of an [`i32`], but outside the
/// range of `2..=36`, the error should be mapped to an [`ArgumentError`].
///
/// The error message for these Ruby exceptions should be derived from the
/// [`fmt::Display`] implementation of [`InvalidRadixError`].
///
/// [`RangeError`]: https://ruby-doc.org/core-3.1.2/RangeError.html
/// [`ArgumentError`]: https://ruby-doc.org/core-3.1.2/ArgumentError.html
#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub enum InvalidRadixExceptionKind {
    /// If the given radix falls outside the range of an [`i32`], the error should
    /// be mapped to a [`RangeError`]:
    ///
    /// ```console
    /// [3.1.2] > begin; Integer "123", (2 ** 31 + 1); rescue => e; p e; end
    /// #<RangeError: integer 2147483649 too big to convert to `int'>
    /// [3.1.2] > begin; Integer "123", -(2 ** 31 + 1); rescue => e; p e; end
    /// #<RangeError: integer -2147483649 too small to convert to `int'>
    /// ```
    ArgumentError,
    /// If the given radix falls within the range of an [`i32`], but outside the
    /// range of `2..=36`, the error should be mapped to an [`ArgumentError`]:
    ///
    /// ```console
    /// [3.1.2] > begin; Integer "123", 49; rescue => e; p e; end
    /// #<ArgumentError: invalid radix 49>
    /// [3.1.2] > begin; Integer "123", -49; rescue => e; p e; end
    /// #<ArgumentError: invalid radix 49>
    /// ```
    RangeError,
}

/// Error that indicates the radix input to [`parse`] was invalid.
///
/// This error can be returned in the following circumstances:
///
/// - The input is out of range of [`i32`].
/// - The input radix is negative (if the input byte string does not have an
///   `0x`-style prefix) and out of range `-36..=-2`.
/// - The input is out of range of `2..=36`.
///
/// This error may map to several Ruby `Exception` types. See
/// [`InvalidRadixExceptionKind`] for more details.
///
/// # Examples
///
/// ```
/// # use scolapasta_int_parse::Radix;
/// let result = scolapasta_int_parse::parse("123", Some(500));
/// let err = result.unwrap_err();
/// assert_eq!(err.to_string(), "invalid radix 500");
/// ```
///
/// [`parse`]: crate::parse
#[allow(clippy::module_name_repetitions)]
#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct InvalidRadixError {
    kind: InvalidRadixErrorKind,
}

impl From<InvalidRadixErrorKind> for InvalidRadixError {
    fn from(kind: InvalidRadixErrorKind) -> Self {
        Self { kind }
    }
}

impl InvalidRadixError {
    /// Map an invalid radix error to the kind of Ruby Exception it should be
    /// raised as.
    ///
    /// See [`InvalidRadixExceptionKind`] for more details.
    #[must_use]
    pub fn exception_kind(&self) -> InvalidRadixExceptionKind {
        match self.kind {
            InvalidRadixErrorKind::Invalid(_) => InvalidRadixExceptionKind::ArgumentError,
            InvalidRadixErrorKind::TooSmall(_) | InvalidRadixErrorKind::TooBig(_) => {
                InvalidRadixExceptionKind::RangeError
            }
        }
    }
}

impl fmt::Display for InvalidRadixError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.kind {
            // ```
            // [3.1.2] > Integer "123", -((2 ** 31 + 1))
            // (irb):14:in `Integer': integer -2147483649 too small to convert to `int' (RangeError)
            //         from (irb):14:in `<main>'
            //         from /usr/local/var/rbenv/versions/3.1.2/lib/ruby/gems/3.1.0/gems/irb-1.4.1/exe/irb:11:in `<top (required)>'
            //         from /usr/local/var/rbenv/versions/3.1.2/bin/irb:25:in `load'
            //         from /usr/local/var/rbenv/versions/3.1.2/bin/irb:25:in `<main>'
            // ```
            InvalidRadixErrorKind::TooSmall(num) => write!(f, "integer {} too small to convert to `int'", num),
            // ```
            // [3.1.2] > Integer "123", (2 ** 31 + 1)
            // (irb):15:in `Integer': integer 2147483649 too big to convert to `int' (RangeError)
            //         from (irb):15:in `<main>'
            //         from /usr/local/var/rbenv/versions/3.1.2/lib/ruby/gems/3.1.0/gems/irb-1.4.1/exe/irb:11:in `<top (required)>'
            //         from /usr/local/var/rbenv/versions/3.1.2/bin/irb:25:in `load'
            //         from /usr/local/var/rbenv/versions/3.1.2/bin/irb:25:in `<main>'
            // ```
            InvalidRadixErrorKind::TooBig(num) => write!(f, "integer {} too big to convert to `int'", num),
            // ```
            // [3.1.2] > Integer "123", 1
            // (irb):17:in `Integer': invalid radix 1 (ArgumentError)
            //         from (irb):17:in `<main>'
            //         from /usr/local/var/rbenv/versions/3.1.2/lib/ruby/gems/3.1.0/gems/irb-1.4.1/exe/irb:11:in `<top (required)>'
            //         from /usr/local/var/rbenv/versions/3.1.2/bin/irb:25:in `load'
            //         from /usr/local/var/rbenv/versions/3.1.2/bin/irb:25:in `<main>'
            // [3.1.2] > Integer "123", 39
            // (irb):18:in `Integer': invalid radix 39 (ArgumentError)
            //         from (irb):18:in `<main>'
            //         from /usr/local/var/rbenv/versions/3.1.2/lib/ruby/gems/3.1.0/gems/irb-1.4.1/exe/irb:11:in `<top (required)>'
            //         from /usr/local/var/rbenv/versions/3.1.2/bin/irb:25:in `load'
            //         from /usr/local/var/rbenv/versions/3.1.2/bin/irb:25:in `<main>'
            // ```
            InvalidRadixErrorKind::Invalid(num) => write!(f, "invalid radix {}", num),
        }
    }
}

#[cfg(feature = "std")]
impl std::error::Error for InvalidRadixError {}

#[cfg(test)]
mod tests {
    use alloc::string::String;
    use core::fmt::Write as _;

    use super::ArgumentError;
    use crate::subject::IntegerString;

    #[test]
    fn argument_error_display_from_integer_string() {
        let test_cases = [
            ["0x", r#"invalid value for Integer(): "0x""#],
            ["0b", r#"invalid value for Integer(): "0b""#],
            ["0o", r#"invalid value for Integer(): "0o""#],
            ["o", r#"invalid value for Integer(): "o""#],
            ["0d", r#"invalid value for Integer(): "0d""#],
            ["0X", r#"invalid value for Integer(): "0X""#],
            ["0B", r#"invalid value for Integer(): "0B""#],
            ["0O", r#"invalid value for Integer(): "0O""#],
            ["O", r#"invalid value for Integer(): "O""#],
            ["0D", r#"invalid value for Integer(): "0D""#],
            ["-0x", r#"invalid value for Integer(): "-0x""#],
            ["-0b", r#"invalid value for Integer(): "-0b""#],
            ["-0o", r#"invalid value for Integer(): "-0o""#],
            ["-o", r#"invalid value for Integer(): "-o""#],
            ["-0d", r#"invalid value for Integer(): "-0d""#],
            ["-0X", r#"invalid value for Integer(): "-0X""#],
            ["-0B", r#"invalid value for Integer(): "-0B""#],
            ["-0O", r#"invalid value for Integer(): "-0O""#],
            ["-O", r#"invalid value for Integer(): "-O""#],
            ["-0D", r#"invalid value for Integer(): "-0D""#],
            ["0z", r#"invalid value for Integer(): "0z""#],
            ["-0z", r#"invalid value for Integer(): "-0z""#],
            ["B1", r#"invalid value for Integer(): "B1""#],
            ["b1", r#"invalid value for Integer(): "b1""#],
            ["O7", r#"invalid value for Integer(): "O7""#],
            ["o7", r#"invalid value for Integer(): "o7""#],
            ["D9", r#"invalid value for Integer(): "D9""#],
            ["d9", r#"invalid value for Integer(): "d9""#],
            ["XF", r#"invalid value for Integer(): "XF""#],
            ["Xf", r#"invalid value for Integer(): "Xf""#],
            ["xF", r#"invalid value for Integer(): "xF""#],
            ["xf", r#"invalid value for Integer(): "xf""#],
            ["0x_0000001234567", r#"invalid value for Integer(): "0x_0000001234567""#],
            ["0_x0000001234567", r#"invalid value for Integer(): "0_x0000001234567""#],
            [
                "___0x0000001234567",
                r#"invalid value for Integer(): "___0x0000001234567""#,
            ],
            ["0x111__11", r#"invalid value for Integer(): "0x111__11""#],
            ["0x111_11_", r#"invalid value for Integer(): "0x111_11_""#],
            ["0x00000_", r#"invalid value for Integer(): "0x00000_""#],
            ["    ", r#"invalid value for Integer(): "    ""#],
            ["", r#"invalid value for Integer(): """#],
            ["++12", r#"invalid value for Integer(): "++12""#],
            ["+-12", r#"invalid value for Integer(): "+-12""#],
            ["-+12", r#"invalid value for Integer(): "-+12""#],
            ["--12", r#"invalid value for Integer(): "--12""#],
        ];
        for [input, message] in test_cases {
            let subject = IntegerString::try_from(input).unwrap();
            let err = ArgumentError::from(subject);
            let mut buf = String::new();
            write!(&mut buf, "{}", err).unwrap();
            assert_eq!(&*buf, message, "unexpected value for test case '{input}'");
        }
    }

    #[test]
    fn argument_error_display_from_invalid_subject() {
        let test_cases: &[(&[u8], &str)] = &[
            (b"\xFF", r#"invalid value for Integer(): "\xFF""#),
            ("🦀".as_bytes(), r#"invalid value for Integer(): "🦀""#),
            // XXX: for UTF-8 strings, "\x00".inspect is "\u0000".
            (b"\x00", r#"invalid value for Integer(): "\x00""#),
        ];
        for (input, message) in test_cases.iter().copied() {
            let err = ArgumentError::from(input);
            let mut buf = String::new();
            write!(&mut buf, "{}", err).unwrap();
            assert_eq!(&*buf, message, "unexpected value for test case '{input:?}'");
        }
    }
}
