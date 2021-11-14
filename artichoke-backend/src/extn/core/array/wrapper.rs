use core::slice;

#[doc(inline)]
pub use spinoso_array::RawParts;

use crate::sys;
use crate::value::Value;

/// A contiguous growable array type based on [`Vec<sys::mrb_value>`](Vec) that
/// implements the [Ruby `Array`][ruby-array] API for `artichoke-backend` and
/// `mruby`.
///
/// `Array` implements indexing and mutating APIs that make an ideal backend for
/// the [Ruby `Array` core class][ruby-array]. In practice, this results in less
/// generic, more single-use APIs. For example, instead of [`Vec::drain`],
/// `Array` implements [`shift`], [`shift_n`], [`pop`], and [`pop_n`].
///
/// Similarly, slicing APIs are more specialized, such as [`first_n`] and
/// [`last_n`]. Slicing APIs do not return [`Option`], instead preferring to
/// return an empty slice.
///
///
/// `Array` implements [`BoxUnboxVmValue`] which enables it to be serialized to
/// a mruby value and unboxed to the Rust `Array` type.
///
/// [ruby-array]: https://ruby-doc.org/core-2.6.3/Array.html
/// [`shift`]: Array::shift
/// [`shift_n`]: Array::shift_n
/// [`drop_n`]: Array::drop_n
/// [`pop`]: Array::pop
/// [`pop_n`]: Array::pop_n
/// [`first_n`]: Array::first_n
/// [`last_n`]: Array::last_n
/// [`BoxUnboxVmValue`]: crate::convert::BoxUnboxVmValue
#[derive(Debug, Clone)]
pub struct Array(spinoso_array::Array<sys::mrb_value>);

impl Default for Array {
    #[inline]
    fn default() -> Self {
        Self::new()
    }
}

impl From<spinoso_array::Array<sys::mrb_value>> for Array {
    fn from(buffer: spinoso_array::Array<sys::mrb_value>) -> Self {
        Self(buffer)
    }
}

impl From<Vec<sys::mrb_value>> for Array {
    fn from(values: Vec<sys::mrb_value>) -> Self {
        Self(values.into())
    }
}

impl From<Vec<Value>> for Array {
    fn from(values: Vec<Value>) -> Self {
        Self(values.iter().map(Value::inner).collect())
    }
}

impl<'a> From<&'a [sys::mrb_value]> for Array {
    fn from(values: &'a [sys::mrb_value]) -> Self {
        Self(values.into())
    }
}

impl<'a> From<&'a [Value]> for Array {
    fn from(values: &'a [Value]) -> Self {
        Self(values.iter().map(Value::inner).collect())
    }
}

impl FromIterator<sys::mrb_value> for Array {
    fn from_iter<I>(iter: I) -> Self
    where
        I: IntoIterator<Item = sys::mrb_value>,
    {
        Self(iter.into_iter().collect())
    }
}

impl FromIterator<Value> for Array {
    fn from_iter<I>(iter: I) -> Self
    where
        I: IntoIterator<Item = Value>,
    {
        Self(iter.into_iter().map(|value| value.inner()).collect())
    }
}

impl FromIterator<Option<Value>> for Array {
    fn from_iter<I>(iter: I) -> Self
    where
        I: IntoIterator<Item = Option<Value>>,
    {
        let array = iter
            .into_iter()
            .map(|value| value.unwrap_or_default().inner())
            .collect();
        Self(array)
    }
}

impl<'a> FromIterator<&'a Option<Value>> for Array {
    fn from_iter<I>(iter: I) -> Self
    where
        I: IntoIterator<Item = &'a Option<Value>>,
    {
        let array = iter
            .into_iter()
            .map(|value| value.unwrap_or_default().inner())
            .collect();
        Self(array)
    }
}

#[derive(Debug)]
pub struct Iter<'a>(slice::Iter<'a, sys::mrb_value>);

impl<'a> Iterator for Iter<'a> {
    type Item = Value;

    fn next(&mut self) -> Option<Self::Item> {
        self.0.next().copied().map(Value::from)
    }
}

impl<'a> IntoIterator for &'a Array {
    type Item = Value;
    type IntoIter = Iter<'a>;

    fn into_iter(self) -> Self::IntoIter {
        Iter(self.0.iter())
    }
}

impl Extend<sys::mrb_value> for Array {
    fn extend<T>(&mut self, iter: T)
    where
        T: IntoIterator<Item = sys::mrb_value>,
    {
        self.0.extend(iter.into_iter());
    }
}

