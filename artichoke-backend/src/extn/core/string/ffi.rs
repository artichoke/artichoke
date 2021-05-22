use core::convert::TryFrom;
use core::slice;
use std::ffi::CStr;

use bstr::ByteSlice;
use spinoso_string::String;

use crate::convert::BoxUnboxVmValue;
use crate::error;
use crate::sys;

// MRB_API mrb_value mrb_str_new_capa(mrb_state *mrb, size_t capa)
#[no_mangle]
unsafe extern "C" fn mrb_str_new_capa(mrb: *mut sys::mrb_state, capa: usize) -> sys::mrb_value {
    unwrap_interpreter!(mrb, to => guard);
    let capacity = usize::try_from(capa).unwrap_or_default();
    let result = String::with_capacity(capacity);
    let result = String::alloc_value(result, &mut guard);
    match result {
        Ok(value) => value.inner(),
        Err(exception) => error::raise(guard, exception),
    }
}

// MRB_API mrb_value mrb_str_new(mrb_state *mrb, const char *p, size_t len)
#[no_mangle]
unsafe extern "C" fn mrb_str_new(mrb: *mut sys::mrb_state, p: *const i8, len: usize) -> sys::mrb_value {
    unwrap_interpreter!(mrb, to => guard);
    let bytes = slice::from_raw_parts(p.cast::<u8>(), len);
    let bytes = bytes.to_vec();
    let result = String::utf8(bytes);
    let result = String::alloc_value(result, &mut guard);
    match result {
        Ok(value) => value.inner(),
        Err(exception) => error::raise(guard, exception),
    }
}

// MRB_API mrb_value mrb_str_new_cstr(mrb_state *mrb, const char *p)
#[no_mangle]
unsafe extern "C" fn mrb_str_new_cstr(mrb: *mut sys::mrb_state, p: *const i8) -> sys::mrb_value {
    unwrap_interpreter!(mrb, to => guard);
    let cstr = CStr::from_ptr(p);
    let bytes = cstr.to_bytes().to_vec();
    let result = String::utf8(bytes);
    let result = String::alloc_value(result, &mut guard);
    match result {
        Ok(value) => value.inner(),
        Err(exception) => error::raise(guard, exception),
    }
}

// MRB_API mrb_value mrb_str_new_static(mrb_state *mrb, const char *p, size_t len)
#[no_mangle]
unsafe extern "C" fn mrb_str_new_static(mrb: *mut sys::mrb_state, p: *const i8, len: usize) -> sys::mrb_value {
    // Artichoke doesn't have a static string optimization.
    mrb_str_new(mrb, p, len)
}

// MRB_API mrb_int mrb_str_index(mrb_state *mrb, mrb_value str, const char *sptr, mrb_int slen, mrb_int offset)
#[no_mangle]
unsafe extern "C" fn mrb_str_index(
    mrb: *mut sys::mrb_state,
    s: sys::mrb_value,
    sptr: *const i8,
    slen: sys::mrb_int,
    offset: sys::mrb_int,
) -> sys::mrb_int {
    unwrap_interpreter!(mrb, to => guard, or_else = -1);
    let mut value = s.into();
    let string = if let Ok(string) = String::unbox_from_value(&mut value, &mut guard) {
        string
    } else {
        return -1;
    };
    let offset = if let Ok(offset) = usize::try_from(offset) {
        offset
    } else {
        let offset = offset
            .checked_neg()
            .and_then(|offset| usize::try_from(offset).ok())
            .and_then(|offset| offset.checked_sub(string.len()));
        if let Some(offset) = offset {
            offset
        } else {
            return -1;
        }
    };
    let haystack = if let Some(haystack) = string.get(offset..) {
        haystack
    } else {
        return -1;
    };
    if slen == 0 {
        return offset as sys::mrb_int;
    }
    let needle = slice::from_raw_parts(sptr.cast::<u8>(), usize::try_from(slen).unwrap_or_default());
    haystack.find(needle).map(|pos| pos as sys::mrb_int).unwrap_or(-1)
}

#[no_mangle]
#[allow(clippy::cast_possible_truncation)]
#[allow(clippy::cast_sign_loss)]
unsafe extern "C" fn mrb_str_artichoke_free(mrb: *mut sys::mrb_state, string: *mut sys::RString) {
    let _ = mrb;

    let ptr = (*string).as_.heap.ptr;
    let len = (*string).as_.heap.len as usize;
    let capacity = (*string).as_.heap.aux.capa as usize;

    // Zero capacity `Vec`s are created with a dangling `ptr`.
    if len == 0 && capacity == 0 {
        let _ = String::from_raw_parts(ptr.cast::<u8>(), len, capacity);
        return;
    }

    // we don't need to free the encoding since `Encoding` is `Copy` and we pack
    // it into the `RString` flags as a `u32`.

    let _ = String::from_raw_parts(ptr.cast::<u8>(), len, capacity);
}
