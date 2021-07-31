use core::fmt;
use std::error::Error;

#[derive(Default, Debug, Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct RangeError {
    message: &'static str,
}

impl RangeError {
    pub const fn new() -> Self {
        Self::with_message("RangeError")
    }

    pub const fn with_message(message: &'static str) -> Self {
        Self { message }
    }

    pub fn message(self) -> &'static str {
        self.message
    }
}

impl fmt::Display for RangeError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.message)
    }
}

impl Error for RangeError {}

#[derive(Default, Debug, Clone, Hash)]
pub struct StringScanner {
    string: Vec<u8>,
    scan_pos: usize,
    previous_scan_pos: Option<usize>,
    last_match: Option<(usize, usize)>,
}

impl From<String> for StringScanner {
    fn from(string: String) -> Self {
        Self::new(string.into_bytes())
    }
}

impl From<Vec<u8>> for StringScanner {
    fn from(string: Vec<u8>) -> Self {
        Self::new(string)
    }
}

impl StringScanner {
    pub const fn new(string: Vec<u8>) -> Self {
        Self {
            string,
            scan_pos: 0,
            previous_scan_pos: None,
            last_match: None,
        }
    }

    pub fn concat<T: AsRef<[u8]>>(&mut self, string: T) {
        self.string.extend_from_slice(string.as_ref());
    }

    pub fn is_beginning_of_line(&self) -> bool {
        let previous_byte = self
            .scan_pos
            .checked_sub(1)
            .and_then(|pos| self.string.get(pos))
            .copied();
        matches!(previous_byte, None | Some(b'\n'))
    }

    pub fn charpos(&self) -> usize {
        let mut consumed = &self.string[..self.scan_pos];
        let mut charpos = 0_usize;
        while !consumed.is_empty() {
            let (ch, size) = bstr::decode_utf8(consumed);
            charpos += ch.map_or(size, |_| 1);
            consumed = &consumed[size..];
        }
        charpos
    }

    pub fn is_end_of_stream(&self) -> bool {
        self.string.get(self.scan_pos).is_none()
    }

    pub fn is_fixed_anchor(&self) -> bool {
        // TODO: implement `fixed_anchor` mode.
        //
        // From https://ruby-doc.org/stdlib-2.6.3/libdoc/strscan/rdoc/StringScanner.html#method-c-new:
        //
        // > If `fixed_anchor` is `true`, `\A` always matches the beginning of
        // > the string. Otherwise, `\A` always matches the current position.
        false
    }

    pub fn get_byte(&mut self) -> Option<u8> {
        let byte = self.string.get(self.scan_pos).copied()?;
        let previous_scan_pos = self.scan_pos;
        self.scan_pos += 1;
        self.previous_scan_pos = Some(previous_scan_pos);
        self.last_match = Some((previous_scan_pos, self.scan_pos));
        Some(byte)
    }

    pub fn get_ch(&mut self) -> Option<&[u8]> {
        let remaining = &self.string[self.scan_pos..];
        let (ch, size) = bstr::decode_utf8(remaining);
        let previous_scan_pos = self.scan_pos;
        self.scan_pos += size;
        self.previous_scan_pos = Some(previous_scan_pos);
        self.last_match = Some((previous_scan_pos, self.scan_pos));
        if ch.is_some() {
            Some(&remaining[..size])
        } else {
            None
        }
    }

    pub fn matched(&self) -> Option<&[u8]> {
        let (start, end) = self.last_match?;
        self.string.get(start..end)
    }

    pub fn is_matched(&self) -> bool {
        self.last_match.is_some()
    }

    pub fn matched_size(&self) -> Option<usize> {
        let (start, end) = self.last_match?;
        Some(end - start)
    }

    pub fn peek(&self, len: usize) -> &[u8] {
        self.string
            .get(self.scan_pos..self.scan_pos.saturating_add(len))
            .or_else(|| self.string.get(self.scan_pos..))
            .unwrap_or_default()
    }

    pub fn pos(&self) -> usize {
        self.scan_pos
    }

    pub fn set_pos(&mut self, pos: usize) -> Result<usize, RangeError> {
        if self.string.get(pos).is_none() {
            return Err(RangeError::with_message("index out of range"));
        }
        self.scan_pos = pos;
        Ok(pos)
    }

    pub fn post_match(&self) -> Option<&[u8]> {
        let (_, end) = self.last_match?;
        self.string.get(end..)
    }

    pub fn pre_match(&self) -> Option<&[u8]> {
        let (start, _) = self.last_match?;
        self.string.get(..start)
    }

    pub fn reset(&mut self) {
        self.scan_pos = 0;
        self.previous_scan_pos = None;
        self.last_match = None;
    }

    pub fn rest(&self) -> &[u8] {
        self.string.get(self.scan_pos..).unwrap_or_default()
    }

    pub fn rest_size(&self) -> usize {
        self.string.len().saturating_sub(self.scan_pos)
    }

    pub fn string(&self) -> &[u8] {
        self.string.as_slice()
    }

    pub fn set_string(&mut self, string: Vec<u8>) {
        self.reset();
        self.string = string;
    }

    pub fn terminate(&mut self) {
        self.scan_pos = self.string.len();
        self.last_match = None;
    }

    pub fn unscan(&mut self) {
        if let Some(previous_scan_pos) = self.previous_scan_pos.take() {
            self.scan_pos = previous_scan_pos;
        }
    }
}
