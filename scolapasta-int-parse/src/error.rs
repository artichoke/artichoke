use core::fmt::{self, Write as _};

use scolapasta_string_escape::format_debug_escape_into;

use crate::subject::IntegerString;

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
/// let result = scolapasta_int_parse::parse("0xBAD", Radix::new(10));
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
    /// # use scolapasta_int_parse::Radix;
    /// let result = scolapasta_int_parse::parse("0xBAD", Radix::new(10));
    /// let err = result.unwrap_err();
    /// assert_eq!(err.subject(), "0xBAD".as_bytes());
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
