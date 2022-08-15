use std::collections::HashMap;
use std::iter::FusedIterator;
use std::vec;

#[derive(Default, Debug, Clone, Hash, PartialEq, Eq)]
pub struct NamedCapture {
    group: Vec<u8>,
    indices: Vec<usize>,
}

impl NamedCapture {
    #[must_use]
    pub(crate) const fn new(group: Vec<u8>, indices: Vec<usize>) -> Self {
        Self { group, indices }
    }

    #[must_use]
    pub fn group(&self) -> &[u8] {
        &self.group[..]
    }

    #[must_use]
    pub fn indices(&self) -> &[usize] {
        &self.indices[..]
    }

    #[must_use]
    pub fn into_group(self) -> Vec<u8> {
        self.group
    }

    #[must_use]
    pub fn into_group_and_indices(self) -> (Vec<u8>, Vec<usize>) {
        (self.group, self.indices)
    }
}

#[derive(Debug, Clone)]
#[must_use = "this `NamedCaptures` is an `Iterator`, which should be consumed if constructed"]
pub struct NamedCaptures {
    items: vec::IntoIter<NamedCapture>,
}

impl Default for NamedCaptures {
    fn default() -> Self {
        vec![].into()
    }
}

impl From<Vec<NamedCapture>> for NamedCaptures {
    fn from(named_captures: Vec<NamedCapture>) -> Self {
        Self {
            items: named_captures.into_iter(),
        }
    }
}

impl Iterator for NamedCaptures {
    type Item = NamedCapture;

    fn next(&mut self) -> Option<Self::Item> {
        self.items.next()
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        self.items.size_hint()
    }

    fn count(self) -> usize {
        self.items.count()
    }
}

impl DoubleEndedIterator for NamedCaptures {
    fn next_back(&mut self) -> Option<Self::Item> {
        self.items.next_back()
    }
}

impl ExactSizeIterator for NamedCaptures {}

impl FusedIterator for NamedCaptures {}

#[derive(Default, Debug, Clone)]
#[allow(clippy::module_name_repetitions)]
pub struct NamedCapturesForHaystack {
    matches: HashMap<Vec<u8>, Option<Vec<u8>>>,
}

impl NamedCapturesForHaystack {
    pub(crate) fn with_capacity(capacity: usize) -> Self {
        let matches = HashMap::with_capacity(capacity);
        Self { matches }
    }

    pub(crate) fn insert(&mut self, name: Vec<u8>, matched: Option<Vec<u8>>) {
        self.matches.insert(name, matched);
    }

    #[must_use]
    pub fn into_map(self) -> HashMap<Vec<u8>, Option<Vec<u8>>> {
        self.matches
    }
}
