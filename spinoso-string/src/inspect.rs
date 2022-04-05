use core::fmt;
use core::iter::FusedIterator;

use crate::enc;

/// An iterator that yields a debug representation of a `String` and its byte
/// contents as a sequence of `char`s.
///
/// This struct is created by the [`inspect`] method on [`String`]. See its
/// documentation for more.
///
/// To format a `String` directly into a writer, see [`format_into`] or
/// [`write_into`].
///
/// # Examples
///
/// To inspect an empty byte string:
///
/// ```
/// # extern crate alloc;
/// use alloc::string::String;
/// # use spinoso_string::Inspect;
/// let inspect = Inspect::default();
/// let debug = inspect.collect::<String>();
/// assert_eq!(debug, r#""""#);
/// ```
///
/// To inspect a well-formed UTF-8 byte string:
///
/// ```
/// # extern crate alloc;
/// use alloc::string::String;
/// # use spinoso_string::Inspect;
/// let s = spinoso_string::String::from("spinoso");
/// let inspect = s.inspect();
/// let debug = inspect.collect::<String>();
/// assert_eq!(debug, "\"spinoso\"");
/// ```
///
/// To inspect a byte string with invalid UTF-8 bytes:
///
/// ```
/// # extern crate alloc;
/// use alloc::string::String;
/// # use spinoso_string::Inspect;
/// let s = spinoso_string::String::utf8(b"invalid-\xFF-utf8".to_vec());
/// let inspect = s.inspect();
/// let debug = inspect.collect::<String>();
/// assert_eq!(debug, r#""invalid-\xFF-utf8""#);
/// ```
///
/// To inspect a binary string:
///
/// ```
/// # extern crate alloc;
/// use alloc::string::String;
/// # use spinoso_string::Inspect;
/// let s = spinoso_string::String::binary("💎".as_bytes().to_vec());
/// let inspect = s.inspect();
/// let debug = inspect.collect::<String>();
/// assert_eq!(debug, r#""\xF0\x9F\x92\x8E""#);
/// ```
///
/// [`inspect`]: crate::String::inspect
/// [`String`]: crate::String
/// [`format_into`]: Self::format_into
/// [`write_into`]: Self::write_into
#[derive(Default, Debug, Clone)]
#[must_use = "this `Inspect` is an `Iterator`, which should be consumed if constructed"]
pub struct Inspect<'a>(enc::Inspect<'a>);

impl<'a> Iterator for Inspect<'a> {
    type Item = char;

    fn next(&mut self) -> Option<Self::Item> {
        self.0.next()
    }
}

impl<'a> FusedIterator for Inspect<'a> {}

impl<'a> Inspect<'a> {
    pub(crate) fn new(value: enc::Inspect<'a>) -> Self {
        Self(value)
    }

    /// Write an `Inspect` iterator into the given destination using the debug
    /// representation of the byte buffer associated with a source `String`.
    ///
    /// This formatter writes content like `"spinoso"` and `"invalid-\xFF-utf8"`.
    /// To see example output of the underlying iterator, see the `Inspect`
    /// documentation.
    ///
    /// To write binary output, use [`write_into`], which requires the **std**
    /// feature to be activated.
    ///
    /// # Errors
    ///
    /// If the given writer returns an error as it is being written to, that
    /// error is returned.
    ///
    /// # Examples
    ///
    /// ```
    /// # extern crate alloc;
    /// # use core::fmt::Write;
    /// use alloc::string::String;
    /// # use spinoso_string::Inspect;
    /// let mut buf = String::new();
    /// let s = spinoso_string::String::from("spinoso");
    /// let iter = s.inspect();
    /// iter.format_into(&mut buf);
    /// assert_eq!(buf, "\"spinoso\"");
    ///
    /// let mut buf = String::new();
    /// let s = spinoso_string::String::utf8(b"\xFF".to_vec());
    /// let iter = s.inspect();
    /// iter.format_into(&mut buf);
    /// assert_eq!(buf, r#""\xFF""#);
    /// ```
    ///
    /// [`write_into`]: Self::write_into
    #[inline]
    pub fn format_into<W>(self, mut dest: W) -> fmt::Result
    where
        W: fmt::Write,
    {
        for ch in self {
            dest.write_char(ch)?;
        }
        Ok(())
    }

    /// Write an `Inspect` iterator into the given destination using the debug
    /// representation of the byte buffer associated with a source `String`.
    ///
    /// This formatter writes content like `"spinoso"` and `"invalid-\xFF-utf8"`.
    /// To see example output of the underlying iterator, see the `Inspect`
    /// documentation.
    ///
    /// To write to a [formatter], use [`format_into`].
    ///
    /// # Errors
    ///
    /// If the given writer returns an error as it is being written to, that
    /// error is returned.
    ///
    /// # Examples
    ///
    /// ```
    /// # use std::io::Write;
    /// # use spinoso_string::{Inspect, String};
    /// let mut buf = Vec::new();
    /// let s = String::from("spinoso");
    /// let iter = s.inspect();
    /// iter.write_into(&mut buf);
    /// assert_eq!(buf, &b"\"spinoso\""[..]);
    ///
    /// let mut buf = Vec::new();
    /// let s = String::utf8(b"\xFF".to_vec());
    /// let iter = s.inspect();
    /// iter.write_into(&mut buf);
    /// assert_eq!(buf, &[b'"', b'\\', b'x', b'F', b'F', b'"']);
    /// ```
    ///
    /// [formatter]: fmt::Write
    /// [`format_into`]: Self::format_into
    #[inline]
    #[cfg(feature = "std")]
    #[cfg_attr(docsrs, doc(cfg(feature = "std")))]
    pub fn write_into<W>(self, mut dest: W) -> std::io::Result<()>
    where
        W: std::io::Write,
    {
        let mut buf = [0; 4];
        for ch in self {
            let utf8 = ch.encode_utf8(&mut buf);
            dest.write_all(utf8.as_bytes())?;
        }
        Ok(())
    }
}

/// Helper iterator-ish struct for tracking when to emit wrapping quotes for
/// inspect iterators.
#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct Flags {
    bits: u8,
}

impl Flags {
    // Bit flags
    const EMIT_LEADING_QUOTE: Self = Self { bits: 0b0000_0001 };
    const EMIT_TRAILING_QUOTE: Self = Self { bits: 0b0000_0010 };

    // Initial states
    pub const DEFAULT: Self = Self {
        bits: Self::EMIT_LEADING_QUOTE.bits | Self::EMIT_TRAILING_QUOTE.bits,
    };

    #[inline]
    pub fn emit_leading_quote(&mut self) -> Option<char> {
        if (self.bits & Self::EMIT_LEADING_QUOTE.bits) == Self::EMIT_LEADING_QUOTE.bits {
            self.bits &= !Self::EMIT_LEADING_QUOTE.bits;
            Some('"')
        } else {
            None
        }
    }

    #[inline]
    pub fn emit_trailing_quote(&mut self) -> Option<char> {
        if (self.bits & Self::EMIT_TRAILING_QUOTE.bits) == Self::EMIT_TRAILING_QUOTE.bits {
            self.bits &= !Self::EMIT_TRAILING_QUOTE.bits;
            Some('"')
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Flags;

    #[test]
    fn flags_default_emit_quotes() {
        let mut flags = Flags::DEFAULT;

        assert_eq!(flags.emit_leading_quote(), Some('"'));
        assert_eq!(flags.emit_leading_quote(), None);

        assert_eq!(flags.emit_trailing_quote(), Some('"'));
        assert_eq!(flags.emit_trailing_quote(), None);
    }
}
