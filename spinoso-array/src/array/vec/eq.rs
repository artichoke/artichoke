use alloc::boxed::Box;
use alloc::vec::Vec;

use crate::array::vec::Array;

impl<T, U> PartialEq<Vec<U>> for Array<T>
where
    T: PartialEq<U>,
{
    #[inline]
    fn eq(&self, other: &Vec<U>) -> bool {
        self[..] == other[..]
    }
}

impl<T, U> PartialEq<Array<U>> for Vec<T>
where
    T: PartialEq<U>,
{
    #[inline]
    fn eq(&self, other: &Array<U>) -> bool {
        self[..] == other[..]
    }
}

impl<T, U> PartialEq<[U]> for Array<T>
where
    T: PartialEq<U>,
{
    #[inline]
    fn eq(&self, other: &[U]) -> bool {
        self[..] == other[..]
    }
}

impl<T, U> PartialEq<Array<U>> for [T]
where
    T: PartialEq<U>,
{
    #[inline]
    fn eq(&self, other: &Array<U>) -> bool {
        self[..] == other[..]
    }
}

impl<T, U> PartialEq<Box<[U]>> for Array<T>
where
    T: PartialEq<U>,
{
    #[inline]
    fn eq(&self, other: &Box<[U]>) -> bool {
        self[..] == other[..]
    }
}

impl<T, U> PartialEq<Array<U>> for Box<[T]>
where
    T: PartialEq<U>,
{
    #[inline]
    fn eq(&self, other: &Array<U>) -> bool {
        self[..] == other[..]
    }
}

macro_rules! __array_T_eq_primitive_array {
    ($len:expr) => {
        impl<T, U> PartialEq<[U; $len]> for Array<T>
        where
            T: PartialEq<U>,
        {
            #[inline]
            fn eq(&self, other: &[U; $len]) -> bool {
                self[..] == other[..]
            }
        }

        impl<T, U> PartialEq<Array<U>> for [T; $len]
        where
            T: PartialEq<U>,
        {
            #[inline]
            fn eq(&self, other: &Array<U>) -> bool {
                self[..] == other[..]
            }
        }

        impl<T, U> PartialEq<&[U; $len]> for Array<T>
        where
            T: PartialEq<U>,
        {
            #[inline]
            fn eq(&self, other: &&[U; $len]) -> bool {
                self[..] == other[..]
            }
        }

        impl<T, U> PartialEq<Array<U>> for &[T; $len]
        where
            T: PartialEq<U>,
        {
            #[inline]
            fn eq(&self, other: &Array<U>) -> bool {
                self[..] == other[..]
            }
        }
    };
}

__array_T_eq_primitive_array!(0);
__array_T_eq_primitive_array!(1);
__array_T_eq_primitive_array!(2);
__array_T_eq_primitive_array!(3);
__array_T_eq_primitive_array!(4);
__array_T_eq_primitive_array!(5);
__array_T_eq_primitive_array!(6);
__array_T_eq_primitive_array!(7);
__array_T_eq_primitive_array!(8);
__array_T_eq_primitive_array!(9);
__array_T_eq_primitive_array!(10);
__array_T_eq_primitive_array!(11);
__array_T_eq_primitive_array!(12);
__array_T_eq_primitive_array!(13);
__array_T_eq_primitive_array!(14);
__array_T_eq_primitive_array!(15);
__array_T_eq_primitive_array!(16);
__array_T_eq_primitive_array!(17);
__array_T_eq_primitive_array!(18);
__array_T_eq_primitive_array!(19);
__array_T_eq_primitive_array!(20);
__array_T_eq_primitive_array!(21);
__array_T_eq_primitive_array!(22);
__array_T_eq_primitive_array!(23);
__array_T_eq_primitive_array!(24);
__array_T_eq_primitive_array!(25);
__array_T_eq_primitive_array!(26);
__array_T_eq_primitive_array!(27);
__array_T_eq_primitive_array!(28);
__array_T_eq_primitive_array!(29);
__array_T_eq_primitive_array!(30);
__array_T_eq_primitive_array!(31);
__array_T_eq_primitive_array!(32);
