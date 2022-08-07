use crate::error::ArgumentError;

#[derive(Default, Debug, Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct IntegerString<'a>(&'a [u8]);

impl<'a> TryFrom<&'a [u8]> for IntegerString<'a> {
    type Error = ArgumentError<'a>;

    fn try_from(subject: &'a [u8]) -> Result<Self, Self::Error> {
        if !subject.is_ascii() {
            return Err(subject.into());
        }
        if subject.contains(&b'\0') {
            return Err(subject.into());
        }
        Ok(Self(subject))
    }
}

impl<'a> TryFrom<&'a str> for IntegerString<'a> {
    type Error = ArgumentError<'a>;

    fn try_from(to_parse: &'a str) -> Result<Self, Self::Error> {
        to_parse.as_bytes().try_into()
    }
}

impl<'a> From<IntegerString<'a>> for &'a [u8] {
    #[inline]
    fn from(subject: IntegerString<'a>) -> &'a [u8] {
        subject.as_bytes()
    }
}

impl<'a> IntegerString<'a> {
    /// Constructs a new, empty `IntegerString`.
    #[inline]
    #[must_use]
    pub const fn new() -> Self {
        Self(b"")
    }

    /// Try to construct an `IntegerString` from the given byte slice.
    ///
    /// This conversion is fallible and requires the subject to be ASCII and
    /// have no NUL bytes.
    ///
    /// This conversion is also available as a [`TryFrom`] conversion which
    /// returns [`ArgumentError`] in the failure case.
    #[must_use]
    pub fn from_slice(subject: &'a [u8]) -> Option<Self> {
        subject.try_into().ok()
    }

    /// Retrieve the inner byte string from the `IntegerString`.
    #[inline]
    #[must_use]
    pub fn as_bytes(self) -> &'a [u8] {
        self.0
    }
}

#[cfg(test)]
mod tests {
    use super::IntegerString;

    #[test]
    fn nul_byte_is_err() {
        IntegerString::try_from("\0").unwrap_err();
        IntegerString::try_from("123\0").unwrap_err();
        IntegerString::try_from("123\x00456").unwrap_err();
    }

    #[test]
    fn emoji_is_err() {
        IntegerString::try_from("🕐").unwrap_err();
    }

    #[test]
    fn invalid_utf8_is_err() {
        IntegerString::try_from(&b"\xFF"[..]).unwrap_err();
    }
}
