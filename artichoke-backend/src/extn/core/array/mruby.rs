use std::ffi::CStr;

use crate::extn::core::array::{trampoline, Array};
use crate::extn::prelude::*;

const ARRAY_CSTR: &CStr = qed::const_cstr_from_str!("Array\0");
static ARRAY_RUBY_SOURCE: &[u8] = include_bytes!("array.rb");

pub fn init(interp: &mut Artichoke) -> InitializeResult<()> {
    if interp.is_class_defined::<Array>() {
        return Ok(());
    }

    let spec = class::Spec::new("Array", ARRAY_CSTR, None, Some(def::box_unbox_free::<Array>))?;
    class::Builder::for_spec(interp, &spec)
        .add_self_method("[]", ary_cls_constructor, sys::mrb_args_rest())?
        .add_method("+", ary_plus, sys::mrb_args_req(1))?
        .add_method("*", ary_mul, sys::mrb_args_req(1))?
        .add_method("[]", ary_element_reference, sys::mrb_args_req_and_opt(1, 1))?
        .add_method("[]=", ary_element_assignment, sys::mrb_args_req_and_opt(2, 1))?
        .add_method("clear", ary_clear, sys::mrb_args_none())?
        .add_method("concat", ary_concat, sys::mrb_args_rest())?
        .add_method("first", ary_first, sys::mrb_args_opt(1))?
        .add_method(
            "initialize",
            ary_initialize,
            sys::mrb_args_opt(2) | sys::mrb_args_block(),
        )?
        .add_method("initialize_copy", ary_initialize_copy, sys::mrb_args_req(1))?
        .add_method("last", ary_last, sys::mrb_args_opt(1))?
        .add_method("length", ary_len, sys::mrb_args_none())?
        .add_method("pop", ary_pop, sys::mrb_args_none())?
        .add_method("reverse", ary_reverse, sys::mrb_args_none())?
        .add_method("reverse!", ary_reverse_bang, sys::mrb_args_none())?
        .add_method("shift", ary_shift, sys::mrb_args_opt(1))?
        .add_method("size", ary_len, sys::mrb_args_none())?
        .define()?;
    interp.def_class::<Array>(spec)?;
    interp.eval(ARRAY_RUBY_SOURCE)?;

    Ok(())
}

unsafe extern "C" fn ary_cls_constructor(mrb: *mut sys::mrb_state, ary: sys::mrb_value) -> sys::mrb_value {
    let rest = mrb_get_args!(mrb, *args);
    unwrap_interpreter!(mrb, to => guard);
    let result = Array::from(rest);
    let result = Array::alloc_value(result, &mut guard);
    match result {
        Ok(value) => {
            let rclass = ary.value.p.cast::<sys::RClass>();
            let value = value.inner();
            let target_rbasic = value.value.p.cast::<sys::RBasic>();

            // Copy `RClass` from source class to newly allocated `Array`.
            (*target_rbasic).c = rclass;

            value
        }
        Err(exception) => error::raise(guard, exception),
    }
}

unsafe extern "C" fn ary_plus(mrb: *mut sys::mrb_state, ary: sys::mrb_value) -> sys::mrb_value {
    let other = mrb_get_args!(mrb, required = 1);
    unwrap_interpreter!(mrb, to => guard);
    let array = Value::from(ary);
    let other = Value::from(other);
    let result = trampoline::plus(&mut guard, array, other);
    match result {
        Ok(value) => {
            let basic = sys::mrb_sys_basic_ptr(ary);
            sys::mrb_write_barrier(mrb, basic);
            value.inner()
        }
        Err(exception) => error::raise(guard, exception),
    }
}

unsafe extern "C" fn ary_mul(mrb: *mut sys::mrb_state, ary: sys::mrb_value) -> sys::mrb_value {
    let joiner = mrb_get_args!(mrb, required = 1);
    unwrap_interpreter!(mrb, to => guard);
    let array = Value::from(ary);
    let joiner = Value::from(joiner);
    let result = trampoline::mul(&mut guard, array, joiner);
    match result {
        Ok(value) => {
            let basic = sys::mrb_sys_basic_ptr(ary);
            sys::mrb_write_barrier(mrb, basic);
            value.inner()
        }
        Err(exception) => error::raise(guard, exception),
    }
}