impl Extend<Value> for Array {
    fn extend<T>(&mut self, iter: T)
    where
        T: IntoIterator<Item = Value>,
    {
        self.0.extend(iter.into_iter().map(|value| value.inner()));
    }
}

impl Array {
    /// Construct a new, empty `Array`.
    ///
    /// The vector will not allocate until elements are pushed into it.
    #[inline]
    #[must_use]
    pub const fn new() -> Self {
        Self(spinoso_array::Array::new())
    }

    /// Construct a new, empty `Array` with the specified capacity.
    ///
    /// The vector will be able to hold exactly `capacity` elements without
    /// reallocating. If `capacity` is 0, the vector will not allocate.
    ///
    /// It is important to note that although the returned vector has the
    /// _capacity_ specified, the vector will have a zero _length_.
    #[inline]
    #[must_use]
    pub fn with_capacity(capacity: usize) -> Self {
        Self(spinoso_array::Array::with_capacity(capacity))
    }

    /// Construct a new two-element `Array` from the given arguments.
    ///
    /// The vector is constructed with `capacity` of 2.
    #[inline]
    #[must_use]
    pub fn assoc(first: Value, second: Value) -> Self {
        Self(spinoso_array::Array::assoc(first.inner(), second.inner()))
    }

    /// Returns an iterator over the slice.
    #[inline]
    #[must_use]
    pub fn iter(&self) -> Iter<'_> {
        self.into_iter()
    }

    /// Extracts a slice containing the entire vector.
    ///
    /// Equivalent to `&ary[..]`.
    #[inline]
    #[must_use]
    pub fn as_slice(&self) -> &[sys::mrb_value] {
        self.0.as_slice()
    }

    /// Extracts a mutable slice containing the entire vector.
    ///
    /// Equivalent to `&mut ary[..]`.
    #[inline]
    #[must_use]
    pub fn as_mut_slice(&mut self) -> &mut [sys::mrb_value] {
        self.0.as_mut_slice()
    }

    /// Returns a raw pointer to the vector's buffer.
    ///
    /// The caller must ensure that the vector outlives the pointer this
    /// function returns, or else it will end up pointing to garbage. Modifying
    /// the vector may cause its buffer to be reallocated, which would also make
    /// any pointers to it invalid.
    ///
    /// The caller must also ensure that the memory the pointer
    /// (non-transitively) points to is never written to (except inside an
    /// `UnsafeCell`) using this pointer or any pointer derived from it. If you
    /// need to mutate the contents of the slice, use
    /// [`as_mut_ptr`](Self::as_mut_ptr).
    #[inline]
    #[must_use]
    pub fn as_ptr(&self) -> *const sys::mrb_value {
        self.0.as_ptr()
    }

    /// Returns an unsafe mutable pointer to the vector's buffer.
    ///
    /// The caller must ensure that the vector outlives the pointer this
    /// function returns, or else it will end up pointing to garbage.
    /// Modifying the vector may cause its buffer to be reallocated, which would
    /// also make any pointers to it invalid.
    ///
    /// # Examples
    ///
    /// This method is primarily used when mutating a `Array` via a raw pointer
    /// passed over FFI.
    ///
    /// See the [`ARY_PTR`] macro in mruby.
    ///
    /// [`ARY_PTR`]: https://github.com/artichoke/mruby/blob/d66440864d08f1c3ac5820d45f11df031b7d43c6/include/mruby/array.h#L52
    #[inline]
    #[must_use]
    pub fn as_mut_ptr(&mut self) -> *mut sys::mrb_value {
        self.0.as_mut_ptr()
    }

    /// Set the vector's length without dropping or moving out elements
    ///
    /// This method is unsafe because it changes the notion of the number of
    /// "valid" elements in the vector. Use with care.
    ///
    /// # Safety
    ///
    /// - `new_len` must be less than or equal to capacity().
    /// - The elements at `old_len..new_len` must be initialized.
    ///
    /// # Examples
    ///
    /// This method is primarily used when mutating a `Array` via a raw pointer
    /// passed over FFI.
    ///
    /// See the [`ARY_PTR`] macro in mruby.
    ///
    /// [`ARY_PTR`]: https://github.com/artichoke/mruby/blob/d66440864d08f1c3ac5820d45f11df031b7d43c6/include/mruby/array.h#L52
    #[inline]
    pub unsafe fn set_len(&mut self, new_len: usize) {
        self.0.set_len(new_len);
    }

    /// Creates an `Array` directly from the raw components of another array.
    ///
    /// # Safety
    ///
    /// This is highly unsafe, due to the number of invariants that aren't
    /// checked:
    ///
    /// - `ptr` needs to have been previously allocated via `Array<T>` (at
    ///   least, it's highly likely to be incorrect if it wasn't).
    /// - `T` needs to have the same size and alignment as what `ptr` was
    ///   allocated with. (`T` having a less strict alignment is not sufficient,
    ///   the alignment really needs to be equal to satisfy the `dealloc`
    ///   requirement that memory must be allocated and deallocated with the
    ///   same layout.)
    /// - `length` needs to be less than or equal to `capacity`.
    /// - `capacity` needs to be the `capacity` that the pointer was allocated
    ///   with.
    ///
    /// Violating these may cause problems like corrupting the allocator's
    /// internal data structures.
    ///
    /// The ownership of `ptr` is effectively transferred to the `Array<T>`
    /// which may then deallocate, reallocate or change the contents of memory
    /// pointed to by the pointer at will. Ensure that nothing else uses the
    /// pointer after calling this function.
    #[must_use]
    pub unsafe fn from_raw_parts(raw_parts: RawParts<sys::mrb_value>) -> Self {
        let array = spinoso_array::Array::from_raw_parts(raw_parts);
        Self(array)
    }

    /// Decomposes an `Array<T>` into its raw components.
    ///
    /// Returns the raw pointer to the underlying data, the length of the array
    /// (in elements), and the allocated capacity of the data (in elements).
    ///
    /// After calling this function, the caller is responsible for the memory
    /// previously managed by the `Array`. The only way to do this is to convert
    /// the raw pointer, length, and capacity back into a `Array` with the
    /// [`from_raw_parts`] function, allowing the destructor to perform the
    /// cleanup.
    ///
    /// [`from_raw_parts`]: Array::from_raw_parts
    #[must_use]
    pub fn into_raw_parts(self) -> RawParts<sys::mrb_value> {
        self.0.into_raw_parts()
    }

    /// Consume the array and return its elements as a [`Vec<T>`].
    ///
    /// For `Array`, this is a cheap operation that unwraps the inner `Vec`.
    ///
    /// [`Vec<T>`]: std::vec::Vec
    #[inline]
    #[must_use]
    pub fn into_vec(self) -> Vec<sys::mrb_value> {
        self.0.into_vec()
    }

    /// Converts the vector into [`Box<[T]>`](Box).
    ///
    /// This will drop any excess capacity.
    #[inline]
    #[must_use]
    pub fn into_boxed_slice(self) -> Box<[sys::mrb_value]> {
        self.0.into_boxed_slice()
    }

    /// Returns the number of elements the vector can hold without reallocating.
    #[inline]
    #[must_use]
    pub fn capacity(&self) -> usize {
        self.0.capacity()
    }

    /// Reserves capacity for at least `additional` more elements to be inserted
    /// in the given `Array<T>`. The collection may reserve more space to avoid
    /// frequent reallocations. After calling reserve, capacity will be greater
    /// than or equal to `self.len() + additional`. Does nothing if capacity is
    /// already sufficient.
    ///
    /// # Panics
    ///
    /// Panics if the new capacity overflows `usize`.
    #[inline]
    pub fn reserve(&mut self, additional: usize) {
        self.0.reserve(additional);
    }

    /// Shrinks the capacity of the vector as much as possible.
    ///
    /// It will drop down as close as possible to the length but the allocator
    /// may still inform the vector that there is space for a few more elements.
    #[inline]
    pub fn shrink_to_fit(&mut self) {
        self.0.shrink_to_fit();
    }

    /// Clears the vector, removing all values.
    ///
    /// Note that this method has no effect on the allocated capacity of the
    /// vector.
    #[inline]
    pub fn clear(&mut self) {
        self.0.clear();
    }

    /// Returns the number of elements in the vector, also referred to as its
    /// "length".
    #[inline]
    #[must_use]
    pub fn len(&self) -> usize {
        self.0.len()
    }

    /// Returns `true` if the vector contains no elements.
    #[inline]
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    /// Returns a reference to an element at the index.
    ///
    /// Unlike [`Vec`], this method does not support indexing with a range.  See
    /// the [`slice`](Self::slice) method for retrieving a sub-slice from the
    /// array.
    #[inline]
    #[must_use]
    pub fn get(&self, index: usize) -> Option<Value> {
        self.0.get(index).copied().map(Value::from)
    }

    /// Deletes the element at the specified `index`, returning that element, or
    /// [`None`] if the `index` is out of range.
    #[inline]
    #[must_use]
    pub fn delete_at(&mut self, index: usize) -> Option<Value> {
        self.0.delete_at(index).map(Value::from)
    }

    /// Returns the first element from the vector, or [`None`] if the vector is
    /// empty.
    ///
    /// To retrieve a slice of the first elements in the vector, use
    /// [`first_n`](Self::first_n).
    #[inline]
    #[must_use]
    pub fn first(&self) -> Option<Value> {
        self.0.first().copied().map(Value::from)
    }

    /// Returns up to `n` of the first elements from the vector, or `&[]` if the
    /// vector is empty.
    ///
    /// To retrieve only the first element in the vector, use
    /// [`first`](Self::first).
    #[inline]
    #[must_use]
    pub fn first_n(&self, n: usize) -> &[sys::mrb_value] {
        self.0.first_n(n)
    }

    /// Returns the last element from the vector, or [`None`] if the vector is
    /// empty.
    ///
    /// To retrieve a slice of the last elements in the vector, use
    /// [`last_n`](Self::last_n).
    #[inline]
    #[must_use]
    pub fn last(&self) -> Option<Value> {
        self.0.last().copied().map(Value::from)
    }

    /// Returns up to `n` of the last elements from the vector, or `&[]` if the
    /// vector is empty.
    ///
    /// To retrieve only the last element in the vector, use
    /// [`last`](Self::last).
    #[inline]
    #[must_use]
    pub fn last_n(&self, n: usize) -> &[sys::mrb_value] {
        self.0.last_n(n)
    }

    /// Returns a slice of the underlying vector that includes only the first
    /// `n` elements.
    ///
    /// If `n` is greater than or equal to the length of the vector, `&self[..]`
    /// is returned.
    ///
    /// The inverse of this operation is [`drop_n`](Self::drop_n).
    #[inline]
    #[must_use]
    pub fn take_n(&self, n: usize) -> &[sys::mrb_value] {
        self.0.take_n(n)
    }

    /// Returns a slice of the underlying vector that excludes the first `n`
    /// elements.
    ///
    /// If `n` is greater than or equal to the length of the vector, `&[]` is
    /// returned.
    ///
    /// The inverse of this operation is [`take_n`](Self::take_n).
    #[inline]
    #[must_use]
    pub fn drop_n(&self, n: usize) -> &[sys::mrb_value] {
        self.0.drop_n(n)
    }

    /// Removes the last element from the vector and returns it, or [`None`] if
    /// the vector is empty.
    ///
    /// To pop more than one element from the end of the vector, use
    /// [`pop_n`](Self::pop_n).
    #[inline]
    #[must_use]
    pub fn pop(&mut self) -> Option<Value> {
        self.0.pop().map(Value::from)
    }

    /// Removes the last `n` elements from the vector.
    ///
    /// To pop a single element from the end of the vector, use
    /// [`pop`](Self::pop).
    #[inline]
    #[must_use]
    pub fn pop_n(&mut self, n: usize) -> Self {
        Self(self.0.pop_n(n))
    }

    /// Appends an element to the back of the vector.
    ///
    /// To push more than one element to the end of the vector, use
    /// [`concat`](Self::concat) or `extend`.
    ///
    /// # Panics
    ///
    /// Panics if the number of elements in the vector overflows a `usize`.
    #[inline]
    pub fn push(&mut self, elem: Value) {
        self.0.push(elem.inner());
    }

    /// Reverses the order of elements of the vector, in place.
    #[inline]
    pub fn reverse(&mut self) {
        self.0.reverse();
    }

    /// Removes the first element of the vector and returns it (shifting all
    /// other elements down by one). Returns [`None`] if the vector is empty.
    ///
    /// This operation is also known as "pop front".
    ///
    /// To remove more than one element from the front of the vector, use
    /// [`shift_n`](Self::shift_n).
    #[inline]
    #[must_use]
    pub fn shift(&mut self) -> Option<Value> {
        self.0.shift().map(Value::from)
    }

    /// Removes the first `n` elements from the vector.
    ///
    /// To shift a single element from the front of the vector, use
    /// [`shift`](Self::shift).
    #[inline]
    #[must_use]
    pub fn shift_n(&mut self, n: usize) -> Self {
        Self(self.0.shift_n(n))
    }

    /// Inserts an element to the front of the vector.
    ///
    /// To insert more than one element to the front of the vector, use
    /// [`unshift_n`](Self::unshift_n).
    ///
    /// This operation is also known as "prepend".
    ///
    /// # Panics
    ///
    /// Panics if the number of elements in the vector overflows a `usize`.
    #[inline]
    pub fn unshift(&mut self, elem: Value) {
        self.0.unshift(elem.inner());
    }

    /// Return a reference to a subslice of the vector.
    ///
    /// This function always returns a slice. If the range specified by `start`
    /// and `end` overlaps the vector (even if only partially), the overlapping
    /// slice is returned. If the range does not overlap the vector, an empty
    /// slice is returned.
    #[inline]
    #[must_use]
    pub fn slice(&self, start: usize, len: usize) -> &[sys::mrb_value] {
        self.0.slice(start, len)
    }
}

