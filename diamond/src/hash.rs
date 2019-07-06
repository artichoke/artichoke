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
    pub type mrb_shared_string;
    #[no_mangle]
    fn __assert_rtn(_: *const libc::c_char, _: *const libc::c_char,
                    _: libc::c_int, _: *const libc::c_char) -> !;
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
    /* *
 * Call existing ruby functions. This is basically the type safe version of mrb_funcall.
 *
 *      #include <stdio.h>
 *      #include <mruby.h>
 *      #include "mruby/compile.h"
 *      int
 *      main()
 *      {
 *        mrb_int i = 99;
 *        mrb_state *mrb = mrb_open();
 *
 *        if (!mrb) { }
 *        mrb_sym m_sym = mrb_intern_lit(mrb, "method_name"); // Symbol for method.
 *
 *        FILE *fp = fopen("test.rb","r");
 *        mrb_value obj = mrb_load_file(mrb,fp);
 *        mrb_funcall_argv(mrb, obj, m_sym, 1, &obj); // Calling ruby function from test.rb.
 *        fclose(fp);
 *        mrb_close(mrb);
 *       }
 * @param [mrb_state*] mrb_state* The current mruby state.
 * @param [mrb_value] mrb_value A reference to an mruby value.
 * @param [mrb_sym] mrb_sym The symbol representing the method.
 * @param [mrb_int] mrb_int The number of arguments the method has.
 * @param [const mrb_value*] mrb_value* Pointer to the object.
 * @return [mrb_value] mrb_value mruby function value.
 * @see mrb_funcall
 */
    #[no_mangle]
    fn mrb_funcall_argv(_: *mut mrb_state, _: mrb_value, _: mrb_sym,
                        _: mrb_int, _: *const mrb_value) -> mrb_value;
    #[no_mangle]
    fn mrb_intern_static(_: *mut mrb_state, _: *const libc::c_char, _: size_t)
     -> mrb_sym;
    #[no_mangle]
    fn mrb_malloc(_: *mut mrb_state, _: size_t) -> *mut libc::c_void;
    #[no_mangle]
    fn mrb_realloc_simple(_: *mut mrb_state, _: *mut libc::c_void, _: size_t)
     -> *mut libc::c_void;
    #[no_mangle]
    fn mrb_obj_alloc(_: *mut mrb_state, _: mrb_vtype, _: *mut RClass)
     -> *mut RBasic;
    #[no_mangle]
    fn mrb_free(_: *mut mrb_state, _: *mut libc::c_void);
    #[no_mangle]
    fn mrb_obj_id(obj: mrb_value) -> mrb_int;
    #[no_mangle]
    fn mrb_obj_equal(_: *mut mrb_state, _: mrb_value, _: mrb_value)
     -> mrb_bool;
    #[no_mangle]
    fn mrb_equal(mrb: *mut mrb_state, obj1: mrb_value, obj2: mrb_value)
     -> mrb_bool;
    #[no_mangle]
    fn mrb_eql(mrb: *mut mrb_state, obj1: mrb_value, obj2: mrb_value)
     -> mrb_bool;
    #[no_mangle]
    fn mrb_gc_mark(_: *mut mrb_state, _: *mut RBasic);
    #[no_mangle]
    fn mrb_field_write_barrier(_: *mut mrb_state, _: *mut RBasic,
                               _: *mut RBasic);
    #[no_mangle]
    fn mrb_write_barrier(_: *mut mrb_state, _: *mut RBasic);
    #[no_mangle]
    fn mrb_obj_class(mrb: *mut mrb_state, obj: mrb_value) -> *mut RClass;
    #[no_mangle]
    fn mrb_raise(mrb: *mut mrb_state, c: *mut RClass,
                 msg: *const libc::c_char) -> !;
    #[no_mangle]
    fn mrb_frozen_error(mrb: *mut mrb_state, frozen_obj: *mut libc::c_void)
     -> !;
    /* mrb_gc_protect() leaves the object in the arena */
    #[no_mangle]
    fn mrb_gc_protect(mrb: *mut mrb_state, obj: mrb_value);
    #[no_mangle]
    fn mrb_func_basic_p(mrb: *mut mrb_state, obj: mrb_value, mid: mrb_sym,
                        func: mrb_func_t) -> mrb_bool;
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
 * Initializes a new array with two initial values
 *
 * Equivalent to:
 *
 *      Array[car, cdr]
 *
 * @param mrb The mruby state reference.
 * @param car The first value.
 * @param cdr The second value.
 * @return The initialized array.
 */
    #[no_mangle]
    fn mrb_assoc_new(mrb: *mut mrb_state, car: mrb_value, cdr: mrb_value)
     -> mrb_value;
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
    #[no_mangle]
    fn mrb_ensure_hash_type(mrb: *mut mrb_state, hash: mrb_value)
     -> mrb_value;
    #[no_mangle]
    fn mrb_str_hash(mrb: *mut mrb_state, str: mrb_value) -> uint32_t;
    /*
 * Returns true if the strings match and false if the strings don't match.
 *
 * @param [mrb_state] mrb The current mruby state.
 * @param [mrb_value] str1 Ruby string to compare.
 * @param [mrb_value] str2 Ruby string to compare.
 * @return [mrb_value] boolean value.
 */
    #[no_mangle]
    fn mrb_str_equal(mrb: *mut mrb_state, str1: mrb_value, str2: mrb_value)
     -> mrb_bool;
    /*
 * Duplicates a string object.
 *
 *
 * @param [mrb_state] mrb The current mruby state.
 * @param [mrb_value] str Ruby string.
 * @return [mrb_value] Duplicated Ruby string.
 */
    #[no_mangle]
    fn mrb_str_dup(mrb: *mut mrb_state, str: mrb_value) -> mrb_value;
    #[no_mangle]
    fn mrb_iv_get(mrb: *mut mrb_state, obj: mrb_value, sym: mrb_sym)
     -> mrb_value;
    #[no_mangle]
    fn mrb_iv_set(mrb: *mut mrb_state, obj: mrb_value, sym: mrb_sym,
                  v: mrb_value);
}
pub type int64_t = libc::c_longlong;
pub type uint8_t = libc::c_uchar;
pub type uint16_t = libc::c_ushort;
pub type uint32_t = libc::c_uint;
pub type __darwin_size_t = libc::c_ulong;
pub type size_t = __darwin_size_t;
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
/*
** mruby/hash.h - Hash class
**
** See Copyright Notice in mruby.h
*/
/* *
 * Hash class
 */
#[derive ( BitfieldStruct , Clone , Copy )]
#[repr(C)]
pub struct RHash {
    #[bitfield(name = "tt", ty = "mrb_vtype", bits = "0..=7")]
    #[bitfield(name = "color", ty = "uint32_t", bits = "8..=10")]
    #[bitfield(name = "flags", ty = "uint32_t", bits = "11..=31")]
    pub tt_color_flags: [u8; 4],
    pub _pad: [u8; 4],
    pub c: *mut RClass,
    pub gcnext: *mut RBasic,
    pub iv: *mut iv_tbl,
    pub ht: *mut htable,
}
/* hash table structure */
#[derive ( Copy , Clone )]
#[repr(C)]
pub struct htable {
    pub rootseg: *mut segment,
    pub lastseg: *mut segment,
    pub size: mrb_int,
    pub last_len: uint16_t,
    pub index: *mut segindex,
}
#[derive ( Copy , Clone )]
#[repr(C)]
pub struct segindex {
    pub size: size_t,
    pub capa: size_t,
    pub table: [*mut segkv; 0],
}
#[derive ( Copy , Clone )]
#[repr(C)]
pub struct segkv {
    pub key: mrb_value,
    pub val: mrb_value,
}
#[derive ( Copy , Clone )]
#[repr(C)]
pub struct segment {
    pub size: uint16_t,
    pub next: *mut segment,
    pub e: [segkv; 0],
}
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
/* return non zero to break the loop */
pub type mrb_hash_foreach_func
    =
    unsafe extern "C" fn(_: *mut mrb_state, _: mrb_value, _: mrb_value,
                         _: *mut libc::c_void) -> libc::c_int;
