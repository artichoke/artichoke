#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
// Tests derived from mrusty @ 1.0.0
// <https://github.com/anima-engine/mrusty/tree/v1.0.0>

// mrusty. mruby safe bindings for Rust
// Copyright (C) 2016  Dragoș Tiselice
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

//! The tests for the mruby `sys` module defined in this module serve as an
//! implementation guide and API examples for the `mruby` higher-level
//! Rust bindings.
//!
//! ## Implementation Notes
//!
//! ### `mrb_value` Types
//!
//! `mrb_type` is defined in the `boxing*.h` family of headers. The mruby
//! bindings includes `mruby/boxing_no.h` in `mrb-sys.h` which defines
//! `mrb_type` with a macro:
//!
//! ```c
//! #define mrb_type(o)     (o).tt
//! ```
//!
//! It is safe to directly access the `tt` field of an `mrb_value` to determine
//! its type.
//!
//! ### Strings
//!
//! There are two ways to pass Rust strings across an FFI boundary:
//!
//! - Call `as_ptr` on a `&str` and pass the length of the `&str`. This does not
//!   create a NUL-terminated ("\0") *char. An mruby function that has this API
//!   is `mrb_load_nstring_cxt`.
//! - Create a `CString` from a `&str` for a traditional `char *` C string. This
//!   creates a NUL-terminated ("\0") `char *`. An mruby function that has this
//!   API is `mrb_define_class`.
//!
//! ### Exceptions
//!
//! If a Rust function that manipulates mruby state can raise a Ruby exception,
//! calling the function via `mrb_protect` executes the function surrounded by
//! a try catch and will return the result of the Rust function unless the code
//! raises, in which case it returns the exception.
//!
//! `mrb_protect` is useful in the context of executing untrusted code. Wrapping
//! all eval'd code in a rust function which is called via `mrb_protect`
//! ensures that code exits cleanly and we can report runtime errors to the
//! caller.
//!
//! ### Boolean return values
//!
//! mrb methods that return `mrb_bool` return a `u8`. FALSE <=> `0_u8`. TRUE <=>
//! any non-zero `u8`.

use std::ffi::{CStr, CString};

use super::*;

#[test]
fn open_close() {
    unsafe {
        let mrb = mrb_open();

        mrb_close(mrb);
    }
}

#[test]
fn sys_ext_nil_check() {
    unsafe {
        // mruby implements `nil` and `false` with `MRB_TT_FALSE` value type.
        // The difference between the two is the value of `value.i`. Ensure we
        // can tell the difference.
        let value = mrb_sys_nil_value();
        assert_eq!(mrb_sys_value_is_nil(value), true);
        assert_eq!(mrb_sys_value_is_false(value), false);
        assert_eq!(mrb_sys_value_is_true(value), false);
    }
}

#[test]
fn sys_ext_false_check() {
    unsafe {
        // mruby implements `nil` and `false` with `MRB_TT_FALSE` value type.
        // The difference between the two is the value of `value.i`. Ensure we
        // can tell the difference.
        let value = mrb_sys_false_value();
        assert_eq!(mrb_sys_value_is_nil(value), false);
        assert_eq!(mrb_sys_value_is_false(value), true);
        assert_eq!(mrb_sys_value_is_true(value), false);
    }
}

#[test]
fn sys_ext_true_check() {
    unsafe {
        // mruby implements `nil` and `false` with `MRB_TT_FALSE` value type.
        // The difference between the two is the value of `value.i`. Ensure we
        // can tell the difference.
        let value = mrb_sys_true_value();
        assert_eq!(mrb_sys_value_is_nil(value), false);
        assert_eq!(mrb_sys_value_is_false(value), false);
        assert_eq!(mrb_sys_value_is_true(value), true);
    }
}

