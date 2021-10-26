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

// The below hand-crafted `From` implementations for fixed-sized arrays length 8
// or shorter relies on the parameterized fixed array of `SmallArray`s
// underlying `SmallVec` being `[T; 8]` to avoid allocating (and making the
// hand rolled implementations worthwhile instead of delegating to
// `<[_]>::to_vec`.
const _: () = [()][!(INLINE_CAPACITY == 8) as usize];

impl<T> From<[T; 0]> for SmallArray<T> {
    #[inline]
    fn from(values: [T; 0]) -> Self {
        // Values is empty, so it can be ignored.
        let _ = values;
        Self::new()
    }
}

impl<T> From<&[T; 0]> for SmallArray<T> {
    #[inline]
    fn from(values: &[T; 0]) -> Self {
        // Values is empty, so it can be ignored.
        let _ = values;
        Self::new()
    }
}

impl<T> From<[T; 1]> for SmallArray<T> {
    #[inline]
    fn from(values: [T; 1]) -> Self {
        let [a] = values;
        let mut vec = SmallVec::new();
        vec.push(a);
        Self(vec)
    }
}

impl<T> From<&[T; 1]> for SmallArray<T>
where
    T: Copy,
{
    #[inline]
    fn from(values: &[T; 1]) -> Self {
        let mut vec = SmallVec::new();
        for value in values.iter().copied() {
            vec.push(value);
        }
        Self(vec)
    }
}

impl<T> From<[T; 2]> for SmallArray<T> {
    #[inline]
    fn from(values: [T; 2]) -> Self {
        let [a, b] = values;
        let mut vec = SmallVec::new();
        vec.push(a);
        vec.push(b);
        Self(vec)
    }
}

impl<T> From<&[T; 2]> for SmallArray<T>
where
    T: Copy,
{
    #[inline]
    fn from(values: &[T; 2]) -> Self {
        let mut vec = SmallVec::new();
        for value in values.iter().copied() {
            vec.push(value);
        }
        Self(vec)
    }
}

impl<T> From<[T; 3]> for SmallArray<T> {
    #[inline]
    fn from(values: [T; 3]) -> Self {
        let [a, b, c] = values;
        let mut vec = SmallVec::new();
        vec.push(a);
        vec.push(b);
        vec.push(c);
        Self(vec)
    }
}

impl<T> From<&[T; 3]> for SmallArray<T>
where
    T: Copy,
{
    #[inline]
    fn from(values: &[T; 3]) -> Self {
        let mut vec = SmallVec::new();
        for value in values.iter().copied() {
            vec.push(value);
        }
        Self(vec)
    }
}

impl<T> From<[T; 4]> for SmallArray<T> {
    #[inline]
    fn from(values: [T; 4]) -> Self {
        let [a, b, c, d] = values;
        let mut vec = SmallVec::new();
        vec.push(a);
        vec.push(b);
        vec.push(c);
        vec.push(d);
        Self(vec)
    }
}

impl<T> From<&[T; 4]> for SmallArray<T>
where
    T: Copy,
{
    #[inline]
    fn from(values: &[T; 4]) -> Self {
        let mut vec = SmallVec::new();
        for value in values.iter().copied() {
            vec.push(value);
        }
        Self(vec)
    }
}

impl<T> From<[T; 5]> for SmallArray<T> {
    #[inline]
    #[allow(clippy::many_single_char_names)]
    fn from(values: [T; 5]) -> Self {
        let [a, b, c, d, e] = values;
        let mut vec = SmallVec::new();
        vec.push(a);
        vec.push(b);
        vec.push(c);
        vec.push(d);
        vec.push(e);
        Self(vec)
    }
}

impl<T> From<&[T; 5]> for SmallArray<T>
where
    T: Copy,
{
    #[inline]
    fn from(values: &[T; 5]) -> Self {
        let mut vec = SmallVec::new();
        for value in values.iter().copied() {
            vec.push(value);
        }
        Self(vec)
    }
}

