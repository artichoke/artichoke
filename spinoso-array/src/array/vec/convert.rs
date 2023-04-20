use alloc::borrow::Cow;
use alloc::boxed::Box;
use alloc::rc::Rc;
use alloc::sync::Arc;
use alloc::vec::Vec;

#[cfg(feature = "small-array")]
use smallvec::SmallVec;

#[cfg(feature = "small-array")]
use crate::array::smallvec::SmallArray;
use crate::array::vec::Array;
#[cfg(feature = "small-array")]
use crate::array::INLINE_CAPACITY;

impl<T> From<Vec<T>> for Array<T> {
    #[inline]
    fn from(values: Vec<T>) -> Self {
        Self(values)
    }
}

impl<T> From<Array<T>> for Vec<T> {
    #[inline]
    fn from(values: Array<T>) -> Self {
        values.into_vec()
    }
}

impl<'a, T> From<&'a [T]> for Array<T>
where
    T: Clone,
{
    #[inline]
    fn from(values: &'a [T]) -> Self {
        Self(values.to_vec())
    }
}

impl<'a, T> From<&'a mut [T]> for Array<T>
where
    T: Clone,
{
    #[inline]
    fn from(values: &'a mut [T]) -> Self {
        Self(values.to_vec())
    }
}

impl<T> From<Box<[T]>> for Array<T> {
    #[inline]
    fn from(values: Box<[T]>) -> Self {
        Self(Vec::from(values))
    }
}

impl<T> From<Array<T>> for Box<[T]> {
    #[inline]
    fn from(values: Array<T>) -> Self {
        values.into_boxed_slice()
    }
}

impl<'a, T> From<Cow<'a, [T]>> for Array<T>
where
    T: Clone,
{
    #[inline]
    fn from(values: Cow<'a, [T]>) -> Self {
        match values {
            Cow::Borrowed(slice) => slice.into(),
            Cow::Owned(vec) => vec.into(),
        }
    }
}

impl<'a, T> From<Array<T>> for Cow<'a, [T]>
where
    T: Clone,
{
    #[inline]
    fn from(values: Array<T>) -> Self {
        values.into_vec().into()
    }
}

impl<T> From<Array<T>> for Rc<[T]> {
    #[inline]
    fn from(values: Array<T>) -> Self {
        values.into_vec().into()
    }
}

impl<T> From<Array<T>> for Arc<[T]> {
    #[inline]
    fn from(values: Array<T>) -> Self {
        values.into_vec().into()
    }
}

impl<T, const N: usize> From<[T; N]> for Array<T> {
    #[inline]
    fn from(values: [T; N]) -> Self {
        Self(values.into())
    }
}

impl<T, const N: usize> From<&[T; N]> for Array<T>
where
    T: Clone,
{
    #[inline]
    fn from(values: &[T; N]) -> Self {
        Self(values.to_vec())
    }
}

#[cfg(feature = "small-array")]
impl<T> From<SmallVec<[T; INLINE_CAPACITY]>> for Array<T> {
    #[inline]
    fn from(values: SmallVec<[T; INLINE_CAPACITY]>) -> Self {
        Self(values.into_vec())
    }
}

#[cfg(feature = "small-array")]
impl<T> From<Array<T>> for SmallVec<[T; INLINE_CAPACITY]> {
    #[inline]
    fn from(values: Array<T>) -> Self {
        SmallVec::from_vec(values.into_vec())
    }
}

#[cfg(feature = "small-array")]
impl<T> From<SmallArray<T>> for Array<T> {
    #[inline]
    fn from(values: SmallArray<T>) -> Self {
        Self::from(values.into_vec())
    }
}

impl<T> FromIterator<T> for Array<T> {
    #[inline]
    fn from_iter<I>(iter: I) -> Self
    where
        I: IntoIterator<Item = T>,
    {
        Self(iter.into_iter().collect())
    }
}

impl<'a, T> FromIterator<&'a T> for Array<T>
where
    T: 'a + Clone,
{
    #[inline]
    fn from_iter<I>(iter: I) -> Self
    where
        I: IntoIterator<Item = &'a T>,
    {
        Self(iter.into_iter().cloned().collect())
    }
}