unsafe extern "C" fn ary_element_reference(mrb: *mut sys::mrb_state, ary: sys::mrb_value) -> sys::mrb_value {
    let (elem, len) = mrb_get_args!(mrb, required = 1, optional = 1);
    unwrap_interpreter!(mrb, to => guard);
    let elem = Value::from(elem);
    let len = len.map(Value::from);
    let array = Value::from(ary);
    let result = trampoline::element_reference(&mut guard, array, elem, len);
    match result {
        Ok(value) => value.inner(),
        Err(exception) => error::raise(guard, exception),
    }
}

unsafe extern "C" fn ary_element_assignment(mrb: *mut sys::mrb_state, ary: sys::mrb_value) -> sys::mrb_value {
    let (first, second, third) = mrb_get_args!(mrb, required = 2, optional = 1);
    unwrap_interpreter!(mrb, to => guard);
    let first = Value::from(first);
    let second = Value::from(second);
    let third = third.map(Value::from);
    let array = Value::from(ary);
    let result = trampoline::element_assignment(&mut guard, array, first, second, third);
    match result {
        Ok(value) => {
            let basic = sys::mrb_sys_basic_ptr(ary);
            sys::mrb_write_barrier(mrb, basic);
            value.inner()
        }
        Err(exception) => error::raise(guard, exception),
    }
}

unsafe extern "C" fn ary_clear(mrb: *mut sys::mrb_state, ary: sys::mrb_value) -> sys::mrb_value {
    mrb_get_args!(mrb, none);
    unwrap_interpreter!(mrb, to => guard);
    let array = Value::from(ary);
    let result = trampoline::clear(&mut guard, array);
    match result {
        Ok(value) => {
            let basic = sys::mrb_sys_basic_ptr(ary);
            sys::mrb_write_barrier(mrb, basic);
            value.inner()
        }
        Err(exception) => error::raise(guard, exception),
    }
}

unsafe extern "C" fn ary_concat(mrb: *mut sys::mrb_state, ary: sys::mrb_value) -> sys::mrb_value {
    let others = mrb_get_args!(mrb, *args);
    unwrap_interpreter!(mrb, to => guard);
    let array = Value::from(ary);
    let others = others.iter().map(|&other| Value::from(other));
    let result = trampoline::concat(&mut guard, array, others);
    match result {
        Ok(value) => {
            let basic = sys::mrb_sys_basic_ptr(ary);
            sys::mrb_write_barrier(mrb, basic);
            value.inner()
        }
        Err(exception) => error::raise(guard, exception),
    }
}

unsafe extern "C" fn ary_first(mrb: *mut sys::mrb_state, ary: sys::mrb_value) -> sys::mrb_value {
    let num = mrb_get_args!(mrb, optional = 1);
    unwrap_interpreter!(mrb, to => guard);
    let array = Value::from(ary);
    let num = num.map(Value::from);
    let result = trampoline::first(&mut guard, array, num);
    match result {
        Ok(value) => {
            let basic = sys::mrb_sys_basic_ptr(ary);
            sys::mrb_write_barrier(mrb, basic);
            value.inner()
        }
        Err(exception) => error::raise(guard, exception),
    }
}

unsafe extern "C" fn ary_initialize(mrb: *mut sys::mrb_state, ary: sys::mrb_value) -> sys::mrb_value {
    let (first, second, block) = mrb_get_args!(mrb, optional = 2, &block);
    unwrap_interpreter!(mrb, to => guard);
    let array = Value::from(ary);
    let first = first.map(Value::from);
    let second = second.map(Value::from);
    let result = trampoline::initialize(&mut guard, array, first, second, block);
    match result {
        Ok(value) => {
            let basic = sys::mrb_sys_basic_ptr(ary);
            sys::mrb_write_barrier(mrb, basic);
            value.inner()
        }
        Err(exception) => error::raise(guard, exception),
    }
}

