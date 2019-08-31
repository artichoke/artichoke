#![deny(clippy::all, clippy::pedantic)]
#![deny(warnings, intra_doc_link_resolution_failure)]

//! This integration test checks for segfaults that stem from the improperly
//! holding a borrow on the interpreter in converters that prevent arbitrary
//! Rust code from taking a mutable borrow.
//!
//! This test creates a Rust-backed object and takes a mutable borrow on the
//! interpreter in its initialize method.
//!
//! If this test segfaults, we are improperly holding a borrow on the
//! interpreter while calling `mrb_obj_new`.

#[macro_use]
extern crate artichoke_backend;

use artichoke_backend::convert::RustBackedValue;
use artichoke_backend::def::{ClassLike, Define};
use artichoke_backend::sys;

struct Obj;

impl RustBackedValue for Obj {}

unsafe extern "C" fn initialize(mrb: *mut sys::mrb_state, slf: sys::mrb_value) -> sys::mrb_value {
    let interp = unwrap_interpreter!(mrb);
    interp.0.borrow_mut();
    slf
}

#[test]
fn obj_new_borrow_mut() {
    let interp = artichoke_backend::interpreter().expect("init");
    let class = interp.0.borrow_mut().def_class::<Obj>("Obj", None, None);
    class
        .borrow_mut()
        .add_method("initialize", initialize, sys::mrb_args_none());
    class.borrow().define(&interp).unwrap();
    unsafe {
        Obj.try_into_ruby(&interp, None).unwrap();
    }
}
