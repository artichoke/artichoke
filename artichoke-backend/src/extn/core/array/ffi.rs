use std::convert::TryFrom;
use std::ptr;
use std::slice;

use crate::extn::core::array::Array;
use crate::extn::prelude::*;
use crate::gc::{MrbGarbageCollection, State as GcState};

// MRB_API mrb_value mrb_ary_new(mrb_state *mrb);
#[no_mangle]
unsafe extern "C" fn artichoke_ary_new(mrb: *mut sys::mrb_state) -> sys::mrb_value {
    let mut interp = unwrap_interpreter!(mrb);
    let mut guard = Guard::new(&mut interp);
    let result = Array::default();
    let result = Array::alloc_value(result, &mut guard);
    match result {
        Ok(value) => value.inner(),
        Err(exception) => exception::raise(guard, exception),
    }
}

// MRB_API mrb_value mrb_ary_new_capa(mrb_state*, mrb_int);
#[no_mangle]
unsafe extern "C" fn artichoke_ary_new_capa(
    mrb: *mut sys::mrb_state,
    capa: sys::mrb_int,
) -> sys::mrb_value {
    let mut interp = unwrap_interpreter!(mrb);
    let mut guard = Guard::new(&mut interp);
    let capacity = usize::try_from(capa).unwrap_or_default();
    let result = Array::with_capacity(capacity);
    let result = Array::alloc_value(result, &mut guard);
    match result {
        Ok(value) => value.inner(),
        Err(exception) => exception::raise(guard, exception),
    }
}

// MRB_API mrb_value mrb_ary_new_from_values(mrb_state *mrb, mrb_int size, const mrb_value *vals);
#[no_mangle]
unsafe extern "C" fn artichoke_ary_new_from_values(
    mrb: *mut sys::mrb_state,
    size: sys::mrb_int,
    vals: *const sys::mrb_value,
) -> sys::mrb_value {
    let mut interp = unwrap_interpreter!(mrb);
    let mut guard = Guard::new(&mut interp);
    let size = usize::try_from(size).unwrap_or_default();
    let values = slice::from_raw_parts(vals, size);
    let result = Array::from(values);
    let result = Array::alloc_value(result, &mut guard);
    match result {
        Ok(value) => {
            let basic = sys::mrb_sys_basic_ptr(value.inner());
            sys::mrb_write_barrier(mrb, basic);
            value.inner()
        }
        Err(exception) => exception::raise(guard, exception),
    }
}

// MRB_API mrb_value mrb_assoc_new(mrb_state *mrb, mrb_value car, mrb_value cdr)
#[no_mangle]
unsafe extern "C" fn artichoke_ary_new_assoc(
    mrb: *mut sys::mrb_state,
    one: sys::mrb_value,
    two: sys::mrb_value,
) -> sys::mrb_value {
    let mut interp = unwrap_interpreter!(mrb);
    let mut guard = Guard::new(&mut interp);
    let result = Array::assoc(one.into(), two.into());
    let result = Array::alloc_value(result, &mut guard);
    match result {
        Ok(value) => {
            let basic = sys::mrb_sys_basic_ptr(value.inner());
            sys::mrb_write_barrier(mrb, basic);
            value.inner()
        }
        Err(exception) => exception::raise(guard, exception),
    }
}

// MRB_API mrb_value mrb_ary_splat(mrb_state *mrb, mrb_value value);
#[no_mangle]
unsafe extern "C" fn artichoke_ary_splat(
    mrb: *mut sys::mrb_state,
    value: sys::mrb_value,
) -> sys::mrb_value {
    let mut interp = unwrap_interpreter!(mrb);
    let mut guard = Guard::new(&mut interp);
    let mut value = Value::from(value);
    let result = if Array::unbox_from_value(&mut value, &mut guard).is_ok() {
        Ok(value)
    } else {
        let mut result = Array::with_capacity(1);
        result.push(value);
        Array::alloc_value(result, &mut guard)
    };
    match result {
        Ok(value) => value.inner(),
        Err(exception) => exception::raise(guard, exception),
    }
}

