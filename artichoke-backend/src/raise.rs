use std::panic;
use std::ptr;

use spinoso_exception::Fatal;

use crate::error::{Error, RubyException};
use crate::gc::MrbGarbageCollection;
use crate::sys;
use crate::Artichoke;

struct ExceptionPayload {
    inner: sys::mrb_value,
}

// mruby is single threaded.
unsafe impl Send for ExceptionPayload {}

// ```c
// static mrb_noreturn void
// exc_throw(mrb_state *mrb, mrb_value exc)
// ```
#[no_mangle]
unsafe extern "C-unwind" fn exc_throw(mrb: *mut sys::mrb_state, exc: sys::mrb_value) -> ! {
    let _ = mrb;
    panic::resume_unwind(Box::new(ExceptionPayload { inner: exc }));
}

// ```c
// MRB_API mrb_value
// mrb_protect(mrb_state *mrb, mrb_func_t body, mrb_value data, mrb_bool *state)
// ```
#[no_mangle]
unsafe extern "C-unwind" fn mrb_protect(
    mrb: *mut sys::mrb_state,
    body: sys::mrb_func_t,
    data: sys::mrb_value,
    state: *mut sys::mrb_bool,
) -> sys::mrb_value {
    unwrap_interpreter!(mrb, to => guard);
    match mrb_protect_inner(&mut guard, body, data, state) {
        Ok(value) => value,
        Err(exc) => exc.as_mrb_value(&mut guard).unwrap_or_default(),
    }
}

unsafe fn mrb_protect_inner(
    interp: &mut Artichoke,
    body: sys::mrb_func_t,
    data: sys::mrb_value,
    state: *mut sys::mrb_bool,
) -> Result<sys::mrb_value, Error> {
    if !state.is_null() {
        *state = false.into();
    }

    let body = body.ok_or_else(|| Fatal::with_message("null function passed to mrb_protect"))?;
    let mut arena = interp
        .create_arena_savepoint()
        .map_err(|err| Fatal::from(err.to_string()))?;

    let result = arena.with_ffi_boundary(|mrb| panic::catch_unwind(panic::AssertUnwindSafe(|| body(mrb, data))));
    arena.restore();

    match result {
        Ok(Ok(value)) => Ok(interp.protect(value.into()).into()),
        Ok(Err(err)) => {
            if !state.is_null() {
                *state = true.into();
            }
            let mrb = interp.mrb.as_ptr();
            (*mrb).exc = ptr::null_mut();

            let exc = err
                .downcast::<ExceptionPayload>()
                .map_err(|_| Fatal::with_message("unexpected panic payload in mrb_protect"))?;
            Ok(exc.inner)
        }
        Err(err) => {
            if !state.is_null() {
                *state = true.into();
            }
            Err(err.into())
        }
    }
}

// ```c
// MRB_API mrb_value
// mrb_ensure(mrb_state *mrb, mrb_func_t body, mrb_value b_data, mrb_func_t ensure, mrb_value e_data)
// ```
#[no_mangle]
unsafe extern "C-unwind" fn mrb_ensure(
    mrb: *mut sys::mrb_state,
    body: sys::mrb_func_t,
    body_data: sys::mrb_value,
    ensure: sys::mrb_func_t,
    ensure_data: sys::mrb_value,
) -> sys::mrb_value {
    unwrap_interpreter!(mrb, to => guard);
    match mrb_ensure_inner(&mut guard, body, body_data, ensure, ensure_data) {
        Ok(value) => value,
        Err(exc) => exc.as_mrb_value(&mut guard).unwrap_or_default(),
    }
}

unsafe fn mrb_ensure_inner(
    interp: &mut Artichoke,
    body: sys::mrb_func_t,
    body_data: sys::mrb_value,
    ensure: sys::mrb_func_t,
    ensure_data: sys::mrb_value,
) -> Result<sys::mrb_value, Error> {
    let body = body.ok_or_else(|| Fatal::with_message("null function passed to mrb_protect"))?;
    let ensure = ensure.ok_or_else(|| Fatal::with_message("null function passed to mrb_protect"))?;
    let mut arena = interp
        .create_arena_savepoint()
        .map_err(|err| Fatal::from(err.to_string()))?;

    let result = arena.with_ffi_boundary(|mrb| panic::catch_unwind(panic::AssertUnwindSafe(|| body(mrb, body_data))));

    match result {
        Ok(Ok(value)) => {
            let _ = arena.with_ffi_boundary(|mrb| ensure(mrb, ensure_data))?;
            arena.restore();
            Ok(interp.protect(value.into()).into())
        }
        Ok(Err(err)) => {
            let exc = err
                .downcast::<ExceptionPayload>()
                .map_err(|_| Fatal::with_message("unexpected panic payload in mrb_protect"))?;
            let _ = arena.with_ffi_boundary(|mrb| ensure(mrb, ensure_data))?;
            arena.restore();
            panic::resume_unwind(exc);
        }
        Err(err) => Err(err.into()),
    }
}
