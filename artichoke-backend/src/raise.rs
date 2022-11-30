use std::panic;
use std::ptr;

use spinoso_exception::Fatal;

use crate::error::{Error, RubyException};
use crate::gc::MrbGarbageCollection;
use crate::sys;
use crate::value::Value;
use crate::Artichoke;

/// Panic payload for unwinding an mruby interpreter with an `Exception` `mrb_value`.
struct ExceptionPayload {
    inner: Value,
}

// SAFETY: this panic payload never crosses a thread boundary.
//
// - mruby is single threaded and cannot be `Send`.
// - This struct is used directly in `std::panic::resume_unwind` and is not used
//   by `std::thread::JoinHandle::join`.
// - This struct is not publicly exported, so it cannot be downcast to outside
//   of a single threaded panic context.
unsafe impl Send for ExceptionPayload {}

// NOTE: this function is aliased to the `static` function `exc_throw` with a
// macro in `exception.c` depending on the value of the `ARTICHOKE` macro.
//
// ```c
// static mrb_noreturn void
// exc_throw(mrb_state *mrb, mrb_value exc)
// ```
#[no_mangle]
unsafe extern "C-unwind" fn artichoke_exc_throw(mrb: *mut sys::mrb_state, exc: sys::mrb_value) -> ! {
    let _ = mrb;
    // Use `panic::resume_unwind` instead of `panic_any!` here to avoid running
    // the built-in panic hook and printing stack to stderr.
    panic::resume_unwind(Box::new(ExceptionPayload {
        inner: Value::from(exc),
    }));
}

// ```c
// typedef mrb_value mrb_protect_error_func(mrb_state *mrb, void *userdata);
// MRB_API mrb_value mrb_protect_error(mrb_state *mrb, mrb_protect_error_func *body, void *userdata, mrb_bool *error);
// ```
//
// NOTE: This remains implemented in mruby in `vm.c`.

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
        Ok(value) => value.inner(),
        Err(exc) => exc.as_mrb_value(&mut guard).unwrap_or_default(),
    }
}

