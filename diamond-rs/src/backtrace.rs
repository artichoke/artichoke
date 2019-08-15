use libc;
use c2rust_bitfields::BitfieldStruct;
extern "C" {
    pub type iv_tbl;
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
    #[no_mangle]
    fn mrb_free(_: *mut mrb_state, _: *mut libc::c_void);
    #[no_mangle]
    fn mrb_malloc(_: *mut mrb_state, _: size_t) -> *mut libc::c_void;
    #[no_mangle]
    fn mrb_sym2name(_: *mut mrb_state, _: mrb_sym) -> *const libc::c_char;
    #[no_mangle]
    fn mrb_intern_static(_: *mut mrb_state, _: *const libc::c_char, _: size_t)
     -> mrb_sym;
    #[no_mangle]
    fn mrb_format(mrb: *mut mrb_state, format: *const libc::c_char, _: ...)
     -> mrb_value;
    #[no_mangle]
    fn mrb_iv_get(mrb: *mut mrb_state, obj: mrb_value, sym: mrb_sym)
     -> mrb_value;
    #[no_mangle]
    fn mrb_iv_set(mrb: *mut mrb_state, obj: mrb_value, sym: mrb_sym,
                  v: mrb_value);
    #[no_mangle]
    fn mrb_iv_defined(_: *mut mrb_state, _: mrb_value, _: mrb_sym)
     -> mrb_bool;
    #[no_mangle]
    fn mrb_ary_new_capa(_: *mut mrb_state, _: mrb_int) -> mrb_value;
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
    fn mrb_str_cat_cstr(mrb: *mut mrb_state, str: mrb_value,
                        ptr: *const libc::c_char) -> mrb_value;
    /*
 * get line from irep's debug info and program counter
 * @return returns NULL if not found
 */
    #[no_mangle]
    fn mrb_debug_get_filename(mrb: *mut mrb_state, irep: *mut mrb_irep,
                              pc: ptrdiff_t) -> *const libc::c_char;
    /*
 * get line from irep's debug info and program counter
 * @return returns -1 if not found
 */
    #[no_mangle]
    fn mrb_debug_get_line(mrb: *mut mrb_state, irep: *mut mrb_irep,
                          pc: ptrdiff_t) -> int32_t;
    #[no_mangle]
    fn mrb_data_object_alloc(mrb: *mut mrb_state, klass: *mut RClass,
                             datap: *mut libc::c_void,
                             type_0: *const mrb_data_type) -> *mut RData;
    #[no_mangle]
    fn mrb_data_check_get_ptr(mrb: *mut mrb_state, _: mrb_value,
                              _: *const mrb_data_type) -> *mut libc::c_void;
}
pub type int32_t = libc::c_int;
pub type int64_t = libc::c_longlong;
pub type uint8_t = libc::c_uchar;
pub type uint16_t = libc::c_ushort;
pub type uint32_t = libc::c_uint;
pub type __darwin_ptrdiff_t = libc::c_long;
pub type __darwin_size_t = libc::c_ulong;
pub type ptrdiff_t = __darwin_ptrdiff_t;
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
/*
** mruby/boxing_no.h - unboxed mrb_value definition
**
** See Copyright Notice in mruby.h
*/
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
pub struct mrb_irep_debug_info {
    pub pc_count: uint32_t,
    pub flen: uint16_t,
    pub files: *mut *mut mrb_irep_debug_info_file,
}
#[derive ( Copy , Clone )]
#[repr(C)]
pub struct mrb_irep_debug_info_file {
    pub start_pos: uint32_t,
    pub filename_sym: mrb_sym,
    pub line_entry_count: uint32_t,
    pub line_type: mrb_debug_line_type,
    pub lines: C2RustUnnamed_3,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub union C2RustUnnamed_3 {
    pub ptr: *mut libc::c_void,
    pub flat_map: *mut mrb_irep_debug_info_line,
    pub ary: *mut uint16_t,
}
#[derive ( Copy , Clone )]
#[repr(C)]
pub struct mrb_irep_debug_info_line {
    pub start_pos: uint32_t,
    pub line: uint16_t,
}
/*
** mruby/debug.h - mruby debug info
**
** See Copyright Notice in mruby.h
*/
/* *
 * MRuby Debugging.
 */
pub type mrb_debug_line_type = libc::c_uint;
pub const mrb_debug_line_flat_map: mrb_debug_line_type = 1;
pub const mrb_debug_line_ary: mrb_debug_line_type = 0;
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
** backtrace.c -
**
** See Copyright Notice in mruby.h
*/
#[derive ( Copy , Clone )]
#[repr(C)]
pub struct backtrace_location {
    pub lineno: int32_t,
    pub method_id: mrb_sym,
    pub filename: *const libc::c_char,
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
pub type each_backtrace_func
    =
    Option<unsafe extern "C" fn(_: *mut mrb_state,
                                _: *const backtrace_location,
                                _: *mut libc::c_void) -> ()>;
#[inline]
unsafe extern "C" fn mrb_gc_arena_save(mut mrb: *mut mrb_state)
 -> libc::c_int {
    return (*mrb).gc.arena_idx;
}
#[inline]
unsafe extern "C" fn mrb_obj_value(mut p: *mut libc::c_void) -> mrb_value {
    let mut v: mrb_value =
        mrb_value{value: C2RustUnnamed_0{f: 0.,}, tt: MRB_TT_FALSE,};
    v.tt = (*(p as *mut RBasic as *mut RObject)).tt();
    v.value.p = p as *mut RBasic as *mut libc::c_void;
    return v;
}
#[inline]
unsafe extern "C" fn mrb_gc_arena_restore(mut mrb: *mut mrb_state,
                                          mut idx: libc::c_int) {
    (*mrb).gc.arena_idx = idx;
}
static mut bt_type: mrb_data_type =
    unsafe {
        mrb_data_type{struct_name:
                          b"Backtrace\x00" as *const u8 as
                              *const libc::c_char,
                      dfree:
                          Some(mrb_free as
                                   unsafe extern "C" fn(_: *mut mrb_state,
                                                        _: *mut libc::c_void)
                                       -> ()),}
    };
unsafe extern "C" fn each_backtrace(mut mrb: *mut mrb_state,
                                    mut ciidx: ptrdiff_t,
                                    mut pc0: *mut mrb_code,
                                    mut func: each_backtrace_func,
                                    mut data: *mut libc::c_void) {
    let mut i: ptrdiff_t = 0;
    if ciidx >=
           (*(*mrb).c).ciend.wrapping_offset_from((*(*mrb).c).cibase) as
               libc::c_long {
        /* ciidx is broken... */
        ciidx = 10i32 as ptrdiff_t
    }
    let mut current_block_16: u64;
    i = ciidx;
    while i >= 0i32 as libc::c_long {
        let mut loc: backtrace_location =
            backtrace_location{lineno: 0,
                               method_id: 0,
                               filename: 0 as *const libc::c_char,};
        let mut ci: *mut mrb_callinfo = 0 as *mut mrb_callinfo;
        let mut irep: *mut mrb_irep = 0 as *mut mrb_irep;
        let mut pc: *mut mrb_code = 0 as *mut mrb_code;
        ci = &mut *(*(*mrb).c).cibase.offset(i as isize) as *mut mrb_callinfo;
        if !(*ci).proc_0.is_null() {
            if !((*(*ci).proc_0).flags() as libc::c_int & 128i32 != 0i32) {
                irep = (*(*ci).proc_0).body.irep;
                if !irep.is_null() {
                    if !(*(*(*mrb).c).cibase.offset(i as isize)).err.is_null()
                       {
                        pc = (*(*(*mrb).c).cibase.offset(i as isize)).err;
                        current_block_16 = 2838571290723028321;
                    } else if i + 1i32 as libc::c_long <= ciidx {
                        if (*(*(*mrb).c).cibase.offset((i +
                                                            1i32 as
                                                                libc::c_long)
                                                           as
                                                           isize)).pc.is_null()
                           {
                            current_block_16 = 12675440807659640239;
                        } else {
                            pc =
                                &mut *(*(*(*mrb).c).cibase.offset((i +
                                                                       1i32 as
                                                                           libc::c_long)
                                                                      as
                                                                      isize)).pc.offset(-1i32
                                                                                            as
                                                                                            isize)
                                    as *mut mrb_code;
                            current_block_16 = 2838571290723028321;
                        }
                    } else {
                        pc = pc0;
                        current_block_16 = 2838571290723028321;
                    }
                    match current_block_16 {
                        12675440807659640239 => { }
                        _ => {
                            loc.lineno =
                                mrb_debug_get_line(mrb, irep,
                                                   pc.wrapping_offset_from((*irep).iseq)
                                                       as libc::c_long);
                            if !(loc.lineno == -1i32) {
                                loc.filename =
                                    mrb_debug_get_filename(mrb, irep,
                                                           pc.wrapping_offset_from((*irep).iseq)
                                                               as
                                                               libc::c_long);
                                if loc.filename.is_null() {
                                    loc.filename =
                                        b"(unknown)\x00" as *const u8 as
                                            *const libc::c_char
                                }
                                loc.method_id = (*ci).mid;
                                func.expect("non-null function pointer")(mrb,
                                                                         &mut loc,
                                                                         data);
                            }
                        }
                    }
                }
            }
        }
        i -= 1
    };
}
#[no_mangle]
pub unsafe extern "C" fn mrb_print_backtrace(mut mrb: *mut mrb_state) { }
unsafe extern "C" fn count_backtrace_i(mut mrb: *mut mrb_state,
                                       mut loc: *const backtrace_location,
                                       mut data: *mut libc::c_void) {
    let mut lenp: *mut libc::c_int = data as *mut libc::c_int;
    if (*loc).filename.is_null() { return }
    *lenp += 1;
}
unsafe extern "C" fn pack_backtrace_i(mut mrb: *mut mrb_state,
                                      mut loc: *const backtrace_location,
                                      mut data: *mut libc::c_void) {
    let mut pptr: *mut *mut backtrace_location =
        data as *mut *mut backtrace_location;
    let mut ptr: *mut backtrace_location = *pptr;
    if (*loc).filename.is_null() { return }
    *ptr = *loc;
    *pptr = ptr.offset(1isize);
}
unsafe extern "C" fn packed_backtrace(mut mrb: *mut mrb_state) -> mrb_value {
    let mut backtrace: *mut RData = 0 as *mut RData;
    let mut ciidx: ptrdiff_t =
        (*(*mrb).c).ci.wrapping_offset_from((*(*mrb).c).cibase) as
            libc::c_long;
    let mut len: libc::c_int = 0i32;
    let mut size: libc::c_int = 0;
    let mut ptr: *mut libc::c_void = 0 as *mut libc::c_void;
    each_backtrace(mrb, ciidx, (*(*(*mrb).c).ci).pc,
                   Some(count_backtrace_i as
                            unsafe extern "C" fn(_: *mut mrb_state,
                                                 _: *const backtrace_location,
                                                 _: *mut libc::c_void) -> ()),
                   &mut len as *mut libc::c_int as *mut libc::c_void);
    size =
        (len as
             libc::c_ulong).wrapping_mul(::std::mem::size_of::<backtrace_location>()
                                             as libc::c_ulong) as libc::c_int;
    ptr = mrb_malloc(mrb, size as size_t);
    backtrace = mrb_data_object_alloc(mrb, 0 as *mut RClass, ptr, &bt_type);
    (*backtrace).set_flags(len as libc::c_uint);
    each_backtrace(mrb, ciidx, (*(*(*mrb).c).ci).pc,
                   Some(pack_backtrace_i as
                            unsafe extern "C" fn(_: *mut mrb_state,
                                                 _: *const backtrace_location,
                                                 _: *mut libc::c_void) -> ()),
                   &mut ptr as *mut *mut libc::c_void as *mut libc::c_void);
    return mrb_obj_value(backtrace as *mut libc::c_void);
}
#[no_mangle]
pub unsafe extern "C" fn mrb_keep_backtrace(mut mrb: *mut mrb_state,
                                            mut exc: mrb_value) {
    let mut sym: mrb_sym =
        mrb_intern_static(mrb,
                          b"backtrace\x00" as *const u8 as
                              *const libc::c_char,
                          (::std::mem::size_of::<[libc::c_char; 10]>() as
                               libc::c_ulong).wrapping_sub(1i32 as
                                                               libc::c_ulong));
    let mut backtrace: mrb_value =
        mrb_value{value: C2RustUnnamed_0{f: 0.,}, tt: MRB_TT_FALSE,};
    let mut ai: libc::c_int = 0;
    if 0 != mrb_iv_defined(mrb, exc, sym) { return }
    ai = mrb_gc_arena_save(mrb);
    backtrace = packed_backtrace(mrb);
    mrb_iv_set(mrb, exc, sym, backtrace);
    mrb_gc_arena_restore(mrb, ai);
}
#[no_mangle]
pub unsafe extern "C" fn mrb_unpack_backtrace(mut mrb: *mut mrb_state,
                                              mut backtrace: mrb_value)
 -> mrb_value {
    let mut bt: *const backtrace_location = 0 as *const backtrace_location;
    let mut n: mrb_int = 0;
    let mut i: mrb_int = 0;
    let mut ai: libc::c_int = 0;
    if !(backtrace.tt as libc::c_uint ==
             MRB_TT_FALSE as libc::c_int as libc::c_uint &&
             0 == backtrace.value.i) {
        if backtrace.tt as libc::c_uint ==
               MRB_TT_ARRAY as libc::c_int as libc::c_uint {
            return backtrace
        }
        bt =
            mrb_data_check_get_ptr(mrb, backtrace, &bt_type) as
                *mut backtrace_location;
        if !bt.is_null() {
            n = (*(backtrace.value.p as *mut RData)).flags() as mrb_int;
            backtrace = mrb_ary_new_capa(mrb, n);
            ai = mrb_gc_arena_save(mrb);
            i = 0i32 as mrb_int;
            while i < n {
                let mut entry: *const backtrace_location =
                    &*bt.offset(i as isize) as *const backtrace_location;
                let mut btline: mrb_value =
                    mrb_value{value: C2RustUnnamed_0{f: 0.,},
                              tt: MRB_TT_FALSE,};
                if !(*entry).filename.is_null() {
                    btline =
                        mrb_format(mrb,
                                   b"%s:%d\x00" as *const u8 as
                                       *const libc::c_char, (*entry).filename,
                                   (*entry).lineno);
                    if (*entry).method_id != 0i32 as libc::c_uint {
                        mrb_str_cat(mrb, btline,
                                    b":in \x00" as *const u8 as
                                        *const libc::c_char,
                                    (::std::mem::size_of::<[libc::c_char; 5]>()
                                         as
                                         libc::c_ulong).wrapping_sub(1i32 as
                                                                         libc::c_ulong));
                        mrb_str_cat_cstr(mrb, btline,
                                         mrb_sym2name(mrb,
                                                      (*entry).method_id));
                    }
                    mrb_ary_push(mrb, backtrace, btline);
                    mrb_gc_arena_restore(mrb, ai);
                }
                i += 1
            }
            return backtrace
        }
    }
    return mrb_ary_new_capa(mrb, 0i32 as mrb_int);
}
#[no_mangle]
pub unsafe extern "C" fn mrb_exc_backtrace(mut mrb: *mut mrb_state,
                                           mut exc: mrb_value) -> mrb_value {
    let mut attr_name: mrb_sym = 0;
    let mut backtrace: mrb_value =
        mrb_value{value: C2RustUnnamed_0{f: 0.,}, tt: MRB_TT_FALSE,};
    attr_name =
        mrb_intern_static(mrb,
                          b"backtrace\x00" as *const u8 as
                              *const libc::c_char,
                          (::std::mem::size_of::<[libc::c_char; 10]>() as
                               libc::c_ulong).wrapping_sub(1i32 as
                                                               libc::c_ulong));
    backtrace = mrb_iv_get(mrb, exc, attr_name);
    if backtrace.tt as libc::c_uint ==
           MRB_TT_FALSE as libc::c_int as libc::c_uint &&
           0 == backtrace.value.i ||
           backtrace.tt as libc::c_uint ==
               MRB_TT_ARRAY as libc::c_int as libc::c_uint {
        return backtrace
    }
    backtrace = mrb_unpack_backtrace(mrb, backtrace);
    mrb_iv_set(mrb, exc, attr_name, backtrace);
    return backtrace;
}
#[no_mangle]
pub unsafe extern "C" fn mrb_get_backtrace(mut mrb: *mut mrb_state)
 -> mrb_value {
    return mrb_unpack_backtrace(mrb, packed_backtrace(mrb));
}