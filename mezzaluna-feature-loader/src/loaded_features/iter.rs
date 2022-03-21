use core::iter::FusedIterator;
use core::slice;
use std::collections::hash_set;
use std::path::{Path, PathBuf};

use super::Feature;

/// An iterator over the feature paths in a `LoadedFeatures`.
///
/// This struct is created by the [`iter`] method on [`LoadedFeatures`]. See its
/// documentation for more.
///
/// [`iter`]: super::LoadedFeatures::iter
/// [`LoadedFeatures`]: super::LoadedFeatures
#[derive(Debug, Clone)]
pub struct Iter<'a> {
    pub(crate) inner: slice::Iter<'a, PathBuf>,
}

impl<'a> Iterator for Iter<'a> {
    type Item = &'a Path;

    fn next(&mut self) -> Option<Self::Item> {
        let next = self.inner.next()?;
        Some(&*next)
    }

    fn nth(&mut self, n: usize) -> Option<Self::Item> {
        let nth = self.inner.nth(n)?;
        Some(&*nth)
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        self.inner.size_hint()
    }

    fn count(self) -> usize {
        self.inner.count()
    }

    fn last(self) -> Option<Self::Item> {
        let last = self.inner.last()?;
        Some(&*last)
    }
}

impl<'a> ExactSizeIterator for Iter<'a> {
    fn len(&self) -> usize {
        self.inner.len()
    }
}

impl<'a> FusedIterator for Iter<'a> {}

/// An iterator over the features in a `LoadedFeatures`.
///
/// This struct is created by the [`features`] method on [`LoadedFeatures`]. See
/// its documentation for more.
///
/// [`features`]: super::LoadedFeatures::features
/// [`LoadedFeatures`]: super::LoadedFeatures
#[derive(Debug, Clone)]
pub struct Features<'a> {
    pub(crate) inner: hash_set::Iter<'a, Feature>,
}

impl<'a> Iterator for Features<'a> {
    type Item = &'a Feature;

    fn next(&mut self) -> Option<Self::Item> {
        self.inner.next()
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        self.inner.size_hint()
    }
}

impl<'a> ExactSizeIterator for Features<'a> {
    fn len(&self) -> usize {
        self.inner.len()
    }
}

impl<'a> FusedIterator for Features<'a> {}
