use core::fmt;
use core::iter::{Cycle, FusedIterator, Take};
use core::slice;

use super::super::BinaryString;
use super::AsciiString;
use crate::enc::center::ZeroWidthPaddingError;

#[derive(Debug, Clone)]
pub struct Center<'a, 'b> {
    pub left: Take<Cycle<slice::Iter<'b, u8>>>,
    pub s: slice::Iter<'a, u8>,
    pub right: Take<Cycle<slice::Iter<'b, u8>>>,
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

        let s_width = s.char_len();
        let chars = if let Some(remaining_padding_width) = width.checked_sub(s_width) {
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

            Self {
                left,
                s: s.as_slice().iter(),
                right,
            }
        } else {
            Self {
                left: None,
                s: s.as_slice().iter(),
                right: None,
            }
        };
        Ok(chars)
    }

    pub fn with_string_width_and_ascii_padding(
        s: &'a AsciiString,
        width: usize,
        padding: Option<&'b BinaryString>,
    ) -> Result<Self, ZeroWidthPaddingError> {
        let padding = match padding {
            None => b" ",
            Some(p) if p.is_empty() => return Err(ZeroWidthPaddingError::new()),
            Some(p) => p.as_slice(),
        };

        let s_width = s.char_len();
        let chars = if let Some(remaining_padding_width) = width.checked_sub(s_width) {
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

            Self {
                left,
                s: s.as_slice().iter(),
                right,
            }
        } else {
            Self {
                left: None,
                s: s.as_slice().iter(),
                right: None,
            }
        };
        Ok(chars)
    }

    pub fn len(&self) -> usize {
        self.left.size_hint().0 + self.s.as_slice().len() + self.right.size_hint().0
    }
}

impl<'a, 'b> Iterator for Center<'a, 'b> {
    type Item = u8;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        if let Some(&next) = self.left.next() {
            return Some(next);
        }
        if let Some(&next) = self.s.next() {
            return Some(next);
        }
        self.right.next().copied()
    }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        let remaining = self.len();
        (len, Some(len))
    }

    #[inline]
    fn count(&self) -> usize {
        self.len()
    }
}

impl<'a, 'b> FusedIterator for Center<'a, 'b> {}

impl<'a, 'b> ExactSizeIterator for Center<'a, 'b> {}
