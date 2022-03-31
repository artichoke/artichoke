use core::iter::{Cycle, FusedIterator, Take};
use core::slice;

use super::super::{BinaryString, Utf8String};
use super::AsciiString;
use crate::enc::center::ZeroWidthPaddingError;

#[derive(Debug, Clone)]
pub struct Center<'a, 'b> {
    inner: Inner<'a, 'b>,
}

#[derive(Debug, Clone)]
enum Inner<'a, 'b> {
    Echo(slice::Iter<'a, u8>),
    PadBytes {
        left: Take<Cycle<slice::Iter<'b, u8>>>,
        s: slice::Iter<'a, u8>,
        right: Take<Cycle<slice::Iter<'b, u8>>>,
    },
}

impl<'a, 'b> Center<'a, 'b> {
    pub fn with_string_width_and_ascii_padding(
        s: &'a AsciiString,
        width: usize,
        padding: Option<&'b AsciiString>,
    ) -> Result<Self, ZeroWidthPaddingError> {
        let padding = match padding {
            None => b" ",
            Some(p) if p.is_empty() => return Err(ZeroWidthPaddingError::new()),
            Some(p) => p.as_slice(),
        };

        let inner = match width.checked_sub(s.char_len()) {
            None | Some(0) => Inner::Echo(s.as_slice().iter()),
            Some(remaining_padding_width) => {
                // Left and right padding starts from the beginning of padding.
                // If padding width is odd, extra padding goes on right side.
                //
                // ```
                // [3.0.3] > "abc".center 10, "123456789"
                // => "123abc1234"
                // ```
                let pre_pad = remaining_padding_width / 2;
                let post_pad = remaining_padding_width - pre_pad;
                let left = padding.iter().cycle().take(pre_pad);
                let right = padding.iter().cycle().take(post_pad);

                Inner::PadBytes {
                    left,
                    s: s.as_slice().iter(),
                    right,
                }
            }
        };
        Ok(Self { inner })
    }

    pub fn with_string_width_and_binary_padding(
        s: &'a AsciiString,
        width: usize,
        padding: Option<&'b BinaryString>,
    ) -> Result<Self, ZeroWidthPaddingError> {
        let padding = match padding {
            None => b" ",
            Some(p) if p.is_empty() => return Err(ZeroWidthPaddingError::new()),
            Some(p) => p.as_slice(),
        };

        let inner = match width.checked_sub(s.char_len()) {
            None | Some(0) => Inner::Echo(s.as_slice().iter()),
            Some(remaining_padding_width) => {
                // Left and right padding starts from the beginning of padding.
                // If padding width is odd, extra padding goes on right side.
                //
                // ```
                // [3.0.3] > "abc".center 10, "123456789"
                // => "123abc1234"
                // ```
                let pre_pad = remaining_padding_width / 2;
                let post_pad = remaining_padding_width - pre_pad;
                let left = padding.iter().cycle().take(pre_pad);
                let right = padding.iter().cycle().take(post_pad);

                Inner::PadBytes {
                    left,
                    s: s.as_slice().iter(),
                    right,
                }
            }
        };
        Ok(Self { inner })
    }
}

impl<'a, 'b> Iterator for Center<'a, 'b> {
    type Item = u8;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        match &mut self.inner {
            Inner::Echo(s) => s.next().copied(),
            Inner::PadBytes { left, s, right } => {
                if let Some(&next) = left.next() {
                    return Some(next);
                }
                if let Some(&next) = s.next() {
                    return Some(next);
                }
                right.next().copied()
            }
        }
    }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        match &self.inner {
            Inner::Echo(s) => {
                let remaining = s.len();
                (remaining, Some(remaining))
            }
            Inner::PadBytes { left, s, right } => {
                let left = left.size_hint();
                let right = right.size_hint();

                let min = left
                    .0
                    .checked_add(s.len())
                    .and_then(|len| len.checked_add(right.0))
                    .unwrap_or(usize::MAX);

                let max = if let (Some(left), Some(right)) = (left.1, right.1) {
                    left.checked_add(right)
                        .and_then(|len| len.checked_add(s.len()))
                        .filter(|&len| len < usize::MAX)
                } else {
                    None
                };
                (min, max)
            }
        }
    }
}

impl<'a, 'b> FusedIterator for Center<'a, 'b> {}

#[cfg(test)]
mod tests {
    use alloc::vec::Vec;

    use bstr::ByteSlice;

    use super::{AsciiString, BinaryString, Center};

    #[test]
    fn padding_width_does_not_overflow_default_ascii_padding() {
        let s = AsciiString::new(b"abc".to_vec());
        let iter = Center::with_string_width_and_ascii_padding(&s, usize::MAX, None).unwrap();
        assert_eq!(iter.size_hint(), (usize::MAX, None));
    }

    #[test]
    fn padding_width_does_not_overflow_default_binary_padding() {
        let s = AsciiString::new(b"abc".to_vec());
        let iter = Center::with_string_width_and_binary_padding(&s, usize::MAX, None).unwrap();
        assert_eq!(iter.size_hint(), (usize::MAX, None));
    }

