#![allow(warnings)]
//! Rust implementation of `mrb_raise` that uses `panic` to unwind.
//!
//! Use [`panic::catch_unwind`] to protect.

use std::convert::TryFrom;
use std::ffi::c_void;
use std::panic;
use std::ptr;

use crate::ffi::*;

/// Wrap calls to the mruby VM to catch panics from [`mrb_raise`].
#[no_mangle]
#[unwind(allowed)]
pub unsafe extern "C" fn mrb_protect(
    mrb: *mut mrb_state,
    body: mrb_func_t,
    data: mrb_value,
    state: *mut mrb_bool,
) -> mrb_value {
    if !state.is_null() {
        state.write(0_u8);
    }
    let unwind =
        panic::catch_unwind(|| body.map_or_else(|| mrb_sys_nil_value(), |body| body(mrb, data)));
    let result = if let Ok(result) = unwind {
        result
    } else {
        let result = mrb_sys_obj_value((*mrb).exc as *mut c_void);
        (*mrb).exc = ptr::null_mut();
        if !state.is_null() {
            state.write(1_u8);
        }
        result
    };
    mrb_gc_protect(mrb, result);
    result
}

/// Wrap calls to the mruby VM to catch panics from [`mrb_raise`].
#[no_mangle]
#[unwind(allowed)]
pub unsafe extern "C" fn mrb_sys_exc_raise(mrb: *mut mrb_state, exc: mrb_value) -> ! {
    let exc_class = mrb_class_get(mrb, b"Exception\0".as_ptr() as *const i8);
    if mrb_obj_is_kind_of(mrb, exc, exc_class) == 0_u8 {
        mrb_raise(
            mrb,
            mrb_exc_get(mrb, b"TypeError\0".as_ptr() as *const i8),
            b"exception object expected\0".as_ptr() as *const i8,
        );
    }
    mrb_exc_set(mrb, exc);
    panic!("unwinding Ruby exception");
}

unsafe fn mrb_exc_set(mrb: *mut mrb_state, exc: mrb_value) {
    println!("exc_set");
    if mrb_sys_value_is_nil(exc) {
        (*mrb).exc = ptr::null_mut();
    } else {
        (*mrb).exc = mrb_sys_obj_ptr(exc);
        let idx = mrb_sys_gc_arena_save(mrb);
        let exc_rbasic = mrb_sys_basic_ptr(exc);
        if let Ok(offset) = isize::try_from(idx - 1) {
            let arena_end = (*mrb).gc.arena.offset(offset) as *mut RBasic;
            if idx > 0 && ptr::eq(exc_rbasic, arena_end) {
                mrb_sys_gc_arena_restore(mrb, idx - 1);
            }
        }
    }
}
