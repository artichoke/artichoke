use std::cell::RefCell;
use std::ffi::c_void;
use std::mem;
use std::ptr;
use std::rc::Rc;

use crate::def::NotDefinedError;
use crate::exception::Exception;
use crate::extn::core::exception::TypeError;
use crate::ffi::InterpreterExtractError;
use crate::sys;
use crate::types::Ruby;
use crate::value::Value;
use crate::Artichoke;

/// Provides converters to and from [`Value`] with ruby type of [`Ruby::Data`].
///
/// This trait provides default implementations of `try_into_ruby` and
/// `try_from_ruby`.
///
/// The 'static type bound comes from the class spec registry on
/// [`State`](crate::state::State).
///
/// **Warning**: `Self` must be allocated on the heap. If `Self` is not heap
/// allocated, malloc will fail when attempting to free the generated Ruby
/// objects.
pub trait RustBackedValue
where
    Self: 'static + Sized,
{
    /// Class or module name of this data type.
    fn ruby_type_name() -> &'static str;

    /// Try to convert a Rust object into a [`Value`].
    ///
    /// This method wraps `self` in an `Rc<RefCell<_>>` and turns it into a raw
    /// pointer suitable for embedding in an [`sys::mrb_value`] of type
    /// `MRB_TT_DATA`.
    ///
    /// If the `slf` parameter is `Some`, the serialized data pointer is used
    /// to initialize the contained [`sys::mrb_value`]. Otherwise, a new
    /// [`sys::mrb_value`] is allocated with [`sys::mrb_obj_new`].
    fn try_into_ruby(
        self,
        interp: &mut Artichoke,
        slf: Option<sys::mrb_value>,
    ) -> Result<Value, Exception> {
        let state = interp.state.as_ref().ok_or(InterpreterExtractError)?;
        let spec = state
            .classes
            .get::<Self>()
            .ok_or_else(|| NotDefinedError::class(Self::ruby_type_name()))?;
        let data = Rc::new(RefCell::new(self));
        let ptr = Rc::into_raw(data);
        let obj = if let Some(mut slf) = slf {
            unsafe {
                sys::mrb_sys_data_init(&mut slf, ptr as *mut c_void, spec.data_type());
            }
            slf
        } else {
            unsafe {
                let mrb = interp.mrb.as_mut();
                let mut rclass = spec
                    .rclass(mrb)
                    .ok_or_else(|| NotDefinedError::class(Self::ruby_type_name()))?;
                let alloc = sys::mrb_data_object_alloc(
                    mrb,
                    rclass.as_mut(),
                    ptr as *mut c_void,
                    spec.data_type(),
                );
                sys::mrb_sys_obj_value(alloc as *mut c_void)
            }
        };

        Ok(Value::new(interp, obj))
    }

    /// Try to extract a Rust object from the [`Value`].
    ///
    /// Extract the data pointer from `slf` and return an `Rc<RefCell<_>>`
    /// containing the Rust object.
    ///
    /// # Safety
    ///
    /// This method performs some safety checks before calling [`Rc::from_raw`]:
    ///
    /// - Ensure `slf` is of type `MRB_TT_DATA`.
    /// - Ensure `slf` has the same [`sys::RClass`] as the bound type.
    /// - Ensure the data pointer is not `NULL`.
    ///
    /// This method assumes the [`Rc`] pointed to by the data pointer has not
    /// been freed, which is built on the assumption that there are no garbage
    /// collector bugs in the mruby VM for Artichoke custom types.
    unsafe fn try_from_ruby(
        interp: &mut Artichoke,
        slf: &Value,
    ) -> Result<Rc<RefCell<Self>>, Exception> {
        // Make sure we have a Data otherwise extraction will fail.
        if slf.ruby_type() != Ruby::Data {
            let mut message = String::from("uninitialized ");
            message.push_str(Self::ruby_type_name());
            return Err(Exception::from(TypeError::new(interp, message)));
        }
        let state = interp.state.as_ref().ok_or(InterpreterExtractError)?;
        let spec = state
            .classes
            .get::<Self>()
            .ok_or_else(|| NotDefinedError::class(Self::ruby_type_name()))?;
        // Sanity check that the RClass matches.
        let mrb = interp.mrb.as_mut();
        let mut rclass = spec
            .rclass(mrb)
            .ok_or_else(|| NotDefinedError::class(Self::ruby_type_name()))?;
        if !ptr::eq(
            sys::mrb_sys_class_of_value(mrb, slf.inner()),
            rclass.as_mut(),
        ) {
            let mut message = String::from("Could not extract ");
            message.push_str(Self::ruby_type_name());
            message.push_str(" from receiver");
            return Err(Exception::from(TypeError::new(interp, message)));
        }
        let ptr = sys::mrb_data_check_get_ptr(mrb, slf.inner(), spec.data_type());
        if ptr.is_null() {
            // `Object#allocate` can be used to create `MRB_TT_DATA` without calling
            // `#initialize`. These objects will return a NULL pointer.
            let mut message = String::from("uninitialized ");
            message.push_str(Self::ruby_type_name());
            return Err(Exception::from(TypeError::new(interp, message)));
        }
        let data = Rc::from_raw(ptr as *const RefCell<Self>);
        let value = Rc::clone(&data);
        mem::forget(data);
        Ok(value)
    }
}

