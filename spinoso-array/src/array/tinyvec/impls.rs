use core::borrow::{Borrow, BorrowMut};
use core::ops::{Deref, DerefMut, Index, IndexMut};
use core::slice::{Iter, IterMut, SliceIndex};

use tinyvec::TinyVec;

use crate::array::tinyvec::TinyArray;
use crate::array::INLINE_CAPACITY;

impl<T> AsRef<TinyVec<[T; INLINE_CAPACITY]>> for TinyArray<T>
where
    T: Default,
{
    #[inline]
    fn as_ref(&self) -> &TinyVec<[T; INLINE_CAPACITY]> {
        &self.0
    }
}

impl<T> AsRef<[T]> for TinyArray<T>
where
    T: Default,
{
    #[inline]
    fn as_ref(&self) -> &[T] {
        self.0.as_ref()
    }
}

impl<T> AsMut<TinyVec<[T; INLINE_CAPACITY]>> for TinyArray<T>
where
    T: Default,
{
    #[inline]
    fn as_mut(&mut self) -> &mut TinyVec<[T; INLINE_CAPACITY]> {
        &mut self.0
    }
}

impl<T> AsMut<[T]> for TinyArray<T>
where
    T: Default,
{
    #[inline]
    fn as_mut(&mut self) -> &mut [T] {
        self.0.as_mut()
    }
}

impl<T> Borrow<[T]> for TinyArray<T>
where
    T: Default,
{
    #[inline]
    fn borrow(&self) -> &[T] {
        self.0.borrow()
    }
}

impl<T> BorrowMut<[T]> for TinyArray<T>
where
    T: Default,
{
    #[inline]
    fn borrow_mut(&mut self) -> &mut [T] {
        self.0.borrow_mut()
    }
}

impl<T> Deref for TinyArray<T>
where
    T: Default,
{
    type Target = [T];

    #[inline]
    fn deref(&self) -> &Self::Target {
        self.0.as_slice()
    }
}

impl<T> DerefMut for TinyArray<T>
where
    T: Default,
{
    #[inline]
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.0.as_mut_slice()
    }
}

impl<T> Extend<T> for TinyArray<T>
where
    T: Default,
{
    #[inline]
    fn extend<I: IntoIterator<Item = T>>(&mut self, iter: I) {
        self.0.extend(iter.into_iter());
    }
}

impl<'a, T> Extend<&'a T> for TinyArray<T>
where
    T: 'a + Clone + Default,
{
    #[inline]
    fn extend<I: IntoIterator<Item = &'a T>>(&mut self, iter: I) {
        self.0.extend(iter.into_iter().cloned());
    }
}

impl<T, I> Index<I> for TinyArray<T>
where
    I: SliceIndex<[T]>,
    T: Default,
{
    type Output = I::Output;

    #[inline]
    fn index(&self, index: I) -> &I::Output {
        &self.0[index]
    }
}

impl<T, I> IndexMut<I> for TinyArray<T>
where
    I: SliceIndex<[T]>,
    T: Default,
{
    #[inline]
    fn index_mut(&mut self, index: I) -> &mut I::Output {
        &mut self.0[index]
    }
}

impl<'a, T> IntoIterator for &'a TinyArray<T>
where
    T: Default,
{
    type Item = &'a T;
    type IntoIter = Iter<'a, T>;

    #[inline]
    fn into_iter(self) -> Self::IntoIter {
        self.0.iter()
    }
}

impl<'a, T> IntoIterator for &'a mut TinyArray<T>
where
    T: Default,
{
    type Item = &'a mut T;
    type IntoIter = IterMut<'a, T>;

    #[inline]
    fn into_iter(self) -> Self::IntoIter {
        self.0.iter_mut()
    }
}