// MRB_API void mrb_ary_concat(mrb_state *mrb, mrb_value self, mrb_value other);
#[no_mangle]
unsafe extern "C" fn artichoke_ary_concat(
    mrb: *mut sys::mrb_state,
    ary: sys::mrb_value,
    other: sys::mrb_value,
) -> sys::mrb_value {
    let mut interp = unwrap_interpreter!(mrb);
    let mut guard = Guard::new(&mut interp);
    let mut ary = Value::from(ary);
    let other = Value::from(other);
    let result = if let Ok(mut array) = Array::unbox_from_value(&mut ary, &mut guard) {
        let prior_gc_state = guard.disable_gc();

        let result = array.concat(&mut guard, other);

        if let GcState::Enabled = prior_gc_state {
            guard.enable_gc();
        }
        result
    } else {
        Ok(())
    };
    match result {
        Ok(()) => {
            let basic = sys::mrb_sys_basic_ptr(ary.inner());
            sys::mrb_write_barrier(mrb, basic);
            ary.inner()
        }
        Err(exception) => exception::raise(guard, exception),
    }
}

// MRB_API mrb_value mrb_ary_pop(mrb_state *mrb, mrb_value ary);
#[no_mangle]
unsafe extern "C" fn artichoke_ary_pop(
    mrb: *mut sys::mrb_state,
    ary: sys::mrb_value,
) -> sys::mrb_value {
    let mut interp = unwrap_interpreter!(mrb);
    let mut guard = Guard::new(&mut interp);
    let mut ary = Value::from(ary);
    let result = if let Ok(mut array) = Array::unbox_from_value(&mut ary, &mut guard) {
        guard.convert(array.pop())
    } else {
        Value::nil()
    };
    let basic = sys::mrb_sys_basic_ptr(ary.inner());
    sys::mrb_write_barrier(mrb, basic);
    result.inner()
}

// MRB_API void mrb_ary_push(mrb_state *mrb, mrb_value array, mrb_value value);
#[no_mangle]
unsafe extern "C" fn artichoke_ary_push(
    mrb: *mut sys::mrb_state,
    ary: sys::mrb_value,
    value: sys::mrb_value,
) -> sys::mrb_value {
    let mut interp = unwrap_interpreter!(mrb);
    let mut guard = Guard::new(&mut interp);
    let mut ary = Value::from(ary);
    let value = Value::from(value);
    if let Ok(mut array) = Array::unbox_from_value(&mut ary, &mut guard) {
        array.push(value);
    }
    let basic = sys::mrb_sys_basic_ptr(ary.inner());
    sys::mrb_write_barrier(mrb, basic);
    ary.inner()
}

// MRB_API mrb_value mrb_ary_ref(mrb_state *mrb, mrb_value ary, mrb_int n);
#[no_mangle]
unsafe extern "C" fn artichoke_ary_ref(
    mrb: *mut sys::mrb_state,
    ary: sys::mrb_value,
    offset: sys::mrb_int,
) -> sys::mrb_value {
    let mut interp = unwrap_interpreter!(mrb);
    let mut guard = Guard::new(&mut interp);
    let mut ary = Value::from(ary);
    let offset = usize::try_from(offset).unwrap_or_default();
    let result = if let Ok(array) = Array::unbox_from_value(&mut ary, &mut guard) {
        guard.convert(array.get(offset))
    } else {
        Value::nil()
    };
    result.inner()
}

// MRB_API void mrb_ary_set(mrb_state *mrb, mrb_value ary, mrb_int n, mrb_value val);
#[no_mangle]
unsafe extern "C" fn artichoke_ary_set(
    mrb: *mut sys::mrb_state,
    ary: sys::mrb_value,
    offset: sys::mrb_int,
    value: sys::mrb_value,
) -> sys::mrb_value {
    let mut interp = unwrap_interpreter!(mrb);
    let mut guard = Guard::new(&mut interp);
    let mut array = Value::from(ary);
    let value = Value::from(value);
    if let Ok(mut array) = Array::unbox_from_value(&mut array, &mut guard) {
        let offset = if let Ok(offset) = usize::try_from(offset) {
            offset
        } else {
            let array_len = array.len();
            let offset = offset
                .checked_neg()
                .and_then(|offset| usize::try_from(offset).ok())
                .and_then(|offset| array_len.checked_sub(offset));
            if let Some(offset) = offset {
                offset
            } else {
                0
            }
        };
        // TODO: properly handle self-referential sets.
        if Value::from(ary) != value {
            array.set(offset, value);
        }
    }
    let basic = sys::mrb_sys_basic_ptr(ary);
    sys::mrb_write_barrier(mrb, basic);
    value.inner()
}

