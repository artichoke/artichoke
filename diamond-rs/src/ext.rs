use libc;
use c2rust_bitfields::BitfieldStruct;
extern "C" {
    pub type iv_tbl;
    /* debug info */
    pub type mrb_irep_debug_info;
    pub type symbol_name;
    /*
** mruby/compile.h - mruby parser
**
** See Copyright Notice in mruby.h
*/
    /* *
 * MRuby Compiler
 */
    pub type mrb_jmpbuf;
    /* *
 * Gets a class.
 * @param [mrb_state*] mrb The current mruby state.
 * @param [const char *] name The name of the class.
 * @return [struct RClass *] A reference to the class.
*/
    #[no_mangle]
    fn mrb_class_get(mrb: *mut mrb_state, name: *const libc::c_char)
     -> *mut RClass;
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
    fn mrb_intern(_: *mut mrb_state, _: *const libc::c_char, _: size_t)
     -> mrb_sym;
    #[no_mangle]
    fn mrb_sym2name(_: *mut mrb_state, _: mrb_sym) -> *const libc::c_char;
    #[no_mangle]
    fn mrb_exc_raise(mrb: *mut mrb_state, exc: mrb_value);
    #[no_mangle]
    fn mrb_object_dead_p(mrb: *mut mrb_state, object: *mut RBasic)
     -> mrb_bool;
    #[no_mangle]
    fn mrb_raise(mrb: *mut mrb_state, c: *mut RClass,
                 msg: *const libc::c_char);
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
    pub c2rust_unnamed: C2RustUnnamed,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub union C2RustUnnamed {
    pub proc_0: *mut RProc,
    pub func: mrb_func_t,
}
/* default method cache size: 128 */
/* cache size needs to be power of 2 */
pub type mrb_func_t
    =
    Option<unsafe extern "C" fn(_: *mut mrb_state, _: mrb_value)
               -> mrb_value>;
#[derive ( Copy , Clone )]
#[repr(C)]
pub struct mrb_value {
    pub value: C2RustUnnamed_0,
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
pub union C2RustUnnamed_0 {
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
    #[bitfield(padding)]
    pub _pad: [u8; 4],
    pub c: *mut RClass,
    pub gcnext: *mut RBasic,
    pub body: C2RustUnnamed_2,
    pub upper: *mut RProc,
    pub e: C2RustUnnamed_1,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub union C2RustUnnamed_1 {
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
    #[bitfield(padding)]
    pub _pad: [u8; 4],
    pub c: *mut RClass,
    pub gcnext: *mut RBasic,
    pub stack: *mut mrb_value,
    pub cxt: *mut mrb_context,
    pub mid: mrb_sym,
    #[bitfield(padding)]
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
#[repr ( C )]
pub union C2RustUnnamed_2 {
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
    #[bitfield(padding)]
    pub _pad: [u8; 4],
    pub c: *mut RClass,
    pub gcnext: *mut RBasic,
    pub as_0: C2RustUnnamed_3,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub union C2RustUnnamed_3 {
    pub heap: C2RustUnnamed_4,
}
#[derive ( Copy , Clone )]
#[repr(C)]
pub struct C2RustUnnamed_4 {
    pub len: mrb_int,
    pub aux: C2RustUnnamed_5,
    pub ptr: *mut mrb_value,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub union C2RustUnnamed_5 {
    pub capa: mrb_int,
    pub shared: *mut mrb_shared_array,
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
pub struct RData {
    #[bitfield(name = "tt", ty = "mrb_vtype", bits = "0..=7")]
    #[bitfield(name = "color", ty = "uint32_t", bits = "8..=10")]
    #[bitfield(name = "flags", ty = "uint32_t", bits = "11..=31")]
    pub tt_color_flags: [u8; 4],
    #[bitfield(padding)]
    pub _pad: [u8; 4],
    pub c: *mut RClass,
    pub gcnext: *mut RBasic,
    pub iv: *mut iv_tbl,
    pub type_0: *const mrb_data_type,
    pub data: *mut libc::c_void,
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
/*
 * Returns a float in Ruby.
 */
#[inline]
unsafe extern "C" fn mrb_float_value(mut mrb: *mut mrb_state,
                                     mut f: mrb_float) -> mrb_value {
    let mut v: mrb_value =
        mrb_value{value: C2RustUnnamed_0{f: 0.,}, tt: MRB_TT_FALSE,};
    v.tt = MRB_TT_FLOAT;
    v.value.f = f;
    return v;
}
#[inline]
unsafe extern "C" fn mrb_cptr_value(mut mrb: *mut mrb_state,
                                    mut p: *mut libc::c_void) -> mrb_value {
    let mut v: mrb_value =
        mrb_value{value: C2RustUnnamed_0{f: 0.,}, tt: MRB_TT_FALSE,};
    v.tt = MRB_TT_CPTR;
    v.value.p = p;
    return v;
}
/*
 * Returns a fixnum in Ruby.
 */
#[inline]
unsafe extern "C" fn mrb_fixnum_value(mut i: mrb_int) -> mrb_value {
    let mut v: mrb_value =
        mrb_value{value: C2RustUnnamed_0{f: 0.,}, tt: MRB_TT_FALSE,};
    v.tt = MRB_TT_FIXNUM;
    v.value.i = i;
    return v;
}
#[inline]
unsafe extern "C" fn mrb_obj_value(mut p: *mut libc::c_void) -> mrb_value {
    let mut v: mrb_value =
        mrb_value{value: C2RustUnnamed_0{f: 0.,}, tt: MRB_TT_FALSE,};
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
        mrb_value{value: C2RustUnnamed_0{f: 0.,}, tt: MRB_TT_FALSE,};
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
        mrb_value{value: C2RustUnnamed_0{f: 0.,}, tt: MRB_TT_FALSE,};
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
        mrb_value{value: C2RustUnnamed_0{f: 0.,}, tt: MRB_TT_FALSE,};
    v.tt = MRB_TT_TRUE;
    v.value.i = 1i32 as mrb_int;
    return v;
}
#[inline]
unsafe extern "C" fn mrb_class(mut mrb: *mut mrb_state, mut v: mrb_value)
 -> *mut RClass {
    match v.tt as libc::c_uint {
        0 => {
            if 0 != v.value.i { return (*mrb).false_class }
            return (*mrb).nil_class
        }
        2 => { return (*mrb).true_class }
        4 => { return (*mrb).symbol_class }
        3 => { return (*mrb).fixnum_class }
        6 => { return (*mrb).float_class }
        7 => { return (*mrb).object_class }
        20 => { return 0 as *mut RClass }
        _ => { return (*(v.value.p as *mut RObject)).c }
    };
}
/* obsolete functions and macros */
#[inline]
unsafe extern "C" fn mrb_data_init(mut v: mrb_value,
                                   mut ptr: *mut libc::c_void,
                                   mut type_0: *const mrb_data_type) {
    let ref mut fresh0 = (*(v.value.p as *mut RData)).data;
    *fresh0 = ptr;
    let ref mut fresh1 = (*(v.value.p as *mut RData)).type_0;
    *fresh1 = type_0;
}
// ext is partially derived from mrusty @ 1.0.0
// <https://github.com/anima-engine/mrusty/tree/v1.0.0>
//
// Copyright (C) 2016  Dragoș Tiselice
// Licensed under the Mozilla Public License 2.0
// ext is partially derived from go-mruby @ cd6a04a
// <https://github.com/mitchellh/go-mruby/tree/cd6a04a>
//
// Copyright (c) 2017 Mitchell Hashimoto
// Licensed under the MIT License
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.
// ext is partially derived from mrusty @ 1.0.0
// <https://github.com/anima-engine/mrusty/tree/v1.0.0>
//
// Copyright (C) 2016  Dragoș Tiselice
// Licensed under the Mozilla Public License 2.0
// ext is partially derived from go-mruby @ cd6a04a
// <https://github.com/mitchellh/go-mruby/tree/cd6a04a>
//
// Copyright (c) 2017 Mitchell Hashimoto
// Licensed under the MIT License
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.
/* *
 * C extension bindings of mruby to make implementing the mruby-sys crate
 * easier. The functions defined in mruby-sys.h are limited to those that are
 * either not possible to implment in Rust (e.g. because the functions are
 * inlined) or are simpler to implement in C (e.g. any of the mrb_value
 * initializers).
 */
// Check whether `mrb_value` is nil, false, or true
// Check whether `mrb_value` is nil, false, or true
#[no_mangle]
pub unsafe extern "C" fn mrb_sys_value_is_nil(mut value: mrb_value) -> bool {
    return value.tt as libc::c_uint ==
               MRB_TT_FALSE as libc::c_int as libc::c_uint &&
               0 == value.value.i;
}
#[no_mangle]
pub unsafe extern "C" fn mrb_sys_value_is_false(mut value: mrb_value)
 -> bool {
    return value.tt as libc::c_uint ==
               MRB_TT_FALSE as libc::c_int as libc::c_uint &&
               0 != value.value.i;
}
#[no_mangle]
pub unsafe extern "C" fn mrb_sys_value_is_true(mut value: mrb_value) -> bool {
    return value.tt as libc::c_uint ==
               MRB_TT_TRUE as libc::c_int as libc::c_uint;
}
// Extract pointers from `mrb_value`s
// Extract pointers from `mrb_value`s
#[no_mangle]
pub unsafe extern "C" fn mrb_sys_fixnum_to_cint(mut value: mrb_value)
 -> mrb_int {
    return value.value.i;
}
#[no_mangle]
pub unsafe extern "C" fn mrb_sys_float_to_cdouble(mut value: mrb_value)
 -> mrb_float {
    return value.value.f;
}
#[no_mangle]
pub unsafe extern "C" fn mrb_sys_cptr_ptr(mut value: mrb_value)
 -> *mut libc::c_void {
    return value.value.p;
}
#[no_mangle]
pub unsafe extern "C" fn mrb_sys_basic_ptr(mut value: mrb_value)
 -> *mut RBasic {
    return value.value.p as *mut RBasic;
}
#[no_mangle]
pub unsafe extern "C" fn mrb_sys_obj_ptr(mut value: mrb_value)
 -> *mut RObject {
    return value.value.p as *mut RObject;
}
#[no_mangle]
pub unsafe extern "C" fn mrb_sys_proc_ptr(mut value: mrb_value)
 -> *mut RProc {
    return value.value.p as *mut RProc;
}
#[no_mangle]
pub unsafe extern "C" fn mrb_sys_class_ptr(mut value: mrb_value)
 -> *mut RClass {
    return value.value.p as *mut RClass;
}
#[no_mangle]
pub unsafe extern "C" fn mrb_sys_class_to_rclass(mut value: mrb_value)
 -> *mut RClass {
    return value.value.p as *mut RClass;
}
#[no_mangle]
pub unsafe extern "C" fn mrb_sys_class_of_value(mut mrb: *mut mrb_state,
                                                mut value: mrb_value)
 -> *mut RClass {
    return mrb_class(mrb, value);
}
// Construct `mrb_value`s
// Construct `mrb_value`s
#[no_mangle]
pub unsafe extern "C" fn mrb_sys_nil_value() -> mrb_value {
    return mrb_nil_value();
}
#[no_mangle]
pub unsafe extern "C" fn mrb_sys_false_value() -> mrb_value {
    return mrb_false_value();
}
#[no_mangle]
pub unsafe extern "C" fn mrb_sys_true_value() -> mrb_value {
    return mrb_true_value();
}
#[no_mangle]
pub unsafe extern "C" fn mrb_sys_fixnum_value(mut value: mrb_int)
 -> mrb_value {
    return mrb_fixnum_value(value);
}
#[no_mangle]
pub unsafe extern "C" fn mrb_sys_float_value(mut mrb: *mut mrb_state,
                                             mut value: mrb_float)
 -> mrb_value {
    return mrb_float_value(mrb, value);
}
#[no_mangle]
pub unsafe extern "C" fn mrb_sys_cptr_value(mut mrb: *mut mrb_state,
                                            mut ptr: *mut libc::c_void)
 -> mrb_value {
    let mut value: mrb_value =
        mrb_value{value: C2RustUnnamed_0{f: 0.,}, tt: MRB_TT_FALSE,};
    value.tt = MRB_TT_CPTR;
    value.value.p = ptr;
    return value;
}
#[no_mangle]
pub unsafe extern "C" fn mrb_sys_obj_value(mut p: *mut libc::c_void)
 -> mrb_value {
    return mrb_obj_value(p);
}
#[no_mangle]
pub unsafe extern "C" fn mrb_sys_class_value(mut klass: *mut RClass)
 -> mrb_value {
    let mut value: mrb_value =
        mrb_value{value: C2RustUnnamed_0{f: 0.,}, tt: MRB_TT_FALSE,};
    value.value.p = klass as *mut libc::c_void;
    value.tt = MRB_TT_CLASS;
    return value;
}
#[no_mangle]
pub unsafe extern "C" fn mrb_sys_module_value(mut module: *mut RClass)
 -> mrb_value {
    let mut value: mrb_value =
        mrb_value{value: C2RustUnnamed_0{f: 0.,}, tt: MRB_TT_FALSE,};
    value.value.p = module as *mut libc::c_void;
    value.tt = MRB_TT_MODULE;
    return value;
}
#[no_mangle]
pub unsafe extern "C" fn mrb_sys_data_value(mut data: *mut RData)
 -> mrb_value {
    let mut value: mrb_value =
        mrb_value{value: C2RustUnnamed_0{f: 0.,}, tt: MRB_TT_FALSE,};
    value.value.p = data as *mut libc::c_void;
    value.tt = MRB_TT_DATA;
    return value;
}
#[no_mangle]
pub unsafe extern "C" fn mrb_sys_proc_value(mut mrb: *mut mrb_state,
                                            mut proc_0: *mut RProc)
 -> mrb_value {
    let mut value: mrb_value =
        mrb_cptr_value(mrb, proc_0 as *mut libc::c_void);
    value.tt = MRB_TT_PROC;
    return value;
}
// Manipulate `Symbol`s
// Manipulate `Symbol`s
#[no_mangle]
pub unsafe extern "C" fn mrb_sys_symbol_name(mut mrb: *mut mrb_state,
                                             mut value: mrb_value)
 -> *const libc::c_char {
    return mrb_sym2name(mrb, value.value.sym);
}
#[no_mangle]
pub unsafe extern "C" fn mrb_sys_new_symbol(mut mrb: *mut mrb_state,
                                            mut string: *const libc::c_char,
                                            mut len: size_t) -> mrb_value {
    let mut value: mrb_value =
        mrb_value{value: C2RustUnnamed_0{f: 0.,}, tt: MRB_TT_FALSE,};
    value.value.sym = mrb_intern(mrb, string, len);
    value.tt = MRB_TT_SYMBOL;
    return value;
}
// Manage Rust-backed `mrb_value`s
// Manage Rust-backed `mrb_value`s
#[no_mangle]
pub unsafe extern "C" fn mrb_sys_set_instance_tt(mut class: *mut RClass,
                                                 mut type_0: mrb_vtype) {
    (*class).set_flags(((*class).flags() as libc::c_int & !0xffi32 |
                            type_0 as libc::c_char as libc::c_int) as
                           uint32_t);
}
#[no_mangle]
pub unsafe extern "C" fn mrb_sys_data_init(mut value: *mut mrb_value,
                                           mut ptr: *mut libc::c_void,
                                           mut type_0: *const mrb_data_type) {
    mrb_data_init(*value, ptr, type_0);
}
// Raise exceptions and debug info
// Raise exceptions and debug info
#[no_mangle]
pub unsafe extern "C" fn mrb_sys_raise(mut mrb: *mut mrb_state,
                                       mut eclass: *const libc::c_char,
                                       mut msg: *const libc::c_char) {
    mrb_raise(mrb, mrb_class_get(mrb, eclass), msg);
}
#[no_mangle]
pub unsafe extern "C" fn mrb_sys_raise_current_exception(mut mrb:
                                                             *mut mrb_state) {
    if !(*mrb).exc.is_null() {
        mrb_exc_raise(mrb, mrb_obj_value((*mrb).exc as *mut libc::c_void));
    };
}
// TODO: implement this debug function in Rust
#[no_mangle]
pub unsafe extern "C" fn mrb_sys_value_debug_str(mut mrb: *mut mrb_state,
                                                 mut value: mrb_value)
 -> mrb_value {
    return mrb_funcall(mrb, value,
                       b"inspect\x00" as *const u8 as *const libc::c_char,
                       0i32 as mrb_int);
}
// Manipulate Array `mrb_value`s
// Manipulate Array `mrb_value`s
#[no_mangle]
pub unsafe extern "C" fn mrb_sys_ary_len(mut value: mrb_value) -> mrb_int {
    return if 0 !=
                  (*(value.value.p as *mut RArray)).flags() as libc::c_int &
                      7i32 {
               (((*(value.value.p as *mut RArray)).flags() as libc::c_int &
                     7i32) - 1i32) as mrb_int
           } else { (*(value.value.p as *mut RArray)).as_0.heap.len };
}
// Manage the mruby garbage collector (GC)
/* *
 * Set save point for garbage collection arena to recycle `mrb_value` objects
 * created with C function calls. Returns an index in the arena stack to restore
 * to when calling `mrb_sys_gc_arena_restore`.
 */
// Manage the mruby garbage collector (GC)
#[no_mangle]
pub unsafe extern "C" fn mrb_sys_gc_arena_save(mut mrb: *mut mrb_state)
 -> libc::c_int {
    return mrb_gc_arena_save(mrb);
}
/* *
 * Restore save point for garbage collection arena to recycle `mrb_value`
 * objects created with C function calls.
 */
#[no_mangle]
pub unsafe extern "C" fn mrb_sys_gc_arena_restore(mut mrb: *mut mrb_state,
                                                  mut arena_index:
                                                      libc::c_int) {
    mrb_gc_arena_restore(mrb, arena_index);
}
/* *
 * Disable GC. Returns previous enabled state.
 */
#[no_mangle]
pub unsafe extern "C" fn mrb_sys_gc_disable(mut mrb: *mut mrb_state) -> bool {
    let mut gc: *mut mrb_gc = &mut (*mrb).gc;
    let mut was_enabled: bool = 0 == (*gc).disabled();
    (*gc).set_disabled(1i32 as mrb_bool);
    return was_enabled;
}
/* *
 * Enable GC. Returns previous enabled state.
 */
#[no_mangle]
pub unsafe extern "C" fn mrb_sys_gc_enable(mut mrb: *mut mrb_state) -> bool {
    let mut gc: *mut mrb_gc = &mut (*mrb).gc;
    let mut was_enabled: bool = 0 == (*gc).disabled();
    (*gc).set_disabled(0i32 as mrb_bool);
    return was_enabled;
}
#[no_mangle]
pub unsafe extern "C" fn mrb_sys_value_is_dead(mut mrb: *mut mrb_state,
                                               mut value: mrb_value) -> bool {
    // immediate values such as Fixnums and Symbols are never garbage
  // collected, so they are never dead. See `mrb_gc_protect` in gc.c.
    if (value.tt as libc::c_uint) <
           MRB_TT_OBJECT as libc::c_int as libc::c_uint {
        return 0 != 0i32
    }
    let mut ptr: *mut RBasic = value.value.p as *mut RBasic;
    if ptr.is_null() { return 0 != 1i32 }
    return 0 != mrb_object_dead_p(mrb, ptr);
}
#[no_mangle]
pub unsafe extern "C" fn mrb_sys_gc_live_objects(mut mrb: *mut mrb_state)
 -> libc::c_int {
    let mut gc: *mut mrb_gc = &mut (*mrb).gc;
    return (*gc).live as libc::c_int;
}