use libc;
use c2rust_bitfields::BitfieldStruct;
extern "C" {
    pub type iv_tbl;
    pub type RClass;
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
    /* memory pool implementation */
    pub type mrb_pool;
    pub type mrb_shared_string;
    #[no_mangle]
    fn memcmp(_: *const libc::c_void, _: *const libc::c_void,
              _: libc::c_ulong) -> libc::c_int;
    #[no_mangle]
    fn memcpy(_: *mut libc::c_void, _: *const libc::c_void, _: libc::c_ulong)
     -> *mut libc::c_void;
    /* *
 * Gets a exception class.
 * @param [mrb_state*] mrb The current mruby state.
 * @param [const char *] name The name of the class.
 * @return [struct RClass *] A reference to the class.
*/
    #[no_mangle]
    fn mrb_exc_get(mrb: *mut mrb_state, name: *const libc::c_char)
     -> *mut RClass;
    #[no_mangle]
    fn mrb_intern(_: *mut mrb_state, _: *const libc::c_char, _: size_t)
     -> mrb_sym;
    #[no_mangle]
    fn mrb_intern_static(_: *mut mrb_state, _: *const libc::c_char, _: size_t)
     -> mrb_sym;
    #[no_mangle]
    fn mrb_malloc(_: *mut mrb_state, _: size_t) -> *mut libc::c_void;
    #[no_mangle]
    fn mrb_free(_: *mut mrb_state, _: *mut libc::c_void);
    #[no_mangle]
    fn mrb_str_new(mrb: *mut mrb_state, p: *const libc::c_char, len: size_t)
     -> mrb_value;
    #[no_mangle]
    fn mrb_str_new_static(mrb: *mut mrb_state, p: *const libc::c_char,
                          len: size_t) -> mrb_value;
    #[no_mangle]
    fn mrb_top_self(_: *mut mrb_state) -> mrb_value;
    #[no_mangle]
    fn mrb_top_run(_: *mut mrb_state, _: *mut RProc, _: mrb_value,
                   _: libc::c_uint) -> mrb_value;
    /* * @internal crc.c */
    #[no_mangle]
    fn calc_crc_16_ccitt(src: *const uint8_t, nbytes: size_t, crc: uint16_t)
     -> uint16_t;
    #[no_mangle]
    fn mrb_add_irep(mrb: *mut mrb_state) -> *mut mrb_irep;
    #[no_mangle]
    fn mrb_irep_decref(_: *mut mrb_state, _: *mut mrb_irep);
    /* aspec access */
    #[no_mangle]
    fn mrb_proc_new(_: *mut mrb_state, _: *mut mrb_irep) -> *mut RProc;
    #[no_mangle]
    fn mrb_str_to_inum(mrb: *mut mrb_state, str: mrb_value, base: mrb_int,
                       badcheck: mrb_bool) -> mrb_value;
    #[no_mangle]
    fn mrb_str_to_dbl(mrb: *mut mrb_state, str: mrb_value, badcheck: mrb_bool)
     -> libc::c_double;
    #[no_mangle]
    fn mrb_str_pool(mrb: *mut mrb_state, str: mrb_value) -> mrb_value;
    #[no_mangle]
    fn mrb_exc_new_str(mrb: *mut mrb_state, c: *mut RClass, str: mrb_value)
     -> mrb_value;
    #[no_mangle]
    fn mrb_exc_set(mrb: *mut mrb_state, exc: mrb_value);
    #[no_mangle]
    fn mrb_codedump_all(_: *mut mrb_state, _: *mut RProc);
}
pub type __darwin_intptr_t = libc::c_long;
pub type __darwin_ptrdiff_t = libc::c_long;
pub type __darwin_size_t = libc::c_ulong;
pub type int64_t = libc::c_longlong;
pub type intptr_t = __darwin_intptr_t;
pub type size_t = __darwin_size_t;
pub type uint8_t = libc::c_uchar;
pub type uint16_t = libc::c_ushort;
pub type uint32_t = libc::c_uint;
pub type ptrdiff_t = __darwin_ptrdiff_t;
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
    pub body: C2RustUnnamed_1,
    pub upper: *mut RProc,
    pub e: C2RustUnnamed,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub union C2RustUnnamed {
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
#[derive ( Copy , Clone )]
#[repr ( C )]
pub union C2RustUnnamed_1 {
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
    pub lines: C2RustUnnamed_2,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub union C2RustUnnamed_2 {
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
/* section header */
#[derive ( Copy , Clone )]
#[repr(C)]
pub struct rite_section_header {
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
pub struct C2RustUnnamed_3 {
    pub len: mrb_int,
    pub aux: C2RustUnnamed_4,
    pub ptr: *mut libc::c_char,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub union C2RustUnnamed_4 {
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
    pub as_0: C2RustUnnamed_5,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub union C2RustUnnamed_5 {
    pub heap: C2RustUnnamed_3,
    pub ary: [libc::c_char; 24],
}
#[derive ( Copy , Clone )]
#[repr(C)]
pub struct rite_section_irep_header {
    pub section_ident: [uint8_t; 4],
    pub section_size: [uint8_t; 4],
    pub rite_version: [uint8_t; 4],
}
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
#[inline]
unsafe extern "C" fn bigendian_p() -> libc::c_int {
    let mut i: libc::c_int = 0;
    let mut p: *mut libc::c_char = 0 as *mut libc::c_char;
    i = 1i32;
    p = &mut i as *mut libc::c_int as *mut libc::c_char;
    return if 0 != *p.offset(0isize) as libc::c_int { 0i32 } else { 1i32 };
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
#[inline]
unsafe extern "C" fn bin_to_uint32(mut bin: *const uint8_t) -> uint32_t {
    return (*bin.offset(0isize) as uint32_t) << 24i32 |
               (*bin.offset(1isize) as uint32_t) << 16i32 |
               (*bin.offset(2isize) as uint32_t) << 8i32 |
               *bin.offset(3isize) as uint32_t;
}
#[inline]
unsafe extern "C" fn bin_to_uint16(mut bin: *const uint8_t) -> uint16_t {
    return ((*bin.offset(0isize) as uint16_t as libc::c_int) << 8i32 |
                *bin.offset(1isize) as uint16_t as libc::c_int) as uint16_t;
}
#[inline]
unsafe extern "C" fn bin_to_uint8(mut bin: *const uint8_t) -> uint8_t {
    return *bin.offset(0isize);
}
unsafe extern "C" fn skip_padding(mut buf: *const uint8_t) -> size_t {
    let align: size_t = ::std::mem::size_of::<uint32_t>() as libc::c_ulong;
    return -(buf as intptr_t) as libc::c_ulong &
               align.wrapping_sub(1i32 as libc::c_ulong);
}
unsafe extern "C" fn offset_crc_body() -> size_t {
    let mut header: rite_binary_header =
        rite_binary_header{binary_ident: [0; 4],
                           binary_version: [0; 4],
                           binary_crc: [0; 2],
                           binary_size: [0; 4],
                           compiler_name: [0; 4],
                           compiler_version: [0; 4],};
    return (header.binary_crc.as_mut_ptr().wrapping_offset_from(&mut header as
                                                                    *mut rite_binary_header
                                                                    as
                                                                    *mut uint8_t)
                as libc::c_long as
                libc::c_ulong).wrapping_add(::std::mem::size_of::<[uint8_t; 2]>()
                                                as libc::c_ulong);
}
unsafe extern "C" fn str_to_double(mut mrb: *mut mrb_state,
                                   mut str: mrb_value) -> libc::c_double {
    let mut p: *const libc::c_char =
        if 0 !=
               (*(str.value.p as *mut RString)).flags() as libc::c_int & 32i32
           {
            (*(str.value.p as *mut RString)).as_0.ary.as_mut_ptr()
        } else { (*(str.value.p as *mut RString)).as_0.heap.ptr };
    let mut len: mrb_int =
        if 0 !=
               (*(str.value.p as *mut RString)).flags() as libc::c_int & 32i32
           {
            (((*(str.value.p as *mut RString)).flags() as libc::c_int &
                  0x7c0i32) >> 6i32) as mrb_int
        } else { (*(str.value.p as *mut RString)).as_0.heap.len };
    /* `i`, `inf`, `infinity` */
    if len > 0i32 as libc::c_longlong &&
           *p.offset(0isize) as libc::c_int == 'i' as i32 {
        return ::std::f32::INFINITY as libc::c_double
    }
    /* `I`, `-inf`, `-infinity` */
    if *p.offset(0isize) as libc::c_int == 'I' as i32 ||
           len > 1i32 as libc::c_longlong &&
               *p.offset(0isize) as libc::c_int == '-' as i32 &&
               *p.offset(1isize) as libc::c_int == 'i' as i32 {
        return -::std::f32::INFINITY as libc::c_double
    }
    return mrb_str_to_dbl(mrb, str, 1i32 as mrb_bool);
}
unsafe extern "C" fn read_irep_record_1(mut mrb: *mut mrb_state,
                                        mut bin: *const uint8_t,
                                        mut len: *mut size_t,
                                        mut flags: uint8_t) -> *mut mrb_irep {
    let mut i: libc::c_int = 0;
    let mut src: *const uint8_t = bin;
    let mut diff: ptrdiff_t = 0;
    let mut tt: uint16_t = 0;
    let mut pool_data_len: uint16_t = 0;
    let mut snl: uint16_t = 0;
    let mut plen: libc::c_int = 0;
    let mut ai: libc::c_int = mrb_gc_arena_save(mrb);
    let mut irep: *mut mrb_irep = mrb_add_irep(mrb);
    /* skip record size */
    src =
        src.offset(::std::mem::size_of::<uint32_t>() as libc::c_ulong as
                       isize);
    /* number of local variable */
    (*irep).nlocals = bin_to_uint16(src);
    src =
        src.offset(::std::mem::size_of::<uint16_t>() as libc::c_ulong as
                       isize);
    /* number of register variable */
    (*irep).nregs = bin_to_uint16(src);
    src =
        src.offset(::std::mem::size_of::<uint16_t>() as libc::c_ulong as
                       isize);
    /* number of child irep */
    (*irep).rlen = bin_to_uint16(src) as size_t as uint16_t;
    src =
        src.offset(::std::mem::size_of::<uint16_t>() as libc::c_ulong as
                       isize);
    /* Binary Data Section */
  /* ISEQ BLOCK */
    (*irep).ilen = bin_to_uint32(src) as uint16_t;
    src =
        src.offset(::std::mem::size_of::<uint32_t>() as libc::c_ulong as
                       isize);
    src = src.offset(skip_padding(src) as isize);
    if (*irep).ilen as libc::c_int > 0i32 {
        if (*irep).ilen as size_t >
               18446744073709551615u64.wrapping_div(::std::mem::size_of::<mrb_code>()
                                                        as libc::c_ulong) {
            return 0 as *mut mrb_irep
        }
        if flags as libc::c_int & 1i32 == 0i32 &&
               0 != flags as libc::c_int & 8i32 {
            (*irep).iseq = src as *mut mrb_code;
            src =
                src.offset((::std::mem::size_of::<mrb_code>() as
                                libc::c_ulong).wrapping_mul((*irep).ilen as
                                                                libc::c_ulong)
                               as isize);
            (*irep).flags = ((*irep).flags as libc::c_int | 1i32) as uint8_t
        } else {
            let mut data_len: size_t =
                (::std::mem::size_of::<mrb_code>() as
                     libc::c_ulong).wrapping_mul((*irep).ilen as
                                                     libc::c_ulong);
            (*irep).iseq = mrb_malloc(mrb, data_len) as *mut mrb_code;
            memcpy((*irep).iseq as *mut libc::c_void,
                   src as *const libc::c_void, data_len);
            src = src.offset(data_len as isize)
        }
    }
    /* POOL BLOCK */
    /* number of pool */
    plen = bin_to_uint32(src) as libc::c_int;
    src =
        src.offset(::std::mem::size_of::<uint32_t>() as libc::c_ulong as
                       isize);
    if plen > 0i32 {
        if plen as size_t >
               18446744073709551615u64.wrapping_div(::std::mem::size_of::<mrb_value>()
                                                        as libc::c_ulong) {
            return 0 as *mut mrb_irep
        }
        (*irep).pool =
            mrb_malloc(mrb,
                       (::std::mem::size_of::<mrb_value>() as
                            libc::c_ulong).wrapping_mul(plen as
                                                            libc::c_ulong)) as
                *mut mrb_value;
        i = 0i32;
        while i < plen {
            let mut s: mrb_value =
                mrb_value{value: C2RustUnnamed_0{f: 0.,}, tt: MRB_TT_FALSE,};
            /* pool TT */
            let fresh0 = src;
            src = src.offset(1);
            tt = *fresh0 as uint16_t;
            /* pool data length */
            pool_data_len = bin_to_uint16(src);
            src =
                src.offset(::std::mem::size_of::<uint16_t>() as libc::c_ulong
                               as isize);
            if 0 != flags as libc::c_int & 1i32 {
                s =
                    mrb_str_new(mrb, src as *mut libc::c_char,
                                pool_data_len as size_t)
            } else {
                s =
                    mrb_str_new_static(mrb, src as *mut libc::c_char,
                                       pool_data_len as size_t)
            }
            src = src.offset(pool_data_len as libc::c_int as isize);
            match tt as libc::c_int {
                1 => {
                    let mut num: mrb_value =
                        mrb_str_to_inum(mrb, s, 10i32 as mrb_int,
                                        0i32 as mrb_bool);
                    *(*irep).pool.offset(i as isize) =
                        if num.tt as libc::c_uint ==
                               MRB_TT_FLOAT as libc::c_int as libc::c_uint {
                            mrb_float_value(mrb, num.value.f)
                        } else { num }
                }
                2 => {
                    *(*irep).pool.offset(i as isize) =
                        mrb_float_value(mrb, str_to_double(mrb, s))
                }
                0 => {
                    *(*irep).pool.offset(i as isize) = mrb_str_pool(mrb, s)
                }
                _ => {
                    /* should not happen */
                    *(*irep).pool.offset(i as isize) = mrb_nil_value()
                }
            }
            (*irep).plen = (*irep).plen.wrapping_add(1);
            mrb_gc_arena_restore(mrb, ai);
            i += 1
        }
    }
    /* SYMS BLOCK */
    /* syms length */
    (*irep).slen = bin_to_uint32(src) as uint16_t;
    src =
        src.offset(::std::mem::size_of::<uint32_t>() as libc::c_ulong as
                       isize);
    if (*irep).slen as libc::c_int > 0i32 {
        if (*irep).slen as size_t >
               18446744073709551615u64.wrapping_div(::std::mem::size_of::<mrb_sym>()
                                                        as libc::c_ulong) {
            return 0 as *mut mrb_irep
        }
        (*irep).syms =
            mrb_malloc(mrb,
                       (::std::mem::size_of::<mrb_sym>() as
                            libc::c_ulong).wrapping_mul((*irep).slen as
                                                            libc::c_ulong)) as
                *mut mrb_sym;
        i = 0i32;
        while i < (*irep).slen as libc::c_int {
            /* symbol name length */
            snl = bin_to_uint16(src);
            src =
                src.offset(::std::mem::size_of::<uint16_t>() as libc::c_ulong
                               as isize);
            if snl as libc::c_int == 0xffffi32 {
                *(*irep).syms.offset(i as isize) = 0i32 as mrb_sym
            } else {
                if 0 != flags as libc::c_int & 1i32 {
                    *(*irep).syms.offset(i as isize) =
                        mrb_intern(mrb, src as *mut libc::c_char,
                                   snl as size_t)
                } else {
                    *(*irep).syms.offset(i as isize) =
                        mrb_intern_static(mrb, src as *mut libc::c_char,
                                          snl as size_t)
                }
                src = src.offset((snl as libc::c_int + 1i32) as isize);
                mrb_gc_arena_restore(mrb, ai);
            }
            i += 1
        }
    }
    (*irep).reps =
        mrb_malloc(mrb,
                   (::std::mem::size_of::<*mut mrb_irep>() as
                        libc::c_ulong).wrapping_mul((*irep).rlen as
                                                        libc::c_ulong)) as
            *mut *mut mrb_irep;
    diff = src.wrapping_offset_from(bin) as libc::c_long;
    *len = diff as size_t;
    return irep;
}
unsafe extern "C" fn read_irep_record(mut mrb: *mut mrb_state,
                                      mut bin: *const uint8_t,
                                      mut len: *mut size_t,
                                      mut flags: uint8_t) -> *mut mrb_irep {
    let mut irep: *mut mrb_irep = read_irep_record_1(mrb, bin, len, flags);
    let mut i: libc::c_int = 0;
    if irep.is_null() { return 0 as *mut mrb_irep }
    bin = bin.offset(*len as isize);
    i = 0i32;
    while i < (*irep).rlen as libc::c_int {
        let mut rlen: size_t = 0;
        let ref mut fresh1 = *(*irep).reps.offset(i as isize);
        *fresh1 = read_irep_record(mrb, bin, &mut rlen, flags);
        if (*(*irep).reps.offset(i as isize)).is_null() {
            return 0 as *mut mrb_irep
        }
        bin = bin.offset(rlen as isize);
        *len = (*len as libc::c_ulong).wrapping_add(rlen) as size_t as size_t;
        i += 1
    }
    return irep;
}
unsafe extern "C" fn read_section_irep(mut mrb: *mut mrb_state,
                                       mut bin: *const uint8_t,
                                       mut flags: uint8_t) -> *mut mrb_irep {
    let mut len: size_t = 0;
    bin =
        bin.offset(::std::mem::size_of::<rite_section_irep_header>() as
                       libc::c_ulong as isize);
    return read_irep_record(mrb, bin, &mut len, flags);
}
unsafe extern "C" fn read_debug_record(mut mrb: *mut mrb_state,
                                       mut start: *const uint8_t,
                                       mut irep: *mut mrb_irep,
                                       mut record_len: *mut size_t,
                                       mut filenames: *const mrb_sym,
                                       mut filenames_len: size_t)
 -> libc::c_int {
    let mut bin: *const uint8_t = start;
    let mut diff: ptrdiff_t = 0;
    let mut record_size: size_t = 0;
    let mut f_idx: uint16_t = 0;
    let mut i: libc::c_int = 0;
    if !(*irep).debug_info.is_null() { return -6i32 }
    (*irep).debug_info =
        mrb_malloc(mrb,
                   ::std::mem::size_of::<mrb_irep_debug_info>() as
                       libc::c_ulong) as *mut mrb_irep_debug_info;
    (*(*irep).debug_info).pc_count = (*irep).ilen as uint32_t;
    record_size = bin_to_uint32(bin) as size_t;
    bin =
        bin.offset(::std::mem::size_of::<uint32_t>() as libc::c_ulong as
                       isize);
    (*(*irep).debug_info).flen = bin_to_uint16(bin);
    (*(*irep).debug_info).files =
        mrb_malloc(mrb,
                   (::std::mem::size_of::<*mut mrb_irep_debug_info>() as
                        libc::c_ulong).wrapping_mul((*(*irep).debug_info).flen
                                                        as libc::c_ulong)) as
            *mut *mut mrb_irep_debug_info_file;
    bin =
        bin.offset(::std::mem::size_of::<uint16_t>() as libc::c_ulong as
                       isize);
    f_idx = 0i32 as uint16_t;
    while (f_idx as libc::c_int) < (*(*irep).debug_info).flen as libc::c_int {
        let mut file: *mut mrb_irep_debug_info_file =
            0 as *mut mrb_irep_debug_info_file;
        let mut filename_idx: uint16_t = 0;
        file =
            mrb_malloc(mrb,
                       ::std::mem::size_of::<mrb_irep_debug_info_file>() as
                           libc::c_ulong) as *mut mrb_irep_debug_info_file;
        let ref mut fresh2 =
            *(*(*irep).debug_info).files.offset(f_idx as isize);
        *fresh2 = file;
        (*file).start_pos = bin_to_uint32(bin);
        bin =
            bin.offset(::std::mem::size_of::<uint32_t>() as libc::c_ulong as
                           isize);
        /* filename */
        filename_idx = bin_to_uint16(bin);
        bin =
            bin.offset(::std::mem::size_of::<uint16_t>() as libc::c_ulong as
                           isize);
        (*file).filename_sym = *filenames.offset(filename_idx as isize);
        (*file).line_entry_count = bin_to_uint32(bin);
        bin =
            bin.offset(::std::mem::size_of::<uint32_t>() as libc::c_ulong as
                           isize);
        (*file).line_type = bin_to_uint8(bin) as mrb_debug_line_type;
        bin =
            bin.offset(::std::mem::size_of::<uint8_t>() as libc::c_ulong as
                           isize);
        match (*file).line_type as libc::c_uint {
            0 => {
                let mut l: uint32_t = 0;
                (*file).lines.ary =
                    mrb_malloc(mrb,
                               (::std::mem::size_of::<uint16_t>() as
                                    libc::c_ulong).wrapping_mul((*file).line_entry_count
                                                                    as
                                                                    size_t))
                        as *mut uint16_t;
                l = 0i32 as uint32_t;
                while l < (*file).line_entry_count {
                    *(*file).lines.ary.offset(l as isize) =
                        bin_to_uint16(bin);
                    bin =
                        bin.offset(::std::mem::size_of::<uint16_t>() as
                                       libc::c_ulong as isize);
                    l = l.wrapping_add(1)
                }
            }
            1 => {
                let mut l_0: uint32_t = 0;
                (*file).lines.flat_map =
                    mrb_malloc(mrb,
                               (::std::mem::size_of::<mrb_irep_debug_info_line>()
                                    as
                                    libc::c_ulong).wrapping_mul((*file).line_entry_count
                                                                    as
                                                                    size_t))
                        as *mut mrb_irep_debug_info_line;
                l_0 = 0i32 as uint32_t;
                while l_0 < (*file).line_entry_count {
                    (*(*file).lines.flat_map.offset(l_0 as isize)).start_pos =
                        bin_to_uint32(bin);
                    bin =
                        bin.offset(::std::mem::size_of::<uint32_t>() as
                                       libc::c_ulong as isize);
                    (*(*file).lines.flat_map.offset(l_0 as isize)).line =
                        bin_to_uint16(bin);
                    bin =
                        bin.offset(::std::mem::size_of::<uint16_t>() as
                                       libc::c_ulong as isize);
                    l_0 = l_0.wrapping_add(1)
                }
            }
            _ => { return -1i32 }
        }
        f_idx = f_idx.wrapping_add(1)
    }
    diff = bin.wrapping_offset_from(start) as libc::c_long;
    if record_size != diff as size_t { return -1i32 }
    i = 0i32;
    while i < (*irep).rlen as libc::c_int {
        let mut len: size_t = 0;
        let mut ret: libc::c_int = 0;
        ret =
            read_debug_record(mrb, bin, *(*irep).reps.offset(i as isize),
                              &mut len, filenames, filenames_len);
        if ret != 0i32 { return ret }
        bin = bin.offset(len as isize);
        i += 1
    }
    diff = bin.wrapping_offset_from(start) as libc::c_long;
    *record_len = diff as size_t;
    return 0i32;
}
unsafe extern "C" fn read_section_debug(mut mrb: *mut mrb_state,
                                        mut start: *const uint8_t,
                                        mut irep: *mut mrb_irep,
                                        mut flags: uint8_t) -> libc::c_int {
    let mut bin: *const uint8_t = 0 as *const uint8_t;
    let mut diff: ptrdiff_t = 0;
    let mut header: *mut rite_section_debug_header =
        0 as *mut rite_section_debug_header;
    let mut i: uint16_t = 0;
    let mut len: size_t = 0i32 as size_t;
    let mut result: libc::c_int = 0;
    let mut filenames_len: uint16_t = 0;
    let mut filenames: *mut mrb_sym = 0 as *mut mrb_sym;
    bin = start;
    header = bin as *mut rite_section_debug_header;
    bin =
        bin.offset(::std::mem::size_of::<rite_section_debug_header>() as
                       libc::c_ulong as isize);
    filenames_len = bin_to_uint16(bin);
    bin =
        bin.offset(::std::mem::size_of::<uint16_t>() as libc::c_ulong as
                       isize);
    filenames =
        mrb_malloc(mrb,
                   (::std::mem::size_of::<mrb_sym>() as
                        libc::c_ulong).wrapping_mul(filenames_len as size_t))
            as *mut mrb_sym;
    i = 0i32 as uint16_t;
    while (i as libc::c_int) < filenames_len as libc::c_int {
        let mut f_len: uint16_t = bin_to_uint16(bin);
        bin =
            bin.offset(::std::mem::size_of::<uint16_t>() as libc::c_ulong as
                           isize);
        if 0 != flags as libc::c_int & 1i32 {
            *filenames.offset(i as isize) =
                mrb_intern(mrb, bin as *const libc::c_char, f_len as size_t)
        } else {
            *filenames.offset(i as isize) =
                mrb_intern_static(mrb, bin as *const libc::c_char,
                                  f_len as size_t)
        }
        bin = bin.offset(f_len as libc::c_int as isize);
        i = i.wrapping_add(1)
    }
    result =
        read_debug_record(mrb, bin, irep, &mut len, filenames,
                          filenames_len as size_t);
    if !(result != 0i32) {
        bin = bin.offset(len as isize);
        diff = bin.wrapping_offset_from(start) as libc::c_long;
        if diff as uint32_t !=
               bin_to_uint32((*header).section_size.as_mut_ptr()) {
            result = -1i32
        }
    }
    mrb_free(mrb, filenames as *mut libc::c_void);
    return result;
}
unsafe extern "C" fn read_lv_record(mut mrb: *mut mrb_state,
                                    mut start: *const uint8_t,
                                    mut irep: *mut mrb_irep,
                                    mut record_len: *mut size_t,
                                    mut syms: *const mrb_sym,
                                    mut syms_len: uint32_t) -> libc::c_int {
    let mut bin: *const uint8_t = start;
    let mut diff: ptrdiff_t = 0;
    let mut i: libc::c_int = 0;
    (*irep).lv =
        mrb_malloc(mrb,
                   (::std::mem::size_of::<mrb_locals>() as
                        libc::c_ulong).wrapping_mul(((*irep).nlocals as
                                                         libc::c_int - 1i32)
                                                        as libc::c_ulong)) as
            *mut mrb_locals;
    i = 0i32;
    while i + 1i32 < (*irep).nlocals as libc::c_int {
        let sym_idx: uint16_t = bin_to_uint16(bin);
        bin =
            bin.offset(::std::mem::size_of::<uint16_t>() as libc::c_ulong as
                           isize);
        if sym_idx as libc::c_int == 65535i32 {
            (*(*irep).lv.offset(i as isize)).name = 0i32 as mrb_sym;
            (*(*irep).lv.offset(i as isize)).r = 0i32 as uint16_t
        } else {
            if sym_idx as libc::c_uint >= syms_len { return -1i32 }
            (*(*irep).lv.offset(i as isize)).name =
                *syms.offset(sym_idx as isize);
            (*(*irep).lv.offset(i as isize)).r = bin_to_uint16(bin)
        }
        bin =
            bin.offset(::std::mem::size_of::<uint16_t>() as libc::c_ulong as
                           isize);
        i += 1
    }
    i = 0i32;
    while i < (*irep).rlen as libc::c_int {
        let mut len: size_t = 0;
        let mut ret: libc::c_int = 0;
        ret =
            read_lv_record(mrb, bin, *(*irep).reps.offset(i as isize),
                           &mut len, syms, syms_len);
        if ret != 0i32 { return ret }
        bin = bin.offset(len as isize);
        i += 1
    }
    diff = bin.wrapping_offset_from(start) as libc::c_long;
    *record_len = diff as size_t;
    return 0i32;
}
unsafe extern "C" fn read_section_lv(mut mrb: *mut mrb_state,
                                     mut start: *const uint8_t,
                                     mut irep: *mut mrb_irep,
                                     mut flags: uint8_t) -> libc::c_int {
    let mut bin: *const uint8_t = 0 as *const uint8_t;
    let mut diff: ptrdiff_t = 0;
    let mut header: *const rite_section_lv_header =
        0 as *const rite_section_lv_header;
    let mut i: uint32_t = 0;
    let mut len: size_t = 0i32 as size_t;
    let mut result: libc::c_int = 0;
    let mut syms_len: uint32_t = 0;
    let mut syms: *mut mrb_sym = 0 as *mut mrb_sym;
    let mut intern_func:
            Option<unsafe extern "C" fn(_: *mut mrb_state,
                                        _: *const libc::c_char, _: size_t)
                       -> mrb_sym> =
        if 0 != flags as libc::c_int & 1i32 {
            Some(mrb_intern as
                     unsafe extern "C" fn(_: *mut mrb_state,
                                          _: *const libc::c_char, _: size_t)
                         -> mrb_sym)
        } else {
            Some(mrb_intern_static as
                     unsafe extern "C" fn(_: *mut mrb_state,
                                          _: *const libc::c_char, _: size_t)
                         -> mrb_sym)
        };
    bin = start;
    header = bin as *const rite_section_lv_header;
    bin =
        bin.offset(::std::mem::size_of::<rite_section_lv_header>() as
                       libc::c_ulong as isize);
    syms_len = bin_to_uint32(bin);
    bin =
        bin.offset(::std::mem::size_of::<uint32_t>() as libc::c_ulong as
                       isize);
    syms =
        mrb_malloc(mrb,
                   (::std::mem::size_of::<mrb_sym>() as
                        libc::c_ulong).wrapping_mul(syms_len as size_t)) as
            *mut mrb_sym;
    i = 0i32 as uint32_t;
    while i < syms_len {
        let str_len: uint16_t = bin_to_uint16(bin);
        bin =
            bin.offset(::std::mem::size_of::<uint16_t>() as libc::c_ulong as
                           isize);
        *syms.offset(i as isize) =
            intern_func.expect("non-null function pointer")(mrb,
                                                            bin as
                                                                *const libc::c_char,
                                                            str_len as
                                                                size_t);
        bin = bin.offset(str_len as libc::c_int as isize);
        i = i.wrapping_add(1)
    }
    result = read_lv_record(mrb, bin, irep, &mut len, syms, syms_len);
    if !(result != 0i32) {
        bin = bin.offset(len as isize);
        diff = bin.wrapping_offset_from(start) as libc::c_long;
        if diff as uint32_t != bin_to_uint32((*header).section_size.as_ptr())
           {
            result = -1i32
        }
    }
    mrb_free(mrb, syms as *mut libc::c_void);
    return result;
}
unsafe extern "C" fn read_binary_header(mut bin: *const uint8_t,
                                        mut bufsize: size_t,
                                        mut bin_size: *mut size_t,
                                        mut crc: *mut uint16_t,
                                        mut flags: *mut uint8_t)
 -> libc::c_int {
    let mut header: *const rite_binary_header =
        bin as *const rite_binary_header;
    if bufsize < ::std::mem::size_of::<rite_binary_header>() as libc::c_ulong
       {
        return -3i32
    }
    if memcmp((*header).binary_ident.as_ptr() as *const libc::c_void,
              b"RITE\x00" as *const u8 as *const libc::c_char as
                  *const libc::c_void,
              ::std::mem::size_of::<[uint8_t; 4]>() as libc::c_ulong) == 0i32
       {
        if 0 != bigendian_p() {
            *flags = (*flags as libc::c_int | 8i32) as uint8_t
        } else { *flags = (*flags as libc::c_int | 2i32) as uint8_t }
    } else if memcmp((*header).binary_ident.as_ptr() as *const libc::c_void,
                     b"ETIR\x00" as *const u8 as *const libc::c_char as
                         *const libc::c_void,
                     ::std::mem::size_of::<[uint8_t; 4]>() as libc::c_ulong)
                  == 0i32 {
        if 0 != bigendian_p() {
            *flags = (*flags as libc::c_int | 4i32) as uint8_t
        } else { *flags = (*flags as libc::c_int | 8i32) as uint8_t }
    } else { return -5i32 }
    if memcmp((*header).binary_version.as_ptr() as *const libc::c_void,
              b"0006\x00" as *const u8 as *const libc::c_char as
                  *const libc::c_void,
              ::std::mem::size_of::<[uint8_t; 4]>() as libc::c_ulong) != 0i32
       {
        return -5i32
    }
    if !crc.is_null() { *crc = bin_to_uint16((*header).binary_crc.as_ptr()) }
    *bin_size = bin_to_uint32((*header).binary_size.as_ptr()) as size_t;
    if bufsize < *bin_size { return -3i32 }
    return 0i32;
}
unsafe extern "C" fn read_irep(mut mrb: *mut mrb_state,
                               mut bin: *const uint8_t, mut bufsize: size_t,
                               mut flags: uint8_t) -> *mut mrb_irep {
    let mut result: libc::c_int = 0;
    let mut irep: *mut mrb_irep = 0 as *mut mrb_irep;
    let mut section_header: *const rite_section_header =
        0 as *const rite_section_header;
    let mut crc: uint16_t = 0;
    let mut bin_size: size_t = 0i32 as size_t;
    let mut n: size_t = 0;
    if mrb.is_null() || bin.is_null() { return 0 as *mut mrb_irep }
    result =
        read_binary_header(bin, bufsize, &mut bin_size, &mut crc, &mut flags);
    if result != 0i32 { return 0 as *mut mrb_irep }
    n = offset_crc_body();
    if crc as libc::c_int !=
           calc_crc_16_ccitt(bin.offset(n as isize), bin_size.wrapping_sub(n),
                             0i32 as uint16_t) as libc::c_int {
        return 0 as *mut mrb_irep
    }
    bin =
        bin.offset(::std::mem::size_of::<rite_binary_header>() as
                       libc::c_ulong as isize);
    loop  {
        section_header = bin as *const rite_section_header;
        if memcmp((*section_header).section_ident.as_ptr() as
                      *const libc::c_void,
                  b"IREP\x00" as *const u8 as *const libc::c_char as
                      *const libc::c_void,
                  ::std::mem::size_of::<[uint8_t; 4]>() as libc::c_ulong) ==
               0i32 {
            irep = read_section_irep(mrb, bin, flags);
            if irep.is_null() { return 0 as *mut mrb_irep }
        } else if memcmp((*section_header).section_ident.as_ptr() as
                             *const libc::c_void,
                         b"DBG\x00\x00" as *const u8 as *const libc::c_char as
                             *const libc::c_void,
                         ::std::mem::size_of::<[uint8_t; 4]>() as
                             libc::c_ulong) == 0i32 {
            if irep.is_null() {
                /* corrupted data */
                return 0 as *mut mrb_irep
            }
            result = read_section_debug(mrb, bin, irep, flags);
            if result < 0i32 { return 0 as *mut mrb_irep }
        } else if memcmp((*section_header).section_ident.as_ptr() as
                             *const libc::c_void,
                         b"LVAR\x00" as *const u8 as *const libc::c_char as
                             *const libc::c_void,
                         ::std::mem::size_of::<[uint8_t; 4]>() as
                             libc::c_ulong) == 0i32 {
            if irep.is_null() { return 0 as *mut mrb_irep }
            result = read_section_lv(mrb, bin, irep, flags);
            if result < 0i32 { return 0 as *mut mrb_irep }
        }
        bin =
            bin.offset(bin_to_uint32((*section_header).section_size.as_ptr())
                           as isize);
        if !(memcmp((*section_header).section_ident.as_ptr() as
                        *const libc::c_void,
                    b"END\x00\x00" as *const u8 as *const libc::c_char as
                        *const libc::c_void,
                    ::std::mem::size_of::<[uint8_t; 4]>() as libc::c_ulong) !=
                 0i32) {
            break ;
        }
    }
    return irep;
}
#[no_mangle]
pub unsafe extern "C" fn mrb_read_irep(mut mrb: *mut mrb_state,
                                       mut bin: *const uint8_t)
 -> *mut mrb_irep {
    let mut flags: uint8_t = 0i32 as uint8_t;
    return read_irep(mrb, bin, -1i32 as size_t, flags);
}
#[no_mangle]
pub unsafe extern "C" fn mrb_read_irep_buf(mut mrb: *mut mrb_state,
                                           mut buf: *const libc::c_void,
                                           mut bufsize: size_t)
 -> *mut mrb_irep {
    return read_irep(mrb, buf as *const uint8_t, bufsize, 1i32 as uint8_t);
}
unsafe extern "C" fn irep_error(mut mrb: *mut mrb_state) {
    mrb_exc_set(mrb,
                mrb_exc_new_str(mrb,
                                mrb_exc_get(mrb,
                                            b"ScriptError\x00" as *const u8 as
                                                *const libc::c_char),
                                mrb_str_new_static(mrb,
                                                   b"irep load error\x00" as
                                                       *const u8 as
                                                       *const libc::c_char,
                                                   (::std::mem::size_of::<[libc::c_char; 16]>()
                                                        as
                                                        libc::c_ulong).wrapping_sub(1i32
                                                                                        as
                                                                                        libc::c_ulong))));
}
unsafe extern "C" fn load_irep(mut mrb: *mut mrb_state,
                               mut irep: *mut mrb_irep,
                               mut c: *mut mrbc_context) -> mrb_value {
    let mut proc_0: *mut RProc = 0 as *mut RProc;
    if irep.is_null() { irep_error(mrb); return mrb_nil_value() }
    proc_0 = mrb_proc_new(mrb, irep);
    (*proc_0).c = 0 as *mut RClass;
    mrb_irep_decref(mrb, irep);
    if !c.is_null() && 0 != (*c).dump_result() as libc::c_int {
        mrb_codedump_all(mrb, proc_0);
    }
    if !c.is_null() && 0 != (*c).no_exec() as libc::c_int {
        return mrb_obj_value(proc_0 as *mut libc::c_void)
    }
    return mrb_top_run(mrb, proc_0, mrb_top_self(mrb), 0i32 as libc::c_uint);
}
/* @param [const uint8_t*] irep code, expected as a literal */
#[no_mangle]
pub unsafe extern "C" fn mrb_load_irep_cxt(mut mrb: *mut mrb_state,
                                           mut bin: *const uint8_t,
                                           mut c: *mut mrbc_context)
 -> mrb_value {
    return load_irep(mrb, mrb_read_irep(mrb, bin), c);
}
/*
 * @param [const void*] irep code
 * @param [size_t] size of irep buffer. If -1 is given, it is considered unrestricted.
 */
#[no_mangle]
pub unsafe extern "C" fn mrb_load_irep_buf_cxt(mut mrb: *mut mrb_state,
                                               mut buf: *const libc::c_void,
                                               mut bufsize: size_t,
                                               mut c: *mut mrbc_context)
 -> mrb_value {
    return load_irep(mrb, mrb_read_irep_buf(mrb, buf, bufsize), c);
}
/* @param [const uint8_t*] irep code, expected as a literal */
#[no_mangle]
pub unsafe extern "C" fn mrb_load_irep(mut mrb: *mut mrb_state,
                                       mut bin: *const uint8_t) -> mrb_value {
    return mrb_load_irep_cxt(mrb, bin, 0 as *mut mrbc_context);
}
/*
 * @param [const void*] irep code
 * @param [size_t] size of irep buffer. If -1 is given, it is considered unrestricted.
 */
#[no_mangle]
pub unsafe extern "C" fn mrb_load_irep_buf(mut mrb: *mut mrb_state,
                                           mut buf: *const libc::c_void,
                                           mut bufsize: size_t) -> mrb_value {
    return mrb_load_irep_buf_cxt(mrb, buf, bufsize, 0 as *mut mrbc_context);
}