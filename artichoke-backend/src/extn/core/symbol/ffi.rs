use std::convert::TryFrom;
use std::ffi::CStr;
use std::ptr;
use std::slice;

use crate::extn::core::symbol::Symbol;
use crate::extn::prelude::*;

#[no_mangle]
unsafe extern "C" fn mrb_intern(
    mrb: *mut sys::mrb_state,
    name: *const i8,
    len: usize,
) -> sys::mrb_sym {
    let bytes = slice::from_raw_parts(name as *const u8, len);
    let bytes = bytes.to_vec();
    unwrap_interpreter!(mrb, to => guard, or_else = 0);
    let sym = guard.intern_bytes(bytes);
    let sym = sym.map(u32::from);
    sym.unwrap_or_default()
}

#[no_mangle]
unsafe extern "C" fn mrb_intern_static(
    mrb: *mut sys::mrb_state,
    name: *const i8,
    len: usize,
) -> sys::mrb_sym {
    let bytes = slice::from_raw_parts::<'static, _>(name as *const u8, len);
    unwrap_interpreter!(mrb, to => guard, or_else = 0);
    let sym = guard.intern_bytes(bytes);
    let sym = sym.map(u32::from);
    sym.unwrap_or_default()
}

#[no_mangle]
unsafe extern "C" fn mrb_intern_cstr(mrb: *mut sys::mrb_state, name: *const i8) -> sys::mrb_sym {
    let string = CStr::from_ptr(name);
    let bytes = string.to_bytes_with_nul().to_vec();
    unwrap_interpreter!(mrb, to => guard, or_else = 0);
    let sym = guard.intern_bytes_with_trailing_nul(bytes);
    let sym = sym.map(u32::from);
    sym.unwrap_or_default()
}

#[no_mangle]
unsafe extern "C" fn mrb_intern_str(
    mrb: *mut sys::mrb_state,
    name: sys::mrb_value,
) -> sys::mrb_sym {
    unwrap_interpreter!(mrb, to => guard, or_else = 0);
    let name = Value::from(name);
    if let Ok(bytes) = name.try_into_mut::<Vec<u8>>(&mut guard) {
        let sym = guard.intern_bytes(bytes);
        let sym = sym.map(u32::from);
        sym.unwrap_or_default()
    } else {
        0
    }
}

#[no_mangle]
unsafe extern "C" fn mrb_check_intern(
    mrb: *mut sys::mrb_state,
    name: *const i8,
    len: usize,
) -> sys::mrb_value {
    let bytes = slice::from_raw_parts(name as *const u8, len);
    unwrap_interpreter!(mrb, to => guard);
    let symbol = if let Ok(Some(sym)) = guard.check_interned_bytes(bytes) {
        Symbol::alloc_value(sym.into(), &mut guard).unwrap_or_default()
    } else {
        Value::nil()
    };
    symbol.inner()
}

#[no_mangle]
unsafe extern "C" fn mrb_check_intern_cstr(
    mrb: *mut sys::mrb_state,
    name: *const i8,
) -> sys::mrb_value {
    let string = CStr::from_ptr(name);
    let bytes = string.to_bytes_with_nul();
    unwrap_interpreter!(mrb, to => guard);
    let symbol = if let Ok(Some(sym)) = guard.check_interned_bytes_with_trailing_nul(bytes) {
        Symbol::alloc_value(sym.into(), &mut guard).unwrap_or_default()
    } else {
        Value::nil()
    };
    symbol.inner()
}

#[no_mangle]
unsafe extern "C" fn mrb_check_intern_str(
    mrb: *mut sys::mrb_state,
    name: sys::mrb_value,
) -> sys::mrb_value {
    unwrap_interpreter!(mrb, to => guard);
    let name = Value::from(name);
    let symbol = if let Ok(bytes) = name.try_into_mut::<&[u8]>(&mut guard) {
        if let Ok(Some(sym)) = guard.check_interned_bytes(bytes) {
            Symbol::alloc_value(sym.into(), &mut guard).unwrap_or_default()
        } else {
            Value::nil()
        }
    } else {
        Value::nil()
    };
    symbol.inner()
}

#[no_mangle]
unsafe extern "C" fn mrb_sym_name_len(
    mrb: *mut sys::mrb_state,
    sym: sys::mrb_sym,
    lenp: *mut sys::mrb_int,
) -> *const i8 {
    if !lenp.is_null() {
        ptr::write(lenp, 0);
    }
    unwrap_interpreter!(mrb, to => guard, or_else = ptr::null());
    if let Ok(Some(bytes)) = guard.lookup_symbol(sym) {
        if !lenp.is_null() {
            if let Ok(len) = sys::mrb_int::try_from(bytes.len()) {
                ptr::write(lenp, len);
            } else {
                return ptr::null();
            }
        }
        bytes.as_ptr() as *const i8
    } else {
        ptr::null()
    }
}

#[no_mangle]
unsafe extern "C" fn mrb_sym_str(mrb: *mut sys::mrb_state, sym: sys::mrb_sym) -> sys::mrb_value {
    unwrap_interpreter!(mrb, to => guard);

    let value = if let Ok(Some(bytes)) = guard.lookup_symbol(sym) {
        let bytes = bytes.to_vec();
        guard.convert_mut(bytes)
    } else {
        guard.convert_mut("")
    };
    value.inner()
}

#[no_mangle]
unsafe extern "C" fn mrb_sym_name(mrb: *mut sys::mrb_state, sym: sys::mrb_sym) -> *const i8 {
    unwrap_interpreter!(mrb, to => guard, or_else = ptr::null());
    if let Ok(Some(bytes)) = guard.lookup_symbol_with_trailing_nul(sym) {
        bytes.as_ptr() as *const i8
    } else {
        ptr::null()
    }
}

#[no_mangle]
unsafe extern "C" fn mrb_sym_dump(mrb: *mut sys::mrb_state, sym: sys::mrb_sym) -> *const i8 {
    unwrap_interpreter!(mrb, to => guard, or_else = ptr::null());
    if let Ok(Some(bytes)) = guard.lookup_symbol(sym) {
        let bytes = bytes.to_vec();
        // Allocate a buffer with the lifetime of the interpreter and return
        // a pointer to it.
        let string = guard.convert_mut(bytes);
        if let Ok(bytes) = string.try_into_mut::<&[u8]>(&mut guard) {
            bytes.as_ptr() as *const i8
        } else {
            ptr::null()
        }
    } else {
        ptr::null()
    }
}

#[no_mangle]
unsafe extern "C" fn mrb_init_symtbl(mrb: *mut sys::mrb_state) {
    // The symbol table is initialized before the call to `mrb_open_allocf` in
    // `crate::interpreter::interpreter`. This function is intended to be called
    // during the initialization of the `mrb_state`.
    let _ = mrb;
}

#[no_mangle]
unsafe extern "C" fn mrb_free_symtbl(mrb: *mut sys::mrb_state) {
    // The symbol table is freed when the Rust `State` is freed.
    let _ = mrb;
}
