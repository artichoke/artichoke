use alloc::boxed::Box;
use alloc::vec::Vec;

use tinyvec::TinyVec;

use crate::array::tinyvec::TinyArray;
use crate::array::vec::Array;
use crate::array::INLINE_CAPACITY;

impl<T, U> PartialEq<TinyVec<[U; INLINE_CAPACITY]>> for TinyArray<T>
where
    T: PartialEq<U> + Default,
    U: Default,
{
    #[inline]
    fn eq(&self, other: &TinyVec<[U; INLINE_CAPACITY]>) -> bool {
        self[..] == other[..]
    }
}

impl<T, U> PartialEq<TinyArray<U>> for TinyVec<[T; INLINE_CAPACITY]>
where
    T: PartialEq<U> + Default,
    U: Default,
{
    #[inline]
    fn eq(&self, other: &TinyArray<U>) -> bool {
        self[..] == other[..]
    }
}

impl<T, U> PartialEq<Vec<U>> for TinyArray<T>
where
    T: PartialEq<U> + Default,
{
    #[inline]
    fn eq(&self, other: &Vec<U>) -> bool {
        self[..] == other[..]
    }
}

impl<T, U> PartialEq<TinyArray<U>> for Vec<T>
where
    T: PartialEq<U>,
    U: Default,
{
    #[inline]
    fn eq(&self, other: &TinyArray<U>) -> bool {
        self[..] == other[..]
    }
}

impl<T, U> PartialEq<[U]> for TinyArray<T>
where
    T: PartialEq<U> + Default,
{
    #[inline]
    fn eq(&self, other: &[U]) -> bool {
        self[..] == other[..]
    }
}

impl<T, U> PartialEq<TinyArray<U>> for [T]
where
    T: PartialEq<U>,
    U: Default,
{
    #[inline]
    fn eq(&self, other: &TinyArray<U>) -> bool {
        self[..] == other[..]
    }
}

impl<T, U> PartialEq<Box<[U]>> for TinyArray<T>
where
    T: PartialEq<U> + Default,
{
    #[inline]
    fn eq(&self, other: &Box<[U]>) -> bool {
        self[..] == other[..]
    }
}

impl<T, U> PartialEq<TinyArray<U>> for Box<[T]>
where
    T: PartialEq<U>,
    U: Default,
{
    #[inline]
    fn eq(&self, other: &TinyArray<U>) -> bool {
        self[..] == other[..]
    }
}

impl<T, U> PartialEq<TinyArray<U>> for Array<T>
where
    T: PartialEq<U>,
    U: Default,
{
    #[inline]
    fn eq(&self, other: &TinyArray<U>) -> bool {
        self[..] == other[..]
    }
}

impl<T, U> PartialEq<Array<U>> for TinyArray<T>
where
    T: PartialEq<U> + Default,
{
    #[inline]
    fn eq(&self, other: &Array<U>) -> bool {
        self[..] == other[..]
    }
}

macro_rules! __tinyarray_T_eq_primitive_array {
    ($len:expr) => {
        impl<T, U> PartialEq<[U; $len]> for TinyArray<T>
        where
            T: PartialEq<U> + Default,
        {
            #[inline]
            fn eq(&self, other: &[U; $len]) -> bool {
                self[..] == other[..]
            }
        }

        impl<T, U> PartialEq<TinyArray<U>> for [T; $len]
        where
            T: PartialEq<U>,
            U: Default,
        {
            #[inline]
            fn eq(&self, other: &TinyArray<U>) -> bool {
                self[..] == other[..]
            }
        }

        impl<T, U> PartialEq<&[U; $len]> for TinyArray<T>
        where
            T: PartialEq<U> + Default,
        {
            #[inline]
            fn eq(&self, other: &&[U; $len]) -> bool {
                self[..] == other[..]
            }
        }

        impl<T, U> PartialEq<TinyArray<U>> for &[T; $len]
        where
            T: PartialEq<U>,
            U: Default,
        {
            #[inline]
            fn eq(&self, other: &TinyArray<U>) -> bool {
                self[..] == other[..]
            }
        }
    };
}

__tinyarray_T_eq_primitive_array!(0);
__tinyarray_T_eq_primitive_array!(1);
__tinyarray_T_eq_primitive_array!(2);
__tinyarray_T_eq_primitive_array!(3);
__tinyarray_T_eq_primitive_array!(4);
__tinyarray_T_eq_primitive_array!(5);
__tinyarray_T_eq_primitive_array!(6);
__tinyarray_T_eq_primitive_array!(7);
__tinyarray_T_eq_primitive_array!(8);
__tinyarray_T_eq_primitive_array!(9);
__tinyarray_T_eq_primitive_array!(10);
__tinyarray_T_eq_primitive_array!(11);
__tinyarray_T_eq_primitive_array!(12);
__tinyarray_T_eq_primitive_array!(13);
__tinyarray_T_eq_primitive_array!(14);
__tinyarray_T_eq_primitive_array!(15);
__tinyarray_T_eq_primitive_array!(16);
__tinyarray_T_eq_primitive_array!(17);
__tinyarray_T_eq_primitive_array!(18);
__tinyarray_T_eq_primitive_array!(19);
__tinyarray_T_eq_primitive_array!(20);
__tinyarray_T_eq_primitive_array!(21);
__tinyarray_T_eq_primitive_array!(22);
__tinyarray_T_eq_primitive_array!(23);
__tinyarray_T_eq_primitive_array!(24);
__tinyarray_T_eq_primitive_array!(25);
__tinyarray_T_eq_primitive_array!(26);
__tinyarray_T_eq_primitive_array!(27);
__tinyarray_T_eq_primitive_array!(28);
__tinyarray_T_eq_primitive_array!(29);
__tinyarray_T_eq_primitive_array!(30);
__tinyarray_T_eq_primitive_array!(31);
__tinyarray_T_eq_primitive_array!(32);