#[derive ( Copy , Clone )]
#[repr(C)]
pub struct has_v_arg {
    pub found: mrb_bool,
    pub val: mrb_value,
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
#[inline]
unsafe extern "C" fn mrb_undef_value() -> mrb_value {
    let mut v: mrb_value =
        mrb_value{value: unnamed{f: 0.,}, tt: MRB_TT_FALSE,};
    v.tt = MRB_TT_UNDEF;
    v.value.i = 0i32 as mrb_int;
    return v;
}
#[inline]
unsafe extern "C" fn mrb_check_frozen(mut mrb: *mut mrb_state,
                                      mut o: *mut libc::c_void) {
    if 0 != (*(o as *mut RBasic)).flags() as libc::c_int & 1i32 << 20i32 {
        mrb_frozen_error(mrb, o);
    };
}
#[no_mangle]
pub unsafe extern "C" fn mrb_hash_new_capa(mut mrb: *mut mrb_state,
                                           mut capa: mrb_int) -> mrb_value {
    let mut h: *mut RHash = 0 as *mut RHash;
    h = mrb_obj_alloc(mrb, MRB_TT_HASH, (*mrb).hash_class) as *mut RHash;
    (*h).ht = ht_new(mrb);
    (*h).iv = 0 as *mut iv_tbl;
    return mrb_obj_value(h as *mut libc::c_void);
}
/* Creates the hash table. */
unsafe extern "C" fn ht_new(mut mrb: *mut mrb_state) -> *mut htable {
    let mut t: *mut htable = 0 as *mut htable;
    t =
        mrb_malloc(mrb, ::std::mem::size_of::<htable>() as libc::c_ulong) as
            *mut htable;
    (*t).size = 0i32 as mrb_int;
    (*t).rootseg = 0 as *mut segment;
    (*t).lastseg = 0 as *mut segment;
    (*t).last_len = 0i32 as uint16_t;
    (*t).index = 0 as *mut segindex;
    return t;
}
/*
 * Initializes a new hash.
 *
 * Equivalent to:
 *
 *      Hash.new
 *
 * @param mrb The mruby state reference.
 * @return The initialized hash.
 */
#[no_mangle]
pub unsafe extern "C" fn mrb_hash_new(mut mrb: *mut mrb_state) -> mrb_value {
    let mut h: *mut RHash = 0 as *mut RHash;
    h = mrb_obj_alloc(mrb, MRB_TT_HASH, (*mrb).hash_class) as *mut RHash;
    (*h).ht = 0 as *mut htable;
    (*h).iv = 0 as *mut iv_tbl;
    return mrb_obj_value(h as *mut libc::c_void);
}
/*
 * Sets a keys and values to hashes.
 *
 * Equivalent to:
 *
 *      hash[key] = val
 *
 * @param mrb The mruby state reference.
 * @param hash The target hash.
 * @param key The key to set.
 * @param val The value to set.
 * @return The value.
 */
#[no_mangle]
pub unsafe extern "C" fn mrb_hash_set(mut mrb: *mut mrb_state,
                                      mut hash: mrb_value, mut key: mrb_value,
                                      mut val: mrb_value) {
    mrb_hash_modify(mrb, hash);
    key = ht_key(mrb, key);
    ht_put(mrb, (*(hash.value.p as *mut RHash)).ht, key, val);
    if !((key.tt as libc::c_uint) <
             MRB_TT_OBJECT as libc::c_int as libc::c_uint) {
        mrb_field_write_barrier(mrb,
                                hash.value.p as *mut RHash as *mut RBasic,
                                key.value.p as *mut RBasic);
    }
    if !((val.tt as libc::c_uint) <
             MRB_TT_OBJECT as libc::c_int as libc::c_uint) {
        mrb_field_write_barrier(mrb,
                                hash.value.p as *mut RHash as *mut RBasic,
                                val.value.p as *mut RBasic);
    };
}
/* Set the value for the key in the hash table. */
unsafe extern "C" fn ht_put(mut mrb: *mut mrb_state, mut t: *mut htable,
                            mut key: mrb_value, mut val: mrb_value) {
    let mut seg: *mut segment = 0 as *mut segment;
    let mut i: mrb_int = 0;
    let mut deleted: mrb_int = 0i32 as mrb_int;
    if t.is_null() { return }
    if !(*t).index.is_null() { ht_index_put(mrb, t, key, val); return }
    seg = (*t).rootseg;
    while !seg.is_null() {
        i = 0i32 as mrb_int;
        while i < (*seg).size as libc::c_longlong {
            let mut k: mrb_value =
                (*(*seg).e.as_mut_ptr().offset(i as isize)).key;
            if (*seg).next.is_null() && i >= (*t).last_len as libc::c_longlong
               {
                (*(*seg).e.as_mut_ptr().offset(i as isize)).key = key;
                (*(*seg).e.as_mut_ptr().offset(i as isize)).val = val;
                (*t).last_len = (i + 1i32 as libc::c_longlong) as uint16_t;
                (*t).size += 1;
                return
            }
            if k.tt as libc::c_uint ==
                   MRB_TT_UNDEF as libc::c_int as libc::c_uint {
                deleted += 1
            } else if 0 != ht_hash_equal(mrb, t, k, key) {
                (*(*seg).e.as_mut_ptr().offset(i as isize)).val = val;
                return
            }
            i += 1
        }
        seg = (*seg).next
    }
    if deleted > 0i32 as libc::c_longlong &&
           deleted > 4i32 as libc::c_longlong {
        ht_compact(mrb, t);
    }
    (*t).size += 1;
    if !(*t).lastseg.is_null() &&
           ((*t).last_len as libc::c_int) <
               (*(*t).lastseg).size as libc::c_int {
        seg = (*t).lastseg;
        i = (*t).last_len as mrb_int
    } else {
        seg = segment_alloc(mrb, (*t).lastseg);
        i = 0i32 as mrb_int;
        if (*t).rootseg.is_null() {
            (*t).rootseg = seg
        } else { (*(*t).lastseg).next = seg }
        (*t).lastseg = seg
    }
    (*(*seg).e.as_mut_ptr().offset(i as isize)).key = key;
    (*(*seg).e.as_mut_ptr().offset(i as isize)).val = val;
    (*t).last_len = (i + 1i32 as libc::c_longlong) as uint16_t;
    if (*t).index.is_null() && (*t).size > (4i32 * 4i32) as libc::c_longlong {
        ht_index(mrb, t);
    };
}
/* Build index for the hash table */
unsafe extern "C" fn ht_index(mut mrb: *mut mrb_state, mut t: *mut htable) {
    let mut size: size_t = (*t).size as size_t;
    let mut mask: size_t = 0;
    let mut index: *mut segindex = (*t).index;
    let mut seg: *mut segment = 0 as *mut segment;
    let mut i: size_t = 0;
    if !index.is_null() &&
           (*index).size >= (*index).capa >> 2i32 | (*index).capa >> 1i32 {
        size = (*index).capa.wrapping_add(1i32 as libc::c_ulong)
    }
    size = size.wrapping_sub(1);
    size |= size >> 1i32;
    size |= size >> 2i32;
    size |= size >> 4i32;
    size |= size >> 8i32;
    size |= size >> 16i32;
    size = size.wrapping_add(1);
    if index.is_null() || (*index).capa < size {
        index =
            mrb_realloc_simple(mrb, index as *mut libc::c_void,
                               (::std::mem::size_of::<segindex>() as
                                    libc::c_ulong).wrapping_add((::std::mem::size_of::<*mut segkv>()
                                                                     as
                                                                     libc::c_ulong).wrapping_mul(size)))
                as *mut segindex;
        if index.is_null() {
            mrb_free(mrb, (*t).index as *mut libc::c_void);
            (*t).index = 0 as *mut segindex;
            return
        }
        (*t).index = index
    }
    (*index).size = (*t).size as size_t;
    (*index).capa = size;
    i = 0i32 as size_t;
    while i < size {
        let ref mut fresh0 = *(*index).table.as_mut_ptr().offset(i as isize);
        *fresh0 = 0 as *mut segkv;
        i = i.wrapping_add(1)
    }
    mask = (*index).capa.wrapping_sub(1i32 as libc::c_ulong);
    seg = (*t).rootseg;
    while !seg.is_null() {
        i = 0i32 as size_t;
        while i < (*seg).size as libc::c_ulong {
            let mut key: mrb_value =
                mrb_value{value: unnamed{f: 0.,}, tt: MRB_TT_FALSE,};
            let mut k: size_t = 0;
            let mut step: size_t = 0i32 as size_t;
            if (*seg).next.is_null() && i >= (*t).last_len as size_t {
                return
            }
            key = (*(*seg).e.as_mut_ptr().offset(i as isize)).key;
            if !(key.tt as libc::c_uint ==
                     MRB_TT_UNDEF as libc::c_int as libc::c_uint) {
                k = ht_hash_func(mrb, t, key) & mask;
                while !(*(*index).table.as_mut_ptr().offset(k as
                                                                isize)).is_null()
                      {
                    step = step.wrapping_add(1);
                    k = k.wrapping_add(step) & mask
                }
                let ref mut fresh1 =
                    *(*index).table.as_mut_ptr().offset(k as isize);
                *fresh1 =
                    &mut *(*seg).e.as_mut_ptr().offset(i as isize) as
                        *mut segkv
            }
            i = i.wrapping_add(1)
        }
        seg = (*seg).next
    };
}
/* inline */
unsafe extern "C" fn ht_hash_func(mut mrb: *mut mrb_state, mut t: *mut htable,
                                  mut key: mrb_value) -> size_t {
    let mut tt: mrb_vtype = key.tt;
    let mut hv: mrb_value =
        mrb_value{value: unnamed{f: 0.,}, tt: MRB_TT_FALSE,};
    let mut h: size_t = 0;
    let mut index: *mut segindex = (*t).index;
    let mut capa: size_t =
        if !index.is_null() { (*index).capa } else { 0i32 as libc::c_ulong };
    match tt as libc::c_uint {
        16 => { h = mrb_str_hash(mrb, key) as size_t }
        2 | 0 | 4 | 3 | 6 => { h = mrb_obj_id(key) as size_t }
        _ => {
            hv =
                mrb_funcall(mrb, key,
                            b"hash\x00" as *const u8 as *const libc::c_char,
                            0i32 as mrb_int);
            h = t as size_t ^ hv.value.i as size_t
        }
    }
    if !index.is_null() && (index != (*t).index || capa != (*index).capa) {
        mrb_raise(mrb,
                  mrb_exc_get(mrb,
                              b"RuntimeError\x00" as *const u8 as
                                  *const libc::c_char),
                  b"hash modified\x00" as *const u8 as *const libc::c_char);
    }
    return h ^ h << 2i32 ^ h >> 2i32;
}
unsafe extern "C" fn segment_alloc(mut mrb: *mut mrb_state,
                                   mut seg: *mut segment) -> *mut segment {
    let mut size: uint32_t = 0;
    if seg.is_null() {
        size = 4i32 as uint32_t
    } else {
        size = ((*seg).size as libc::c_int * 6i32 / 5i32 + 1i32) as uint32_t;
        if size > 65535i32 as libc::c_uint { size = 65535i32 as uint32_t }
    }
    seg =
        mrb_malloc(mrb,
                   (::std::mem::size_of::<segment>() as
                        libc::c_ulong).wrapping_add((::std::mem::size_of::<segkv>()
                                                         as
                                                         libc::c_ulong).wrapping_mul(size
                                                                                         as
                                                                                         libc::c_ulong)))
            as *mut segment;
    (*seg).size = size as uint16_t;
    (*seg).next = 0 as *mut segment;
    return seg;
}
/* Compacts the hash table removing deleted entries. */
unsafe extern "C" fn ht_compact(mut mrb: *mut mrb_state, mut t: *mut htable) {
    let mut seg: *mut segment = 0 as *mut segment;
    let mut i: mrb_int = 0;
    let mut seg2: *mut segment = 0 as *mut segment;
    let mut i2: mrb_int = 0;
    let mut size: mrb_int = 0i32 as mrb_int;
    if t.is_null() { return }
    seg = (*t).rootseg;
    if !(*t).index.is_null() && (*t).size as size_t == (*(*t).index).size {
        ht_index(mrb, t);
        return
    }
    's_45:
        while !seg.is_null() {
            i = 0i32 as mrb_int;
            while i < (*seg).size as libc::c_longlong {
                let mut k: mrb_value =
                    (*(*seg).e.as_mut_ptr().offset(i as isize)).key;
                if (*seg).next.is_null() &&
                       i >= (*t).last_len as libc::c_longlong {
                    break 's_45 ;
                }
                if k.tt as libc::c_uint ==
                       MRB_TT_UNDEF as libc::c_int as libc::c_uint {
                    if seg2.is_null() { seg2 = seg; i2 = i }
                } else {
                    size += 1;
                    if !seg2.is_null() {
                        let fresh2 = i2;
                        i2 = i2 + 1;
                        *(*seg2).e.as_mut_ptr().offset(fresh2 as isize) =
                            *(*seg).e.as_mut_ptr().offset(i as isize);
                        if i2 >= (*seg2).size as libc::c_longlong {
                            seg2 = (*seg2).next;
                            i2 = 0i32 as mrb_int
                        }
                    }
                }
                i += 1
            }
            seg = (*seg).next
        }
    (*t).size = size;
    if !seg2.is_null() {
        seg = (*seg2).next;
        (*seg2).next = 0 as *mut segment;
        (*t).last_len = i2 as uint16_t;
        (*t).lastseg = seg2;
        while !seg.is_null() {
            seg2 = (*seg).next;
            mrb_free(mrb, seg as *mut libc::c_void);
            seg = seg2
        }
    }
    if !(*t).index.is_null() { ht_index(mrb, t); };
}
#[inline]
unsafe extern "C" fn ht_hash_equal(mut mrb: *mut mrb_state,
                                   mut t: *mut htable, mut a: mrb_value,
                                   mut b: mrb_value) -> mrb_bool {
    let mut tt: mrb_vtype = a.tt;
    match tt as libc::c_uint {
        16 => { return mrb_str_equal(mrb, a, b) }
        4 => {
            if b.tt as libc::c_uint !=
                   MRB_TT_SYMBOL as libc::c_int as libc::c_uint {
                return 0i32 as mrb_bool
            }
            return (a.value.sym == b.value.sym) as libc::c_int as mrb_bool
        }
        3 => {
            match b.tt as libc::c_uint {
                3 => {
                    return (a.value.i == b.value.i) as libc::c_int as mrb_bool
                }
                6 => {
                    return (a.value.i as mrb_float == b.value.f) as
                               libc::c_int as mrb_bool
                }
                _ => { return 0i32 as mrb_bool }
            }
        }
        6 => {
            match b.tt as libc::c_uint {
                3 => {
                    return (a.value.f == b.value.i as mrb_float) as
                               libc::c_int as mrb_bool
                }
                6 => {
                    return (a.value.f == b.value.f) as libc::c_int as mrb_bool
                }
                _ => { return 0i32 as mrb_bool }
            }
        }
        _ => {
            let mut index: *mut segindex = (*t).index;
            let mut capa: size_t =
                if !index.is_null() {
                    (*index).capa
                } else { 0i32 as libc::c_ulong };
            let mut eql: mrb_bool = mrb_eql(mrb, a, b);
            if !index.is_null() &&
                   (index != (*t).index || capa != (*index).capa) {
                mrb_raise(mrb,
                          mrb_exc_get(mrb,
                                      b"RuntimeError\x00" as *const u8 as
                                          *const libc::c_char),
                          b"hash modified\x00" as *const u8 as
                              *const libc::c_char);
            }
            return eql
        }
    };
}
/* Set the value for the key in the indexed table. */
unsafe extern "C" fn ht_index_put(mut mrb: *mut mrb_state, mut t: *mut htable,
                                  mut key: mrb_value, mut val: mrb_value) {
    let mut index: *mut segindex = (*t).index;
    let mut k: size_t = 0;
    let mut sp: size_t = 0;
    let mut step: size_t = 0i32 as size_t;
    let mut mask: size_t = 0;
    let mut seg: *mut segment = 0 as *mut segment;
    if (*index).size >= (*index).capa >> 2i32 | (*index).capa >> 1i32 {
        ht_compact(mrb, t);
        index = (*t).index
    }
    mask = (*index).capa.wrapping_sub(1i32 as libc::c_ulong);
    sp = (*index).capa;
    k = ht_hash_func(mrb, t, key) & mask;
    while !(*(*index).table.as_mut_ptr().offset(k as isize)).is_null() {
        let mut key2: mrb_value =
            (**(*index).table.as_mut_ptr().offset(k as isize)).key;
        if key2.tt as libc::c_uint ==
               MRB_TT_UNDEF as libc::c_int as libc::c_uint {
            if sp == (*index).capa { sp = k }
        } else if 0 != ht_hash_equal(mrb, t, key, key2) {
            (**(*index).table.as_mut_ptr().offset(k as isize)).val = val;
            return
        }
        step = step.wrapping_add(1);
        k = k.wrapping_add(step) & mask
    }
    if sp < (*index).capa { k = sp }
    seg = (*t).lastseg;
    if ((*t).last_len as libc::c_int) < (*seg).size as libc::c_int {
        let ref mut fresh4 = *(*index).table.as_mut_ptr().offset(k as isize);
        let fresh3 = (*t).last_len;
        (*t).last_len = (*t).last_len.wrapping_add(1);
        *fresh4 =
            &mut *(*seg).e.as_mut_ptr().offset(fresh3 as isize) as *mut segkv
    } else {
        (*seg).next = segment_alloc(mrb, seg);
        seg = (*seg).next;
        (*seg).next = 0 as *mut segment;
        (*t).lastseg = seg;
        (*t).last_len = 1i32 as uint16_t;
        let ref mut fresh5 = *(*index).table.as_mut_ptr().offset(k as isize);
        *fresh5 = &mut *(*seg).e.as_mut_ptr().offset(0isize) as *mut segkv
    }
    (**(*index).table.as_mut_ptr().offset(k as isize)).key = key;
    (**(*index).table.as_mut_ptr().offset(k as isize)).val = val;
    (*index).size = (*index).size.wrapping_add(1);
    (*t).size += 1;
}
#[inline]
unsafe extern "C" fn ht_key(mut mrb: *mut mrb_state, mut key: mrb_value)
 -> mrb_value {
    if key.tt as libc::c_uint == MRB_TT_STRING as libc::c_int as libc::c_uint
           &&
           0 ==
               (*(key.value.p as *mut RString)).flags() as libc::c_int &
                   1i32 << 20i32 {
        key = mrb_str_dup(mrb, key);
        let ref mut fresh6 = *(key.value.p as *mut RString);
        (*fresh6).set_flags((*fresh6).flags() | (1i32 << 20i32) as uint32_t)
    }
    return key;
}
unsafe extern "C" fn mrb_hash_modify(mut mrb: *mut mrb_state,
                                     mut hash: mrb_value) {
    mrb_check_frozen(mrb, hash.value.p as *mut RHash as *mut libc::c_void);
    if (*(hash.value.p as *mut RHash)).ht.is_null() {
        let ref mut fresh7 = (*(hash.value.p as *mut RHash)).ht;
        *fresh7 = ht_new(mrb)
    };
}
/*
 * Gets a value from a key. If the key is not found, the default of the
 * hash is used.
 *
 * Equivalent to:
 *
 *     hash[key]
 *
 * @param mrb The mruby state reference.
 * @param hash The target hash.
 * @param key The key to get.
 * @return The found value.
 */
#[no_mangle]
pub unsafe extern "C" fn mrb_hash_get(mut mrb: *mut mrb_state,
                                      mut hash: mrb_value, mut key: mrb_value)
 -> mrb_value {
    let mut val: mrb_value =
        mrb_value{value: unnamed{f: 0.,}, tt: MRB_TT_FALSE,};
    let mut mid: mrb_sym = 0;
    if 0 != ht_get(mrb, (*(hash.value.p as *mut RHash)).ht, key, &mut val) {
        return val
    }
    mid =
        mrb_intern_static(mrb,
                          b"default\x00" as *const u8 as *const libc::c_char,
                          (::std::mem::size_of::<[libc::c_char; 8]>() as
                               libc::c_ulong).wrapping_sub(1i32 as
                                                               libc::c_ulong));
    if 0 != mrb_func_basic_p(mrb, hash, mid, Some(mrb_hash_default)) {
        return hash_default(mrb, hash, key)
    }
    return mrb_funcall_argv(mrb, hash, mid, 1i32 as mrb_int, &mut key);
}
unsafe extern "C" fn hash_default(mut mrb: *mut mrb_state,
                                  mut hash: mrb_value, mut key: mrb_value)
 -> mrb_value {
    if 0 != (*(hash.value.p as *mut RHash)).flags() as libc::c_int & 1i32 {
        if 0 != (*(hash.value.p as *mut RHash)).flags() as libc::c_int & 2i32
           {
            return mrb_funcall(mrb,
                               mrb_iv_get(mrb, hash,
                                          mrb_intern_static(mrb,
                                                            b"ifnone\x00" as
                                                                *const u8 as
                                                                *const libc::c_char,
                                                            (::std::mem::size_of::<[libc::c_char; 7]>()
                                                                 as
                                                                 libc::c_ulong).wrapping_sub(1i32
                                                                                                 as
                                                                                                 libc::c_ulong))),
                               b"call\x00" as *const u8 as
                                   *const libc::c_char, 2i32 as mrb_int, hash,
                               key)
        } else {
            return mrb_iv_get(mrb, hash,
                              mrb_intern_static(mrb,
                                                b"ifnone\x00" as *const u8 as
                                                    *const libc::c_char,
                                                (::std::mem::size_of::<[libc::c_char; 7]>()
                                                     as
                                                     libc::c_ulong).wrapping_sub(1i32
                                                                                     as
                                                                                     libc::c_ulong)))
        }
    }
    return mrb_nil_value();
}
unsafe extern "C" fn mrb_hash_default(mut mrb: *mut mrb_state,
                                      mut hash: mrb_value) -> mrb_value {
    let mut key: mrb_value =
        mrb_value{value: unnamed{f: 0.,}, tt: MRB_TT_FALSE,};
    let mut given: mrb_bool = 0;
    mrb_get_args(mrb, b"|o?\x00" as *const u8 as *const libc::c_char,
                 &mut key as *mut mrb_value, &mut given as *mut mrb_bool);
    if 0 != (*(hash.value.p as *mut RHash)).flags() as libc::c_int & 1i32 {
        if 0 != (*(hash.value.p as *mut RHash)).flags() as libc::c_int & 2i32
           {
            if 0 == given { return mrb_nil_value() }
            return mrb_funcall(mrb,
                               mrb_iv_get(mrb, hash,
                                          mrb_intern_static(mrb,
                                                            b"ifnone\x00" as
                                                                *const u8 as
                                                                *const libc::c_char,
                                                            (::std::mem::size_of::<[libc::c_char; 7]>()
                                                                 as
                                                                 libc::c_ulong).wrapping_sub(1i32
                                                                                                 as
                                                                                                 libc::c_ulong))),
                               b"call\x00" as *const u8 as
                                   *const libc::c_char, 2i32 as mrb_int, hash,
                               key)
        } else {
            return mrb_iv_get(mrb, hash,
                              mrb_intern_static(mrb,
                                                b"ifnone\x00" as *const u8 as
                                                    *const libc::c_char,
                                                (::std::mem::size_of::<[libc::c_char; 7]>()
                                                     as
                                                     libc::c_ulong).wrapping_sub(1i32
                                                                                     as
                                                                                     libc::c_ulong)))
        }
    }
    return mrb_nil_value();
}
/* Get a value for a key from the hash table. */
unsafe extern "C" fn ht_get(mut mrb: *mut mrb_state, mut t: *mut htable,
                            mut key: mrb_value, mut vp: *mut mrb_value)
 -> mrb_bool {
    let mut seg: *mut segment = 0 as *mut segment;
    let mut i: mrb_int = 0;
    if t.is_null() { return 0i32 as mrb_bool }
    if !(*t).index.is_null() { return ht_index_get(mrb, t, key, vp) }
    seg = (*t).rootseg;
    while !seg.is_null() {
        i = 0i32 as mrb_int;
        while i < (*seg).size as libc::c_longlong {
            let mut k: mrb_value =
                (*(*seg).e.as_mut_ptr().offset(i as isize)).key;
            if (*seg).next.is_null() && i >= (*t).last_len as libc::c_longlong
               {
                return 0i32 as mrb_bool
            }
            if !(k.tt as libc::c_uint ==
                     MRB_TT_UNDEF as libc::c_int as libc::c_uint) {
                if 0 != ht_hash_equal(mrb, t, k, key) {
                    if !vp.is_null() {
                        *vp = (*(*seg).e.as_mut_ptr().offset(i as isize)).val
                    }
                    return 1i32 as mrb_bool
                }
            }
            i += 1
        }
        seg = (*seg).next
    }
    return 0i32 as mrb_bool;
}
/* Get a value for a key from the indexed table. */
unsafe extern "C" fn ht_index_get(mut mrb: *mut mrb_state, mut t: *mut htable,
                                  mut key: mrb_value, mut vp: *mut mrb_value)
 -> mrb_bool {
    let mut index: *mut segindex = (*t).index;
    let mut mask: size_t = (*index).capa.wrapping_sub(1i32 as libc::c_ulong);
    let mut k: size_t = ht_hash_func(mrb, t, key) & mask;
    let mut step: size_t = 0i32 as size_t;
    while !(*(*index).table.as_mut_ptr().offset(k as isize)).is_null() {
        let mut key2: mrb_value =
            (**(*index).table.as_mut_ptr().offset(k as isize)).key;
        if !(key2.tt as libc::c_uint ==
                 MRB_TT_UNDEF as libc::c_int as libc::c_uint) &&
               0 != ht_hash_equal(mrb, t, key, key2) as libc::c_int {
            if !vp.is_null() {
                *vp = (**(*index).table.as_mut_ptr().offset(k as isize)).val
            }
            return 1i32 as mrb_bool
        }
        step = step.wrapping_add(1);
        k = k.wrapping_add(step) & mask
    }
    return 0i32 as mrb_bool;
}
/*
 * Gets a value from a key. If the key is not found, the default parameter is
 * used.
 *
 * Equivalent to:
 *
 *     hash.key?(key) ? hash[key] : def
 *
 * @param mrb The mruby state reference.
 * @param hash The target hash.
 * @param key The key to get.
 * @param def The default value.
 * @return The found value.
 */
#[no_mangle]
pub unsafe extern "C" fn mrb_hash_fetch(mut mrb: *mut mrb_state,
                                        mut hash: mrb_value,
                                        mut key: mrb_value,
                                        mut def: mrb_value) -> mrb_value {
    let mut val: mrb_value =
        mrb_value{value: unnamed{f: 0.,}, tt: MRB_TT_FALSE,};
    if 0 != ht_get(mrb, (*(hash.value.p as *mut RHash)).ht, key, &mut val) {
        return val
    }
    return def;
}
/*
 * Deletes hash key and value pair.
 *
 * Equivalent to:
 *
 *     hash.delete(key)
 *
 * @param mrb The mruby state reference.
 * @param hash The target hash.
 * @param key The key to delete.
 * @return The deleted value.
 */
#[no_mangle]
pub unsafe extern "C" fn mrb_hash_delete_key(mut mrb: *mut mrb_state,
                                             mut hash: mrb_value,
                                             mut key: mrb_value)
 -> mrb_value {
    let mut t: *mut htable = (*(hash.value.p as *mut RHash)).ht;
    let mut del_val: mrb_value =
        mrb_value{value: unnamed{f: 0.,}, tt: MRB_TT_FALSE,};
    if 0 != ht_del(mrb, t, key, &mut del_val) { return del_val }
    return mrb_nil_value();
}
/* Deletes the value for the symbol from the hash table. */
/* Deletion is done by overwriting keys by `undef`. */
unsafe extern "C" fn ht_del(mut mrb: *mut mrb_state, mut t: *mut htable,
                            mut key: mrb_value, mut vp: *mut mrb_value)
 -> mrb_bool {
    let mut seg: *mut segment = 0 as *mut segment;
    let mut i: mrb_int = 0;
    if t.is_null() { return 0i32 as mrb_bool }
    seg = (*t).rootseg;
    while !seg.is_null() {
        i = 0i32 as mrb_int;
        while i < (*seg).size as libc::c_longlong {
            let mut key2: mrb_value =
                mrb_value{value: unnamed{f: 0.,}, tt: MRB_TT_FALSE,};
            if (*seg).next.is_null() && i >= (*t).last_len as libc::c_longlong
               {
                return 0i32 as mrb_bool
            }
            key2 = (*(*seg).e.as_mut_ptr().offset(i as isize)).key;
            if !(key2.tt as libc::c_uint ==
                     MRB_TT_UNDEF as libc::c_int as libc::c_uint) &&
                   0 != ht_hash_equal(mrb, t, key, key2) as libc::c_int {
                if !vp.is_null() {
                    *vp = (*(*seg).e.as_mut_ptr().offset(i as isize)).val
                }
                (*(*seg).e.as_mut_ptr().offset(i as isize)).key =
                    mrb_undef_value();
                (*t).size -= 1;
                return 1i32 as mrb_bool
            }
            i += 1
        }
        seg = (*seg).next
    }
    return 0i32 as mrb_bool;
}
/*
 * Gets an array of keys.
 *
 * Equivalent to:
 *
 *     hash.keys
 *
 * @param mrb The mruby state reference.
 * @param hash The target hash.
 * @return An array with the keys of the hash.
 */
#[no_mangle]
pub unsafe extern "C" fn mrb_hash_keys(mut mrb: *mut mrb_state,
                                       mut hash: mrb_value) -> mrb_value {
    let mut t: *mut htable = (*(hash.value.p as *mut RHash)).ht;
    let mut size: mrb_int = 0;
    let mut ary: mrb_value =
        mrb_value{value: unnamed{f: 0.,}, tt: MRB_TT_FALSE,};
    if t.is_null() || { size = (*t).size; size == 0i32 as libc::c_longlong } {
        return mrb_ary_new(mrb)
    }
    ary = mrb_ary_new_capa(mrb, size);
    ht_foreach(mrb, t, Some(hash_keys_i),
               &mut ary as *mut mrb_value as *mut libc::c_void);
    return ary;
}
unsafe extern "C" fn hash_keys_i(mut mrb: *mut mrb_state, mut key: mrb_value,
                                 mut val: mrb_value, mut p: *mut libc::c_void)
 -> libc::c_int {
    mrb_ary_push(mrb, *(p as *mut mrb_value), key);
    return 0i32;
}
/* Iterates over the hash table. */
unsafe extern "C" fn ht_foreach(mut mrb: *mut mrb_state, mut t: *mut htable,
                                mut func:
                                    Option<unsafe extern "C" fn(_:
                                                                    *mut mrb_state,
                                                                _: mrb_value,
                                                                _: mrb_value,
                                                                _:
                                                                    *mut libc::c_void)
                                               -> libc::c_int>,
                                mut p: *mut libc::c_void) {
    let mut seg: *mut segment = 0 as *mut segment;
    let mut i: mrb_int = 0;
    if t.is_null() { return }
    seg = (*t).rootseg;
    while !seg.is_null() {
        i = 0i32 as mrb_int;
        while i < (*seg).size as libc::c_longlong {
            if (*seg).next.is_null() && i >= (*t).last_len as libc::c_longlong
               {
                return
            }
            if !((*(*seg).e.as_mut_ptr().offset(i as isize)).key.tt as
                     libc::c_uint ==
                     MRB_TT_UNDEF as libc::c_int as libc::c_uint) {
                if func.expect("non-null function pointer")(mrb,
                                                            (*(*seg).e.as_mut_ptr().offset(i
                                                                                               as
                                                                                               isize)).key,
                                                            (*(*seg).e.as_mut_ptr().offset(i
                                                                                               as
                                                                                               isize)).val,
                                                            p) != 0i32 {
                    return
                }
            }
            i += 1
        }
        seg = (*seg).next
    };
}
/*
 * Check if the hash has the key.
 *
 * Equivalent to:
 *
 *     hash.key?(key)
 *
 * @param mrb The mruby state reference.
 * @param hash The target hash.
 * @param key The key to check existence.
 * @return True if the hash has the key
 */
#[no_mangle]
pub unsafe extern "C" fn mrb_hash_key_p(mut mrb: *mut mrb_state,
                                        mut hash: mrb_value,
                                        mut key: mrb_value) -> mrb_bool {
    let mut t: *mut htable = 0 as *mut htable;
    t = (*(hash.value.p as *mut RHash)).ht;
    if 0 != ht_get(mrb, t, key, 0 as *mut mrb_value) {
        return 1i32 as mrb_bool
    }
    return 0i32 as mrb_bool;
}
/*
 * Check if the hash is empty
 *
 * Equivalent to:
 *
 *     hash.empty?
 *
 * @param mrb The mruby state reference.
 * @param self The target hash.
 * @return True if the hash is empty, false otherwise.
 */
#[no_mangle]
pub unsafe extern "C" fn mrb_hash_empty_p(mut mrb: *mut mrb_state,
                                          mut self_0: mrb_value) -> mrb_bool {
    let mut t: *mut htable = (*(self_0.value.p as *mut RHash)).ht;
    if t.is_null() { return 1i32 as mrb_bool }
    return ((*t).size == 0i32 as libc::c_longlong) as libc::c_int as mrb_bool;
}
/*
 * Gets an array of values.
 *
 * Equivalent to:
 *
 *     hash.values
 *
 * @param mrb The mruby state reference.
 * @param hash The target hash.
 * @return An array with the values of the hash.
 */
#[no_mangle]
pub unsafe extern "C" fn mrb_hash_values(mut mrb: *mut mrb_state,
                                         mut hash: mrb_value) -> mrb_value {
    let mut t: *mut htable = (*(hash.value.p as *mut RHash)).ht;
    let mut size: mrb_int = 0;
    let mut ary: mrb_value =
        mrb_value{value: unnamed{f: 0.,}, tt: MRB_TT_FALSE,};
    if t.is_null() || { size = (*t).size; size == 0i32 as libc::c_longlong } {
        return mrb_ary_new(mrb)
    }
    ary = mrb_ary_new_capa(mrb, size);
    ht_foreach(mrb, t, Some(hash_vals_i),
               &mut ary as *mut mrb_value as *mut libc::c_void);
    return ary;
}
unsafe extern "C" fn hash_vals_i(mut mrb: *mut mrb_state, mut key: mrb_value,
                                 mut val: mrb_value, mut p: *mut libc::c_void)
 -> libc::c_int {
    mrb_ary_push(mrb, *(p as *mut mrb_value), val);
    return 0i32;
}
/*
 * Clears the hash.
 *
 * Equivalent to:
 *
 *     hash.clear
 *
 * @param mrb The mruby state reference.
 * @param hash The target hash.
 * @return The hash
 */
#[no_mangle]
pub unsafe extern "C" fn mrb_hash_clear(mut mrb: *mut mrb_state,
                                        mut hash: mrb_value) -> mrb_value {
    let mut t: *mut htable = (*(hash.value.p as *mut RHash)).ht;
    mrb_hash_modify(mrb, hash);
    if !t.is_null() {
        ht_free(mrb, t);
        let ref mut fresh8 = (*(hash.value.p as *mut RHash)).ht;
        *fresh8 = 0 as *mut htable
    }
    return hash;
}
/* Free memory of the hash table. */
unsafe extern "C" fn ht_free(mut mrb: *mut mrb_state, mut t: *mut htable) {
    let mut seg: *mut segment = 0 as *mut segment;
    if t.is_null() { return }
    seg = (*t).rootseg;
    while !seg.is_null() {
        let mut p: *mut segment = seg;
        seg = (*seg).next;
        mrb_free(mrb, p as *mut libc::c_void);
    }
    if !(*t).index.is_null() {
        mrb_free(mrb, (*t).index as *mut libc::c_void);
    }
    mrb_free(mrb, t as *mut libc::c_void);
}
/*
 * Get hash size.
 *
 * Equivalent to:
 *
 *      hash.size
 *
 * @param mrb The mruby state reference.
 * @param hash The target hash.
 * @return The hash size.
 */
#[no_mangle]
pub unsafe extern "C" fn mrb_hash_size(mut mrb: *mut mrb_state,
                                       mut hash: mrb_value) -> mrb_int {
    let mut t: *mut htable = (*(hash.value.p as *mut RHash)).ht;
    if t.is_null() { return 0i32 as mrb_int }
    return (*t).size;
}
/*
 * Copies the hash.
 *
 *
 * @param mrb The mruby state reference.
 * @param hash The target hash.
 * @return The copy of the hash
 */
#[no_mangle]
pub unsafe extern "C" fn mrb_hash_dup(mut mrb: *mut mrb_state,
                                      mut self_0: mrb_value) -> mrb_value {
    let mut copy: *mut RHash = 0 as *mut RHash;
    let mut orig_h: *mut htable = 0 as *mut htable;
    orig_h = (*(self_0.value.p as *mut RHash)).ht;
    copy = mrb_obj_alloc(mrb, MRB_TT_HASH, (*mrb).hash_class) as *mut RHash;
    (*copy).ht =
        if !orig_h.is_null() {
            ht_copy(mrb, orig_h)
        } else { 0 as *mut htable };
    return mrb_obj_value(copy as *mut libc::c_void);
}
/* Copy the hash table. */
unsafe extern "C" fn ht_copy(mut mrb: *mut mrb_state, mut t: *mut htable)
 -> *mut htable {
    let mut seg: *mut segment = 0 as *mut segment;
    let mut t2: *mut htable = 0 as *mut htable;
    let mut i: mrb_int = 0;
    seg = (*t).rootseg;
    t2 = ht_new(mrb);
    if (*t).size == 0i32 as libc::c_longlong { return t2 }
    while !seg.is_null() {
        i = 0i32 as mrb_int;
        while i < (*seg).size as libc::c_longlong {
            let mut key: mrb_value =
                (*(*seg).e.as_mut_ptr().offset(i as isize)).key;
            let mut val: mrb_value =
                (*(*seg).e.as_mut_ptr().offset(i as isize)).val;
            if (*seg).next.is_null() && i >= (*t).last_len as libc::c_longlong
               {
                return t2
            }
            ht_put(mrb, t2, key, val);
            i += 1
        }
        seg = (*seg).next
    }
    return t2;
}
/*
 * Merges two hashes. The first hash will be modified by the
 * second hash.
 *
 * @param mrb The mruby state reference.
 * @param hash1 The target hash.
 * @param hash2 Updating hash
 */
#[no_mangle]
pub unsafe extern "C" fn mrb_hash_merge(mut mrb: *mut mrb_state,
                                        mut hash1: mrb_value,
                                        mut hash2: mrb_value) {
    let mut h1: *mut htable = 0 as *mut htable;
    let mut h2: *mut htable = 0 as *mut htable;
    mrb_hash_modify(mrb, hash1);
    hash2 = mrb_ensure_hash_type(mrb, hash2);
    h1 = (*(hash1.value.p as *mut RHash)).ht;
    h2 = (*(hash2.value.p as *mut RHash)).ht;
    if h2.is_null() { return }
    if h1.is_null() {
        let ref mut fresh9 = (*(hash1.value.p as *mut RHash)).ht;
        *fresh9 = ht_copy(mrb, h2);
        return
    }
    ht_foreach(mrb, h2, Some(merge_i), h1 as *mut libc::c_void);
    mrb_write_barrier(mrb, hash1.value.p as *mut RHash as *mut RBasic);
}
unsafe extern "C" fn merge_i(mut mrb: *mut mrb_state, mut key: mrb_value,
                             mut val: mrb_value, mut data: *mut libc::c_void)
 -> libc::c_int {
    let mut h1: *mut htable = data as *mut htable;
    ht_put(mrb, h1, key, val);
    return 0i32;
}
/* RHASH_TBL allocates st_table if not available. */
/* GC functions */
#[no_mangle]
pub unsafe extern "C" fn mrb_gc_mark_hash(mut mrb: *mut mrb_state,
                                          mut hash: *mut RHash) {
    ht_foreach(mrb, (*hash).ht, Some(hash_mark_i), 0 as *mut libc::c_void);
}
unsafe extern "C" fn hash_mark_i(mut mrb: *mut mrb_state, mut key: mrb_value,
                                 mut val: mrb_value, mut p: *mut libc::c_void)
 -> libc::c_int {
    if !((key.tt as libc::c_uint) <
             MRB_TT_OBJECT as libc::c_int as libc::c_uint) {
        mrb_gc_mark(mrb, key.value.p as *mut RBasic);
    }
    if !((val.tt as libc::c_uint) <
             MRB_TT_OBJECT as libc::c_int as libc::c_uint) {
        mrb_gc_mark(mrb, val.value.p as *mut RBasic);
    }
    return 0i32;
}
#[no_mangle]
pub unsafe extern "C" fn mrb_gc_mark_hash_size(mut mrb: *mut mrb_state,
                                               mut hash: *mut RHash)
 -> size_t {
    if (*hash).ht.is_null() { return 0i32 as size_t }
    return ((*(*hash).ht).size * 2i32 as libc::c_longlong) as size_t;
}
#[no_mangle]
pub unsafe extern "C" fn mrb_gc_free_hash(mut mrb: *mut mrb_state,
                                          mut hash: *mut RHash) {
    ht_free(mrb, (*hash).ht);
}
#[no_mangle]
pub unsafe extern "C" fn mrb_hash_foreach(mut mrb: *mut mrb_state,
                                          mut hash: *mut RHash,
                                          mut func:
                                              Option<unsafe extern "C" fn(_:
                                                                              *mut mrb_state,
                                                                          _:
                                                                              mrb_value,
                                                                          _:
                                                                              mrb_value,
                                                                          _:
                                                                              *mut libc::c_void)
                                                         -> libc::c_int>,
                                          mut p: *mut libc::c_void) {
    ht_foreach(mrb, (*hash).ht, func, p);
}
unsafe extern "C" fn mrb_hash_init_copy(mut mrb: *mut mrb_state,
                                        mut self_0: mrb_value) -> mrb_value {
    let mut orig: mrb_value =
        mrb_value{value: unnamed{f: 0.,}, tt: MRB_TT_FALSE,};
    let mut copy: *mut RHash = 0 as *mut RHash;
    let mut orig_h: *mut htable = 0 as *mut htable;
    let mut ifnone: mrb_value =
        mrb_value{value: unnamed{f: 0.,}, tt: MRB_TT_FALSE,};
    let mut vret: mrb_value =
        mrb_value{value: unnamed{f: 0.,}, tt: MRB_TT_FALSE,};
    mrb_get_args(mrb, b"o\x00" as *const u8 as *const libc::c_char,
                 &mut orig as *mut mrb_value);
    if 0 != mrb_obj_equal(mrb, self_0, orig) { return self_0 }
    if self_0.tt as libc::c_uint != orig.tt as libc::c_uint ||
           mrb_obj_class(mrb, self_0) != mrb_obj_class(mrb, orig) {
        mrb_raise(mrb,
                  mrb_exc_get(mrb,
                              b"TypeError\x00" as *const u8 as
                                  *const libc::c_char),
                  b"initialize_copy should take same class object\x00" as
                      *const u8 as *const libc::c_char);
    }
    orig_h = (*(self_0.value.p as *mut RHash)).ht;
    copy = mrb_obj_alloc(mrb, MRB_TT_HASH, (*mrb).hash_class) as *mut RHash;
    (*copy).ht = ht_copy(mrb, orig_h);
    if 0 != (*(self_0.value.p as *mut RHash)).flags() as libc::c_int & 1i32 {
        (*copy).set_flags((*copy).flags() | 1i32 as uint32_t)
    }
    if 0 != (*(self_0.value.p as *mut RHash)).flags() as libc::c_int & 2i32 {
        (*copy).set_flags((*copy).flags() | 2i32 as uint32_t)
    }
    vret = mrb_obj_value(copy as *mut libc::c_void);
    ifnone =
        mrb_iv_get(mrb, self_0,
                   mrb_intern_static(mrb,
                                     b"ifnone\x00" as *const u8 as
                                         *const libc::c_char,
                                     (::std::mem::size_of::<[libc::c_char; 7]>()
                                          as
                                          libc::c_ulong).wrapping_sub(1i32 as
                                                                          libc::c_ulong)));
    if !(ifnone.tt as libc::c_uint ==
             MRB_TT_FALSE as libc::c_int as libc::c_uint &&
             0 == ifnone.value.i) {
        mrb_iv_set(mrb, vret,
                   mrb_intern_static(mrb,
                                     b"ifnone\x00" as *const u8 as
                                         *const libc::c_char,
                                     (::std::mem::size_of::<[libc::c_char; 7]>()
                                          as
                                          libc::c_ulong).wrapping_sub(1i32 as
                                                                          libc::c_ulong)),
                   ifnone);
    }
    return vret;
}
unsafe extern "C" fn check_kdict_i(mut mrb: *mut mrb_state,
                                   mut key: mrb_value, mut val: mrb_value,
                                   mut data: *mut libc::c_void)
 -> libc::c_int {
    if !(key.tt as libc::c_uint ==
             MRB_TT_SYMBOL as libc::c_int as libc::c_uint) {
        mrb_raise(mrb,
                  mrb_exc_get(mrb,
                              b"ArgumentError\x00" as *const u8 as
                                  *const libc::c_char),
                  b"keyword argument hash with non symbol keys\x00" as
                      *const u8 as *const libc::c_char);
    }
    return 0i32;
}
#[no_mangle]
pub unsafe extern "C" fn mrb_hash_check_kdict(mut mrb: *mut mrb_state,
                                              mut self_0: mrb_value) {
    let mut t: *mut htable = 0 as *mut htable;
    t = (*(self_0.value.p as *mut RHash)).ht;
    if t.is_null() || (*t).size == 0i32 as libc::c_longlong { return }
    ht_foreach(mrb, t, Some(check_kdict_i), 0 as *mut libc::c_void);
}
/* 15.2.13.4.16 */
/*
 *  call-seq:
 *     Hash.new                          -> new_hash
 *     Hash.new(obj)                     -> new_hash
 *     Hash.new {|hash, key| block }     -> new_hash
 *
 *  Returns a new, empty hash. If this hash is subsequently accessed by
 *  a key that doesn't correspond to a hash entry, the value returned
 *  depends on the style of <code>new</code> used to create the hash. In
 *  the first form, the access returns <code>nil</code>. If
 *  <i>obj</i> is specified, this single object will be used for
 *  all <em>default values</em>. If a block is specified, it will be
 *  called with the hash object and the key, and should return the
 *  default value. It is the block's responsibility to store the value
 *  in the hash if required.
 *
 *      h = Hash.new("Go Fish")
 *      h["a"] = 100
 *      h["b"] = 200
 *      h["a"]           #=> 100
 *      h["c"]           #=> "Go Fish"
 *      # The following alters the single default object
 *      h["c"].upcase!   #=> "GO FISH"
 *      h["d"]           #=> "GO FISH"
 *      h.keys           #=> ["a", "b"]
 *
 *      # While this creates a new default object each time
 *      h = Hash.new { |hash, key| hash[key] = "Go Fish: #{key}" }
 *      h["c"]           #=> "Go Fish: c"
 *      h["c"].upcase!   #=> "GO FISH: C"
 *      h["d"]           #=> "Go Fish: d"
 *      h.keys           #=> ["c", "d"]
 *
 */
unsafe extern "C" fn mrb_hash_init(mut mrb: *mut mrb_state,
                                   mut hash: mrb_value) -> mrb_value {
    let mut block: mrb_value =
        mrb_value{value: unnamed{f: 0.,}, tt: MRB_TT_FALSE,};
    let mut ifnone: mrb_value =
        mrb_value{value: unnamed{f: 0.,}, tt: MRB_TT_FALSE,};
    let mut ifnone_p: mrb_bool = 0;
    ifnone = mrb_nil_value();
    mrb_get_args(mrb, b"&|o?\x00" as *const u8 as *const libc::c_char,
                 &mut block as *mut mrb_value, &mut ifnone as *mut mrb_value,
                 &mut ifnone_p as *mut mrb_bool);
    mrb_hash_modify(mrb, hash);
    if !(block.tt as libc::c_uint ==
             MRB_TT_FALSE as libc::c_int as libc::c_uint &&
             0 == block.value.i) {
        if 0 != ifnone_p {
            mrb_raise(mrb,
                      mrb_exc_get(mrb,
                                  b"ArgumentError\x00" as *const u8 as
                                      *const libc::c_char),
                      b"wrong number of arguments\x00" as *const u8 as
                          *const libc::c_char);
        }
        let ref mut fresh10 = *(hash.value.p as *mut RHash);
        (*fresh10).set_flags((*fresh10).flags() | 2i32 as uint32_t);
        ifnone = block
    }
    if !(ifnone.tt as libc::c_uint ==
             MRB_TT_FALSE as libc::c_int as libc::c_uint &&
             0 == ifnone.value.i) {
        let ref mut fresh11 = *(hash.value.p as *mut RHash);
        (*fresh11).set_flags((*fresh11).flags() | 1i32 as uint32_t);
        mrb_iv_set(mrb, hash,
                   mrb_intern_static(mrb,
                                     b"ifnone\x00" as *const u8 as
                                         *const libc::c_char,
                                     (::std::mem::size_of::<[libc::c_char; 7]>()
                                          as
                                          libc::c_ulong).wrapping_sub(1i32 as
                                                                          libc::c_ulong)),
                   ifnone);
    }
    return hash;
}
/* 15.2.13.4.2  */
/*
 *  call-seq:
 *     hsh[key]    ->  value
 *
 *  Element Reference---Retrieves the <i>value</i> object corresponding
 *  to the <i>key</i> object. If not found, returns the default value (see
 *  <code>Hash::new</code> for details).
 *
 *     h = { "a" => 100, "b" => 200 }
 *     h["a"]   #=> 100
 *     h["c"]   #=> nil
 *
 */
unsafe extern "C" fn mrb_hash_aget(mut mrb: *mut mrb_state,
                                   mut self_0: mrb_value) -> mrb_value {
    let mut key: mrb_value =
        mrb_value{value: unnamed{f: 0.,}, tt: MRB_TT_FALSE,};
    mrb_get_args(mrb, b"o\x00" as *const u8 as *const libc::c_char,
                 &mut key as *mut mrb_value);
    return mrb_hash_get(mrb, self_0, key);
}
/* 15.2.13.4.6  */
/*
 *  call-seq:
 *     hsh.default = obj     -> obj
 *
 *  Sets the default value, the value returned for a key that does not
 *  exist in the hash. It is not possible to set the default to a
 *  <code>Proc</code> that will be executed on each key lookup.
 *
 *     h = { "a" => 100, "b" => 200 }
 *     h.default = "Go fish"
 *     h["a"]     #=> 100
 *     h["z"]     #=> "Go fish"
 *     # This doesn't do what you might hope...
 *     h.default = proc do |hash, key|
 *       hash[key] = key + key
 *     end
 *     h[2]       #=> #<Proc:0x401b3948@-:6>
 *     h["cat"]   #=> #<Proc:0x401b3948@-:6>
 */
unsafe extern "C" fn mrb_hash_set_default(mut mrb: *mut mrb_state,
                                          mut hash: mrb_value) -> mrb_value {
    let mut ifnone: mrb_value =
        mrb_value{value: unnamed{f: 0.,}, tt: MRB_TT_FALSE,};
    mrb_get_args(mrb, b"o\x00" as *const u8 as *const libc::c_char,
                 &mut ifnone as *mut mrb_value);
    mrb_hash_modify(mrb, hash);
    mrb_iv_set(mrb, hash,
               mrb_intern_static(mrb,
                                 b"ifnone\x00" as *const u8 as
                                     *const libc::c_char,
                                 (::std::mem::size_of::<[libc::c_char; 7]>()
                                      as
                                      libc::c_ulong).wrapping_sub(1i32 as
                                                                      libc::c_ulong)),
               ifnone);
    let ref mut fresh12 = *(hash.value.p as *mut RHash);
    (*fresh12).set_flags((*fresh12).flags() & !2i32 as uint32_t);
    if !(ifnone.tt as libc::c_uint ==
             MRB_TT_FALSE as libc::c_int as libc::c_uint &&
             0 == ifnone.value.i) {
        let ref mut fresh13 = *(hash.value.p as *mut RHash);
        (*fresh13).set_flags((*fresh13).flags() | 1i32 as uint32_t)
    } else {
        let ref mut fresh14 = *(hash.value.p as *mut RHash);
        (*fresh14).set_flags((*fresh14).flags() & !1i32 as uint32_t)
    }
    return ifnone;
}
/* 15.2.13.4.7  */
/*
 *  call-seq:
 *     hsh.default_proc -> anObject
 *
 *  If <code>Hash::new</code> was invoked with a block, return that
 *  block, otherwise return <code>nil</code>.
 *
 *     h = Hash.new {|h,k| h[k] = k*k }   #=> {}
 *     p = h.default_proc                 #=> #<Proc:0x401b3d08@-:1>
 *     a = []                             #=> []
 *     p.call(a, 2)
 *     a                                  #=> [nil, nil, 4]
 */
unsafe extern "C" fn mrb_hash_default_proc(mut mrb: *mut mrb_state,
                                           mut hash: mrb_value) -> mrb_value {
    if 0 != (*(hash.value.p as *mut RHash)).flags() as libc::c_int & 2i32 {
        return mrb_iv_get(mrb, hash,
                          mrb_intern_static(mrb,
                                            b"ifnone\x00" as *const u8 as
                                                *const libc::c_char,
                                            (::std::mem::size_of::<[libc::c_char; 7]>()
                                                 as
                                                 libc::c_ulong).wrapping_sub(1i32
                                                                                 as
                                                                                 libc::c_ulong)))
    }
    return mrb_nil_value();
}
/*
 *  call-seq:
 *     hsh.default_proc = proc_obj     -> proc_obj
 *
 *  Sets the default proc to be executed on each key lookup.
 *
 *     h.default_proc = proc do |hash, key|
 *       hash[key] = key + key
 *     end
 *     h[2]       #=> 4
 *     h["cat"]   #=> "catcat"
 */
unsafe extern "C" fn mrb_hash_set_default_proc(mut mrb: *mut mrb_state,
                                               mut hash: mrb_value)
 -> mrb_value {
    let mut ifnone: mrb_value =
        mrb_value{value: unnamed{f: 0.,}, tt: MRB_TT_FALSE,};
    mrb_get_args(mrb, b"o\x00" as *const u8 as *const libc::c_char,
                 &mut ifnone as *mut mrb_value);
    mrb_hash_modify(mrb, hash);
    mrb_iv_set(mrb, hash,
               mrb_intern_static(mrb,
                                 b"ifnone\x00" as *const u8 as
                                     *const libc::c_char,
                                 (::std::mem::size_of::<[libc::c_char; 7]>()
                                      as
                                      libc::c_ulong).wrapping_sub(1i32 as
                                                                      libc::c_ulong)),
               ifnone);
    if !(ifnone.tt as libc::c_uint ==
             MRB_TT_FALSE as libc::c_int as libc::c_uint &&
             0 == ifnone.value.i) {
        let ref mut fresh15 = *(hash.value.p as *mut RHash);
        (*fresh15).set_flags((*fresh15).flags() | 2i32 as uint32_t);
        let ref mut fresh16 = *(hash.value.p as *mut RHash);
        (*fresh16).set_flags((*fresh16).flags() | 1i32 as uint32_t)
    } else {
        let ref mut fresh17 = *(hash.value.p as *mut RHash);
        (*fresh17).set_flags((*fresh17).flags() & !1i32 as uint32_t);
        let ref mut fresh18 = *(hash.value.p as *mut RHash);
        (*fresh18).set_flags((*fresh18).flags() & !2i32 as uint32_t)
    }
    return ifnone;
}
unsafe extern "C" fn mrb_hash_delete(mut mrb: *mut mrb_state,
                                     mut self_0: mrb_value) -> mrb_value {
    let mut key: mrb_value =
        mrb_value{value: unnamed{f: 0.,}, tt: MRB_TT_FALSE,};
    mrb_get_args(mrb, b"o\x00" as *const u8 as *const libc::c_char,
                 &mut key as *mut mrb_value);
    mrb_hash_modify(mrb, self_0);
    return mrb_hash_delete_key(mrb, self_0, key);
}
/* find first element in the hash table, and remove it. */
unsafe extern "C" fn ht_shift(mut mrb: *mut mrb_state, mut t: *mut htable,
                              mut kp: *mut mrb_value,
                              mut vp: *mut mrb_value) {
    let mut seg: *mut segment = (*t).rootseg;
    let mut i: mrb_int = 0;
    while !seg.is_null() {
        i = 0i32 as mrb_int;
        while i < (*seg).size as libc::c_longlong {
            let mut key: mrb_value =
                mrb_value{value: unnamed{f: 0.,}, tt: MRB_TT_FALSE,};
            if (*seg).next.is_null() && i >= (*t).last_len as libc::c_longlong
               {
                return
            }
            key = (*(*seg).e.as_mut_ptr().offset(i as isize)).key;
            if key.tt as libc::c_uint ==
                   MRB_TT_UNDEF as libc::c_int as libc::c_uint {
                i += 1
            } else {
                *kp = key;
                *vp = (*(*seg).e.as_mut_ptr().offset(i as isize)).val;
                (*(*seg).e.as_mut_ptr().offset(i as isize)).key =
                    mrb_undef_value();
                (*t).size -= 1;
                return
            }
        }
        seg = (*seg).next
    };
}
/* 15.2.13.4.24 */
/*
 *  call-seq:
 *     hsh.shift -> anArray or obj
 *
 *  Removes a key-value pair from <i>hsh</i> and returns it as the
 *  two-item array <code>[</code> <i>key, value</i> <code>]</code>, or
 *  the hash's default value if the hash is empty.
 *
 *      h = { 1 => "a", 2 => "b", 3 => "c" }
 *      h.shift   #=> [1, "a"]
 *      h         #=> {2=>"b", 3=>"c"}
 */
unsafe extern "C" fn mrb_hash_shift(mut mrb: *mut mrb_state,
                                    mut hash: mrb_value) -> mrb_value {
    let mut t: *mut htable = (*(hash.value.p as *mut RHash)).ht;
    mrb_hash_modify(mrb, hash);
    if !t.is_null() && (*t).size > 0i32 as libc::c_longlong {
        let mut del_key: mrb_value =
            mrb_value{value: unnamed{f: 0.,}, tt: MRB_TT_FALSE,};
        let mut del_val: mrb_value =
            mrb_value{value: unnamed{f: 0.,}, tt: MRB_TT_FALSE,};
        ht_shift(mrb, t, &mut del_key, &mut del_val);
        mrb_gc_protect(mrb, del_key);
        mrb_gc_protect(mrb, del_val);
        return mrb_assoc_new(mrb, del_key, del_val)
    }
    if 0 != (*(hash.value.p as *mut RHash)).flags() as libc::c_int & 1i32 {
        if 0 != (*(hash.value.p as *mut RHash)).flags() as libc::c_int & 2i32
           {
            return mrb_funcall(mrb,
                               mrb_iv_get(mrb, hash,
                                          mrb_intern_static(mrb,
                                                            b"ifnone\x00" as
                                                                *const u8 as
                                                                *const libc::c_char,
                                                            (::std::mem::size_of::<[libc::c_char; 7]>()
                                                                 as
                                                                 libc::c_ulong).wrapping_sub(1i32
                                                                                                 as
                                                                                                 libc::c_ulong))),
                               b"call\x00" as *const u8 as
                                   *const libc::c_char, 2i32 as mrb_int, hash,
                               mrb_nil_value())
        } else {
            return mrb_iv_get(mrb, hash,
                              mrb_intern_static(mrb,
                                                b"ifnone\x00" as *const u8 as
                                                    *const libc::c_char,
                                                (::std::mem::size_of::<[libc::c_char; 7]>()
                                                     as
                                                     libc::c_ulong).wrapping_sub(1i32
                                                                                     as
                                                                                     libc::c_ulong)))
        }
    }
    return mrb_nil_value();
}
/* 15.2.13.4.3  */
/* 15.2.13.4.26 */
/*
 *  call-seq:
 *     hsh[key] = value        -> value
 *     hsh.store(key, value)   -> value
 *
 *  Element Assignment---Associates the value given by
 *  <i>value</i> with the key given by <i>key</i>.
 *  <i>key</i> should not have its value changed while it is in
 *  use as a key (a <code>String</code> passed as a key will be
 *  duplicated and frozen).
 *
 *      h = { "a" => 100, "b" => 200 }
 *      h["a"] = 9
 *      h["c"] = 4
 *      h   #=> {"a"=>9, "b"=>200, "c"=>4}
 *
 */
unsafe extern "C" fn mrb_hash_aset(mut mrb: *mut mrb_state,
                                   mut self_0: mrb_value) -> mrb_value {
    let mut key: mrb_value =
        mrb_value{value: unnamed{f: 0.,}, tt: MRB_TT_FALSE,};
    let mut val: mrb_value =
        mrb_value{value: unnamed{f: 0.,}, tt: MRB_TT_FALSE,};
    mrb_get_args(mrb, b"oo\x00" as *const u8 as *const libc::c_char,
                 &mut key as *mut mrb_value, &mut val as *mut mrb_value);
    mrb_hash_set(mrb, self_0, key, val);
    return val;
}
/* 15.2.13.4.20 */
/* 15.2.13.4.25 */
/*
 *  call-seq:
 *     hsh.length    ->  fixnum
 *     hsh.size      ->  fixnum
 *
 *  Returns the number of key-value pairs in the hash.
 *
 *     h = { "d" => 100, "a" => 200, "v" => 300, "e" => 400 }
 *     h.length        #=> 4
 *     h.delete("a")   #=> 200
 *     h.length        #=> 3
 */
unsafe extern "C" fn mrb_hash_size_m(mut mrb: *mut mrb_state,
                                     mut self_0: mrb_value) -> mrb_value {
    let mut size: mrb_int = mrb_hash_size(mrb, self_0);
    return mrb_fixnum_value(size);
}
/* 15.2.13.4.12 */
/*
 *  call-seq:
 *     hsh.empty?    -> true or false
 *
 *  Returns <code>true</code> if <i>hsh</i> contains no key-value pairs.
 *
 *     {}.empty?   #=> true
 *
 */
unsafe extern "C" fn mrb_hash_empty_m(mut mrb: *mut mrb_state,
                                      mut self_0: mrb_value) -> mrb_value {
    return mrb_bool_value(mrb_hash_empty_p(mrb, self_0));
}
unsafe extern "C" fn mrb_hash_has_key(mut mrb: *mut mrb_state,
                                      mut hash: mrb_value) -> mrb_value {
    let mut key: mrb_value =
        mrb_value{value: unnamed{f: 0.,}, tt: MRB_TT_FALSE,};
    let mut key_p: mrb_bool = 0;
    mrb_get_args(mrb, b"o\x00" as *const u8 as *const libc::c_char,
                 &mut key as *mut mrb_value);
    key_p = mrb_hash_key_p(mrb, hash, key);
    return mrb_bool_value(key_p);
}
unsafe extern "C" fn hash_has_value_i(mut mrb: *mut mrb_state,
                                      mut key: mrb_value, mut val: mrb_value,
                                      mut p: *mut libc::c_void)
 -> libc::c_int {
    let mut arg: *mut has_v_arg = p as *mut has_v_arg;
    if 0 != mrb_equal(mrb, (*arg).val, val) {
        (*arg).found = 1i32 as mrb_bool;
        return 1i32
    }
    return 0i32;
}
/* 15.2.13.4.14 */
/* 15.2.13.4.27 */
/*
 *  call-seq:
 *     hsh.has_value?(value)    -> true or false
 *     hsh.value?(value)        -> true or false
 *
 *  Returns <code>true</code> if the given value is present for some key
 *  in <i>hsh</i>.
 *
 *     h = { "a" => 100, "b" => 200 }
 *     h.has_value?(100)   #=> true
 *     h.has_value?(999)   #=> false
 */
unsafe extern "C" fn mrb_hash_has_value(mut mrb: *mut mrb_state,
                                        mut hash: mrb_value) -> mrb_value {
    let mut val: mrb_value =
        mrb_value{value: unnamed{f: 0.,}, tt: MRB_TT_FALSE,};
    let mut arg: has_v_arg =
        has_v_arg{found: 0,
                  val: mrb_value{value: unnamed{f: 0.,}, tt: MRB_TT_FALSE,},};
    mrb_get_args(mrb, b"o\x00" as *const u8 as *const libc::c_char,
                 &mut val as *mut mrb_value);
    arg.found = 0i32 as mrb_bool;
    arg.val = val;
    ht_foreach(mrb, (*(hash.value.p as *mut RHash)).ht,
               Some(hash_has_value_i),
               &mut arg as *mut has_v_arg as *mut libc::c_void);
    return mrb_bool_value(arg.found);
}
/*
 *  call-seq:
 *    hsh.rehash -> hsh
 *
 *  Rebuilds the hash based on the current hash values for each key. If
 *  values of key objects have changed since they were inserted, this
 *  method will reindex <i>hsh</i>.
 *
 *     h = {"AAA" => "b"}
 *     h.keys[0].chop!
 *     h.rehash   #=> {"AA"=>"b"}
 *     h["AA"]    #=> "b"
 */
unsafe extern "C" fn mrb_hash_rehash(mut mrb: *mut mrb_state,
                                     mut self_0: mrb_value) -> mrb_value {
    ht_compact(mrb, (*(self_0.value.p as *mut RHash)).ht);
    return self_0;
}
#[no_mangle]
pub unsafe extern "C" fn mrb_init_hash(mut mrb: *mut mrb_state) {
    let mut h: *mut RClass = 0 as *mut RClass;
    h =
        mrb_define_class(mrb, b"Hash\x00" as *const u8 as *const libc::c_char,
                         (*mrb).object_class);
    (*mrb).hash_class = h;
    (*h).set_flags(((*h).flags() as libc::c_int & !0xffi32 |
                        MRB_TT_HASH as libc::c_int as libc::c_char as
                            libc::c_int) as uint32_t);
    mrb_define_method(mrb, h,
                      b"initialize_copy\x00" as *const u8 as
                          *const libc::c_char, Some(mrb_hash_init_copy),
                      ((1i32 & 0x1fi32) as mrb_aspec) << 18i32);
    mrb_define_method(mrb, h, b"[]\x00" as *const u8 as *const libc::c_char,
                      Some(mrb_hash_aget),
                      ((1i32 & 0x1fi32) as mrb_aspec) << 18i32);
    mrb_define_method(mrb, h, b"[]=\x00" as *const u8 as *const libc::c_char,
                      Some(mrb_hash_aset),
                      ((2i32 & 0x1fi32) as mrb_aspec) << 18i32);
    mrb_define_method(mrb, h,
                      b"clear\x00" as *const u8 as *const libc::c_char,
                      Some(mrb_hash_clear), 0i32 as mrb_aspec);
    mrb_define_method(mrb, h,
                      b"default\x00" as *const u8 as *const libc::c_char,
                      Some(mrb_hash_default), (1i32 << 12i32) as mrb_aspec);
    mrb_define_method(mrb, h,
                      b"default=\x00" as *const u8 as *const libc::c_char,
                      Some(mrb_hash_set_default),
                      ((1i32 & 0x1fi32) as mrb_aspec) << 18i32);
    mrb_define_method(mrb, h,
                      b"default_proc\x00" as *const u8 as *const libc::c_char,
                      Some(mrb_hash_default_proc), 0i32 as mrb_aspec);
    mrb_define_method(mrb, h,
                      b"default_proc=\x00" as *const u8 as
                          *const libc::c_char,
                      Some(mrb_hash_set_default_proc),
                      ((1i32 & 0x1fi32) as mrb_aspec) << 18i32);
    mrb_define_method(mrb, h,
                      b"__delete\x00" as *const u8 as *const libc::c_char,
                      Some(mrb_hash_delete),
                      ((1i32 & 0x1fi32) as mrb_aspec) << 18i32);
    mrb_define_method(mrb, h,
                      b"empty?\x00" as *const u8 as *const libc::c_char,
                      Some(mrb_hash_empty_m), 0i32 as mrb_aspec);
    mrb_define_method(mrb, h,
                      b"has_key?\x00" as *const u8 as *const libc::c_char,
                      Some(mrb_hash_has_key),
                      ((1i32 & 0x1fi32) as mrb_aspec) << 18i32);
    mrb_define_method(mrb, h,
                      b"has_value?\x00" as *const u8 as *const libc::c_char,
                      Some(mrb_hash_has_value),
                      ((1i32 & 0x1fi32) as mrb_aspec) << 18i32);
    mrb_define_method(mrb, h,
                      b"include?\x00" as *const u8 as *const libc::c_char,
                      Some(mrb_hash_has_key),
                      ((1i32 & 0x1fi32) as mrb_aspec) << 18i32);
    mrb_define_method(mrb, h,
                      b"initialize\x00" as *const u8 as *const libc::c_char,
                      Some(mrb_hash_init),
                      ((1i32 & 0x1fi32) as mrb_aspec) << 13i32);
    mrb_define_method(mrb, h, b"key?\x00" as *const u8 as *const libc::c_char,
                      Some(mrb_hash_has_key),
                      ((1i32 & 0x1fi32) as mrb_aspec) << 18i32);
    mrb_define_method(mrb, h, b"keys\x00" as *const u8 as *const libc::c_char,
                      Some(mrb_hash_keys), 0i32 as mrb_aspec);
    mrb_define_method(mrb, h,
                      b"length\x00" as *const u8 as *const libc::c_char,
                      Some(mrb_hash_size_m), 0i32 as mrb_aspec);
    mrb_define_method(mrb, h,
                      b"member?\x00" as *const u8 as *const libc::c_char,
                      Some(mrb_hash_has_key),
                      ((1i32 & 0x1fi32) as mrb_aspec) << 18i32);
    mrb_define_method(mrb, h,
                      b"shift\x00" as *const u8 as *const libc::c_char,
                      Some(mrb_hash_shift), 0i32 as mrb_aspec);
    mrb_define_method(mrb, h, b"size\x00" as *const u8 as *const libc::c_char,
                      Some(mrb_hash_size_m), 0i32 as mrb_aspec);
    mrb_define_method(mrb, h,
                      b"store\x00" as *const u8 as *const libc::c_char,
                      Some(mrb_hash_aset),
                      ((2i32 & 0x1fi32) as mrb_aspec) << 18i32);
    mrb_define_method(mrb, h,
                      b"value?\x00" as *const u8 as *const libc::c_char,
                      Some(mrb_hash_has_value),
                      ((1i32 & 0x1fi32) as mrb_aspec) << 18i32);
    mrb_define_method(mrb, h,
                      b"values\x00" as *const u8 as *const libc::c_char,
                      Some(mrb_hash_values), 0i32 as mrb_aspec);
    mrb_define_method(mrb, h,
                      b"rehash\x00" as *const u8 as *const libc::c_char,
                      Some(mrb_hash_rehash), 0i32 as mrb_aspec);
}