unsafe fn mrb_protect_inner(
    interp: &mut Artichoke,
    body: sys::mrb_func_t,
    data: sys::mrb_value,
    state: *mut sys::mrb_bool,
) -> Result<Value, Error> {
    if !state.is_null() {
        *state = false.into();
    }

    let body = body.ok_or_else(|| Fatal::with_message("null function passed to mrb_protect"))?;

    // stack frame munging
    let c = (*interp.mrb.as_ptr()).c;
    let ci_index = (*c).ci.offset_from((*c).cibase);

    let result = {
        let mut arena = interp.create_arena_savepoint()?;
        arena.with_ffi_boundary(|mrb| {
            panic::catch_unwind(panic::AssertUnwindSafe(|| {
                let result = body(mrb, data);
                Value::from(result)
            }))
        })
    };

    match result {
        Ok(Ok(value)) => Ok(interp.protect(value)),
        Ok(Err(payload)) => {
            let mrb = interp.mrb.as_ptr();
            (*mrb).exc = ptr::null_mut();

            if !state.is_null() {
                *state = true.into();
            }

            if (*mrb).c == c {
                while (*c).ci.offset_from((*c).cibase) > ci_index {
                    cipop(mrb);
                }
            } else {
                // Fiber code. not implemented.
                unreachable!("mrb_protect fiber failure pathway");
            }

            let exc = if let Some(payload) = payload.downcast_ref::<ExceptionPayload>() {
                payload.inner
            } else {
                // Something other than `mrb_raise` resulted in a `panic!`. This
                // is likely due to a programming error in Rust code, so propagate
                // the panic so we can crash the process.
                panic::resume_unwind(payload);
            };
            Ok(exc)
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
        Ok(value) => value.inner(),
        Err(exc) => exc.as_mrb_value(&mut guard).unwrap_or_default(),
    }
}

unsafe fn mrb_ensure_inner(
    interp: &mut Artichoke,
    body: sys::mrb_func_t,
    body_data: sys::mrb_value,
    ensure: sys::mrb_func_t,
    ensure_data: sys::mrb_value,
) -> Result<Value, Error> {
    let body = body.ok_or_else(|| Fatal::with_message("null function passed to mrb_protect"))?;
    let ensure = ensure.ok_or_else(|| Fatal::with_message("null function passed to mrb_protect"))?;

    // stack frame munging
    let c = (*interp.mrb.as_ptr()).c;
    let ci_index = (*c).ci.offset_from((*c).cibase);

    let mut arena = interp.create_arena_savepoint()?;

    let result = arena.with_ffi_boundary(|mrb| {
        panic::catch_unwind(panic::AssertUnwindSafe(|| {
            let result = body(mrb, body_data);
            Value::from(result)
        }))
    });

    match result {
        Ok(Ok(value)) => {
            let _ = arena.with_ffi_boundary(|mrb| ensure(mrb, ensure_data))?;
            arena.restore();
            Ok(interp.protect(value))
        }
        Ok(Err(payload)) => {
            let mrb = arena.interp().mrb.as_ptr();
            (*mrb).exc = ptr::null_mut();

            if (*mrb).c == c {
                while (*c).ci.offset_from((*c).cibase) > ci_index {
                    cipop(mrb);
                }
            } else {
                // Fiber code. not implemented.
                unreachable!("mrb_protect fiber failure pathway");
            }

            if payload.downcast_ref::<ExceptionPayload>().is_none() {
                // Something other than `mrb_raise` resulted in a `panic!`. This
                // is likely due to a programming error in Rust code, so propagate
                // the panic so we can crash the process.
                panic::resume_unwind(payload);
            };
            let _ = arena.with_ffi_boundary(|mrb| ensure(mrb, ensure_data))?;
            arena.restore();
            // `mrb_ensure` continues to unwind if an `Exception` was triggered.
            panic::resume_unwind(payload);
        }
        Err(err) => Err(err.into()),
    }
}

// TODO: not implemented.
//
// ```c
// MRB_API mrb_value
// mrb_rescue(mrb_state *mrb, mrb_func_t body, mrb_value b_data,
//            mrb_func_t rescue, mrb_value r_data)
//
// MRB_API mrb_value
// mrb_rescue_exceptions(mrb_state *mrb, mrb_func_t body, mrb_value b_data, mrb_func_t rescue, mrb_value r_data,
//                       mrb_int len, struct RClass **classes)
// ```

// ```c
// static inline mrb_callinfo*
// cipop(mrb_state *mrb)
// {
//   struct mrb_context *c = mrb->c;
//   struct REnv *env = mrb_vm_ci_env(c->ci);
//
//   c->ci--;
//   if (env) mrb_env_unshare(mrb, env);
//   return c->ci;
// }
// ```
unsafe fn cipop(mrb: *mut sys::mrb_state) -> *mut sys::mrb_callinfo {
    let c = (*mrb).c;
    let env = mrb_vm_ci_env((*c).ci);

    (*c).ci = (*c).ci.offset(-1);
    if !env.is_null() {
        sys::mrb_env_unshare(mrb, env);
    }
    (*c).ci
}

// ```c
// static inline struct REnv *
// mrb_vm_ci_env(const mrb_callinfo *ci)
// {
//   if (ci->u.env && ci->u.env->tt == MRB_TT_ENV) {
//     return ci->u.env;
//   }
//   else {
//     return NULL;
//   }
// }
// ```
unsafe fn mrb_vm_ci_env(ci: *const sys::mrb_callinfo) -> *mut sys::REnv {
    let env = (*ci).u.env;
    if env.is_null() {
        return ptr::null_mut();
    }
    if (*env).tt() != sys::mrb_vtype::MRB_TT_ENV {
        return ptr::null_mut();
    }
    env
}
