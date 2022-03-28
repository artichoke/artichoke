use std::ffi::c_void;
use std::fmt;
use std::marker::PhantomData;
use std::mem::ManuallyDrop;
use std::ops::{Deref, DerefMut};
use std::ptr;

use crate::core::Value as _;
use crate::def::NotDefinedError;
use crate::error::Error;
use crate::extn::core::exception::TypeError;
use crate::ffi::InterpreterExtractError;
use crate::sys;
use crate::types::Ruby;
use crate::value::Value;
use crate::Artichoke;

pub struct UnboxedValueGuard<'a, T> {
    guarded: ManuallyDrop<T>,
    phantom: PhantomData<&'a mut T>,
}

impl<'a, T> fmt::Debug for UnboxedValueGuard<'a, T>
where
    T: fmt::Debug,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("UnboxedValueGuard")
            .field("guarded", &self.guarded)
            .finish()
    }
}

impl<'a, T> UnboxedValueGuard<'a, T> {
    /// Construct a new guard around the given `T`.
    ///
    /// `UnboxedValueGuard` allows passing around a `&mut` reference without
    /// dropping the `T` when returning control to mruby C code. This is
    /// desirable because the `T` is owned by the mruby heap until the mruby
    /// garbage collector frees the `mrb_value` that holds it.
    #[must_use]
    pub fn new(value: T) -> Self {
        Self {
            guarded: ManuallyDrop::new(value),
            phantom: PhantomData,
        }
    }

    /// Get a shared reference to the inner `T`.
    #[inline]
    #[must_use]
    pub fn as_inner_ref(&self) -> &T {
        &*self.guarded
    }

    /// Get a unique reference to the inner `T`.
    ///
    /// # Safety
    ///
    /// Callers must ensure the raw parts stored in the source `mrb_value` are
    /// not invalidated OR that the raw parts are repacked before an mruby
    /// allocation occurs.
    #[inline]
    #[must_use]
    pub unsafe fn as_inner_mut(&mut self) -> &mut T {
        &mut *self.guarded
    }

    /// Take the inner `T` out of the guard.
    ///
    /// # Safety
    ///
    /// Callers must ensure that `T` is not dropped. Once `T` is taken out of
    /// the guard, it must ultimately be passed to [`ManuallyDrop`] before it is
    /// dropped.
    ///
    /// An example of safe usage is calling taking an `Array` out of the guard
    /// and then immediately calling `Array::into_raw_parts` on the returned
    /// value.
    #[inline]
    #[must_use]
    pub unsafe fn take(mut self) -> T {
        ManuallyDrop::take(&mut self.guarded)
    }
}

#[derive(Debug)]
pub struct HeapAllocated<T>(Box<T>);

impl<T> HeapAllocated<T> {
    #[must_use]
    pub fn new(obj: Box<T>) -> Self {
        Self(obj)
    }
}

impl<'a, T> AsRef<T> for UnboxedValueGuard<'a, HeapAllocated<T>> {
    fn as_ref(&self) -> &T {
        self.guarded.deref().0.as_ref()
    }
}

impl<'a, T> AsMut<T> for UnboxedValueGuard<'a, HeapAllocated<T>> {
    fn as_mut(&mut self) -> &mut T {
        self.guarded.deref_mut().0.as_mut()
    }
}

impl<'a, T> Deref for UnboxedValueGuard<'a, HeapAllocated<T>> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        self.guarded.deref().0.as_ref()
    }
}

impl<'a, T> DerefMut for UnboxedValueGuard<'a, HeapAllocated<T>> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        // Safety:
        //
        // `HeapAllocated` data objects are boxed and the raw box pointer is
        // stored in the `mrb_value`.
        //
        // `Deref::Target` is `T`, not `Box<T>`.
        //
        // Giving out a `&mut T` means the box pointer cannot be invalidated.
        let inner = unsafe { self.as_inner_mut() };
        inner.0.as_mut()
    }
}

pub trait HeapAllocatedData {
    const RUBY_TYPE: &'static str;
}

#[derive(Debug)]
pub struct Immediate<T>(T);

impl<T> Immediate<T> {
    pub fn new(obj: T) -> Self {
        Self(obj)
    }
}

impl<'a, T> Deref for UnboxedValueGuard<'a, Immediate<T>> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.guarded.deref().0
    }
}

impl<'a, T> DerefMut for UnboxedValueGuard<'a, Immediate<T>> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.guarded.deref_mut().0
    }
}

pub trait BoxUnboxVmValue {
    type Unboxed;
    type Guarded;

    const RUBY_TYPE: &'static str;