impl<T> From<[T; 6]> for SmallArray<T> {
    #[inline]
    #[allow(clippy::many_single_char_names)]
    fn from(values: [T; 6]) -> Self {
        let [a, b, c, d, e, f] = values;
        let mut vec = SmallVec::new();
        vec.push(a);
        vec.push(b);
        vec.push(c);
        vec.push(d);
        vec.push(e);
        vec.push(f);
        Self(vec)
    }
}

impl<T> From<&[T; 6]> for SmallArray<T>
where
    T: Copy,
{
    #[inline]
    fn from(values: &[T; 6]) -> Self {
        let mut vec = SmallVec::new();
        for value in values.iter().copied() {
            vec.push(value);
        }
        Self(vec)
    }
}

impl<T> From<[T; 7]> for SmallArray<T> {
    #[inline]
    #[allow(clippy::many_single_char_names)]
    fn from(values: [T; 7]) -> Self {
        let [a, b, c, d, e, f, g] = values;
        let mut vec = SmallVec::new();
        vec.push(a);
        vec.push(b);
        vec.push(c);
        vec.push(d);
        vec.push(e);
        vec.push(f);
        vec.push(g);
        Self(vec)
    }
}

impl<T> From<&[T; 7]> for SmallArray<T>
where
    T: Copy,
{
    #[inline]
    fn from(values: &[T; 7]) -> Self {
        let mut vec = SmallVec::new();
        for value in values.iter().copied() {
            vec.push(value);
        }
        Self(vec)
    }
}

impl<T> From<[T; INLINE_CAPACITY]> for SmallArray<T> {
    #[inline]
    fn from(values: [T; INLINE_CAPACITY]) -> Self {
        Self(SmallVec::from(values))
    }
}

impl<T> From<&[T; INLINE_CAPACITY]> for SmallArray<T>
where
    T: Copy,
{
    #[inline]
    fn from(values: &[T; INLINE_CAPACITY]) -> Self {
        Self(SmallVec::from(*values))
    }
}

macro_rules! __smallarray_T_from_primitive_array {
    ($len:expr) => {
        impl<T> From<[T; $len]> for SmallArray<T> {
            #[inline]
            fn from(values: [T; $len]) -> Self {
                Self(SmallVec::from_vec(Vec::from(values)))
            }
        }

        impl<T> From<&[T; $len]> for SmallArray<T>
        where
            T: Copy,
        {
            #[inline]
            fn from(values: &[T; $len]) -> Self {
                Self(SmallVec::from_slice(values))
            }
        }
    };
}

// Skip to avoid a vec allocation because the lengths are less than
// `INLINE_CAPACITY`.
// __smallarray_T_from_primitive_array!(1);
// __smallarray_T_from_primitive_array!(2);
// __smallarray_T_from_primitive_array!(3);
// __smallarray_T_from_primitive_array!(4);
// __smallarray_T_from_primitive_array!(5);
// __smallarray_T_from_primitive_array!(6);
// __smallarray_T_from_primitive_array!(7);
// Skip because we have manually implemented for `INLINE_CAPACITY` arrays.
// __smallarray_T_from_primitive_array!(8);
__smallarray_T_from_primitive_array!(9);
__smallarray_T_from_primitive_array!(10);
__smallarray_T_from_primitive_array!(11);
__smallarray_T_from_primitive_array!(12);
__smallarray_T_from_primitive_array!(13);
__smallarray_T_from_primitive_array!(14);
__smallarray_T_from_primitive_array!(15);
__smallarray_T_from_primitive_array!(16);
__smallarray_T_from_primitive_array!(17);
__smallarray_T_from_primitive_array!(18);
__smallarray_T_from_primitive_array!(19);
__smallarray_T_from_primitive_array!(20);
__smallarray_T_from_primitive_array!(21);
__smallarray_T_from_primitive_array!(22);
__smallarray_T_from_primitive_array!(23);
__smallarray_T_from_primitive_array!(24);
__smallarray_T_from_primitive_array!(25);
__smallarray_T_from_primitive_array!(26);
__smallarray_T_from_primitive_array!(27);
__smallarray_T_from_primitive_array!(28);
__smallarray_T_from_primitive_array!(29);
__smallarray_T_from_primitive_array!(30);
__smallarray_T_from_primitive_array!(31);
__smallarray_T_from_primitive_array!(32);

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
