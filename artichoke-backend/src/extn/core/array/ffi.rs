use std::convert::TryFrom;
use std::ptr;
use std::slice;

use crate::extn::core::array::{Array, ArrayType, InlineBuffer};
use crate::extn::prelude::*;
use crate::gc::MrbGarbageCollection;

// MRB_API mrb_value mrb_ary_new(mrb_state *mrb);
#[no_mangle]
unsafe extern "C" fn artichoke_ary_new(mrb: *mut sys::mrb_state) -> sys::mrb_value {
    let interp = unwrap_interpreter!(mrb);
    let result = Array(InlineBuffer::default());
    let result = result.try_into_ruby(&interp, None);
    match result {
        Ok(value) => value.inner(),
        Err(exception) => exception::raise(interp, exception),
    }
}

// MRB_API mrb_value mrb_ary_new_capa(mrb_state*, mrb_int);
#[no_mangle]
unsafe extern "C" fn artichoke_ary_new_capa(
    mrb: *mut sys::mrb_state,
    capa: sys::mrb_int,
) -> sys::mrb_value {
    let interp = unwrap_interpreter!(mrb);
    let result = Array(InlineBuffer::with_capacity(
        usize::try_from(capa).unwrap_or_default(),
    ));
    let result = result.try_into_ruby(&interp, None);
    match result {
        Ok(value) => value.inner(),
        Err(exception) => exception::raise(interp, exception),
    }
}

// MRB_API mrb_value mrb_ary_new_from_values(mrb_state *mrb, mrb_int size, const mrb_value *vals);
#[no_mangle]
unsafe extern "C" fn artichoke_ary_new_from_values(
    mrb: *mut sys::mrb_state,
    size: sys::mrb_int,
    vals: *const sys::mrb_value,
) -> sys::mrb_value {
    let interp = unwrap_interpreter!(mrb);
    let size = usize::try_from(size).unwrap_or_default();
    let values = slice::from_raw_parts(vals, size);
    let result = InlineBuffer::from(values);
    let result = Array(result);
    let result = result.try_into_ruby(&interp, None);
    match result {
        Ok(value) => {
            let basic = sys::mrb_sys_basic_ptr(value.inner());
            sys::mrb_write_barrier(mrb, basic);
            value.inner()
        }
        Err(exception) => exception::raise(interp, exception),
    }
}

// MRB_API mrb_value mrb_assoc_new(mrb_state *mrb, mrb_value car, mrb_value cdr)
#[no_mangle]
unsafe extern "C" fn artichoke_ary_new_assoc(
    mrb: *mut sys::mrb_state,
    one: sys::mrb_value,
    two: sys::mrb_value,
) -> sys::mrb_value {
    let interp = unwrap_interpreter!(mrb);
    let result = InlineBuffer::from(&[one, two][..]);
    let result = Array(result);
    let result = result.try_into_ruby(&interp, None);
    match result {
        Ok(value) => {
            let basic = sys::mrb_sys_basic_ptr(value.inner());
            sys::mrb_write_barrier(mrb, basic);
            value.inner()
        }
        Err(exception) => exception::raise(interp, exception),
    }
}

// MRB_API mrb_value mrb_ary_splat(mrb_state *mrb, mrb_value value);
#[no_mangle]
unsafe extern "C" fn artichoke_ary_splat(
    mrb: *mut sys::mrb_state,
    value: sys::mrb_value,
) -> sys::mrb_value {
    let interp = unwrap_interpreter!(mrb);
    let value = Value::new(&interp, value);
    if Array::try_from_ruby(&interp, &value).is_ok() {
        return value.inner();
    }
    let result = InlineBuffer::from(vec![value.inner()]);
    let result = Array(result);
    let result = result.try_into_ruby(&interp, None);
    match result {
        Ok(value) => value.inner(),
        Err(exception) => exception::raise(interp, exception),
    }
}