#[test]
fn symbol_to_string() {
    unsafe {
        let mrb = mrb_open();

        let name = CString::new(":symbol").expect(":symbol literal");
        let literal = name.as_ptr() as *const i8;
        let symbol = mrb_intern_cstr(mrb, literal);

        let s = mrb_sym2name(mrb, symbol) as *const i8;
        let s = CStr::from_ptr(s).to_str().expect(":symbol name");
        assert_eq!(s, r#"":symbol""#);

        let mut s = mrb_sym2str(mrb, symbol);
        assert_eq!(s.tt, mrb_vtype::MRB_TT_STRING);
        let s = mrb_string_value_cstr(mrb, &mut s) as *const i8;
        let s = CStr::from_ptr(s).to_str().expect(":symbol.to_s");
        assert_eq!(s, ":symbol");

        mrb_close(mrb);
    }
}

#[test]
fn define_method() {
    unsafe {
        extern "C" fn rust__mruby__test_class__method__value(
            _mrb: *mut mrb_state,
            _slf: mrb_value,
        ) -> mrb_value {
            unsafe { mrb_sys_fixnum_value(2) }
        }

        let mrb = mrb_open();
        let context = mrbc_context_new(mrb);

        let obj_str = CString::new("Object").unwrap();
        let obj_class = mrb_class_get(mrb, obj_str.as_ptr());
        let new_class_str = CString::new("TestClass").unwrap();
        let new_class = mrb_define_class(mrb, new_class_str.as_ptr(), obj_class);

        let new_method_str = CString::new("value").unwrap();

        mrb_define_method(
            mrb,
            new_class,
            new_method_str.as_ptr(),
            Some(rust__mruby__test_class__method__value),
            0,
        );

        let code = "TestClass.new.value";
        let result = mrb_load_nstring_cxt(mrb, code.as_ptr() as *const i8, code.len(), context);
        assert_eq!(mrb_sys_fixnum_to_cint(result), 2);

        mrbc_context_free(mrb, context);
        mrb_close(mrb);
    }
}

#[test]
fn class_defined() {
    unsafe {
        let mrb = mrb_open();

        let obj_str = CString::new("Object").unwrap();
        let kernel_str = CString::new("Kernel").unwrap();
        let unknown_str = CString::new("TestClass").unwrap();

        assert_eq!(mrb_class_defined(mrb, obj_str.as_ptr()), 1_u8);
        assert_eq!(mrb_class_defined(mrb, kernel_str.as_ptr()), 1_u8);
        assert_eq!(mrb_class_defined(mrb, unknown_str.as_ptr()), 0_u8);

        mrb_close(mrb);
    }
}

#[test]
fn class_name() {
    unsafe {
        let mrb = mrb_open();

        let obj_str = CString::new("Object").unwrap();
        let obj_class = mrb_class_get(mrb, obj_str.as_ptr());
        let new_class_str = CString::new("TestClass").unwrap();
        let new_class = mrb_define_class(mrb, new_class_str.as_ptr(), obj_class);

        let kernel_str = CString::new("Kernel").unwrap();
        let kernel = mrb_module_get(mrb, kernel_str.as_ptr());

        let class_name = mrb_class_name(mrb, new_class);

        assert_eq!(CStr::from_ptr(class_name).to_str().unwrap(), "TestClass");

        let kernel_name = mrb_class_name(mrb, kernel);

        assert_eq!(CStr::from_ptr(kernel_name).to_str().unwrap(), "Kernel");

        mrb_close(mrb);
    }
}

#[test]
fn class_value() {
    unsafe {
        let mrb = mrb_open();

        let obj_str = CString::new("Object").unwrap();
        let obj_class = mrb_class_get(mrb, obj_str.as_ptr());
        let obj_class = mrb_sys_class_value(obj_class);

        let to_s_str = CString::new("to_s").unwrap();
        let args = &[];

        let sym = mrb_intern(mrb, to_s_str.as_ptr(), 4_usize);

        let result = mrb_funcall_argv(mrb, obj_class, sym, 0, args.as_ptr());

        let s = mrb_str_to_cstr(mrb, result) as *const i8;
        assert_eq!(result.tt, mrb_vtype::MRB_TT_STRING);
        assert_eq!(CStr::from_ptr(s).to_str().unwrap(), "Object");

        mrb_close(mrb);
    }
}

#[test]
fn nil_class() {
    unsafe {
        let mrb = mrb_open();

        let nil = CString::new("NilClass").unwrap();
        let nil_class = mrb_class_get(mrb, nil.as_ptr() as *const i8);

        let name = mrb_class_name(mrb, nil_class);

        assert_eq!(CStr::from_ptr(name).to_str().unwrap(), "NilClass");

        mrb_close(mrb);
    }
}

#[test]
fn nil_class_eval() {
    unsafe {
        let mrb = mrb_open();
        let context = mrbc_context_new(mrb);

        let code = "nil.class";
        let result = mrb_load_nstring_cxt(mrb, code.as_ptr() as *const i8, code.len(), context);
        assert_eq!(result.tt, mrb_vtype::MRB_TT_CLASS);
        let s = mrb_class_name(mrb, result.value.p as *mut ffi::RClass);
        assert_eq!(CStr::from_ptr(s).to_str().unwrap(), "NilClass");

        mrbc_context_free(mrb, context);
        mrb_close(mrb);
    }
}

#[test]
fn nil_class_name_eval() {
    unsafe {
        let mrb = mrb_open();
        let context = mrbc_context_new(mrb);

        let code = "nil.class.to_s";
        let result = mrb_load_nstring_cxt(mrb, code.as_ptr() as *const i8, code.len(), context);
        assert_eq!(result.tt, mrb_vtype::MRB_TT_STRING);
        let s = mrb_str_to_cstr(mrb, result) as *const i8;
        assert_eq!(CStr::from_ptr(s).to_str().unwrap(), "NilClass");

        mrbc_context_free(mrb, context);
        mrb_close(mrb);
    }
}

#[test]
fn define_module() {
    unsafe {
        let mrb = mrb_open();

        let mod_str = CString::new("TestModule").unwrap();
        let unknown_str = CString::new("UnknownModule").unwrap();

        mrb_define_module(mrb, mod_str.as_ptr());

        assert_eq!(mrb_class_defined(mrb, mod_str.as_ptr()), 1_u8);
        assert_eq!(mrb_class_defined(mrb, unknown_str.as_ptr()), 0_u8);

        mrb_close(mrb);
    }
}

#[test]
fn defined_under() {
    unsafe {
        let mrb = mrb_open();

        let kernel_str = CString::new("Kernel").unwrap();
        let kernel = mrb_module_get(mrb, kernel_str.as_ptr());
        let name_str = CString::new("TestModule").unwrap();
        let name = name_str.as_ptr();

        mrb_define_module_under(mrb, kernel, name);
        assert_eq!(mrb_class_defined_under(mrb, kernel, name), 1_u8);

        let mod_str = CString::new("TestModule").unwrap();
        let module = mrb_module_get(mrb, mod_str.as_ptr());
        let module_name = mrb_class_name(mrb, module);
        assert_eq!(
            CStr::from_ptr(module_name).to_str().unwrap(),
            "Kernel::TestModule"
        );

        mrb_close(mrb);
    }
}

#[test]
fn class_under() {
    unsafe {
        let mrb = mrb_open();

        let obj_str = CString::new("Object").unwrap();
        let obj_class = mrb_class_get(mrb, obj_str.as_ptr());
        let name_str = CString::new("TestClass").unwrap();
        let name = name_str.as_ptr();

        mrb_define_class_under(mrb, obj_class, name, obj_class);
        let new_class = mrb_class_get_under(mrb, obj_class, name);

        let class_name = mrb_class_name(mrb, new_class);

        assert_eq!(CStr::from_ptr(class_name).to_str().unwrap(), "TestClass");

        mrb_close(mrb);
    }
}

#[test]
fn module_under() {
    unsafe {
        let mrb = mrb_open();

        let kernel_str = CString::new("Kernel").unwrap();
        let kernel = mrb_module_get(mrb, kernel_str.as_ptr());
        let name_str = CString::new("TestModule").unwrap();
        let name = name_str.as_ptr();

        mrb_define_module_under(mrb, kernel, name);
        let new_module = mrb_module_get_under(mrb, kernel, name);

        let module_name = mrb_class_name(mrb, new_module);

        assert_eq!(
            CStr::from_ptr(module_name).to_str().unwrap(),
            "Kernel::TestModule"
        );

        mrb_close(mrb);
    }
}

#[test]
fn include_module() {
    unsafe {
        let mrb = mrb_open();
        let context = mrbc_context_new(mrb);

        let module_code = "module Increment; def inc; self + 1; end; end";
        mrb_load_nstring_cxt(
            mrb,
            module_code.as_ptr() as *const i8,
            module_code.len(),
            context,
        );

        let fixnum_str = CString::new("Fixnum").unwrap();
        let fixnum = mrb_class_get(mrb, fixnum_str.as_ptr());
        let increment_str = CString::new("Increment").unwrap();
        let increment = mrb_module_get(mrb, increment_str.as_ptr());

        mrb_include_module(mrb, fixnum, increment);

        let code = "1.inc";
        let result = mrb_load_nstring_cxt(mrb, code.as_ptr() as *const i8, code.len(), context);
        assert_eq!(mrb_sys_fixnum_to_cint(result), 2);

        mrbc_context_free(mrb, context);
        mrb_close(mrb);
    }
}

#[test]
fn define_and_include_module() {
    unsafe {
        extern "C" fn rust__mruby__increment__method__inc(
            mrb: *mut mrb_state,
            slf: mrb_value,
        ) -> mrb_value {
            unsafe {
                // Assert self is a Fixnum and raise an ArgumentError otherwise.
                // This function requires self to be a Fixnum to safely access
                // the `i` field of the value union in the unsafe block.
                //
                // TODO: Write a standalone test for this behavior, see GH-153.
                if slf.tt == mrb_vtype::MRB_TT_FIXNUM {
                    // `unsafe` block required because we're accessing a union
                    // field which might access uninitialized memory. We know we
                    // this operation is safe because of the above assert on
                    // `slf.tt`.
                    mrb_sys_fixnum_value(slf.value.i + 1)
                } else {
                    let eclass = "ArgumentError";
                    let emsg = "expected Fixnum";
                    mrb_sys_raise(
                        mrb,
                        // TODO: scream! needs null terminator, see GH-153.
                        eclass.as_ptr() as *const i8,
                        emsg.as_ptr() as *const i8,
                    );
                    mrb_sys_nil_value()
                }
            }
        }

        let mrb = mrb_open();
        let context = mrbc_context_new(mrb);

        let name_str = CString::new("Increment").unwrap();
        let name = name_str.as_ptr();
        mrb_define_module(mrb, name);
        let increment = mrb_module_get(mrb, name);

        let inc_method_str = CString::new("inc").unwrap();

        mrb_define_method(
            mrb,
            increment,
            inc_method_str.as_ptr(),
            Some(rust__mruby__increment__method__inc),
            0,
        );

        let fixnum_str = CString::new("Fixnum").unwrap();
        let fixnum = mrb_class_get(mrb, fixnum_str.as_ptr());

        mrb_include_module(mrb, fixnum, increment);

        let code = "1.inc";
        let result = mrb_load_nstring_cxt(mrb, code.as_ptr() as *const i8, code.len(), context);
        assert_eq!(mrb_sys_fixnum_to_cint(result), 2);

        mrbc_context_free(mrb, context);
        mrb_close(mrb);
    }
}

#[test]
fn define_class_method() {
    unsafe {
        extern "C" fn rust__mruby__test_class__class_method__value(
            _mrb: *mut mrb_state,
            _slf: mrb_value,
        ) -> mrb_value {
            unsafe { mrb_sys_fixnum_value(2) }
        }

        let mrb = mrb_open();
        let context = mrbc_context_new(mrb);

        let obj_str = CString::new("Object").unwrap();
        let obj_class = mrb_class_get(mrb, obj_str.as_ptr());
        let new_class_str = CString::new("TestClass").unwrap();
        let new_class = mrb_define_class(mrb, new_class_str.as_ptr(), obj_class);

        let new_method_str = CString::new("value").unwrap();

        mrb_define_class_method(
            mrb,
            new_class,
            new_method_str.as_ptr(),
            Some(rust__mruby__test_class__class_method__value),
            0,
        );

        let code = "TestClass.value";
        let result = mrb_load_nstring_cxt(mrb, code.as_ptr() as *const i8, code.len(), context);
        assert_eq!(mrb_sys_fixnum_to_cint(result), 2);

        mrbc_context_free(mrb, context);
        mrb_close(mrb);
    }
}

#[test]
fn define_class_and_instance_method_with_one_rust_function() {
    unsafe {
        extern "C" fn rust__mruby__test_class__method__value(
            _mrb: *mut mrb_state,
            slf: mrb_value,
        ) -> mrb_value {
            unsafe {
                match slf.tt {
                    mrb_vtype::MRB_TT_OBJECT => mrb_sys_fixnum_value(2),
                    mrb_vtype::MRB_TT_CLASS => mrb_sys_fixnum_value(3),
                    tt => unreachable!("unexpected mrb_value type: {:?}", tt),
                }
            }
        }

        let mrb = mrb_open();
        let context = mrbc_context_new(mrb);

        let obj_str = CString::new("Object").unwrap();
        let obj_class = mrb_class_get(mrb, obj_str.as_ptr());
        let new_class_str = CString::new("TestClass").unwrap();
        let new_class = mrb_define_class(mrb, new_class_str.as_ptr(), obj_class);

        let rust__mruby__test_class__class_method__value = rust__mruby__test_class__method__value;

        let new_method_str = CString::new("value").unwrap();
        let new_class_method_str = CString::new("value").unwrap();

        mrb_define_method(
            mrb,
            new_class,
            new_method_str.as_ptr(),
            Some(rust__mruby__test_class__method__value),
            0,
        );
        mrb_define_class_method(
            mrb,
            new_class,
            new_class_method_str.as_ptr(),
            Some(rust__mruby__test_class__class_method__value),
            0,
        );

        let code = "TestClass.value + TestClass.new.value";
        let result = mrb_load_nstring_cxt(mrb, code.as_ptr() as *const i8, code.len(), context);
        assert_eq!(mrb_sys_fixnum_to_cint(result), 5);

        mrbc_context_free(mrb, context);
        mrb_close(mrb);
    }
}

#[test]
fn define_constant() {
    unsafe {
        let mrb = mrb_open();
        let context = mrbc_context_new(mrb);

        let obj_str = CString::new("Object").unwrap();
        let obj_class = mrb_class_get(mrb, obj_str.as_ptr());
        let kernel_str = CString::new("Kernel").unwrap();
        let kernel = mrb_module_get(mrb, kernel_str.as_ptr());

        let one = mrb_sys_fixnum_value(1);
        let one_str = CString::new("ONE").unwrap();

        // Define constant on Class
        mrb_define_const(mrb, obj_class, one_str.as_ptr(), one);
        // Define constant on Module
        mrb_define_const(mrb, kernel, one_str.as_ptr(), one);

        let object_one_code = "Object::ONE";

        let result = mrb_load_nstring_cxt(
            mrb,
            object_one_code.as_ptr() as *const i8,
            object_one_code.len(),
            context,
        );
        assert_eq!(mrb_sys_fixnum_to_cint(result), 1);

        let kernel_one_code = "Kernel::ONE";

        let result = mrb_load_nstring_cxt(
            mrb,
            kernel_one_code.as_ptr() as *const i8,
            kernel_one_code.len(),
            context,
        );
        assert_eq!(mrb_sys_fixnum_to_cint(result), 1);

        mrbc_context_free(mrb, context);
        mrb_close(mrb);
    }
}

#[test]
fn protect() {
    use std::mem;

    unsafe {
        // This rust function raises a runtime error in mruby
        extern "C" fn rust__mruby__test_class__method__value(
            mrb: *mut mrb_state,
            _data: mrb_value,
        ) -> mrb_value {
            unsafe {
                let eclass = CString::new("RuntimeError").unwrap();
                let msg = CString::new("excepting").unwrap();
                mrb_sys_raise(mrb as *mut mrb_state, eclass.as_ptr(), msg.as_ptr());
                mrb_sys_fixnum_value(7)
            }
        }

        let mrb = mrb_open();

        let mut state = <mem::MaybeUninit<mrb_bool>>::uninit();
        let nil = mrb_sys_nil_value();

        // `mrb_protect` calls the passed function with `data` as an argument.
        // Protect wraps the execution of the provided function in a try catch.
        // If the passed function raises, the `state` out variable is set to
        // true and the returned `mrb_value` will be an exception. If no
        // exception is thrown, the returned `mrb_value` is the value returned
        // by the provided function.
        //
        // The function we are passing below raises a `RuntimeException`, so we
        // expect `exc` to be an `Exception`.
        let exc = mrb_protect(
            mrb as *mut mrb_state,
            Some(rust__mruby__test_class__method__value),
            nil,
            state.as_mut_ptr(),
        );

        // state == true means an exception was thrown by the protected function
        assert_ne!(state.assume_init(), 0_u8);
        assert_eq!(exc.tt, mrb_vtype::MRB_TT_EXCEPTION);

        let args = &[];

        // This code calls `.class.to_s` on the `RuntimeException` our
        // protected `value` method raised.
        let class_str = "class";
        let class_sym = mrb_intern(mrb, class_str.as_ptr() as *const i8, class_str.len());
        let to_s_str = "to_s";
        let to_s_sym = mrb_intern(mrb, to_s_str.as_ptr() as *const i8, to_s_str.len());

        // `mrb_funcall_argv` calls a method named by a symbol with a list of
        // arguments on an `mrb_value`.
        let class = mrb_funcall_argv(mrb, exc, class_sym, 0, args.as_ptr());
        let result = mrb_funcall_argv(mrb, class, to_s_sym, 0, args.as_ptr());

        assert_eq!(result.tt, mrb_vtype::MRB_TT_STRING);
        let s = mrb_str_to_cstr(mrb, result) as *const i8;
        assert_eq!(CStr::from_ptr(s).to_str().unwrap(), "RuntimeError");

        mrb_close(mrb);
    }
}

#[test]
pub fn args() {
    use std::mem;

    unsafe {
        extern "C" fn add(mrb: *mut mrb_state, _slf: mrb_value) -> mrb_value {
            unsafe {
                let mut a = <mem::MaybeUninit<mrb_value>>::uninit();
                let mut b = <mem::MaybeUninit<mrb_value>>::uninit();

                let argspec = CString::new("oo").unwrap();
                mrb_get_args(mrb, argspec.as_ptr(), a.as_mut_ptr(), b.as_mut_ptr());
                let args = &[b.assume_init()];

                let plus = "+";
                let sym = mrb_intern(mrb, plus.as_ptr() as *const i8, plus.len());

                mrb_funcall_argv(mrb, a.assume_init(), sym, 1, args.as_ptr())
            }
        }

        let mrb = mrb_open();
        let context = mrbc_context_new(mrb);

        let obj_str = CString::new("Object").unwrap();
        let obj_class = mrb_class_get(mrb, obj_str.as_ptr());
        let new_class_str = CString::new("Mine").unwrap();
        let new_class = mrb_define_class(mrb, new_class_str.as_ptr(), obj_class);

        let add_str = CString::new("add").unwrap();

        mrb_define_method(
            mrb,
            new_class,
            add_str.as_ptr(),
            Some(add),
            crate::sys::args::mrb_args_req(2),
        );

        let code = "Mine.new.add(1, 1)";

        assert_eq!(
            mrb_sys_fixnum_to_cint(mrb_load_nstring_cxt(
                mrb,
                code.as_ptr() as *const i8,
                code.len(),
                context
            )),
            2
        );

        mrbc_context_free(mrb, context);
        mrb_close(mrb);
    }
}

#[test]
pub fn str_args() {
    use std::ffi::CStr;
    use std::mem;
    use std::os::raw::c_char;

    unsafe {
        extern "C" fn add(mrb: *mut mrb_state, _slf: mrb_value) -> mrb_value {
            unsafe {
                let mut a = <mem::MaybeUninit<*const c_char>>::uninit();
                let mut b = <mem::MaybeUninit<*const c_char>>::uninit();

                let argspec = CString::new("zz").unwrap();
                mrb_get_args(mrb, argspec.as_ptr(), a.as_mut_ptr(), b.as_mut_ptr());

                let a = CStr::from_ptr(a.assume_init()).to_str().unwrap();
                let b = CStr::from_ptr(b.assume_init()).to_str().unwrap();

                let value = mrb_str_new_cstr(mrb, a.as_ptr() as *const i8);
                let args = [mrb_str_new_cstr(mrb, b.as_ptr() as *const i8)];

                let plus = "+";
                let sym = mrb_intern(mrb, plus.as_ptr() as *const i8, plus.len());

                mrb_funcall_argv(mrb, value, sym, 1, args.as_ptr())
            }
        }

        let mrb = mrb_open();
        let context = mrbc_context_new(mrb);

        let obj_str = CString::new("Object").unwrap();
        let obj_class = mrb_class_get(mrb, obj_str.as_ptr());
        let new_class_str = CString::new("Mine").unwrap();
        let new_class = mrb_define_class(mrb, new_class_str.as_ptr(), obj_class);

        let add_str = CString::new("add").unwrap();

        mrb_define_method(
            mrb,
            new_class,
            add_str.as_ptr(),
            Some(add),
            args::mrb_args_req(2),
        );

        let code = "Mine.new.add('a', 'b')";

        let result = mrb_load_nstring_cxt(mrb, code.as_ptr() as *const i8, code.len(), context);
        let result = mrb_string_value_ptr(mrb, result);
        assert_eq!(CStr::from_ptr(result).to_str().unwrap(), "ab");

        mrbc_context_free(mrb, context);
        mrb_close(mrb);
    }
}

#[test]
fn funcall_argv() {
    unsafe {
        let mrb = mrb_open();

        let one = mrb_sys_fixnum_value(1);
        let two = mrb_sys_fixnum_value(2);
        let args = &[two];

        let plus_str = CString::new("+").unwrap();
        let sym = mrb_intern(mrb, plus_str.as_ptr(), 1_usize);

        let result = mrb_funcall_argv(mrb, one, sym, 1, args.as_ptr());

        assert_eq!(mrb_sys_fixnum_to_cint(result), 3);

        mrb_close(mrb);
    }
}

#[test]
fn iv() {
    unsafe {
        let mrb = mrb_open();
        let context = mrbc_context_new(mrb);

        let obj_str = CString::new("Object").unwrap();
        let obj_class = mrb_class_get(mrb, obj_str.as_ptr());
        let new_str = CString::new("Mine").unwrap();

        mrb_define_class(mrb, new_str.as_ptr(), obj_class);

        let one = mrb_sys_fixnum_value(1);

        let code = "Mine.new";
        let obj = mrb_load_nstring_cxt(mrb, code.as_ptr() as *const i8, code.len(), context);

        let value_str = CString::new("value").unwrap();

        let sym = mrb_intern(mrb, value_str.as_ptr(), 1);

        assert_eq!(mrb_iv_defined(mrb, obj, sym), 0_u8);

        mrb_iv_set(mrb, obj, sym, one);

        assert_eq!(mrb_iv_defined(mrb, obj, sym), 1_u8);
        assert_eq!(mrb_sys_fixnum_to_cint(mrb_iv_get(mrb, obj, sym)), 1);

        mrbc_context_free(mrb, context);
        mrb_close(mrb);
    }
}
