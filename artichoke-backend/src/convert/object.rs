use std::borrow::Cow;
use std::cell::RefCell;
use std::ffi::c_void;
use std::mem;
use std::ptr;
use std::rc::Rc;

use crate::sys;
use crate::types::{Ruby, Rust};
use crate::value::Value;
use crate::{Artichoke, ArtichokeError};

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
        interp: &Artichoke,
        slf: Option<sys::mrb_value>,
    ) -> Result<Value, ArtichokeError> {
        let borrow = interp.0.borrow();
        let mrb = borrow.mrb;
        let spec = borrow
            .class_spec::<Self>()
            .ok_or_else(|| ArtichokeError::ConvertToRuby {
                from: Rust::Object,
                to: Ruby::Object,
            })?;
        let data = Rc::new(RefCell::new(self));
        let ptr = Rc::into_raw(data);
        let obj = if let Some(mut slf) = slf {
            unsafe {
                sys::mrb_sys_data_init(&mut slf, ptr as *mut c_void, spec.data_type());
            }
            slf
        } else {
            let rclass = slf
                .map(|obj| unsafe { sys::mrb_sys_class_of_value(mrb, obj) })
                .or_else(|| spec.rclass(interp))
                .ok_or_else(|| ArtichokeError::ConvertToRuby {
                    from: Rust::Object,
                    to: Ruby::Object,
                })?;
            unsafe {
                let alloc =
                    sys::mrb_data_object_alloc(mrb, rclass, ptr as *mut c_void, spec.data_type());
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
        interp: &Artichoke,
        slf: &Value,
    ) -> Result<Rc<RefCell<Self>>, ArtichokeError> {
        // Make sure we have a Data otherwise extraction will fail.
        if slf.ruby_type() != Ruby::Data {
            return Err(ArtichokeError::ConvertToRust {
                from: slf.ruby_type(),
                to: Rust::Object,
            });
        }
        let borrow = interp.0.borrow();
        let mrb = borrow.mrb;
        let spec = borrow
            .class_spec::<Self>()
            .ok_or_else(|| ArtichokeError::NotDefined(Cow::Borrowed(Self::ruby_type_name())))?;
        // Sanity check that the RClass matches.
        let rclass = spec
            .rclass(interp)
            .ok_or_else(|| ArtichokeError::NotDefined(Cow::Borrowed(Self::ruby_type_name())))?;
        if !ptr::eq(sys::mrb_sys_class_of_value(mrb, slf.inner()), rclass) {
            return Err(ArtichokeError::ConvertToRust {
                from: slf.ruby_type(),
                to: Rust::Object,
            });
        }
        let ptr = sys::mrb_data_check_get_ptr(mrb, slf.inner(), spec.data_type());
        if ptr.is_null() {
            // `Object#allocate` can be used to create `MRB_TT_DATA` without calling
            // `#initialize`. These objects will return a NULL pointer.
            return Err(ArtichokeError::UninitializedValue(Self::ruby_type_name()));
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
    #[must_use]
    fn ruby_type_name() -> &'static str {
        T::ruby_type_name()
    }
}

#[cfg(test)]
mod tests {
    use std::rc::Rc;

    use crate::class;
    use crate::convert::object::RustBackedValue;
    use crate::convert::Convert;
    use crate::def;
    use crate::sys;
    use crate::value::{Value, ValueLike};

    #[derive(Debug, Default, Clone)]
    struct Container {
        inner: String,
    }

    impl Container {
        unsafe extern "C" fn value(
            mrb: *mut sys::mrb_state,
            slf: sys::mrb_value,
        ) -> sys::mrb_value {
            let interp = unwrap_interpreter!(mrb);

            let value = Value::new(&interp, slf);
            if let Ok(container) = Self::try_from_ruby(&interp, &value) {
                let borrow = container.borrow();
                interp.convert(borrow.inner.as_bytes()).inner()
            } else {
                interp.convert(None::<Value>).inner()
            }
        }
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
        let interp = crate::interpreter().expect("init");
        let spec = class::Spec::new("Container", None, Some(def::rust_data_free::<Container>));
        class::Builder::for_spec(&interp, &spec)
            .value_is_rust_object()
            .add_method("value", Container::value, sys::mrb_args_none())
            .define()
            .unwrap();
        interp.0.borrow_mut().def_class::<Container>(spec);
        let obj = Container {
            inner: "contained string contents".to_owned(),
        };

        let value = unsafe { obj.try_into_ruby(&interp, None) }.expect("convert");
        let class = value.funcall::<Value>("class", &[], None).expect("funcall");
        assert_eq!(class.to_s(), b"Container");
        let data = unsafe { Container::try_from_ruby(&interp, &value) }.expect("convert");
        assert_eq!(Rc::strong_count(&data), 2);
        assert_eq!(&data.borrow().inner, "contained string contents");
        drop(data);
        let inner = value.funcall::<&str>("value", &[], None).expect("funcall");
        assert_eq!(inner, "contained string contents");
    }

    #[test]
    fn convert_obj_not_data() {
        let interp = crate::interpreter().expect("init");
        let spec = class::Spec::new("Container", None, Some(def::rust_data_free::<Container>));
        class::Builder::for_spec(&interp, &spec)
            .value_is_rust_object()
            .add_method("value", Container::value, sys::mrb_args_none())
            .define()
            .unwrap();
        interp.0.borrow_mut().def_class::<Container>(spec);
        let spec = class::Spec::new("Other", None, Some(def::rust_data_free::<Container>));
        class::Builder::for_spec(&interp, &spec)
            .value_is_rust_object()
            .define()
            .unwrap();
        interp.0.borrow_mut().def_class::<Box<Other>>(spec);

        let value = interp.convert("string");
        let class = value.funcall::<Value>("class", &[], None).expect("funcall");
        assert_eq!(class.to_s(), b"String");
        let data = unsafe { Container::try_from_ruby(&interp, &value) };
        assert!(data.is_err());
        let value =
            unsafe { Box::new(Other::default()).try_into_ruby(&interp, None) }.expect("convert");
        let class = value.funcall::<Value>("class", &[], None).expect("funcall");
        assert_eq!(class.to_s(), b"Other");
        let data = unsafe { Container::try_from_ruby(&interp, &value) };
        assert!(data.is_err());
    }
}
