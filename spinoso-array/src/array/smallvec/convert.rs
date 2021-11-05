use alloc::borrow::Cow;
use alloc::boxed::Box;
use alloc::rc::Rc;
use alloc::sync::Arc;
use alloc::vec::Vec;

use smallvec::SmallVec;

use crate::array::smallvec::SmallArray;
use crate::array::vec::Array;
use crate::array::INLINE_CAPACITY;

impl<T> From<Vec<T>> for SmallArray<T> {
    #[inline]
    fn from(values: Vec<T>) -> Self {
        Self(values.into())
    }
}

impl<T> From<SmallArray<T>> for Vec<T> {
    #[inline]
    fn from(values: SmallArray<T>) -> Self {
        values.into_vec()
    }
}

impl<'a, T> From<&'a [T]> for SmallArray<T>
where
    T: Copy,
{
    #[inline]
    fn from(values: &'a [T]) -> Self {
        Self(SmallVec::from_slice(values))
    }
}

impl<'a, T> From<&'a mut [T]> for SmallArray<T>
where
    T: Copy,
{
    #[inline]
    fn from(values: &'a mut [T]) -> Self {
        Self(SmallVec::from_slice(values))
    }
}

impl<T> From<Box<[T]>> for SmallArray<T> {
    #[inline]
    fn from(values: Box<[T]>) -> Self {
        Self(Vec::from(values).into())
    }
}

impl<T> From<SmallArray<T>> for Box<[T]> {
    #[inline]
    fn from(values: SmallArray<T>) -> Self {
        values.into_boxed_slice()
    }
}

impl<'a, T> From<Cow<'a, [T]>> for SmallArray<T>
where
    T: Copy,
{
    #[inline]
    fn from(values: Cow<'a, [T]>) -> Self {
        match values {
            Cow::Borrowed(slice) => slice.into(),
            Cow::Owned(vec) => vec.into(),
        }
    }
}

impl<'a, T> From<SmallArray<T>> for Cow<'a, [T]>
where
    T: Clone,
{
    #[inline]
    fn from(values: SmallArray<T>) -> Self {
        values.into_vec().into()
    }
}

impl<T> From<SmallArray<T>> for Rc<[T]> {
    #[inline]
    fn from(values: SmallArray<T>) -> Self {
        values.into_vec().into()
    }
}

impl<T> From<SmallArray<T>> for Arc<[T]> {
    #[inline]
    fn from(values: SmallArray<T>) -> Self {
        values.into_vec().into()
    }
}

impl<T, const N: usize> From<[T; N]> for SmallArray<T> {
    #[inline]
    fn from(values: [T; N]) -> Self {
        Self(values.into_iter().collect())
    }
}

impl<T, const N: usize> From<&[T; N]> for SmallArray<T>
where
    T: Copy,
{
    #[inline]
    fn from(values: &[T; N]) -> Self {
        Self(SmallVec::from_slice(values))
    }
}

impl<T> From<SmallVec<[T; INLINE_CAPACITY]>> for SmallArray<T> {
    #[inline]
    fn from(values: SmallVec<[T; INLINE_CAPACITY]>) -> Self {
        Self(values)
    }
}

impl<T> From<SmallArray<T>> for SmallVec<[T; INLINE_CAPACITY]> {
    #[inline]
    fn from(values: SmallArray<T>) -> Self {
        values.into_inner()
    }
}

impl<T> From<Array<T>> for SmallArray<T> {
    #[inline]
    fn from(values: Array<T>) -> Self {
        Self::from(values.into_vec())
    }
}

impl<T> FromIterator<T> for SmallArray<T> {
    #[inline]
    fn from_iter<I>(iter: I) -> Self
    where
        I: IntoIterator<Item = T>,
    {
        Self(iter.into_iter().collect())
    }
}

impl<'a, T> FromIterator<&'a T> for SmallArray<T>
where
    T: 'a + Copy,
{
    #[inline]
    fn from_iter<I>(iter: I) -> Self
    where
        I: IntoIterator<Item = &'a T>,
    {
        Self(iter.into_iter().copied().collect())
    }
}
