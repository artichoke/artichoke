use libc;
use c2rust_bitfields::{BitfieldStruct};
extern "C" {
    pub type iv_tbl;
    pub type kh_mt;
    pub type mrb_irep;
    pub type symbol_name;
    pub type RProc;
    pub type REnv;
    pub type mrb_jmpbuf;
    #[no_mangle]
    fn memchr(_: *const libc::c_void, _: libc::c_int, _: libc::c_ulong)
     -> *mut libc::c_void;
    #[no_mangle]
    fn memcmp(_: *const libc::c_void, _: *const libc::c_void,
              _: libc::c_ulong) -> libc::c_int;
    #[no_mangle]
    fn memcpy(_: *mut libc::c_void, _: *const libc::c_void, _: libc::c_ulong)
     -> *mut libc::c_void;
    #[no_mangle]
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    #[no_mangle]
    fn __assert_rtn(_: *const libc::c_char, _: *const libc::c_char,
                    _: libc::c_int, _: *const libc::c_char) -> !;
    #[no_mangle]
    fn __error() -> *mut libc::c_int;
    /* *
 * Defines a new class.
 *
 * If you're creating a gem it may look something like this:
 *
 *      !!!c
 *      void mrb_example_gem_init(mrb_state* mrb) {
 *          struct RClass *example_class;
 *          example_class = mrb_define_class(mrb, "Example_Class", mrb->object_class);
 *      }
 *
 *      void mrb_example_gem_final(mrb_state* mrb) {
 *          //free(TheAnimals);
 *      }
 *
 * @param [mrb_state *] mrb The current mruby state.
 * @param [const char *] name The name of the defined class.
 * @param [struct RClass *] super The new class parent.
 * @return [struct RClass *] Reference to the newly defined class.
 * @see mrb_define_class_under
 */
    #[no_mangle]
    fn mrb_define_class(mrb: *mut mrb_state, name: *const libc::c_char,
                        super_0: *mut RClass) -> *mut RClass;
    /* *
 * Defines a global function in ruby.
 *
 * If you're creating a gem it may look something like this
 *
 * Example:
 *
 *     !!!c
 *     mrb_value example_method(mrb_state* mrb, mrb_value self)
 *     {
 *          puts("Executing example command!");
 *          return self;
 *     }
 *
 *     void mrb_example_gem_init(mrb_state* mrb)
 *     {
 *           mrb_define_method(mrb, mrb->kernel_module, "example_method", example_method, MRB_ARGS_NONE());
 *     }
 *
 * @param [mrb_state *] mrb The MRuby state reference.
 * @param [struct RClass *] cla The class pointer where the method will be defined.
 * @param [const char *] name The name of the method being defined.
 * @param [mrb_func_t] func The function pointer to the method definition.
 * @param [mrb_aspec] aspec The method parameters declaration.
 */
    #[no_mangle]
    fn mrb_define_method(mrb: *mut mrb_state, cla: *mut RClass,
                         name: *const libc::c_char, func: mrb_func_t,
                         aspec: mrb_aspec);
    /* *
 * Gets a exception class.
 * @param [mrb_state*] mrb The current mruby state.
 * @param [const char *] name The name of the class.
 * @return [struct RClass *] A reference to the class.
*/
    #[no_mangle]
    fn mrb_exc_get(mrb: *mut mrb_state, name: *const libc::c_char)
     -> *mut RClass;
    /* *
 * Retrieve arguments from mrb_state.
 *
 * @param mrb The current MRuby state.
 * @param format [mrb_args_format] is a list of format specifiers
 * @param ... The passing variadic arguments must be a pointer of retrieving type.
 * @return the number of arguments retrieved.
 * @see mrb_args_format
 */
    #[no_mangle]
    fn mrb_get_args(mrb: *mut mrb_state, format: mrb_args_format, _: ...)
     -> mrb_int;
    /* `strlen` for character string literals (use with caution or `strlen` instead)
    Adjacent string literals are concatenated in C/C++ in translation phase 6.
    If `lit` is not one, the compiler will report a syntax error:
     MSVC: "error C2143: syntax error : missing ')' before 'string'"
     GCC:  "error: expected ')' before string constant"
*/
    /* *
 * Call existing ruby functions.
 *
 *      #include <stdio.h>
 *      #include <mruby.h>
 *      #include "mruby/compile.h"
 *
 *      int
 *      main()
 *      {
 *        mrb_int i = 99;
 *        mrb_state *mrb = mrb_open();
 *
 *        if (!mrb) { }
 *        FILE *fp = fopen("test.rb","r");
 *        mrb_value obj = mrb_load_file(mrb,fp);
 *        mrb_funcall(mrb, obj, "method_name", 1, mrb_fixnum_value(i));
 *        fclose(fp);
 *        mrb_close(mrb);
 *       }
 * @param [mrb_state*] mrb_state* The current mruby state.
 * @param [mrb_value] mrb_value A reference to an mruby value.
 * @param [const char*] const char* The name of the method.
 * @param [mrb_int] mrb_int The number of arguments the method has.
 * @param [...] ... Variadic values(not type safe!).
 * @return [mrb_value] mrb_value mruby function value.
 */
    #[no_mangle]
    fn mrb_funcall(_: *mut mrb_state, _: mrb_value, _: *const libc::c_char,
                   _: mrb_int, _: ...) -> mrb_value;
    #[no_mangle]
    fn mrb_intern_static(_: *mut mrb_state, _: *const libc::c_char, _: size_t)
     -> mrb_sym;
    #[no_mangle]
    fn mrb_intern_str(_: *mut mrb_state, _: mrb_value) -> mrb_sym;
    #[no_mangle]
    fn mrb_malloc(_: *mut mrb_state, _: size_t) -> *mut libc::c_void;
    #[no_mangle]
    fn mrb_realloc(_: *mut mrb_state, _: *mut libc::c_void, _: size_t)
     -> *mut libc::c_void;
    #[no_mangle]
    fn mrb_obj_alloc(_: *mut mrb_state, _: mrb_vtype, _: *mut RClass)
     -> *mut RBasic;
    #[no_mangle]
    fn mrb_free(_: *mut mrb_state, _: *mut libc::c_void);
    #[no_mangle]
    fn mrb_raise(mrb: *mut mrb_state, c: *mut RClass,
                 msg: *const libc::c_char) -> !;
    #[no_mangle]
    fn mrb_Integer(mrb: *mut mrb_state, val: mrb_value) -> mrb_value;
    #[no_mangle]
    fn mrb_inspect(mrb: *mut mrb_state, obj: mrb_value) -> mrb_value;
    #[no_mangle]
    fn mrb_any_to_s(mrb: *mut mrb_state, obj: mrb_value) -> mrb_value;
    #[no_mangle]
    fn mrb_obj_class(mrb: *mut mrb_state, obj: mrb_value) -> *mut RClass;
    #[no_mangle]
    fn mrb_convert_type(mrb: *mut mrb_state, val: mrb_value,
                        type_0: mrb_vtype, tname: *const libc::c_char,
                        method: *const libc::c_char) -> mrb_value;
    #[no_mangle]
    fn mrb_raisef(mrb: *mut mrb_state, c: *mut RClass,
                  fmt: *const libc::c_char, _: ...) -> !;
    #[no_mangle]
    fn mrb_frozen_error(mrb: *mut mrb_state, frozen_obj: *mut libc::c_void)
     -> !;
    #[no_mangle]
    fn mrb_to_str(mrb: *mut mrb_state, val: mrb_value) -> mrb_value;
    #[no_mangle]
    fn mrb_respond_to(mrb: *mut mrb_state, obj: mrb_value, mid: mrb_sym)
     -> mrb_bool;
    #[no_mangle]
    fn mrb_ary_new_capa(_: *mut mrb_state, _: mrb_int) -> mrb_value;
    /*
 * Initializes a new array.
 *
 * Equivalent to:
 *
 *      Array.new
 *
 * @param mrb The mruby state reference.
 * @return The initialized array.
 */
    #[no_mangle]
    fn mrb_ary_new(mrb: *mut mrb_state) -> mrb_value;
    /*
 * Initializes a new array with initial values
 *
 * Equivalent to:
 *
 *      Array[value1, value2, ...]
 *
 * @param mrb The mruby state reference.
 * @param size The numer of values.
 * @param vals The actual values.
 * @return The initialized array.
 */
    #[no_mangle]
    fn mrb_ary_new_from_values(mrb: *mut mrb_state, size: mrb_int,
                               vals: *const mrb_value) -> mrb_value;
    /*
 * Pushes value into array.
 *
 * Equivalent to:
 *
 *      ary << value
 *
 * @param mrb The mruby state reference.
 * @param ary The array in which the value will be pushed
 * @param value The value to be pushed into array
 */
    #[no_mangle]
    fn mrb_ary_push(mrb: *mut mrb_state, array: mrb_value, value: mrb_value);
    /*
 * Pops the last element from the array.
 *
 * Equivalent to:
 *
 *      ary.pop
 *
 * @param mrb The mruby state reference.
 * @param ary The array from which the value will be popped.
 * @return The popped value.
 */
    #[no_mangle]
    fn mrb_ary_pop(mrb: *mut mrb_state, ary: mrb_value) -> mrb_value;
    #[no_mangle]
    fn mrb_range_beg_len(mrb: *mut mrb_state, range: mrb_value,
                         begp: *mut mrb_int, lenp: *mut mrb_int, len: mrb_int,
                         trunc: mrb_bool) -> mrb_range_beg_len;
    /* ---------------------------------- */
    #[no_mangle]
    fn mrb_mod_to_s(mrb: *mut mrb_state, klass: mrb_value) -> mrb_value;
    #[no_mangle]
    fn mrb_fixnum_to_str(mrb: *mut mrb_state, x: mrb_value, base: mrb_int)
     -> mrb_value;
    #[no_mangle]
    fn mrb_check_string_type(mrb: *mut mrb_state, str: mrb_value)
     -> mrb_value;
}
pub type __darwin_ptrdiff_t = libc::c_long;
pub type __darwin_size_t = libc::c_ulong;
pub type int64_t = libc::c_longlong;
pub type uintptr_t = libc::c_ulong;
pub type ptrdiff_t = __darwin_ptrdiff_t;
pub type size_t = __darwin_size_t;
pub type uint8_t = libc::c_uchar;
pub type uint16_t = libc::c_ushort;
pub type uint32_t = libc::c_uint;
pub type uint64_t = libc::c_ulonglong;
/*
** mruby/value.h - mruby value definitions
**
** See Copyright Notice in mruby.h
*/
/* *
 * MRuby Value definition functions and macros.
 */
pub type mrb_sym = uint32_t;
pub type mrb_bool = uint8_t;
#[derive ( Copy , Clone )]
#[repr(C)]
pub struct mrb_state {
    pub jmp: *mut mrb_jmpbuf,
    pub allocf: mrb_allocf,
    pub allocf_ud: *mut libc::c_void,
    pub c: *mut mrb_context,
    pub root_c: *mut mrb_context,
    pub globals: *mut iv_tbl,
    pub exc: *mut RObject,
    pub top_self: *mut RObject,
    pub object_class: *mut RClass,
    pub class_class: *mut RClass,
    pub module_class: *mut RClass,
    pub proc_class: *mut RClass,
    pub string_class: *mut RClass,
    pub array_class: *mut RClass,
    pub hash_class: *mut RClass,
    pub range_class: *mut RClass,
    pub float_class: *mut RClass,
    pub fixnum_class: *mut RClass,
    pub true_class: *mut RClass,
    pub false_class: *mut RClass,
    pub nil_class: *mut RClass,
    pub symbol_class: *mut RClass,
    pub kernel_module: *mut RClass,
    pub gc: mrb_gc,
    pub symidx: mrb_sym,
    pub symtbl: *mut symbol_name,
    pub symhash: [mrb_sym; 256],
    pub symcapa: size_t,
    pub symbuf: [libc::c_char; 8],
    pub code_fetch_hook: Option<unsafe extern "C" fn(_: *mut mrb_state,
                                                     _: *mut mrb_irep,
                                                     _: *mut mrb_code,
                                                     _: *mut mrb_value)
                                    -> ()>,
    pub debug_op_hook: Option<unsafe extern "C" fn(_: *mut mrb_state,
                                                   _: *mut mrb_irep,
                                                   _: *mut mrb_code,
                                                   _: *mut mrb_value) -> ()>,
    pub eException_class: *mut RClass,
    pub eStandardError_class: *mut RClass,
    pub nomem_err: *mut RObject,
    pub stack_err: *mut RObject,
    pub ud: *mut libc::c_void,
    pub atexit_stack: *mut mrb_atexit_func,
    pub atexit_stack_len: uint16_t,
    pub ecall_nest: uint16_t,
}
pub type mrb_atexit_func
    =
    Option<unsafe extern "C" fn(_: *mut mrb_state) -> ()>;
#[derive ( BitfieldStruct , Clone , Copy )]
#[repr(C)]
pub struct RObject {
    #[bitfield(name = "tt", ty = "mrb_vtype", bits = "0..=7")]
    #[bitfield(name = "color", ty = "uint32_t", bits = "8..=10")]
    #[bitfield(name = "flags", ty = "uint32_t", bits = "11..=31")]
    pub tt_color_flags: [u8; 4],
    pub _pad: [u8; 4],
    pub c: *mut RClass,
    pub gcnext: *mut RBasic,
    pub iv: *mut iv_tbl,
}
/*
** mruby/object.h - mruby object definition
**
** See Copyright Notice in mruby.h
*/
#[derive ( BitfieldStruct , Clone , Copy )]
#[repr(C)]
pub struct RBasic {
    #[bitfield(name = "tt", ty = "mrb_vtype", bits = "0..=7")]
    #[bitfield(name = "color", ty = "uint32_t", bits = "8..=10")]
    #[bitfield(name = "flags", ty = "uint32_t", bits = "11..=31")]
    pub tt_color_flags: [u8; 4],
    pub _pad: [u8; 4],
    pub c: *mut RClass,
    pub gcnext: *mut RBasic,
}
/*
** mruby/class.h - Class class
**
** See Copyright Notice in mruby.h
*/
/* *
 * Class class
 */
