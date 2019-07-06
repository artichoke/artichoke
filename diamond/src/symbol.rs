use libc;
use c2rust_bitfields::{BitfieldStruct};
extern "C" {
    pub type iv_tbl;
    pub type kh_mt;
    pub type RProc;
    pub type REnv;
    pub type mrb_jmpbuf;
    pub type mrb_shared_string;
    #[no_mangle]
    fn memcmp(_: *const libc::c_void, _: *const libc::c_void,
              _: libc::c_ulong) -> libc::c_int;
    #[no_mangle]
    fn memcpy(_: *mut libc::c_void, _: *const libc::c_void, _: libc::c_ulong)
     -> *mut libc::c_void;
    #[no_mangle]
    fn strchr(_: *const libc::c_char, _: libc::c_int) -> *mut libc::c_char;
    #[no_mangle]
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
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
 * Undefine a class method.
 * Example:
 *
 *      # Ruby style
 *      class ExampleClass
 *        def self.example_method
 *          "example"
 *        end
 *      end
 *
 *     ExampleClass.example_method
 *
 *     // C style
 *     #include <stdio.h>
 *     #include <mruby.h>
 *
 *     mrb_value
 *     mrb_example_method(mrb_state *mrb){
 *       return mrb_str_new_lit(mrb, "example");
 *     }
 *
 *     void
 *     mrb_example_gem_init(mrb_state* mrb){
 *       struct RClass *example_class;
 *       example_class = mrb_define_class(mrb, "ExampleClass", mrb->object_class);
 *       mrb_define_class_method(mrb, example_class, "example_method", mrb_example_method, MRB_ARGS_NONE());
 *       mrb_undef_class_method(mrb, example_class, "example_method");
 *      }
 *
 *      void
 *      mrb_example_gem_final(mrb_state* mrb){
 *      }
 * @param [mrb_state*] mrb_state* The mruby state reference.
 * @param [RClass*] RClass* A class the class method will be undefined from.
 * @param [const char*] const char* The name of the class method to be undefined.
 */
    #[no_mangle]
    fn mrb_undef_class_method(_: *mut mrb_state, _: *mut RClass,
                              _: *const libc::c_char);
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
    #[no_mangle]
    fn mrb_malloc(_: *mut mrb_state, _: size_t) -> *mut libc::c_void;
    #[no_mangle]
    fn mrb_realloc(_: *mut mrb_state, _: *mut libc::c_void, _: size_t)
     -> *mut libc::c_void;
    #[no_mangle]
    fn mrb_raise(mrb: *mut mrb_state, c: *mut RClass,
                 msg: *const libc::c_char) -> !;
    #[no_mangle]
    fn mrb_str_dump(mrb: *mut mrb_state, str: mrb_value) -> mrb_value;
    #[no_mangle]
    fn mrb_str_new_static(mrb: *mut mrb_state, p: *const libc::c_char,
                          len: size_t) -> mrb_value;
    #[no_mangle]
    fn mrb_str_new(mrb: *mut mrb_state, p: *const libc::c_char, len: size_t)
     -> mrb_value;
    #[no_mangle]
    fn mrb_free(_: *mut mrb_state, _: *mut libc::c_void);
}
pub type __darwin_size_t = libc::c_ulong;
pub type size_t = __darwin_size_t;
pub type int64_t = libc::c_longlong;
pub type uint8_t = libc::c_uchar;
pub type uint16_t = libc::c_ushort;
pub type uint32_t = libc::c_uint;
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
/*
** symbol.c - Symbol class
**
** See Copyright Notice in mruby.h
*/
/* ------------------------------------------------------ */
#[derive ( BitfieldStruct , Clone , Copy )]
#[repr(C)]
pub struct symbol_name {
    #[bitfield(name = "lit", ty = "mrb_bool", bits = "0..=0")]
    pub lit: [u8; 1],
    pub prev: uint8_t,
    pub len: uint16_t,
    pub _pad: [u8; 4],
    pub name: *const libc::c_char,
}
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
#[derive ( Copy , Clone )]
#[repr(C)]
pub struct unnamed_0 {
    pub len: mrb_int,
    pub aux: unnamed_1,
    pub ptr: *mut libc::c_char,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub union unnamed_1 {
    pub capa: mrb_int,
    pub shared: *mut mrb_shared_string,
    pub fshared: *mut RString,
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
    pub as_0: unnamed_2,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub union unnamed_2 {
    pub heap: unnamed_0,
    pub ary: [libc::c_char; 24],
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
unsafe extern "C" fn mrb_undef_value() -> mrb_value {
    let mut v: mrb_value =
        mrb_value{value: unnamed{f: 0.,}, tt: MRB_TT_FALSE,};
    v.tt = MRB_TT_UNDEF;
    v.value.i = 0i32 as mrb_int;
    return v;
}
/* *
 * Create a symbol
 *
 *     # Ruby style:
 *     :pizza # => :pizza
 *
 *     // C style:
 *     mrb_sym m_sym = mrb_intern_lit(mrb, "pizza"); //  => :pizza
 * @param [mrb_state*] mrb_state* The current mruby state.
 * @param [const char*] const char* The name of the method.
 * @return [mrb_sym] mrb_sym A symbol.
 */
#[no_mangle]
pub unsafe extern "C" fn mrb_intern_cstr(mut mrb: *mut mrb_state,
                                         mut name: *const libc::c_char)
 -> mrb_sym {
    return mrb_intern(mrb, name, strlen(name));
}
#[no_mangle]
pub unsafe extern "C" fn mrb_intern(mut mrb: *mut mrb_state,
                                    mut name: *const libc::c_char,
                                    mut len: size_t) -> mrb_sym {
    return sym_intern(mrb, name, len, 0i32 as mrb_bool);
}
unsafe extern "C" fn sym_intern(mut mrb: *mut mrb_state,
                                mut name: *const libc::c_char,
                                mut len: size_t, mut lit: mrb_bool)
 -> mrb_sym {
    let mut sym: mrb_sym = 0;
    let mut sname: *mut symbol_name = 0 as *mut symbol_name;
    let mut hash: uint8_t = 0;
    sym_validate_len(mrb, len);
    hash = symhash(name, len);
    sym = find_symbol(mrb, name, len as uint16_t, hash);
    if sym > 0i32 as libc::c_uint { return sym }
    (*mrb).symidx = (*mrb).symidx.wrapping_add(1);
    sym = (*mrb).symidx;
    if (*mrb).symcapa < sym as libc::c_ulong {
        if (*mrb).symcapa == 0i32 as libc::c_ulong {
            (*mrb).symcapa = 100i32 as size_t
        } else {
            (*mrb).symcapa =
                (*mrb).symcapa.wrapping_mul(6i32 as
                                                libc::c_ulong).wrapping_div(5i32
                                                                                as
                                                                                libc::c_ulong)
                    as size_t
        }
        (*mrb).symtbl =
            mrb_realloc(mrb, (*mrb).symtbl as *mut libc::c_void,
                        (::std::mem::size_of::<symbol_name>() as
                             libc::c_ulong).wrapping_mul((*mrb).symcapa.wrapping_add(1i32
                                                                                         as
                                                                                         libc::c_ulong)))
                as *mut symbol_name
    }
    sname = &mut *(*mrb).symtbl.offset(sym as isize) as *mut symbol_name;
    (*sname).len = len as uint16_t;
    if 0 != lit as libc::c_int || 0 != 0i32 {
        (*sname).name = name;
        (*sname).set_lit(1i32 as mrb_bool)
    } else {
        let mut p: *mut libc::c_char =
            mrb_malloc(mrb, len.wrapping_add(1i32 as libc::c_ulong)) as
                *mut libc::c_char;
        memcpy(p as *mut libc::c_void, name as *const libc::c_void, len);
        *p.offset(len as isize) = 0i32 as libc::c_char;
        (*sname).name = p as *const libc::c_char;
        (*sname).set_lit(0i32 as mrb_bool)
    }
    if 0 != (*mrb).symhash[hash as usize] {
        let mut i: mrb_sym = sym.wrapping_sub((*mrb).symhash[hash as usize]);
        if i > 0xffi32 as libc::c_uint {
            (*sname).prev = 0xffi32 as uint8_t
        } else { (*sname).prev = i as uint8_t }
    } else { (*sname).prev = 0i32 as uint8_t }
    (*mrb).symhash[hash as usize] = sym;
    return sym << 1i32;
}
unsafe extern "C" fn find_symbol(mut mrb: *mut mrb_state,
                                 mut name: *const libc::c_char,
                                 mut len: uint16_t, mut hash: uint8_t)
 -> mrb_sym {
    let mut i: mrb_sym = 0;
    let mut sname: *mut symbol_name = 0 as *mut symbol_name;
    i = sym_inline_pack(name, len);
    if i > 0i32 as libc::c_uint { return i }
    i = (*mrb).symhash[hash as usize];
    if i == 0i32 as libc::c_uint { return 0i32 as mrb_sym }
    loop  {
        sname = &mut *(*mrb).symtbl.offset(i as isize) as *mut symbol_name;
        if (*sname).len as libc::c_int == len as libc::c_int &&
               memcmp((*sname).name as *const libc::c_void,
                      name as *const libc::c_void, len as libc::c_ulong) ==
                   0i32 {
            return i << 1i32
        }
        if (*sname).prev as libc::c_int == 0xffi32 {
            i =
                (i as libc::c_uint).wrapping_sub(0xffi32 as libc::c_uint) as
                    mrb_sym as mrb_sym;
            sname =
                &mut *(*mrb).symtbl.offset(i as isize) as *mut symbol_name;
            while (*mrb).symtbl < sname {
                if (*sname).len as libc::c_int == len as libc::c_int &&
                       memcmp((*sname).name as *const libc::c_void,
                              name as *const libc::c_void,
                              len as libc::c_ulong) == 0i32 {
                    return (sname.wrapping_offset_from((*mrb).symtbl) as
                                libc::c_long as mrb_sym) << 1i32
                }
                sname = sname.offset(-1isize)
            }
            return 0i32 as mrb_sym
        }
        i =
            (i as libc::c_uint).wrapping_sub((*sname).prev as libc::c_uint) as
                mrb_sym as mrb_sym;
        if !((*sname).prev as libc::c_int > 0i32) { break ; }
    }
    return 0i32 as mrb_sym;
}
unsafe extern "C" fn sym_inline_pack(mut name: *const libc::c_char,
                                     mut len: uint16_t) -> mrb_sym {
    let lower_length_max: libc::c_int =
        (::std::mem::size_of::<mrb_sym>() as
             libc::c_ulong).wrapping_mul(8i32 as
                                             libc::c_ulong).wrapping_sub(2i32
                                                                             as
                                                                             libc::c_ulong).wrapping_div(5i32
                                                                                                             as
                                                                                                             libc::c_ulong)
            as libc::c_int;
    let mix_length_max: libc::c_int =
        (::std::mem::size_of::<mrb_sym>() as
             libc::c_ulong).wrapping_mul(8i32 as
                                             libc::c_ulong).wrapping_sub(2i32
                                                                             as
                                                                             libc::c_ulong).wrapping_div(6i32
                                                                                                             as
                                                                                                             libc::c_ulong)
            as libc::c_int;
    let mut c: libc::c_char = 0;
    let mut p: *const libc::c_char = 0 as *const libc::c_char;
    let mut i: libc::c_int = 0;
    let mut sym: mrb_sym = 0i32 as mrb_sym;
    let mut lower: libc::c_int = 1i32;
    if len as libc::c_int > lower_length_max { return 0i32 as mrb_sym }
    i = 0i32;
    while i < len as libc::c_int {
        let mut bits: uint32_t = 0;
        c = *name.offset(i as isize);
        if c as libc::c_int == 0i32 { return 0i32 as mrb_sym }
        p = strchr(pack_table.as_ptr(), c as libc::c_int);
        if p.is_null() { return 0i32 as mrb_sym }
        bits =
            (p.wrapping_offset_from(pack_table.as_ptr()) as libc::c_long as
                 uint32_t).wrapping_add(1i32 as libc::c_uint);
        if bits > 27i32 as libc::c_uint { lower = 0i32 }
        if i >= mix_length_max { break ; }
        sym |= bits << i * 6i32 + 2i32;
        i += 1
    }
    if 0 != lower {
        sym = 0i32 as mrb_sym;
        i = 0i32;
        while i < len as libc::c_int {
            let mut bits_0: uint32_t = 0;
            c = *name.offset(i as isize);
            p = strchr(pack_table.as_ptr(), c as libc::c_int);
            bits_0 =
                (p.wrapping_offset_from(pack_table.as_ptr()) as libc::c_long
                     as uint32_t).wrapping_add(1i32 as libc::c_uint);
            sym |= bits_0 << i * 5i32 + 2i32;
            i += 1
        }
        return sym | 3i32 as libc::c_uint
    }
    if len as libc::c_int > mix_length_max { return 0i32 as mrb_sym }
    return sym | 1i32 as libc::c_uint;
}
static mut pack_table: [libc::c_char; 64] =
    [95, 97, 98, 99, 100, 101, 102, 103, 104, 105, 106, 107, 108, 109, 110,
     111, 112, 113, 114, 115, 116, 117, 118, 119, 120, 121, 122, 65, 66, 67,
     68, 69, 70, 71, 72, 73, 74, 75, 76, 77, 78, 79, 80, 81, 82, 83, 84, 85,
     86, 87, 88, 89, 90, 48, 49, 50, 51, 52, 53, 54, 55, 56, 57, 0];
unsafe extern "C" fn symhash(mut key: *const libc::c_char, mut len: size_t)
 -> uint8_t {
    let mut hash: uint32_t = 0;
    let mut i: uint32_t = 0;
    i = 0i32 as uint32_t;
    hash = i;
    while (i as libc::c_ulong) < len {
        hash =
            (hash as
                 libc::c_uint).wrapping_add(*key.offset(i as isize) as
                                                libc::c_uint) as uint32_t as
                uint32_t;
        hash =
            (hash as libc::c_uint).wrapping_add(hash << 10i32) as uint32_t as
                uint32_t;
        hash ^= hash >> 6i32;
        i = i.wrapping_add(1)
    }
    hash =
        (hash as libc::c_uint).wrapping_add(hash << 3i32) as uint32_t as
            uint32_t;
    hash ^= hash >> 11i32;
    hash =
        (hash as libc::c_uint).wrapping_add(hash << 15i32) as uint32_t as
            uint32_t;
    return (hash & 0xffi32 as libc::c_uint) as uint8_t;
}
unsafe extern "C" fn sym_validate_len(mut mrb: *mut mrb_state,
                                      mut len: size_t) {
    if len >= 65535i32 as libc::c_ulong {
        mrb_raise(mrb,
                  mrb_exc_get(mrb,
                              b"ArgumentError\x00" as *const u8 as
                                  *const libc::c_char),
                  b"symbol length too long\x00" as *const u8 as
                      *const libc::c_char);
    };
}
#[no_mangle]
pub unsafe extern "C" fn mrb_intern_static(mut mrb: *mut mrb_state,
                                           mut name: *const libc::c_char,
                                           mut len: size_t) -> mrb_sym {
    return sym_intern(mrb, name, len, 1i32 as mrb_bool);
}
#[no_mangle]
pub unsafe extern "C" fn mrb_intern_str(mut mrb: *mut mrb_state,
                                        mut str: mrb_value) -> mrb_sym {
    return mrb_intern(mrb,
                      if 0 !=
                             (*(str.value.p as *mut RString)).flags() as
                                 libc::c_int & 32i32 {
                          (*(str.value.p as
                                 *mut RString)).as_0.ary.as_mut_ptr()
                      } else {
                          (*(str.value.p as *mut RString)).as_0.heap.ptr
                      },
                      (if 0 !=
                              (*(str.value.p as *mut RString)).flags() as
                                  libc::c_int & 32i32 {
                           (((*(str.value.p as *mut RString)).flags() as
                                 libc::c_int & 0x7c0i32) >> 6i32) as mrb_int
                       } else {
                           (*(str.value.p as *mut RString)).as_0.heap.len
                       }) as size_t);
}
#[no_mangle]
pub unsafe extern "C" fn mrb_check_intern_cstr(mut mrb: *mut mrb_state,
                                               mut name: *const libc::c_char)
 -> mrb_value {
    return mrb_check_intern(mrb, name, strlen(name));
}
#[no_mangle]
pub unsafe extern "C" fn mrb_check_intern(mut mrb: *mut mrb_state,
                                          mut name: *const libc::c_char,
                                          mut len: size_t) -> mrb_value {
    let mut sym: mrb_sym = 0;
    sym_validate_len(mrb, len);
    sym = find_symbol(mrb, name, len as uint16_t, symhash(name, len));
    if sym > 0i32 as libc::c_uint { return mrb_symbol_value(sym) }
    return mrb_nil_value();
}
#[no_mangle]
pub unsafe extern "C" fn mrb_check_intern_str(mut mrb: *mut mrb_state,
                                              mut str: mrb_value)
 -> mrb_value {
    return mrb_check_intern(mrb,
                            if 0 !=
                                   (*(str.value.p as *mut RString)).flags() as
                                       libc::c_int & 32i32 {
                                (*(str.value.p as
                                       *mut RString)).as_0.ary.as_mut_ptr()
                            } else {
                                (*(str.value.p as *mut RString)).as_0.heap.ptr
                            },
                            (if 0 !=
                                    (*(str.value.p as *mut RString)).flags()
                                        as libc::c_int & 32i32 {
                                 (((*(str.value.p as *mut RString)).flags() as
                                       libc::c_int & 0x7c0i32) >> 6i32) as
                                     mrb_int
                             } else {
                                 (*(str.value.p as
                                        *mut RString)).as_0.heap.len
                             }) as size_t);
}
#[no_mangle]
pub unsafe extern "C" fn mrb_sym2name(mut mrb: *mut mrb_state,
                                      mut sym: mrb_sym)
 -> *const libc::c_char {
    let mut len: mrb_int = 0;
    let mut name: *const libc::c_char = mrb_sym2name_len(mrb, sym, &mut len);
    if name.is_null() { return 0 as *const libc::c_char }
    if 0 != symname_p(name) as libc::c_int && strlen(name) == len as size_t {
        return name
    } else {
        let mut str: mrb_value =
            mrb_value{value: unnamed{f: 0.,}, tt: MRB_TT_FALSE,};
        if 0 != sym & 1i32 as libc::c_uint {
            str = mrb_str_new(mrb, name, len as size_t)
        } else { str = mrb_str_new_static(mrb, name, len as size_t) }
        str = mrb_str_dump(mrb, str);
        return if 0 !=
                      (*(str.value.p as *mut RString)).flags() as libc::c_int
                          & 32i32 {
                   (*(str.value.p as *mut RString)).as_0.ary.as_mut_ptr()
               } else { (*(str.value.p as *mut RString)).as_0.heap.ptr }
    };
}
#[no_mangle]
pub unsafe extern "C" fn mrb_sym2name_len(mut mrb: *mut mrb_state,
                                          mut sym: mrb_sym,
                                          mut lenp: *mut mrb_int)
 -> *const libc::c_char {
    return sym2name_len(mrb, sym, (*mrb).symbuf.as_mut_ptr(), lenp);
}
unsafe extern "C" fn sym2name_len(mut mrb: *mut mrb_state, mut sym: mrb_sym,
                                  mut buf: *mut libc::c_char,
                                  mut lenp: *mut mrb_int)
 -> *const libc::c_char {
    if 0 != sym & 1i32 as libc::c_uint {
        return sym_inline_unpack(sym, buf, lenp)
    }
    sym >>= 1i32;
    if sym == 0i32 as libc::c_uint || (*mrb).symidx < sym {
        if !lenp.is_null() { *lenp = 0i32 as mrb_int }
        return 0 as *const libc::c_char
    }
    if !lenp.is_null() {
        *lenp = (*(*mrb).symtbl.offset(sym as isize)).len as mrb_int
    }
    return (*(*mrb).symtbl.offset(sym as isize)).name;
}
unsafe extern "C" fn sym_inline_unpack(mut sym: mrb_sym,
                                       mut buf: *mut libc::c_char,
                                       mut lenp: *mut mrb_int)
 -> *const libc::c_char {
    /* all lower case if `sym&2` is true */
    let mut bit_per_char: libc::c_int =
        if 0 != sym & 2i32 as libc::c_uint { 5i32 } else { 6i32 };
    let mut i: libc::c_int = 0;
    i = 0i32;
    while i < 30i32 / bit_per_char {
        let mut bits: uint32_t =
            sym >> i * bit_per_char + 2i32 &
                ((1i32 << bit_per_char) - 1i32) as libc::c_uint;
        if bits == 0i32 as libc::c_uint { break ; }
        *buf.offset(i as isize) =
            pack_table[bits.wrapping_sub(1i32 as libc::c_uint) as usize];
        i += 1
    }
    *buf.offset(i as isize) = '\u{0}' as i32 as libc::c_char;
    if !lenp.is_null() { *lenp = i as mrb_int }
    return buf;
}
unsafe extern "C" fn symname_p(mut name: *const libc::c_char) -> mrb_bool {
    let mut current_block: u64;
    let mut m: *const libc::c_char = name;
    let mut localid: mrb_bool = 0i32 as mrb_bool;
    if m.is_null() { return 0i32 as mrb_bool }
    match *m as libc::c_int {
        0 => { return 0i32 as mrb_bool }
        36 => {
            m = m.offset(1isize);
            if 0 != is_special_global_name(m) { return 1i32 as mrb_bool }
            current_block = 6409029412943682534;
        }
        64 => {
            m = m.offset(1isize);
            if *m as libc::c_int == '@' as i32 { m = m.offset(1isize) }
            current_block = 6409029412943682534;
        }
        60 => {
            m = m.offset(1isize);
            match *m as libc::c_int {
                60 => { m = m.offset(1isize) }
                61 => {
                    m = m.offset(1isize);
                    if *m as libc::c_int == '>' as i32 {
                        m = m.offset(1isize)
                    }
                }
                _ => { }
            }
            current_block = 9437013279121998969;
        }
        62 => {
            m = m.offset(1isize);
            match *m as libc::c_int {
                62 | 61 => { m = m.offset(1isize) }
                _ => { }
            }
            current_block = 9437013279121998969;
        }
        61 => {
            m = m.offset(1isize);
            match *m as libc::c_int {
                126 => { m = m.offset(1isize) }
                61 => {
                    m = m.offset(1isize);
                    if *m as libc::c_int == '=' as i32 {
                        m = m.offset(1isize)
                    }
                }
                _ => { return 0i32 as mrb_bool }
            }
            current_block = 9437013279121998969;
        }
        42 => {
            m = m.offset(1isize);
            if *m as libc::c_int == '*' as i32 { m = m.offset(1isize) }
            current_block = 9437013279121998969;
        }
        33 => {
            m = m.offset(1isize);
            match *m as libc::c_int {
                61 | 126 => { m = m.offset(1isize) }
                _ => { }
            }
            current_block = 9437013279121998969;
        }
        43 | 45 => {
            m = m.offset(1isize);
            if *m as libc::c_int == '@' as i32 { m = m.offset(1isize) }
            current_block = 9437013279121998969;
        }
        124 => {
            m = m.offset(1isize);
            if *m as libc::c_int == '|' as i32 { m = m.offset(1isize) }
            current_block = 9437013279121998969;
        }
        38 => {
            m = m.offset(1isize);
            if *m as libc::c_int == '&' as i32 { m = m.offset(1isize) }
            current_block = 9437013279121998969;
        }
        94 | 47 | 37 | 126 | 96 => {
            m = m.offset(1isize);
            current_block = 9437013279121998969;
        }
        91 => {
            m = m.offset(1isize);
            if *m as libc::c_int != ']' as i32 { return 0i32 as mrb_bool }
            m = m.offset(1isize);
            if *m as libc::c_int == '=' as i32 { m = m.offset(1isize) }
            current_block = 9437013279121998969;
        }
        _ => {
            localid =
                !((*m as
                       libc::c_uint).wrapping_sub('A' as i32 as libc::c_uint)
                      < 26i32 as libc::c_uint) as libc::c_int as mrb_bool;
            current_block = 6409029412943682534;
        }
    }
    match current_block {
        6409029412943682534 => {
            if *m as libc::c_int != '_' as i32 &&
                   !((*m as libc::c_uint |
                          0x20i32 as
                              libc::c_uint).wrapping_sub('a' as i32 as
                                                             libc::c_uint) <
                         26i32 as libc::c_uint) {
                return 0i32 as mrb_bool
            }
            while *m as libc::c_schar as libc::c_int != -1i32 &&
                      ((*m as libc::c_uint |
                            0x20i32 as
                                libc::c_uint).wrapping_sub('a' as i32 as
                                                               libc::c_uint) <
                           26i32 as libc::c_uint ||
                           (*m as
                                libc::c_uint).wrapping_sub('0' as i32 as
                                                               libc::c_uint) <
                               10i32 as libc::c_uint ||
                           *m as libc::c_int == '_' as i32) {
                m = m.offset(1isize)
            }
            if 0 != localid {
                match *m as libc::c_int {
                    33 | 63 | 61 => { m = m.offset(1isize) }
                    _ => { }
                }
            }
        }
        _ => { }
    }
    return (if 0 != *m as libc::c_int { 0i32 } else { 1i32 }) as mrb_bool;
}
/* 15.2.11.3.5(x)  */
/*
 *  call-seq:
 *     sym.inspect    -> string
 *
 *  Returns the representation of <i>sym</i> as a symbol literal.
 *
 *     :fred.inspect   #=> ":fred"
 */
/* not __STDC__ */
unsafe extern "C" fn is_special_global_name(mut m: *const libc::c_char)
 -> mrb_bool {
    match *m as libc::c_int {
        126 | 42 | 36 | 63 | 33 | 64 | 47 | 92 | 59 | 44 | 46 | 61 | 58 | 60 |
        62 | 34 | 38 | 96 | 39 | 43 | 48 => {
            m = m.offset(1isize)
        }
        45 => {
            m = m.offset(1isize);
            if *m as libc::c_schar as libc::c_int != -1i32 &&
                   ((*m as libc::c_uint |
                         0x20i32 as
                             libc::c_uint).wrapping_sub('a' as i32 as
                                                            libc::c_uint) <
                        26i32 as libc::c_uint ||
                        (*m as
                             libc::c_uint).wrapping_sub('0' as i32 as
                                                            libc::c_uint) <
                            10i32 as libc::c_uint ||
                        *m as libc::c_int == '_' as i32) {
                m = m.offset(1isize)
            }
        }
        _ => {
            if !((*m as libc::c_uint).wrapping_sub('0' as i32 as libc::c_uint)
                     < 10i32 as libc::c_uint) {
                return 0i32 as mrb_bool
            }
            loop  {
                m = m.offset(1isize);
                if !((*m as
                          libc::c_uint).wrapping_sub('0' as i32 as
                                                         libc::c_uint) <
                         10i32 as libc::c_uint) {
                    break ;
                }
            }
        }
    }
    return (0 == *m) as libc::c_int as mrb_bool;
}
#[no_mangle]
pub unsafe extern "C" fn mrb_sym2str(mut mrb: *mut mrb_state,
                                     mut sym: mrb_sym) -> mrb_value {
    let mut len: mrb_int = 0;
    let mut name: *const libc::c_char = mrb_sym2name_len(mrb, sym, &mut len);
    if name.is_null() { return mrb_undef_value() }
    if 0 != sym & 1i32 as libc::c_uint {
        return mrb_str_new(mrb, name, len as size_t)
    }
    return mrb_str_new_static(mrb, name, len as size_t);
}
#[no_mangle]
pub unsafe extern "C" fn mrb_free_symtbl(mut mrb: *mut mrb_state) {
    let mut i: mrb_sym = 0;
    let mut lim: mrb_sym = 0;
    i = 1i32 as mrb_sym;
    lim = (*mrb).symidx.wrapping_add(1i32 as libc::c_uint);
    while i < lim {
        if 0 == (*(*mrb).symtbl.offset(i as isize)).lit() {
            mrb_free(mrb,
                     (*(*mrb).symtbl.offset(i as isize)).name as
                         *mut libc::c_char as *mut libc::c_void);
        }
        i = i.wrapping_add(1)
    }
    mrb_free(mrb, (*mrb).symtbl as *mut libc::c_void);
}
#[no_mangle]
pub unsafe extern "C" fn mrb_init_symtbl(mut mrb: *mut mrb_state) { }
/* *********************************************************************
 * Document-class: Symbol
 *
 *  <code>Symbol</code> objects represent names and some strings
 *  inside the Ruby
 *  interpreter. They are generated using the <code>:name</code> and
 *  <code>:"string"</code> literals
 *  syntax, and by the various <code>to_sym</code> methods. The same
 *  <code>Symbol</code> object will be created for a given name or string
 *  for the duration of a program's execution, regardless of the context
 *  or meaning of that name. Thus if <code>Fred</code> is a constant in
 *  one context, a method in another, and a class in a third, the
 *  <code>Symbol</code> <code>:Fred</code> will be the same object in
 *  all three contexts.
 *
 *     module One
 *       class Fred
 *       end
 *       $f1 = :Fred
 *     end
 *     module Two
 *       Fred = 1
 *       $f2 = :Fred
 *     end
 *     def Fred()
 *     end
 *     $f3 = :Fred
 *     $f1.object_id   #=> 2514190
 *     $f2.object_id   #=> 2514190
 *     $f3.object_id   #=> 2514190
 *
 */
/* 15.2.11.3.2  */
/* 15.2.11.3.3  */
/*
 *  call-seq:
 *     sym.id2name   -> string
 *     sym.to_s      -> string
 *
 *  Returns the name or string corresponding to <i>sym</i>.
 *
 *     :fred.id2name   #=> "fred"
 */
unsafe extern "C" fn sym_to_s(mut mrb: *mut mrb_state, mut sym: mrb_value)
 -> mrb_value {
    return mrb_sym2str(mrb, sym.value.sym);
}
/* 15.2.11.3.4  */
/*
 * call-seq:
 *   sym.to_sym   -> sym
 *   sym.intern   -> sym
 *
 * In general, <code>to_sym</code> returns the <code>Symbol</code> corresponding
 * to an object. As <i>sym</i> is already a symbol, <code>self</code> is returned
 * in this case.
 */
unsafe extern "C" fn sym_to_sym(mut mrb: *mut mrb_state, mut sym: mrb_value)
 -> mrb_value {
    return sym;
}
unsafe extern "C" fn sym_inspect(mut mrb: *mut mrb_state, mut sym: mrb_value)
 -> mrb_value {
    let mut str: mrb_value =
        mrb_value{value: unnamed{f: 0.,}, tt: MRB_TT_FALSE,};
    let mut name: *const libc::c_char = 0 as *const libc::c_char;
    let mut len: mrb_int = 0;
    let mut id: mrb_sym = sym.value.sym;
    let mut sp: *mut libc::c_char = 0 as *mut libc::c_char;
    name = mrb_sym2name_len(mrb, id, &mut len);
    str =
        mrb_str_new(mrb, 0 as *const libc::c_char,
                    (len + 1i32 as libc::c_longlong) as size_t);
    sp =
        if 0 !=
               (*(str.value.p as *mut RString)).flags() as libc::c_int & 32i32
           {
            (*(str.value.p as *mut RString)).as_0.ary.as_mut_ptr()
        } else { (*(str.value.p as *mut RString)).as_0.heap.ptr };
    *if 0 != (*(str.value.p as *mut RString)).flags() as libc::c_int & 32i32 {
         (*(str.value.p as *mut RString)).as_0.ary.as_mut_ptr()
     } else { (*(str.value.p as *mut RString)).as_0.heap.ptr }.offset(0isize)
        = ':' as i32 as libc::c_char;
    memcpy(sp.offset(1isize) as *mut libc::c_void,
           name as *const libc::c_void, len as libc::c_ulong);
    if 0 == symname_p(name) || strlen(name) != len as size_t {
        str = mrb_str_dump(mrb, str);
        sp =
            if 0 !=
                   (*(str.value.p as *mut RString)).flags() as libc::c_int &
                       32i32 {
                (*(str.value.p as *mut RString)).as_0.ary.as_mut_ptr()
            } else { (*(str.value.p as *mut RString)).as_0.heap.ptr };
        *sp.offset(0isize) = ':' as i32 as libc::c_char;
        *sp.offset(1isize) = '\"' as i32 as libc::c_char
    }
    return str;
}
unsafe extern "C" fn sym_cmp(mut mrb: *mut mrb_state, mut s1: mrb_value)
 -> mrb_value {
    let mut s2: mrb_value =
        mrb_value{value: unnamed{f: 0.,}, tt: MRB_TT_FALSE,};
    let mut sym1: mrb_sym = 0;
    let mut sym2: mrb_sym = 0;
    mrb_get_args(mrb, b"o\x00" as *const u8 as *const libc::c_char,
                 &mut s2 as *mut mrb_value);
    if s2.tt as libc::c_uint != MRB_TT_SYMBOL as libc::c_int as libc::c_uint {
        return mrb_nil_value()
    }
    sym1 = s1.value.sym;
    sym2 = s2.value.sym;
    if sym1 == sym2 {
        return mrb_fixnum_value(0i32 as mrb_int)
    } else {
        let mut p1: *const libc::c_char = 0 as *const libc::c_char;
        let mut p2: *const libc::c_char = 0 as *const libc::c_char;
        let mut retval: libc::c_int = 0;
        let mut len: mrb_int = 0;
        let mut len1: mrb_int = 0;
        let mut len2: mrb_int = 0;
        let mut buf1: [libc::c_char; 8] = [0; 8];
        let mut buf2: [libc::c_char; 8] = [0; 8];
        p1 = sym2name_len(mrb, sym1, buf1.as_mut_ptr(), &mut len1);
        p2 = sym2name_len(mrb, sym2, buf2.as_mut_ptr(), &mut len2);
        len = if len1 > len2 { len2 } else { len1 };
        retval =
            memcmp(p1 as *const libc::c_void, p2 as *const libc::c_void,
                   len as libc::c_ulong);
        if retval == 0i32 {
            if len1 == len2 { return mrb_fixnum_value(0i32 as mrb_int) }
            if len1 > len2 { return mrb_fixnum_value(1i32 as mrb_int) }
            return mrb_fixnum_value(-1i32 as mrb_int)
        }
        if retval > 0i32 { return mrb_fixnum_value(1i32 as mrb_int) }
        return mrb_fixnum_value(-1i32 as mrb_int)
    };
}
#[no_mangle]
pub unsafe extern "C" fn mrb_init_symbol(mut mrb: *mut mrb_state) {
    let mut sym: *mut RClass = 0 as *mut RClass;
    sym =
        mrb_define_class(mrb,
                         b"Symbol\x00" as *const u8 as *const libc::c_char,
                         (*mrb).object_class);
    (*mrb).symbol_class = sym;
    (*sym).set_flags(((*sym).flags() as libc::c_int & !0xffi32 |
                          MRB_TT_SYMBOL as libc::c_int as libc::c_char as
                              libc::c_int) as uint32_t);
    mrb_undef_class_method(mrb, sym,
                           b"new\x00" as *const u8 as *const libc::c_char);
    mrb_define_method(mrb, sym,
                      b"id2name\x00" as *const u8 as *const libc::c_char,
                      Some(sym_to_s), 0i32 as mrb_aspec);
    mrb_define_method(mrb, sym,
                      b"to_s\x00" as *const u8 as *const libc::c_char,
                      Some(sym_to_s), 0i32 as mrb_aspec);
    mrb_define_method(mrb, sym,
                      b"to_sym\x00" as *const u8 as *const libc::c_char,
                      Some(sym_to_sym), 0i32 as mrb_aspec);
    mrb_define_method(mrb, sym,
                      b"inspect\x00" as *const u8 as *const libc::c_char,
                      Some(sym_inspect), 0i32 as mrb_aspec);
    mrb_define_method(mrb, sym,
                      b"<=>\x00" as *const u8 as *const libc::c_char,
                      Some(sym_cmp),
                      ((1i32 & 0x1fi32) as mrb_aspec) << 18i32);
}