impl Array
where
    sys::mrb_value: Clone,
{
    /// Construct a new `Array` with length `len` and all elements set to
    /// `default`. The `Array` will have capacity `len`.
    #[inline]
    #[must_use]
    pub fn with_len_and_default(len: usize, default: Value) -> Self {
        Self(spinoso_array::Array::with_len_and_default(len, default.inner()))
    }

    /// Appends the elements of `other` to self.
    ///
    /// Slice version of `extend`. This operation is analogous to "push n".
    #[inline]
    pub fn concat(&mut self, other: &[sys::mrb_value]) {
        self.0.concat(other);
    }

    /// Prepends the elements of `other` to self.
    ///
    /// To insert one element to the front of the vector, use
    /// [`unshift`](Self::unshift).
    ///
    /// This operation is also known as "prepend".
    ///
    /// # Panics
    ///
    /// Panics if the number of elements in the vector overflows a `usize`.
    #[inline]
    pub fn unshift_n(&mut self, other: &[sys::mrb_value]) {
        self.0.unshift_n(other);
    }
}

impl Array
where
    sys::mrb_value: Copy,
{
    /// Creates a new array by repeating this array `n` times.
    ///
    /// This function will not panic. If the resulting `Array`'s capacity would
    /// overflow, [`None`] is returned.
    #[must_use]
    pub fn repeat(&self, n: usize) -> Option<Self> {
        self.0.repeat(n).map(Self)
    }
}