#[derive ( BitfieldStruct , Clone , Copy )]
#[repr(C)]
pub struct RClass {
    #[bitfield(name = "tt", ty = "mrb_vtype", bits = "0..=7")]
    #[bitfield(name = "color", ty = "uint32_t", bits = "8..=10")]
    #[bitfield(name = "flags", ty = "uint32_t", bits = "11..=31")]
    pub tt_color_flags: [u8; 4],
    pub _pad: [u8; 4],
    pub c: *mut RClass,
    pub gcnext: *mut RBasic,
    pub iv: *mut iv_tbl,
    pub mt: *mut kh_mt,
    pub super_0: *mut RClass,
}
pub type mrb_vtype = libc::c_uint;
/*  25 */
pub const MRB_TT_MAXDEFINE: mrb_vtype = 25;
/*  24 */
pub const MRB_TT_BREAK: mrb_vtype = 24;
/*  23 */
pub const MRB_TT_ISTRUCT: mrb_vtype = 23;
/*  22 */
pub const MRB_TT_FIBER: mrb_vtype = 22;
/*  21 */
pub const MRB_TT_DATA: mrb_vtype = 21;
/*  20 */
pub const MRB_TT_ENV: mrb_vtype = 20;
/*  19 */
pub const MRB_TT_FILE: mrb_vtype = 19;
/*  18 */
pub const MRB_TT_EXCEPTION: mrb_vtype = 18;
/*  17 */
pub const MRB_TT_RANGE: mrb_vtype = 17;
/*  16 */
pub const MRB_TT_STRING: mrb_vtype = 16;
/*  15 */
pub const MRB_TT_HASH: mrb_vtype = 15;
/*  14 */
pub const MRB_TT_ARRAY: mrb_vtype = 14;
/*  13 */
pub const MRB_TT_PROC: mrb_vtype = 13;
/*  12 */
pub const MRB_TT_SCLASS: mrb_vtype = 12;
/*  11 */
pub const MRB_TT_ICLASS: mrb_vtype = 11;
/*  10 */
pub const MRB_TT_MODULE: mrb_vtype = 10;
/*   9 */
pub const MRB_TT_CLASS: mrb_vtype = 9;
/*   8 */
pub const MRB_TT_OBJECT: mrb_vtype = 8;
/*   7 */
pub const MRB_TT_CPTR: mrb_vtype = 7;
/*   6 */
pub const MRB_TT_FLOAT: mrb_vtype = 6;
/*   5 */
pub const MRB_TT_UNDEF: mrb_vtype = 5;
/*   4 */
pub const MRB_TT_SYMBOL: mrb_vtype = 4;
/*   3 */
pub const MRB_TT_FIXNUM: mrb_vtype = 3;
/*   2 */
pub const MRB_TT_TRUE: mrb_vtype = 2;
/*   1 */
pub const MRB_TT_FREE: mrb_vtype = 1;
/*   0 */
pub const MRB_TT_FALSE: mrb_vtype = 0;
/*
** mruby/boxing_no.h - unboxed mrb_value definition
**
** See Copyright Notice in mruby.h
*/
#[derive ( Copy , Clone )]
#[repr(C)]
pub struct mrb_value {
    pub value: unnamed,
    pub tt: mrb_vtype,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub union unnamed {
    pub f: mrb_float,
    pub p: *mut libc::c_void,
    pub i: mrb_int,
    pub sym: mrb_sym,
}
pub type mrb_int = int64_t;
pub type mrb_float = libc::c_double;
/*
** mruby - An embeddable Ruby implementation
**
** Copyright (c) mruby developers 2010-2019
**
** Permission is hereby granted, free of charge, to any person obtaining
** a copy of this software and associated documentation files (the
** "Software"), to deal in the Software without restriction, including
** without limitation the rights to use, copy, modify, merge, publish,
** distribute, sublicense, and/or sell copies of the Software, and to
** permit persons to whom the Software is furnished to do so, subject to
** the following conditions:
**
** The above copyright notice and this permission notice shall be
** included in all copies or substantial portions of the Software.
**
** THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND,
** EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF
** MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT.
** IN NO EVENT SHALL THE AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY
** CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT,
** TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN CONNECTION WITH THE
** SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE.
**
** [ MIT license: http://www.opensource.org/licenses/mit-license.php ]
*/
/* *
 * MRuby C API entry point
 */
pub type mrb_code = uint8_t;
#[derive ( BitfieldStruct , Clone , Copy )]
#[repr(C)]
pub struct mrb_gc {
    pub heaps: *mut mrb_heap_page,
    pub sweeps: *mut mrb_heap_page,
    pub free_heaps: *mut mrb_heap_page,
    pub live: size_t,
    pub arena: *mut *mut RBasic,
    pub arena_capa: libc::c_int,
    pub arena_idx: libc::c_int,
    pub state: mrb_gc_state,
    pub current_white_part: libc::c_int,
    pub gray_list: *mut RBasic,
    pub atomic_gray_list: *mut RBasic,
    pub live_after_mark: size_t,
    pub threshold: size_t,
    pub interval_ratio: libc::c_int,
    pub step_ratio: libc::c_int,
    #[bitfield(name = "iterating", ty = "mrb_bool", bits = "0..=0")]
    #[bitfield(name = "disabled", ty = "mrb_bool", bits = "1..=1")]
    #[bitfield(name = "full", ty = "mrb_bool", bits = "2..=2")]
    #[bitfield(name = "generational", ty = "mrb_bool", bits = "3..=3")]
    #[bitfield(name = "out_of_memory", ty = "mrb_bool", bits = "4..=4")]
    pub iterating_disabled_full_generational_out_of_memory: [u8; 1],
    pub _pad: [u8; 7],
    pub majorgc_old_threshold: size_t,
}
pub type mrb_gc_state = libc::c_uint;
pub const MRB_GC_STATE_SWEEP: mrb_gc_state = 2;
pub const MRB_GC_STATE_MARK: mrb_gc_state = 1;
pub const MRB_GC_STATE_ROOT: mrb_gc_state = 0;
/* Disable MSVC warning "C4200: nonstandard extension used: zero-sized array
 * in struct/union" when in C++ mode */
#[derive ( BitfieldStruct , Clone , Copy )]
#[repr(C)]
pub struct mrb_heap_page {
    pub freelist: *mut RBasic,
    pub prev: *mut mrb_heap_page,
    pub next: *mut mrb_heap_page,
    pub free_next: *mut mrb_heap_page,
    pub free_prev: *mut mrb_heap_page,
    #[bitfield(name = "old", ty = "mrb_bool", bits = "0..=0")]
    pub old: [u8; 1],
    pub _pad: [u8; 7],
    pub objects: [*mut libc::c_void; 0],
}
#[derive ( Copy , Clone )]
#[repr(C)]
pub struct mrb_context {
    pub prev: *mut mrb_context,
    pub stack: *mut mrb_value,
    pub stbase: *mut mrb_value,
    pub stend: *mut mrb_value,
    pub ci: *mut mrb_callinfo,
    pub cibase: *mut mrb_callinfo,
    pub ciend: *mut mrb_callinfo,
    pub rescue: *mut uint16_t,
    pub rsize: uint16_t,
    pub ensure: *mut *mut RProc,
    pub esize: uint16_t,
    pub eidx: uint16_t,
    pub status: mrb_fiber_state,
    pub vmexec: mrb_bool,
    pub fib: *mut RFiber,
}
#[derive ( BitfieldStruct , Clone , Copy )]
#[repr(C)]
pub struct RFiber {
    #[bitfield(name = "tt", ty = "mrb_vtype", bits = "0..=7")]
    #[bitfield(name = "color", ty = "uint32_t", bits = "8..=10")]
    #[bitfield(name = "flags", ty = "uint32_t", bits = "11..=31")]
    pub tt_color_flags: [u8; 4],
    pub _pad: [u8; 4],
    pub c: *mut RClass,
    pub gcnext: *mut RBasic,
    pub cxt: *mut mrb_context,
}
pub type mrb_fiber_state = libc::c_uint;
pub const MRB_FIBER_TERMINATED: mrb_fiber_state = 5;
pub const MRB_FIBER_TRANSFERRED: mrb_fiber_state = 4;
pub const MRB_FIBER_SUSPENDED: mrb_fiber_state = 3;
pub const MRB_FIBER_RESUMED: mrb_fiber_state = 2;
pub const MRB_FIBER_RUNNING: mrb_fiber_state = 1;
pub const MRB_FIBER_CREATED: mrb_fiber_state = 0;
#[derive ( Copy , Clone )]
#[repr(C)]
pub struct mrb_callinfo {
    pub mid: mrb_sym,
    pub proc_0: *mut RProc,
    pub stackent: *mut mrb_value,
    pub ridx: uint16_t,
    pub epos: uint16_t,
    pub env: *mut REnv,
    pub pc: *mut mrb_code,
    pub err: *mut mrb_code,
    pub argc: libc::c_int,
    pub acc: libc::c_int,
    pub target_class: *mut RClass,
}
/* *
 * Function pointer type of custom allocator used in @see mrb_open_allocf.
 *
 * The function pointing it must behave similarly as realloc except:
 * - If ptr is NULL it must allocate new space.
 * - If s is NULL, ptr must be freed.
 *
 * See @see mrb_default_allocf for the default implementation.
 */
pub type mrb_allocf
    =
    Option<unsafe extern "C" fn(_: *mut mrb_state, _: *mut libc::c_void,
                                _: size_t, _: *mut libc::c_void)
               -> *mut libc::c_void>;
/* *
 * Required arguments signature type.
 */
pub type mrb_aspec = uint32_t;
/* default method cache size: 128 */
/* cache size needs to be power of 2 */
pub type mrb_func_t
    =
    Option<unsafe extern "C" fn(_: *mut mrb_state, _: mrb_value)
               -> mrb_value>;
/* *
 * Function requires n arguments.
 *
 * @param n
 *      The number of required arguments.
 */
/* *
 * Function takes n optional arguments
 *
 * @param n
 *      The number of optional arguments.
 */
/* *
 * Function takes n1 mandatory arguments and n2 optional arguments
 *
 * @param n1
 *      The number of required arguments.
 * @param n2
 *      The number of optional arguments.
 */
/* * rest argument */
/* * required arguments after rest */
/* * keyword arguments (n of keys, kdict) */
/* *
 * Function takes a block argument
 */
/* *
 * Function accepts any number of arguments
 */
/* *
 * Function accepts no arguments
 */
/* *
 * Format specifiers for {mrb_get_args} function
 *
 * Must be a C string composed of the following format specifiers:
 *
 * | char | Ruby type      | C types           | Notes                                              |
 * |:----:|----------------|-------------------|----------------------------------------------------|
 * | `o`  | {Object}       | {mrb_value}       | Could be used to retrieve any type of argument     |
 * | `C`  | {Class}/{Module} | {mrb_value}     |                                                    |
 * | `S`  | {String}       | {mrb_value}       | when `!` follows, the value may be `nil`           |
 * | `A`  | {Array}        | {mrb_value}       | when `!` follows, the value may be `nil`           |
 * | `H`  | {Hash}         | {mrb_value}       | when `!` follows, the value may be `nil`           |
 * | `s`  | {String}       | char *, {mrb_int} | Receive two arguments; `s!` gives (`NULL`,`0`) for `nil`       |
 * | `z`  | {String}       | char *            | `NULL` terminated string; `z!` gives `NULL` for `nil`           |
 * | `a`  | {Array}        | {mrb_value} *, {mrb_int} | Receive two arguments; `a!` gives (`NULL`,`0`) for `nil` |
 * | `f`  | {Float}        | {mrb_float}       |                                                    |
 * | `i`  | {Integer}      | {mrb_int}         |                                                    |
 * | `b`  | boolean        | {mrb_bool}        |                                                    |
 * | `n`  | {Symbol}       | {mrb_sym}         |                                                    |
 * | `&`  | block          | {mrb_value}       | &! raises exception if no block given.             |
 * | `*`  | rest arguments | {mrb_value} *, {mrb_int} | Receive the rest of arguments as an array; *! avoid copy of the stack.  |
 * | &vert; | optional     |                   | After this spec following specs would be optional. |
 * | `?`  | optional given | {mrb_bool}        | `TRUE` if preceding argument is given. Used to check optional argument is given. |
 *
 * @see mrb_get_args
 */
pub type mrb_args_format = *const libc::c_char;
#[derive ( BitfieldStruct , Clone , Copy )]
#[repr(C)]
pub struct RString {
    #[bitfield(name = "tt", ty = "mrb_vtype", bits = "0..=7")]
    #[bitfield(name = "color", ty = "uint32_t", bits = "8..=10")]
    #[bitfield(name = "flags", ty = "uint32_t", bits = "11..=31")]
    pub tt_color_flags: [u8; 4],
    pub _pad: [u8; 4],
    pub c: *mut RClass,
    pub gcnext: *mut RBasic,
    pub as_0: unnamed_0,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub union unnamed_0 {
    pub heap: unnamed_1,
    pub ary: [libc::c_char; 24],
}
#[derive ( Copy , Clone )]
#[repr(C)]
pub struct unnamed_1 {
    pub len: mrb_int,
    pub aux: unnamed_2,
    pub ptr: *mut libc::c_char,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub union unnamed_2 {
    pub capa: mrb_int,
    pub shared: *mut mrb_shared_string,
    pub fshared: *mut RString,
}
/*
** string.c - String class
**
** See Copyright Notice in mruby.h
*/
#[derive ( BitfieldStruct , Clone , Copy )]
#[repr(C)]
pub struct mrb_shared_string {
    #[bitfield(name = "nofree", ty = "mrb_bool", bits = "0..=0")]
    pub nofree: [u8; 1],
    pub _pad: [u8; 3],
    pub refcnt: libc::c_int,
    pub ptr: *mut libc::c_char,
    pub len: mrb_int,
}
/*
** mruby/array.h - Array class
**
** See Copyright Notice in mruby.h
*/
/*
 * Array class
 */
#[derive ( Copy , Clone )]
#[repr(C)]
pub struct mrb_shared_array {
    pub refcnt: libc::c_int,
    pub len: mrb_int,
    pub ptr: *mut mrb_value,
}
#[derive ( BitfieldStruct , Clone , Copy )]
#[repr(C)]
pub struct RArray {
    #[bitfield(name = "tt", ty = "mrb_vtype", bits = "0..=7")]
    #[bitfield(name = "color", ty = "uint32_t", bits = "8..=10")]
    #[bitfield(name = "flags", ty = "uint32_t", bits = "11..=31")]
    pub tt_color_flags: [u8; 4],
    pub _pad: [u8; 4],
    pub c: *mut RClass,
    pub gcnext: *mut RBasic,
    pub as_0: unnamed_3,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub union unnamed_3 {
    pub heap: unnamed_4,
}
#[derive ( Copy , Clone )]
#[repr(C)]
pub struct unnamed_4 {
    pub len: mrb_int,
    pub aux: unnamed_5,
    pub ptr: *mut mrb_value,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub union unnamed_5 {
    pub capa: mrb_int,
    pub shared: *mut mrb_shared_array,
}
pub type mrb_range_beg_len = libc::c_uint;
/* (failure) out of range */
pub const MRB_RANGE_OUT: mrb_range_beg_len = 2;
/* (success) range */
pub const MRB_RANGE_OK: mrb_range_beg_len = 1;
/* (failure) not range */
pub const MRB_RANGE_TYPE_MISMATCH: mrb_range_beg_len = 0;
pub type unnamed_6 = libc::c_uint;
pub const max_width: unnamed_6 = 20;
pub const awk: unnamed_7 = 0;
pub type unnamed_7 = libc::c_uint;
pub const regexp: unnamed_7 = 2;
pub const string: unnamed_7 = 1;
#[no_mangle]
pub unsafe extern "C" fn mrb_float_read(mut string_0: *const libc::c_char,
                                        mut endPtr: *mut *mut libc::c_char)
 -> libc::c_double {
    let mut sign: libc::c_int = 0;
    let mut expSign: libc::c_int = 0i32;
    let mut fraction: libc::c_double = 0.;
    let mut dblExp: libc::c_double = 0.;
    let mut d: *const libc::c_double = 0 as *const libc::c_double;
    let mut p: *const libc::c_char = 0 as *const libc::c_char;
    let mut c: libc::c_int = 0;
    /* Exponent read from "EX" field. */
    let mut exp: libc::c_int = 0i32;
    /* Exponent that derives from the fractional
                                 * part.  Under normal circumstatnces, it is
                                 * the negative of the number of digits in F.
                                 * However, if I is very long, the last digits
                                 * of I get dropped (otherwise a long I with a
                                 * large negative exponent could cause an
                                 * unnecessary overflow on I alone).  In this
                                 * case, fracExp is incremented one for each
                                 * dropped digit. */
    let mut fracExp: libc::c_int = 0i32;
    /* Number of digits in mantissa. */
    let mut mantSize: libc::c_int = 0;
    /* Number of mantissa digits BEFORE decimal
                                 * point. */
    let mut decPt: libc::c_int = 0;
    /* Temporarily holds location of exponent
                                 * in string. */
    let mut pExp: *const libc::c_char = 0 as *const libc::c_char;
    p = string_0;
    while *p as libc::c_int == ' ' as i32 ||
              (*p as libc::c_uint).wrapping_sub('\t' as i32 as libc::c_uint) <
                  5i32 as libc::c_uint {
        p = p.offset(1isize)
    }
    if *p as libc::c_int == '-' as i32 {
        sign = 1i32;
        p = p.offset(1isize)
    } else {
        if *p as libc::c_int == '+' as i32 { p = p.offset(1isize) }
        sign = 0i32
    }
    decPt = -1i32;
    mantSize = 0i32;
    loop  {
        c = *p as libc::c_int;
        if !((c as libc::c_uint).wrapping_sub('0' as i32 as libc::c_uint) <
                 10i32 as libc::c_uint) {
            if c != '.' as i32 || decPt >= 0i32 { break ; }
            decPt = mantSize
        }
        p = p.offset(1isize);
        mantSize += 1i32
    }
    pExp = p;
    p = p.offset(-(mantSize as isize));
    if decPt < 0i32 { decPt = mantSize } else { mantSize -= 1i32 }
    if mantSize > 18i32 {
        if decPt - 18i32 > 29999i32 {
            fracExp = 29999i32
        } else { fracExp = decPt - 18i32 }
        mantSize = 18i32
    } else { fracExp = decPt - mantSize }
    if mantSize == 0i32 {
        fraction = 0.0f64;
        p = string_0
    } else {
        let mut frac1: libc::c_int = 0;
        let mut frac2: libc::c_int = 0;
        frac1 = 0i32;
        while mantSize > 9i32 {
            c = *p as libc::c_int;
            p = p.offset(1isize);
            if c == '.' as i32 { c = *p as libc::c_int; p = p.offset(1isize) }
            frac1 = 10i32 * frac1 + (c - '0' as i32);
            mantSize -= 1i32
        }
        frac2 = 0i32;
        while mantSize > 0i32 {
            c = *p as libc::c_int;
            p = p.offset(1isize);
            if c == '.' as i32 { c = *p as libc::c_int; p = p.offset(1isize) }
            frac2 = 10i32 * frac2 + (c - '0' as i32);
            mantSize -= 1i32
        }
        fraction =
            1.0e9f64 * frac1 as libc::c_double + frac2 as libc::c_double;
        p = pExp;
        if *p as libc::c_int == 'E' as i32 || *p as libc::c_int == 'e' as i32
           {
            p = p.offset(1isize);
            if *p as libc::c_int == '-' as i32 {
                expSign = 1i32;
                p = p.offset(1isize)
            } else {
                if *p as libc::c_int == '+' as i32 { p = p.offset(1isize) }
                expSign = 0i32
            }
            while (*p as
                       libc::c_uint).wrapping_sub('0' as i32 as libc::c_uint)
                      < 10i32 as libc::c_uint {
                exp = exp * 10i32 + (*p as libc::c_int - '0' as i32);
                if exp > 19999i32 { exp = 19999i32 }
                p = p.offset(1isize)
            }
        }
        if 0 != expSign { exp = fracExp - exp } else { exp = fracExp + exp }
        if exp < 0i32 { expSign = 1i32; exp = -exp } else { expSign = 0i32 }
        if exp > maxExponent { exp = maxExponent; *__error() = 34i32 }
        dblExp = 1.0f64;
        d = powersOf10.as_ptr();
        while exp != 0i32 {
            if 0 != exp & 0o1i32 { dblExp *= *d }
            exp >>= 1i32;
            d = d.offset(1isize)
        }
        if 0 != expSign { fraction /= dblExp } else { fraction *= dblExp }
    }
    if !endPtr.is_null() { *endPtr = p as *mut libc::c_char }
    if 0 != sign { return -fraction }
    return fraction;
}
/* Table giving binary powers of 10.  Entry */
static mut powersOf10: [libc::c_double; 9] =
    [10.0f64, 100.0f64, 1.0e4f64, 1.0e8f64, 1.0e16f64, 1.0e32f64, 1.0e64f64,
     1.0e128f64, 1.0e256f64];
/*
 * Source code for the "strtod" library procedure.
 *
 * Copyright (c) 1988-1993 The Regents of the University of California.
 * Copyright (c) 1994 Sun Microsystems, Inc.
 *
 * Permission to use, copy, modify, and distribute this
 * software and its documentation for any purpose and without
 * fee is hereby granted, provided that the above copyright
 * notice appear in all copies.  The University of California
 * makes no representations about the suitability of this
 * software for any purpose.  It is provided "as is" without
 * express or implied warranty.
 *
 * RCS: @(#) $Id: strtod.c 11708 2007-02-12 23:01:19Z shyouhei $
 */
/* Largest possible base 10 exponent.  Any
                                     * exponent larger than this will already
                                     * produce underflow or overflow, so there's
                                     * no need to worry about additional digits.
                                     */
static mut maxExponent: libc::c_int = 511i32;
/*
 * Returns a float in Ruby.
 */
#[inline]
unsafe extern "C" fn mrb_float_value(mut mrb: *mut mrb_state,
                                     mut f: mrb_float) -> mrb_value {
    let mut v: mrb_value =
        mrb_value{value: unnamed{f: 0.,}, tt: MRB_TT_FALSE,};
    v.tt = MRB_TT_FLOAT;
    v.value.f = f;
    return v;
}
/*
 * Returns a fixnum in Ruby.
 */
#[inline]
unsafe extern "C" fn mrb_fixnum_value(mut i: mrb_int) -> mrb_value {
    let mut v: mrb_value =
        mrb_value{value: unnamed{f: 0.,}, tt: MRB_TT_FALSE,};
    v.tt = MRB_TT_FIXNUM;
    v.value.i = i;
    return v;
}
#[inline]
unsafe extern "C" fn mrb_symbol_value(mut i: mrb_sym) -> mrb_value {
    let mut v: mrb_value =
        mrb_value{value: unnamed{f: 0.,}, tt: MRB_TT_FALSE,};
    v.tt = MRB_TT_SYMBOL;
    v.value.sym = i;
    return v;
}
#[inline]
unsafe extern "C" fn mrb_obj_value(mut p: *mut libc::c_void) -> mrb_value {
    let mut v: mrb_value =
        mrb_value{value: unnamed{f: 0.,}, tt: MRB_TT_FALSE,};
    v.tt = (*(p as *mut RBasic as *mut RObject)).tt();
    v.value.p = p as *mut RBasic as *mut libc::c_void;
    if 0 != !(p == v.value.p) as libc::c_int as libc::c_long {
        __assert_rtn((*::std::mem::transmute::<&[u8; 14],
                                               &[libc::c_char; 14]>(b"mrb_obj_value\x00")).as_ptr(),
                     b"/Users/lopopolo/dev/repos/ferrocarril/target/debug/build/mruby-sys-8c6c0a5ce5dfa484/out/mruby-1685c45/include/mruby/value.h\x00"
                         as *const u8 as *const libc::c_char, 226i32,
                     b"p == (v).value.p\x00" as *const u8 as
                         *const libc::c_char);
    } else { };
    if 0 !=
           !((*(p as *mut RBasic)).tt() as libc::c_uint ==
                 v.tt as libc::c_uint) as libc::c_int as libc::c_long {
        __assert_rtn((*::std::mem::transmute::<&[u8; 14],
                                               &[libc::c_char; 14]>(b"mrb_obj_value\x00")).as_ptr(),
                     b"/Users/lopopolo/dev/repos/ferrocarril/target/debug/build/mruby-sys-8c6c0a5ce5dfa484/out/mruby-1685c45/include/mruby/value.h\x00"
                         as *const u8 as *const libc::c_char, 227i32,
                     b"((struct RBasic*)p)->tt == (v).tt\x00" as *const u8 as
                         *const libc::c_char);
    } else { };
    return v;
}
/*
 * Get a nil mrb_value object.
 *
 * @return
 *      nil mrb_value object reference.
 */
#[inline]
unsafe extern "C" fn mrb_nil_value() -> mrb_value {
    let mut v: mrb_value =
        mrb_value{value: unnamed{f: 0.,}, tt: MRB_TT_FALSE,};
    v.tt = MRB_TT_FALSE;
    v.value.i = 0i32 as mrb_int;
    return v;
}
#[inline]
unsafe extern "C" fn mrb_bool_value(mut boolean: mrb_bool) -> mrb_value {
    let mut v: mrb_value =
        mrb_value{value: unnamed{f: 0.,}, tt: MRB_TT_FALSE,};
    v.tt =
        (if 0 != boolean as libc::c_int {
             MRB_TT_TRUE as libc::c_int
         } else { MRB_TT_FALSE as libc::c_int }) as mrb_vtype;
    v.value.i = 1i32 as mrb_int;
    return v;
}
#[no_mangle]
pub unsafe extern "C" fn mrb_str_new(mut mrb: *mut mrb_state,
                                     mut p: *const libc::c_char,
                                     mut len: size_t) -> mrb_value {
    return mrb_obj_value(str_new(mrb, p, len) as *mut libc::c_void);
}
unsafe extern "C" fn str_new(mut mrb: *mut mrb_state,
                             mut p: *const libc::c_char, mut len: size_t)
 -> *mut RString {
    let mut s: *mut RString = 0 as *mut RString;
    if !p.is_null() && 0 != 0i32 { return str_new_static(mrb, p, len) }
    s =
        mrb_obj_alloc(mrb, MRB_TT_STRING, (*mrb).string_class) as
            *mut RString;
    if len as libc::c_ulonglong <=
           (::std::mem::size_of::<*mut libc::c_void>() as
                libc::c_ulong).wrapping_mul(3i32 as
                                                libc::c_ulong).wrapping_sub(1i32
                                                                                as
                                                                                libc::c_ulong)
               as mrb_int as libc::c_ulonglong {
        (*s).set_flags((*s).flags() | 32i32 as uint32_t);
        let mut tmp_n: size_t = len;
        (*s).set_flags((*s).flags() & !0x7c0i32 as uint32_t);
        (*s).set_flags((*s).flags() | (tmp_n << 6i32) as uint32_t);
        if !p.is_null() {
            memcpy((*s).as_0.ary.as_mut_ptr() as *mut libc::c_void,
                   p as *const libc::c_void, len);
        }
    } else {
        if len as libc::c_ulonglong >=
               (9223372036854775807i64 >> 0i32) as libc::c_ulonglong {
            mrb_raise(mrb,
                      mrb_exc_get(mrb,
                                  b"ArgumentError\x00" as *const u8 as
                                      *const libc::c_char),
                      b"string size too big\x00" as *const u8 as
                          *const libc::c_char);
        }
        (*s).as_0.heap.ptr =
            mrb_malloc(mrb, len.wrapping_add(1i32 as libc::c_ulong)) as
                *mut libc::c_char;
        (*s).as_0.heap.len = len as mrb_int;
        (*s).as_0.heap.aux.capa = len as mrb_int;
        if !p.is_null() {
            memcpy((*s).as_0.heap.ptr as *mut libc::c_void,
                   p as *const libc::c_void, len);
        }
    }
    *if 0 != (*s).flags() as libc::c_int & 32i32 {
         (*s).as_0.ary.as_mut_ptr()
     } else { (*s).as_0.heap.ptr }.offset(len as isize) =
        '\u{0}' as i32 as libc::c_char;
    return s;
}
unsafe extern "C" fn str_new_static(mut mrb: *mut mrb_state,
                                    mut p: *const libc::c_char,
                                    mut len: size_t) -> *mut RString {
    let mut s: *mut RString = 0 as *mut RString;
    if len as libc::c_ulonglong >=
           (9223372036854775807i64 >> 0i32) as libc::c_ulonglong {
        mrb_raise(mrb,
                  mrb_exc_get(mrb,
                              b"ArgumentError\x00" as *const u8 as
                                  *const libc::c_char),
                  b"string size too big\x00" as *const u8 as
                      *const libc::c_char);
    }
    s =
        mrb_obj_alloc(mrb, MRB_TT_STRING, (*mrb).string_class) as
            *mut RString;
    (*s).as_0.heap.len = len as mrb_int;
    (*s).as_0.heap.aux.capa = 0i32 as mrb_int;
    (*s).as_0.heap.ptr = p as *mut libc::c_char;
    (*s).set_flags(4i32 as uint32_t);
    return s;
}
/* *
 * Turns a C string into a Ruby string value.
 */
#[no_mangle]
pub unsafe extern "C" fn mrb_str_new_cstr(mut mrb: *mut mrb_state,
                                          mut p: *const libc::c_char)
 -> mrb_value {
    let mut s: *mut RString = 0 as *mut RString;
    let mut len: size_t = 0;
    if !p.is_null() { len = strlen(p) } else { len = 0i32 as size_t }
    s = str_new(mrb, p, len);
    return mrb_obj_value(s as *mut libc::c_void);
}
#[no_mangle]
pub unsafe extern "C" fn mrb_str_new_static(mut mrb: *mut mrb_state,
                                            mut p: *const libc::c_char,
                                            mut len: size_t) -> mrb_value {
    let mut s: *mut RString = str_new_static(mrb, p, len);
    return mrb_obj_value(s as *mut libc::c_void);
}
#[inline]
unsafe extern "C" fn mrb_gc_arena_save(mut mrb: *mut mrb_state)
 -> libc::c_int {
    return (*mrb).gc.arena_idx;
}
#[inline]
unsafe extern "C" fn mrb_gc_arena_restore(mut mrb: *mut mrb_state,
                                          mut idx: libc::c_int) {
    (*mrb).gc.arena_idx = idx;
}
#[inline]
unsafe extern "C" fn mrb_check_frozen(mut mrb: *mut mrb_state,
                                      mut o: *mut libc::c_void) {
    if 0 != (*(o as *mut RBasic)).flags() as libc::c_int & 1i32 << 20i32 {
        mrb_frozen_error(mrb, o);
    };
}
/*
** mruby/string.h - String class
**
** See Copyright Notice in mruby.h
*/
/* *
 * String class
 */
#[no_mangle]
pub static mut mrb_digitmap: [libc::c_char; 37] =
    [48, 49, 50, 51, 52, 53, 54, 55, 56, 57, 97, 98, 99, 100, 101, 102, 103,
     104, 105, 106, 107, 108, 109, 110, 111, 112, 113, 114, 115, 116, 117,
     118, 119, 120, 121, 122, 0];
/*
 * Returns a pointer from a Ruby string
 */
#[no_mangle]
pub unsafe extern "C" fn mrb_str_strlen(mut mrb: *mut mrb_state,
                                        mut s: *mut RString) -> mrb_int {
    let mut i: mrb_int = 0;
    let mut max: mrb_int =
        if 0 != (*s).flags() as libc::c_int & 32i32 {
            (((*s).flags() as libc::c_int & 0x7c0i32) >> 6i32) as mrb_int
        } else { (*s).as_0.heap.len };
    let mut p: *mut libc::c_char =
        if 0 != (*s).flags() as libc::c_int & 32i32 {
            (*s).as_0.ary.as_mut_ptr()
        } else { (*s).as_0.heap.ptr };
    if p.is_null() { return 0i32 as mrb_int }
    i = 0i32 as mrb_int;
    while i < max {
        if *p.offset(i as isize) as libc::c_int == '\u{0}' as i32 {
            mrb_raise(mrb,
                      mrb_exc_get(mrb,
                                  b"ArgumentError\x00" as *const u8 as
                                      *const libc::c_char),
                      b"string contains null byte\x00" as *const u8 as
                          *const libc::c_char);
        }
        i += 1
    }
    return max;
}
#[no_mangle]
pub unsafe extern "C" fn mrb_gc_free_str(mut mrb: *mut mrb_state,
                                         mut str: *mut RString) {
    if !(0 != (*str).flags() as libc::c_int & 32i32) {
        if 0 != (*str).flags() as libc::c_int & 1i32 {
            str_decref(mrb, (*str).as_0.heap.aux.shared);
        } else if 0 == (*str).flags() as libc::c_int & 4i32 &&
                      0 == (*str).flags() as libc::c_int & 2i32 {
            mrb_free(mrb, (*str).as_0.heap.ptr as *mut libc::c_void);
        }
    };
}
unsafe extern "C" fn str_decref(mut mrb: *mut mrb_state,
                                mut shared: *mut mrb_shared_string) {
    (*shared).refcnt -= 1;
    if (*shared).refcnt == 0i32 {
        if 0 == (*shared).nofree() {
            mrb_free(mrb, (*shared).ptr as *mut libc::c_void);
        }
        mrb_free(mrb, shared as *mut libc::c_void);
    };
}
#[no_mangle]
pub unsafe extern "C" fn mrb_str_modify(mut mrb: *mut mrb_state,
                                        mut s: *mut RString) {
    mrb_check_frozen(mrb, s as *mut libc::c_void);
    (*s).set_flags((*s).flags() & !16i32 as uint32_t);
    if 0 != (*s).flags() as libc::c_int & 1i32 {
        let mut shared: *mut mrb_shared_string = (*s).as_0.heap.aux.shared;
        if (*shared).nofree() as libc::c_int == 0i32 &&
               (*shared).refcnt == 1i32 && (*s).as_0.heap.ptr == (*shared).ptr
           {
            (*s).as_0.heap.ptr = (*shared).ptr;
            (*s).as_0.heap.aux.capa = (*shared).len;
            *if 0 != (*s).flags() as libc::c_int & 32i32 {
                 (*s).as_0.ary.as_mut_ptr()
             } else { (*s).as_0.heap.ptr }.offset((*s).as_0.heap.len as isize)
                = '\u{0}' as i32 as libc::c_char;
            mrb_free(mrb, shared as *mut libc::c_void);
        } else {
            let mut ptr: *mut libc::c_char = 0 as *mut libc::c_char;
            let mut p: *mut libc::c_char = 0 as *mut libc::c_char;
            let mut len: mrb_int = 0;
            p =
                if 0 != (*s).flags() as libc::c_int & 32i32 {
                    (*s).as_0.ary.as_mut_ptr()
                } else { (*s).as_0.heap.ptr };
            len = (*s).as_0.heap.len;
            if len <
                   (::std::mem::size_of::<*mut libc::c_void>() as
                        libc::c_ulong).wrapping_mul(3i32 as
                                                        libc::c_ulong).wrapping_sub(1i32
                                                                                        as
                                                                                        libc::c_ulong)
                       as mrb_int {
                (*s).set_flags((*s).flags() | 32i32 as uint32_t);
                let mut tmp_n: size_t = len as size_t;
                (*s).set_flags((*s).flags() & !0x7c0i32 as uint32_t);
                (*s).set_flags((*s).flags() | (tmp_n << 6i32) as uint32_t);
                ptr =
                    if 0 != (*s).flags() as libc::c_int & 32i32 {
                        (*s).as_0.ary.as_mut_ptr()
                    } else { (*s).as_0.heap.ptr }
            } else {
                ptr =
                    mrb_malloc(mrb,
                               (len as
                                    size_t).wrapping_add(1i32 as
                                                             libc::c_ulong))
                        as *mut libc::c_char;
                (*s).as_0.heap.ptr = ptr;
                (*s).as_0.heap.aux.capa = len
            }
            if !p.is_null() {
                memcpy(ptr as *mut libc::c_void, p as *const libc::c_void,
                       len as libc::c_ulong);
            }
            *ptr.offset(len as isize) = '\u{0}' as i32 as libc::c_char;
            str_decref(mrb, shared);
        }
        (*s).set_flags((*s).flags() & !1i32 as uint32_t);
        return
    }
    if 0 != (*s).flags() as libc::c_int & 4i32 ||
           0 != (*s).flags() as libc::c_int & 2i32 {
        let mut p_0: *mut libc::c_char = (*s).as_0.heap.ptr;
        let mut len_0: mrb_int = (*s).as_0.heap.len;
        (*s).set_flags((*s).flags() & !2i32 as uint32_t);
        (*s).set_flags((*s).flags() & !4i32 as uint32_t);
        (*s).set_flags((*s).flags() & !2i32 as uint32_t);
        if len_0 <
               (::std::mem::size_of::<*mut libc::c_void>() as
                    libc::c_ulong).wrapping_mul(3i32 as
                                                    libc::c_ulong).wrapping_sub(1i32
                                                                                    as
                                                                                    libc::c_ulong)
                   as mrb_int {
            (*s).set_flags((*s).flags() | 32i32 as uint32_t);
            let mut tmp_n_0: size_t = len_0 as size_t;
            (*s).set_flags((*s).flags() & !0x7c0i32 as uint32_t);
            (*s).set_flags((*s).flags() | (tmp_n_0 << 6i32) as uint32_t)
        } else {
            (*s).as_0.heap.ptr =
                mrb_malloc(mrb,
                           (len_0 as
                                size_t).wrapping_add(1i32 as libc::c_ulong))
                    as *mut libc::c_char;
            (*s).as_0.heap.aux.capa = len_0
        }
        if !p_0.is_null() {
            memcpy((if 0 != (*s).flags() as libc::c_int & 32i32 {
                        (*s).as_0.ary.as_mut_ptr()
                    } else { (*s).as_0.heap.ptr }) as *mut libc::c_void,
                   p_0 as *const libc::c_void, len_0 as libc::c_ulong);
        }
        *if 0 != (*s).flags() as libc::c_int & 32i32 {
             (*s).as_0.ary.as_mut_ptr()
         } else { (*s).as_0.heap.ptr }.offset(len_0 as isize) =
            '\u{0}' as i32 as libc::c_char;
        return
    };
}
/*
 * Finds the index of a substring in a string
 */
#[no_mangle]
pub unsafe extern "C" fn mrb_str_index(mut mrb: *mut mrb_state,
                                       mut str: mrb_value,
                                       mut sptr: *const libc::c_char,
                                       mut slen: mrb_int, mut offset: mrb_int)
 -> mrb_int {
    let mut pos: mrb_int = 0;
    let mut s: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut len: mrb_int = 0;
    len =
        if 0 !=
               (*(str.value.p as *mut RString)).flags() as libc::c_int & 32i32
           {
            (((*(str.value.p as *mut RString)).flags() as libc::c_int &
                  0x7c0i32) >> 6i32) as mrb_int
        } else { (*(str.value.p as *mut RString)).as_0.heap.len };
    if offset < 0i32 as libc::c_longlong {
        offset += len;
        if offset < 0i32 as libc::c_longlong { return -1i32 as mrb_int }
    }
    if len - offset < slen { return -1i32 as mrb_int }
    s =
        if 0 !=
               (*(str.value.p as *mut RString)).flags() as libc::c_int & 32i32
           {
            (*(str.value.p as *mut RString)).as_0.ary.as_mut_ptr()
        } else { (*(str.value.p as *mut RString)).as_0.heap.ptr };
    if 0 != offset { s = s.offset(offset as isize) }
    if slen == 0i32 as libc::c_longlong { return offset }
    len =
        if 0 !=
               (*(str.value.p as *mut RString)).flags() as libc::c_int & 32i32
           {
            (((*(str.value.p as *mut RString)).flags() as libc::c_int &
                  0x7c0i32) >> 6i32) as mrb_int
        } else { (*(str.value.p as *mut RString)).as_0.heap.len } - offset;
    pos =
        mrb_memsearch(sptr as *const libc::c_void, slen,
                      s as *const libc::c_void, len);
    if pos < 0i32 as libc::c_longlong { return pos }
    return pos + offset;
}
unsafe extern "C" fn mrb_memsearch(mut x0: *const libc::c_void,
                                   mut m: mrb_int,
                                   mut y0: *const libc::c_void,
                                   mut n: mrb_int) -> mrb_int {
    let mut x: *const libc::c_uchar = x0 as *const libc::c_uchar;
    let mut y: *const libc::c_uchar = y0 as *const libc::c_uchar;
    if m > n {
        return -1i32 as mrb_int
    } else {
        if m == n {
            return (if memcmp(x0, y0, m as libc::c_ulong) == 0i32 {
                        0i32
                    } else { -1i32 }) as mrb_int
        } else {
            if m < 1i32 as libc::c_longlong {
                return 0i32 as mrb_int
            } else {
                if m == 1i32 as libc::c_longlong {
                    let mut ys: *const libc::c_uchar =
                        memchr(y as *const libc::c_void, *x as libc::c_int,
                               n as libc::c_ulong) as *const libc::c_uchar;
                    if !ys.is_null() {
                        return ys.wrapping_offset_from(y) as libc::c_long as
                                   mrb_int
                    } else { return -1i32 as mrb_int }
                }
            }
        }
    }
    return mrb_memsearch_qs(x0 as *const libc::c_uchar, m,
                            y0 as *const libc::c_uchar, n);
}
#[inline]
unsafe extern "C" fn mrb_memsearch_qs(mut xs: *const libc::c_uchar,
                                      mut m: mrb_int,
                                      mut ys: *const libc::c_uchar,
                                      mut n: mrb_int) -> mrb_int {
    let mut x: *const libc::c_uchar = xs;
    let mut xe: *const libc::c_uchar = xs.offset(m as isize);
    let mut y: *const libc::c_uchar = ys;
    let mut i: libc::c_int = 0;
    let mut qstable: [ptrdiff_t; 256] = [0; 256];
    i = 0i32;
    while i < 256i32 {
        qstable[i as usize] = (m + 1i32 as libc::c_longlong) as ptrdiff_t;
        i += 1
    }
    while x < xe {
        qstable[*x as usize] = xe.wrapping_offset_from(x) as libc::c_long;
        x = x.offset(1isize)
    }
    while y.offset(m as isize) <= ys.offset(n as isize) {
        if *xs as libc::c_int == *y as libc::c_int &&
               memcmp(xs as *const libc::c_void, y as *const libc::c_void,
                      m as libc::c_ulong) == 0i32 {
            return y.wrapping_offset_from(ys) as libc::c_long as mrb_int
        }
        y =
            y.offset(*qstable.as_mut_ptr().offset(*y.offset(m as isize) as
                                                      libc::c_int as isize) as
                         isize)
    }
    return -1i32 as mrb_int;
}
/*
 * Appends self to other. Returns self as a concatenated string.
 *
 *
 *  Example:
 *
 *     !!!c
 *     int
 *     main(int argc,
 *          char **argv)
 *     {
 *       // Variable declarations.
 *       mrb_value str1;
 *       mrb_value str2;
 *
 *       mrb_state *mrb = mrb_open();
 *       if (!mrb)
 *       {
 *          // handle error
 *       }
 *
 *       // Creates new Ruby strings.
 *       str1 = mrb_str_new_lit(mrb, "abc");
 *       str2 = mrb_str_new_lit(mrb, "def");
 *
 *       // Concatenates str2 to str1.
 *       mrb_str_concat(mrb, str1, str2);
 *
 *      // Prints new Concatenated Ruby string.
 *      mrb_p(mrb, str1);
 *
 *      mrb_close(mrb);
 *      return 0;
 *    }
 *
 *
 *  Result:
 *
 *     => "abcdef"
 *
 * @param [mrb_state] mrb The current mruby state.
 * @param [mrb_value] self String to concatenate.
 * @param [mrb_value] other String to append to self.
 * @return [mrb_value] Returns a new String appending other to self.
 */
#[no_mangle]
pub unsafe extern "C" fn mrb_str_concat(mut mrb: *mut mrb_state,
                                        mut self_0: mrb_value,
                                        mut other: mrb_value) {
    other = mrb_str_to_str(mrb, other);
    mrb_str_cat_str(mrb, self_0, other);
}
#[no_mangle]
pub unsafe extern "C" fn mrb_str_cat_str(mut mrb: *mut mrb_state,
                                         mut str: mrb_value,
                                         mut str2: mrb_value) -> mrb_value {
    if str.value.p as *mut RString == str2.value.p as *mut RString {
        mrb_str_modify(mrb, str.value.p as *mut RString);
    }
    return mrb_str_cat(mrb, str,
                       if 0 !=
                              (*(str2.value.p as *mut RString)).flags() as
                                  libc::c_int & 32i32 {
                           (*(str2.value.p as
                                  *mut RString)).as_0.ary.as_mut_ptr()
                       } else {
                           (*(str2.value.p as *mut RString)).as_0.heap.ptr
                       },
                       (if 0 !=
                               (*(str2.value.p as *mut RString)).flags() as
                                   libc::c_int & 32i32 {
                            (((*(str2.value.p as *mut RString)).flags() as
                                  libc::c_int & 0x7c0i32) >> 6i32) as mrb_int
                        } else {
                            (*(str2.value.p as *mut RString)).as_0.heap.len
                        }) as size_t);
}
/*
 * Returns a concated string comprised of a Ruby string and a C string.
 *
 * @param [mrb_state] mrb The current mruby state.
 * @param [mrb_value] str Ruby string.
 * @param [const char *] ptr A C string.
 * @param [size_t] len length of C string.
 * @return [mrb_value] A Ruby string.
 * @see mrb_str_cat_cstr
 */
#[no_mangle]
pub unsafe extern "C" fn mrb_str_cat(mut mrb: *mut mrb_state,
                                     mut str: mrb_value,
                                     mut ptr: *const libc::c_char,
                                     mut len: size_t) -> mrb_value {
    let mut current_block: u64;
    let mut s: *mut RString = str.value.p as *mut RString;
    let mut capa: size_t = 0;
    let mut total: size_t = 0;
    let mut off: ptrdiff_t = -1i32 as ptrdiff_t;
    if len == 0i32 as libc::c_ulong { return str }
    mrb_str_modify(mrb, s);
    if ptr >=
           (if 0 != (*s).flags() as libc::c_int & 32i32 {
                (*s).as_0.ary.as_mut_ptr()
            } else { (*s).as_0.heap.ptr }) as *const libc::c_char &&
           ptr <=
               if 0 != (*s).flags() as libc::c_int & 32i32 {
                   (*s).as_0.ary.as_mut_ptr()
               } else {
                   (*s).as_0.heap.ptr
               }.offset((if 0 != (*s).flags() as libc::c_int & 32i32 {
                             (((*s).flags() as libc::c_int & 0x7c0i32) >>
                                  6i32) as mrb_int
                         } else { (*s).as_0.heap.len }) as size_t as isize) as
                   *const libc::c_char {
        off =
            ptr.wrapping_offset_from(if 0 !=
                                            (*s).flags() as libc::c_int &
                                                32i32 {
                                         (*s).as_0.ary.as_mut_ptr()
                                     } else { (*s).as_0.heap.ptr }) as
                libc::c_long
    }
    capa =
        (if 0 != (*s).flags() as libc::c_int & 32i32 {
             (::std::mem::size_of::<*mut libc::c_void>() as
                  libc::c_ulong).wrapping_mul(3i32 as
                                                  libc::c_ulong).wrapping_sub(1i32
                                                                                  as
                                                                                  libc::c_ulong)
                 as mrb_int
         } else { (*s).as_0.heap.aux.capa }) as size_t;
    total =
        ((if 0 != (*s).flags() as libc::c_int & 32i32 {
              (((*s).flags() as libc::c_int & 0x7c0i32) >> 6i32) as mrb_int
          } else { (*s).as_0.heap.len }) as
             libc::c_ulonglong).wrapping_add(len as libc::c_ulonglong) as
            size_t;
    if !(total as libc::c_ulonglong >=
             (9223372036854775807i64 >> 0i32) as libc::c_ulonglong) {
        if capa <= total {
            if capa == 0i32 as libc::c_ulong { capa = 1i32 as size_t }
            while capa <= total {
                if capa as libc::c_ulonglong <=
                       ((9223372036854775807i64 >> 0i32) /
                            2i32 as libc::c_longlong) as libc::c_ulonglong {
                    capa =
                        (capa as
                             libc::c_ulong).wrapping_mul(2i32 as
                                                             libc::c_ulong) as
                            size_t as size_t
                } else { capa = total.wrapping_add(1i32 as libc::c_ulong) }
            }
            if capa <= total ||
                   capa as libc::c_ulonglong >
                       (9223372036854775807i64 >> 0i32) as libc::c_ulonglong {
                current_block = 10416056351061627482;
            } else {
                resize_capa(mrb, s, capa);
                current_block = 11194104282611034094;
            }
        } else { current_block = 11194104282611034094; }
        match current_block {
            10416056351061627482 => { }
            _ => {
                if off != -1i32 as libc::c_long {
                    ptr =
                        if 0 != (*s).flags() as libc::c_int & 32i32 {
                            (*s).as_0.ary.as_mut_ptr()
                        } else { (*s).as_0.heap.ptr }.offset(off as isize)
                }
                memcpy(if 0 != (*s).flags() as libc::c_int & 32i32 {
                           (*s).as_0.ary.as_mut_ptr()
                       } else {
                           (*s).as_0.heap.ptr
                       }.offset((if 0 != (*s).flags() as libc::c_int & 32i32 {
                                     (((*s).flags() as libc::c_int & 0x7c0i32)
                                          >> 6i32) as mrb_int
                                 } else { (*s).as_0.heap.len }) as isize) as
                           *mut libc::c_void, ptr as *const libc::c_void,
                       len);
                if 0 !=
                       !(total >= 0i32 as libc::c_ulong &&
                             (::std::mem::size_of::<size_t>() as libc::c_ulong
                                  <=
                                  ::std::mem::size_of::<mrb_int>() as
                                      libc::c_ulong ||
                                  total <=
                                      (9223372036854775807i64 >> 0i32) as
                                          size_t)) as libc::c_int as
                           libc::c_long {
                    __assert_rtn((*::std::mem::transmute::<&[u8; 12],
                                                           &[libc::c_char; 12]>(b"mrb_str_cat\x00")).as_ptr(),
                                 b"src/string.c\x00" as *const u8 as
                                     *const libc::c_char, 2532i32,
                                 b"(total)>=0 && ((sizeof(total)<=sizeof(mrb_int))||(total<=(size_t)((9223372036854775807LL>>0))))\x00"
                                     as *const u8 as *const libc::c_char);
                } else { };
                if 0 != (*s).flags() as libc::c_int & 32i32 {
                    let mut tmp_n: size_t = total;
                    (*s).set_flags((*s).flags() & !0x7c0i32 as uint32_t);
                    (*s).set_flags((*s).flags() | (tmp_n << 6i32) as uint32_t)
                } else { (*s).as_0.heap.len = total as mrb_int }
                *if 0 != (*s).flags() as libc::c_int & 32i32 {
                     (*s).as_0.ary.as_mut_ptr()
                 } else { (*s).as_0.heap.ptr }.offset(total as isize) =
                    '\u{0}' as i32 as libc::c_char;
                return str
            }
        }
    }
    mrb_raise(mrb,
              mrb_exc_get(mrb,
                          b"ArgumentError\x00" as *const u8 as
                              *const libc::c_char),
              b"string size too big\x00" as *const u8 as *const libc::c_char);
}
unsafe extern "C" fn resize_capa(mut mrb: *mut mrb_state, mut s: *mut RString,
                                 mut capacity: size_t) {
    if 0 !=
           !((capacity as libc::c_ulonglong) <
                 (9223372036854775807i64 >> 0i32) as libc::c_ulonglong) as
               libc::c_int as libc::c_long {
        __assert_rtn((*::std::mem::transmute::<&[u8; 12],
                                               &[libc::c_char; 12]>(b"resize_capa\x00")).as_ptr(),
                     b"src/string.c\x00" as *const u8 as *const libc::c_char,
                     135i32,
                     b"capacity < (9223372036854775807LL>>0)\x00" as *const u8
                         as *const libc::c_char);
    } else { };
    if 0 != (*s).flags() as libc::c_int & 32i32 {
        if ((::std::mem::size_of::<*mut libc::c_void>() as
                 libc::c_ulong).wrapping_mul(3i32 as
                                                 libc::c_ulong).wrapping_sub(1i32
                                                                                 as
                                                                                 libc::c_ulong)
                as mrb_int as libc::c_ulonglong) <
               capacity as libc::c_ulonglong {
            let tmp: *mut libc::c_char =
                mrb_malloc(mrb, capacity.wrapping_add(1i32 as libc::c_ulong))
                    as *mut libc::c_char;
            let len: mrb_int =
                (((*s).flags() as libc::c_int & 0x7c0i32) >> 6i32) as mrb_int;
            memcpy(tmp as *mut libc::c_void,
                   (*s).as_0.ary.as_mut_ptr() as *const libc::c_void,
                   len as libc::c_ulong);
            (*s).set_flags((*s).flags() & !(32i32 | 0x7c0i32) as uint32_t);
            (*s).as_0.heap.ptr = tmp;
            (*s).as_0.heap.len = len;
            (*s).as_0.heap.aux.capa = capacity as mrb_int
        }
    } else {
        (*s).as_0.heap.ptr =
            mrb_realloc(mrb,
                        (if 0 != (*s).flags() as libc::c_int & 32i32 {
                             (*s).as_0.ary.as_mut_ptr()
                         } else { (*s).as_0.heap.ptr }) as *mut libc::c_void,
                        capacity.wrapping_add(1i32 as libc::c_ulong)) as
                *mut libc::c_char;
        (*s).as_0.heap.aux.capa = capacity as mrb_int
    };
}
/*
 * Returns a converted string type.
 * For type checking, non converting `mrb_to_str` is recommended.
 */
#[no_mangle]
pub unsafe extern "C" fn mrb_str_to_str(mut mrb: *mut mrb_state,
                                        mut str: mrb_value) -> mrb_value {
    match str.tt as libc::c_uint {
        16 => { return str }
        3 => { return mrb_fixnum_to_str(mrb, str, 10i32 as mrb_int) }
        9 | 10 => { return mrb_mod_to_s(mrb, str) }
        _ => {
            return mrb_convert_type(mrb, str, MRB_TT_STRING,
                                    b"String\x00" as *const u8 as
                                        *const libc::c_char,
                                    b"to_s\x00" as *const u8 as
                                        *const libc::c_char)
        }
    };
}
/*
 * Adds two strings together.
 *
 *
 *  Example:
 *
 *     !!!c
 *     int
 *     main(int argc,
 *          char **argv)
 *     {
 *       // Variable declarations.
 *       mrb_value a;
 *       mrb_value b;
 *       mrb_value c;
 *
 *       mrb_state *mrb = mrb_open();
 *       if (!mrb)
 *       {
 *          // handle error
 *       }
 *
 *       // Creates two Ruby strings from the passed in C strings.
 *       a = mrb_str_new_lit(mrb, "abc");
 *       b = mrb_str_new_lit(mrb, "def");
 *
 *       // Prints both C strings.
 *       mrb_p(mrb, a);
 *       mrb_p(mrb, b);
 *
 *       // Concatenates both Ruby strings.
 *       c = mrb_str_plus(mrb, a, b);
 *
 *      // Prints new Concatenated Ruby string.
 *      mrb_p(mrb, c);
 *
 *      mrb_close(mrb);
 *      return 0;
 *    }
 *
 *
 *  Result:
 *
 *     => "abc"  # First string
 *     => "def"  # Second string
 *     => "abcdef" # First & Second concatenated.
 *
 * @param [mrb_state] mrb The current mruby state.
 * @param [mrb_value] a First string to concatenate.
 * @param [mrb_value] b Second string to concatenate.
 * @return [mrb_value] Returns a new String containing a concatenated to b.
 */
#[no_mangle]
pub unsafe extern "C" fn mrb_str_plus(mut mrb: *mut mrb_state,
                                      mut a: mrb_value, mut b: mrb_value)
 -> mrb_value {
    let mut s: *mut RString = a.value.p as *mut RString;
    let mut s2: *mut RString = b.value.p as *mut RString;
    let mut t: *mut RString = 0 as *mut RString;
    t =
        str_new(mrb, 0 as *const libc::c_char,
                (if 0 != (*s).flags() as libc::c_int & 32i32 {
                     (((*s).flags() as libc::c_int & 0x7c0i32) >> 6i32) as
                         mrb_int
                 } else { (*s).as_0.heap.len } +
                     if 0 != (*s2).flags() as libc::c_int & 32i32 {
                         (((*s2).flags() as libc::c_int & 0x7c0i32) >> 6i32)
                             as mrb_int
                     } else { (*s2).as_0.heap.len }) as size_t);
    memcpy((if 0 != (*t).flags() as libc::c_int & 32i32 {
                (*t).as_0.ary.as_mut_ptr()
            } else { (*t).as_0.heap.ptr }) as *mut libc::c_void,
           (if 0 != (*s).flags() as libc::c_int & 32i32 {
                (*s).as_0.ary.as_mut_ptr()
            } else { (*s).as_0.heap.ptr }) as *const libc::c_void,
           (if 0 != (*s).flags() as libc::c_int & 32i32 {
                (((*s).flags() as libc::c_int & 0x7c0i32) >> 6i32) as mrb_int
            } else { (*s).as_0.heap.len }) as libc::c_ulong);
    memcpy(if 0 != (*t).flags() as libc::c_int & 32i32 {
               (*t).as_0.ary.as_mut_ptr()
           } else {
               (*t).as_0.heap.ptr
           }.offset((if 0 != (*s).flags() as libc::c_int & 32i32 {
                         (((*s).flags() as libc::c_int & 0x7c0i32) >> 6i32) as
                             mrb_int
                     } else { (*s).as_0.heap.len }) as isize) as
               *mut libc::c_void,
           (if 0 != (*s2).flags() as libc::c_int & 32i32 {
                (*s2).as_0.ary.as_mut_ptr()
            } else { (*s2).as_0.heap.ptr }) as *const libc::c_void,
           (if 0 != (*s2).flags() as libc::c_int & 32i32 {
                (((*s2).flags() as libc::c_int & 0x7c0i32) >> 6i32) as mrb_int
            } else { (*s2).as_0.heap.len }) as libc::c_ulong);
    return mrb_obj_value(t as *mut libc::c_void);
}
/*
 * Converts pointer into a Ruby string.
 *
 * @param [mrb_state] mrb The current mruby state.
 * @param [void*] p The pointer to convert to Ruby string.
 * @return [mrb_value] Returns a new Ruby String.
 */
#[no_mangle]
pub unsafe extern "C" fn mrb_ptr_to_str(mut mrb: *mut mrb_state,
                                        mut p: *mut libc::c_void)
 -> mrb_value {
    let mut p_str: *mut RString = 0 as *mut RString;
    let mut p1: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut p2: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut n: uintptr_t = p as uintptr_t;
    p_str =
        str_new(mrb, 0 as *const libc::c_char,
                (2i32 as
                     libc::c_ulong).wrapping_add((::std::mem::size_of::<uintptr_t>()
                                                      as
                                                      libc::c_ulong).wrapping_mul(8i32
                                                                                      as
                                                                                      libc::c_ulong).wrapping_div(4i32
                                                                                                                      as
                                                                                                                      libc::c_ulong)));
    p1 =
        if 0 != (*p_str).flags() as libc::c_int & 32i32 {
            (*p_str).as_0.ary.as_mut_ptr()
        } else { (*p_str).as_0.heap.ptr };
    let fresh0 = p1;
    p1 = p1.offset(1);
    *fresh0 = '0' as i32 as libc::c_char;
    let fresh1 = p1;
    p1 = p1.offset(1);
    *fresh1 = 'x' as i32 as libc::c_char;
    p2 = p1;
    loop  {
        let fresh2 = p2;
        p2 = p2.offset(1);
        *fresh2 =
            mrb_digitmap[n.wrapping_rem(16i32 as libc::c_ulong) as usize];
        n =
            (n as libc::c_ulong).wrapping_div(16i32 as libc::c_ulong) as
                uintptr_t as uintptr_t;
        if !(n > 0i32 as libc::c_ulong) { break ; }
    }
    *p2 = '\u{0}' as i32 as libc::c_char;
    if 0 != (*p_str).flags() as libc::c_int & 32i32 {
        let mut tmp_n: size_t =
            p2.wrapping_offset_from(if 0 !=
                                           (*p_str).flags() as libc::c_int &
                                               32i32 {
                                        (*p_str).as_0.ary.as_mut_ptr()
                                    } else { (*p_str).as_0.heap.ptr }) as
                libc::c_long as mrb_int as size_t;
        (*p_str).set_flags((*p_str).flags() & !0x7c0i32 as uint32_t);
        (*p_str).set_flags((*p_str).flags() | (tmp_n << 6i32) as uint32_t)
    } else {
        (*p_str).as_0.heap.len =
            p2.wrapping_offset_from(if 0 !=
                                           (*p_str).flags() as libc::c_int &
                                               32i32 {
                                        (*p_str).as_0.ary.as_mut_ptr()
                                    } else { (*p_str).as_0.heap.ptr }) as
                libc::c_long as mrb_int as mrb_int
    }
    while p1 < p2 {
        let c: libc::c_char = *p1;
        let fresh3 = p1;
        p1 = p1.offset(1);
        p2 = p2.offset(-1isize);
        *fresh3 = *p2;
        *p2 = c
    }
    return mrb_obj_value(p_str as *mut libc::c_void);
}
/*
 * Returns an object as a Ruby string.
 *
 * @param [mrb_state] mrb The current mruby state.
 * @param [mrb_value] obj An object to return as a Ruby string.
 * @return [mrb_value] An object as a Ruby string.
 */
#[no_mangle]
pub unsafe extern "C" fn mrb_obj_as_string(mut mrb: *mut mrb_state,
                                           mut obj: mrb_value) -> mrb_value {
    let mut str: mrb_value =
        mrb_value{value: unnamed{f: 0.,}, tt: MRB_TT_FALSE,};
    if obj.tt as libc::c_uint == MRB_TT_STRING as libc::c_int as libc::c_uint
       {
        return obj
    }
    str =
        mrb_funcall(mrb, obj, b"to_s\x00" as *const u8 as *const libc::c_char,
                    0i32 as mrb_int);
    if !(str.tt as libc::c_uint ==
             MRB_TT_STRING as libc::c_int as libc::c_uint) {
        return mrb_any_to_s(mrb, obj)
    }
    return str;
}
/*
 * Resizes the string's length. Returns the amount of characters
 * in the specified by len.
 *
 * Example:
 *
 *     !!!c
 *     int
 *     main(int argc,
 *          char **argv)
 *     {
 *         // Variable declaration.
 *         mrb_value str;
 *
 *         mrb_state *mrb = mrb_open();
 *         if (!mrb)
 *         {
 *            // handle error
 *         }
 *         // Creates a new string.
 *         str = mrb_str_new_lit(mrb, "Hello, world!");
 *         // Returns 5 characters of
 *         mrb_str_resize(mrb, str, 5);
 *         mrb_p(mrb, str);
 *
 *         mrb_close(mrb);
 *         return 0;
 *      }
 *
 * Result:
 *
 *     => "Hello"
 *
 * @param [mrb_state] mrb The current mruby state.
 * @param [mrb_value] str The Ruby string to resize.
 * @param [mrb_value] len The length.
 * @return [mrb_value] An object as a Ruby string.
 */
#[no_mangle]
pub unsafe extern "C" fn mrb_str_resize(mut mrb: *mut mrb_state,
                                        mut str: mrb_value, mut len: mrb_int)
 -> mrb_value {
    let mut slen: mrb_int = 0;
    let mut s: *mut RString = str.value.p as *mut RString;
    if len < 0i32 as libc::c_longlong {
        mrb_raise(mrb,
                  mrb_exc_get(mrb,
                              b"ArgumentError\x00" as *const u8 as
                                  *const libc::c_char),
                  b"negative (or overflowed) string size\x00" as *const u8 as
                      *const libc::c_char);
    }
    mrb_str_modify(mrb, s);
    slen =
        if 0 != (*s).flags() as libc::c_int & 32i32 {
            (((*s).flags() as libc::c_int & 0x7c0i32) >> 6i32) as mrb_int
        } else { (*s).as_0.heap.len };
    if len != slen {
        if slen < len || slen - len > 256i32 as libc::c_longlong {
            resize_capa(mrb, s, len as size_t);
        }
        if 0 != (*s).flags() as libc::c_int & 32i32 {
            let mut tmp_n: size_t = len as size_t;
            (*s).set_flags((*s).flags() & !0x7c0i32 as uint32_t);
            (*s).set_flags((*s).flags() | (tmp_n << 6i32) as uint32_t)
        } else { (*s).as_0.heap.len = len as mrb_int }
        *if 0 != (*s).flags() as libc::c_int & 32i32 {
             (*s).as_0.ary.as_mut_ptr()
         } else { (*s).as_0.heap.ptr }.offset(len as isize) =
            '\u{0}' as i32 as libc::c_char
    }
    return str;
}
/*
 * Returns a sub string.
 *
 *  Example:
 *
 *     !!!c
 *     int
 *     main(int argc,
 *     char const **argv)
 *     {
 *       // Variable declarations.
 *       mrb_value str1;
 *       mrb_value str2;
 *
 *       mrb_state *mrb = mrb_open();
 *       if (!mrb)
 *       {
 *         // handle error
 *       }
 *       // Creates new string.
 *       str1 = mrb_str_new_lit(mrb, "Hello, world!");
 *       // Returns a sub-string within the range of 0..2
 *       str2 = mrb_str_substr(mrb, str1, 0, 2);
 *
 *       // Prints sub-string.
 *       mrb_p(mrb, str2);
 *
 *       mrb_close(mrb);
 *       return 0;
 *     }
 *
 *  Result:
 *
 *     => "He"
 *
 * @param [mrb_state] mrb The current mruby state.
 * @param [mrb_value] str Ruby string.
 * @param [mrb_int] beg The beginning point of the sub-string.
 * @param [mrb_int] len The end point of the sub-string.
 * @return [mrb_value] An object as a Ruby sub-string.
 */
#[no_mangle]
pub unsafe extern "C" fn mrb_str_substr(mut mrb: *mut mrb_state,
                                        mut str: mrb_value, mut beg: mrb_int,
                                        mut len: mrb_int) -> mrb_value {
    return str_substr(mrb, str, beg, len);
}
unsafe extern "C" fn str_substr(mut mrb: *mut mrb_state, mut str: mrb_value,
                                mut beg: mrb_int, mut len: mrb_int)
 -> mrb_value {
    let mut clen: mrb_int = utf8_strlen(str);
    if len < 0i32 as libc::c_longlong { return mrb_nil_value() }
    if clen == 0i32 as libc::c_longlong { len = 0i32 as mrb_int }
    if beg > clen { return mrb_nil_value() }
    if beg < 0i32 as libc::c_longlong {
        beg += clen;
        if beg < 0i32 as libc::c_longlong { return mrb_nil_value() }
    }
    if len > clen - beg { len = clen - beg }
    if len <= 0i32 as libc::c_longlong { len = 0i32 as mrb_int }
    return str_subseq(mrb, str, beg, len);
}
#[inline]
unsafe extern "C" fn str_subseq(mut mrb: *mut mrb_state, mut str: mrb_value,
                                mut beg: mrb_int, mut len: mrb_int)
 -> mrb_value {
    beg = chars2bytes(str, 0i32 as mrb_int, beg);
    len = chars2bytes(str, beg, len);
    return byte_subseq(mrb, str, beg, len);
}
unsafe extern "C" fn byte_subseq(mut mrb: *mut mrb_state, mut str: mrb_value,
                                 mut beg: mrb_int, mut len: mrb_int)
 -> mrb_value {
    let mut orig: *mut RString = 0 as *mut RString;
    let mut s: *mut RString = 0 as *mut RString;
    orig = str.value.p as *mut RString;
    if 0 != (*orig).flags() as libc::c_int & 32i32 ||
           if 0 != (*orig).flags() as libc::c_int & 32i32 {
               (((*orig).flags() as libc::c_int & 0x7c0i32) >> 6i32) as
                   mrb_int
           } else { (*orig).as_0.heap.len } == 0i32 as libc::c_longlong ||
           len <=
               (::std::mem::size_of::<*mut libc::c_void>() as
                    libc::c_ulong).wrapping_mul(3i32 as
                                                    libc::c_ulong).wrapping_sub(1i32
                                                                                    as
                                                                                    libc::c_ulong)
                   as mrb_int {
        s =
            str_new(mrb,
                    if 0 != (*orig).flags() as libc::c_int & 32i32 {
                        (*orig).as_0.ary.as_mut_ptr()
                    } else { (*orig).as_0.heap.ptr }.offset(beg as isize),
                    len as size_t)
    } else {
        s =
            mrb_obj_alloc(mrb, MRB_TT_STRING, (*mrb).string_class) as
                *mut RString;
        str_make_shared(mrb, orig, s);
        (*s).as_0.heap.ptr = (*s).as_0.heap.ptr.offset(beg as isize);
        (*s).as_0.heap.len = len
    }
    return mrb_obj_value(s as *mut libc::c_void);
}
unsafe extern "C" fn str_make_shared(mut mrb: *mut mrb_state,
                                     mut orig: *mut RString,
                                     mut s: *mut RString) {
    let mut shared: *mut mrb_shared_string = 0 as *mut mrb_shared_string;
    let mut len: mrb_int =
        if 0 != (*orig).flags() as libc::c_int & 32i32 {
            (((*orig).flags() as libc::c_int & 0x7c0i32) >> 6i32) as mrb_int
        } else { (*orig).as_0.heap.len };
    if 0 !=
           (0 != (*orig).flags() as libc::c_int & 32i32) as libc::c_int as
               libc::c_long {
        __assert_rtn((*::std::mem::transmute::<&[u8; 16],
                                               &[libc::c_char; 16]>(b"str_make_shared\x00")).as_ptr(),
                     b"src/string.c\x00" as *const u8 as *const libc::c_char,
                     364i32,
                     b"!((orig)->flags & 32)\x00" as *const u8 as
                         *const libc::c_char);
    } else { };
    if 0 != (*orig).flags() as libc::c_int & 1i32 {
        shared = (*orig).as_0.heap.aux.shared;
        (*shared).refcnt += 1;
        (*s).as_0.heap.ptr = (*orig).as_0.heap.ptr;
        (*s).as_0.heap.len = len;
        (*s).as_0.heap.aux.shared = shared;
        (*s).set_flags((*s).flags() | 1i32 as uint32_t);
        (*s).set_flags((*s).flags() & !(32i32 | 0x7c0i32) as uint32_t)
    } else if 0 != (*orig).flags() as libc::c_int & 2i32 {
        let mut fs: *mut RString = 0 as *mut RString;
        fs = (*orig).as_0.heap.aux.fshared;
        (*s).as_0.heap.ptr = (*orig).as_0.heap.ptr;
        (*s).as_0.heap.len = len;
        (*s).as_0.heap.aux.fshared = fs;
        (*s).set_flags((*s).flags() | 2i32 as uint32_t);
        (*s).set_flags((*s).flags() & !(32i32 | 0x7c0i32) as uint32_t)
    } else if 0 != (*orig).flags() as libc::c_int & 1i32 << 20i32 &&
                  0 == (*orig).flags() as libc::c_int & 8i32 {
        (*s).as_0.heap.ptr = (*orig).as_0.heap.ptr;
        (*s).as_0.heap.len = len;
        (*s).as_0.heap.aux.fshared = orig;
        (*s).set_flags((*s).flags() | 2i32 as uint32_t);
        (*s).set_flags((*s).flags() & !(32i32 | 0x7c0i32) as uint32_t)
    } else {
        shared =
            mrb_malloc(mrb,
                       ::std::mem::size_of::<mrb_shared_string>() as
                           libc::c_ulong) as *mut mrb_shared_string;
        (*shared).refcnt = 2i32;
        (*shared).set_nofree((0 != (*orig).flags() as libc::c_int & 4i32) as
                                 libc::c_int as mrb_bool);
        if 0 == (*shared).nofree() &&
               (*orig).as_0.heap.aux.capa > (*orig).as_0.heap.len {
            (*shared).ptr =
                mrb_realloc(mrb, (*orig).as_0.heap.ptr as *mut libc::c_void,
                            (len + 1i32 as libc::c_longlong) as size_t) as
                    *mut libc::c_char;
            (*orig).as_0.heap.ptr = (*shared).ptr
        } else { (*shared).ptr = (*orig).as_0.heap.ptr }
        (*orig).as_0.heap.aux.shared = shared;
        (*orig).set_flags((*orig).flags() | 1i32 as uint32_t);
        (*shared).len = len;
        (*s).as_0.heap.aux.shared = shared;
        (*s).as_0.heap.ptr = (*shared).ptr;
        (*s).as_0.heap.len = len;
        (*s).set_flags((*s).flags() | 1i32 as uint32_t);
        (*s).set_flags((*s).flags() & !(32i32 | 0x7c0i32) as uint32_t)
    };
}
/* map character index to byte offset index */
unsafe extern "C" fn chars2bytes(mut s: mrb_value, mut off: mrb_int,
                                 mut idx: mrb_int) -> mrb_int {
    let mut i: mrb_int = 0;
    let mut b: mrb_int = 0;
    let mut n: mrb_int = 0;
    let mut p: *const libc::c_char =
        if 0 != (*(s.value.p as *mut RString)).flags() as libc::c_int & 32i32
           {
            (*(s.value.p as *mut RString)).as_0.ary.as_mut_ptr()
        } else {
            (*(s.value.p as *mut RString)).as_0.heap.ptr
        }.offset(off as isize);
    let mut e: *const libc::c_char =
        if 0 != (*(s.value.p as *mut RString)).flags() as libc::c_int & 32i32
           {
            (*(s.value.p as *mut RString)).as_0.ary.as_mut_ptr()
        } else {
            (*(s.value.p as *mut RString)).as_0.heap.ptr
        }.offset((if 0 !=
                         (*(s.value.p as *mut RString)).flags() as libc::c_int
                             & 32i32 {
                      (((*(s.value.p as *mut RString)).flags() as libc::c_int
                            & 0x7c0i32) >> 6i32) as mrb_int
                  } else { (*(s.value.p as *mut RString)).as_0.heap.len }) as
                     isize);
    i = 0i32 as mrb_int;
    b = i;
    while p < e && i < idx {
        n = utf8len(p, e);
        b += n;
        p = p.offset(n as isize);
        i += 1
    }
    return b;
}
unsafe extern "C" fn utf8len(mut p: *const libc::c_char,
                             mut e: *const libc::c_char) -> mrb_int {
    let mut len: mrb_int = 0;
    let mut i: mrb_int = 0;
    len = utf8len_codepage[*p as libc::c_uchar as usize] as mrb_int;
    if p.offset(len as isize) > e { return 1i32 as mrb_int }
    i = 1i32 as mrb_int;
    while i < len {
        if *p.offset(i as isize) as libc::c_int & 0xc0i32 != 0x80i32 {
            return 1i32 as mrb_int
        }
        i += 1
    }
    return len;
}
static mut utf8len_codepage: [libc::c_char; 256] =
    [1i32 as libc::c_char, 1i32 as libc::c_char, 1i32 as libc::c_char,
     1i32 as libc::c_char, 1i32 as libc::c_char, 1i32 as libc::c_char,
     1i32 as libc::c_char, 1i32 as libc::c_char, 1i32 as libc::c_char,
     1i32 as libc::c_char, 1i32 as libc::c_char, 1i32 as libc::c_char,
     1i32 as libc::c_char, 1i32 as libc::c_char, 1i32 as libc::c_char,
     1i32 as libc::c_char, 1i32 as libc::c_char, 1i32 as libc::c_char,
     1i32 as libc::c_char, 1i32 as libc::c_char, 1i32 as libc::c_char,
     1i32 as libc::c_char, 1i32 as libc::c_char, 1i32 as libc::c_char,
     1i32 as libc::c_char, 1i32 as libc::c_char, 1i32 as libc::c_char,
     1i32 as libc::c_char, 1i32 as libc::c_char, 1i32 as libc::c_char,
     1i32 as libc::c_char, 1i32 as libc::c_char, 1i32 as libc::c_char,
     1i32 as libc::c_char, 1i32 as libc::c_char, 1i32 as libc::c_char,
     1i32 as libc::c_char, 1i32 as libc::c_char, 1i32 as libc::c_char,
     1i32 as libc::c_char, 1i32 as libc::c_char, 1i32 as libc::c_char,
     1i32 as libc::c_char, 1i32 as libc::c_char, 1i32 as libc::c_char,
     1i32 as libc::c_char, 1i32 as libc::c_char, 1i32 as libc::c_char,
     1i32 as libc::c_char, 1i32 as libc::c_char, 1i32 as libc::c_char,
     1i32 as libc::c_char, 1i32 as libc::c_char, 1i32 as libc::c_char,
     1i32 as libc::c_char, 1i32 as libc::c_char, 1i32 as libc::c_char,
     1i32 as libc::c_char, 1i32 as libc::c_char, 1i32 as libc::c_char,
     1i32 as libc::c_char, 1i32 as libc::c_char, 1i32 as libc::c_char,
     1i32 as libc::c_char, 1i32 as libc::c_char, 1i32 as libc::c_char,
     1i32 as libc::c_char, 1i32 as libc::c_char, 1i32 as libc::c_char,
     1i32 as libc::c_char, 1i32 as libc::c_char, 1i32 as libc::c_char,
     1i32 as libc::c_char, 1i32 as libc::c_char, 1i32 as libc::c_char,
     1i32 as libc::c_char, 1i32 as libc::c_char, 1i32 as libc::c_char,
     1i32 as libc::c_char, 1i32 as libc::c_char, 1i32 as libc::c_char,
     1i32 as libc::c_char, 1i32 as libc::c_char, 1i32 as libc::c_char,
     1i32 as libc::c_char, 1i32 as libc::c_char, 1i32 as libc::c_char,
     1i32 as libc::c_char, 1i32 as libc::c_char, 1i32 as libc::c_char,
     1i32 as libc::c_char, 1i32 as libc::c_char, 1i32 as libc::c_char,
     1i32 as libc::c_char, 1i32 as libc::c_char, 1i32 as libc::c_char,
     1i32 as libc::c_char, 1i32 as libc::c_char, 1i32 as libc::c_char,
     1i32 as libc::c_char, 1i32 as libc::c_char, 1i32 as libc::c_char,
     1i32 as libc::c_char, 1i32 as libc::c_char, 1i32 as libc::c_char,
     1i32 as libc::c_char, 1i32 as libc::c_char, 1i32 as libc::c_char,
     1i32 as libc::c_char, 1i32 as libc::c_char, 1i32 as libc::c_char,
     1i32 as libc::c_char, 1i32 as libc::c_char, 1i32 as libc::c_char,
     1i32 as libc::c_char, 1i32 as libc::c_char, 1i32 as libc::c_char,
     1i32 as libc::c_char, 1i32 as libc::c_char, 1i32 as libc::c_char,
     1i32 as libc::c_char, 1i32 as libc::c_char, 1i32 as libc::c_char,
     1i32 as libc::c_char, 1i32 as libc::c_char, 1i32 as libc::c_char,
     1i32 as libc::c_char, 1i32 as libc::c_char, 1i32 as libc::c_char,
     1i32 as libc::c_char, 1i32 as libc::c_char, 1i32 as libc::c_char,
     1i32 as libc::c_char, 1i32 as libc::c_char, 1i32 as libc::c_char,
     1i32 as libc::c_char, 1i32 as libc::c_char, 1i32 as libc::c_char,
     1i32 as libc::c_char, 1i32 as libc::c_char, 1i32 as libc::c_char,
     1i32 as libc::c_char, 1i32 as libc::c_char, 1i32 as libc::c_char,
     1i32 as libc::c_char, 1i32 as libc::c_char, 1i32 as libc::c_char,
     1i32 as libc::c_char, 1i32 as libc::c_char, 1i32 as libc::c_char,
     1i32 as libc::c_char, 1i32 as libc::c_char, 1i32 as libc::c_char,
     1i32 as libc::c_char, 1i32 as libc::c_char, 1i32 as libc::c_char,
     1i32 as libc::c_char, 1i32 as libc::c_char, 1i32 as libc::c_char,
     1i32 as libc::c_char, 1i32 as libc::c_char, 1i32 as libc::c_char,
     1i32 as libc::c_char, 1i32 as libc::c_char, 1i32 as libc::c_char,
     1i32 as libc::c_char, 1i32 as libc::c_char, 1i32 as libc::c_char,
     1i32 as libc::c_char, 1i32 as libc::c_char, 1i32 as libc::c_char,
     1i32 as libc::c_char, 1i32 as libc::c_char, 1i32 as libc::c_char,
     1i32 as libc::c_char, 1i32 as libc::c_char, 1i32 as libc::c_char,
     1i32 as libc::c_char, 1i32 as libc::c_char, 1i32 as libc::c_char,
     1i32 as libc::c_char, 1i32 as libc::c_char, 1i32 as libc::c_char,
     1i32 as libc::c_char, 1i32 as libc::c_char, 1i32 as libc::c_char,
     1i32 as libc::c_char, 1i32 as libc::c_char, 1i32 as libc::c_char,
     1i32 as libc::c_char, 1i32 as libc::c_char, 1i32 as libc::c_char,
     2i32 as libc::c_char, 2i32 as libc::c_char, 2i32 as libc::c_char,
     2i32 as libc::c_char, 2i32 as libc::c_char, 2i32 as libc::c_char,
     2i32 as libc::c_char, 2i32 as libc::c_char, 2i32 as libc::c_char,
     2i32 as libc::c_char, 2i32 as libc::c_char, 2i32 as libc::c_char,
     2i32 as libc::c_char, 2i32 as libc::c_char, 2i32 as libc::c_char,
     2i32 as libc::c_char, 2i32 as libc::c_char, 2i32 as libc::c_char,
     2i32 as libc::c_char, 2i32 as libc::c_char, 2i32 as libc::c_char,
     2i32 as libc::c_char, 2i32 as libc::c_char, 2i32 as libc::c_char,
     2i32 as libc::c_char, 2i32 as libc::c_char, 2i32 as libc::c_char,
     2i32 as libc::c_char, 2i32 as libc::c_char, 2i32 as libc::c_char,
     2i32 as libc::c_char, 2i32 as libc::c_char, 3i32 as libc::c_char,
     3i32 as libc::c_char, 3i32 as libc::c_char, 3i32 as libc::c_char,
     3i32 as libc::c_char, 3i32 as libc::c_char, 3i32 as libc::c_char,
     3i32 as libc::c_char, 3i32 as libc::c_char, 3i32 as libc::c_char,
     3i32 as libc::c_char, 3i32 as libc::c_char, 3i32 as libc::c_char,
     3i32 as libc::c_char, 3i32 as libc::c_char, 3i32 as libc::c_char,
     4i32 as libc::c_char, 4i32 as libc::c_char, 4i32 as libc::c_char,
     4i32 as libc::c_char, 4i32 as libc::c_char, 1i32 as libc::c_char,
     1i32 as libc::c_char, 1i32 as libc::c_char, 1i32 as libc::c_char,
     1i32 as libc::c_char, 1i32 as libc::c_char, 1i32 as libc::c_char,
     1i32 as libc::c_char, 1i32 as libc::c_char, 1i32 as libc::c_char,
     1i32 as libc::c_char];
unsafe extern "C" fn utf8_strlen(mut str: mrb_value) -> mrb_int {
    let mut byte_len: mrb_int =
        if 0 !=
               (*(str.value.p as *mut RString)).flags() as libc::c_int & 32i32
           {
            (((*(str.value.p as *mut RString)).flags() as libc::c_int &
                  0x7c0i32) >> 6i32) as mrb_int
        } else { (*(str.value.p as *mut RString)).as_0.heap.len };
    if 0 != (*(str.value.p as *mut RString)).flags() as libc::c_int & 16i32 {
        return byte_len
    } else {
        let mut utf8_len: mrb_int =
            mrb_utf8_len(if 0 !=
                                (*(str.value.p as *mut RString)).flags() as
                                    libc::c_int & 32i32 {
                             (*(str.value.p as
                                    *mut RString)).as_0.ary.as_mut_ptr()
                         } else {
                             (*(str.value.p as *mut RString)).as_0.heap.ptr
                         }, byte_len);
        if byte_len == utf8_len {
            let ref mut fresh4 = *(str.value.p as *mut RString);
            (*fresh4).set_flags((*fresh4).flags() | 16i32 as uint32_t)
        }
        return utf8_len
    };
}
/* For backward compatibility */
#[no_mangle]
pub unsafe extern "C" fn mrb_utf8_len(mut str: *const libc::c_char,
                                      mut byte_len: mrb_int) -> mrb_int {
    let mut total: mrb_int = 0i32 as mrb_int;
    let mut p: *const libc::c_char = str;
    let mut e: *const libc::c_char = p.offset(byte_len as isize);
    while p < e { p = p.offset(utf8len(p, e) as isize); total += 1 }
    return total;
}
#[no_mangle]
pub unsafe extern "C" fn mrb_str_new_capa(mut mrb: *mut mrb_state,
                                          mut capa: size_t) -> mrb_value {
    let mut s: *mut RString = 0 as *mut RString;
    s =
        mrb_obj_alloc(mrb, MRB_TT_STRING, (*mrb).string_class) as
            *mut RString;
    if capa as libc::c_ulonglong >=
           (9223372036854775807i64 >> 0i32) as libc::c_ulonglong {
        mrb_raise(mrb,
                  mrb_exc_get(mrb,
                              b"ArgumentError\x00" as *const u8 as
                                  *const libc::c_char),
                  b"string capacity size too big\x00" as *const u8 as
                      *const libc::c_char);
    }
    (*s).as_0.heap.len = 0i32 as mrb_int;
    (*s).as_0.heap.aux.capa = capa as mrb_int;
    (*s).as_0.heap.ptr =
        mrb_malloc(mrb, capa.wrapping_add(1i32 as libc::c_ulong)) as
            *mut libc::c_char;
    *if 0 != (*s).flags() as libc::c_int & 32i32 {
         (*s).as_0.ary.as_mut_ptr()
     } else { (*s).as_0.heap.ptr }.offset(0isize) =
        '\u{0}' as i32 as libc::c_char;
    return mrb_obj_value(s as *mut libc::c_void);
}
#[no_mangle]
pub unsafe extern "C" fn mrb_str_buf_new(mut mrb: *mut mrb_state,
                                         mut capa: size_t) -> mrb_value {
    if capa < 128i32 as libc::c_ulong { capa = 128i32 as size_t }
    return mrb_str_new_capa(mrb, capa);
}
#[no_mangle]
pub unsafe extern "C" fn mrb_string_value_cstr(mut mrb: *mut mrb_state,
                                               mut ptr: *mut mrb_value)
 -> *const libc::c_char {
    let mut ps: *mut RString = 0 as *mut RString;
    let mut p: *const libc::c_char = 0 as *const libc::c_char;
    let mut len: mrb_int = 0;
    check_null_byte(mrb, *ptr);
    ps = (*ptr).value.p as *mut RString;
    p =
        if 0 != (*ps).flags() as libc::c_int & 32i32 {
            (*ps).as_0.ary.as_mut_ptr()
        } else { (*ps).as_0.heap.ptr };
    len =
        if 0 != (*ps).flags() as libc::c_int & 32i32 {
            (((*ps).flags() as libc::c_int & 0x7c0i32) >> 6i32) as mrb_int
        } else { (*ps).as_0.heap.len };
    if *p.offset(len as isize) as libc::c_int == '\u{0}' as i32 {
        return p
    } else {
        if 0 != (*ps).flags() as libc::c_int & 1i32 << 20i32 {
            ps = str_new(mrb, p, len as size_t);
            *ptr = mrb_obj_value(ps as *mut libc::c_void)
        } else { mrb_str_modify(mrb, ps); }
        return if 0 != (*ps).flags() as libc::c_int & 32i32 {
                   (*ps).as_0.ary.as_mut_ptr()
               } else { (*ps).as_0.heap.ptr }
    };
}
unsafe extern "C" fn check_null_byte(mut mrb: *mut mrb_state,
                                     mut str: mrb_value) {
    mrb_to_str(mrb, str);
    if !memchr((if 0 !=
                       (*(str.value.p as *mut RString)).flags() as libc::c_int
                           & 32i32 {
                    (*(str.value.p as *mut RString)).as_0.ary.as_mut_ptr()
                } else { (*(str.value.p as *mut RString)).as_0.heap.ptr }) as
                   *const libc::c_void, '\u{0}' as i32,
               (if 0 !=
                       (*(str.value.p as *mut RString)).flags() as libc::c_int
                           & 32i32 {
                    (((*(str.value.p as *mut RString)).flags() as libc::c_int
                          & 0x7c0i32) >> 6i32) as mrb_int
                } else { (*(str.value.p as *mut RString)).as_0.heap.len }) as
                   libc::c_ulong).is_null() {
        mrb_raise(mrb,
                  mrb_exc_get(mrb,
                              b"ArgumentError\x00" as *const u8 as
                                  *const libc::c_char),
                  b"string contains null byte\x00" as *const u8 as
                      *const libc::c_char);
    };
}
#[no_mangle]
pub unsafe extern "C" fn mrb_string_value_ptr(mut mrb: *mut mrb_state,
                                              mut str: mrb_value)
 -> *const libc::c_char {
    str = mrb_str_to_str(mrb, str);
    return if 0 !=
                  (*(str.value.p as *mut RString)).flags() as libc::c_int &
                      32i32 {
               (*(str.value.p as *mut RString)).as_0.ary.as_mut_ptr()
           } else { (*(str.value.p as *mut RString)).as_0.heap.ptr };
}
/*
 * Returns the length of the Ruby string.
 *
 *
 * @param [mrb_state] mrb The current mruby state.
 * @param [mrb_value] str Ruby string.
 * @return [mrb_int] The length of the passed in Ruby string.
 */
#[no_mangle]
pub unsafe extern "C" fn mrb_string_value_len(mut mrb: *mut mrb_state,
                                              mut ptr: mrb_value) -> mrb_int {
    mrb_to_str(mrb, ptr);
    return if 0 !=
                  (*(ptr.value.p as *mut RString)).flags() as libc::c_int &
                      32i32 {
               (((*(ptr.value.p as *mut RString)).flags() as libc::c_int &
                     0x7c0i32) >> 6i32) as mrb_int
           } else { (*(ptr.value.p as *mut RString)).as_0.heap.len };
}
/*
 * Duplicates a string object.
 *
 *
 * @param [mrb_state] mrb The current mruby state.
 * @param [mrb_value] str Ruby string.
 * @return [mrb_value] Duplicated Ruby string.
 */
#[no_mangle]
pub unsafe extern "C" fn mrb_str_dup(mut mrb: *mut mrb_state,
                                     mut str: mrb_value) -> mrb_value {
    let mut s: *mut RString = str.value.p as *mut RString;
    let mut dup: *mut RString =
        str_new(mrb, 0 as *const libc::c_char, 0i32 as size_t);
    str_with_class(mrb, dup, str);
    return str_replace(mrb, dup, s);
}
unsafe extern "C" fn str_replace(mut mrb: *mut mrb_state,
                                 mut s1: *mut RString, mut s2: *mut RString)
 -> mrb_value {
    let mut len: mrb_int = 0;
    mrb_check_frozen(mrb, s1 as *mut libc::c_void);
    if s1 == s2 { return mrb_obj_value(s1 as *mut libc::c_void) }
    (*s1).set_flags((*s1).flags() & !16i32 as uint32_t);
    (*s1).set_flags((*s1).flags() |
                        ((*s2).flags() as libc::c_int & 16i32) as uint32_t);
    len =
        if 0 != (*s2).flags() as libc::c_int & 32i32 {
            (((*s2).flags() as libc::c_int & 0x7c0i32) >> 6i32) as mrb_int
        } else { (*s2).as_0.heap.len };
    if 0 != (*s1).flags() as libc::c_int & 1i32 {
        str_decref(mrb, (*s1).as_0.heap.aux.shared);
        (*s1).set_flags((*s1).flags() & !1i32 as uint32_t)
    } else if 0 == (*s1).flags() as libc::c_int & 32i32 &&
                  0 == (*s1).flags() as libc::c_int & 4i32 &&
                  0 == (*s1).flags() as libc::c_int & 2i32 &&
                  !(*s1).as_0.heap.ptr.is_null() {
        mrb_free(mrb, (*s1).as_0.heap.ptr as *mut libc::c_void);
    }
    (*s1).set_flags((*s1).flags() & !2i32 as uint32_t);
    (*s1).set_flags((*s1).flags() & !4i32 as uint32_t);
    if len <=
           (::std::mem::size_of::<*mut libc::c_void>() as
                libc::c_ulong).wrapping_mul(3i32 as
                                                libc::c_ulong).wrapping_sub(1i32
                                                                                as
                                                                                libc::c_ulong)
               as mrb_int {
        (*s1).set_flags((*s1).flags() & !1i32 as uint32_t);
        (*s1).set_flags((*s1).flags() & !2i32 as uint32_t);
        (*s1).set_flags((*s1).flags() | 32i32 as uint32_t);
        memcpy((*s1).as_0.ary.as_mut_ptr() as *mut libc::c_void,
               (if 0 != (*s2).flags() as libc::c_int & 32i32 {
                    (*s2).as_0.ary.as_mut_ptr()
                } else { (*s2).as_0.heap.ptr }) as *const libc::c_void,
               len as libc::c_ulong);
        let mut tmp_n: size_t = len as size_t;
        (*s1).set_flags((*s1).flags() & !0x7c0i32 as uint32_t);
        (*s1).set_flags((*s1).flags() | (tmp_n << 6i32) as uint32_t)
    } else { str_make_shared(mrb, s2, s1); }
    return mrb_obj_value(s1 as *mut libc::c_void);
}
#[inline]
unsafe extern "C" fn str_with_class(mut mrb: *mut mrb_state,
                                    mut s: *mut RString, mut obj: mrb_value) {
    (*s).c = (*(obj.value.p as *mut RString)).c;
}
/*
 * Returns a symbol from a passed in Ruby string.
 *
 * @param [mrb_state] mrb The current mruby state.
 * @param [mrb_value] self Ruby string.
 * @return [mrb_value] A symbol.
 */
#[no_mangle]
pub unsafe extern "C" fn mrb_str_intern(mut mrb: *mut mrb_state,
                                        mut self_0: mrb_value) -> mrb_value {
    return mrb_symbol_value(mrb_intern_str(mrb, self_0));
}
#[no_mangle]
pub unsafe extern "C" fn mrb_str_to_inum(mut mrb: *mut mrb_state,
                                         mut str: mrb_value,
                                         mut base: mrb_int,
                                         mut badcheck: mrb_bool)
 -> mrb_value {
    let mut s: *const libc::c_char = 0 as *const libc::c_char;
    let mut len: mrb_int = 0;
    s = mrb_string_value_ptr(mrb, str);
    len =
        if 0 !=
               (*(str.value.p as *mut RString)).flags() as libc::c_int & 32i32
           {
            (((*(str.value.p as *mut RString)).flags() as libc::c_int &
                  0x7c0i32) >> 6i32) as mrb_int
        } else { (*(str.value.p as *mut RString)).as_0.heap.len };
    return mrb_str_len_to_inum(mrb, s, len, base, badcheck as libc::c_int);
}
unsafe extern "C" fn mrb_str_len_to_inum(mut mrb: *mut mrb_state,
                                         mut str: *const libc::c_char,
                                         mut len: mrb_int, mut base: mrb_int,
                                         mut badcheck: libc::c_int)
 -> mrb_value {
    let mut current_block: u64;
    let mut p: *const libc::c_char = str;
    let mut pend: *const libc::c_char = str.offset(len as isize);
    let mut sign: libc::c_char = 1i32 as libc::c_char;
    let mut c: libc::c_int = 0;
    let mut n: uint64_t = 0i32 as uint64_t;
    let mut val: mrb_int = 0;
    if p.is_null() {
        if !(0 != badcheck) { return mrb_fixnum_value(0i32 as mrb_int) }
    } else {
        while p < pend &&
                  (*p as libc::c_int == ' ' as i32 ||
                       (*p as
                            libc::c_uint).wrapping_sub('\t' as i32 as
                                                           libc::c_uint) <
                           5i32 as libc::c_uint) {
            p = p.offset(1isize)
        }
        if *p.offset(0isize) as libc::c_int == '+' as i32 {
            p = p.offset(1isize)
        } else if *p.offset(0isize) as libc::c_int == '-' as i32 {
            p = p.offset(1isize);
            sign = 0i32 as libc::c_char
        }
        if base <= 0i32 as libc::c_longlong {
            if *p.offset(0isize) as libc::c_int == '0' as i32 {
                match *p.offset(1isize) as libc::c_int {
                    120 | 88 => { base = 16i32 as mrb_int }
                    98 | 66 => { base = 2i32 as mrb_int }
                    111 | 79 => { base = 8i32 as mrb_int }
                    100 | 68 => { base = 10i32 as mrb_int }
                    _ => { base = 8i32 as mrb_int }
                }
            } else if base < -1i32 as libc::c_longlong {
                base = -base
            } else { base = 10i32 as mrb_int }
        }
        match base {
            2 => {
                if *p.offset(0isize) as libc::c_int == '0' as i32 &&
                       (*p.offset(1isize) as libc::c_int == 'b' as i32 ||
                            *p.offset(1isize) as libc::c_int == 'B' as i32) {
                    p = p.offset(2isize)
                }
            }
            8 => {
                if *p.offset(0isize) as libc::c_int == '0' as i32 &&
                       (*p.offset(1isize) as libc::c_int == 'o' as i32 ||
                            *p.offset(1isize) as libc::c_int == 'O' as i32) {
                    p = p.offset(2isize)
                }
            }
            10 => {
                if *p.offset(0isize) as libc::c_int == '0' as i32 &&
                       (*p.offset(1isize) as libc::c_int == 'd' as i32 ||
                            *p.offset(1isize) as libc::c_int == 'D' as i32) {
                    p = p.offset(2isize)
                }
            }
            3 | 4 | 5 | 6 | 7 | 9 | 11 | 12 | 13 | 14 | 15 => { }
            16 => {
                if *p.offset(0isize) as libc::c_int == '0' as i32 &&
                       (*p.offset(1isize) as libc::c_int == 'x' as i32 ||
                            *p.offset(1isize) as libc::c_int == 'X' as i32) {
                    p = p.offset(2isize)
                }
            }
            _ => {
                if base < 2i32 as libc::c_longlong ||
                       (36i32 as libc::c_longlong) < base {
                    mrb_raisef(mrb,
                               mrb_exc_get(mrb,
                                           b"ArgumentError\x00" as *const u8
                                               as *const libc::c_char),
                               b"illegal radix %S\x00" as *const u8 as
                                   *const libc::c_char,
                               mrb_fixnum_value(base));
                }
            }
        }
        /* end of switch (base) { */
        if p >= pend {
            if !(0 != badcheck) { return mrb_fixnum_value(0i32 as mrb_int) }
        } else {
            if *p as libc::c_int == '0' as i32 {
                /* squeeze preceding 0s */
                p = p.offset(1isize);
                loop  {
                    if !(p < pend) {
                        current_block = 5684854171168229155;
                        break ;
                    }
                    let fresh5 = p;
                    p = p.offset(1);
                    c = *fresh5 as libc::c_int;
                    if c == '_' as i32 {
                        if !(p < pend && *p as libc::c_int == '_' as i32) {
                            continue ;
                        }
                        if 0 != badcheck {
                            current_block = 9377192742058030363;
                            break ;
                        } else {
                            current_block = 5684854171168229155;
                            break ;
                        }
                    } else {
                        if !(c != '0' as i32) { continue ; }
                        p = p.offset(-1isize);
                        current_block = 5684854171168229155;
                        break ;
                    }
                }
                match current_block {
                    9377192742058030363 => { }
                    _ => {
                        if *p.offset(-1isize) as libc::c_int == '0' as i32 {
                            p = p.offset(-1isize)
                        }
                        current_block = 3229571381435211107;
                    }
                }
            } else { current_block = 3229571381435211107; }
            match current_block {
                9377192742058030363 => { }
                _ => {
                    if p == pend {
                        if !(0 != badcheck) {
                            return mrb_fixnum_value(0i32 as mrb_int)
                        }
                    } else {
                        loop  {
                            if !(p < pend) {
                                current_block = 1428307939028130064;
                                break ;
                            }
                            if *p as libc::c_int == '_' as i32 {
                                p = p.offset(1isize);
                                if p == pend {
                                    if 0 != badcheck {
                                        current_block = 9377192742058030363;
                                        break ;
                                    }
                                    current_block = 479107131381816815;
                                } else if *p as libc::c_int == '_' as i32 {
                                    if 0 != badcheck {
                                        current_block = 9377192742058030363;
                                        break ;
                                    } else {
                                        current_block = 1428307939028130064;
                                        break ;
                                    }
                                } else {
                                    current_block = 10393716428851982524;
                                }
                            } else { current_block = 10393716428851982524; }
                            match current_block {
                                10393716428851982524 => {
                                    if 0 != badcheck &&
                                           *p as libc::c_int == '\u{0}' as i32
                                       {
                                        current_block = 5600110514324209808;
                                        break ;
                                    }
                                    c =
                                        if (*p as
                                                libc::c_uint).wrapping_sub('0'
                                                                               as
                                                                               i32
                                                                               as
                                                                               libc::c_uint)
                                               < 10i32 as libc::c_uint {
                                            *p as libc::c_int - '0' as i32
                                        } else if (*p as
                                                       libc::c_uint).wrapping_sub('a'
                                                                                      as
                                                                                      i32
                                                                                      as
                                                                                      libc::c_uint)
                                                      < 26i32 as libc::c_uint
                                         {
                                            *p as libc::c_int - 'a' as i32 +
                                                10i32
                                        } else if (*p as
                                                       libc::c_uint).wrapping_sub('A'
                                                                                      as
                                                                                      i32
                                                                                      as
                                                                                      libc::c_uint)
                                                      < 26i32 as libc::c_uint
                                         {
                                            *p as libc::c_int - 'A' as i32 +
                                                10i32
                                        } else { -1i32 };
                                    if c < 0i32 ||
                                           c as libc::c_longlong >= base {
                                        current_block = 1428307939028130064;
                                        break ;
                                    }
                                    n =
                                        (n as
                                             libc::c_ulonglong).wrapping_mul(base
                                                                                 as
                                                                                 libc::c_ulonglong)
                                            as uint64_t as uint64_t;
                                    n =
                                        (n as
                                             libc::c_ulonglong).wrapping_add(c
                                                                                 as
                                                                                 libc::c_ulonglong)
                                            as uint64_t as uint64_t;
                                    if n >
                                           ((9223372036854775807i64 >> 0i32)
                                                as
                                                uint64_t).wrapping_add((if 0
                                                                               !=
                                                                               sign
                                                                                   as
                                                                                   libc::c_int
                                                                           {
                                                                            0i32
                                                                        } else {
                                                                            1i32
                                                                        }) as
                                                                           libc::c_ulonglong)
                                       {
                                        if base == 10i32 as libc::c_longlong {
                                            return mrb_float_value(mrb,
                                                                   mrb_str_to_dbl(mrb,
                                                                                  mrb_str_new(mrb,
                                                                                              str,
                                                                                              len
                                                                                                  as
                                                                                                  size_t),
                                                                                  badcheck
                                                                                      as
                                                                                      mrb_bool))
                                        } else {
                                            mrb_raisef(mrb,
                                                       mrb_exc_get(mrb,
                                                                   b"RangeError\x00"
                                                                       as
                                                                       *const u8
                                                                       as
                                                                       *const libc::c_char),
                                                       b"string (%S) too big for integer\x00"
                                                           as *const u8 as
                                                           *const libc::c_char,
                                                       mrb_str_new(mrb, str,
                                                                   pend.wrapping_offset_from(str)
                                                                       as
                                                                       libc::c_long
                                                                       as
                                                                       size_t));
                                        }
                                    }
                                }
                                _ => { }
                            }
                            p = p.offset(1isize)
                        }
                        match current_block {
                            9377192742058030363 => { }
                            _ => {
                                match current_block {
                                    5600110514324209808 => {
                                        mrb_raise(mrb,
                                                  mrb_exc_get(mrb,
                                                              b"ArgumentError\x00"
                                                                  as *const u8
                                                                  as
                                                                  *const libc::c_char),
                                                  b"string contains null byte\x00"
                                                      as *const u8 as
                                                      *const libc::c_char);
                                    }
                                    _ => {
                                        val = n as mrb_int;
                                        if 0 != badcheck {
                                            if p == str {
                                                /* no number */
                                                current_block =
                                                    9377192742058030363;
                                            } else {
                                                while p < pend &&
                                                          (*p as libc::c_int
                                                               == ' ' as i32
                                                               ||
                                                               (*p as
                                                                    libc::c_uint).wrapping_sub('\t'
                                                                                                   as
                                                                                                   i32
                                                                                                   as
                                                                                                   libc::c_uint)
                                                                   <
                                                                   5i32 as
                                                                       libc::c_uint)
                                                      {
                                                    p = p.offset(1isize)
                                                }
                                                if p < pend {
                                                    /* trailing garbage */
                                                    current_block =
                                                        9377192742058030363;
                                                } else {
                                                    current_block =
                                                        14851765859726653900;
                                                }
                                            }
                                        } else {
                                            current_block =
                                                14851765859726653900;
                                        }
                                        match current_block {
                                            9377192742058030363 => { }
                                            _ => {
                                                return mrb_fixnum_value(if 0
                                                                               !=
                                                                               sign
                                                                                   as
                                                                                   libc::c_int
                                                                           {
                                                                            val
                                                                        } else {
                                                                            -val
                                                                        })
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
    mrb_raisef(mrb,
               mrb_exc_get(mrb,
                           b"ArgumentError\x00" as *const u8 as
                               *const libc::c_char),
               b"invalid string for number(%S)\x00" as *const u8 as
                   *const libc::c_char,
               mrb_inspect(mrb,
                           mrb_str_new(mrb, str,
                                       pend.wrapping_offset_from(str) as
                                           libc::c_long as size_t)));
}
#[no_mangle]
pub unsafe extern "C" fn mrb_str_to_dbl(mut mrb: *mut mrb_state,
                                        mut str: mrb_value,
                                        mut badcheck: mrb_bool)
 -> libc::c_double {
    return mrb_cstr_to_dbl(mrb, mrb_string_value_cstr(mrb, &mut str),
                           badcheck);
}
#[no_mangle]
pub unsafe extern "C" fn mrb_cstr_to_dbl(mut mrb: *mut mrb_state,
                                         mut p: *const libc::c_char,
                                         mut badcheck: mrb_bool)
 -> libc::c_double {
    let mut current_block: u64;
    let mut end: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut buf: [libc::c_char; 70] = [0; 70];
    let mut d: libc::c_double = 0.;
    if p.is_null() { return 0.0f64 }
    while *p as libc::c_int == ' ' as i32 ||
              (*p as libc::c_uint).wrapping_sub('\t' as i32 as libc::c_uint) <
                  5i32 as libc::c_uint {
        p = p.offset(1isize)
    }
    if 0 == badcheck && *p.offset(0isize) as libc::c_int == '0' as i32 &&
           (*p.offset(1isize) as libc::c_int == 'x' as i32 ||
                *p.offset(1isize) as libc::c_int == 'X' as i32) {
        return 0.0f64
    }
    d = mrb_float_read(p, &mut end);
    if p == end as *const libc::c_char {
        if !(0 != badcheck) { return d }
    } else {
        if 0 != *end {
            let mut n: *mut libc::c_char = buf.as_mut_ptr();
            let mut e: *mut libc::c_char =
                buf.as_mut_ptr().offset(::std::mem::size_of::<[libc::c_char; 70]>()
                                            as libc::c_ulong as
                                            isize).offset(-1isize);
            let mut prev: libc::c_char = 0i32 as libc::c_char;
            while p < end as *const libc::c_char && n < e {
                let fresh7 = n;
                n = n.offset(1);
                let fresh6 = p;
                p = p.offset(1);
                *fresh7 = *fresh6;
                prev = *fresh7
            }
            loop  {
                if !(0 != *p) {
                    current_block = 15897653523371991391;
                    break ;
                }
                if *p as libc::c_int == '_' as i32 {
                    /* remove underscores between digits */
                    if 0 != badcheck {
                        if n == buf.as_mut_ptr() ||
                               !((prev as
                                      libc::c_uint).wrapping_sub('0' as i32 as
                                                                     libc::c_uint)
                                     < 10i32 as libc::c_uint) {
                            current_block = 5845434537436470380;
                            break ;
                        }
                        p = p.offset(1isize);
                        if !((*p as
                                  libc::c_uint).wrapping_sub('0' as i32 as
                                                                 libc::c_uint)
                                 < 10i32 as libc::c_uint) {
                            current_block = 5845434537436470380;
                            break ;
                        }
                    } else {
                        loop  {
                            p = p.offset(1isize);
                            if !(*p as libc::c_int == '_' as i32) { break ; }
                        }
                        continue ;
                    }
                }
                let fresh8 = p;
                p = p.offset(1);
                prev = *fresh8;
                if n < e { let fresh9 = n; n = n.offset(1); *fresh9 = prev }
            }
            match current_block {
                5845434537436470380 => { }
                _ => {
                    *n = '\u{0}' as i32 as libc::c_char;
                    p = buf.as_mut_ptr();
                    if 0 == badcheck &&
                           *p.offset(0isize) as libc::c_int == '0' as i32 &&
                           (*p.offset(1isize) as libc::c_int == 'x' as i32 ||
                                *p.offset(1isize) as libc::c_int ==
                                    'X' as i32) {
                        return 0.0f64
                    }
                    d = mrb_float_read(p, &mut end);
                    if 0 != badcheck {
                        if end.is_null() || p == end as *const libc::c_char {
                            current_block = 5845434537436470380;
                        } else {
                            while 0 != *end as libc::c_int &&
                                      (*end as libc::c_int == ' ' as i32 ||
                                           (*end as
                                                libc::c_uint).wrapping_sub('\t'
                                                                               as
                                                                               i32
                                                                               as
                                                                               libc::c_uint)
                                               < 5i32 as libc::c_uint) {
                                end = end.offset(1isize)
                            }
                            if 0 != *end {
                                current_block = 5845434537436470380;
                            } else { current_block = 2543120759711851213; }
                        }
                    } else { current_block = 2543120759711851213; }
                }
            }
        } else { current_block = 2543120759711851213; }
        match current_block { 5845434537436470380 => { } _ => { return d } }
    }
    mrb_raisef(mrb,
               mrb_exc_get(mrb,
                           b"ArgumentError\x00" as *const u8 as
                               *const libc::c_char),
               b"invalid string for float(%S)\x00" as *const u8 as
                   *const libc::c_char, mrb_str_new_cstr(mrb, p));
}
#[no_mangle]
pub unsafe extern "C" fn mrb_cstr_to_inum(mut mrb: *mut mrb_state,
                                          mut str: *const libc::c_char,
                                          mut base: mrb_int,
                                          mut badcheck: mrb_bool)
 -> mrb_value {
    return mrb_str_len_to_inum(mrb, str, strlen(str) as mrb_int, base,
                               badcheck as libc::c_int);
}
/*
 * Returns true if the strings match and false if the strings don't match.
 *
 * @param [mrb_state] mrb The current mruby state.
 * @param [mrb_value] str1 Ruby string to compare.
 * @param [mrb_value] str2 Ruby string to compare.
 * @return [mrb_value] boolean value.
 */
#[no_mangle]
pub unsafe extern "C" fn mrb_str_equal(mut mrb: *mut mrb_state,
                                       mut str1: mrb_value,
                                       mut str2: mrb_value) -> mrb_bool {
    if !(str2.tt as libc::c_uint ==
             MRB_TT_STRING as libc::c_int as libc::c_uint) {
        return 0i32 as mrb_bool
    }
    return str_eql(mrb, str1, str2);
}
unsafe extern "C" fn str_eql(mut mrb: *mut mrb_state, str1: mrb_value,
                             str2: mrb_value) -> mrb_bool {
    let len: mrb_int =
        if 0 !=
               (*(str1.value.p as *mut RString)).flags() as libc::c_int &
                   32i32 {
            (((*(str1.value.p as *mut RString)).flags() as libc::c_int &
                  0x7c0i32) >> 6i32) as mrb_int
        } else { (*(str1.value.p as *mut RString)).as_0.heap.len };
    if len !=
           if 0 !=
                  (*(str2.value.p as *mut RString)).flags() as libc::c_int &
                      32i32 {
               (((*(str2.value.p as *mut RString)).flags() as libc::c_int &
                     0x7c0i32) >> 6i32) as mrb_int
           } else { (*(str2.value.p as *mut RString)).as_0.heap.len } {
        return 0i32 as mrb_bool
    }
    if memcmp((if 0 !=
                      (*(str1.value.p as *mut RString)).flags() as libc::c_int
                          & 32i32 {
                   (*(str1.value.p as *mut RString)).as_0.ary.as_mut_ptr()
               } else { (*(str1.value.p as *mut RString)).as_0.heap.ptr }) as
                  *const libc::c_void,
              (if 0 !=
                      (*(str2.value.p as *mut RString)).flags() as libc::c_int
                          & 32i32 {
                   (*(str2.value.p as *mut RString)).as_0.ary.as_mut_ptr()
               } else { (*(str2.value.p as *mut RString)).as_0.heap.ptr }) as
                  *const libc::c_void, len as size_t) == 0i32 {
        return 1i32 as mrb_bool
    }
    return 0i32 as mrb_bool;
}
/*
 * Returns a concated string comprised of a Ruby string and a C string.
 *
 * @param [mrb_state] mrb The current mruby state.
 * @param [mrb_value] str Ruby string.
 * @param [const char *] ptr A C string.
 * @return [mrb_value] A Ruby string.
 * @see mrb_str_cat
 */
#[no_mangle]
pub unsafe extern "C" fn mrb_str_cat_cstr(mut mrb: *mut mrb_state,
                                          mut str: mrb_value,
                                          mut ptr: *const libc::c_char)
 -> mrb_value {
    return mrb_str_cat(mrb, str, ptr, strlen(ptr));
}
/*
 * Adds str2 to the end of str1.
 */
#[no_mangle]
pub unsafe extern "C" fn mrb_str_append(mut mrb: *mut mrb_state,
                                        mut str1: mrb_value,
                                        mut str2: mrb_value) -> mrb_value {
    mrb_to_str(mrb, str2);
    return mrb_str_cat_str(mrb, str1, str2);
}
/*
 * Returns 0 if both Ruby strings are equal. Returns a value < 0 if Ruby str1 is less than Ruby str2. Returns a value > 0 if Ruby str2 is greater than Ruby str1.
 */
#[no_mangle]
pub unsafe extern "C" fn mrb_str_cmp(mut mrb: *mut mrb_state,
                                     mut str1: mrb_value, mut str2: mrb_value)
 -> libc::c_int {
    let mut len: mrb_int = 0;
    let mut retval: mrb_int = 0;
    let mut s1: *mut RString = str1.value.p as *mut RString;
    let mut s2: *mut RString = str2.value.p as *mut RString;
    len =
        if if 0 != (*s1).flags() as libc::c_int & 32i32 {
               (((*s1).flags() as libc::c_int & 0x7c0i32) >> 6i32) as mrb_int
           } else { (*s1).as_0.heap.len } >
               if 0 != (*s2).flags() as libc::c_int & 32i32 {
                   (((*s2).flags() as libc::c_int & 0x7c0i32) >> 6i32) as
                       mrb_int
               } else { (*s2).as_0.heap.len } {
            if 0 != (*s2).flags() as libc::c_int & 32i32 {
                (((*s2).flags() as libc::c_int & 0x7c0i32) >> 6i32) as mrb_int
            } else { (*s2).as_0.heap.len }
        } else if 0 != (*s1).flags() as libc::c_int & 32i32 {
            (((*s1).flags() as libc::c_int & 0x7c0i32) >> 6i32) as mrb_int
        } else { (*s1).as_0.heap.len };
    retval =
        memcmp((if 0 != (*s1).flags() as libc::c_int & 32i32 {
                    (*s1).as_0.ary.as_mut_ptr()
                } else { (*s1).as_0.heap.ptr }) as *const libc::c_void,
               (if 0 != (*s2).flags() as libc::c_int & 32i32 {
                    (*s2).as_0.ary.as_mut_ptr()
                } else { (*s2).as_0.heap.ptr }) as *const libc::c_void,
               len as libc::c_ulong) as mrb_int;
    if retval == 0i32 as libc::c_longlong {
        if if 0 != (*s1).flags() as libc::c_int & 32i32 {
               (((*s1).flags() as libc::c_int & 0x7c0i32) >> 6i32) as mrb_int
           } else { (*s1).as_0.heap.len } ==
               if 0 != (*s2).flags() as libc::c_int & 32i32 {
                   (((*s2).flags() as libc::c_int & 0x7c0i32) >> 6i32) as
                       mrb_int
               } else { (*s2).as_0.heap.len } {
            return 0i32
        }
        if if 0 != (*s1).flags() as libc::c_int & 32i32 {
               (((*s1).flags() as libc::c_int & 0x7c0i32) >> 6i32) as mrb_int
           } else { (*s1).as_0.heap.len } >
               if 0 != (*s2).flags() as libc::c_int & 32i32 {
                   (((*s2).flags() as libc::c_int & 0x7c0i32) >> 6i32) as
                       mrb_int
               } else { (*s2).as_0.heap.len } {
            return 1i32
        }
        return -1i32
    }
    if retval > 0i32 as libc::c_longlong { return 1i32 }
    return -1i32;
}
/*
 * Returns a newly allocated C string from a Ruby string.
 * This is an utility function to pass a Ruby string to C library functions.
 *
 * - Returned string does not contain any NUL characters (but terminator).
 * - It raises an ArgumentError exception if Ruby string contains
 *   NUL characters.
 * - Retured string will be freed automatically on next GC.
 * - Caller can modify returned string without affecting Ruby string
 *   (e.g. it can be used for mkstemp(3)).
 *
 * @param [mrb_state *] mrb The current mruby state.
 * @param [mrb_value] str Ruby string. Must be an instance of String.
 * @return [char *] A newly allocated C string.
 */
#[no_mangle]
pub unsafe extern "C" fn mrb_str_to_cstr(mut mrb: *mut mrb_state,
                                         mut str0: mrb_value)
 -> *mut libc::c_char {
    let mut s: *mut RString = 0 as *mut RString;
    check_null_byte(mrb, str0);
    s =
        str_new(mrb,
                if 0 !=
                       (*(str0.value.p as *mut RString)).flags() as
                           libc::c_int & 32i32 {
                    (*(str0.value.p as *mut RString)).as_0.ary.as_mut_ptr()
                } else { (*(str0.value.p as *mut RString)).as_0.heap.ptr },
                (if 0 !=
                        (*(str0.value.p as *mut RString)).flags() as
                            libc::c_int & 32i32 {
                     (((*(str0.value.p as *mut RString)).flags() as
                           libc::c_int & 0x7c0i32) >> 6i32) as mrb_int
                 } else { (*(str0.value.p as *mut RString)).as_0.heap.len })
                    as size_t);
    return if 0 != (*s).flags() as libc::c_int & 32i32 {
               (*s).as_0.ary.as_mut_ptr()
           } else { (*s).as_0.heap.ptr };
}
#[no_mangle]
pub unsafe extern "C" fn mrb_str_hash(mut mrb: *mut mrb_state,
                                      mut str: mrb_value) -> uint32_t {
    /* 1-8-7 */
    let mut s: *mut RString = str.value.p as *mut RString;
    let mut len: mrb_int =
        if 0 != (*s).flags() as libc::c_int & 32i32 {
            (((*s).flags() as libc::c_int & 0x7c0i32) >> 6i32) as mrb_int
        } else { (*s).as_0.heap.len };
    let mut p: *mut libc::c_char =
        if 0 != (*s).flags() as libc::c_int & 32i32 {
            (*s).as_0.ary.as_mut_ptr()
        } else { (*s).as_0.heap.ptr };
    let mut key: uint64_t = 0i32 as uint64_t;
    loop  {
        let fresh10 = len;
        len = len - 1;
        if !(0 != fresh10) { break ; }
        key =
            key.wrapping_mul(65599i32 as
                                 libc::c_ulonglong).wrapping_add(*p as
                                                                     libc::c_ulonglong);
        p = p.offset(1isize)
    }
    return key.wrapping_add(key >> 5i32) as uint32_t;
}
#[no_mangle]
pub unsafe extern "C" fn mrb_str_dump(mut mrb: *mut mrb_state,
                                      mut str: mrb_value) -> mrb_value {
    let mut len: mrb_int = 0;
    let mut p: *const libc::c_char = 0 as *const libc::c_char;
    let mut pend: *const libc::c_char = 0 as *const libc::c_char;
    let mut q: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut result: *mut RString = 0 as *mut RString;
    len = 2i32 as mrb_int;
    p =
        if 0 !=
               (*(str.value.p as *mut RString)).flags() as libc::c_int & 32i32
           {
            (*(str.value.p as *mut RString)).as_0.ary.as_mut_ptr()
        } else { (*(str.value.p as *mut RString)).as_0.heap.ptr };
    pend =
        p.offset((if 0 !=
                         (*(str.value.p as *mut RString)).flags() as
                             libc::c_int & 32i32 {
                      (((*(str.value.p as *mut RString)).flags() as
                            libc::c_int & 0x7c0i32) >> 6i32) as mrb_int
                  } else { (*(str.value.p as *mut RString)).as_0.heap.len })
                     as isize);
    while p < pend {
        let fresh11 = p;
        p = p.offset(1);
        let mut c: libc::c_uchar = *fresh11 as libc::c_uchar;
        match c as libc::c_int {
            34 | 92 | 10 | 13 | 9 | 12 | 11 | 8 | 7 | 27 => {
                len += 2i32 as libc::c_longlong
            }
            35 => {
                len +=
                    (if p < pend &&
                            (*p as libc::c_int == '$' as i32 ||
                                 *p as libc::c_int == '@' as i32 ||
                                 *p as libc::c_int == '{' as i32) {
                         2i32
                     } else { 1i32 }) as libc::c_longlong
            }
            _ => {
                if (c as libc::c_uint).wrapping_sub(0x20i32 as libc::c_uint) <
                       0x5fi32 as libc::c_uint {
                    len += 1
                } else { len += 4i32 as libc::c_longlong }
            }
        }
    }
    result = str_new(mrb, 0 as *const libc::c_char, len as size_t);
    str_with_class(mrb, result, str);
    p =
        if 0 !=
               (*(str.value.p as *mut RString)).flags() as libc::c_int & 32i32
           {
            (*(str.value.p as *mut RString)).as_0.ary.as_mut_ptr()
        } else { (*(str.value.p as *mut RString)).as_0.heap.ptr };
    pend =
        p.offset((if 0 !=
                         (*(str.value.p as *mut RString)).flags() as
                             libc::c_int & 32i32 {
                      (((*(str.value.p as *mut RString)).flags() as
                            libc::c_int & 0x7c0i32) >> 6i32) as mrb_int
                  } else { (*(str.value.p as *mut RString)).as_0.heap.len })
                     as isize);
    q =
        if 0 != (*result).flags() as libc::c_int & 32i32 {
            (*result).as_0.ary.as_mut_ptr()
        } else { (*result).as_0.heap.ptr };
    let fresh12 = q;
    q = q.offset(1);
    *fresh12 = '\"' as i32 as libc::c_char;
    while p < pend {
        let fresh13 = p;
        p = p.offset(1);
        let mut c_0: libc::c_uchar = *fresh13 as libc::c_uchar;
        match c_0 as libc::c_int {
            34 | 92 => {
                let fresh14 = q;
                q = q.offset(1);
                *fresh14 = '\\' as i32 as libc::c_char;
                let fresh15 = q;
                q = q.offset(1);
                *fresh15 = c_0 as libc::c_char
            }
            10 => {
                let fresh16 = q;
                q = q.offset(1);
                *fresh16 = '\\' as i32 as libc::c_char;
                let fresh17 = q;
                q = q.offset(1);
                *fresh17 = 'n' as i32 as libc::c_char
            }
            13 => {
                let fresh18 = q;
                q = q.offset(1);
                *fresh18 = '\\' as i32 as libc::c_char;
                let fresh19 = q;
                q = q.offset(1);
                *fresh19 = 'r' as i32 as libc::c_char
            }
            9 => {
                let fresh20 = q;
                q = q.offset(1);
                *fresh20 = '\\' as i32 as libc::c_char;
                let fresh21 = q;
                q = q.offset(1);
                *fresh21 = 't' as i32 as libc::c_char
            }
            12 => {
                let fresh22 = q;
                q = q.offset(1);
                *fresh22 = '\\' as i32 as libc::c_char;
                let fresh23 = q;
                q = q.offset(1);
                *fresh23 = 'f' as i32 as libc::c_char
            }
            11 => {
                let fresh24 = q;
                q = q.offset(1);
                *fresh24 = '\\' as i32 as libc::c_char;
                let fresh25 = q;
                q = q.offset(1);
                *fresh25 = 'v' as i32 as libc::c_char
            }
            8 => {
                let fresh26 = q;
                q = q.offset(1);
                *fresh26 = '\\' as i32 as libc::c_char;
                let fresh27 = q;
                q = q.offset(1);
                *fresh27 = 'b' as i32 as libc::c_char
            }
            7 => {
                let fresh28 = q;
                q = q.offset(1);
                *fresh28 = '\\' as i32 as libc::c_char;
                let fresh29 = q;
                q = q.offset(1);
                *fresh29 = 'a' as i32 as libc::c_char
            }
            27 => {
                let fresh30 = q;
                q = q.offset(1);
                *fresh30 = '\\' as i32 as libc::c_char;
                let fresh31 = q;
                q = q.offset(1);
                *fresh31 = 'e' as i32 as libc::c_char
            }
            35 => {
                if p < pend &&
                       (*p as libc::c_int == '$' as i32 ||
                            *p as libc::c_int == '@' as i32 ||
                            *p as libc::c_int == '{' as i32) {
                    let fresh32 = q;
                    q = q.offset(1);
                    *fresh32 = '\\' as i32 as libc::c_char
                }
                let fresh33 = q;
                q = q.offset(1);
                *fresh33 = '#' as i32 as libc::c_char
            }
            _ => {
                if (c_0 as libc::c_uint).wrapping_sub(0x20i32 as libc::c_uint)
                       < 0x5fi32 as libc::c_uint {
                    let fresh34 = q;
                    q = q.offset(1);
                    *fresh34 = c_0 as libc::c_char
                } else {
                    let fresh35 = q;
                    q = q.offset(1);
                    *fresh35 = '\\' as i32 as libc::c_char;
                    let fresh36 = q;
                    q = q.offset(1);
                    *fresh36 = 'x' as i32 as libc::c_char;
                    *q.offset(1isize) =
                        mrb_digitmap[(c_0 as libc::c_int % 16i32) as usize];
                    c_0 = (c_0 as libc::c_int / 16i32) as libc::c_uchar;
                    *q.offset(0isize) =
                        mrb_digitmap[(c_0 as libc::c_int % 16i32) as usize];
                    q = q.offset(2isize)
                }
            }
        }
    }
    *q = '\"' as i32 as libc::c_char;
    return mrb_obj_value(result as *mut libc::c_void);
}
/*
 * Returns a printable version of str, surrounded by quote marks, with special characters escaped.
 */
#[no_mangle]
pub unsafe extern "C" fn mrb_str_inspect(mut mrb: *mut mrb_state,
                                         mut str: mrb_value) -> mrb_value {
    let mut p: *const libc::c_char = 0 as *const libc::c_char;
    let mut pend: *const libc::c_char = 0 as *const libc::c_char;
    let mut buf: [libc::c_char; 14] = [0; 14];
    let mut result: mrb_value =
        mrb_str_new_static(mrb, b"\"\x00" as *const u8 as *const libc::c_char,
                           (::std::mem::size_of::<[libc::c_char; 2]>() as
                                libc::c_ulong).wrapping_sub(1i32 as
                                                                libc::c_ulong));
    p =
        if 0 !=
               (*(str.value.p as *mut RString)).flags() as libc::c_int & 32i32
           {
            (*(str.value.p as *mut RString)).as_0.ary.as_mut_ptr()
        } else { (*(str.value.p as *mut RString)).as_0.heap.ptr };
    pend =
        if 0 !=
               (*(str.value.p as *mut RString)).flags() as libc::c_int & 32i32
           {
            (*(str.value.p as *mut RString)).as_0.ary.as_mut_ptr()
        } else {
            (*(str.value.p as *mut RString)).as_0.heap.ptr
        }.offset((if 0 !=
                         (*(str.value.p as *mut RString)).flags() as
                             libc::c_int & 32i32 {
                      (((*(str.value.p as *mut RString)).flags() as
                            libc::c_int & 0x7c0i32) >> 6i32) as mrb_int
                  } else { (*(str.value.p as *mut RString)).as_0.heap.len })
                     as isize);
    while p < pend {
        let mut c: libc::c_uchar = 0;
        let mut cc: libc::c_uchar = 0;
        let mut clen: mrb_int = 0;
        clen = utf8len(p, pend);
        if clen > 1i32 as libc::c_longlong {
            let mut i: mrb_int = 0;
            i = 0i32 as mrb_int;
            while i < clen { buf[i as usize] = *p.offset(i as isize); i += 1 }
            mrb_str_cat(mrb, result, buf.as_mut_ptr(), clen as size_t);
            p = p.offset((clen - 1i32 as libc::c_longlong) as isize)
        } else {
            c = *p as libc::c_uchar;
            if c as libc::c_int == '\"' as i32 ||
                   c as libc::c_int == '\\' as i32 ||
                   c as libc::c_int == '#' as i32 &&
                       (p.offset(1isize) < pend &&
                            (*p.offset(1isize) as libc::c_int == '$' as i32 ||
                                 *p.offset(1isize) as libc::c_int ==
                                     '@' as i32 ||
                                 *p.offset(1isize) as libc::c_int ==
                                     '{' as i32)) {
                buf[0usize] = '\\' as i32 as libc::c_char;
                buf[1usize] = c as libc::c_char;
                mrb_str_cat(mrb, result, buf.as_mut_ptr(), 2i32 as size_t);
            } else if (c as
                           libc::c_uint).wrapping_sub(0x20i32 as libc::c_uint)
                          < 0x5fi32 as libc::c_uint {
                buf[0usize] = c as libc::c_char;
                mrb_str_cat(mrb, result, buf.as_mut_ptr(), 1i32 as size_t);
            } else {
                match c as libc::c_int {
                    10 => { cc = 'n' as i32 as libc::c_uchar }
                    13 => { cc = 'r' as i32 as libc::c_uchar }
                    9 => { cc = 't' as i32 as libc::c_uchar }
                    12 => { cc = 'f' as i32 as libc::c_uchar }
                    11 => { cc = 'v' as i32 as libc::c_uchar }
                    8 => { cc = 'b' as i32 as libc::c_uchar }
                    7 => { cc = 'a' as i32 as libc::c_uchar }
                    27 => { cc = 'e' as i32 as libc::c_uchar }
                    _ => { cc = 0i32 as libc::c_uchar }
                }
                if 0 != cc {
                    buf[0usize] = '\\' as i32 as libc::c_char;
                    buf[1usize] = cc as libc::c_char;
                    mrb_str_cat(mrb, result, buf.as_mut_ptr(),
                                2i32 as size_t);
                } else {
                    buf[0usize] = '\\' as i32 as libc::c_char;
                    buf[1usize] = 'x' as i32 as libc::c_char;
                    buf[3usize] =
                        mrb_digitmap[(c as libc::c_int % 16i32) as usize];
                    c = (c as libc::c_int / 16i32) as libc::c_uchar;
                    buf[2usize] =
                        mrb_digitmap[(c as libc::c_int % 16i32) as usize];
                    mrb_str_cat(mrb, result, buf.as_mut_ptr(),
                                4i32 as size_t);
                }
            }
        }
        p = p.offset(1isize)
    }
    mrb_str_cat(mrb, result, b"\"\x00" as *const u8 as *const libc::c_char,
                (::std::mem::size_of::<[libc::c_char; 2]>() as
                     libc::c_ulong).wrapping_sub(1i32 as libc::c_ulong));
    return result;
}
unsafe extern "C" fn mrb_str_new_empty(mut mrb: *mut mrb_state,
                                       mut str: mrb_value) -> mrb_value {
    let mut s: *mut RString =
        str_new(mrb, 0 as *const libc::c_char, 0i32 as size_t);
    str_with_class(mrb, s, str);
    return mrb_obj_value(s as *mut libc::c_void);
}
/* map byte offset to character index */
unsafe extern "C" fn bytes2chars(mut p: *mut libc::c_char, mut bi: mrb_int)
 -> mrb_int {
    let mut i: mrb_int = 0;
    let mut b: mrb_int = 0;
    let mut n: mrb_int = 0;
    i = 0i32 as mrb_int;
    b = i;
    while b < bi {
        n = utf8len_codepage[*p as libc::c_uchar as usize] as mrb_int;
        b += n;
        p = p.offset(n as isize);
        i += 1
    }
    if b != bi { return -1i32 as mrb_int }
    return i;
}
unsafe extern "C" fn str_index_str(mut mrb: *mut mrb_state,
                                   mut str: mrb_value, mut str2: mrb_value,
                                   mut offset: mrb_int) -> mrb_int {
    let mut ptr: *const libc::c_char = 0 as *const libc::c_char;
    let mut len: mrb_int = 0;
    ptr =
        if 0 !=
               (*(str2.value.p as *mut RString)).flags() as libc::c_int &
                   32i32 {
            (*(str2.value.p as *mut RString)).as_0.ary.as_mut_ptr()
        } else { (*(str2.value.p as *mut RString)).as_0.heap.ptr };
    len =
        if 0 !=
               (*(str2.value.p as *mut RString)).flags() as libc::c_int &
                   32i32 {
            (((*(str2.value.p as *mut RString)).flags() as libc::c_int &
                  0x7c0i32) >> 6i32) as mrb_int
        } else { (*(str2.value.p as *mut RString)).as_0.heap.len };
    return mrb_str_index(mrb, str, ptr, len, offset);
}
unsafe extern "C" fn str_rindex(mut mrb: *mut mrb_state, mut str: mrb_value,
                                mut sub: mrb_value, mut pos: mrb_int)
 -> mrb_int {
    let mut s: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut sbeg: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut t: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut ps: *mut RString = str.value.p as *mut RString;
    let mut len: mrb_int =
        if 0 !=
               (*(sub.value.p as *mut RString)).flags() as libc::c_int & 32i32
           {
            (((*(sub.value.p as *mut RString)).flags() as libc::c_int &
                  0x7c0i32) >> 6i32) as mrb_int
        } else { (*(sub.value.p as *mut RString)).as_0.heap.len };
    if if 0 != (*ps).flags() as libc::c_int & 32i32 {
           (((*ps).flags() as libc::c_int & 0x7c0i32) >> 6i32) as mrb_int
       } else { (*ps).as_0.heap.len } < len {
        return -1i32 as mrb_int
    }
    if if 0 != (*ps).flags() as libc::c_int & 32i32 {
           (((*ps).flags() as libc::c_int & 0x7c0i32) >> 6i32) as mrb_int
       } else { (*ps).as_0.heap.len } - pos < len {
        pos =
            if 0 != (*ps).flags() as libc::c_int & 32i32 {
                (((*ps).flags() as libc::c_int & 0x7c0i32) >> 6i32) as mrb_int
            } else { (*ps).as_0.heap.len } - len
    }
    sbeg =
        if 0 != (*ps).flags() as libc::c_int & 32i32 {
            (*ps).as_0.ary.as_mut_ptr()
        } else { (*ps).as_0.heap.ptr };
    s =
        if 0 != (*ps).flags() as libc::c_int & 32i32 {
            (*ps).as_0.ary.as_mut_ptr()
        } else { (*ps).as_0.heap.ptr }.offset(pos as isize);
    t =
        if 0 !=
               (*(sub.value.p as *mut RString)).flags() as libc::c_int & 32i32
           {
            (*(sub.value.p as *mut RString)).as_0.ary.as_mut_ptr()
        } else { (*(sub.value.p as *mut RString)).as_0.heap.ptr };
    if 0 != len {
        while sbeg <= s {
            if memcmp(s as *const libc::c_void, t as *const libc::c_void,
                      len as libc::c_ulong) == 0i32 {
                return s.wrapping_offset_from(if 0 !=
                                                     (*ps).flags() as
                                                         libc::c_int & 32i32 {
                                                  (*ps).as_0.ary.as_mut_ptr()
                                              } else { (*ps).as_0.heap.ptr })
                           as libc::c_long as mrb_int
            }
            s = s.offset(-1isize)
        }
        return -1i32 as mrb_int
    } else { return pos };
}
/* 15.2.10.5.2  */
/*
 *  call-seq:
 *     str + other_str   -> new_str
 *
 *  Concatenation---Returns a new <code>String</code> containing
 *  <i>other_str</i> concatenated to <i>str</i>.
 *
 *     "Hello from " + self.to_s   #=> "Hello from main"
 */
unsafe extern "C" fn mrb_str_plus_m(mut mrb: *mut mrb_state,
                                    mut self_0: mrb_value) -> mrb_value {
    let mut str: mrb_value =
        mrb_value{value: unnamed{f: 0.,}, tt: MRB_TT_FALSE,};
    mrb_get_args(mrb, b"S\x00" as *const u8 as *const libc::c_char,
                 &mut str as *mut mrb_value);
    return mrb_str_plus(mrb, self_0, str);
}
/* 15.2.10.5.26 */
/* 15.2.10.5.33 */
/*
 *  call-seq:
 *     "abcd".size   => int
 *
 *  Returns the length of string.
 */
unsafe extern "C" fn mrb_str_size(mut mrb: *mut mrb_state,
                                  mut self_0: mrb_value) -> mrb_value {
    let mut len: mrb_int = utf8_strlen(self_0);
    return mrb_fixnum_value(len);
}
unsafe extern "C" fn mrb_str_bytesize(mut mrb: *mut mrb_state,
                                      mut self_0: mrb_value) -> mrb_value {
    let mut len: mrb_int =
        if 0 !=
               (*(self_0.value.p as *mut RString)).flags() as libc::c_int &
                   32i32 {
            (((*(self_0.value.p as *mut RString)).flags() as libc::c_int &
                  0x7c0i32) >> 6i32) as mrb_int
        } else { (*(self_0.value.p as *mut RString)).as_0.heap.len };
    return mrb_fixnum_value(len);
}
/* 15.2.10.5.1  */
/*
 *  call-seq:
 *     str * integer   => new_str
 *
 *  Copy---Returns a new <code>String</code> containing <i>integer</i> copies of
 *  the receiver.
 *
 *     "Ho! " * 3   #=> "Ho! Ho! Ho! "
 */
unsafe extern "C" fn mrb_str_times(mut mrb: *mut mrb_state,
                                   mut self_0: mrb_value) -> mrb_value {
    let mut n: mrb_int = 0;
    let mut len: mrb_int = 0;
    let mut times: mrb_int = 0;
    let mut str2: *mut RString = 0 as *mut RString;
    let mut p: *mut libc::c_char = 0 as *mut libc::c_char;
    mrb_get_args(mrb, b"i\x00" as *const u8 as *const libc::c_char,
                 &mut times as *mut mrb_int);
    if times < 0i32 as libc::c_longlong {
        mrb_raise(mrb,
                  mrb_exc_get(mrb,
                              b"ArgumentError\x00" as *const u8 as
                                  *const libc::c_char),
                  b"negative argument\x00" as *const u8 as
                      *const libc::c_char);
    }
    if 0 != times &&
           (9223372036854775807i64 >> 0i32) / times <
               if 0 !=
                      (*(self_0.value.p as *mut RString)).flags() as
                          libc::c_int & 32i32 {
                   (((*(self_0.value.p as *mut RString)).flags() as
                         libc::c_int & 0x7c0i32) >> 6i32) as mrb_int
               } else { (*(self_0.value.p as *mut RString)).as_0.heap.len } {
        mrb_raise(mrb,
                  mrb_exc_get(mrb,
                              b"ArgumentError\x00" as *const u8 as
                                  *const libc::c_char),
                  b"argument too big\x00" as *const u8 as
                      *const libc::c_char);
    }
    len =
        if 0 !=
               (*(self_0.value.p as *mut RString)).flags() as libc::c_int &
                   32i32 {
            (((*(self_0.value.p as *mut RString)).flags() as libc::c_int &
                  0x7c0i32) >> 6i32) as mrb_int
        } else { (*(self_0.value.p as *mut RString)).as_0.heap.len } * times;
    str2 = str_new(mrb, 0 as *const libc::c_char, len as size_t);
    str_with_class(mrb, str2, self_0);
    p =
        if 0 != (*str2).flags() as libc::c_int & 32i32 {
            (*str2).as_0.ary.as_mut_ptr()
        } else { (*str2).as_0.heap.ptr };
    if len > 0i32 as libc::c_longlong {
        n =
            if 0 !=
                   (*(self_0.value.p as *mut RString)).flags() as libc::c_int
                       & 32i32 {
                (((*(self_0.value.p as *mut RString)).flags() as libc::c_int &
                      0x7c0i32) >> 6i32) as mrb_int
            } else { (*(self_0.value.p as *mut RString)).as_0.heap.len };
        memcpy(p as *mut libc::c_void,
               (if 0 !=
                       (*(self_0.value.p as *mut RString)).flags() as
                           libc::c_int & 32i32 {
                    (*(self_0.value.p as *mut RString)).as_0.ary.as_mut_ptr()
                } else { (*(self_0.value.p as *mut RString)).as_0.heap.ptr })
                   as *const libc::c_void, n as libc::c_ulong);
        while n <= len / 2i32 as libc::c_longlong {
            memcpy(p.offset(n as isize) as *mut libc::c_void,
                   p as *const libc::c_void, n as libc::c_ulong);
            n *= 2i32 as libc::c_longlong
        }
        memcpy(p.offset(n as isize) as *mut libc::c_void,
               p as *const libc::c_void, (len - n) as libc::c_ulong);
    }
    *p.offset((if 0 != (*str2).flags() as libc::c_int & 32i32 {
                   (((*str2).flags() as libc::c_int & 0x7c0i32) >> 6i32) as
                       mrb_int
               } else { (*str2).as_0.heap.len }) as isize) =
        '\u{0}' as i32 as libc::c_char;
    return mrb_obj_value(str2 as *mut libc::c_void);
}
/* 15.2.10.5.3  */
/*
 *  call-seq:
 *     str <=> other_str   => -1, 0, +1
 *
 *  Comparison---Returns -1 if <i>other_str</i> is less than, 0 if
 *  <i>other_str</i> is equal to, and +1 if <i>other_str</i> is greater than
 *  <i>str</i>. If the strings are of different lengths, and the strings are
 *  equal when compared up to the shortest length, then the longer string is
 *  considered greater than the shorter one. If the variable <code>$=</code> is
 *  <code>false</code>, the comparison is based on comparing the binary values
 *  of each character in the string. In older versions of Ruby, setting
 *  <code>$=</code> allowed case-insensitive comparisons; this is now deprecated
 *  in favor of using <code>String#casecmp</code>.
 *
 *  <code><=></code> is the basis for the methods <code><</code>,
 *  <code><=</code>, <code>></code>, <code>>=</code>, and <code>between?</code>,
 *  included from module <code>Comparable</code>.  The method
 *  <code>String#==</code> does not use <code>Comparable#==</code>.
 *
 *     "abcdef" <=> "abcde"     #=> 1
 *     "abcdef" <=> "abcdef"    #=> 0
 *     "abcdef" <=> "abcdefg"   #=> -1
 *     "abcdef" <=> "ABCDEF"    #=> 1
 */
unsafe extern "C" fn mrb_str_cmp_m(mut mrb: *mut mrb_state,
                                   mut str1: mrb_value) -> mrb_value {
    let mut str2: mrb_value =
        mrb_value{value: unnamed{f: 0.,}, tt: MRB_TT_FALSE,};
    let mut result: mrb_int = 0;
    mrb_get_args(mrb, b"o\x00" as *const u8 as *const libc::c_char,
                 &mut str2 as *mut mrb_value);
    if !(str2.tt as libc::c_uint ==
             MRB_TT_STRING as libc::c_int as libc::c_uint) {
        if 0 ==
               mrb_respond_to(mrb, str2,
                              mrb_intern_static(mrb,
                                                b"to_s\x00" as *const u8 as
                                                    *const libc::c_char,
                                                (::std::mem::size_of::<[libc::c_char; 5]>()
                                                     as
                                                     libc::c_ulong).wrapping_sub(1i32
                                                                                     as
                                                                                     libc::c_ulong)))
           {
            return mrb_nil_value()
        } else {
            if 0 ==
                   mrb_respond_to(mrb, str2,
                                  mrb_intern_static(mrb,
                                                    b"<=>\x00" as *const u8 as
                                                        *const libc::c_char,
                                                    (::std::mem::size_of::<[libc::c_char; 4]>()
                                                         as
                                                         libc::c_ulong).wrapping_sub(1i32
                                                                                         as
                                                                                         libc::c_ulong)))
               {
                return mrb_nil_value()
            } else {
                let mut tmp: mrb_value =
                    mrb_funcall(mrb, str2,
                                b"<=>\x00" as *const u8 as
                                    *const libc::c_char, 1i32 as mrb_int,
                                str1);
                if tmp.tt as libc::c_uint ==
                       MRB_TT_FALSE as libc::c_int as libc::c_uint &&
                       0 == tmp.value.i {
                    return mrb_nil_value()
                }
                if !(tmp.tt as libc::c_uint ==
                         MRB_TT_FIXNUM as libc::c_int as libc::c_uint) {
                    return mrb_funcall(mrb, mrb_fixnum_value(0i32 as mrb_int),
                                       b"-\x00" as *const u8 as
                                           *const libc::c_char,
                                       1i32 as mrb_int, tmp)
                }
                result = -tmp.value.i
            }
        }
    } else { result = mrb_str_cmp(mrb, str1, str2) as mrb_int }
    return mrb_fixnum_value(result);
}
/* 15.2.10.5.4  */
/*
 *  call-seq:
 *     str == obj   => true or false
 *
 *  Equality---
 *  If <i>obj</i> is not a <code>String</code>, returns <code>false</code>.
 *  Otherwise, returns <code>false</code> or <code>true</code>
 *
 *   caution:if <i>str</i> <code><=></code> <i>obj</i> returns zero.
 */
unsafe extern "C" fn mrb_str_equal_m(mut mrb: *mut mrb_state,
                                     mut str1: mrb_value) -> mrb_value {
    let mut str2: mrb_value =
        mrb_value{value: unnamed{f: 0.,}, tt: MRB_TT_FALSE,};
    mrb_get_args(mrb, b"o\x00" as *const u8 as *const libc::c_char,
                 &mut str2 as *mut mrb_value);
    return mrb_bool_value(mrb_str_equal(mrb, str1, str2));
}
unsafe extern "C" fn mrb_str_aref(mut mrb: *mut mrb_state, mut str: mrb_value,
                                  mut indx: mrb_value) -> mrb_value {
    let mut idx: mrb_int = 0;
    let mut current_block_15: u64;
    match indx.tt as libc::c_uint {
        3 => { idx = indx.value.i; current_block_15 = 11571908411073679048; }
        16 => {
            if str_index_str(mrb, str, indx, 0i32 as mrb_int) !=
                   -1i32 as libc::c_longlong {
                return mrb_str_dup(mrb, indx)
            }
            return mrb_nil_value()
        }
        17 => { current_block_15 = 14222806698499446664; }
        _ => {
            indx = mrb_Integer(mrb, indx);
            if indx.tt as libc::c_uint ==
                   MRB_TT_FALSE as libc::c_int as libc::c_uint &&
                   0 == indx.value.i {
                current_block_15 = 14222806698499446664;
            } else {
                idx = indx.value.i;
                current_block_15 = 11571908411073679048;
            }
        }
    }
    match current_block_15 {
        11571908411073679048 => {
            str = str_substr(mrb, str, idx, 1i32 as mrb_int);
            if !(str.tt as libc::c_uint ==
                     MRB_TT_FALSE as libc::c_int as libc::c_uint &&
                     0 == str.value.i) &&
                   if 0 !=
                          (*(str.value.p as *mut RString)).flags() as
                              libc::c_int & 32i32 {
                       (((*(str.value.p as *mut RString)).flags() as
                             libc::c_int & 0x7c0i32) >> 6i32) as mrb_int
                   } else { (*(str.value.p as *mut RString)).as_0.heap.len }
                       == 0i32 as libc::c_longlong {
                return mrb_nil_value()
            }
            return str
        }
        _ => {
            let mut beg: mrb_int = 0;
            let mut len: mrb_int = 0;
            len = utf8_strlen(str);
            match mrb_range_beg_len(mrb, indx, &mut beg, &mut len, len,
                                    1i32 as mrb_bool) as libc::c_uint {
                1 => { return str_subseq(mrb, str, beg, len) }
                2 => { return mrb_nil_value() }
                _ => { }
            }
            mrb_raise(mrb,
                      mrb_exc_get(mrb,
                                  b"TypeError\x00" as *const u8 as
                                      *const libc::c_char),
                      b"can\'t convert to Fixnum\x00" as *const u8 as
                          *const libc::c_char);
        }
    };
}
/* 15.2.10.5.6  */
/* 15.2.10.5.34 */
/*
 *  call-seq:
 *     str[fixnum]                 => fixnum or nil
 *     str[fixnum, fixnum]         => new_str or nil
 *     str[range]                  => new_str or nil
 *     str[regexp]                 => new_str or nil
 *     str[regexp, fixnum]         => new_str or nil
 *     str[other_str]              => new_str or nil
 *     str.slice(fixnum)           => fixnum or nil
 *     str.slice(fixnum, fixnum)   => new_str or nil
 *     str.slice(range)            => new_str or nil
 *     str.slice(other_str)        => new_str or nil
 *
 *  Element Reference---If passed a single <code>Fixnum</code>, returns the code
 *  of the character at that position. If passed two <code>Fixnum</code>
 *  objects, returns a substring starting at the offset given by the first, and
 *  a length given by the second. If given a range, a substring containing
 *  characters at offsets given by the range is returned. In all three cases, if
 *  an offset is negative, it is counted from the end of <i>str</i>. Returns
 *  <code>nil</code> if the initial offset falls outside the string, the length
 *  is negative, or the beginning of the range is greater than the end.
 *
 *  If a <code>String</code> is given, that string is returned if it occurs in
 *  <i>str</i>. In both cases, <code>nil</code> is returned if there is no
 *  match.
 *
 *     a = "hello there"
 *     a[1]                   #=> 101(1.8.7) "e"(1.9.2)
 *     a[1.1]                 #=>            "e"(1.9.2)
 *     a[1,3]                 #=> "ell"
 *     a[1..3]                #=> "ell"
 *     a[-3,2]                #=> "er"
 *     a[-4..-2]              #=> "her"
 *     a[12..-1]              #=> nil
 *     a[-2..-4]              #=> ""
 *     a["lo"]                #=> "lo"
 *     a["bye"]               #=> nil
 */
unsafe extern "C" fn mrb_str_aref_m(mut mrb: *mut mrb_state,
                                    mut str: mrb_value) -> mrb_value {
    let mut a1: mrb_value =
        mrb_value{value: unnamed{f: 0.,}, tt: MRB_TT_FALSE,};
    let mut a2: mrb_value =
        mrb_value{value: unnamed{f: 0.,}, tt: MRB_TT_FALSE,};
    let mut argc: mrb_int = 0;
    argc =
        mrb_get_args(mrb, b"o|o\x00" as *const u8 as *const libc::c_char,
                     &mut a1 as *mut mrb_value, &mut a2 as *mut mrb_value);
    if argc == 2i32 as libc::c_longlong {
        let mut n1: mrb_int = 0;
        let mut n2: mrb_int = 0;
        mrb_get_args(mrb, b"ii\x00" as *const u8 as *const libc::c_char,
                     &mut n1 as *mut mrb_int, &mut n2 as *mut mrb_int);
        return str_substr(mrb, str, n1, n2)
    }
    return mrb_str_aref(mrb, str, a1);
}
/* 15.2.10.5.8  */
/*
 *  call-seq:
 *     str.capitalize!   => str or nil
 *
 *  Modifies <i>str</i> by converting the first character to uppercase and the
 *  remainder to lowercase. Returns <code>nil</code> if no changes are made.
 *
 *     a = "hello"
 *     a.capitalize!   #=> "Hello"
 *     a               #=> "Hello"
 *     a.capitalize!   #=> nil
 */
unsafe extern "C" fn mrb_str_capitalize_bang(mut mrb: *mut mrb_state,
                                             mut str: mrb_value)
 -> mrb_value {
    let mut p: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut pend: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut modify: mrb_bool = 0i32 as mrb_bool;
    let mut s: *mut RString = str.value.p as *mut RString;
    mrb_str_modify(mrb, s);
    if if 0 != (*s).flags() as libc::c_int & 32i32 {
           (((*s).flags() as libc::c_int & 0x7c0i32) >> 6i32) as mrb_int
       } else { (*s).as_0.heap.len } == 0i32 as libc::c_longlong ||
           if 0 != (*s).flags() as libc::c_int & 32i32 {
               (*s).as_0.ary.as_mut_ptr()
           } else { (*s).as_0.heap.ptr }.is_null() {
        return mrb_nil_value()
    }
    p =
        if 0 != (*s).flags() as libc::c_int & 32i32 {
            (*s).as_0.ary.as_mut_ptr()
        } else { (*s).as_0.heap.ptr };
    pend =
        if 0 != (*s).flags() as libc::c_int & 32i32 {
            (*s).as_0.ary.as_mut_ptr()
        } else {
            (*s).as_0.heap.ptr
        }.offset((if 0 != (*s).flags() as libc::c_int & 32i32 {
                      (((*s).flags() as libc::c_int & 0x7c0i32) >> 6i32) as
                          mrb_int
                  } else { (*s).as_0.heap.len }) as isize);
    if (*p as libc::c_uint).wrapping_sub('a' as i32 as libc::c_uint) <
           26i32 as libc::c_uint {
        *p =
            (if (*p as libc::c_uint).wrapping_sub('a' as i32 as libc::c_uint)
                    < 26i32 as libc::c_uint {
                 *p as libc::c_int & 0x5fi32
             } else { *p as libc::c_int }) as libc::c_char;
        modify = 1i32 as mrb_bool
    }
    loop  {
        p = p.offset(1isize);
        if !(p < pend) { break ; }
        if (*p as libc::c_uint).wrapping_sub('A' as i32 as libc::c_uint) <
               26i32 as libc::c_uint {
            *p =
                (if (*p as
                         libc::c_uint).wrapping_sub('A' as i32 as
                                                        libc::c_uint) <
                        26i32 as libc::c_uint {
                     *p as libc::c_int | 0x20i32
                 } else { *p as libc::c_int }) as libc::c_char;
            modify = 1i32 as mrb_bool
        }
    }
    if 0 != modify { return str }
    return mrb_nil_value();
}
/* 15.2.10.5.7  */
/*
 *  call-seq:
 *     str.capitalize   => new_str
 *
 *  Returns a copy of <i>str</i> with the first character converted to uppercase
 *  and the remainder to lowercase.
 *
 *     "hello".capitalize    #=> "Hello"
 *     "HELLO".capitalize    #=> "Hello"
 *     "123ABC".capitalize   #=> "123abc"
 */
unsafe extern "C" fn mrb_str_capitalize(mut mrb: *mut mrb_state,
                                        mut self_0: mrb_value) -> mrb_value {
    let mut str: mrb_value =
        mrb_value{value: unnamed{f: 0.,}, tt: MRB_TT_FALSE,};
    str = mrb_str_dup(mrb, self_0);
    mrb_str_capitalize_bang(mrb, str);
    return str;
}
/* 15.2.10.5.10  */
/*
 *  call-seq:
 *     str.chomp!(separator="\n")   => str or nil
 *
 *  Modifies <i>str</i> in place as described for <code>String#chomp</code>,
 *  returning <i>str</i>, or <code>nil</code> if no modifications were made.
 */
unsafe extern "C" fn mrb_str_chomp_bang(mut mrb: *mut mrb_state,
                                        mut str: mrb_value) -> mrb_value {
    let mut rs: mrb_value =
        mrb_value{value: unnamed{f: 0.,}, tt: MRB_TT_FALSE,};
    let mut newline: mrb_int = 0;
    let mut p: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut pp: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut rslen: mrb_int = 0;
    let mut len: mrb_int = 0;
    let mut argc: mrb_int = 0;
    let mut s: *mut RString = str.value.p as *mut RString;
    argc =
        mrb_get_args(mrb, b"|S\x00" as *const u8 as *const libc::c_char,
                     &mut rs as *mut mrb_value);
    mrb_str_modify(mrb, s);
    len =
        if 0 != (*s).flags() as libc::c_int & 32i32 {
            (((*s).flags() as libc::c_int & 0x7c0i32) >> 6i32) as mrb_int
        } else { (*s).as_0.heap.len };
    if argc == 0i32 as libc::c_longlong {
        if len == 0i32 as libc::c_longlong { return mrb_nil_value() }
    } else {
        if len == 0i32 as libc::c_longlong ||
               rs.tt as libc::c_uint ==
                   MRB_TT_FALSE as libc::c_int as libc::c_uint &&
                   0 == rs.value.i {
            return mrb_nil_value()
        }
        p =
            if 0 != (*s).flags() as libc::c_int & 32i32 {
                (*s).as_0.ary.as_mut_ptr()
            } else { (*s).as_0.heap.ptr };
        rslen =
            if 0 !=
                   (*(rs.value.p as *mut RString)).flags() as libc::c_int &
                       32i32 {
                (((*(rs.value.p as *mut RString)).flags() as libc::c_int &
                      0x7c0i32) >> 6i32) as mrb_int
            } else { (*(rs.value.p as *mut RString)).as_0.heap.len };
        if rslen == 0i32 as libc::c_longlong {
            while len > 0i32 as libc::c_longlong &&
                      *p.offset((len - 1i32 as libc::c_longlong) as isize) as
                          libc::c_int == '\n' as i32 {
                len -= 1;
                if len > 0i32 as libc::c_longlong &&
                       *p.offset((len - 1i32 as libc::c_longlong) as isize) as
                           libc::c_int == '\r' as i32 {
                    len -= 1
                }
            }
            if len <
                   if 0 != (*s).flags() as libc::c_int & 32i32 {
                       (((*s).flags() as libc::c_int & 0x7c0i32) >> 6i32) as
                           mrb_int
                   } else { (*s).as_0.heap.len } {
                if 0 != (*s).flags() as libc::c_int & 32i32 {
                    let mut tmp_n_2: size_t = len as size_t;
                    (*s).set_flags((*s).flags() & !0x7c0i32 as uint32_t);
                    (*s).set_flags((*s).flags() |
                                       (tmp_n_2 << 6i32) as uint32_t)
                } else { (*s).as_0.heap.len = len as mrb_int }
                *p.offset(len as isize) = '\u{0}' as i32 as libc::c_char;
                return str
            }
            return mrb_nil_value()
        }
        if rslen > len { return mrb_nil_value() }
        newline =
            *if 0 !=
                    (*(rs.value.p as *mut RString)).flags() as libc::c_int &
                        32i32 {
                 (*(rs.value.p as *mut RString)).as_0.ary.as_mut_ptr()
             } else {
                 (*(rs.value.p as *mut RString)).as_0.heap.ptr
             }.offset((rslen - 1i32 as libc::c_longlong) as isize) as mrb_int;
        if rslen == 1i32 as libc::c_longlong &&
               newline == '\n' as i32 as libc::c_longlong {
            newline =
                *if 0 !=
                        (*(rs.value.p as *mut RString)).flags() as libc::c_int
                            & 32i32 {
                     (*(rs.value.p as *mut RString)).as_0.ary.as_mut_ptr()
                 } else {
                     (*(rs.value.p as *mut RString)).as_0.heap.ptr
                 }.offset((rslen - 1i32 as libc::c_longlong) as isize) as
                    mrb_int
        }
        if !(rslen == 1i32 as libc::c_longlong &&
                 newline == '\n' as i32 as libc::c_longlong) {
            pp = p.offset(len as isize).offset(-(rslen as isize));
            if *p.offset((len - 1i32 as libc::c_longlong) as isize) as
                   libc::c_longlong == newline &&
                   (rslen <= 1i32 as libc::c_longlong ||
                        memcmp((if 0 !=
                                       (*(rs.value.p as *mut RString)).flags()
                                           as libc::c_int & 32i32 {
                                    (*(rs.value.p as
                                           *mut RString)).as_0.ary.as_mut_ptr()
                                } else {
                                    (*(rs.value.p as
                                           *mut RString)).as_0.heap.ptr
                                }) as *const libc::c_void,
                               pp as *const libc::c_void,
                               rslen as libc::c_ulong) == 0i32) {
                if 0 != (*s).flags() as libc::c_int & 32i32 {
                    let mut tmp_n_3: size_t = (len - rslen) as size_t;
                    (*s).set_flags((*s).flags() & !0x7c0i32 as uint32_t);
                    (*s).set_flags((*s).flags() |
                                       (tmp_n_3 << 6i32) as uint32_t)
                } else { (*s).as_0.heap.len = (len - rslen) as mrb_int }
                *p.offset((if 0 != (*s).flags() as libc::c_int & 32i32 {
                               (((*s).flags() as libc::c_int & 0x7c0i32) >>
                                    6i32) as mrb_int
                           } else { (*s).as_0.heap.len }) as isize) =
                    '\u{0}' as i32 as libc::c_char;
                return str
            }
            return mrb_nil_value()
        }
    }
    if *if 0 != (*s).flags() as libc::c_int & 32i32 {
            (*s).as_0.ary.as_mut_ptr()
        } else {
            (*s).as_0.heap.ptr
        }.offset((len - 1i32 as libc::c_longlong) as isize) as libc::c_int ==
           '\n' as i32 {
        if 0 != (*s).flags() as libc::c_int & 32i32 {
            let mut tmp_n: size_t =
                (if 0 != (*s).flags() as libc::c_int & 32i32 {
                     (((*s).flags() as libc::c_int & 0x7c0i32) >> 6i32) as
                         mrb_int
                 } else { (*s).as_0.heap.len } - 1i32 as libc::c_longlong) as
                    size_t;
            (*s).set_flags((*s).flags() & !0x7c0i32 as uint32_t);
            (*s).set_flags((*s).flags() | (tmp_n << 6i32) as uint32_t)
        } else {
            (*s).as_0.heap.len =
                (if 0 != (*s).flags() as libc::c_int & 32i32 {
                     (((*s).flags() as libc::c_int & 0x7c0i32) >> 6i32) as
                         mrb_int
                 } else { (*s).as_0.heap.len } - 1i32 as libc::c_longlong) as
                    mrb_int
        }
        if if 0 != (*s).flags() as libc::c_int & 32i32 {
               (((*s).flags() as libc::c_int & 0x7c0i32) >> 6i32) as mrb_int
           } else { (*s).as_0.heap.len } > 0i32 as libc::c_longlong &&
               *if 0 != (*s).flags() as libc::c_int & 32i32 {
                    (*s).as_0.ary.as_mut_ptr()
                } else {
                    (*s).as_0.heap.ptr
                }.offset((if 0 != (*s).flags() as libc::c_int & 32i32 {
                              (((*s).flags() as libc::c_int & 0x7c0i32) >>
                                   6i32) as mrb_int
                          } else { (*s).as_0.heap.len } -
                              1i32 as libc::c_longlong) as isize) as
                   libc::c_int == '\r' as i32 {
            if 0 != (*s).flags() as libc::c_int & 32i32 {
                let mut tmp_n_0: size_t =
                    (if 0 != (*s).flags() as libc::c_int & 32i32 {
                         (((*s).flags() as libc::c_int & 0x7c0i32) >> 6i32) as
                             mrb_int
                     } else { (*s).as_0.heap.len } - 1i32 as libc::c_longlong)
                        as size_t;
                (*s).set_flags((*s).flags() & !0x7c0i32 as uint32_t);
                (*s).set_flags((*s).flags() | (tmp_n_0 << 6i32) as uint32_t)
            } else {
                (*s).as_0.heap.len =
                    (if 0 != (*s).flags() as libc::c_int & 32i32 {
                         (((*s).flags() as libc::c_int & 0x7c0i32) >> 6i32) as
                             mrb_int
                     } else { (*s).as_0.heap.len } - 1i32 as libc::c_longlong)
                        as mrb_int
            }
        }
    } else if *if 0 != (*s).flags() as libc::c_int & 32i32 {
                   (*s).as_0.ary.as_mut_ptr()
               } else {
                   (*s).as_0.heap.ptr
               }.offset((len - 1i32 as libc::c_longlong) as isize) as
                  libc::c_int == '\r' as i32 {
        if 0 != (*s).flags() as libc::c_int & 32i32 {
            let mut tmp_n_1: size_t =
                (if 0 != (*s).flags() as libc::c_int & 32i32 {
                     (((*s).flags() as libc::c_int & 0x7c0i32) >> 6i32) as
                         mrb_int
                 } else { (*s).as_0.heap.len } - 1i32 as libc::c_longlong) as
                    size_t;
            (*s).set_flags((*s).flags() & !0x7c0i32 as uint32_t);
            (*s).set_flags((*s).flags() | (tmp_n_1 << 6i32) as uint32_t)
        } else {
            (*s).as_0.heap.len =
                (if 0 != (*s).flags() as libc::c_int & 32i32 {
                     (((*s).flags() as libc::c_int & 0x7c0i32) >> 6i32) as
                         mrb_int
                 } else { (*s).as_0.heap.len } - 1i32 as libc::c_longlong) as
                    mrb_int
        }
    } else { return mrb_nil_value() }
    *if 0 != (*s).flags() as libc::c_int & 32i32 {
         (*s).as_0.ary.as_mut_ptr()
     } else {
         (*s).as_0.heap.ptr
     }.offset((if 0 != (*s).flags() as libc::c_int & 32i32 {
                   (((*s).flags() as libc::c_int & 0x7c0i32) >> 6i32) as
                       mrb_int
               } else { (*s).as_0.heap.len }) as isize) =
        '\u{0}' as i32 as libc::c_char;
    return str;
}
/* 15.2.10.5.9  */
/*
 *  call-seq:
 *     str.chomp(separator="\n")   => new_str
 *
 *  Returns a new <code>String</code> with the given record separator removed
 *  from the end of <i>str</i> (if present). If <code>$/</code> has not been
 *  changed from the default Ruby record separator, then <code>chomp</code> also
 *  removes carriage return characters (that is it will remove <code>\n</code>,
 *  <code>\r</code>, and <code>\r\n</code>).
 *
 *     "hello".chomp            #=> "hello"
 *     "hello\n".chomp          #=> "hello"
 *     "hello\r\n".chomp        #=> "hello"
 *     "hello\n\r".chomp        #=> "hello\n"
 *     "hello\r".chomp          #=> "hello"
 *     "hello \n there".chomp   #=> "hello \n there"
 *     "hello".chomp("llo")     #=> "he"
 */
unsafe extern "C" fn mrb_str_chomp(mut mrb: *mut mrb_state,
                                   mut self_0: mrb_value) -> mrb_value {
    let mut str: mrb_value =
        mrb_value{value: unnamed{f: 0.,}, tt: MRB_TT_FALSE,};
    str = mrb_str_dup(mrb, self_0);
    mrb_str_chomp_bang(mrb, str);
    return str;
}
/* 15.2.10.5.12 */
/*
 *  call-seq:
 *     str.chop!   => str or nil
 *
 *  Processes <i>str</i> as for <code>String#chop</code>, returning <i>str</i>,
 *  or <code>nil</code> if <i>str</i> is the empty string.  See also
 *  <code>String#chomp!</code>.
 */
unsafe extern "C" fn mrb_str_chop_bang(mut mrb: *mut mrb_state,
                                       mut str: mrb_value) -> mrb_value {
    let mut s: *mut RString = str.value.p as *mut RString;
    mrb_str_modify(mrb, s);
    if if 0 != (*s).flags() as libc::c_int & 32i32 {
           (((*s).flags() as libc::c_int & 0x7c0i32) >> 6i32) as mrb_int
       } else { (*s).as_0.heap.len } > 0i32 as libc::c_longlong {
        let mut len: mrb_int = 0;
        let mut t: *const libc::c_char =
            if 0 != (*s).flags() as libc::c_int & 32i32 {
                (*s).as_0.ary.as_mut_ptr()
            } else { (*s).as_0.heap.ptr };
        let mut p: *const libc::c_char = t;
        let mut e: *const libc::c_char =
            p.offset((if 0 != (*s).flags() as libc::c_int & 32i32 {
                          (((*s).flags() as libc::c_int & 0x7c0i32) >> 6i32)
                              as mrb_int
                      } else { (*s).as_0.heap.len }) as isize);
        while p < e {
            let mut clen: mrb_int = utf8len(p, e);
            if p.offset(clen as isize) >= e { break ; }
            p = p.offset(clen as isize)
        }
        len = p.wrapping_offset_from(t) as libc::c_long as mrb_int;
        if *if 0 != (*s).flags() as libc::c_int & 32i32 {
                (*s).as_0.ary.as_mut_ptr()
            } else { (*s).as_0.heap.ptr }.offset(len as isize) as libc::c_int
               == '\n' as i32 {
            if len > 0i32 as libc::c_longlong &&
                   *if 0 != (*s).flags() as libc::c_int & 32i32 {
                        (*s).as_0.ary.as_mut_ptr()
                    } else {
                        (*s).as_0.heap.ptr
                    }.offset((len - 1i32 as libc::c_longlong) as isize) as
                       libc::c_int == '\r' as i32 {
                len -= 1
            }
        }
        if 0 != (*s).flags() as libc::c_int & 32i32 {
            let mut tmp_n: size_t = len as size_t;
            (*s).set_flags((*s).flags() & !0x7c0i32 as uint32_t);
            (*s).set_flags((*s).flags() | (tmp_n << 6i32) as uint32_t)
        } else { (*s).as_0.heap.len = len as mrb_int }
        *if 0 != (*s).flags() as libc::c_int & 32i32 {
             (*s).as_0.ary.as_mut_ptr()
         } else { (*s).as_0.heap.ptr }.offset(len as isize) =
            '\u{0}' as i32 as libc::c_char;
        return str
    }
    return mrb_nil_value();
}
/* 15.2.10.5.11 */
/*
 *  call-seq:
 *     str.chop   => new_str
 *
 *  Returns a new <code>String</code> with the last character removed.  If the
 *  string ends with <code>\r\n</code>, both characters are removed. Applying
 *  <code>chop</code> to an empty string returns an empty
 *  string. <code>String#chomp</code> is often a safer alternative, as it leaves
 *  the string unchanged if it doesn't end in a record separator.
 *
 *     "string\r\n".chop   #=> "string"
 *     "string\n\r".chop   #=> "string\n"
 *     "string\n".chop     #=> "string"
 *     "string".chop       #=> "strin"
 *     "x".chop            #=> ""
 */
unsafe extern "C" fn mrb_str_chop(mut mrb: *mut mrb_state,
                                  mut self_0: mrb_value) -> mrb_value {
    let mut str: mrb_value =
        mrb_value{value: unnamed{f: 0.,}, tt: MRB_TT_FALSE,};
    str = mrb_str_dup(mrb, self_0);
    mrb_str_chop_bang(mrb, str);
    return str;
}
/* 15.2.10.5.14 */
/*
 *  call-seq:
 *     str.downcase!   => str or nil
 *
 *  Downcases the contents of <i>str</i>, returning <code>nil</code> if no
 *  changes were made.
 */
unsafe extern "C" fn mrb_str_downcase_bang(mut mrb: *mut mrb_state,
                                           mut str: mrb_value) -> mrb_value {
    let mut p: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut pend: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut modify: mrb_bool = 0i32 as mrb_bool;
    let mut s: *mut RString = str.value.p as *mut RString;
    mrb_str_modify(mrb, s);
    p =
        if 0 != (*s).flags() as libc::c_int & 32i32 {
            (*s).as_0.ary.as_mut_ptr()
        } else { (*s).as_0.heap.ptr };
    pend =
        if 0 != (*s).flags() as libc::c_int & 32i32 {
            (*s).as_0.ary.as_mut_ptr()
        } else {
            (*s).as_0.heap.ptr
        }.offset((if 0 != (*s).flags() as libc::c_int & 32i32 {
                      (((*s).flags() as libc::c_int & 0x7c0i32) >> 6i32) as
                          mrb_int
                  } else { (*s).as_0.heap.len }) as isize);
    while p < pend {
        if (*p as libc::c_uint).wrapping_sub('A' as i32 as libc::c_uint) <
               26i32 as libc::c_uint {
            *p =
                (if (*p as
                         libc::c_uint).wrapping_sub('A' as i32 as
                                                        libc::c_uint) <
                        26i32 as libc::c_uint {
                     *p as libc::c_int | 0x20i32
                 } else { *p as libc::c_int }) as libc::c_char;
            modify = 1i32 as mrb_bool
        }
        p = p.offset(1isize)
    }
    if 0 != modify { return str }
    return mrb_nil_value();
}
/* 15.2.10.5.13 */
/*
 *  call-seq:
 *     str.downcase   => new_str
 *
 *  Returns a copy of <i>str</i> with all uppercase letters replaced with their
 *  lowercase counterparts. The operation is locale insensitive---only
 *  characters 'A' to 'Z' are affected.
 *
 *     "hEllO".downcase   #=> "hello"
 */
unsafe extern "C" fn mrb_str_downcase(mut mrb: *mut mrb_state,
                                      mut self_0: mrb_value) -> mrb_value {
    let mut str: mrb_value =
        mrb_value{value: unnamed{f: 0.,}, tt: MRB_TT_FALSE,};
    str = mrb_str_dup(mrb, self_0);
    mrb_str_downcase_bang(mrb, str);
    return str;
}
/* 15.2.10.5.16 */
/*
 *  call-seq:
 *     str.empty?   => true or false
 *
 *  Returns <code>true</code> if <i>str</i> has a length of zero.
 *
 *     "hello".empty?   #=> false
 *     "".empty?        #=> true
 */
unsafe extern "C" fn mrb_str_empty_p(mut mrb: *mut mrb_state,
                                     mut self_0: mrb_value) -> mrb_value {
    let mut s: *mut RString = self_0.value.p as *mut RString;
    return mrb_bool_value((if 0 != (*s).flags() as libc::c_int & 32i32 {
                               (((*s).flags() as libc::c_int & 0x7c0i32) >>
                                    6i32) as mrb_int
                           } else { (*s).as_0.heap.len } ==
                               0i32 as libc::c_longlong) as libc::c_int as
                              mrb_bool);
}
/* 15.2.10.5.17 */
/*
 * call-seq:
 *   str.eql?(other)   => true or false
 *
 * Two strings are equal if the have the same length and content.
 */
unsafe extern "C" fn mrb_str_eql(mut mrb: *mut mrb_state,
                                 mut self_0: mrb_value) -> mrb_value {
    let mut str2: mrb_value =
        mrb_value{value: unnamed{f: 0.,}, tt: MRB_TT_FALSE,};
    let mut eql_p: mrb_bool = 0;
    mrb_get_args(mrb, b"o\x00" as *const u8 as *const libc::c_char,
                 &mut str2 as *mut mrb_value);
    eql_p =
        (str2.tt as libc::c_uint ==
             MRB_TT_STRING as libc::c_int as libc::c_uint &&
             0 != str_eql(mrb, self_0, str2) as libc::c_int) as libc::c_int as
            mrb_bool;
    return mrb_bool_value(eql_p);
}
/* 15.2.10.5.20 */
/*
 * call-seq:
 *    str.hash   => fixnum
 *
 * Return a hash based on the string's length and content.
 */
unsafe extern "C" fn mrb_str_hash_m(mut mrb: *mut mrb_state,
                                    mut self_0: mrb_value) -> mrb_value {
    let mut key: mrb_int = mrb_str_hash(mrb, self_0) as mrb_int;
    return mrb_fixnum_value(key);
}
/* 15.2.10.5.21 */
/*
 *  call-seq:
 *     str.include? other_str   => true or false
 *     str.include? fixnum      => true or false
 *
 *  Returns <code>true</code> if <i>str</i> contains the given string or
 *  character.
 *
 *     "hello".include? "lo"   #=> true
 *     "hello".include? "ol"   #=> false
 *     "hello".include? ?h     #=> true
 */
unsafe extern "C" fn mrb_str_include(mut mrb: *mut mrb_state,
                                     mut self_0: mrb_value) -> mrb_value {
    let mut str2: mrb_value =
        mrb_value{value: unnamed{f: 0.,}, tt: MRB_TT_FALSE,};
    mrb_get_args(mrb, b"S\x00" as *const u8 as *const libc::c_char,
                 &mut str2 as *mut mrb_value);
    if str_index_str(mrb, self_0, str2, 0i32 as mrb_int) <
           0i32 as libc::c_longlong {
        return mrb_bool_value(0i32 as mrb_bool)
    }
    return mrb_bool_value(1i32 as mrb_bool);
}
/* 15.2.10.5.22 */
/*
 *  call-seq:
 *     str.index(substring [, offset])   => fixnum or nil
 *     str.index(fixnum [, offset])      => fixnum or nil
 *     str.index(regexp [, offset])      => fixnum or nil
 *
 *  Returns the index of the first occurrence of the given
 *  <i>substring</i>,
 *  character (<i>fixnum</i>), or pattern (<i>regexp</i>) in <i>str</i>.
 *  Returns
 *  <code>nil</code> if not found.
 *  If the second parameter is present, it
 *  specifies the position in the string to begin the search.
 *
 *     "hello".index('e')             #=> 1
 *     "hello".index('lo')            #=> 3
 *     "hello".index('a')             #=> nil
 *     "hello".index(101)             #=> 1(101=0x65='e')
 *     "hello".index(/[aeiou]/, -3)   #=> 4
 */
unsafe extern "C" fn mrb_str_index_m(mut mrb: *mut mrb_state,
                                     mut str: mrb_value) -> mrb_value {
    let mut argv: *mut mrb_value = 0 as *mut mrb_value;
    let mut argc: mrb_int = 0;
    let mut sub: mrb_value =
        mrb_value{value: unnamed{f: 0.,}, tt: MRB_TT_FALSE,};
    let mut pos: mrb_int = 0;
    let mut clen: mrb_int = 0;
    mrb_get_args(mrb, b"*!\x00" as *const u8 as *const libc::c_char,
                 &mut argv as *mut *mut mrb_value, &mut argc as *mut mrb_int);
    if argc == 2i32 as libc::c_longlong {
        mrb_get_args(mrb, b"oi\x00" as *const u8 as *const libc::c_char,
                     &mut sub as *mut mrb_value, &mut pos as *mut mrb_int);
    } else {
        pos = 0i32 as mrb_int;
        if argc > 0i32 as libc::c_longlong {
            sub = *argv.offset(0isize)
        } else { sub = mrb_nil_value() }
    }
    clen = utf8_strlen(str);
    if pos < 0i32 as libc::c_longlong {
        pos += clen;
        if pos < 0i32 as libc::c_longlong { return mrb_nil_value() }
    }
    if pos > clen { return mrb_nil_value() }
    pos = chars2bytes(str, 0i32 as mrb_int, pos);
    match sub.tt as libc::c_uint {
        16 => { }
        _ => {
            let mut tmp: mrb_value =
                mrb_value{value: unnamed{f: 0.,}, tt: MRB_TT_FALSE,};
            tmp = mrb_check_string_type(mrb, sub);
            if tmp.tt as libc::c_uint ==
                   MRB_TT_FALSE as libc::c_int as libc::c_uint &&
                   0 == tmp.value.i {
                mrb_raisef(mrb,
                           mrb_exc_get(mrb,
                                       b"TypeError\x00" as *const u8 as
                                           *const libc::c_char),
                           b"type mismatch: %S given\x00" as *const u8 as
                               *const libc::c_char, sub);
            }
            sub = tmp
        }
    }
    pos = str_index_str(mrb, str, sub, pos);
    if pos == -1i32 as libc::c_longlong { return mrb_nil_value() }
    pos =
        bytes2chars(if 0 !=
                           (*(str.value.p as *mut RString)).flags() as
                               libc::c_int & 32i32 {
                        (*(str.value.p as *mut RString)).as_0.ary.as_mut_ptr()
                    } else { (*(str.value.p as *mut RString)).as_0.heap.ptr },
                    pos);
    if pos < 0i32 as libc::c_longlong { return mrb_nil_value() }
    return mrb_fixnum_value(pos);
}
/* 15.2.10.5.24 */
/* 15.2.10.5.28 */
/*
 *  call-seq:
 *     str.replace(other_str)   => str
 *
 *     s = "hello"         #=> "hello"
 *     s.replace "world"   #=> "world"
 */
unsafe extern "C" fn mrb_str_replace(mut mrb: *mut mrb_state,
                                     mut str: mrb_value) -> mrb_value {
    let mut str2: mrb_value =
        mrb_value{value: unnamed{f: 0.,}, tt: MRB_TT_FALSE,};
    mrb_get_args(mrb, b"S\x00" as *const u8 as *const libc::c_char,
                 &mut str2 as *mut mrb_value);
    return str_replace(mrb, str.value.p as *mut RString,
                       str2.value.p as *mut RString);
}
/* 15.2.10.5.23 */
/*
 *  call-seq:
 *     String.new(str="")   => new_str
 *
 *  Returns a new string object containing a copy of <i>str</i>.
 */
unsafe extern "C" fn mrb_str_init(mut mrb: *mut mrb_state,
                                  mut self_0: mrb_value) -> mrb_value {
    let mut str2: mrb_value =
        mrb_value{value: unnamed{f: 0.,}, tt: MRB_TT_FALSE,};
    if mrb_get_args(mrb, b"|S\x00" as *const u8 as *const libc::c_char,
                    &mut str2 as *mut mrb_value) == 0i32 as libc::c_longlong {
        let mut s: *mut RString =
            str_new(mrb, 0 as *const libc::c_char, 0i32 as size_t);
        str2 = mrb_obj_value(s as *mut libc::c_void)
    }
    str_replace(mrb, self_0.value.p as *mut RString,
                str2.value.p as *mut RString);
    return self_0;
}
/* 15.2.10.5.30 */
/*
 *  call-seq:
 *     str.reverse!   => str
 *
 *  Reverses <i>str</i> in place.
 */
unsafe extern "C" fn mrb_str_reverse_bang(mut mrb: *mut mrb_state,
                                          mut str: mrb_value) -> mrb_value {
    let mut utf8_len: mrb_int = utf8_strlen(str);
    let mut len: mrb_int =
        if 0 !=
               (*(str.value.p as *mut RString)).flags() as libc::c_int & 32i32
           {
            (((*(str.value.p as *mut RString)).flags() as libc::c_int &
                  0x7c0i32) >> 6i32) as mrb_int
        } else { (*(str.value.p as *mut RString)).as_0.heap.len };
    if utf8_len == len {
        let mut s: *mut RString = str.value.p as *mut RString;
        let mut p_0: *mut libc::c_char = 0 as *mut libc::c_char;
        let mut e_0: *mut libc::c_char = 0 as *mut libc::c_char;
        let mut c: libc::c_char = 0;
        mrb_str_modify(mrb, s);
        if if 0 != (*s).flags() as libc::c_int & 32i32 {
               (((*s).flags() as libc::c_int & 0x7c0i32) >> 6i32) as mrb_int
           } else { (*s).as_0.heap.len } > 1i32 as libc::c_longlong {
            p_0 =
                if 0 != (*s).flags() as libc::c_int & 32i32 {
                    (*s).as_0.ary.as_mut_ptr()
                } else { (*s).as_0.heap.ptr };
            e_0 =
                p_0.offset((if 0 != (*s).flags() as libc::c_int & 32i32 {
                                (((*s).flags() as libc::c_int & 0x7c0i32) >>
                                     6i32) as mrb_int
                            } else { (*s).as_0.heap.len }) as
                               isize).offset(-1isize);
            while p_0 < e_0 {
                c = *p_0;
                let fresh37 = p_0;
                p_0 = p_0.offset(1);
                *fresh37 = *e_0;
                let fresh38 = e_0;
                e_0 = e_0.offset(-1);
                *fresh38 = c
            }
        }
        return str
    } else {
        if utf8_len > 1i32 as libc::c_longlong {
            let mut buf: *mut libc::c_char = 0 as *mut libc::c_char;
            let mut p: *mut libc::c_char = 0 as *mut libc::c_char;
            let mut e: *mut libc::c_char = 0 as *mut libc::c_char;
            let mut r: *mut libc::c_char = 0 as *mut libc::c_char;
            mrb_str_modify(mrb, str.value.p as *mut RString);
            len =
                if 0 !=
                       (*(str.value.p as *mut RString)).flags() as libc::c_int
                           & 32i32 {
                    (((*(str.value.p as *mut RString)).flags() as libc::c_int
                          & 0x7c0i32) >> 6i32) as mrb_int
                } else { (*(str.value.p as *mut RString)).as_0.heap.len };
            buf = mrb_malloc(mrb, len as size_t) as *mut libc::c_char;
            p = buf;
            e = buf.offset(len as isize);
            memcpy(buf as *mut libc::c_void,
                   (if 0 !=
                           (*(str.value.p as *mut RString)).flags() as
                               libc::c_int & 32i32 {
                        (*(str.value.p as *mut RString)).as_0.ary.as_mut_ptr()
                    } else { (*(str.value.p as *mut RString)).as_0.heap.ptr })
                       as *const libc::c_void, len as libc::c_ulong);
            r =
                if 0 !=
                       (*(str.value.p as *mut RString)).flags() as libc::c_int
                           & 32i32 {
                    (*(str.value.p as *mut RString)).as_0.ary.as_mut_ptr()
                } else {
                    (*(str.value.p as *mut RString)).as_0.heap.ptr
                }.offset(len as isize);
            while p < e {
                let mut clen: mrb_int = utf8len(p, e);
                r = r.offset(-(clen as isize));
                memcpy(r as *mut libc::c_void, p as *const libc::c_void,
                       clen as libc::c_ulong);
                p = p.offset(clen as isize)
            }
            mrb_free(mrb, buf as *mut libc::c_void);
        }
        return str
    };
}
/* ---------------------------------- */
/* 15.2.10.5.29 */
/*
 *  call-seq:
 *     str.reverse   => new_str
 *
 *  Returns a new string with the characters from <i>str</i> in reverse order.
 *
 *     "stressed".reverse   #=> "desserts"
 */
unsafe extern "C" fn mrb_str_reverse(mut mrb: *mut mrb_state,
                                     mut str: mrb_value) -> mrb_value {
    let mut str2: mrb_value = mrb_str_dup(mrb, str);
    mrb_str_reverse_bang(mrb, str2);
    return str2;
}
/* 15.2.10.5.31 */
/*
 *  call-seq:
 *     str.rindex(substring [, fixnum])   => fixnum or nil
 *     str.rindex(fixnum [, fixnum])   => fixnum or nil
 *     str.rindex(regexp [, fixnum])   => fixnum or nil
 *
 *  Returns the index of the last occurrence of the given <i>substring</i>,
 *  character (<i>fixnum</i>), or pattern (<i>regexp</i>) in <i>str</i>. Returns
 *  <code>nil</code> if not found. If the second parameter is present, it
 *  specifies the position in the string to end the search---characters beyond
 *  this point will not be considered.
 *
 *     "hello".rindex('e')             #=> 1
 *     "hello".rindex('l')             #=> 3
 *     "hello".rindex('a')             #=> nil
 *     "hello".rindex(101)             #=> 1
 *     "hello".rindex(/[aeiou]/, -2)   #=> 1
 */
unsafe extern "C" fn mrb_str_rindex(mut mrb: *mut mrb_state,
                                    mut str: mrb_value) -> mrb_value {
    let mut argv: *mut mrb_value = 0 as *mut mrb_value;
    let mut argc: mrb_int = 0;
    let mut sub: mrb_value =
        mrb_value{value: unnamed{f: 0.,}, tt: MRB_TT_FALSE,};
    let mut pos: mrb_int = 0;
    let mut len: mrb_int = utf8_strlen(str);
    mrb_get_args(mrb, b"*!\x00" as *const u8 as *const libc::c_char,
                 &mut argv as *mut *mut mrb_value, &mut argc as *mut mrb_int);
    if argc == 2i32 as libc::c_longlong {
        mrb_get_args(mrb, b"oi\x00" as *const u8 as *const libc::c_char,
                     &mut sub as *mut mrb_value, &mut pos as *mut mrb_int);
        if pos < 0i32 as libc::c_longlong {
            pos += len;
            if pos < 0i32 as libc::c_longlong { return mrb_nil_value() }
        }
        if pos > len { pos = len }
    } else {
        pos = len;
        if argc > 0i32 as libc::c_longlong {
            sub = *argv.offset(0isize)
        } else { sub = mrb_nil_value() }
    }
    pos = chars2bytes(str, 0i32 as mrb_int, pos);
    match sub.tt as libc::c_uint {
        16 => { }
        _ => {
            let mut tmp: mrb_value =
                mrb_value{value: unnamed{f: 0.,}, tt: MRB_TT_FALSE,};
            tmp = mrb_check_string_type(mrb, sub);
            if tmp.tt as libc::c_uint ==
                   MRB_TT_FALSE as libc::c_int as libc::c_uint &&
                   0 == tmp.value.i {
                mrb_raisef(mrb,
                           mrb_exc_get(mrb,
                                       b"TypeError\x00" as *const u8 as
                                           *const libc::c_char),
                           b"type mismatch: %S given\x00" as *const u8 as
                               *const libc::c_char, sub);
            }
            sub = tmp
        }
    }
    pos = str_rindex(mrb, str, sub, pos);
    if pos >= 0i32 as libc::c_longlong {
        pos =
            bytes2chars(if 0 !=
                               (*(str.value.p as *mut RString)).flags() as
                                   libc::c_int & 32i32 {
                            (*(str.value.p as
                                   *mut RString)).as_0.ary.as_mut_ptr()
                        } else {
                            (*(str.value.p as *mut RString)).as_0.heap.ptr
                        }, pos);
        if pos < 0i32 as libc::c_longlong { return mrb_nil_value() }
        return mrb_fixnum_value(pos)
    }
    return mrb_nil_value();
}
/* 15.2.10.5.35 */
/*
 *  call-seq:
 *     str.split(pattern="\n", [limit])   => anArray
 *
 *  Divides <i>str</i> into substrings based on a delimiter, returning an array
 *  of these substrings.
 *
 *  If <i>pattern</i> is a <code>String</code>, then its contents are used as
 *  the delimiter when splitting <i>str</i>. If <i>pattern</i> is a single
 *  space, <i>str</i> is split on whitespace, with leading whitespace and runs
 *  of contiguous whitespace characters ignored.
 *
 *  If <i>pattern</i> is a <code>Regexp</code>, <i>str</i> is divided where the
 *  pattern matches. Whenever the pattern matches a zero-length string,
 *  <i>str</i> is split into individual characters.
 *
 *  If <i>pattern</i> is omitted, the value of <code>$;</code> is used.  If
 *  <code>$;</code> is <code>nil</code> (which is the default), <i>str</i> is
 *  split on whitespace as if ' ' were specified.
 *
 *  If the <i>limit</i> parameter is omitted, trailing null fields are
 *  suppressed. If <i>limit</i> is a positive number, at most that number of
 *  fields will be returned (if <i>limit</i> is <code>1</code>, the entire
 *  string is returned as the only entry in an array). If negative, there is no
 *  limit to the number of fields returned, and trailing null fields are not
 *  suppressed.
 *
 *     " now's  the time".split        #=> ["now's", "the", "time"]
 *     " now's  the time".split(' ')   #=> ["now's", "the", "time"]
 *     " now's  the time".split(/ /)   #=> ["", "now's", "", "the", "time"]
 *     "hello".split(//)               #=> ["h", "e", "l", "l", "o"]
 *     "hello".split(//, 3)            #=> ["h", "e", "llo"]
 *
 *     "mellow yellow".split("ello")   #=> ["m", "w y", "w"]
 *     "1,2,,3,4,,".split(',')         #=> ["1", "2", "", "3", "4"]
 *     "1,2,,3,4,,".split(',', 4)      #=> ["1", "2", "", "3,4,,"]
 *     "1,2,,3,4,,".split(',', -4)     #=> ["1", "2", "", "3", "4", "", ""]
 */
unsafe extern "C" fn mrb_str_split_m(mut mrb: *mut mrb_state,
                                     mut str: mrb_value) -> mrb_value {
    let mut argc: mrb_int = 0;
    let mut spat: mrb_value = mrb_nil_value();
    let mut split_type: unnamed_7 = string;
    let mut i: mrb_int = 0i32 as mrb_int;
    let mut beg: mrb_int = 0;
    let mut end: mrb_int = 0;
    let mut lim: mrb_int = 0i32 as mrb_int;
    let mut lim_p: mrb_bool = 0;
    let mut result: mrb_value =
        mrb_value{value: unnamed{f: 0.,}, tt: MRB_TT_FALSE,};
    let mut tmp: mrb_value =
        mrb_value{value: unnamed{f: 0.,}, tt: MRB_TT_FALSE,};
    argc =
        mrb_get_args(mrb, b"|oi\x00" as *const u8 as *const libc::c_char,
                     &mut spat as *mut mrb_value, &mut lim as *mut mrb_int);
    lim_p =
        (lim > 0i32 as libc::c_longlong && argc == 2i32 as libc::c_longlong)
            as libc::c_int as mrb_bool;
    if argc == 2i32 as libc::c_longlong {
        if lim == 1i32 as libc::c_longlong {
            if if 0 !=
                      (*(str.value.p as *mut RString)).flags() as libc::c_int
                          & 32i32 {
                   (((*(str.value.p as *mut RString)).flags() as libc::c_int &
                         0x7c0i32) >> 6i32) as mrb_int
               } else { (*(str.value.p as *mut RString)).as_0.heap.len } ==
                   0i32 as libc::c_longlong {
                return mrb_ary_new_capa(mrb, 0i32 as mrb_int)
            }
            return mrb_ary_new_from_values(mrb, 1i32 as mrb_int, &mut str)
        }
        i = 1i32 as mrb_int
    }
    if argc == 0i32 as libc::c_longlong ||
           spat.tt as libc::c_uint ==
               MRB_TT_FALSE as libc::c_int as libc::c_uint &&
               0 == spat.value.i {
        split_type = awk
    } else if !(spat.tt as libc::c_uint ==
                    MRB_TT_STRING as libc::c_int as libc::c_uint) {
        mrb_raise(mrb,
                  mrb_exc_get(mrb,
                              b"TypeError\x00" as *const u8 as
                                  *const libc::c_char),
                  b"expected String\x00" as *const u8 as *const libc::c_char);
    } else {
        if if 0 !=
                  (*(spat.value.p as *mut RString)).flags() as libc::c_int &
                      32i32 {
               (((*(spat.value.p as *mut RString)).flags() as libc::c_int &
                     0x7c0i32) >> 6i32) as mrb_int
           } else { (*(spat.value.p as *mut RString)).as_0.heap.len } ==
               1i32 as libc::c_longlong &&
               *if 0 !=
                       (*(spat.value.p as *mut RString)).flags() as
                           libc::c_int & 32i32 {
                    (*(spat.value.p as *mut RString)).as_0.ary.as_mut_ptr()
                } else {
                    (*(spat.value.p as *mut RString)).as_0.heap.ptr
                }.offset(0isize) as libc::c_int == ' ' as i32 {
            split_type = awk
        }
    }
    result = mrb_ary_new(mrb);
    beg = 0i32 as mrb_int;
    if split_type as libc::c_uint == awk as libc::c_int as libc::c_uint {
        let mut skip: mrb_bool = 1i32 as mrb_bool;
        let mut idx: mrb_int = 0i32 as mrb_int;
        let mut str_len: mrb_int =
            if 0 !=
                   (*(str.value.p as *mut RString)).flags() as libc::c_int &
                       32i32 {
                (((*(str.value.p as *mut RString)).flags() as libc::c_int &
                      0x7c0i32) >> 6i32) as mrb_int
            } else { (*(str.value.p as *mut RString)).as_0.heap.len };
        let mut c: libc::c_uint = 0;
        let mut ai: libc::c_int = mrb_gc_arena_save(mrb);
        end = beg;
        idx = end;
        while idx < str_len {
            let fresh39 = idx;
            idx = idx + 1;
            c =
                *if 0 !=
                        (*(str.value.p as *mut RString)).flags() as
                            libc::c_int & 32i32 {
                     (*(str.value.p as *mut RString)).as_0.ary.as_mut_ptr()
                 } else {
                     (*(str.value.p as *mut RString)).as_0.heap.ptr
                 }.offset(fresh39 as isize) as libc::c_uchar as libc::c_uint;
            if 0 != skip {
                if c == ' ' as i32 as libc::c_uint ||
                       (c as
                            libc::c_uint).wrapping_sub('\t' as i32 as
                                                           libc::c_uint) <
                           5i32 as libc::c_uint {
                    beg = idx
                } else {
                    end = idx;
                    skip = 0i32 as mrb_bool;
                    if 0 != lim_p as libc::c_int && lim <= i { break ; }
                }
            } else if c == ' ' as i32 as libc::c_uint ||
                          (c as
                               libc::c_uint).wrapping_sub('\t' as i32 as
                                                              libc::c_uint) <
                              5i32 as libc::c_uint {
                mrb_ary_push(mrb, result,
                             byte_subseq(mrb, str, beg, end - beg));
                mrb_gc_arena_restore(mrb, ai);
                skip = 1i32 as mrb_bool;
                beg = idx;
                if 0 != lim_p { i += 1 }
            } else { end = idx }
        }
    } else {
        let mut str_len_0: mrb_int =
            if 0 !=
                   (*(str.value.p as *mut RString)).flags() as libc::c_int &
                       32i32 {
                (((*(str.value.p as *mut RString)).flags() as libc::c_int &
                      0x7c0i32) >> 6i32) as mrb_int
            } else { (*(str.value.p as *mut RString)).as_0.heap.len };
        let mut pat_len: mrb_int =
            if 0 !=
                   (*(spat.value.p as *mut RString)).flags() as libc::c_int &
                       32i32 {
                (((*(spat.value.p as *mut RString)).flags() as libc::c_int &
                      0x7c0i32) >> 6i32) as mrb_int
            } else { (*(spat.value.p as *mut RString)).as_0.heap.len };
        let mut idx_0: mrb_int = 0i32 as mrb_int;
        let mut ai_0: libc::c_int = mrb_gc_arena_save(mrb);
        while idx_0 < str_len_0 {
            if pat_len > 0i32 as libc::c_longlong {
                end =
                    mrb_memsearch((if 0 !=
                                          (*(spat.value.p as
                                                 *mut RString)).flags() as
                                              libc::c_int & 32i32 {
                                       (*(spat.value.p as
                                              *mut RString)).as_0.ary.as_mut_ptr()
                                   } else {
                                       (*(spat.value.p as
                                              *mut RString)).as_0.heap.ptr
                                   }) as *const libc::c_void, pat_len,
                                  if 0 !=
                                         (*(str.value.p as
                                                *mut RString)).flags() as
                                             libc::c_int & 32i32 {
                                      (*(str.value.p as
                                             *mut RString)).as_0.ary.as_mut_ptr()
                                  } else {
                                      (*(str.value.p as
                                             *mut RString)).as_0.heap.ptr
                                  }.offset(idx_0 as isize) as
                                      *const libc::c_void, str_len_0 - idx_0);
                if end < 0i32 as libc::c_longlong { break ; }
            } else { end = chars2bytes(str, idx_0, 1i32 as mrb_int) }
            mrb_ary_push(mrb, result, byte_subseq(mrb, str, idx_0, end));
            mrb_gc_arena_restore(mrb, ai_0);
            idx_0 += end + pat_len;
            if 0 != lim_p as libc::c_int && { i += 1; lim <= i } { break ; }
        }
        beg = idx_0
    }
    if if 0 != (*(str.value.p as *mut RString)).flags() as libc::c_int & 32i32
          {
           (((*(str.value.p as *mut RString)).flags() as libc::c_int &
                 0x7c0i32) >> 6i32) as mrb_int
       } else { (*(str.value.p as *mut RString)).as_0.heap.len } >
           0i32 as libc::c_longlong &&
           (0 != lim_p as libc::c_int ||
                if 0 !=
                       (*(str.value.p as *mut RString)).flags() as libc::c_int
                           & 32i32 {
                    (((*(str.value.p as *mut RString)).flags() as libc::c_int
                          & 0x7c0i32) >> 6i32) as mrb_int
                } else { (*(str.value.p as *mut RString)).as_0.heap.len } >
                    beg || lim < 0i32 as libc::c_longlong) {
        if if 0 !=
                  (*(str.value.p as *mut RString)).flags() as libc::c_int &
                      32i32 {
               (((*(str.value.p as *mut RString)).flags() as libc::c_int &
                     0x7c0i32) >> 6i32) as mrb_int
           } else { (*(str.value.p as *mut RString)).as_0.heap.len } == beg {
            tmp = mrb_str_new_empty(mrb, str)
        } else {
            tmp =
                byte_subseq(mrb, str, beg,
                            if 0 !=
                                   (*(str.value.p as *mut RString)).flags() as
                                       libc::c_int & 32i32 {
                                (((*(str.value.p as *mut RString)).flags() as
                                      libc::c_int & 0x7c0i32) >> 6i32) as
                                    mrb_int
                            } else {
                                (*(str.value.p as *mut RString)).as_0.heap.len
                            } - beg)
        }
        mrb_ary_push(mrb, result, tmp);
    }
    if 0 == lim_p && lim == 0i32 as libc::c_longlong {
        let mut len: mrb_int = 0;
        loop  {
            len =
                if 0 !=
                       (*(result.value.p as *mut RArray)).flags() as
                           libc::c_int & 7i32 {
                    (((*(result.value.p as *mut RArray)).flags() as
                          libc::c_int & 7i32) - 1i32) as mrb_int
                } else { (*(result.value.p as *mut RArray)).as_0.heap.len };
            if !(len > 0i32 as libc::c_longlong &&
                     {
                         tmp =
                             *if 0 !=
                                     (*(result.value.p as
                                            *mut RArray)).flags() as
                                         libc::c_int & 7i32 {
                                  &mut (*(result.value.p as *mut RArray)).as_0
                                      as *mut unnamed_3 as *mut mrb_value
                              } else {
                                  (*(result.value.p as
                                         *mut RArray)).as_0.heap.ptr
                              }.offset((len - 1i32 as libc::c_longlong) as
                                           isize);
                         if 0 !=
                                (*(tmp.value.p as *mut RString)).flags() as
                                    libc::c_int & 32i32 {
                             (((*(tmp.value.p as *mut RString)).flags() as
                                   libc::c_int & 0x7c0i32) >> 6i32) as mrb_int
                         } else {
                             (*(tmp.value.p as *mut RString)).as_0.heap.len
                         } == 0i32 as libc::c_longlong
                     }) {
                break ;
            }
            mrb_ary_pop(mrb, result);
        }
    }
    return result;
}
/* 15.2.10.5.38 */
/*
 *  call-seq:
 *     str.to_i(base=10)   => integer
 *
 *  Returns the result of interpreting leading characters in <i>str</i> as an
 *  integer base <i>base</i> (between 2 and 36). Extraneous characters past the
 *  end of a valid number are ignored. If there is not a valid number at the
 *  start of <i>str</i>, <code>0</code> is returned. This method never raises an
 *  exception.
 *
 *     "12345".to_i             #=> 12345
 *     "99 red balloons".to_i   #=> 99
 *     "0a".to_i                #=> 0
 *     "0a".to_i(16)            #=> 10
 *     "hello".to_i             #=> 0
 *     "1100101".to_i(2)        #=> 101
 *     "1100101".to_i(8)        #=> 294977
 *     "1100101".to_i(10)       #=> 1100101
 *     "1100101".to_i(16)       #=> 17826049
 */
unsafe extern "C" fn mrb_str_to_i(mut mrb: *mut mrb_state,
                                  mut self_0: mrb_value) -> mrb_value {
    let mut base: mrb_int = 10i32 as mrb_int;
    mrb_get_args(mrb, b"|i\x00" as *const u8 as *const libc::c_char,
                 &mut base as *mut mrb_int);
    if base < 0i32 as libc::c_longlong {
        mrb_raisef(mrb,
                   mrb_exc_get(mrb,
                               b"ArgumentError\x00" as *const u8 as
                                   *const libc::c_char),
                   b"illegal radix %S\x00" as *const u8 as
                       *const libc::c_char, mrb_fixnum_value(base));
    }
    return mrb_str_to_inum(mrb, self_0, base, 0i32 as mrb_bool);
}
/* 15.2.10.5.39 */
/*
 *  call-seq:
 *     str.to_f   => float
 *
 *  Returns the result of interpreting leading characters in <i>str</i> as a
 *  floating point number. Extraneous characters past the end of a valid number
 *  are ignored. If there is not a valid number at the start of <i>str</i>,
 *  <code>0.0</code> is returned. This method never raises an exception.
 *
 *     "123.45e1".to_f        #=> 1234.5
 *     "45.67 degrees".to_f   #=> 45.67
 *     "thx1138".to_f         #=> 0.0
 */
unsafe extern "C" fn mrb_str_to_f(mut mrb: *mut mrb_state,
                                  mut self_0: mrb_value) -> mrb_value {
    return mrb_float_value(mrb,
                           mrb_str_to_dbl(mrb, self_0, 0i32 as mrb_bool));
}
/* 15.2.10.5.40 */
/*
 *  call-seq:
 *     str.to_s     => str
 *
 *  Returns the receiver.
 */
unsafe extern "C" fn mrb_str_to_s(mut mrb: *mut mrb_state,
                                  mut self_0: mrb_value) -> mrb_value {
    if mrb_obj_class(mrb, self_0) != (*mrb).string_class {
        return mrb_str_dup(mrb, self_0)
    }
    return self_0;
}
/* 15.2.10.5.43 */
/*
 *  call-seq:
 *     str.upcase!   => str or nil
 *
 *  Upcases the contents of <i>str</i>, returning <code>nil</code> if no changes
 *  were made.
 */
unsafe extern "C" fn mrb_str_upcase_bang(mut mrb: *mut mrb_state,
                                         mut str: mrb_value) -> mrb_value {
    let mut s: *mut RString = str.value.p as *mut RString;
    let mut p: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut pend: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut modify: mrb_bool = 0i32 as mrb_bool;
    mrb_str_modify(mrb, s);
    p =
        if 0 !=
               (*(str.value.p as *mut RString)).flags() as libc::c_int & 32i32
           {
            (*(str.value.p as *mut RString)).as_0.ary.as_mut_ptr()
        } else { (*(str.value.p as *mut RString)).as_0.heap.ptr };
    pend =
        if 0 !=
               (*(str.value.p as *mut RString)).flags() as libc::c_int & 32i32
           {
            (*(str.value.p as *mut RString)).as_0.ary.as_mut_ptr()
        } else {
            (*(str.value.p as *mut RString)).as_0.heap.ptr
        }.offset((if 0 !=
                         (*(str.value.p as *mut RString)).flags() as
                             libc::c_int & 32i32 {
                      (((*(str.value.p as *mut RString)).flags() as
                            libc::c_int & 0x7c0i32) >> 6i32) as mrb_int
                  } else { (*(str.value.p as *mut RString)).as_0.heap.len })
                     as isize);
    while p < pend {
        if (*p as libc::c_uint).wrapping_sub('a' as i32 as libc::c_uint) <
               26i32 as libc::c_uint {
            *p =
                (if (*p as
                         libc::c_uint).wrapping_sub('a' as i32 as
                                                        libc::c_uint) <
                        26i32 as libc::c_uint {
                     *p as libc::c_int & 0x5fi32
                 } else { *p as libc::c_int }) as libc::c_char;
            modify = 1i32 as mrb_bool
        }
        p = p.offset(1isize)
    }
    if 0 != modify { return str }
    return mrb_nil_value();
}
/* 15.2.10.5.42 */
/*
 *  call-seq:
 *     str.upcase   => new_str
 *
 *  Returns a copy of <i>str</i> with all lowercase letters replaced with their
 *  uppercase counterparts. The operation is locale insensitive---only
 *  characters 'a' to 'z' are affected.
 *
 *     "hEllO".upcase   #=> "HELLO"
 */
unsafe extern "C" fn mrb_str_upcase(mut mrb: *mut mrb_state,
                                    mut self_0: mrb_value) -> mrb_value {
    let mut str: mrb_value =
        mrb_value{value: unnamed{f: 0.,}, tt: MRB_TT_FALSE,};
    str = mrb_str_dup(mrb, self_0);
    mrb_str_upcase_bang(mrb, str);
    return str;
}
/*
 * call-seq:
 *   str.bytes   -> array of fixnums
 *
 * Returns an array of bytes in _str_.
 *
 *    str = "hello"
 *    str.bytes       #=> [104, 101, 108, 108, 111]
 */
unsafe extern "C" fn mrb_str_bytes(mut mrb: *mut mrb_state,
                                   mut str: mrb_value) -> mrb_value {
    let mut s: *mut RString = str.value.p as *mut RString;
    let mut a: mrb_value =
        mrb_ary_new_capa(mrb,
                         if 0 != (*s).flags() as libc::c_int & 32i32 {
                             (((*s).flags() as libc::c_int & 0x7c0i32) >>
                                  6i32) as mrb_int
                         } else { (*s).as_0.heap.len });
    let mut p: *mut libc::c_uchar =
        (if 0 != (*s).flags() as libc::c_int & 32i32 {
             (*s).as_0.ary.as_mut_ptr()
         } else { (*s).as_0.heap.ptr }) as *mut libc::c_uchar;
    let mut pend: *mut libc::c_uchar =
        p.offset((if 0 != (*s).flags() as libc::c_int & 32i32 {
                      (((*s).flags() as libc::c_int & 0x7c0i32) >> 6i32) as
                          mrb_int
                  } else { (*s).as_0.heap.len }) as isize);
    while p < pend {
        mrb_ary_push(mrb, a, mrb_fixnum_value(*p.offset(0isize) as mrb_int));
        p = p.offset(1isize)
    }
    return a;
}
/* ---------------------------*/
#[no_mangle]
pub unsafe extern "C" fn mrb_init_string(mut mrb: *mut mrb_state) {
    let mut s: *mut RClass = 0 as *mut RClass;
    if 0 !=
           !(((::std::mem::size_of::<*mut libc::c_void>() as
                   libc::c_ulong).wrapping_mul(3i32 as
                                                   libc::c_ulong).wrapping_sub(1i32
                                                                                   as
                                                                                   libc::c_ulong)
                  as mrb_int) < (1i32 << 5i32) as libc::c_longlong) as
               libc::c_int as libc::c_long {
        __assert_rtn((*::std::mem::transmute::<&[u8; 16],
                                               &[libc::c_char; 16]>(b"mrb_init_string\x00")).as_ptr(),
                     b"src/string.c\x00" as *const u8 as *const libc::c_char,
                     2669i32,
                     b"((mrb_int)(sizeof(void*) * 3 - 1)) < (1 << 5)\x00" as
                         *const u8 as *const libc::c_char);
    } else { };
    s =
        mrb_define_class(mrb,
                         b"String\x00" as *const u8 as *const libc::c_char,
                         (*mrb).object_class);
    (*mrb).string_class = s;
    (*s).set_flags(((*s).flags() as libc::c_int & !0xffi32 |
                        MRB_TT_STRING as libc::c_int as libc::c_char as
                            libc::c_int) as uint32_t);
    mrb_define_method(mrb, s,
                      b"bytesize\x00" as *const u8 as *const libc::c_char,
                      Some(mrb_str_bytesize), 0i32 as mrb_aspec);
    mrb_define_method(mrb, s, b"<=>\x00" as *const u8 as *const libc::c_char,
                      Some(mrb_str_cmp_m),
                      ((1i32 & 0x1fi32) as mrb_aspec) << 18i32);
    mrb_define_method(mrb, s, b"==\x00" as *const u8 as *const libc::c_char,
                      Some(mrb_str_equal_m),
                      ((1i32 & 0x1fi32) as mrb_aspec) << 18i32);
    mrb_define_method(mrb, s, b"+\x00" as *const u8 as *const libc::c_char,
                      Some(mrb_str_plus_m),
                      ((1i32 & 0x1fi32) as mrb_aspec) << 18i32);
    mrb_define_method(mrb, s, b"*\x00" as *const u8 as *const libc::c_char,
                      Some(mrb_str_times),
                      ((1i32 & 0x1fi32) as mrb_aspec) << 18i32);
    mrb_define_method(mrb, s, b"[]\x00" as *const u8 as *const libc::c_char,
                      Some(mrb_str_aref_m), (1i32 << 12i32) as mrb_aspec);
    mrb_define_method(mrb, s,
                      b"capitalize\x00" as *const u8 as *const libc::c_char,
                      Some(mrb_str_capitalize), 0i32 as mrb_aspec);
    mrb_define_method(mrb, s,
                      b"capitalize!\x00" as *const u8 as *const libc::c_char,
                      Some(mrb_str_capitalize_bang), 0i32 as mrb_aspec);
    mrb_define_method(mrb, s,
                      b"chomp\x00" as *const u8 as *const libc::c_char,
                      Some(mrb_str_chomp), (1i32 << 12i32) as mrb_aspec);
    mrb_define_method(mrb, s,
                      b"chomp!\x00" as *const u8 as *const libc::c_char,
                      Some(mrb_str_chomp_bang), (1i32 << 12i32) as mrb_aspec);
    mrb_define_method(mrb, s, b"chop\x00" as *const u8 as *const libc::c_char,
                      Some(mrb_str_chop), 0i32 as mrb_aspec);
    mrb_define_method(mrb, s,
                      b"chop!\x00" as *const u8 as *const libc::c_char,
                      Some(mrb_str_chop_bang), 0i32 as mrb_aspec);
    mrb_define_method(mrb, s,
                      b"downcase\x00" as *const u8 as *const libc::c_char,
                      Some(mrb_str_downcase), 0i32 as mrb_aspec);
    mrb_define_method(mrb, s,
                      b"downcase!\x00" as *const u8 as *const libc::c_char,
                      Some(mrb_str_downcase_bang), 0i32 as mrb_aspec);
    mrb_define_method(mrb, s,
                      b"empty?\x00" as *const u8 as *const libc::c_char,
                      Some(mrb_str_empty_p), 0i32 as mrb_aspec);
    mrb_define_method(mrb, s, b"eql?\x00" as *const u8 as *const libc::c_char,
                      Some(mrb_str_eql),
                      ((1i32 & 0x1fi32) as mrb_aspec) << 18i32);
    mrb_define_method(mrb, s, b"hash\x00" as *const u8 as *const libc::c_char,
                      Some(mrb_str_hash_m), 0i32 as mrb_aspec);
    mrb_define_method(mrb, s,
                      b"include?\x00" as *const u8 as *const libc::c_char,
                      Some(mrb_str_include),
                      ((1i32 & 0x1fi32) as mrb_aspec) << 18i32);
    mrb_define_method(mrb, s,
                      b"index\x00" as *const u8 as *const libc::c_char,
                      Some(mrb_str_index_m), (1i32 << 12i32) as mrb_aspec);
    mrb_define_method(mrb, s,
                      b"initialize\x00" as *const u8 as *const libc::c_char,
                      Some(mrb_str_init),
                      ((1i32 & 0x1fi32) as mrb_aspec) << 18i32);
    mrb_define_method(mrb, s,
                      b"initialize_copy\x00" as *const u8 as
                          *const libc::c_char, Some(mrb_str_replace),
                      ((1i32 & 0x1fi32) as mrb_aspec) << 18i32);
    mrb_define_method(mrb, s,
                      b"intern\x00" as *const u8 as *const libc::c_char,
                      Some(mrb_str_intern), 0i32 as mrb_aspec);
    mrb_define_method(mrb, s,
                      b"length\x00" as *const u8 as *const libc::c_char,
                      Some(mrb_str_size), 0i32 as mrb_aspec);
    mrb_define_method(mrb, s,
                      b"replace\x00" as *const u8 as *const libc::c_char,
                      Some(mrb_str_replace),
                      ((1i32 & 0x1fi32) as mrb_aspec) << 18i32);
    mrb_define_method(mrb, s,
                      b"reverse\x00" as *const u8 as *const libc::c_char,
                      Some(mrb_str_reverse), 0i32 as mrb_aspec);
    mrb_define_method(mrb, s,
                      b"reverse!\x00" as *const u8 as *const libc::c_char,
                      Some(mrb_str_reverse_bang), 0i32 as mrb_aspec);
    mrb_define_method(mrb, s,
                      b"rindex\x00" as *const u8 as *const libc::c_char,
                      Some(mrb_str_rindex), (1i32 << 12i32) as mrb_aspec);
    mrb_define_method(mrb, s, b"size\x00" as *const u8 as *const libc::c_char,
                      Some(mrb_str_size), 0i32 as mrb_aspec);
    mrb_define_method(mrb, s,
                      b"slice\x00" as *const u8 as *const libc::c_char,
                      Some(mrb_str_aref_m), (1i32 << 12i32) as mrb_aspec);
    mrb_define_method(mrb, s,
                      b"split\x00" as *const u8 as *const libc::c_char,
                      Some(mrb_str_split_m), (1i32 << 12i32) as mrb_aspec);
    mrb_define_method(mrb, s, b"to_f\x00" as *const u8 as *const libc::c_char,
                      Some(mrb_str_to_f), 0i32 as mrb_aspec);
    mrb_define_method(mrb, s, b"to_i\x00" as *const u8 as *const libc::c_char,
                      Some(mrb_str_to_i), (1i32 << 12i32) as mrb_aspec);
    mrb_define_method(mrb, s, b"to_s\x00" as *const u8 as *const libc::c_char,
                      Some(mrb_str_to_s), 0i32 as mrb_aspec);
    mrb_define_method(mrb, s,
                      b"to_str\x00" as *const u8 as *const libc::c_char,
                      Some(mrb_str_to_s), 0i32 as mrb_aspec);
    mrb_define_method(mrb, s,
                      b"to_sym\x00" as *const u8 as *const libc::c_char,
                      Some(mrb_str_intern), 0i32 as mrb_aspec);
    mrb_define_method(mrb, s,
                      b"upcase\x00" as *const u8 as *const libc::c_char,
                      Some(mrb_str_upcase), 0i32 as mrb_aspec);
    mrb_define_method(mrb, s,
                      b"upcase!\x00" as *const u8 as *const libc::c_char,
                      Some(mrb_str_upcase_bang), 0i32 as mrb_aspec);
    mrb_define_method(mrb, s,
                      b"inspect\x00" as *const u8 as *const libc::c_char,
                      Some(mrb_str_inspect), 0i32 as mrb_aspec);
    mrb_define_method(mrb, s,
                      b"bytes\x00" as *const u8 as *const libc::c_char,
                      Some(mrb_str_bytes), 0i32 as mrb_aspec);
}