use core::char;
use core::convert::TryFrom;
use core::hash::{BuildHasher, Hash, Hasher};
use core::ptr;
use core::slice;
use core::str;
use std::ffi::{c_void, CStr};
use std::io::Write as _;
use std::os::raw::{c_char, c_double, c_int};

use artichoke_core::convert::Convert;
use artichoke_core::hash::Hash as _;
use bstr::ByteSlice;
use spinoso_exception::ArgumentError;
use spinoso_exception::NoMemoryError;
use spinoso_string::{RawParts, String};

use crate::convert::BoxUnboxVmValue;
use crate::error;
use crate::sys;
use crate::value::Value;

// MRB_API mrb_value mrb_str_new_capa(mrb_state *mrb, size_t capa)
#[no_mangle]
unsafe extern "C" fn mrb_str_new_capa(mrb: *mut sys::mrb_state, capa: usize) -> sys::mrb_value {
    unwrap_interpreter!(mrb, to => guard);
    let result = String::with_capacity(capa);
    let result = String::alloc_value(result, &mut guard);
    match result {
        Ok(value) => value.inner(),
        Err(exception) => error::raise(guard, exception),
    }
}

// MRB_API mrb_value mrb_str_new(mrb_state *mrb, const char *p, size_t len)
#[no_mangle]
unsafe extern "C" fn mrb_str_new(mrb: *mut sys::mrb_state, p: *const c_char, len: usize) -> sys::mrb_value {
    unwrap_interpreter!(mrb, to => guard);
    let s = if p.is_null() {
        String::utf8(vec![0; len])
    } else {
        let bytes = slice::from_raw_parts(p.cast::<u8>(), len);
        let bytes = bytes.to_vec();
        String::utf8(bytes)
    };
    let result = String::alloc_value(s, &mut guard);
    match result {
        Ok(value) => value.inner(),
        Err(exception) => error::raise(guard, exception),
    }
}