#[no_mangle]
unsafe extern "C" fn artichoke_ary_shift(
    mrb: *mut sys::mrb_state,
    ary: sys::mrb_value,
) -> sys::mrb_value {
    let mut interp = unwrap_interpreter!(mrb);
    let mut guard = Guard::new(&mut interp);
    let mut array = Value::from(ary);
    let result = if let Ok(mut array) = Array::unbox_from_value(&mut array, &mut guard) {
        let result = array.get(0);
        let _ = array.set_slice(0, 1, &[]);
        guard.convert(result)
    } else {
        Value::nil()
    };
    let basic = sys::mrb_sys_basic_ptr(ary);
    sys::mrb_write_barrier(mrb, basic);
    result.inner()
}

#[no_mangle]
unsafe extern "C" fn artichoke_ary_unshift(
    mrb: *mut sys::mrb_state,
    ary: sys::mrb_value,
    value: sys::mrb_value,
) -> sys::mrb_value {
    let mut interp = unwrap_interpreter!(mrb);
    let mut guard = Guard::new(&mut interp);
    let mut array = Value::from(ary);
    let value = Value::from(value);
    if let Ok(mut array) = Array::unbox_from_value(&mut array, &mut guard) {
        let _ = array.set_with_drain(0, 0, value);
    }
    let basic = sys::mrb_sys_basic_ptr(ary);
    sys::mrb_write_barrier(mrb, basic);
    value.inner()
}

#[no_mangle]
unsafe extern "C" fn artichoke_ary_len(
    mrb: *mut sys::mrb_state,
    ary: sys::mrb_value,
) -> sys::mrb_int {
    let mut interp = unwrap_interpreter!(mrb, or_else = 0);
    let mut guard = Guard::new(&mut interp);
    let mut ary = Value::from(ary);
    if let Ok(array) = Array::unbox_from_value(&mut ary, &mut guard) {
        Int::try_from(array.len()).unwrap_or_default()
    } else {
        0
    }
}

#[no_mangle]
unsafe extern "C" fn artichoke_ary_set_len(
    mrb: *mut sys::mrb_state,
    ary: sys::mrb_value,
    len: sys::mrb_int,
) {
    let mut interp = unwrap_interpreter!(mrb, or_else = ());
    let mut guard = Guard::new(&mut interp);
    let mut ary = Value::from(ary);
    if let Ok(mut array) = Array::unbox_from_value(&mut ary, &mut guard) {
        let len = usize::try_from(len).unwrap_or_default();
        array.0.set_len(len);
    }
}

#[no_mangle]
unsafe extern "C" fn artichoke_ary_ptr(
    mrb: *mut sys::mrb_state,
    ary: sys::mrb_value,
) -> *mut sys::mrb_value {
    let mut interp = unwrap_interpreter!(mrb, or_else = ptr::null_mut());
    let mut guard = Guard::new(&mut interp);
    let mut ary = Value::from(ary);
    if let Ok(mut array) = Array::unbox_from_value(&mut ary, &mut guard) {
        array.0.as_mut_ptr()
    } else {
        ptr::null_mut()
    }
}

#[no_mangle]
unsafe extern "C" fn artichoke_ary_check(
    mrb: *mut sys::mrb_state,
    ary: sys::mrb_value,
) -> sys::mrb_bool {
    let mut interp = unwrap_interpreter!(mrb, or_else = 0);
    let mut guard = Guard::new(&mut interp);
    let mut ary = Value::from(ary);
    if Array::unbox_from_value(&mut ary, &mut guard).is_ok() {
        1
    } else {
        0
    }
}

#[no_mangle]
unsafe extern "C" fn artichoke_gc_mark_ary(mrb: *mut sys::mrb_state, ary: sys::mrb_value) {
    let mut interp = unwrap_interpreter!(mrb, or_else = ());
    let mut guard = Guard::new(&mut interp);
    let mut array = Value::from(ary);
    if let Ok(array) = Array::unbox_from_value(&mut array, &mut guard) {
        array.gc_mark(&mut guard);
    }
}

#[no_mangle]
unsafe extern "C" fn artichoke_gc_mark_ary_size(
    mrb: *mut sys::mrb_state,
    ary: sys::mrb_value,
) -> usize {
    let mut interp = unwrap_interpreter!(mrb, or_else = 0);
    let mut guard = Guard::new(&mut interp);
    let mut array = Value::from(ary);
    if let Ok(array) = Array::unbox_from_value(&mut array, &mut guard) {
        array.real_children()
    } else {
        0
    }
}
