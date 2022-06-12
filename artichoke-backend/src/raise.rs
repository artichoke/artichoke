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
    panic::panic_any(ExceptionPayload { inner: exc });
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
    let mut arena = interp
        .create_arena_savepoint()
        .map_err(|err| Fatal::from(err.to_string()))?;
    let body = body.ok_or_else(|| Fatal::with_message("null function passed to mrb_protect"))?;

    //let hook = panic::take_hook();
    //panic::set_hook(Box::new(|_| {}));

    let result = arena.with_ffi_boundary(|mrb| panic::catch_unwind(panic::AssertUnwindSafe(|| body(mrb, data))));

    //panic::set_hook(hook);
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
