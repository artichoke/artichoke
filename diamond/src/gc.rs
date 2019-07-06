use libc;
use c2rust_bitfields::{BitfieldStruct};
extern "C" {
    pub type iv_tbl;
    /* debug info */
    pub type mrb_irep_debug_info;
    pub type symbol_name;
    pub type htable;
    pub type mrb_shared_string;
    #[no_mangle]
    fn memmove(_: *mut libc::c_void, _: *const libc::c_void, _: libc::c_ulong)
     -> *mut libc::c_void;
    #[no_mangle]
    fn memset(_: *mut libc::c_void, _: libc::c_int, _: libc::c_ulong)
     -> *mut libc::c_void;
    #[no_mangle]
    fn __assert_rtn(_: *const libc::c_char, _: *const libc::c_char,
                    _: libc::c_int, _: *const libc::c_char) -> !;
    #[no_mangle]
    fn _longjmp(_: *mut libc::c_int, _: libc::c_int) -> !;
    #[no_mangle]
    fn _setjmp(_: *mut libc::c_int) -> libc::c_int;
    #[no_mangle]
    fn mrb_gc_free_iv(_: *mut mrb_state, _: *mut RObject);
    #[no_mangle]
    fn mrb_irep_decref(_: *mut mrb_state, _: *mut mrb_irep);
    #[no_mangle]
    fn mrb_irep_cutref(_: *mut mrb_state, _: *mut mrb_irep);
    #[no_mangle]
    fn mrb_gc_free_str(_: *mut mrb_state, _: *mut RString);
    #[no_mangle]
    fn mrb_gc_free_hash(_: *mut mrb_state, _: *mut RHash);
    #[no_mangle]
    fn mrb_ary_decref(_: *mut mrb_state, _: *mut mrb_shared_array);
    #[no_mangle]
    fn mrb_free_context(mrb: *mut mrb_state, c: *mut mrb_context);
    /* flags (21bits): 1(shared flag):10(cioff/bidx):10(stack_len) */
    #[no_mangle]
    fn mrb_env_unshare(_: *mut mrb_state, _: *mut REnv);
    #[no_mangle]
    fn mrb_gc_free_mt(_: *mut mrb_state, _: *mut RClass);
    #[no_mangle]
    fn mrb_gc_mark_range(mrb: *mut mrb_state, r: *mut RRange);
    /* RHASH_TBL allocates st_table if not available. */
    /* GC functions */
    #[no_mangle]
    fn mrb_gc_mark_hash(_: *mut mrb_state, _: *mut RHash);
    #[no_mangle]
    fn mrb_gc_mark_iv(_: *mut mrb_state, _: *mut RObject);
    #[no_mangle]
    fn mrb_gc_mark_mt(_: *mut mrb_state, _: *mut RClass);
    /* GC functions */
    #[no_mangle]
    fn mrb_gc_mark_gv(_: *mut mrb_state);
    #[no_mangle]
    fn mrb_gc_mark_hash_size(_: *mut mrb_state, _: *mut RHash) -> size_t;
    #[no_mangle]
    fn mrb_gc_mark_iv_size(_: *mut mrb_state, _: *mut RObject) -> size_t;
    #[no_mangle]
    fn mrb_gc_mark_mt_size(_: *mut mrb_state, _: *mut RClass) -> size_t;
    /* *
 * Defines a new module.
 *
 * @param [mrb_state *] mrb_state* The current mruby state.
 * @param [const char *] char* The name of the module.
 * @return [struct RClass *] Reference to the newly defined module.
 */
    #[no_mangle]
    fn mrb_define_module(_: *mut mrb_state, _: *const libc::c_char)
     -> *mut RClass;
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
    #[no_mangle]
    fn mrb_intern_static(_: *mut mrb_state, _: *const libc::c_char, _: size_t)
     -> mrb_sym;
    #[no_mangle]
    fn mrb_exc_raise(mrb: *mut mrb_state, exc: mrb_value) -> !;
    #[no_mangle]
    fn mrb_raisef(mrb: *mut mrb_state, c: *mut RClass,
                  fmt: *const libc::c_char, _: ...) -> !;
    #[no_mangle]
    fn mrb_raise(mrb: *mut mrb_state, c: *mut RClass,
                 msg: *const libc::c_char) -> !;
    #[no_mangle]
    fn mrb_str_new(mrb: *mut mrb_state, p: *const libc::c_char, len: size_t)
     -> mrb_value;
    #[no_mangle]
    fn mrb_obj_eq(_: *mut mrb_state, _: mrb_value, _: mrb_value) -> mrb_bool;
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
    /* *
 * Set a global variable
 *
 * Example:
 *
 *     !!!ruby
 *     # Ruby style
 *     $value = "foo"
 *
 *     !!!c
 *     // C style
 *     mrb_sym sym = mrb_intern_lit(mrb, "$value");
 *     mrb_gv_set(mrb, sym, mrb_str_new_lit("foo"));
 *
 * @param mrb The mruby state reference
 * @param sym The name of the global variable
 * @param val The value of the global variable
 */
    #[no_mangle]
    fn mrb_gv_set(mrb: *mut mrb_state, sym: mrb_sym, val: mrb_value);
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
    /* *
 * Get a global variable. Will return nil if the var does not exist
 *
 * Example:
 *
 *     !!!ruby
 *     # Ruby style
 *     var = $value
 *
 *     !!!c
 *     // C style
 *     mrb_sym sym = mrb_intern_lit(mrb, "$value");
 *     mrb_value var = mrb_gv_get(mrb, sym);
 *
 * @param mrb The mruby state reference
 * @param sym The name of the global variable
 * @return The value of that global variable. May be nil
 */
    #[no_mangle]
    fn mrb_gv_get(mrb: *mut mrb_state, sym: mrb_sym) -> mrb_value;
    #[no_mangle]
    fn mrb_ary_modify(_: *mut mrb_state, _: *mut RArray);
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
/*
** mruby/throw.h - mruby exception throwing handler
**
** See Copyright Notice in mruby.h
*/
#[derive ( Copy , Clone )]
#[repr(C)]
pub struct mrb_jmpbuf {
    pub impl_0: jmp_buf,
}
pub type jmp_buf = [libc::c_int; 37];
/*
** mruby/gc.h - garbage collector for mruby
**
** See Copyright Notice in mruby.h
*/
/* *
 * Uncommon memory management stuffs.
 */
pub type mrb_each_object_callback
    =
    unsafe extern "C" fn(_: *mut mrb_state, _: *mut RBasic,
                         _: *mut libc::c_void) -> libc::c_int;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub union unnamed_3 {
    pub free: free_obj,
    pub basic: RBasic,
    pub object: RObject,
    pub klass: RClass,
    pub string: RString,
    pub array: RArray,
    pub hash: RHash,
    pub range: RRange,
    pub data: RData,
    pub istruct: RIStruct,
    pub proc_0: RProc,
    pub env: REnv,
    pub fiber: RFiber,
    pub exc: RException,
    pub brk: RBreak,
}
#[derive ( BitfieldStruct , Clone , Copy )]
#[repr(C)]
pub struct RBreak {
    #[bitfield(name = "tt", ty = "mrb_vtype", bits = "0..=7")]
    #[bitfield(name = "color", ty = "uint32_t", bits = "8..=10")]
    #[bitfield(name = "flags", ty = "uint32_t", bits = "11..=31")]
    pub tt_color_flags: [u8; 4],
    pub _pad: [u8; 4],
    pub c: *mut RClass,
    pub gcnext: *mut RBasic,
    pub proc_0: *mut RProc,
    pub val: mrb_value,
}
/*
** mruby/error.h - Exception class
**
** See Copyright Notice in mruby.h
*/
/* *
 * MRuby error handling.
 */
#[derive ( BitfieldStruct , Clone , Copy )]
#[repr(C)]
pub struct RException {
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
** mruby/istruct.h - Inline structures
**
** See Copyright Notice in mruby.h
*/
/* *
 * Inline structures that fit in RVALUE
 *
 * They cannot have finalizer, and cannot have instance variables.
 */
#[derive ( BitfieldStruct , Clone , Copy )]
#[repr(C)]
pub struct RIStruct {
    #[bitfield(name = "tt", ty = "mrb_vtype", bits = "0..=7")]
    #[bitfield(name = "color", ty = "uint32_t", bits = "8..=10")]
    #[bitfield(name = "flags", ty = "uint32_t", bits = "11..=31")]
    pub tt_color_flags: [u8; 4],
    pub _pad: [u8; 4],
    pub c: *mut RClass,
    pub gcnext: *mut RBasic,
    pub inline_data: [libc::c_char; 24],
}
#[derive ( BitfieldStruct , Clone , Copy )]
#[repr(C)]
pub struct RData {
    #[bitfield(name = "tt", ty = "mrb_vtype", bits = "0..=7")]
    #[bitfield(name = "color", ty = "uint32_t", bits = "8..=10")]
    #[bitfield(name = "flags", ty = "uint32_t", bits = "11..=31")]
    pub tt_color_flags: [u8; 4],
    pub _pad: [u8; 4],
    pub c: *mut RClass,
    pub gcnext: *mut RBasic,
    pub iv: *mut iv_tbl,
    pub type_0: *const mrb_data_type,
    pub data: *mut libc::c_void,
}
/*
** mruby/data.h - Data class
**
** See Copyright Notice in mruby.h
*/
/* *
 * Custom C wrapped data.
 *
 * Defining Ruby wrappers around native objects.
 */
/* *
 * Custom data type description.
 */
#[derive ( Copy , Clone )]
#[repr(C)]
pub struct mrb_data_type {
    pub struct_name: *const libc::c_char,
    pub dfree: Option<unsafe extern "C" fn(_: *mut mrb_state,
                                           _: *mut libc::c_void) -> ()>,
}
#[derive ( BitfieldStruct , Clone , Copy )]
#[repr(C)]
pub struct RRange {
    #[bitfield(name = "tt", ty = "mrb_vtype", bits = "0..=7")]
    #[bitfield(name = "color", ty = "uint32_t", bits = "8..=10")]
    #[bitfield(name = "flags", ty = "uint32_t", bits = "11..=31")]
    pub tt_color_flags: [u8; 4],
    pub _pad: [u8; 4],
    pub c: *mut RClass,
    pub gcnext: *mut RBasic,
    pub edges: *mut mrb_range_edges,
    pub excl: mrb_bool,
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
    pub as_0: unnamed_4,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub union unnamed_4 {
    pub heap: unnamed_5,
}
#[derive ( Copy , Clone )]
#[repr(C)]
pub struct unnamed_5 {
    pub len: mrb_int,
    pub aux: unnamed_6,
    pub ptr: *mut mrb_value,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub union unnamed_6 {
    pub capa: mrb_int,
    pub shared: *mut mrb_shared_array,
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
pub struct RString {
    #[bitfield(name = "tt", ty = "mrb_vtype", bits = "0..=7")]
    #[bitfield(name = "color", ty = "uint32_t", bits = "8..=10")]
    #[bitfield(name = "flags", ty = "uint32_t", bits = "11..=31")]
    pub tt_color_flags: [u8; 4],
    pub _pad: [u8; 4],
    pub c: *mut RClass,
    pub gcnext: *mut RBasic,
    pub as_0: unnamed_7,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub union unnamed_7 {
    pub heap: unnamed_8,
    pub ary: [libc::c_char; 24],
}
#[derive ( Copy , Clone )]
#[repr(C)]
pub struct unnamed_8 {
    pub len: mrb_int,
    pub aux: unnamed_9,
    pub ptr: *mut libc::c_char,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub union unnamed_9 {
    pub capa: mrb_int,
    pub shared: *mut mrb_shared_string,
    pub fshared: *mut RString,
}
/*
** gc.c - garbage collector for mruby
**
** See Copyright Notice in mruby.h
*/
/*
  = Tri-color Incremental Garbage Collection

  mruby's GC is Tri-color Incremental GC with Mark & Sweep.
  Algorithm details are omitted.
  Instead, the implementation part is described below.

  == Object's Color

  Each object can be painted in three colors:

    * White - Unmarked.
    * Gray - Marked, But the child objects are unmarked.
    * Black - Marked, the child objects are also marked.

  == Two White Types

  There're two white color types in a flip-flop fashion: White-A and White-B,
  which respectively represent the Current White color (the newly allocated
  objects in the current GC cycle) and the Sweep Target White color (the
  dead objects to be swept).

  A and B will be switched just at the beginning of the next GC cycle. At
  that time, all the dead objects have been swept, while the newly created
  objects in the current GC cycle which finally remains White are now
  regarded as dead objects. Instead of traversing all the White-A objects and
  painting them as White-B, just switch the meaning of White-A and White-B as
  this will be much cheaper.

  As a result, the objects we sweep in the current GC cycle are always
  left from the previous GC cycle. This allows us to sweep objects
  incrementally, without the disturbance of the newly created objects.

  == Execution Timing

  GC Execution Time and Each step interval are decided by live objects count.
  List of Adjustment API:

    * gc_interval_ratio_set
    * gc_step_ratio_set

  For details, see the comments for each function.

  == Write Barrier

  mruby implementer and C extension library writer must insert a write
  barrier when updating a reference from a field of an object.
  When updating a reference from a field of object A to object B,
  two different types of write barrier are available:

    * mrb_field_write_barrier - target B object for a mark.
    * mrb_write_barrier       - target A object for a mark.

  == Generational Mode

  mruby's GC offers an Generational Mode while re-using the tri-color GC
  infrastructure. It will treat the Black objects as Old objects after each
  sweep phase, instead of painting them White. The key ideas are still the same
  as traditional generational GC:

    * Minor GC - just traverse the Young objects (Gray objects) in the mark
                 phase, then only sweep the newly created objects, and leave
                 the Old objects live.

    * Major GC - same as a full regular GC cycle.

  The difference from "traditional" generational GC is, that the major GC
  in mruby is triggered incrementally in a tri-color manner.


  For details, see the comments for each function.

*/
#[derive ( BitfieldStruct , Clone , Copy )]
#[repr(C)]
pub struct free_obj {
    #[bitfield(name = "tt", ty = "mrb_vtype", bits = "0..=7")]
    #[bitfield(name = "color", ty = "uint32_t", bits = "8..=10")]
    #[bitfield(name = "flags", ty = "uint32_t", bits = "11..=31")]
    pub tt_color_flags: [u8; 4],
    pub _pad: [u8; 4],
    pub c: *mut RClass,
    pub gcnext: *mut RBasic,
    pub next: *mut RBasic,
}
#[derive ( Copy , Clone )]
#[repr(C)]
pub struct RVALUE {
    pub as_0: unnamed_3,
}
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
/*
 * Get a nil mrb_value object.
 *
 * @return
 *      nil mrb_value object reference.
 */
#[inline]
unsafe extern "C" fn mrb_nil_value() -> mrb_value {
    let mut v: mrb_value =
        mrb_value{value: unnamed_0{f: 0.,}, tt: MRB_TT_FALSE,};
    v.tt = MRB_TT_FALSE;
    v.value.i = 0i32 as mrb_int;
    return v;
}
#[inline]
unsafe extern "C" fn mrb_bool_value(mut boolean: mrb_bool) -> mrb_value {
    let mut v: mrb_value =
        mrb_value{value: unnamed_0{f: 0.,}, tt: MRB_TT_FALSE,};
    v.tt =
        (if 0 != boolean as libc::c_int {
             MRB_TT_TRUE as libc::c_int
         } else { MRB_TT_FALSE as libc::c_int }) as mrb_vtype;
    v.value.i = 1i32 as mrb_int;
    return v;
}
#[no_mangle]
pub unsafe extern "C" fn mrb_objspace_each_objects(mut mrb: *mut mrb_state,
                                                   mut callback:
                                                       Option<unsafe extern "C" fn(_:
                                                                                       *mut mrb_state,
                                                                                   _:
                                                                                       *mut RBasic,
                                                                                   _:
                                                                                       *mut libc::c_void)
                                                                  ->
                                                                      libc::c_int>,
                                                   mut data:
                                                       *mut libc::c_void) {
    let mut iterating: mrb_bool = (*mrb).gc.iterating();
    mrb_full_gc(mrb);
    (*mrb).gc.set_iterating(1i32 as mrb_bool);
    if 0 != iterating {
        gc_each_objects(mrb, &mut (*mrb).gc, callback, data);
    } else {
        let mut prev_jmp: *mut mrb_jmpbuf = (*mrb).jmp;
        let mut c_jmp: mrb_jmpbuf = mrb_jmpbuf{impl_0: [0; 37],};
        if _setjmp(c_jmp.impl_0.as_mut_ptr()) == 0i32 {
            (*mrb).jmp = &mut c_jmp;
            gc_each_objects(mrb, &mut (*mrb).gc, callback, data);
            (*mrb).jmp = prev_jmp;
            (*mrb).gc.set_iterating(iterating)
        } else {
            (*mrb).gc.set_iterating(iterating);
            (*mrb).jmp = prev_jmp;
            _longjmp((*prev_jmp).impl_0.as_mut_ptr(), 1i32);
        }
    };
}
unsafe extern "C" fn gc_each_objects(mut mrb: *mut mrb_state,
                                     mut gc: *mut mrb_gc,
                                     mut callback:
                                         Option<unsafe extern "C" fn(_:
                                                                         *mut mrb_state,
                                                                     _:
                                                                         *mut RBasic,
                                                                     _:
                                                                         *mut libc::c_void)
                                                    -> libc::c_int>,
                                     mut data: *mut libc::c_void) {
    let mut page: *mut mrb_heap_page = 0 as *mut mrb_heap_page;
    page = (*gc).heaps;
    while !page.is_null() {
        let mut p: *mut RVALUE = 0 as *mut RVALUE;
        let mut i: libc::c_int = 0;
        p = (*page).objects.as_mut_ptr() as *mut RVALUE;
        i = 0i32;
        while i < 1024i32 {
            if callback.expect("non-null function pointer")(mrb,
                                                            &mut (*p.offset(i
                                                                                as
                                                                                isize)).as_0.basic,
                                                            data) == 1i32 {
                return
            }
            i += 1
        }
        page = (*page).next
    };
}
#[no_mangle]
pub unsafe extern "C" fn mrb_full_gc(mut mrb: *mut mrb_state) {
    let mut gc: *mut mrb_gc = &mut (*mrb).gc;
    if 0 != (*gc).disabled() as libc::c_int ||
           0 != (*gc).iterating() as libc::c_int {
        return
    }
    if 0 != (*gc).generational() {
        clear_all_old(mrb, gc);
        (*gc).set_full(1i32 as mrb_bool)
    } else if (*gc).state as libc::c_uint !=
                  MRB_GC_STATE_ROOT as libc::c_int as libc::c_uint {
        incremental_gc_until(mrb, gc, MRB_GC_STATE_ROOT);
    }
    incremental_gc_until(mrb, gc, MRB_GC_STATE_ROOT);
    (*gc).threshold =
        (*gc).live_after_mark.wrapping_div(100i32 as
                                               libc::c_ulong).wrapping_mul((*gc).interval_ratio
                                                                               as
                                                                               libc::c_ulong);
    if 0 != (*gc).generational() {
        (*gc).majorgc_old_threshold =
            (*gc).live_after_mark.wrapping_div(100i32 as
                                                   libc::c_ulong).wrapping_mul(120i32
                                                                                   as
                                                                                   libc::c_ulong);
        (*gc).set_full(0i32 as mrb_bool)
    };
}
unsafe extern "C" fn incremental_gc_until(mut mrb: *mut mrb_state,
                                          mut gc: *mut mrb_gc,
                                          mut to_state: mrb_gc_state) {
    loop  {
        incremental_gc(mrb, gc, 18446744073709551615u64);
        if !((*gc).state as libc::c_uint != to_state as libc::c_uint) {
            break ;
        }
    };
}
unsafe extern "C" fn incremental_gc(mut mrb: *mut mrb_state,
                                    mut gc: *mut mrb_gc, mut limit: size_t)
 -> size_t {
    match (*gc).state as libc::c_uint {
        0 => {
            root_scan_phase(mrb, gc);
            (*gc).state = MRB_GC_STATE_MARK;
            (*gc).current_white_part =
                (*gc).current_white_part ^ (1i32 | 1i32 << 1i32);
            return 0i32 as size_t
        }
        1 => {
            if !(*gc).gray_list.is_null() {
                return incremental_marking_phase(mrb, gc, limit)
            } else {
                final_marking_phase(mrb, gc);
                prepare_incremental_sweep(mrb, gc);
                return 0i32 as size_t
            }
        }
        2 => {
            let mut tried_sweep: size_t = 0i32 as size_t;
            tried_sweep = incremental_sweep_phase(mrb, gc, limit);
            if tried_sweep == 0i32 as libc::c_ulong {
                (*gc).state = MRB_GC_STATE_ROOT
            }
            return tried_sweep
        }
        _ => {
            if 0 != (0 == 0i32) as libc::c_int as libc::c_long {
                __assert_rtn((*::std::mem::transmute::<&[u8; 15],
                                                       &[libc::c_char; 15]>(b"incremental_gc\x00")).as_ptr(),
                             b"src/gc.c\x00" as *const u8 as
                                 *const libc::c_char, 1191i32,
                             b"0\x00" as *const u8 as *const libc::c_char);
            } else { };
            return 0i32 as size_t
        }
    };
}
unsafe extern "C" fn incremental_sweep_phase(mut mrb: *mut mrb_state,
                                             mut gc: *mut mrb_gc,
                                             mut limit: size_t) -> size_t {
    let mut page: *mut mrb_heap_page = (*gc).sweeps;
    let mut tried_sweep: size_t = 0i32 as size_t;
    while !page.is_null() && tried_sweep < limit {
        let mut p: *mut RVALUE = (*page).objects.as_mut_ptr() as *mut RVALUE;
        let mut e: *mut RVALUE = p.offset(1024isize);
        let mut freed: size_t = 0i32 as size_t;
        let mut dead_slot: mrb_bool = 1i32 as mrb_bool;
        let mut full: mrb_bool =
            ((*page).freelist == 0 as *mut libc::c_void as *mut RBasic) as
                libc::c_int as mrb_bool;
        if 0 != (*gc).generational() as libc::c_int && 0 == (*gc).full() &&
               0 != (*page).old() as libc::c_int {
            p = e;
            dead_slot = 0i32 as mrb_bool
        }
        while p < e {
            if 0 !=
                   (*p).as_0.basic.color() as libc::c_int &
                       ((*gc).current_white_part ^ (1i32 | 1i32 << 1i32)) &
                       (1i32 | 1i32 << 1i32) ||
                   (*p).as_0.basic.tt() as libc::c_int ==
                       MRB_TT_FREE as libc::c_int {
                if (*p).as_0.basic.tt() as libc::c_int !=
                       MRB_TT_FREE as libc::c_int {
                    obj_free(mrb, &mut (*p).as_0.basic, 0i32);
                    if (*p).as_0.basic.tt() as libc::c_int ==
                           MRB_TT_FREE as libc::c_int {
                        (*p).as_0.free.next = (*page).freelist;
                        (*page).freelist = p as *mut RBasic;
                        freed = freed.wrapping_add(1)
                    } else { dead_slot = 0i32 as mrb_bool }
                }
            } else {
                if 0 == (*gc).generational() {
                    (*p).as_0.basic.set_color((*gc).current_white_part as
                                                  uint32_t)
                }
                dead_slot = 0i32 as mrb_bool
            }
            p = p.offset(1isize)
        }
        if 0 != dead_slot as libc::c_int && freed < 1024i32 as libc::c_ulong {
            let mut next: *mut mrb_heap_page = (*page).next;
            unlink_heap_page(gc, page);
            unlink_free_heap_page(gc, page);
            mrb_free(mrb, page as *mut libc::c_void);
            page = next
        } else {
            if 0 != full as libc::c_int && freed > 0i32 as libc::c_ulong {
                link_free_heap_page(gc, page);
            }
            if (*page).freelist.is_null() &&
                   (0 != (*gc).generational() as libc::c_int &&
                        0 == (*gc).full()) {
                (*page).set_old(1i32 as mrb_bool)
            } else { (*page).set_old(0i32 as mrb_bool) }
            page = (*page).next
        }
        tried_sweep =
            (tried_sweep as
                 libc::c_ulong).wrapping_add(1024i32 as libc::c_ulong) as
                size_t as size_t;
        (*gc).live =
            ((*gc).live as libc::c_ulong).wrapping_sub(freed) as size_t as
                size_t;
        (*gc).live_after_mark =
            ((*gc).live_after_mark as libc::c_ulong).wrapping_sub(freed) as
                size_t as size_t
    }
    (*gc).sweeps = page;
    return tried_sweep;
}
unsafe extern "C" fn link_free_heap_page(mut gc: *mut mrb_gc,
                                         mut page: *mut mrb_heap_page) {
    (*page).free_next = (*gc).free_heaps;
    if !(*gc).free_heaps.is_null() { (*(*gc).free_heaps).free_prev = page }
    (*gc).free_heaps = page;
}
#[no_mangle]
pub unsafe extern "C" fn mrb_free(mut mrb: *mut mrb_state,
                                  mut p: *mut libc::c_void) {
    (*mrb).allocf.expect("non-null function pointer")(mrb, p, 0i32 as size_t,
                                                      (*mrb).allocf_ud);
}
unsafe extern "C" fn unlink_free_heap_page(mut gc: *mut mrb_gc,
                                           mut page: *mut mrb_heap_page) {
    if !(*page).free_prev.is_null() {
        (*(*page).free_prev).free_next = (*page).free_next
    }
    if !(*page).free_next.is_null() {
        (*(*page).free_next).free_prev = (*page).free_prev
    }
    if (*gc).free_heaps == page { (*gc).free_heaps = (*page).free_next }
    (*page).free_prev = 0 as *mut mrb_heap_page;
    (*page).free_next = 0 as *mut mrb_heap_page;
}
unsafe extern "C" fn unlink_heap_page(mut gc: *mut mrb_gc,
                                      mut page: *mut mrb_heap_page) {
    if !(*page).prev.is_null() { (*(*page).prev).next = (*page).next }
    if !(*page).next.is_null() { (*(*page).next).prev = (*page).prev }
    if (*gc).heaps == page { (*gc).heaps = (*page).next }
    (*page).prev = 0 as *mut mrb_heap_page;
    (*page).next = 0 as *mut mrb_heap_page;
}
unsafe extern "C" fn obj_free(mut mrb: *mut mrb_state, mut obj: *mut RBasic,
                              mut end: libc::c_int) {
    match (*obj).tt() as libc::c_int {
        2 | 3 | 4 => { return }
        6 => { return }
        8 => { mrb_gc_free_iv(mrb, obj as *mut RObject); }
        18 => { mrb_gc_free_iv(mrb, obj as *mut RObject); }
        9 | 10 | 12 => {
            mrb_gc_free_mt(mrb, obj as *mut RClass);
            mrb_gc_free_iv(mrb, obj as *mut RObject);
        }
        11 => {
            if 0 != (*obj).flags() as libc::c_int & 1i32 << 18i32 {
                mrb_gc_free_mt(mrb, obj as *mut RClass);
            }
        }
        20 => {
            let mut e: *mut REnv = obj as *mut REnv;
            if (*e).flags() as libc::c_int & 1i32 << 20i32 == 0i32 {
                (*e).stack = 0 as *mut mrb_value
            } else {
                mrb_free(mrb, (*e).stack as *mut libc::c_void);
                (*e).stack = 0 as *mut mrb_value
            }
        }
        22 => {
            let mut c: *mut mrb_context = (*(obj as *mut RFiber)).cxt;
            if !c.is_null() && c != (*mrb).root_c {
                if 0 == end &&
                       (*c).status as libc::c_uint !=
                           MRB_FIBER_TERMINATED as libc::c_int as libc::c_uint
                   {
                    let mut ci: *mut mrb_callinfo = (*c).ci;
                    let mut ce: *mut mrb_callinfo = (*c).cibase;
                    while ce <= ci {
                        let mut e_0: *mut REnv = (*ci).env;
                        if !e_0.is_null() &&
                               0 == mrb_object_dead_p(mrb, e_0 as *mut RBasic)
                               &&
                               (*e_0).tt() as libc::c_int ==
                                   MRB_TT_ENV as libc::c_int &&
                               (*e_0).flags() as libc::c_int & 1i32 << 20i32
                                   == 0i32 {
                            mrb_env_unshare(mrb, e_0);
                        }
                        ci = ci.offset(-1isize)
                    }
                }
                mrb_free_context(mrb, c);
            }
        }
        14 => {
            if 0 != (*obj).flags() as libc::c_int & 256i32 {
                mrb_ary_decref(mrb,
                               (*(obj as *mut RArray)).as_0.heap.aux.shared);
            } else if 0 == (*obj).flags() as libc::c_int & 7i32 {
                mrb_free(mrb,
                         (*(obj as *mut RArray)).as_0.heap.ptr as
                             *mut libc::c_void);
            }
        }
        15 => {
            mrb_gc_free_iv(mrb, obj as *mut RObject);
            mrb_gc_free_hash(mrb, obj as *mut RHash);
        }
        16 => { mrb_gc_free_str(mrb, obj as *mut RString); }
        13 => {
            let mut p: *mut RProc = obj as *mut RProc;
            if !((*p).flags() as libc::c_int & 128i32 != 0i32) &&
                   !(*p).body.irep.is_null() {
                let mut irep: *mut mrb_irep = (*p).body.irep;
                if 0 != end { mrb_irep_cutref(mrb, irep); }
                mrb_irep_decref(mrb, irep);
            }
        }
        17 => {
            mrb_free(mrb, (*(obj as *mut RRange)).edges as *mut libc::c_void);
        }
        21 => {
            let mut d: *mut RData = obj as *mut RData;
            if !(*d).type_0.is_null() && (*(*d).type_0).dfree.is_some() {
                (*(*d).type_0).dfree.expect("non-null function pointer")(mrb,
                                                                         (*d).data);
            }
            mrb_gc_free_iv(mrb, obj as *mut RObject);
        }
        _ => { }
    }
    (*obj).set_tt(MRB_TT_FREE);
}
#[no_mangle]
pub unsafe extern "C" fn mrb_object_dead_p(mut mrb: *mut mrb_state,
                                           mut object: *mut RBasic)
 -> mrb_bool {
    let mut gc: *mut mrb_gc = &mut (*mrb).gc;
    if 0 == heap_p(gc, object) { return 1i32 as mrb_bool }
    return (0 !=
                (*object).color() as libc::c_int &
                    ((*gc).current_white_part ^ (1i32 | 1i32 << 1i32)) &
                    (1i32 | 1i32 << 1i32) ||
                (*object).tt() as libc::c_int == MRB_TT_FREE as libc::c_int)
               as libc::c_int as mrb_bool;
}
unsafe extern "C" fn heap_p(mut gc: *mut mrb_gc, mut object: *mut RBasic)
 -> mrb_bool {
    let mut page: *mut mrb_heap_page = 0 as *mut mrb_heap_page;
    page = (*gc).heaps;
    while !page.is_null() {
        let mut p: *mut RVALUE = 0 as *mut RVALUE;
        p = (*page).objects.as_mut_ptr() as *mut RVALUE;
        if &mut (*p.offset(0isize)).as_0.basic as *mut RBasic <= object &&
               object <= &mut (*p.offset(1024isize)).as_0.basic as *mut RBasic
           {
            return 1i32 as mrb_bool
        }
        page = (*page).next
    }
    return 0i32 as mrb_bool;
}
unsafe extern "C" fn prepare_incremental_sweep(mut mrb: *mut mrb_state,
                                               mut gc: *mut mrb_gc) {
    (*gc).state = MRB_GC_STATE_SWEEP;
    (*gc).sweeps = (*gc).heaps;
    (*gc).live_after_mark = (*gc).live;
}
unsafe extern "C" fn final_marking_phase(mut mrb: *mut mrb_state,
                                         mut gc: *mut mrb_gc) {
    let mut i: libc::c_int = 0;
    let mut e: libc::c_int = 0;
    i = 0i32;
    e = (*gc).arena_idx;
    while i < e { mrb_gc_mark(mrb, *(*gc).arena.offset(i as isize)); i += 1 }
    mrb_gc_mark_gv(mrb);
    mark_context(mrb, (*mrb).c);
    mark_context(mrb, (*mrb).root_c);
    mrb_gc_mark(mrb, (*mrb).exc as *mut RBasic);
    gc_mark_gray_list(mrb, gc);
    if 0 != !(*gc).gray_list.is_null() as libc::c_int as libc::c_long {
        __assert_rtn((*::std::mem::transmute::<&[u8; 20],
                                               &[libc::c_char; 20]>(b"final_marking_phase\x00")).as_ptr(),
                     b"src/gc.c\x00" as *const u8 as *const libc::c_char,
                     1082i32,
                     b"gc->gray_list == ((void *)0)\x00" as *const u8 as
                         *const libc::c_char);
    } else { };
    (*gc).gray_list = (*gc).atomic_gray_list;
    (*gc).atomic_gray_list = 0 as *mut RBasic;
    gc_mark_gray_list(mrb, gc);
    if 0 != !(*gc).gray_list.is_null() as libc::c_int as libc::c_long {
        __assert_rtn((*::std::mem::transmute::<&[u8; 20],
                                               &[libc::c_char; 20]>(b"final_marking_phase\x00")).as_ptr(),
                     b"src/gc.c\x00" as *const u8 as *const libc::c_char,
                     1086i32,
                     b"gc->gray_list == ((void *)0)\x00" as *const u8 as
                         *const libc::c_char);
    } else { };
}
unsafe extern "C" fn gc_mark_gray_list(mut mrb: *mut mrb_state,
                                       mut gc: *mut mrb_gc) {
    while !(*gc).gray_list.is_null() {
        if (*(*gc).gray_list).color() as libc::c_int == 0i32 {
            gc_mark_children(mrb, gc, (*gc).gray_list);
        } else { (*gc).gray_list = (*(*gc).gray_list).gcnext }
    };
}
unsafe extern "C" fn gc_mark_children(mut mrb: *mut mrb_state,
                                      mut gc: *mut mrb_gc,
                                      mut obj: *mut RBasic) {
    if 0 !=
           !((*obj).color() as libc::c_int == 0i32) as libc::c_int as
               libc::c_long {
        __assert_rtn((*::std::mem::transmute::<&[u8; 17],
                                               &[libc::c_char; 17]>(b"gc_mark_children\x00")).as_ptr(),
                     b"src/gc.c\x00" as *const u8 as *const libc::c_char,
                     671i32,
                     b"((obj)->color == 0)\x00" as *const u8 as
                         *const libc::c_char);
    } else { };
    (*obj).set_color((1i32 << 2i32) as uint32_t);
    (*gc).gray_list = (*obj).gcnext;
    mrb_gc_mark(mrb, (*obj).c as *mut RBasic);
    let mut current_block_43: u64;
    match (*obj).tt() as libc::c_int {
        11 => {
            let mut c: *mut RClass = obj as *mut RClass;
            if 0 != (*c).flags() as libc::c_int & 1i32 << 18i32 {
                mrb_gc_mark_mt(mrb, c);
            }
            mrb_gc_mark(mrb, (*(obj as *mut RClass)).super_0 as *mut RBasic);
            current_block_43 = 13325891313334703151;
        }
        9 | 10 | 12 => {
            let mut c_0: *mut RClass = obj as *mut RClass;
            mrb_gc_mark_mt(mrb, c_0);
            mrb_gc_mark(mrb, (*c_0).super_0 as *mut RBasic);
            /* fall through */
            current_block_43 = 11327052197689759932;
        }
        8 | 21 | 18 => { current_block_43 = 11327052197689759932; }
        13 => {
            let mut p: *mut RProc = obj as *mut RProc;
            mrb_gc_mark(mrb, (*p).upper as *mut RBasic);
            mrb_gc_mark(mrb, (*p).e.env as *mut RBasic);
            current_block_43 = 13325891313334703151;
        }
        20 => {
            let mut e: *mut REnv = obj as *mut REnv;
            let mut i: mrb_int = 0;
            let mut len: mrb_int = 0;
            if (*e).flags() as libc::c_int & 1i32 << 20i32 == 0i32 &&
                   !(*e).cxt.is_null() && !(*(*e).cxt).fib.is_null() {
                mrb_gc_mark(mrb, (*(*e).cxt).fib as *mut RBasic);
            }
            len = ((*e).flags() as libc::c_int & 0x3ffi32) as mrb_int;
            i = 0i32 as mrb_int;
            while i < len {
                if !(((*(*e).stack.offset(i as isize)).tt as libc::c_uint) <
                         MRB_TT_OBJECT as libc::c_int as libc::c_uint) {
                    mrb_gc_mark(mrb,
                                (*(*e).stack.offset(i as isize)).value.p as
                                    *mut RBasic);
                }
                i += 1
            }
            current_block_43 = 13325891313334703151;
        }
        22 => {
            let mut c_1: *mut mrb_context = (*(obj as *mut RFiber)).cxt;
            if !c_1.is_null() { mark_context(mrb, c_1); }
            current_block_43 = 13325891313334703151;
        }
        14 => {
            let mut a: *mut RArray = obj as *mut RArray;
            let mut i_0: size_t = 0;
            let mut e_0: size_t = 0;
            i_0 = 0i32 as size_t;
            e_0 =
                (if 0 != (*a).flags() as libc::c_int & 7i32 {
                     (((*a).flags() as libc::c_int & 7i32) - 1i32) as mrb_int
                 } else { (*a).as_0.heap.len }) as size_t;
            while i_0 < e_0 {
                if !(((*if 0 != (*a).flags() as libc::c_int & 7i32 {
                            &mut (*a).as_0 as *mut unnamed_4 as *mut mrb_value
                        } else { (*a).as_0.heap.ptr }.offset(i_0 as isize)).tt
                          as libc::c_uint) <
                         MRB_TT_OBJECT as libc::c_int as libc::c_uint) {
                    mrb_gc_mark(mrb,
                                (*if 0 != (*a).flags() as libc::c_int & 7i32 {
                                      &mut (*a).as_0 as *mut unnamed_4 as
                                          *mut mrb_value
                                  } else {
                                      (*a).as_0.heap.ptr
                                  }.offset(i_0 as isize)).value.p as
                                    *mut RBasic);
                }
                i_0 = i_0.wrapping_add(1)
            }
            current_block_43 = 13325891313334703151;
        }
        15 => {
            mrb_gc_mark_iv(mrb, obj as *mut RObject);
            mrb_gc_mark_hash(mrb, obj as *mut RHash);
            current_block_43 = 13325891313334703151;
        }
        16 => {
            if 0 != (*obj).flags() as libc::c_int & 2i32 &&
                   0 == (*obj).flags() as libc::c_int & 4i32 {
                let mut s: *mut RString = obj as *mut RString;
                mrb_gc_mark(mrb, (*s).as_0.heap.aux.fshared as *mut RBasic);
            }
            current_block_43 = 13325891313334703151;
        }
        17 => {
            mrb_gc_mark_range(mrb, obj as *mut RRange);
            current_block_43 = 13325891313334703151;
        }
        _ => { current_block_43 = 13325891313334703151; }
    }
    match current_block_43 {
        11327052197689759932 => { mrb_gc_mark_iv(mrb, obj as *mut RObject); }
        _ => { }
    };
}
#[no_mangle]
pub unsafe extern "C" fn mrb_gc_mark(mut mrb: *mut mrb_state,
                                     mut obj: *mut RBasic) {
    if obj.is_null() { return }
    if 0 == (*obj).color() as libc::c_int & (1i32 | 1i32 << 1i32) { return }
    if 0 !=
           !((*obj).tt() as libc::c_int != MRB_TT_FREE as libc::c_int) as
               libc::c_int as libc::c_long {
        __assert_rtn((*::std::mem::transmute::<&[u8; 12],
                                               &[libc::c_char; 12]>(b"mrb_gc_mark\x00")).as_ptr(),
                     b"src/gc.c\x00" as *const u8 as *const libc::c_char,
                     771i32,
                     b"(obj)->tt != MRB_TT_FREE\x00" as *const u8 as
                         *const libc::c_char);
    } else { };
    add_gray_list(mrb, &mut (*mrb).gc, obj);
}
#[inline]
unsafe extern "C" fn add_gray_list(mut mrb: *mut mrb_state,
                                   mut gc: *mut mrb_gc,
                                   mut obj: *mut RBasic) {
    (*obj).set_color(0i32 as uint32_t);
    (*obj).gcnext = (*gc).gray_list;
    (*gc).gray_list = obj;
}
unsafe extern "C" fn mark_context(mut mrb: *mut mrb_state,
                                  mut c: *mut mrb_context) {
    let mut i: libc::c_int = 0;
    let mut ci: *mut mrb_callinfo = 0 as *mut mrb_callinfo;
    loop  {
        if (*c).status as libc::c_uint ==
               MRB_FIBER_TERMINATED as libc::c_int as libc::c_uint {
            return
        }
        mark_context_stack(mrb, c);
        if !(*c).cibase.is_null() {
            ci = (*c).cibase;
            while ci <= (*c).ci {
                mrb_gc_mark(mrb, (*ci).env as *mut RBasic);
                mrb_gc_mark(mrb, (*ci).proc_0 as *mut RBasic);
                mrb_gc_mark(mrb, (*ci).target_class as *mut RBasic);
                ci = ci.offset(1isize)
            }
        }
        i = 0i32;
        while i < (*c).eidx as libc::c_int {
            mrb_gc_mark(mrb, *(*c).ensure.offset(i as isize) as *mut RBasic);
            i += 1
        }
        mrb_gc_mark(mrb, (*c).fib as *mut RBasic);
        if (*c).prev.is_null() { break ; }
        c = (*c).prev
    };
}
unsafe extern "C" fn mark_context_stack(mut mrb: *mut mrb_state,
                                        mut c: *mut mrb_context) {
    let mut i: size_t = 0;
    let mut e: size_t = 0;
    let mut nil: mrb_value =
        mrb_value{value: unnamed_0{f: 0.,}, tt: MRB_TT_FALSE,};
    if (*c).stack.is_null() { return }
    e =
        (*c).stack.wrapping_offset_from((*c).stbase) as libc::c_long as
            size_t;
    if !(*c).ci.is_null() {
        e =
            (e as
                 libc::c_ulong).wrapping_add(ci_nregs((*c).ci) as
                                                 libc::c_ulong) as size_t as
                size_t
    }
    if (*c).stbase.offset(e as isize) > (*c).stend {
        e =
            (*c).stend.wrapping_offset_from((*c).stbase) as libc::c_long as
                size_t
    }
    i = 0i32 as size_t;
    while i < e {
        let mut v: mrb_value = *(*c).stbase.offset(i as isize);
        if !((v.tt as libc::c_uint) <
                 MRB_TT_OBJECT as libc::c_int as libc::c_uint) {
            mrb_gc_mark(mrb, v.value.p as *mut RBasic);
        }
        i = i.wrapping_add(1)
    }
    e =
        (*c).stend.wrapping_offset_from((*c).stbase) as libc::c_long as
            size_t;
    nil = mrb_nil_value();
    while i < e {
        *(*c).stbase.offset(i as isize) = nil;
        i = i.wrapping_add(1)
    };
}
unsafe extern "C" fn ci_nregs(mut ci: *mut mrb_callinfo) -> libc::c_int {
    let mut p: *mut RProc = (*ci).proc_0;
    let mut n: libc::c_int = 0i32;
    if p.is_null() {
        if (*ci).argc < 0i32 { return 3i32 }
        return (*ci).argc + 2i32
    }
    if !((*p).flags() as libc::c_int & 128i32 != 0i32) &&
           !(*p).body.irep.is_null() {
        n = (*(*p).body.irep).nregs as libc::c_int
    }
    if (*ci).argc < 0i32 { if n < 3i32 { n = 3i32 } }
    if (*ci).argc > n { n = (*ci).argc + 2i32 }
    return n;
}
unsafe extern "C" fn incremental_marking_phase(mut mrb: *mut mrb_state,
                                               mut gc: *mut mrb_gc,
                                               mut limit: size_t) -> size_t {
    let mut tried_marks: size_t = 0i32 as size_t;
    while !(*gc).gray_list.is_null() && tried_marks < limit {
        tried_marks =
            (tried_marks as
                 libc::c_ulong).wrapping_add(gc_gray_mark(mrb, gc,
                                                          (*gc).gray_list)) as
                size_t as size_t
    }
    return tried_marks;
}
unsafe extern "C" fn gc_gray_mark(mut mrb: *mut mrb_state,
                                  mut gc: *mut mrb_gc, mut obj: *mut RBasic)
 -> size_t {
    let mut children: size_t = 0i32 as size_t;
    gc_mark_children(mrb, gc, obj);
    match (*obj).tt() as libc::c_int {
        11 => { children = children.wrapping_add(1) }
        9 | 12 | 10 => {
            let mut c: *mut RClass = obj as *mut RClass;
            children =
                (children as
                     libc::c_ulong).wrapping_add(mrb_gc_mark_iv_size(mrb,
                                                                     obj as
                                                                         *mut RObject))
                    as size_t as size_t;
            children =
                (children as
                     libc::c_ulong).wrapping_add(mrb_gc_mark_mt_size(mrb, c))
                    as size_t as size_t;
            children = children.wrapping_add(1)
        }
        8 | 21 | 18 => {
            children =
                (children as
                     libc::c_ulong).wrapping_add(mrb_gc_mark_iv_size(mrb,
                                                                     obj as
                                                                         *mut RObject))
                    as size_t as size_t
        }
        20 => {
            children =
                (children as
                     libc::c_ulonglong).wrapping_add(((*obj).flags() as
                                                          libc::c_int &
                                                          0x3ffi32) as mrb_int
                                                         as libc::c_ulonglong)
                    as size_t as size_t
        }
        22 => {
            let mut c_0: *mut mrb_context = (*(obj as *mut RFiber)).cxt;
            let mut i: size_t = 0;
            let mut ci: *mut mrb_callinfo = 0 as *mut mrb_callinfo;
            if !(c_0.is_null() ||
                     (*c_0).status as libc::c_uint ==
                         MRB_FIBER_TERMINATED as libc::c_int as libc::c_uint)
               {
                i =
                    (*c_0).stack.wrapping_offset_from((*c_0).stbase) as
                        libc::c_long as size_t;
                if !(*c_0).ci.is_null() {
                    i =
                        (i as
                             libc::c_ulong).wrapping_add(ci_nregs((*c_0).ci)
                                                             as libc::c_ulong)
                            as size_t as size_t
                }
                if (*c_0).stbase.offset(i as isize) > (*c_0).stend {
                    i =
                        (*c_0).stend.wrapping_offset_from((*c_0).stbase) as
                            libc::c_long as size_t
                }
                children =
                    (children as libc::c_ulong).wrapping_add(i) as size_t as
                        size_t;
                children =
                    (children as
                         libc::c_ulong).wrapping_add((*c_0).eidx as
                                                         libc::c_ulong) as
                        size_t as size_t;
                if !(*c_0).cibase.is_null() {
                    i = 0i32 as size_t;
                    ci = (*c_0).cibase;
                    while ci <= (*c_0).ci {
                        i = i.wrapping_add(1);
                        ci = ci.offset(1isize)
                    }
                }
                children =
                    (children as libc::c_ulong).wrapping_add(i) as size_t as
                        size_t
            }
        }
        14 => {
            let mut a: *mut RArray = obj as *mut RArray;
            children =
                (children as
                     libc::c_ulonglong).wrapping_add((if 0 !=
                                                             (*a).flags() as
                                                                 libc::c_int &
                                                                 7i32 {
                                                          (((*a).flags() as
                                                                libc::c_int &
                                                                7i32) - 1i32)
                                                              as mrb_int
                                                      } else {
                                                          (*a).as_0.heap.len
                                                      }) as libc::c_ulonglong)
                    as size_t as size_t
        }
        15 => {
            children =
                (children as
                     libc::c_ulong).wrapping_add(mrb_gc_mark_iv_size(mrb,
                                                                     obj as
                                                                         *mut RObject))
                    as size_t as size_t;
            children =
                (children as
                     libc::c_ulong).wrapping_add(mrb_gc_mark_hash_size(mrb,
                                                                       obj as
                                                                           *mut RHash))
                    as size_t as size_t
        }
        13 | 17 => {
            children =
                (children as
                     libc::c_ulong).wrapping_add(2i32 as libc::c_ulong) as
                    size_t as size_t
        }
        _ => { }
    }
    return children;
}
unsafe extern "C" fn root_scan_phase(mut mrb: *mut mrb_state,
                                     mut gc: *mut mrb_gc) {
    let mut i: libc::c_int = 0;
    let mut e: libc::c_int = 0;
    if !(0 != (*gc).generational() as libc::c_int && 0 == (*gc).full()) {
        (*gc).gray_list = 0 as *mut RBasic;
        (*gc).atomic_gray_list = 0 as *mut RBasic
    }
    mrb_gc_mark_gv(mrb);
    i = 0i32;
    e = (*gc).arena_idx;
    while i < e { mrb_gc_mark(mrb, *(*gc).arena.offset(i as isize)); i += 1 }
    mrb_gc_mark(mrb, (*mrb).object_class as *mut RBasic);
    mrb_gc_mark(mrb, (*mrb).class_class as *mut RBasic);
    mrb_gc_mark(mrb, (*mrb).module_class as *mut RBasic);
    mrb_gc_mark(mrb, (*mrb).proc_class as *mut RBasic);
    mrb_gc_mark(mrb, (*mrb).string_class as *mut RBasic);
    mrb_gc_mark(mrb, (*mrb).array_class as *mut RBasic);
    mrb_gc_mark(mrb, (*mrb).hash_class as *mut RBasic);
    mrb_gc_mark(mrb, (*mrb).range_class as *mut RBasic);
    mrb_gc_mark(mrb, (*mrb).float_class as *mut RBasic);
    mrb_gc_mark(mrb, (*mrb).fixnum_class as *mut RBasic);
    mrb_gc_mark(mrb, (*mrb).true_class as *mut RBasic);
    mrb_gc_mark(mrb, (*mrb).false_class as *mut RBasic);
    mrb_gc_mark(mrb, (*mrb).nil_class as *mut RBasic);
    mrb_gc_mark(mrb, (*mrb).symbol_class as *mut RBasic);
    mrb_gc_mark(mrb, (*mrb).kernel_module as *mut RBasic);
    mrb_gc_mark(mrb, (*mrb).eException_class as *mut RBasic);
    mrb_gc_mark(mrb, (*mrb).eStandardError_class as *mut RBasic);
    mrb_gc_mark(mrb, (*mrb).top_self as *mut RBasic);
    mrb_gc_mark(mrb, (*mrb).exc as *mut RBasic);
    mrb_gc_mark(mrb, (*mrb).nomem_err as *mut RBasic);
    mrb_gc_mark(mrb, (*mrb).stack_err as *mut RBasic);
    mark_context(mrb, (*mrb).c);
    if (*mrb).root_c != (*mrb).c { mark_context(mrb, (*mrb).root_c); };
}
unsafe extern "C" fn clear_all_old(mut mrb: *mut mrb_state,
                                   mut gc: *mut mrb_gc) {
    let mut origin_mode: mrb_bool = (*gc).generational();
    if 0 != (0 == (*gc).generational()) as libc::c_int as libc::c_long {
        __assert_rtn((*::std::mem::transmute::<&[u8; 14],
                                               &[libc::c_char; 14]>(b"clear_all_old\x00")).as_ptr(),
                     b"src/gc.c\x00" as *const u8 as *const libc::c_char,
                     1223i32,
                     b"((gc)->generational)\x00" as *const u8 as
                         *const libc::c_char);
    } else { };
    if 0 != (*gc).generational() as libc::c_int &&
           0 != (*gc).full() as libc::c_int {
        incremental_gc_until(mrb, gc, MRB_GC_STATE_ROOT);
    }
    (*gc).set_generational(0i32 as mrb_bool);
    prepare_incremental_sweep(mrb, gc);
    incremental_gc_until(mrb, gc, MRB_GC_STATE_ROOT);
    (*gc).set_generational(origin_mode);
    (*gc).gray_list = 0 as *mut RBasic;
    (*gc).atomic_gray_list = (*gc).gray_list;
}
/* raise RuntimeError if no mem */
#[no_mangle]
pub unsafe extern "C" fn mrb_malloc(mut mrb: *mut mrb_state, mut len: size_t)
 -> *mut libc::c_void {
    return mrb_realloc(mrb, 0 as *mut libc::c_void, len);
}
/* ditto */
#[no_mangle]
pub unsafe extern "C" fn mrb_realloc(mut mrb: *mut mrb_state,
                                     mut p: *mut libc::c_void,
                                     mut len: size_t) -> *mut libc::c_void {
    let mut p2: *mut libc::c_void = 0 as *mut libc::c_void;
    p2 = mrb_realloc_simple(mrb, p, len);
    if len == 0i32 as libc::c_ulong { return p2 }
    if p2.is_null() {
        if 0 != (*mrb).gc.out_of_memory() {
            mrb_exc_raise(mrb,
                          mrb_obj_value((*mrb).nomem_err as
                                            *mut libc::c_void));
        } else {
            (*mrb).gc.set_out_of_memory(1i32 as mrb_bool);
            mrb_exc_raise(mrb,
                          mrb_obj_value((*mrb).nomem_err as
                                            *mut libc::c_void));
        }
    } else { (*mrb).gc.set_out_of_memory(0i32 as mrb_bool) }
    return p2;
}
/* return NULL if no memory available */
#[no_mangle]
pub unsafe extern "C" fn mrb_realloc_simple(mut mrb: *mut mrb_state,
                                            mut p: *mut libc::c_void,
                                            mut len: size_t)
 -> *mut libc::c_void {
    let mut p2: *mut libc::c_void = 0 as *mut libc::c_void;
    p2 =
        (*mrb).allocf.expect("non-null function pointer")(mrb, p, len,
                                                          (*mrb).allocf_ud);
    if p2.is_null() && len > 0i32 as libc::c_ulong &&
           !(*mrb).gc.heaps.is_null() {
        mrb_full_gc(mrb);
        p2 =
            (*mrb).allocf.expect("non-null function pointer")(mrb, p, len,
                                                              (*mrb).allocf_ud)
    }
    return p2;
}
/* ditto */
#[no_mangle]
pub unsafe extern "C" fn mrb_calloc(mut mrb: *mut mrb_state,
                                    mut nelem: size_t, mut len: size_t)
 -> *mut libc::c_void {
    let mut p: *mut libc::c_void = 0 as *mut libc::c_void;
    if nelem > 0i32 as libc::c_ulong && len > 0i32 as libc::c_ulong &&
           nelem <= 18446744073709551615u64.wrapping_div(len) {
        let mut size: size_t = 0;
        size = nelem.wrapping_mul(len);
        p = mrb_malloc(mrb, size);
        memset(p, 0i32, size);
    } else { p = 0 as *mut libc::c_void }
    return p;
}
/* return NULL if no memory available */
#[no_mangle]
pub unsafe extern "C" fn mrb_malloc_simple(mut mrb: *mut mrb_state,
                                           mut len: size_t)
 -> *mut libc::c_void {
    return mrb_realloc_simple(mrb, 0 as *mut libc::c_void, len);
}
// Initialized in run_static_initializers
static mut RVALUE_zero: RVALUE =
    RVALUE{as_0:
               unnamed_3{free:
                             free_obj{tt_color_flags: [0; 4],
                                      _pad: [0; 4],
                                      c: 0 as *const RClass as *mut RClass,
                                      gcnext:
                                          0 as *const RBasic as *mut RBasic,
                                      next:
                                          0 as *const RBasic as
                                              *mut RBasic,},},};
#[no_mangle]
pub unsafe extern "C" fn mrb_obj_alloc(mut mrb: *mut mrb_state,
                                       mut ttype: mrb_vtype,
                                       mut cls: *mut RClass) -> *mut RBasic {
    let mut p: *mut RBasic = 0 as *mut RBasic;
    let mut gc: *mut mrb_gc = &mut (*mrb).gc;
    if !cls.is_null() {
        let mut tt: mrb_vtype = MRB_TT_FALSE;
        match (*cls).tt() as libc::c_int {
            9 | 12 | 10 | 20 => { }
            _ => {
                mrb_raise(mrb,
                          mrb_exc_get(mrb,
                                      b"TypeError\x00" as *const u8 as
                                          *const libc::c_char),
                          b"allocation failure\x00" as *const u8 as
                              *const libc::c_char);
            }
        }
        tt = ((*cls).flags() as libc::c_int & 0xffi32) as mrb_vtype;
        if tt as libc::c_uint != MRB_TT_FALSE as libc::c_int as libc::c_uint
               &&
               ttype as libc::c_uint !=
                   MRB_TT_SCLASS as libc::c_int as libc::c_uint &&
               ttype as libc::c_uint !=
                   MRB_TT_ICLASS as libc::c_int as libc::c_uint &&
               ttype as libc::c_uint !=
                   MRB_TT_ENV as libc::c_int as libc::c_uint &&
               ttype as libc::c_uint != tt as libc::c_uint {
            mrb_raisef(mrb,
                       mrb_exc_get(mrb,
                                   b"TypeError\x00" as *const u8 as
                                       *const libc::c_char),
                       b"allocation failure of %S\x00" as *const u8 as
                           *const libc::c_char,
                       mrb_obj_value(cls as *mut libc::c_void));
        }
    }
    if (*gc).threshold < (*gc).live { mrb_incremental_gc(mrb); }
    if (*gc).free_heaps.is_null() { add_heap(mrb, gc); }
    p = (*(*gc).free_heaps).freelist;
    (*(*gc).free_heaps).freelist = (*(p as *mut free_obj)).next;
    if (*(*gc).free_heaps).freelist.is_null() {
        unlink_free_heap_page(gc, (*gc).free_heaps);
    }
    (*gc).live = (*gc).live.wrapping_add(1);
    gc_protect(mrb, gc, p);
    *(p as *mut RVALUE) = RVALUE_zero;
    (*p).set_tt(ttype);
    (*p).c = cls;
    (*p).set_color((*gc).current_white_part as uint32_t);
    return p;
}
unsafe extern "C" fn gc_protect(mut mrb: *mut mrb_state, mut gc: *mut mrb_gc,
                                mut p: *mut RBasic) {
    if (*gc).arena_idx >= (*gc).arena_capa {
        (*gc).arena_capa = ((*gc).arena_capa * 3i32 / 2i32) as libc::c_int;
        (*gc).arena =
            mrb_realloc(mrb, (*gc).arena as *mut libc::c_void,
                        (::std::mem::size_of::<*mut RBasic>() as
                             libc::c_ulong).wrapping_mul((*gc).arena_capa as
                                                             libc::c_ulong))
                as *mut *mut RBasic
    }
    let fresh0 = (*gc).arena_idx;
    (*gc).arena_idx = (*gc).arena_idx + 1;
    let ref mut fresh1 = *(*gc).arena.offset(fresh0 as isize);
    *fresh1 = p;
}
unsafe extern "C" fn add_heap(mut mrb: *mut mrb_state, mut gc: *mut mrb_gc) {
    let mut page: *mut mrb_heap_page =
        mrb_calloc(mrb, 1i32 as size_t,
                   (::std::mem::size_of::<mrb_heap_page>() as
                        libc::c_ulong).wrapping_add((1024i32 as
                                                         libc::c_ulong).wrapping_mul(::std::mem::size_of::<RVALUE>()
                                                                                         as
                                                                                         libc::c_ulong)))
            as *mut mrb_heap_page;
    let mut p: *mut RVALUE = 0 as *mut RVALUE;
    let mut e: *mut RVALUE = 0 as *mut RVALUE;
    let mut prev: *mut RBasic = 0 as *mut RBasic;
    p = (*page).objects.as_mut_ptr() as *mut RVALUE;
    e = p.offset(1024isize);
    while p < e {
        (*p).as_0.free.set_tt(MRB_TT_FREE);
        (*p).as_0.free.next = prev;
        prev = &mut (*p).as_0.basic;
        p = p.offset(1isize)
    }
    (*page).freelist = prev;
    link_heap_page(gc, page);
    link_free_heap_page(gc, page);
}
unsafe extern "C" fn link_heap_page(mut gc: *mut mrb_gc,
                                    mut page: *mut mrb_heap_page) {
    (*page).next = (*gc).heaps;
    if !(*gc).heaps.is_null() { (*(*gc).heaps).prev = page }
    (*gc).heaps = page;
}
#[no_mangle]
pub unsafe extern "C" fn mrb_incremental_gc(mut mrb: *mut mrb_state) {
    let mut gc: *mut mrb_gc = &mut (*mrb).gc;
    if 0 != (*gc).disabled() as libc::c_int ||
           0 != (*gc).iterating() as libc::c_int {
        return
    }
    if 0 != (*gc).generational() as libc::c_int && 0 == (*gc).full() {
        incremental_gc_until(mrb, gc, MRB_GC_STATE_ROOT);
    } else { incremental_gc_step(mrb, gc); }
    if (*gc).state as libc::c_uint ==
           MRB_GC_STATE_ROOT as libc::c_int as libc::c_uint {
        if 0 !=
               !((*gc).live >= (*gc).live_after_mark) as libc::c_int as
                   libc::c_long {
            __assert_rtn((*::std::mem::transmute::<&[u8; 19],
                                                   &[libc::c_char; 19]>(b"mrb_incremental_gc\x00")).as_ptr(),
                         b"src/gc.c\x00" as *const u8 as *const libc::c_char,
                         1258i32,
                         b"gc->live >= gc->live_after_mark\x00" as *const u8
                             as *const libc::c_char);
        } else { };
        (*gc).threshold =
            (*gc).live_after_mark.wrapping_div(100i32 as
                                                   libc::c_ulong).wrapping_mul((*gc).interval_ratio
                                                                                   as
                                                                                   libc::c_ulong);
        if (*gc).threshold < 1024i32 as libc::c_ulong {
            (*gc).threshold = 1024i32 as size_t
        }
        if 0 != (*gc).generational() as libc::c_int &&
               0 != (*gc).full() as libc::c_int {
            let mut threshold: size_t =
                (*gc).live_after_mark.wrapping_div(100i32 as
                                                       libc::c_ulong).wrapping_mul(120i32
                                                                                       as
                                                                                       libc::c_ulong);
            (*gc).set_full(0i32 as mrb_bool);
            if threshold < 10000i32 as libc::c_ulong {
                (*gc).majorgc_old_threshold = threshold
            } else { mrb_full_gc(mrb); }
        } else if 0 != (*gc).generational() as libc::c_int &&
                      0 == (*gc).full() {
            if (*gc).live > (*gc).majorgc_old_threshold {
                clear_all_old(mrb, gc);
                (*gc).set_full(1i32 as mrb_bool)
            }
        }
    };
}
unsafe extern "C" fn incremental_gc_step(mut mrb: *mut mrb_state,
                                         mut gc: *mut mrb_gc) {
    let mut limit: size_t = 0i32 as size_t;
    let mut result: size_t = 0i32 as size_t;
    limit = (1024i32 / 100i32 * (*gc).step_ratio) as size_t;
    while result < limit {
        result =
            (result as
                 libc::c_ulong).wrapping_add(incremental_gc(mrb, gc, limit))
                as size_t as size_t;
        if (*gc).state as libc::c_uint ==
               MRB_GC_STATE_ROOT as libc::c_int as libc::c_uint {
            break ;
        }
    }
    (*gc).threshold = (*gc).live.wrapping_add(1024i32 as libc::c_ulong);
}
#[no_mangle]
pub unsafe extern "C" fn mrb_garbage_collect(mut mrb: *mut mrb_state) {
    mrb_full_gc(mrb);
}
#[no_mangle]
pub unsafe extern "C" fn mrb_field_write_barrier(mut mrb: *mut mrb_state,
                                                 mut obj: *mut RBasic,
                                                 mut value: *mut RBasic) {
    let mut gc: *mut mrb_gc = &mut (*mrb).gc;
    if 0 == (*obj).color() as libc::c_int & 1i32 << 2i32 { return }
    if 0 == (*value).color() as libc::c_int & (1i32 | 1i32 << 1i32) { return }
    if 0 !=
           !((*gc).state as libc::c_uint ==
                 MRB_GC_STATE_MARK as libc::c_int as libc::c_uint ||
                 !(0 !=
                       (*value).color() as libc::c_int &
                           ((*gc).current_white_part ^ (1i32 | 1i32 << 1i32))
                           & (1i32 | 1i32 << 1i32) ||
                       (*value).tt() as libc::c_int ==
                           MRB_TT_FREE as libc::c_int) &&
                     !(0 !=
                           (*obj).color() as libc::c_int &
                               ((*gc).current_white_part ^
                                    (1i32 | 1i32 << 1i32)) &
                               (1i32 | 1i32 << 1i32) ||
                           (*obj).tt() as libc::c_int ==
                               MRB_TT_FREE as libc::c_int)) as libc::c_int as
               libc::c_long {
        __assert_rtn((*::std::mem::transmute::<&[u8; 24],
                                               &[libc::c_char; 24]>(b"mrb_field_write_barrier\x00")).as_ptr(),
                     b"src/gc.c\x00" as *const u8 as *const libc::c_char,
                     1339i32,
                     b"gc->state == MRB_GC_STATE_MARK || (!(((value)->color & ((gc)->current_white_part ^ (1 | (1 << 1))) & (1 | (1 << 1))) || (value)->tt == MRB_TT_FREE) && !(((obj)->color & ((gc)->current_white_part ^ (1 | (1 << 1))) & (1 | (1 << 1))) || (obj)->tt == MRB_TT_FREE))\x00"
                         as *const u8 as *const libc::c_char);
    } else { };
    if 0 !=
           !(0 != (*gc).generational() as libc::c_int ||
                 (*gc).state as libc::c_uint !=
                     MRB_GC_STATE_ROOT as libc::c_int as libc::c_uint) as
               libc::c_int as libc::c_long {
        __assert_rtn((*::std::mem::transmute::<&[u8; 24],
                                               &[libc::c_char; 24]>(b"mrb_field_write_barrier\x00")).as_ptr(),
                     b"src/gc.c\x00" as *const u8 as *const libc::c_char,
                     1340i32,
                     b"((gc)->generational) || gc->state != MRB_GC_STATE_ROOT\x00"
                         as *const u8 as *const libc::c_char);
    } else { };
    if 0 != (*gc).generational() as libc::c_int ||
           (*gc).state as libc::c_uint ==
               MRB_GC_STATE_MARK as libc::c_int as libc::c_uint {
        add_gray_list(mrb, gc, value);
    } else {
        if 0 !=
               !((*gc).state as libc::c_uint ==
                     MRB_GC_STATE_SWEEP as libc::c_int as libc::c_uint) as
                   libc::c_int as libc::c_long {
            __assert_rtn((*::std::mem::transmute::<&[u8; 24],
                                                   &[libc::c_char; 24]>(b"mrb_field_write_barrier\x00")).as_ptr(),
                         b"src/gc.c\x00" as *const u8 as *const libc::c_char,
                         1346i32,
                         b"gc->state == MRB_GC_STATE_SWEEP\x00" as *const u8
                             as *const libc::c_char);
        } else { };
        (*obj).set_color((*gc).current_white_part as uint32_t)
    };
}
#[no_mangle]
pub unsafe extern "C" fn mrb_write_barrier(mut mrb: *mut mrb_state,
                                           mut obj: *mut RBasic) {
    let mut gc: *mut mrb_gc = &mut (*mrb).gc;
    if 0 == (*obj).color() as libc::c_int & 1i32 << 2i32 { return }
    if 0 !=
           (0 !=
                (*obj).color() as libc::c_int &
                    ((*gc).current_white_part ^ (1i32 | 1i32 << 1i32)) &
                    (1i32 | 1i32 << 1i32) ||
                (*obj).tt() as libc::c_int == MRB_TT_FREE as libc::c_int) as
               libc::c_int as libc::c_long {
        __assert_rtn((*::std::mem::transmute::<&[u8; 18],
                                               &[libc::c_char; 18]>(b"mrb_write_barrier\x00")).as_ptr(),
                     b"src/gc.c\x00" as *const u8 as *const libc::c_char,
                     1367i32,
                     b"!(((obj)->color & ((gc)->current_white_part ^ (1 | (1 << 1))) & (1 | (1 << 1))) || (obj)->tt == MRB_TT_FREE)\x00"
                         as *const u8 as *const libc::c_char);
    } else { };
    if 0 !=
           !(0 != (*gc).generational() as libc::c_int ||
                 (*gc).state as libc::c_uint !=
                     MRB_GC_STATE_ROOT as libc::c_int as libc::c_uint) as
               libc::c_int as libc::c_long {
        __assert_rtn((*::std::mem::transmute::<&[u8; 18],
                                               &[libc::c_char; 18]>(b"mrb_write_barrier\x00")).as_ptr(),
                     b"src/gc.c\x00" as *const u8 as *const libc::c_char,
                     1368i32,
                     b"((gc)->generational) || gc->state != MRB_GC_STATE_ROOT\x00"
                         as *const u8 as *const libc::c_char);
    } else { };
    (*obj).set_color(0i32 as uint32_t);
    (*obj).gcnext = (*gc).atomic_gray_list;
    (*gc).atomic_gray_list = obj;
}
/* mrb_gc_protect() leaves the object in the arena */
#[no_mangle]
pub unsafe extern "C" fn mrb_gc_protect(mut mrb: *mut mrb_state,
                                        mut obj: mrb_value) {
    if (obj.tt as libc::c_uint) < MRB_TT_OBJECT as libc::c_int as libc::c_uint
       {
        return
    }
    gc_protect(mrb, &mut (*mrb).gc, obj.value.p as *mut RBasic);
}
/* mrb_gc_register() keeps the object from GC. */
#[no_mangle]
pub unsafe extern "C" fn mrb_gc_register(mut mrb: *mut mrb_state,
                                         mut obj: mrb_value) {
    let mut root: mrb_sym = 0;
    let mut table: mrb_value =
        mrb_value{value: unnamed_0{f: 0.,}, tt: MRB_TT_FALSE,};
    if (obj.tt as libc::c_uint) < MRB_TT_OBJECT as libc::c_int as libc::c_uint
       {
        return
    }
    root =
        mrb_intern_static(mrb,
                          b"_gc_root_\x00" as *const u8 as
                              *const libc::c_char,
                          (::std::mem::size_of::<[libc::c_char; 10]>() as
                               libc::c_ulong).wrapping_sub(1i32 as
                                                               libc::c_ulong));
    table = mrb_gv_get(mrb, root);
    if table.tt as libc::c_uint == MRB_TT_FALSE as libc::c_int as libc::c_uint
           && 0 == table.value.i ||
           table.tt as libc::c_uint !=
               MRB_TT_ARRAY as libc::c_int as libc::c_uint {
        table = mrb_ary_new(mrb);
        mrb_gv_set(mrb, root, table);
    }
    mrb_ary_push(mrb, table, obj);
}
/* mrb_gc_unregister() removes the object from GC root. */
#[no_mangle]
pub unsafe extern "C" fn mrb_gc_unregister(mut mrb: *mut mrb_state,
                                           mut obj: mrb_value) {
    let mut root: mrb_sym = 0;
    let mut table: mrb_value =
        mrb_value{value: unnamed_0{f: 0.,}, tt: MRB_TT_FALSE,};
    let mut a: *mut RArray = 0 as *mut RArray;
    let mut i: mrb_int = 0;
    if (obj.tt as libc::c_uint) < MRB_TT_OBJECT as libc::c_int as libc::c_uint
       {
        return
    }
    root =
        mrb_intern_static(mrb,
                          b"_gc_root_\x00" as *const u8 as
                              *const libc::c_char,
                          (::std::mem::size_of::<[libc::c_char; 10]>() as
                               libc::c_ulong).wrapping_sub(1i32 as
                                                               libc::c_ulong));
    table = mrb_gv_get(mrb, root);
    if table.tt as libc::c_uint == MRB_TT_FALSE as libc::c_int as libc::c_uint
           && 0 == table.value.i {
        return
    }
    if table.tt as libc::c_uint != MRB_TT_ARRAY as libc::c_int as libc::c_uint
       {
        mrb_gv_set(mrb, root, mrb_nil_value());
        return
    }
    a = table.value.p as *mut RArray;
    mrb_ary_modify(mrb, a);
    i = 0i32 as mrb_int;
    while i <
              if 0 != (*a).flags() as libc::c_int & 7i32 {
                  (((*a).flags() as libc::c_int & 7i32) - 1i32) as mrb_int
              } else { (*a).as_0.heap.len } {
        if 0 !=
               mrb_obj_eq(mrb,
                          *if 0 != (*a).flags() as libc::c_int & 7i32 {
                               &mut (*a).as_0 as *mut unnamed_4 as
                                   *mut mrb_value
                           } else { (*a).as_0.heap.ptr }.offset(i as isize),
                          obj) {
            let mut len: mrb_int =
                if 0 != (*a).flags() as libc::c_int & 7i32 {
                    (((*a).flags() as libc::c_int & 7i32) - 1i32) as mrb_int
                } else { (*a).as_0.heap.len } - 1i32 as libc::c_longlong;
            let mut ptr: *mut mrb_value =
                if 0 != (*a).flags() as libc::c_int & 7i32 {
                    &mut (*a).as_0 as *mut unnamed_4 as *mut mrb_value
                } else { (*a).as_0.heap.ptr };
            if 0 != (*a).flags() as libc::c_int & 7i32 {
                if 0 !=
                       !(len <=
                             (::std::mem::size_of::<*mut libc::c_void>() as
                                  libc::c_ulong).wrapping_mul(3i32 as
                                                                  libc::c_ulong).wrapping_div(::std::mem::size_of::<mrb_value>()
                                                                                                  as
                                                                                                  libc::c_ulong)
                                 as mrb_int) as libc::c_int as libc::c_long {
                    __assert_rtn((*::std::mem::transmute::<&[u8; 18],
                                                           &[libc::c_char; 18]>(b"mrb_gc_unregister\x00")).as_ptr(),
                                 b"src/gc.c\x00" as *const u8 as
                                     *const libc::c_char, 513i32,
                                 b"(len) <= ((mrb_int)(sizeof(void*)*3/sizeof(mrb_value)))\x00"
                                     as *const u8 as *const libc::c_char);
                } else { };
                (*a).set_flags(((*a).flags() as libc::c_int & !7i32) as
                                   libc::c_uint |
                                   (len as
                                        uint32_t).wrapping_add(1i32 as
                                                                   libc::c_uint))
            } else { (*a).as_0.heap.len = len }
            memmove(&mut *ptr.offset(i as isize) as *mut mrb_value as
                        *mut libc::c_void,
                    &mut *ptr.offset((i + 1i32 as libc::c_longlong) as isize)
                        as *mut mrb_value as *const libc::c_void,
                    ((len - i) as
                         libc::c_ulonglong).wrapping_mul(::std::mem::size_of::<mrb_value>()
                                                             as libc::c_ulong
                                                             as
                                                             libc::c_ulonglong)
                        as libc::c_ulong);
            break ;
        } else { i += 1 }
    };
}
/* temporary memory allocation, only effective while GC arena is kept */
#[no_mangle]
pub unsafe extern "C" fn mrb_alloca(mut mrb: *mut mrb_state, mut size: size_t)
 -> *mut libc::c_void {
    let mut str: mrb_value = mrb_str_new(mrb, 0 as *const libc::c_char, size);
    return (if 0 !=
                   (*(str.value.p as *mut RString)).flags() as libc::c_int &
                       32i32 {
                (*(str.value.p as *mut RString)).as_0.ary.as_mut_ptr()
            } else { (*(str.value.p as *mut RString)).as_0.heap.ptr }) as
               *mut libc::c_void;
}
#[no_mangle]
pub unsafe extern "C" fn mrb_gc_init(mut mrb: *mut mrb_state,
                                     mut gc: *mut mrb_gc) {
    (*gc).arena =
        mrb_malloc(mrb,
                   (::std::mem::size_of::<*mut RBasic>() as
                        libc::c_ulong).wrapping_mul(100i32 as libc::c_ulong))
            as *mut *mut RBasic;
    (*gc).arena_capa = 100i32;
    (*gc).current_white_part = 1i32;
    (*gc).heaps = 0 as *mut mrb_heap_page;
    (*gc).free_heaps = 0 as *mut mrb_heap_page;
    add_heap(mrb, gc);
    (*gc).interval_ratio = 200i32;
    (*gc).step_ratio = 200i32;
    (*gc).set_generational(1i32 as mrb_bool);
    (*gc).set_full(1i32 as mrb_bool);
}
unsafe extern "C" fn free_heap(mut mrb: *mut mrb_state, mut gc: *mut mrb_gc) {
    let mut page: *mut mrb_heap_page = (*gc).heaps;
    let mut tmp: *mut mrb_heap_page = 0 as *mut mrb_heap_page;
    let mut p: *mut RVALUE = 0 as *mut RVALUE;
    let mut e: *mut RVALUE = 0 as *mut RVALUE;
    while !page.is_null() {
        tmp = page;
        page = (*page).next;
        p = (*tmp).objects.as_mut_ptr() as *mut RVALUE;
        e = p.offset(1024isize);
        while p < e {
            if (*p).as_0.free.tt() as libc::c_int !=
                   MRB_TT_FREE as libc::c_int {
                obj_free(mrb, &mut (*p).as_0.basic, 1i32);
            }
            p = p.offset(1isize)
        }
        mrb_free(mrb, tmp as *mut libc::c_void);
    };
}
#[no_mangle]
pub unsafe extern "C" fn mrb_gc_destroy(mut mrb: *mut mrb_state,
                                        mut gc: *mut mrb_gc) {
    free_heap(mrb, gc);
    mrb_free(mrb, (*gc).arena as *mut libc::c_void);
}
/*
 *  call-seq:
 *     GC.start                     -> nil
 *
 *  Initiates full garbage collection.
 *
 */
unsafe extern "C" fn gc_start(mut mrb: *mut mrb_state, mut obj: mrb_value)
 -> mrb_value {
    mrb_full_gc(mrb);
    return mrb_nil_value();
}
/*
 *  call-seq:
 *     GC.enable    -> true or false
 *
 *  Enables garbage collection, returning <code>true</code> if garbage
 *  collection was previously disabled.
 *
 *     GC.disable   #=> false
 *     GC.enable    #=> true
 *     GC.enable    #=> false
 *
 */
unsafe extern "C" fn gc_enable(mut mrb: *mut mrb_state, mut obj: mrb_value)
 -> mrb_value {
    let mut old: mrb_bool = (*mrb).gc.disabled();
    (*mrb).gc.set_disabled(0i32 as mrb_bool);
    return mrb_bool_value(old);
}
/*
 *  call-seq:
 *     GC.disable    -> true or false
 *
 *  Disables garbage collection, returning <code>true</code> if garbage
 *  collection was already disabled.
 *
 *     GC.disable   #=> false
 *     GC.disable   #=> true
 *
 */
unsafe extern "C" fn gc_disable(mut mrb: *mut mrb_state, mut obj: mrb_value)
 -> mrb_value {
    let mut old: mrb_bool = (*mrb).gc.disabled();
    (*mrb).gc.set_disabled(1i32 as mrb_bool);
    return mrb_bool_value(old);
}
/*
 *  call-seq:
 *     GC.interval_ratio      -> fixnum
 *
 *  Returns ratio of GC interval. Default value is 200(%).
 *
 */
unsafe extern "C" fn gc_interval_ratio_get(mut mrb: *mut mrb_state,
                                           mut obj: mrb_value) -> mrb_value {
    return mrb_fixnum_value((*mrb).gc.interval_ratio as mrb_int);
}
/*
 *  call-seq:
 *     GC.interval_ratio = fixnum    -> nil
 *
 *  Updates ratio of GC interval. Default value is 200(%).
 *  GC start as soon as after end all step of GC if you set 100(%).
 *
 */
unsafe extern "C" fn gc_interval_ratio_set(mut mrb: *mut mrb_state,
                                           mut obj: mrb_value) -> mrb_value {
    let mut ratio: mrb_int = 0;
    mrb_get_args(mrb, b"i\x00" as *const u8 as *const libc::c_char,
                 &mut ratio as *mut mrb_int);
    (*mrb).gc.interval_ratio = ratio as libc::c_int;
    return mrb_nil_value();
}
/*
 *  call-seq:
 *     GC.step_ratio    -> fixnum
 *
 *  Returns step span ratio of Incremental GC. Default value is 200(%).
 *
 */
unsafe extern "C" fn gc_step_ratio_get(mut mrb: *mut mrb_state,
                                       mut obj: mrb_value) -> mrb_value {
    return mrb_fixnum_value((*mrb).gc.step_ratio as mrb_int);
}
/*
 *  call-seq:
 *     GC.step_ratio = fixnum   -> nil
 *
 *  Updates step span ratio of Incremental GC. Default value is 200(%).
 *  1 step of incrementalGC becomes long if a rate is big.
 *
 */
unsafe extern "C" fn gc_step_ratio_set(mut mrb: *mut mrb_state,
                                       mut obj: mrb_value) -> mrb_value {
    let mut ratio: mrb_int = 0;
    mrb_get_args(mrb, b"i\x00" as *const u8 as *const libc::c_char,
                 &mut ratio as *mut mrb_int);
    (*mrb).gc.step_ratio = ratio as libc::c_int;
    return mrb_nil_value();
}
unsafe extern "C" fn change_gen_gc_mode(mut mrb: *mut mrb_state,
                                        mut gc: *mut mrb_gc,
                                        mut enable: mrb_bool) {
    if 0 != (*gc).disabled() as libc::c_int ||
           0 != (*gc).iterating() as libc::c_int {
        mrb_raise(mrb,
                  mrb_exc_get(mrb,
                              b"RuntimeError\x00" as *const u8 as
                                  *const libc::c_char),
                  b"generational mode changed when GC disabled\x00" as
                      *const u8 as *const libc::c_char);
    }
    if 0 != (*gc).generational() as libc::c_int && 0 == enable {
        clear_all_old(mrb, gc);
        if 0 !=
               !((*gc).state as libc::c_uint ==
                     MRB_GC_STATE_ROOT as libc::c_int as libc::c_uint) as
                   libc::c_int as libc::c_long {
            __assert_rtn((*::std::mem::transmute::<&[u8; 19],
                                                   &[libc::c_char; 19]>(b"change_gen_gc_mode\x00")).as_ptr(),
                         b"src/gc.c\x00" as *const u8 as *const libc::c_char,
                         1509i32,
                         b"gc->state == MRB_GC_STATE_ROOT\x00" as *const u8 as
                             *const libc::c_char);
        } else { };
        (*gc).set_full(0i32 as mrb_bool)
    } else if 0 == (*gc).generational() && 0 != enable as libc::c_int {
        incremental_gc_until(mrb, gc, MRB_GC_STATE_ROOT);
        (*gc).majorgc_old_threshold =
            (*gc).live_after_mark.wrapping_div(100i32 as
                                                   libc::c_ulong).wrapping_mul(120i32
                                                                                   as
                                                                                   libc::c_ulong);
        (*gc).set_full(0i32 as mrb_bool)
    }
    (*gc).set_generational(enable);
}
/*
 *  call-seq:
 *     GC.generational_mode -> true or false
 *
 *  Returns generational or normal gc mode.
 *
 */
unsafe extern "C" fn gc_generational_mode_get(mut mrb: *mut mrb_state,
                                              mut self_0: mrb_value)
 -> mrb_value {
    return mrb_bool_value((*mrb).gc.generational());
}
/*
 *  call-seq:
 *     GC.generational_mode = true or false -> true or false
 *
 *  Changes to generational or normal gc mode.
 *
 */
unsafe extern "C" fn gc_generational_mode_set(mut mrb: *mut mrb_state,
                                              mut self_0: mrb_value)
 -> mrb_value {
    let mut enable: mrb_bool = 0;
    mrb_get_args(mrb, b"b\x00" as *const u8 as *const libc::c_char,
                 &mut enable as *mut mrb_bool);
    if (*mrb).gc.generational() as libc::c_int != enable as libc::c_int {
        change_gen_gc_mode(mrb, &mut (*mrb).gc, enable);
    }
    return mrb_bool_value(enable);
}
#[no_mangle]
pub unsafe extern "C" fn mrb_init_gc(mut mrb: *mut mrb_state) {
    let mut gc: *mut RClass = 0 as *mut RClass;
    gc =
        mrb_define_module(mrb, b"GC\x00" as *const u8 as *const libc::c_char);
    mrb_define_class_method(mrb, gc,
                            b"start\x00" as *const u8 as *const libc::c_char,
                            Some(gc_start), 0i32 as mrb_aspec);
    mrb_define_class_method(mrb, gc,
                            b"enable\x00" as *const u8 as *const libc::c_char,
                            Some(gc_enable), 0i32 as mrb_aspec);
    mrb_define_class_method(mrb, gc,
                            b"disable\x00" as *const u8 as
                                *const libc::c_char, Some(gc_disable),
                            0i32 as mrb_aspec);
    mrb_define_class_method(mrb, gc,
                            b"interval_ratio\x00" as *const u8 as
                                *const libc::c_char,
                            Some(gc_interval_ratio_get), 0i32 as mrb_aspec);
    mrb_define_class_method(mrb, gc,
                            b"interval_ratio=\x00" as *const u8 as
                                *const libc::c_char,
                            Some(gc_interval_ratio_set),
                            ((1i32 & 0x1fi32) as mrb_aspec) << 18i32);
    mrb_define_class_method(mrb, gc,
                            b"step_ratio\x00" as *const u8 as
                                *const libc::c_char, Some(gc_step_ratio_get),
                            0i32 as mrb_aspec);
    mrb_define_class_method(mrb, gc,
                            b"step_ratio=\x00" as *const u8 as
                                *const libc::c_char, Some(gc_step_ratio_set),
                            ((1i32 & 0x1fi32) as mrb_aspec) << 18i32);
    mrb_define_class_method(mrb, gc,
                            b"generational_mode=\x00" as *const u8 as
                                *const libc::c_char,
                            Some(gc_generational_mode_set),
                            ((1i32 & 0x1fi32) as mrb_aspec) << 18i32);
    mrb_define_class_method(mrb, gc,
                            b"generational_mode\x00" as *const u8 as
                                *const libc::c_char,
                            Some(gc_generational_mode_get),
                            0i32 as mrb_aspec);
}
unsafe extern "C" fn run_static_initializers() {
    RVALUE_zero =
        unsafe {
            RVALUE{as_0:
                       unnamed_3{free:
                                     {
                                         let mut init =
                                             free_obj{tt_color_flags: [0; 4],
                                                      _pad: [0; 4],
                                                      c:
                                                          0 as *const RClass
                                                              as *mut RClass,
                                                      gcnext:
                                                          0 as *const RBasic
                                                              as *mut RBasic,
                                                      next:
                                                          0 as *const RBasic
                                                              as
                                                              *mut RBasic,};
                                         init.set_tt(MRB_TT_FALSE);
                                         init.set_color(0);
                                         init.set_flags(0);
                                         init
                                     },},}
        }
}
#[used]
#[cfg_attr ( target_os = "linux" , link_section = ".init_array" )]
#[cfg_attr ( target_os = "windows" , link_section = ".CRT$XIB" )]
#[cfg_attr ( target_os = "macos" , link_section = "__DATA,__mod_init_func" )]
static INIT_ARRAY: [unsafe extern "C" fn(); 1] = [run_static_initializers];