use std::cell::RefCell;
use std::ffi::c_void;
use std::mem;
use std::ptr;
use std::rc::Rc;

use crate::def::ClassLike;
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
    /// Try to convert a Rust object into a [`Value`].
    ///
    /// Inject the data pointer into `slf` if it is provided, otherwise call
    /// [`sys::mrb_obj_new`] to get a new instance of the class associated with
    /// `Self`.
    ///
    /// To store `self` in a [`sys::mrb_value`], this function wraps `self` in
    /// an `Rc<RefCell<_>>`.
    unsafe fn try_into_ruby(
        self,
        interp: &Artichoke,
        slf: Option<sys::mrb_value>,
    ) -> Result<Value, ArtichokeError> {
        let mrb = interp.0.borrow().mrb;
        let spec = interp.0.borrow().class_spec::<Self>().ok_or_else(|| {
            ArtichokeError::ConvertToRuby {
                from: Rust::Object,
                to: Ruby::Object,
            }
        })?;
        let rclass = slf
            .map(|obj| sys::mrb_sys_class_of_value(mrb, obj))
            .or_else(|| spec.borrow().rclass(interp))
            .ok_or_else(|| ArtichokeError::ConvertToRuby {
                from: Rust::Object,
                to: Ruby::Object,
            })?;
        let obj = if let Some(mut slf) = slf {
            let data = Rc::new(RefCell::new(self));
            let ptr = Rc::into_raw(data);
            sys::mrb_sys_data_init(&mut slf, ptr as *mut c_void, spec.borrow().data_type());
            slf
        } else {
            let data = Rc::new(RefCell::new(self));
            let ptr = Rc::into_raw(data);
            let data = sys::mrb_data_object_alloc(
                mrb,
                rclass,
                ptr as *mut c_void,
                spec.borrow().data_type(),
            );
            sys::mrb_sys_obj_value(data as *mut c_void)
        };

        Ok(Value::new(interp, obj))
    }

    /// Extract the Rust object from the [`Value`] if the [`Value`] is backed by
    /// `Self`.
    ///
    /// Extract the data pointer from `slf` and return an `Rc<RefCell<_>>`
    /// containing the Rust object.
    ///
    /// This function sanity checks to make sure that [`Value`] is a
    /// [`Ruby::Data`] and that the `RClass *` of the spec matches the
    /// [`Value`].
    unsafe fn try_from_ruby(
        interp: &Artichoke,
        slf: &Value,
    ) -> Result<Rc<RefCell<Self>>, ArtichokeError> {
        if slf.ruby_type() != Ruby::Data {
            return Err(ArtichokeError::ConvertToRust {
                from: slf.ruby_type(),
                to: Rust::Object,
            });
        }
        let mrb = interp.0.borrow().mrb;
        // Make sure we have a Data otherwise extraction will fail.
        if slf.ruby_type() != Ruby::Data {
            return Err(ArtichokeError::ConvertToRust {
                from: slf.ruby_type(),
                to: Rust::Object,
            });
        }
        let spec = interp
            .0
            .borrow()
            .class_spec::<Self>()
            .ok_or_else(|| ArtichokeError::NotDefined("class".to_owned()))?;
        // Sanity check that the RClass matches.
        if let Some(rclass) = spec.borrow().rclass(interp) {
            if !ptr::eq(sys::mrb_sys_class_of_value(mrb, slf.inner()), rclass) {
                return Err(ArtichokeError::ConvertToRust {
                    from: slf.ruby_type(),
                    to: Rust::Object,
                });
            }
        } else {
            return Err(ArtichokeError::NotDefined("class".to_owned()));
        }
        let ptr = {
            let borrow = spec.borrow();
            sys::mrb_data_get_ptr(mrb, slf.inner(), borrow.data_type())
        };
        if ptr.is_null() {
            return Err(ArtichokeError::ConvertToRust {
                from: slf.ruby_type(),
                to: Rust::Object,
            });
        }
        let data = Rc::from_raw(ptr as *const RefCell<Self>);
        let value = Rc::clone(&data);
        mem::forget(data);
        Ok(value)
    }
}

impl<T> RustBackedValue for Box<T> where T: RustBackedValue {}

#[cfg(test)]
mod tests {
    use std::rc::Rc;

    use crate::convert::object::RustBackedValue;
    use crate::convert::Convert;
    use crate::def::{rust_data_free, ClassLike, Define};
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

    impl RustBackedValue for Container {}

    #[derive(Debug, Default, Clone, Copy)]
    // this struct is stack allocated
    struct Other {
        _inner: bool,
    }

    impl RustBackedValue for Other {}

    #[test]
    fn convert_obj_roundtrip() {
        let interp = crate::interpreter().expect("init");
        let spec = interp.0.borrow_mut().def_class::<Container>(
            "Container",
            None,
            Some(rust_data_free::<Container>),
        );
        spec.borrow_mut().mrb_value_is_rust_backed(true);
        spec.borrow_mut()
            .add_method("value", Container::value, sys::mrb_args_none());
        spec.borrow().define(&interp).expect("class install");
        let obj = Container {
            inner: "contained string contents".to_owned(),
        };

        let value = unsafe { obj.try_into_ruby(&interp, None) }.expect("convert");
        let class = value.funcall::<Value>("class", &[], None).expect("funcall");
        assert_eq!(class.to_s(), "Container");
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
        let spec = interp.0.borrow_mut().def_class::<Container>(
            "Container",
            None,
            Some(rust_data_free::<Container>),
        );
        spec.borrow_mut().mrb_value_is_rust_backed(true);
        spec.borrow_mut()
            .add_method("value", Container::value, sys::mrb_args_none());
        spec.borrow().define(&interp).expect("class install");
        let spec = interp.0.borrow_mut().def_class::<Box<Other>>(
            "Other",
            None,
            Some(rust_data_free::<Container>),
        );
        spec.borrow_mut().mrb_value_is_rust_backed(true);
        spec.borrow().define(&interp).expect("class install");

        let value = interp.convert("string");
        let class = value.funcall::<Value>("class", &[], None).expect("funcall");
        assert_eq!(class.to_s(), "String");
        let data = unsafe { Container::try_from_ruby(&interp, &value) };
        assert!(data.is_err());
        let value =
            unsafe { Box::new(Other::default()).try_into_ruby(&interp, None) }.expect("convert");
        let class = value.funcall::<Value>("class", &[], None).expect("funcall");
        assert_eq!(class.to_s(), "Other");
        let data = unsafe { Container::try_from_ruby(&interp, &value) };
        assert!(data.is_err());
    }
}
