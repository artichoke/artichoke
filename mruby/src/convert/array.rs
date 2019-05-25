use std::convert::TryFrom;

use crate::convert::{Error, FromMrb, TryFromMrb};
use crate::interpreter::Mrb;
use crate::sys;
use crate::value::types::{Ruby, Rust};
use crate::value::Value;

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
impl FromMrb<Vec<Value>> for Value {
    type From = Rust;
    type To = Ruby;

    fn from_mrb(interp: &Mrb, value: Vec<Self>) -> Self {
        // We can initalize an `Array` with a known capacity using
        // `sys::mrb_ary_new_capa`, but doing so requires converting from
        // `usize` to `i64` which is fallible. To simplify the code and make
        // `Vec<Value>` easier to work with, use an infallible `Array`
        // constructor.
        let array = unsafe { sys::mrb_ary_new(interp.borrow().mrb) };
        let mut idx = 0;

        // Lint disabled because I should be casting or converting but do not
        // want to to preserve this converter implementation being infallible.
        // See: https://github.com/rust-lang/rust-clippy/issues/4139
        #[allow(clippy::explicit_counter_loop)]
        for item in value {
            unsafe { sys::mrb_ary_set(interp.borrow().mrb, array, idx, item.inner()) };
            idx += 1;
        }
        Self::new(interp, array)
    }
}

impl TryFromMrb<Value> for Vec<Value> {
    type From = Ruby;
    type To = Rust;

    unsafe fn try_from_mrb(
        interp: &Mrb,
        value: Value,
    ) -> Result<Self, Error<Self::From, Self::To>> {
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
                    let item =
                        Value::new(interp, sys::mrb_ary_ref(interp.borrow().mrb, array, idx));
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
