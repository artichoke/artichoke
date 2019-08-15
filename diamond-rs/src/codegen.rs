use libc;
use c2rust_bitfields::BitfieldStruct;
extern "C" {
    pub type iv_tbl;
    pub type RClass;
    pub type symbol_name;
    /* memory pool implementation */
    pub type mrb_pool;
    pub type mrb_shared_string;
    #[no_mangle]
    fn __tolower(_: __darwin_ct_rune_t) -> __darwin_ct_rune_t;
    #[no_mangle]
    fn memcmp(_: *const libc::c_void, _: *const libc::c_void,
              _: libc::c_ulong) -> libc::c_int;
    #[no_mangle]
    fn memcpy(_: *mut libc::c_void, _: *const libc::c_void, _: libc::c_ulong)
     -> *mut libc::c_void;
    #[no_mangle]
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    #[no_mangle]
    fn mrb_intern(_: *mut mrb_state, _: *const libc::c_char, _: size_t)
     -> mrb_sym;
    #[no_mangle]
    fn mrb_intern_static(_: *mut mrb_state, _: *const libc::c_char, _: size_t)
     -> mrb_sym;
    #[no_mangle]
    fn mrb_format(mrb: *mut mrb_state, format: *const libc::c_char, _: ...)
     -> mrb_value;
    #[no_mangle]
    fn mrb_float_read(_: *const libc::c_char, _: *mut *mut libc::c_char)
     -> libc::c_double;
    #[no_mangle]
    fn mrb_pool_alloc(_: *mut mrb_pool, _: size_t) -> *mut libc::c_void;
    #[no_mangle]
    fn mrb_intern_str(_: *mut mrb_state, _: mrb_value) -> mrb_sym;
    #[no_mangle]
    fn mrb_sym2name_len(_: *mut mrb_state, _: mrb_sym, _: *mut mrb_int)
     -> *const libc::c_char;
    #[no_mangle]
    fn mrb_malloc(_: *mut mrb_state, _: size_t) -> *mut libc::c_void;
    #[no_mangle]
    fn mrb_realloc_simple(_: *mut mrb_state, _: *mut libc::c_void, _: size_t)
     -> *mut libc::c_void;
    #[no_mangle]
    fn mrb_free(_: *mut mrb_state, _: *mut libc::c_void);
    #[no_mangle]
    fn mrb_str_new(mrb: *mut mrb_state, p: *const libc::c_char, len: size_t)
     -> mrb_value;
    /* *
 * Turns a C string into a Ruby string value.
 */
    #[no_mangle]
    fn mrb_str_new_cstr(_: *mut mrb_state, _: *const libc::c_char)
     -> mrb_value;
    #[no_mangle]
    fn mrb_pool_open(_: *mut mrb_state) -> *mut mrb_pool;
    #[no_mangle]
    fn mrb_pool_close(_: *mut mrb_pool);
    #[no_mangle]
    fn mrb_parser_get_filename(_: *mut mrb_parser_state, idx: uint16_t)
     -> mrb_sym;
    #[no_mangle]
    fn mrb_add_irep(mrb: *mut mrb_state) -> *mut mrb_irep;
    #[no_mangle]
    fn mrb_irep_decref(_: *mut mrb_state, _: *mut mrb_irep);
    /* aspec access */
    #[no_mangle]
    fn mrb_proc_new(_: *mut mrb_state, _: *mut mrb_irep) -> *mut RProc;
    /*
** mruby/string.h - String class
**
** See Copyright Notice in mruby.h
*/
    /* *
 * String class
 */
    #[no_mangle]
    static mrb_digitmap: [libc::c_char; 0];
    #[no_mangle]
    fn mrb_str_pool(mrb: *mut mrb_state, str: mrb_value) -> mrb_value;
    #[no_mangle]
    fn mrb_debug_info_alloc(mrb: *mut mrb_state, irep: *mut mrb_irep)
     -> *mut mrb_irep_debug_info;
    #[no_mangle]
    fn mrb_debug_info_append_file(mrb: *mut mrb_state,
                                  info: *mut mrb_irep_debug_info,
                                  filename: *const libc::c_char,
                                  lines: *mut uint16_t, start_pos: uint32_t,
                                  end_pos: uint32_t)
     -> *mut mrb_irep_debug_info_file;
    #[no_mangle]
    fn _longjmp(_: *mut libc::c_int, _: libc::c_int) -> !;
    #[no_mangle]
    fn _setjmp(_: *mut libc::c_int) -> libc::c_int;
}
pub type __darwin_intptr_t = libc::c_long;
pub type __darwin_ct_rune_t = libc::c_int;
pub type __darwin_size_t = libc::c_ulong;
pub type size_t = __darwin_size_t;
pub type int64_t = libc::c_longlong;
pub type intptr_t = __darwin_intptr_t;
pub type uint8_t = libc::c_uchar;
pub type uint16_t = libc::c_ushort;
pub type uint32_t = libc::c_uint;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub union C2RustUnnamed {
    pub __f: libc::c_float,
    pub __u: libc::c_uint,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub union C2RustUnnamed_0 {
    pub __f: libc::c_double,
    pub __u: libc::c_ulonglong,
}
#[derive ( Copy , Clone )]
#[repr(C)]
pub struct C2RustUnnamed_1 {
    pub __m: libc::c_ulonglong,
    pub __sexp: libc::c_ushort,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub union C2RustUnnamed_2 {
    pub __ld: f128::f128,
    pub __p: C2RustUnnamed_1,
}
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
    pub body: C2RustUnnamed_5,
    pub upper: *mut RProc,
    pub e: C2RustUnnamed_3,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub union C2RustUnnamed_3 {
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
pub struct mrb_value {
    pub value: C2RustUnnamed_4,
    pub tt: mrb_vtype,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub union C2RustUnnamed_4 {
    pub f: mrb_float,
    pub p: *mut libc::c_void,
    pub i: mrb_int,
    pub sym: mrb_sym,
}
pub type mrb_int = int64_t;
pub type mrb_float = libc::c_double;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub union C2RustUnnamed_5 {
    pub irep: *mut mrb_irep,
    pub func: mrb_func_t,
}
/* default method cache size: 128 */
/* cache size needs to be power of 2 */
pub type mrb_func_t
    =
    Option<unsafe extern "C" fn(_: *mut mrb_state, _: mrb_value)
               -> mrb_value>;
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
    pub lines: C2RustUnnamed_6,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub union C2RustUnnamed_6 {
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
/*
** mruby/compile.h - mruby parser
**
** See Copyright Notice in mruby.h
*/
/* *
 * MRuby Compiler
 */
#[derive ( Copy , Clone )]
#[repr(C)]
pub struct mrb_jmpbuf {
    pub impl_0: jmp_buf,
}
pub type jmp_buf = [libc::c_int; 37];
/* *
 * Required arguments signature type.
 */
pub type mrb_aspec = uint32_t;
/* parser structure */
#[derive ( BitfieldStruct , Clone , Copy )]
#[repr(C)]
pub struct mrb_parser_state {
    pub mrb: *mut mrb_state,
    pub pool: *mut mrb_pool,
    pub cells: *mut mrb_ast_node,
    pub s: *const libc::c_char,
    pub send: *const libc::c_char,
    pub cxt: *mut mrbc_context,
    pub filename_sym: mrb_sym,
    pub lineno: uint16_t,
    #[bitfield(padding)]
    pub _pad: [u8; 2],
    pub column: libc::c_int,
    pub lstate: mrb_lex_state_enum,
    pub lex_strterm: *mut mrb_ast_node,
    pub cond_stack: libc::c_uint,
    pub cmdarg_stack: libc::c_uint,
    pub paren_nest: libc::c_int,
    pub lpar_beg: libc::c_int,
    pub in_def: libc::c_int,
    pub in_single: libc::c_int,
    #[bitfield(name = "cmd_start", ty = "mrb_bool", bits = "0..=0")]
    pub cmd_start: [u8; 1],
    #[bitfield(padding)]
    pub _pad2: [u8; 7],
    pub locals: *mut mrb_ast_node,
    pub pb: *mut mrb_ast_node,
    pub tokbuf: *mut libc::c_char,
    pub buf: [libc::c_char; 256],
    pub tidx: libc::c_int,
    pub tsiz: libc::c_int,
    pub all_heredocs: *mut mrb_ast_node,
    pub heredocs_from_nextline: *mut mrb_ast_node,
    pub parsing_heredoc: *mut mrb_ast_node,
    pub lex_strterm_before_heredoc: *mut mrb_ast_node,
    pub ylval: *mut libc::c_void,
    pub nerr: size_t,
    pub nwarn: size_t,
    pub tree: *mut mrb_ast_node,
    #[bitfield(name = "no_optimize", ty = "mrb_bool", bits = "0..=0")]
    #[bitfield(name = "on_eval", ty = "mrb_bool", bits = "1..=1")]
    #[bitfield(name = "capture_errors", ty = "mrb_bool", bits = "2..=2")]
    pub no_optimize_on_eval_capture_errors: [u8; 1],
    #[bitfield(padding)]
    pub _pad3: [u8; 7],
    pub error_buffer: [mrb_parser_message; 10],
    pub warn_buffer: [mrb_parser_message; 10],
    pub filename_table: *mut mrb_sym,
    pub filename_table_length: uint16_t,
    pub current_filename_index: uint16_t,
    #[bitfield(padding)]
    pub _pad4: [u8; 4],
    pub jmp: *mut mrb_jmpbuf,
}
/* saved error message */
#[derive ( Copy , Clone )]
#[repr(C)]
pub struct mrb_parser_message {
    pub lineno: uint16_t,
    pub column: libc::c_int,
    pub message: *mut libc::c_char,
}
/* AST node structure */
#[derive ( Copy , Clone )]
#[repr(C)]
pub struct mrb_ast_node {
    pub car: *mut mrb_ast_node,
    pub cdr: *mut mrb_ast_node,
    pub lineno: uint16_t,
    pub filename_index: uint16_t,
}
/* lexer states */
pub type mrb_lex_state_enum = libc::c_uint;
pub const EXPR_MAX_STATE: mrb_lex_state_enum = 11;
/* alike EXPR_BEG but label is disallowed. */
pub const EXPR_VALUE: mrb_lex_state_enum = 10;
/* immediate after 'class', no here document. */
pub const EXPR_CLASS: mrb_lex_state_enum = 9;
/* right after '.' or '::', no reserved words. */
pub const EXPR_DOT: mrb_lex_state_enum = 8;
/* ignore newline, no reserved words. */
pub const EXPR_FNAME: mrb_lex_state_enum = 7;
/* newline significant, +/- is an operator. */
pub const EXPR_MID: mrb_lex_state_enum = 6;
/* newline significant, +/- is an operator. */
pub const EXPR_CMDARG: mrb_lex_state_enum = 5;
/* newline significant, +/- is an operator. */
pub const EXPR_ARG: mrb_lex_state_enum = 4;
/* ditto, and unbound braces. */
pub const EXPR_ENDFN: mrb_lex_state_enum = 3;
/* ditto, and unbound braces. */
pub const EXPR_ENDARG: mrb_lex_state_enum = 2;
/* newline significant, +/- is an operator. */
pub const EXPR_END: mrb_lex_state_enum = 1;
/* ignore newline, +/- is a sign. */
pub const EXPR_BEG: mrb_lex_state_enum = 0;
/* load context */
#[derive ( BitfieldStruct , Clone , Copy )]
#[repr(C)]
pub struct mrbc_context {
    pub syms: *mut mrb_sym,
    pub slen: libc::c_int,
    #[bitfield(padding)]
    pub _pad: [u8; 4],
    pub filename: *mut libc::c_char,
    pub lineno: uint16_t,
    #[bitfield(padding)]
    pub _pad2: [u8; 6],
    pub partial_hook: Option<unsafe extern "C" fn(_: *mut mrb_parser_state)
                                 -> libc::c_int>,
    pub partial_data: *mut libc::c_void,
    pub target_class: *mut RClass,
    #[bitfield(name = "capture_errors", ty = "mrb_bool", bits = "0..=0")]
    #[bitfield(name = "dump_result", ty = "mrb_bool", bits = "1..=1")]
    #[bitfield(name = "no_exec", ty = "mrb_bool", bits = "2..=2")]
    #[bitfield(name = "keep_lv", ty = "mrb_bool", bits = "3..=3")]
    #[bitfield(name = "no_optimize", ty = "mrb_bool", bits = "4..=4")]
    #[bitfield(name = "on_eval", ty = "mrb_bool", bits = "5..=5")]
    pub capture_errors_dump_result_no_exec_keep_lv_no_optimize_on_eval: [u8; 1],
    #[bitfield(padding)]
    pub _pad3: [u8; 7],
    pub parser_nerr: size_t,
}
pub type mrb_string_type = libc::c_uint;
pub const str_xquote: mrb_string_type = 131;
pub const str_heredoc: mrb_string_type = 65;
pub const str_dsymbols: mrb_string_type = 51;
pub const str_ssymbols: mrb_string_type = 49;
pub const str_ssym: mrb_string_type = 17;
pub const str_dword: mrb_string_type = 43;
pub const str_sword: mrb_string_type = 41;
pub const str_regexp: mrb_string_type = 7;
pub const str_dquote: mrb_string_type = 3;
pub const str_squote: mrb_string_type = 1;
pub const str_not_parsing: mrb_string_type = 0;
/* heredoc structure */
#[derive ( BitfieldStruct , Clone , Copy )]
#[repr(C)]
pub struct mrb_parser_heredoc_info {
    #[bitfield(name = "allow_indent", ty = "mrb_bool", bits = "0..=0")]
    #[bitfield(name = "line_head", ty = "mrb_bool", bits = "1..=1")]
    pub allow_indent_line_head: [u8; 1],
    #[bitfield(padding)]
    pub _pad: [u8; 3],
    pub type_0: mrb_string_type,
    pub term: *const libc::c_char,
    pub term_len: libc::c_int,
    #[bitfield(padding)]
    pub _pad2: [u8; 4],
    pub doc: *mut mrb_ast_node,
}
pub type parser_state = mrb_parser_state;
pub type codegen_scope = scope;
#[derive ( BitfieldStruct , Clone , Copy )]
#[repr(C)]
pub struct scope {
    pub mrb: *mut mrb_state,
    pub mpool: *mut mrb_pool,
    pub jmp: mrb_jmpbuf,
    #[bitfield(padding)]
    pub _pad: [u8; 4],
    pub prev: *mut scope,
    pub lv: *mut node,
    pub sp: uint16_t,
    pub pc: uint16_t,
    pub lastpc: uint16_t,
    pub lastlabel: uint16_t,
    #[bitfield(name = "ainfo", ty = "libc::c_int", bits = "0..=14")]
    #[bitfield(name = "mscope", ty = "mrb_bool", bits = "15..=15")]
    pub ainfo_mscope: [u8; 2],
    #[bitfield(padding)]
    pub _pad2: [u8; 6],
    pub loop_0: *mut loopinfo,
    pub ensure_level: libc::c_int,
    pub filename_sym: mrb_sym,
    pub lineno: uint16_t,
    #[bitfield(padding)]
    pub _pad3: [u8; 6],
    pub iseq: *mut mrb_code,
    pub lines: *mut uint16_t,
    pub icapa: uint32_t,
    #[bitfield(padding)]
    pub _pad4: [u8; 4],
    pub irep: *mut mrb_irep,
    pub pcapa: uint32_t,
    pub scapa: uint32_t,
    pub rcapa: uint32_t,
    pub nlocals: uint16_t,
    pub nregs: uint16_t,
    pub ai: libc::c_int,
    pub debug_start_pos: libc::c_int,
    pub filename_index: uint16_t,
    #[bitfield(padding)]
    pub _pad5: [u8; 6],
    pub parser: *mut parser_state,
    pub rlev: libc::c_int,
    #[bitfield(padding)]
    pub _pad6: [u8; 4],
}
#[derive ( Copy , Clone )]
#[repr(C)]
pub struct loopinfo {
    pub type_0: looptype,
    pub pc0: libc::c_int,
    pub pc1: libc::c_int,
    pub pc2: libc::c_int,
    pub pc3: libc::c_int,
    pub acc: libc::c_int,
    pub ensure_level: libc::c_int,
    pub prev: *mut loopinfo,
}
pub type looptype = libc::c_uint;
pub const LOOP_RESCUE: looptype = 4;
pub const LOOP_BEGIN: looptype = 3;
pub const LOOP_FOR: looptype = 2;
pub const LOOP_BLOCK: looptype = 1;
pub const LOOP_NORMAL: looptype = 0;
pub type node = mrb_ast_node;
pub const NODE_POSTEXE: node_type = 82;
/* R(a) = Syms(b) */
pub const OP_LOADSYM: mrb_insn = 14;
/* make 1st operand 16bit */
pub const OP_EXT1: mrb_insn = 100;
/* make 2nd operand 16bit */
pub const OP_EXT2: mrb_insn = 101;
/* make 1st and 2nd operands 16bit */
pub const OP_EXT3: mrb_insn = 102;
/* R(a).newmethod(Syms(b),R(a+1)) */
pub const OP_DEF: mrb_insn = 93;
/* R(a) = nil */
pub const OP_LOADNIL: mrb_insn = 15;
/* return R(a) (normal) */
pub const OP_RETURN: mrb_insn = 55;
#[derive ( Copy , Clone )]
#[repr(C)]
pub struct mrb_insn_data {
    pub insn: uint8_t,
    pub a: uint16_t,
    pub b: uint16_t,
    pub c: uint8_t,
}
/* stop VM */
pub const OP_STOP: mrb_insn = 103;
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
/* R(a) = kdict[Syms(b)]; kdict.delete(Syms(b))    # todo */
pub const OP_KARG: mrb_insn = 54;
/* raise unless kdict.empty?                       # todo */
pub const OP_KEYEND: mrb_insn = 53;
/* R(a) = kdict.key?(Syms(b))                      # todo */
pub const OP_KEY_P: mrb_insn = 52;
/* arg setup according to flags (23=m5:o5:r1:m5:k5:d1:b1) */
pub const OP_ENTER: mrb_insn = 51;
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
pub const NODE_NIL: node_type = 78;
pub const NODE_MASGN: node_type = 20;
pub const NODE_SCALL: node_type = 27;
pub const NODE_SPLAT: node_type = 62;
pub const NODE_ARRAY: node_type = 31;
pub const NODE_CALL: node_type = 26;
pub const NODE_COLON2: node_type = 73;
pub const NODE_CONST: node_type = 41;
pub const NODE_CVAR: node_type = 42;
pub const NODE_IVAR: node_type = 40;
pub const NODE_LVAR: node_type = 37;
pub const NODE_ARG: node_type = 58;
pub const NODE_GVAR: node_type = 39;
#[derive ( Copy , Clone )]
#[repr(C)]
pub struct C2RustUnnamed_7 {
    pub len: mrb_int,
    pub aux: C2RustUnnamed_8,
    pub ptr: *mut libc::c_char,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub union C2RustUnnamed_8 {
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
    #[bitfield(padding)]
    pub _pad: [u8; 4],
    pub c: *mut RClass,
    pub gcnext: *mut RBasic,
    pub as_0: C2RustUnnamed_9,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub union C2RustUnnamed_9 {
    pub heap: C2RustUnnamed_7,
    pub ary: [libc::c_char; 24],
}
pub const NODE_SDEF: node_type = 67;
pub const NODE_DEF: node_type = 66;
pub const NODE_BEGIN: node_type = 14;
pub const NODE_SCLASS: node_type = 72;
pub const NODE_MODULE: node_type = 71;
pub const NODE_CLASS: node_type = 70;
pub const NODE_UNDEF: node_type = 69;
pub const NODE_ALIAS: node_type = 68;
pub const NODE_FALSE: node_type = 80;
pub const NODE_TRUE: node_type = 79;
pub const NODE_SELF: node_type = 77;
pub const NODE_DSYM: node_type = 83;
pub const NODE_SYM: node_type = 50;
pub const NODE_STR: node_type = 51;
pub const NODE_DREGX: node_type = 56;
pub const NODE_REGX: node_type = 55;
pub const NODE_XSTR: node_type = 53;
pub const NODE_DXSTR: node_type = 54;
pub const NODE_BLOCK: node_type = 2;
pub const NODE_LITERAL_DELIM: node_type = 85;
pub const NODE_SYMBOLS: node_type = 87;
pub const NODE_WORDS: node_type = 86;
pub const NODE_DSTR: node_type = 52;
pub const NODE_HEREDOC: node_type = 84;
pub const NODE_INT: node_type = 46;
pub const NODE_FLOAT: node_type = 47;
pub const NODE_NEGATE: node_type = 48;
pub const NODE_BLOCK_ARG: node_type = 65;
pub const NODE_NTH_REF: node_type = 43;
pub const NODE_BACK_REF: node_type = 44;
pub const NODE_DEFINED: node_type = 81;
pub const NODE_RETRY: node_type = 13;
pub const NODE_REDO: node_type = 12;
pub const NODE_NEXT: node_type = 11;
pub const NODE_BREAK: node_type = 10;
pub const NODE_YIELD: node_type = 36;
pub const NODE_RETURN: node_type = 35;
pub const NODE_ZSUPER: node_type = 30;
pub const NODE_SUPER: node_type = 29;
pub const NODE_OP_ASGN: node_type = 25;
pub const NODE_ASGN: node_type = 21;
pub const NODE_KW_REST_ARGS: node_type = 61;
pub const NODE_KW_HASH: node_type = 34;
pub const NODE_HASH: node_type = 33;
pub const NODE_COLON3: node_type = 74;
pub const NODE_DOT3: node_type = 76;
pub const NODE_DOT2: node_type = 75;
pub const NODE_FCALL: node_type = 28;
pub const NODE_SCOPE: node_type = 1;
pub const NODE_CASE: node_type = 4;
pub const NODE_FOR: node_type = 9;
pub const NODE_UNTIL: node_type = 7;
pub const NODE_WHILE: node_type = 6;
pub const NODE_OR: node_type = 18;
pub const NODE_AND: node_type = 17;
pub const NODE_IF: node_type = 3;
pub const NODE_LAMBDA: node_type = 49;
pub const NODE_ENSURE: node_type = 16;
pub const NODE_RESCUE: node_type = 15;
/*
** node.h - nodes of abstract syntax tree
**
** See Copyright Notice in mruby.h
*/
pub type node_type = libc::c_uint;
pub const NODE_LAST: node_type = 88;
pub const NODE_SVALUE: node_type = 64;
pub const NODE_TO_ARY: node_type = 63;
pub const NODE_KW_ARG: node_type = 60;
pub const NODE_ARGS_TAIL: node_type = 59;
pub const NODE_DREGX_ONCE: node_type = 57;
pub const NODE_MATCH: node_type = 45;
pub const NODE_DVAR: node_type = 38;
pub const NODE_ZARRAY: node_type = 32;
pub const NODE_CVDECL: node_type = 24;
pub const NODE_CVASGN: node_type = 23;
pub const NODE_CDECL: node_type = 22;
pub const NODE_NOT: node_type = 19;
pub const NODE_ITER: node_type = 8;
pub const NODE_WHEN: node_type = 5;
pub const NODE_METHOD: node_type = 0;
/*
** mruby/opcode.h - RiteVM operation codes
**
** See Copyright Notice in mruby.h
*/
pub type mrb_insn = libc::c_uint;
#[no_mangle]
#[inline]
#[linkage = "external"]
pub unsafe extern "C" fn tolower(mut _c: libc::c_int) -> libc::c_int {
    return __tolower(_c);
}
#[inline(always)]
unsafe extern "C" fn __inline_signbitf(mut __x: libc::c_float)
 -> libc::c_int {
    let mut __u: C2RustUnnamed = C2RustUnnamed{__f: 0.,};
    __u.__f = __x;
    return (__u.__u >> 31i32) as libc::c_int;
}
#[inline(always)]
unsafe extern "C" fn __inline_signbitd(mut __x: libc::c_double)
 -> libc::c_int {
    let mut __u: C2RustUnnamed_0 = C2RustUnnamed_0{__f: 0.,};
    __u.__f = __x;
    return (__u.__u >> 63i32) as libc::c_int;
}
#[inline(always)]
unsafe extern "C" fn __inline_signbitl(mut __x: f128::f128) -> libc::c_int {
    let mut __u: C2RustUnnamed_2 = C2RustUnnamed_2{__ld: f128::f128::ZERO,};
    __u.__ld = __x;
    return __u.__p.__sexp as libc::c_int >> 15i32;
}
/*
 * Returns a fixnum in Ruby.
 */
#[inline]
unsafe extern "C" fn mrb_fixnum_value(mut i: mrb_int) -> mrb_value {
    let mut v: mrb_value =
        mrb_value{value: C2RustUnnamed_4{f: 0.,}, tt: MRB_TT_FALSE,};
    v.tt = MRB_TT_FIXNUM;
    v.value.i = i;
    return v;
}
/*
 * Returns a float in Ruby.
 */
#[inline]
unsafe extern "C" fn mrb_float_value(mut mrb: *mut mrb_state,
                                     mut f: mrb_float) -> mrb_value {
    let mut v: mrb_value =
        mrb_value{value: C2RustUnnamed_4{f: 0.,}, tt: MRB_TT_FALSE,};
    v.tt = MRB_TT_FLOAT;
    v.value.f = f;
    return v;
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
unsafe extern "C" fn codegen_error(mut s: *mut codegen_scope,
                                   mut message: *const libc::c_char) {
    if s.is_null() { return }
    while !(*s).prev.is_null() {
        let mut tmp: *mut codegen_scope = (*s).prev;
        mrb_free((*s).mrb, (*s).iseq as *mut libc::c_void);
        mrb_free((*s).mrb, (*s).lines as *mut libc::c_void);
        mrb_pool_close((*s).mpool);
        s = tmp
    }
    _longjmp((*s).jmp.impl_0.as_mut_ptr(), 1i32);
}
unsafe extern "C" fn codegen_palloc(mut s: *mut codegen_scope,
                                    mut len: size_t) -> *mut libc::c_void {
    let mut p: *mut libc::c_void = mrb_pool_alloc((*s).mpool, len);
    if p.is_null() {
        codegen_error(s,
                      b"pool memory allocation\x00" as *const u8 as
                          *const libc::c_char);
    }
    return p;
}
unsafe extern "C" fn codegen_realloc(mut s: *mut codegen_scope,
                                     mut p: *mut libc::c_void,
                                     mut len: size_t) -> *mut libc::c_void {
    p = mrb_realloc_simple((*s).mrb, p, len);
    if p.is_null() && len > 0i32 as libc::c_ulong {
        codegen_error(s,
                      b"mrb_realloc\x00" as *const u8 as *const libc::c_char);
    }
    return p;
}
unsafe extern "C" fn new_label(mut s: *mut codegen_scope) -> libc::c_int {
    (*s).lastlabel = (*s).pc;
    return (*s).lastlabel as libc::c_int;
}
unsafe extern "C" fn emit_B(mut s: *mut codegen_scope, mut pc: uint32_t,
                            mut i: uint8_t) {
    if pc >= (1i32 << 16i32) as libc::c_uint ||
           (*s).icapa >= (1i32 << 16i32) as libc::c_uint {
        codegen_error(s,
                      b"too big code block\x00" as *const u8 as
                          *const libc::c_char);
    }
    if pc >= (*s).icapa {
        (*s).icapa =
            ((*s).icapa as libc::c_uint).wrapping_mul(2i32 as libc::c_uint) as
                uint32_t as uint32_t;
        if (*s).icapa > (1i32 << 16i32) as libc::c_uint {
            (*s).icapa = (1i32 << 16i32) as uint32_t
        }
        (*s).iseq =
            codegen_realloc(s, (*s).iseq as *mut libc::c_void,
                            (::std::mem::size_of::<mrb_code>() as
                                 libc::c_ulong).wrapping_mul((*s).icapa as
                                                                 libc::c_ulong))
                as *mut mrb_code;
        if !(*s).lines.is_null() {
            (*s).lines =
                codegen_realloc(s, (*s).lines as *mut libc::c_void,
                                (::std::mem::size_of::<uint16_t>() as
                                     libc::c_ulong).wrapping_mul((*s).icapa as
                                                                     libc::c_ulong))
                    as *mut uint16_t
        }
    }
    if !(*s).lines.is_null() {
        if (*s).lineno as libc::c_int > 0i32 || pc == 0i32 as libc::c_uint {
            *(*s).lines.offset(pc as isize) = (*s).lineno
        } else {
            *(*s).lines.offset(pc as isize) =
                *(*s).lines.offset(pc.wrapping_sub(1i32 as libc::c_uint) as
                                       isize)
        }
    }
    *(*s).iseq.offset(pc as isize) = i;
}
unsafe extern "C" fn emit_S(mut s: *mut codegen_scope, mut pc: libc::c_int,
                            mut i: uint16_t) {
    let mut hi: uint8_t = (i as libc::c_int >> 8i32) as uint8_t;
    let mut lo: uint8_t = (i as libc::c_int & 0xffi32) as uint8_t;
    emit_B(s, pc as uint32_t, hi);
    emit_B(s, (pc + 1i32) as uint32_t, lo);
}
unsafe extern "C" fn gen_B(mut s: *mut codegen_scope, mut i: uint8_t) {
    emit_B(s, (*s).pc as uint32_t, i);
    (*s).pc = (*s).pc.wrapping_add(1);
}
unsafe extern "C" fn gen_S(mut s: *mut codegen_scope, mut i: uint16_t) {
    emit_S(s, (*s).pc as libc::c_int, i);
    (*s).pc = ((*s).pc as libc::c_int + 2i32) as uint16_t;
}
unsafe extern "C" fn genop_0(mut s: *mut codegen_scope, mut i: mrb_code) {
    (*s).lastpc = (*s).pc;
    gen_B(s, i);
}
unsafe extern "C" fn genop_1(mut s: *mut codegen_scope, mut i: mrb_code,
                             mut a: uint16_t) {
    (*s).lastpc = (*s).pc;
    if a as libc::c_int > 0xffi32 {
        gen_B(s, OP_EXT1 as libc::c_int as uint8_t);
        gen_B(s, i);
        gen_S(s, a);
    } else { gen_B(s, i); gen_B(s, a as uint8_t); };
}
unsafe extern "C" fn genop_2(mut s: *mut codegen_scope, mut i: mrb_code,
                             mut a: uint16_t, mut b: uint16_t) {
    (*s).lastpc = (*s).pc;
    if a as libc::c_int > 0xffi32 && b as libc::c_int > 0xffi32 {
        gen_B(s, OP_EXT3 as libc::c_int as uint8_t);
        gen_B(s, i);
        gen_S(s, a);
        gen_S(s, b);
    } else if b as libc::c_int > 0xffi32 {
        gen_B(s, OP_EXT2 as libc::c_int as uint8_t);
        gen_B(s, i);
        gen_B(s, a as uint8_t);
        gen_S(s, b);
    } else if a as libc::c_int > 0xffi32 {
        gen_B(s, OP_EXT1 as libc::c_int as uint8_t);
        gen_B(s, i);
        gen_S(s, a);
        gen_B(s, b as uint8_t);
    } else { gen_B(s, i); gen_B(s, a as uint8_t); gen_B(s, b as uint8_t); };
}
unsafe extern "C" fn genop_3(mut s: *mut codegen_scope, mut i: mrb_code,
                             mut a: uint16_t, mut b: uint16_t,
                             mut c: uint8_t) {
    genop_2(s, i, a, b);
    gen_B(s, c);
}
unsafe extern "C" fn genop_2S(mut s: *mut codegen_scope, mut i: mrb_code,
                              mut a: uint16_t, mut b: uint16_t) {
    genop_1(s, i, a);
    gen_S(s, b);
}
unsafe extern "C" fn genop_W(mut s: *mut codegen_scope, mut i: mrb_code,
                             mut a: uint32_t) {
    let mut a1: uint8_t = (a >> 16i32 & 0xffi32 as libc::c_uint) as uint8_t;
    let mut a2: uint8_t = (a >> 8i32 & 0xffi32 as libc::c_uint) as uint8_t;
    let mut a3: uint8_t = (a & 0xffi32 as libc::c_uint) as uint8_t;
    (*s).lastpc = (*s).pc;
    gen_B(s, i);
    gen_B(s, a1);
    gen_B(s, a2);
    gen_B(s, a3);
}
unsafe extern "C" fn no_optimize(mut s: *mut codegen_scope) -> mrb_bool {
    if !s.is_null() && !(*s).parser.is_null() &&
           0 != (*(*s).parser).no_optimize() as libc::c_int {
        return 1i32 as mrb_bool
    }
    return 0i32 as mrb_bool;
}
unsafe extern "C" fn on_eval(mut s: *mut codegen_scope) -> mrb_bool {
    if !s.is_null() && !(*s).parser.is_null() &&
           0 != (*(*s).parser).on_eval() as libc::c_int {
        return 1i32 as mrb_bool
    }
    return 0i32 as mrb_bool;
}
#[no_mangle]
pub unsafe extern "C" fn mrb_decode_insn(mut pc: *mut mrb_code)
 -> mrb_insn_data {
    let mut data: mrb_insn_data =
        mrb_insn_data{insn: 0i32 as uint8_t, a: 0, b: 0, c: 0,};
    let fresh0 = pc;
    pc = pc.offset(1);
    let mut insn: mrb_code = *fresh0;
    let mut a: uint16_t = 0i32 as uint16_t;
    let mut b: uint16_t = 0i32 as uint16_t;
    let mut c: uint8_t = 0i32 as uint8_t;
    match insn as libc::c_int {
        1 => {
            let fresh1 = pc;
            pc = pc.offset(1);
            a = *fresh1 as uint16_t;
            /* R(a) = R(b) */
            let fresh2 = pc;
            pc = pc.offset(1);
            b = *fresh2 as uint16_t
        }
        2 => {
            let fresh3 = pc;
            pc = pc.offset(1);
            a = *fresh3 as uint16_t;
            /* R(a) = Pool(b) */
            let fresh4 = pc;
            pc = pc.offset(1);
            b = *fresh4 as uint16_t
        }
        3 => {
            let fresh5 = pc;
            pc = pc.offset(1);
            a = *fresh5 as uint16_t;
            /* R(a) = mrb_int(b) */
            let fresh6 = pc;
            pc = pc.offset(1);
            b = *fresh6 as uint16_t
        }
        4 => {
            let fresh7 = pc;
            pc = pc.offset(1);
            a = *fresh7 as uint16_t;
            /* R(a) = mrb_int(-b) */
            let fresh8 = pc;
            pc = pc.offset(1);
            b = *fresh8 as uint16_t
        }
        5 => {
            /* R(a) = mrb_int(-1) */
            let fresh9 = pc;
            pc = pc.offset(1);
            a = *fresh9 as uint16_t
        }
        6 => {
            /* R(a) = mrb_int(0) */
            let fresh10 = pc;
            pc = pc.offset(1);
            a = *fresh10 as uint16_t
        }
        7 => {
            /* R(a) = mrb_int(1) */
            let fresh11 = pc;
            pc = pc.offset(1);
            a = *fresh11 as uint16_t
        }
        8 => {
            /* R(a) = mrb_int(2) */
            let fresh12 = pc;
            pc = pc.offset(1);
            a = *fresh12 as uint16_t
        }
        9 => {
            /* R(a) = mrb_int(3) */
            let fresh13 = pc;
            pc = pc.offset(1);
            a = *fresh13 as uint16_t
        }
        10 => {
            /* R(a) = mrb_int(4) */
            let fresh14 = pc;
            pc = pc.offset(1);
            a = *fresh14 as uint16_t
        }
        11 => {
            /* R(a) = mrb_int(5) */
            let fresh15 = pc;
            pc = pc.offset(1);
            a = *fresh15 as uint16_t
        }
        12 => {
            /* R(a) = mrb_int(6) */
            let fresh16 = pc;
            pc = pc.offset(1);
            a = *fresh16 as uint16_t
        }
        13 => {
            /* R(a) = mrb_int(7) */
            let fresh17 = pc;
            pc = pc.offset(1);
            a = *fresh17 as uint16_t
        }
        14 => {
            let fresh18 = pc;
            pc = pc.offset(1);
            a = *fresh18 as uint16_t;
            /* R(a) = Syms(b) */
            let fresh19 = pc;
            pc = pc.offset(1);
            b = *fresh19 as uint16_t
        }
        15 => {
            /* R(a) = nil */
            let fresh20 = pc;
            pc = pc.offset(1);
            a = *fresh20 as uint16_t
        }
        16 => {
            /* R(a) = self */
            let fresh21 = pc;
            pc = pc.offset(1);
            a = *fresh21 as uint16_t
        }
        17 => {
            /* R(a) = true */
            let fresh22 = pc;
            pc = pc.offset(1);
            a = *fresh22 as uint16_t
        }
        18 => {
            /* R(a) = false */
            let fresh23 = pc;
            pc = pc.offset(1);
            a = *fresh23 as uint16_t
        }
        19 => {
            let fresh24 = pc;
            pc = pc.offset(1);
            a = *fresh24 as uint16_t;
            /* R(a) = getglobal(Syms(b)) */
            let fresh25 = pc;
            pc = pc.offset(1);
            b = *fresh25 as uint16_t
        }
        20 => {
            let fresh26 = pc;
            pc = pc.offset(1);
            a = *fresh26 as uint16_t;
            /* setglobal(Syms(b), R(a)) */
            let fresh27 = pc;
            pc = pc.offset(1);
            b = *fresh27 as uint16_t
        }
        21 => {
            let fresh28 = pc;
            pc = pc.offset(1);
            a = *fresh28 as uint16_t;
            /* R(a) = Special[Syms(b)] */
            let fresh29 = pc;
            pc = pc.offset(1);
            b = *fresh29 as uint16_t
        }
        22 => {
            let fresh30 = pc;
            pc = pc.offset(1);
            a = *fresh30 as uint16_t;
            /* Special[Syms(b)] = R(a) */
            let fresh31 = pc;
            pc = pc.offset(1);
            b = *fresh31 as uint16_t
        }
        23 => {
            let fresh32 = pc;
            pc = pc.offset(1);
            a = *fresh32 as uint16_t;
            /* R(a) = ivget(Syms(b)) */
            let fresh33 = pc;
            pc = pc.offset(1);
            b = *fresh33 as uint16_t
        }
        24 => {
            let fresh34 = pc;
            pc = pc.offset(1);
            a = *fresh34 as uint16_t;
            /* ivset(Syms(b),R(a)) */
            let fresh35 = pc;
            pc = pc.offset(1);
            b = *fresh35 as uint16_t
        }
        25 => {
            let fresh36 = pc;
            pc = pc.offset(1);
            a = *fresh36 as uint16_t;
            /* R(a) = cvget(Syms(b)) */
            let fresh37 = pc;
            pc = pc.offset(1);
            b = *fresh37 as uint16_t
        }
        26 => {
            let fresh38 = pc;
            pc = pc.offset(1);
            a = *fresh38 as uint16_t;
            /* cvset(Syms(b),R(a)) */
            let fresh39 = pc;
            pc = pc.offset(1);
            b = *fresh39 as uint16_t
        }
        27 => {
            let fresh40 = pc;
            pc = pc.offset(1);
            a = *fresh40 as uint16_t;
            /* R(a) = constget(Syms(b)) */
            let fresh41 = pc;
            pc = pc.offset(1);
            b = *fresh41 as uint16_t
        }
        28 => {
            let fresh42 = pc;
            pc = pc.offset(1);
            a = *fresh42 as uint16_t;
            /* constset(Syms(b),R(a)) */
            let fresh43 = pc;
            pc = pc.offset(1);
            b = *fresh43 as uint16_t
        }
        29 => {
            let fresh44 = pc;
            pc = pc.offset(1);
            a = *fresh44 as uint16_t;
            /* R(a) = R(a)::Syms(b) */
            let fresh45 = pc;
            pc = pc.offset(1);
            b = *fresh45 as uint16_t
        }
        30 => {
            let fresh46 = pc;
            pc = pc.offset(1);
            a = *fresh46 as uint16_t;
            /* R(a+1)::Syms(b) = R(a) */
            let fresh47 = pc;
            pc = pc.offset(1);
            b = *fresh47 as uint16_t
        }
        31 => {
            let fresh48 = pc;
            pc = pc.offset(1);
            a = *fresh48 as uint16_t;
            let fresh49 = pc;
            pc = pc.offset(1);
            b = *fresh49 as uint16_t;
            /* R(a) = uvget(b,c) */
            let fresh50 = pc;
            pc = pc.offset(1);
            c = *fresh50
        }
        32 => {
            let fresh51 = pc;
            pc = pc.offset(1);
            a = *fresh51 as uint16_t;
            let fresh52 = pc;
            pc = pc.offset(1);
            b = *fresh52 as uint16_t;
            /* uvset(b,c,R(a)) */
            let fresh53 = pc;
            pc = pc.offset(1);
            c = *fresh53
        }
        33 => {
            /* pc=a */
            pc = pc.offset(2isize);
            a =
                ((*pc.offset(-2isize).offset(0isize) as libc::c_int) << 8i32 |
                     *pc.offset(-2isize).offset(1isize) as libc::c_int) as
                    uint16_t
        }
        34 => {
            let fresh54 = pc;
            pc = pc.offset(1);
            a = *fresh54 as uint16_t;
            /* if R(b) pc=a */
            pc = pc.offset(2isize);
            b =
                ((*pc.offset(-2isize).offset(0isize) as libc::c_int) << 8i32 |
                     *pc.offset(-2isize).offset(1isize) as libc::c_int) as
                    uint16_t
        }
        35 => {
            let fresh55 = pc;
            pc = pc.offset(1);
            a = *fresh55 as uint16_t;
            /* if !R(b) pc=a */
            pc = pc.offset(2isize);
            b =
                ((*pc.offset(-2isize).offset(0isize) as libc::c_int) << 8i32 |
                     *pc.offset(-2isize).offset(1isize) as libc::c_int) as
                    uint16_t
        }
        36 => {
            let fresh56 = pc;
            pc = pc.offset(1);
            a = *fresh56 as uint16_t;
            /* if R(b)==nil pc=a */
            pc = pc.offset(2isize);
            b =
                ((*pc.offset(-2isize).offset(0isize) as libc::c_int) << 8i32 |
                     *pc.offset(-2isize).offset(1isize) as libc::c_int) as
                    uint16_t
        }
        37 => {
            /* rescue_push(a) */
            pc = pc.offset(2isize);
            a =
                ((*pc.offset(-2isize).offset(0isize) as libc::c_int) << 8i32 |
                     *pc.offset(-2isize).offset(1isize) as libc::c_int) as
                    uint16_t
        }
        38 => {
            /* R(a) = exc */
            let fresh57 = pc;
            pc = pc.offset(1);
            a = *fresh57 as uint16_t
        }
        39 => {
            let fresh58 = pc;
            pc = pc.offset(1);
            a = *fresh58 as uint16_t;
            /* R(b) = R(a).isa?(R(b)) */
            let fresh59 = pc;
            pc = pc.offset(1);
            b = *fresh59 as uint16_t
        }
        40 => {
            /* a.times{rescue_pop()} */
            let fresh60 = pc;
            pc = pc.offset(1);
            a = *fresh60 as uint16_t
        }
        41 => {
            /* raise(R(a)) */
            let fresh61 = pc;
            pc = pc.offset(1);
            a = *fresh61 as uint16_t
        }
        42 => {
            /* ensure_push(SEQ[a]) */
            let fresh62 = pc;
            pc = pc.offset(1);
            a = *fresh62 as uint16_t
        }
        43 => {
            /* A.times{ensure_pop().call} */
            let fresh63 = pc;
            pc = pc.offset(1);
            a = *fresh63 as uint16_t
        }
        44 => {
            let fresh64 = pc;
            pc = pc.offset(1);
            a = *fresh64 as uint16_t;
            /* R(a) = call(R(a),Syms(b),*R(a+1)) */
            let fresh65 = pc;
            pc = pc.offset(1);
            b = *fresh65 as uint16_t
        }
        45 => {
            let fresh66 = pc;
            pc = pc.offset(1);
            a = *fresh66 as uint16_t;
            /* R(a) = call(R(a),Syms(b),*R(a+1),&R(a+2)) */
            let fresh67 = pc;
            pc = pc.offset(1);
            b = *fresh67 as uint16_t
        }
        46 => {
            let fresh68 = pc;
            pc = pc.offset(1);
            a = *fresh68 as uint16_t;
            let fresh69 = pc;
            pc = pc.offset(1);
            b = *fresh69 as uint16_t;
            /* R(a) = call(R(a),Syms(b),R(a+1),...,R(a+c)) */
            let fresh70 = pc;
            pc = pc.offset(1);
            c = *fresh70
        }
        47 => {
            let fresh71 = pc;
            pc = pc.offset(1);
            a = *fresh71 as uint16_t;
            let fresh72 = pc;
            pc = pc.offset(1);
            b = *fresh72 as uint16_t;
            /* R(a) = call(R(a),Syms(Bx),R(a+1),...,R(a+c),&R(a+c+1)) */
            let fresh73 = pc;
            pc = pc.offset(1);
            c = *fresh73
        }
        49 => {
            let fresh74 = pc;
            pc = pc.offset(1);
            a = *fresh74 as uint16_t;
            /* R(a) = super(R(a+1),... ,R(a+b+1)) */
            let fresh75 = pc;
            pc = pc.offset(1);
            b = *fresh75 as uint16_t
        }
        50 => {
            let fresh76 = pc;
            pc = pc.offset(1);
            a = *fresh76 as uint16_t;
            /* R(a) = argument array (16=m5:r1:m5:d1:lv4) */
            pc = pc.offset(2isize);
            b =
                ((*pc.offset(-2isize).offset(0isize) as libc::c_int) << 8i32 |
                     *pc.offset(-2isize).offset(1isize) as libc::c_int) as
                    uint16_t
        }
        51 => {
            /* arg setup according to flags (23=m5:o5:r1:m5:k5:d1:b1) */
            pc = pc.offset(3isize);
            a =
                ((*pc.offset(-3isize).offset(0isize) as libc::c_int) << 16i32
                     |
                     (*pc.offset(-3isize).offset(1isize) as libc::c_int) <<
                         8i32 |
                     *pc.offset(-3isize).offset(2isize) as libc::c_int) as
                    uint16_t
        }
        52 => {
            let fresh77 = pc;
            pc = pc.offset(1);
            a = *fresh77 as uint16_t;
            /* R(a) = kdict.key?(Syms(b))                      # todo */
            let fresh78 = pc;
            pc = pc.offset(1);
            b = *fresh78 as uint16_t
        }
        54 => {
            let fresh79 = pc;
            pc = pc.offset(1);
            a = *fresh79 as uint16_t;
            /* R(a) = kdict[Syms(b)]; kdict.delete(Syms(b))    # todo */
            let fresh80 = pc;
            pc = pc.offset(1);
            b = *fresh80 as uint16_t
        }
        55 => {
            /* return R(a) (normal) */
            let fresh81 = pc;
            pc = pc.offset(1);
            a = *fresh81 as uint16_t
        }
        56 => {
            /* return R(a) (in-block return) */
            let fresh82 = pc;
            pc = pc.offset(1);
            a = *fresh82 as uint16_t
        }
        57 => {
            /* break R(a) */
            let fresh83 = pc;
            pc = pc.offset(1);
            a = *fresh83 as uint16_t
        }
        58 => {
            let fresh84 = pc;
            pc = pc.offset(1);
            a = *fresh84 as uint16_t;
            /* R(a) = block (16=m5:r1:m5:d1:lv4) */
            pc = pc.offset(2isize);
            b =
                ((*pc.offset(-2isize).offset(0isize) as libc::c_int) << 8i32 |
                     *pc.offset(-2isize).offset(1isize) as libc::c_int) as
                    uint16_t
        }
        59 => {
            /* R(a) = R(a)+R(a+1) */
            let fresh85 = pc;
            pc = pc.offset(1);
            a = *fresh85 as uint16_t
        }
        60 => {
            let fresh86 = pc;
            pc = pc.offset(1);
            a = *fresh86 as uint16_t;
            /* R(a) = R(a)+mrb_int(c)  */
            let fresh87 = pc;
            pc = pc.offset(1);
            b = *fresh87 as uint16_t
        }
        61 => {
            /* R(a) = R(a)-R(a+1) */
            let fresh88 = pc;
            pc = pc.offset(1);
            a = *fresh88 as uint16_t
        }
        62 => {
            let fresh89 = pc;
            pc = pc.offset(1);
            a = *fresh89 as uint16_t;
            /* R(a) = R(a)-C */
            let fresh90 = pc;
            pc = pc.offset(1);
            b = *fresh90 as uint16_t
        }
        63 => {
            /* R(a) = R(a)*R(a+1) */
            let fresh91 = pc;
            pc = pc.offset(1);
            a = *fresh91 as uint16_t
        }
        64 => {
            /* R(a) = R(a)/R(a+1) */
            let fresh92 = pc;
            pc = pc.offset(1);
            a = *fresh92 as uint16_t
        }
        65 => {
            /* R(a) = R(a)==R(a+1) */
            let fresh93 = pc;
            pc = pc.offset(1);
            a = *fresh93 as uint16_t
        }
        66 => {
            /* R(a) = R(a)<R(a+1) */
            let fresh94 = pc;
            pc = pc.offset(1);
            a = *fresh94 as uint16_t
        }
        67 => {
            /* R(a) = R(a)<=R(a+1) */
            let fresh95 = pc;
            pc = pc.offset(1);
            a = *fresh95 as uint16_t
        }
        68 => {
            /* R(a) = R(a)>R(a+1) */
            let fresh96 = pc;
            pc = pc.offset(1);
            a = *fresh96 as uint16_t
        }
        69 => {
            /* R(a) = R(a)>=R(a+1) */
            let fresh97 = pc;
            pc = pc.offset(1);
            a = *fresh97 as uint16_t
        }
        70 => {
            let fresh98 = pc;
            pc = pc.offset(1);
            a = *fresh98 as uint16_t;
            /* R(a) = ary_new(R(a),R(a+1)..R(a+b)) */
            let fresh99 = pc;
            pc = pc.offset(1);
            b = *fresh99 as uint16_t
        }
        71 => {
            let fresh100 = pc;
            pc = pc.offset(1);
            a = *fresh100 as uint16_t;
            let fresh101 = pc;
            pc = pc.offset(1);
            b = *fresh101 as uint16_t;
            /* R(a) = ary_new(R(b),R(b+1)..R(b+c)) */
            let fresh102 = pc;
            pc = pc.offset(1);
            c = *fresh102
        }
        72 => {
            /* ary_cat(R(a),R(a+1)) */
            let fresh103 = pc;
            pc = pc.offset(1);
            a = *fresh103 as uint16_t
        }
        73 => {
            /* ary_push(R(a),R(a+1)) */
            let fresh104 = pc;
            pc = pc.offset(1);
            a = *fresh104 as uint16_t
        }
        74 => {
            /* R(a) = ary_dup(R(a)) */
            let fresh105 = pc;
            pc = pc.offset(1);
            a = *fresh105 as uint16_t
        }
        75 => {
            let fresh106 = pc;
            pc = pc.offset(1);
            a = *fresh106 as uint16_t;
            let fresh107 = pc;
            pc = pc.offset(1);
            b = *fresh107 as uint16_t;
            /* R(a) = R(b)[c] */
            let fresh108 = pc;
            pc = pc.offset(1);
            c = *fresh108
        }
        76 => {
            let fresh109 = pc;
            pc = pc.offset(1);
            a = *fresh109 as uint16_t;
            let fresh110 = pc;
            pc = pc.offset(1);
            b = *fresh110 as uint16_t;
            /* R(a)[c] = R(b) */
            let fresh111 = pc;
            pc = pc.offset(1);
            c = *fresh111
        }
        77 => {
            let fresh112 = pc;
            pc = pc.offset(1);
            a = *fresh112 as uint16_t;
            let fresh113 = pc;
            pc = pc.offset(1);
            b = *fresh113 as uint16_t;
            /* *R(a),R(a+1)..R(a+c) = R(a)[b..] */
            let fresh114 = pc;
            pc = pc.offset(1);
            c = *fresh114
        }
        78 => {
            /* R(a) = intern(R(a)) */
            let fresh115 = pc;
            pc = pc.offset(1);
            a = *fresh115 as uint16_t
        }
        79 => {
            let fresh116 = pc;
            pc = pc.offset(1);
            a = *fresh116 as uint16_t;
            /* R(a) = str_dup(Lit(b)) */
            let fresh117 = pc;
            pc = pc.offset(1);
            b = *fresh117 as uint16_t
        }
        80 => {
            /* str_cat(R(a),R(a+1)) */
            let fresh118 = pc;
            pc = pc.offset(1);
            a = *fresh118 as uint16_t
        }
        81 => {
            let fresh119 = pc;
            pc = pc.offset(1);
            a = *fresh119 as uint16_t;
            /* R(a) = hash_new(R(a),R(a+1)..R(a+b)) */
            let fresh120 = pc;
            pc = pc.offset(1);
            b = *fresh120 as uint16_t
        }
        82 => {
            let fresh121 = pc;
            pc = pc.offset(1);
            a = *fresh121 as uint16_t;
            /* R(a) = hash_push(R(a),R(a+1)..R(a+b)) */
            let fresh122 = pc;
            pc = pc.offset(1);
            b = *fresh122 as uint16_t
        }
        83 => {
            /* R(a) = hash_cat(R(a),R(a+1)) */
            let fresh123 = pc;
            pc = pc.offset(1);
            a = *fresh123 as uint16_t
        }
        84 => {
            let fresh124 = pc;
            pc = pc.offset(1);
            a = *fresh124 as uint16_t;
            /* R(a) = lambda(SEQ[b],L_LAMBDA) */
            let fresh125 = pc;
            pc = pc.offset(1);
            b = *fresh125 as uint16_t
        }
        85 => {
            let fresh126 = pc;
            pc = pc.offset(1);
            a = *fresh126 as uint16_t;
            /* R(a) = lambda(SEQ[b],L_BLOCK) */
            let fresh127 = pc;
            pc = pc.offset(1);
            b = *fresh127 as uint16_t
        }
        86 => {
            let fresh128 = pc;
            pc = pc.offset(1);
            a = *fresh128 as uint16_t;
            /* R(a) = lambda(SEQ[b],L_METHOD) */
            let fresh129 = pc;
            pc = pc.offset(1);
            b = *fresh129 as uint16_t
        }
        87 => {
            /* R(a) = range_new(R(a),R(a+1),FALSE) */
            let fresh130 = pc;
            pc = pc.offset(1);
            a = *fresh130 as uint16_t
        }
        88 => {
            /* R(a) = range_new(R(a),R(a+1),TRUE) */
            let fresh131 = pc;
            pc = pc.offset(1);
            a = *fresh131 as uint16_t
        }
        89 => {
            /* R(a) = ::Object */
            let fresh132 = pc;
            pc = pc.offset(1);
            a = *fresh132 as uint16_t
        }
        90 => {
            let fresh133 = pc;
            pc = pc.offset(1);
            a = *fresh133 as uint16_t;
            /* R(a) = newclass(R(a),Syms(b),R(a+1)) */
            let fresh134 = pc;
            pc = pc.offset(1);
            b = *fresh134 as uint16_t
        }
        91 => {
            let fresh135 = pc;
            pc = pc.offset(1);
            a = *fresh135 as uint16_t;
            /* R(a) = newmodule(R(a),Syms(b)) */
            let fresh136 = pc;
            pc = pc.offset(1);
            b = *fresh136 as uint16_t
        }
        92 => {
            let fresh137 = pc;
            pc = pc.offset(1);
            a = *fresh137 as uint16_t;
            /* R(a) = blockexec(R(a),SEQ[b]) */
            let fresh138 = pc;
            pc = pc.offset(1);
            b = *fresh138 as uint16_t
        }
        93 => {
            let fresh139 = pc;
            pc = pc.offset(1);
            a = *fresh139 as uint16_t;
            /* R(a).newmethod(Syms(b),R(a+1)) */
            let fresh140 = pc;
            pc = pc.offset(1);
            b = *fresh140 as uint16_t
        }
        94 => {
            let fresh141 = pc;
            pc = pc.offset(1);
            a = *fresh141 as uint16_t;
            /* alias_method(target_class,Syms(a),Syms(b)) */
            let fresh142 = pc;
            pc = pc.offset(1);
            b = *fresh142 as uint16_t
        }
        95 => {
            /* undef_method(target_class,Syms(a)) */
            let fresh143 = pc;
            pc = pc.offset(1);
            a = *fresh143 as uint16_t
        }
        96 => {
            /* R(a) = R(a).singleton_class */
            let fresh144 = pc;
            pc = pc.offset(1);
            a = *fresh144 as uint16_t
        }
        97 => {
            /* R(a) = target_class */
            let fresh145 = pc;
            pc = pc.offset(1);
            a = *fresh145 as uint16_t
        }
        98 => {
            let fresh146 = pc;
            pc = pc.offset(1);
            a = *fresh146 as uint16_t;
            let fresh147 = pc;
            pc = pc.offset(1);
            b = *fresh147 as uint16_t;
            /* print a,b,c */
            let fresh148 = pc;
            pc = pc.offset(1);
            c = *fresh148
        }
        99 => {
            /* raise(LocalJumpError, Lit(a)) */
            let fresh149 = pc;
            pc = pc.offset(1);
            a = *fresh149 as uint16_t
        }
        0 | 48 | 53 | 100 | 101 | 102 | 103 | _ => { }
    }
    /* empty */
    match insn as libc::c_int {
        100 => {
            let fresh150 = pc;
            pc = pc.offset(1);
            insn = *fresh150;
            match insn as libc::c_int {
                1 => {
                    pc = pc.offset(2isize);
                    a =
                        ((*pc.offset(-2isize).offset(0isize) as libc::c_int)
                             << 8i32 |
                             *pc.offset(-2isize).offset(1isize) as
                                 libc::c_int) as uint16_t;
                    /* R(a) = R(b) */
                    let fresh151 = pc;
                    pc = pc.offset(1);
                    b = *fresh151 as uint16_t
                }
                2 => {
                    pc = pc.offset(2isize);
                    a =
                        ((*pc.offset(-2isize).offset(0isize) as libc::c_int)
                             << 8i32 |
                             *pc.offset(-2isize).offset(1isize) as
                                 libc::c_int) as uint16_t;
                    /* R(a) = Pool(b) */
                    let fresh152 = pc;
                    pc = pc.offset(1);
                    b = *fresh152 as uint16_t
                }
                3 => {
                    pc = pc.offset(2isize);
                    a =
                        ((*pc.offset(-2isize).offset(0isize) as libc::c_int)
                             << 8i32 |
                             *pc.offset(-2isize).offset(1isize) as
                                 libc::c_int) as uint16_t;
                    /* R(a) = mrb_int(b) */
                    let fresh153 = pc;
                    pc = pc.offset(1);
                    b = *fresh153 as uint16_t
                }
                4 => {
                    pc = pc.offset(2isize);
                    a =
                        ((*pc.offset(-2isize).offset(0isize) as libc::c_int)
                             << 8i32 |
                             *pc.offset(-2isize).offset(1isize) as
                                 libc::c_int) as uint16_t;
                    /* R(a) = mrb_int(-b) */
                    let fresh154 = pc;
                    pc = pc.offset(1);
                    b = *fresh154 as uint16_t
                }
                5 => {
                    /* R(a) = mrb_int(-1) */
                    pc = pc.offset(2isize);
                    a =
                        ((*pc.offset(-2isize).offset(0isize) as libc::c_int)
                             << 8i32 |
                             *pc.offset(-2isize).offset(1isize) as
                                 libc::c_int) as uint16_t
                }
                6 => {
                    /* R(a) = mrb_int(0) */
                    pc = pc.offset(2isize);
                    a =
                        ((*pc.offset(-2isize).offset(0isize) as libc::c_int)
                             << 8i32 |
                             *pc.offset(-2isize).offset(1isize) as
                                 libc::c_int) as uint16_t
                }
                7 => {
                    /* R(a) = mrb_int(1) */
                    pc = pc.offset(2isize);
                    a =
                        ((*pc.offset(-2isize).offset(0isize) as libc::c_int)
                             << 8i32 |
                             *pc.offset(-2isize).offset(1isize) as
                                 libc::c_int) as uint16_t
                }
                8 => {
                    /* R(a) = mrb_int(2) */
                    pc = pc.offset(2isize);
                    a =
                        ((*pc.offset(-2isize).offset(0isize) as libc::c_int)
                             << 8i32 |
                             *pc.offset(-2isize).offset(1isize) as
                                 libc::c_int) as uint16_t
                }
                9 => {
                    /* R(a) = mrb_int(3) */
                    pc = pc.offset(2isize);
                    a =
                        ((*pc.offset(-2isize).offset(0isize) as libc::c_int)
                             << 8i32 |
                             *pc.offset(-2isize).offset(1isize) as
                                 libc::c_int) as uint16_t
                }
                10 => {
                    /* R(a) = mrb_int(4) */
                    pc = pc.offset(2isize);
                    a =
                        ((*pc.offset(-2isize).offset(0isize) as libc::c_int)
                             << 8i32 |
                             *pc.offset(-2isize).offset(1isize) as
                                 libc::c_int) as uint16_t
                }
                11 => {
                    /* R(a) = mrb_int(5) */
                    pc = pc.offset(2isize);
                    a =
                        ((*pc.offset(-2isize).offset(0isize) as libc::c_int)
                             << 8i32 |
                             *pc.offset(-2isize).offset(1isize) as
                                 libc::c_int) as uint16_t
                }
                12 => {
                    /* R(a) = mrb_int(6) */
                    pc = pc.offset(2isize);
                    a =
                        ((*pc.offset(-2isize).offset(0isize) as libc::c_int)
                             << 8i32 |
                             *pc.offset(-2isize).offset(1isize) as
                                 libc::c_int) as uint16_t
                }
                13 => {
                    /* R(a) = mrb_int(7) */
                    pc = pc.offset(2isize);
                    a =
                        ((*pc.offset(-2isize).offset(0isize) as libc::c_int)
                             << 8i32 |
                             *pc.offset(-2isize).offset(1isize) as
                                 libc::c_int) as uint16_t
                }
                14 => {
                    pc = pc.offset(2isize);
                    a =
                        ((*pc.offset(-2isize).offset(0isize) as libc::c_int)
                             << 8i32 |
                             *pc.offset(-2isize).offset(1isize) as
                                 libc::c_int) as uint16_t;
                    /* R(a) = Syms(b) */
                    let fresh155 = pc;
                    pc = pc.offset(1);
                    b = *fresh155 as uint16_t
                }
                15 => {
                    /* R(a) = nil */
                    pc = pc.offset(2isize);
                    a =
                        ((*pc.offset(-2isize).offset(0isize) as libc::c_int)
                             << 8i32 |
                             *pc.offset(-2isize).offset(1isize) as
                                 libc::c_int) as uint16_t
                }
                16 => {
                    /* R(a) = self */
                    pc = pc.offset(2isize);
                    a =
                        ((*pc.offset(-2isize).offset(0isize) as libc::c_int)
                             << 8i32 |
                             *pc.offset(-2isize).offset(1isize) as
                                 libc::c_int) as uint16_t
                }
                17 => {
                    /* R(a) = true */
                    pc = pc.offset(2isize);
                    a =
                        ((*pc.offset(-2isize).offset(0isize) as libc::c_int)
                             << 8i32 |
                             *pc.offset(-2isize).offset(1isize) as
                                 libc::c_int) as uint16_t
                }
                18 => {
                    /* R(a) = false */
                    pc = pc.offset(2isize);
                    a =
                        ((*pc.offset(-2isize).offset(0isize) as libc::c_int)
                             << 8i32 |
                             *pc.offset(-2isize).offset(1isize) as
                                 libc::c_int) as uint16_t
                }
                19 => {
                    pc = pc.offset(2isize);
                    a =
                        ((*pc.offset(-2isize).offset(0isize) as libc::c_int)
                             << 8i32 |
                             *pc.offset(-2isize).offset(1isize) as
                                 libc::c_int) as uint16_t;
                    /* R(a) = getglobal(Syms(b)) */
                    let fresh156 = pc;
                    pc = pc.offset(1);
                    b = *fresh156 as uint16_t
                }
                20 => {
                    pc = pc.offset(2isize);
                    a =
                        ((*pc.offset(-2isize).offset(0isize) as libc::c_int)
                             << 8i32 |
                             *pc.offset(-2isize).offset(1isize) as
                                 libc::c_int) as uint16_t;
                    /* setglobal(Syms(b), R(a)) */
                    let fresh157 = pc;
                    pc = pc.offset(1);
                    b = *fresh157 as uint16_t
                }
                21 => {
                    pc = pc.offset(2isize);
                    a =
                        ((*pc.offset(-2isize).offset(0isize) as libc::c_int)
                             << 8i32 |
                             *pc.offset(-2isize).offset(1isize) as
                                 libc::c_int) as uint16_t;
                    /* R(a) = Special[Syms(b)] */
                    let fresh158 = pc;
                    pc = pc.offset(1);
                    b = *fresh158 as uint16_t
                }
                22 => {
                    pc = pc.offset(2isize);
                    a =
                        ((*pc.offset(-2isize).offset(0isize) as libc::c_int)
                             << 8i32 |
                             *pc.offset(-2isize).offset(1isize) as
                                 libc::c_int) as uint16_t;
                    /* Special[Syms(b)] = R(a) */
                    let fresh159 = pc;
                    pc = pc.offset(1);
                    b = *fresh159 as uint16_t
                }
                23 => {
                    pc = pc.offset(2isize);
                    a =
                        ((*pc.offset(-2isize).offset(0isize) as libc::c_int)
                             << 8i32 |
                             *pc.offset(-2isize).offset(1isize) as
                                 libc::c_int) as uint16_t;
                    /* R(a) = ivget(Syms(b)) */
                    let fresh160 = pc;
                    pc = pc.offset(1);
                    b = *fresh160 as uint16_t
                }
                24 => {
                    pc = pc.offset(2isize);
                    a =
                        ((*pc.offset(-2isize).offset(0isize) as libc::c_int)
                             << 8i32 |
                             *pc.offset(-2isize).offset(1isize) as
                                 libc::c_int) as uint16_t;
                    /* ivset(Syms(b),R(a)) */
                    let fresh161 = pc;
                    pc = pc.offset(1);
                    b = *fresh161 as uint16_t
                }
                25 => {
                    pc = pc.offset(2isize);
                    a =
                        ((*pc.offset(-2isize).offset(0isize) as libc::c_int)
                             << 8i32 |
                             *pc.offset(-2isize).offset(1isize) as
                                 libc::c_int) as uint16_t;
                    /* R(a) = cvget(Syms(b)) */
                    let fresh162 = pc;
                    pc = pc.offset(1);
                    b = *fresh162 as uint16_t
                }
                26 => {
                    pc = pc.offset(2isize);
                    a =
                        ((*pc.offset(-2isize).offset(0isize) as libc::c_int)
                             << 8i32 |
                             *pc.offset(-2isize).offset(1isize) as
                                 libc::c_int) as uint16_t;
                    /* cvset(Syms(b),R(a)) */
                    let fresh163 = pc;
                    pc = pc.offset(1);
                    b = *fresh163 as uint16_t
                }
                27 => {
                    pc = pc.offset(2isize);
                    a =
                        ((*pc.offset(-2isize).offset(0isize) as libc::c_int)
                             << 8i32 |
                             *pc.offset(-2isize).offset(1isize) as
                                 libc::c_int) as uint16_t;
                    /* R(a) = constget(Syms(b)) */
                    let fresh164 = pc;
                    pc = pc.offset(1);
                    b = *fresh164 as uint16_t
                }
                28 => {
                    pc = pc.offset(2isize);
                    a =
                        ((*pc.offset(-2isize).offset(0isize) as libc::c_int)
                             << 8i32 |
                             *pc.offset(-2isize).offset(1isize) as
                                 libc::c_int) as uint16_t;
                    /* constset(Syms(b),R(a)) */
                    let fresh165 = pc;
                    pc = pc.offset(1);
                    b = *fresh165 as uint16_t
                }
                29 => {
                    pc = pc.offset(2isize);
                    a =
                        ((*pc.offset(-2isize).offset(0isize) as libc::c_int)
                             << 8i32 |
                             *pc.offset(-2isize).offset(1isize) as
                                 libc::c_int) as uint16_t;
                    /* R(a) = R(a)::Syms(b) */
                    let fresh166 = pc;
                    pc = pc.offset(1);
                    b = *fresh166 as uint16_t
                }
                30 => {
                    pc = pc.offset(2isize);
                    a =
                        ((*pc.offset(-2isize).offset(0isize) as libc::c_int)
                             << 8i32 |
                             *pc.offset(-2isize).offset(1isize) as
                                 libc::c_int) as uint16_t;
                    /* R(a+1)::Syms(b) = R(a) */
                    let fresh167 = pc;
                    pc = pc.offset(1);
                    b = *fresh167 as uint16_t
                }
                31 => {
                    pc = pc.offset(2isize);
                    a =
                        ((*pc.offset(-2isize).offset(0isize) as libc::c_int)
                             << 8i32 |
                             *pc.offset(-2isize).offset(1isize) as
                                 libc::c_int) as uint16_t;
                    let fresh168 = pc;
                    pc = pc.offset(1);
                    b = *fresh168 as uint16_t;
                    /* R(a) = uvget(b,c) */
                    let fresh169 = pc;
                    pc = pc.offset(1);
                    c = *fresh169
                }
                32 => {
                    pc = pc.offset(2isize);
                    a =
                        ((*pc.offset(-2isize).offset(0isize) as libc::c_int)
                             << 8i32 |
                             *pc.offset(-2isize).offset(1isize) as
                                 libc::c_int) as uint16_t;
                    let fresh170 = pc;
                    pc = pc.offset(1);
                    b = *fresh170 as uint16_t;
                    /* uvset(b,c,R(a)) */
                    let fresh171 = pc;
                    pc = pc.offset(1);
                    c = *fresh171
                }
                33 => {
                    /* pc=a */
                    pc = pc.offset(2isize);
                    a =
                        ((*pc.offset(-2isize).offset(0isize) as libc::c_int)
                             << 8i32 |
                             *pc.offset(-2isize).offset(1isize) as
                                 libc::c_int) as uint16_t
                }
                34 => {
                    pc = pc.offset(2isize);
                    a =
                        ((*pc.offset(-2isize).offset(0isize) as libc::c_int)
                             << 8i32 |
                             *pc.offset(-2isize).offset(1isize) as
                                 libc::c_int) as uint16_t;
                    /* if R(b) pc=a */
                    pc = pc.offset(2isize);
                    b =
                        ((*pc.offset(-2isize).offset(0isize) as libc::c_int)
                             << 8i32 |
                             *pc.offset(-2isize).offset(1isize) as
                                 libc::c_int) as uint16_t
                }
                35 => {
                    pc = pc.offset(2isize);
                    a =
                        ((*pc.offset(-2isize).offset(0isize) as libc::c_int)
                             << 8i32 |
                             *pc.offset(-2isize).offset(1isize) as
                                 libc::c_int) as uint16_t;
                    /* if !R(b) pc=a */
                    pc = pc.offset(2isize);
                    b =
                        ((*pc.offset(-2isize).offset(0isize) as libc::c_int)
                             << 8i32 |
                             *pc.offset(-2isize).offset(1isize) as
                                 libc::c_int) as uint16_t
                }
                36 => {
                    pc = pc.offset(2isize);
                    a =
                        ((*pc.offset(-2isize).offset(0isize) as libc::c_int)
                             << 8i32 |
                             *pc.offset(-2isize).offset(1isize) as
                                 libc::c_int) as uint16_t;
                    /* if R(b)==nil pc=a */
                    pc = pc.offset(2isize);
                    b =
                        ((*pc.offset(-2isize).offset(0isize) as libc::c_int)
                             << 8i32 |
                             *pc.offset(-2isize).offset(1isize) as
                                 libc::c_int) as uint16_t
                }
                37 => {
                    /* rescue_push(a) */
                    pc = pc.offset(2isize);
                    a =
                        ((*pc.offset(-2isize).offset(0isize) as libc::c_int)
                             << 8i32 |
                             *pc.offset(-2isize).offset(1isize) as
                                 libc::c_int) as uint16_t
                }
                38 => {
                    /* R(a) = exc */
                    pc = pc.offset(2isize);
                    a =
                        ((*pc.offset(-2isize).offset(0isize) as libc::c_int)
                             << 8i32 |
                             *pc.offset(-2isize).offset(1isize) as
                                 libc::c_int) as uint16_t
                }
                39 => {
                    pc = pc.offset(2isize);
                    a =
                        ((*pc.offset(-2isize).offset(0isize) as libc::c_int)
                             << 8i32 |
                             *pc.offset(-2isize).offset(1isize) as
                                 libc::c_int) as uint16_t;
                    /* R(b) = R(a).isa?(R(b)) */
                    let fresh172 = pc;
                    pc = pc.offset(1);
                    b = *fresh172 as uint16_t
                }
                40 => {
                    /* a.times{rescue_pop()} */
                    pc = pc.offset(2isize);
                    a =
                        ((*pc.offset(-2isize).offset(0isize) as libc::c_int)
                             << 8i32 |
                             *pc.offset(-2isize).offset(1isize) as
                                 libc::c_int) as uint16_t
                }
                41 => {
                    /* raise(R(a)) */
                    pc = pc.offset(2isize);
                    a =
                        ((*pc.offset(-2isize).offset(0isize) as libc::c_int)
                             << 8i32 |
                             *pc.offset(-2isize).offset(1isize) as
                                 libc::c_int) as uint16_t
                }
                42 => {
                    /* ensure_push(SEQ[a]) */
                    pc = pc.offset(2isize);
                    a =
                        ((*pc.offset(-2isize).offset(0isize) as libc::c_int)
                             << 8i32 |
                             *pc.offset(-2isize).offset(1isize) as
                                 libc::c_int) as uint16_t
                }
                43 => {
                    /* A.times{ensure_pop().call} */
                    pc = pc.offset(2isize);
                    a =
                        ((*pc.offset(-2isize).offset(0isize) as libc::c_int)
                             << 8i32 |
                             *pc.offset(-2isize).offset(1isize) as
                                 libc::c_int) as uint16_t
                }
                44 => {
                    pc = pc.offset(2isize);
                    a =
                        ((*pc.offset(-2isize).offset(0isize) as libc::c_int)
                             << 8i32 |
                             *pc.offset(-2isize).offset(1isize) as
                                 libc::c_int) as uint16_t;
                    /* R(a) = call(R(a),Syms(b),*R(a+1)) */
                    let fresh173 = pc;
                    pc = pc.offset(1);
                    b = *fresh173 as uint16_t
                }
                45 => {
                    pc = pc.offset(2isize);
                    a =
                        ((*pc.offset(-2isize).offset(0isize) as libc::c_int)
                             << 8i32 |
                             *pc.offset(-2isize).offset(1isize) as
                                 libc::c_int) as uint16_t;
                    /* R(a) = call(R(a),Syms(b),*R(a+1),&R(a+2)) */
                    let fresh174 = pc;
                    pc = pc.offset(1);
                    b = *fresh174 as uint16_t
                }
                46 => {
                    pc = pc.offset(2isize);
                    a =
                        ((*pc.offset(-2isize).offset(0isize) as libc::c_int)
                             << 8i32 |
                             *pc.offset(-2isize).offset(1isize) as
                                 libc::c_int) as uint16_t;
                    let fresh175 = pc;
                    pc = pc.offset(1);
                    b = *fresh175 as uint16_t;
                    /* R(a) = call(R(a),Syms(b),R(a+1),...,R(a+c)) */
                    let fresh176 = pc;
                    pc = pc.offset(1);
                    c = *fresh176
                }
                47 => {
                    pc = pc.offset(2isize);
                    a =
                        ((*pc.offset(-2isize).offset(0isize) as libc::c_int)
                             << 8i32 |
                             *pc.offset(-2isize).offset(1isize) as
                                 libc::c_int) as uint16_t;
                    let fresh177 = pc;
                    pc = pc.offset(1);
                    b = *fresh177 as uint16_t;
                    /* R(a) = call(R(a),Syms(Bx),R(a+1),...,R(a+c),&R(a+c+1)) */
                    let fresh178 = pc;
                    pc = pc.offset(1);
                    c = *fresh178
                }
                49 => {
                    pc = pc.offset(2isize);
                    a =
                        ((*pc.offset(-2isize).offset(0isize) as libc::c_int)
                             << 8i32 |
                             *pc.offset(-2isize).offset(1isize) as
                                 libc::c_int) as uint16_t;
                    /* R(a) = super(R(a+1),... ,R(a+b+1)) */
                    let fresh179 = pc;
                    pc = pc.offset(1);
                    b = *fresh179 as uint16_t
                }
                50 => {
                    pc = pc.offset(2isize);
                    a =
                        ((*pc.offset(-2isize).offset(0isize) as libc::c_int)
                             << 8i32 |
                             *pc.offset(-2isize).offset(1isize) as
                                 libc::c_int) as uint16_t;
                    /* R(a) = argument array (16=m5:r1:m5:d1:lv4) */
                    pc = pc.offset(2isize);
                    b =
                        ((*pc.offset(-2isize).offset(0isize) as libc::c_int)
                             << 8i32 |
                             *pc.offset(-2isize).offset(1isize) as
                                 libc::c_int) as uint16_t
                }
                51 => {
                    /* arg setup according to flags (23=m5:o5:r1:m5:k5:d1:b1) */
                    pc = pc.offset(3isize);
                    a =
                        ((*pc.offset(-3isize).offset(0isize) as libc::c_int)
                             << 16i32 |
                             (*pc.offset(-3isize).offset(1isize) as
                                  libc::c_int) << 8i32 |
                             *pc.offset(-3isize).offset(2isize) as
                                 libc::c_int) as uint16_t
                }
                52 => {
                    pc = pc.offset(2isize);
                    a =
                        ((*pc.offset(-2isize).offset(0isize) as libc::c_int)
                             << 8i32 |
                             *pc.offset(-2isize).offset(1isize) as
                                 libc::c_int) as uint16_t;
                    /* R(a) = kdict.key?(Syms(b))                      # todo */
                    let fresh180 = pc;
                    pc = pc.offset(1);
                    b = *fresh180 as uint16_t
                }
                54 => {
                    pc = pc.offset(2isize);
                    a =
                        ((*pc.offset(-2isize).offset(0isize) as libc::c_int)
                             << 8i32 |
                             *pc.offset(-2isize).offset(1isize) as
                                 libc::c_int) as uint16_t;
                    /* R(a) = kdict[Syms(b)]; kdict.delete(Syms(b))    # todo */
                    let fresh181 = pc;
                    pc = pc.offset(1);
                    b = *fresh181 as uint16_t
                }
                55 => {
                    /* return R(a) (normal) */
                    pc = pc.offset(2isize);
                    a =
                        ((*pc.offset(-2isize).offset(0isize) as libc::c_int)
                             << 8i32 |
                             *pc.offset(-2isize).offset(1isize) as
                                 libc::c_int) as uint16_t
                }
                56 => {
                    /* return R(a) (in-block return) */
                    pc = pc.offset(2isize);
                    a =
                        ((*pc.offset(-2isize).offset(0isize) as libc::c_int)
                             << 8i32 |
                             *pc.offset(-2isize).offset(1isize) as
                                 libc::c_int) as uint16_t
                }
                57 => {
                    /* break R(a) */
                    pc = pc.offset(2isize);
                    a =
                        ((*pc.offset(-2isize).offset(0isize) as libc::c_int)
                             << 8i32 |
                             *pc.offset(-2isize).offset(1isize) as
                                 libc::c_int) as uint16_t
                }
                58 => {
                    pc = pc.offset(2isize);
                    a =
                        ((*pc.offset(-2isize).offset(0isize) as libc::c_int)
                             << 8i32 |
                             *pc.offset(-2isize).offset(1isize) as
                                 libc::c_int) as uint16_t;
                    /* R(a) = block (16=m5:r1:m5:d1:lv4) */
                    pc = pc.offset(2isize);
                    b =
                        ((*pc.offset(-2isize).offset(0isize) as libc::c_int)
                             << 8i32 |
                             *pc.offset(-2isize).offset(1isize) as
                                 libc::c_int) as uint16_t
                }
                59 => {
                    /* R(a) = R(a)+R(a+1) */
                    pc = pc.offset(2isize);
                    a =
                        ((*pc.offset(-2isize).offset(0isize) as libc::c_int)
                             << 8i32 |
                             *pc.offset(-2isize).offset(1isize) as
                                 libc::c_int) as uint16_t
                }
                60 => {
                    pc = pc.offset(2isize);
                    a =
                        ((*pc.offset(-2isize).offset(0isize) as libc::c_int)
                             << 8i32 |
                             *pc.offset(-2isize).offset(1isize) as
                                 libc::c_int) as uint16_t;
                    /* R(a) = R(a)+mrb_int(c)  */
                    let fresh182 = pc;
                    pc = pc.offset(1);
                    b = *fresh182 as uint16_t
                }
                61 => {
                    /* R(a) = R(a)-R(a+1) */
                    pc = pc.offset(2isize);
                    a =
                        ((*pc.offset(-2isize).offset(0isize) as libc::c_int)
                             << 8i32 |
                             *pc.offset(-2isize).offset(1isize) as
                                 libc::c_int) as uint16_t
                }
                62 => {
                    pc = pc.offset(2isize);
                    a =
                        ((*pc.offset(-2isize).offset(0isize) as libc::c_int)
                             << 8i32 |
                             *pc.offset(-2isize).offset(1isize) as
                                 libc::c_int) as uint16_t;
                    /* R(a) = R(a)-C */
                    let fresh183 = pc;
                    pc = pc.offset(1);
                    b = *fresh183 as uint16_t
                }
                63 => {
                    /* R(a) = R(a)*R(a+1) */
                    pc = pc.offset(2isize);
                    a =
                        ((*pc.offset(-2isize).offset(0isize) as libc::c_int)
                             << 8i32 |
                             *pc.offset(-2isize).offset(1isize) as
                                 libc::c_int) as uint16_t
                }
                64 => {
                    /* R(a) = R(a)/R(a+1) */
                    pc = pc.offset(2isize);
                    a =
                        ((*pc.offset(-2isize).offset(0isize) as libc::c_int)
                             << 8i32 |
                             *pc.offset(-2isize).offset(1isize) as
                                 libc::c_int) as uint16_t
                }
                65 => {
                    /* R(a) = R(a)==R(a+1) */
                    pc = pc.offset(2isize);
                    a =
                        ((*pc.offset(-2isize).offset(0isize) as libc::c_int)
                             << 8i32 |
                             *pc.offset(-2isize).offset(1isize) as
                                 libc::c_int) as uint16_t
                }
                66 => {
                    /* R(a) = R(a)<R(a+1) */
                    pc = pc.offset(2isize);
                    a =
                        ((*pc.offset(-2isize).offset(0isize) as libc::c_int)
                             << 8i32 |
                             *pc.offset(-2isize).offset(1isize) as
                                 libc::c_int) as uint16_t
                }
                67 => {
                    /* R(a) = R(a)<=R(a+1) */
                    pc = pc.offset(2isize);
                    a =
                        ((*pc.offset(-2isize).offset(0isize) as libc::c_int)
                             << 8i32 |
                             *pc.offset(-2isize).offset(1isize) as
                                 libc::c_int) as uint16_t
                }
                68 => {
                    /* R(a) = R(a)>R(a+1) */
                    pc = pc.offset(2isize);
                    a =
                        ((*pc.offset(-2isize).offset(0isize) as libc::c_int)
                             << 8i32 |
                             *pc.offset(-2isize).offset(1isize) as
                                 libc::c_int) as uint16_t
                }
                69 => {
                    /* R(a) = R(a)>=R(a+1) */
                    pc = pc.offset(2isize);
                    a =
                        ((*pc.offset(-2isize).offset(0isize) as libc::c_int)
                             << 8i32 |
                             *pc.offset(-2isize).offset(1isize) as
                                 libc::c_int) as uint16_t
                }
                70 => {
                    pc = pc.offset(2isize);
                    a =
                        ((*pc.offset(-2isize).offset(0isize) as libc::c_int)
                             << 8i32 |
                             *pc.offset(-2isize).offset(1isize) as
                                 libc::c_int) as uint16_t;
                    /* R(a) = ary_new(R(a),R(a+1)..R(a+b)) */
                    let fresh184 = pc;
                    pc = pc.offset(1);
                    b = *fresh184 as uint16_t
                }
                71 => {
                    pc = pc.offset(2isize);
                    a =
                        ((*pc.offset(-2isize).offset(0isize) as libc::c_int)
                             << 8i32 |
                             *pc.offset(-2isize).offset(1isize) as
                                 libc::c_int) as uint16_t;
                    let fresh185 = pc;
                    pc = pc.offset(1);
                    b = *fresh185 as uint16_t;
                    /* R(a) = ary_new(R(b),R(b+1)..R(b+c)) */
                    let fresh186 = pc;
                    pc = pc.offset(1);
                    c = *fresh186
                }
                72 => {
                    /* ary_cat(R(a),R(a+1)) */
                    pc = pc.offset(2isize);
                    a =
                        ((*pc.offset(-2isize).offset(0isize) as libc::c_int)
                             << 8i32 |
                             *pc.offset(-2isize).offset(1isize) as
                                 libc::c_int) as uint16_t
                }
                73 => {
                    /* ary_push(R(a),R(a+1)) */
                    pc = pc.offset(2isize);
                    a =
                        ((*pc.offset(-2isize).offset(0isize) as libc::c_int)
                             << 8i32 |
                             *pc.offset(-2isize).offset(1isize) as
                                 libc::c_int) as uint16_t
                }
                74 => {
                    /* R(a) = ary_dup(R(a)) */
                    pc = pc.offset(2isize);
                    a =
                        ((*pc.offset(-2isize).offset(0isize) as libc::c_int)
                             << 8i32 |
                             *pc.offset(-2isize).offset(1isize) as
                                 libc::c_int) as uint16_t
                }
                75 => {
                    pc = pc.offset(2isize);
                    a =
                        ((*pc.offset(-2isize).offset(0isize) as libc::c_int)
                             << 8i32 |
                             *pc.offset(-2isize).offset(1isize) as
                                 libc::c_int) as uint16_t;
                    let fresh187 = pc;
                    pc = pc.offset(1);
                    b = *fresh187 as uint16_t;
                    /* R(a) = R(b)[c] */
                    let fresh188 = pc;
                    pc = pc.offset(1);
                    c = *fresh188
                }
                76 => {
                    pc = pc.offset(2isize);
                    a =
                        ((*pc.offset(-2isize).offset(0isize) as libc::c_int)
                             << 8i32 |
                             *pc.offset(-2isize).offset(1isize) as
                                 libc::c_int) as uint16_t;
                    let fresh189 = pc;
                    pc = pc.offset(1);
                    b = *fresh189 as uint16_t;
                    /* R(a)[c] = R(b) */
                    let fresh190 = pc;
                    pc = pc.offset(1);
                    c = *fresh190
                }
                77 => {
                    pc = pc.offset(2isize);
                    a =
                        ((*pc.offset(-2isize).offset(0isize) as libc::c_int)
                             << 8i32 |
                             *pc.offset(-2isize).offset(1isize) as
                                 libc::c_int) as uint16_t;
                    let fresh191 = pc;
                    pc = pc.offset(1);
                    b = *fresh191 as uint16_t;
                    /* *R(a),R(a+1)..R(a+c) = R(a)[b..] */
                    let fresh192 = pc;
                    pc = pc.offset(1);
                    c = *fresh192
                }
                78 => {
                    /* R(a) = intern(R(a)) */
                    pc = pc.offset(2isize);
                    a =
                        ((*pc.offset(-2isize).offset(0isize) as libc::c_int)
                             << 8i32 |
                             *pc.offset(-2isize).offset(1isize) as
                                 libc::c_int) as uint16_t
                }
                79 => {
                    pc = pc.offset(2isize);
                    a =
                        ((*pc.offset(-2isize).offset(0isize) as libc::c_int)
                             << 8i32 |
                             *pc.offset(-2isize).offset(1isize) as
                                 libc::c_int) as uint16_t;
                    /* R(a) = str_dup(Lit(b)) */
                    let fresh193 = pc;
                    pc = pc.offset(1);
                    b = *fresh193 as uint16_t
                }
                80 => {
                    /* str_cat(R(a),R(a+1)) */
                    pc = pc.offset(2isize);
                    a =
                        ((*pc.offset(-2isize).offset(0isize) as libc::c_int)
                             << 8i32 |
                             *pc.offset(-2isize).offset(1isize) as
                                 libc::c_int) as uint16_t
                }
                81 => {
                    pc = pc.offset(2isize);
                    a =
                        ((*pc.offset(-2isize).offset(0isize) as libc::c_int)
                             << 8i32 |
                             *pc.offset(-2isize).offset(1isize) as
                                 libc::c_int) as uint16_t;
                    /* R(a) = hash_new(R(a),R(a+1)..R(a+b)) */
                    let fresh194 = pc;
                    pc = pc.offset(1);
                    b = *fresh194 as uint16_t
                }
                82 => {
                    pc = pc.offset(2isize);
                    a =
                        ((*pc.offset(-2isize).offset(0isize) as libc::c_int)
                             << 8i32 |
                             *pc.offset(-2isize).offset(1isize) as
                                 libc::c_int) as uint16_t;
                    /* R(a) = hash_push(R(a),R(a+1)..R(a+b)) */
                    let fresh195 = pc;
                    pc = pc.offset(1);
                    b = *fresh195 as uint16_t
                }
                83 => {
                    /* R(a) = hash_cat(R(a),R(a+1)) */
                    pc = pc.offset(2isize);
                    a =
                        ((*pc.offset(-2isize).offset(0isize) as libc::c_int)
                             << 8i32 |
                             *pc.offset(-2isize).offset(1isize) as
                                 libc::c_int) as uint16_t
                }
                84 => {
                    pc = pc.offset(2isize);
                    a =
                        ((*pc.offset(-2isize).offset(0isize) as libc::c_int)
                             << 8i32 |
                             *pc.offset(-2isize).offset(1isize) as
                                 libc::c_int) as uint16_t;
                    /* R(a) = lambda(SEQ[b],L_LAMBDA) */
                    let fresh196 = pc;
                    pc = pc.offset(1);
                    b = *fresh196 as uint16_t
                }
                85 => {
                    pc = pc.offset(2isize);
                    a =
                        ((*pc.offset(-2isize).offset(0isize) as libc::c_int)
                             << 8i32 |
                             *pc.offset(-2isize).offset(1isize) as
                                 libc::c_int) as uint16_t;
                    /* R(a) = lambda(SEQ[b],L_BLOCK) */
                    let fresh197 = pc;
                    pc = pc.offset(1);
                    b = *fresh197 as uint16_t
                }
                86 => {
                    pc = pc.offset(2isize);
                    a =
                        ((*pc.offset(-2isize).offset(0isize) as libc::c_int)
                             << 8i32 |
                             *pc.offset(-2isize).offset(1isize) as
                                 libc::c_int) as uint16_t;
                    /* R(a) = lambda(SEQ[b],L_METHOD) */
                    let fresh198 = pc;
                    pc = pc.offset(1);
                    b = *fresh198 as uint16_t
                }
                87 => {
                    /* R(a) = range_new(R(a),R(a+1),FALSE) */
                    pc = pc.offset(2isize);
                    a =
                        ((*pc.offset(-2isize).offset(0isize) as libc::c_int)
                             << 8i32 |
                             *pc.offset(-2isize).offset(1isize) as
                                 libc::c_int) as uint16_t
                }
                88 => {
                    /* R(a) = range_new(R(a),R(a+1),TRUE) */
                    pc = pc.offset(2isize);
                    a =
                        ((*pc.offset(-2isize).offset(0isize) as libc::c_int)
                             << 8i32 |
                             *pc.offset(-2isize).offset(1isize) as
                                 libc::c_int) as uint16_t
                }
                89 => {
                    /* R(a) = ::Object */
                    pc = pc.offset(2isize);
                    a =
                        ((*pc.offset(-2isize).offset(0isize) as libc::c_int)
                             << 8i32 |
                             *pc.offset(-2isize).offset(1isize) as
                                 libc::c_int) as uint16_t
                }
                90 => {
                    pc = pc.offset(2isize);
                    a =
                        ((*pc.offset(-2isize).offset(0isize) as libc::c_int)
                             << 8i32 |
                             *pc.offset(-2isize).offset(1isize) as
                                 libc::c_int) as uint16_t;
                    /* R(a) = newclass(R(a),Syms(b),R(a+1)) */
                    let fresh199 = pc;
                    pc = pc.offset(1);
                    b = *fresh199 as uint16_t
                }
                91 => {
                    pc = pc.offset(2isize);
                    a =
                        ((*pc.offset(-2isize).offset(0isize) as libc::c_int)
                             << 8i32 |
                             *pc.offset(-2isize).offset(1isize) as
                                 libc::c_int) as uint16_t;
                    /* R(a) = newmodule(R(a),Syms(b)) */
                    let fresh200 = pc;
                    pc = pc.offset(1);
                    b = *fresh200 as uint16_t
                }
                92 => {
                    pc = pc.offset(2isize);
                    a =
                        ((*pc.offset(-2isize).offset(0isize) as libc::c_int)
                             << 8i32 |
                             *pc.offset(-2isize).offset(1isize) as
                                 libc::c_int) as uint16_t;
                    /* R(a) = blockexec(R(a),SEQ[b]) */
                    let fresh201 = pc;
                    pc = pc.offset(1);
                    b = *fresh201 as uint16_t
                }
                93 => {
                    pc = pc.offset(2isize);
                    a =
                        ((*pc.offset(-2isize).offset(0isize) as libc::c_int)
                             << 8i32 |
                             *pc.offset(-2isize).offset(1isize) as
                                 libc::c_int) as uint16_t;
                    /* R(a).newmethod(Syms(b),R(a+1)) */
                    let fresh202 = pc;
                    pc = pc.offset(1);
                    b = *fresh202 as uint16_t
                }
                94 => {
                    pc = pc.offset(2isize);
                    a =
                        ((*pc.offset(-2isize).offset(0isize) as libc::c_int)
                             << 8i32 |
                             *pc.offset(-2isize).offset(1isize) as
                                 libc::c_int) as uint16_t;
                    /* alias_method(target_class,Syms(a),Syms(b)) */
                    let fresh203 = pc;
                    pc = pc.offset(1);
                    b = *fresh203 as uint16_t
                }
                95 => {
                    /* undef_method(target_class,Syms(a)) */
                    pc = pc.offset(2isize);
                    a =
                        ((*pc.offset(-2isize).offset(0isize) as libc::c_int)
                             << 8i32 |
                             *pc.offset(-2isize).offset(1isize) as
                                 libc::c_int) as uint16_t
                }
                96 => {
                    /* R(a) = R(a).singleton_class */
                    pc = pc.offset(2isize);
                    a =
                        ((*pc.offset(-2isize).offset(0isize) as libc::c_int)
                             << 8i32 |
                             *pc.offset(-2isize).offset(1isize) as
                                 libc::c_int) as uint16_t
                }
                97 => {
                    /* R(a) = target_class */
                    pc = pc.offset(2isize);
                    a =
                        ((*pc.offset(-2isize).offset(0isize) as libc::c_int)
                             << 8i32 |
                             *pc.offset(-2isize).offset(1isize) as
                                 libc::c_int) as uint16_t
                }
                98 => {
                    pc = pc.offset(2isize);
                    a =
                        ((*pc.offset(-2isize).offset(0isize) as libc::c_int)
                             << 8i32 |
                             *pc.offset(-2isize).offset(1isize) as
                                 libc::c_int) as uint16_t;
                    let fresh204 = pc;
                    pc = pc.offset(1);
                    b = *fresh204 as uint16_t;
                    /* print a,b,c */
                    let fresh205 = pc;
                    pc = pc.offset(1);
                    c = *fresh205
                }
                99 => {
                    /* raise(LocalJumpError, Lit(a)) */
                    pc = pc.offset(2isize);
                    a =
                        ((*pc.offset(-2isize).offset(0isize) as libc::c_int)
                             << 8i32 |
                             *pc.offset(-2isize).offset(1isize) as
                                 libc::c_int) as uint16_t
                }
                0 | 48 | 53 | 100 | 101 | 102 | 103 | _ => { }
            }
        }
        101 => {
            let fresh206 = pc;
            pc = pc.offset(1);
            insn = *fresh206;
            match insn as libc::c_int {
                1 => {
                    let fresh207 = pc;
                    pc = pc.offset(1);
                    a = *fresh207 as uint16_t;
                    /* R(a) = R(b) */
                    pc = pc.offset(2isize);
                    b =
                        ((*pc.offset(-2isize).offset(0isize) as libc::c_int)
                             << 8i32 |
                             *pc.offset(-2isize).offset(1isize) as
                                 libc::c_int) as uint16_t
                }
                2 => {
                    let fresh208 = pc;
                    pc = pc.offset(1);
                    a = *fresh208 as uint16_t;
                    /* R(a) = Pool(b) */
                    pc = pc.offset(2isize);
                    b =
                        ((*pc.offset(-2isize).offset(0isize) as libc::c_int)
                             << 8i32 |
                             *pc.offset(-2isize).offset(1isize) as
                                 libc::c_int) as uint16_t
                }
                3 => {
                    let fresh209 = pc;
                    pc = pc.offset(1);
                    a = *fresh209 as uint16_t;
                    /* R(a) = mrb_int(b) */
                    pc = pc.offset(2isize);
                    b =
                        ((*pc.offset(-2isize).offset(0isize) as libc::c_int)
                             << 8i32 |
                             *pc.offset(-2isize).offset(1isize) as
                                 libc::c_int) as uint16_t
                }
                4 => {
                    let fresh210 = pc;
                    pc = pc.offset(1);
                    a = *fresh210 as uint16_t;
                    /* R(a) = mrb_int(-b) */
                    pc = pc.offset(2isize);
                    b =
                        ((*pc.offset(-2isize).offset(0isize) as libc::c_int)
                             << 8i32 |
                             *pc.offset(-2isize).offset(1isize) as
                                 libc::c_int) as uint16_t
                }
                5 => {
                    /* R(a) = mrb_int(-1) */
                    let fresh211 = pc;
                    pc = pc.offset(1);
                    a = *fresh211 as uint16_t
                }
                6 => {
                    /* R(a) = mrb_int(0) */
                    let fresh212 = pc;
                    pc = pc.offset(1);
                    a = *fresh212 as uint16_t
                }
                7 => {
                    /* R(a) = mrb_int(1) */
                    let fresh213 = pc;
                    pc = pc.offset(1);
                    a = *fresh213 as uint16_t
                }
                8 => {
                    /* R(a) = mrb_int(2) */
                    let fresh214 = pc;
                    pc = pc.offset(1);
                    a = *fresh214 as uint16_t
                }
                9 => {
                    /* R(a) = mrb_int(3) */
                    let fresh215 = pc;
                    pc = pc.offset(1);
                    a = *fresh215 as uint16_t
                }
                10 => {
                    /* R(a) = mrb_int(4) */
                    let fresh216 = pc;
                    pc = pc.offset(1);
                    a = *fresh216 as uint16_t
                }
                11 => {
                    /* R(a) = mrb_int(5) */
                    let fresh217 = pc;
                    pc = pc.offset(1);
                    a = *fresh217 as uint16_t
                }
                12 => {
                    /* R(a) = mrb_int(6) */
                    let fresh218 = pc;
                    pc = pc.offset(1);
                    a = *fresh218 as uint16_t
                }
                13 => {
                    /* R(a) = mrb_int(7) */
                    let fresh219 = pc;
                    pc = pc.offset(1);
                    a = *fresh219 as uint16_t
                }
                14 => {
                    let fresh220 = pc;
                    pc = pc.offset(1);
                    a = *fresh220 as uint16_t;
                    /* R(a) = Syms(b) */
                    pc = pc.offset(2isize);
                    b =
                        ((*pc.offset(-2isize).offset(0isize) as libc::c_int)
                             << 8i32 |
                             *pc.offset(-2isize).offset(1isize) as
                                 libc::c_int) as uint16_t
                }
                15 => {
                    /* R(a) = nil */
                    let fresh221 = pc;
                    pc = pc.offset(1);
                    a = *fresh221 as uint16_t
                }
                16 => {
                    /* R(a) = self */
                    let fresh222 = pc;
                    pc = pc.offset(1);
                    a = *fresh222 as uint16_t
                }
                17 => {
                    /* R(a) = true */
                    let fresh223 = pc;
                    pc = pc.offset(1);
                    a = *fresh223 as uint16_t
                }
                18 => {
                    /* R(a) = false */
                    let fresh224 = pc;
                    pc = pc.offset(1);
                    a = *fresh224 as uint16_t
                }
                19 => {
                    let fresh225 = pc;
                    pc = pc.offset(1);
                    a = *fresh225 as uint16_t;
                    /* R(a) = getglobal(Syms(b)) */
                    pc = pc.offset(2isize);
                    b =
                        ((*pc.offset(-2isize).offset(0isize) as libc::c_int)
                             << 8i32 |
                             *pc.offset(-2isize).offset(1isize) as
                                 libc::c_int) as uint16_t
                }
                20 => {
                    let fresh226 = pc;
                    pc = pc.offset(1);
                    a = *fresh226 as uint16_t;
                    /* setglobal(Syms(b), R(a)) */
                    pc = pc.offset(2isize);
                    b =
                        ((*pc.offset(-2isize).offset(0isize) as libc::c_int)
                             << 8i32 |
                             *pc.offset(-2isize).offset(1isize) as
                                 libc::c_int) as uint16_t
                }
                21 => {
                    let fresh227 = pc;
                    pc = pc.offset(1);
                    a = *fresh227 as uint16_t;
                    /* R(a) = Special[Syms(b)] */
                    pc = pc.offset(2isize);
                    b =
                        ((*pc.offset(-2isize).offset(0isize) as libc::c_int)
                             << 8i32 |
                             *pc.offset(-2isize).offset(1isize) as
                                 libc::c_int) as uint16_t
                }
                22 => {
                    let fresh228 = pc;
                    pc = pc.offset(1);
                    a = *fresh228 as uint16_t;
                    /* Special[Syms(b)] = R(a) */
                    pc = pc.offset(2isize);
                    b =
                        ((*pc.offset(-2isize).offset(0isize) as libc::c_int)
                             << 8i32 |
                             *pc.offset(-2isize).offset(1isize) as
                                 libc::c_int) as uint16_t
                }
                23 => {
                    let fresh229 = pc;
                    pc = pc.offset(1);
                    a = *fresh229 as uint16_t;
                    /* R(a) = ivget(Syms(b)) */
                    pc = pc.offset(2isize);
                    b =
                        ((*pc.offset(-2isize).offset(0isize) as libc::c_int)
                             << 8i32 |
                             *pc.offset(-2isize).offset(1isize) as
                                 libc::c_int) as uint16_t
                }
                24 => {
                    let fresh230 = pc;
                    pc = pc.offset(1);
                    a = *fresh230 as uint16_t;
                    /* ivset(Syms(b),R(a)) */
                    pc = pc.offset(2isize);
                    b =
                        ((*pc.offset(-2isize).offset(0isize) as libc::c_int)
                             << 8i32 |
                             *pc.offset(-2isize).offset(1isize) as
                                 libc::c_int) as uint16_t
                }
                25 => {
                    let fresh231 = pc;
                    pc = pc.offset(1);
                    a = *fresh231 as uint16_t;
                    /* R(a) = cvget(Syms(b)) */
                    pc = pc.offset(2isize);
                    b =
                        ((*pc.offset(-2isize).offset(0isize) as libc::c_int)
                             << 8i32 |
                             *pc.offset(-2isize).offset(1isize) as
                                 libc::c_int) as uint16_t
                }
                26 => {
                    let fresh232 = pc;
                    pc = pc.offset(1);
                    a = *fresh232 as uint16_t;
                    /* cvset(Syms(b),R(a)) */
                    pc = pc.offset(2isize);
                    b =
                        ((*pc.offset(-2isize).offset(0isize) as libc::c_int)
                             << 8i32 |
                             *pc.offset(-2isize).offset(1isize) as
                                 libc::c_int) as uint16_t
                }
                27 => {
                    let fresh233 = pc;
                    pc = pc.offset(1);
                    a = *fresh233 as uint16_t;
                    /* R(a) = constget(Syms(b)) */
                    pc = pc.offset(2isize);
                    b =
                        ((*pc.offset(-2isize).offset(0isize) as libc::c_int)
                             << 8i32 |
                             *pc.offset(-2isize).offset(1isize) as
                                 libc::c_int) as uint16_t
                }
                28 => {
                    let fresh234 = pc;
                    pc = pc.offset(1);
                    a = *fresh234 as uint16_t;
                    /* constset(Syms(b),R(a)) */
                    pc = pc.offset(2isize);
                    b =
                        ((*pc.offset(-2isize).offset(0isize) as libc::c_int)
                             << 8i32 |
                             *pc.offset(-2isize).offset(1isize) as
                                 libc::c_int) as uint16_t
                }
                29 => {
                    let fresh235 = pc;
                    pc = pc.offset(1);
                    a = *fresh235 as uint16_t;
                    /* R(a) = R(a)::Syms(b) */
                    pc = pc.offset(2isize);
                    b =
                        ((*pc.offset(-2isize).offset(0isize) as libc::c_int)
                             << 8i32 |
                             *pc.offset(-2isize).offset(1isize) as
                                 libc::c_int) as uint16_t
                }
                30 => {
                    let fresh236 = pc;
                    pc = pc.offset(1);
                    a = *fresh236 as uint16_t;
                    /* R(a+1)::Syms(b) = R(a) */
                    pc = pc.offset(2isize);
                    b =
                        ((*pc.offset(-2isize).offset(0isize) as libc::c_int)
                             << 8i32 |
                             *pc.offset(-2isize).offset(1isize) as
                                 libc::c_int) as uint16_t
                }
                31 => {
                    let fresh237 = pc;
                    pc = pc.offset(1);
                    a = *fresh237 as uint16_t;
                    pc = pc.offset(2isize);
                    b =
                        ((*pc.offset(-2isize).offset(0isize) as libc::c_int)
                             << 8i32 |
                             *pc.offset(-2isize).offset(1isize) as
                                 libc::c_int) as uint16_t;
                    /* R(a) = uvget(b,c) */
                    let fresh238 = pc;
                    pc = pc.offset(1);
                    c = *fresh238
                }
                32 => {
                    let fresh239 = pc;
                    pc = pc.offset(1);
                    a = *fresh239 as uint16_t;
                    pc = pc.offset(2isize);
                    b =
                        ((*pc.offset(-2isize).offset(0isize) as libc::c_int)
                             << 8i32 |
                             *pc.offset(-2isize).offset(1isize) as
                                 libc::c_int) as uint16_t;
                    /* uvset(b,c,R(a)) */
                    let fresh240 = pc;
                    pc = pc.offset(1);
                    c = *fresh240
                }
                33 => {
                    /* pc=a */
                    pc = pc.offset(2isize);
                    a =
                        ((*pc.offset(-2isize).offset(0isize) as libc::c_int)
                             << 8i32 |
                             *pc.offset(-2isize).offset(1isize) as
                                 libc::c_int) as uint16_t
                }
                34 => {
                    let fresh241 = pc;
                    pc = pc.offset(1);
                    a = *fresh241 as uint16_t;
                    /* if R(b) pc=a */
                    pc = pc.offset(2isize);
                    b =
                        ((*pc.offset(-2isize).offset(0isize) as libc::c_int)
                             << 8i32 |
                             *pc.offset(-2isize).offset(1isize) as
                                 libc::c_int) as uint16_t
                }
                35 => {
                    let fresh242 = pc;
                    pc = pc.offset(1);
                    a = *fresh242 as uint16_t;
                    /* if !R(b) pc=a */
                    pc = pc.offset(2isize);
                    b =
                        ((*pc.offset(-2isize).offset(0isize) as libc::c_int)
                             << 8i32 |
                             *pc.offset(-2isize).offset(1isize) as
                                 libc::c_int) as uint16_t
                }
                36 => {
                    let fresh243 = pc;
                    pc = pc.offset(1);
                    a = *fresh243 as uint16_t;
                    /* if R(b)==nil pc=a */
                    pc = pc.offset(2isize);
                    b =
                        ((*pc.offset(-2isize).offset(0isize) as libc::c_int)
                             << 8i32 |
                             *pc.offset(-2isize).offset(1isize) as
                                 libc::c_int) as uint16_t
                }
                37 => {
                    /* rescue_push(a) */
                    pc = pc.offset(2isize);
                    a =
                        ((*pc.offset(-2isize).offset(0isize) as libc::c_int)
                             << 8i32 |
                             *pc.offset(-2isize).offset(1isize) as
                                 libc::c_int) as uint16_t
                }
                38 => {
                    /* R(a) = exc */
                    let fresh244 = pc;
                    pc = pc.offset(1);
                    a = *fresh244 as uint16_t
                }
                39 => {
                    let fresh245 = pc;
                    pc = pc.offset(1);
                    a = *fresh245 as uint16_t;
                    /* R(b) = R(a).isa?(R(b)) */
                    pc = pc.offset(2isize);
                    b =
                        ((*pc.offset(-2isize).offset(0isize) as libc::c_int)
                             << 8i32 |
                             *pc.offset(-2isize).offset(1isize) as
                                 libc::c_int) as uint16_t
                }
                40 => {
                    /* a.times{rescue_pop()} */
                    let fresh246 = pc;
                    pc = pc.offset(1);
                    a = *fresh246 as uint16_t
                }
                41 => {
                    /* raise(R(a)) */
                    let fresh247 = pc;
                    pc = pc.offset(1);
                    a = *fresh247 as uint16_t
                }
                42 => {
                    /* ensure_push(SEQ[a]) */
                    let fresh248 = pc;
                    pc = pc.offset(1);
                    a = *fresh248 as uint16_t
                }
                43 => {
                    /* A.times{ensure_pop().call} */
                    let fresh249 = pc;
                    pc = pc.offset(1);
                    a = *fresh249 as uint16_t
                }
                44 => {
                    let fresh250 = pc;
                    pc = pc.offset(1);
                    a = *fresh250 as uint16_t;
                    /* R(a) = call(R(a),Syms(b),*R(a+1)) */
                    pc = pc.offset(2isize);
                    b =
                        ((*pc.offset(-2isize).offset(0isize) as libc::c_int)
                             << 8i32 |
                             *pc.offset(-2isize).offset(1isize) as
                                 libc::c_int) as uint16_t
                }
                45 => {
                    let fresh251 = pc;
                    pc = pc.offset(1);
                    a = *fresh251 as uint16_t;
                    /* R(a) = call(R(a),Syms(b),*R(a+1),&R(a+2)) */
                    pc = pc.offset(2isize);
                    b =
                        ((*pc.offset(-2isize).offset(0isize) as libc::c_int)
                             << 8i32 |
                             *pc.offset(-2isize).offset(1isize) as
                                 libc::c_int) as uint16_t
                }
                46 => {
                    let fresh252 = pc;
                    pc = pc.offset(1);
                    a = *fresh252 as uint16_t;
                    pc = pc.offset(2isize);
                    b =
                        ((*pc.offset(-2isize).offset(0isize) as libc::c_int)
                             << 8i32 |
                             *pc.offset(-2isize).offset(1isize) as
                                 libc::c_int) as uint16_t;
                    /* R(a) = call(R(a),Syms(b),R(a+1),...,R(a+c)) */
                    let fresh253 = pc;
                    pc = pc.offset(1);
                    c = *fresh253
                }
                47 => {
                    let fresh254 = pc;
                    pc = pc.offset(1);
                    a = *fresh254 as uint16_t;
                    pc = pc.offset(2isize);
                    b =
                        ((*pc.offset(-2isize).offset(0isize) as libc::c_int)
                             << 8i32 |
                             *pc.offset(-2isize).offset(1isize) as
                                 libc::c_int) as uint16_t;
                    /* R(a) = call(R(a),Syms(Bx),R(a+1),...,R(a+c),&R(a+c+1)) */
                    let fresh255 = pc;
                    pc = pc.offset(1);
                    c = *fresh255
                }
                49 => {
                    let fresh256 = pc;
                    pc = pc.offset(1);
                    a = *fresh256 as uint16_t;
                    /* R(a) = super(R(a+1),... ,R(a+b+1)) */
                    pc = pc.offset(2isize);
                    b =
                        ((*pc.offset(-2isize).offset(0isize) as libc::c_int)
                             << 8i32 |
                             *pc.offset(-2isize).offset(1isize) as
                                 libc::c_int) as uint16_t
                }
                50 => {
                    let fresh257 = pc;
                    pc = pc.offset(1);
                    a = *fresh257 as uint16_t;
                    /* R(a) = argument array (16=m5:r1:m5:d1:lv4) */
                    pc = pc.offset(2isize);
                    b =
                        ((*pc.offset(-2isize).offset(0isize) as libc::c_int)
                             << 8i32 |
                             *pc.offset(-2isize).offset(1isize) as
                                 libc::c_int) as uint16_t
                }
                51 => {
                    /* arg setup according to flags (23=m5:o5:r1:m5:k5:d1:b1) */
                    pc = pc.offset(3isize);
                    a =
                        ((*pc.offset(-3isize).offset(0isize) as libc::c_int)
                             << 16i32 |
                             (*pc.offset(-3isize).offset(1isize) as
                                  libc::c_int) << 8i32 |
                             *pc.offset(-3isize).offset(2isize) as
                                 libc::c_int) as uint16_t
                }
                52 => {
                    let fresh258 = pc;
                    pc = pc.offset(1);
                    a = *fresh258 as uint16_t;
                    /* R(a) = kdict.key?(Syms(b))                      # todo */
                    pc = pc.offset(2isize);
                    b =
                        ((*pc.offset(-2isize).offset(0isize) as libc::c_int)
                             << 8i32 |
                             *pc.offset(-2isize).offset(1isize) as
                                 libc::c_int) as uint16_t
                }
                54 => {
                    let fresh259 = pc;
                    pc = pc.offset(1);
                    a = *fresh259 as uint16_t;
                    /* R(a) = kdict[Syms(b)]; kdict.delete(Syms(b))    # todo */
                    pc = pc.offset(2isize);
                    b =
                        ((*pc.offset(-2isize).offset(0isize) as libc::c_int)
                             << 8i32 |
                             *pc.offset(-2isize).offset(1isize) as
                                 libc::c_int) as uint16_t
                }
                55 => {
                    /* return R(a) (normal) */
                    let fresh260 = pc;
                    pc = pc.offset(1);
                    a = *fresh260 as uint16_t
                }
                56 => {
                    /* return R(a) (in-block return) */
                    let fresh261 = pc;
                    pc = pc.offset(1);
                    a = *fresh261 as uint16_t
                }
                57 => {
                    /* break R(a) */
                    let fresh262 = pc;
                    pc = pc.offset(1);
                    a = *fresh262 as uint16_t
                }
                58 => {
                    let fresh263 = pc;
                    pc = pc.offset(1);
                    a = *fresh263 as uint16_t;
                    /* R(a) = block (16=m5:r1:m5:d1:lv4) */
                    pc = pc.offset(2isize);
                    b =
                        ((*pc.offset(-2isize).offset(0isize) as libc::c_int)
                             << 8i32 |
                             *pc.offset(-2isize).offset(1isize) as
                                 libc::c_int) as uint16_t
                }
                59 => {
                    /* R(a) = R(a)+R(a+1) */
                    let fresh264 = pc;
                    pc = pc.offset(1);
                    a = *fresh264 as uint16_t
                }
                60 => {
                    let fresh265 = pc;
                    pc = pc.offset(1);
                    a = *fresh265 as uint16_t;
                    /* R(a) = R(a)+mrb_int(c)  */
                    pc = pc.offset(2isize);
                    b =
                        ((*pc.offset(-2isize).offset(0isize) as libc::c_int)
                             << 8i32 |
                             *pc.offset(-2isize).offset(1isize) as
                                 libc::c_int) as uint16_t
                }
                61 => {
                    /* R(a) = R(a)-R(a+1) */
                    let fresh266 = pc;
                    pc = pc.offset(1);
                    a = *fresh266 as uint16_t
                }
                62 => {
                    let fresh267 = pc;
                    pc = pc.offset(1);
                    a = *fresh267 as uint16_t;
                    /* R(a) = R(a)-C */
                    pc = pc.offset(2isize);
                    b =
                        ((*pc.offset(-2isize).offset(0isize) as libc::c_int)
                             << 8i32 |
                             *pc.offset(-2isize).offset(1isize) as
                                 libc::c_int) as uint16_t
                }
                63 => {
                    /* R(a) = R(a)*R(a+1) */
                    let fresh268 = pc;
                    pc = pc.offset(1);
                    a = *fresh268 as uint16_t
                }
                64 => {
                    /* R(a) = R(a)/R(a+1) */
                    let fresh269 = pc;
                    pc = pc.offset(1);
                    a = *fresh269 as uint16_t
                }
                65 => {
                    /* R(a) = R(a)==R(a+1) */
                    let fresh270 = pc;
                    pc = pc.offset(1);
                    a = *fresh270 as uint16_t
                }
                66 => {
                    /* R(a) = R(a)<R(a+1) */
                    let fresh271 = pc;
                    pc = pc.offset(1);
                    a = *fresh271 as uint16_t
                }
                67 => {
                    /* R(a) = R(a)<=R(a+1) */
                    let fresh272 = pc;
                    pc = pc.offset(1);
                    a = *fresh272 as uint16_t
                }
                68 => {
                    /* R(a) = R(a)>R(a+1) */
                    let fresh273 = pc;
                    pc = pc.offset(1);
                    a = *fresh273 as uint16_t
                }
                69 => {
                    /* R(a) = R(a)>=R(a+1) */
                    let fresh274 = pc;
                    pc = pc.offset(1);
                    a = *fresh274 as uint16_t
                }
                70 => {
                    let fresh275 = pc;
                    pc = pc.offset(1);
                    a = *fresh275 as uint16_t;
                    /* R(a) = ary_new(R(a),R(a+1)..R(a+b)) */
                    pc = pc.offset(2isize);
                    b =
                        ((*pc.offset(-2isize).offset(0isize) as libc::c_int)
                             << 8i32 |
                             *pc.offset(-2isize).offset(1isize) as
                                 libc::c_int) as uint16_t
                }
                71 => {
                    let fresh276 = pc;
                    pc = pc.offset(1);
                    a = *fresh276 as uint16_t;
                    pc = pc.offset(2isize);
                    b =
                        ((*pc.offset(-2isize).offset(0isize) as libc::c_int)
                             << 8i32 |
                             *pc.offset(-2isize).offset(1isize) as
                                 libc::c_int) as uint16_t;
                    /* R(a) = ary_new(R(b),R(b+1)..R(b+c)) */
                    let fresh277 = pc;
                    pc = pc.offset(1);
                    c = *fresh277
                }
                72 => {
                    /* ary_cat(R(a),R(a+1)) */
                    let fresh278 = pc;
                    pc = pc.offset(1);
                    a = *fresh278 as uint16_t
                }
                73 => {
                    /* ary_push(R(a),R(a+1)) */
                    let fresh279 = pc;
                    pc = pc.offset(1);
                    a = *fresh279 as uint16_t
                }
                74 => {
                    /* R(a) = ary_dup(R(a)) */
                    let fresh280 = pc;
                    pc = pc.offset(1);
                    a = *fresh280 as uint16_t
                }
                75 => {
                    let fresh281 = pc;
                    pc = pc.offset(1);
                    a = *fresh281 as uint16_t;
                    pc = pc.offset(2isize);
                    b =
                        ((*pc.offset(-2isize).offset(0isize) as libc::c_int)
                             << 8i32 |
                             *pc.offset(-2isize).offset(1isize) as
                                 libc::c_int) as uint16_t;
                    /* R(a) = R(b)[c] */
                    let fresh282 = pc;
                    pc = pc.offset(1);
                    c = *fresh282
                }
                76 => {
                    let fresh283 = pc;
                    pc = pc.offset(1);
                    a = *fresh283 as uint16_t;
                    pc = pc.offset(2isize);
                    b =
                        ((*pc.offset(-2isize).offset(0isize) as libc::c_int)
                             << 8i32 |
                             *pc.offset(-2isize).offset(1isize) as
                                 libc::c_int) as uint16_t;
                    /* R(a)[c] = R(b) */
                    let fresh284 = pc;
                    pc = pc.offset(1);
                    c = *fresh284
                }
                77 => {
                    let fresh285 = pc;
                    pc = pc.offset(1);
                    a = *fresh285 as uint16_t;
                    pc = pc.offset(2isize);
                    b =
                        ((*pc.offset(-2isize).offset(0isize) as libc::c_int)
                             << 8i32 |
                             *pc.offset(-2isize).offset(1isize) as
                                 libc::c_int) as uint16_t;
                    /* *R(a),R(a+1)..R(a+c) = R(a)[b..] */
                    let fresh286 = pc;
                    pc = pc.offset(1);
                    c = *fresh286
                }
                78 => {
                    /* R(a) = intern(R(a)) */
                    let fresh287 = pc;
                    pc = pc.offset(1);
                    a = *fresh287 as uint16_t
                }
                79 => {
                    let fresh288 = pc;
                    pc = pc.offset(1);
                    a = *fresh288 as uint16_t;
                    /* R(a) = str_dup(Lit(b)) */
                    pc = pc.offset(2isize);
                    b =
                        ((*pc.offset(-2isize).offset(0isize) as libc::c_int)
                             << 8i32 |
                             *pc.offset(-2isize).offset(1isize) as
                                 libc::c_int) as uint16_t
                }
                80 => {
                    /* str_cat(R(a),R(a+1)) */
                    let fresh289 = pc;
                    pc = pc.offset(1);
                    a = *fresh289 as uint16_t
                }
                81 => {
                    let fresh290 = pc;
                    pc = pc.offset(1);
                    a = *fresh290 as uint16_t;
                    /* R(a) = hash_new(R(a),R(a+1)..R(a+b)) */
                    pc = pc.offset(2isize);
                    b =
                        ((*pc.offset(-2isize).offset(0isize) as libc::c_int)
                             << 8i32 |
                             *pc.offset(-2isize).offset(1isize) as
                                 libc::c_int) as uint16_t
                }
                82 => {
                    let fresh291 = pc;
                    pc = pc.offset(1);
                    a = *fresh291 as uint16_t;
                    /* R(a) = hash_push(R(a),R(a+1)..R(a+b)) */
                    pc = pc.offset(2isize);
                    b =
                        ((*pc.offset(-2isize).offset(0isize) as libc::c_int)
                             << 8i32 |
                             *pc.offset(-2isize).offset(1isize) as
                                 libc::c_int) as uint16_t
                }
                83 => {
                    /* R(a) = hash_cat(R(a),R(a+1)) */
                    let fresh292 = pc;
                    pc = pc.offset(1);
                    a = *fresh292 as uint16_t
                }
                84 => {
                    let fresh293 = pc;
                    pc = pc.offset(1);
                    a = *fresh293 as uint16_t;
                    /* R(a) = lambda(SEQ[b],L_LAMBDA) */
                    pc = pc.offset(2isize);
                    b =
                        ((*pc.offset(-2isize).offset(0isize) as libc::c_int)
                             << 8i32 |
                             *pc.offset(-2isize).offset(1isize) as
                                 libc::c_int) as uint16_t
                }
                85 => {
                    let fresh294 = pc;
                    pc = pc.offset(1);
                    a = *fresh294 as uint16_t;
                    /* R(a) = lambda(SEQ[b],L_BLOCK) */
                    pc = pc.offset(2isize);
                    b =
                        ((*pc.offset(-2isize).offset(0isize) as libc::c_int)
                             << 8i32 |
                             *pc.offset(-2isize).offset(1isize) as
                                 libc::c_int) as uint16_t
                }
                86 => {
                    let fresh295 = pc;
                    pc = pc.offset(1);
                    a = *fresh295 as uint16_t;
                    /* R(a) = lambda(SEQ[b],L_METHOD) */
                    pc = pc.offset(2isize);
                    b =
                        ((*pc.offset(-2isize).offset(0isize) as libc::c_int)
                             << 8i32 |
                             *pc.offset(-2isize).offset(1isize) as
                                 libc::c_int) as uint16_t
                }
                87 => {
                    /* R(a) = range_new(R(a),R(a+1),FALSE) */
                    let fresh296 = pc;
                    pc = pc.offset(1);
                    a = *fresh296 as uint16_t
                }
                88 => {
                    /* R(a) = range_new(R(a),R(a+1),TRUE) */
                    let fresh297 = pc;
                    pc = pc.offset(1);
                    a = *fresh297 as uint16_t
                }
                89 => {
                    /* R(a) = ::Object */
                    let fresh298 = pc;
                    pc = pc.offset(1);
                    a = *fresh298 as uint16_t
                }
                90 => {
                    let fresh299 = pc;
                    pc = pc.offset(1);
                    a = *fresh299 as uint16_t;
                    /* R(a) = newclass(R(a),Syms(b),R(a+1)) */
                    pc = pc.offset(2isize);
                    b =
                        ((*pc.offset(-2isize).offset(0isize) as libc::c_int)
                             << 8i32 |
                             *pc.offset(-2isize).offset(1isize) as
                                 libc::c_int) as uint16_t
                }
                91 => {
                    let fresh300 = pc;
                    pc = pc.offset(1);
                    a = *fresh300 as uint16_t;
                    /* R(a) = newmodule(R(a),Syms(b)) */
                    pc = pc.offset(2isize);
                    b =
                        ((*pc.offset(-2isize).offset(0isize) as libc::c_int)
                             << 8i32 |
                             *pc.offset(-2isize).offset(1isize) as
                                 libc::c_int) as uint16_t
                }
                92 => {
                    let fresh301 = pc;
                    pc = pc.offset(1);
                    a = *fresh301 as uint16_t;
                    /* R(a) = blockexec(R(a),SEQ[b]) */
                    pc = pc.offset(2isize);
                    b =
                        ((*pc.offset(-2isize).offset(0isize) as libc::c_int)
                             << 8i32 |
                             *pc.offset(-2isize).offset(1isize) as
                                 libc::c_int) as uint16_t
                }
                93 => {
                    let fresh302 = pc;
                    pc = pc.offset(1);
                    a = *fresh302 as uint16_t;
                    /* R(a).newmethod(Syms(b),R(a+1)) */
                    pc = pc.offset(2isize);
                    b =
                        ((*pc.offset(-2isize).offset(0isize) as libc::c_int)
                             << 8i32 |
                             *pc.offset(-2isize).offset(1isize) as
                                 libc::c_int) as uint16_t
                }
                94 => {
                    let fresh303 = pc;
                    pc = pc.offset(1);
                    a = *fresh303 as uint16_t;
                    /* alias_method(target_class,Syms(a),Syms(b)) */
                    pc = pc.offset(2isize);
                    b =
                        ((*pc.offset(-2isize).offset(0isize) as libc::c_int)
                             << 8i32 |
                             *pc.offset(-2isize).offset(1isize) as
                                 libc::c_int) as uint16_t
                }
                95 => {
                    /* undef_method(target_class,Syms(a)) */
                    let fresh304 = pc;
                    pc = pc.offset(1);
                    a = *fresh304 as uint16_t
                }
                96 => {
                    /* R(a) = R(a).singleton_class */
                    let fresh305 = pc;
                    pc = pc.offset(1);
                    a = *fresh305 as uint16_t
                }
                97 => {
                    /* R(a) = target_class */
                    let fresh306 = pc;
                    pc = pc.offset(1);
                    a = *fresh306 as uint16_t
                }
                98 => {
                    let fresh307 = pc;
                    pc = pc.offset(1);
                    a = *fresh307 as uint16_t;
                    pc = pc.offset(2isize);
                    b =
                        ((*pc.offset(-2isize).offset(0isize) as libc::c_int)
                             << 8i32 |
                             *pc.offset(-2isize).offset(1isize) as
                                 libc::c_int) as uint16_t;
                    /* print a,b,c */
                    let fresh308 = pc;
                    pc = pc.offset(1);
                    c = *fresh308
                }
                99 => {
                    /* raise(LocalJumpError, Lit(a)) */
                    let fresh309 = pc;
                    pc = pc.offset(1);
                    a = *fresh309 as uint16_t
                }
                0 | 48 | 53 | 100 | 101 | 102 | 103 | _ => { }
            }
        }
        102 => {
            let fresh310 = pc;
            pc = pc.offset(1);
            insn = *fresh310;
            match insn as libc::c_int {
                1 => {
                    pc = pc.offset(2isize);
                    a =
                        ((*pc.offset(-2isize).offset(0isize) as libc::c_int)
                             << 8i32 |
                             *pc.offset(-2isize).offset(1isize) as
                                 libc::c_int) as uint16_t;
                    /* R(a) = R(b) */
                    pc = pc.offset(2isize);
                    b =
                        ((*pc.offset(-2isize).offset(0isize) as libc::c_int)
                             << 8i32 |
                             *pc.offset(-2isize).offset(1isize) as
                                 libc::c_int) as uint16_t
                }
                2 => {
                    pc = pc.offset(2isize);
                    a =
                        ((*pc.offset(-2isize).offset(0isize) as libc::c_int)
                             << 8i32 |
                             *pc.offset(-2isize).offset(1isize) as
                                 libc::c_int) as uint16_t;
                    /* R(a) = Pool(b) */
                    pc = pc.offset(2isize);
                    b =
                        ((*pc.offset(-2isize).offset(0isize) as libc::c_int)
                             << 8i32 |
                             *pc.offset(-2isize).offset(1isize) as
                                 libc::c_int) as uint16_t
                }
                3 => {
                    pc = pc.offset(2isize);
                    a =
                        ((*pc.offset(-2isize).offset(0isize) as libc::c_int)
                             << 8i32 |
                             *pc.offset(-2isize).offset(1isize) as
                                 libc::c_int) as uint16_t;
                    /* R(a) = mrb_int(b) */
                    pc = pc.offset(2isize);
                    b =
                        ((*pc.offset(-2isize).offset(0isize) as libc::c_int)
                             << 8i32 |
                             *pc.offset(-2isize).offset(1isize) as
                                 libc::c_int) as uint16_t
                }
                4 => {
                    pc = pc.offset(2isize);
                    a =
                        ((*pc.offset(-2isize).offset(0isize) as libc::c_int)
                             << 8i32 |
                             *pc.offset(-2isize).offset(1isize) as
                                 libc::c_int) as uint16_t;
                    /* R(a) = mrb_int(-b) */
                    pc = pc.offset(2isize);
                    b =
                        ((*pc.offset(-2isize).offset(0isize) as libc::c_int)
                             << 8i32 |
                             *pc.offset(-2isize).offset(1isize) as
                                 libc::c_int) as uint16_t
                }
                5 => {
                    /* R(a) = mrb_int(-1) */
                    let fresh311 = pc;
                    pc = pc.offset(1);
                    a = *fresh311 as uint16_t
                }
                6 => {
                    /* R(a) = mrb_int(0) */
                    let fresh312 = pc;
                    pc = pc.offset(1);
                    a = *fresh312 as uint16_t
                }
                7 => {
                    /* R(a) = mrb_int(1) */
                    let fresh313 = pc;
                    pc = pc.offset(1);
                    a = *fresh313 as uint16_t
                }
                8 => {
                    /* R(a) = mrb_int(2) */
                    let fresh314 = pc;
                    pc = pc.offset(1);
                    a = *fresh314 as uint16_t
                }
                9 => {
                    /* R(a) = mrb_int(3) */
                    let fresh315 = pc;
                    pc = pc.offset(1);
                    a = *fresh315 as uint16_t
                }
                10 => {
                    /* R(a) = mrb_int(4) */
                    let fresh316 = pc;
                    pc = pc.offset(1);
                    a = *fresh316 as uint16_t
                }
                11 => {
                    /* R(a) = mrb_int(5) */
                    let fresh317 = pc;
                    pc = pc.offset(1);
                    a = *fresh317 as uint16_t
                }
                12 => {
                    /* R(a) = mrb_int(6) */
                    let fresh318 = pc;
                    pc = pc.offset(1);
                    a = *fresh318 as uint16_t
                }
                13 => {
                    /* R(a) = mrb_int(7) */
                    let fresh319 = pc;
                    pc = pc.offset(1);
                    a = *fresh319 as uint16_t
                }
                14 => {
                    pc = pc.offset(2isize);
                    a =
                        ((*pc.offset(-2isize).offset(0isize) as libc::c_int)
                             << 8i32 |
                             *pc.offset(-2isize).offset(1isize) as
                                 libc::c_int) as uint16_t;
                    /* R(a) = Syms(b) */
                    pc = pc.offset(2isize);
                    b =
                        ((*pc.offset(-2isize).offset(0isize) as libc::c_int)
                             << 8i32 |
                             *pc.offset(-2isize).offset(1isize) as
                                 libc::c_int) as uint16_t
                }
                15 => {
                    /* R(a) = nil */
                    let fresh320 = pc;
                    pc = pc.offset(1);
                    a = *fresh320 as uint16_t
                }
                16 => {
                    /* R(a) = self */
                    let fresh321 = pc;
                    pc = pc.offset(1);
                    a = *fresh321 as uint16_t
                }
                17 => {
                    /* R(a) = true */
                    let fresh322 = pc;
                    pc = pc.offset(1);
                    a = *fresh322 as uint16_t
                }
                18 => {
                    /* R(a) = false */
                    let fresh323 = pc;
                    pc = pc.offset(1);
                    a = *fresh323 as uint16_t
                }
                19 => {
                    pc = pc.offset(2isize);
                    a =
                        ((*pc.offset(-2isize).offset(0isize) as libc::c_int)
                             << 8i32 |
                             *pc.offset(-2isize).offset(1isize) as
                                 libc::c_int) as uint16_t;
                    /* R(a) = getglobal(Syms(b)) */
                    pc = pc.offset(2isize);
                    b =
                        ((*pc.offset(-2isize).offset(0isize) as libc::c_int)
                             << 8i32 |
                             *pc.offset(-2isize).offset(1isize) as
                                 libc::c_int) as uint16_t
                }
                20 => {
                    pc = pc.offset(2isize);
                    a =
                        ((*pc.offset(-2isize).offset(0isize) as libc::c_int)
                             << 8i32 |
                             *pc.offset(-2isize).offset(1isize) as
                                 libc::c_int) as uint16_t;
                    /* setglobal(Syms(b), R(a)) */
                    pc = pc.offset(2isize);
                    b =
                        ((*pc.offset(-2isize).offset(0isize) as libc::c_int)
                             << 8i32 |
                             *pc.offset(-2isize).offset(1isize) as
                                 libc::c_int) as uint16_t
                }
                21 => {
                    pc = pc.offset(2isize);
                    a =
                        ((*pc.offset(-2isize).offset(0isize) as libc::c_int)
                             << 8i32 |
                             *pc.offset(-2isize).offset(1isize) as
                                 libc::c_int) as uint16_t;
                    /* R(a) = Special[Syms(b)] */
                    pc = pc.offset(2isize);
                    b =
                        ((*pc.offset(-2isize).offset(0isize) as libc::c_int)
                             << 8i32 |
                             *pc.offset(-2isize).offset(1isize) as
                                 libc::c_int) as uint16_t
                }
                22 => {
                    pc = pc.offset(2isize);
                    a =
                        ((*pc.offset(-2isize).offset(0isize) as libc::c_int)
                             << 8i32 |
                             *pc.offset(-2isize).offset(1isize) as
                                 libc::c_int) as uint16_t;
                    /* Special[Syms(b)] = R(a) */
                    pc = pc.offset(2isize);
                    b =
                        ((*pc.offset(-2isize).offset(0isize) as libc::c_int)
                             << 8i32 |
                             *pc.offset(-2isize).offset(1isize) as
                                 libc::c_int) as uint16_t
                }
                23 => {
                    pc = pc.offset(2isize);
                    a =
                        ((*pc.offset(-2isize).offset(0isize) as libc::c_int)
                             << 8i32 |
                             *pc.offset(-2isize).offset(1isize) as
                                 libc::c_int) as uint16_t;
                    /* R(a) = ivget(Syms(b)) */
                    pc = pc.offset(2isize);
                    b =
                        ((*pc.offset(-2isize).offset(0isize) as libc::c_int)
                             << 8i32 |
                             *pc.offset(-2isize).offset(1isize) as
                                 libc::c_int) as uint16_t
                }
                24 => {
                    pc = pc.offset(2isize);
                    a =
                        ((*pc.offset(-2isize).offset(0isize) as libc::c_int)
                             << 8i32 |
                             *pc.offset(-2isize).offset(1isize) as
                                 libc::c_int) as uint16_t;
                    /* ivset(Syms(b),R(a)) */
                    pc = pc.offset(2isize);
                    b =
                        ((*pc.offset(-2isize).offset(0isize) as libc::c_int)
                             << 8i32 |
                             *pc.offset(-2isize).offset(1isize) as
                                 libc::c_int) as uint16_t
                }
                25 => {
                    pc = pc.offset(2isize);
                    a =
                        ((*pc.offset(-2isize).offset(0isize) as libc::c_int)
                             << 8i32 |
                             *pc.offset(-2isize).offset(1isize) as
                                 libc::c_int) as uint16_t;
                    /* R(a) = cvget(Syms(b)) */
                    pc = pc.offset(2isize);
                    b =
                        ((*pc.offset(-2isize).offset(0isize) as libc::c_int)
                             << 8i32 |
                             *pc.offset(-2isize).offset(1isize) as
                                 libc::c_int) as uint16_t
                }
                26 => {
                    pc = pc.offset(2isize);
                    a =
                        ((*pc.offset(-2isize).offset(0isize) as libc::c_int)
                             << 8i32 |
                             *pc.offset(-2isize).offset(1isize) as
                                 libc::c_int) as uint16_t;
                    /* cvset(Syms(b),R(a)) */
                    pc = pc.offset(2isize);
                    b =
                        ((*pc.offset(-2isize).offset(0isize) as libc::c_int)
                             << 8i32 |
                             *pc.offset(-2isize).offset(1isize) as
                                 libc::c_int) as uint16_t
                }
                27 => {
                    pc = pc.offset(2isize);
                    a =
                        ((*pc.offset(-2isize).offset(0isize) as libc::c_int)
                             << 8i32 |
                             *pc.offset(-2isize).offset(1isize) as
                                 libc::c_int) as uint16_t;
                    /* R(a) = constget(Syms(b)) */
                    pc = pc.offset(2isize);
                    b =
                        ((*pc.offset(-2isize).offset(0isize) as libc::c_int)
                             << 8i32 |
                             *pc.offset(-2isize).offset(1isize) as
                                 libc::c_int) as uint16_t
                }
                28 => {
                    pc = pc.offset(2isize);
                    a =
                        ((*pc.offset(-2isize).offset(0isize) as libc::c_int)
                             << 8i32 |
                             *pc.offset(-2isize).offset(1isize) as
                                 libc::c_int) as uint16_t;
                    /* constset(Syms(b),R(a)) */
                    pc = pc.offset(2isize);
                    b =
                        ((*pc.offset(-2isize).offset(0isize) as libc::c_int)
                             << 8i32 |
                             *pc.offset(-2isize).offset(1isize) as
                                 libc::c_int) as uint16_t
                }
                29 => {
                    pc = pc.offset(2isize);
                    a =
                        ((*pc.offset(-2isize).offset(0isize) as libc::c_int)
                             << 8i32 |
                             *pc.offset(-2isize).offset(1isize) as
                                 libc::c_int) as uint16_t;
                    /* R(a) = R(a)::Syms(b) */
                    pc = pc.offset(2isize);
                    b =
                        ((*pc.offset(-2isize).offset(0isize) as libc::c_int)
                             << 8i32 |
                             *pc.offset(-2isize).offset(1isize) as
                                 libc::c_int) as uint16_t
                }
                30 => {
                    pc = pc.offset(2isize);
                    a =
                        ((*pc.offset(-2isize).offset(0isize) as libc::c_int)
                             << 8i32 |
                             *pc.offset(-2isize).offset(1isize) as
                                 libc::c_int) as uint16_t;
                    /* R(a+1)::Syms(b) = R(a) */
                    pc = pc.offset(2isize);
                    b =
                        ((*pc.offset(-2isize).offset(0isize) as libc::c_int)
                             << 8i32 |
                             *pc.offset(-2isize).offset(1isize) as
                                 libc::c_int) as uint16_t
                }
                31 => {
                    pc = pc.offset(2isize);
                    a =
                        ((*pc.offset(-2isize).offset(0isize) as libc::c_int)
                             << 8i32 |
                             *pc.offset(-2isize).offset(1isize) as
                                 libc::c_int) as uint16_t;
                    pc = pc.offset(2isize);
                    b =
                        ((*pc.offset(-2isize).offset(0isize) as libc::c_int)
                             << 8i32 |
                             *pc.offset(-2isize).offset(1isize) as
                                 libc::c_int) as uint16_t;
                    /* R(a) = uvget(b,c) */
                    let fresh324 = pc;
                    pc = pc.offset(1);
                    c = *fresh324
                }
                32 => {
                    pc = pc.offset(2isize);
                    a =
                        ((*pc.offset(-2isize).offset(0isize) as libc::c_int)
                             << 8i32 |
                             *pc.offset(-2isize).offset(1isize) as
                                 libc::c_int) as uint16_t;
                    pc = pc.offset(2isize);
                    b =
                        ((*pc.offset(-2isize).offset(0isize) as libc::c_int)
                             << 8i32 |
                             *pc.offset(-2isize).offset(1isize) as
                                 libc::c_int) as uint16_t;
                    /* uvset(b,c,R(a)) */
                    let fresh325 = pc;
                    pc = pc.offset(1);
                    c = *fresh325
                }
                33 => {
                    /* pc=a */
                    pc = pc.offset(2isize);
                    a =
                        ((*pc.offset(-2isize).offset(0isize) as libc::c_int)
                             << 8i32 |
                             *pc.offset(-2isize).offset(1isize) as
                                 libc::c_int) as uint16_t
                }
                34 => {
                    pc = pc.offset(2isize);
                    a =
                        ((*pc.offset(-2isize).offset(0isize) as libc::c_int)
                             << 8i32 |
                             *pc.offset(-2isize).offset(1isize) as
                                 libc::c_int) as uint16_t;
                    /* if R(b) pc=a */
                    pc = pc.offset(2isize);
                    b =
                        ((*pc.offset(-2isize).offset(0isize) as libc::c_int)
                             << 8i32 |
                             *pc.offset(-2isize).offset(1isize) as
                                 libc::c_int) as uint16_t
                }
                35 => {
                    pc = pc.offset(2isize);
                    a =
                        ((*pc.offset(-2isize).offset(0isize) as libc::c_int)
                             << 8i32 |
                             *pc.offset(-2isize).offset(1isize) as
                                 libc::c_int) as uint16_t;
                    /* if !R(b) pc=a */
                    pc = pc.offset(2isize);
                    b =
                        ((*pc.offset(-2isize).offset(0isize) as libc::c_int)
                             << 8i32 |
                             *pc.offset(-2isize).offset(1isize) as
                                 libc::c_int) as uint16_t
                }
                36 => {
                    pc = pc.offset(2isize);
                    a =
                        ((*pc.offset(-2isize).offset(0isize) as libc::c_int)
                             << 8i32 |
                             *pc.offset(-2isize).offset(1isize) as
                                 libc::c_int) as uint16_t;
                    /* if R(b)==nil pc=a */
                    pc = pc.offset(2isize);
                    b =
                        ((*pc.offset(-2isize).offset(0isize) as libc::c_int)
                             << 8i32 |
                             *pc.offset(-2isize).offset(1isize) as
                                 libc::c_int) as uint16_t
                }
                37 => {
                    /* rescue_push(a) */
                    pc = pc.offset(2isize);
                    a =
                        ((*pc.offset(-2isize).offset(0isize) as libc::c_int)
                             << 8i32 |
                             *pc.offset(-2isize).offset(1isize) as
                                 libc::c_int) as uint16_t
                }
                38 => {
                    /* R(a) = exc */
                    let fresh326 = pc;
                    pc = pc.offset(1);
                    a = *fresh326 as uint16_t
                }
                39 => {
                    pc = pc.offset(2isize);
                    a =
                        ((*pc.offset(-2isize).offset(0isize) as libc::c_int)
                             << 8i32 |
                             *pc.offset(-2isize).offset(1isize) as
                                 libc::c_int) as uint16_t;
                    /* R(b) = R(a).isa?(R(b)) */
                    pc = pc.offset(2isize);
                    b =
                        ((*pc.offset(-2isize).offset(0isize) as libc::c_int)
                             << 8i32 |
                             *pc.offset(-2isize).offset(1isize) as
                                 libc::c_int) as uint16_t
                }
                40 => {
                    /* a.times{rescue_pop()} */
                    let fresh327 = pc;
                    pc = pc.offset(1);
                    a = *fresh327 as uint16_t
                }
                41 => {
                    /* raise(R(a)) */
                    let fresh328 = pc;
                    pc = pc.offset(1);
                    a = *fresh328 as uint16_t
                }
                42 => {
                    /* ensure_push(SEQ[a]) */
                    let fresh329 = pc;
                    pc = pc.offset(1);
                    a = *fresh329 as uint16_t
                }
                43 => {
                    /* A.times{ensure_pop().call} */
                    let fresh330 = pc;
                    pc = pc.offset(1);
                    a = *fresh330 as uint16_t
                }
                44 => {
                    pc = pc.offset(2isize);
                    a =
                        ((*pc.offset(-2isize).offset(0isize) as libc::c_int)
                             << 8i32 |
                             *pc.offset(-2isize).offset(1isize) as
                                 libc::c_int) as uint16_t;
                    /* R(a) = call(R(a),Syms(b),*R(a+1)) */
                    pc = pc.offset(2isize);
                    b =
                        ((*pc.offset(-2isize).offset(0isize) as libc::c_int)
                             << 8i32 |
                             *pc.offset(-2isize).offset(1isize) as
                                 libc::c_int) as uint16_t
                }
                45 => {
                    pc = pc.offset(2isize);
                    a =
                        ((*pc.offset(-2isize).offset(0isize) as libc::c_int)
                             << 8i32 |
                             *pc.offset(-2isize).offset(1isize) as
                                 libc::c_int) as uint16_t;
                    /* R(a) = call(R(a),Syms(b),*R(a+1),&R(a+2)) */
                    pc = pc.offset(2isize);
                    b =
                        ((*pc.offset(-2isize).offset(0isize) as libc::c_int)
                             << 8i32 |
                             *pc.offset(-2isize).offset(1isize) as
                                 libc::c_int) as uint16_t
                }
                46 => {
                    pc = pc.offset(2isize);
                    a =
                        ((*pc.offset(-2isize).offset(0isize) as libc::c_int)
                             << 8i32 |
                             *pc.offset(-2isize).offset(1isize) as
                                 libc::c_int) as uint16_t;
                    pc = pc.offset(2isize);
                    b =
                        ((*pc.offset(-2isize).offset(0isize) as libc::c_int)
                             << 8i32 |
                             *pc.offset(-2isize).offset(1isize) as
                                 libc::c_int) as uint16_t;
                    /* R(a) = call(R(a),Syms(b),R(a+1),...,R(a+c)) */
                    let fresh331 = pc;
                    pc = pc.offset(1);
                    c = *fresh331
                }
                47 => {
                    pc = pc.offset(2isize);
                    a =
                        ((*pc.offset(-2isize).offset(0isize) as libc::c_int)
                             << 8i32 |
                             *pc.offset(-2isize).offset(1isize) as
                                 libc::c_int) as uint16_t;
                    pc = pc.offset(2isize);
                    b =
                        ((*pc.offset(-2isize).offset(0isize) as libc::c_int)
                             << 8i32 |
                             *pc.offset(-2isize).offset(1isize) as
                                 libc::c_int) as uint16_t;
                    /* R(a) = call(R(a),Syms(Bx),R(a+1),...,R(a+c),&R(a+c+1)) */
                    let fresh332 = pc;
                    pc = pc.offset(1);
                    c = *fresh332
                }
                49 => {
                    pc = pc.offset(2isize);
                    a =
                        ((*pc.offset(-2isize).offset(0isize) as libc::c_int)
                             << 8i32 |
                             *pc.offset(-2isize).offset(1isize) as
                                 libc::c_int) as uint16_t;
                    /* R(a) = super(R(a+1),... ,R(a+b+1)) */
                    pc = pc.offset(2isize);
                    b =
                        ((*pc.offset(-2isize).offset(0isize) as libc::c_int)
                             << 8i32 |
                             *pc.offset(-2isize).offset(1isize) as
                                 libc::c_int) as uint16_t
                }
                50 => {
                    pc = pc.offset(2isize);
                    a =
                        ((*pc.offset(-2isize).offset(0isize) as libc::c_int)
                             << 8i32 |
                             *pc.offset(-2isize).offset(1isize) as
                                 libc::c_int) as uint16_t;
                    /* R(a) = argument array (16=m5:r1:m5:d1:lv4) */
                    pc = pc.offset(2isize);
                    b =
                        ((*pc.offset(-2isize).offset(0isize) as libc::c_int)
                             << 8i32 |
                             *pc.offset(-2isize).offset(1isize) as
                                 libc::c_int) as uint16_t
                }
                51 => {
                    /* arg setup according to flags (23=m5:o5:r1:m5:k5:d1:b1) */
                    pc = pc.offset(3isize);
                    a =
                        ((*pc.offset(-3isize).offset(0isize) as libc::c_int)
                             << 16i32 |
                             (*pc.offset(-3isize).offset(1isize) as
                                  libc::c_int) << 8i32 |
                             *pc.offset(-3isize).offset(2isize) as
                                 libc::c_int) as uint16_t
                }
                52 => {
                    pc = pc.offset(2isize);
                    a =
                        ((*pc.offset(-2isize).offset(0isize) as libc::c_int)
                             << 8i32 |
                             *pc.offset(-2isize).offset(1isize) as
                                 libc::c_int) as uint16_t;
                    /* R(a) = kdict.key?(Syms(b))                      # todo */
                    pc = pc.offset(2isize);
                    b =
                        ((*pc.offset(-2isize).offset(0isize) as libc::c_int)
                             << 8i32 |
                             *pc.offset(-2isize).offset(1isize) as
                                 libc::c_int) as uint16_t
                }
                54 => {
                    pc = pc.offset(2isize);
                    a =
                        ((*pc.offset(-2isize).offset(0isize) as libc::c_int)
                             << 8i32 |
                             *pc.offset(-2isize).offset(1isize) as
                                 libc::c_int) as uint16_t;
                    /* R(a) = kdict[Syms(b)]; kdict.delete(Syms(b))    # todo */
                    pc = pc.offset(2isize);
                    b =
                        ((*pc.offset(-2isize).offset(0isize) as libc::c_int)
                             << 8i32 |
                             *pc.offset(-2isize).offset(1isize) as
                                 libc::c_int) as uint16_t
                }
                55 => {
                    /* return R(a) (normal) */
                    let fresh333 = pc;
                    pc = pc.offset(1);
                    a = *fresh333 as uint16_t
                }
                56 => {
                    /* return R(a) (in-block return) */
                    let fresh334 = pc;
                    pc = pc.offset(1);
                    a = *fresh334 as uint16_t
                }
                57 => {
                    /* break R(a) */
                    let fresh335 = pc;
                    pc = pc.offset(1);
                    a = *fresh335 as uint16_t
                }
                58 => {
                    pc = pc.offset(2isize);
                    a =
                        ((*pc.offset(-2isize).offset(0isize) as libc::c_int)
                             << 8i32 |
                             *pc.offset(-2isize).offset(1isize) as
                                 libc::c_int) as uint16_t;
                    /* R(a) = block (16=m5:r1:m5:d1:lv4) */
                    pc = pc.offset(2isize);
                    b =
                        ((*pc.offset(-2isize).offset(0isize) as libc::c_int)
                             << 8i32 |
                             *pc.offset(-2isize).offset(1isize) as
                                 libc::c_int) as uint16_t
                }
                59 => {
                    /* R(a) = R(a)+R(a+1) */
                    let fresh336 = pc;
                    pc = pc.offset(1);
                    a = *fresh336 as uint16_t
                }
                60 => {
                    pc = pc.offset(2isize);
                    a =
                        ((*pc.offset(-2isize).offset(0isize) as libc::c_int)
                             << 8i32 |
                             *pc.offset(-2isize).offset(1isize) as
                                 libc::c_int) as uint16_t;
                    /* R(a) = R(a)+mrb_int(c)  */
                    pc = pc.offset(2isize);
                    b =
                        ((*pc.offset(-2isize).offset(0isize) as libc::c_int)
                             << 8i32 |
                             *pc.offset(-2isize).offset(1isize) as
                                 libc::c_int) as uint16_t
                }
                61 => {
                    /* R(a) = R(a)-R(a+1) */
                    let fresh337 = pc;
                    pc = pc.offset(1);
                    a = *fresh337 as uint16_t
                }
                62 => {
                    pc = pc.offset(2isize);
                    a =
                        ((*pc.offset(-2isize).offset(0isize) as libc::c_int)
                             << 8i32 |
                             *pc.offset(-2isize).offset(1isize) as
                                 libc::c_int) as uint16_t;
                    /* R(a) = R(a)-C */
                    pc = pc.offset(2isize);
                    b =
                        ((*pc.offset(-2isize).offset(0isize) as libc::c_int)
                             << 8i32 |
                             *pc.offset(-2isize).offset(1isize) as
                                 libc::c_int) as uint16_t
                }
                63 => {
                    /* R(a) = R(a)*R(a+1) */
                    let fresh338 = pc;
                    pc = pc.offset(1);
                    a = *fresh338 as uint16_t
                }
                64 => {
                    /* R(a) = R(a)/R(a+1) */
                    let fresh339 = pc;
                    pc = pc.offset(1);
                    a = *fresh339 as uint16_t
                }
                65 => {
                    /* R(a) = R(a)==R(a+1) */
                    let fresh340 = pc;
                    pc = pc.offset(1);
                    a = *fresh340 as uint16_t
                }
                66 => {
                    /* R(a) = R(a)<R(a+1) */
                    let fresh341 = pc;
                    pc = pc.offset(1);
                    a = *fresh341 as uint16_t
                }
                67 => {
                    /* R(a) = R(a)<=R(a+1) */
                    let fresh342 = pc;
                    pc = pc.offset(1);
                    a = *fresh342 as uint16_t
                }
                68 => {
                    /* R(a) = R(a)>R(a+1) */
                    let fresh343 = pc;
                    pc = pc.offset(1);
                    a = *fresh343 as uint16_t
                }
                69 => {
                    /* R(a) = R(a)>=R(a+1) */
                    let fresh344 = pc;
                    pc = pc.offset(1);
                    a = *fresh344 as uint16_t
                }
                70 => {
                    pc = pc.offset(2isize);
                    a =
                        ((*pc.offset(-2isize).offset(0isize) as libc::c_int)
                             << 8i32 |
                             *pc.offset(-2isize).offset(1isize) as
                                 libc::c_int) as uint16_t;
                    /* R(a) = ary_new(R(a),R(a+1)..R(a+b)) */
                    pc = pc.offset(2isize);
                    b =
                        ((*pc.offset(-2isize).offset(0isize) as libc::c_int)
                             << 8i32 |
                             *pc.offset(-2isize).offset(1isize) as
                                 libc::c_int) as uint16_t
                }
                71 => {
                    pc = pc.offset(2isize);
                    a =
                        ((*pc.offset(-2isize).offset(0isize) as libc::c_int)
                             << 8i32 |
                             *pc.offset(-2isize).offset(1isize) as
                                 libc::c_int) as uint16_t;
                    pc = pc.offset(2isize);
                    b =
                        ((*pc.offset(-2isize).offset(0isize) as libc::c_int)
                             << 8i32 |
                             *pc.offset(-2isize).offset(1isize) as
                                 libc::c_int) as uint16_t;
                    /* R(a) = ary_new(R(b),R(b+1)..R(b+c)) */
                    let fresh345 = pc;
                    pc = pc.offset(1);
                    c = *fresh345
                }
                72 => {
                    /* ary_cat(R(a),R(a+1)) */
                    let fresh346 = pc;
                    pc = pc.offset(1);
                    a = *fresh346 as uint16_t
                }
                73 => {
                    /* ary_push(R(a),R(a+1)) */
                    let fresh347 = pc;
                    pc = pc.offset(1);
                    a = *fresh347 as uint16_t
                }
                74 => {
                    /* R(a) = ary_dup(R(a)) */
                    let fresh348 = pc;
                    pc = pc.offset(1);
                    a = *fresh348 as uint16_t
                }
                75 => {
                    pc = pc.offset(2isize);
                    a =
                        ((*pc.offset(-2isize).offset(0isize) as libc::c_int)
                             << 8i32 |
                             *pc.offset(-2isize).offset(1isize) as
                                 libc::c_int) as uint16_t;
                    pc = pc.offset(2isize);
                    b =
                        ((*pc.offset(-2isize).offset(0isize) as libc::c_int)
                             << 8i32 |
                             *pc.offset(-2isize).offset(1isize) as
                                 libc::c_int) as uint16_t;
                    /* R(a) = R(b)[c] */
                    let fresh349 = pc;
                    pc = pc.offset(1);
                    c = *fresh349
                }
                76 => {
                    pc = pc.offset(2isize);
                    a =
                        ((*pc.offset(-2isize).offset(0isize) as libc::c_int)
                             << 8i32 |
                             *pc.offset(-2isize).offset(1isize) as
                                 libc::c_int) as uint16_t;
                    pc = pc.offset(2isize);
                    b =
                        ((*pc.offset(-2isize).offset(0isize) as libc::c_int)
                             << 8i32 |
                             *pc.offset(-2isize).offset(1isize) as
                                 libc::c_int) as uint16_t;
                    /* R(a)[c] = R(b) */
                    let fresh350 = pc;
                    pc = pc.offset(1);
                    c = *fresh350
                }
                77 => {
                    pc = pc.offset(2isize);
                    a =
                        ((*pc.offset(-2isize).offset(0isize) as libc::c_int)
                             << 8i32 |
                             *pc.offset(-2isize).offset(1isize) as
                                 libc::c_int) as uint16_t;
                    pc = pc.offset(2isize);
                    b =
                        ((*pc.offset(-2isize).offset(0isize) as libc::c_int)
                             << 8i32 |
                             *pc.offset(-2isize).offset(1isize) as
                                 libc::c_int) as uint16_t;
                    /* *R(a),R(a+1)..R(a+c) = R(a)[b..] */
                    let fresh351 = pc;
                    pc = pc.offset(1);
                    c = *fresh351
                }
                78 => {
                    /* R(a) = intern(R(a)) */
                    let fresh352 = pc;
                    pc = pc.offset(1);
                    a = *fresh352 as uint16_t
                }
                79 => {
                    pc = pc.offset(2isize);
                    a =
                        ((*pc.offset(-2isize).offset(0isize) as libc::c_int)
                             << 8i32 |
                             *pc.offset(-2isize).offset(1isize) as
                                 libc::c_int) as uint16_t;
                    /* R(a) = str_dup(Lit(b)) */
                    pc = pc.offset(2isize);
                    b =
                        ((*pc.offset(-2isize).offset(0isize) as libc::c_int)
                             << 8i32 |
                             *pc.offset(-2isize).offset(1isize) as
                                 libc::c_int) as uint16_t
                }
                80 => {
                    /* str_cat(R(a),R(a+1)) */
                    let fresh353 = pc;
                    pc = pc.offset(1);
                    a = *fresh353 as uint16_t
                }
                81 => {
                    pc = pc.offset(2isize);
                    a =
                        ((*pc.offset(-2isize).offset(0isize) as libc::c_int)
                             << 8i32 |
                             *pc.offset(-2isize).offset(1isize) as
                                 libc::c_int) as uint16_t;
                    /* R(a) = hash_new(R(a),R(a+1)..R(a+b)) */
                    pc = pc.offset(2isize);
                    b =
                        ((*pc.offset(-2isize).offset(0isize) as libc::c_int)
                             << 8i32 |
                             *pc.offset(-2isize).offset(1isize) as
                                 libc::c_int) as uint16_t
                }
                82 => {
                    pc = pc.offset(2isize);
                    a =
                        ((*pc.offset(-2isize).offset(0isize) as libc::c_int)
                             << 8i32 |
                             *pc.offset(-2isize).offset(1isize) as
                                 libc::c_int) as uint16_t;
                    /* R(a) = hash_push(R(a),R(a+1)..R(a+b)) */
                    pc = pc.offset(2isize);
                    b =
                        ((*pc.offset(-2isize).offset(0isize) as libc::c_int)
                             << 8i32 |
                             *pc.offset(-2isize).offset(1isize) as
                                 libc::c_int) as uint16_t
                }
                83 => {
                    /* R(a) = hash_cat(R(a),R(a+1)) */
                    let fresh354 = pc;
                    pc = pc.offset(1);
                    a = *fresh354 as uint16_t
                }
                84 => {
                    pc = pc.offset(2isize);
                    a =
                        ((*pc.offset(-2isize).offset(0isize) as libc::c_int)
                             << 8i32 |
                             *pc.offset(-2isize).offset(1isize) as
                                 libc::c_int) as uint16_t;
                    /* R(a) = lambda(SEQ[b],L_LAMBDA) */
                    pc = pc.offset(2isize);
                    b =
                        ((*pc.offset(-2isize).offset(0isize) as libc::c_int)
                             << 8i32 |
                             *pc.offset(-2isize).offset(1isize) as
                                 libc::c_int) as uint16_t
                }
                85 => {
                    pc = pc.offset(2isize);
                    a =
                        ((*pc.offset(-2isize).offset(0isize) as libc::c_int)
                             << 8i32 |
                             *pc.offset(-2isize).offset(1isize) as
                                 libc::c_int) as uint16_t;
                    /* R(a) = lambda(SEQ[b],L_BLOCK) */
                    pc = pc.offset(2isize);
                    b =
                        ((*pc.offset(-2isize).offset(0isize) as libc::c_int)
                             << 8i32 |
                             *pc.offset(-2isize).offset(1isize) as
                                 libc::c_int) as uint16_t
                }
                86 => {
                    pc = pc.offset(2isize);
                    a =
                        ((*pc.offset(-2isize).offset(0isize) as libc::c_int)
                             << 8i32 |
                             *pc.offset(-2isize).offset(1isize) as
                                 libc::c_int) as uint16_t;
                    /* R(a) = lambda(SEQ[b],L_METHOD) */
                    pc = pc.offset(2isize);
                    b =
                        ((*pc.offset(-2isize).offset(0isize) as libc::c_int)
                             << 8i32 |
                             *pc.offset(-2isize).offset(1isize) as
                                 libc::c_int) as uint16_t
                }
                87 => {
                    /* R(a) = range_new(R(a),R(a+1),FALSE) */
                    let fresh355 = pc;
                    pc = pc.offset(1);
                    a = *fresh355 as uint16_t
                }
                88 => {
                    /* R(a) = range_new(R(a),R(a+1),TRUE) */
                    let fresh356 = pc;
                    pc = pc.offset(1);
                    a = *fresh356 as uint16_t
                }
                89 => {
                    /* R(a) = ::Object */
                    let fresh357 = pc;
                    pc = pc.offset(1);
                    a = *fresh357 as uint16_t
                }
                90 => {
                    pc = pc.offset(2isize);
                    a =
                        ((*pc.offset(-2isize).offset(0isize) as libc::c_int)
                             << 8i32 |
                             *pc.offset(-2isize).offset(1isize) as
                                 libc::c_int) as uint16_t;
                    /* R(a) = newclass(R(a),Syms(b),R(a+1)) */
                    pc = pc.offset(2isize);
                    b =
                        ((*pc.offset(-2isize).offset(0isize) as libc::c_int)
                             << 8i32 |
                             *pc.offset(-2isize).offset(1isize) as
                                 libc::c_int) as uint16_t
                }
                91 => {
                    pc = pc.offset(2isize);
                    a =
                        ((*pc.offset(-2isize).offset(0isize) as libc::c_int)
                             << 8i32 |
                             *pc.offset(-2isize).offset(1isize) as
                                 libc::c_int) as uint16_t;
                    /* R(a) = newmodule(R(a),Syms(b)) */
                    pc = pc.offset(2isize);
                    b =
                        ((*pc.offset(-2isize).offset(0isize) as libc::c_int)
                             << 8i32 |
                             *pc.offset(-2isize).offset(1isize) as
                                 libc::c_int) as uint16_t
                }
                92 => {
                    pc = pc.offset(2isize);
                    a =
                        ((*pc.offset(-2isize).offset(0isize) as libc::c_int)
                             << 8i32 |
                             *pc.offset(-2isize).offset(1isize) as
                                 libc::c_int) as uint16_t;
                    /* R(a) = blockexec(R(a),SEQ[b]) */
                    pc = pc.offset(2isize);
                    b =
                        ((*pc.offset(-2isize).offset(0isize) as libc::c_int)
                             << 8i32 |
                             *pc.offset(-2isize).offset(1isize) as
                                 libc::c_int) as uint16_t
                }
                93 => {
                    pc = pc.offset(2isize);
                    a =
                        ((*pc.offset(-2isize).offset(0isize) as libc::c_int)
                             << 8i32 |
                             *pc.offset(-2isize).offset(1isize) as
                                 libc::c_int) as uint16_t;
                    /* R(a).newmethod(Syms(b),R(a+1)) */
                    pc = pc.offset(2isize);
                    b =
                        ((*pc.offset(-2isize).offset(0isize) as libc::c_int)
                             << 8i32 |
                             *pc.offset(-2isize).offset(1isize) as
                                 libc::c_int) as uint16_t
                }
                94 => {
                    pc = pc.offset(2isize);
                    a =
                        ((*pc.offset(-2isize).offset(0isize) as libc::c_int)
                             << 8i32 |
                             *pc.offset(-2isize).offset(1isize) as
                                 libc::c_int) as uint16_t;
                    /* alias_method(target_class,Syms(a),Syms(b)) */
                    pc = pc.offset(2isize);
                    b =
                        ((*pc.offset(-2isize).offset(0isize) as libc::c_int)
                             << 8i32 |
                             *pc.offset(-2isize).offset(1isize) as
                                 libc::c_int) as uint16_t
                }
                95 => {
                    /* undef_method(target_class,Syms(a)) */
                    let fresh358 = pc;
                    pc = pc.offset(1);
                    a = *fresh358 as uint16_t
                }
                96 => {
                    /* R(a) = R(a).singleton_class */
                    let fresh359 = pc;
                    pc = pc.offset(1);
                    a = *fresh359 as uint16_t
                }
                97 => {
                    /* R(a) = target_class */
                    let fresh360 = pc;
                    pc = pc.offset(1);
                    a = *fresh360 as uint16_t
                }
                98 => {
                    pc = pc.offset(2isize);
                    a =
                        ((*pc.offset(-2isize).offset(0isize) as libc::c_int)
                             << 8i32 |
                             *pc.offset(-2isize).offset(1isize) as
                                 libc::c_int) as uint16_t;
                    pc = pc.offset(2isize);
                    b =
                        ((*pc.offset(-2isize).offset(0isize) as libc::c_int)
                             << 8i32 |
                             *pc.offset(-2isize).offset(1isize) as
                                 libc::c_int) as uint16_t;
                    /* print a,b,c */
                    let fresh361 = pc;
                    pc = pc.offset(1);
                    c = *fresh361
                }
                99 => {
                    /* raise(LocalJumpError, Lit(a)) */
                    let fresh362 = pc;
                    pc = pc.offset(1);
                    a = *fresh362 as uint16_t
                }
                0 | 48 | 53 | 100 | 101 | 102 | 103 | _ => { }
            }
        }
        _ => { }
    }
    data.insn = insn;
    data.a = a;
    data.b = b;
    data.c = c;
    return data;
}
unsafe extern "C" fn mrb_last_insn(mut s: *mut codegen_scope)
 -> mrb_insn_data {
    if (*s).pc as libc::c_int == (*s).lastpc as libc::c_int {
        let mut data: mrb_insn_data =
            mrb_insn_data{insn: 0, a: 0, b: 0, c: 0,};
        data.insn = OP_NOP as libc::c_int as uint8_t;
        return data
    }
    return mrb_decode_insn(&mut *(*s).iseq.offset((*s).lastpc as isize));
}
unsafe extern "C" fn no_peephole(mut s: *mut codegen_scope) -> mrb_bool {
    return (0 != no_optimize(s) as libc::c_int ||
                (*s).lastlabel as libc::c_int == (*s).pc as libc::c_int ||
                (*s).pc as libc::c_int == 0i32 ||
                (*s).pc as libc::c_int == (*s).lastpc as libc::c_int) as
               libc::c_int as mrb_bool;
}
unsafe extern "C" fn genjmp(mut s: *mut codegen_scope, mut i: mrb_code,
                            mut pc: uint16_t) -> uint16_t {
    let mut pos: uint16_t = 0;
    (*s).lastpc = (*s).pc;
    gen_B(s, i);
    pos = (*s).pc;
    gen_S(s, pc);
    return pos;
}
unsafe extern "C" fn genjmp2(mut s: *mut codegen_scope, mut i: mrb_code,
                             mut a: uint16_t, mut pc: libc::c_int,
                             mut val: libc::c_int) -> uint16_t {
    let mut pos: uint16_t = 0;
    if 0 == no_peephole(s) && 0 == val {
        let mut data: mrb_insn_data = mrb_last_insn(s);
        if data.insn as libc::c_int == OP_MOVE as libc::c_int &&
               data.a as libc::c_int == a as libc::c_int {
            (*s).pc = (*s).lastpc;
            a = data.b
        }
    }
    (*s).lastpc = (*s).pc;
    if a as libc::c_int > 0xffi32 {
        gen_B(s, OP_EXT1 as libc::c_int as uint8_t);
        gen_B(s, i);
        gen_S(s, a);
        pos = (*s).pc;
        gen_S(s, pc as uint16_t);
    } else {
        gen_B(s, i);
        gen_B(s, a as uint8_t);
        pos = (*s).pc;
        gen_S(s, pc as uint16_t);
    }
    return pos;
}
unsafe extern "C" fn gen_move(mut s: *mut codegen_scope, mut dst: uint16_t,
                              mut src: uint16_t, mut nopeep: libc::c_int) {
    let mut current_block: u64;
    if !(0 != no_peephole(s)) {
        let mut data: mrb_insn_data = mrb_last_insn(s);
        match data.insn as libc::c_int {
            1 => {
                current_block = 8695478541019159461;
                match current_block {
                    8695478541019159461 => {
                        if dst as libc::c_int == src as libc::c_int {
                            /* remove useless MOVE */
                            return
                        }
                        /* skip swapping MOVE */
                        if data.b as libc::c_int == dst as libc::c_int &&
                               data.a as libc::c_int == src as libc::c_int {
                            return
                        }
                        current_block = 15487034753282153540;
                    }
                    17770521996153494535 => {
                        if 0 != nopeep ||
                               data.a as libc::c_int != src as libc::c_int ||
                               (data.a as libc::c_int) <
                                   (*s).nlocals as libc::c_int {
                            current_block = 15487034753282153540;
                        } else {
                            (*s).pc = (*s).lastpc;
                            genop_1(s, data.insn, dst);
                            current_block = 17788412896529399552;
                        }
                    }
                    _ => {
                        if 0 != nopeep ||
                               data.a as libc::c_int != src as libc::c_int ||
                               (data.a as libc::c_int) <
                                   (*s).nlocals as libc::c_int {
                            current_block = 15487034753282153540;
                        } else {
                            (*s).pc = (*s).lastpc;
                            genop_2(s, data.insn, dst, data.b);
                            current_block = 17788412896529399552;
                        }
                    }
                }
                match current_block {
                    15487034753282153540 => { }
                    _ => { return; }
                }
            }
            15 | 16 | 17 | 18 | 5 | 6 | 7 | 8 | 9 | 10 | 11 | 12 | 13 => {
                current_block = 17770521996153494535;
                match current_block {
                    8695478541019159461 => {
                        if dst as libc::c_int == src as libc::c_int { return }
                        if data.b as libc::c_int == dst as libc::c_int &&
                               data.a as libc::c_int == src as libc::c_int {
                            return
                        }
                        current_block = 15487034753282153540;
                    }
                    17770521996153494535 => {
                        if 0 != nopeep ||
                               data.a as libc::c_int != src as libc::c_int ||
                               (data.a as libc::c_int) <
                                   (*s).nlocals as libc::c_int {
                            current_block = 15487034753282153540;
                        } else {
                            (*s).pc = (*s).lastpc;
                            genop_1(s, data.insn, dst);
                            current_block = 17788412896529399552;
                        }
                    }
                    _ => {
                        if 0 != nopeep ||
                               data.a as libc::c_int != src as libc::c_int ||
                               (data.a as libc::c_int) <
                                   (*s).nlocals as libc::c_int {
                            current_block = 15487034753282153540;
                        } else {
                            (*s).pc = (*s).lastpc;
                            genop_2(s, data.insn, dst, data.b);
                            current_block = 17788412896529399552;
                        }
                    }
                }
                match current_block {
                    15487034753282153540 => { }
                    _ => { return; }
                }
            }
            3 | 4 | 2 | 14 | 19 | 21 | 23 | 25 | 27 | 79 | 84 | 85 | 86 | 58
            => {
                current_block = 10801606655216325246;
                match current_block {
                    8695478541019159461 => {
                        if dst as libc::c_int == src as libc::c_int { return }
                        if data.b as libc::c_int == dst as libc::c_int &&
                               data.a as libc::c_int == src as libc::c_int {
                            return
                        }
                        current_block = 15487034753282153540;
                    }
                    17770521996153494535 => {
                        if 0 != nopeep ||
                               data.a as libc::c_int != src as libc::c_int ||
                               (data.a as libc::c_int) <
                                   (*s).nlocals as libc::c_int {
                            current_block = 15487034753282153540;
                        } else {
                            (*s).pc = (*s).lastpc;
                            genop_1(s, data.insn, dst);
                            current_block = 17788412896529399552;
                        }
                    }
                    _ => {
                        if 0 != nopeep ||
                               data.a as libc::c_int != src as libc::c_int ||
                               (data.a as libc::c_int) <
                                   (*s).nlocals as libc::c_int {
                            current_block = 15487034753282153540;
                        } else {
                            (*s).pc = (*s).lastpc;
                            genop_2(s, data.insn, dst, data.b);
                            current_block = 17788412896529399552;
                        }
                    }
                }
                match current_block {
                    15487034753282153540 => { }
                    _ => { return; }
                }
            }
            _ => { }
        }
    }
    genop_2(s, OP_MOVE as libc::c_int as mrb_code, dst, src);
    if 0 != on_eval(s) { genop_0(s, OP_NOP as libc::c_int as mrb_code); };
}
unsafe extern "C" fn gen_return(mut s: *mut codegen_scope, mut op: uint8_t,
                                mut src: uint16_t) {
    if 0 != no_peephole(s) {
        genop_1(s, op, src);
    } else {
        let mut data: mrb_insn_data = mrb_last_insn(s);
        if data.insn as libc::c_int == OP_MOVE as libc::c_int &&
               src as libc::c_int == data.a as libc::c_int {
            (*s).pc = (*s).lastpc;
            genop_1(s, op, data.b);
        } else if data.insn as libc::c_int != OP_RETURN as libc::c_int {
            genop_1(s, op, src);
        }
    };
}
unsafe extern "C" fn gen_addsub(mut s: *mut codegen_scope, mut op: uint8_t,
                                mut dst: uint16_t) {
    let mut current_block: u64;
    if !(0 != no_peephole(s)) {
        let mut data: mrb_insn_data = mrb_last_insn(s);
        match data.insn as libc::c_int {
            5 => {
                current_block = 12439645634401656168;
                match current_block {
                    15894668686862115448 => {
                        data.b =
                            (data.insn as libc::c_int -
                                 OP_LOADI_0 as libc::c_int) as uint16_t
                    }
                    12439645634401656168 => {
                        /* fall through */
                        if op as libc::c_int == OP_ADD as libc::c_int {
                            op = OP_SUB as libc::c_int as uint8_t
                        } else { op = OP_ADD as libc::c_int as uint8_t }
                        data.b = 1i32 as uint16_t
                    }
                    _ => { }
                }
                if !(data.b as libc::c_int >= 128i32) {
                    (*s).pc = (*s).lastpc;
                    if op as libc::c_int == OP_ADD as libc::c_int {
                        genop_2(s, OP_ADDI as libc::c_int as mrb_code, dst,
                                data.b as uint8_t as uint16_t);
                    } else {
                        genop_2(s, OP_SUBI as libc::c_int as mrb_code, dst,
                                data.b as uint8_t as uint16_t);
                    }
                    return;
                }
            }
            6 | 7 | 8 | 9 | 10 | 11 | 12 | 13 => {
                current_block = 15894668686862115448;
                match current_block {
                    15894668686862115448 => {
                        data.b =
                            (data.insn as libc::c_int -
                                 OP_LOADI_0 as libc::c_int) as uint16_t
                    }
                    12439645634401656168 => {
                        /* fall through */
                        if op as libc::c_int == OP_ADD as libc::c_int {
                            op = OP_SUB as libc::c_int as uint8_t
                        } else { op = OP_ADD as libc::c_int as uint8_t }
                        data.b = 1i32 as uint16_t
                    }
                    _ => { }
                }
                if !(data.b as libc::c_int >= 128i32) {
                    (*s).pc = (*s).lastpc;
                    if op as libc::c_int == OP_ADD as libc::c_int {
                        genop_2(s, OP_ADDI as libc::c_int as mrb_code, dst,
                                data.b as uint8_t as uint16_t);
                    } else {
                        genop_2(s, OP_SUBI as libc::c_int as mrb_code, dst,
                                data.b as uint8_t as uint16_t);
                    }
                    return;
                }
            }
            3 => {
                current_block = 6770949863025170494;
                match current_block {
                    15894668686862115448 => {
                        data.b =
                            (data.insn as libc::c_int -
                                 OP_LOADI_0 as libc::c_int) as uint16_t
                    }
                    12439645634401656168 => {
                        /* fall through */
                        if op as libc::c_int == OP_ADD as libc::c_int {
                            op = OP_SUB as libc::c_int as uint8_t
                        } else { op = OP_ADD as libc::c_int as uint8_t }
                        data.b = 1i32 as uint16_t
                    }
                    _ => { }
                }
                if !(data.b as libc::c_int >= 128i32) {
                    (*s).pc = (*s).lastpc;
                    if op as libc::c_int == OP_ADD as libc::c_int {
                        genop_2(s, OP_ADDI as libc::c_int as mrb_code, dst,
                                data.b as uint8_t as uint16_t);
                    } else {
                        genop_2(s, OP_SUBI as libc::c_int as mrb_code, dst,
                                data.b as uint8_t as uint16_t);
                    }
                    return;
                }
            }
            _ => { }
        }
    }
    genop_1(s, op, dst);
}
unsafe extern "C" fn dispatch(mut s: *mut codegen_scope, mut pos0: uint16_t)
 -> libc::c_int {
    let mut newpos: uint16_t = 0;
    (*s).lastlabel = (*s).pc;
    newpos =
        ((*(*s).iseq.offset(pos0 as libc::c_int as isize).offset(0isize) as
              libc::c_int) << 8i32 |
             *(*s).iseq.offset(pos0 as libc::c_int as isize).offset(1isize) as
                 libc::c_int) as uint16_t;
    emit_S(s, pos0 as libc::c_int, (*s).pc);
    return newpos as libc::c_int;
}
unsafe extern "C" fn dispatch_linked(mut s: *mut codegen_scope,
                                     mut pos: uint16_t) {
    if pos as libc::c_int == 0i32 { return }
    loop  {
        pos = dispatch(s, pos) as uint16_t;
        if pos as libc::c_int == 0i32 { break ; }
    };
}
unsafe extern "C" fn push_n_(mut s: *mut codegen_scope, mut n: libc::c_int) {
    if (*s).sp as libc::c_int + n >= 0xffffi32 {
        codegen_error(s,
                      b"too complex expression\x00" as *const u8 as
                          *const libc::c_char);
    }
    (*s).sp = ((*s).sp as libc::c_int + n) as uint16_t;
    if (*s).sp as libc::c_int > (*s).nregs as libc::c_int {
        (*s).nregs = (*s).sp
    };
}
unsafe extern "C" fn pop_n_(mut s: *mut codegen_scope, mut n: libc::c_int) {
    if (*s).sp as libc::c_int - n < 0i32 {
        codegen_error(s,
                      b"stack pointer underflow\x00" as *const u8 as
                          *const libc::c_char);
    }
    (*s).sp = ((*s).sp as libc::c_int - n) as uint16_t;
}
#[inline]
unsafe extern "C" fn new_lit(mut s: *mut codegen_scope, mut val: mrb_value)
 -> libc::c_int {
    let mut i: libc::c_int = 0;
    let mut pv: *mut mrb_value = 0 as *mut mrb_value;
    match val.tt as libc::c_uint {
        16 => {
            i = 0i32;
            while i < (*(*s).irep).plen as libc::c_int {
                let mut len: mrb_int = 0;
                pv =
                    &mut *(*(*s).irep).pool.offset(i as isize) as
                        *mut mrb_value;
                if !((*pv).tt as libc::c_uint !=
                         MRB_TT_STRING as libc::c_int as libc::c_uint) {
                    len =
                        (if 0 !=
                                (*((*pv).value.p as *mut RString)).flags() as
                                    libc::c_int & 32i32 {
                             (((*((*pv).value.p as *mut RString)).flags() as
                                   libc::c_int & 0x7c0i32) >> 6i32) as mrb_int
                         } else {
                             (*((*pv).value.p as *mut RString)).as_0.heap.len
                         });
                    if !(len !=
                             (if 0 !=
                                     (*(val.value.p as *mut RString)).flags()
                                         as libc::c_int & 32i32 {
                                  (((*(val.value.p as *mut RString)).flags()
                                        as libc::c_int & 0x7c0i32) >> 6i32) as
                                      mrb_int
                              } else {
                                  (*(val.value.p as
                                         *mut RString)).as_0.heap.len
                              })) {
                        if memcmp((if 0 !=
                                          (*((*pv).value.p as
                                                 *mut RString)).flags() as
                                              libc::c_int & 32i32 {
                                       (*((*pv).value.p as
                                              *mut RString)).as_0.ary.as_mut_ptr()
                                   } else {
                                       (*((*pv).value.p as
                                              *mut RString)).as_0.heap.ptr
                                   }) as *const libc::c_void,
                                  (if 0 !=
                                          (*(val.value.p as
                                                 *mut RString)).flags() as
                                              libc::c_int & 32i32 {
                                       (*(val.value.p as
                                              *mut RString)).as_0.ary.as_mut_ptr()
                                   } else {
                                       (*(val.value.p as
                                              *mut RString)).as_0.heap.ptr
                                   }) as *const libc::c_void,
                                  len as libc::c_ulong) == 0i32 {
                            return i
                        }
                    }
                }
                i += 1
            }
        }
        6 => {
            i = 0i32;
            while i < (*(*s).irep).plen as libc::c_int {
                let mut f1: mrb_float = 0.;
                let mut f2: mrb_float = 0.;
                pv =
                    &mut *(*(*s).irep).pool.offset(i as isize) as
                        *mut mrb_value;
                if !((*pv).tt as libc::c_uint !=
                         MRB_TT_FLOAT as libc::c_int as libc::c_uint) {
                    f1 = (*pv).value.f;
                    f2 = val.value.f;
                    if f1 == f2 &&
                           (0 ==
                                (if ::std::mem::size_of::<mrb_float>() as
                                        libc::c_ulong ==
                                        ::std::mem::size_of::<libc::c_float>()
                                            as libc::c_ulong {
                                     __inline_signbitf(f1 as libc::c_float)
                                 } else {
                                     (if ::std::mem::size_of::<mrb_float>() as
                                             libc::c_ulong ==
                                             ::std::mem::size_of::<libc::c_double>()
                                                 as libc::c_ulong {
                                          __inline_signbitd(f1)
                                      } else {
                                          __inline_signbitl(f128::f128::new(f1))
                                      })
                                 })) as libc::c_int ==
                               (0 ==
                                    (if ::std::mem::size_of::<mrb_float>() as
                                            libc::c_ulong ==
                                            ::std::mem::size_of::<libc::c_float>()
                                                as libc::c_ulong {
                                         __inline_signbitf(f2 as
                                                               libc::c_float)
                                     } else {
                                         (if ::std::mem::size_of::<mrb_float>()
                                                 as libc::c_ulong ==
                                                 ::std::mem::size_of::<libc::c_double>()
                                                     as libc::c_ulong {
                                              __inline_signbitd(f2)
                                          } else {
                                              __inline_signbitl(f128::f128::new(f2))
                                          })
                                     })) as libc::c_int {
                        return i
                    }
                }
                i += 1
            }
        }
        3 => {
            i = 0i32;
            while i < (*(*s).irep).plen as libc::c_int {
                pv =
                    &mut *(*(*s).irep).pool.offset(i as isize) as
                        *mut mrb_value;
                if (*pv).tt as libc::c_uint ==
                       MRB_TT_FIXNUM as libc::c_int as libc::c_uint {
                    if (*pv).value.i == val.value.i { return i }
                }
                i += 1
            }
        }
        _ => {
            /* should not happen */
            return 0i32
        }
    }
    if (*(*s).irep).plen as libc::c_uint == (*s).pcapa {
        (*s).pcapa =
            ((*s).pcapa as libc::c_uint).wrapping_mul(2i32 as libc::c_uint) as
                uint32_t as uint32_t;
        (*(*s).irep).pool =
            codegen_realloc(s, (*(*s).irep).pool as *mut libc::c_void,
                            (::std::mem::size_of::<mrb_value>() as
                                 libc::c_ulong).wrapping_mul((*s).pcapa as
                                                                 libc::c_ulong))
                as *mut mrb_value
    }
    pv =
        &mut *(*(*s).irep).pool.offset((*(*s).irep).plen as isize) as
            *mut mrb_value;
    let fresh363 = (*(*s).irep).plen;
    (*(*s).irep).plen = (*(*s).irep).plen.wrapping_add(1);
    i = fresh363 as libc::c_int;
    match val.tt as libc::c_uint {
        16 => { *pv = mrb_str_pool((*s).mrb, val) }
        6 | 3 => { *pv = val }
        _ => { }
    }
    return i;
}
/* maximum symbol numbers */
unsafe extern "C" fn new_sym(mut s: *mut codegen_scope, mut sym: mrb_sym)
 -> libc::c_int {
    let mut i: libc::c_int = 0;
    let mut len: libc::c_int = 0;
    len = (*(*s).irep).slen as libc::c_int;
    i = 0i32;
    while i < len {
        if *(*(*s).irep).syms.offset(i as isize) == sym { return i }
        i += 1
    }
    if (*(*s).irep).slen as libc::c_uint >= (*s).scapa {
        (*s).scapa =
            ((*s).scapa as libc::c_uint).wrapping_mul(2i32 as libc::c_uint) as
                uint32_t as uint32_t;
        (*(*s).irep).syms =
            codegen_realloc(s, (*(*s).irep).syms as *mut libc::c_void,
                            (::std::mem::size_of::<mrb_sym>() as
                                 libc::c_ulong).wrapping_mul((*s).scapa as
                                                                 libc::c_ulong))
                as *mut mrb_sym
    }
    *(*(*s).irep).syms.offset((*(*s).irep).slen as isize) = sym;
    let fresh364 = (*(*s).irep).slen;
    (*(*s).irep).slen = (*(*s).irep).slen.wrapping_add(1);
    return fresh364 as libc::c_int;
}
unsafe extern "C" fn node_len(mut tree: *mut node) -> libc::c_int {
    let mut n: libc::c_int = 0i32;
    while !tree.is_null() { n += 1; tree = (*tree).cdr }
    return n;
}
unsafe extern "C" fn lv_idx(mut s: *mut codegen_scope, mut id: mrb_sym)
 -> libc::c_int {
    let mut lv: *mut node = (*s).lv;
    let mut n: libc::c_int = 1i32;
    while !lv.is_null() {
        if (*lv).car as intptr_t as mrb_sym == id { return n }
        n += 1;
        lv = (*lv).cdr
    }
    return 0i32;
}
unsafe extern "C" fn for_body(mut s: *mut codegen_scope,
                              mut tree: *mut node) {
    let mut prev: *mut codegen_scope = s;
    let mut idx: libc::c_int = 0;
    let mut lp: *mut loopinfo = 0 as *mut loopinfo;
    let mut n2: *mut node = 0 as *mut node;
    /* generate receiver */
    codegen(s, (*(*tree).cdr).car, 1i32);
    /* generate loop-block */
    s = scope_new((*s).mrb, s, 0 as *mut node);
    if s.is_null() {
        raise_error(prev,
                    b"unexpected scope\x00" as *const u8 as
                        *const libc::c_char);
    }
    /* push for a block parameter */
    push_n_(s, 1i32);
    /* generate loop variable */
    n2 = (*tree).car;
    genop_W(s, OP_ENTER as libc::c_int as mrb_code, 0x40000i32 as uint32_t);
    if !(*n2).car.is_null() && (*(*n2).car).cdr.is_null() &&
           (*n2).cdr.is_null() {
        gen_assignment(s, (*(*n2).car).car, 1i32, 0i32);
    } else { gen_vmassignment(s, n2, 1i32, 1i32); }
    /* construct loop */
    lp = loop_push(s, LOOP_FOR);
    (*lp).pc2 = new_label(s);
    /* loop body */
    codegen(s, (*(*(*tree).cdr).cdr).car, 1i32);
    pop_n_(s, 1i32);
    gen_return(s, OP_RETURN as libc::c_int as uint8_t, (*s).sp);
    loop_pop(s, 0i32);
    scope_finish(s);
    s = prev;
    genop_2(s, OP_BLOCK as libc::c_int as mrb_code, (*s).sp,
            ((*(*s).irep).rlen as libc::c_int - 1i32) as uint16_t);
    push_n_(s, 1i32);
    /* space for a block */
    pop_n_(s, 1i32);
    pop_n_(s, 1i32);
    idx =
        new_sym(s,
                mrb_intern_static((*s).mrb,
                                  b"each\x00" as *const u8 as
                                      *const libc::c_char,
                                  (::std::mem::size_of::<[libc::c_char; 5]>()
                                       as
                                       libc::c_ulong).wrapping_sub(1i32 as
                                                                       libc::c_ulong)));
    genop_3(s, OP_SENDB as libc::c_int as mrb_code, (*s).sp, idx as uint16_t,
            0i32 as uint8_t);
}
unsafe extern "C" fn lambda_body(mut s: *mut codegen_scope,
                                 mut tree: *mut node, mut blk: libc::c_int)
 -> libc::c_int {
    let mut parent: *mut codegen_scope = s;
    s = scope_new((*s).mrb, s, (*tree).car);
    if s.is_null() {
        raise_error(parent,
                    b"unexpected scope\x00" as *const u8 as
                        *const libc::c_char);
    }
    (*s).set_mscope((0 == blk) as libc::c_int as mrb_bool);
    if 0 != blk {
        let mut lp: *mut loopinfo = loop_push(s, LOOP_BLOCK);
        (*lp).pc0 = new_label(s)
    }
    tree = (*tree).cdr;
    if (*tree).car.is_null() {
        genop_W(s, OP_ENTER as libc::c_int as mrb_code, 0i32 as uint32_t);
    } else {
        let mut a: mrb_aspec = 0;
        let mut ma: libc::c_int = 0;
        let mut oa: libc::c_int = 0;
        let mut ra: libc::c_int = 0;
        let mut pa: libc::c_int = 0;
        let mut ka: libc::c_int = 0;
        let mut kd: libc::c_int = 0;
        let mut ba: libc::c_int = 0;
        let mut pos: libc::c_int = 0;
        let mut i: libc::c_int = 0;
        let mut opt: *mut node = 0 as *mut node;
        let mut margs: *mut node = 0 as *mut node;
        let mut pargs: *mut node = 0 as *mut node;
        let mut tail: *mut node = 0 as *mut node;
        /* mandatory arguments */
        ma = node_len((*(*tree).car).car);
        margs = (*(*tree).car).car;
        tail = (*(*(*(*(*tree).car).cdr).cdr).cdr).cdr;
        /* optional arguments */
        oa = node_len((*(*(*tree).car).cdr).car);
        /* rest argument? */
        ra =
            if !(*(*(*(*tree).car).cdr).cdr).car.is_null() {
                1i32
            } else { 0i32 };
        /* mandatory arugments after rest argument */
        pa = node_len((*(*(*(*(*tree).car).cdr).cdr).cdr).car);
        pargs = (*(*(*(*(*tree).car).cdr).cdr).cdr).car;
        /* keyword arguments */
        ka =
            if !tail.is_null() { node_len((*(*tail).cdr).car) } else { 0i32 };
        /* keyword dictionary? */
        kd =
            if !tail.is_null() && !(*(*(*tail).cdr).cdr).car.is_null() {
                1i32
            } else { 0i32 };
        /* block argument? */
        ba =
            if !tail.is_null() && !(*(*(*(*tail).cdr).cdr).cdr).car.is_null()
               {
                1i32
            } else { 0i32 };
        if ma > 0x1fi32 || oa > 0x1fi32 || pa > 0x1fi32 || ka > 0x1fi32 {
            codegen_error(s,
                          b"too many formal arguments\x00" as *const u8 as
                              *const libc::c_char);
        }
        a =
            ((ma & 0x1fi32) as mrb_aspec) << 18i32 |
                ((oa & 0x1fi32) as mrb_aspec) << 13i32 |
                (if 0 != ra {
                     (1i32 << 12i32) as mrb_aspec
                 } else { 0i32 as libc::c_uint }) |
                ((pa & 0x1fi32) as mrb_aspec) << 7i32 |
                ((ka & 0x1fi32) << 2i32 |
                     (if 0 != kd { 1i32 << 1i32 } else { 0i32 })) as mrb_aspec
                |
                (if 0 != ba {
                     1i32 as mrb_aspec
                 } else { 0i32 as libc::c_uint });
        /* (12bits = 5:1:5:1) */
        (*s).set_ainfo((ma + oa & 0x3fi32) << 7i32 | (ra & 0x1i32) << 6i32 |
                           (pa & 0x1fi32) << 1i32 | kd & 0x1i32);
        genop_W(s, OP_ENTER as libc::c_int as mrb_code, a);
        /* generate jump table for optional arguments initializer */
        pos = new_label(s);
        i = 0i32;
        while i < oa {
            new_label(s);
            genjmp(s, OP_JMP as libc::c_int as mrb_code, 0i32 as uint16_t);
            i += 1
        }
        if oa > 0i32 {
            genjmp(s, OP_JMP as libc::c_int as mrb_code, 0i32 as uint16_t);
        }
        opt = (*(*(*tree).car).cdr).car;
        i = 0i32;
        while !opt.is_null() {
            let mut idx: libc::c_int = 0;
            dispatch(s, (pos + i * 3i32 + 1i32) as uint16_t);
            codegen(s, (*(*opt).car).cdr, 1i32);
            pop_n_(s, 1i32);
            idx = lv_idx(s, (*(*opt).car).car as intptr_t as mrb_sym);
            gen_move(s, idx as uint16_t, (*s).sp, 0i32);
            i += 1;
            opt = (*opt).cdr
        }
        if oa > 0i32 { dispatch(s, (pos + i * 3i32 + 1i32) as uint16_t); }
        /* keyword arguments */
        if !tail.is_null() {
            let mut kwds: *mut node = (*(*tail).cdr).car;
            let mut kwrest: libc::c_int = 0i32;
            if !(*(*(*tail).cdr).cdr).car.is_null() { kwrest = 1i32 }
            while !kwds.is_null() {
                let mut jmpif_key_p: libc::c_int = 0;
                let mut jmp_def_set: libc::c_int = -1i32;
                let mut kwd: *mut node = (*kwds).car;
                let mut def_arg: *mut node = (*(*(*kwd).cdr).cdr).car;
                let mut kwd_sym: mrb_sym =
                    (*(*kwd).cdr).car as intptr_t as mrb_sym;
                if !def_arg.is_null() {
                    genop_2(s, OP_KEY_P as libc::c_int as mrb_code,
                            lv_idx(s, kwd_sym) as uint16_t,
                            new_sym(s, kwd_sym) as uint16_t);
                    jmpif_key_p =
                        genjmp2(s, OP_JMPIF as libc::c_int as mrb_code,
                                lv_idx(s, kwd_sym) as uint16_t, 0i32, 0i32) as
                            libc::c_int;
                    codegen(s, def_arg, 1i32);
                    pop_n_(s, 1i32);
                    gen_move(s, lv_idx(s, kwd_sym) as uint16_t, (*s).sp,
                             0i32);
                    jmp_def_set =
                        genjmp(s, OP_JMP as libc::c_int as mrb_code,
                               0i32 as uint16_t) as libc::c_int;
                    dispatch(s, jmpif_key_p as uint16_t);
                }
                genop_2(s, OP_KARG as libc::c_int as mrb_code,
                        lv_idx(s, kwd_sym) as uint16_t,
                        new_sym(s, kwd_sym) as uint16_t);
                if jmp_def_set != -1i32 {
                    dispatch(s, jmp_def_set as uint16_t);
                }
                i += 1;
                kwds = (*kwds).cdr
            }
            if !(*(*tail).cdr).car.is_null() && 0 == kwrest {
                genop_0(s, OP_KEYEND as libc::c_int as mrb_code);
            }
        }
        /* argument destructuring */
        if !margs.is_null() {
            let mut n: *mut node = margs;
            pos = 1i32;
            while !n.is_null() {
                if (*(*n).car).car as intptr_t as libc::c_int ==
                       NODE_MASGN as libc::c_int {
                    gen_vmassignment(s, (*(*(*n).car).cdr).car, pos, 0i32);
                }
                pos += 1;
                n = (*n).cdr
            }
        }
        if !pargs.is_null() {
            let mut n_0: *mut node = margs;
            pos = ma + oa + ra + 1i32;
            while !n_0.is_null() {
                if (*(*n_0).car).car as intptr_t as libc::c_int ==
                       NODE_MASGN as libc::c_int {
                    gen_vmassignment(s, (*(*(*n_0).car).cdr).car, pos, 0i32);
                }
                pos += 1;
                n_0 = (*n_0).cdr
            }
        }
    }
    codegen(s, (*(*tree).cdr).car, 1i32);
    pop_n_(s, 1i32);
    if (*s).pc as libc::c_int > 0i32 {
        gen_return(s, OP_RETURN as libc::c_int as uint8_t, (*s).sp);
    }
    if 0 != blk { loop_pop(s, 0i32); }
    scope_finish(s);
    return (*(*parent).irep).rlen as libc::c_int - 1i32;
}
unsafe extern "C" fn scope_body(mut s: *mut codegen_scope,
                                mut tree: *mut node, mut val: libc::c_int)
 -> libc::c_int {
    let mut scope: *mut codegen_scope = scope_new((*s).mrb, s, (*tree).car);
    if scope.is_null() {
        codegen_error(s,
                      b"unexpected scope\x00" as *const u8 as
                          *const libc::c_char);
    }
    codegen(scope, (*tree).cdr, 1i32);
    gen_return(scope, OP_RETURN as libc::c_int as uint8_t,
               ((*scope).sp as libc::c_int - 1i32) as uint16_t);
    if (*s).iseq.is_null() {
        genop_0(scope, OP_STOP as libc::c_int as mrb_code);
    }
    scope_finish(scope);
    if (*s).irep.is_null() {
        /* should not happen */
        return 0i32
    }
    return (*(*s).irep).rlen as libc::c_int - 1i32;
}
unsafe extern "C" fn nosplat(mut t: *mut node) -> mrb_bool {
    while !t.is_null() {
        if (*(*t).car).car as intptr_t as libc::c_int ==
               NODE_SPLAT as libc::c_int {
            return 0i32 as mrb_bool
        }
        t = (*t).cdr
    }
    return 1i32 as mrb_bool;
}
unsafe extern "C" fn attrsym(mut s: *mut codegen_scope, mut a: mrb_sym)
 -> mrb_sym {
    let mut name: *const libc::c_char = 0 as *const libc::c_char;
    let mut len: mrb_int = 0;
    let mut name2: *mut libc::c_char = 0 as *mut libc::c_char;
    name = mrb_sym2name_len((*s).mrb, a, &mut len);
    name2 =
        codegen_palloc(s,
                       (len as
                            size_t).wrapping_add(1i32 as
                                                     libc::c_ulong).wrapping_add(1i32
                                                                                     as
                                                                                     libc::c_ulong))
            as *mut libc::c_char;
    memcpy(name2 as *mut libc::c_void, name as *const libc::c_void,
           len as size_t);
    *name2.offset(len as isize) = '=' as i32 as libc::c_char;
    *name2.offset((len + 1i32 as libc::c_longlong) as isize) =
        '\u{0}' as i32 as libc::c_char;
    return mrb_intern((*s).mrb, name2,
                      (len + 1i32 as libc::c_longlong) as size_t);
}
unsafe extern "C" fn gen_values(mut s: *mut codegen_scope, mut t: *mut node,
                                mut val: libc::c_int, mut extra: libc::c_int)
 -> libc::c_int {
    let mut n: libc::c_int = 0i32;
    let mut is_splat: libc::c_int = 0;
    while !t.is_null() {
        /* splat mode */
        is_splat =
            ((*(*t).car).car as intptr_t as libc::c_int ==
                 NODE_SPLAT as libc::c_int) as libc::c_int;
        if n + extra >= 127i32 - 1i32 || 0 != is_splat {
            /* need to subtract one because vm.c expects an array if n == CALL_MAXARGS */
            if 0 != val {
                if 0 != is_splat && n == 0i32 &&
                       (*(*(*t).car).cdr).car as intptr_t as libc::c_int ==
                           NODE_ARRAY as libc::c_int {
                    codegen(s, (*(*t).car).cdr, 1i32);
                    pop_n_(s, 1i32);
                } else {
                    pop_n_(s, n);
                    genop_2(s, OP_ARRAY as libc::c_int as mrb_code, (*s).sp,
                            n as uint16_t);
                    push_n_(s, 1i32);
                    codegen(s, (*t).car, 1i32);
                    pop_n_(s, 1i32);
                    pop_n_(s, 1i32);
                    if 0 != is_splat {
                        genop_1(s, OP_ARYCAT as libc::c_int as mrb_code,
                                (*s).sp);
                    } else {
                        genop_1(s, OP_ARYPUSH as libc::c_int as mrb_code,
                                (*s).sp);
                    }
                }
                t = (*t).cdr;
                while !t.is_null() {
                    push_n_(s, 1i32);
                    codegen(s, (*t).car, 1i32);
                    pop_n_(s, 1i32);
                    pop_n_(s, 1i32);
                    if (*(*t).car).car as intptr_t as libc::c_int ==
                           NODE_SPLAT as libc::c_int {
                        genop_1(s, OP_ARYCAT as libc::c_int as mrb_code,
                                (*s).sp);
                    } else {
                        genop_1(s, OP_ARYPUSH as libc::c_int as mrb_code,
                                (*s).sp);
                    }
                    t = (*t).cdr
                }
            } else {
                while !t.is_null() {
                    codegen(s, (*t).car, 0i32);
                    t = (*t).cdr
                }
            }
            return -1i32
        }
        /* normal (no splat) mode */
        codegen(s, (*t).car, val);
        n += 1;
        t = (*t).cdr
    }
    return n;
}
unsafe extern "C" fn gen_call(mut s: *mut codegen_scope, mut tree: *mut node,
                              mut name: mrb_sym, mut sp: libc::c_int,
                              mut val: libc::c_int, mut safe: libc::c_int) {
    let mut sym: mrb_sym =
        if 0 != name {
            name
        } else { (*(*tree).cdr).car as intptr_t as mrb_sym };
    let mut skip: libc::c_int = 0i32;
    let mut n: libc::c_int = 0i32;
    let mut noop: libc::c_int = 0i32;
    let mut sendv: libc::c_int = 0i32;
    let mut blk: libc::c_int = 0i32;
    /* receiver */
    codegen(s, (*tree).car, 1i32);
    if 0 != safe {
        let mut recv: libc::c_int = (*s).sp as libc::c_int - 1i32;
        gen_move(s, (*s).sp, recv as uint16_t, 1i32);
        skip =
            genjmp2(s, OP_JMPNIL as libc::c_int as mrb_code, (*s).sp, 0i32,
                    val) as libc::c_int
    }
    tree = (*(*(*tree).cdr).cdr).car;
    if !tree.is_null() {
        n =
            gen_values(s, (*tree).car, 1i32,
                       if 0 != sp { 1i32 } else { 0i32 });
        if n < 0i32 {
            sendv = 1i32;
            noop = sendv;
            n = noop;
            push_n_(s, 1i32);
        }
    }
    if 0 != sp {
        /* last argument pushed (attr=) */
        if 0 != sendv {
            gen_move(s, (*s).sp, sp as uint16_t, 0i32);
            pop_n_(s, 1i32);
            genop_1(s, OP_ARYPUSH as libc::c_int as mrb_code, (*s).sp);
            push_n_(s, 1i32);
        } else {
            gen_move(s, (*s).sp, sp as uint16_t, 0i32);
            push_n_(s, 1i32);
            n += 1
        }
    }
    if !tree.is_null() && !(*tree).cdr.is_null() {
        noop = 1i32;
        codegen(s, (*tree).cdr, 1i32);
        pop_n_(s, 1i32);
        blk = 1i32
    }
    push_n_(s, 1i32);
    pop_n_(s, 1i32);
    pop_n_(s, n + 1i32);
    let mut symlen: mrb_int = 0;
    let mut symname: *const libc::c_char =
        mrb_sym2name_len((*s).mrb, sym, &mut symlen);
    if 0 == noop && symlen == 1i32 as libc::c_longlong &&
           *symname.offset(0isize) as libc::c_int == '+' as i32 && n == 1i32 {
        gen_addsub(s, OP_ADD as libc::c_int as uint8_t, (*s).sp);
    } else if 0 == noop && symlen == 1i32 as libc::c_longlong &&
                  *symname.offset(0isize) as libc::c_int == '-' as i32 &&
                  n == 1i32 {
        gen_addsub(s, OP_SUB as libc::c_int as uint8_t, (*s).sp);
    } else if 0 == noop && symlen == 1i32 as libc::c_longlong &&
                  *symname.offset(0isize) as libc::c_int == '*' as i32 &&
                  n == 1i32 {
        genop_1(s, OP_MUL as libc::c_int as mrb_code, (*s).sp);
    } else if 0 == noop && symlen == 1i32 as libc::c_longlong &&
                  *symname.offset(0isize) as libc::c_int == '/' as i32 &&
                  n == 1i32 {
        genop_1(s, OP_DIV as libc::c_int as mrb_code, (*s).sp);
    } else if 0 == noop && symlen == 1i32 as libc::c_longlong &&
                  *symname.offset(0isize) as libc::c_int == '<' as i32 &&
                  n == 1i32 {
        genop_1(s, OP_LT as libc::c_int as mrb_code, (*s).sp);
    } else if 0 == noop && symlen == 2i32 as libc::c_longlong &&
                  *symname.offset(0isize) as libc::c_int == '<' as i32 &&
                  *symname.offset(1isize) as libc::c_int == '=' as i32 &&
                  n == 1i32 {
        genop_1(s, OP_LE as libc::c_int as mrb_code, (*s).sp);
    } else if 0 == noop && symlen == 1i32 as libc::c_longlong &&
                  *symname.offset(0isize) as libc::c_int == '>' as i32 &&
                  n == 1i32 {
        genop_1(s, OP_GT as libc::c_int as mrb_code, (*s).sp);
    } else if 0 == noop && symlen == 2i32 as libc::c_longlong &&
                  *symname.offset(0isize) as libc::c_int == '>' as i32 &&
                  *symname.offset(1isize) as libc::c_int == '=' as i32 &&
                  n == 1i32 {
        genop_1(s, OP_GE as libc::c_int as mrb_code, (*s).sp);
    } else if 0 == noop && symlen == 2i32 as libc::c_longlong &&
                  *symname.offset(0isize) as libc::c_int == '=' as i32 &&
                  *symname.offset(1isize) as libc::c_int == '=' as i32 &&
                  n == 1i32 {
        genop_1(s, OP_EQ as libc::c_int as mrb_code, (*s).sp);
    } else {
        let mut idx: libc::c_int = new_sym(s, sym);
        if 0 != sendv {
            genop_2(s,
                    (if 0 != blk {
                         OP_SENDVB as libc::c_int
                     } else { OP_SENDV as libc::c_int }) as mrb_code, (*s).sp,
                    idx as uint16_t);
        } else {
            genop_3(s,
                    (if 0 != blk {
                         OP_SENDB as libc::c_int
                     } else { OP_SEND as libc::c_int }) as mrb_code, (*s).sp,
                    idx as uint16_t, n as uint8_t);
        }
    }
    if 0 != safe { dispatch(s, skip as uint16_t); }
    if 0 != val { push_n_(s, 1i32); };
}
unsafe extern "C" fn gen_assignment(mut s: *mut codegen_scope,
                                    mut tree: *mut node, mut sp: libc::c_int,
                                    mut val: libc::c_int) {
    let mut idx: libc::c_int = 0;
    let mut type_0: libc::c_int = (*tree).car as intptr_t as libc::c_int;
    tree = (*tree).cdr;
    match type_0 {
        39 => {
            idx = new_sym(s, tree as intptr_t as mrb_sym);
            genop_2(s, OP_SETGV as libc::c_int as mrb_code, sp as uint16_t,
                    idx as uint16_t);
        }
        58 | 37 => {
            idx = lv_idx(s, tree as intptr_t as mrb_sym);
            if idx > 0i32 {
                if idx != sp {
                    gen_move(s, idx as uint16_t, sp as uint16_t, val);
                    if 0 != val && 0 != on_eval(s) as libc::c_int {
                        genop_0(s, OP_NOP as libc::c_int as mrb_code);
                    }
                }
            } else {
                /* upvar */
                let mut lv: libc::c_int = 0i32;
                let mut up: *mut codegen_scope = (*s).prev;
                while !up.is_null() {
                    idx = lv_idx(up, tree as intptr_t as mrb_sym);
                    if idx > 0i32 {
                        genop_3(s, OP_SETUPVAR as libc::c_int as mrb_code,
                                sp as uint16_t, idx as uint16_t,
                                lv as uint8_t);
                        break ;
                    } else { lv += 1; up = (*up).prev }
                }
            }
        }
        40 => {
            idx = new_sym(s, tree as intptr_t as mrb_sym);
            genop_2(s, OP_SETIV as libc::c_int as mrb_code, sp as uint16_t,
                    idx as uint16_t);
        }
        42 => {
            idx = new_sym(s, tree as intptr_t as mrb_sym);
            genop_2(s, OP_SETCV as libc::c_int as mrb_code, sp as uint16_t,
                    idx as uint16_t);
        }
        41 => {
            idx = new_sym(s, tree as intptr_t as mrb_sym);
            genop_2(s, OP_SETCONST as libc::c_int as mrb_code, sp as uint16_t,
                    idx as uint16_t);
        }
        73 => {
            gen_move(s, (*s).sp, sp as uint16_t, 0i32);
            push_n_(s, 1i32);
            codegen(s, (*tree).car, 1i32);
            pop_n_(s, 2i32);
            idx = new_sym(s, (*tree).cdr as intptr_t as mrb_sym);
            genop_2(s, OP_SETMCNST as libc::c_int as mrb_code, sp as uint16_t,
                    idx as uint16_t);
        }
        26 | 27 => {
            push_n_(s, 1i32);
            gen_call(s, tree,
                     attrsym(s, (*(*tree).cdr).car as intptr_t as mrb_sym),
                     sp, 0i32,
                     (type_0 == NODE_SCALL as libc::c_int) as libc::c_int);
            pop_n_(s, 1i32);
            if 0 != val { gen_move(s, (*s).sp, sp as uint16_t, 0i32); }
        }
        20 => { gen_vmassignment(s, (*tree).car, sp, val); }
        78 | _ => { }
    }
    if 0 != val { push_n_(s, 1i32); };
}
unsafe extern "C" fn gen_vmassignment(mut s: *mut codegen_scope,
                                      mut tree: *mut node,
                                      mut rhs: libc::c_int,
                                      mut val: libc::c_int) {
    let mut n: libc::c_int = 0i32;
    let mut post: libc::c_int = 0i32;
    let mut t: *mut node = 0 as *mut node;
    let mut p: *mut node = 0 as *mut node;
    if !(*tree).car.is_null() {
        /* pre */
        t = (*tree).car;
        n = 0i32;
        while !t.is_null() {
            let mut sp: libc::c_int = (*s).sp as libc::c_int;
            genop_3(s, OP_AREF as libc::c_int as mrb_code, sp as uint16_t,
                    rhs as uint16_t, n as uint8_t);
            push_n_(s, 1i32);
            gen_assignment(s, (*t).car, sp, 0i32);
            pop_n_(s, 1i32);
            n += 1;
            t = (*t).cdr
        }
    }
    t = (*tree).cdr;
    if !t.is_null() {
        if !(*t).cdr.is_null() {
            /* post count */
            p = (*(*t).cdr).car;
            while !p.is_null() { post += 1; p = (*p).cdr }
        }
        gen_move(s, (*s).sp, rhs as uint16_t, val);
        push_n_(s, post + 1i32);
        pop_n_(s, post + 1i32);
        genop_3(s, OP_APOST as libc::c_int as mrb_code, (*s).sp,
                n as uint16_t, post as uint8_t);
        n = 1i32;
        if !(*t).car.is_null() && (*t).car != -1i32 as *mut node {
            /* rest */
            gen_assignment(s, (*t).car, (*s).sp as libc::c_int, 0i32);
        }
        if !(*t).cdr.is_null() && !(*(*t).cdr).car.is_null() {
            t = (*(*t).cdr).car;
            while !t.is_null() {
                gen_assignment(s, (*t).car, (*s).sp as libc::c_int + n, 0i32);
                t = (*t).cdr;
                n += 1
            }
        }
        if 0 != val { gen_move(s, (*s).sp, rhs as uint16_t, 0i32); }
    };
}
unsafe extern "C" fn gen_intern(mut s: *mut codegen_scope) {
    pop_n_(s, 1i32);
    genop_1(s, OP_INTERN as libc::c_int as mrb_code, (*s).sp);
    push_n_(s, 1i32);
}
unsafe extern "C" fn gen_literal_array(mut s: *mut codegen_scope,
                                       mut tree: *mut node, mut sym: mrb_bool,
                                       mut val: libc::c_int) {
    if 0 != val {
        let mut i: libc::c_int = 0i32;
        let mut j: libc::c_int = 0i32;
        while !tree.is_null() {
            let mut current_block_7: u64;
            match (*(*tree).car).car as intptr_t as libc::c_int {
                51 => {
                    if (*tree).cdr.is_null() &&
                           (*(*(*tree).car).cdr).cdr as intptr_t as
                               libc::c_int == 0i32 {
                        current_block_7 = 4166486009154926805;
                    } else {
                        /* fall through */
                        current_block_7 = 15103274462620934986;
                    }
                }
                14 => { current_block_7 = 15103274462620934986; }
                85 => {
                    if j > 0i32 {
                        j = 0i32;
                        i += 1;
                        if 0 != sym { gen_intern(s); }
                    }
                    current_block_7 = 4166486009154926805;
                }
                _ => { current_block_7 = 4166486009154926805; }
            }
            match current_block_7 {
                15103274462620934986 => {
                    codegen(s, (*tree).car, 1i32);
                    j += 1
                }
                _ => { }
            }
            while j >= 2i32 {
                pop_n_(s, 1i32);
                pop_n_(s, 1i32);
                genop_1(s, OP_STRCAT as libc::c_int as mrb_code, (*s).sp);
                push_n_(s, 1i32);
                j -= 1
            }
            tree = (*tree).cdr
        }
        if j > 0i32 { i += 1; if 0 != sym { gen_intern(s); } }
        pop_n_(s, i);
        genop_2(s, OP_ARRAY as libc::c_int as mrb_code, (*s).sp,
                i as uint16_t);
        push_n_(s, 1i32);
    } else {
        while !tree.is_null() {
            match (*(*tree).car).car as intptr_t as libc::c_int {
                14 | 2 => { codegen(s, (*tree).car, 0i32); }
                _ => { }
            }
            tree = (*tree).cdr
        }
    };
}
unsafe extern "C" fn raise_error(mut s: *mut codegen_scope,
                                 mut msg: *const libc::c_char) {
    let mut idx: libc::c_int = new_lit(s, mrb_str_new_cstr((*s).mrb, msg));
    genop_1(s, OP_ERR as libc::c_int as mrb_code, idx as uint16_t);
}
unsafe extern "C" fn readint_float(mut s: *mut codegen_scope,
                                   mut p: *const libc::c_char,
                                   mut base: libc::c_int) -> libc::c_double {
    let mut e: *const libc::c_char = p.offset(strlen(p) as isize);
    let mut f: libc::c_double = 0i32 as libc::c_double;
    let mut n: libc::c_int = 0;
    if *p as libc::c_int == '+' as i32 { p = p.offset(1isize) }
    while p < e {
        let mut c: libc::c_char = *p;
        c = tolower(c as libc::c_uchar as libc::c_int) as libc::c_char;
        n = 0i32;
        while n < base {
            if *mrb_digitmap.as_ptr().offset(n as isize) as libc::c_int ==
                   c as libc::c_int {
                f *= base as libc::c_double;
                f += n as libc::c_double;
                break ;
            } else { n += 1 }
        }
        if n == base {
            codegen_error(s,
                          b"malformed readint input\x00" as *const u8 as
                              *const libc::c_char);
        }
        p = p.offset(1isize)
    }
    return f;
}
unsafe extern "C" fn readint_mrb_int(mut s: *mut codegen_scope,
                                     mut p: *const libc::c_char,
                                     mut base: libc::c_int, mut neg: mrb_bool,
                                     mut overflow: *mut mrb_bool) -> mrb_int {
    let mut e: *const libc::c_char = p.offset(strlen(p) as isize);
    let mut result: mrb_int = 0i32 as mrb_int;
    let mut n: libc::c_int = 0;
    if *p as libc::c_int == '+' as i32 { p = p.offset(1isize) }
    while p < e {
        let mut c: libc::c_char = *p;
        c = tolower(c as libc::c_uchar as libc::c_int) as libc::c_char;
        n = 0i32;
        while n < base {
            if *mrb_digitmap.as_ptr().offset(n as isize) as libc::c_int ==
                   c as libc::c_int {
                break ;
            }
            n += 1
        }
        if n == base {
            codegen_error(s,
                          b"malformed readint input\x00" as *const u8 as
                              *const libc::c_char);
        }
        if 0 != neg {
            if ((-9223372036854775807i64 - 1i32 as libc::c_longlong >> 0i32) +
                    n as libc::c_longlong) / base as libc::c_longlong > result
               {
                *overflow = 1i32 as mrb_bool;
                return 0i32 as mrb_int
            }
            result *= base as libc::c_longlong;
            result -= n as libc::c_longlong
        } else {
            if (((9223372036854775807i64 >> 0i32) - n as libc::c_longlong) /
                    base as libc::c_longlong) < result {
                *overflow = 1i32 as mrb_bool;
                return 0i32 as mrb_int
            }
            result *= base as libc::c_longlong;
            result += n as libc::c_longlong
        }
        p = p.offset(1isize)
    }
    *overflow = 0i32 as mrb_bool;
    return result;
}
unsafe extern "C" fn gen_retval(mut s: *mut codegen_scope,
                                mut tree: *mut node) {
    if (*tree).car as intptr_t as libc::c_int == NODE_SPLAT as libc::c_int {
        codegen(s, tree, 1i32);
        pop_n_(s, 1i32);
        genop_1(s, OP_ARYDUP as libc::c_int as mrb_code, (*s).sp);
    } else { codegen(s, tree, 1i32); pop_n_(s, 1i32); };
}
unsafe extern "C" fn codegen(mut s: *mut codegen_scope, mut tree: *mut node,
                             mut val: libc::c_int) {
    let mut current_block: u64;
    let mut nt: libc::c_int = 0;
    let mut rlev: libc::c_int = (*s).rlev;
    if tree.is_null() {
        if 0 != val {
            genop_1(s, OP_LOADNIL as libc::c_int as mrb_code, (*s).sp);
            push_n_(s, 1i32);
        }
        return
    }
    (*s).rlev += 1;
    if (*s).rlev > 1024i32 {
        codegen_error(s,
                      b"too complex expression\x00" as *const u8 as
                          *const libc::c_char);
    }
    if !(*s).irep.is_null() &&
           (*s).filename_index as libc::c_int !=
               (*tree).filename_index as libc::c_int {
        let mut fname: mrb_sym =
            mrb_parser_get_filename((*s).parser, (*s).filename_index);
        let mut filename: *const libc::c_char =
            mrb_sym2name_len((*s).mrb, fname, 0 as *mut mrb_int);
        mrb_debug_info_append_file((*s).mrb, (*(*s).irep).debug_info,
                                   filename, (*s).lines,
                                   (*s).debug_start_pos as uint32_t,
                                   (*s).pc as uint32_t);
        (*s).debug_start_pos = (*s).pc as libc::c_int;
        (*s).filename_index = (*tree).filename_index;
        (*s).filename_sym =
            mrb_parser_get_filename((*s).parser, (*tree).filename_index)
    }
    nt = (*tree).car as intptr_t as libc::c_int;
    (*s).lineno = (*tree).lineno;
    tree = (*tree).cdr;
    match nt {
        14 => {
            if 0 != val && tree.is_null() {
                genop_1(s, OP_LOADNIL as libc::c_int as mrb_code, (*s).sp);
                push_n_(s, 1i32);
            }
            while !tree.is_null() {
                codegen(s, (*tree).car,
                        if !(*tree).cdr.is_null() { 0i32 } else { val });
                tree = (*tree).cdr
            }
            current_block = 5764103746738363178;
        }
        15 => {
            let mut noexc: libc::c_int = 0;
            let mut exend: libc::c_int = 0;
            let mut pos1: libc::c_int = 0;
            let mut pos2: libc::c_int = 0;
            let mut tmp: libc::c_int = 0;
            let mut lp: *mut loopinfo = 0 as *mut loopinfo;
            if (*tree).car.is_null() {
                current_block = 5764103746738363178;
            } else {
                lp = loop_push(s, LOOP_BEGIN);
                (*lp).pc0 = new_label(s);
                (*lp).pc1 =
                    genjmp(s, OP_ONERR as libc::c_int as mrb_code,
                           0i32 as uint16_t) as libc::c_int;
                codegen(s, (*tree).car, 1i32);
                pop_n_(s, 1i32);
                (*lp).type_0 = LOOP_RESCUE;
                noexc =
                    genjmp(s, OP_JMP as libc::c_int as mrb_code,
                           0i32 as uint16_t) as libc::c_int;
                dispatch(s, (*lp).pc1 as uint16_t);
                tree = (*tree).cdr;
                exend = 0i32;
                pos1 = 0i32;
                if !(*tree).car.is_null() {
                    let mut n2: *mut node = (*tree).car;
                    let mut exc: libc::c_int = (*s).sp as libc::c_int;
                    genop_1(s, OP_EXCEPT as libc::c_int as mrb_code,
                            exc as uint16_t);
                    push_n_(s, 1i32);
                    while !n2.is_null() {
                        let mut n3: *mut node = (*n2).car;
                        let mut n4: *mut node = (*n3).car;
                        if 0 != pos1 { dispatch(s, pos1 as uint16_t); }
                        pos2 = 0i32;
                        loop  {
                            if !n4.is_null() && !(*n4).car.is_null() &&
                                   (*(*n4).car).car as intptr_t as libc::c_int
                                       == NODE_SPLAT as libc::c_int {
                                codegen(s, (*n4).car, 1i32);
                                gen_move(s, (*s).sp, exc as uint16_t, 0i32);
                                push_n_(s, 2i32);
                                /* space for one arg and a block */
                                pop_n_(s, 2i32);
                                pop_n_(s, 1i32);
                                genop_3(s, OP_SEND as libc::c_int as mrb_code,
                                        (*s).sp,
                                        new_sym(s,
                                                mrb_intern_static((*s).mrb,
                                                                  b"__case_eqq\x00"
                                                                      as
                                                                      *const u8
                                                                      as
                                                                      *const libc::c_char,
                                                                  (::std::mem::size_of::<[libc::c_char; 11]>()
                                                                       as
                                                                       libc::c_ulong).wrapping_sub(1i32
                                                                                                       as
                                                                                                       libc::c_ulong)))
                                            as uint16_t, 1i32 as uint8_t);
                            } else {
                                if !n4.is_null() {
                                    codegen(s, (*n4).car, 1i32);
                                } else {
                                    genop_2(s,
                                            OP_GETCONST as libc::c_int as
                                                mrb_code, (*s).sp,
                                            new_sym(s,
                                                    mrb_intern_static((*s).mrb,
                                                                      b"StandardError\x00"
                                                                          as
                                                                          *const u8
                                                                          as
                                                                          *const libc::c_char,
                                                                      (::std::mem::size_of::<[libc::c_char; 14]>()
                                                                           as
                                                                           libc::c_ulong).wrapping_sub(1i32
                                                                                                           as
                                                                                                           libc::c_ulong)))
                                                as uint16_t);
                                    push_n_(s, 1i32);
                                }
                                pop_n_(s, 1i32);
                                genop_2(s,
                                        OP_RESCUE as libc::c_int as mrb_code,
                                        exc as uint16_t, (*s).sp);
                            }
                            tmp =
                                genjmp2(s,
                                        OP_JMPIF as libc::c_int as mrb_code,
                                        (*s).sp, pos2, val) as libc::c_int;
                            pos2 = tmp;
                            if !n4.is_null() { n4 = (*n4).cdr }
                            if n4.is_null() { break ; }
                        }
                        pos1 =
                            genjmp(s, OP_JMP as libc::c_int as mrb_code,
                                   0i32 as uint16_t) as libc::c_int;
                        dispatch_linked(s, pos2 as uint16_t);
                        pop_n_(s, 1i32);
                        if !(*(*n3).cdr).car.is_null() {
                            gen_assignment(s, (*(*n3).cdr).car, exc, 0i32);
                        }
                        if !(*(*(*n3).cdr).cdr).car.is_null() {
                            codegen(s, (*(*(*n3).cdr).cdr).car, val);
                            if 0 != val { pop_n_(s, 1i32); }
                        }
                        tmp =
                            genjmp(s, OP_JMP as libc::c_int as mrb_code,
                                   exend as uint16_t) as libc::c_int;
                        exend = tmp;
                        n2 = (*n2).cdr;
                        push_n_(s, 1i32);
                    }
                    if 0 != pos1 {
                        dispatch(s, pos1 as uint16_t);
                        genop_1(s, OP_RAISE as libc::c_int as mrb_code,
                                exc as uint16_t);
                    }
                }
                pop_n_(s, 1i32);
                tree = (*tree).cdr;
                dispatch(s, noexc as uint16_t);
                genop_1(s, OP_POPERR as libc::c_int as mrb_code,
                        1i32 as uint16_t);
                if !(*tree).car.is_null() {
                    codegen(s, (*tree).car, val);
                } else if 0 != val { push_n_(s, 1i32); }
                dispatch_linked(s, exend as uint16_t);
                loop_pop(s, 0i32);
                current_block = 5764103746738363178;
            }
        }
        16 => {
            if (*tree).cdr.is_null() || (*(*tree).cdr).cdr.is_null() ||
                   (*(*(*tree).cdr).cdr).car as intptr_t as libc::c_int ==
                       NODE_BEGIN as libc::c_int &&
                       !(*(*(*tree).cdr).cdr).cdr.is_null() {
                let mut idx: libc::c_int = 0;
                (*s).ensure_level += 1;
                idx = scope_body(s, (*tree).cdr, 0i32);
                genop_1(s, OP_EPUSH as libc::c_int as mrb_code,
                        idx as uint16_t);
                codegen(s, (*tree).car, val);
                (*s).ensure_level -= 1;
                genop_1(s, OP_EPOP as libc::c_int as mrb_code,
                        1i32 as uint16_t);
            } else {
                /* empty ensure ignored */
                codegen(s, (*tree).car, val);
            }
            current_block = 5764103746738363178;
        }
        49 => {
            if 0 != val {
                let mut idx_0: libc::c_int = lambda_body(s, tree, 1i32);
                genop_2(s, OP_LAMBDA as libc::c_int as mrb_code, (*s).sp,
                        idx_0 as uint16_t);
                push_n_(s, 1i32);
            }
            current_block = 5764103746738363178;
        }
        2 => {
            if 0 != val {
                let mut idx_1: libc::c_int = lambda_body(s, tree, 1i32);
                genop_2(s, OP_BLOCK as libc::c_int as mrb_code, (*s).sp,
                        idx_1 as uint16_t);
                push_n_(s, 1i32);
            }
            current_block = 5764103746738363178;
        }
        3 => {
            let mut pos1_0: libc::c_int = 0;
            let mut pos2_0: libc::c_int = 0;
            let mut elsepart: *mut node = (*(*(*tree).cdr).cdr).car;
            if (*tree).car.is_null() {
                codegen(s, elsepart, val);
            } else {
                match (*(*tree).car).car as intptr_t as libc::c_int {
                    79 | 46 | 51 => { codegen(s, (*(*tree).cdr).car, val); }
                    80 | 78 => { codegen(s, elsepart, val); }
                    _ => {
                        codegen(s, (*tree).car, 1i32);
                        pop_n_(s, 1i32);
                        pos1_0 =
                            genjmp2(s, OP_JMPNOT as libc::c_int as mrb_code,
                                    (*s).sp, 0i32, val) as libc::c_int;
                        codegen(s, (*(*tree).cdr).car, val);
                        if !elsepart.is_null() {
                            if 0 != val { pop_n_(s, 1i32); }
                            pos2_0 =
                                genjmp(s, OP_JMP as libc::c_int as mrb_code,
                                       0i32 as uint16_t) as libc::c_int;
                            dispatch(s, pos1_0 as uint16_t);
                            codegen(s, elsepart, val);
                            dispatch(s, pos2_0 as uint16_t);
                        } else if 0 != val {
                            pop_n_(s, 1i32);
                            pos2_0 =
                                genjmp(s, OP_JMP as libc::c_int as mrb_code,
                                       0i32 as uint16_t) as libc::c_int;
                            dispatch(s, pos1_0 as uint16_t);
                            genop_1(s, OP_LOADNIL as libc::c_int as mrb_code,
                                    (*s).sp);
                            dispatch(s, pos2_0 as uint16_t);
                            push_n_(s, 1i32);
                        } else { dispatch(s, pos1_0 as uint16_t); }
                    }
                }
            }
            current_block = 5764103746738363178;
        }
        17 => {
            let mut pos: libc::c_int = 0;
            codegen(s, (*tree).car, 1i32);
            pop_n_(s, 1i32);
            pos =
                genjmp2(s, OP_JMPNOT as libc::c_int as mrb_code, (*s).sp,
                        0i32, val) as libc::c_int;
            codegen(s, (*tree).cdr, val);
            dispatch(s, pos as uint16_t);
            current_block = 5764103746738363178;
        }
        18 => {
            let mut pos_0: libc::c_int = 0;
            codegen(s, (*tree).car, 1i32);
            pop_n_(s, 1i32);
            pos_0 =
                genjmp2(s, OP_JMPIF as libc::c_int as mrb_code, (*s).sp, 0i32,
                        val) as libc::c_int;
            codegen(s, (*tree).cdr, val);
            dispatch(s, pos_0 as uint16_t);
            current_block = 5764103746738363178;
        }
        6 => {
            let mut lp_0: *mut loopinfo = loop_push(s, LOOP_NORMAL);
            (*lp_0).pc0 = new_label(s);
            (*lp_0).pc1 =
                genjmp(s, OP_JMP as libc::c_int as mrb_code, 0i32 as uint16_t)
                    as libc::c_int;
            (*lp_0).pc2 = new_label(s);
            codegen(s, (*tree).cdr, 0i32);
            dispatch(s, (*lp_0).pc1 as uint16_t);
            codegen(s, (*tree).car, 1i32);
            pop_n_(s, 1i32);
            genjmp2(s, OP_JMPIF as libc::c_int as mrb_code, (*s).sp,
                    (*lp_0).pc2, 0i32);
            loop_pop(s, val);
            current_block = 5764103746738363178;
        }
        7 => {
            let mut lp_1: *mut loopinfo = loop_push(s, LOOP_NORMAL);
            (*lp_1).pc0 = new_label(s);
            (*lp_1).pc1 =
                genjmp(s, OP_JMP as libc::c_int as mrb_code, 0i32 as uint16_t)
                    as libc::c_int;
            (*lp_1).pc2 = new_label(s);
            codegen(s, (*tree).cdr, 0i32);
            dispatch(s, (*lp_1).pc1 as uint16_t);
            codegen(s, (*tree).car, 1i32);
            pop_n_(s, 1i32);
            genjmp2(s, OP_JMPNOT as libc::c_int as mrb_code, (*s).sp,
                    (*lp_1).pc2, 0i32);
            loop_pop(s, val);
            current_block = 5764103746738363178;
        }
        9 => {
            for_body(s, tree);
            if 0 != val { push_n_(s, 1i32); }
            current_block = 5764103746738363178;
        }
        4 => {
            let mut head: libc::c_int = 0i32;
            let mut pos1_1: libc::c_int = 0;
            let mut pos2_1: libc::c_int = 0;
            let mut pos3: libc::c_int = 0;
            let mut tmp_0: libc::c_int = 0;
            let mut n: *mut node = 0 as *mut node;
            pos3 = 0i32;
            if !(*tree).car.is_null() {
                head = (*s).sp as libc::c_int;
                codegen(s, (*tree).car, 1i32);
            }
            tree = (*tree).cdr;
            while !tree.is_null() {
                n = (*(*tree).car).car;
                pos2_1 = 0i32;
                pos1_1 = pos2_1;
                while !n.is_null() {
                    codegen(s, (*n).car, 1i32);
                    if 0 != head {
                        gen_move(s, (*s).sp, head as uint16_t, 0i32);
                        push_n_(s, 1i32);
                        push_n_(s, 1i32);
                        pop_n_(s, 1i32);
                        pop_n_(s, 1i32);
                        pop_n_(s, 1i32);
                        if (*(*n).car).car as intptr_t as libc::c_int ==
                               NODE_SPLAT as libc::c_int {
                            genop_3(s, OP_SEND as libc::c_int as mrb_code,
                                    (*s).sp,
                                    new_sym(s,
                                            mrb_intern_static((*s).mrb,
                                                              b"__case_eqq\x00"
                                                                  as *const u8
                                                                  as
                                                                  *const libc::c_char,
                                                              (::std::mem::size_of::<[libc::c_char; 11]>()
                                                                   as
                                                                   libc::c_ulong).wrapping_sub(1i32
                                                                                                   as
                                                                                                   libc::c_ulong)))
                                        as uint16_t, 1i32 as uint8_t);
                        } else {
                            genop_3(s, OP_SEND as libc::c_int as mrb_code,
                                    (*s).sp,
                                    new_sym(s,
                                            mrb_intern_static((*s).mrb,
                                                              b"===\x00" as
                                                                  *const u8 as
                                                                  *const libc::c_char,
                                                              (::std::mem::size_of::<[libc::c_char; 4]>()
                                                                   as
                                                                   libc::c_ulong).wrapping_sub(1i32
                                                                                                   as
                                                                                                   libc::c_ulong)))
                                        as uint16_t, 1i32 as uint8_t);
                        }
                    } else { pop_n_(s, 1i32); }
                    tmp_0 =
                        genjmp2(s, OP_JMPIF as libc::c_int as mrb_code,
                                (*s).sp, pos2_1, 0i32) as libc::c_int;
                    pos2_1 = tmp_0;
                    n = (*n).cdr
                }
                if !(*(*tree).car).car.is_null() {
                    pos1_1 =
                        genjmp(s, OP_JMP as libc::c_int as mrb_code,
                               0i32 as uint16_t) as libc::c_int;
                    dispatch_linked(s, pos2_1 as uint16_t);
                }
                codegen(s, (*(*tree).car).cdr, val);
                if 0 != val { pop_n_(s, 1i32); }
                tmp_0 =
                    genjmp(s, OP_JMP as libc::c_int as mrb_code,
                           pos3 as uint16_t) as libc::c_int;
                pos3 = tmp_0;
                if 0 != pos1_1 { dispatch(s, pos1_1 as uint16_t); }
                tree = (*tree).cdr
            }
            if 0 != val {
                let mut pos_1: libc::c_int = (*s).sp as libc::c_int;
                genop_1(s, OP_LOADNIL as libc::c_int as mrb_code, (*s).sp);
                if 0 != pos3 { dispatch_linked(s, pos3 as uint16_t); }
                if 0 != head { pop_n_(s, 1i32); }
                if (*s).sp as libc::c_int != pos_1 {
                    gen_move(s, (*s).sp, pos_1 as uint16_t, 0i32);
                }
                push_n_(s, 1i32);
            } else {
                if 0 != pos3 { dispatch_linked(s, pos3 as uint16_t); }
                if 0 != head { pop_n_(s, 1i32); }
            }
            current_block = 5764103746738363178;
        }
        1 => {
            scope_body(s, tree, 0i32);
            current_block = 5764103746738363178;
        }
        28 | 26 => {
            gen_call(s, tree, 0i32 as mrb_sym, 0i32, val, 0i32);
            current_block = 5764103746738363178;
        }
        27 => {
            gen_call(s, tree, 0i32 as mrb_sym, 0i32, val, 1i32);
            current_block = 5764103746738363178;
        }
        75 => {
            codegen(s, (*tree).car, val);
            codegen(s, (*tree).cdr, val);
            if 0 != val {
                pop_n_(s, 1i32);
                pop_n_(s, 1i32);
                genop_1(s, OP_RANGE_INC as libc::c_int as mrb_code, (*s).sp);
                push_n_(s, 1i32);
            }
            current_block = 5764103746738363178;
        }
        76 => {
            codegen(s, (*tree).car, val);
            codegen(s, (*tree).cdr, val);
            if 0 != val {
                pop_n_(s, 1i32);
                pop_n_(s, 1i32);
                genop_1(s, OP_RANGE_EXC as libc::c_int as mrb_code, (*s).sp);
                push_n_(s, 1i32);
            }
            current_block = 5764103746738363178;
        }
        73 => {
            let mut sym: libc::c_int =
                new_sym(s, (*tree).cdr as intptr_t as mrb_sym);
            codegen(s, (*tree).car, 1i32);
            pop_n_(s, 1i32);
            genop_2(s, OP_GETMCNST as libc::c_int as mrb_code, (*s).sp,
                    sym as uint16_t);
            if 0 != val { push_n_(s, 1i32); }
            current_block = 5764103746738363178;
        }
        74 => {
            let mut sym_0: libc::c_int =
                new_sym(s, tree as intptr_t as mrb_sym);
            genop_1(s, OP_OCLASS as libc::c_int as mrb_code, (*s).sp);
            genop_2(s, OP_GETMCNST as libc::c_int as mrb_code, (*s).sp,
                    sym_0 as uint16_t);
            if 0 != val { push_n_(s, 1i32); }
            current_block = 5764103746738363178;
        }
        31 => {
            let mut n_0: libc::c_int = 0;
            n_0 = gen_values(s, tree, val, 0i32);
            if n_0 >= 0i32 {
                if 0 != val {
                    pop_n_(s, n_0);
                    genop_2(s, OP_ARRAY as libc::c_int as mrb_code, (*s).sp,
                            n_0 as uint16_t);
                    push_n_(s, 1i32);
                }
            } else if 0 != val { push_n_(s, 1i32); }
            current_block = 5764103746738363178;
        }
        33 | 34 => {
            let mut len: libc::c_int = 0i32;
            let mut update: mrb_bool = 0i32 as mrb_bool;
            while !tree.is_null() {
                if (*(*(*tree).car).car).car as intptr_t as libc::c_int ==
                       NODE_KW_REST_ARGS as libc::c_int {
                    if len > 0i32 {
                        pop_n_(s, len * 2i32);
                        if 0 == update {
                            genop_2(s, OP_HASH as libc::c_int as mrb_code,
                                    (*s).sp, len as uint16_t);
                        } else {
                            pop_n_(s, 1i32);
                            genop_2(s, OP_HASHADD as libc::c_int as mrb_code,
                                    (*s).sp, len as uint16_t);
                        }
                        push_n_(s, 1i32);
                    }
                    codegen(s, (*(*tree).car).cdr, 1i32);
                    if len > 0i32 || 0 != update as libc::c_int {
                        pop_n_(s, 1i32);
                        pop_n_(s, 1i32);
                        genop_1(s, OP_HASHCAT as libc::c_int as mrb_code,
                                (*s).sp);
                        push_n_(s, 1i32);
                    }
                    update = 1i32 as mrb_bool;
                    len = 0i32
                } else {
                    codegen(s, (*(*tree).car).car, val);
                    codegen(s, (*(*tree).car).cdr, val);
                    len += 1
                }
                tree = (*tree).cdr;
                if 0 != val && len == 255i32 {
                    pop_n_(s, len * 2i32);
                    if 0 == update {
                        genop_2(s, OP_HASH as libc::c_int as mrb_code,
                                (*s).sp, len as uint16_t);
                    } else {
                        pop_n_(s, 1i32);
                        genop_2(s, OP_HASHADD as libc::c_int as mrb_code,
                                (*s).sp, len as uint16_t);
                    }
                    push_n_(s, 1i32);
                    update = 1i32 as mrb_bool;
                    len = 0i32
                }
            }
            if 0 != val {
                pop_n_(s, len * 2i32);
                if 0 == update {
                    genop_2(s, OP_HASH as libc::c_int as mrb_code, (*s).sp,
                            len as uint16_t);
                } else {
                    pop_n_(s, 1i32);
                    if len > 0i32 {
                        genop_2(s, OP_HASHADD as libc::c_int as mrb_code,
                                (*s).sp, len as uint16_t);
                    }
                }
                push_n_(s, 1i32);
            }
            current_block = 5764103746738363178;
        }
        62 => { codegen(s, tree, val); current_block = 5764103746738363178; }
        21 => {
            codegen(s, (*tree).cdr, 1i32);
            pop_n_(s, 1i32);
            gen_assignment(s, (*tree).car, (*s).sp as libc::c_int, val);
            current_block = 5764103746738363178;
        }
        20 => {
            let mut len_0: libc::c_int = 0i32;
            let mut n_1: libc::c_int = 0i32;
            let mut post: libc::c_int = 0i32;
            let mut t: *mut node = (*tree).cdr;
            let mut p: *mut node = 0 as *mut node;
            let mut rhs: libc::c_int = (*s).sp as libc::c_int;
            if (*t).car as intptr_t as libc::c_int ==
                   NODE_ARRAY as libc::c_int && !(*t).cdr.is_null() &&
                   0 != nosplat((*t).cdr) as libc::c_int {
                /* fixed rhs */
                t = (*t).cdr;
                while !t.is_null() {
                    codegen(s, (*t).car, 1i32);
                    len_0 += 1;
                    t = (*t).cdr
                }
                tree = (*tree).car;
                if !(*tree).car.is_null() {
                    /* pre */
                    t = (*tree).car;
                    n_1 = 0i32;
                    while !t.is_null() {
                        if n_1 < len_0 {
                            gen_assignment(s, (*t).car, rhs + n_1, 0i32);
                            n_1 += 1
                        } else {
                            genop_1(s, OP_LOADNIL as libc::c_int as mrb_code,
                                    (rhs + n_1) as uint16_t);
                            gen_assignment(s, (*t).car, rhs + n_1, 0i32);
                        }
                        t = (*t).cdr
                    }
                }
                t = (*tree).cdr;
                if !t.is_null() {
                    if !(*t).cdr.is_null() {
                        /* post count */
                        p = (*(*t).cdr).car;
                        while !p.is_null() { post += 1; p = (*p).cdr }
                    }
                    if !(*t).car.is_null() {
                        /* rest (len - pre - post) */
                        let mut rn: libc::c_int = 0;
                        if len_0 < post + n_1 {
                            rn = 0i32
                        } else { rn = len_0 - post - n_1 }
                        genop_3(s, OP_ARRAY2 as libc::c_int as mrb_code,
                                (*s).sp, (rhs + n_1) as uint16_t,
                                rn as uint8_t);
                        gen_assignment(s, (*t).car, (*s).sp as libc::c_int,
                                       0i32);
                        n_1 += rn
                    }
                    if !(*t).cdr.is_null() && !(*(*t).cdr).car.is_null() {
                        t = (*(*t).cdr).car;
                        while n_1 < len_0 {
                            gen_assignment(s, (*t).car, rhs + n_1, 0i32);
                            t = (*t).cdr;
                            n_1 += 1
                        }
                    }
                }
                pop_n_(s, len_0);
                if 0 != val {
                    genop_2(s, OP_ARRAY as libc::c_int as mrb_code,
                            rhs as uint16_t, len_0 as uint16_t);
                    push_n_(s, 1i32);
                }
            } else {
                /* variable rhs */
                codegen(s, t, 1i32);
                gen_vmassignment(s, (*tree).car, rhs, val);
                if 0 == val { pop_n_(s, 1i32); }
            }
            current_block = 5764103746738363178;
        }
        25 => {
            let mut sym_1: mrb_sym =
                (*(*tree).cdr).car as intptr_t as mrb_sym;
            let mut len_1: mrb_int = 0;
            let mut name: *const libc::c_char =
                mrb_sym2name_len((*s).mrb, sym_1, &mut len_1);
            let mut idx_2: libc::c_int = 0;
            let mut callargs: libc::c_int = -1i32;
            let mut vsp: libc::c_int = -1i32;
            if len_1 == 2i32 as libc::c_longlong &&
                   *name.offset(0isize) as libc::c_int == '|' as i32 &&
                   *name.offset(1isize) as libc::c_int == '|' as i32 &&
                   ((*(*tree).car).car as intptr_t as libc::c_int ==
                        NODE_CONST as libc::c_int ||
                        (*(*tree).car).car as intptr_t as libc::c_int ==
                            NODE_CVAR as libc::c_int) {
                let mut onerr: libc::c_int = 0;
                let mut noexc_0: libc::c_int = 0;
                let mut exc_0: libc::c_int = 0;
                let mut lp_2: *mut loopinfo = 0 as *mut loopinfo;
                onerr =
                    genjmp(s, OP_ONERR as libc::c_int as mrb_code,
                           0i32 as uint16_t) as libc::c_int;
                lp_2 = loop_push(s, LOOP_BEGIN);
                (*lp_2).pc1 = onerr;
                exc_0 = (*s).sp as libc::c_int;
                codegen(s, (*tree).car, 1i32);
                (*lp_2).type_0 = LOOP_RESCUE;
                genop_1(s, OP_POPERR as libc::c_int as mrb_code,
                        1i32 as uint16_t);
                noexc_0 =
                    genjmp(s, OP_JMP as libc::c_int as mrb_code,
                           0i32 as uint16_t) as libc::c_int;
                dispatch(s, onerr as uint16_t);
                genop_1(s, OP_EXCEPT as libc::c_int as mrb_code,
                        exc_0 as uint16_t);
                genop_1(s, OP_LOADF as libc::c_int as mrb_code,
                        exc_0 as uint16_t);
                dispatch(s, noexc_0 as uint16_t);
                loop_pop(s, 0i32);
            } else if (*(*tree).car).car as intptr_t as libc::c_int ==
                          NODE_CALL as libc::c_int {
                let mut n_2: *mut node = (*(*tree).car).cdr;
                let mut base: libc::c_int = 0;
                let mut i: libc::c_int = 0;
                let mut nargs: libc::c_int = 0i32;
                callargs = 0i32;
                if 0 != val {
                    vsp = (*s).sp as libc::c_int;
                    push_n_(s, 1i32);
                }
                /* receiver */
                codegen(s, (*n_2).car, 1i32);
                idx_2 = new_sym(s, (*(*n_2).cdr).car as intptr_t as mrb_sym);
                base = (*s).sp as libc::c_int - 1i32;
                if !(*(*(*n_2).cdr).cdr).car.is_null() {
                    nargs =
                        gen_values(s, (*(*(*(*n_2).cdr).cdr).car).car, 1i32,
                                   1i32);
                    if nargs >= 0i32 {
                        callargs = nargs
                    } else {
                        /* varargs */
                        push_n_(s, 1i32);
                        nargs = 1i32;
                        callargs = 127i32
                    }
                }
                /* copy receiver and arguments */
                gen_move(s, (*s).sp, base as uint16_t, 1i32);
                i = 0i32;
                while i < nargs {
                    gen_move(s,
                             ((*s).sp as libc::c_int + i + 1i32) as uint16_t,
                             (base + i + 1i32) as uint16_t, 1i32);
                    i += 1
                }
                push_n_(s, nargs + 2i32);
                /* space for receiver, arguments and a block */
                pop_n_(s, nargs + 2i32);
                genop_3(s, OP_SEND as libc::c_int as mrb_code, (*s).sp,
                        idx_2 as uint16_t, callargs as uint8_t);
                push_n_(s, 1i32);
            } else { codegen(s, (*tree).car, 1i32); }
            if len_1 == 2i32 as libc::c_longlong &&
                   (*name.offset(0isize) as libc::c_int == '|' as i32 &&
                        *name.offset(1isize) as libc::c_int == '|' as i32 ||
                        *name.offset(0isize) as libc::c_int == '&' as i32 &&
                            *name.offset(1isize) as libc::c_int == '&' as i32)
               {
                let mut pos_2: libc::c_int = 0;
                pop_n_(s, 1i32);
                if 0 != val {
                    if vsp >= 0i32 {
                        gen_move(s, vsp as uint16_t, (*s).sp, 1i32);
                    }
                    pos_2 =
                        genjmp2(s,
                                (if *name.offset(0isize) as libc::c_int ==
                                        '|' as i32 {
                                     OP_JMPIF as libc::c_int
                                 } else { OP_JMPNOT as libc::c_int }) as
                                    mrb_code, (*s).sp, 0i32, val) as
                            libc::c_int
                } else {
                    pos_2 =
                        genjmp2(s,
                                (if *name.offset(0isize) as libc::c_int ==
                                        '|' as i32 {
                                     OP_JMPIF as libc::c_int
                                 } else { OP_JMPNOT as libc::c_int }) as
                                    mrb_code, (*s).sp, 0i32, val) as
                            libc::c_int
                }
                codegen(s, (*(*(*tree).cdr).cdr).car, 1i32);
                pop_n_(s, 1i32);
                if 0 != val && vsp >= 0i32 {
                    gen_move(s, vsp as uint16_t, (*s).sp, 1i32);
                }
                if (*(*tree).car).car as intptr_t as libc::c_int ==
                       NODE_CALL as libc::c_int {
                    if callargs == 127i32 {
                        pop_n_(s, 1i32);
                        genop_1(s, OP_ARYPUSH as libc::c_int as mrb_code,
                                (*s).sp);
                    } else { pop_n_(s, callargs); callargs += 1 }
                    pop_n_(s, 1i32);
                    idx_2 =
                        new_sym(s,
                                attrsym(s,
                                        (*(*(*(*tree).car).cdr).cdr).car as
                                            intptr_t as mrb_sym));
                    genop_3(s, OP_SEND as libc::c_int as mrb_code, (*s).sp,
                            idx_2 as uint16_t, callargs as uint8_t);
                } else {
                    gen_assignment(s, (*tree).car, (*s).sp as libc::c_int,
                                   val);
                }
                dispatch(s, pos_2 as uint16_t);
            } else {
                codegen(s, (*(*(*tree).cdr).cdr).car, 1i32);
                push_n_(s, 1i32);
                pop_n_(s, 1i32);
                pop_n_(s, 1i32);
                pop_n_(s, 1i32);
                if len_1 == 1i32 as libc::c_longlong &&
                       *name.offset(0isize) as libc::c_int == '+' as i32 {
                    gen_addsub(s, OP_ADD as libc::c_int as uint8_t, (*s).sp);
                } else if len_1 == 1i32 as libc::c_longlong &&
                              *name.offset(0isize) as libc::c_int ==
                                  '-' as i32 {
                    gen_addsub(s, OP_SUB as libc::c_int as uint8_t, (*s).sp);
                } else if len_1 == 1i32 as libc::c_longlong &&
                              *name.offset(0isize) as libc::c_int ==
                                  '*' as i32 {
                    genop_1(s, OP_MUL as libc::c_int as mrb_code, (*s).sp);
                } else if len_1 == 1i32 as libc::c_longlong &&
                              *name.offset(0isize) as libc::c_int ==
                                  '/' as i32 {
                    genop_1(s, OP_DIV as libc::c_int as mrb_code, (*s).sp);
                } else if len_1 == 1i32 as libc::c_longlong &&
                              *name.offset(0isize) as libc::c_int ==
                                  '<' as i32 {
                    genop_1(s, OP_LT as libc::c_int as mrb_code, (*s).sp);
                } else if len_1 == 2i32 as libc::c_longlong &&
                              *name.offset(0isize) as libc::c_int ==
                                  '<' as i32 &&
                              *name.offset(1isize) as libc::c_int ==
                                  '=' as i32 {
                    genop_1(s, OP_LE as libc::c_int as mrb_code, (*s).sp);
                } else if len_1 == 1i32 as libc::c_longlong &&
                              *name.offset(0isize) as libc::c_int ==
                                  '>' as i32 {
                    genop_1(s, OP_GT as libc::c_int as mrb_code, (*s).sp);
                } else if len_1 == 2i32 as libc::c_longlong &&
                              *name.offset(0isize) as libc::c_int ==
                                  '>' as i32 &&
                              *name.offset(1isize) as libc::c_int ==
                                  '=' as i32 {
                    genop_1(s, OP_GE as libc::c_int as mrb_code, (*s).sp);
                } else {
                    idx_2 = new_sym(s, sym_1);
                    genop_3(s, OP_SEND as libc::c_int as mrb_code, (*s).sp,
                            idx_2 as uint16_t, 1i32 as uint8_t);
                }
                if callargs < 0i32 {
                    gen_assignment(s, (*tree).car, (*s).sp as libc::c_int,
                                   val);
                } else {
                    if 0 != val && vsp >= 0i32 {
                        gen_move(s, vsp as uint16_t, (*s).sp, 0i32);
                    }
                    if callargs == 127i32 {
                        pop_n_(s, 1i32);
                        genop_1(s, OP_ARYPUSH as libc::c_int as mrb_code,
                                (*s).sp);
                    } else { pop_n_(s, callargs); callargs += 1 }
                    pop_n_(s, 1i32);
                    idx_2 =
                        new_sym(s,
                                attrsym(s,
                                        (*(*(*(*tree).car).cdr).cdr).car as
                                            intptr_t as mrb_sym));
                    genop_3(s, OP_SEND as libc::c_int as mrb_code, (*s).sp,
                            idx_2 as uint16_t, callargs as uint8_t);
                }
            }
            current_block = 5764103746738363178;
        }
        29 => {
            let mut s2: *mut codegen_scope = s;
            let mut lv: libc::c_int = 0i32;
            let mut n_3: libc::c_int = 0i32;
            let mut noop: libc::c_int = 0i32;
            let mut sendv: libc::c_int = 0i32;
            /* room for receiver */
            push_n_(s, 1i32);
            while 0 == (*s2).mscope() {
                lv += 1;
                s2 = (*s2).prev;
                if s2.is_null() { break ; }
            }
            genop_2S(s, OP_ARGARY as libc::c_int as mrb_code, (*s).sp,
                     (lv & 0xfi32) as uint16_t);
            push_n_(s, 1i32);
            /* ARGARY pushes two values */
            push_n_(s, 1i32);
            pop_n_(s, 1i32);
            pop_n_(s, 1i32);
            if !tree.is_null() {
                let mut args: *mut node = (*tree).car;
                if !args.is_null() {
                    n_3 = gen_values(s, args, 1i32, 0i32);
                    if n_3 < 0i32 {
                        sendv = 1i32;
                        noop = sendv;
                        n_3 = noop;
                        push_n_(s, 1i32);
                    }
                }
            }
            if !tree.is_null() && !(*tree).cdr.is_null() {
                codegen(s, (*tree).cdr, 1i32);
                pop_n_(s, 1i32);
            } else {
                genop_1(s, OP_LOADNIL as libc::c_int as mrb_code, (*s).sp);
                push_n_(s, 1i32);
                pop_n_(s, 1i32);
            }
            pop_n_(s, n_3 + 1i32);
            if 0 != sendv { n_3 = 127i32 }
            genop_2(s, OP_SUPER as libc::c_int as mrb_code, (*s).sp,
                    n_3 as uint16_t);
            if 0 != val { push_n_(s, 1i32); }
            current_block = 5764103746738363178;
        }
        30 => {
            let mut s2_0: *mut codegen_scope = s;
            let mut lv_0: libc::c_int = 0i32;
            let mut ainfo: libc::c_int = 0i32;
            /* room for receiver */
            push_n_(s, 1i32);
            while 0 == (*s2_0).mscope() {
                lv_0 += 1;
                s2_0 = (*s2_0).prev;
                if s2_0.is_null() { break ; }
            }
            if !s2_0.is_null() { ainfo = (*s2_0).ainfo() }
            genop_2S(s, OP_ARGARY as libc::c_int as mrb_code, (*s).sp,
                     (ainfo << 4i32 | lv_0 & 0xfi32) as uint16_t);
            push_n_(s, 1i32);
            push_n_(s, 1i32);
            /* ARGARY pushes two values */
            pop_n_(s, 1i32);
            if !tree.is_null() && !(*tree).cdr.is_null() {
                codegen(s, (*tree).cdr, 1i32);
                pop_n_(s, 1i32);
            }
            pop_n_(s, 1i32);
            pop_n_(s, 1i32);
            genop_2(s, OP_SUPER as libc::c_int as mrb_code, (*s).sp,
                    127i32 as uint16_t);
            if 0 != val { push_n_(s, 1i32); }
            current_block = 5764103746738363178;
        }
        35 => {
            if !tree.is_null() {
                gen_retval(s, tree);
            } else {
                genop_1(s, OP_LOADNIL as libc::c_int as mrb_code, (*s).sp);
            }
            if !(*s).loop_0.is_null() {
                gen_return(s, OP_RETURN_BLK as libc::c_int as uint8_t,
                           (*s).sp);
            } else {
                gen_return(s, OP_RETURN as libc::c_int as uint8_t, (*s).sp);
            }
            if 0 != val { push_n_(s, 1i32); }
            current_block = 5764103746738363178;
        }
        36 => {
            let mut s2_1: *mut codegen_scope = s;
            let mut lv_1: libc::c_int = 0i32;
            let mut ainfo_0: libc::c_int = 0i32;
            let mut n_4: libc::c_int = 0i32;
            let mut sendv_0: libc::c_int = 0i32;
            while 0 == (*s2_1).mscope() {
                lv_1 += 1;
                s2_1 = (*s2_1).prev;
                if s2_1.is_null() { break ; }
            }
            if !s2_1.is_null() { ainfo_0 = (*s2_1).ainfo() }
            push_n_(s, 1i32);
            if !tree.is_null() {
                n_4 = gen_values(s, tree, 1i32, 0i32);
                if n_4 < 0i32 {
                    sendv_0 = 1i32;
                    n_4 = sendv_0;
                    push_n_(s, 1i32);
                }
            }
            push_n_(s, 1i32);
            /* space for a block */
            pop_n_(s, 1i32);
            pop_n_(s, n_4 + 1i32);
            genop_2S(s, OP_BLKPUSH as libc::c_int as mrb_code, (*s).sp,
                     (ainfo_0 << 4i32 | lv_1 & 0xfi32) as uint16_t);
            if 0 != sendv_0 { n_4 = 127i32 }
            genop_3(s, OP_SEND as libc::c_int as mrb_code, (*s).sp,
                    new_sym(s,
                            mrb_intern_static((*s).mrb,
                                              b"call\x00" as *const u8 as
                                                  *const libc::c_char,
                                              (::std::mem::size_of::<[libc::c_char; 5]>()
                                                   as
                                                   libc::c_ulong).wrapping_sub(1i32
                                                                                   as
                                                                                   libc::c_ulong)))
                        as uint16_t, n_4 as uint8_t);
            if 0 != val { push_n_(s, 1i32); }
            current_block = 5764103746738363178;
        }
        10 => {
            loop_break(s, tree);
            if 0 != val { push_n_(s, 1i32); }
            current_block = 5764103746738363178;
        }
        11 => {
            if (*s).loop_0.is_null() {
                raise_error(s,
                            b"unexpected next\x00" as *const u8 as
                                *const libc::c_char);
            } else if (*(*s).loop_0).type_0 as libc::c_uint ==
                          LOOP_NORMAL as libc::c_int as libc::c_uint {
                if (*s).ensure_level > (*(*s).loop_0).ensure_level {
                    genop_1(s, OP_EPOP as libc::c_int as mrb_code,
                            ((*s).ensure_level - (*(*s).loop_0).ensure_level)
                                as uint16_t);
                }
                codegen(s, tree, 0i32);
                genjmp(s, OP_JMP as libc::c_int as mrb_code,
                       (*(*s).loop_0).pc0 as uint16_t);
            } else {
                if !tree.is_null() {
                    codegen(s, tree, 1i32);
                    pop_n_(s, 1i32);
                } else {
                    genop_1(s, OP_LOADNIL as libc::c_int as mrb_code,
                            (*s).sp);
                }
                gen_return(s, OP_RETURN as libc::c_int as uint8_t, (*s).sp);
            }
            if 0 != val { push_n_(s, 1i32); }
            current_block = 5764103746738363178;
        }
        12 => {
            if (*s).loop_0.is_null() ||
                   (*(*s).loop_0).type_0 as libc::c_uint ==
                       LOOP_BEGIN as libc::c_int as libc::c_uint ||
                   (*(*s).loop_0).type_0 as libc::c_uint ==
                       LOOP_RESCUE as libc::c_int as libc::c_uint {
                raise_error(s,
                            b"unexpected redo\x00" as *const u8 as
                                *const libc::c_char);
            } else {
                if (*s).ensure_level > (*(*s).loop_0).ensure_level {
                    genop_1(s, OP_EPOP as libc::c_int as mrb_code,
                            ((*s).ensure_level - (*(*s).loop_0).ensure_level)
                                as uint16_t);
                }
                genjmp(s, OP_JMP as libc::c_int as mrb_code,
                       (*(*s).loop_0).pc2 as uint16_t);
            }
            if 0 != val { push_n_(s, 1i32); }
            current_block = 5764103746738363178;
        }
        13 => {
            let mut msg: *const libc::c_char =
                b"unexpected retry\x00" as *const u8 as *const libc::c_char;
            if (*s).loop_0.is_null() {
                raise_error(s, msg);
            } else {
                let mut lp_3: *mut loopinfo = (*s).loop_0;
                let mut n_5: libc::c_int = 0i32;
                while !lp_3.is_null() &&
                          (*lp_3).type_0 as libc::c_uint !=
                              LOOP_RESCUE as libc::c_int as libc::c_uint {
                    if (*lp_3).type_0 as libc::c_uint ==
                           LOOP_BEGIN as libc::c_int as libc::c_uint {
                        n_5 += 1
                    }
                    lp_3 = (*lp_3).prev
                }
                if lp_3.is_null() {
                    raise_error(s, msg);
                } else {
                    if n_5 > 0i32 {
                        genop_1(s, OP_POPERR as libc::c_int as mrb_code,
                                n_5 as uint16_t);
                    }
                    if (*s).ensure_level > (*lp_3).ensure_level {
                        genop_1(s, OP_EPOP as libc::c_int as mrb_code,
                                ((*s).ensure_level - (*lp_3).ensure_level) as
                                    uint16_t);
                    }
                    genjmp(s, OP_JMP as libc::c_int as mrb_code,
                           (*lp_3).pc0 as uint16_t);
                }
            }
            if 0 != val { push_n_(s, 1i32); }
            current_block = 5764103746738363178;
        }
        37 => {
            if 0 != val {
                let mut idx_3: libc::c_int =
                    lv_idx(s, tree as intptr_t as mrb_sym);
                if idx_3 > 0i32 {
                    gen_move(s, (*s).sp, idx_3 as uint16_t, val);
                    if 0 != val && 0 != on_eval(s) as libc::c_int {
                        genop_0(s, OP_NOP as libc::c_int as mrb_code);
                    }
                } else {
                    let mut lv_2: libc::c_int = 0i32;
                    let mut up: *mut codegen_scope = (*s).prev;
                    while !up.is_null() {
                        idx_3 = lv_idx(up, tree as intptr_t as mrb_sym);
                        if idx_3 > 0i32 {
                            genop_3(s, OP_GETUPVAR as libc::c_int as mrb_code,
                                    (*s).sp, idx_3 as uint16_t,
                                    lv_2 as uint8_t);
                            break ;
                        } else { lv_2 += 1; up = (*up).prev }
                    }
                }
                push_n_(s, 1i32);
            }
            current_block = 5764103746738363178;
        }
        39 => {
            let mut sym_2: libc::c_int =
                new_sym(s, tree as intptr_t as mrb_sym);
            genop_2(s, OP_GETGV as libc::c_int as mrb_code, (*s).sp,
                    sym_2 as uint16_t);
            if 0 != val { push_n_(s, 1i32); }
            current_block = 5764103746738363178;
        }
        40 => {
            let mut sym_3: libc::c_int =
                new_sym(s, tree as intptr_t as mrb_sym);
            genop_2(s, OP_GETIV as libc::c_int as mrb_code, (*s).sp,
                    sym_3 as uint16_t);
            if 0 != val { push_n_(s, 1i32); }
            current_block = 5764103746738363178;
        }
        42 => {
            let mut sym_4: libc::c_int =
                new_sym(s, tree as intptr_t as mrb_sym);
            genop_2(s, OP_GETCV as libc::c_int as mrb_code, (*s).sp,
                    sym_4 as uint16_t);
            if 0 != val { push_n_(s, 1i32); }
            current_block = 5764103746738363178;
        }
        41 => {
            let mut sym_5: libc::c_int =
                new_sym(s, tree as intptr_t as mrb_sym);
            genop_2(s, OP_GETCONST as libc::c_int as mrb_code, (*s).sp,
                    sym_5 as uint16_t);
            if 0 != val { push_n_(s, 1i32); }
            current_block = 5764103746738363178;
        }
        81 => { codegen(s, tree, val); current_block = 5764103746738363178; }
        44 => {
            if 0 != val {
                let mut buf: [libc::c_char; 2] =
                    ['$' as i32 as libc::c_char,
                     tree as intptr_t as libc::c_char];
                let mut sym_6: libc::c_int =
                    new_sym(s,
                            mrb_intern((*s).mrb, buf.as_mut_ptr(),
                                       ::std::mem::size_of::<[libc::c_char; 2]>()
                                           as libc::c_ulong));
                genop_2(s, OP_GETGV as libc::c_int as mrb_code, (*s).sp,
                        sym_6 as uint16_t);
                push_n_(s, 1i32);
            }
            current_block = 5764103746738363178;
        }
        43 => {
            if 0 != val {
                let mut mrb: *mut mrb_state = (*s).mrb;
                let mut str: mrb_value =
                    mrb_value{value: C2RustUnnamed_4{f: 0.,},
                              tt: MRB_TT_FALSE,};
                let mut sym_7: libc::c_int = 0;
                str =
                    mrb_format(mrb,
                               b"$%d\x00" as *const u8 as *const libc::c_char,
                               tree as intptr_t as libc::c_int);
                sym_7 = new_sym(s, mrb_intern_str(mrb, str));
                genop_2(s, OP_GETGV as libc::c_int as mrb_code, (*s).sp,
                        sym_7 as uint16_t);
                push_n_(s, 1i32);
            }
            current_block = 5764103746738363178;
        }
        58 => {
            /* should not happen */
            current_block = 5764103746738363178;
        }
        65 => { codegen(s, tree, val); current_block = 5764103746738363178; }
        46 => {
            if 0 != val {
                let mut p_0: *mut libc::c_char =
                    (*tree).car as *mut libc::c_char;
                let mut base_0: libc::c_int =
                    (*(*tree).cdr).car as intptr_t as libc::c_int;
                let mut i_0: mrb_int = 0;
                let mut overflow: mrb_bool = 0;
                i_0 =
                    readint_mrb_int(s, p_0, base_0, 0i32 as mrb_bool,
                                    &mut overflow);
                if 0 != overflow {
                    let mut f: libc::c_double = readint_float(s, p_0, base_0);
                    let mut off: libc::c_int =
                        new_lit(s, mrb_float_value((*s).mrb, f));
                    genop_2(s, OP_LOADL as libc::c_int as mrb_code, (*s).sp,
                            off as uint16_t);
                } else if i_0 == -1i32 as libc::c_longlong {
                    genop_1(s, OP_LOADI__1 as libc::c_int as mrb_code,
                            (*s).sp);
                } else if i_0 < 0i32 as libc::c_longlong {
                    genop_2(s, OP_LOADINEG as libc::c_int as mrb_code,
                            (*s).sp, -i_0 as uint16_t);
                } else if i_0 < 8i32 as libc::c_longlong {
                    genop_1(s,
                            (OP_LOADI_0 as libc::c_int +
                                 i_0 as uint8_t as libc::c_int) as mrb_code,
                            (*s).sp);
                } else if i_0 <= 0xffffi32 as libc::c_longlong {
                    genop_2(s, OP_LOADI as libc::c_int as mrb_code, (*s).sp,
                            i_0 as uint16_t);
                } else {
                    let mut off_0: libc::c_int =
                        new_lit(s, mrb_fixnum_value(i_0));
                    genop_2(s, OP_LOADL as libc::c_int as mrb_code, (*s).sp,
                            off_0 as uint16_t);
                }
                push_n_(s, 1i32);
            }
            current_block = 5764103746738363178;
        }
        47 => {
            if 0 != val {
                let mut p_1: *mut libc::c_char = tree as *mut libc::c_char;
                let mut f_0: mrb_float =
                    mrb_float_read(p_1, 0 as *mut *mut libc::c_char);
                let mut off_1: libc::c_int =
                    new_lit(s, mrb_float_value((*s).mrb, f_0));
                genop_2(s, OP_LOADL as libc::c_int as mrb_code, (*s).sp,
                        off_1 as uint16_t);
                push_n_(s, 1i32);
            }
            current_block = 5764103746738363178;
        }
        48 => {
            nt = (*tree).car as intptr_t as libc::c_int;
            match nt {
                47 => {
                    if 0 != val {
                        let mut p_2: *mut libc::c_char =
                            (*tree).cdr as *mut libc::c_char;
                        let mut f_1: mrb_float =
                            mrb_float_read(p_2, 0 as *mut *mut libc::c_char);
                        let mut off_2: libc::c_int =
                            new_lit(s, mrb_float_value((*s).mrb, -f_1));
                        genop_2(s, OP_LOADL as libc::c_int as mrb_code,
                                (*s).sp, off_2 as uint16_t);
                        push_n_(s, 1i32);
                    }
                }
                46 => {
                    if 0 != val {
                        let mut p_3: *mut libc::c_char =
                            (*(*tree).cdr).car as *mut libc::c_char;
                        let mut base_1: libc::c_int =
                            (*(*(*tree).cdr).cdr).car as intptr_t as
                                libc::c_int;
                        let mut i_1: mrb_int = 0;
                        let mut overflow_0: mrb_bool = 0;
                        i_1 =
                            readint_mrb_int(s, p_3, base_1, 1i32 as mrb_bool,
                                            &mut overflow_0);
                        if 0 != overflow_0 {
                            let mut f_2: libc::c_double =
                                readint_float(s, p_3, base_1);
                            let mut off_3: libc::c_int =
                                new_lit(s, mrb_float_value((*s).mrb, -f_2));
                            genop_2(s, OP_LOADL as libc::c_int as mrb_code,
                                    (*s).sp, off_3 as uint16_t);
                        } else if i_1 == -1i32 as libc::c_longlong {
                            genop_1(s, OP_LOADI__1 as libc::c_int as mrb_code,
                                    (*s).sp);
                        } else if i_1 >= -0xffffi32 as libc::c_longlong {
                            genop_2(s, OP_LOADINEG as libc::c_int as mrb_code,
                                    (*s).sp, -i_1 as uint16_t);
                        } else {
                            let mut off_4: libc::c_int =
                                new_lit(s, mrb_fixnum_value(i_1));
                            genop_2(s, OP_LOADL as libc::c_int as mrb_code,
                                    (*s).sp, off_4 as uint16_t);
                        }
                        push_n_(s, 1i32);
                    }
                }
                _ => {
                    if 0 != val {
                        let mut sym_8: libc::c_int =
                            new_sym(s,
                                    mrb_intern_static((*s).mrb,
                                                      b"-@\x00" as *const u8
                                                          as
                                                          *const libc::c_char,
                                                      (::std::mem::size_of::<[libc::c_char; 3]>()
                                                           as
                                                           libc::c_ulong).wrapping_sub(1i32
                                                                                           as
                                                                                           libc::c_ulong)));
                        codegen(s, tree, 1i32);
                        pop_n_(s, 1i32);
                        genop_3(s, OP_SEND as libc::c_int as mrb_code,
                                (*s).sp, sym_8 as uint16_t, 0i32 as uint8_t);
                        push_n_(s, 1i32);
                    } else { codegen(s, tree, 0i32); }
                }
            }
            current_block = 5764103746738363178;
        }
        51 => {
            if 0 != val {
                let mut p_4: *mut libc::c_char =
                    (*tree).car as *mut libc::c_char;
                let mut len_2: size_t = (*tree).cdr as intptr_t as size_t;
                let mut ai: libc::c_int = mrb_gc_arena_save((*s).mrb);
                let mut off_5: libc::c_int =
                    new_lit(s, mrb_str_new((*s).mrb, p_4, len_2));
                mrb_gc_arena_restore((*s).mrb, ai);
                genop_2(s, OP_STRING as libc::c_int as mrb_code, (*s).sp,
                        off_5 as uint16_t);
                push_n_(s, 1i32);
            }
            current_block = 5764103746738363178;
        }
        84 => {
            tree = (*(tree as *mut mrb_parser_heredoc_info)).doc;
            /* fall through */
            current_block = 15274472712294785679;
        }
        52 => { current_block = 15274472712294785679; }
        86 => {
            gen_literal_array(s, tree, 0i32 as mrb_bool, val);
            current_block = 5764103746738363178;
        }
        87 => {
            gen_literal_array(s, tree, 1i32 as mrb_bool, val);
            current_block = 5764103746738363178;
        }
        54 => {
            let mut n_8: *mut node = 0 as *mut node;
            let mut ai_0: libc::c_int = mrb_gc_arena_save((*s).mrb);
            let mut sym_9: libc::c_int =
                new_sym(s,
                        mrb_intern_static((*s).mrb,
                                          b"Kernel\x00" as *const u8 as
                                              *const libc::c_char,
                                          (::std::mem::size_of::<[libc::c_char; 7]>()
                                               as
                                               libc::c_ulong).wrapping_sub(1i32
                                                                               as
                                                                               libc::c_ulong)));
            genop_1(s, OP_LOADSELF as libc::c_int as mrb_code, (*s).sp);
            push_n_(s, 1i32);
            codegen(s, (*tree).car, 1i32);
            n_8 = (*tree).cdr;
            while !n_8.is_null() {
                if (*(*n_8).car).car as intptr_t as libc::c_int ==
                       NODE_XSTR as libc::c_int {
                    (*(*n_8).car).car =
                        NODE_STR as libc::c_int as intptr_t as
                            *mut mrb_ast_node
                }
                codegen(s, (*n_8).car, 1i32);
                pop_n_(s, 1i32);
                pop_n_(s, 1i32);
                genop_1(s, OP_STRCAT as libc::c_int as mrb_code, (*s).sp);
                push_n_(s, 1i32);
                n_8 = (*n_8).cdr
            }
            /* for block */
            push_n_(s, 1i32);
            pop_n_(s, 3i32);
            sym_9 =
                new_sym(s,
                        mrb_intern_static((*s).mrb,
                                          b"`\x00" as *const u8 as
                                              *const libc::c_char,
                                          (::std::mem::size_of::<[libc::c_char; 2]>()
                                               as
                                               libc::c_ulong).wrapping_sub(1i32
                                                                               as
                                                                               libc::c_ulong)));
            genop_3(s, OP_SEND as libc::c_int as mrb_code, (*s).sp,
                    sym_9 as uint16_t, 1i32 as uint8_t);
            if 0 != val { push_n_(s, 1i32); }
            mrb_gc_arena_restore((*s).mrb, ai_0);
            current_block = 5764103746738363178;
        }
        53 => {
            let mut p_5: *mut libc::c_char = (*tree).car as *mut libc::c_char;
            let mut len_3: size_t = (*tree).cdr as intptr_t as size_t;
            let mut ai_1: libc::c_int = mrb_gc_arena_save((*s).mrb);
            let mut off_6: libc::c_int =
                new_lit(s, mrb_str_new((*s).mrb, p_5, len_3));
            let mut sym_10: libc::c_int = 0;
            genop_1(s, OP_LOADSELF as libc::c_int as mrb_code, (*s).sp);
            push_n_(s, 1i32);
            genop_2(s, OP_STRING as libc::c_int as mrb_code, (*s).sp,
                    off_6 as uint16_t);
            push_n_(s, 1i32);
            push_n_(s, 1i32);
            pop_n_(s, 3i32);
            sym_10 =
                new_sym(s,
                        mrb_intern_static((*s).mrb,
                                          b"`\x00" as *const u8 as
                                              *const libc::c_char,
                                          (::std::mem::size_of::<[libc::c_char; 2]>()
                                               as
                                               libc::c_ulong).wrapping_sub(1i32
                                                                               as
                                                                               libc::c_ulong)));
            genop_3(s, OP_SEND as libc::c_int as mrb_code, (*s).sp,
                    sym_10 as uint16_t, 1i32 as uint8_t);
            if 0 != val { push_n_(s, 1i32); }
            mrb_gc_arena_restore((*s).mrb, ai_1);
            current_block = 5764103746738363178;
        }
        55 => {
            if 0 != val {
                let mut p1: *mut libc::c_char =
                    (*tree).car as *mut libc::c_char;
                let mut p2: *mut libc::c_char =
                    (*(*tree).cdr).car as *mut libc::c_char;
                let mut p3: *mut libc::c_char =
                    (*(*tree).cdr).cdr as *mut libc::c_char;
                let mut ai_2: libc::c_int = mrb_gc_arena_save((*s).mrb);
                let mut sym_11: libc::c_int =
                    new_sym(s,
                            mrb_intern_static((*s).mrb,
                                              b"Regexp\x00" as *const u8 as
                                                  *const libc::c_char,
                                              (::std::mem::size_of::<[libc::c_char; 7]>()
                                                   as
                                                   libc::c_ulong).wrapping_sub(1i32
                                                                                   as
                                                                                   libc::c_ulong)));
                let mut off_7: libc::c_int =
                    new_lit(s, mrb_str_new_cstr((*s).mrb, p1));
                let mut argc: libc::c_int = 1i32;
                genop_1(s, OP_OCLASS as libc::c_int as mrb_code, (*s).sp);
                genop_2(s, OP_GETMCNST as libc::c_int as mrb_code, (*s).sp,
                        sym_11 as uint16_t);
                push_n_(s, 1i32);
                genop_2(s, OP_STRING as libc::c_int as mrb_code, (*s).sp,
                        off_7 as uint16_t);
                push_n_(s, 1i32);
                if !p2.is_null() || !p3.is_null() {
                    if !p2.is_null() {
                        /* opt */
                        off_7 = new_lit(s, mrb_str_new_cstr((*s).mrb, p2));
                        genop_2(s, OP_STRING as libc::c_int as mrb_code,
                                (*s).sp, off_7 as uint16_t);
                    } else {
                        genop_1(s, OP_LOADNIL as libc::c_int as mrb_code,
                                (*s).sp);
                    }
                    push_n_(s, 1i32);
                    argc += 1;
                    if !p3.is_null() {
                        /* enc */
                        off_7 =
                            new_lit(s,
                                    mrb_str_new((*s).mrb, p3,
                                                1i32 as size_t));
                        genop_2(s, OP_STRING as libc::c_int as mrb_code,
                                (*s).sp, off_7 as uint16_t);
                        push_n_(s, 1i32);
                        argc += 1
                    }
                }
                /* space for a block */
                push_n_(s, 1i32);
                pop_n_(s, argc + 2i32);
                sym_11 =
                    new_sym(s,
                            mrb_intern_static((*s).mrb,
                                              b"compile\x00" as *const u8 as
                                                  *const libc::c_char,
                                              (::std::mem::size_of::<[libc::c_char; 8]>()
                                                   as
                                                   libc::c_ulong).wrapping_sub(1i32
                                                                                   as
                                                                                   libc::c_ulong)));
                genop_3(s, OP_SEND as libc::c_int as mrb_code, (*s).sp,
                        sym_11 as uint16_t, argc as uint8_t);
                mrb_gc_arena_restore((*s).mrb, ai_2);
                push_n_(s, 1i32);
            }
            current_block = 5764103746738363178;
        }
        56 => {
            if 0 != val {
                let mut n_9: *mut node = (*tree).car;
                let mut ai_3: libc::c_int = mrb_gc_arena_save((*s).mrb);
                let mut sym_12: libc::c_int =
                    new_sym(s,
                            mrb_intern_static((*s).mrb,
                                              b"Regexp\x00" as *const u8 as
                                                  *const libc::c_char,
                                              (::std::mem::size_of::<[libc::c_char; 7]>()
                                                   as
                                                   libc::c_ulong).wrapping_sub(1i32
                                                                                   as
                                                                                   libc::c_ulong)));
                let mut argc_0: libc::c_int = 1i32;
                let mut off_8: libc::c_int = 0;
                let mut p_6: *mut libc::c_char = 0 as *mut libc::c_char;
                genop_1(s, OP_OCLASS as libc::c_int as mrb_code, (*s).sp);
                genop_2(s, OP_GETMCNST as libc::c_int as mrb_code, (*s).sp,
                        sym_12 as uint16_t);
                push_n_(s, 1i32);
                codegen(s, (*n_9).car, 1i32);
                n_9 = (*n_9).cdr;
                while !n_9.is_null() {
                    codegen(s, (*n_9).car, 1i32);
                    pop_n_(s, 1i32);
                    pop_n_(s, 1i32);
                    genop_1(s, OP_STRCAT as libc::c_int as mrb_code, (*s).sp);
                    push_n_(s, 1i32);
                    n_9 = (*n_9).cdr
                }
                n_9 = (*(*tree).cdr).cdr;
                if !(*n_9).car.is_null() {
                    /* tail */
                    p_6 = (*n_9).car as *mut libc::c_char;
                    off_8 = new_lit(s, mrb_str_new_cstr((*s).mrb, p_6));
                    codegen(s, (*tree).car, 1i32);
                    genop_2(s, OP_STRING as libc::c_int as mrb_code, (*s).sp,
                            off_8 as uint16_t);
                    pop_n_(s, 1i32);
                    genop_1(s, OP_STRCAT as libc::c_int as mrb_code, (*s).sp);
                    push_n_(s, 1i32);
                }
                if !(*(*n_9).cdr).car.is_null() {
                    /* opt */
                    let mut p2_0: *mut libc::c_char =
                        (*(*n_9).cdr).car as *mut libc::c_char;
                    off_8 = new_lit(s, mrb_str_new_cstr((*s).mrb, p2_0));
                    genop_2(s, OP_STRING as libc::c_int as mrb_code, (*s).sp,
                            off_8 as uint16_t);
                    push_n_(s, 1i32);
                    argc_0 += 1
                }
                if !(*(*n_9).cdr).cdr.is_null() {
                    /* enc */
                    let mut p2_1: *mut libc::c_char =
                        (*(*n_9).cdr).cdr as *mut libc::c_char;
                    off_8 = new_lit(s, mrb_str_new_cstr((*s).mrb, p2_1));
                    genop_2(s, OP_STRING as libc::c_int as mrb_code, (*s).sp,
                            off_8 as uint16_t);
                    push_n_(s, 1i32);
                    argc_0 += 1
                }
                /* space for a block */
                push_n_(s, 1i32);
                pop_n_(s, argc_0 + 2i32);
                sym_12 =
                    new_sym(s,
                            mrb_intern_static((*s).mrb,
                                              b"compile\x00" as *const u8 as
                                                  *const libc::c_char,
                                              (::std::mem::size_of::<[libc::c_char; 8]>()
                                                   as
                                                   libc::c_ulong).wrapping_sub(1i32
                                                                                   as
                                                                                   libc::c_ulong)));
                genop_3(s, OP_SEND as libc::c_int as mrb_code, (*s).sp,
                        sym_12 as uint16_t, argc_0 as uint8_t);
                mrb_gc_arena_restore((*s).mrb, ai_3);
                push_n_(s, 1i32);
            } else {
                let mut n_10: *mut node = (*tree).car;
                while !n_10.is_null() {
                    if (*(*n_10).car).car as intptr_t as libc::c_int !=
                           NODE_STR as libc::c_int {
                        codegen(s, (*n_10).car, 0i32);
                    }
                    n_10 = (*n_10).cdr
                }
            }
            current_block = 5764103746738363178;
        }
        50 => {
            if 0 != val {
                let mut sym_13: libc::c_int =
                    new_sym(s, tree as intptr_t as mrb_sym);
                genop_2(s, OP_LOADSYM as libc::c_int as mrb_code, (*s).sp,
                        sym_13 as uint16_t);
                push_n_(s, 1i32);
            }
            current_block = 5764103746738363178;
        }
        83 => {
            codegen(s, tree, val);
            if 0 != val { gen_intern(s); }
            current_block = 5764103746738363178;
        }
        77 => {
            if 0 != val {
                genop_1(s, OP_LOADSELF as libc::c_int as mrb_code, (*s).sp);
                push_n_(s, 1i32);
            }
            current_block = 5764103746738363178;
        }
        78 => {
            if 0 != val {
                genop_1(s, OP_LOADNIL as libc::c_int as mrb_code, (*s).sp);
                push_n_(s, 1i32);
            }
            current_block = 5764103746738363178;
        }
        79 => {
            if 0 != val {
                genop_1(s, OP_LOADT as libc::c_int as mrb_code, (*s).sp);
                push_n_(s, 1i32);
            }
            current_block = 5764103746738363178;
        }
        80 => {
            if 0 != val {
                genop_1(s, OP_LOADF as libc::c_int as mrb_code, (*s).sp);
                push_n_(s, 1i32);
            }
            current_block = 5764103746738363178;
        }
        68 => {
            let mut a: libc::c_int =
                new_sym(s, (*tree).car as intptr_t as mrb_sym);
            let mut b: libc::c_int =
                new_sym(s, (*tree).cdr as intptr_t as mrb_sym);
            genop_2(s, OP_ALIAS as libc::c_int as mrb_code, a as uint16_t,
                    b as uint16_t);
            if 0 != val {
                genop_1(s, OP_LOADNIL as libc::c_int as mrb_code, (*s).sp);
                push_n_(s, 1i32);
            }
            current_block = 5764103746738363178;
        }
        69 => {
            let mut t_0: *mut node = tree;
            while !t_0.is_null() {
                let mut symbol: libc::c_int =
                    new_sym(s, (*t_0).car as intptr_t as mrb_sym);
                genop_1(s, OP_UNDEF as libc::c_int as mrb_code,
                        symbol as uint16_t);
                t_0 = (*t_0).cdr
            }
            if 0 != val {
                genop_1(s, OP_LOADNIL as libc::c_int as mrb_code, (*s).sp);
                push_n_(s, 1i32);
            }
            current_block = 5764103746738363178;
        }
        70 => {
            let mut idx_4: libc::c_int = 0;
            let mut body: *mut node = 0 as *mut node;
            if (*(*tree).car).car.is_null() {
                genop_1(s, OP_LOADNIL as libc::c_int as mrb_code, (*s).sp);
                push_n_(s, 1i32);
            } else if (*(*tree).car).car == 1i32 as *mut node {
                genop_1(s, OP_OCLASS as libc::c_int as mrb_code, (*s).sp);
                push_n_(s, 1i32);
            } else { codegen(s, (*(*tree).car).car, 1i32); }
            if !(*(*tree).cdr).car.is_null() {
                codegen(s, (*(*tree).cdr).car, 1i32);
            } else {
                genop_1(s, OP_LOADNIL as libc::c_int as mrb_code, (*s).sp);
                push_n_(s, 1i32);
            }
            pop_n_(s, 1i32);
            pop_n_(s, 1i32);
            idx_4 = new_sym(s, (*(*tree).car).cdr as intptr_t as mrb_sym);
            genop_2(s, OP_CLASS as libc::c_int as mrb_code, (*s).sp,
                    idx_4 as uint16_t);
            body = (*(*(*tree).cdr).cdr).car;
            if (*(*body).cdr).car as intptr_t as libc::c_int ==
                   NODE_BEGIN as libc::c_int && (*(*body).cdr).cdr.is_null() {
                genop_1(s, OP_LOADNIL as libc::c_int as mrb_code, (*s).sp);
            } else {
                idx_4 = scope_body(s, body, val);
                genop_2(s, OP_EXEC as libc::c_int as mrb_code, (*s).sp,
                        idx_4 as uint16_t);
            }
            if 0 != val { push_n_(s, 1i32); }
            current_block = 5764103746738363178;
        }
        71 => {
            let mut idx_5: libc::c_int = 0;
            if (*(*tree).car).car.is_null() {
                genop_1(s, OP_LOADNIL as libc::c_int as mrb_code, (*s).sp);
                push_n_(s, 1i32);
            } else if (*(*tree).car).car == 1i32 as *mut node {
                genop_1(s, OP_OCLASS as libc::c_int as mrb_code, (*s).sp);
                push_n_(s, 1i32);
            } else { codegen(s, (*(*tree).car).car, 1i32); }
            pop_n_(s, 1i32);
            idx_5 = new_sym(s, (*(*tree).car).cdr as intptr_t as mrb_sym);
            genop_2(s, OP_MODULE as libc::c_int as mrb_code, (*s).sp,
                    idx_5 as uint16_t);
            if (*(*(*(*tree).cdr).car).cdr).car as intptr_t as libc::c_int ==
                   NODE_BEGIN as libc::c_int &&
                   (*(*(*(*tree).cdr).car).cdr).cdr.is_null() {
                genop_1(s, OP_LOADNIL as libc::c_int as mrb_code, (*s).sp);
            } else {
                idx_5 = scope_body(s, (*(*tree).cdr).car, val);
                genop_2(s, OP_EXEC as libc::c_int as mrb_code, (*s).sp,
                        idx_5 as uint16_t);
            }
            if 0 != val { push_n_(s, 1i32); }
            current_block = 5764103746738363178;
        }
        72 => {
            let mut idx_6: libc::c_int = 0;
            codegen(s, (*tree).car, 1i32);
            pop_n_(s, 1i32);
            genop_1(s, OP_SCLASS as libc::c_int as mrb_code, (*s).sp);
            if (*(*(*(*tree).cdr).car).cdr).car as intptr_t as libc::c_int ==
                   NODE_BEGIN as libc::c_int &&
                   (*(*(*(*tree).cdr).car).cdr).cdr.is_null() {
                genop_1(s, OP_LOADNIL as libc::c_int as mrb_code, (*s).sp);
            } else {
                idx_6 = scope_body(s, (*(*tree).cdr).car, val);
                genop_2(s, OP_EXEC as libc::c_int as mrb_code, (*s).sp,
                        idx_6 as uint16_t);
            }
            if 0 != val { push_n_(s, 1i32); }
            current_block = 5764103746738363178;
        }
        66 => {
            let mut sym_14: libc::c_int =
                new_sym(s, (*tree).car as intptr_t as mrb_sym);
            let mut idx_7: libc::c_int = lambda_body(s, (*tree).cdr, 0i32);
            genop_1(s, OP_TCLASS as libc::c_int as mrb_code, (*s).sp);
            push_n_(s, 1i32);
            genop_2(s, OP_METHOD as libc::c_int as mrb_code, (*s).sp,
                    idx_7 as uint16_t);
            push_n_(s, 1i32);
            pop_n_(s, 1i32);
            pop_n_(s, 1i32);
            genop_2(s, OP_DEF as libc::c_int as mrb_code, (*s).sp,
                    sym_14 as uint16_t);
            if 0 != val {
                genop_2(s, OP_LOADSYM as libc::c_int as mrb_code, (*s).sp,
                        sym_14 as uint16_t);
                push_n_(s, 1i32);
            }
            current_block = 5764103746738363178;
        }
        67 => {
            let mut recv: *mut node = (*tree).car;
            let mut sym_15: libc::c_int =
                new_sym(s, (*(*tree).cdr).car as intptr_t as mrb_sym);
            let mut idx_8: libc::c_int =
                lambda_body(s, (*(*tree).cdr).cdr, 0i32);
            codegen(s, recv, 1i32);
            pop_n_(s, 1i32);
            genop_1(s, OP_SCLASS as libc::c_int as mrb_code, (*s).sp);
            push_n_(s, 1i32);
            genop_2(s, OP_METHOD as libc::c_int as mrb_code, (*s).sp,
                    idx_8 as uint16_t);
            pop_n_(s, 1i32);
            genop_2(s, OP_DEF as libc::c_int as mrb_code, (*s).sp,
                    sym_15 as uint16_t);
            if 0 != val {
                genop_2(s, OP_LOADSYM as libc::c_int as mrb_code, (*s).sp,
                        sym_15 as uint16_t);
                push_n_(s, 1i32);
            }
            current_block = 5764103746738363178;
        }
        82 => { codegen(s, tree, 0i32); current_block = 5764103746738363178; }
        _ => { current_block = 5764103746738363178; }
    }
    match current_block {
        15274472712294785679 => {
            if 0 != val {
                let mut n_6: *mut node = tree;
                if n_6.is_null() {
                    genop_1(s, OP_LOADNIL as libc::c_int as mrb_code,
                            (*s).sp);
                    push_n_(s, 1i32);
                } else {
                    codegen(s, (*n_6).car, 1i32);
                    n_6 = (*n_6).cdr;
                    while !n_6.is_null() {
                        codegen(s, (*n_6).car, 1i32);
                        pop_n_(s, 1i32);
                        pop_n_(s, 1i32);
                        genop_1(s, OP_STRCAT as libc::c_int as mrb_code,
                                (*s).sp);
                        push_n_(s, 1i32);
                        n_6 = (*n_6).cdr
                    }
                }
            } else {
                let mut n_7: *mut node = tree;
                while !n_7.is_null() {
                    if (*(*n_7).car).car as intptr_t as libc::c_int !=
                           NODE_STR as libc::c_int {
                        codegen(s, (*n_7).car, 0i32);
                    }
                    n_7 = (*n_7).cdr
                }
            }
        }
        _ => { }
    }
    (*s).rlev = rlev;
}
unsafe extern "C" fn scope_add_irep(mut s: *mut codegen_scope,
                                    mut irep: *mut mrb_irep) {
    if (*s).irep.is_null() { (*s).irep = irep; return }
    if (*(*s).irep).rlen as libc::c_uint == (*s).rcapa {
        (*s).rcapa =
            ((*s).rcapa as libc::c_uint).wrapping_mul(2i32 as libc::c_uint) as
                uint32_t as uint32_t;
        (*(*s).irep).reps =
            codegen_realloc(s, (*(*s).irep).reps as *mut libc::c_void,
                            (::std::mem::size_of::<*mut mrb_irep>() as
                                 libc::c_ulong).wrapping_mul((*s).rcapa as
                                                                 libc::c_ulong))
                as *mut *mut mrb_irep
    }
    let ref mut fresh365 =
        *(*(*s).irep).reps.offset((*(*s).irep).rlen as isize);
    *fresh365 = irep;
    (*(*s).irep).rlen = (*(*s).irep).rlen.wrapping_add(1);
}
// Initialized in run_static_initializers
static mut codegen_scope_zero: codegen_scope =
    codegen_scope{mrb: 0 as *const mrb_state as *mut mrb_state,
                  mpool: 0 as *const mrb_pool as *mut mrb_pool,
                  jmp: mrb_jmpbuf{impl_0: [0; 37],},
                  _pad: [0; 4],
                  prev: 0 as *const scope as *mut scope,
                  lv: 0 as *const node as *mut node,
                  sp: 0,
                  pc: 0,
                  lastpc: 0,
                  lastlabel: 0,
                  ainfo_mscope: [0; 2],
                  _pad2: [0; 6],
                  loop_0: 0 as *const loopinfo as *mut loopinfo,
                  ensure_level: 0,
                  filename_sym: 0,
                  lineno: 0,
                  _pad3: [0; 6],
                  iseq: 0 as *const mrb_code as *mut mrb_code,
                  lines: 0 as *const uint16_t as *mut uint16_t,
                  icapa: 0,
                  _pad4: [0; 4],
                  irep: 0 as *const mrb_irep as *mut mrb_irep,
                  pcapa: 0,
                  scapa: 0,
                  rcapa: 0,
                  nlocals: 0,
                  nregs: 0,
                  ai: 0,
                  debug_start_pos: 0,
                  filename_index: 0,
                  _pad5: [0; 6],
                  parser: 0 as *const parser_state as *mut parser_state,
                  rlev: 0,
                  _pad6: [0; 4],};
unsafe extern "C" fn scope_new(mut mrb: *mut mrb_state,
                               mut prev: *mut codegen_scope,
                               mut lv: *mut node) -> *mut codegen_scope {
    let mut pool: *mut mrb_pool = mrb_pool_open(mrb);
    let mut p: *mut codegen_scope =
        mrb_pool_alloc(pool,
                       ::std::mem::size_of::<codegen_scope>() as
                           libc::c_ulong) as *mut codegen_scope;
    if p.is_null() { return 0 as *mut codegen_scope }
    *p = codegen_scope_zero;
    (*p).mrb = mrb;
    (*p).mpool = pool;
    if prev.is_null() { return p }
    (*p).prev = prev;
    (*p).set_ainfo(-1i32);
    (*p).set_mscope(0i32 as mrb_bool);
    (*p).irep = mrb_add_irep(mrb);
    scope_add_irep(prev, (*p).irep);
    (*p).rcapa = 8i32 as uint32_t;
    (*(*p).irep).reps =
        mrb_malloc(mrb,
                   (::std::mem::size_of::<*mut mrb_irep>() as
                        libc::c_ulong).wrapping_mul((*p).rcapa as
                                                        libc::c_ulong)) as
            *mut *mut mrb_irep;
    (*p).icapa = 1024i32 as uint32_t;
    (*p).iseq =
        mrb_malloc(mrb,
                   (::std::mem::size_of::<mrb_code>() as
                        libc::c_ulong).wrapping_mul((*p).icapa as
                                                        libc::c_ulong)) as
            *mut mrb_code;
    (*(*p).irep).iseq = 0 as *mut mrb_code;
    (*p).pcapa = 32i32 as uint32_t;
    (*(*p).irep).pool =
        mrb_malloc(mrb,
                   (::std::mem::size_of::<mrb_value>() as
                        libc::c_ulong).wrapping_mul((*p).pcapa as
                                                        libc::c_ulong)) as
            *mut mrb_value;
    (*(*p).irep).plen = 0i32 as uint16_t;
    (*p).scapa = 256i32 as uint32_t;
    (*(*p).irep).syms =
        mrb_malloc(mrb,
                   (::std::mem::size_of::<mrb_sym>() as
                        libc::c_ulong).wrapping_mul((*p).scapa as
                                                        libc::c_ulong)) as
            *mut mrb_sym;
    (*(*p).irep).slen = 0i32 as uint16_t;
    (*p).lv = lv;
    /* add self */
    (*p).sp = ((*p).sp as libc::c_int + (node_len(lv) + 1i32)) as uint16_t;
    (*p).nlocals = (*p).sp;
    if !lv.is_null() {
        let mut n: *mut node = lv;
        let mut i: size_t = 0i32 as size_t;
        (*(*p).irep).lv =
            mrb_malloc(mrb,
                       (::std::mem::size_of::<mrb_locals>() as
                            libc::c_ulong).wrapping_mul(((*p).nlocals as
                                                             libc::c_int -
                                                             1i32) as
                                                            libc::c_ulong)) as
                *mut mrb_locals;
        i = 0i32 as size_t;
        n = lv;
        while !n.is_null() {
            (*(*(*p).irep).lv.offset(i as isize)).name =
                (*n).car as intptr_t as mrb_sym;
            if 0 != (*n).car as intptr_t as mrb_sym {
                (*(*(*p).irep).lv.offset(i as isize)).r =
                    lv_idx(p, (*n).car as intptr_t as mrb_sym) as uint16_t
            } else {
                (*(*(*p).irep).lv.offset(i as isize)).r = 0i32 as uint16_t
            }
            i = i.wrapping_add(1);
            n = (*n).cdr
        }
    }
    (*p).ai = mrb_gc_arena_save(mrb);
    (*p).filename_sym = (*prev).filename_sym;
    if 0 != (*p).filename_sym {
        (*p).lines =
            mrb_malloc(mrb,
                       (::std::mem::size_of::<libc::c_short>() as
                            libc::c_ulong).wrapping_mul((*p).icapa as
                                                            libc::c_ulong)) as
                *mut uint16_t
    }
    (*p).lineno = (*prev).lineno;
    /* debug setting */
    (*p).debug_start_pos = 0i32;
    if 0 != (*p).filename_sym {
        mrb_debug_info_alloc(mrb, (*p).irep);
    } else { (*(*p).irep).debug_info = 0 as *mut mrb_irep_debug_info }
    (*p).parser = (*prev).parser;
    (*p).filename_index = (*prev).filename_index;
    (*p).rlev = (*prev).rlev + 1i32;
    return p;
}
unsafe extern "C" fn scope_finish(mut s: *mut codegen_scope) {
    let mut mrb: *mut mrb_state = (*s).mrb;
    let mut irep: *mut mrb_irep = (*s).irep;
    if (*s).nlocals as libc::c_int >= 0x3ffi32 {
        codegen_error(s,
                      b"too many local variables\x00" as *const u8 as
                          *const libc::c_char);
    }
    (*irep).flags = 0i32 as uint8_t;
    if !(*s).iseq.is_null() {
        (*irep).iseq =
            codegen_realloc(s, (*s).iseq as *mut libc::c_void,
                            (::std::mem::size_of::<mrb_code>() as
                                 libc::c_ulong).wrapping_mul((*s).pc as
                                                                 libc::c_ulong))
                as *mut mrb_code;
        (*irep).ilen = (*s).pc
    }
    (*irep).pool =
        codegen_realloc(s, (*irep).pool as *mut libc::c_void,
                        (::std::mem::size_of::<mrb_value>() as
                             libc::c_ulong).wrapping_mul((*irep).plen as
                                                             libc::c_ulong))
            as *mut mrb_value;
    (*irep).syms =
        codegen_realloc(s, (*irep).syms as *mut libc::c_void,
                        (::std::mem::size_of::<mrb_sym>() as
                             libc::c_ulong).wrapping_mul((*irep).slen as
                                                             libc::c_ulong))
            as *mut mrb_sym;
    (*irep).reps =
        codegen_realloc(s, (*irep).reps as *mut libc::c_void,
                        (::std::mem::size_of::<*mut mrb_irep>() as
                             libc::c_ulong).wrapping_mul((*irep).rlen as
                                                             libc::c_ulong))
            as *mut *mut mrb_irep;
    if 0 != (*s).filename_sym {
        let mut fname: mrb_sym =
            mrb_parser_get_filename((*s).parser, (*s).filename_index);
        let mut filename: *const libc::c_char =
            mrb_sym2name_len((*s).mrb, fname, 0 as *mut mrb_int);
        mrb_debug_info_append_file((*s).mrb, (*(*s).irep).debug_info,
                                   filename, (*s).lines,
                                   (*s).debug_start_pos as uint32_t,
                                   (*s).pc as uint32_t);
    }
    mrb_free((*s).mrb, (*s).lines as *mut libc::c_void);
    (*irep).nlocals = (*s).nlocals;
    (*irep).nregs = (*s).nregs;
    mrb_gc_arena_restore(mrb, (*s).ai);
    mrb_pool_close((*s).mpool);
}
unsafe extern "C" fn loop_push(mut s: *mut codegen_scope, mut t: looptype)
 -> *mut loopinfo {
    let mut p: *mut loopinfo =
        codegen_palloc(s, ::std::mem::size_of::<loopinfo>() as libc::c_ulong)
            as *mut loopinfo;
    (*p).type_0 = t;
    (*p).pc3 = 0i32;
    (*p).pc2 = (*p).pc3;
    (*p).pc1 = (*p).pc2;
    (*p).pc0 = (*p).pc1;
    (*p).prev = (*s).loop_0;
    (*p).ensure_level = (*s).ensure_level;
    (*p).acc = (*s).sp as libc::c_int;
    (*s).loop_0 = p;
    return p;
}
unsafe extern "C" fn loop_break(mut s: *mut codegen_scope,
                                mut tree: *mut node) {
    if (*s).loop_0.is_null() {
        codegen(s, tree, 0i32);
        raise_error(s,
                    b"unexpected break\x00" as *const u8 as
                        *const libc::c_char);
    } else {
        let mut loop_0: *mut loopinfo = 0 as *mut loopinfo;
        let mut n: libc::c_int = 0i32;
        if !tree.is_null() { gen_retval(s, tree); }
        loop_0 = (*s).loop_0;
        while !loop_0.is_null() {
            if (*loop_0).type_0 as libc::c_uint ==
                   LOOP_BEGIN as libc::c_int as libc::c_uint {
                n += 1;
                loop_0 = (*loop_0).prev
            } else {
                if !((*loop_0).type_0 as libc::c_uint ==
                         LOOP_RESCUE as libc::c_int as libc::c_uint) {
                    break ;
                }
                loop_0 = (*loop_0).prev
            }
        }
        if loop_0.is_null() {
            raise_error(s,
                        b"unexpected break\x00" as *const u8 as
                            *const libc::c_char);
            return
        }
        if n > 0i32 {
            genop_1(s, OP_POPERR as libc::c_int as mrb_code, n as uint16_t);
        }
        if (*loop_0).type_0 as libc::c_uint ==
               LOOP_NORMAL as libc::c_int as libc::c_uint {
            let mut tmp: libc::c_int = 0;
            if (*s).ensure_level > (*(*s).loop_0).ensure_level {
                genop_1(s, OP_EPOP as libc::c_int as mrb_code,
                        ((*s).ensure_level - (*(*s).loop_0).ensure_level) as
                            uint16_t);
            }
            if !tree.is_null() {
                gen_move(s, (*loop_0).acc as uint16_t, (*s).sp, 0i32);
            }
            tmp =
                genjmp(s, OP_JMP as libc::c_int as mrb_code,
                       (*loop_0).pc3 as uint16_t) as libc::c_int;
            (*loop_0).pc3 = tmp
        } else {
            if tree.is_null() {
                genop_1(s, OP_LOADNIL as libc::c_int as mrb_code, (*s).sp);
            }
            gen_return(s, OP_BREAK as libc::c_int as uint8_t, (*s).sp);
        }
    };
}
unsafe extern "C" fn loop_pop(mut s: *mut codegen_scope,
                              mut val: libc::c_int) {
    if 0 != val {
        genop_1(s, OP_LOADNIL as libc::c_int as mrb_code, (*s).sp);
    }
    dispatch_linked(s, (*(*s).loop_0).pc3 as uint16_t);
    (*s).loop_0 = (*(*s).loop_0).prev;
    if 0 != val { push_n_(s, 1i32); };
}
unsafe extern "C" fn generate_code(mut mrb: *mut mrb_state,
                                   mut p: *mut parser_state,
                                   mut val: libc::c_int) -> *mut RProc {
    let mut scope: *mut codegen_scope =
        scope_new(mrb, 0 as *mut codegen_scope, 0 as *mut node);
    let mut proc_0: *mut RProc = 0 as *mut RProc;
    let mut prev_jmp: *mut mrb_jmpbuf = (*mrb).jmp;
    if scope.is_null() { return 0 as *mut RProc }
    (*scope).mrb = mrb;
    (*scope).parser = p;
    (*scope).filename_sym = (*p).filename_sym;
    (*scope).filename_index = (*p).current_filename_index;
    if _setjmp((*scope).jmp.impl_0.as_mut_ptr()) == 0i32 {
        (*mrb).jmp = &mut (*scope).jmp;
        /* prepare irep */
        codegen(scope, (*p).tree, val);
        proc_0 = mrb_proc_new(mrb, (*scope).irep);
        mrb_irep_decref(mrb, (*scope).irep);
        mrb_pool_close((*scope).mpool);
        (*proc_0).c = 0 as *mut RClass;
        if !(*(*mrb).c).cibase.is_null() &&
               (*(*(*mrb).c).cibase).proc_0 == (*proc_0).upper {
            (*proc_0).upper = 0 as *mut RProc
        }
        (*mrb).jmp = prev_jmp;
        return proc_0
    } else {
        mrb_irep_decref(mrb, (*scope).irep);
        mrb_pool_close((*scope).mpool);
        (*mrb).jmp = prev_jmp;
        return 0 as *mut RProc
    };
}
#[no_mangle]
pub unsafe extern "C" fn mrb_generate_code(mut mrb: *mut mrb_state,
                                           mut p: *mut parser_state)
 -> *mut RProc {
    return generate_code(mrb, p, 1i32);
}
#[no_mangle]
pub unsafe extern "C" fn mrb_irep_remove_lv(mut mrb: *mut mrb_state,
                                            mut irep: *mut mrb_irep) {
    let mut i: libc::c_int = 0;
    if !(*irep).lv.is_null() {
        mrb_free(mrb, (*irep).lv as *mut libc::c_void);
        (*irep).lv = 0 as *mut mrb_locals
    }
    i = 0i32;
    while i < (*irep).rlen as libc::c_int {
        mrb_irep_remove_lv(mrb, *(*irep).reps.offset(i as isize));
        i += 1
    };
}
/* instruction sizes */
#[no_mangle]
pub static mut mrb_insn_size: [uint8_t; 104] =
    [1i32 as uint8_t, 3i32 as uint8_t, 3i32 as uint8_t, 3i32 as uint8_t,
     3i32 as uint8_t, 2i32 as uint8_t, 2i32 as uint8_t, 2i32 as uint8_t,
     2i32 as uint8_t, 2i32 as uint8_t, 2i32 as uint8_t, 2i32 as uint8_t,
     2i32 as uint8_t, 2i32 as uint8_t, 3i32 as uint8_t, 2i32 as uint8_t,
     2i32 as uint8_t, 2i32 as uint8_t, 2i32 as uint8_t, 3i32 as uint8_t,
     3i32 as uint8_t, 3i32 as uint8_t, 3i32 as uint8_t, 3i32 as uint8_t,
     3i32 as uint8_t, 3i32 as uint8_t, 3i32 as uint8_t, 3i32 as uint8_t,
     3i32 as uint8_t, 3i32 as uint8_t, 3i32 as uint8_t, 4i32 as uint8_t,
     4i32 as uint8_t, 3i32 as uint8_t, 4i32 as uint8_t, 4i32 as uint8_t,
     4i32 as uint8_t, 3i32 as uint8_t, 2i32 as uint8_t, 3i32 as uint8_t,
     2i32 as uint8_t, 2i32 as uint8_t, 2i32 as uint8_t, 2i32 as uint8_t,
     3i32 as uint8_t, 3i32 as uint8_t, 4i32 as uint8_t, 4i32 as uint8_t,
     1i32 as uint8_t, 3i32 as uint8_t, 4i32 as uint8_t, 4i32 as uint8_t,
     3i32 as uint8_t, 1i32 as uint8_t, 3i32 as uint8_t, 2i32 as uint8_t,
     2i32 as uint8_t, 2i32 as uint8_t, 4i32 as uint8_t, 2i32 as uint8_t,
     3i32 as uint8_t, 2i32 as uint8_t, 3i32 as uint8_t, 2i32 as uint8_t,
     2i32 as uint8_t, 2i32 as uint8_t, 2i32 as uint8_t, 2i32 as uint8_t,
     2i32 as uint8_t, 2i32 as uint8_t, 3i32 as uint8_t, 4i32 as uint8_t,
     2i32 as uint8_t, 2i32 as uint8_t, 2i32 as uint8_t, 4i32 as uint8_t,
     4i32 as uint8_t, 4i32 as uint8_t, 2i32 as uint8_t, 3i32 as uint8_t,
     2i32 as uint8_t, 3i32 as uint8_t, 3i32 as uint8_t, 2i32 as uint8_t,
     3i32 as uint8_t, 3i32 as uint8_t, 3i32 as uint8_t, 2i32 as uint8_t,
     2i32 as uint8_t, 2i32 as uint8_t, 3i32 as uint8_t, 3i32 as uint8_t,
     3i32 as uint8_t, 3i32 as uint8_t, 3i32 as uint8_t, 2i32 as uint8_t,
     2i32 as uint8_t, 2i32 as uint8_t, 4i32 as uint8_t, 2i32 as uint8_t,
     1i32 as uint8_t, 1i32 as uint8_t, 1i32 as uint8_t, 1i32 as uint8_t];
/* EXT1 instruction sizes */
#[no_mangle]
pub static mut mrb_insn_size1: [uint8_t; 104] =
    [1i32 as uint8_t, 4i32 as uint8_t, 4i32 as uint8_t, 4i32 as uint8_t,
     4i32 as uint8_t, 3i32 as uint8_t, 3i32 as uint8_t, 3i32 as uint8_t,
     3i32 as uint8_t, 3i32 as uint8_t, 3i32 as uint8_t, 3i32 as uint8_t,
     3i32 as uint8_t, 3i32 as uint8_t, 4i32 as uint8_t, 3i32 as uint8_t,
     3i32 as uint8_t, 3i32 as uint8_t, 3i32 as uint8_t, 4i32 as uint8_t,
     4i32 as uint8_t, 4i32 as uint8_t, 4i32 as uint8_t, 4i32 as uint8_t,
     4i32 as uint8_t, 4i32 as uint8_t, 4i32 as uint8_t, 4i32 as uint8_t,
     4i32 as uint8_t, 4i32 as uint8_t, 4i32 as uint8_t, 5i32 as uint8_t,
     5i32 as uint8_t, 3i32 as uint8_t, 5i32 as uint8_t, 5i32 as uint8_t,
     5i32 as uint8_t, 3i32 as uint8_t, 3i32 as uint8_t, 4i32 as uint8_t,
     3i32 as uint8_t, 3i32 as uint8_t, 3i32 as uint8_t, 3i32 as uint8_t,
     4i32 as uint8_t, 4i32 as uint8_t, 5i32 as uint8_t, 5i32 as uint8_t,
     1i32 as uint8_t, 4i32 as uint8_t, 5i32 as uint8_t, 4i32 as uint8_t,
     4i32 as uint8_t, 1i32 as uint8_t, 4i32 as uint8_t, 3i32 as uint8_t,
     3i32 as uint8_t, 3i32 as uint8_t, 5i32 as uint8_t, 3i32 as uint8_t,
     4i32 as uint8_t, 3i32 as uint8_t, 4i32 as uint8_t, 3i32 as uint8_t,
     3i32 as uint8_t, 3i32 as uint8_t, 3i32 as uint8_t, 3i32 as uint8_t,
     3i32 as uint8_t, 3i32 as uint8_t, 4i32 as uint8_t, 5i32 as uint8_t,
     3i32 as uint8_t, 3i32 as uint8_t, 3i32 as uint8_t, 5i32 as uint8_t,
     5i32 as uint8_t, 5i32 as uint8_t, 3i32 as uint8_t, 4i32 as uint8_t,
     3i32 as uint8_t, 4i32 as uint8_t, 4i32 as uint8_t, 3i32 as uint8_t,
     4i32 as uint8_t, 4i32 as uint8_t, 4i32 as uint8_t, 3i32 as uint8_t,
     3i32 as uint8_t, 3i32 as uint8_t, 4i32 as uint8_t, 4i32 as uint8_t,
     4i32 as uint8_t, 4i32 as uint8_t, 4i32 as uint8_t, 3i32 as uint8_t,
     3i32 as uint8_t, 3i32 as uint8_t, 5i32 as uint8_t, 3i32 as uint8_t,
     1i32 as uint8_t, 1i32 as uint8_t, 1i32 as uint8_t, 1i32 as uint8_t];
/* EXT2 instruction sizes */
#[no_mangle]
pub static mut mrb_insn_size2: [uint8_t; 104] =
    [1i32 as uint8_t, 4i32 as uint8_t, 4i32 as uint8_t, 4i32 as uint8_t,
     4i32 as uint8_t, 2i32 as uint8_t, 2i32 as uint8_t, 2i32 as uint8_t,
     2i32 as uint8_t, 2i32 as uint8_t, 2i32 as uint8_t, 2i32 as uint8_t,
     2i32 as uint8_t, 2i32 as uint8_t, 4i32 as uint8_t, 2i32 as uint8_t,
     2i32 as uint8_t, 2i32 as uint8_t, 2i32 as uint8_t, 4i32 as uint8_t,
     4i32 as uint8_t, 4i32 as uint8_t, 4i32 as uint8_t, 4i32 as uint8_t,
     4i32 as uint8_t, 4i32 as uint8_t, 4i32 as uint8_t, 4i32 as uint8_t,
     4i32 as uint8_t, 4i32 as uint8_t, 4i32 as uint8_t, 5i32 as uint8_t,
     5i32 as uint8_t, 3i32 as uint8_t, 5i32 as uint8_t, 5i32 as uint8_t,
     5i32 as uint8_t, 3i32 as uint8_t, 2i32 as uint8_t, 4i32 as uint8_t,
     2i32 as uint8_t, 2i32 as uint8_t, 2i32 as uint8_t, 2i32 as uint8_t,
     4i32 as uint8_t, 4i32 as uint8_t, 5i32 as uint8_t, 5i32 as uint8_t,
     1i32 as uint8_t, 4i32 as uint8_t, 5i32 as uint8_t, 4i32 as uint8_t,
     4i32 as uint8_t, 1i32 as uint8_t, 4i32 as uint8_t, 2i32 as uint8_t,
     2i32 as uint8_t, 2i32 as uint8_t, 5i32 as uint8_t, 2i32 as uint8_t,
     4i32 as uint8_t, 2i32 as uint8_t, 4i32 as uint8_t, 2i32 as uint8_t,
     2i32 as uint8_t, 2i32 as uint8_t, 2i32 as uint8_t, 2i32 as uint8_t,
     2i32 as uint8_t, 2i32 as uint8_t, 4i32 as uint8_t, 5i32 as uint8_t,
     2i32 as uint8_t, 2i32 as uint8_t, 2i32 as uint8_t, 5i32 as uint8_t,
     5i32 as uint8_t, 5i32 as uint8_t, 2i32 as uint8_t, 4i32 as uint8_t,
     2i32 as uint8_t, 4i32 as uint8_t, 4i32 as uint8_t, 2i32 as uint8_t,
     4i32 as uint8_t, 4i32 as uint8_t, 4i32 as uint8_t, 2i32 as uint8_t,
     2i32 as uint8_t, 2i32 as uint8_t, 4i32 as uint8_t, 4i32 as uint8_t,
     4i32 as uint8_t, 4i32 as uint8_t, 4i32 as uint8_t, 2i32 as uint8_t,
     2i32 as uint8_t, 2i32 as uint8_t, 5i32 as uint8_t, 2i32 as uint8_t,
     1i32 as uint8_t, 1i32 as uint8_t, 1i32 as uint8_t, 1i32 as uint8_t];
/* EXT3 instruction sizes */
#[no_mangle]
pub static mut mrb_insn_size3: [uint8_t; 104] =
    [1i32 as uint8_t, 5i32 as uint8_t, 5i32 as uint8_t, 5i32 as uint8_t,
     5i32 as uint8_t, 2i32 as uint8_t, 2i32 as uint8_t, 2i32 as uint8_t,
     2i32 as uint8_t, 2i32 as uint8_t, 2i32 as uint8_t, 2i32 as uint8_t,
     2i32 as uint8_t, 2i32 as uint8_t, 5i32 as uint8_t, 2i32 as uint8_t,
     2i32 as uint8_t, 2i32 as uint8_t, 2i32 as uint8_t, 5i32 as uint8_t,
     5i32 as uint8_t, 5i32 as uint8_t, 5i32 as uint8_t, 5i32 as uint8_t,
     5i32 as uint8_t, 5i32 as uint8_t, 5i32 as uint8_t, 5i32 as uint8_t,
     5i32 as uint8_t, 5i32 as uint8_t, 5i32 as uint8_t, 6i32 as uint8_t,
     6i32 as uint8_t, 3i32 as uint8_t, 4i32 as uint8_t, 4i32 as uint8_t,
     4i32 as uint8_t, 3i32 as uint8_t, 2i32 as uint8_t, 5i32 as uint8_t,
     2i32 as uint8_t, 2i32 as uint8_t, 2i32 as uint8_t, 2i32 as uint8_t,
     5i32 as uint8_t, 5i32 as uint8_t, 6i32 as uint8_t, 6i32 as uint8_t,
     1i32 as uint8_t, 5i32 as uint8_t, 4i32 as uint8_t, 4i32 as uint8_t,
     5i32 as uint8_t, 1i32 as uint8_t, 5i32 as uint8_t, 2i32 as uint8_t,
     2i32 as uint8_t, 2i32 as uint8_t, 4i32 as uint8_t, 2i32 as uint8_t,
     5i32 as uint8_t, 2i32 as uint8_t, 5i32 as uint8_t, 2i32 as uint8_t,
     2i32 as uint8_t, 2i32 as uint8_t, 2i32 as uint8_t, 2i32 as uint8_t,
     2i32 as uint8_t, 2i32 as uint8_t, 5i32 as uint8_t, 6i32 as uint8_t,
     2i32 as uint8_t, 2i32 as uint8_t, 2i32 as uint8_t, 6i32 as uint8_t,
     6i32 as uint8_t, 6i32 as uint8_t, 2i32 as uint8_t, 5i32 as uint8_t,
     2i32 as uint8_t, 5i32 as uint8_t, 5i32 as uint8_t, 2i32 as uint8_t,
     5i32 as uint8_t, 5i32 as uint8_t, 5i32 as uint8_t, 2i32 as uint8_t,
     2i32 as uint8_t, 2i32 as uint8_t, 5i32 as uint8_t, 5i32 as uint8_t,
     5i32 as uint8_t, 5i32 as uint8_t, 5i32 as uint8_t, 2i32 as uint8_t,
     2i32 as uint8_t, 2i32 as uint8_t, 6i32 as uint8_t, 2i32 as uint8_t,
     1i32 as uint8_t, 1i32 as uint8_t, 1i32 as uint8_t, 1i32 as uint8_t];
unsafe extern "C" fn run_static_initializers() {
    codegen_scope_zero =
        {
            let mut init =
                scope{_pad: [0; 4],
                      ainfo_mscope: [0; 2],
                      _pad2: [0; 6],
                      _pad3: [0; 6],
                      _pad4: [0; 4],
                      _pad5: [0; 6],
                      _pad6: [0; 4],
                      mrb: 0 as *const mrb_state as *mut mrb_state,
                      mpool: 0 as *const mrb_pool as *mut mrb_pool,
                      jmp: mrb_jmpbuf{impl_0: [0; 37],},
                      prev: 0 as *const scope as *mut scope,
                      lv: 0 as *const node as *mut node,
                      sp: 0,
                      pc: 0,
                      lastpc: 0,
                      lastlabel: 0,
                      loop_0: 0 as *const loopinfo as *mut loopinfo,
                      ensure_level: 0,
                      filename_sym: 0,
                      lineno: 0,
                      iseq: 0 as *const mrb_code as *mut mrb_code,
                      lines: 0 as *const uint16_t as *mut uint16_t,
                      icapa: 0,
                      irep: 0 as *const mrb_irep as *mut mrb_irep,
                      pcapa: 0,
                      scapa: 0,
                      rcapa: 0,
                      nlocals: 0,
                      nregs: 0,
                      ai: 0,
                      debug_start_pos: 0,
                      filename_index: 0,
                      parser: 0 as *const parser_state as *mut parser_state,
                      rlev: 0,};
            init.set_ainfo(0);
            init.set_mscope(0);
            init
        }
}
#[used]
#[cfg_attr ( target_os = "linux" , link_section = ".init_array" )]
#[cfg_attr ( target_os = "windows" , link_section = ".CRT$XIB" )]
#[cfg_attr ( target_os = "macos" , link_section = "__DATA,__mod_init_func" )]
static INIT_ARRAY: [unsafe extern "C" fn(); 1] = [run_static_initializers];