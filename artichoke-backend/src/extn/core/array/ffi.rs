use std::convert::TryFrom;
use std::slice;

use crate::convert::{Convert, RustBackedValue};
use crate::extn::core::array::{backend, Array};
use crate::extn::core::exception::{self, Fatal};
use crate::gc::MrbGarbageCollection;
use crate::sys;
use crate::types::Int;
use crate::value::Value;

// MRB_API mrb_value mrb_ary_new(mrb_state *mrb);
#[no_mangle]
unsafe extern "C" fn artichoke_ary_new(mrb: *mut sys::mrb_state) -> sys::mrb_value {
    let interp = unwrap_interpreter!(mrb);
    let result = backend::fixed::empty();
    let result = Array(result);
    let result = result.try_into_ruby(&interp, None).map_err(|_| {
        Box::new(Fatal::new(
            &interp,
            "Unable to initialize Ruby Array from Rust Array",
        ))
    });
    match result {
        Ok(value) => value.inner(),
        Err(exception) => exception::raise(interp, exception),
    }
}

// MRB_API mrb_value mrb_ary_new_capa(mrb_state*, mrb_int);
#[no_mangle]
unsafe extern "C" fn artichoke_ary_new_capa(
    mrb: *mut sys::mrb_state,
    capacity: sys::mrb_int,
) -> sys::mrb_value {
    let interp = unwrap_interpreter!(mrb);
    let capacity = usize::try_from(capacity).unwrap_or_default();
    let result = if capacity == 0 {
        backend::fixed::empty()
    } else {
        let buffer = backend::buffer::Buffer::with_capacity(capacity);
        Box::new(buffer)
    };
    let result = Array(result);
    let result = result.try_into_ruby(&interp, None).map_err(|_| {
        Box::new(Fatal::new(
            &interp,
            "Unable to initialize Ruby Array from Rust Array",
        ))
    });
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
    let result = if values.is_empty() {
        backend::fixed::empty()
    } else if values.len() == 1 {
        backend::fixed::one(Value::new(&interp, values[0]))
    } else if values.len() == 2 {
        backend::fixed::two(
            Value::new(&interp, values[0]),
            Value::new(&interp, values[1]),
        )
    } else {
        let values = values
            .iter()
            .copied()
            .map(|val| Value::new(&interp, val))
            .collect::<Vec<_>>();
        let buffer = backend::buffer::Buffer::from(values);
        Box::new(buffer)
    };
    let result = Array(result);
    let result = result.try_into_ruby(&interp, None).map_err(|_| {
        Box::new(Fatal::new(
            &interp,
            "Unable to initialize Ruby Array from Rust Array",
        ))
    });
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
    let result = backend::fixed::one(value);
    let result = Array(result);
    let result = result.try_into_ruby(&interp, None).map_err(|_| {
        Box::new(Fatal::new(
            &interp,
            "Unable to initialize Ruby Array from Rust Array",
        ))
    });
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
        let idx = array.borrow().len_usize();
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
        let result = borrow.0.get(&interp, offset);
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
            let len = array.borrow().len_usize();
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
unsafe extern "C" fn artichoke_ary_clone(
    mrb: *mut sys::mrb_state,
    value: sys::mrb_value,
) -> sys::mrb_value {
    let interp = unwrap_interpreter!(mrb);
    let ary = Value::new(&interp, value);
    let result = if let Ok(array) = Array::try_from_ruby(&interp, &ary) {
        let borrow = array.borrow();
        let result = borrow.clone();
        result.try_into_ruby(&interp, None).map_err(|_| {
            Box::new(Fatal::new(
                &interp,
                "Unable to initialize Ruby Array from Rust Array",
            ))
        })
    } else {
        Ok(interp.convert(None::<Value>))
    };
    match result {
        Ok(value) => value.inner(),
        Err(exception) => exception::raise(interp, exception),
    }
}

#[no_mangle]
unsafe extern "C" fn artichoke_value_to_ary(
    mrb: *mut sys::mrb_state,
    value: sys::mrb_value,
) -> sys::mrb_value {
    let interp = unwrap_interpreter!(mrb);
    let value = Value::new(&interp, value);
    if Array::try_from_ruby(&interp, &value).is_ok() {
        value.inner()
    } else {
        let result = backend::fixed::one(value);
        let result = Array(result);
        let result = result.try_into_ruby(&interp, None).map_err(|_| {
            Box::new(Fatal::new(
                &interp,
                "Unable to initialize Ruby Array from Rust Array",
            ))
        });
        match result {
            Ok(value) => value.inner(),
            Err(exception) => exception::raise(interp, exception),
        }
    }
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
        let _ = borrow.set_slice(&interp, 0, 1, backend::fixed::empty());
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
    val: sys::mrb_value,
) -> sys::mrb_value {
    let interp = unwrap_interpreter!(mrb);
    let array = Value::new(&interp, ary);
    let value = Value::new(&interp, val);
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