impl Array
where
    sys::mrb_value: Default,
{
    /// Set element at position `index` within the vector, extending the vector
    /// with `nil` if `index` is out of bounds.
    #[inline]
    pub fn set(&mut self, index: usize, elem: Value) {
        self.0.set(index, elem.inner());
    }

    /// Insert element at position `start` within the vector and remove the
    /// following `drain` elements. If `start` is out of bounds, the vector will
    /// be extended with `nil`.
    ///
    /// This method sets a slice of the `Array` to a single element, including
    /// the zero-length slice. It is similar in intent to calling
    /// [`Vec::splice`] with a one-element iterator.
    ///
    /// `set_with_drain` will only drain up to the end of the vector.
    ///
    /// To set a single element without draining, use [`set`](Self::set).
    #[inline]
    pub fn set_with_drain(&mut self, start: usize, drain: usize, elem: Value) -> usize {
        self.0.set_with_drain(start, drain, elem.inner())
    }
}

impl Array
where
    sys::mrb_value: Default + Clone,
{
    /// Insert the elements from a slice at a position `index` in the vector,
    /// extending the vector with `nil` if `index` is out of bounds.
    ///
    /// This method is similar to [`Vec::splice`] when called with a zero-length
    /// range.
    #[inline]
    pub fn insert_slice(&mut self, index: usize, values: &[sys::mrb_value]) {
        self.0.insert_slice(index, values);
    }

    /// Insert the elements from a slice at a position `index` in the vector and
    /// remove the following `drain` elements. The vector is extended with
    /// `nil` if `index` is out of bounds.
    ///
    /// This method is similar to [`Vec::splice`] when called with a
    /// nonzero-length range.
    ///
    /// When called with `drain == 0`, this method is equivalent to
    /// [`insert_slice`](Self::insert_slice).
    ///
    /// If `drain >= src.len()` or the tail of the vector is replaced, this
    /// method is efficient. Otherwise, a temporary buffer is used to move the
    /// elements.
    #[inline]
    pub fn set_slice(&mut self, index: usize, drain: usize, values: &[sys::mrb_value]) -> usize {
        self.0.set_slice(index, drain, values)
    }
}
