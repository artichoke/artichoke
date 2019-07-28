use std::convert::TryFrom;

use crate::convert::{Convert, Error, TryConvert};
use crate::sys;
use crate::value::types::{Ruby, Rust};
use crate::value::Value;
use crate::Mrb;

mod boolean;
mod bytes;
mod fixnum;
mod float;
mod string;

pub use self::boolean::*;
pub use self::bytes::*;
pub use self::fixnum::*;
pub use self::float::*;
pub use self::string::*;

// bail out implementation for mixed-type collections
impl Convert<Vec<Value>> for Value {
    type From = Rust;
    type To = Ruby;

    fn convert(interp: &Mrb, value: Vec<Self>) -> Self {
        let mrb = interp.borrow().mrb;
        let array =
            unsafe { sys::mrb_ary_new_capa(mrb, i64::try_from(value.len()).unwrap_or_default()) };

        for (idx, item) in value.iter().enumerate() {
            unsafe {
                sys::mrb_ary_set(
                    mrb,
                    array,
                    i64::try_from(idx).unwrap_or_default(),
                    item.inner(),
                )
            };
        }
        Self::new(interp, array)
    }
}

impl Convert<Vec<Option<Value>>> for Value {
    type From = Rust;
    type To = Ruby;

    fn convert(interp: &Mrb, value: Vec<Option<Self>>) -> Self {
        let mrb = interp.borrow().mrb;
        let array =
            unsafe { sys::mrb_ary_new_capa(mrb, i64::try_from(value.len()).unwrap_or_default()) };

        for (idx, item) in value.iter().enumerate() {
            if let Some(item) = item {
                unsafe {
                    sys::mrb_ary_set(
                        mrb,
                        array,
                        i64::try_from(idx).unwrap_or_default(),
                        item.inner(),
                    )
                };
            } else {
                unsafe {
                    sys::mrb_ary_set(
                        mrb,
                        array,
                        i64::try_from(idx).unwrap_or_default(),
                        sys::mrb_sys_nil_value(),
                    )
                };
            }
        }
        Self::new(interp, array)
    }
}

impl TryConvert<Value> for Vec<Value> {
    type From = Ruby;
    type To = Rust;

    unsafe fn try_convert(interp: &Mrb, value: Value) -> Result<Self, Error<Self::From, Self::To>> {
        let mrb = interp.borrow().mrb;
        match value.ruby_type() {
            Ruby::Array => {
                let array = value.inner();
                let size = sys::mrb_sys_ary_len(array);
                let cap = usize::try_from(size).map_err(|_| Error {
                    from: Ruby::Array,
                    to: Rust::Vec,
                })?;
                let mut items = Self::with_capacity(cap);
                for idx in 0..size {
                    let item = Value::new(interp, sys::mrb_ary_ref(mrb, array, idx));
                    items.push(item);
                }
                Ok(items)
            }
            type_tag => Err(Error {
                from: type_tag,
                to: Rust::Vec,
            }),
        }
    }
}

impl TryConvert<Value> for Vec<Option<Value>> {
    type From = Ruby;
    type To = Rust;

    unsafe fn try_convert(interp: &Mrb, value: Value) -> Result<Self, Error<Self::From, Self::To>> {
        let mrb = interp.borrow().mrb;
        match value.ruby_type() {
            Ruby::Array => {
                let array = value.inner();
                let size = sys::mrb_sys_ary_len(array);
                let cap = usize::try_from(size).map_err(|_| Error {
                    from: Ruby::Array,
                    to: Rust::Vec,
                })?;
                let mut items = Self::with_capacity(cap);
                for idx in 0..size {
                    let element = sys::mrb_ary_ref(mrb, array, idx);
                    if sys::mrb_sys_value_is_nil(element) {
                        items.push(None);
                    } else {
                        items.push(Some(Value::new(interp, element)));
                    }
                }
                Ok(items)
            }
            type_tag => Err(Error {
                from: type_tag,
                to: Rust::Vec,
            }),
        }
    }
}