    /// # Safety
    ///
    /// Implementations may return owned values. These values must not outlive
    /// the underlying `mrb_value`, which may be garbage collected by mruby.
    ///
    /// The values returned by this method should not be stored for more than
    /// the current FFI trampoline entry point.
    unsafe fn unbox_from_value<'a>(
        value: &'a mut Value,
        interp: &mut Artichoke,
    ) -> Result<UnboxedValueGuard<'a, Self::Guarded>, Error>;

    fn alloc_value(value: Self::Unboxed, interp: &mut Artichoke) -> Result<Value, Error>;

    fn box_into_value(value: Self::Unboxed, into: Value, interp: &mut Artichoke) -> Result<Value, Error>;

    fn free(data: *mut c_void);
}

impl<T> BoxUnboxVmValue for T
where
    T: HeapAllocatedData + Sized + 'static,
{
    type Unboxed = Self;
    type Guarded = HeapAllocated<Self::Unboxed>;

    const RUBY_TYPE: &'static str = <Self as HeapAllocatedData>::RUBY_TYPE;

    unsafe fn unbox_from_value<'a>(
        value: &'a mut Value,
        interp: &mut Artichoke,
    ) -> Result<UnboxedValueGuard<'a, Self::Guarded>, Error> {
        // Make sure we have a Data otherwise extraction will fail.
        if value.ruby_type() != Ruby::Data {
            let mut message = String::from("uninitialized ");
            message.push_str(Self::RUBY_TYPE);
            return Err(TypeError::from(message).into());
        }

        let mut rclass = {
            let state = interp.state.as_ref().ok_or_else(InterpreterExtractError::new)?;
            let spec = state
                .classes
                .get::<Self>()
                .ok_or_else(|| NotDefinedError::class(Self::RUBY_TYPE))?;
            let rclass = spec.rclass();
            interp
                .with_ffi_boundary(|mrb| rclass.resolve(mrb))?
                .ok_or_else(|| NotDefinedError::class(Self::RUBY_TYPE))?
        };

        // Sanity check that the RClass matches.
        let value_rclass = interp.with_ffi_boundary(|mrb| sys::mrb_sys_class_of_value(mrb, value.inner()))?;
        if !ptr::eq(value_rclass, rclass.as_mut()) {
            let mut message = String::from("Could not extract ");
            message.push_str(Self::RUBY_TYPE);
            message.push_str(" from receiver");
            return Err(TypeError::from(message).into());
        }

        // Copy data pointer out of the `mrb_value` box.
        let state = interp.state.as_ref().ok_or_else(InterpreterExtractError::new)?;
        let spec = state
            .classes
            .get::<Self>()
            .ok_or_else(|| NotDefinedError::class(Self::RUBY_TYPE))?;
        let data_type = spec.data_type();
        let embedded_data_ptr =
            interp.with_ffi_boundary(|mrb| sys::mrb_data_check_get_ptr(mrb, value.inner(), data_type))?;
        if embedded_data_ptr.is_null() {
            // `Object#allocate` can be used to create `MRB_TT_DATA` without calling
            // `#initialize`. These objects will return a NULL pointer.
            let mut message = String::from("uninitialized ");
            message.push_str(Self::RUBY_TYPE);
            return Err(TypeError::from(message).into());
        }

        // Move the data pointer into a `Box`.
        let value = Box::from_raw(embedded_data_ptr.cast::<Self>());
        // `UnboxedValueGuard` ensures the `Box` wrapper will be forgotten. The
        // mruby GC is responsible for freeing the value.
        Ok(UnboxedValueGuard::new(HeapAllocated::new(value)))
    }

    fn alloc_value(value: Self::Unboxed, interp: &mut Artichoke) -> Result<Value, Error> {
        let mut rclass = {
            let state = interp.state.as_ref().ok_or_else(InterpreterExtractError::new)?;
            let spec = state
                .classes
                .get::<Self>()
                .ok_or_else(|| NotDefinedError::class(Self::RUBY_TYPE))?;
            let rclass = spec.rclass();
            unsafe { interp.with_ffi_boundary(|mrb| rclass.resolve(mrb)) }?
                .ok_or_else(|| NotDefinedError::class(Self::RUBY_TYPE))?
        };

        // Convert to a raw pointer.
        let data = Box::new(value);
        let ptr = Box::into_raw(data);

        // Allocate a new `mrb_value` and inject the raw data pointer.
        let state = interp.state.as_ref().ok_or_else(InterpreterExtractError::new)?;
        let spec = state
            .classes
            .get::<Self>()
            .ok_or_else(|| NotDefinedError::class(Self::RUBY_TYPE))?;
        let data_type = spec.data_type();
        let obj = unsafe {
            interp.with_ffi_boundary(|mrb| {
                let alloc = sys::mrb_data_object_alloc(mrb, rclass.as_mut(), ptr.cast::<c_void>(), data_type);
                sys::mrb_sys_obj_value(alloc.cast::<c_void>())
            })?
        };

        Ok(interp.protect(Value::from(obj)))
    }

    fn box_into_value(value: Self::Unboxed, into: Value, interp: &mut Artichoke) -> Result<Value, Error> {
        let state = interp.state.as_ref().ok_or_else(InterpreterExtractError::new)?;
        let spec = state
            .classes
            .get::<Self>()
            .ok_or_else(|| NotDefinedError::class(Self::RUBY_TYPE))?;

        // Convert to a raw pointer.
        let data = Box::new(value);
        let ptr = Box::into_raw(data);

        // Inject the raw data pointer into the given `mrb_value`.
        let mut obj = into.inner();
        unsafe {
            sys::mrb_sys_data_init(&mut obj, ptr.cast::<c_void>(), spec.data_type());
        }
        Ok(Value::from(obj))
    }

    fn free(data: *mut c_void) {
        // Cast the raw data pointer into a pointer to `Self`.
        let data = data.cast::<Self>();
        // Convert the raw pointer back into a `Box`.
        let unboxed = unsafe { Box::from_raw(data) };
        // And free the memory.
        drop(unboxed);
    }
}

