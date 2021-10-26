use alloc::borrow::Cow;
use alloc::boxed::Box;
use alloc::rc::Rc;
use alloc::sync::Arc;
use alloc::vec::Vec;

#[cfg(feature = "small-array")]
use smallvec::SmallVec;

use crate::array::vec::Array;

#[cfg(feature = "small-array")]
use crate::array::smallvec::SmallArray;
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

impl<T> From<[T; 0]> for Array<T> {
    #[inline]
    fn from(values: [T; 0]) -> Self {
        // Values is empty, so it can be ignored.
        let _ = values;
        Self::new()
    }
}

impl<T> From<&[T; 0]> for Array<T> {
    #[inline]
    fn from(values: &[T; 0]) -> Self {
        // Values is empty, so it can be ignored.
        let _ = values;
        Self::new()
    }
}

macro_rules! __array_T_from_primitive_array {
    ($len:expr) => {
        impl<T> From<[T; $len]> for Array<T> {
            #[inline]
            fn from(values: [T; $len]) -> Self {
                Self(Vec::from(values))
            }
        }

        impl<T> From<&[T; $len]> for Array<T>
        where
            T: Clone,
        {
            #[inline]
            fn from(values: &[T; $len]) -> Self {
                Self(values.to_vec())
            }
        }
    };
}

__array_T_from_primitive_array!(1);
__array_T_from_primitive_array!(2);
__array_T_from_primitive_array!(3);
__array_T_from_primitive_array!(4);
__array_T_from_primitive_array!(5);
__array_T_from_primitive_array!(6);
__array_T_from_primitive_array!(7);
__array_T_from_primitive_array!(8);
__array_T_from_primitive_array!(9);
__array_T_from_primitive_array!(10);
__array_T_from_primitive_array!(11);
__array_T_from_primitive_array!(12);
__array_T_from_primitive_array!(13);
__array_T_from_primitive_array!(14);
__array_T_from_primitive_array!(15);
__array_T_from_primitive_array!(16);
__array_T_from_primitive_array!(17);
__array_T_from_primitive_array!(18);
__array_T_from_primitive_array!(19);
__array_T_from_primitive_array!(20);
__array_T_from_primitive_array!(21);
__array_T_from_primitive_array!(22);
__array_T_from_primitive_array!(23);
__array_T_from_primitive_array!(24);
__array_T_from_primitive_array!(25);
__array_T_from_primitive_array!(26);
__array_T_from_primitive_array!(27);
__array_T_from_primitive_array!(28);
__array_T_from_primitive_array!(29);
__array_T_from_primitive_array!(30);
__array_T_from_primitive_array!(31);
__array_T_from_primitive_array!(32);

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