// MRB_API mrb_value mrb_str_new_cstr(mrb_state *mrb, const char *p)
#[no_mangle]
unsafe extern "C" fn mrb_str_new_cstr(mrb: *mut sys::mrb_state, p: *const c_char) -> sys::mrb_value {
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
unsafe extern "C" fn mrb_str_new_static(mrb: *mut sys::mrb_state, p: *const c_char, len: usize) -> sys::mrb_value {
    // Artichoke doesn't have a static string optimization.
    mrb_str_new(mrb, p, len)
}

// MRB_API mrb_int mrb_str_index(mrb_state *mrb, mrb_value str, const char *sptr, mrb_int slen, mrb_int offset)
#[no_mangle]
unsafe extern "C" fn mrb_str_index(
    mrb: *mut sys::mrb_state,
    s: sys::mrb_value,
    sptr: *const c_char,
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
    let needle = slice::from_raw_parts(sptr.cast::<u8>(), usize::try_from(slen).unwrap_or_default());
    if needle.is_empty() {
        return offset as sys::mrb_int;
    }
    haystack.find(needle).map_or(-1, |pos| pos as sys::mrb_int)
}

// MRB_API mrb_int mrb_str_strlen(mrb_state *mrb, struct RString *s)
//
// NOTE: Implemented in C in `mruby-sys/src/mruby-sys/ext.c`.

// MRB_API void mrb_str_modify_keep_ascii(mrb_state *mrb, struct RString *s)
//
// MRB_API void mrb_str_modify(mrb_state *mrb, struct RString *s)
//
// NOTE: Implemented in C in `mruby-sys/src/mruby-sys/ext.c`.

// MRB_API mrb_value mrb_str_resize(mrb_state *mrb, mrb_value str, mrb_int len)
#[no_mangle]
unsafe extern "C" fn mrb_str_resize(mrb: *mut sys::mrb_state, s: sys::mrb_value, len: sys::mrb_int) -> sys::mrb_value {
    unwrap_interpreter!(mrb, to => guard, or_else = s);
    let mut value = s.into();
    let mut allocation_failure = None;
    let result = if let Ok(mut string) = String::unbox_from_value(&mut value, &mut guard) {
        let len = if let Ok(len) = usize::try_from(len) {
            len
        } else {
            return s;
        };
        // Safety:
        //
        // The string is repacked before any intervening use of the interpreter.
        // The string is repacked before any intervening mruby heap allocations.
        let string_mut = string.as_inner_mut();

        match len.checked_sub(string_mut.len()) {
            Some(0) => return s,
            Some(additional) => {
                if let Err(err) = string_mut.try_reserve(additional) {
                    allocation_failure = Some(err);
                }
            }
            // If the given length is less than the length of the `String`, truncate.
            None => string_mut.truncate(len),
        }

        let inner = string.take();
        let value = String::box_into_value(inner, value, &mut guard).expect("String reboxing should not fail");

        value.inner()
    } else {
        s
    };
    if allocation_failure.is_some() {
        // NOTE: Ideally this code would distinguish between a capacity overflow
        // (string too large) vs an out of memory condition (allocation failure).
        // This is not possible on stable Rust since `TryReserveErrorKind` is
        // unstable.

        // NOTE: This code can't use an `Error` unified exception trait object.
        // Since we're in memory error territory, we're not sure if we can
        // allocate the `Box` it requires.
        let err = NoMemoryError::with_message("out of memory");
        error::raise(guard, err);
    } else {
        result
    }
}

// MRB_API char* mrb_str_to_cstr(mrb_state *mrb, mrb_value str0)
//
// NOTE: Not implemented.

// MRB_API void mrb_str_concat(mrb_state *mrb, mrb_value self, mrb_value other)
//
// NOTE: Implemented in C in `mruby-sys/src/mruby-sys/ext.c`.
//
// #[no_mangle]
// unsafe extern "C" mrb_str_concat(mrb: *mut sys::mrb_state, this: sys::mrb_value, other: sys::mrb_value) {
//     unwrap_interpreter!(mrb, to => guard, or_else = ());
// }

// MRB_API mrb_value mrb_str_plus(mrb_state *mrb, mrb_value a, mrb_value b)
#[no_mangle]
unsafe extern "C" fn mrb_str_plus(mrb: *mut sys::mrb_state, a: sys::mrb_value, b: sys::mrb_value) -> sys::mrb_value {
    unwrap_interpreter!(mrb, to => guard);
    let mut a = Value::from(a);
    let mut b = Value::from(b);

    let a = if let Ok(a) = String::unbox_from_value(&mut a, &mut guard) {
        a
    } else {
        return Value::nil().into();
    };
    let b = if let Ok(b) = String::unbox_from_value(&mut b, &mut guard) {
        b
    } else {
        return Value::nil().into();
    };

    let mut s = String::with_capacity_and_encoding(a.len() + b.len(), a.encoding());

    s.extend_from_slice(a.as_slice());
    s.extend_from_slice(b.as_slice());

    let s = String::alloc_value(s, &mut guard).unwrap_or_default();
    s.into()
}

// MRB_API int mrb_str_cmp(mrb_state *mrb, mrb_value str1, mrb_value str2)
#[no_mangle]
unsafe extern "C" fn mrb_str_cmp(mrb: *mut sys::mrb_state, str1: sys::mrb_value, str2: sys::mrb_value) -> c_int {
    unwrap_interpreter!(mrb, to => guard, or_else = -1);
    let mut a = Value::from(str1);
    let mut b = Value::from(str2);

    let a = if let Ok(a) = String::unbox_from_value(&mut a, &mut guard) {
        a
    } else {
        return -1;
    };
    let b = if let Ok(b) = String::unbox_from_value(&mut b, &mut guard) {
        b
    } else {
        return -1;
    };

    a.cmp(&*b) as c_int
}

// MRB_API mrb_bool mrb_str_equal(mrb_state *mrb, mrb_value str1, mrb_value str2)
#[no_mangle]
unsafe extern "C" fn mrb_str_equal(
    mrb: *mut sys::mrb_state,
    str1: sys::mrb_value,
    str2: sys::mrb_value,
) -> sys::mrb_bool {
    unwrap_interpreter!(mrb, to => guard, or_else = false as sys::mrb_bool);
    let mut a = Value::from(str1);
    let mut b = Value::from(str2);

    let a = if let Ok(a) = String::unbox_from_value(&mut a, &mut guard) {
        a
    } else {
        return false as sys::mrb_bool;
    };
    let b = if let Ok(b) = String::unbox_from_value(&mut b, &mut guard) {
        b
    } else {
        return false as sys::mrb_bool;
    };

    (*a == *b) as sys::mrb_bool
}

// MRB_API const char* mrb_string_value_ptr(mrb_state *mrb, mrb_value str)
//
// obsolete: use RSTRING_PTR()
//
// NOTE: not implemented

// MRB_API mrb_int mrb_string_value_len(mrb_state *mrb, mrb_value ptr)
//
// obsolete: use RSTRING_LEN()
//
// NOTE: not implemented

// MRB_API mrb_value mrb_str_dup(mrb_state *mrb, mrb_value str)
#[no_mangle]
unsafe extern "C" fn mrb_str_dup(mrb: *mut sys::mrb_state, s: sys::mrb_value) -> sys::mrb_value {
    unwrap_interpreter!(mrb, to => guard);
    let mut string = Value::from(s);
    let basic = sys::mrb_sys_basic_ptr(s).cast::<sys::RString>();
    let class = (*basic).c;

    if let Ok(string) = String::unbox_from_value(&mut string, &mut guard) {
        let dup = string.clone();
        if let Ok(value) = String::alloc_value(dup, &mut guard) {
            let value = value.inner();

            // dup'd strings keep the class of the source `String`.
            let dup_basic = sys::mrb_sys_basic_ptr(value).cast::<sys::RString>();
            (*dup_basic).c = class;

            return value;
        }
    }
    Value::nil().into()
}

// MRB_API mrb_value mrb_str_substr(mrb_state *mrb, mrb_value str, mrb_int beg, mrb_int len)
#[no_mangle]
unsafe extern "C" fn mrb_str_substr(
    mrb: *mut sys::mrb_state,
    s: sys::mrb_value,
    beg: sys::mrb_int,
    len: sys::mrb_int,
) -> sys::mrb_value {
    if len < 0 {
        return Value::nil().into();
    }
    unwrap_interpreter!(mrb, to => guard);

    let mut string = Value::from(s);
    let string = if let Ok(string) = String::unbox_from_value(&mut string, &mut guard) {
        string
    } else {
        return Value::nil().into();
    };

    let offset = if let Ok(offset) = usize::try_from(beg) {
        offset
    } else {
        let offset = beg
            .checked_neg()
            .and_then(|offset| usize::try_from(offset).ok())
            .and_then(|offset| offset.checked_sub(string.len()));
        if let Some(offset) = offset {
            offset
        } else {
            return Value::nil().into();
        }
    };

    if let Some(slice) = string.get(offset..) {
        let substr = String::with_bytes_and_encoding(slice.to_vec(), string.encoding());
        String::alloc_value(substr, &mut guard).unwrap_or_default().into()
    } else {
        Value::nil().into()
    }
}

// MRB_API mrb_value mrb_ptr_to_str(mrb_state *mrb, void *p)
#[no_mangle]
unsafe extern "C" fn mrb_ptr_to_str(mrb: *mut sys::mrb_state, p: *mut c_void) -> sys::mrb_value {
    unwrap_interpreter!(mrb, to => guard);
    let mut s = String::with_capacity(16 + 2);
    let _ignore = write!(s, "{:p}", p);
    String::alloc_value(s, &mut guard).unwrap_or_default().into()
}

// MRB_API mrb_value mrb_cstr_to_inum(mrb_state *mrb, const char *str, mrb_int base, mrb_bool badcheck)
//
// NOTE: not implemented.

// MRB_API const char* mrb_string_value_cstr(mrb_state *mrb, mrb_value *ptr)
//
// obsolete: use RSTRING_CSTR() or mrb_string_cstr()
#[no_mangle]
unsafe extern "C" fn mrb_string_value_cstr(mrb: *mut sys::mrb_state, ptr: *mut sys::mrb_value) -> *const c_char {
    unwrap_interpreter!(mrb, to => guard, or_else = ptr::null());
    let mut s = Value::from(*ptr);
    let mut string = if let Ok(string) = String::unbox_from_value(&mut s, &mut guard) {
        if let Some(b'\0') = string.last() {
            return string.as_ptr().cast();
        }
        string
    } else {
        return ptr::null();
    };
    // Safety:
    //
    // The string is repacked before any intervening use of the interpreter.
    // The string is repacked before any intervening mruby heap allocations.
    let string_mut = string.as_inner_mut();
    string_mut.push_byte(b'\0');
    // Safety:
    //
    // This raw pointer will not be invalidated since we rebox this `String`
    // into the mruby heap where the GC will keep it alive.
    let cstr = string.as_ptr().cast::<c_char>();

    let inner = string.take();
    String::box_into_value(inner, s, &mut guard).expect("String reboxing should not fail");

    cstr
}

// MRB_API const char* mrb_string_cstr(mrb_state *mrb, mrb_value str)
#[no_mangle]
unsafe extern "C" fn mrb_string_cstr(mrb: *mut sys::mrb_state, s: sys::mrb_value) -> *const c_char {
    unwrap_interpreter!(mrb, to => guard, or_else = ptr::null());
    let mut s = Value::from(s);
    let mut string = if let Ok(string) = String::unbox_from_value(&mut s, &mut guard) {
        if let Some(b'\0') = string.last() {
            return string.as_ptr().cast();
        }
        string
    } else {
        return ptr::null();
    };
    // Safety:
    //
    // The string is repacked before any intervening use of the interpreter.
    // The string is repacked before any intervening mruby heap allocations.
    let string_mut = string.as_inner_mut();
    string_mut.push_byte(b'\0');
    // Safety:
    //
    // This raw pointer will not be invalidated since we rebox this `String`
    // into the mruby heap where the GC will keep it alive.
    let cstr = string.as_ptr().cast::<c_char>();

    let inner = string.take();
    String::box_into_value(inner, s, &mut guard).expect("String reboxing should not fail");

    cstr
}

// MRB_API mrb_value mrb_str_to_inum(mrb_state *mrb, mrb_value str, mrb_int base, mrb_bool badcheck)
//
// This function converts a numeric string to numeric mrb_value with the given base.
#[no_mangle]
#[allow(clippy::cast_sign_loss)]
#[allow(clippy::cast_possible_truncation)]
unsafe extern "C" fn mrb_str_to_inum(
    mrb: *mut sys::mrb_state,
    s: sys::mrb_value,
    base: sys::mrb_int,
    badcheck: sys::mrb_bool,
) -> sys::mrb_value {
    unwrap_interpreter!(mrb, to => guard);
    let badcheck = badcheck != 0;

    let mut s = Value::from(s);
    let s = if let Ok(s) = String::unbox_from_value(&mut s, &mut guard) {
        s
    } else if badcheck {
        let err = ArgumentError::with_message("not a string");
        error::raise(guard, err);
    } else {
        return guard.convert(0_i64).into();
    };
    let num = if let Ok(s) = str::from_utf8(s.as_slice()) {
        if let Ok(num) = s.parse::<i64>() {
            num
        } else if badcheck {
            let err = ArgumentError::with_message("invalid number");
            error::raise(guard, err);
        } else {
            return guard.convert(0_i64).into();
        }
    } else if badcheck {
        let err = ArgumentError::with_message("invalid number");
        error::raise(guard, err);
    } else {
        return guard.convert(0_i64).into();
    };
    let radix = match u32::try_from(base) {
        Ok(base) if (2..=36).contains(&base) => base,
        Ok(_) | Err(_) => {
            let err = ArgumentError::with_message("illegal radix");
            error::raise(guard, err);
        }
    };
    let mut result = vec![];
    let mut x = num;

    loop {
        let m = x % base;
        x /= base;

        // will panic if you use a bad radix (< 2 or > 36).
        result.push(char::from_digit(m as u32, radix).unwrap());
        if x == 0 {
            break;
        }
    }
    let int = result.into_iter().rev().collect::<String>();
    String::alloc_value(int, &mut guard).unwrap_or_default().into()
}

// MRB_API double mrb_cstr_to_dbl(mrb_state *mrb, const char *s, mrb_bool badcheck)
//
// NOTE: not implemented

// MRB_API double mrb_str_to_dbl(mrb_state *mrb, mrb_value str, mrb_bool badcheck)
#[no_mangle]
unsafe extern "C" fn mrb_str_to_dbl(mrb: *mut sys::mrb_state, s: sys::mrb_value, badcheck: sys::mrb_bool) -> c_double {
    unwrap_interpreter!(mrb, to => guard, or_else = 0.0);
    let badcheck = badcheck != 0;

    let mut s = Value::from(s);
    let s = if let Ok(s) = String::unbox_from_value(&mut s, &mut guard) {
        s
    } else if badcheck {
        let err = ArgumentError::with_message("not a string");
        error::raise(guard, err);
    } else {
        return 0.0;
    };
    if let Ok(s) = str::from_utf8(s.as_slice()) {
        if let Ok(num) = s.parse::<c_double>() {
            num
        } else if badcheck {
            let err = ArgumentError::with_message("invalid number");
            error::raise(guard, err);
        } else {
            0.0
        }
    } else if badcheck {
        let err = ArgumentError::with_message("invalid number");
        error::raise(guard, err);
    } else {
        0.0
    }
}

// MRB_API mrb_value mrb_str_cat(mrb_state *mrb, mrb_value str, const char *ptr, size_t len)
#[no_mangle]
unsafe extern "C" fn mrb_str_cat(
    mrb: *mut sys::mrb_state,
    s: sys::mrb_value,
    ptr: *const c_char,
    len: usize,
) -> sys::mrb_value {
    unwrap_interpreter!(mrb, to => guard, or_else = s);
    let mut s = Value::from(s);
    if let Ok(mut string) = String::unbox_from_value(&mut s, &mut guard) {
        let slice = slice::from_raw_parts(ptr.cast::<u8>(), len);

        // Safety:
        //
        // The string is repacked before any intervening use of the interpreter.
        // The string is repacked before any intervening mruby heap allocations.
        let string_mut = string.as_inner_mut();
        string_mut.extend_from_slice(slice);
        let inner = string.take();
        let value = String::box_into_value(inner, s, &mut guard).expect("String reboxing should not fail");

        value.inner()
    } else {
        s.inner()
    }
}

// MRB_API mrb_value mrb_str_cat_cstr(mrb_state *mrb, mrb_value str, const char *ptr)
//
// MRB_API mrb_value mrb_str_cat_str(mrb_state *mrb, mrb_value str, mrb_value str2)
//
// MRB_API mrb_value mrb_str_append(mrb_state *mrb, mrb_value str1, mrb_value str2)
//
// NOTE: Implemented in C in `mruby-sys/src/mruby-sys/ext.c`.

// MRB_API double mrb_float_read(const char *string, char **endPtr)
//
// NOTE: impl kept in C.

// uint32_t mrb_str_hash(mrb_state *mrb, mrb_value str);
#[no_mangle]
unsafe extern "C" fn mrb_str_hash(mrb: *mut sys::mrb_state, s: sys::mrb_value) -> u32 {
    unwrap_interpreter!(mrb, to => guard, or_else = 0);
    let mut s = Value::from(s);
    let mut hasher = if let Ok(global_build_hasher) = guard.global_build_hasher() {
        global_build_hasher.build_hasher()
    } else {
        return 0;
    };

    let s = if let Ok(s) = String::unbox_from_value(&mut s, &mut guard) {
        s
    } else {
        return 0;
    };
    s.as_slice().hash(&mut hasher);
    #[allow(clippy::cast_possible_truncation)]
    let hash = hasher.finish() as u32;
    hash
}

#[no_mangle]
#[allow(clippy::cast_possible_truncation)]
#[allow(clippy::cast_sign_loss)]
unsafe extern "C" fn mrb_gc_free_str(mrb: *mut sys::mrb_state, string: *mut sys::RString) {
    let _ = mrb;

    let ptr = (*string).as_.heap.ptr;
    let length = (*string).as_.heap.len as usize;
    let capacity = (*string).as_.heap.aux.capa as usize;

    // we don't need to free the encoding since `Encoding` is `Copy` and we pack
    // it into the `RString` flags as a `u32`.

    let raw_parts = RawParts {
        ptr: ptr.cast::<u8>(),
        length,
        capacity,
    };
    drop(String::from_raw_parts(raw_parts));
}
