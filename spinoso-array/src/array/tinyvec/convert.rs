use alloc::borrow::Cow;
use alloc::boxed::Box;
use alloc::rc::Rc;
use alloc::sync::Arc;
use alloc::vec::Vec;

use tinyvec::TinyVec;

use crate::array::tinyvec::TinyArray;
use crate::array::vec::Array;
use crate::array::INLINE_CAPACITY;

impl<T> From<Vec<T>> for TinyArray<T>
where
    T: Default,
{
    #[inline]
    fn from(values: Vec<T>) -> Self {
        Self(values.into_iter().collect())
    }
}

impl<T> From<TinyArray<T>> for Vec<T>
where
    T: Clone + Default,
{
    #[inline]
    fn from(values: TinyArray<T>) -> Self {
        values.into_vec()
    }
}

impl<'a, T> From<&'a [T]> for TinyArray<T>
where
    T: Clone + Default,
{
    #[inline]
    fn from(values: &'a [T]) -> Self {
        Self(TinyVec::from(values))
    }
}

impl<'a, T> From<&'a mut [T]> for TinyArray<T>
where
    T: Clone + Default,
{
    #[inline]
    fn from(values: &'a mut [T]) -> Self {
        Self(TinyVec::from(values))
    }
}

impl<T> From<Box<[T]>> for TinyArray<T>
where
    T: Default,
{
    #[inline]
    fn from(values: Box<[T]>) -> Self {
        Self(Vec::from(values).into_iter().collect())
    }
}

impl<T> From<TinyArray<T>> for Box<[T]>
where
    T: Clone + Default,
{
    #[inline]
    fn from(values: TinyArray<T>) -> Self {
        values.into_boxed_slice()
    }
}

impl<'a, T> From<Cow<'a, [T]>> for TinyArray<T>
where
    T: Clone + Default,
{
    #[inline]
    fn from(values: Cow<'a, [T]>) -> Self {
        match values {
            Cow::Borrowed(slice) => slice.into(),
            Cow::Owned(vec) => vec.into(),
        }
    }
}

impl<'a, T> From<TinyArray<T>> for Cow<'a, [T]>
where
    T: Clone + Default,
{
    #[inline]
    fn from(values: TinyArray<T>) -> Self {
        values.into_vec().into()
    }
}

impl<T> From<TinyArray<T>> for Rc<[T]>
where
    T: Clone + Default,
{
    #[inline]
    fn from(values: TinyArray<T>) -> Self {
        values.into_vec().into()
    }
}

impl<T> From<TinyArray<T>> for Arc<[T]>
where
    T: Clone + Default,
{
    #[inline]
    fn from(values: TinyArray<T>) -> Self {
        values.into_vec().into()
    }
}

impl<T, const N: usize> From<[T; N]> for TinyArray<T>
where
    T: Default,
{
    #[inline]
    fn from(values: [T; N]) -> Self {
        Self(values.into_iter().collect())
    }
}

impl<T, const N: usize> From<&[T; N]> for TinyArray<T>
where
    T: Clone + Default,
{
    #[inline]
    fn from(values: &[T; N]) -> Self {
        Self(TinyVec::from(&values[..]))
    }
}

impl<T> From<TinyVec<[T; INLINE_CAPACITY]>> for TinyArray<T>
where
    T: Default,
{
    #[inline]
    fn from(values: TinyVec<[T; INLINE_CAPACITY]>) -> Self {
        Self(values)
    }
}

impl<T> From<TinyArray<T>> for TinyVec<[T; INLINE_CAPACITY]>
where
    T: Default,
{
    #[inline]
    fn from(values: TinyArray<T>) -> Self {
        values.into_inner()
    }
}

impl<T> From<Array<T>> for TinyArray<T>
where
    T: Default,
{
    #[inline]
    fn from(values: Array<T>) -> Self {
        Self::from(values.into_vec())
    }
}

impl<T> FromIterator<T> for TinyArray<T>
where
    T: Default,
{
    #[inline]
    fn from_iter<I>(iter: I) -> Self
    where
        I: IntoIterator<Item = T>,
    {
        Self(iter.into_iter().collect())
    }
}

impl<'a, T> FromIterator<&'a T> for TinyArray<T>
where
    T: 'a + Clone + Default,
{
    #[inline]
    fn from_iter<I>(iter: I) -> Self
    where
        I: IntoIterator<Item = &'a T>,
    {
        Self(iter.into_iter().cloned().collect())
    }
}