impl<T> RustBackedValue for Box<T>
where
    T: RustBackedValue,
{
    fn ruby_type_name() -> &'static str {
        T::ruby_type_name()
    }
}

#[cfg(test)]
mod tests {
    use std::rc::Rc;

    use crate::test::prelude::*;

    #[derive(Debug, Default, Clone)]
    struct Container {
        inner: String,
    }

    unsafe extern "C" fn container_value(
        mrb: *mut sys::mrb_state,
        slf: sys::mrb_value,
    ) -> sys::mrb_value {
        let mut interp = unwrap_interpreter!(mrb);
        let mut guard = Guard::new(&mut interp);

        let value = Value::new(&guard, slf);
        let result = if let Ok(container) = Container::try_from_ruby(&mut guard, &value) {
            let borrow = container.borrow();
            guard.convert_mut(borrow.inner.as_bytes())
        } else {
            guard.convert(None::<Value>)
        };
        result.inner()
    }

    impl RustBackedValue for Container {
        fn ruby_type_name() -> &'static str {
            "Container"
        }
    }

    #[derive(Debug, Default, Clone, Copy)]
    // this struct is stack allocated
    struct Other {
        _inner: bool,
    }

    impl RustBackedValue for Other {
        fn ruby_type_name() -> &'static str {
            "Other"
        }
    }

    #[test]
    fn convert_obj_roundtrip() {
        let mut interp = crate::interpreter().unwrap();
        let spec =
            class::Spec::new("Container", None, Some(def::rust_data_free::<Container>)).unwrap();
        class::Builder::for_spec(&interp, &spec)
            .value_is_rust_object()
            .add_method("value", container_value, sys::mrb_args_none())
            .unwrap()
            .define()
            .unwrap();
        interp.0.borrow_mut().def_class::<Container>(spec);
        let obj = Container {
            inner: "contained string contents".to_owned(),
        };

        let value = obj.try_into_ruby(&mut interp, None).unwrap();
        let class = value
            .funcall::<Value>(&mut interp, "class", &[], None)
            .unwrap();
        assert_eq!(class.to_s(&mut interp), b"Container");
        let data = unsafe { Container::try_from_ruby(&mut interp, &value) }.unwrap();
        assert_eq!(Rc::strong_count(&data), 2);
        assert_eq!(&data.borrow().inner, "contained string contents");
        drop(data);
        let inner = value
            .funcall::<&str>(&mut interp, "value", &[], None)
            .unwrap();
        assert_eq!(inner, "contained string contents");
    }

    #[test]
    fn convert_obj_not_data() {
        let mut interp = crate::interpreter().unwrap();
        let spec =
            class::Spec::new("Container", None, Some(def::rust_data_free::<Container>)).unwrap();
        class::Builder::for_spec(&interp, &spec)
            .value_is_rust_object()
            .add_method("value", container_value, sys::mrb_args_none())
            .unwrap()
            .define()
            .unwrap();
        interp.0.borrow_mut().def_class::<Container>(spec);
        let spec = class::Spec::new("Other", None, Some(def::rust_data_free::<Container>)).unwrap();
        class::Builder::for_spec(&interp, &spec)
            .value_is_rust_object()
            .define()
            .unwrap();
        interp.0.borrow_mut().def_class::<Box<Other>>(spec);

        let value = interp.convert_mut("string");
        let class = value
            .funcall::<Value>(&mut interp, "class", &[], None)
            .unwrap();
        assert_eq!(class.to_s(&mut interp), b"String");
        let data = unsafe { Container::try_from_ruby(&mut interp, &value) };
        assert!(data.is_err());
        let value = Box::new(Other::default())
            .try_into_ruby(&mut interp, None)
            .unwrap();
        let class = value
            .funcall::<Value>(&mut interp, "class", &[], None)
            .unwrap();
        assert_eq!(class.to_s(&mut interp), b"Other");
        let data = unsafe { Container::try_from_ruby(&mut interp, &value) };
        assert!(data.is_err());
    }
}
