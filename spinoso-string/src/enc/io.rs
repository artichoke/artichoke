use core::fmt;
use std::io::{self, IoSlice, Write};

use super::EncodedString;

impl Write for EncodedString {
    #[inline]
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        match self {
            EncodedString::Ascii(inner) => inner.write(buf),
            EncodedString::Binary(inner) => inner.write(buf),
            EncodedString::Utf8(inner) => inner.write(buf),
        }
    }

    #[inline]
    fn write_all(&mut self, buf: &[u8]) -> io::Result<()> {
        match self {
            EncodedString::Ascii(inner) => inner.write_all(buf),
            EncodedString::Binary(inner) => inner.write_all(buf),
            EncodedString::Utf8(inner) => inner.write_all(buf),
        }
    }

    #[inline]
    fn write_fmt(&mut self, fmt: fmt::Arguments<'_>) -> io::Result<()> {
        match self {
            EncodedString::Ascii(inner) => inner.write_fmt(fmt),
            EncodedString::Binary(inner) => inner.write_fmt(fmt),
            EncodedString::Utf8(inner) => inner.write_fmt(fmt),
        }
    }

    #[inline]
    fn write_vectored(&mut self, bufs: &[IoSlice<'_>]) -> io::Result<usize> {
        match self {
            EncodedString::Ascii(inner) => inner.write_vectored(bufs),
            EncodedString::Binary(inner) => inner.write_vectored(bufs),
            EncodedString::Utf8(inner) => inner.write_vectored(bufs),
        }
    }

    #[inline]
    fn flush(&mut self) -> io::Result<()> {
        match self {
            EncodedString::Ascii(inner) => inner.flush(),
            EncodedString::Binary(inner) => inner.flush(),
            EncodedString::Utf8(inner) => inner.flush(),
        }
    }
}
