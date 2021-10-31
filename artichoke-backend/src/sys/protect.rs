use std::ffi::c_void;
use std::mem;
use std::ptr::{self, NonNull};

use crate::sys;

pub unsafe fn funcall(
    mrb: *mut sys::mrb_state,
    slf: sys::mrb_value,
    func: sys::mrb_sym,
    args: &[sys::mrb_value],
    block: Option<sys::mrb_value>,
) -> Result<sys::mrb_value, sys::mrb_value> {
    let data = Funcall { slf, func, args, block };
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

unsafe fn protect<T>(mrb: *mut sys::mrb_state, data: T) -> Result<sys::mrb_value, sys::mrb_value>
where
    T: Protect,
{
    let data = Box::new(data);
    let data = Box::into_raw(data);
    let data = sys::mrb_sys_cptr_value(mrb, data.cast::<c_void>());
    let mut state = 0;

    let value = sys::mrb_protect(mrb, Some(T::run), data, &mut state);
    if let Some(exc) = NonNull::new((*mrb).exc) {
        (*mrb).exc = ptr::null_mut();
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

// `Funcall` must be `Copy` because we may unwind past the frames in which
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
        let Self { slf, func, args, block } = *Box::from_raw(ptr.cast::<Self>());

        // This will always unwrap because we've already checked that we
        // have fewer than `MRB_FUNCALL_ARGC_MAX` args, which is less than
        // i64 max value.
        let argslen = if let Ok(argslen) = i64::try_from(args.len()) {
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

// `Eval` must be `Copy` because we may unwind past the frames in which
// it is used with `longjmp` which does not allow Rust  to run destructors.
#[derive(Clone, Copy)]
struct Eval<'a> {
    context: *mut sys::mrbc_context,
    code: &'a [u8],
}

impl<'a> Protect for Eval<'a> {
    unsafe extern "C" fn run(mrb: *mut sys::mrb_state, data: sys::mrb_value) -> sys::mrb_value {
        let ptr = sys::mrb_sys_cptr_ptr(data);
        let Self { context, code } = *Box::from_raw(ptr.cast::<Self>());

        // Execute arbitrary ruby code, which may generate objects with C APIs
        // if backed by Rust functions.
        //
        // `mrb_load_nstring_ctx` sets the "stack keep" field on the context
        // which means the most recent value returned by eval will always be
        // considered live by the GC.
        sys::mrb_load_nstring_cxt(mrb, code.as_ptr().cast::<i8>(), code.len(), context)
    }
}

// `BlockYield` must be `Copy` because we may unwind past the frames in which
// it is used with `longjmp` which does not allow Rust  to run destructors.
#[derive(Clone, Copy)]
struct BlockYield {
    block: sys::mrb_value,
    arg: sys::mrb_value,
}

impl Protect for BlockYield {
    unsafe extern "C" fn run(mrb: *mut sys::mrb_state, data: sys::mrb_value) -> sys::mrb_value {
        let ptr = sys::mrb_sys_cptr_ptr(data);
        let Self { block, arg } = *Box::from_raw(ptr.cast::<Self>());
        sys::mrb_yield(mrb, block, arg)
    }
}

pub unsafe fn is_range(
    mrb: *mut sys::mrb_state,
    value: sys::mrb_value,
    len: i64,
) -> Result<Option<Range>, sys::mrb_value> {
    let data = IsRange { value, len };
    let is_range = protect(mrb, data)?;
    if sys::mrb_sys_value_is_nil(is_range) {
        Ok(None)
    } else {
        let ptr = sys::mrb_sys_cptr_ptr(is_range);
        let out = *Box::from_raw(ptr.cast::<Range>());
        Ok(Some(out))
    }
}

#[derive(Default, Debug, Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct Range {
    pub start: sys::mrb_int,
    pub len: sys::mrb_int,
}

// `IsRange` must be `Copy` because we may unwind past the frames in which
// it is used with `longjmp` which does not allow Rust  to run destructors.
#[derive(Default, Debug, Clone, Copy)]
struct IsRange {
    value: sys::mrb_value,
    len: sys::mrb_int,
}

impl Protect for IsRange {
    unsafe extern "C" fn run(mrb: *mut sys::mrb_state, data: sys::mrb_value) -> sys::mrb_value {
        let ptr = sys::mrb_sys_cptr_ptr(data);
        let Self { value, len } = *Box::from_raw(ptr.cast::<Self>());
        let mut start = mem::MaybeUninit::<sys::mrb_int>::uninit();
        let mut range_len = mem::MaybeUninit::<sys::mrb_int>::uninit();
        let check_range = sys::mrb_range_beg_len(mrb, value, start.as_mut_ptr(), range_len.as_mut_ptr(), len, 0_u8);
        if check_range == sys::mrb_range_beg_len::MRB_RANGE_OK {
            let start = start.assume_init();
            let range_len = range_len.assume_init();
            let out = Range { start, len: range_len };
            let out = Box::new(out);
            let out = Box::into_raw(out);
            sys::mrb_sys_cptr_value(mrb, out.cast::<c_void>())
        } else {
            sys::mrb_sys_nil_value()
        }
    }
}
