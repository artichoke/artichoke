use libc;
use c2rust_bitfields::{BitfieldStruct};
extern "C" {
    pub type iv_tbl;
    /* debug info */
    pub type mrb_irep_debug_info;
    pub type symbol_name;
    pub type mrb_jmpbuf;
    #[no_mangle]
    fn __assert_rtn(_: *const libc::c_char, _: *const libc::c_char,
                    _: libc::c_int, _: *const libc::c_char) -> !;
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
 * Call existing ruby functions with a block.
 */
    #[no_mangle]
    fn mrb_funcall_with_block(_: *mut mrb_state, _: mrb_value, _: mrb_sym,
                              _: mrb_int, _: *const mrb_value, _: mrb_value)
     -> mrb_value;
    #[no_mangle]
    fn mrb_intern_static(_: *mut mrb_state, _: *const libc::c_char, _: size_t)
     -> mrb_sym;
    #[no_mangle]
    fn mrb_malloc(_: *mut mrb_state, _: size_t) -> *mut libc::c_void;
    #[no_mangle]
    fn mrb_obj_alloc(_: *mut mrb_state, _: mrb_vtype, _: *mut RClass)
     -> *mut RBasic;
    #[no_mangle]
    fn mrb_field_write_barrier(_: *mut mrb_state, _: *mut RBasic,
                               _: *mut RBasic);
    #[no_mangle]
    fn mrb_raise(mrb: *mut mrb_state, c: *mut RClass,
                 msg: *const libc::c_char) -> !;
    #[no_mangle]
    fn mrb_raisef(mrb: *mut mrb_state, c: *mut RClass,
                  fmt: *const libc::c_char, _: ...) -> !;
    #[no_mangle]
    fn mrb_define_method_raw(_: *mut mrb_state, _: *mut RClass, _: mrb_sym,
                             _: mrb_method_t);
    #[no_mangle]
    fn mrb_irep_incref(_: *mut mrb_state, _: *mut mrb_irep);
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
/* old name */
/* MRB_METHOD_TABLE_INLINE */
#[derive ( Copy , Clone )]
#[repr(C)]
pub struct kh_mt {
    pub n_buckets: khint_t,
    pub size: khint_t,
    pub n_occupied: khint_t,
    pub ed_flags: *mut uint8_t,
    pub keys: *mut mrb_sym,
    pub vals: *mut mrb_method_t,
}
#[derive ( Copy , Clone )]
#[repr(C)]
pub struct mrb_method_t {
    pub func_p: mrb_bool,
    pub unnamed: unnamed,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub union unnamed {
    pub proc_0: *mut RProc,
    pub func: mrb_func_t,
}
/* default method cache size: 128 */
/* cache size needs to be power of 2 */
pub type mrb_func_t
    =
    Option<unsafe extern "C" fn(_: *mut mrb_state, _: mrb_value)
               -> mrb_value>;
/*
** mruby/boxing_no.h - unboxed mrb_value definition
**
** See Copyright Notice in mruby.h
*/
#[derive ( Copy , Clone )]
#[repr(C)]
pub struct mrb_value {
    pub value: unnamed_0,
    pub tt: mrb_vtype,
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
#[derive ( Copy , Clone )]
#[repr ( C )]
pub union unnamed_0 {
    pub f: mrb_float,
    pub p: *mut libc::c_void,
    pub i: mrb_int,
    pub sym: mrb_sym,
}
pub type mrb_int = int64_t;
pub type mrb_float = libc::c_double;
#[derive ( BitfieldStruct , Clone , Copy )]
#[repr(C)]
pub struct RProc {
    #[bitfield(name = "tt", ty = "mrb_vtype", bits = "0..=7")]
    #[bitfield(name = "color", ty = "uint32_t", bits = "8..=10")]
    #[bitfield(name = "flags", ty = "uint32_t", bits = "11..=31")]
    pub tt_color_flags: [u8; 4],
    pub _pad: [u8; 4],
    pub c: *mut RClass,
    pub gcnext: *mut RBasic,
    pub body: unnamed_2,
    pub upper: *mut RProc,
    pub e: unnamed_1,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub union unnamed_1 {
    pub target_class: *mut RClass,
    pub env: *mut REnv,
}
/*
** mruby/proc.h - Proc class
**
** See Copyright Notice in mruby.h
*/
/* *
 * Proc class
 */
#[derive ( BitfieldStruct , Clone , Copy )]
#[repr(C)]
pub struct REnv {
    #[bitfield(name = "tt", ty = "mrb_vtype", bits = "0..=7")]
    #[bitfield(name = "color", ty = "uint32_t", bits = "8..=10")]
    #[bitfield(name = "flags", ty = "uint32_t", bits = "11..=31")]
    pub tt_color_flags: [u8; 4],
    pub _pad: [u8; 4],
    pub c: *mut RClass,
    pub gcnext: *mut RBasic,
    pub stack: *mut mrb_value,
    pub cxt: *mut mrb_context,
    pub mid: mrb_sym,
    pub _pad2: [u8; 4],
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
#[derive ( Copy , Clone )]
#[repr ( C )]
pub union unnamed_2 {
    pub irep: *mut mrb_irep,
    pub func: mrb_func_t,
}
/* Program data array struct */
#[derive ( Copy , Clone )]
#[repr(C)]
pub struct mrb_irep {
    pub nlocals: uint16_t,
    pub nregs: uint16_t,
    pub flags: uint8_t,
    pub iseq: *mut mrb_code,
    pub pool: *mut mrb_value,
    pub syms: *mut mrb_sym,
    pub reps: *mut *mut mrb_irep,
    pub lv: *mut mrb_locals,
    pub debug_info: *mut mrb_irep_debug_info,
    pub ilen: uint16_t,
    pub plen: uint16_t,
    pub slen: uint16_t,
    pub rlen: uint16_t,
    pub refcnt: uint32_t,
}
#[derive ( Copy , Clone )]
#[repr(C)]
pub struct mrb_locals {
    pub name: mrb_sym,
    pub r: uint16_t,
}
/*
** mruby/khash.c - Hash for mruby
**
** See Copyright Notice in mruby.h
*/
/* *
 * khash definitions used in mruby's hash table.
 */
pub type khint_t = uint32_t;
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
/* arg setup according to flags (23=m5:o5:r1:m5:k5:d1:b1) */
pub const OP_ENTER: mrb_insn = 51;
/*
** mruby/opcode.h - RiteVM operation codes
**
** See Copyright Notice in mruby.h
*/
pub type mrb_insn = libc::c_uint;
/* stop VM */
pub const OP_STOP: mrb_insn = 103;
/* make 1st and 2nd operands 16bit */
pub const OP_EXT3: mrb_insn = 102;
/* make 2nd operand 16bit */
pub const OP_EXT2: mrb_insn = 101;
/* make 1st operand 16bit */
pub const OP_EXT1: mrb_insn = 100;
/* raise(LocalJumpError, Lit(a)) */
pub const OP_ERR: mrb_insn = 99;
/* print a,b,c */
pub const OP_DEBUG: mrb_insn = 98;
/* R(a) = target_class */
pub const OP_TCLASS: mrb_insn = 97;
/* R(a) = R(a).singleton_class */
pub const OP_SCLASS: mrb_insn = 96;
/* undef_method(target_class,Syms(a)) */
pub const OP_UNDEF: mrb_insn = 95;
/* alias_method(target_class,Syms(a),Syms(b)) */
pub const OP_ALIAS: mrb_insn = 94;
/* R(a).newmethod(Syms(b),R(a+1)) */
pub const OP_DEF: mrb_insn = 93;
/* R(a) = blockexec(R(a),SEQ[b]) */
pub const OP_EXEC: mrb_insn = 92;
/* R(a) = newmodule(R(a),Syms(b)) */
pub const OP_MODULE: mrb_insn = 91;
/* R(a) = newclass(R(a),Syms(b),R(a+1)) */
pub const OP_CLASS: mrb_insn = 90;
/* R(a) = ::Object */
pub const OP_OCLASS: mrb_insn = 89;
/* R(a) = range_new(R(a),R(a+1),TRUE) */
pub const OP_RANGE_EXC: mrb_insn = 88;
/* R(a) = range_new(R(a),R(a+1),FALSE) */
pub const OP_RANGE_INC: mrb_insn = 87;
/* R(a) = lambda(SEQ[b],L_METHOD) */
pub const OP_METHOD: mrb_insn = 86;
/* R(a) = lambda(SEQ[b],L_BLOCK) */
pub const OP_BLOCK: mrb_insn = 85;
/* R(a) = lambda(SEQ[b],L_LAMBDA) */
pub const OP_LAMBDA: mrb_insn = 84;
/* R(a) = hash_cat(R(a),R(a+1)) */
pub const OP_HASHCAT: mrb_insn = 83;
/* R(a) = hash_push(R(a),R(a+1)..R(a+b)) */
pub const OP_HASHADD: mrb_insn = 82;
/* R(a) = hash_new(R(a),R(a+1)..R(a+b)) */
pub const OP_HASH: mrb_insn = 81;
/* str_cat(R(a),R(a+1)) */
pub const OP_STRCAT: mrb_insn = 80;
/* R(a) = str_dup(Lit(b)) */
pub const OP_STRING: mrb_insn = 79;
/* R(a) = intern(R(a)) */
pub const OP_INTERN: mrb_insn = 78;
/* *R(a),R(a+1)..R(a+c) = R(a)[b..] */
pub const OP_APOST: mrb_insn = 77;
/* R(a)[c] = R(b) */
pub const OP_ASET: mrb_insn = 76;
/* R(a) = R(b)[c] */
pub const OP_AREF: mrb_insn = 75;
/* R(a) = ary_dup(R(a)) */
pub const OP_ARYDUP: mrb_insn = 74;
/* ary_push(R(a),R(a+1)) */
pub const OP_ARYPUSH: mrb_insn = 73;
/* ary_cat(R(a),R(a+1)) */
pub const OP_ARYCAT: mrb_insn = 72;
/* R(a) = ary_new(R(b),R(b+1)..R(b+c)) */
pub const OP_ARRAY2: mrb_insn = 71;
/* R(a) = ary_new(R(a),R(a+1)..R(a+b)) */
pub const OP_ARRAY: mrb_insn = 70;
/* R(a) = R(a)>=R(a+1) */
pub const OP_GE: mrb_insn = 69;
/* R(a) = R(a)>R(a+1) */
pub const OP_GT: mrb_insn = 68;
/* R(a) = R(a)<=R(a+1) */
pub const OP_LE: mrb_insn = 67;
/* R(a) = R(a)<R(a+1) */
pub const OP_LT: mrb_insn = 66;
/* R(a) = R(a)==R(a+1) */
pub const OP_EQ: mrb_insn = 65;
/* R(a) = R(a)/R(a+1) */
pub const OP_DIV: mrb_insn = 64;
/* R(a) = R(a)*R(a+1) */
pub const OP_MUL: mrb_insn = 63;
/* R(a) = R(a)-C */
pub const OP_SUBI: mrb_insn = 62;
/* R(a) = R(a)-R(a+1) */
pub const OP_SUB: mrb_insn = 61;
/* R(a) = R(a)+mrb_int(c)  */
pub const OP_ADDI: mrb_insn = 60;
/* R(a) = R(a)+R(a+1) */
pub const OP_ADD: mrb_insn = 59;
/* R(a) = block (16=m5:r1:m5:d1:lv4) */
pub const OP_BLKPUSH: mrb_insn = 58;
/* break R(a) */
pub const OP_BREAK: mrb_insn = 57;
/* return R(a) (in-block return) */
pub const OP_RETURN_BLK: mrb_insn = 56;
/* return R(a) (normal) */
pub const OP_RETURN: mrb_insn = 55;
/* R(a) = kdict[Syms(b)]; kdict.delete(Syms(b))    # todo */
pub const OP_KARG: mrb_insn = 54;
/* raise unless kdict.empty?                       # todo */
pub const OP_KEYEND: mrb_insn = 53;
/* R(a) = kdict.key?(Syms(b))                      # todo */
pub const OP_KEY_P: mrb_insn = 52;
/* R(a) = argument array (16=m5:r1:m5:d1:lv4) */
pub const OP_ARGARY: mrb_insn = 50;
/* R(a) = super(R(a+1),... ,R(a+b+1)) */
pub const OP_SUPER: mrb_insn = 49;
/* R(0) = self.call(frame.argc, frame.argv) */
pub const OP_CALL: mrb_insn = 48;
/* R(a) = call(R(a),Syms(Bx),R(a+1),...,R(a+c),&R(a+c+1)) */
pub const OP_SENDB: mrb_insn = 47;
/* R(a) = call(R(a),Syms(b),R(a+1),...,R(a+c)) */
pub const OP_SEND: mrb_insn = 46;
/* R(a) = call(R(a),Syms(b),*R(a+1),&R(a+2)) */
pub const OP_SENDVB: mrb_insn = 45;
/* R(a) = call(R(a),Syms(b),*R(a+1)) */
pub const OP_SENDV: mrb_insn = 44;
/* A.times{ensure_pop().call} */
pub const OP_EPOP: mrb_insn = 43;
/* ensure_push(SEQ[a]) */
pub const OP_EPUSH: mrb_insn = 42;
/* raise(R(a)) */
pub const OP_RAISE: mrb_insn = 41;
/* a.times{rescue_pop()} */
pub const OP_POPERR: mrb_insn = 40;
/* R(b) = R(a).isa?(R(b)) */
pub const OP_RESCUE: mrb_insn = 39;
/* R(a) = exc */
pub const OP_EXCEPT: mrb_insn = 38;
/* rescue_push(a) */
pub const OP_ONERR: mrb_insn = 37;
/* if R(b)==nil pc=a */
pub const OP_JMPNIL: mrb_insn = 36;
/* if !R(b) pc=a */
pub const OP_JMPNOT: mrb_insn = 35;
/* if R(b) pc=a */
pub const OP_JMPIF: mrb_insn = 34;
/* pc=a */
pub const OP_JMP: mrb_insn = 33;
/* uvset(b,c,R(a)) */
pub const OP_SETUPVAR: mrb_insn = 32;
/* R(a) = uvget(b,c) */
pub const OP_GETUPVAR: mrb_insn = 31;
/* R(a+1)::Syms(b) = R(a) */
pub const OP_SETMCNST: mrb_insn = 30;
/* R(a) = R(a)::Syms(b) */
pub const OP_GETMCNST: mrb_insn = 29;
/* constset(Syms(b),R(a)) */
pub const OP_SETCONST: mrb_insn = 28;
/* R(a) = constget(Syms(b)) */
pub const OP_GETCONST: mrb_insn = 27;
/* cvset(Syms(b),R(a)) */
pub const OP_SETCV: mrb_insn = 26;
/* R(a) = cvget(Syms(b)) */
pub const OP_GETCV: mrb_insn = 25;
/* ivset(Syms(b),R(a)) */
pub const OP_SETIV: mrb_insn = 24;
/* R(a) = ivget(Syms(b)) */
pub const OP_GETIV: mrb_insn = 23;
/* Special[Syms(b)] = R(a) */
pub const OP_SETSV: mrb_insn = 22;
/* R(a) = Special[Syms(b)] */
pub const OP_GETSV: mrb_insn = 21;
/* setglobal(Syms(b), R(a)) */
pub const OP_SETGV: mrb_insn = 20;
/* R(a) = getglobal(Syms(b)) */
pub const OP_GETGV: mrb_insn = 19;
/* R(a) = false */
pub const OP_LOADF: mrb_insn = 18;
/* R(a) = true */
pub const OP_LOADT: mrb_insn = 17;
/* R(a) = self */
pub const OP_LOADSELF: mrb_insn = 16;
/* R(a) = nil */
pub const OP_LOADNIL: mrb_insn = 15;
/* R(a) = Syms(b) */
pub const OP_LOADSYM: mrb_insn = 14;
/* R(a) = mrb_int(7) */
pub const OP_LOADI_7: mrb_insn = 13;
/* R(a) = mrb_int(6) */
pub const OP_LOADI_6: mrb_insn = 12;
/* R(a) = mrb_int(5) */
pub const OP_LOADI_5: mrb_insn = 11;
/* R(a) = mrb_int(4) */
pub const OP_LOADI_4: mrb_insn = 10;
/* R(a) = mrb_int(3) */
pub const OP_LOADI_3: mrb_insn = 9;
/* R(a) = mrb_int(2) */
pub const OP_LOADI_2: mrb_insn = 8;
/* R(a) = mrb_int(1) */
pub const OP_LOADI_1: mrb_insn = 7;
/* R(a) = mrb_int(0) */
pub const OP_LOADI_0: mrb_insn = 6;
/* R(a) = mrb_int(-1) */
pub const OP_LOADI__1: mrb_insn = 5;
/* R(a) = mrb_int(-b) */
pub const OP_LOADINEG: mrb_insn = 4;
/* R(a) = mrb_int(b) */
pub const OP_LOADI: mrb_insn = 3;
/* R(a) = Pool(b) */
pub const OP_LOADL: mrb_insn = 2;
/* R(a) = R(b) */
pub const OP_MOVE: mrb_insn = 1;
/* operand types:
   + Z: no operand (Z,Z,Z,Z)
   + B: 8bit (B,S,B,B)
   + BB: 8+8bit (BB,SB,BS,SS)
   + BBB: 8+8+8bit (BBB,SBB,BSB,SSB)
   + BS: 8+16bit (BS,SS,BS,BS)
   + S: 16bit (S,S,S,S)
   + W: 24bit (W,W,W,W)
*/
/*-----------------------------------------------------------------------
operation code    operands      semantics
------------------------------------------------------------------------*/
/* no operation */
pub const OP_NOP: mrb_insn = 0;
/*
 * Returns a fixnum in Ruby.
 */
#[inline]
unsafe extern "C" fn mrb_fixnum_value(mut i: mrb_int) -> mrb_value {
    let mut v: mrb_value =
        mrb_value{value: unnamed_0{f: 0.,}, tt: MRB_TT_FALSE,};
    v.tt = MRB_TT_FIXNUM;
    v.value.i = i;
    return v;
}
#[inline]
unsafe extern "C" fn mrb_obj_value(mut p: *mut libc::c_void) -> mrb_value {
    let mut v: mrb_value =
        mrb_value{value: unnamed_0{f: 0.,}, tt: MRB_TT_FALSE,};
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
/* aspec access */
#[no_mangle]
pub unsafe extern "C" fn mrb_proc_new(mut mrb: *mut mrb_state,
                                      mut irep: *mut mrb_irep) -> *mut RProc {
    let mut p: *mut RProc = 0 as *mut RProc;
    let mut ci: *mut mrb_callinfo = (*(*mrb).c).ci;
    p = mrb_obj_alloc(mrb, MRB_TT_PROC, (*mrb).proc_class) as *mut RProc;
    if !ci.is_null() {
        let mut tc: *mut RClass = 0 as *mut RClass;
        if !(*ci).proc_0.is_null() {
            tc =
                if (*(*ci).proc_0).flags() as libc::c_int & 1024i32 != 0i32 {
                    (*(*(*ci).proc_0).e.env).c
                } else { (*(*ci).proc_0).e.target_class }
        }
        if tc.is_null() { tc = (*ci).target_class }
        (*p).upper = (*ci).proc_0;
        (*p).e.target_class = tc
    }
    (*p).body.irep = irep;
    mrb_irep_incref(mrb, irep);
    return p;
}
#[no_mangle]
pub unsafe extern "C" fn mrb_closure_new(mut mrb: *mut mrb_state,
                                         mut irep: *mut mrb_irep)
 -> *mut RProc {
    let mut p: *mut RProc = mrb_proc_new(mrb, irep);
    closure_setup(mrb, p);
    return p;
}
unsafe extern "C" fn closure_setup(mut mrb: *mut mrb_state,
                                   mut p: *mut RProc) {
    let mut ci: *mut mrb_callinfo = (*(*mrb).c).ci;
    let mut up: *mut RProc = (*p).upper;
    let mut e: *mut REnv = 0 as *mut REnv;
    if !ci.is_null() && !(*ci).env.is_null() {
        e = (*ci).env
    } else if !up.is_null() {
        let mut tc: *mut RClass =
            if (*p).flags() as libc::c_int & 1024i32 != 0i32 {
                (*(*p).e.env).c
            } else { (*p).e.target_class };
        e = env_new(mrb, (*(*up).body.irep).nlocals as mrb_int);
        (*ci).env = e;
        if !tc.is_null() {
            (*e).c = tc;
            mrb_field_write_barrier(mrb, e as *mut RBasic, tc as *mut RBasic);
        }
    }
    if !e.is_null() {
        (*p).e.env = e;
        (*p).set_flags((*p).flags() | 1024i32 as uint32_t);
        mrb_field_write_barrier(mrb, p as *mut RBasic, e as *mut RBasic);
    };
}
unsafe extern "C" fn env_new(mut mrb: *mut mrb_state, mut nlocals: mrb_int)
 -> *mut REnv {
    let mut e: *mut REnv = 0 as *mut REnv;
    let mut ci: *mut mrb_callinfo = (*(*mrb).c).ci;
    let mut bidx: libc::c_int = 0;
    e = mrb_obj_alloc(mrb, MRB_TT_ENV, 0 as *mut RClass) as *mut REnv;
    (*e).set_flags(((*e).flags() as libc::c_int & !0x3ffi32) as libc::c_uint |
                       nlocals as libc::c_uint & 0x3ffi32 as libc::c_uint);
    bidx = (*ci).argc;
    if (*ci).argc < 0i32 { bidx = 2i32 } else { bidx += 1i32 }
    (*e).set_flags(((*e).flags() as libc::c_int & !(0x3ffi32 << 10i32)) as
                       libc::c_uint |
                       (bidx as libc::c_uint & 0x3ffi32 as libc::c_uint) <<
                           10i32);
    (*e).mid = (*ci).mid;
    (*e).stack = (*(*mrb).c).stack;
    (*e).cxt = (*mrb).c;
    return e;
}
#[no_mangle]
pub unsafe extern "C" fn mrb_proc_new_cfunc(mut mrb: *mut mrb_state,
                                            mut func: mrb_func_t)
 -> *mut RProc {
    let mut p: *mut RProc = 0 as *mut RProc;
    p = mrb_obj_alloc(mrb, MRB_TT_PROC, (*mrb).proc_class) as *mut RProc;
    (*p).body.func = func;
    (*p).set_flags((*p).flags() | 128i32 as uint32_t);
    (*p).upper = 0 as *mut RProc;
    (*p).e.target_class = 0 as *mut RClass;
    return p;
}
#[no_mangle]
pub unsafe extern "C" fn mrb_closure_new_cfunc(mut mrb: *mut mrb_state,
                                               mut func: mrb_func_t,
                                               mut nlocals: libc::c_int)
 -> *mut RProc {
    return mrb_proc_new_cfunc_with_env(mrb, func, nlocals as mrb_int,
                                       0 as *const mrb_value);
}
/* following functions are defined in mruby-proc-ext so please include it when using */
#[no_mangle]
pub unsafe extern "C" fn mrb_proc_new_cfunc_with_env(mut mrb: *mut mrb_state,
                                                     mut func: mrb_func_t,
                                                     mut argc: mrb_int,
                                                     mut argv:
                                                         *const mrb_value)
 -> *mut RProc {
    let mut p: *mut RProc = mrb_proc_new_cfunc(mrb, func);
    let mut e: *mut REnv = 0 as *mut REnv;
    let mut i: libc::c_int = 0;
    e = env_new(mrb, argc);
    (*p).e.env = e;
    (*p).set_flags((*p).flags() | 1024i32 as uint32_t);
    mrb_field_write_barrier(mrb, p as *mut RBasic, e as *mut RBasic);
    (*e).set_flags((*e).flags() | (1i32 << 20i32) as uint32_t);
    (*e).stack =
        mrb_malloc(mrb,
                   (::std::mem::size_of::<mrb_value>() as libc::c_ulong as
                        libc::c_ulonglong).wrapping_mul(argc as
                                                            libc::c_ulonglong)
                       as size_t) as *mut mrb_value;
    if !argv.is_null() {
        i = 0i32;
        while (i as libc::c_longlong) < argc {
            *(*e).stack.offset(i as isize) = *argv.offset(i as isize);
            i += 1
        }
    } else {
        i = 0i32;
        while (i as libc::c_longlong) < argc {
            (*(*e).stack.offset(i as isize)).tt = MRB_TT_FALSE;
            (*(*e).stack.offset(i as isize)).value.i = 0i32 as mrb_int;
            i += 1
        }
    }
    return p;
}
#[no_mangle]
pub unsafe extern "C" fn mrb_proc_copy(mut a: *mut RProc, mut b: *mut RProc) {
    if !(*a).body.irep.is_null() { return }
    (*a).set_flags((*b).flags());
    (*a).body = (*b).body;
    if !((*a).flags() as libc::c_int & 128i32 != 0i32) &&
           !(*a).body.irep.is_null() {
        (*(*a).body.irep).refcnt = (*(*a).body.irep).refcnt.wrapping_add(1)
    }
    (*a).upper = (*b).upper;
    (*a).e.env = (*b).e.env;
}
#[no_mangle]
pub unsafe extern "C" fn mrb_proc_arity(mut p: *const RProc) -> mrb_int {
    let mut irep: *mut mrb_irep = 0 as *mut mrb_irep;
    let mut pc: *mut mrb_code = 0 as *mut mrb_code;
    let mut aspec: mrb_aspec = 0;
    let mut ma: libc::c_int = 0;
    let mut op: libc::c_int = 0;
    let mut ra: libc::c_int = 0;
    let mut pa: libc::c_int = 0;
    let mut arity: libc::c_int = 0;
    if (*p).flags() as libc::c_int & 128i32 != 0i32 {
        return -1i32 as mrb_int
    }
    irep = (*p).body.irep;
    if irep.is_null() { return 0i32 as mrb_int }
    pc = (*irep).iseq;
    if *pc as libc::c_int != OP_ENTER as libc::c_int {
        return 0i32 as mrb_int
    }
    aspec =
        ((*pc.offset(1isize).offset(0isize) as libc::c_int) << 16i32 |
             (*pc.offset(1isize).offset(1isize) as libc::c_int) << 8i32 |
             *pc.offset(1isize).offset(2isize) as libc::c_int) as mrb_aspec;
    ma = (aspec >> 18i32 & 0x1fi32 as libc::c_uint) as libc::c_int;
    op = (aspec >> 13i32 & 0x1fi32 as libc::c_uint) as libc::c_int;
    ra = (aspec >> 12i32 & 0x1i32 as libc::c_uint) as libc::c_int;
    pa = (aspec >> 7i32 & 0x1fi32 as libc::c_uint) as libc::c_int;
    arity =
        if 0 != ra || (*p).flags() as libc::c_int & 256i32 != 0i32 && 0 != op
           {
            -(ma + pa + 1i32)
        } else { ma + pa };
    return arity as mrb_int;
}
#[no_mangle]
pub unsafe extern "C" fn mrb_proc_cfunc_env_get(mut mrb: *mut mrb_state,
                                                mut idx: mrb_int)
 -> mrb_value {
    let mut p: *mut RProc = (*(*(*mrb).c).ci).proc_0;
    let mut e: *mut REnv = 0 as *mut REnv;
    if p.is_null() || !((*p).flags() as libc::c_int & 128i32 != 0i32) {
        mrb_raise(mrb,
                  mrb_exc_get(mrb,
                              b"TypeError\x00" as *const u8 as
                                  *const libc::c_char),
                  b"Can\'t get cfunc env from non-cfunc proc.\x00" as
                      *const u8 as *const libc::c_char);
    }
    e =
        if (*p).flags() as libc::c_int & 1024i32 != 0i32 {
            (*p).e.env
        } else { 0 as *mut REnv };
    if e.is_null() {
        mrb_raise(mrb,
                  mrb_exc_get(mrb,
                              b"TypeError\x00" as *const u8 as
                                  *const libc::c_char),
                  b"Can\'t get cfunc env from cfunc Proc without REnv.\x00" as
                      *const u8 as *const libc::c_char);
    }
    if idx < 0i32 as libc::c_longlong ||
           ((*e).flags() as libc::c_int & 0x3ffi32) as mrb_int <= idx {
        mrb_raisef(mrb,
                   mrb_exc_get(mrb,
                               b"IndexError\x00" as *const u8 as
                                   *const libc::c_char),
                   b"Env index out of range: %S (expected: 0 <= index < %S)\x00"
                       as *const u8 as *const libc::c_char,
                   mrb_fixnum_value(idx),
                   mrb_fixnum_value(((*e).flags() as libc::c_int & 0x3ffi32)
                                        as mrb_int));
    }
    return *(*e).stack.offset(idx as isize);
}
/*
** proc.c - Proc class
**
** See Copyright Notice in mruby.h
*/
static mut call_iseq: [mrb_code; 1] = [OP_CALL as libc::c_int as mrb_code];
/* a->e.target_class = a->e.target_class; */
unsafe extern "C" fn mrb_proc_s_new(mut mrb: *mut mrb_state,
                                    mut proc_class: mrb_value) -> mrb_value {
    let mut blk: mrb_value =
        mrb_value{value: unnamed_0{f: 0.,}, tt: MRB_TT_FALSE,};
    let mut proc_0: mrb_value =
        mrb_value{value: unnamed_0{f: 0.,}, tt: MRB_TT_FALSE,};
    let mut p: *mut RProc = 0 as *mut RProc;
    mrb_get_args(mrb, b"&\x00" as *const u8 as *const libc::c_char,
                 &mut blk as *mut mrb_value);
    if blk.tt as libc::c_uint == MRB_TT_FALSE as libc::c_int as libc::c_uint
           && 0 == blk.value.i {
        mrb_raise(mrb,
                  mrb_exc_get(mrb,
                              b"ArgumentError\x00" as *const u8 as
                                  *const libc::c_char),
                  b"tried to create Proc object without a block\x00" as
                      *const u8 as *const libc::c_char);
    }
    p =
        mrb_obj_alloc(mrb, MRB_TT_PROC, proc_class.value.p as *mut RClass) as
            *mut RProc;
    mrb_proc_copy(p, blk.value.p as *mut RProc);
    proc_0 = mrb_obj_value(p as *mut libc::c_void);
    mrb_funcall_with_block(mrb, proc_0,
                           mrb_intern_static(mrb,
                                             b"initialize\x00" as *const u8 as
                                                 *const libc::c_char,
                                             (::std::mem::size_of::<[libc::c_char; 11]>()
                                                  as
                                                  libc::c_ulong).wrapping_sub(1i32
                                                                                  as
                                                                                  libc::c_ulong)),
                           0i32 as mrb_int, 0 as *const mrb_value, proc_0);
    if !((*p).flags() as libc::c_int & 256i32 != 0i32) &&
           (*(*mrb).c).ci > (*(*mrb).c).cibase &&
           if (*p).flags() as libc::c_int & 1024i32 != 0i32 {
               (*p).e.env
           } else { 0 as *mut REnv } ==
               (*(*(*mrb).c).ci.offset(-1i32 as isize)).env {
        (*p).set_flags((*p).flags() | 512i32 as uint32_t)
    }
    return proc_0;
}
unsafe extern "C" fn mrb_proc_init_copy(mut mrb: *mut mrb_state,
                                        mut self_0: mrb_value) -> mrb_value {
    let mut proc_0: mrb_value =
        mrb_value{value: unnamed_0{f: 0.,}, tt: MRB_TT_FALSE,};
    mrb_get_args(mrb, b"o\x00" as *const u8 as *const libc::c_char,
                 &mut proc_0 as *mut mrb_value);
    if proc_0.tt as libc::c_uint != MRB_TT_PROC as libc::c_int as libc::c_uint
       {
        mrb_raise(mrb,
                  mrb_exc_get(mrb,
                              b"ArgumentError\x00" as *const u8 as
                                  *const libc::c_char),
                  b"not a proc\x00" as *const u8 as *const libc::c_char);
    }
    mrb_proc_copy(self_0.value.p as *mut RProc, proc_0.value.p as *mut RProc);
    return self_0;
}
/* 15.2.17.4.2 */
unsafe extern "C" fn proc_arity(mut mrb: *mut mrb_state,
                                mut self_0: mrb_value) -> mrb_value {
    return mrb_fixnum_value(mrb_proc_arity(self_0.value.p as *mut RProc));
}
/* 15.3.1.2.6  */
/* 15.3.1.3.27 */
/*
 * call-seq:
 *   lambda { |...| block }  -> a_proc
 *
 * Equivalent to <code>Proc.new</code>, except the resulting Proc objects
 * check the number of parameters passed when called.
 */
unsafe extern "C" fn proc_lambda(mut mrb: *mut mrb_state,
                                 mut self_0: mrb_value) -> mrb_value {
    let mut blk: mrb_value =
        mrb_value{value: unnamed_0{f: 0.,}, tt: MRB_TT_FALSE,};
    let mut p: *mut RProc = 0 as *mut RProc;
    mrb_get_args(mrb, b"&\x00" as *const u8 as *const libc::c_char,
                 &mut blk as *mut mrb_value);
    if blk.tt as libc::c_uint == MRB_TT_FALSE as libc::c_int as libc::c_uint
           && 0 == blk.value.i {
        mrb_raise(mrb,
                  mrb_exc_get(mrb,
                              b"ArgumentError\x00" as *const u8 as
                                  *const libc::c_char),
                  b"tried to create Proc object without a block\x00" as
                      *const u8 as *const libc::c_char);
    }
    if blk.tt as libc::c_uint != MRB_TT_PROC as libc::c_int as libc::c_uint {
        mrb_raise(mrb,
                  mrb_exc_get(mrb,
                              b"ArgumentError\x00" as *const u8 as
                                  *const libc::c_char),
                  b"not a proc\x00" as *const u8 as *const libc::c_char);
    }
    p = blk.value.p as *mut RProc;
    if !((*p).flags() as libc::c_int & 256i32 != 0i32) {
        let mut p2: *mut RProc =
            mrb_obj_alloc(mrb, MRB_TT_PROC, (*p).c) as *mut RProc;
        mrb_proc_copy(p2, p);
        (*p2).set_flags((*p2).flags() | 256i32 as uint32_t);
        return mrb_obj_value(p2 as *mut libc::c_void)
    }
    return blk;
}
#[no_mangle]
pub unsafe extern "C" fn mrb_init_proc(mut mrb: *mut mrb_state) {
    let mut p: *mut RProc = 0 as *mut RProc;
    let mut m: mrb_method_t =
        mrb_method_t{func_p: 0, unnamed: unnamed{proc_0: 0 as *mut RProc,},};
    let mut call_irep: *mut mrb_irep =
        mrb_malloc(mrb, ::std::mem::size_of::<mrb_irep>() as libc::c_ulong) as
            *mut mrb_irep;
    static mut mrb_irep_zero: mrb_irep =
        mrb_irep{nlocals: 0i32 as uint16_t,
                 nregs: 0,
                 flags: 0,
                 iseq: 0 as *const mrb_code as *mut mrb_code,
                 pool: 0 as *const mrb_value as *mut mrb_value,
                 syms: 0 as *const mrb_sym as *mut mrb_sym,
                 reps: 0 as *const *mut mrb_irep as *mut *mut mrb_irep,
                 lv: 0 as *const mrb_locals as *mut mrb_locals,
                 debug_info:
                     0 as *const mrb_irep_debug_info as
                         *mut mrb_irep_debug_info,
                 ilen: 0,
                 plen: 0,
                 slen: 0,
                 rlen: 0,
                 refcnt: 0,};
    *call_irep = mrb_irep_zero;
    (*call_irep).flags = 1i32 as uint8_t;
    (*call_irep).iseq = call_iseq.as_mut_ptr();
    (*call_irep).ilen = 1i32 as uint16_t;
    (*call_irep).nregs = 2i32 as uint16_t;
    mrb_define_class_method(mrb, (*mrb).proc_class,
                            b"new\x00" as *const u8 as *const libc::c_char,
                            Some(mrb_proc_s_new),
                            0i32 as mrb_aspec | 1i32 as mrb_aspec);
    mrb_define_method(mrb, (*mrb).proc_class,
                      b"initialize_copy\x00" as *const u8 as
                          *const libc::c_char, Some(mrb_proc_init_copy),
                      ((1i32 & 0x1fi32) as mrb_aspec) << 18i32);
    mrb_define_method(mrb, (*mrb).proc_class,
                      b"arity\x00" as *const u8 as *const libc::c_char,
                      Some(proc_arity), 0i32 as mrb_aspec);
    p = mrb_proc_new(mrb, call_irep);
    m.func_p = 0i32 as mrb_bool;
    m.unnamed.proc_0 = p;
    mrb_define_method_raw(mrb, (*mrb).proc_class,
                          mrb_intern_static(mrb,
                                            b"call\x00" as *const u8 as
                                                *const libc::c_char,
                                            (::std::mem::size_of::<[libc::c_char; 5]>()
                                                 as
                                                 libc::c_ulong).wrapping_sub(1i32
                                                                                 as
                                                                                 libc::c_ulong)),
                          m);
    mrb_define_method_raw(mrb, (*mrb).proc_class,
                          mrb_intern_static(mrb,
                                            b"[]\x00" as *const u8 as
                                                *const libc::c_char,
                                            (::std::mem::size_of::<[libc::c_char; 3]>()
                                                 as
                                                 libc::c_ulong).wrapping_sub(1i32
                                                                                 as
                                                                                 libc::c_ulong)),
                          m);
    mrb_define_class_method(mrb, (*mrb).kernel_module,
                            b"lambda\x00" as *const u8 as *const libc::c_char,
                            Some(proc_lambda),
                            0i32 as mrb_aspec | 1i32 as mrb_aspec);
    mrb_define_method(mrb, (*mrb).kernel_module,
                      b"lambda\x00" as *const u8 as *const libc::c_char,
                      Some(proc_lambda),
                      0i32 as mrb_aspec | 1i32 as mrb_aspec);
}