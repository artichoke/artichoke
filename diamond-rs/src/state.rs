use libc;
use c2rust_bitfields::BitfieldStruct;
extern "C" {
    pub type iv_tbl;
    pub type kh_mt;
    pub type symbol_name;
    pub type RProc;
    pub type REnv;
    /*
** mruby/compile.h - mruby parser
**
** See Copyright Notice in mruby.h
*/
    /* *
 * MRuby Compiler
 */
    pub type mrb_jmpbuf;
    pub type mrb_shared_string;
    #[no_mangle]
    fn free(_: *mut libc::c_void);
    #[no_mangle]
    fn realloc(_: *mut libc::c_void, _: libc::c_ulong) -> *mut libc::c_void;
    #[no_mangle]
    fn memcpy(_: *mut libc::c_void, _: *const libc::c_void, _: libc::c_ulong)
     -> *mut libc::c_void;
    #[no_mangle]
    fn mrb_free(_: *mut mrb_state, _: *mut libc::c_void);
    #[no_mangle]
    fn mrb_realloc(_: *mut mrb_state, _: *mut libc::c_void, _: size_t)
     -> *mut libc::c_void;
    #[no_mangle]
    fn mrb_malloc(_: *mut mrb_state, _: size_t) -> *mut libc::c_void;
    #[no_mangle]
    fn mrb_gc_free_gv(_: *mut mrb_state);
    #[no_mangle]
    fn mrb_debug_info_free(mrb: *mut mrb_state, d: *mut mrb_irep_debug_info);
    #[no_mangle]
    fn mrb_gc_free_str(_: *mut mrb_state, _: *mut RString);
    /*
** state.c - mrb_state open/close functions
**
** See Copyright Notice in mruby.h
*/
    #[no_mangle]
    fn mrb_init_core(_: *mut mrb_state);
    #[no_mangle]
    fn mrb_init_mrbgems(_: *mut mrb_state);
    #[no_mangle]
    fn mrb_gc_init(_: *mut mrb_state, gc: *mut mrb_gc);
    #[no_mangle]
    fn mrb_gc_destroy(_: *mut mrb_state, gc: *mut mrb_gc);
    #[no_mangle]
    fn mrb_free_symtbl(mrb: *mut mrb_state);
}
pub type __darwin_size_t = libc::c_ulong;
pub type int64_t = libc::c_longlong;
pub type size_t = __darwin_size_t;
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
/*
** mruby/boxing_no.h - unboxed mrb_value definition
**
** See Copyright Notice in mruby.h
*/
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
    pub lines: C2RustUnnamed_0,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub union C2RustUnnamed_0 {
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
#[derive ( BitfieldStruct , Clone , Copy )]
#[repr(C)]
pub struct RString {
    #[bitfield(name = "tt", ty = "mrb_vtype", bits = "0..=7")]
    #[bitfield(name = "color", ty = "uint32_t", bits = "8..=10")]
    #[bitfield(name = "flags", ty = "uint32_t", bits = "11..=31")]
    pub tt_color_flags: [u8; 4],
    #[bitfield(padding)]
    pub _pad: [u8; 4],
    pub c: *mut RClass,
    pub gcnext: *mut RBasic,
    pub as_0: C2RustUnnamed_1,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub union C2RustUnnamed_1 {
    pub heap: C2RustUnnamed_2,
    pub ary: [libc::c_char; 24],
}
#[derive ( Copy , Clone )]
#[repr(C)]
pub struct C2RustUnnamed_2 {
    pub len: mrb_int,
    pub aux: C2RustUnnamed_3,
    pub ptr: *mut libc::c_char,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub union C2RustUnnamed_3 {
    pub capa: mrb_int,
    pub shared: *mut mrb_shared_string,
    pub fshared: *mut RString,
}
#[inline]
unsafe extern "C" fn mrb_obj_value(mut p: *mut libc::c_void) -> mrb_value {
    let mut v: mrb_value =
        mrb_value{value: C2RustUnnamed{f: 0.,}, tt: MRB_TT_FALSE,};
    v.tt = (*(p as *mut RBasic as *mut RObject)).tt();
    v.value.p = p as *mut RBasic as *mut libc::c_void;
    return v;
}
#[inline]
unsafe extern "C" fn mrb_gc_arena_restore(mut mrb: *mut mrb_state,
                                          mut idx: libc::c_int) {
    (*mrb).gc.arena_idx = idx;
}
/* *
 * Create new mrb_state with just the MRuby core
 *
 * @param f
 *      Reference to the allocation function.
 *      Use mrb_default_allocf for the default
 * @param ud
 *      User data will be passed to custom allocator f.
 *      If user data isn't required just pass NULL.
 * @return
 *      Pointer to the newly created mrb_state.
 */
#[no_mangle]
pub unsafe extern "C" fn mrb_open_core(mut f: mrb_allocf,
                                       mut ud: *mut libc::c_void)
 -> *mut mrb_state {
    static mut mrb_state_zero: mrb_state =
        mrb_state{jmp: 0 as *const mrb_jmpbuf as *mut mrb_jmpbuf,
                  allocf: None,
                  allocf_ud: 0 as *const libc::c_void as *mut libc::c_void,
                  c: 0 as *const mrb_context as *mut mrb_context,
                  root_c: 0 as *const mrb_context as *mut mrb_context,
                  globals: 0 as *const iv_tbl as *mut iv_tbl,
                  exc: 0 as *const RObject as *mut RObject,
                  top_self: 0 as *const RObject as *mut RObject,
                  object_class: 0 as *const RClass as *mut RClass,
                  class_class: 0 as *const RClass as *mut RClass,
                  module_class: 0 as *const RClass as *mut RClass,
                  proc_class: 0 as *const RClass as *mut RClass,
                  string_class: 0 as *const RClass as *mut RClass,
                  array_class: 0 as *const RClass as *mut RClass,
                  hash_class: 0 as *const RClass as *mut RClass,
                  range_class: 0 as *const RClass as *mut RClass,
                  float_class: 0 as *const RClass as *mut RClass,
                  fixnum_class: 0 as *const RClass as *mut RClass,
                  true_class: 0 as *const RClass as *mut RClass,
                  false_class: 0 as *const RClass as *mut RClass,
                  nil_class: 0 as *const RClass as *mut RClass,
                  symbol_class: 0 as *const RClass as *mut RClass,
                  kernel_module: 0 as *const RClass as *mut RClass,
                  gc:
                      mrb_gc{heaps:
                                 0 as *const mrb_heap_page as
                                     *mut mrb_heap_page,
                             sweeps:
                                 0 as *const mrb_heap_page as
                                     *mut mrb_heap_page,
                             free_heaps:
                                 0 as *const mrb_heap_page as
                                     *mut mrb_heap_page,
                             live: 0,
                             arena:
                                 0 as *const *mut RBasic as *mut *mut RBasic,
                             arena_capa: 0,
                             arena_idx: 0,
                             state: MRB_GC_STATE_ROOT,
                             current_white_part: 0,
                             gray_list: 0 as *const RBasic as *mut RBasic,
                             atomic_gray_list:
                                 0 as *const RBasic as *mut RBasic,
                             live_after_mark: 0,
                             threshold: 0,
                             interval_ratio: 0,
                             step_ratio: 0,
                             iterating_disabled_full_generational_out_of_memory:
                                 [0; 1],
                             _pad: [0; 7],
                             majorgc_old_threshold: 0,},
                  symidx: 0,
                  symtbl: 0 as *const symbol_name as *mut symbol_name,
                  symhash: [0; 256],
                  symcapa: 0,
                  symbuf: [0; 8],
                  eException_class: 0 as *const RClass as *mut RClass,
                  eStandardError_class: 0 as *const RClass as *mut RClass,
                  nomem_err: 0 as *const RObject as *mut RObject,
                  stack_err: 0 as *const RObject as *mut RObject,
                  ud: 0 as *const libc::c_void as *mut libc::c_void,
                  atexit_stack:
                      0 as *const mrb_atexit_func as *mut mrb_atexit_func,
                  atexit_stack_len: 0,
                  ecall_nest: 0,};
    static mut mrb_context_zero: mrb_context =
        mrb_context{prev: 0 as *const mrb_context as *mut mrb_context,
                    stack: 0 as *const mrb_value as *mut mrb_value,
                    stbase: 0 as *const mrb_value as *mut mrb_value,
                    stend: 0 as *const mrb_value as *mut mrb_value,
                    ci: 0 as *const mrb_callinfo as *mut mrb_callinfo,
                    cibase: 0 as *const mrb_callinfo as *mut mrb_callinfo,
                    ciend: 0 as *const mrb_callinfo as *mut mrb_callinfo,
                    rescue: 0 as *const uint16_t as *mut uint16_t,
                    rsize: 0,
                    ensure: 0 as *const *mut RProc as *mut *mut RProc,
                    esize: 0,
                    eidx: 0,
                    status: MRB_FIBER_CREATED,
                    vmexec: 0,
                    fib: 0 as *const RFiber as *mut RFiber,};
    let mut mrb: *mut mrb_state = 0 as *mut mrb_state;
    if f.is_none() {
        f =
            Some(mrb_default_allocf as
                     unsafe extern "C" fn(_: *mut mrb_state,
                                          _: *mut libc::c_void, _: size_t,
                                          _: *mut libc::c_void)
                         -> *mut libc::c_void)
    }
    mrb =
        f.expect("non-null function pointer")(0 as *mut mrb_state,
                                              0 as *mut libc::c_void,
                                              ::std::mem::size_of::<mrb_state>()
                                                  as libc::c_ulong, ud) as
            *mut mrb_state;
    if mrb.is_null() { return 0 as *mut mrb_state }
    *mrb = mrb_state_zero;
    (*mrb).allocf_ud = ud;
    (*mrb).allocf = f;
    (*mrb).atexit_stack_len = 0i32 as uint16_t;
    mrb_gc_init(mrb, &mut (*mrb).gc);
    (*mrb).c =
        mrb_malloc(mrb, ::std::mem::size_of::<mrb_context>() as libc::c_ulong)
            as *mut mrb_context;
    *(*mrb).c = mrb_context_zero;
    (*mrb).root_c = (*mrb).c;
    mrb_init_core(mrb);
    return mrb;
}
/* *
 * The default allocation function.
 *
 * @see mrb_allocf
 */
#[no_mangle]
pub unsafe extern "C" fn mrb_default_allocf(mut mrb: *mut mrb_state,
                                            mut p: *mut libc::c_void,
                                            mut size: size_t,
                                            mut ud: *mut libc::c_void)
 -> *mut libc::c_void {
    if size == 0i32 as libc::c_ulong {
        free(p);
        return 0 as *mut libc::c_void
    } else { return realloc(p, size) };
}
/* *
 * Creates new mrb_state.
 *
 * @return
 *      Pointer to the newly created mrb_state.
 */
#[no_mangle]
pub unsafe extern "C" fn mrb_open() -> *mut mrb_state {
    let mut mrb: *mut mrb_state =
        mrb_open_allocf(Some(mrb_default_allocf as
                                 unsafe extern "C" fn(_: *mut mrb_state,
                                                      _: *mut libc::c_void,
                                                      _: size_t,
                                                      _: *mut libc::c_void)
                                     -> *mut libc::c_void),
                        0 as *mut libc::c_void);
    return mrb;
}
/* *
 * Create new mrb_state with custom allocators.
 *
 * @param f
 *      Reference to the allocation function.
 * @param ud
 *      User data will be passed to custom allocator f.
 *      If user data isn't required just pass NULL.
 * @return
 *      Pointer to the newly created mrb_state.
 */
#[no_mangle]
pub unsafe extern "C" fn mrb_open_allocf(mut f: mrb_allocf,
                                         mut ud: *mut libc::c_void)
 -> *mut mrb_state {
    let mut mrb: *mut mrb_state = mrb_open_core(f, ud);
    if mrb.is_null() { return 0 as *mut mrb_state }
    mrb_init_mrbgems(mrb);
    mrb_gc_arena_restore(mrb, 0i32);
    return mrb;
}
#[no_mangle]
pub unsafe extern "C" fn mrb_irep_incref(mut mrb: *mut mrb_state,
                                         mut irep: *mut mrb_irep) {
    (*irep).refcnt = (*irep).refcnt.wrapping_add(1);
}
#[no_mangle]
pub unsafe extern "C" fn mrb_irep_decref(mut mrb: *mut mrb_state,
                                         mut irep: *mut mrb_irep) {
    (*irep).refcnt = (*irep).refcnt.wrapping_sub(1);
    if (*irep).refcnt == 0i32 as libc::c_uint { mrb_irep_free(mrb, irep); };
}
#[no_mangle]
pub unsafe extern "C" fn mrb_irep_cutref(mut mrb: *mut mrb_state,
                                         mut irep: *mut mrb_irep) {
    let mut tmp: *mut mrb_irep = 0 as *mut mrb_irep;
    let mut i: libc::c_int = 0;
    i = 0i32;
    while i < (*irep).rlen as libc::c_int {
        tmp = *(*irep).reps.offset(i as isize);
        let ref mut fresh0 = *(*irep).reps.offset(i as isize);
        *fresh0 = 0 as *mut mrb_irep;
        if !tmp.is_null() { mrb_irep_decref(mrb, tmp); }
        i += 1
    };
}
#[no_mangle]
pub unsafe extern "C" fn mrb_irep_free(mut mrb: *mut mrb_state,
                                       mut irep: *mut mrb_irep) {
    let mut i: libc::c_int = 0;
    if 0 == (*irep).flags as libc::c_int & 1i32 {
        mrb_free(mrb, (*irep).iseq as *mut libc::c_void);
    }
    if !(*irep).pool.is_null() {
        i = 0i32;
        while i < (*irep).plen as libc::c_int {
            if (*(*irep).pool.offset(i as isize)).tt as libc::c_uint ==
                   MRB_TT_STRING as libc::c_int as libc::c_uint {
                mrb_gc_free_str(mrb,
                                (*(*irep).pool.offset(i as isize)).value.p as
                                    *mut RString);
                mrb_free(mrb,
                         (*(*irep).pool.offset(i as isize)).value.p as
                             *mut RObject as *mut libc::c_void);
            }
            i += 1
        }
    }
    mrb_free(mrb, (*irep).pool as *mut libc::c_void);
    mrb_free(mrb, (*irep).syms as *mut libc::c_void);
    i = 0i32;
    while i < (*irep).rlen as libc::c_int {
        if !(*(*irep).reps.offset(i as isize)).is_null() {
            mrb_irep_decref(mrb, *(*irep).reps.offset(i as isize));
        }
        i += 1
    }
    mrb_free(mrb, (*irep).reps as *mut libc::c_void);
    mrb_free(mrb, (*irep).lv as *mut libc::c_void);
    mrb_debug_info_free(mrb, (*irep).debug_info);
    mrb_free(mrb, irep as *mut libc::c_void);
}
#[no_mangle]
pub unsafe extern "C" fn mrb_str_pool(mut mrb: *mut mrb_state,
                                      mut str: mrb_value) -> mrb_value {
    let mut s: *mut RString = str.value.p as *mut RString;
    let mut ns: *mut RString = 0 as *mut RString;
    let mut ptr: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut len: mrb_int = 0;
    ns =
        mrb_malloc(mrb, ::std::mem::size_of::<RString>() as libc::c_ulong) as
            *mut RString;
    (*ns).set_tt(MRB_TT_STRING);
    (*ns).c = (*mrb).string_class;
    if 0 != (*s).flags() as libc::c_int & 4i32 {
        (*ns).set_flags(4i32 as uint32_t);
        (*ns).as_0.heap.ptr = (*s).as_0.heap.ptr;
        (*ns).as_0.heap.len = (*s).as_0.heap.len;
        (*ns).as_0.heap.aux.capa = 0i32 as mrb_int
    } else {
        (*ns).set_flags(0i32 as uint32_t);
        if 0 != (*s).flags() as libc::c_int & 32i32 {
            ptr = (*s).as_0.ary.as_mut_ptr();
            len =
                (((*s).flags() as libc::c_int & 0x7c0i32) >> 6i32) as mrb_int
        } else { ptr = (*s).as_0.heap.ptr; len = (*s).as_0.heap.len }
        if len <=
               (::std::mem::size_of::<*mut libc::c_void>() as
                    libc::c_ulong).wrapping_mul(3i32 as
                                                    libc::c_ulong).wrapping_sub(1i32
                                                                                    as
                                                                                    libc::c_ulong)
                   as mrb_int {
            (*ns).set_flags((*ns).flags() | 32i32 as uint32_t);
            let mut tmp_n: size_t = len as size_t;
            (*ns).set_flags((*ns).flags() & !0x7c0i32 as uint32_t);
            (*ns).set_flags((*ns).flags() | (tmp_n << 6i32) as uint32_t);
            if !ptr.is_null() {
                memcpy((*ns).as_0.ary.as_mut_ptr() as *mut libc::c_void,
                       ptr as *const libc::c_void, len as libc::c_ulong);
            }
            (*ns).as_0.ary[len as usize] = '\u{0}' as i32 as libc::c_char
        } else {
            (*ns).as_0.heap.ptr =
                mrb_malloc(mrb,
                           (len as
                                size_t).wrapping_add(1i32 as libc::c_ulong))
                    as *mut libc::c_char;
            (*ns).as_0.heap.len = len;
            (*ns).as_0.heap.aux.capa = len;
            if !ptr.is_null() {
                memcpy((*ns).as_0.heap.ptr as *mut libc::c_void,
                       ptr as *const libc::c_void, len as libc::c_ulong);
            }
            *(*ns).as_0.heap.ptr.offset(len as isize) =
                '\u{0}' as i32 as libc::c_char
        }
    }
    (*ns).set_flags((*ns).flags() | 8i32 as uint32_t);
    (*ns).set_flags((*ns).flags() | (1i32 << 20i32) as uint32_t);
    return mrb_obj_value(ns as *mut libc::c_void);
}
#[no_mangle]
pub unsafe extern "C" fn mrb_free_context(mut mrb: *mut mrb_state,
                                          mut c: *mut mrb_context) {
    if c.is_null() { return }
    mrb_free(mrb, (*c).stbase as *mut libc::c_void);
    mrb_free(mrb, (*c).cibase as *mut libc::c_void);
    mrb_free(mrb, (*c).rescue as *mut libc::c_void);
    mrb_free(mrb, (*c).ensure as *mut libc::c_void);
    mrb_free(mrb, c as *mut libc::c_void);
}
/* *
 * Closes and frees a mrb_state.
 *
 * @param mrb
 *      Pointer to the mrb_state to be closed.
 */
#[no_mangle]
pub unsafe extern "C" fn mrb_close(mut mrb: *mut mrb_state) {
    if mrb.is_null() { return }
    if (*mrb).atexit_stack_len as libc::c_int > 0i32 {
        let mut i: mrb_int = 0;
        i = (*mrb).atexit_stack_len as mrb_int;
        while i > 0i32 as libc::c_longlong {
            (*(*mrb).atexit_stack.offset((i - 1i32 as libc::c_longlong) as
                                             isize)).expect("non-null function pointer")(mrb);
            i -= 1
        }
        mrb_free(mrb, (*mrb).atexit_stack as *mut libc::c_void);
    }
    /* free */
    mrb_gc_destroy(mrb, &mut (*mrb).gc);
    mrb_free_context(mrb, (*mrb).root_c);
    mrb_gc_free_gv(mrb);
    mrb_free_symtbl(mrb);
    mrb_free(mrb, mrb as *mut libc::c_void);
}
#[no_mangle]
pub unsafe extern "C" fn mrb_add_irep(mut mrb: *mut mrb_state)
 -> *mut mrb_irep {
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
    let mut irep: *mut mrb_irep = 0 as *mut mrb_irep;
    irep =
        mrb_malloc(mrb, ::std::mem::size_of::<mrb_irep>() as libc::c_ulong) as
            *mut mrb_irep;
    *irep = mrb_irep_zero;
    (*irep).refcnt = 1i32 as uint32_t;
    return irep;
}
#[no_mangle]
pub unsafe extern "C" fn mrb_top_self(mut mrb: *mut mrb_state) -> mrb_value {
    return mrb_obj_value((*mrb).top_self as *mut libc::c_void);
}
#[no_mangle]
pub unsafe extern "C" fn mrb_state_atexit(mut mrb: *mut mrb_state,
                                          mut f: mrb_atexit_func) {
    let mut stack_size: size_t = 0;
    stack_size =
        (::std::mem::size_of::<mrb_atexit_func>() as
             libc::c_ulong).wrapping_mul(((*mrb).atexit_stack_len as
                                              libc::c_int + 1i32) as
                                             libc::c_ulong);
    if (*mrb).atexit_stack_len as libc::c_int == 0i32 {
        (*mrb).atexit_stack =
            mrb_malloc(mrb, stack_size) as *mut mrb_atexit_func
    } else {
        (*mrb).atexit_stack =
            mrb_realloc(mrb, (*mrb).atexit_stack as *mut libc::c_void,
                        stack_size) as *mut mrb_atexit_func
    }
    let fresh1 = (*mrb).atexit_stack_len;
    (*mrb).atexit_stack_len = (*mrb).atexit_stack_len.wrapping_add(1);
    let ref mut fresh2 = *(*mrb).atexit_stack.offset(fresh1 as isize);
    *fresh2 = f;
}