// MRB_API void mrb_ary_concat(mrb_state *mrb, mrb_value self, mrb_value other);
#[no_mangle]
unsafe extern "C" fn artichoke_ary_concat(
    mrb: *mut sys::mrb_state,
    ary: sys::mrb_value,
    other: sys::mrb_value,
) -> sys::mrb_value {
    let interp = unwrap_interpreter!(mrb);
    let ary = Value::new(&interp, ary);
    let other = Value::new(&interp, other);
    let result = if let Ok(array) = Array::try_from_ruby(&interp, &ary) {
        let mut borrow = array.borrow_mut();
        let gc_was_enabled = interp.disable_gc();
        let result = borrow.concat(&interp, other);
        if gc_was_enabled {
            interp.enable_gc();
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
        Err(exception) => exception::raise(interp, exception),
    }
}

// MRB_API mrb_value mrb_ary_pop(mrb_state *mrb, mrb_value ary);
#[no_mangle]
unsafe extern "C" fn artichoke_ary_pop(
    mrb: *mut sys::mrb_state,
    ary: sys::mrb_value,
) -> sys::mrb_value {
    let interp = unwrap_interpreter!(mrb);
    let ary = Value::new(&interp, ary);
    let result = if let Ok(array) = Array::try_from_ruby(&interp, &ary) {
        let mut borrow = array.borrow_mut();
        let gc_was_enabled = interp.disable_gc();
        let result = borrow.pop(&interp);
        if gc_was_enabled {
            interp.enable_gc();
        }
        result
    } else {
        Ok(interp.convert(None::<Value>))
    };
    match result {
        Ok(value) => {
            let basic = sys::mrb_sys_basic_ptr(ary.inner());
            sys::mrb_write_barrier(mrb, basic);
            value.inner()
        }
        Err(exception) => exception::raise(interp, exception),
    }
}

// MRB_API void mrb_ary_push(mrb_state *mrb, mrb_value array, mrb_value value);
#[no_mangle]
unsafe extern "C" fn artichoke_ary_push(
    mrb: *mut sys::mrb_state,
    ary: sys::mrb_value,
    value: sys::mrb_value,
) -> sys::mrb_value {
    let interp = unwrap_interpreter!(mrb);
    let ary = Value::new(&interp, ary);
    let value = Value::new(&interp, value);
    let result = if let Ok(array) = Array::try_from_ruby(&interp, &ary) {
        let idx = array.borrow().len();
        let mut borrow = array.borrow_mut();
        let gc_was_enabled = interp.disable_gc();
        let result = borrow.set(&interp, idx, value);
        if gc_was_enabled {
            interp.enable_gc();
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
        Err(exception) => exception::raise(interp, exception),
    }
}

// MRB_API mrb_value mrb_ary_ref(mrb_state *mrb, mrb_value ary, mrb_int n);
#[no_mangle]
unsafe extern "C" fn artichoke_ary_ref(
    mrb: *mut sys::mrb_state,
    ary: sys::mrb_value,
    offset: sys::mrb_int,
) -> sys::mrb_value {
    let interp = unwrap_interpreter!(mrb);
    let ary = Value::new(&interp, ary);
    let offset = usize::try_from(offset).unwrap_or_default();
    let result = if let Ok(array) = Array::try_from_ruby(&interp, &ary) {
        let borrow = array.borrow();
        let gc_was_enabled = interp.disable_gc();
        let result = borrow.get(&interp, offset);
        if gc_was_enabled {
            interp.enable_gc();
        }
        result
    } else {
        Ok(interp.convert(None::<Value>))
    };
    match result {
        Ok(value) => value.inner(),
        Err(exception) => exception::raise(interp, exception),
    }
}

// MRB_API void mrb_ary_set(mrb_state *mrb, mrb_value ary, mrb_int n, mrb_value val);
#[no_mangle]
unsafe extern "C" fn artichoke_ary_set(
    mrb: *mut sys::mrb_state,
    ary: sys::mrb_value,
    offset: sys::mrb_int,
    value: sys::mrb_value,
) -> sys::mrb_value {
    let interp = unwrap_interpreter!(mrb);
    let ary = Value::new(&interp, ary);
    let value = Value::new(&interp, value);
    let result = if let Ok(array) = Array::try_from_ruby(&interp, &ary) {
        let offset = if offset >= 0 {
            usize::try_from(offset).unwrap_or_default()
        } else {
            let len = array.borrow().len();
            // Positive Int must be usize
            let idx = usize::try_from(-offset).unwrap_or_default();
            if let Some(offset) = len.checked_sub(idx) {
                offset
            } else {
                0
            }
        };
        // TODO: properly handle self-referential sets.
        if ary == value {
            Ok(())
        } else {
            let mut borrow = array.borrow_mut();
            let gc_was_enabled = interp.disable_gc();
            let result = borrow.set(&interp, offset, value.clone());
            if gc_was_enabled {
                interp.enable_gc();
            }
            result
        }
    } else {
        Ok(())
    };
    match result {
        Ok(()) => {
            let basic = sys::mrb_sys_basic_ptr(ary.inner());
            sys::mrb_write_barrier(mrb, basic);
            value.inner()
        }
        Err(exception) => exception::raise(interp, exception),
    }
}

#[no_mangle]
unsafe extern "C" fn artichoke_ary_shift(
    mrb: *mut sys::mrb_state,
    ary: sys::mrb_value,
) -> sys::mrb_value {
    let interp = unwrap_interpreter!(mrb);
    let array = Value::new(&interp, ary);
    let result = if let Ok(array) = Array::try_from_ruby(&interp, &array) {
        let mut borrow = array.borrow_mut();
        let gc_was_enabled = interp.disable_gc();
        let result = borrow.get(&interp, 0);
        let _ = borrow.set_slice(&interp, 0, 1, &InlineBuffer::default());
        if gc_was_enabled {
            interp.enable_gc();
        }
        result
    } else {
        Ok(interp.convert(None::<Value>))
    };
    match result {
        Ok(value) => {
            let basic = sys::mrb_sys_basic_ptr(ary);
            sys::mrb_write_barrier(mrb, basic);
            value.inner()
        }
        Err(exception) => exception::raise(interp, exception),
    }
}

#[no_mangle]
unsafe extern "C" fn artichoke_ary_unshift(
    mrb: *mut sys::mrb_state,
    ary: sys::mrb_value,
    value: sys::mrb_value,
) -> sys::mrb_value {
    let interp = unwrap_interpreter!(mrb);
    let array = Value::new(&interp, ary);
    let value = Value::new(&interp, value);
    if let Ok(array) = Array::try_from_ruby(&interp, &array) {
        let mut borrow = array.borrow_mut();
        let gc_was_enabled = interp.disable_gc();
        let _ = borrow.set_with_drain(&interp, 0, 0, value.clone());
        if gc_was_enabled {
            interp.enable_gc();
        }
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
    let interp = unwrap_interpreter!(mrb, or_else = 0);
    let ary = Value::new(&interp, ary);
    if let Ok(array) = Array::try_from_ruby(&interp, &ary) {
        let borrow = array.borrow();
        Int::try_from(borrow.0.len()).unwrap_or_default()
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
    let interp = unwrap_interpreter!(mrb, or_else = ());
    let ary = Value::new(&interp, ary);
    if let Ok(array) = Array::try_from_ruby(&interp, &ary) {
        let len = usize::try_from(len).unwrap_or_default();
        let mut borrow = array.borrow_mut();
        borrow.0.set_len(len);
    }
}

#[no_mangle]
unsafe extern "C" fn artichoke_ary_ptr(
    mrb: *mut sys::mrb_state,
    ary: sys::mrb_value,
) -> *mut sys::mrb_value {
    let interp = unwrap_interpreter!(mrb, or_else = ptr::null_mut());
    let ary = Value::new(&interp, ary);
    if let Ok(array) = Array::try_from_ruby(&interp, &ary) {
        let mut borrow = array.borrow_mut();
        borrow.0.as_mut_ptr()
    } else {
        ptr::null_mut()
    }
}

#[no_mangle]
unsafe extern "C" fn artichoke_ary_check(
    mrb: *mut sys::mrb_state,
    ary: sys::mrb_value,
) -> sys::mrb_bool {
    let interp = unwrap_interpreter!(mrb, or_else = 0);
    let ary = Value::new(&interp, ary);
    if Array::try_from_ruby(&interp, &ary).is_ok() {
        1
    } else {
        0
    }
}

#[no_mangle]
unsafe extern "C" fn artichoke_gc_mark_ary(mrb: *mut sys::mrb_state, ary: sys::mrb_value) {
    let interp = unwrap_interpreter!(mrb, or_else = ());
    let array = Value::new(&interp, ary);
    if let Ok(array) = Array::try_from_ruby(&interp, &array) {
        let borrow = array.borrow();
        borrow.gc_mark(&interp);
    }
}

#[no_mangle]
unsafe extern "C" fn artichoke_gc_mark_ary_size(
    mrb: *mut sys::mrb_state,
    ary: sys::mrb_value,
) -> usize {
    let interp = unwrap_interpreter!(mrb, or_else = 0);
    let array = Value::new(&interp, ary);
    if let Ok(array) = Array::try_from_ruby(&interp, &array) {
        let borrow = array.borrow();
        borrow.real_children()
    } else {
        0
    }
}
