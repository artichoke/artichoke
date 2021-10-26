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

// The below hand-crafted `From` implementations for fixed-sized arrays length 8
// or shorter relies on the parameterized fixed array of `TinyArray`s
// underlying `TinyVec` being `[T; 8]` to avoid allocating (and making the
// hand rolled implementations worthwhile instead of delegating to
// `<[_]>::to_vec`.
const _: () = [()][!(INLINE_CAPACITY == 8) as usize];

impl<T> From<[T; 0]> for TinyArray<T>
where
    T: Default,
{
    #[inline]
    fn from(values: [T; 0]) -> Self {
        // Values is empty, so it can be ignored.
        let _ = values;
        Self::new()
    }
}

impl<T> From<&[T; 0]> for TinyArray<T>
where
    T: Default,
{
    #[inline]
    fn from(values: &[T; 0]) -> Self {
        // Values is empty, so it can be ignored.
        let _ = values;
        Self::new()
    }
}

impl<T> From<[T; 1]> for TinyArray<T>
where
    T: Default,
{
    #[inline]
    fn from(values: [T; 1]) -> Self {
        let [a] = values;
        let mut vec = TinyVec::new();
        vec.push(a);
        Self(vec)
    }
}

impl<T> From<&[T; 1]> for TinyArray<T>
where
    T: Clone + Default,
{
    #[inline]
    fn from(values: &[T; 1]) -> Self {
        let mut vec = TinyVec::new();
        for value in values.iter().cloned() {
            vec.push(value);
        }
        Self(vec)
    }
}

impl<T> From<[T; 2]> for TinyArray<T>
where
    T: Default,
{
    #[inline]
    fn from(values: [T; 2]) -> Self {
        let [a, b] = values;
        let mut vec = TinyVec::new();
        vec.push(a);
        vec.push(b);
        Self(vec)
    }
}

impl<T> From<&[T; 2]> for TinyArray<T>
where
    T: Clone + Default,
{
    #[inline]
    fn from(values: &[T; 2]) -> Self {
        let mut vec = TinyVec::new();
        for value in values.iter().cloned() {
            vec.push(value);
        }
        Self(vec)
    }
}

impl<T> From<[T; 3]> for TinyArray<T>
where
    T: Default,
{
    #[inline]
    fn from(values: [T; 3]) -> Self {
        let [a, b, c] = values;
        let mut vec = TinyVec::new();
        vec.push(a);
        vec.push(b);
        vec.push(c);
        Self(vec)
    }
}

impl<T> From<&[T; 3]> for TinyArray<T>
where
    T: Clone + Default,
{
    #[inline]
    fn from(values: &[T; 3]) -> Self {
        let mut vec = TinyVec::new();
        for value in values.iter().cloned() {
            vec.push(value);
        }
        Self(vec)
    }
}

impl<T> From<[T; 4]> for TinyArray<T>
where
    T: Default,
{
    #[inline]
    fn from(values: [T; 4]) -> Self {
        let [a, b, c, d] = values;
        let mut vec = TinyVec::new();
        vec.push(a);
        vec.push(b);
        vec.push(c);
        vec.push(d);
        Self(vec)
    }
}

impl<T> From<&[T; 4]> for TinyArray<T>
where
    T: Clone + Default,
{
    #[inline]
    fn from(values: &[T; 4]) -> Self {
        let mut vec = TinyVec::new();
        for value in values.iter().cloned() {
            vec.push(value);
        }
        Self(vec)
    }
}

impl<T> From<[T; 5]> for TinyArray<T>
where
    T: Default,
{
    #[inline]
    #[allow(clippy::many_single_char_names)]
    fn from(values: [T; 5]) -> Self {
        let [a, b, c, d, e] = values;
        let mut vec = TinyVec::new();
        vec.push(a);
        vec.push(b);
        vec.push(c);
        vec.push(d);
        vec.push(e);
        Self(vec)
    }
}

impl<T> From<&[T; 5]> for TinyArray<T>
where
    T: Clone + Default,
{
    #[inline]
    fn from(values: &[T; 5]) -> Self {
        let mut vec = TinyVec::new();
        for value in values.iter().cloned() {
            vec.push(value);
        }
        Self(vec)
    }
}

