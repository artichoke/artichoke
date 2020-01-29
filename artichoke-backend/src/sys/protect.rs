use std::convert::TryFrom;
use std::ffi::c_void;
use std::ptr::NonNull;

use crate::sys;
use crate::types::Int;

pub unsafe fn funcall(
    mrb: *mut sys::mrb_state,
    slf: sys::mrb_value,
    func: sys::mrb_sym,
    args: &[sys::mrb_value],
    block: Option<sys::mrb_value>,
) -> Result<sys::mrb_value, sys::mrb_value> {
    let data = Funcall {
        slf,
        func,
        args,
        block,
    };
    protect(mrb, data)
}

pub unsafe fn eval(
    mrb: *mut sys::mrb_state,
    context: *mut sys::mrbc_context,
    code: &[u8],
) -> Result<sys::mrb_value, sys::mrb_value> {
    let data = Eval { context, code };
    protect(mrb, data)
}

pub unsafe fn block_yield(
    mrb: *mut sys::mrb_state,
    block: sys::mrb_value,
    arg: sys::mrb_value,
) -> Result<sys::mrb_value, sys::mrb_value> {
    let data = BlockYield { block, arg };
    protect(mrb, data)
}

unsafe fn protect<T: Protect>(
    mrb: *mut sys::mrb_state,
    data: T,
) -> Result<sys::mrb_value, sys::mrb_value> {
    let data = Box::new(data);
    let data = Box::into_raw(data);
    let data = sys::mrb_sys_cptr_value(mrb, data as *mut c_void);
    let mut state = 0;

    let value = sys::mrb_protect(mrb, Some(T::run), data, &mut state);
    if let Some(exc) = NonNull::new((*mrb).exc) {
        Err(sys::mrb_sys_obj_value(exc.cast::<c_void>().as_ptr()))
    } else if state == 0 {
        Ok(value)
    } else {
        Err(value)
    }
}

trait Protect {
    unsafe extern "C" fn run(mrb: *mut sys::mrb_state, data: sys::mrb_value) -> sys::mrb_value;
}

// `Funcall` must be `Copy` because the we may unwind past the frames in which
// it is used with `longjmp` which does not allow Rust  to run destructors.
#[derive(Clone, Copy)]
struct Funcall<'a> {
    slf: sys::mrb_value,
    func: u32,
    args: &'a [sys::mrb_value],
    block: Option<sys::mrb_value>,
}

impl<'a> Protect for Funcall<'a> {
    unsafe extern "C" fn run(mrb: *mut sys::mrb_state, data: sys::mrb_value) -> sys::mrb_value {
        let ptr = sys::mrb_sys_cptr_ptr(data);
        // `protect` must be `Copy` because the call to a function in the
        // `mrb_funcall...` family can unwind with `longjmp` which does not
        // allow Rust to run destructors.
        let Self {
            slf,
            func,
            args,
            block,
        } = *Box::from_raw(ptr as *mut Self);

        // This will always unwrap because we've already checked that we
        // have fewer than `MRB_FUNCALL_ARGC_MAX` args, which is less than
        // i64 max value.
        let argslen = if let Ok(argslen) = Int::try_from(args.len()) {
            argslen
        } else {
            return sys::mrb_sys_nil_value();
        };

        if let Some(block) = block {
            sys::mrb_funcall_with_block(mrb, slf, func, argslen, args.as_ptr(), block)
        } else {
            sys::mrb_funcall_argv(mrb, slf, func, argslen, args.as_ptr())
        }
    }
}

// `Eval` must be `Copy` because the we may unwind past the frames in which
// it is used with `longjmp` which does not allow Rust  to run destructors.
#[derive(Clone, Copy)]
struct Eval<'a> {
    context: *mut sys::mrbc_context,
    code: &'a [u8],
}

impl<'a> Protect for Eval<'a> {
    unsafe extern "C" fn run(mrb: *mut sys::mrb_state, data: sys::mrb_value) -> sys::mrb_value {
        let ptr = sys::mrb_sys_cptr_ptr(data);
        let Self { context, code } = *Box::from_raw(ptr as *mut Self);

        // Execute arbitrary ruby code, which may generate objects with C APIs
        // if backed by Rust functions.
        //
        // `mrb_load_nstring_ctx` sets the "stack keep" field on the context
        // which means the most recent value returned by eval will always be
        // considered live by the GC.
        sys::mrb_load_nstring_cxt(mrb, code.as_ptr() as *const i8, code.len(), context)
    }
}

// `BlockYield` must be `Copy` because the we may unwind past the frames in which
// it is used with `longjmp` which does not allow Rust  to run destructors.
#[derive(Clone, Copy)]
struct BlockYield {
    block: sys::mrb_value,
    arg: sys::mrb_value,
}

impl Protect for BlockYield {
    unsafe extern "C" fn run(mrb: *mut sys::mrb_state, data: sys::mrb_value) -> sys::mrb_value {
        let ptr = sys::mrb_sys_cptr_ptr(data);
        let Self { block, arg } = *Box::from_raw(ptr as *mut Self);
        sys::mrb_yield(mrb, block, arg)
    }
}
