use std::convert::TryFrom;
use std::iter::{self, FromIterator};
use std::ptr;
use std::slice;

use crate::extn::core::array::{Array, ArrayType, InlineBuffer};
use crate::extn::prelude::*;
use crate::gc::MrbGarbageCollection;

// MRB_API mrb_value mrb_ary_new(mrb_state *mrb);
#[no_mangle]
unsafe extern "C" fn artichoke_ary_new(mrb: *mut sys::mrb_state) -> sys::mrb_value {
    let mut interp = unwrap_interpreter!(mrb);
    let mut guard = Guard::new(&mut interp);
    let result = Array(InlineBuffer::default());
    let result = result.try_into_ruby(&mut guard, None);
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
    let result = Array(InlineBuffer::with_capacity(
        usize::try_from(capa).unwrap_or_default(),
    ));
    let result = result.try_into_ruby(&mut guard, None);
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
    let result = InlineBuffer::from(values);
    let result = Array(result);
    let result = result.try_into_ruby(&mut guard, None);
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
    let result = InlineBuffer::from(&[one, two][..]);
    let result = Array(result);
    let result = result.try_into_ruby(&mut guard, None);
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
    let value = Value::new(&guard, value);
    let result = if Array::try_from_ruby(&mut guard, &value).is_ok() {
        Ok(value)
    } else {
        let result = InlineBuffer::from_iter(iter::once(value.inner()));
        let result = Array(result);
        result.try_into_ruby(&mut guard, None)
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
    let ary = Value::new(&guard, ary);
    let other = Value::new(&guard, other);
    let result = if let Ok(array) = Array::try_from_ruby(&mut guard, &ary) {
        let mut borrow = array.borrow_mut();
        let gc_was_enabled = interp.disable_gc();
        let result = borrow.concat(&mut guard, other);
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
    let ary = Value::new(&guard, ary);
    let result = if let Ok(array) = Array::try_from_ruby(&mut guard, &ary) {
        let mut borrow = array.borrow_mut();
        let gc_was_enabled = interp.disable_gc();
        let result = borrow.pop(&mut guard);
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
        Err(exception) => exception::raise(guard, exception),
    }
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
    let ary = Value::new(&guard, ary);
    let value = Value::new(&guard, value);
    let result = if let Ok(array) = Array::try_from_ruby(&mut guard, &ary) {
        let idx = array.borrow().len();
        let mut borrow = array.borrow_mut();
        let gc_was_enabled = interp.disable_gc();
        let result = borrow.set(&mut guard, idx, value);
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
        Err(exception) => exception::raise(guard, exception),
    }
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
    let ary = Value::new(&guard, ary);
    let offset = usize::try_from(offset).unwrap_or_default();
    let result = if let Ok(array) = Array::try_from_ruby(&mut guard, &ary) {
        let borrow = array.borrow();
        let gc_was_enabled = interp.disable_gc();
        let result = borrow.get(&mut guard, offset);
        if gc_was_enabled {
            interp.enable_gc();
        }
        result
    } else {
        Ok(interp.convert(None::<Value>))
    };
    match result {
        Ok(value) => value.inner(),
        Err(exception) => exception::raise(guard, exception),
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
    let mut interp = unwrap_interpreter!(mrb);
    let mut guard = Guard::new(&mut interp);
    let ary = Value::new(&guard, ary);
    let value = Value::new(&guard, value);
    let result = if let Ok(array) = Array::try_from_ruby(&mut guard, &ary) {
        let offset = if let Ok(offset) = usize::try_from(offset) {
            offset
        } else {
            let array_len = array.borrow().len();
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
        if ary == value {
            Ok(())
        } else {
            let mut borrow = array.borrow_mut();
            let gc_was_enabled = interp.disable_gc();
            let result = borrow.set(&mut guard, offset, value.clone());
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
        Err(exception) => exception::raise(guard, exception),
    }
}

#[no_mangle]
unsafe extern "C" fn artichoke_ary_shift(
    mrb: *mut sys::mrb_state,
    ary: sys::mrb_value,
) -> sys::mrb_value {
    let mut interp = unwrap_interpreter!(mrb);
    let mut guard = Guard::new(&mut interp);
    let array = Value::new(&guard, ary);
    let result = if let Ok(array) = Array::try_from_ruby(&mut guard, &array) {
        let mut borrow = array.borrow_mut();
        let gc_was_enabled = interp.disable_gc();
        let result = borrow.get(&mut guard, 0);
        let _ = borrow.set_slice(&mut guard, 0, 1, &InlineBuffer::default());
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
        Err(exception) => exception::raise(guard, exception),
    }
}

#[no_mangle]
unsafe extern "C" fn artichoke_ary_unshift(
    mrb: *mut sys::mrb_state,
    ary: sys::mrb_value,
    value: sys::mrb_value,
) -> sys::mrb_value {
    let mut interp = unwrap_interpreter!(mrb);
    let mut guard = Guard::new(&mut interp);
    let array = Value::new(&guard, ary);
    let value = Value::new(&guard, value);
    if let Ok(array) = Array::try_from_ruby(&mut guard, &array) {
        let mut borrow = array.borrow_mut();
        let gc_was_enabled = interp.disable_gc();
        let _ = borrow.set_with_drain(&mut guard, 0, 0, value.clone());
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
    let mut interp = unwrap_interpreter!(mrb, or_else = 0);
    let mut guard = Guard::new(&mut interp);
    let ary = Value::new(&guard, ary);
    if let Ok(array) = Array::try_from_ruby(&mut guard, &ary) {
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
    let mut interp = unwrap_interpreter!(mrb, or_else = ());
    let mut guard = Guard::new(&mut interp);
    let ary = Value::new(&guard, ary);
    if let Ok(array) = Array::try_from_ruby(&mut guard, &ary) {
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
    let mut interp = unwrap_interpreter!(mrb, or_else = ptr::null_mut());
    let mut guard = Guard::new(&mut interp);
    let ary = Value::new(&guard, ary);
    if let Ok(array) = Array::try_from_ruby(&mut guard, &ary) {
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
    let mut interp = unwrap_interpreter!(mrb, or_else = 0);
    let mut guard = Guard::new(&mut interp);
    let ary = Value::new(&guard, ary);
    if Array::try_from_ruby(&mut guard, &ary).is_ok() {
        1
    } else {
        0
    }
}

#[no_mangle]
unsafe extern "C" fn artichoke_gc_mark_ary(mrb: *mut sys::mrb_state, ary: sys::mrb_value) {
    let mut interp = unwrap_interpreter!(mrb, or_else = ());
    let mut guard = Guard::new(&mut interp);
    let array = Value::new(&guard, ary);
    if let Ok(array) = Array::try_from_ruby(&mut guard, &array) {
        let borrow = array.borrow();
        borrow.gc_mark(&mut guard);
    }
}

#[no_mangle]
unsafe extern "C" fn artichoke_gc_mark_ary_size(
    mrb: *mut sys::mrb_state,
    ary: sys::mrb_value,
) -> usize {
    let mut interp = unwrap_interpreter!(mrb, or_else = 0);
    let mut guard = Guard::new(&mut interp);
    let array = Value::new(&guard, ary);
    if let Ok(array) = Array::try_from_ruby(&mut guard, &array) {
        let borrow = array.borrow();
        borrow.real_children()
    } else {
        0
    }
}