impl<T> From<[T; 6]> for TinyArray<T>
where
    T: Default,
{
    #[inline]
    #[allow(clippy::many_single_char_names)]
    fn from(values: [T; 6]) -> Self {
        let [a, b, c, d, e, f] = values;
        let mut vec = TinyVec::new();
        vec.push(a);
        vec.push(b);
        vec.push(c);
        vec.push(d);
        vec.push(e);
        vec.push(f);
        Self(vec)
    }
}

impl<T> From<&[T; 6]> for TinyArray<T>
where
    T: Clone + Default,
{
    #[inline]
    fn from(values: &[T; 6]) -> Self {
        let mut vec = TinyVec::new();
        for value in values.iter().cloned() {
            vec.push(value);
        }
        Self(vec)
    }
}

impl<T> From<[T; 7]> for TinyArray<T>
where
    T: Default,
{
    #[inline]
    #[allow(clippy::many_single_char_names)]
    fn from(values: [T; 7]) -> Self {
        let [a, b, c, d, e, f, g] = values;
        let mut vec = TinyVec::new();
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

impl<T> From<&[T; 7]> for TinyArray<T>
where
    T: Clone + Default,
{
    #[inline]
    fn from(values: &[T; 7]) -> Self {
        let mut vec = TinyVec::new();
        for value in values.iter().cloned() {
            vec.push(value);
        }
        Self(vec)
    }
}

impl<T> From<[T; INLINE_CAPACITY]> for TinyArray<T>
where
    T: Default,
{
    #[inline]
    fn from(values: [T; INLINE_CAPACITY]) -> Self {
        Self(TinyVec::from(values))
    }
}

impl<T> From<&[T; INLINE_CAPACITY]> for TinyArray<T>
where
    T: Clone + Default,
{
    #[inline]
    fn from(values: &[T; INLINE_CAPACITY]) -> Self {
        Self(TinyVec::from(&values[..]))
    }
}

macro_rules! __tinyarray_T_from_primitive_array {
    ($len:expr) => {
        impl<T> From<[T; $len]> for TinyArray<T>
        where
            T: Default,
        {
            #[inline]
            fn from(values: [T; $len]) -> Self {
                // TODO: use a by-value array iter once `min_const_generics`
                // stabilizes.
                Self(Vec::from(values).into_iter().collect())
            }
        }

        impl<T> From<&[T; $len]> for TinyArray<T>
        where
            T: Clone + Default,
        {
            #[inline]
            fn from(values: &[T; $len]) -> Self {
                Self(TinyVec::from(&values[..]))
            }
        }
    };
}

// Skip to avoid a vec allocation because the lengths are less than
// `INLINE_CAPACITY`.
// __tinyarray_T_from_primitive_array!(1);
// __tinyarray_T_from_primitive_array!(2);
// __tinyarray_T_from_primitive_array!(3);
// __tinyarray_T_from_primitive_array!(4);
// __tinyarray_T_from_primitive_array!(5);
// __tinyarray_T_from_primitive_array!(6);
// __tinyarray_T_from_primitive_array!(7);
// Skip because we have manually implemented for `INLINE_CAPACITY` arrays.
// __tinyarray_T_from_primitive_array!(8);
__tinyarray_T_from_primitive_array!(9);
__tinyarray_T_from_primitive_array!(10);
__tinyarray_T_from_primitive_array!(11);
__tinyarray_T_from_primitive_array!(12);
__tinyarray_T_from_primitive_array!(13);
__tinyarray_T_from_primitive_array!(14);
__tinyarray_T_from_primitive_array!(15);
__tinyarray_T_from_primitive_array!(16);
__tinyarray_T_from_primitive_array!(17);
__tinyarray_T_from_primitive_array!(18);
__tinyarray_T_from_primitive_array!(19);
__tinyarray_T_from_primitive_array!(20);
__tinyarray_T_from_primitive_array!(21);
__tinyarray_T_from_primitive_array!(22);
__tinyarray_T_from_primitive_array!(23);
__tinyarray_T_from_primitive_array!(24);
__tinyarray_T_from_primitive_array!(25);
__tinyarray_T_from_primitive_array!(26);
__tinyarray_T_from_primitive_array!(27);
__tinyarray_T_from_primitive_array!(28);
__tinyarray_T_from_primitive_array!(29);
__tinyarray_T_from_primitive_array!(30);
__tinyarray_T_from_primitive_array!(31);
__tinyarray_T_from_primitive_array!(32);

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