#[cfg(test)]
mod tests {
    use bstr::ByteSlice;

    use crate::test::prelude::*;

    // this struct is heap allocated.
    #[derive(Default, Debug, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
    struct Container(String);

    unsafe extern "C" fn container_value(mrb: *mut sys::mrb_state, slf: sys::mrb_value) -> sys::mrb_value {
        unwrap_interpreter!(mrb, to => guard);

        let mut value = Value::from(slf);
        let result = if let Ok(container) = Container::unbox_from_value(&mut value, &mut guard) {
            guard.try_convert_mut(container.0.as_bytes()).unwrap_or_default()
        } else {
            Value::nil()
        };
        result.inner()
    }

    impl HeapAllocatedData for Container {
        const RUBY_TYPE: &'static str = "Container";
    }

    // this struct is stack allocated
    #[derive(Default, Debug, Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord)]
    struct Flag(bool);

    impl HeapAllocatedData for Box<Flag> {
        const RUBY_TYPE: &'static str = "Flag";
    }

    #[test]
    fn convert_obj_roundtrip() {
        let mut interp = interpreter();
        let spec = class::Spec::new(
            "Container",
            cstr::cstr!("Container"),
            None,
            Some(def::box_unbox_free::<Container>),
        )
        .unwrap();
        class::Builder::for_spec(&mut interp, &spec)
            .value_is_rust_object()
            .add_method("value", container_value, sys::mrb_args_none())
            .unwrap()
            .define()
            .unwrap();
        interp.def_class::<Container>(spec).unwrap();
        let obj = Container(String::from("contained string contents"));

        let mut value = Container::alloc_value(obj, &mut interp).unwrap();
        let class = value.funcall(&mut interp, "class", &[], None).unwrap();
        let class_display = class.to_s(&mut interp);
        assert_eq!(class_display.as_bstr(), b"Container".as_bstr());

        let data = unsafe { Container::unbox_from_value(&mut value, &mut interp) }.unwrap();

        let inner = data.0.as_str();
        assert_eq!(inner, "contained string contents");
        drop(data);

        let inner = value.funcall(&mut interp, "value", &[], None).unwrap();
        let inner = inner.try_convert_into_mut::<&str>(&mut interp).unwrap();
        assert_eq!(inner, "contained string contents");
    }

    #[test]
    fn convert_obj_not_data() {
        let mut interp = interpreter();

        let spec = class::Spec::new(
            "Container",
            cstr::cstr!("Container"),
            None,
            Some(def::box_unbox_free::<Container>),
        )
        .unwrap();
        class::Builder::for_spec(&mut interp, &spec)
            .value_is_rust_object()
            .add_method("value", container_value, sys::mrb_args_none())
            .unwrap()
            .define()
            .unwrap();
        interp.def_class::<Container>(spec).unwrap();

        let spec = class::Spec::new(
            "Flag",
            cstr::cstr!("Flag"),
            None,
            Some(def::box_unbox_free::<Box<Flag>>),
        )
        .unwrap();
        class::Builder::for_spec(&mut interp, &spec)
            .value_is_rust_object()
            .define()
            .unwrap();
        interp.def_class::<Box<Flag>>(spec).unwrap();

        let mut value = interp.try_convert_mut("string").unwrap();
        let class = value.funcall(&mut interp, "class", &[], None).unwrap();
        let class_display = class.to_s(&mut interp);
        assert_eq!(class_display.as_bstr(), b"String".as_bstr());

        let data = unsafe { Container::unbox_from_value(&mut value, &mut interp) };
        assert!(data.is_err());

        let flag = Box::new(Flag::default());
        let mut value = Box::<Flag>::alloc_value(flag, &mut interp).unwrap();
        let class = value.funcall(&mut interp, "class", &[], None).unwrap();
        let class_display = class.to_s(&mut interp);
        assert_eq!(class_display.as_bstr(), b"Flag".as_bstr());

        let data = unsafe { Container::unbox_from_value(&mut value, &mut interp) };
        assert!(data.is_err());
    }
}
