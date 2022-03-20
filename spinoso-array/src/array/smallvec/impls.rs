use core::borrow::{Borrow, BorrowMut};
use core::ops::{Deref, DerefMut, Index, IndexMut};
use core::slice::{Iter, IterMut, SliceIndex};

use smallvec::SmallVec;

use crate::array::smallvec::SmallArray;
use crate::array::INLINE_CAPACITY;

impl<T> AsRef<SmallVec<[T; INLINE_CAPACITY]>> for SmallArray<T> {
    #[inline]
    fn as_ref(&self) -> &SmallVec<[T; INLINE_CAPACITY]> {
        &self.0
    }
}

impl<T> AsRef<[T]> for SmallArray<T> {
    #[inline]
    fn as_ref(&self) -> &[T] {
        self.0.as_ref()
    }
}

impl<T> AsMut<SmallVec<[T; INLINE_CAPACITY]>> for SmallArray<T> {
    #[inline]
    fn as_mut(&mut self) -> &mut SmallVec<[T; INLINE_CAPACITY]> {
        &mut self.0
    }
}

impl<T> AsMut<[T]> for SmallArray<T> {
    #[inline]
    fn as_mut(&mut self) -> &mut [T] {
        self.0.as_mut()
    }
}

impl<T> Borrow<[T]> for SmallArray<T> {
    #[inline]
    fn borrow(&self) -> &[T] {
        self.0.borrow()
    }
}

impl<T> BorrowMut<[T]> for SmallArray<T> {
    #[inline]
    fn borrow_mut(&mut self) -> &mut [T] {
        self.0.borrow_mut()
    }
}

impl<T> Deref for SmallArray<T> {
    type Target = [T];

    #[inline]
    fn deref(&self) -> &Self::Target {
        &*self.0
    }
}

impl<T> DerefMut for SmallArray<T> {
    #[inline]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut *self.0
    }
}

impl<T> Extend<T> for SmallArray<T> {
    #[inline]
    fn extend<I: IntoIterator<Item = T>>(&mut self, iter: I) {
        self.0.extend(iter.into_iter());
    }
}

impl<'a, T> Extend<&'a T> for SmallArray<T>
where
    T: 'a + Copy,
{
    #[inline]
    fn extend<I: IntoIterator<Item = &'a T>>(&mut self, iter: I) {
        self.0.extend(iter.into_iter().copied());
    }
}

impl<T, I> Index<I> for SmallArray<T>
where
    I: SliceIndex<[T]>,
{
    type Output = I::Output;

    #[inline]
    fn index(&self, index: I) -> &I::Output {
        &self.0[index]
    }
}

impl<T, I> IndexMut<I> for SmallArray<T>
where
    I: SliceIndex<[T]>,
{
    #[inline]
    fn index_mut(&mut self, index: I) -> &mut I::Output {
        &mut self.0[index]
    }
}

impl<'a, T> IntoIterator for &'a SmallArray<T> {
    type Item = &'a T;
    type IntoIter = Iter<'a, T>;

    #[inline]
    fn into_iter(self) -> Self::IntoIter {
        self.0.iter()
    }
}

impl<'a, T> IntoIterator for &'a mut SmallArray<T> {
    type Item = &'a mut T;
    type IntoIter = IterMut<'a, T>;

    #[inline]
    fn into_iter(self) -> Self::IntoIter {
        self.0.iter_mut()
    }
}
