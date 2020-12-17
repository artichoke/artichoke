use alloc::boxed::Box;
use alloc::vec::Vec;

use smallvec::SmallVec;

use crate::array::smallvec::SmallArray;
use crate::array::vec::Array;
use crate::array::INLINE_CAPACITY;

impl<T, U> PartialEq<SmallVec<[U; INLINE_CAPACITY]>> for SmallArray<T>
where
    T: PartialEq<U>,
{
    #[inline]
    fn eq(&self, other: &SmallVec<[U; INLINE_CAPACITY]>) -> bool {
        self[..] == other[..]
    }
}

impl<T, U> PartialEq<SmallArray<U>> for SmallVec<[T; INLINE_CAPACITY]>
where
    T: PartialEq<U>,
{
    #[inline]
    fn eq(&self, other: &SmallArray<U>) -> bool {
        self[..] == other[..]
    }
}

impl<T, U> PartialEq<Vec<U>> for SmallArray<T>
where
    T: PartialEq<U>,
{
    #[inline]
    fn eq(&self, other: &Vec<U>) -> bool {
        self[..] == other[..]
    }
}

impl<T, U> PartialEq<SmallArray<U>> for Vec<T>
where
    T: PartialEq<U>,
{
    #[inline]
    fn eq(&self, other: &SmallArray<U>) -> bool {
        self[..] == other[..]
    }
}

impl<T, U> PartialEq<[U]> for SmallArray<T>
where
    T: PartialEq<U>,
{
    #[inline]
    fn eq(&self, other: &[U]) -> bool {
        self[..] == other[..]
    }
}

impl<T, U> PartialEq<SmallArray<U>> for [T]
where
    T: PartialEq<U>,
{
    #[inline]
    fn eq(&self, other: &SmallArray<U>) -> bool {
        self[..] == other[..]
    }
}

impl<T, U> PartialEq<Box<[U]>> for SmallArray<T>
where
    T: PartialEq<U>,
{
    #[inline]
    fn eq(&self, other: &Box<[U]>) -> bool {
        self[..] == other[..]
    }
}

impl<T, U> PartialEq<SmallArray<U>> for Box<[T]>
where
    T: PartialEq<U>,
{
    #[inline]
    fn eq(&self, other: &SmallArray<U>) -> bool {
        self[..] == other[..]
    }
}

impl<T, U> PartialEq<SmallArray<U>> for Array<T>
where
    T: PartialEq<U>,
{
    #[inline]
    fn eq(&self, other: &SmallArray<U>) -> bool {
        self[..] == other[..]
    }
}

impl<T, U> PartialEq<Array<U>> for SmallArray<T>
where
    T: PartialEq<U>,
{
    #[inline]
    fn eq(&self, other: &Array<U>) -> bool {
        self[..] == other[..]
    }
}

macro_rules! __smallarray_T_eq_primitive_array {
    ($len:expr) => {
        impl<T, U> PartialEq<[U; $len]> for SmallArray<T>
        where
            T: PartialEq<U>,
        {
            #[inline]
            fn eq(&self, other: &[U; $len]) -> bool {
                self[..] == other[..]
            }
        }

        impl<T, U> PartialEq<SmallArray<U>> for [T; $len]
        where
            T: PartialEq<U>,
        {
            #[inline]
            fn eq(&self, other: &SmallArray<U>) -> bool {
                self[..] == other[..]
            }
        }

        impl<T, U> PartialEq<&[U; $len]> for SmallArray<T>
        where
            T: PartialEq<U>,
        {
            #[inline]
            fn eq(&self, other: &&[U; $len]) -> bool {
                self[..] == other[..]
            }
        }

        impl<T, U> PartialEq<SmallArray<U>> for &[T; $len]
        where
            T: PartialEq<U>,
        {
            #[inline]
            fn eq(&self, other: &SmallArray<U>) -> bool {
                self[..] == other[..]
            }
        }
    };
}

__smallarray_T_eq_primitive_array!(0);
__smallarray_T_eq_primitive_array!(1);
__smallarray_T_eq_primitive_array!(2);
__smallarray_T_eq_primitive_array!(3);
__smallarray_T_eq_primitive_array!(4);
__smallarray_T_eq_primitive_array!(5);
__smallarray_T_eq_primitive_array!(6);
__smallarray_T_eq_primitive_array!(7);
__smallarray_T_eq_primitive_array!(8);
__smallarray_T_eq_primitive_array!(9);
__smallarray_T_eq_primitive_array!(10);
__smallarray_T_eq_primitive_array!(11);
__smallarray_T_eq_primitive_array!(12);
__smallarray_T_eq_primitive_array!(13);
__smallarray_T_eq_primitive_array!(14);
__smallarray_T_eq_primitive_array!(15);
__smallarray_T_eq_primitive_array!(16);
__smallarray_T_eq_primitive_array!(17);
__smallarray_T_eq_primitive_array!(18);
__smallarray_T_eq_primitive_array!(19);
__smallarray_T_eq_primitive_array!(20);
__smallarray_T_eq_primitive_array!(21);
__smallarray_T_eq_primitive_array!(22);
__smallarray_T_eq_primitive_array!(23);
__smallarray_T_eq_primitive_array!(24);
__smallarray_T_eq_primitive_array!(25);
__smallarray_T_eq_primitive_array!(26);
__smallarray_T_eq_primitive_array!(27);
__smallarray_T_eq_primitive_array!(28);
__smallarray_T_eq_primitive_array!(29);
__smallarray_T_eq_primitive_array!(30);
__smallarray_T_eq_primitive_array!(31);
__smallarray_T_eq_primitive_array!(32);