unsafe extern "C" fn ary_initialize_copy(mrb: *mut sys::mrb_state, ary: sys::mrb_value) -> sys::mrb_value {
    let other = mrb_get_args!(mrb, required = 1);
    unwrap_interpreter!(mrb, to => guard);
    let array = Value::from(ary);
    let other = Value::from(other);
    let result = trampoline::initialize_copy(&mut guard, array, other);
    match result {
        Ok(value) => {
            let basic = sys::mrb_sys_basic_ptr(ary);
            sys::mrb_write_barrier(mrb, basic);
            value.inner()
        }
        Err(exception) => error::raise(guard, exception),
    }
}

unsafe extern "C" fn ary_last(mrb: *mut sys::mrb_state, ary: sys::mrb_value) -> sys::mrb_value {
    let num = mrb_get_args!(mrb, optional = 1);
    unwrap_interpreter!(mrb, to => guard);
    let array = Value::from(ary);
    let num = num.map(Value::from);
    let result = trampoline::last(&mut guard, array, num);
    match result {
        Ok(value) => {
            let basic = sys::mrb_sys_basic_ptr(ary);
            sys::mrb_write_barrier(mrb, basic);
            value.inner()
        }
        Err(exception) => error::raise(guard, exception),
    }
}

unsafe extern "C" fn ary_len(mrb: *mut sys::mrb_state, ary: sys::mrb_value) -> sys::mrb_value {
    mrb_get_args!(mrb, none);
    unwrap_interpreter!(mrb, to => guard);
    let ary = Value::from(ary);
    let result = trampoline::len(&mut guard, ary).and_then(|len| {
        if let Ok(len) = sys::mrb_int::try_from(len) {
            Ok(len)
        } else {
            Err(Fatal::from("Array length does not fit in mruby Integer max").into())
        }
    });
    match result {
        Ok(len) => {
            let len = guard.convert(len);
            len.inner()
        }
        Err(exception) => error::raise(guard, exception),
    }
}

unsafe extern "C" fn ary_pop(mrb: *mut sys::mrb_state, ary: sys::mrb_value) -> sys::mrb_value {
    mrb_get_args!(mrb, none);
    unwrap_interpreter!(mrb, to => guard);
    let array = Value::from(ary);
    let result = trampoline::pop(&mut guard, array);
    match result {
        Ok(value) => {
            let basic = sys::mrb_sys_basic_ptr(ary);
            sys::mrb_write_barrier(mrb, basic);
            value.inner()
        }
        Err(exception) => error::raise(guard, exception),
    }
}

unsafe extern "C" fn ary_reverse(mrb: *mut sys::mrb_state, ary: sys::mrb_value) -> sys::mrb_value {
    mrb_get_args!(mrb, none);
    unwrap_interpreter!(mrb, to => guard);
    let array = Value::from(ary);
    let result = trampoline::reverse(&mut guard, array);
    match result {
        Ok(value) => {
            let basic = sys::mrb_sys_basic_ptr(ary);
            sys::mrb_write_barrier(mrb, basic);
            value.inner()
        }
        Err(exception) => error::raise(guard, exception),
    }
}

unsafe extern "C" fn ary_reverse_bang(mrb: *mut sys::mrb_state, ary: sys::mrb_value) -> sys::mrb_value {
    mrb_get_args!(mrb, none);
    unwrap_interpreter!(mrb, to => guard);
    let array = Value::from(ary);
    let result = trampoline::reverse_bang(&mut guard, array);
    match result {
        Ok(value) => {
            let basic = sys::mrb_sys_basic_ptr(ary);
            sys::mrb_write_barrier(mrb, basic);
            value.inner()
        }
        Err(exception) => error::raise(guard, exception),
    }
}

unsafe extern "C" fn ary_shift(mrb: *mut sys::mrb_state, ary: sys::mrb_value) -> sys::mrb_value {
    let count = mrb_get_args!(mrb, optional = 1);
    unwrap_interpreter!(mrb, to => guard);
    let count = count.map(Value::from);
    let array = Value::from(ary);
    let result = trampoline::shift(&mut guard, array, count);
    match result {
        Ok(value) => {
            let basic = sys::mrb_sys_basic_ptr(ary);
            sys::mrb_write_barrier(mrb, basic);
            value.inner()
        }
        Err(exception) => error::raise(guard, exception),
    }
}
