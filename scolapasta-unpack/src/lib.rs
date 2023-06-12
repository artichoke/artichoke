mod directive;
mod repetition;

use directive::{Directive, MiscDirective};
use repetition::Repetition;

#[derive(Default, Debug, Clone, PartialEq, Eq)]
pub struct RangeError {
    message: &'static str,
}

impl RangeError {
    pub fn with_message(message: &'static str) -> Self {
        Self { message }
    }

    pub fn message(&self) -> &'static str {
        self.message
    }
}

/// Iterator that parses a format string into directives.
pub struct FormatStringIterator<'a> {
    bytes: &'a [u8],
    idx: usize,
    last_directive: Option<(Directive, Repetition)>,
}

impl<'a> FormatStringIterator<'a> {
    pub fn new(format_string: &'a [u8]) -> Self {
        FormatStringIterator {
            bytes: format_string,
            idx: 0,
            last_directive: None,
        }
    }
}

impl<'a> Iterator for FormatStringIterator<'a> {
    type Item = Result<Directive, RangeError>;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some((directive, repetition)) = self.last_directive.take() {
            match repetition.next() {
                None | Some(Repetition::Finished) => {}
                Some(next_repetition) => {
                    self.last_directive = Some((directive, next_repetition));
                }
            }
            return Some(Ok(directive));
        }

        let misc = None;
        loop {
            let s = match misc {
                Some((_, Repetition::ConsumeToEnd)) => {
                    self.idx = usize::MAX;
                    &[]
                }
                Some((MiscDirective::SkipToOffset, Repetition::Finished)) => {
                    self.idx = 0;
                    self.bytes
                }
                Some((MiscDirective::SkipToOffset, Repetition::Repeat(idx))) => {
                    self.idx = idx.get();
                    match self.bytes.get(self.idx..) {
                        Some(s) => s,
                        None => return Some(Err(RangeError::with_message("@ outside of string"))),
                    }
                }
                Some((MiscDirective::SkipBackward | MiscDirective::SkipForward, Repetition::Finished)) => self.bytes,
                Some((MiscDirective::SkipBackward, Repetition::Repeat(idx))) => {
                    self.idx = idx.get();
                    match self.bytes.get(self.idx..) {
                        Some(s) => s,
                        None => return Some(Err(RangeError::with_message("@ outside of string"))),
                    }
                }
                Some((MiscDirective::SkipForward, Repetition::Repeat(idx))) => {
                    self.idx = idx.get();
                    match self.bytes.get(self.idx..) {
                        Some(s) => s,
                        None => return Some(Err(RangeError::with_message("@ outside of string"))),
                    }
                }
                None => self.bytes.get(self.idx..)?,
            };

            let mut format = s;
            let directive = Directive::next_from_format_bytes(&mut format);
            let amount = match Repetition::next_from_format_bytes(&mut format) {
                Ok(amount) => amount,
                Err(err) => {
                    self.idx = usize::MAX;
                    return Some(Err(err));
                }
            };
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
}