    #[test]
    fn padding_width_does_not_overflow_given_ascii_padding() {
        let s = AsciiString::new(b"abc".to_vec());
        let pad = AsciiString::new(b"1234567890".to_vec());
        let iter = Center::with_string_width_and_ascii_padding(&s, usize::MAX, Some(&pad)).unwrap();
        assert_eq!(iter.size_hint(), (usize::MAX, None));
    }

    #[test]
    fn padding_width_does_not_overflow_given_ascii_padding_and_non_overflowing_width() {
        let s = AsciiString::new(b"abc".to_vec());
        let pad = AsciiString::new(b"1234567890".to_vec());
        let iter = Center::with_string_width_and_ascii_padding(&s, usize::MAX / 7 - 3, Some(&pad)).unwrap();
        assert_eq!(iter.size_hint(), (usize::MAX / 7 - 3, Some(usize::MAX / 7 - 3)));
    }

    #[test]
    fn padding_width_does_not_overflow_given_binary_padding() {
        let s = AsciiString::new(b"abc".to_vec());
        let pad = BinaryString::new(b"1234567890".to_vec());
        let iter = Center::with_string_width_and_binary_padding(&s, usize::MAX, Some(&pad)).unwrap();
        assert_eq!(iter.size_hint(), (usize::MAX, None));
    }

    #[test]
    fn padding_width_does_not_overflow_given_binary_padding_and_non_overflowing_width() {
        let s = AsciiString::new(b"abc".to_vec());
        let pad = BinaryString::new(b"1234567890".to_vec());
        let iter = Center::with_string_width_and_binary_padding(&s, usize::MAX / 7 - 3, Some(&pad)).unwrap();
        assert_eq!(iter.size_hint(), (usize::MAX / 7 - 3, Some(usize::MAX / 7 - 3)));
    }

    #[test]
    fn empty_ascii_padding_returns_zero_width_padding_error() {
        let s = AsciiString::new(b"abc".to_vec());
        let pad = AsciiString::new(b"".to_vec());
        Center::with_string_width_and_ascii_padding(&s, usize::MAX, Some(&pad)).unwrap_err();
    }

    #[test]
    fn empty_binary_padding_returns_zero_width_padding_error() {
        let s = AsciiString::new(b"abc".to_vec());
        let pad = BinaryString::new(b"".to_vec());
        Center::with_string_width_and_binary_padding(&s, usize::MAX, Some(&pad)).unwrap_err();
    }

    #[test]
    fn empty_ascii_padding_with_string_wider_than_padded_width_returns_zero_width_padding_error() {
        let s = AsciiString::new(b"12345".to_vec());
        let pad = AsciiString::new(b"".to_vec());
        Center::with_string_width_and_ascii_padding(&s, 3, Some(&pad)).unwrap_err();
    }

    #[test]
    fn empty_binary_padding_with_string_wider_than_padded_width_returns_zero_width_padding_error() {
        let s = AsciiString::new(b"12345".to_vec());
        let pad = BinaryString::new(b"".to_vec());
        Center::with_string_width_and_binary_padding(&s, 3, Some(&pad)).unwrap_err();
    }

    #[test]
    fn centers_with_ascii_padding() {
        let s = AsciiString::new(b"12345".to_vec());
        let pad = AsciiString::new(b"xyz".to_vec());
        let iter = Center::with_string_width_and_ascii_padding(&s, 10, Some(&pad)).unwrap();
        let centered = iter.collect::<Vec<_>>();
        assert_eq!(centered.as_bstr(), b"xy12345xyz".as_bstr());
    }

    #[test]
    fn centers_with_binary_padding() {
        let s = AsciiString::new(b"12345".to_vec());
        let pad = AsciiString::new(b"xyz".to_vec());
        let iter = Center::with_string_width_and_ascii_padding(&s, 10, Some(&pad)).unwrap();
        let centered = iter.collect::<Vec<_>>();
        assert_eq!(centered.as_bstr(), b"xy12345xyz".as_bstr());
    }

    #[test]
    fn yields_receiver_with_ascii_padding_when_receiver_len_exceeds_width() {
        let s = AsciiString::new(b"12345".to_vec());
        let pad = AsciiString::new(b"xyz".to_vec());
        let iter = Center::with_string_width_and_ascii_padding(&s, 3, Some(&pad)).unwrap();
        let centered = iter.collect::<Vec<_>>();
        assert_eq!(centered.as_bstr(), b"12345".as_bstr());
    }

    #[test]
    fn yields_receiver_with_binary_padding_when_receiver_len_exceeds_width() {
        let s = AsciiString::new(b"12345".to_vec());
        let pad = BinaryString::new(b"xyz".to_vec());
        let iter = Center::with_string_width_and_binary_padding(&s, 3, Some(&pad)).unwrap();
        let centered = iter.collect::<Vec<_>>();
        assert_eq!(centered.as_bstr(), b"12345".as_bstr());
    }
}
