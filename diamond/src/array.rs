use libc;
use c2rust_bitfields::{BitfieldStruct};
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
 * Defines a class method.
 *
 * Example:
 *
 *     # Ruby style
 *     class Foo
 *       def Foo.bar
 *       end
 *     end
 *     // C style
 *     mrb_value bar_method(mrb_state* mrb, mrb_value self){
 *       return mrb_nil_value();
 *     }
 *     void mrb_example_gem_init(mrb_state* mrb){
 *       struct RClass *foo;
 *       foo = mrb_define_class(mrb, "Foo", mrb->object_class);
 *       mrb_define_class_method(mrb, foo, "bar", bar_method, MRB_ARGS_NONE());
 *     }
 * @param [mrb_state *] mrb_state* The MRuby state reference.
 * @param [struct RClass *] RClass* The class where the class method will be defined.
 * @param [const char *] char* The name of the class method being defined.
 * @param [mrb_func_t] mrb_func_t The function pointer to the class method definition.
 * @param [mrb_aspec] mrb_aspec The method parameters declaration.
 */
    #[no_mangle]
    fn mrb_define_class_method(_: *mut mrb_state, _: *mut RClass,
                               _: *const libc::c_char, _: mrb_func_t,
                               _: mrb_aspec);
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
    /* *
 * Retrieve number of arguments from mrb_state.
 *
 * Correctly handles *splat arguments.
 */
    #[no_mangle]
    fn mrb_get_argc(mrb: *mut mrb_state) -> mrb_int;
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
    fn mrb_realloc(_: *mut mrb_state, _: *mut libc::c_void, _: size_t)
     -> *mut libc::c_void;
    #[no_mangle]
    fn mrb_obj_alloc(_: *mut mrb_state, _: mrb_vtype, _: *mut RClass)
     -> *mut RBasic;
    #[no_mangle]
    fn mrb_free(_: *mut mrb_state, _: *mut libc::c_void);
    #[no_mangle]
    fn mrb_obj_equal(_: *mut mrb_state, _: mrb_value, _: mrb_value)
     -> mrb_bool;
    #[no_mangle]
    fn mrb_equal(mrb: *mut mrb_state, obj1: mrb_value, obj2: mrb_value)
     -> mrb_bool;
    #[no_mangle]
    fn mrb_field_write_barrier(_: *mut mrb_state, _: *mut RBasic,
                               _: *mut RBasic);
    #[no_mangle]
    fn mrb_write_barrier(_: *mut mrb_state, _: *mut RBasic);
    #[no_mangle]
    fn mrb_raise(mrb: *mut mrb_state, c: *mut RClass,
                 msg: *const libc::c_char) -> !;
    #[no_mangle]
    fn mrb_raisef(mrb: *mut mrb_state, c: *mut RClass,
                  fmt: *const libc::c_char, _: ...) -> !;
    #[no_mangle]
    fn mrb_frozen_error(mrb: *mut mrb_state, frozen_obj: *mut libc::c_void)
     -> !;
    #[no_mangle]
    fn mrb_respond_to(mrb: *mut mrb_state, obj: mrb_value, mid: mrb_sym)
     -> mrb_bool;
    #[no_mangle]
    fn mrb_ensure_array_type(mrb: *mut mrb_state, self_0: mrb_value)
     -> mrb_value;
    #[no_mangle]
    fn mrb_check_array_type(mrb: *mut mrb_state, self_0: mrb_value)
     -> mrb_value;
    #[no_mangle]
    fn mrb_str_cat_str(mrb: *mut mrb_state, str: mrb_value, str2: mrb_value)
     -> mrb_value;
    /*
 * Returns an object as a Ruby string.
 *
 * @param [mrb_state] mrb The current mruby state.
 * @param [mrb_value] obj An object to return as a Ruby string.
 * @return [mrb_value] An object as a Ruby string.
 */
    #[no_mangle]
    fn mrb_obj_as_string(mrb: *mut mrb_state, obj: mrb_value) -> mrb_value;
    #[no_mangle]
    fn mrb_check_string_type(mrb: *mut mrb_state, str: mrb_value)
     -> mrb_value;
    #[no_mangle]
    fn mrb_str_new_capa(mrb: *mut mrb_state, capa: size_t) -> mrb_value;
    #[no_mangle]
    fn mrb_range_beg_len(mrb: *mut mrb_state, range: mrb_value,
                         begp: *mut mrb_int, lenp: *mut mrb_int, len: mrb_int,
                         trunc: mrb_bool) -> mrb_range_beg_len;
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
    pub as_0: unnamed_0,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub union unnamed_0 {
    pub heap: unnamed_1,
}
#[derive ( Copy , Clone )]
#[repr(C)]
pub struct unnamed_1 {
    pub len: mrb_int,
    pub aux: unnamed_2,
    pub ptr: *mut mrb_value,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub union unnamed_2 {
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
/*
 * Returns false in Ruby.
 */
#[inline]
unsafe extern "C" fn mrb_false_value() -> mrb_value {
    let mut v: mrb_value =
        mrb_value{value: unnamed{f: 0.,}, tt: MRB_TT_FALSE,};
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
        mrb_value{value: unnamed{f: 0.,}, tt: MRB_TT_FALSE,};
    v.tt = MRB_TT_TRUE;
    v.value.i = 1i32 as mrb_int;
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
unsafe extern "C" fn mrb_check_frozen(mut mrb: *mut mrb_state,
                                      mut o: *mut libc::c_void) {
    if 0 != (*(o as *mut RBasic)).flags() as libc::c_int & 1i32 << 20i32 {
        mrb_frozen_error(mrb, o);
    };
}
#[no_mangle]
pub unsafe extern "C" fn mrb_ary_decref(mut mrb: *mut mrb_state,
                                        mut shared: *mut mrb_shared_array) {
    (*shared).refcnt -= 1;
    if (*shared).refcnt == 0i32 {
        mrb_free(mrb, (*shared).ptr as *mut libc::c_void);
        mrb_free(mrb, shared as *mut libc::c_void);
    };
}
#[no_mangle]
pub unsafe extern "C" fn mrb_ary_modify(mut mrb: *mut mrb_state,
                                        mut a: *mut RArray) {
    mrb_write_barrier(mrb, a as *mut RBasic);
    ary_modify(mrb, a);
}
unsafe extern "C" fn ary_modify(mut mrb: *mut mrb_state, mut a: *mut RArray) {
    ary_modify_check(mrb, a);
    if 0 != (*a).flags() as libc::c_int & 256i32 {
        let mut shared: *mut mrb_shared_array = (*a).as_0.heap.aux.shared;
        if (*shared).refcnt == 1i32 && (*a).as_0.heap.ptr == (*shared).ptr {
            (*a).as_0.heap.ptr = (*shared).ptr;
            (*a).as_0.heap.aux.capa = (*a).as_0.heap.len;
            mrb_free(mrb, shared as *mut libc::c_void);
        } else {
            let mut ptr: *mut mrb_value = 0 as *mut mrb_value;
            let mut p: *mut mrb_value = 0 as *mut mrb_value;
            let mut len: mrb_int = 0;
            p = (*a).as_0.heap.ptr;
            len =
                ((*a).as_0.heap.len as
                     libc::c_ulonglong).wrapping_mul(::std::mem::size_of::<mrb_value>()
                                                         as libc::c_ulong as
                                                         libc::c_ulonglong) as
                    mrb_int;
            ptr = mrb_malloc(mrb, len as size_t) as *mut mrb_value;
            if !p.is_null() { array_copy(ptr, p, (*a).as_0.heap.len); }
            (*a).as_0.heap.ptr = ptr;
            (*a).as_0.heap.aux.capa = (*a).as_0.heap.len;
            mrb_ary_decref(mrb, shared);
        }
        (*a).set_flags((*a).flags() & !256i32 as uint32_t)
    };
}
/*
 * to copy array, use this instead of memcpy because of portability
 * * gcc on ARM may fail optimization of memcpy
 *   http://infocenter.arm.com/help/index.jsp?topic=/com.arm.doc.faqs/ka3934.html
 * * gcc on MIPS also fail
 *   http://gcc.gnu.org/bugzilla/show_bug.cgi?id=39755
 * * memcpy doesn't exist on freestanding environment
 *
 * If you optimize for binary size, use memcpy instead of this at your own risk
 * of above portability issue.
 *
 * see also http://togetter.com/li/462898
 *
 */
#[inline]
unsafe extern "C" fn array_copy(mut dst: *mut mrb_value,
                                mut src: *const mrb_value,
                                mut size: mrb_int) {
    let mut i: mrb_int = 0;
    i = 0i32 as mrb_int;
    while i < size {
        *dst.offset(i as isize) = *src.offset(i as isize);
        i += 1
    };
}
unsafe extern "C" fn ary_modify_check(mut mrb: *mut mrb_state,
                                      mut a: *mut RArray) {
    mrb_check_frozen(mrb, a as *mut libc::c_void);
}
#[no_mangle]
pub unsafe extern "C" fn mrb_ary_new_capa(mut mrb: *mut mrb_state,
                                          mut capa: mrb_int) -> mrb_value {
    let mut a: *mut RArray = ary_new_capa(mrb, capa);
    return mrb_obj_value(a as *mut libc::c_void);
}
/*
** array.c - Array class
**
** See Copyright Notice in mruby.h
*/
/* must be larger than 2 */
unsafe extern "C" fn ary_new_capa(mut mrb: *mut mrb_state, mut capa: mrb_int)
 -> *mut RArray {
    let mut a: *mut RArray = 0 as *mut RArray;
    let mut blen: size_t = 0;
    if capa >
           (if 18446744073709551615u64.wrapping_div(::std::mem::size_of::<mrb_value>()
                                                        as libc::c_ulong) <
                   (9223372036854775807i64 >> 0i32) as size_t {
                18446744073709551615u64.wrapping_div(::std::mem::size_of::<mrb_value>()
                                                         as libc::c_ulong) as
                    libc::c_ulonglong
            } else {
                ((9223372036854775807i64 >> 0i32) - 1i32 as libc::c_longlong)
                    as libc::c_ulonglong
            }) as mrb_int {
        mrb_raise(mrb,
                  mrb_exc_get(mrb,
                              b"ArgumentError\x00" as *const u8 as
                                  *const libc::c_char),
                  b"array size too big\x00" as *const u8 as
                      *const libc::c_char);
    }
    blen =
        (capa as
             libc::c_ulonglong).wrapping_mul(::std::mem::size_of::<mrb_value>()
                                                 as libc::c_ulong as
                                                 libc::c_ulonglong) as size_t;
    a = mrb_obj_alloc(mrb, MRB_TT_ARRAY, (*mrb).array_class) as *mut RArray;
    if capa <=
           (::std::mem::size_of::<*mut libc::c_void>() as
                libc::c_ulong).wrapping_mul(3i32 as
                                                libc::c_ulong).wrapping_div(::std::mem::size_of::<mrb_value>()
                                                                                as
                                                                                libc::c_ulong)
               as mrb_int {
        (*a).set_flags(((*a).flags() as libc::c_int & !7i32) as libc::c_uint |
                           (0i32 as
                                uint32_t).wrapping_add(1i32 as libc::c_uint))
    } else {
        (*a).as_0.heap.ptr = mrb_malloc(mrb, blen) as *mut mrb_value;
        (*a).as_0.heap.aux.capa = capa;
        (*a).as_0.heap.len = 0i32 as mrb_int
    }
    return a;
}
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
pub unsafe extern "C" fn mrb_ary_new(mut mrb: *mut mrb_state) -> mrb_value {
    return mrb_ary_new_capa(mrb, 0i32 as mrb_int);
}
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
pub unsafe extern "C" fn mrb_ary_new_from_values(mut mrb: *mut mrb_state,
                                                 mut size: mrb_int,
                                                 mut vals: *const mrb_value)
 -> mrb_value {
    let mut a: *mut RArray = ary_new_from_values(mrb, size, vals);
    return mrb_obj_value(a as *mut libc::c_void);
}
unsafe extern "C" fn ary_new_from_values(mut mrb: *mut mrb_state,
                                         mut size: mrb_int,
                                         mut vals: *const mrb_value)
 -> *mut RArray {
    let mut a: *mut RArray = ary_new_capa(mrb, size);
    array_copy(if 0 != (*a).flags() as libc::c_int & 7i32 {
                   &mut (*a).as_0 as *mut unnamed_0 as *mut mrb_value
               } else { (*a).as_0.heap.ptr }, vals, size);
    if 0 != (*a).flags() as libc::c_int & 7i32 {
        (*a).set_flags(((*a).flags() as libc::c_int & !7i32) as libc::c_uint |
                           (size as
                                uint32_t).wrapping_add(1i32 as libc::c_uint))
    } else { (*a).as_0.heap.len = size }
    return a;
}
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
pub unsafe extern "C" fn mrb_assoc_new(mut mrb: *mut mrb_state,
                                       mut car: mrb_value, mut cdr: mrb_value)
 -> mrb_value {
    let mut a: *mut RArray = 0 as *mut RArray;
    a = ary_new_capa(mrb, 2i32 as mrb_int);
    *if 0 != (*a).flags() as libc::c_int & 7i32 {
         &mut (*a).as_0 as *mut unnamed_0 as *mut mrb_value
     } else { (*a).as_0.heap.ptr }.offset(0isize) = car;
    *if 0 != (*a).flags() as libc::c_int & 7i32 {
         &mut (*a).as_0 as *mut unnamed_0 as *mut mrb_value
     } else { (*a).as_0.heap.ptr }.offset(1isize) = cdr;
    if 0 != (*a).flags() as libc::c_int & 7i32 {
        (*a).set_flags(((*a).flags() as libc::c_int & !7i32) as libc::c_uint |
                           (2i32 as
                                uint32_t).wrapping_add(1i32 as libc::c_uint))
    } else { (*a).as_0.heap.len = 2i32 as mrb_int }
    return mrb_obj_value(a as *mut libc::c_void);
}
/*
 * Concatenate two arrays. The target array will be modified
 *
 * Equivalent to:
 *      ary.concat(other)
 *
 * @param mrb The mruby state reference.
 * @param self The target array.
 * @param other The array that will be concatenated to self.
 */
#[no_mangle]
pub unsafe extern "C" fn mrb_ary_concat(mut mrb: *mut mrb_state,
                                        mut self_0: mrb_value,
                                        mut other: mrb_value) {
    let mut a2: *mut RArray = other.value.p as *mut RArray;
    ary_concat(mrb, self_0.value.p as *mut RArray, a2);
}
unsafe extern "C" fn ary_concat(mut mrb: *mut mrb_state, mut a: *mut RArray,
                                mut a2: *mut RArray) {
    let mut len: mrb_int = 0;
    if if 0 != (*a).flags() as libc::c_int & 7i32 {
           (((*a).flags() as libc::c_int & 7i32) - 1i32) as mrb_int
       } else { (*a).as_0.heap.len } == 0i32 as libc::c_longlong {
        ary_replace(mrb, a, a2);
        return
    }
    if if 0 != (*a2).flags() as libc::c_int & 7i32 {
           (((*a2).flags() as libc::c_int & 7i32) - 1i32) as mrb_int
       } else { (*a2).as_0.heap.len } >
           (if 18446744073709551615u64.wrapping_div(::std::mem::size_of::<mrb_value>()
                                                        as libc::c_ulong) <
                   (9223372036854775807i64 >> 0i32) as size_t {
                18446744073709551615u64.wrapping_div(::std::mem::size_of::<mrb_value>()
                                                         as libc::c_ulong) as
                    libc::c_ulonglong
            } else {
                ((9223372036854775807i64 >> 0i32) - 1i32 as libc::c_longlong)
                    as libc::c_ulonglong
            }) as mrb_int -
               if 0 != (*a).flags() as libc::c_int & 7i32 {
                   (((*a).flags() as libc::c_int & 7i32) - 1i32) as mrb_int
               } else { (*a).as_0.heap.len } {
        mrb_raise(mrb,
                  mrb_exc_get(mrb,
                              b"ArgumentError\x00" as *const u8 as
                                  *const libc::c_char),
                  b"array size too big\x00" as *const u8 as
                      *const libc::c_char);
    }
    len =
        if 0 != (*a).flags() as libc::c_int & 7i32 {
            (((*a).flags() as libc::c_int & 7i32) - 1i32) as mrb_int
        } else { (*a).as_0.heap.len } +
            if 0 != (*a2).flags() as libc::c_int & 7i32 {
                (((*a2).flags() as libc::c_int & 7i32) - 1i32) as mrb_int
            } else { (*a2).as_0.heap.len };
    ary_modify(mrb, a);
    if if 0 != (*a).flags() as libc::c_int & 7i32 {
           (::std::mem::size_of::<*mut libc::c_void>() as
                libc::c_ulong).wrapping_mul(3i32 as
                                                libc::c_ulong).wrapping_div(::std::mem::size_of::<mrb_value>()
                                                                                as
                                                                                libc::c_ulong)
               as mrb_int
       } else { (*a).as_0.heap.aux.capa } < len {
        ary_expand_capa(mrb, a, len);
    }
    array_copy(if 0 != (*a).flags() as libc::c_int & 7i32 {
                   &mut (*a).as_0 as *mut unnamed_0 as *mut mrb_value
               } else {
                   (*a).as_0.heap.ptr
               }.offset((if 0 != (*a).flags() as libc::c_int & 7i32 {
                             (((*a).flags() as libc::c_int & 7i32) - 1i32) as
                                 mrb_int
                         } else { (*a).as_0.heap.len }) as isize),
               if 0 != (*a2).flags() as libc::c_int & 7i32 {
                   &mut (*a2).as_0 as *mut unnamed_0 as *mut mrb_value
               } else { (*a2).as_0.heap.ptr },
               if 0 != (*a2).flags() as libc::c_int & 7i32 {
                   (((*a2).flags() as libc::c_int & 7i32) - 1i32) as mrb_int
               } else { (*a2).as_0.heap.len });
    mrb_write_barrier(mrb, a as *mut RBasic);
    if 0 != (*a).flags() as libc::c_int & 7i32 {
        (*a).set_flags(((*a).flags() as libc::c_int & !7i32) as libc::c_uint |
                           (len as
                                uint32_t).wrapping_add(1i32 as libc::c_uint))
    } else { (*a).as_0.heap.len = len };
}
unsafe extern "C" fn ary_expand_capa(mut mrb: *mut mrb_state,
                                     mut a: *mut RArray, mut len: mrb_int) {
    let mut capa: mrb_int =
        if 0 != (*a).flags() as libc::c_int & 7i32 {
            (::std::mem::size_of::<*mut libc::c_void>() as
                 libc::c_ulong).wrapping_mul(3i32 as
                                                 libc::c_ulong).wrapping_div(::std::mem::size_of::<mrb_value>()
                                                                                 as
                                                                                 libc::c_ulong)
                as mrb_int
        } else { (*a).as_0.heap.aux.capa };
    if !(len >
             (if 18446744073709551615u64.wrapping_div(::std::mem::size_of::<mrb_value>()
                                                          as libc::c_ulong) <
                     (9223372036854775807i64 >> 0i32) as size_t {
                  18446744073709551615u64.wrapping_div(::std::mem::size_of::<mrb_value>()
                                                           as libc::c_ulong)
                      as libc::c_ulonglong
              } else {
                  ((9223372036854775807i64 >> 0i32) -
                       1i32 as libc::c_longlong) as libc::c_ulonglong
              }) as mrb_int || len < 0i32 as libc::c_longlong) {
        if capa < 4i32 as libc::c_longlong { capa = 4i32 as mrb_int }
        while capa < len {
            if capa <=
                   (if 18446744073709551615u64.wrapping_div(::std::mem::size_of::<mrb_value>()
                                                                as
                                                                libc::c_ulong)
                           < (9223372036854775807i64 >> 0i32) as size_t {
                        18446744073709551615u64.wrapping_div(::std::mem::size_of::<mrb_value>()
                                                                 as
                                                                 libc::c_ulong)
                            as libc::c_ulonglong
                    } else {
                        ((9223372036854775807i64 >> 0i32) -
                             1i32 as libc::c_longlong) as libc::c_ulonglong
                    }) as mrb_int / 2i32 as libc::c_longlong {
                capa *= 2i32 as libc::c_longlong
            } else { capa = len }
        }
        if !(capa < len ||
                 capa >
                     (if 18446744073709551615u64.wrapping_div(::std::mem::size_of::<mrb_value>()
                                                                  as
                                                                  libc::c_ulong)
                             < (9223372036854775807i64 >> 0i32) as size_t {
                          18446744073709551615u64.wrapping_div(::std::mem::size_of::<mrb_value>()
                                                                   as
                                                                   libc::c_ulong)
                              as libc::c_ulonglong
                      } else {
                          ((9223372036854775807i64 >> 0i32) -
                               1i32 as libc::c_longlong) as libc::c_ulonglong
                      }) as mrb_int) {
            if 0 != (*a).flags() as libc::c_int & 7i32 {
                let mut ptr: *mut mrb_value =
                    &mut (*a).as_0 as *mut unnamed_0 as *mut mrb_value;
                let mut len_0: mrb_int =
                    (((*a).flags() as libc::c_int & 7i32) - 1i32) as mrb_int;
                let mut expanded_ptr: *mut mrb_value =
                    mrb_malloc(mrb,
                               (::std::mem::size_of::<mrb_value>() as
                                    libc::c_ulong as
                                    libc::c_ulonglong).wrapping_mul(capa as
                                                                        libc::c_ulonglong)
                                   as size_t) as *mut mrb_value;
                (*a).set_flags((*a).flags() & !7i32 as uint32_t);
                array_copy(expanded_ptr, ptr, len_0);
                (*a).as_0.heap.len = len_0;
                (*a).as_0.heap.aux.capa = capa;
                (*a).as_0.heap.ptr = expanded_ptr
            } else if capa > (*a).as_0.heap.aux.capa {
                let mut expanded_ptr_0: *mut mrb_value =
                    mrb_realloc(mrb, (*a).as_0.heap.ptr as *mut libc::c_void,
                                (::std::mem::size_of::<mrb_value>() as
                                     libc::c_ulong as
                                     libc::c_ulonglong).wrapping_mul(capa as
                                                                         libc::c_ulonglong)
                                    as size_t) as *mut mrb_value;
                (*a).as_0.heap.aux.capa = capa;
                (*a).as_0.heap.ptr = expanded_ptr_0
            }
            return;
        }
    }
    mrb_raise(mrb,
              mrb_exc_get(mrb,
                          b"ArgumentError\x00" as *const u8 as
                              *const libc::c_char),
              b"array size too big\x00" as *const u8 as *const libc::c_char);
}
unsafe extern "C" fn ary_replace(mut mrb: *mut mrb_state, mut a: *mut RArray,
                                 mut b: *mut RArray) {
    let mut len: mrb_int =
        if 0 != (*b).flags() as libc::c_int & 7i32 {
            (((*b).flags() as libc::c_int & 7i32) - 1i32) as mrb_int
        } else { (*b).as_0.heap.len };
    ary_modify_check(mrb, a);
    if a == b { return }
    if 0 != (*a).flags() as libc::c_int & 256i32 {
        mrb_ary_decref(mrb, (*a).as_0.heap.aux.shared);
        (*a).as_0.heap.aux.capa = 0i32 as mrb_int;
        (*a).as_0.heap.len = 0i32 as mrb_int;
        (*a).as_0.heap.ptr = 0 as *mut mrb_value;
        (*a).set_flags((*a).flags() & !256i32 as uint32_t)
    }
    if !(0 != (*b).flags() as libc::c_int & 256i32) {
        if 0 == (*b).flags() as libc::c_int & 1i32 << 20i32 &&
               len > 20i32 as libc::c_longlong {
            ary_make_shared(mrb, b);
        } else {
            if if 0 != (*a).flags() as libc::c_int & 7i32 {
                   (::std::mem::size_of::<*mut libc::c_void>() as
                        libc::c_ulong).wrapping_mul(3i32 as
                                                        libc::c_ulong).wrapping_div(::std::mem::size_of::<mrb_value>()
                                                                                        as
                                                                                        libc::c_ulong)
                       as mrb_int
               } else { (*a).as_0.heap.aux.capa } < len {
                ary_expand_capa(mrb, a, len);
            }
            array_copy(if 0 != (*a).flags() as libc::c_int & 7i32 {
                           &mut (*a).as_0 as *mut unnamed_0 as *mut mrb_value
                       } else { (*a).as_0.heap.ptr },
                       if 0 != (*b).flags() as libc::c_int & 7i32 {
                           &mut (*b).as_0 as *mut unnamed_0 as *mut mrb_value
                       } else { (*b).as_0.heap.ptr }, len);
            mrb_write_barrier(mrb, a as *mut RBasic);
            if 0 != (*a).flags() as libc::c_int & 7i32 {
                (*a).set_flags(((*a).flags() as libc::c_int & !7i32) as
                                   libc::c_uint |
                                   (len as
                                        uint32_t).wrapping_add(1i32 as
                                                                   libc::c_uint))
            } else { (*a).as_0.heap.len = len }
            return;
        }
    }
    if 0 != (*a).flags() as libc::c_int & 7i32 {
        (*a).set_flags((*a).flags() & !7i32 as uint32_t)
    } else { mrb_free(mrb, (*a).as_0.heap.ptr as *mut libc::c_void); }
    (*a).as_0.heap.ptr = (*b).as_0.heap.ptr;
    (*a).as_0.heap.len = len;
    (*a).as_0.heap.aux.shared = (*b).as_0.heap.aux.shared;
    (*(*a).as_0.heap.aux.shared).refcnt += 1;
    (*a).set_flags((*a).flags() | 256i32 as uint32_t);
    mrb_write_barrier(mrb, a as *mut RBasic);
}
unsafe extern "C" fn ary_make_shared(mut mrb: *mut mrb_state,
                                     mut a: *mut RArray) {
    if 0 == (*a).flags() as libc::c_int & 256i32 &&
           0 == (*a).flags() as libc::c_int & 7i32 {
        let mut shared: *mut mrb_shared_array =
            mrb_malloc(mrb,
                       ::std::mem::size_of::<mrb_shared_array>() as
                           libc::c_ulong) as *mut mrb_shared_array;
        let mut ptr: *mut mrb_value = (*a).as_0.heap.ptr;
        let mut len: mrb_int = (*a).as_0.heap.len;
        (*shared).refcnt = 1i32;
        if (*a).as_0.heap.aux.capa > len {
            (*shared).ptr =
                mrb_realloc(mrb, ptr as *mut libc::c_void,
                            (::std::mem::size_of::<mrb_value>() as
                                 libc::c_ulong as
                                 libc::c_ulonglong).wrapping_mul(len as
                                                                     libc::c_ulonglong).wrapping_add(1i32
                                                                                                         as
                                                                                                         libc::c_ulonglong)
                                as size_t) as *mut mrb_value;
            (*a).as_0.heap.ptr = (*shared).ptr
        } else { (*shared).ptr = ptr }
        (*shared).len = len;
        (*a).as_0.heap.aux.shared = shared;
        (*a).set_flags((*a).flags() | 256i32 as uint32_t)
    };
}
/*
 * Create an array from the input. It tries calling to_a on the
 * value. If value does not respond to that, it creates a new
 * array with just this value.
 *
 * @param mrb The mruby state reference.
 * @param value The value to change into an array.
 * @return An array representation of value.
 */
#[no_mangle]
pub unsafe extern "C" fn mrb_ary_splat(mut mrb: *mut mrb_state,
                                       mut v: mrb_value) -> mrb_value {
    let mut a: mrb_value =
        mrb_value{value: unnamed{f: 0.,}, tt: MRB_TT_FALSE,};
    if v.tt as libc::c_uint == MRB_TT_ARRAY as libc::c_int as libc::c_uint {
        return v
    }
    if 0 ==
           mrb_respond_to(mrb, v,
                          mrb_intern_static(mrb,
                                            b"to_a\x00" as *const u8 as
                                                *const libc::c_char,
                                            (::std::mem::size_of::<[libc::c_char; 5]>()
                                                 as
                                                 libc::c_ulong).wrapping_sub(1i32
                                                                                 as
                                                                                 libc::c_ulong)))
       {
        return mrb_ary_new_from_values(mrb, 1i32 as mrb_int, &mut v)
    }
    a =
        mrb_funcall(mrb, v, b"to_a\x00" as *const u8 as *const libc::c_char,
                    0i32 as mrb_int);
    if a.tt as libc::c_uint == MRB_TT_FALSE as libc::c_int as libc::c_uint &&
           0 == a.value.i {
        return mrb_ary_new_from_values(mrb, 1i32 as mrb_int, &mut v)
    }
    mrb_ensure_array_type(mrb, a);
    return a;
}
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
pub unsafe extern "C" fn mrb_ary_push(mut mrb: *mut mrb_state,
                                      mut ary: mrb_value,
                                      mut elem: mrb_value) {
    let mut a: *mut RArray = ary.value.p as *mut RArray;
    let mut len: mrb_int =
        if 0 != (*a).flags() as libc::c_int & 7i32 {
            (((*a).flags() as libc::c_int & 7i32) - 1i32) as mrb_int
        } else { (*a).as_0.heap.len };
    ary_modify(mrb, a);
    if len ==
           if 0 != (*a).flags() as libc::c_int & 7i32 {
               (::std::mem::size_of::<*mut libc::c_void>() as
                    libc::c_ulong).wrapping_mul(3i32 as
                                                    libc::c_ulong).wrapping_div(::std::mem::size_of::<mrb_value>()
                                                                                    as
                                                                                    libc::c_ulong)
                   as mrb_int
           } else { (*a).as_0.heap.aux.capa } {
        ary_expand_capa(mrb, a, len + 1i32 as libc::c_longlong);
    }
    *if 0 != (*a).flags() as libc::c_int & 7i32 {
         &mut (*a).as_0 as *mut unnamed_0 as *mut mrb_value
     } else { (*a).as_0.heap.ptr }.offset(len as isize) = elem;
    if 0 != (*a).flags() as libc::c_int & 7i32 {
        (*a).set_flags(((*a).flags() as libc::c_int & !7i32) as libc::c_uint |
                           ((len + 1i32 as libc::c_longlong) as
                                uint32_t).wrapping_add(1i32 as libc::c_uint))
    } else { (*a).as_0.heap.len = len + 1i32 as libc::c_longlong }
    if !((elem.tt as libc::c_uint) <
             MRB_TT_OBJECT as libc::c_int as libc::c_uint) {
        mrb_field_write_barrier(mrb, a as *mut RBasic,
                                elem.value.p as *mut RBasic);
    };
}
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
pub unsafe extern "C" fn mrb_ary_pop(mut mrb: *mut mrb_state,
                                     mut ary: mrb_value) -> mrb_value {
    let mut a: *mut RArray = ary.value.p as *mut RArray;
    let mut len: mrb_int =
        if 0 != (*a).flags() as libc::c_int & 7i32 {
            (((*a).flags() as libc::c_int & 7i32) - 1i32) as mrb_int
        } else { (*a).as_0.heap.len };
    ary_modify_check(mrb, a);
    if len == 0i32 as libc::c_longlong { return mrb_nil_value() }
    if 0 != (*a).flags() as libc::c_int & 7i32 {
        (*a).set_flags(((*a).flags() as libc::c_int & !7i32) as libc::c_uint |
                           ((len - 1i32 as libc::c_longlong) as
                                uint32_t).wrapping_add(1i32 as libc::c_uint))
    } else { (*a).as_0.heap.len = len - 1i32 as libc::c_longlong }
    return *if 0 != (*a).flags() as libc::c_int & 7i32 {
                &mut (*a).as_0 as *mut unnamed_0 as *mut mrb_value
            } else {
                (*a).as_0.heap.ptr
            }.offset((len - 1i32 as libc::c_longlong) as isize);
}
/*
 * Returns a reference to an element of the array on the given index.
 *
 * Equivalent to:
 *
 *      ary[n]
 *
 * @param mrb The mruby state reference.
 * @param ary The target array.
 * @param n The array index being referenced
 * @return The referenced value.
 */
#[no_mangle]
pub unsafe extern "C" fn mrb_ary_ref(mut mrb: *mut mrb_state,
                                     mut ary: mrb_value, mut n: mrb_int)
 -> mrb_value {
    let mut a: *mut RArray = ary.value.p as *mut RArray;
    let mut len: mrb_int =
        if 0 != (*a).flags() as libc::c_int & 7i32 {
            (((*a).flags() as libc::c_int & 7i32) - 1i32) as mrb_int
        } else { (*a).as_0.heap.len };
    if n < 0i32 as libc::c_longlong { n += len }
    if n < 0i32 as libc::c_longlong || len <= n { return mrb_nil_value() }
    return *if 0 != (*a).flags() as libc::c_int & 7i32 {
                &mut (*a).as_0 as *mut unnamed_0 as *mut mrb_value
            } else { (*a).as_0.heap.ptr }.offset(n as isize);
}
/*
 * Sets a value on an array at the given index
 *
 * Equivalent to:
 *
 *      ary[n] = val
 *
 * @param mrb The mruby state reference.
 * @param ary The target array.
 * @param n The array index being referenced.
 * @param val The value being setted.
 */
#[no_mangle]
pub unsafe extern "C" fn mrb_ary_set(mut mrb: *mut mrb_state,
                                     mut ary: mrb_value, mut n: mrb_int,
                                     mut val: mrb_value) {
    let mut a: *mut RArray = ary.value.p as *mut RArray;
    let mut len: mrb_int =
        if 0 != (*a).flags() as libc::c_int & 7i32 {
            (((*a).flags() as libc::c_int & 7i32) - 1i32) as mrb_int
        } else { (*a).as_0.heap.len };
    ary_modify(mrb, a);
    if n < 0i32 as libc::c_longlong {
        n += len;
        if n < 0i32 as libc::c_longlong {
            mrb_raisef(mrb,
                       mrb_exc_get(mrb,
                                   b"IndexError\x00" as *const u8 as
                                       *const libc::c_char),
                       b"index %S out of array\x00" as *const u8 as
                           *const libc::c_char, mrb_fixnum_value(n - len));
        }
    }
    if len <= n {
        if if 0 != (*a).flags() as libc::c_int & 7i32 {
               (::std::mem::size_of::<*mut libc::c_void>() as
                    libc::c_ulong).wrapping_mul(3i32 as
                                                    libc::c_ulong).wrapping_div(::std::mem::size_of::<mrb_value>()
                                                                                    as
                                                                                    libc::c_ulong)
                   as mrb_int
           } else { (*a).as_0.heap.aux.capa } <= n {
            ary_expand_capa(mrb, a, n + 1i32 as libc::c_longlong);
        }
        ary_fill_with_nil(if 0 != (*a).flags() as libc::c_int & 7i32 {
                              &mut (*a).as_0 as *mut unnamed_0 as
                                  *mut mrb_value
                          } else { (*a).as_0.heap.ptr }.offset(len as isize),
                          n + 1i32 as libc::c_longlong - len);
        if 0 != (*a).flags() as libc::c_int & 7i32 {
            (*a).set_flags(((*a).flags() as libc::c_int & !7i32) as
                               libc::c_uint |
                               ((n + 1i32 as libc::c_longlong) as
                                    uint32_t).wrapping_add(1i32 as
                                                               libc::c_uint))
        } else { (*a).as_0.heap.len = n + 1i32 as libc::c_longlong }
    }
    *if 0 != (*a).flags() as libc::c_int & 7i32 {
         &mut (*a).as_0 as *mut unnamed_0 as *mut mrb_value
     } else { (*a).as_0.heap.ptr }.offset(n as isize) = val;
    if !((val.tt as libc::c_uint) <
             MRB_TT_OBJECT as libc::c_int as libc::c_uint) {
        mrb_field_write_barrier(mrb, a as *mut RBasic,
                                val.value.p as *mut RBasic);
    };
}
unsafe extern "C" fn ary_fill_with_nil(mut ptr: *mut mrb_value,
                                       mut size: mrb_int) {
    let mut nil: mrb_value = mrb_nil_value();
    loop  {
        let fresh0 = size;
        size = size - 1;
        if !(0 != fresh0) { break ; }
        let fresh1 = ptr;
        ptr = ptr.offset(1);
        *fresh1 = nil
    };
}
/*
 * Replace the array with another array
 *
 * Equivalent to:
 *
 *      ary.replace(other)
 *
 * @param mrb The mruby state reference
 * @param self The target array.
 * @param other The array to replace it with.
 */
#[no_mangle]
pub unsafe extern "C" fn mrb_ary_replace(mut mrb: *mut mrb_state,
                                         mut self_0: mrb_value,
                                         mut other: mrb_value) {
    let mut a1: *mut RArray = self_0.value.p as *mut RArray;
    let mut a2: *mut RArray = other.value.p as *mut RArray;
    if a1 != a2 { ary_replace(mrb, a1, a2); };
}
/*
 * Unshift an element into the array
 *
 * Equivalent to:
 *
 *     ary.unshift(item)
 *
 * @param mrb The mruby state reference.
 * @param self The target array.
 * @param item The item to unshift.
 */
#[no_mangle]
pub unsafe extern "C" fn mrb_ary_unshift(mut mrb: *mut mrb_state,
                                         mut self_0: mrb_value,
                                         mut item: mrb_value) -> mrb_value {
    let mut a: *mut RArray = self_0.value.p as *mut RArray;
    let mut len: mrb_int =
        if 0 != (*a).flags() as libc::c_int & 7i32 {
            (((*a).flags() as libc::c_int & 7i32) - 1i32) as mrb_int
        } else { (*a).as_0.heap.len };
    if 0 != (*a).flags() as libc::c_int & 256i32 &&
           (*(*a).as_0.heap.aux.shared).refcnt == 1i32 &&
           (*a).as_0.heap.ptr.wrapping_offset_from((*(*a).as_0.heap.aux.shared).ptr)
               as libc::c_long >= 1i32 as libc::c_long {
        (*a).as_0.heap.ptr = (*a).as_0.heap.ptr.offset(-1isize);
        *(*a).as_0.heap.ptr.offset(0isize) = item
    } else {
        let mut ptr: *mut mrb_value = 0 as *mut mrb_value;
        ary_modify(mrb, a);
        if if 0 != (*a).flags() as libc::c_int & 7i32 {
               (::std::mem::size_of::<*mut libc::c_void>() as
                    libc::c_ulong).wrapping_mul(3i32 as
                                                    libc::c_ulong).wrapping_div(::std::mem::size_of::<mrb_value>()
                                                                                    as
                                                                                    libc::c_ulong)
                   as mrb_int
           } else { (*a).as_0.heap.aux.capa } < len + 1i32 as libc::c_longlong
           {
            ary_expand_capa(mrb, a, len + 1i32 as libc::c_longlong);
        }
        ptr =
            if 0 != (*a).flags() as libc::c_int & 7i32 {
                &mut (*a).as_0 as *mut unnamed_0 as *mut mrb_value
            } else { (*a).as_0.heap.ptr };
        value_move(ptr.offset(1isize), ptr, len as size_t);
        *ptr.offset(0isize) = item
    }
    if 0 != (*a).flags() as libc::c_int & 7i32 {
        (*a).set_flags(((*a).flags() as libc::c_int & !7i32) as libc::c_uint |
                           ((len + 1i32 as libc::c_longlong) as
                                uint32_t).wrapping_add(1i32 as libc::c_uint))
    } else { (*a).as_0.heap.len = len + 1i32 as libc::c_longlong }
    if !((item.tt as libc::c_uint) <
             MRB_TT_OBJECT as libc::c_int as libc::c_uint) {
        mrb_field_write_barrier(mrb, a as *mut RBasic,
                                item.value.p as *mut RBasic);
    }
    return self_0;
}
#[inline]
unsafe extern "C" fn value_move(mut s1: *mut mrb_value,
                                mut s2: *const mrb_value, mut n: size_t) {
    if s1 > s2 as *mut mrb_value &&
           s1 < s2.offset(n as isize) as *mut mrb_value {
        s1 = s1.offset(n as isize);
        s2 = s2.offset(n as isize);
        loop  {
            let fresh2 = n;
            n = n.wrapping_sub(1);
            if !(fresh2 > 0i32 as libc::c_ulong) { break ; }
            s1 = s1.offset(-1isize);
            s2 = s2.offset(-1isize);
            *s1 = *s2
        }
    } else if s1 != s2 as *mut mrb_value {
        loop  {
            let fresh3 = n;
            n = n.wrapping_sub(1);
            if !(fresh3 > 0i32 as libc::c_ulong) { break ; }
            let fresh5 = s1;
            s1 = s1.offset(1);
            let fresh4 = s2;
            s2 = s2.offset(1);
            *fresh5 = *fresh4
        }
    };
}
/*
 * Get nth element in the array
 *
 * Equivalent to:
 *
 *     ary[offset]
 *
 * @param ary The target array.
 * @param offset The element position (negative counts from the tail).
 */
#[no_mangle]
pub unsafe extern "C" fn mrb_ary_entry(mut ary: mrb_value,
                                       mut offset: mrb_int) -> mrb_value {
    if offset < 0i32 as libc::c_longlong {
        offset +=
            if 0 !=
                   (*(ary.value.p as *mut RArray)).flags() as libc::c_int &
                       7i32 {
                (((*(ary.value.p as *mut RArray)).flags() as libc::c_int &
                      7i32) - 1i32) as mrb_int
            } else { (*(ary.value.p as *mut RArray)).as_0.heap.len }
    }
    if offset < 0i32 as libc::c_longlong ||
           if 0 !=
                  (*(ary.value.p as *mut RArray)).flags() as libc::c_int &
                      7i32 {
               (((*(ary.value.p as *mut RArray)).flags() as libc::c_int &
                     7i32) - 1i32) as mrb_int
           } else { (*(ary.value.p as *mut RArray)).as_0.heap.len } <= offset
       {
        return mrb_nil_value()
    }
    return *if 0 !=
                   (*(ary.value.p as *mut RArray)).flags() as libc::c_int &
                       7i32 {
                &mut (*(ary.value.p as *mut RArray)).as_0 as *mut unnamed_0 as
                    *mut mrb_value
            } else {
                (*(ary.value.p as *mut RArray)).as_0.heap.ptr
            }.offset(offset as isize);
}
/*
 * Replace subsequence of an array.
 *
 * Equivalent to:
 *
 *      ary.shift
 *
 * @param mrb The mruby state reference.
 * @param self The array from which the value will be shifted.
 * @param head Beginning position of a replacement subsequence.
 * @param len Length of a replacement subsequence.
 * @param rpl The array of replacement elements.
 * @return The receiver array.
 */
#[no_mangle]
pub unsafe extern "C" fn mrb_ary_splice(mut mrb: *mut mrb_state,
                                        mut ary: mrb_value, mut head: mrb_int,
                                        mut len: mrb_int, mut rpl: mrb_value)
 -> mrb_value {
    let mut a: *mut RArray = ary.value.p as *mut RArray;
    let mut alen: mrb_int =
        if 0 != (*a).flags() as libc::c_int & 7i32 {
            (((*a).flags() as libc::c_int & 7i32) - 1i32) as mrb_int
        } else { (*a).as_0.heap.len };
    let mut argv: *const mrb_value = 0 as *const mrb_value;
    let mut argc: mrb_int = 0;
    let mut tail: mrb_int = 0;
    ary_modify(mrb, a);
    if len < 0i32 as libc::c_longlong {
        mrb_raisef(mrb,
                   mrb_exc_get(mrb,
                               b"IndexError\x00" as *const u8 as
                                   *const libc::c_char),
                   b"negative length (%S)\x00" as *const u8 as
                       *const libc::c_char, mrb_fixnum_value(len));
    }
    if head < 0i32 as libc::c_longlong {
        head += alen;
        if head < 0i32 as libc::c_longlong {
            mrb_raise(mrb,
                      mrb_exc_get(mrb,
                                  b"IndexError\x00" as *const u8 as
                                      *const libc::c_char),
                      b"index is out of array\x00" as *const u8 as
                          *const libc::c_char);
        }
    }
    tail = head + len;
    if alen < len || alen < tail { len = alen - head }
    if rpl.tt as libc::c_uint == MRB_TT_ARRAY as libc::c_int as libc::c_uint {
        argc =
            if 0 !=
                   (*(rpl.value.p as *mut RArray)).flags() as libc::c_int &
                       7i32 {
                (((*(rpl.value.p as *mut RArray)).flags() as libc::c_int &
                      7i32) - 1i32) as mrb_int
            } else { (*(rpl.value.p as *mut RArray)).as_0.heap.len };
        argv =
            if 0 !=
                   (*(rpl.value.p as *mut RArray)).flags() as libc::c_int &
                       7i32 {
                &mut (*(rpl.value.p as *mut RArray)).as_0 as *mut unnamed_0 as
                    *mut mrb_value
            } else { (*(rpl.value.p as *mut RArray)).as_0.heap.ptr };
        if argv ==
               (if 0 != (*a).flags() as libc::c_int & 7i32 {
                    &mut (*a).as_0 as *mut unnamed_0 as *mut mrb_value
                } else { (*a).as_0.heap.ptr }) as *const mrb_value {
            let mut r: *mut RArray = 0 as *mut RArray;
            if argc > 32767i32 as libc::c_longlong {
                mrb_raise(mrb,
                          mrb_exc_get(mrb,
                                      b"ArgumentError\x00" as *const u8 as
                                          *const libc::c_char),
                          b"too big recursive splice\x00" as *const u8 as
                              *const libc::c_char);
            }
            r = ary_dup(mrb, a);
            argv =
                if 0 != (*r).flags() as libc::c_int & 7i32 {
                    &mut (*r).as_0 as *mut unnamed_0 as *mut mrb_value
                } else { (*r).as_0.heap.ptr }
        }
    } else { argc = 1i32 as mrb_int; argv = &mut rpl }
    if head >= alen {
        if head >
               (if 18446744073709551615u64.wrapping_div(::std::mem::size_of::<mrb_value>()
                                                            as libc::c_ulong)
                       < (9223372036854775807i64 >> 0i32) as size_t {
                    18446744073709551615u64.wrapping_div(::std::mem::size_of::<mrb_value>()
                                                             as libc::c_ulong)
                        as libc::c_ulonglong
                } else {
                    ((9223372036854775807i64 >> 0i32) -
                         1i32 as libc::c_longlong) as libc::c_ulonglong
                }) as mrb_int - argc {
            mrb_raisef(mrb,
                       mrb_exc_get(mrb,
                                   b"IndexError\x00" as *const u8 as
                                       *const libc::c_char),
                       b"index %S too big\x00" as *const u8 as
                           *const libc::c_char, mrb_fixnum_value(head));
        }
        len = head + argc;
        if len >
               if 0 != (*a).flags() as libc::c_int & 7i32 {
                   (::std::mem::size_of::<*mut libc::c_void>() as
                        libc::c_ulong).wrapping_mul(3i32 as
                                                        libc::c_ulong).wrapping_div(::std::mem::size_of::<mrb_value>()
                                                                                        as
                                                                                        libc::c_ulong)
                       as mrb_int
               } else { (*a).as_0.heap.aux.capa } {
            ary_expand_capa(mrb, a, head + argc);
        }
        ary_fill_with_nil(if 0 != (*a).flags() as libc::c_int & 7i32 {
                              &mut (*a).as_0 as *mut unnamed_0 as
                                  *mut mrb_value
                          } else { (*a).as_0.heap.ptr }.offset(alen as isize),
                          head - alen);
        if argc > 0i32 as libc::c_longlong {
            array_copy(if 0 != (*a).flags() as libc::c_int & 7i32 {
                           &mut (*a).as_0 as *mut unnamed_0 as *mut mrb_value
                       } else { (*a).as_0.heap.ptr }.offset(head as isize),
                       argv, argc);
        }
        if 0 != (*a).flags() as libc::c_int & 7i32 {
            (*a).set_flags(((*a).flags() as libc::c_int & !7i32) as
                               libc::c_uint |
                               (len as
                                    uint32_t).wrapping_add(1i32 as
                                                               libc::c_uint))
        } else { (*a).as_0.heap.len = len }
    } else {
        let mut newlen: mrb_int = 0;
        if alen - len >
               (if 18446744073709551615u64.wrapping_div(::std::mem::size_of::<mrb_value>()
                                                            as libc::c_ulong)
                       < (9223372036854775807i64 >> 0i32) as size_t {
                    18446744073709551615u64.wrapping_div(::std::mem::size_of::<mrb_value>()
                                                             as libc::c_ulong)
                        as libc::c_ulonglong
                } else {
                    ((9223372036854775807i64 >> 0i32) -
                         1i32 as libc::c_longlong) as libc::c_ulonglong
                }) as mrb_int - argc {
            mrb_raisef(mrb,
                       mrb_exc_get(mrb,
                                   b"IndexError\x00" as *const u8 as
                                       *const libc::c_char),
                       b"index %S too big\x00" as *const u8 as
                           *const libc::c_char,
                       mrb_fixnum_value(alen + argc - len));
        }
        newlen = alen + argc - len;
        if newlen >
               if 0 != (*a).flags() as libc::c_int & 7i32 {
                   (::std::mem::size_of::<*mut libc::c_void>() as
                        libc::c_ulong).wrapping_mul(3i32 as
                                                        libc::c_ulong).wrapping_div(::std::mem::size_of::<mrb_value>()
                                                                                        as
                                                                                        libc::c_ulong)
                       as mrb_int
               } else { (*a).as_0.heap.aux.capa } {
            ary_expand_capa(mrb, a, newlen);
        }
        if len != argc {
            let mut ptr: *mut mrb_value =
                if 0 != (*a).flags() as libc::c_int & 7i32 {
                    &mut (*a).as_0 as *mut unnamed_0 as *mut mrb_value
                } else { (*a).as_0.heap.ptr };
            tail = head + len;
            value_move(ptr.offset(head as isize).offset(argc as isize),
                       ptr.offset(tail as isize), (alen - tail) as size_t);
            if 0 != (*a).flags() as libc::c_int & 7i32 {
                (*a).set_flags(((*a).flags() as libc::c_int & !7i32) as
                                   libc::c_uint |
                                   (newlen as
                                        uint32_t).wrapping_add(1i32 as
                                                                   libc::c_uint))
            } else { (*a).as_0.heap.len = newlen }
        }
        if argc > 0i32 as libc::c_longlong {
            value_move(if 0 != (*a).flags() as libc::c_int & 7i32 {
                           &mut (*a).as_0 as *mut unnamed_0 as *mut mrb_value
                       } else { (*a).as_0.heap.ptr }.offset(head as isize),
                       argv, argc as size_t);
        }
    }
    mrb_write_barrier(mrb, a as *mut RBasic);
    return ary;
}
unsafe extern "C" fn ary_dup(mut mrb: *mut mrb_state, mut a: *mut RArray)
 -> *mut RArray {
    return ary_new_from_values(mrb,
                               if 0 != (*a).flags() as libc::c_int & 7i32 {
                                   (((*a).flags() as libc::c_int & 7i32) -
                                        1i32) as mrb_int
                               } else { (*a).as_0.heap.len },
                               if 0 != (*a).flags() as libc::c_int & 7i32 {
                                   &mut (*a).as_0 as *mut unnamed_0 as
                                       *mut mrb_value
                               } else { (*a).as_0.heap.ptr });
}
/*
 * Shifts the first element from the array.
 *
 * Equivalent to:
 *
 *      ary.shift
 *
 * @param mrb The mruby state reference.
 * @param self The array from which the value will be shifted.
 * @return The shifted value.
 */
#[no_mangle]
pub unsafe extern "C" fn mrb_ary_shift(mut mrb: *mut mrb_state,
                                       mut self_0: mrb_value) -> mrb_value {
    let mut a: *mut RArray = self_0.value.p as *mut RArray;
    let mut len: mrb_int =
        if 0 != (*a).flags() as libc::c_int & 7i32 {
            (((*a).flags() as libc::c_int & 7i32) - 1i32) as mrb_int
        } else { (*a).as_0.heap.len };
    let mut val: mrb_value =
        mrb_value{value: unnamed{f: 0.,}, tt: MRB_TT_FALSE,};
    ary_modify_check(mrb, a);
    if len == 0i32 as libc::c_longlong { return mrb_nil_value() }
    if !(0 != (*a).flags() as libc::c_int & 256i32) {
        if len > 10i32 as libc::c_longlong {
            ary_make_shared(mrb, a);
        } else {
            let mut ptr: *mut mrb_value =
                if 0 != (*a).flags() as libc::c_int & 7i32 {
                    &mut (*a).as_0 as *mut unnamed_0 as *mut mrb_value
                } else { (*a).as_0.heap.ptr };
            let mut size: mrb_int = len;
            val = *ptr;
            loop  {
                size -= 1;
                if !(0 != size) { break ; }
                *ptr = *ptr.offset(1isize);
                ptr = ptr.offset(1isize)
            }
            if 0 != (*a).flags() as libc::c_int & 7i32 {
                (*a).set_flags(((*a).flags() as libc::c_int & !7i32) as
                                   libc::c_uint |
                                   ((len - 1i32 as libc::c_longlong) as
                                        uint32_t).wrapping_add(1i32 as
                                                                   libc::c_uint))
            } else { (*a).as_0.heap.len = len - 1i32 as libc::c_longlong }
            return val
        }
    }
    val = *(*a).as_0.heap.ptr.offset(0isize);
    (*a).as_0.heap.ptr = (*a).as_0.heap.ptr.offset(1isize);
    (*a).as_0.heap.len -= 1;
    return val;
}
/*
 * Removes all elements from the array
 *
 * Equivalent to:
 *
 *      ary.clear
 *
 * @param mrb The mruby state reference.
 * @param self The target array.
 * @return self
 */
#[no_mangle]
pub unsafe extern "C" fn mrb_ary_clear(mut mrb: *mut mrb_state,
                                       mut self_0: mrb_value) -> mrb_value {
    let mut a: *mut RArray = self_0.value.p as *mut RArray;
    ary_modify(mrb, a);
    if 0 != (*a).flags() as libc::c_int & 256i32 {
        mrb_ary_decref(mrb, (*a).as_0.heap.aux.shared);
        (*a).set_flags((*a).flags() & !256i32 as uint32_t)
    } else if 0 == (*a).flags() as libc::c_int & 7i32 {
        mrb_free(mrb, (*a).as_0.heap.ptr as *mut libc::c_void);
    }
    (*a).set_flags(((*a).flags() as libc::c_int & !7i32) as libc::c_uint |
                       (0i32 as uint32_t).wrapping_add(1i32 as libc::c_uint));
    return self_0;
}
/*
 * Join the array elements together in a string
 *
 * Equivalent to:
 *
 *      ary.join(sep="")
 *
 * @param mrb The mruby state reference.
 * @param ary The target array
 * @param sep The separater, can be NULL
 */
#[no_mangle]
pub unsafe extern "C" fn mrb_ary_join(mut mrb: *mut mrb_state,
                                      mut ary: mrb_value, mut sep: mrb_value)
 -> mrb_value {
    if !(sep.tt as libc::c_uint == MRB_TT_FALSE as libc::c_int as libc::c_uint
             && 0 == sep.value.i) {
        sep = mrb_obj_as_string(mrb, sep)
    }
    return join_ary(mrb, ary, sep, mrb_ary_new(mrb));
}
unsafe extern "C" fn join_ary(mut mrb: *mut mrb_state, mut ary: mrb_value,
                              mut sep: mrb_value, mut list: mrb_value)
 -> mrb_value {
    let mut current_block: u64;
    let mut i: mrb_int = 0;
    let mut result: mrb_value =
        mrb_value{value: unnamed{f: 0.,}, tt: MRB_TT_FALSE,};
    let mut val: mrb_value =
        mrb_value{value: unnamed{f: 0.,}, tt: MRB_TT_FALSE,};
    let mut tmp: mrb_value =
        mrb_value{value: unnamed{f: 0.,}, tt: MRB_TT_FALSE,};
    i = 0i32 as mrb_int;
    while i <
              if 0 !=
                     (*(list.value.p as *mut RArray)).flags() as libc::c_int &
                         7i32 {
                  (((*(list.value.p as *mut RArray)).flags() as libc::c_int &
                        7i32) - 1i32) as mrb_int
              } else { (*(list.value.p as *mut RArray)).as_0.heap.len } {
        if 0 !=
               mrb_obj_equal(mrb, ary,
                             *if 0 !=
                                     (*(list.value.p as *mut RArray)).flags()
                                         as libc::c_int & 7i32 {
                                  &mut (*(list.value.p as *mut RArray)).as_0
                                      as *mut unnamed_0 as *mut mrb_value
                              } else {
                                  (*(list.value.p as
                                         *mut RArray)).as_0.heap.ptr
                              }.offset(i as isize)) {
            mrb_raise(mrb,
                      mrb_exc_get(mrb,
                                  b"ArgumentError\x00" as *const u8 as
                                      *const libc::c_char),
                      b"recursive array join\x00" as *const u8 as
                          *const libc::c_char);
        }
        i += 1
    }
    mrb_ary_push(mrb, list, ary);
    result = mrb_str_new_capa(mrb, 64i32 as size_t);
    i = 0i32 as mrb_int;
    while i <
              if 0 !=
                     (*(ary.value.p as *mut RArray)).flags() as libc::c_int &
                         7i32 {
                  (((*(ary.value.p as *mut RArray)).flags() as libc::c_int &
                        7i32) - 1i32) as mrb_int
              } else { (*(ary.value.p as *mut RArray)).as_0.heap.len } {
        if i > 0i32 as libc::c_longlong &&
               !(sep.tt as libc::c_uint ==
                     MRB_TT_FALSE as libc::c_int as libc::c_uint &&
                     0 == sep.value.i) {
            mrb_str_cat_str(mrb, result, sep);
        }
        val =
            *if 0 !=
                    (*(ary.value.p as *mut RArray)).flags() as libc::c_int &
                        7i32 {
                 &mut (*(ary.value.p as *mut RArray)).as_0 as *mut unnamed_0
                     as *mut mrb_value
             } else {
                 (*(ary.value.p as *mut RArray)).as_0.heap.ptr
             }.offset(i as isize);
        match val.tt as libc::c_uint {
            14 => { current_block = 2689073758899521587; }
            16 => { current_block = 17965347522327025173; }
            _ => {
                if !((val.tt as libc::c_uint) <
                         MRB_TT_OBJECT as libc::c_int as libc::c_uint) {
                    tmp = mrb_check_string_type(mrb, val);
                    if !(tmp.tt as libc::c_uint ==
                             MRB_TT_FALSE as libc::c_int as libc::c_uint &&
                             0 == tmp.value.i) {
                        val = tmp;
                        current_block = 17965347522327025173;
                    } else {
                        tmp = mrb_check_array_type(mrb, val);
                        if !(tmp.tt as libc::c_uint ==
                                 MRB_TT_FALSE as libc::c_int as libc::c_uint
                                 && 0 == tmp.value.i) {
                            val = tmp;
                            current_block = 2689073758899521587;
                        } else { current_block = 15125582407903384992; }
                    }
                } else { current_block = 15125582407903384992; }
                match current_block {
                    2689073758899521587 => { }
                    17965347522327025173 => { }
                    _ => {
                        val = mrb_obj_as_string(mrb, val);
                        current_block = 17965347522327025173;
                    }
                }
            }
        }
        match current_block {
            2689073758899521587 => { val = join_ary(mrb, val, sep, list) }
            _ => { }
        }
        /* fall through */
        mrb_str_cat_str(mrb, result, val);
        i += 1
    }
    mrb_ary_pop(mrb, list);
    return result;
}
/*
 * Update the capacity of the array
 *
 * @param mrb The mruby state reference.
 * @param ary The target array.
 * @param new_len The new capacity of the array
 */
#[no_mangle]
pub unsafe extern "C" fn mrb_ary_resize(mut mrb: *mut mrb_state,
                                        mut ary: mrb_value,
                                        mut new_len: mrb_int) -> mrb_value {
    let mut old_len: mrb_int = 0;
    let mut a: *mut RArray = ary.value.p as *mut RArray;
    ary_modify(mrb, a);
    old_len =
        if 0 != (*(ary.value.p as *mut RArray)).flags() as libc::c_int & 7i32
           {
            (((*(ary.value.p as *mut RArray)).flags() as libc::c_int & 7i32) -
                 1i32) as mrb_int
        } else { (*(ary.value.p as *mut RArray)).as_0.heap.len };
    if old_len != new_len {
        if new_len < old_len {
            ary_shrink_capa(mrb, a);
        } else {
            ary_expand_capa(mrb, a, new_len);
            ary_fill_with_nil(if 0 != (*a).flags() as libc::c_int & 7i32 {
                                  &mut (*a).as_0 as *mut unnamed_0 as
                                      *mut mrb_value
                              } else {
                                  (*a).as_0.heap.ptr
                              }.offset(old_len as isize), new_len - old_len);
        }
        if 0 != (*a).flags() as libc::c_int & 7i32 {
            (*a).set_flags(((*a).flags() as libc::c_int & !7i32) as
                               libc::c_uint |
                               (new_len as
                                    uint32_t).wrapping_add(1i32 as
                                                               libc::c_uint))
        } else { (*a).as_0.heap.len = new_len }
    }
    return ary;
}
unsafe extern "C" fn ary_shrink_capa(mut mrb: *mut mrb_state,
                                     mut a: *mut RArray) {
    let mut capa: mrb_int = 0;
    if 0 != (*a).flags() as libc::c_int & 7i32 { return }
    capa = (*a).as_0.heap.aux.capa;
    if capa < (4i32 * 2i32) as libc::c_longlong { return }
    if capa <= (*a).as_0.heap.len * 5i32 as libc::c_longlong { return }
    loop  {
        capa /= 2i32 as libc::c_longlong;
        if capa < 4i32 as libc::c_longlong {
            capa = 4i32 as mrb_int;
            break ;
        } else if !(capa > (*a).as_0.heap.len * 5i32 as libc::c_longlong) {
            break ;
        }
    }
    if capa > (*a).as_0.heap.len && capa < (*a).as_0.heap.aux.capa {
        (*a).as_0.heap.aux.capa = capa;
        (*a).as_0.heap.ptr =
            mrb_realloc(mrb, (*a).as_0.heap.ptr as *mut libc::c_void,
                        (::std::mem::size_of::<mrb_value>() as libc::c_ulong
                             as
                             libc::c_ulonglong).wrapping_mul(capa as
                                                                 libc::c_ulonglong)
                            as size_t) as *mut mrb_value
    };
}
unsafe extern "C" fn mrb_ary_s_create(mut mrb: *mut mrb_state,
                                      mut klass: mrb_value) -> mrb_value {
    let mut ary: mrb_value =
        mrb_value{value: unnamed{f: 0.,}, tt: MRB_TT_FALSE,};
    let mut vals: *mut mrb_value = 0 as *mut mrb_value;
    let mut len: mrb_int = 0;
    let mut a: *mut RArray = 0 as *mut RArray;
    mrb_get_args(mrb, b"*!\x00" as *const u8 as *const libc::c_char,
                 &mut vals as *mut *mut mrb_value, &mut len as *mut mrb_int);
    ary = mrb_ary_new_from_values(mrb, len, vals);
    a = ary.value.p as *mut RArray;
    (*a).c = klass.value.p as *mut RClass;
    return ary;
}
unsafe extern "C" fn mrb_ary_concat_m(mut mrb: *mut mrb_state,
                                      mut self_0: mrb_value) -> mrb_value {
    let mut ary: mrb_value =
        mrb_value{value: unnamed{f: 0.,}, tt: MRB_TT_FALSE,};
    mrb_get_args(mrb, b"A\x00" as *const u8 as *const libc::c_char,
                 &mut ary as *mut mrb_value);
    mrb_ary_concat(mrb, self_0, ary);
    return self_0;
}
unsafe extern "C" fn mrb_ary_plus(mut mrb: *mut mrb_state,
                                  mut self_0: mrb_value) -> mrb_value {
    let mut a1: *mut RArray = self_0.value.p as *mut RArray;
    let mut a2: *mut RArray = 0 as *mut RArray;
    let mut ptr: *mut mrb_value = 0 as *mut mrb_value;
    let mut blen: mrb_int = 0;
    let mut len1: mrb_int = 0;
    mrb_get_args(mrb, b"a\x00" as *const u8 as *const libc::c_char,
                 &mut ptr as *mut *mut mrb_value, &mut blen as *mut mrb_int);
    if (if 18446744073709551615u64.wrapping_div(::std::mem::size_of::<mrb_value>()
                                                    as libc::c_ulong) <
               (9223372036854775807i64 >> 0i32) as size_t {
            18446744073709551615u64.wrapping_div(::std::mem::size_of::<mrb_value>()
                                                     as libc::c_ulong) as
                libc::c_ulonglong
        } else {
            ((9223372036854775807i64 >> 0i32) - 1i32 as libc::c_longlong) as
                libc::c_ulonglong
        }) as mrb_int - blen <
           if 0 != (*a1).flags() as libc::c_int & 7i32 {
               (((*a1).flags() as libc::c_int & 7i32) - 1i32) as mrb_int
           } else { (*a1).as_0.heap.len } {
        mrb_raise(mrb,
                  mrb_exc_get(mrb,
                              b"ArgumentError\x00" as *const u8 as
                                  *const libc::c_char),
                  b"array size too big\x00" as *const u8 as
                      *const libc::c_char);
    }
    len1 =
        if 0 != (*a1).flags() as libc::c_int & 7i32 {
            (((*a1).flags() as libc::c_int & 7i32) - 1i32) as mrb_int
        } else { (*a1).as_0.heap.len };
    a2 = ary_new_capa(mrb, len1 + blen);
    array_copy(if 0 != (*a2).flags() as libc::c_int & 7i32 {
                   &mut (*a2).as_0 as *mut unnamed_0 as *mut mrb_value
               } else { (*a2).as_0.heap.ptr },
               if 0 != (*a1).flags() as libc::c_int & 7i32 {
                   &mut (*a1).as_0 as *mut unnamed_0 as *mut mrb_value
               } else { (*a1).as_0.heap.ptr }, len1);
    array_copy(if 0 != (*a2).flags() as libc::c_int & 7i32 {
                   &mut (*a2).as_0 as *mut unnamed_0 as *mut mrb_value
               } else { (*a2).as_0.heap.ptr }.offset(len1 as isize), ptr,
               blen);
    if 0 != (*a2).flags() as libc::c_int & 7i32 {
        (*a2).set_flags(((*a2).flags() as libc::c_int & !7i32) as libc::c_uint
                            |
                            ((len1 + blen) as
                                 uint32_t).wrapping_add(1i32 as libc::c_uint))
    } else { (*a2).as_0.heap.len = len1 + blen }
    return mrb_obj_value(a2 as *mut libc::c_void);
}
unsafe extern "C" fn mrb_ary_replace_m(mut mrb: *mut mrb_state,
                                       mut self_0: mrb_value) -> mrb_value {
    let mut other: mrb_value =
        mrb_value{value: unnamed{f: 0.,}, tt: MRB_TT_FALSE,};
    mrb_get_args(mrb, b"A\x00" as *const u8 as *const libc::c_char,
                 &mut other as *mut mrb_value);
    mrb_ary_replace(mrb, self_0, other);
    return self_0;
}
unsafe extern "C" fn mrb_ary_times(mut mrb: *mut mrb_state,
                                   mut self_0: mrb_value) -> mrb_value {
    let mut a1: *mut RArray = self_0.value.p as *mut RArray;
    let mut a2: *mut RArray = 0 as *mut RArray;
    let mut ptr: *mut mrb_value = 0 as *mut mrb_value;
    let mut times: mrb_int = 0;
    let mut len1: mrb_int = 0;
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
    if times == 0i32 as libc::c_longlong { return mrb_ary_new(mrb) }
    if (if 18446744073709551615u64.wrapping_div(::std::mem::size_of::<mrb_value>()
                                                    as libc::c_ulong) <
               (9223372036854775807i64 >> 0i32) as size_t {
            18446744073709551615u64.wrapping_div(::std::mem::size_of::<mrb_value>()
                                                     as libc::c_ulong) as
                libc::c_ulonglong
        } else {
            ((9223372036854775807i64 >> 0i32) - 1i32 as libc::c_longlong) as
                libc::c_ulonglong
        }) as mrb_int / times <
           if 0 != (*a1).flags() as libc::c_int & 7i32 {
               (((*a1).flags() as libc::c_int & 7i32) - 1i32) as mrb_int
           } else { (*a1).as_0.heap.len } {
        mrb_raise(mrb,
                  mrb_exc_get(mrb,
                              b"ArgumentError\x00" as *const u8 as
                                  *const libc::c_char),
                  b"array size too big\x00" as *const u8 as
                      *const libc::c_char);
    }
    len1 =
        if 0 != (*a1).flags() as libc::c_int & 7i32 {
            (((*a1).flags() as libc::c_int & 7i32) - 1i32) as mrb_int
        } else { (*a1).as_0.heap.len };
    a2 = ary_new_capa(mrb, len1 * times);
    if 0 != (*a2).flags() as libc::c_int & 7i32 {
        (*a2).set_flags(((*a2).flags() as libc::c_int & !7i32) as libc::c_uint
                            |
                            ((len1 * times) as
                                 uint32_t).wrapping_add(1i32 as libc::c_uint))
    } else { (*a2).as_0.heap.len = len1 * times }
    ptr =
        if 0 != (*a2).flags() as libc::c_int & 7i32 {
            &mut (*a2).as_0 as *mut unnamed_0 as *mut mrb_value
        } else { (*a2).as_0.heap.ptr };
    loop  {
        let fresh6 = times;
        times = times - 1;
        if !(0 != fresh6) { break ; }
        array_copy(ptr,
                   if 0 != (*a1).flags() as libc::c_int & 7i32 {
                       &mut (*a1).as_0 as *mut unnamed_0 as *mut mrb_value
                   } else { (*a1).as_0.heap.ptr }, len1);
        ptr = ptr.offset(len1 as isize)
    }
    return mrb_obj_value(a2 as *mut libc::c_void);
}
unsafe extern "C" fn mrb_ary_reverse_bang(mut mrb: *mut mrb_state,
                                          mut self_0: mrb_value)
 -> mrb_value {
    let mut a: *mut RArray = self_0.value.p as *mut RArray;
    let mut len: mrb_int =
        if 0 != (*a).flags() as libc::c_int & 7i32 {
            (((*a).flags() as libc::c_int & 7i32) - 1i32) as mrb_int
        } else { (*a).as_0.heap.len };
    if len > 1i32 as libc::c_longlong {
        let mut p1: *mut mrb_value = 0 as *mut mrb_value;
        let mut p2: *mut mrb_value = 0 as *mut mrb_value;
        ary_modify(mrb, a);
        p1 =
            if 0 != (*a).flags() as libc::c_int & 7i32 {
                &mut (*a).as_0 as *mut unnamed_0 as *mut mrb_value
            } else { (*a).as_0.heap.ptr };
        p2 = p1.offset(len as isize).offset(-1isize);
        while p1 < p2 {
            let mut tmp: mrb_value = *p1;
            let fresh7 = p1;
            p1 = p1.offset(1);
            *fresh7 = *p2;
            let fresh8 = p2;
            p2 = p2.offset(-1);
            *fresh8 = tmp
        }
    }
    return self_0;
}
unsafe extern "C" fn mrb_ary_reverse(mut mrb: *mut mrb_state,
                                     mut self_0: mrb_value) -> mrb_value {
    let mut a: *mut RArray = self_0.value.p as *mut RArray;
    let mut b: *mut RArray =
        ary_new_capa(mrb,
                     if 0 != (*a).flags() as libc::c_int & 7i32 {
                         (((*a).flags() as libc::c_int & 7i32) - 1i32) as
                             mrb_int
                     } else { (*a).as_0.heap.len });
    let mut len: mrb_int =
        if 0 != (*a).flags() as libc::c_int & 7i32 {
            (((*a).flags() as libc::c_int & 7i32) - 1i32) as mrb_int
        } else { (*a).as_0.heap.len };
    if len > 0i32 as libc::c_longlong {
        let mut p1: *mut mrb_value = 0 as *mut mrb_value;
        let mut p2: *mut mrb_value = 0 as *mut mrb_value;
        let mut e: *mut mrb_value = 0 as *mut mrb_value;
        p1 =
            if 0 != (*a).flags() as libc::c_int & 7i32 {
                &mut (*a).as_0 as *mut unnamed_0 as *mut mrb_value
            } else { (*a).as_0.heap.ptr };
        e = p1.offset(len as isize);
        p2 =
            if 0 != (*b).flags() as libc::c_int & 7i32 {
                &mut (*b).as_0 as *mut unnamed_0 as *mut mrb_value
            } else {
                (*b).as_0.heap.ptr
            }.offset(len as isize).offset(-1isize);
        while p1 < e {
            let fresh10 = p2;
            p2 = p2.offset(-1);
            let fresh9 = p1;
            p1 = p1.offset(1);
            *fresh10 = *fresh9
        }
        if 0 != (*b).flags() as libc::c_int & 7i32 {
            (*b).set_flags(((*b).flags() as libc::c_int & !7i32) as
                               libc::c_uint |
                               (len as
                                    uint32_t).wrapping_add(1i32 as
                                                               libc::c_uint))
        } else { (*b).as_0.heap.len = len }
    }
    return mrb_obj_value(b as *mut libc::c_void);
}
unsafe extern "C" fn mrb_ary_push_m(mut mrb: *mut mrb_state,
                                    mut self_0: mrb_value) -> mrb_value {
    let mut argv: *mut mrb_value = 0 as *mut mrb_value;
    let mut len: mrb_int = 0;
    let mut len2: mrb_int = 0;
    let mut alen: mrb_int = 0;
    let mut a: *mut RArray = 0 as *mut RArray;
    mrb_get_args(mrb, b"*!\x00" as *const u8 as *const libc::c_char,
                 &mut argv as *mut *mut mrb_value, &mut alen as *mut mrb_int);
    a = self_0.value.p as *mut RArray;
    ary_modify(mrb, a);
    len =
        if 0 != (*a).flags() as libc::c_int & 7i32 {
            (((*a).flags() as libc::c_int & 7i32) - 1i32) as mrb_int
        } else { (*a).as_0.heap.len };
    len2 = len + alen;
    if if 0 != (*a).flags() as libc::c_int & 7i32 {
           (::std::mem::size_of::<*mut libc::c_void>() as
                libc::c_ulong).wrapping_mul(3i32 as
                                                libc::c_ulong).wrapping_div(::std::mem::size_of::<mrb_value>()
                                                                                as
                                                                                libc::c_ulong)
               as mrb_int
       } else { (*a).as_0.heap.aux.capa } < len2 {
        ary_expand_capa(mrb, a, len2);
    }
    array_copy(if 0 != (*a).flags() as libc::c_int & 7i32 {
                   &mut (*a).as_0 as *mut unnamed_0 as *mut mrb_value
               } else { (*a).as_0.heap.ptr }.offset(len as isize), argv,
               alen);
    if 0 != (*a).flags() as libc::c_int & 7i32 {
        (*a).set_flags(((*a).flags() as libc::c_int & !7i32) as libc::c_uint |
                           (len2 as
                                uint32_t).wrapping_add(1i32 as libc::c_uint))
    } else { (*a).as_0.heap.len = len2 }
    mrb_write_barrier(mrb, a as *mut RBasic);
    return self_0;
}
unsafe extern "C" fn mrb_ary_unshift_m(mut mrb: *mut mrb_state,
                                       mut self_0: mrb_value) -> mrb_value {
    let mut a: *mut RArray = self_0.value.p as *mut RArray;
    let mut vals: *mut mrb_value = 0 as *mut mrb_value;
    let mut ptr: *mut mrb_value = 0 as *mut mrb_value;
    let mut alen: mrb_int = 0;
    let mut len: mrb_int = 0;
    mrb_get_args(mrb, b"*!\x00" as *const u8 as *const libc::c_char,
                 &mut vals as *mut *mut mrb_value, &mut alen as *mut mrb_int);
    if alen == 0i32 as libc::c_longlong {
        ary_modify_check(mrb, a);
        return self_0
    }
    len =
        if 0 != (*a).flags() as libc::c_int & 7i32 {
            (((*a).flags() as libc::c_int & 7i32) - 1i32) as mrb_int
        } else { (*a).as_0.heap.len };
    if alen >
           (if 18446744073709551615u64.wrapping_div(::std::mem::size_of::<mrb_value>()
                                                        as libc::c_ulong) <
                   (9223372036854775807i64 >> 0i32) as size_t {
                18446744073709551615u64.wrapping_div(::std::mem::size_of::<mrb_value>()
                                                         as libc::c_ulong) as
                    libc::c_ulonglong
            } else {
                ((9223372036854775807i64 >> 0i32) - 1i32 as libc::c_longlong)
                    as libc::c_ulonglong
            }) as mrb_int - len {
        mrb_raise(mrb,
                  mrb_exc_get(mrb,
                              b"ArgumentError\x00" as *const u8 as
                                  *const libc::c_char),
                  b"array size too big\x00" as *const u8 as
                      *const libc::c_char);
    }
    if 0 != (*a).flags() as libc::c_int & 256i32 &&
           (*(*a).as_0.heap.aux.shared).refcnt == 1i32 &&
           (*a).as_0.heap.ptr.wrapping_offset_from((*(*a).as_0.heap.aux.shared).ptr)
               as libc::c_long as libc::c_longlong >= alen {
        ary_modify_check(mrb, a);
        (*a).as_0.heap.ptr = (*a).as_0.heap.ptr.offset(-(alen as isize));
        ptr = (*a).as_0.heap.ptr
    } else {
        ary_modify(mrb, a);
        if if 0 != (*a).flags() as libc::c_int & 7i32 {
               (::std::mem::size_of::<*mut libc::c_void>() as
                    libc::c_ulong).wrapping_mul(3i32 as
                                                    libc::c_ulong).wrapping_div(::std::mem::size_of::<mrb_value>()
                                                                                    as
                                                                                    libc::c_ulong)
                   as mrb_int
           } else { (*a).as_0.heap.aux.capa } < len + alen {
            ary_expand_capa(mrb, a, len + alen);
        }
        ptr =
            if 0 != (*a).flags() as libc::c_int & 7i32 {
                &mut (*a).as_0 as *mut unnamed_0 as *mut mrb_value
            } else { (*a).as_0.heap.ptr };
        value_move(ptr.offset(alen as isize), ptr, len as size_t);
    }
    array_copy(ptr, vals, alen);
    if 0 != (*a).flags() as libc::c_int & 7i32 {
        (*a).set_flags(((*a).flags() as libc::c_int & !7i32) as libc::c_uint |
                           ((len + alen) as
                                uint32_t).wrapping_add(1i32 as libc::c_uint))
    } else { (*a).as_0.heap.len = len + alen }
    loop  {
        let fresh11 = alen;
        alen = alen - 1;
        if !(0 != fresh11) { break ; }
        if !(((*vals.offset(alen as isize)).tt as libc::c_uint) <
                 MRB_TT_OBJECT as libc::c_int as libc::c_uint) {
            mrb_field_write_barrier(mrb, a as *mut RBasic,
                                    (*vals.offset(alen as isize)).value.p as
                                        *mut RBasic);
        }
    }
    return self_0;
}
unsafe extern "C" fn ary_subseq(mut mrb: *mut mrb_state, mut a: *mut RArray,
                                mut beg: mrb_int, mut len: mrb_int)
 -> mrb_value {
    let mut b: *mut RArray = 0 as *mut RArray;
    if 0 == (*a).flags() as libc::c_int & 256i32 &&
           len <= 10i32 as libc::c_longlong {
        return mrb_ary_new_from_values(mrb, len,
                                       if 0 !=
                                              (*a).flags() as libc::c_int &
                                                  7i32 {
                                           &mut (*a).as_0 as *mut unnamed_0 as
                                               *mut mrb_value
                                       } else {
                                           (*a).as_0.heap.ptr
                                       }.offset(beg as isize))
    }
    ary_make_shared(mrb, a);
    b = mrb_obj_alloc(mrb, MRB_TT_ARRAY, (*mrb).array_class) as *mut RArray;
    (*b).as_0.heap.ptr = (*a).as_0.heap.ptr.offset(beg as isize);
    (*b).as_0.heap.len = len;
    (*b).as_0.heap.aux.shared = (*a).as_0.heap.aux.shared;
    (*(*b).as_0.heap.aux.shared).refcnt += 1;
    (*b).set_flags((*b).flags() | 256i32 as uint32_t);
    return mrb_obj_value(b as *mut libc::c_void);
}
unsafe extern "C" fn aget_index(mut mrb: *mut mrb_state, mut index: mrb_value)
 -> mrb_int {
    if index.tt as libc::c_uint ==
           MRB_TT_FIXNUM as libc::c_int as libc::c_uint {
        return index.value.i
    } else if index.tt as libc::c_uint ==
                  MRB_TT_FLOAT as libc::c_int as libc::c_uint {
        return index.value.f as mrb_int
    } else {
        let mut i: mrb_int = 0;
        let mut argc: mrb_int = 0;
        let mut argv: *mut mrb_value = 0 as *mut mrb_value;
        mrb_get_args(mrb, b"i*!\x00" as *const u8 as *const libc::c_char,
                     &mut i as *mut mrb_int, &mut argv as *mut *mut mrb_value,
                     &mut argc as *mut mrb_int);
        return i
    };
}
/*
 *  call-seq:
 *     ary[index]                -> obj     or nil
 *     ary[start, length]        -> new_ary or nil
 *     ary[range]                -> new_ary or nil
 *     ary.slice(index)          -> obj     or nil
 *     ary.slice(start, length)  -> new_ary or nil
 *     ary.slice(range)          -> new_ary or nil
 *
 *  Element Reference --- Returns the element at +index+, or returns a
 *  subarray starting at the +start+ index and continuing for +length+
 *  elements, or returns a subarray specified by +range+ of indices.
 *
 *  Negative indices count backward from the end of the array (-1 is the last
 *  element).  For +start+ and +range+ cases the starting index is just before
 *  an element.  Additionally, an empty array is returned when the starting
 *  index for an element range is at the end of the array.
 *
 *  Returns +nil+ if the index (or starting index) are out of range.
 *
 *  a = [ "a", "b", "c", "d", "e" ]
 *  a[1]     => "b"
 *  a[1,2]   => ["b", "c"]
 *  a[1..-2] => ["b", "c", "d"]
 *
 */
unsafe extern "C" fn mrb_ary_aget(mut mrb: *mut mrb_state,
                                  mut self_0: mrb_value) -> mrb_value {
    let mut a: *mut RArray = self_0.value.p as *mut RArray;
    let mut i: mrb_int = 0;
    let mut len: mrb_int = 0;
    let mut alen: mrb_int = 0;
    let mut index: mrb_value =
        mrb_value{value: unnamed{f: 0.,}, tt: MRB_TT_FALSE,};
    if mrb_get_args(mrb, b"o|i\x00" as *const u8 as *const libc::c_char,
                    &mut index as *mut mrb_value, &mut len as *mut mrb_int) ==
           1i32 as libc::c_longlong {
        match index.tt as libc::c_uint {
            17 => {
                if mrb_range_beg_len(mrb, index, &mut i, &mut len,
                                     if 0 !=
                                            (*a).flags() as libc::c_int & 7i32
                                        {
                                         (((*a).flags() as libc::c_int & 7i32)
                                              - 1i32) as mrb_int
                                     } else { (*a).as_0.heap.len },
                                     1i32 as mrb_bool) as libc::c_uint ==
                       MRB_RANGE_OK as libc::c_int as libc::c_uint {
                    return ary_subseq(mrb, a, i, len)
                } else { return mrb_nil_value() }
            }
            3 => { return mrb_ary_ref(mrb, self_0, index.value.i) }
            _ => { return mrb_ary_ref(mrb, self_0, aget_index(mrb, index)) }
        }
    }
    i = aget_index(mrb, index);
    alen =
        if 0 != (*a).flags() as libc::c_int & 7i32 {
            (((*a).flags() as libc::c_int & 7i32) - 1i32) as mrb_int
        } else { (*a).as_0.heap.len };
    if i < 0i32 as libc::c_longlong { i += alen }
    if i < 0i32 as libc::c_longlong || alen < i { return mrb_nil_value() }
    if len < 0i32 as libc::c_longlong { return mrb_nil_value() }
    if alen == i { return mrb_ary_new(mrb) }
    if len > alen - i { len = alen - i }
    return ary_subseq(mrb, a, i, len);
}
/*
 *  call-seq:
 *     ary[index]         = obj                      ->  obj
 *     ary[start, length] = obj or other_ary or nil  ->  obj or other_ary or nil
 *     ary[range]         = obj or other_ary or nil  ->  obj or other_ary or nil
 *
 *  Element Assignment --- Sets the element at +index+, or replaces a subarray
 *  from the +start+ index for +length+ elements, or replaces a subarray
 *  specified by the +range+ of indices.
 *
 *  If indices are greater than the current capacity of the array, the array
 *  grows automatically.  Elements are inserted into the array at +start+ if
 *  +length+ is zero.
 *
 *  Negative indices will count backward from the end of the array.  For
 *  +start+ and +range+ cases the starting index is just before an element.
 *
 *  An IndexError is raised if a negative index points past the beginning of
 *  the array.
 *
 *  See also Array#push, and Array#unshift.
 *
 *     a = Array.new
 *     a[4] = "4";                 #=> [nil, nil, nil, nil, "4"]
 *     a[0, 3] = [ 'a', 'b', 'c' ] #=> ["a", "b", "c", nil, "4"]
 *     a[1..2] = [ 1, 2 ]          #=> ["a", 1, 2, nil, "4"]
 *     a[0, 2] = "?"               #=> ["?", 2, nil, "4"]
 *     a[0..2] = "A"               #=> ["A", "4"]
 *     a[-1]   = "Z"               #=> ["A", "Z"]
 *     a[1..-1] = nil              #=> ["A", nil]
 *     a[1..-1] = []               #=> ["A"]
 *     a[0, 0] = [ 1, 2 ]          #=> [1, 2, "A"]
 *     a[3, 0] = "B"               #=> [1, 2, "A", "B"]
 */
unsafe extern "C" fn mrb_ary_aset(mut mrb: *mut mrb_state,
                                  mut self_0: mrb_value) -> mrb_value {
    let mut v1: mrb_value =
        mrb_value{value: unnamed{f: 0.,}, tt: MRB_TT_FALSE,};
    let mut v2: mrb_value =
        mrb_value{value: unnamed{f: 0.,}, tt: MRB_TT_FALSE,};
    let mut v3: mrb_value =
        mrb_value{value: unnamed{f: 0.,}, tt: MRB_TT_FALSE,};
    let mut i: mrb_int = 0;
    let mut len: mrb_int = 0;
    mrb_ary_modify(mrb, self_0.value.p as *mut RArray);
    if mrb_get_args(mrb, b"oo|o\x00" as *const u8 as *const libc::c_char,
                    &mut v1 as *mut mrb_value, &mut v2 as *mut mrb_value,
                    &mut v3 as *mut mrb_value) == 2i32 as libc::c_longlong {
        match mrb_range_beg_len(mrb, v1, &mut i, &mut len,
                                if 0 !=
                                       (*(self_0.value.p as
                                              *mut RArray)).flags() as
                                           libc::c_int & 7i32 {
                                    (((*(self_0.value.p as
                                             *mut RArray)).flags() as
                                          libc::c_int & 7i32) - 1i32) as
                                        mrb_int
                                } else {
                                    (*(self_0.value.p as
                                           *mut RArray)).as_0.heap.len
                                }, 0i32 as mrb_bool) as libc::c_uint {
            0 => { mrb_ary_set(mrb, self_0, aget_index(mrb, v1), v2); }
            1 => { mrb_ary_splice(mrb, self_0, i, len, v2); }
            2 => {
                mrb_raisef(mrb,
                           mrb_exc_get(mrb,
                                       b"RangeError\x00" as *const u8 as
                                           *const libc::c_char),
                           b"%S out of range\x00" as *const u8 as
                               *const libc::c_char, v1);
            }
            _ => { }
        }
        return v2
    }
    mrb_ary_splice(mrb, self_0, aget_index(mrb, v1), aget_index(mrb, v2), v3);
    return v3;
}
unsafe extern "C" fn mrb_ary_delete_at(mut mrb: *mut mrb_state,
                                       mut self_0: mrb_value) -> mrb_value {
    let mut a: *mut RArray = self_0.value.p as *mut RArray;
    let mut index: mrb_int = 0;
    let mut val: mrb_value =
        mrb_value{value: unnamed{f: 0.,}, tt: MRB_TT_FALSE,};
    let mut ptr: *mut mrb_value = 0 as *mut mrb_value;
    let mut len: mrb_int = 0;
    let mut alen: mrb_int = 0;
    mrb_get_args(mrb, b"i\x00" as *const u8 as *const libc::c_char,
                 &mut index as *mut mrb_int);
    alen =
        if 0 != (*a).flags() as libc::c_int & 7i32 {
            (((*a).flags() as libc::c_int & 7i32) - 1i32) as mrb_int
        } else { (*a).as_0.heap.len };
    if index < 0i32 as libc::c_longlong { index += alen }
    if index < 0i32 as libc::c_longlong || alen <= index {
        return mrb_nil_value()
    }
    ary_modify(mrb, a);
    ptr =
        if 0 != (*a).flags() as libc::c_int & 7i32 {
            &mut (*a).as_0 as *mut unnamed_0 as *mut mrb_value
        } else { (*a).as_0.heap.ptr };
    val = *ptr.offset(index as isize);
    ptr = ptr.offset(index as isize);
    len = alen - index;
    loop  {
        len -= 1;
        if !(0 != len) { break ; }
        *ptr = *ptr.offset(1isize);
        ptr = ptr.offset(1isize)
    }
    if 0 != (*a).flags() as libc::c_int & 7i32 {
        (*a).set_flags(((*a).flags() as libc::c_int & !7i32) as libc::c_uint |
                           ((alen - 1i32 as libc::c_longlong) as
                                uint32_t).wrapping_add(1i32 as libc::c_uint))
    } else { (*a).as_0.heap.len = alen - 1i32 as libc::c_longlong }
    ary_shrink_capa(mrb, a);
    return val;
}
unsafe extern "C" fn mrb_ary_first(mut mrb: *mut mrb_state,
                                   mut self_0: mrb_value) -> mrb_value {
    let mut a: *mut RArray = self_0.value.p as *mut RArray;
    let mut size: mrb_int = 0;
    let mut alen: mrb_int = 0;
    if mrb_get_argc(mrb) == 0i32 as libc::c_longlong {
        return if if 0 != (*a).flags() as libc::c_int & 7i32 {
                      (((*a).flags() as libc::c_int & 7i32) - 1i32) as mrb_int
                  } else { (*a).as_0.heap.len } > 0i32 as libc::c_longlong {
                   *if 0 != (*a).flags() as libc::c_int & 7i32 {
                        &mut (*a).as_0 as *mut unnamed_0 as *mut mrb_value
                    } else { (*a).as_0.heap.ptr }.offset(0isize)
               } else { mrb_nil_value() }
    }
    mrb_get_args(mrb, b"|i\x00" as *const u8 as *const libc::c_char,
                 &mut size as *mut mrb_int);
    if size < 0i32 as libc::c_longlong {
        mrb_raise(mrb,
                  mrb_exc_get(mrb,
                              b"ArgumentError\x00" as *const u8 as
                                  *const libc::c_char),
                  b"negative array size\x00" as *const u8 as
                      *const libc::c_char);
    }
    alen =
        if 0 != (*a).flags() as libc::c_int & 7i32 {
            (((*a).flags() as libc::c_int & 7i32) - 1i32) as mrb_int
        } else { (*a).as_0.heap.len };
    if size > alen { size = alen }
    if 0 != (*a).flags() as libc::c_int & 256i32 {
        return ary_subseq(mrb, a, 0i32 as mrb_int, size)
    }
    return mrb_ary_new_from_values(mrb, size,
                                   if 0 != (*a).flags() as libc::c_int & 7i32
                                      {
                                       &mut (*a).as_0 as *mut unnamed_0 as
                                           *mut mrb_value
                                   } else { (*a).as_0.heap.ptr });
}
unsafe extern "C" fn mrb_ary_last(mut mrb: *mut mrb_state,
                                  mut self_0: mrb_value) -> mrb_value {
    let mut a: *mut RArray = self_0.value.p as *mut RArray;
    let mut n: mrb_int = 0;
    let mut size: mrb_int = 0;
    let mut alen: mrb_int = 0;
    n =
        mrb_get_args(mrb, b"|i\x00" as *const u8 as *const libc::c_char,
                     &mut size as *mut mrb_int);
    alen =
        if 0 != (*a).flags() as libc::c_int & 7i32 {
            (((*a).flags() as libc::c_int & 7i32) - 1i32) as mrb_int
        } else { (*a).as_0.heap.len };
    if n == 0i32 as libc::c_longlong {
        return if alen > 0i32 as libc::c_longlong {
                   *if 0 != (*a).flags() as libc::c_int & 7i32 {
                        &mut (*a).as_0 as *mut unnamed_0 as *mut mrb_value
                    } else {
                        (*a).as_0.heap.ptr
                    }.offset((alen - 1i32 as libc::c_longlong) as isize)
               } else { mrb_nil_value() }
    }
    if size < 0i32 as libc::c_longlong {
        mrb_raise(mrb,
                  mrb_exc_get(mrb,
                              b"ArgumentError\x00" as *const u8 as
                                  *const libc::c_char),
                  b"negative array size\x00" as *const u8 as
                      *const libc::c_char);
    }
    if size > alen { size = alen }
    if 0 != (*a).flags() as libc::c_int & 256i32 ||
           size > 4i32 as libc::c_longlong {
        return ary_subseq(mrb, a, alen - size, size)
    }
    return mrb_ary_new_from_values(mrb, size,
                                   if 0 != (*a).flags() as libc::c_int & 7i32
                                      {
                                       &mut (*a).as_0 as *mut unnamed_0 as
                                           *mut mrb_value
                                   } else {
                                       (*a).as_0.heap.ptr
                                   }.offset(alen as
                                                isize).offset(-(size as
                                                                    isize)));
}
unsafe extern "C" fn mrb_ary_index_m(mut mrb: *mut mrb_state,
                                     mut self_0: mrb_value) -> mrb_value {
    let mut obj: mrb_value =
        mrb_value{value: unnamed{f: 0.,}, tt: MRB_TT_FALSE,};
    let mut i: mrb_int = 0;
    mrb_get_args(mrb, b"o\x00" as *const u8 as *const libc::c_char,
                 &mut obj as *mut mrb_value);
    i = 0i32 as mrb_int;
    while i <
              if 0 !=
                     (*(self_0.value.p as *mut RArray)).flags() as libc::c_int
                         & 7i32 {
                  (((*(self_0.value.p as *mut RArray)).flags() as libc::c_int
                        & 7i32) - 1i32) as mrb_int
              } else { (*(self_0.value.p as *mut RArray)).as_0.heap.len } {
        if 0 !=
               mrb_equal(mrb,
                         *if 0 !=
                                 (*(self_0.value.p as *mut RArray)).flags() as
                                     libc::c_int & 7i32 {
                              &mut (*(self_0.value.p as *mut RArray)).as_0 as
                                  *mut unnamed_0 as *mut mrb_value
                          } else {
                              (*(self_0.value.p as *mut RArray)).as_0.heap.ptr
                          }.offset(i as isize), obj) {
            return mrb_fixnum_value(i)
        }
        i += 1
    }
    return mrb_nil_value();
}
unsafe extern "C" fn mrb_ary_rindex_m(mut mrb: *mut mrb_state,
                                      mut self_0: mrb_value) -> mrb_value {
    let mut obj: mrb_value =
        mrb_value{value: unnamed{f: 0.,}, tt: MRB_TT_FALSE,};
    let mut i: mrb_int = 0;
    let mut len: mrb_int = 0;
    mrb_get_args(mrb, b"o\x00" as *const u8 as *const libc::c_char,
                 &mut obj as *mut mrb_value);
    i =
        if 0 !=
               (*(self_0.value.p as *mut RArray)).flags() as libc::c_int &
                   7i32 {
            (((*(self_0.value.p as *mut RArray)).flags() as libc::c_int &
                  7i32) - 1i32) as mrb_int
        } else { (*(self_0.value.p as *mut RArray)).as_0.heap.len } -
            1i32 as libc::c_longlong;
    while i >= 0i32 as libc::c_longlong {
        if 0 !=
               mrb_equal(mrb,
                         *if 0 !=
                                 (*(self_0.value.p as *mut RArray)).flags() as
                                     libc::c_int & 7i32 {
                              &mut (*(self_0.value.p as *mut RArray)).as_0 as
                                  *mut unnamed_0 as *mut mrb_value
                          } else {
                              (*(self_0.value.p as *mut RArray)).as_0.heap.ptr
                          }.offset(i as isize), obj) {
            return mrb_fixnum_value(i)
        }
        len =
            if 0 !=
                   (*(self_0.value.p as *mut RArray)).flags() as libc::c_int &
                       7i32 {
                (((*(self_0.value.p as *mut RArray)).flags() as libc::c_int &
                      7i32) - 1i32) as mrb_int
            } else { (*(self_0.value.p as *mut RArray)).as_0.heap.len };
        if i > len { i = len }
        i -= 1
    }
    return mrb_nil_value();
}
unsafe extern "C" fn mrb_ary_size(mut mrb: *mut mrb_state,
                                  mut self_0: mrb_value) -> mrb_value {
    let mut a: *mut RArray = self_0.value.p as *mut RArray;
    return mrb_fixnum_value(if 0 != (*a).flags() as libc::c_int & 7i32 {
                                (((*a).flags() as libc::c_int & 7i32) - 1i32)
                                    as mrb_int
                            } else { (*a).as_0.heap.len });
}
unsafe extern "C" fn mrb_ary_clear_m(mut mrb: *mut mrb_state,
                                     mut self_0: mrb_value) -> mrb_value {
    mrb_get_args(mrb, b"\x00" as *const u8 as *const libc::c_char);
    return mrb_ary_clear(mrb, self_0);
}
unsafe extern "C" fn mrb_ary_empty_p(mut mrb: *mut mrb_state,
                                     mut self_0: mrb_value) -> mrb_value {
    let mut a: *mut RArray = self_0.value.p as *mut RArray;
    return mrb_bool_value((if 0 != (*a).flags() as libc::c_int & 7i32 {
                               (((*a).flags() as libc::c_int & 7i32) - 1i32)
                                   as mrb_int
                           } else { (*a).as_0.heap.len } ==
                               0i32 as libc::c_longlong) as libc::c_int as
                              mrb_bool);
}
/*
 *  call-seq:
 *     ary.join(sep="")    -> str
 *
 *  Returns a string created by converting each element of the array to
 *  a string, separated by <i>sep</i>.
 *
 *     [ "a", "b", "c" ].join        #=> "abc"
 *     [ "a", "b", "c" ].join("-")   #=> "a-b-c"
 */
unsafe extern "C" fn mrb_ary_join_m(mut mrb: *mut mrb_state,
                                    mut ary: mrb_value) -> mrb_value {
    let mut sep: mrb_value = mrb_nil_value();
    mrb_get_args(mrb, b"|S!\x00" as *const u8 as *const libc::c_char,
                 &mut sep as *mut mrb_value);
    return mrb_ary_join(mrb, ary, sep);
}
unsafe extern "C" fn mrb_ary_eq(mut mrb: *mut mrb_state, mut ary1: mrb_value)
 -> mrb_value {
    let mut ary2: mrb_value =
        mrb_value{value: unnamed{f: 0.,}, tt: MRB_TT_FALSE,};
    mrb_get_args(mrb, b"o\x00" as *const u8 as *const libc::c_char,
                 &mut ary2 as *mut mrb_value);
    if 0 != mrb_obj_equal(mrb, ary1, ary2) { return mrb_true_value() }
    if !(ary2.tt as libc::c_uint ==
             MRB_TT_ARRAY as libc::c_int as libc::c_uint) {
        return mrb_false_value()
    }
    if if 0 != (*(ary1.value.p as *mut RArray)).flags() as libc::c_int & 7i32
          {
           (((*(ary1.value.p as *mut RArray)).flags() as libc::c_int & 7i32) -
                1i32) as mrb_int
       } else { (*(ary1.value.p as *mut RArray)).as_0.heap.len } !=
           if 0 !=
                  (*(ary2.value.p as *mut RArray)).flags() as libc::c_int &
                      7i32 {
               (((*(ary2.value.p as *mut RArray)).flags() as libc::c_int &
                     7i32) - 1i32) as mrb_int
           } else { (*(ary2.value.p as *mut RArray)).as_0.heap.len } {
        return mrb_false_value()
    }
    return ary2;
}
unsafe extern "C" fn mrb_ary_cmp(mut mrb: *mut mrb_state, mut ary1: mrb_value)
 -> mrb_value {
    let mut ary2: mrb_value =
        mrb_value{value: unnamed{f: 0.,}, tt: MRB_TT_FALSE,};
    mrb_get_args(mrb, b"o\x00" as *const u8 as *const libc::c_char,
                 &mut ary2 as *mut mrb_value);
    if 0 != mrb_obj_equal(mrb, ary1, ary2) {
        return mrb_fixnum_value(0i32 as mrb_int)
    }
    if !(ary2.tt as libc::c_uint ==
             MRB_TT_ARRAY as libc::c_int as libc::c_uint) {
        return mrb_nil_value()
    }
    return ary2;
}
/* internal method to convert multi-value to single value */
unsafe extern "C" fn mrb_ary_svalue(mut mrb: *mut mrb_state,
                                    mut ary: mrb_value) -> mrb_value {
    mrb_get_args(mrb, b"\x00" as *const u8 as *const libc::c_char);
    match if 0 !=
                 (*(ary.value.p as *mut RArray)).flags() as libc::c_int & 7i32
             {
              (((*(ary.value.p as *mut RArray)).flags() as libc::c_int & 7i32)
                   - 1i32) as mrb_int
          } else { (*(ary.value.p as *mut RArray)).as_0.heap.len } {
        0 => { return mrb_nil_value() }
        1 => {
            return *if 0 !=
                           (*(ary.value.p as *mut RArray)).flags() as
                               libc::c_int & 7i32 {
                        &mut (*(ary.value.p as *mut RArray)).as_0 as
                            *mut unnamed_0 as *mut mrb_value
                    } else {
                        (*(ary.value.p as *mut RArray)).as_0.heap.ptr
                    }.offset(0isize)
        }
        _ => { return ary }
    };
}
#[no_mangle]
pub unsafe extern "C" fn mrb_init_array(mut mrb: *mut mrb_state) {
    let mut a: *mut RClass = 0 as *mut RClass;
    a =
        mrb_define_class(mrb,
                         b"Array\x00" as *const u8 as *const libc::c_char,
                         (*mrb).object_class);
    (*mrb).array_class = a;
    (*a).set_flags(((*a).flags() as libc::c_int & !0xffi32 |
                        MRB_TT_ARRAY as libc::c_int as libc::c_char as
                            libc::c_int) as uint32_t);
    mrb_define_class_method(mrb, a,
                            b"[]\x00" as *const u8 as *const libc::c_char,
                            Some(mrb_ary_s_create),
                            (1i32 << 12i32) as mrb_aspec);
    mrb_define_method(mrb, a, b"+\x00" as *const u8 as *const libc::c_char,
                      Some(mrb_ary_plus),
                      ((1i32 & 0x1fi32) as mrb_aspec) << 18i32);
    mrb_define_method(mrb, a, b"*\x00" as *const u8 as *const libc::c_char,
                      Some(mrb_ary_times),
                      ((1i32 & 0x1fi32) as mrb_aspec) << 18i32);
    mrb_define_method(mrb, a, b"<<\x00" as *const u8 as *const libc::c_char,
                      Some(mrb_ary_push_m),
                      ((1i32 & 0x1fi32) as mrb_aspec) << 18i32);
    mrb_define_method(mrb, a, b"[]\x00" as *const u8 as *const libc::c_char,
                      Some(mrb_ary_aget), (1i32 << 12i32) as mrb_aspec);
    mrb_define_method(mrb, a, b"[]=\x00" as *const u8 as *const libc::c_char,
                      Some(mrb_ary_aset), (1i32 << 12i32) as mrb_aspec);
    mrb_define_method(mrb, a,
                      b"clear\x00" as *const u8 as *const libc::c_char,
                      Some(mrb_ary_clear_m), 0i32 as mrb_aspec);
    mrb_define_method(mrb, a,
                      b"concat\x00" as *const u8 as *const libc::c_char,
                      Some(mrb_ary_concat_m),
                      ((1i32 & 0x1fi32) as mrb_aspec) << 18i32);
    mrb_define_method(mrb, a,
                      b"delete_at\x00" as *const u8 as *const libc::c_char,
                      Some(mrb_ary_delete_at),
                      ((1i32 & 0x1fi32) as mrb_aspec) << 18i32);
    mrb_define_method(mrb, a,
                      b"empty?\x00" as *const u8 as *const libc::c_char,
                      Some(mrb_ary_empty_p), 0i32 as mrb_aspec);
    mrb_define_method(mrb, a,
                      b"first\x00" as *const u8 as *const libc::c_char,
                      Some(mrb_ary_first),
                      ((1i32 & 0x1fi32) as mrb_aspec) << 13i32);
    mrb_define_method(mrb, a,
                      b"index\x00" as *const u8 as *const libc::c_char,
                      Some(mrb_ary_index_m),
                      ((1i32 & 0x1fi32) as mrb_aspec) << 18i32);
    mrb_define_method(mrb, a,
                      b"initialize_copy\x00" as *const u8 as
                          *const libc::c_char, Some(mrb_ary_replace_m),
                      ((1i32 & 0x1fi32) as mrb_aspec) << 18i32);
    mrb_define_method(mrb, a, b"join\x00" as *const u8 as *const libc::c_char,
                      Some(mrb_ary_join_m), (1i32 << 12i32) as mrb_aspec);
    mrb_define_method(mrb, a, b"last\x00" as *const u8 as *const libc::c_char,
                      Some(mrb_ary_last), (1i32 << 12i32) as mrb_aspec);
    mrb_define_method(mrb, a,
                      b"length\x00" as *const u8 as *const libc::c_char,
                      Some(mrb_ary_size), 0i32 as mrb_aspec);
    mrb_define_method(mrb, a, b"pop\x00" as *const u8 as *const libc::c_char,
                      Some(mrb_ary_pop), 0i32 as mrb_aspec);
    mrb_define_method(mrb, a, b"push\x00" as *const u8 as *const libc::c_char,
                      Some(mrb_ary_push_m), (1i32 << 12i32) as mrb_aspec);
    mrb_define_method(mrb, a,
                      b"replace\x00" as *const u8 as *const libc::c_char,
                      Some(mrb_ary_replace_m),
                      ((1i32 & 0x1fi32) as mrb_aspec) << 18i32);
    mrb_define_method(mrb, a,
                      b"reverse\x00" as *const u8 as *const libc::c_char,
                      Some(mrb_ary_reverse), 0i32 as mrb_aspec);
    mrb_define_method(mrb, a,
                      b"reverse!\x00" as *const u8 as *const libc::c_char,
                      Some(mrb_ary_reverse_bang), 0i32 as mrb_aspec);
    mrb_define_method(mrb, a,
                      b"rindex\x00" as *const u8 as *const libc::c_char,
                      Some(mrb_ary_rindex_m),
                      ((1i32 & 0x1fi32) as mrb_aspec) << 18i32);
    mrb_define_method(mrb, a,
                      b"shift\x00" as *const u8 as *const libc::c_char,
                      Some(mrb_ary_shift), 0i32 as mrb_aspec);
    mrb_define_method(mrb, a, b"size\x00" as *const u8 as *const libc::c_char,
                      Some(mrb_ary_size), 0i32 as mrb_aspec);
    mrb_define_method(mrb, a,
                      b"slice\x00" as *const u8 as *const libc::c_char,
                      Some(mrb_ary_aget), (1i32 << 12i32) as mrb_aspec);
    mrb_define_method(mrb, a,
                      b"unshift\x00" as *const u8 as *const libc::c_char,
                      Some(mrb_ary_unshift_m), (1i32 << 12i32) as mrb_aspec);
    mrb_define_method(mrb, a,
                      b"__ary_eq\x00" as *const u8 as *const libc::c_char,
                      Some(mrb_ary_eq),
                      ((1i32 & 0x1fi32) as mrb_aspec) << 18i32);
    mrb_define_method(mrb, a,
                      b"__ary_cmp\x00" as *const u8 as *const libc::c_char,
                      Some(mrb_ary_cmp),
                      ((1i32 & 0x1fi32) as mrb_aspec) << 18i32);
    mrb_define_method(mrb, a,
                      b"__ary_index\x00" as *const u8 as *const libc::c_char,
                      Some(mrb_ary_index_m),
                      ((1i32 & 0x1fi32) as mrb_aspec) << 18i32);
    mrb_define_method(mrb, a,
                      b"__svalue\x00" as *const u8 as *const libc::c_char,
                      Some(mrb_ary_svalue), 0i32 as mrb_aspec);
}