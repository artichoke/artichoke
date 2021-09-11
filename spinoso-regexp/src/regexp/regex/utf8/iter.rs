use core::iter::{Enumerate, FusedIterator};

use regex::CaptureNames;

#[derive(Debug)]
pub struct Captures<'a> {
    captures: regex::Captures<'a>,
    idx: usize,
}

impl<'a> From<regex::Captures<'a>> for Captures<'a> {
    fn from(captures: regex::Captures<'a>) -> Self {
        Self { captures, idx: 0 }
    }
}

impl<'a> Iterator for Captures<'a> {
    type Item = Option<&'a [u8]>;

    fn next(&mut self) -> Option<Self::Item> {
        let subcapture = match self.captures.iter().skip(self.idx).next() {
            Some(Some(capture)) => Some(capture.as_str().as_bytes()),
            Some(None) => None,
            None => return None,
        };
        self.idx += 1;
        Some(subcapture)
    }

    fn nth(&mut self, n: usize) -> Option<Self::Item> {
        let subcapture = match self.captures.iter().skip(self.idx).nth(n) {
            Some(Some(capture)) => Some(capture.as_str().as_bytes()),
            Some(None) => None,
            None => return None,
        };
        self.idx += n;
        Some(subcapture)
    }

    fn count(self) -> usize {
        self.captures.iter().skip(self.idx).count()
    }
}

impl<'a> FusedIterator for Captures<'a> {}

#[derive(Debug)]
pub struct CaptureIndices<'a, 'b> {
    name: &'b [u8],
    capture_names: Enumerate<CaptureNames<'a>>,
}

impl<'a, 'b> CaptureIndices<'a, 'b> {
    pub(crate) fn with_name_and_iter(name: &'b [u8], iter: CaptureNames<'a>) -> Self {
        Self {
            name,
            capture_names: iter.enumerate(),
        }
    }

    /// The name of the capture group this iterator targets.
    pub const fn name(&self) -> &'b [u8] {
        self.name
    }
}

impl<'a, 'b> Iterator for CaptureIndices<'a, 'b> {
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        while let Some((index, group)) = self.capture_names.next() {
            let group = group.map(str::as_bytes);
            if matches!(group, Some(group) if group == self.name) {
                return Some(index);
            }
        }
        None
    }
}

impl<'a, 'b> FusedIterator for CaptureIndices<'a, 'b> {}
