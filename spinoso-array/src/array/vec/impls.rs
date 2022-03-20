use alloc::vec::Vec;
use core::borrow::{Borrow, BorrowMut};
use core::ops::{Deref, DerefMut, Index, IndexMut};
use core::slice::{Iter, IterMut, SliceIndex};

use crate::array::vec::Array;

impl<T> AsRef<Vec<T>> for Array<T> {
    #[inline]
    fn as_ref(&self) -> &Vec<T> {
        &self.0
    }
}

impl<T> AsRef<[T]> for Array<T> {
    #[inline]
    fn as_ref(&self) -> &[T] {
        self.0.as_ref()
    }
}

impl<T> AsMut<Vec<T>> for Array<T> {
    #[inline]
    fn as_mut(&mut self) -> &mut Vec<T> {
        &mut self.0
    }
}

impl<T> AsMut<[T]> for Array<T> {
    #[inline]
    fn as_mut(&mut self) -> &mut [T] {
        self.0.as_mut()
    }
}

impl<T> Borrow<[T]> for Array<T> {
    #[inline]
    fn borrow(&self) -> &[T] {
        self.0.borrow()
    }
}

impl<T> BorrowMut<[T]> for Array<T> {
    #[inline]
    fn borrow_mut(&mut self) -> &mut [T] {
        self.0.borrow_mut()
    }
}

impl<T> Deref for Array<T> {
    type Target = [T];

    #[inline]
    fn deref(&self) -> &Self::Target {
        &*self.0
    }
}

impl<T> DerefMut for Array<T> {
    #[inline]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut *self.0
    }
}

impl<T> Extend<T> for Array<T> {
    #[inline]
    fn extend<I: IntoIterator<Item = T>>(&mut self, iter: I) {
        self.0.extend(iter.into_iter());
    }
}

impl<'a, T> Extend<&'a T> for Array<T>
where
    T: 'a + Copy,
{
    #[inline]
    fn extend<I: IntoIterator<Item = &'a T>>(&mut self, iter: I) {
        self.0.extend(iter.into_iter());
    }
}

impl<T, I> Index<I> for Array<T>
where
    I: SliceIndex<[T]>,
{
    type Output = I::Output;

    #[inline]
    fn index(&self, index: I) -> &I::Output {
        &self.0[index]
    }
}

impl<T, I> IndexMut<I> for Array<T>
where
    I: SliceIndex<[T]>,
{
    #[inline]
    fn index_mut(&mut self, index: I) -> &mut I::Output {
        &mut self.0[index]
    }
}

impl<'a, T> IntoIterator for &'a Array<T> {
    type Item = &'a T;
    type IntoIter = Iter<'a, T>;

    #[inline]
    fn into_iter(self) -> Self::IntoIter {
        self.0.iter()
    }
}

impl<'a, T> IntoIterator for &'a mut Array<T> {
    type Item = &'a mut T;
    type IntoIter = IterMut<'a, T>;

    #[inline]
    fn into_iter(self) -> Self::IntoIter {
        self.0.iter_mut()
    }
}
