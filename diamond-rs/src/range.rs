use libc;
use c2rust_bitfields::BitfieldStruct;
extern "C" {
    pub type iv_tbl;
    pub type kh_mt;
    pub type symbol_name;
    pub type RProc;
    pub type REnv;
    pub type mrb_jmpbuf;
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
    fn mrb_malloc(_: *mut mrb_state, _: size_t) -> *mut libc::c_void;
    #[no_mangle]
    fn mrb_obj_alloc(_: *mut mrb_state, _: mrb_vtype, _: *mut RClass)
     -> *mut RBasic;
    #[no_mangle]
    fn mrb_obj_equal(_: *mut mrb_state, _: mrb_value, _: mrb_value)
     -> mrb_bool;
    #[no_mangle]
    fn mrb_inspect(mrb: *mut mrb_state, obj: mrb_value) -> mrb_value;
    #[no_mangle]
    fn mrb_eql(mrb: *mut mrb_state, obj1: mrb_value, obj2: mrb_value)
     -> mrb_bool;
    #[no_mangle]
    fn mrb_gc_mark(_: *mut mrb_state, _: *mut RBasic);
    #[no_mangle]
    fn mrb_write_barrier(_: *mut mrb_state, _: *mut RBasic);
    #[no_mangle]
    fn mrb_obj_class(mrb: *mut mrb_state, obj: mrb_value) -> *mut RClass;
    #[no_mangle]
    fn mrb_obj_is_kind_of(mrb: *mut mrb_state, obj: mrb_value, c: *mut RClass)
     -> mrb_bool;
    #[no_mangle]
    fn mrb_raise(mrb: *mut mrb_state, c: *mut RClass,
                 msg: *const libc::c_char);
    #[no_mangle]
    fn mrb_raisef(mrb: *mut mrb_state, c: *mut RClass,
                  fmt: *const libc::c_char, _: ...);
    #[no_mangle]
    fn mrb_name_error(mrb: *mut mrb_state, id: mrb_sym,
                      fmt: *const libc::c_char, _: ...);
    #[no_mangle]
    fn mrb_to_int(mrb: *mut mrb_state, val: mrb_value) -> mrb_value;
    #[no_mangle]
    fn mrb_obj_is_instance_of(mrb: *mut mrb_state, obj: mrb_value,
                              c: *mut RClass) -> mrb_bool;
    /*
 * Returns an object as a Ruby string.
 *
 * @param [mrb_state] mrb The current mruby state.
 * @param [mrb_value] obj An object to return as a Ruby string.
 * @return [mrb_value] An object as a Ruby string.
 */
    #[no_mangle]
    fn mrb_obj_as_string(mrb: *mut mrb_state, obj: mrb_value) -> mrb_value;
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
    fn mrb_str_cat(mrb: *mut mrb_state, str: mrb_value,
                   ptr: *const libc::c_char, len: size_t) -> mrb_value;
    #[no_mangle]
    fn mrb_str_cat_str(mrb: *mut mrb_state, str: mrb_value, str2: mrb_value)
     -> mrb_value;
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
/*
** mruby/gc.h - garbage collector for mruby
**
** See Copyright Notice in mruby.h
*/
/* *
 * Uncommon memory management stuffs.
 */
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
    #[bitfield(padding)]
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
    #[bitfield(padding)]
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
    #[bitfield(padding)]
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
    #[bitfield(padding)]
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
    #[bitfield(padding)]
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
    #[bitfield(padding)]
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
#[derive ( Copy , Clone )]
#[repr(C)]
pub struct mrb_value {
    pub value: C2RustUnnamed,
    pub tt: mrb_vtype,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub union C2RustUnnamed {
    pub f: mrb_float,
    pub p: *mut libc::c_void,
    pub i: mrb_int,
    pub sym: mrb_sym,
}
pub type mrb_int = int64_t;
pub type mrb_float = libc::c_double;
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
 * | `s`  | {String}       | char *, {mrb_int} | Receive two arguments; `s!` gives (`NULL`,`0`) for `nil`        |
 * | `z`  | {String}       | char *            | `NULL` terminated string; `z!` gives `NULL` for `nil`           |
 * | `a`  | {Array}        | {mrb_value} *, {mrb_int} | Receive two arguments; `a!` gives (`NULL`,`0`) for `nil` |
 * | `f`  | {Fixnum}/{Float} | {mrb_float}       |                                                    |
 * | `i`  | {Fixnum}/{Float} | {mrb_int}         |                                                    |
 * | `b`  | boolean        | {mrb_bool}        |                                                    |
 * | `n`  | {String}/{Symbol} | {mrb_sym}         |                                                    |
 * | `d`  | data           | void *, {mrb_data_type} const | 2nd argument will be used to check data type so it won't be modified; when `!` follows, the value may be `nil` |
 * | `I`  | inline struct  | void *          |                                                    |
 * | `&`  | block          | {mrb_value}       | &! raises exception if no block given.             |
 * | `*`  | rest arguments | {mrb_value} *, {mrb_int} | Receive the rest of arguments as an array; `*!` avoid copy of the stack.  |
 * | &vert; | optional     |                   | After this spec following specs would be optional. |
 * | `?`  | optional given | {mrb_bool}        | `TRUE` if preceding argument is given. Used to check optional argument is given. |
 *
 * @see mrb_get_args
 */
pub type mrb_args_format = *const libc::c_char;
#[derive ( BitfieldStruct , Clone , Copy )]
#[repr(C)]
pub struct RRange {
    #[bitfield(name = "tt", ty = "mrb_vtype", bits = "0..=7")]
    #[bitfield(name = "color", ty = "uint32_t", bits = "8..=10")]
    #[bitfield(name = "flags", ty = "uint32_t", bits = "11..=31")]
    pub tt_color_flags: [u8; 4],
    #[bitfield(padding)]
    pub _pad: [u8; 4],
    pub c: *mut RClass,
    pub gcnext: *mut RBasic,
    pub edges: *mut mrb_range_edges,
    pub excl: mrb_bool,
    #[bitfield(padding)]
    pub _pad2: [u8; 7],
}
/*
** mruby/range.h - Range class
**
** See Copyright Notice in mruby.h
*/
/* *
 * Range class
 */
#[derive ( Copy , Clone )]
#[repr(C)]
pub struct mrb_range_edges {
    pub beg: mrb_value,
    pub end: mrb_value,
}
pub type mrb_range_beg_len = libc::c_uint;
/* (failure) out of range */
pub const MRB_RANGE_OUT: mrb_range_beg_len = 2;
/* (success) range */
pub const MRB_RANGE_OK: mrb_range_beg_len = 1;
/* (failure) not range */
pub const MRB_RANGE_TYPE_MISMATCH: mrb_range_beg_len = 0;
#[inline]
unsafe extern "C" fn mrb_obj_value(mut p: *mut libc::c_void) -> mrb_value {
    let mut v: mrb_value =
        mrb_value{value: C2RustUnnamed{f: 0.,}, tt: MRB_TT_FALSE,};
    v.tt = (*(p as *mut RBasic as *mut RObject)).tt();
    v.value.p = p as *mut RBasic as *mut libc::c_void;
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
        mrb_value{value: C2RustUnnamed{f: 0.,}, tt: MRB_TT_FALSE,};
    v.tt = MRB_TT_FALSE;
    v.value.i = 0i32 as mrb_int;
    return v;
}
/*
 * Returns false in Ruby.
 */
#[inline]
unsafe extern "C" fn mrb_false_value() -> mrb_value {
    let mut v: mrb_value =
        mrb_value{value: C2RustUnnamed{f: 0.,}, tt: MRB_TT_FALSE,};
    v.tt = MRB_TT_FALSE;
    v.value.i = 1i32 as mrb_int;
    return v;
}
/*
 * Returns true in Ruby.
 */
#[inline]
unsafe extern "C" fn mrb_true_value() -> mrb_value {
    let mut v: mrb_value =
        mrb_value{value: C2RustUnnamed{f: 0.,}, tt: MRB_TT_FALSE,};
    v.tt = MRB_TT_TRUE;
    v.value.i = 1i32 as mrb_int;
    return v;
}
#[inline]
unsafe extern "C" fn mrb_bool_value(mut boolean: mrb_bool) -> mrb_value {
    let mut v: mrb_value =
        mrb_value{value: C2RustUnnamed{f: 0.,}, tt: MRB_TT_FALSE,};
    v.tt =
        (if 0 != boolean as libc::c_int {
             MRB_TT_TRUE as libc::c_int
         } else { MRB_TT_FALSE as libc::c_int }) as mrb_vtype;
    v.value.i = 1i32 as mrb_int;
    return v;
}
unsafe extern "C" fn r_check(mut mrb: *mut mrb_state, mut a: mrb_value,
                             mut b: mrb_value) {
    let mut ans: mrb_value =
        mrb_value{value: C2RustUnnamed{f: 0.,}, tt: MRB_TT_FALSE,};
    let mut ta: mrb_vtype = MRB_TT_FALSE;
    let mut tb: mrb_vtype = MRB_TT_FALSE;
    ta = a.tt;
    tb = b.tt;
    if (ta as libc::c_uint == MRB_TT_FIXNUM as libc::c_int as libc::c_uint ||
            ta as libc::c_uint == MRB_TT_FLOAT as libc::c_int as libc::c_uint)
           &&
           (tb as libc::c_uint == MRB_TT_FIXNUM as libc::c_int as libc::c_uint
                ||
                tb as libc::c_uint ==
                    MRB_TT_FLOAT as libc::c_int as libc::c_uint) {
        return
    }
    ans =
        mrb_funcall(mrb, a, b"<=>\x00" as *const u8 as *const libc::c_char,
                    1i32 as mrb_int, b);
    if ans.tt as libc::c_uint == MRB_TT_FALSE as libc::c_int as libc::c_uint
           && 0 == ans.value.i {
        /* can not be compared */
        mrb_raise(mrb,
                  mrb_exc_get(mrb,
                              b"ArgumentError\x00" as *const u8 as
                                  *const libc::c_char),
                  b"bad value for range\x00" as *const u8 as
                      *const libc::c_char);
    };
}
unsafe extern "C" fn r_le(mut mrb: *mut mrb_state, mut a: mrb_value,
                          mut b: mrb_value) -> mrb_bool {
    /* compare result */
    let mut r: mrb_value =
        mrb_funcall(mrb, a, b"<=>\x00" as *const u8 as *const libc::c_char,
                    1i32 as mrb_int, b);
    /* output :a < b => -1, a = b =>  0, a > b => +1 */
    if r.tt as libc::c_uint == MRB_TT_FIXNUM as libc::c_int as libc::c_uint {
        let mut c: mrb_int = r.value.i;
        if c == 0i32 as libc::c_longlong || c == -1i32 as libc::c_longlong {
            return 1i32 as mrb_bool
        }
    }
    return 0i32 as mrb_bool;
}
unsafe extern "C" fn r_gt(mut mrb: *mut mrb_state, mut a: mrb_value,
                          mut b: mrb_value) -> mrb_bool {
    let mut r: mrb_value =
        mrb_funcall(mrb, a, b"<=>\x00" as *const u8 as *const libc::c_char,
                    1i32 as mrb_int, b);
    /* output :a < b => -1, a = b =>  0, a > b => +1 */
    return (r.tt as libc::c_uint ==
                MRB_TT_FIXNUM as libc::c_int as libc::c_uint &&
                r.value.i == 1i32 as libc::c_longlong) as libc::c_int as
               mrb_bool;
}
unsafe extern "C" fn r_ge(mut mrb: *mut mrb_state, mut a: mrb_value,
                          mut b: mrb_value) -> mrb_bool {
    /* compare result */
    let mut r: mrb_value =
        mrb_funcall(mrb, a, b"<=>\x00" as *const u8 as *const libc::c_char,
                    1i32 as mrb_int, b);
    /* output :a < b => -1, a = b =>  0, a > b => +1 */
    if r.tt as libc::c_uint == MRB_TT_FIXNUM as libc::c_int as libc::c_uint {
        let mut c: mrb_int = r.value.i;
        if c == 0i32 as libc::c_longlong || c == 1i32 as libc::c_longlong {
            return 1i32 as mrb_bool
        }
    }
    return 0i32 as mrb_bool;
}
unsafe extern "C" fn range_ptr_alloc_edges(mut mrb: *mut mrb_state,
                                           mut r: *mut RRange) {
    (*r).edges =
        mrb_malloc(mrb,
                   ::std::mem::size_of::<mrb_range_edges>() as libc::c_ulong)
            as *mut mrb_range_edges;
}
unsafe extern "C" fn range_ptr_init(mut mrb: *mut mrb_state,
                                    mut r: *mut RRange, mut beg: mrb_value,
                                    mut end: mrb_value, mut excl: mrb_bool)
 -> *mut RRange {
    r_check(mrb, beg, end);
    if !r.is_null() {
        if 0 != (*r).flags() as libc::c_int & 1i32 {
            /* Ranges are immutable, so that they should be initialized only once. */
            mrb_name_error(mrb,
                           mrb_intern_static(mrb,
                                             b"initialize\x00" as *const u8 as
                                                 *const libc::c_char,
                                             (::std::mem::size_of::<[libc::c_char; 11]>()
                                                  as
                                                  libc::c_ulong).wrapping_sub(1i32
                                                                                  as
                                                                                  libc::c_ulong)),
                           b"\'initialize\' called twice\x00" as *const u8 as
                               *const libc::c_char);
        } else { range_ptr_alloc_edges(mrb, r); }
    } else {
        r =
            mrb_obj_alloc(mrb, MRB_TT_RANGE, (*mrb).range_class) as
                *mut RRange;
        range_ptr_alloc_edges(mrb, r);
    }
    (*(*r).edges).beg = beg;
    (*(*r).edges).end = end;
    (*r).excl = excl;
    (*r).set_flags((*r).flags() | 1i32 as uint32_t);
    return r;
}
unsafe extern "C" fn range_ptr_replace(mut mrb: *mut mrb_state,
                                       mut r: *mut RRange, mut beg: mrb_value,
                                       mut end: mrb_value,
                                       mut excl: mrb_bool) {
    range_ptr_init(mrb, r, beg, end, excl);
    mrb_write_barrier(mrb, r as *mut RBasic);
}
/*
 *  call-seq:
 *     rng.first    => obj
 *     rng.begin    => obj
 *
 *  Returns the first object in <i>rng</i>.
 */
unsafe extern "C" fn range_beg(mut mrb: *mut mrb_state, mut range: mrb_value)
 -> mrb_value {
    return (*(*mrb_range_ptr(mrb, range)).edges).beg;
}
/*
 *  call-seq:
 *     rng.end    => obj
 *     rng.last   => obj
 *
 *  Returns the object that defines the end of <i>rng</i>.
 *
 *     (1..10).end    #=> 10
 *     (1...10).end   #=> 10
 */
unsafe extern "C" fn range_end(mut mrb: *mut mrb_state, mut range: mrb_value)
 -> mrb_value {
    return (*(*mrb_range_ptr(mrb, range)).edges).end;
}
/*
 *  call-seq:
 *     range.exclude_end?    => true or false
 *
 *  Returns <code>true</code> if <i>range</i> excludes its end value.
 */
unsafe extern "C" fn range_excl(mut mrb: *mut mrb_state, mut range: mrb_value)
 -> mrb_value {
    return mrb_bool_value((*mrb_range_ptr(mrb, range)).excl);
}
/*
 *  call-seq:
 *     Range.new(start, end, exclusive=false)    => range
 *
 *  Constructs a range using the given <i>start</i> and <i>end</i>. If the third
 *  parameter is omitted or is <code>false</code>, the <i>range</i> will include
 *  the end object; otherwise, it will be excluded.
 */
unsafe extern "C" fn range_initialize(mut mrb: *mut mrb_state,
                                      mut range: mrb_value) -> mrb_value {
    let mut beg: mrb_value =
        mrb_value{value: C2RustUnnamed{f: 0.,}, tt: MRB_TT_FALSE,};
    let mut end: mrb_value =
        mrb_value{value: C2RustUnnamed{f: 0.,}, tt: MRB_TT_FALSE,};
    let mut exclusive: mrb_bool = 0i32 as mrb_bool;
    mrb_get_args(mrb, b"oo|b\x00" as *const u8 as *const libc::c_char,
                 &mut beg as *mut mrb_value, &mut end as *mut mrb_value,
                 &mut exclusive as *mut mrb_bool);
    range_ptr_replace(mrb, range.value.p as *mut RRange, beg, end, exclusive);
    return range;
}
/*
 *  call-seq:
 *     range == obj    => true or false
 *
 *  Returns <code>true</code> only if
 *  1) <i>obj</i> is a Range,
 *  2) <i>obj</i> has equivalent beginning and end items (by comparing them with <code>==</code>),
 *  3) <i>obj</i> has the same #exclude_end? setting as <i>rng</t>.
 *
 *    (0..2) == (0..2)            #=> true
 *    (0..2) == Range.new(0,2)    #=> true
 *    (0..2) == (0...2)           #=> false
 */
unsafe extern "C" fn range_eq(mut mrb: *mut mrb_state, mut range: mrb_value)
 -> mrb_value {
    let mut rr: *mut RRange = 0 as *mut RRange;
    let mut ro: *mut RRange = 0 as *mut RRange;
    let mut obj: mrb_value =
        mrb_value{value: C2RustUnnamed{f: 0.,}, tt: MRB_TT_FALSE,};
    let mut v1: mrb_value =
        mrb_value{value: C2RustUnnamed{f: 0.,}, tt: MRB_TT_FALSE,};
    let mut v2: mrb_value =
        mrb_value{value: C2RustUnnamed{f: 0.,}, tt: MRB_TT_FALSE,};
    mrb_get_args(mrb, b"o\x00" as *const u8 as *const libc::c_char,
                 &mut obj as *mut mrb_value);
    if 0 != mrb_obj_equal(mrb, range, obj) { return mrb_true_value() }
    if 0 == mrb_obj_is_instance_of(mrb, obj, mrb_obj_class(mrb, range)) {
        /* same class? */
        return mrb_false_value()
    }
    rr = mrb_range_ptr(mrb, range);
    ro = mrb_range_ptr(mrb, obj);
    v1 =
        mrb_funcall(mrb, (*(*rr).edges).beg,
                    b"==\x00" as *const u8 as *const libc::c_char,
                    1i32 as mrb_int, (*(*ro).edges).beg);
    v2 =
        mrb_funcall(mrb, (*(*rr).edges).end,
                    b"==\x00" as *const u8 as *const libc::c_char,
                    1i32 as mrb_int, (*(*ro).edges).end);
    if !(v1.tt as libc::c_uint != MRB_TT_FALSE as libc::c_int as libc::c_uint)
           ||
           !(v2.tt as libc::c_uint !=
                 MRB_TT_FALSE as libc::c_int as libc::c_uint) ||
           (*rr).excl as libc::c_int != (*ro).excl as libc::c_int {
        return mrb_false_value()
    }
    return mrb_true_value();
}
/*
 *  call-seq:
 *     range === obj       =>  true or false
 *     range.member?(val)  =>  true or false
 *     range.include?(val) =>  true or false
 */
unsafe extern "C" fn range_include(mut mrb: *mut mrb_state,
                                   mut range: mrb_value) -> mrb_value {
    let mut val: mrb_value =
        mrb_value{value: C2RustUnnamed{f: 0.,}, tt: MRB_TT_FALSE,};
    let mut r: *mut RRange = mrb_range_ptr(mrb, range);
    let mut beg: mrb_value =
        mrb_value{value: C2RustUnnamed{f: 0.,}, tt: MRB_TT_FALSE,};
    let mut end: mrb_value =
        mrb_value{value: C2RustUnnamed{f: 0.,}, tt: MRB_TT_FALSE,};
    let mut include_p: mrb_bool = 0;
    mrb_get_args(mrb, b"o\x00" as *const u8 as *const libc::c_char,
                 &mut val as *mut mrb_value);
    beg = (*(*r).edges).beg;
    end = (*(*r).edges).end;
    /* beg <= val */
    include_p =
        (0 != r_le(mrb, beg, val) as libc::c_int &&
             0 !=
                 (if 0 != (*r).excl as libc::c_int {
                      r_gt(mrb, end, val) as libc::c_int
                  } else { r_ge(mrb, end, val) as libc::c_int })) as
            libc::c_int as mrb_bool;
    /* end >  val */
    /* end >= val */
    return mrb_bool_value(include_p);
}
/* 15.2.14.4.12(x) */
/*
 * call-seq:
 *   rng.to_s   -> string
 *
 * Convert this range object to a printable form.
 */
unsafe extern "C" fn range_to_s(mut mrb: *mut mrb_state, mut range: mrb_value)
 -> mrb_value {
    let mut str: mrb_value =
        mrb_value{value: C2RustUnnamed{f: 0.,}, tt: MRB_TT_FALSE,};
    let mut str2: mrb_value =
        mrb_value{value: C2RustUnnamed{f: 0.,}, tt: MRB_TT_FALSE,};
    let mut r: *mut RRange = mrb_range_ptr(mrb, range);
    str = mrb_obj_as_string(mrb, (*(*r).edges).beg);
    str2 = mrb_obj_as_string(mrb, (*(*r).edges).end);
    str = mrb_str_dup(mrb, str);
    mrb_str_cat(mrb, str, b"...\x00" as *const u8 as *const libc::c_char,
                (if 0 != (*r).excl as libc::c_int { 3i32 } else { 2i32 }) as
                    size_t);
    mrb_str_cat_str(mrb, str, str2);
    return str;
}
/* 15.2.14.4.13(x) */
/*
 * call-seq:
 *   rng.inspect  -> string
 *
 * Convert this range object to a printable form (using
 * <code>inspect</code> to convert the start and end
 * objects).
 */
unsafe extern "C" fn range_inspect(mut mrb: *mut mrb_state,
                                   mut range: mrb_value) -> mrb_value {
    let mut str: mrb_value =
        mrb_value{value: C2RustUnnamed{f: 0.,}, tt: MRB_TT_FALSE,};
    let mut str2: mrb_value =
        mrb_value{value: C2RustUnnamed{f: 0.,}, tt: MRB_TT_FALSE,};
    let mut r: *mut RRange = mrb_range_ptr(mrb, range);
    str = mrb_inspect(mrb, (*(*r).edges).beg);
    str2 = mrb_inspect(mrb, (*(*r).edges).end);
    str = mrb_str_dup(mrb, str);
    mrb_str_cat(mrb, str, b"...\x00" as *const u8 as *const libc::c_char,
                (if 0 != (*r).excl as libc::c_int { 3i32 } else { 2i32 }) as
                    size_t);
    mrb_str_cat_str(mrb, str, str2);
    return str;
}
/* 15.2.14.4.14(x) */
/*
 *  call-seq:
 *     rng.eql?(obj)    -> true or false
 *
 *  Returns <code>true</code> only if <i>obj</i> is a Range, has equivalent
 *  beginning and end items (by comparing them with #eql?), and has the same
 *  #exclude_end? setting as <i>rng</i>.
 *
 *    (0..2).eql?(0..2)            #=> true
 *    (0..2).eql?(Range.new(0,2))  #=> true
 *    (0..2).eql?(0...2)           #=> false
 */
unsafe extern "C" fn range_eql(mut mrb: *mut mrb_state, mut range: mrb_value)
 -> mrb_value {
    let mut obj: mrb_value =
        mrb_value{value: C2RustUnnamed{f: 0.,}, tt: MRB_TT_FALSE,};
    let mut r: *mut RRange = 0 as *mut RRange;
    let mut o: *mut RRange = 0 as *mut RRange;
    mrb_get_args(mrb, b"o\x00" as *const u8 as *const libc::c_char,
                 &mut obj as *mut mrb_value);
    if 0 != mrb_obj_equal(mrb, range, obj) { return mrb_true_value() }
    if 0 == mrb_obj_is_kind_of(mrb, obj, (*mrb).range_class) {
        return mrb_false_value()
    }
    if obj.tt as libc::c_uint != MRB_TT_RANGE as libc::c_int as libc::c_uint {
        return mrb_false_value()
    }
    r = mrb_range_ptr(mrb, range);
    o = mrb_range_ptr(mrb, obj);
    if 0 == mrb_eql(mrb, (*(*r).edges).beg, (*(*o).edges).beg) ||
           0 == mrb_eql(mrb, (*(*r).edges).end, (*(*o).edges).end) ||
           (*r).excl as libc::c_int != (*o).excl as libc::c_int {
        return mrb_false_value()
    }
    return mrb_true_value();
}
/* 15.2.14.4.15(x) */
unsafe extern "C" fn range_initialize_copy(mut mrb: *mut mrb_state,
                                           mut copy: mrb_value) -> mrb_value {
    let mut src: mrb_value =
        mrb_value{value: C2RustUnnamed{f: 0.,}, tt: MRB_TT_FALSE,};
    let mut r: *mut RRange = 0 as *mut RRange;
    mrb_get_args(mrb, b"o\x00" as *const u8 as *const libc::c_char,
                 &mut src as *mut mrb_value);
    if 0 != mrb_obj_equal(mrb, copy, src) { return copy }
    if 0 == mrb_obj_is_instance_of(mrb, src, mrb_obj_class(mrb, copy)) {
        mrb_raise(mrb,
                  mrb_exc_get(mrb,
                              b"TypeError\x00" as *const u8 as
                                  *const libc::c_char),
                  b"wrong argument class\x00" as *const u8 as
                      *const libc::c_char);
    }
    r = mrb_range_ptr(mrb, src);
    range_ptr_replace(mrb, copy.value.p as *mut RRange, (*(*r).edges).beg,
                      (*(*r).edges).end, (*r).excl);
    return copy;
}
#[no_mangle]
pub unsafe extern "C" fn mrb_get_values_at(mut mrb: *mut mrb_state,
                                           mut obj: mrb_value,
                                           mut olen: mrb_int,
                                           mut argc: mrb_int,
                                           mut argv: *const mrb_value,
                                           mut func:
                                               Option<unsafe extern "C" fn(_:
                                                                               *mut mrb_state,
                                                                           _:
                                                                               mrb_value,
                                                                           _:
                                                                               mrb_int)
                                                          -> mrb_value>)
 -> mrb_value {
    let mut i: mrb_int = 0;
    let mut j: mrb_int = 0;
    let mut beg: mrb_int = 0;
    let mut len: mrb_int = 0;
    let mut result: mrb_value =
        mrb_value{value: C2RustUnnamed{f: 0.,}, tt: MRB_TT_FALSE,};
    result = mrb_ary_new(mrb);
    i = 0i32 as mrb_int;
    while i < argc {
        if (*argv.offset(i as isize)).tt as libc::c_uint ==
               MRB_TT_FIXNUM as libc::c_int as libc::c_uint {
            mrb_ary_push(mrb, result,
                         func.expect("non-null function pointer")(mrb, obj,
                                                                  (*argv.offset(i
                                                                                    as
                                                                                    isize)).value.i));
        } else if mrb_range_beg_len(mrb, *argv.offset(i as isize), &mut beg,
                                    &mut len, olen, 0i32 as mrb_bool) as
                      libc::c_uint ==
                      MRB_RANGE_OK as libc::c_int as libc::c_uint {
            let end: mrb_int =
                if olen < beg + len { olen } else { beg + len };
            j = beg;
            while j < end {
                mrb_ary_push(mrb, result,
                             func.expect("non-null function pointer")(mrb,
                                                                      obj,
                                                                      j));
                j += 1
            }
            while j < beg + len {
                mrb_ary_push(mrb, result, mrb_nil_value());
                j += 1
            }
        } else {
            mrb_raisef(mrb,
                       mrb_exc_get(mrb,
                                   b"TypeError\x00" as *const u8 as
                                       *const libc::c_char),
                       b"invalid values selector: %v\x00" as *const u8 as
                           *const libc::c_char, *argv.offset(i as isize));
        }
        i += 1
    }
    return result;
}
#[no_mangle]
pub unsafe extern "C" fn mrb_gc_mark_range(mut mrb: *mut mrb_state,
                                           mut r: *mut RRange) {
    if 0 != (*r).flags() as libc::c_int & 1i32 {
        if !(((*(*r).edges).beg.tt as libc::c_uint) <
                 MRB_TT_OBJECT as libc::c_int as libc::c_uint) {
            mrb_gc_mark(mrb, (*(*r).edges).beg.value.p as *mut RBasic);
        }
        if !(((*(*r).edges).end.tt as libc::c_uint) <
                 MRB_TT_OBJECT as libc::c_int as libc::c_uint) {
            mrb_gc_mark(mrb, (*(*r).edges).end.value.p as *mut RBasic);
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn mrb_range_ptr(mut mrb: *mut mrb_state,
                                       mut range: mrb_value) -> *mut RRange {
    let mut r: *mut RRange = range.value.p as *mut RRange;
    /* check for if #initialize_copy was removed [#3320] */
    if 0 == (*r).flags() as libc::c_int & 1i32 {
        mrb_raise(mrb,
                  mrb_exc_get(mrb,
                              b"ArgumentError\x00" as *const u8 as
                                  *const libc::c_char),
                  b"uninitialized range\x00" as *const u8 as
                      *const libc::c_char);
    }
    return r;
}
/*
 * Initializes a Range.
 *
 * If the third parameter is FALSE then it includes the last value in the range.
 * If the third parameter is TRUE then it excludes the last value in the range.
 *
 * @param start the beginning value.
 * @param end the ending value.
 * @param exclude represents the inclusion or exclusion of the last value.
 */
#[no_mangle]
pub unsafe extern "C" fn mrb_range_new(mut mrb: *mut mrb_state,
                                       mut beg: mrb_value, mut end: mrb_value,
                                       mut excl: mrb_bool) -> mrb_value {
    let mut r: *mut RRange =
        range_ptr_init(mrb, 0 as *mut RRange, beg, end, excl);
    return mrb_obj_value(r as *mut libc::c_void);
}
#[no_mangle]
pub unsafe extern "C" fn mrb_range_beg_len(mut mrb: *mut mrb_state,
                                           mut range: mrb_value,
                                           mut begp: *mut mrb_int,
                                           mut lenp: *mut mrb_int,
                                           mut len: mrb_int,
                                           mut trunc: mrb_bool)
 -> mrb_range_beg_len {
    let mut beg: mrb_int = 0;
    let mut end: mrb_int = 0;
    let mut r: *mut RRange = 0 as *mut RRange;
    if range.tt as libc::c_uint != MRB_TT_RANGE as libc::c_int as libc::c_uint
       {
        return MRB_RANGE_TYPE_MISMATCH
    }
    r = mrb_range_ptr(mrb, range);
    beg = mrb_to_int(mrb, (*(*r).edges).beg).value.i;
    end = mrb_to_int(mrb, (*(*r).edges).end).value.i;
    if beg < 0i32 as libc::c_longlong {
        beg += len;
        if beg < 0i32 as libc::c_longlong { return MRB_RANGE_OUT }
    }
    if 0 != trunc {
        if beg > len { return MRB_RANGE_OUT }
        if end > len { end = len }
    }
    if end < 0i32 as libc::c_longlong { end += len }
    if 0 == (*r).excl && (0 == trunc || end < len) {
        /* include end point */
        end += 1
    }
    len = end - beg;
    if len < 0i32 as libc::c_longlong { len = 0i32 as mrb_int }
    *begp = beg;
    *lenp = len;
    return MRB_RANGE_OK;
}
#[no_mangle]
pub unsafe extern "C" fn mrb_init_range(mut mrb: *mut mrb_state) {
    let mut r: *mut RClass = 0 as *mut RClass;
    /* 15.2.14 */
    r =
        mrb_define_class(mrb,
                         b"Range\x00" as *const u8 as *const libc::c_char,
                         (*mrb).object_class);
    (*mrb).range_class = r;
    (*r).set_flags(((*r).flags() as libc::c_int & !0xffi32 |
                        MRB_TT_RANGE as libc::c_int as libc::c_char as
                            libc::c_int) as uint32_t);
    /* 15.2.14.4.3  */
    mrb_define_method(mrb, r,
                      b"begin\x00" as *const u8 as *const libc::c_char,
                      Some(range_beg as
                               unsafe extern "C" fn(_: *mut mrb_state,
                                                    _: mrb_value)
                                   -> mrb_value), 0i32 as mrb_aspec);
    /* 15.2.14.4.5  */
    mrb_define_method(mrb, r, b"end\x00" as *const u8 as *const libc::c_char,
                      Some(range_end as
                               unsafe extern "C" fn(_: *mut mrb_state,
                                                    _: mrb_value)
                                   -> mrb_value), 0i32 as mrb_aspec);
    /* 15.2.14.4.1  */
    mrb_define_method(mrb, r, b"==\x00" as *const u8 as *const libc::c_char,
                      Some(range_eq as
                               unsafe extern "C" fn(_: *mut mrb_state,
                                                    _: mrb_value)
                                   -> mrb_value),
                      ((1i32 & 0x1fi32) as mrb_aspec) << 18i32);
    /* 15.2.14.4.2  */
    mrb_define_method(mrb, r, b"===\x00" as *const u8 as *const libc::c_char,
                      Some(range_include as
                               unsafe extern "C" fn(_: *mut mrb_state,
                                                    _: mrb_value)
                                   -> mrb_value),
                      ((1i32 & 0x1fi32) as mrb_aspec) << 18i32);
    /* 15.2.14.4.6  */
    mrb_define_method(mrb, r,
                      b"exclude_end?\x00" as *const u8 as *const libc::c_char,
                      Some(range_excl as
                               unsafe extern "C" fn(_: *mut mrb_state,
                                                    _: mrb_value)
                                   -> mrb_value), 0i32 as mrb_aspec);
    /* 15.2.14.4.7  */
    mrb_define_method(mrb, r,
                      b"first\x00" as *const u8 as *const libc::c_char,
                      Some(range_beg as
                               unsafe extern "C" fn(_: *mut mrb_state,
                                                    _: mrb_value)
                                   -> mrb_value), 0i32 as mrb_aspec);
    /* 15.2.14.4.8  */
    mrb_define_method(mrb, r,
                      b"include?\x00" as *const u8 as *const libc::c_char,
                      Some(range_include as
                               unsafe extern "C" fn(_: *mut mrb_state,
                                                    _: mrb_value)
                                   -> mrb_value),
                      ((1i32 & 0x1fi32) as mrb_aspec) << 18i32);
    /* 15.2.14.4.9  */
    mrb_define_method(mrb, r,
                      b"initialize\x00" as *const u8 as *const libc::c_char,
                      Some(range_initialize as
                               unsafe extern "C" fn(_: *mut mrb_state,
                                                    _: mrb_value)
                                   -> mrb_value),
                      (1i32 << 12i32) as mrb_aspec);
    /* 15.2.14.4.10 */
    mrb_define_method(mrb, r, b"last\x00" as *const u8 as *const libc::c_char,
                      Some(range_end as
                               unsafe extern "C" fn(_: *mut mrb_state,
                                                    _: mrb_value)
                                   -> mrb_value), 0i32 as mrb_aspec);
    /* 15.2.14.4.11 */
    mrb_define_method(mrb, r,
                      b"member?\x00" as *const u8 as *const libc::c_char,
                      Some(range_include as
                               unsafe extern "C" fn(_: *mut mrb_state,
                                                    _: mrb_value)
                                   -> mrb_value),
                      ((1i32 & 0x1fi32) as mrb_aspec) << 18i32);
    /* 15.2.14.4.12(x) */
    mrb_define_method(mrb, r, b"to_s\x00" as *const u8 as *const libc::c_char,
                      Some(range_to_s as
                               unsafe extern "C" fn(_: *mut mrb_state,
                                                    _: mrb_value)
                                   -> mrb_value), 0i32 as mrb_aspec);
    /* 15.2.14.4.13(x) */
    mrb_define_method(mrb, r,
                      b"inspect\x00" as *const u8 as *const libc::c_char,
                      Some(range_inspect as
                               unsafe extern "C" fn(_: *mut mrb_state,
                                                    _: mrb_value)
                                   -> mrb_value), 0i32 as mrb_aspec);
    /* 15.2.14.4.14(x) */
    mrb_define_method(mrb, r, b"eql?\x00" as *const u8 as *const libc::c_char,
                      Some(range_eql as
                               unsafe extern "C" fn(_: *mut mrb_state,
                                                    _: mrb_value)
                                   -> mrb_value),
                      ((1i32 & 0x1fi32) as mrb_aspec) << 18i32);
    /* 15.2.14.4.15(x) */
    mrb_define_method(mrb, r,
                      b"initialize_copy\x00" as *const u8 as
                          *const libc::c_char,
                      Some(range_initialize_copy as
                               unsafe extern "C" fn(_: *mut mrb_state,
                                                    _: mrb_value)
                                   -> mrb_value),
                      ((1i32 & 0x1fi32) as mrb_aspec) << 18i32);
}