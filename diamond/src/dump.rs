use libc;
use c2rust_bitfields::{BitfieldStruct};
extern "C" {
    pub type __sFILEX;
    pub type iv_tbl;
    pub type RClass;
    pub type symbol_name;
    pub type RProc;
    pub type REnv;
    pub type mrb_jmpbuf;
    pub type mrb_shared_string;
    #[no_mangle]
    fn memcpy(_: *mut libc::c_void, _: *const libc::c_void, _: libc::c_ulong)
     -> *mut libc::c_void;
    #[no_mangle]
    fn memset(_: *mut libc::c_void, _: libc::c_int, _: libc::c_ulong)
     -> *mut libc::c_void;
    #[no_mangle]
    fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
    #[no_mangle]
    fn fputs(_: *const libc::c_char, _: *mut FILE) -> libc::c_int;
    #[no_mangle]
    fn fwrite(_: *const libc::c_void, _: libc::c_ulong, _: libc::c_ulong,
              _: *mut FILE) -> libc::c_ulong;
    #[no_mangle]
    fn mrb_sym2name_len(_: *mut mrb_state, _: mrb_sym, _: *mut mrb_int)
     -> *const libc::c_char;
    #[no_mangle]
    fn mrb_malloc(_: *mut mrb_state, _: size_t) -> *mut libc::c_void;
    #[no_mangle]
    fn mrb_realloc(_: *mut mrb_state, _: *mut libc::c_void, _: size_t)
     -> *mut libc::c_void;
    #[no_mangle]
    fn mrb_free(_: *mut mrb_state, _: *mut libc::c_void);
    #[no_mangle]
    fn mrb_str_new_static(mrb: *mut mrb_state, p: *const libc::c_char,
                          len: size_t) -> mrb_value;
    /* * @internal crc.c */
    #[no_mangle]
    fn calc_crc_16_ccitt(src: *const uint8_t, nbytes: size_t, crc: uint16_t)
     -> uint16_t;
    /* ArgumentError if format string doesn't match /%(\.[0-9]+)?[aAeEfFgG]/ */
    #[no_mangle]
    fn mrb_float_to_str(mrb: *mut mrb_state, x: mrb_value,
                        fmt: *const libc::c_char) -> mrb_value;
    #[no_mangle]
    fn mrb_fixnum_to_str(mrb: *mut mrb_state, x: mrb_value, base: mrb_int)
     -> mrb_value;
}
pub type __int64_t = libc::c_longlong;
pub type __darwin_intptr_t = libc::c_long;
pub type __darwin_ptrdiff_t = libc::c_long;
pub type __darwin_size_t = libc::c_ulong;
pub type __darwin_off_t = __int64_t;
pub type size_t = __darwin_size_t;
pub type int64_t = libc::c_longlong;
pub type intptr_t = __darwin_intptr_t;
pub type uint8_t = libc::c_uchar;
pub type uint16_t = libc::c_ushort;
pub type uint32_t = libc::c_uint;
pub type ptrdiff_t = __darwin_ptrdiff_t;
pub type fpos_t = __darwin_off_t;
#[derive ( Copy , Clone )]
#[repr(C)]
pub struct __sbuf {
    pub _base: *mut libc::c_uchar,
    pub _size: libc::c_int,
}
#[derive ( Copy , Clone )]
#[repr(C)]
pub struct __sFILE {
    pub _p: *mut libc::c_uchar,
    pub _r: libc::c_int,
    pub _w: libc::c_int,
    pub _flags: libc::c_short,
    pub _file: libc::c_short,
    pub _bf: __sbuf,
    pub _lbfsize: libc::c_int,
    pub _cookie: *mut libc::c_void,
    pub _close: Option<unsafe extern "C" fn(_: *mut libc::c_void)
                           -> libc::c_int>,
    pub _read: Option<unsafe extern "C" fn(_: *mut libc::c_void,
                                           _: *mut libc::c_char,
                                           _: libc::c_int) -> libc::c_int>,
    pub _seek: Option<unsafe extern "C" fn(_: *mut libc::c_void, _: fpos_t,
                                           _: libc::c_int) -> fpos_t>,
    pub _write: Option<unsafe extern "C" fn(_: *mut libc::c_void,
                                            _: *const libc::c_char,
                                            _: libc::c_int) -> libc::c_int>,
    pub _ub: __sbuf,
    pub _extra: *mut __sFILEX,
    pub _ur: libc::c_int,
    pub _ubuf: [libc::c_uchar; 3],
    pub _nbuf: [libc::c_uchar; 1],
    pub _lb: __sbuf,
    pub _blksize: libc::c_int,
    pub _offset: fpos_t,
}
pub type FILE = __sFILE;
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
    pub lines: unnamed_0,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub union unnamed_0 {
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
** mruby/irep.h - mrb_irep structure
**
** See Copyright Notice in mruby.h
*/
/* *
 * Compiled mruby scripts.
 */
pub type irep_pool_type = libc::c_uint;
pub const IREP_TT_FLOAT: irep_pool_type = 2;
pub const IREP_TT_FIXNUM: irep_pool_type = 1;
pub const IREP_TT_STRING: irep_pool_type = 0;
/* dump/load error code
 *
 * NOTE: MRB_DUMP_GENERAL_FAILURE is caused by
 * unspecified issues like malloc failed.
 */
/* null symbol length */
/* Rite Binary File header */
/* binary header */
#[derive ( Copy , Clone )]
#[repr(C)]
pub struct rite_binary_header {
    pub binary_ident: [uint8_t; 4],
    pub binary_version: [uint8_t; 4],
    pub binary_crc: [uint8_t; 2],
    pub binary_size: [uint8_t; 4],
    pub compiler_name: [uint8_t; 4],
    pub compiler_version: [uint8_t; 4],
}
#[derive ( Copy , Clone )]
#[repr(C)]
pub struct rite_binary_footer {
    pub section_ident: [uint8_t; 4],
    pub section_size: [uint8_t; 4],
}
#[derive ( Copy , Clone )]
#[repr(C)]
pub struct rite_section_lv_header {
    pub section_ident: [uint8_t; 4],
    pub section_size: [uint8_t; 4],
}
#[derive ( Copy , Clone )]
#[repr(C)]
pub struct rite_section_debug_header {
    pub section_ident: [uint8_t; 4],
    pub section_size: [uint8_t; 4],
}
#[derive ( Copy , Clone )]
#[repr(C)]
pub struct rite_section_irep_header {
    pub section_ident: [uint8_t; 4],
    pub section_size: [uint8_t; 4],
    pub rite_version: [uint8_t; 4],
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
    pub as_0: unnamed_3,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub union unnamed_3 {
    pub heap: unnamed_1,
    pub ary: [libc::c_char; 24],
}
#[inline(always)]
unsafe extern "C" fn __inline_isinff(mut __x: libc::c_float) -> libc::c_int {
    return (__x.abs() == ::std::f32::INFINITY) as libc::c_int;
}
#[inline(always)]
unsafe extern "C" fn __inline_isinfd(mut __x: libc::c_double) -> libc::c_int {
    return (__x.abs() == ::std::f64::INFINITY) as libc::c_int;
}
#[inline(always)]
unsafe extern "C" fn __inline_isinfl(mut __x: f128::f128) -> libc::c_int {
    return (__x.abs() == ::std::f64::INFINITY) as libc::c_int;
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
** mruby/dump.h - mruby binary dumper (mrbc binary format)
**
** See Copyright Notice in mruby.h
*/
/* *
 * Dumping compiled mruby script.
 */
#[no_mangle]
pub unsafe extern "C" fn mrb_dump_irep(mut mrb: *mut mrb_state,
                                       mut irep: *mut mrb_irep,
                                       mut flags: uint8_t,
                                       mut bin: *mut *mut uint8_t,
                                       mut bin_size: *mut size_t)
 -> libc::c_int {
    return dump_irep(mrb, irep, dump_flags(flags, 0i32 as uint8_t), bin,
                     bin_size);
}
unsafe extern "C" fn dump_flags(mut flags: uint8_t, mut native: uint8_t)
 -> uint8_t {
    if native as libc::c_int == 2i32 {
        if flags as libc::c_int & 6i32 == 0i32 {
            return (flags as libc::c_int & 1i32 | 6i32) as uint8_t
        }
        return flags
    }
    if flags as libc::c_int & 6i32 == 0i32 {
        return (flags as libc::c_int & 1i32 | 2i32) as uint8_t
    }
    return flags;
}
unsafe extern "C" fn dump_irep(mut mrb: *mut mrb_state,
                               mut irep: *mut mrb_irep, mut flags: uint8_t,
                               mut bin: *mut *mut uint8_t,
                               mut bin_size: *mut size_t) -> libc::c_int {
    let mut current_block: u64;
    let mut result: libc::c_int = -1i32;
    let mut malloc_size: size_t = 0;
    let mut section_irep_size: size_t = 0;
    let mut section_lineno_size: size_t = 0i32 as size_t;
    let mut section_lv_size: size_t = 0i32 as size_t;
    let mut cur: *mut uint8_t = 0 as *mut uint8_t;
    let debug_info_defined: mrb_bool = debug_info_defined_p(irep);
    let lv_defined: mrb_bool = lv_defined_p(irep);
    let mut lv_syms: *mut mrb_sym = 0 as *mut mrb_sym;
    let mut lv_syms_len: uint32_t = 0i32 as uint32_t;
    let mut filenames: *mut mrb_sym = 0 as *mut mrb_sym;
    let mut filenames_len: uint16_t = 0i32 as uint16_t;
    if mrb.is_null() { *bin = 0 as *mut uint8_t; return -1i32 }
    section_irep_size =
        ::std::mem::size_of::<rite_section_irep_header>() as libc::c_ulong;
    section_irep_size =
        (section_irep_size as
             libc::c_ulong).wrapping_add(get_irep_record_size(mrb, irep)) as
            size_t as size_t;
    if 0 != flags as libc::c_int & 1i32 {
        if 0 != debug_info_defined {
            section_lineno_size =
                (section_lineno_size as
                     libc::c_ulong).wrapping_add(::std::mem::size_of::<rite_section_debug_header>()
                                                     as libc::c_ulong) as
                    size_t as size_t;
            filenames =
                mrb_malloc(mrb,
                           (::std::mem::size_of::<mrb_sym>() as
                                libc::c_ulong).wrapping_add(1i32 as
                                                                libc::c_ulong))
                    as *mut mrb_sym;
            section_lineno_size =
                (section_lineno_size as
                     libc::c_ulong).wrapping_add(::std::mem::size_of::<uint16_t>()
                                                     as libc::c_ulong) as
                    size_t as size_t;
            section_lineno_size =
                (section_lineno_size as
                     libc::c_ulong).wrapping_add(get_filename_table_size(mrb,
                                                                         irep,
                                                                         &mut filenames,
                                                                         &mut filenames_len))
                    as size_t as size_t;
            section_lineno_size =
                (section_lineno_size as
                     libc::c_ulong).wrapping_add(get_debug_record_size(mrb,
                                                                       irep))
                    as size_t as size_t
        }
    }
    if 0 != lv_defined {
        section_lv_size =
            (section_lv_size as
                 libc::c_ulong).wrapping_add(::std::mem::size_of::<rite_section_lv_header>()
                                                 as libc::c_ulong) as size_t
                as size_t;
        create_lv_sym_table(mrb, irep, &mut lv_syms, &mut lv_syms_len);
        section_lv_size =
            (section_lv_size as
                 libc::c_ulong).wrapping_add(get_lv_section_size(mrb, irep,
                                                                 lv_syms,
                                                                 lv_syms_len))
                as size_t as size_t
    }
    malloc_size =
        (::std::mem::size_of::<rite_binary_header>() as
             libc::c_ulong).wrapping_add(section_irep_size).wrapping_add(section_lineno_size).wrapping_add(section_lv_size).wrapping_add(::std::mem::size_of::<rite_binary_footer>()
                                                                                                                                             as
                                                                                                                                             libc::c_ulong);
    *bin = mrb_malloc(mrb, malloc_size) as *mut uint8_t;
    cur = *bin;
    cur =
        cur.offset(::std::mem::size_of::<rite_binary_header>() as
                       libc::c_ulong as isize);
    result =
        write_section_irep(mrb, irep, cur, &mut section_irep_size, flags);
    if !(result != 0i32) {
        cur = cur.offset(section_irep_size as isize);
        *bin_size =
            (::std::mem::size_of::<rite_binary_header>() as
                 libc::c_ulong).wrapping_add(section_irep_size).wrapping_add(section_lineno_size).wrapping_add(section_lv_size).wrapping_add(::std::mem::size_of::<rite_binary_footer>()
                                                                                                                                                 as
                                                                                                                                                 libc::c_ulong);
        /* write DEBUG section */
        if 0 != flags as libc::c_int & 1i32 {
            if 0 != debug_info_defined {
                result =
                    write_section_debug(mrb, irep, cur, filenames,
                                        filenames_len);
                if result != 0i32 {
                    current_block = 16012914581245700207;
                } else { current_block = 11385396242402735691; }
            } else { current_block = 11385396242402735691; }
            match current_block {
                16012914581245700207 => { }
                _ => {
                    cur = cur.offset(section_lineno_size as isize);
                    current_block = 10692455896603418738;
                }
            }
        } else { current_block = 10692455896603418738; }
        match current_block {
            16012914581245700207 => { }
            _ => {
                if 0 != lv_defined {
                    result =
                        write_section_lv(mrb, irep, cur, lv_syms,
                                         lv_syms_len);
                    if result != 0i32 {
                        current_block = 16012914581245700207;
                    } else {
                        cur = cur.offset(section_lv_size as isize);
                        current_block = 572715077006366937;
                    }
                } else { current_block = 572715077006366937; }
                match current_block {
                    16012914581245700207 => { }
                    _ => {
                        write_footer(mrb, cur);
                        write_rite_binary_header(mrb, *bin_size, *bin, flags);
                    }
                }
            }
        }
    }
    if result != 0i32 {
        mrb_free(mrb, *bin as *mut libc::c_void);
        *bin = 0 as *mut uint8_t
    }
    mrb_free(mrb, lv_syms as *mut libc::c_void);
    mrb_free(mrb, filenames as *mut libc::c_void);
    return result;
}
unsafe extern "C" fn write_rite_binary_header(mut mrb: *mut mrb_state,
                                              mut binary_size: size_t,
                                              mut bin: *mut uint8_t,
                                              mut flags: uint8_t)
 -> libc::c_int {
    let mut header: *mut rite_binary_header = bin as *mut rite_binary_header;
    let mut crc: uint16_t = 0;
    let mut offset: uint32_t = 0;
    let mut current_block_1: u64;
    match flags as libc::c_int & 6i32 {
        2 => { current_block_1 = 507007278313083068; }
        4 => { current_block_1 = 5676631007156301930; }
        6 => {
            if 0 != bigendian_p() {
                current_block_1 = 507007278313083068;
            } else { current_block_1 = 5676631007156301930; }
        }
        _ => { current_block_1 = 3276175668257526147; }
    }
    match current_block_1 {
        507007278313083068 => {
            memcpy((*header).binary_ident.as_mut_ptr() as *mut libc::c_void,
                   b"RITE\x00" as *const u8 as *const libc::c_char as
                       *const libc::c_void,
                   ::std::mem::size_of::<[uint8_t; 4]>() as libc::c_ulong);
        }
        5676631007156301930 => {
            memcpy((*header).binary_ident.as_mut_ptr() as *mut libc::c_void,
                   b"ETIR\x00" as *const u8 as *const libc::c_char as
                       *const libc::c_void,
                   ::std::mem::size_of::<[uint8_t; 4]>() as libc::c_ulong);
        }
        _ => { }
    }
    memcpy((*header).binary_version.as_mut_ptr() as *mut libc::c_void,
           b"0006\x00" as *const u8 as *const libc::c_char as
               *const libc::c_void,
           ::std::mem::size_of::<[uint8_t; 4]>() as libc::c_ulong);
    memcpy((*header).compiler_name.as_mut_ptr() as *mut libc::c_void,
           b"MATZ\x00" as *const u8 as *const libc::c_char as
               *const libc::c_void,
           ::std::mem::size_of::<[uint8_t; 4]>() as libc::c_ulong);
    memcpy((*header).compiler_version.as_mut_ptr() as *mut libc::c_void,
           b"0000\x00" as *const u8 as *const libc::c_char as
               *const libc::c_void,
           ::std::mem::size_of::<[uint8_t; 4]>() as libc::c_ulong);
    uint32_to_bin(binary_size as uint32_t,
                  (*header).binary_size.as_mut_ptr());
    offset =
        ((&mut *(*header).binary_crc.as_mut_ptr().offset(0isize) as
              *mut uint8_t).wrapping_offset_from(bin) as libc::c_long as
             libc::c_ulong).wrapping_add(::std::mem::size_of::<uint16_t>() as
                                             libc::c_ulong) as uint32_t;
    crc =
        calc_crc_16_ccitt(bin.offset(offset as isize),
                          binary_size.wrapping_sub(offset as libc::c_ulong),
                          0i32 as uint16_t);
    uint16_to_bin(crc, (*header).binary_crc.as_mut_ptr());
    return 0i32;
}
#[inline]
unsafe extern "C" fn uint16_to_bin(mut s: uint16_t, mut bin: *mut uint8_t)
 -> size_t {
    let fresh0 = bin;
    bin = bin.offset(1);
    *fresh0 = (s as libc::c_int >> 8i32 & 0xffi32) as uint8_t;
    *bin = (s as libc::c_int & 0xffi32) as uint8_t;
    return ::std::mem::size_of::<uint16_t>() as libc::c_ulong;
}
#[inline]
unsafe extern "C" fn uint32_to_bin(mut l: uint32_t, mut bin: *mut uint8_t)
 -> size_t {
    let fresh1 = bin;
    bin = bin.offset(1);
    *fresh1 = (l >> 24i32 & 0xffi32 as libc::c_uint) as uint8_t;
    let fresh2 = bin;
    bin = bin.offset(1);
    *fresh2 = (l >> 16i32 & 0xffi32 as libc::c_uint) as uint8_t;
    let fresh3 = bin;
    bin = bin.offset(1);
    *fresh3 = (l >> 8i32 & 0xffi32 as libc::c_uint) as uint8_t;
    *bin = (l & 0xffi32 as libc::c_uint) as uint8_t;
    return ::std::mem::size_of::<uint32_t>() as libc::c_ulong;
}
#[inline]
unsafe extern "C" fn bigendian_p() -> libc::c_int {
    let mut i: libc::c_int = 0;
    let mut p: *mut libc::c_char = 0 as *mut libc::c_char;
    i = 1i32;
    p = &mut i as *mut libc::c_int as *mut libc::c_char;
    return if 0 != *p.offset(0isize) as libc::c_int { 0i32 } else { 1i32 };
}
unsafe extern "C" fn write_footer(mut mrb: *mut mrb_state,
                                  mut bin: *mut uint8_t) -> uint32_t {
    let mut footer: rite_binary_footer =
        rite_binary_footer{section_ident: [0; 4], section_size: [0; 4],};
    memcpy(footer.section_ident.as_mut_ptr() as *mut libc::c_void,
           b"END\x00\x00" as *const u8 as *const libc::c_char as
               *const libc::c_void,
           ::std::mem::size_of::<[uint8_t; 4]>() as libc::c_ulong);
    uint32_to_bin(::std::mem::size_of::<rite_binary_footer>() as libc::c_ulong
                      as uint32_t, footer.section_size.as_mut_ptr());
    memcpy(bin as *mut libc::c_void,
           &mut footer as *mut rite_binary_footer as *const libc::c_void,
           ::std::mem::size_of::<rite_binary_footer>() as libc::c_ulong);
    return ::std::mem::size_of::<rite_binary_footer>() as libc::c_ulong as
               uint32_t;
}
unsafe extern "C" fn write_section_lv(mut mrb: *mut mrb_state,
                                      mut irep: *mut mrb_irep,
                                      mut start: *mut uint8_t,
                                      mut syms: *const mrb_sym,
                                      syms_len: uint32_t) -> libc::c_int {
    let mut cur: *mut uint8_t = start;
    let mut header: *mut rite_section_lv_header =
        0 as *mut rite_section_lv_header;
    let mut diff: ptrdiff_t = 0;
    let mut result: libc::c_int = 0i32;
    if mrb.is_null() || cur.is_null() { return -7i32 }
    header = cur as *mut rite_section_lv_header;
    cur =
        cur.offset(::std::mem::size_of::<rite_section_lv_header>() as
                       libc::c_ulong as isize);
    result = write_lv_sym_table(mrb, &mut cur, syms, syms_len);
    if !(result != 0i32) {
        result = write_lv_record(mrb, irep, &mut cur, syms, syms_len);
        if !(result != 0i32) {
            memcpy((*header).section_ident.as_mut_ptr() as *mut libc::c_void,
                   b"LVAR\x00" as *const u8 as *const libc::c_char as
                       *const libc::c_void,
                   ::std::mem::size_of::<[uint8_t; 4]>() as libc::c_ulong);
            diff = cur.wrapping_offset_from(start) as libc::c_long;
            uint32_to_bin(diff as uint32_t,
                          (*header).section_size.as_mut_ptr());
        }
    }
    return result;
}
unsafe extern "C" fn write_lv_record(mut mrb: *mut mrb_state,
                                     mut irep: *const mrb_irep,
                                     mut start: *mut *mut uint8_t,
                                     mut syms: *const mrb_sym,
                                     mut syms_len: uint32_t) -> libc::c_int {
    let mut cur: *mut uint8_t = *start;
    let mut i: libc::c_int = 0;
    i = 0i32;
    while i + 1i32 < (*irep).nlocals as libc::c_int {
        if (*(*irep).lv.offset(i as isize)).name == 0i32 as libc::c_uint {
            cur =
                cur.offset(uint16_to_bin(65535i32 as uint16_t, cur) as isize);
            cur = cur.offset(uint16_to_bin(0i32 as uint16_t, cur) as isize)
        } else {
            let sym_idx: libc::c_int =
                find_filename_index(syms, syms_len as libc::c_int,
                                    (*(*irep).lv.offset(i as isize)).name);
            cur =
                cur.offset(uint16_to_bin(sym_idx as uint16_t, cur) as isize);
            cur =
                cur.offset(uint16_to_bin((*(*irep).lv.offset(i as isize)).r,
                                         cur) as isize)
        }
        i += 1
    }
    i = 0i32;
    while i < (*irep).rlen as libc::c_int {
        write_lv_record(mrb, *(*irep).reps.offset(i as isize), &mut cur, syms,
                        syms_len);
        i += 1
    }
    *start = cur;
    return 0i32;
}
unsafe extern "C" fn find_filename_index(mut ary: *const mrb_sym,
                                         mut ary_len: libc::c_int,
                                         mut s: mrb_sym) -> libc::c_int {
    let mut i: libc::c_int = 0;
    i = 0i32;
    while i < ary_len { if *ary.offset(i as isize) == s { return i } i += 1 }
    return -1i32;
}
unsafe extern "C" fn write_lv_sym_table(mut mrb: *mut mrb_state,
                                        mut start: *mut *mut uint8_t,
                                        mut syms: *const mrb_sym,
                                        mut syms_len: uint32_t)
 -> libc::c_int {
    let mut cur: *mut uint8_t = *start;
    let mut i: uint32_t = 0;
    let mut str: *const libc::c_char = 0 as *const libc::c_char;
    let mut str_len: mrb_int = 0;
    cur = cur.offset(uint32_to_bin(syms_len, cur) as isize);
    i = 0i32 as uint32_t;
    while i < syms_len {
        str = mrb_sym2name_len(mrb, *syms.offset(i as isize), &mut str_len);
        cur = cur.offset(uint16_to_bin(str_len as uint16_t, cur) as isize);
        memcpy(cur as *mut libc::c_void, str as *const libc::c_void,
               str_len as libc::c_ulong);
        cur = cur.offset(str_len as isize);
        i = i.wrapping_add(1)
    }
    *start = cur;
    return 0i32;
}
unsafe extern "C" fn lv_defined_p(mut irep: *mut mrb_irep) -> mrb_bool {
    let mut i: libc::c_int = 0;
    if !(*irep).lv.is_null() { return 1i32 as mrb_bool }
    i = 0i32;
    while i < (*irep).rlen as libc::c_int {
        if 0 != lv_defined_p(*(*irep).reps.offset(i as isize)) {
            return 1i32 as mrb_bool
        }
        i += 1
    }
    return 0i32 as mrb_bool;
}
unsafe extern "C" fn write_section_debug(mut mrb: *mut mrb_state,
                                         mut irep: *mut mrb_irep,
                                         mut cur: *mut uint8_t,
                                         mut filenames: *const mrb_sym,
                                         mut filenames_len: uint16_t)
 -> libc::c_int {
    let mut section_size: size_t = 0i32 as size_t;
    let mut bin: *const uint8_t = cur;
    let mut header: *mut rite_section_debug_header =
        0 as *mut rite_section_debug_header;
    let mut dlen: size_t = 0;
    let mut i: uint16_t = 0;
    let mut sym: *const libc::c_char = 0 as *const libc::c_char;
    let mut sym_len: mrb_int = 0;
    if mrb.is_null() || cur.is_null() { return -7i32 }
    header = bin as *mut rite_section_debug_header;
    cur =
        cur.offset(::std::mem::size_of::<rite_section_debug_header>() as
                       libc::c_ulong as isize);
    section_size =
        (section_size as
             libc::c_ulong).wrapping_add(::std::mem::size_of::<rite_section_debug_header>()
                                             as libc::c_ulong) as size_t as
            size_t;
    cur = cur.offset(uint16_to_bin(filenames_len, cur) as isize);
    section_size =
        (section_size as
             libc::c_ulong).wrapping_add(::std::mem::size_of::<uint16_t>() as
                                             libc::c_ulong) as size_t as
            size_t;
    i = 0i32 as uint16_t;
    while (i as libc::c_int) < filenames_len as libc::c_int {
        sym =
            mrb_sym2name_len(mrb, *filenames.offset(i as isize),
                             &mut sym_len);
        cur = cur.offset(uint16_to_bin(sym_len as uint16_t, cur) as isize);
        memcpy(cur as *mut libc::c_void, sym as *const libc::c_void,
               sym_len as libc::c_ulong);
        cur = cur.offset(sym_len as isize);
        section_size =
            (section_size as
                 libc::c_ulonglong).wrapping_add((::std::mem::size_of::<uint16_t>()
                                                      as libc::c_ulong as
                                                      libc::c_ulonglong).wrapping_add(sym_len
                                                                                          as
                                                                                          libc::c_ulonglong))
                as size_t as size_t;
        i = i.wrapping_add(1)
    }
    dlen = write_debug_record(mrb, irep, cur, filenames, filenames_len);
    section_size =
        (section_size as libc::c_ulong).wrapping_add(dlen) as size_t as
            size_t;
    memcpy((*header).section_ident.as_mut_ptr() as *mut libc::c_void,
           b"DBG\x00\x00" as *const u8 as *const libc::c_char as
               *const libc::c_void,
           ::std::mem::size_of::<[uint8_t; 4]>() as libc::c_ulong);
    uint32_to_bin(section_size as uint32_t,
                  (*header).section_size.as_mut_ptr());
    return 0i32;
}
unsafe extern "C" fn write_debug_record(mut mrb: *mut mrb_state,
                                        mut irep: *mut mrb_irep,
                                        mut bin: *mut uint8_t,
                                        mut filenames: *const mrb_sym,
                                        mut filenames_len: uint16_t)
 -> size_t {
    let mut size: size_t = 0;
    let mut len: size_t = 0;
    let mut irep_no: libc::c_int = 0;
    len = write_debug_record_1(mrb, irep, bin, filenames, filenames_len);
    size = len;
    bin = bin.offset(len as isize);
    irep_no = 0i32;
    while irep_no < (*irep).rlen as libc::c_int {
        len =
            write_debug_record(mrb, *(*irep).reps.offset(irep_no as isize),
                               bin, filenames, filenames_len);
        bin = bin.offset(len as isize);
        size = (size as libc::c_ulong).wrapping_add(len) as size_t as size_t;
        irep_no += 1
    }
    return size;
}
unsafe extern "C" fn write_debug_record_1(mut mrb: *mut mrb_state,
                                          mut irep: *mut mrb_irep,
                                          mut bin: *mut uint8_t,
                                          mut filenames: *const mrb_sym,
                                          mut filenames_len: uint16_t)
 -> size_t {
    let mut cur: *mut uint8_t = 0 as *mut uint8_t;
    let mut f_idx: uint16_t = 0;
    let mut ret: ptrdiff_t = 0;
    cur =
        bin.offset(::std::mem::size_of::<uint32_t>() as libc::c_ulong as
                       isize);
    cur = cur.offset(uint16_to_bin((*(*irep).debug_info).flen, cur) as isize);
    f_idx = 0i32 as uint16_t;
    while (f_idx as libc::c_int) < (*(*irep).debug_info).flen as libc::c_int {
        let mut filename_idx: libc::c_int = 0;
        let mut file: *const mrb_irep_debug_info_file =
            *(*(*irep).debug_info).files.offset(f_idx as isize);
        cur = cur.offset(uint32_to_bin((*file).start_pos, cur) as isize);
        filename_idx =
            find_filename_index(filenames, filenames_len as libc::c_int,
                                (*file).filename_sym);
        cur =
            cur.offset(uint16_to_bin(filename_idx as uint16_t, cur) as isize);
        cur =
            cur.offset(uint32_to_bin((*file).line_entry_count, cur) as isize);
        cur =
            cur.offset(uint8_to_bin((*file).line_type as uint8_t, cur) as
                           isize);
        match (*file).line_type as libc::c_uint {
            0 => {
                let mut l: uint32_t = 0;
                l = 0i32 as uint32_t;
                while l < (*file).line_entry_count {
                    cur =
                        cur.offset(uint16_to_bin(*(*file).lines.ary.offset(l
                                                                               as
                                                                               isize),
                                                 cur) as isize);
                    l = l.wrapping_add(1)
                }
            }
            1 => {
                let mut line: uint32_t = 0;
                line = 0i32 as uint32_t;
                while line < (*file).line_entry_count {
                    cur =
                        cur.offset(uint32_to_bin((*(*file).lines.flat_map.offset(line
                                                                                     as
                                                                                     isize)).start_pos,
                                                 cur) as isize);
                    cur =
                        cur.offset(uint16_to_bin((*(*file).lines.flat_map.offset(line
                                                                                     as
                                                                                     isize)).line,
                                                 cur) as isize);
                    line = line.wrapping_add(1)
                }
            }
            _ => { }
        }
        f_idx = f_idx.wrapping_add(1)
    }
    ret = cur.wrapping_offset_from(bin) as libc::c_long;
    uint32_to_bin(ret as uint32_t, bin);
    return ret as size_t;
}
#[inline]
unsafe extern "C" fn uint8_to_bin(mut s: uint8_t, mut bin: *mut uint8_t)
 -> size_t {
    *bin = s;
    return ::std::mem::size_of::<uint8_t>() as libc::c_ulong;
}
unsafe extern "C" fn debug_info_defined_p(mut irep: *mut mrb_irep)
 -> mrb_bool {
    let mut i: libc::c_int = 0;
    if (*irep).debug_info.is_null() { return 0i32 as mrb_bool }
    i = 0i32;
    while i < (*irep).rlen as libc::c_int {
        if 0 == debug_info_defined_p(*(*irep).reps.offset(i as isize)) {
            return 0i32 as mrb_bool
        }
        i += 1
    }
    return 1i32 as mrb_bool;
}
unsafe extern "C" fn write_section_irep(mut mrb: *mut mrb_state,
                                        mut irep: *mut mrb_irep,
                                        mut bin: *mut uint8_t,
                                        mut len_p: *mut size_t,
                                        mut flags: uint8_t) -> libc::c_int {
    let mut result: libc::c_int = 0;
    let mut rsize: size_t = 0i32 as size_t;
    let mut cur: *mut uint8_t = bin;
    if mrb.is_null() || bin.is_null() { return -7i32 }
    cur =
        cur.offset(::std::mem::size_of::<rite_section_irep_header>() as
                       libc::c_ulong as isize);
    result = write_irep_record(mrb, irep, cur, &mut rsize, flags);
    if result != 0i32 { return result }
    *len_p =
        (cur.wrapping_offset_from(bin) as libc::c_long as
             libc::c_ulong).wrapping_add(rsize);
    write_section_irep_header(mrb, *len_p, bin);
    return 0i32;
}
unsafe extern "C" fn write_section_irep_header(mut mrb: *mut mrb_state,
                                               mut section_size: size_t,
                                               mut bin: *mut uint8_t)
 -> libc::c_int {
    let mut header: *mut rite_section_irep_header =
        bin as *mut rite_section_irep_header;
    memcpy((*header).section_ident.as_mut_ptr() as *mut libc::c_void,
           b"IREP\x00" as *const u8 as *const libc::c_char as
               *const libc::c_void,
           ::std::mem::size_of::<[uint8_t; 4]>() as libc::c_ulong);
    uint32_to_bin(section_size as uint32_t,
                  (*header).section_size.as_mut_ptr());
    memcpy((*header).rite_version.as_mut_ptr() as *mut libc::c_void,
           b"0002\x00" as *const u8 as *const libc::c_char as
               *const libc::c_void,
           ::std::mem::size_of::<[uint8_t; 4]>() as libc::c_ulong);
    return 0i32;
}
unsafe extern "C" fn write_irep_record(mut mrb: *mut mrb_state,
                                       mut irep: *mut mrb_irep,
                                       mut bin: *mut uint8_t,
                                       mut irep_record_size: *mut size_t,
                                       mut flags: uint8_t) -> libc::c_int {
    let mut i: libc::c_int = 0;
    let mut src: *mut uint8_t = bin;
    if irep.is_null() { return -6i32 }
    *irep_record_size = get_irep_record_size_1(mrb, irep);
    if *irep_record_size == 0i32 as libc::c_ulong { return -1i32 }
    bin = bin.offset(write_irep_header(mrb, irep, bin) as isize);
    bin = bin.offset(write_iseq_block(mrb, irep, bin, flags) as isize);
    bin = bin.offset(write_pool_block(mrb, irep, bin) as isize);
    bin = bin.offset(write_syms_block(mrb, irep, bin) as isize);
    i = 0i32;
    while i < (*irep).rlen as libc::c_int {
        let mut result: libc::c_int = 0;
        let mut rsize: size_t = 0;
        result =
            write_irep_record(mrb, *(*irep).reps.offset(i as isize), bin,
                              &mut rsize, flags);
        if result != 0i32 { return result }
        bin = bin.offset(rsize as isize);
        i += 1
    }
    *irep_record_size =
        bin.wrapping_offset_from(src) as libc::c_long as size_t;
    return 0i32;
}
unsafe extern "C" fn write_syms_block(mut mrb: *mut mrb_state,
                                      mut irep: *mut mrb_irep,
                                      mut buf: *mut uint8_t) -> ptrdiff_t {
    let mut sym_no: libc::c_int = 0;
    let mut cur: *mut uint8_t = buf;
    let mut name: *const libc::c_char = 0 as *const libc::c_char;
    cur = cur.offset(uint32_to_bin((*irep).slen as uint32_t, cur) as isize);
    sym_no = 0i32;
    while sym_no < (*irep).slen as libc::c_int {
        if *(*irep).syms.offset(sym_no as isize) != 0i32 as libc::c_uint {
            let mut len: mrb_int = 0;
            name =
                mrb_sym2name_len(mrb, *(*irep).syms.offset(sym_no as isize),
                                 &mut len);
            cur = cur.offset(uint16_to_bin(len as uint16_t, cur) as isize);
            memcpy(cur as *mut libc::c_void, name as *const libc::c_void,
                   len as libc::c_ulong);
            cur = cur.offset(len as uint16_t as libc::c_int as isize);
            let fresh4 = cur;
            cur = cur.offset(1);
            *fresh4 = '\u{0}' as i32 as uint8_t
        } else {
            cur =
                cur.offset(uint16_to_bin(0xffffi32 as uint16_t, cur) as isize)
        }
        sym_no += 1
    }
    return cur.wrapping_offset_from(buf) as libc::c_long;
}
unsafe extern "C" fn write_pool_block(mut mrb: *mut mrb_state,
                                      mut irep: *mut mrb_irep,
                                      mut buf: *mut uint8_t) -> ptrdiff_t {
    let mut pool_no: libc::c_int = 0;
    let mut cur: *mut uint8_t = buf;
    let mut len: uint16_t = 0;
    let mut str: mrb_value =
        mrb_value{value: unnamed{f: 0.,}, tt: MRB_TT_FALSE,};
    let mut char_ptr: *const libc::c_char = 0 as *const libc::c_char;
    cur = cur.offset(uint32_to_bin((*irep).plen as uint32_t, cur) as isize);
    let mut current_block_15: u64;
    pool_no = 0i32;
    while pool_no < (*irep).plen as libc::c_int {
        let mut ai: libc::c_int = mrb_gc_arena_save(mrb);
        match (*(*irep).pool.offset(pool_no as isize)).tt as libc::c_uint {
            3 => {
                cur =
                    cur.offset(uint8_to_bin(IREP_TT_FIXNUM as libc::c_int as
                                                uint8_t, cur) as isize);
                str =
                    mrb_fixnum_to_str(mrb,
                                      *(*irep).pool.offset(pool_no as isize),
                                      10i32 as mrb_int);
                current_block_15 = 9606288038608642794;
            }
            6 => {
                cur =
                    cur.offset(uint8_to_bin(IREP_TT_FLOAT as libc::c_int as
                                                uint8_t, cur) as isize);
                str =
                    float_to_str(mrb, *(*irep).pool.offset(pool_no as isize));
                current_block_15 = 9606288038608642794;
            }
            16 => {
                cur =
                    cur.offset(uint8_to_bin(IREP_TT_STRING as libc::c_int as
                                                uint8_t, cur) as isize);
                str = *(*irep).pool.offset(pool_no as isize);
                current_block_15 = 9606288038608642794;
            }
            _ => { current_block_15 = 735147466149431745; }
        }
        match current_block_15 {
            9606288038608642794 => {
                char_ptr =
                    if 0 !=
                           (*(str.value.p as *mut RString)).flags() as
                               libc::c_int & 32i32 {
                        (*(str.value.p as *mut RString)).as_0.ary.as_mut_ptr()
                    } else { (*(str.value.p as *mut RString)).as_0.heap.ptr };
                let mut tlen: mrb_int =
                    if 0 !=
                           (*(str.value.p as *mut RString)).flags() as
                               libc::c_int & 32i32 {
                        (((*(str.value.p as *mut RString)).flags() as
                              libc::c_int & 0x7c0i32) >> 6i32) as mrb_int
                    } else { (*(str.value.p as *mut RString)).as_0.heap.len };
                len = tlen as uint16_t;
                cur = cur.offset(uint16_to_bin(len, cur) as isize);
                memcpy(cur as *mut libc::c_void,
                       char_ptr as *const libc::c_void, len as size_t);
                cur = cur.offset(len as libc::c_int as isize);
                mrb_gc_arena_restore(mrb, ai);
            }
            _ => { }
        }
        pool_no += 1
    }
    return cur.wrapping_offset_from(buf) as libc::c_long;
}
unsafe extern "C" fn float_to_str(mut mrb: *mut mrb_state, mut flt: mrb_value)
 -> mrb_value {
    let mut f: mrb_float = flt.value.f;
    if 0 !=
           if ::std::mem::size_of::<mrb_float>() as libc::c_ulong ==
                  ::std::mem::size_of::<libc::c_float>() as libc::c_ulong {
               __inline_isinff(f as libc::c_float)
           } else if ::std::mem::size_of::<mrb_float>() as libc::c_ulong ==
                         ::std::mem::size_of::<libc::c_double>() as
                             libc::c_ulong {
               __inline_isinfd(f as libc::c_double)
           } else { __inline_isinfl(f128::f128::new(f)) } {
        return if f < 0i32 as libc::c_double {
                   mrb_str_new_static(mrb,
                                      b"I\x00" as *const u8 as
                                          *const libc::c_char,
                                      (::std::mem::size_of::<[libc::c_char; 2]>()
                                           as
                                           libc::c_ulong).wrapping_sub(1i32 as
                                                                           libc::c_ulong))
               } else {
                   mrb_str_new_static(mrb,
                                      b"i\x00" as *const u8 as
                                          *const libc::c_char,
                                      (::std::mem::size_of::<[libc::c_char; 2]>()
                                           as
                                           libc::c_ulong).wrapping_sub(1i32 as
                                                                           libc::c_ulong))
               }
    }
    return mrb_float_to_str(mrb, flt,
                            b"%.17g\x00" as *const u8 as *const libc::c_char);
}
unsafe extern "C" fn write_iseq_block(mut mrb: *mut mrb_state,
                                      mut irep: *mut mrb_irep,
                                      mut buf: *mut uint8_t,
                                      mut flags: uint8_t) -> ptrdiff_t {
    let mut cur: *mut uint8_t = buf;
    cur = cur.offset(uint32_to_bin((*irep).ilen as uint32_t, cur) as isize);
    cur = cur.offset(write_padding(cur) as isize);
    memcpy(cur as *mut libc::c_void, (*irep).iseq as *const libc::c_void,
           ((*irep).ilen as
                libc::c_ulong).wrapping_mul(::std::mem::size_of::<mrb_code>()
                                                as libc::c_ulong));
    cur =
        cur.offset(((*irep).ilen as
                        libc::c_ulong).wrapping_mul(::std::mem::size_of::<mrb_code>()
                                                        as libc::c_ulong) as
                       isize);
    return cur.wrapping_offset_from(buf) as libc::c_long;
}
unsafe extern "C" fn write_padding(mut buf: *mut uint8_t) -> size_t {
    let align: size_t = ::std::mem::size_of::<uint32_t>() as libc::c_ulong;
    let mut pad_len: size_t =
        -(buf as intptr_t) as libc::c_ulong &
            align.wrapping_sub(1i32 as libc::c_ulong);
    if pad_len > 0i32 as libc::c_ulong {
        memset(buf as *mut libc::c_void, 0i32, pad_len);
    }
    return pad_len;
}
unsafe extern "C" fn write_irep_header(mut mrb: *mut mrb_state,
                                       mut irep: *mut mrb_irep,
                                       mut buf: *mut uint8_t) -> ptrdiff_t {
    let mut cur: *mut uint8_t = buf;
    cur =
        cur.offset(uint32_to_bin(get_irep_record_size_1(mrb, irep) as
                                     uint32_t, cur) as isize);
    cur =
        cur.offset(uint16_to_bin((*irep).nlocals as uint16_t, cur) as isize);
    cur = cur.offset(uint16_to_bin((*irep).nregs as uint16_t, cur) as isize);
    cur = cur.offset(uint16_to_bin((*irep).rlen as uint16_t, cur) as isize);
    return cur.wrapping_offset_from(buf) as libc::c_long;
}
/*
** dump.c - mruby binary dumper (mrbc binary format)
**
** See Copyright Notice in mruby.h
*/
unsafe extern "C" fn get_irep_record_size_1(mut mrb: *mut mrb_state,
                                            mut irep: *mut mrb_irep)
 -> size_t {
    let mut size: size_t = 0i32 as size_t;
    size =
        (size as libc::c_ulong).wrapping_add(get_irep_header_size(mrb)) as
            size_t as size_t;
    size =
        (size as libc::c_ulong).wrapping_add(get_iseq_block_size(mrb, irep))
            as size_t as size_t;
    size =
        (size as libc::c_ulong).wrapping_add(get_pool_block_size(mrb, irep))
            as size_t as size_t;
    size =
        (size as libc::c_ulong).wrapping_add(get_syms_block_size(mrb, irep))
            as size_t as size_t;
    return size;
}
unsafe extern "C" fn get_syms_block_size(mut mrb: *mut mrb_state,
                                         mut irep: *mut mrb_irep) -> size_t {
    let mut size: size_t = 0i32 as size_t;
    let mut sym_no: libc::c_int = 0;
    let mut len: mrb_int = 0;
    size =
        (size as
             libc::c_ulong).wrapping_add(::std::mem::size_of::<uint32_t>() as
                                             libc::c_ulong) as size_t as
            size_t;
    sym_no = 0i32;
    while sym_no < (*irep).slen as libc::c_int {
        size =
            (size as
                 libc::c_ulong).wrapping_add(::std::mem::size_of::<uint16_t>()
                                                 as libc::c_ulong) as size_t
                as size_t;
        if *(*irep).syms.offset(sym_no as isize) != 0i32 as libc::c_uint {
            mrb_sym2name_len(mrb, *(*irep).syms.offset(sym_no as isize),
                             &mut len);
            size =
                (size as
                     libc::c_ulonglong).wrapping_add((len +
                                                          1i32 as
                                                              libc::c_longlong)
                                                         as libc::c_ulonglong)
                    as size_t as size_t
        }
        sym_no += 1
    }
    return size;
}
unsafe extern "C" fn get_pool_block_size(mut mrb: *mut mrb_state,
                                         mut irep: *mut mrb_irep) -> size_t {
    let mut pool_no: libc::c_int = 0;
    let mut size: size_t = 0i32 as size_t;
    let mut str: mrb_value =
        mrb_value{value: unnamed{f: 0.,}, tt: MRB_TT_FALSE,};
    size =
        (size as
             libc::c_ulong).wrapping_add(::std::mem::size_of::<uint32_t>() as
                                             libc::c_ulong) as size_t as
            size_t;
    size =
        (size as
             libc::c_ulong).wrapping_add(((*irep).plen as
                                              libc::c_ulong).wrapping_mul((::std::mem::size_of::<uint8_t>()
                                                                               as
                                                                               libc::c_ulong).wrapping_add(::std::mem::size_of::<uint16_t>()
                                                                                                               as
                                                                                                               libc::c_ulong)))
            as size_t as size_t;
    pool_no = 0i32;
    while pool_no < (*irep).plen as libc::c_int {
        let mut ai: libc::c_int = mrb_gc_arena_save(mrb);
        match (*(*irep).pool.offset(pool_no as isize)).tt as libc::c_uint {
            3 => {
                str =
                    mrb_fixnum_to_str(mrb,
                                      *(*irep).pool.offset(pool_no as isize),
                                      10i32 as mrb_int);
                let mut len: mrb_int =
                    if 0 !=
                           (*(str.value.p as *mut RString)).flags() as
                               libc::c_int & 32i32 {
                        (((*(str.value.p as *mut RString)).flags() as
                              libc::c_int & 0x7c0i32) >> 6i32) as mrb_int
                    } else { (*(str.value.p as *mut RString)).as_0.heap.len };
                size =
                    (size as libc::c_ulong).wrapping_add(len as size_t) as
                        size_t as size_t
            }
            6 => {
                str =
                    float_to_str(mrb, *(*irep).pool.offset(pool_no as isize));
                let mut len_0: mrb_int =
                    if 0 !=
                           (*(str.value.p as *mut RString)).flags() as
                               libc::c_int & 32i32 {
                        (((*(str.value.p as *mut RString)).flags() as
                              libc::c_int & 0x7c0i32) >> 6i32) as mrb_int
                    } else { (*(str.value.p as *mut RString)).as_0.heap.len };
                size =
                    (size as libc::c_ulong).wrapping_add(len_0 as size_t) as
                        size_t as size_t
            }
            16 => {
                let mut len_1: mrb_int =
                    if 0 !=
                           (*((*(*irep).pool.offset(pool_no as isize)).value.p
                                  as *mut RString)).flags() as libc::c_int &
                               32i32 {
                        (((*((*(*irep).pool.offset(pool_no as isize)).value.p
                                 as *mut RString)).flags() as libc::c_int &
                              0x7c0i32) >> 6i32) as mrb_int
                    } else {
                        (*((*(*irep).pool.offset(pool_no as isize)).value.p as
                               *mut RString)).as_0.heap.len
                    };
                size =
                    (size as libc::c_ulong).wrapping_add(len_1 as size_t) as
                        size_t as size_t
            }
            _ => { }
        }
        mrb_gc_arena_restore(mrb, ai);
        pool_no += 1
    }
    return size;
}
unsafe extern "C" fn get_iseq_block_size(mut mrb: *mut mrb_state,
                                         mut irep: *mut mrb_irep) -> size_t {
    let mut size: size_t = 0i32 as size_t;
    size =
        (size as
             libc::c_ulong).wrapping_add(::std::mem::size_of::<uint32_t>() as
                                             libc::c_ulong) as size_t as
            size_t;
    size =
        (size as
             libc::c_ulong).wrapping_add(::std::mem::size_of::<uint32_t>() as
                                             libc::c_ulong) as size_t as
            size_t;
    size =
        (size as
             libc::c_ulong).wrapping_add((::std::mem::size_of::<uint32_t>() as
                                              libc::c_ulong).wrapping_mul((*irep).ilen
                                                                              as
                                                                              libc::c_ulong))
            as size_t as size_t;
    return size;
}
unsafe extern "C" fn get_irep_header_size(mut mrb: *mut mrb_state) -> size_t {
    let mut size: size_t = 0i32 as size_t;
    size =
        (size as
             libc::c_ulong).wrapping_add((::std::mem::size_of::<uint32_t>() as
                                              libc::c_ulong).wrapping_mul(1i32
                                                                              as
                                                                              libc::c_ulong))
            as size_t as size_t;
    size =
        (size as
             libc::c_ulong).wrapping_add((::std::mem::size_of::<uint16_t>() as
                                              libc::c_ulong).wrapping_mul(3i32
                                                                              as
                                                                              libc::c_ulong))
            as size_t as size_t;
    return size;
}
unsafe extern "C" fn get_lv_section_size(mut mrb: *mut mrb_state,
                                         mut irep: *mut mrb_irep,
                                         mut syms: *const mrb_sym,
                                         mut syms_len: uint32_t) -> size_t {
    let mut ret: size_t = 0i32 as size_t;
    let mut i: size_t = 0;
    ret =
        (ret as
             libc::c_ulong).wrapping_add(::std::mem::size_of::<uint32_t>() as
                                             libc::c_ulong) as size_t as
            size_t;
    ret =
        (ret as
             libc::c_ulong).wrapping_add((::std::mem::size_of::<uint16_t>() as
                                              libc::c_ulong).wrapping_mul(syms_len
                                                                              as
                                                                              libc::c_ulong))
            as size_t as size_t;
    i = 0i32 as size_t;
    while i < syms_len as libc::c_ulong {
        let mut str_len: mrb_int = 0;
        mrb_sym2name_len(mrb, *syms.offset(i as isize), &mut str_len);
        ret =
            (ret as
                 libc::c_ulonglong).wrapping_add(str_len as libc::c_ulonglong)
                as size_t as size_t;
        i = i.wrapping_add(1)
    }
    ret =
        (ret as libc::c_ulong).wrapping_add(get_lv_record_size(mrb, irep)) as
            size_t as size_t;
    return ret;
}
unsafe extern "C" fn get_lv_record_size(mut mrb: *mut mrb_state,
                                        mut irep: *mut mrb_irep) -> size_t {
    let mut ret: size_t = 0i32 as size_t;
    let mut i: libc::c_int = 0;
    ret =
        (ret as
             libc::c_ulong).wrapping_add((::std::mem::size_of::<uint16_t>() as
                                              libc::c_ulong).wrapping_add(::std::mem::size_of::<uint16_t>()
                                                                              as
                                                                              libc::c_ulong).wrapping_mul(((*irep).nlocals
                                                                                                               as
                                                                                                               libc::c_int
                                                                                                               -
                                                                                                               1i32)
                                                                                                              as
                                                                                                              libc::c_ulong))
            as size_t as size_t;
    i = 0i32;
    while i < (*irep).rlen as libc::c_int {
        ret =
            (ret as
                 libc::c_ulong).wrapping_add(get_lv_record_size(mrb,
                                                                *(*irep).reps.offset(i
                                                                                         as
                                                                                         isize)))
                as size_t as size_t;
        i += 1
    }
    return ret;
}
unsafe extern "C" fn create_lv_sym_table(mut mrb: *mut mrb_state,
                                         mut irep: *const mrb_irep,
                                         mut syms: *mut *mut mrb_sym,
                                         mut syms_len: *mut uint32_t) {
    let mut i: libc::c_int = 0;
    if (*syms).is_null() {
        *syms =
            mrb_malloc(mrb,
                       (::std::mem::size_of::<mrb_sym>() as
                            libc::c_ulong).wrapping_mul(1i32 as
                                                            libc::c_ulong)) as
                *mut mrb_sym
    }
    i = 0i32;
    while i + 1i32 < (*irep).nlocals as libc::c_int {
        let name: mrb_sym = (*(*irep).lv.offset(i as isize)).name;
        if !(name == 0i32 as libc::c_uint) {
            if !(find_filename_index(*syms, *syms_len as libc::c_int, name) !=
                     -1i32) {
                *syms_len = (*syms_len).wrapping_add(1);
                *syms =
                    mrb_realloc(mrb, *syms as *mut libc::c_void,
                                (::std::mem::size_of::<mrb_sym>() as
                                     libc::c_ulong).wrapping_mul(*syms_len as
                                                                     libc::c_ulong))
                        as *mut mrb_sym;
                *(*syms).offset((*syms_len).wrapping_sub(1i32 as libc::c_uint)
                                    as isize) = name
            }
        }
        i += 1
    }
    i = 0i32;
    while i < (*irep).rlen as libc::c_int {
        create_lv_sym_table(mrb, *(*irep).reps.offset(i as isize), syms,
                            syms_len);
        i += 1
    };
}
unsafe extern "C" fn get_debug_record_size(mut mrb: *mut mrb_state,
                                           mut irep: *mut mrb_irep)
 -> size_t {
    let mut ret: size_t = 0i32 as size_t;
    let mut f_idx: uint16_t = 0;
    let mut i: libc::c_int = 0;
    ret =
        (ret as
             libc::c_ulong).wrapping_add(::std::mem::size_of::<uint32_t>() as
                                             libc::c_ulong) as size_t as
            size_t;
    ret =
        (ret as
             libc::c_ulong).wrapping_add(::std::mem::size_of::<uint16_t>() as
                                             libc::c_ulong) as size_t as
            size_t;
    f_idx = 0i32 as uint16_t;
    while (f_idx as libc::c_int) < (*(*irep).debug_info).flen as libc::c_int {
        let mut file: *const mrb_irep_debug_info_file =
            *(*(*irep).debug_info).files.offset(f_idx as isize);
        ret =
            (ret as
                 libc::c_ulong).wrapping_add(::std::mem::size_of::<uint32_t>()
                                                 as libc::c_ulong) as size_t
                as size_t;
        ret =
            (ret as
                 libc::c_ulong).wrapping_add(::std::mem::size_of::<uint16_t>()
                                                 as libc::c_ulong) as size_t
                as size_t;
        ret =
            (ret as
                 libc::c_ulong).wrapping_add(::std::mem::size_of::<uint32_t>()
                                                 as libc::c_ulong) as size_t
                as size_t;
        ret =
            (ret as
                 libc::c_ulong).wrapping_add(::std::mem::size_of::<uint8_t>()
                                                 as libc::c_ulong) as size_t
                as size_t;
        match (*file).line_type as libc::c_uint {
            0 => {
                ret =
                    (ret as
                         libc::c_ulong).wrapping_add((::std::mem::size_of::<uint16_t>()
                                                          as
                                                          libc::c_ulong).wrapping_mul((*file).line_entry_count
                                                                                          as
                                                                                          size_t))
                        as size_t as size_t
            }
            1 => {
                ret =
                    (ret as
                         libc::c_ulong).wrapping_add((::std::mem::size_of::<uint32_t>()
                                                          as
                                                          libc::c_ulong).wrapping_add(::std::mem::size_of::<uint16_t>()
                                                                                          as
                                                                                          libc::c_ulong).wrapping_mul((*file).line_entry_count
                                                                                                                          as
                                                                                                                          size_t))
                        as size_t as size_t
            }
            _ => { }
        }
        f_idx = f_idx.wrapping_add(1)
    }
    i = 0i32;
    while i < (*irep).rlen as libc::c_int {
        ret =
            (ret as
                 libc::c_ulong).wrapping_add(get_debug_record_size(mrb,
                                                                   *(*irep).reps.offset(i
                                                                                            as
                                                                                            isize)))
                as size_t as size_t;
        i += 1
    }
    return ret;
}
unsafe extern "C" fn get_filename_table_size(mut mrb: *mut mrb_state,
                                             mut irep: *mut mrb_irep,
                                             mut fp: *mut *mut mrb_sym,
                                             mut lp: *mut uint16_t)
 -> size_t {
    let mut filenames: *mut mrb_sym = *fp;
    let mut size: size_t = 0i32 as size_t;
    let mut di: *mut mrb_irep_debug_info = (*irep).debug_info;
    let mut i: libc::c_int = 0;
    i = 0i32;
    while i < (*di).flen as libc::c_int {
        let mut file: *mut mrb_irep_debug_info_file =
            0 as *mut mrb_irep_debug_info_file;
        let mut filename_len: mrb_int = 0;
        file = *(*di).files.offset(i as isize);
        if find_filename_index(filenames, *lp as libc::c_int,
                               (*file).filename_sym) == -1i32 {
            *lp = (*lp as libc::c_int + 1i32) as uint16_t;
            filenames =
                mrb_realloc(mrb, filenames as *mut libc::c_void,
                            (::std::mem::size_of::<mrb_sym>() as
                                 libc::c_ulong).wrapping_mul(*lp as
                                                                 libc::c_ulong))
                    as *mut mrb_sym;
            *fp = filenames;
            *filenames.offset((*lp as libc::c_int - 1i32) as isize) =
                (*file).filename_sym;
            mrb_sym2name_len(mrb, (*file).filename_sym, &mut filename_len);
            size =
                (size as
                     libc::c_ulong).wrapping_add((::std::mem::size_of::<uint16_t>()
                                                      as
                                                      libc::c_ulong).wrapping_add(filename_len
                                                                                      as
                                                                                      size_t))
                    as size_t as size_t
        }
        i += 1
    }
    i = 0i32;
    while i < (*irep).rlen as libc::c_int {
        size =
            (size as
                 libc::c_ulong).wrapping_add(get_filename_table_size(mrb,
                                                                     *(*irep).reps.offset(i
                                                                                              as
                                                                                              isize),
                                                                     fp, lp))
                as size_t as size_t;
        i += 1
    }
    return size;
}
unsafe extern "C" fn get_irep_record_size(mut mrb: *mut mrb_state,
                                          mut irep: *mut mrb_irep) -> size_t {
    let mut size: size_t = 0i32 as size_t;
    let mut irep_no: libc::c_int = 0;
    size = get_irep_record_size_1(mrb, irep);
    irep_no = 0i32;
    while irep_no < (*irep).rlen as libc::c_int {
        size =
            (size as
                 libc::c_ulong).wrapping_add(get_irep_record_size(mrb,
                                                                  *(*irep).reps.offset(irep_no
                                                                                           as
                                                                                           isize)))
                as size_t as size_t;
        irep_no += 1
    }
    return size;
}
#[no_mangle]
pub unsafe extern "C" fn mrb_dump_irep_binary(mut mrb: *mut mrb_state,
                                              mut irep: *mut mrb_irep,
                                              mut flags: uint8_t,
                                              mut fp: *mut FILE)
 -> libc::c_int {
    let mut bin: *mut uint8_t = 0 as *mut uint8_t;
    let mut bin_size: size_t = 0i32 as size_t;
    let mut result: libc::c_int = 0;
    if fp.is_null() { return -7i32 }
    result =
        dump_irep(mrb, irep, dump_flags(flags, 0i32 as uint8_t), &mut bin,
                  &mut bin_size);
    if result == 0i32 {
        if fwrite(bin as *const libc::c_void,
                  ::std::mem::size_of::<uint8_t>() as libc::c_ulong, bin_size,
                  fp) != bin_size {
            result = -2i32
        }
    }
    mrb_free(mrb, bin as *mut libc::c_void);
    return result;
}
#[no_mangle]
pub unsafe extern "C" fn mrb_dump_irep_cfunc(mut mrb: *mut mrb_state,
                                             mut irep: *mut mrb_irep,
                                             mut flags: uint8_t,
                                             mut fp: *mut FILE,
                                             mut initname:
                                                 *const libc::c_char)
 -> libc::c_int {
    let mut bin: *mut uint8_t = 0 as *mut uint8_t;
    let mut bin_size: size_t = 0i32 as size_t;
    let mut bin_idx: size_t = 0i32 as size_t;
    let mut result: libc::c_int = 0;
    if fp.is_null() || initname.is_null() ||
           *initname.offset(0isize) as libc::c_int == '\u{0}' as i32 {
        return -7i32
    }
    flags = dump_flags(flags, 2i32 as uint8_t);
    result = dump_irep(mrb, irep, flags, &mut bin, &mut bin_size);
    if result == 0i32 {
        if 0 == dump_bigendian_p(flags) {
            if fprintf(fp,
                       b"/* dumped in little endian order.\n   use `mrbc -E` option for big endian CPU. */\n\x00"
                           as *const u8 as *const libc::c_char) < 0i32 {
                mrb_free(mrb, bin as *mut libc::c_void);
                return -2i32
            }
        } else if fprintf(fp,
                          b"/* dumped in big endian order.\n   use `mrbc -e` option for better performance on little endian CPU. */\n\x00"
                              as *const u8 as *const libc::c_char) < 0i32 {
            mrb_free(mrb, bin as *mut libc::c_void);
            return -2i32
        }
        if fprintf(fp,
                   b"#include <stdint.h>\n\x00" as *const u8 as
                       *const libc::c_char) < 0i32 {
            mrb_free(mrb, bin as *mut libc::c_void);
            return -2i32
        }
        if fprintf(fp,
                   b"extern const uint8_t %s[];\nconst uint8_t\n#if defined __GNUC__\n__attribute__((aligned(%u)))\n#elif defined _MSC_VER\n__declspec(align(%u))\n#endif\n%s[] = {\x00"
                       as *const u8 as *const libc::c_char, initname,
                   ::std::mem::size_of::<uint32_t>() as libc::c_ulong as
                       uint16_t as libc::c_int,
                   ::std::mem::size_of::<uint32_t>() as libc::c_ulong as
                       uint16_t as libc::c_int, initname) < 0i32 {
            mrb_free(mrb, bin as *mut libc::c_void);
            return -2i32
        }
        while bin_idx < bin_size {
            if bin_idx.wrapping_rem(16i32 as libc::c_ulong) ==
                   0i32 as libc::c_ulong {
                if fputs(b"\n\x00" as *const u8 as *const libc::c_char, fp) ==
                       -1i32 {
                    mrb_free(mrb, bin as *mut libc::c_void);
                    return -2i32
                }
            }
            let fresh5 = bin_idx;
            bin_idx = bin_idx.wrapping_add(1);
            if fprintf(fp, b"0x%02x,\x00" as *const u8 as *const libc::c_char,
                       *bin.offset(fresh5 as isize) as libc::c_int) < 0i32 {
                mrb_free(mrb, bin as *mut libc::c_void);
                return -2i32
            }
        }
        if fputs(b"\n};\n\x00" as *const u8 as *const libc::c_char, fp) ==
               -1i32 {
            mrb_free(mrb, bin as *mut libc::c_void);
            return -2i32
        }
    }
    mrb_free(mrb, bin as *mut libc::c_void);
    return result;
}
unsafe extern "C" fn dump_bigendian_p(mut flags: uint8_t) -> mrb_bool {
    match flags as libc::c_int & 6i32 {
        2 => { return 1i32 as mrb_bool }
        4 => { return 0i32 as mrb_bool }
        6 | _ => { return bigendian_p() as mrb_bool }
    };
}