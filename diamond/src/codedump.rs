use libc;
use c2rust_bitfields::{BitfieldStruct};
extern "C" {
    pub type iv_tbl;
    pub type RClass;
    pub type symbol_name;
    pub type mrb_jmpbuf;
    pub type mrb_shared_string;
    #[no_mangle]
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
    #[no_mangle]
    fn mrb_sym2name(_: *mut mrb_state, _: mrb_sym) -> *const libc::c_char;
    #[no_mangle]
    fn mrb_str_new(mrb: *mut mrb_state, p: *const libc::c_char, len: size_t)
     -> mrb_value;
    #[no_mangle]
    fn mrb_inspect(mrb: *mut mrb_state, obj: mrb_value) -> mrb_value;
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
    fn mrb_str_dump(mrb: *mut mrb_state, str: mrb_value) -> mrb_value;
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
    pub body: unnamed_1,
    pub upper: *mut RProc,
    pub e: unnamed,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub union unnamed {
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
#[derive ( Copy , Clone )]
#[repr ( C )]
pub union unnamed_1 {
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
    pub lines: unnamed_2,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub union unnamed_2 {
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
    pub heap: unnamed_4,
    pub ary: [libc::c_char; 24],
}
#[derive ( Copy , Clone )]
#[repr(C)]
pub struct unnamed_4 {
    pub len: mrb_int,
    pub aux: unnamed_5,
    pub ptr: *mut libc::c_char,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub union unnamed_5 {
    pub capa: mrb_int,
    pub shared: *mut mrb_shared_string,
    pub fshared: *mut RString,
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
unsafe extern "C" fn print_r(mut mrb: *mut mrb_state, mut irep: *mut mrb_irep,
                             mut n: size_t) {
    let mut i: size_t = 0;
    if n == 0i32 as libc::c_ulong { return }
    i = 0i32 as size_t;
    while i.wrapping_add(1i32 as libc::c_ulong) <
              (*irep).nlocals as libc::c_ulong {
        if (*(*irep).lv.offset(i as isize)).r as libc::c_ulong == n {
            let mut sym: mrb_sym = (*(*irep).lv.offset(i as isize)).name;
            printf(b" R%d:%s\x00" as *const u8 as *const libc::c_char,
                   n as libc::c_int, mrb_sym2name(mrb, sym));
            break ;
        } else { i = i.wrapping_add(1) }
    };
}
unsafe extern "C" fn print_lv_a(mut mrb: *mut mrb_state,
                                mut irep: *mut mrb_irep, mut a: uint16_t) {
    if (*irep).lv.is_null() ||
           a as libc::c_int >= (*irep).nlocals as libc::c_int ||
           a as libc::c_int == 0i32 {
        printf(b"\n\x00" as *const u8 as *const libc::c_char);
        return
    }
    printf(b"\t;\x00" as *const u8 as *const libc::c_char);
    print_r(mrb, irep, a as size_t);
    printf(b"\n\x00" as *const u8 as *const libc::c_char);
}
unsafe extern "C" fn print_lv_ab(mut mrb: *mut mrb_state,
                                 mut irep: *mut mrb_irep, mut a: uint16_t,
                                 mut b: uint16_t) {
    if (*irep).lv.is_null() ||
           a as libc::c_int >= (*irep).nlocals as libc::c_int &&
               b as libc::c_int >= (*irep).nlocals as libc::c_int ||
           a as libc::c_int + b as libc::c_int == 0i32 {
        printf(b"\n\x00" as *const u8 as *const libc::c_char);
        return
    }
    printf(b"\t;\x00" as *const u8 as *const libc::c_char);
    if a as libc::c_int > 0i32 { print_r(mrb, irep, a as size_t); }
    if b as libc::c_int > 0i32 { print_r(mrb, irep, b as size_t); }
    printf(b"\n\x00" as *const u8 as *const libc::c_char);
}
unsafe extern "C" fn print_header(mut mrb: *mut mrb_state,
                                  mut irep: *mut mrb_irep, mut i: ptrdiff_t) {
    let mut line: int32_t = 0;
    line = mrb_debug_get_line(mrb, irep, i);
    if line < 0i32 {
        printf(b"      \x00" as *const u8 as *const libc::c_char);
    } else { printf(b"%5d \x00" as *const u8 as *const libc::c_char, line); }
    printf(b"%03d \x00" as *const u8 as *const libc::c_char,
           i as libc::c_int);
}
unsafe extern "C" fn codedump(mut mrb: *mut mrb_state,
                              mut irep: *mut mrb_irep) {
    let mut current_block: u64;
    let mut ai: libc::c_int = 0;
    let mut pc: *mut mrb_code = 0 as *mut mrb_code;
    let mut pcend: *mut mrb_code = 0 as *mut mrb_code;
    let mut ins: mrb_code = 0;
    let mut file: *const libc::c_char = 0 as *const libc::c_char;
    let mut next_file: *const libc::c_char = 0 as *const libc::c_char;
    if irep.is_null() { return }
    printf(b"irep %p nregs=%d nlocals=%d pools=%d syms=%d reps=%d iseq=%d\n\x00"
               as *const u8 as *const libc::c_char, irep as *mut libc::c_void,
           (*irep).nregs as libc::c_int, (*irep).nlocals as libc::c_int,
           (*irep).plen as libc::c_int, (*irep).slen as libc::c_int,
           (*irep).rlen as libc::c_int, (*irep).ilen as libc::c_int);
    if !(*irep).lv.is_null() {
        let mut i: libc::c_int = 0;
        printf(b"local variable names:\n\x00" as *const u8 as
                   *const libc::c_char);
        i = 1i32;
        while i < (*irep).nlocals as libc::c_int {
            let mut s: *const libc::c_char =
                mrb_sym2name(mrb,
                             (*(*irep).lv.offset((i - 1i32) as isize)).name);
            let mut n: libc::c_int =
                if 0 !=
                       (*(*irep).lv.offset((i - 1i32) as isize)).r as
                           libc::c_int {
                    (*(*irep).lv.offset((i - 1i32) as isize)).r as libc::c_int
                } else { i };
            printf(b"  R%d:%s\n\x00" as *const u8 as *const libc::c_char, n,
                   if !s.is_null() {
                       s
                   } else { b"\x00" as *const u8 as *const libc::c_char });
            i += 1
        }
    }
    pc = (*irep).iseq;
    pcend = pc.offset((*irep).ilen as libc::c_int as isize);
    while pc < pcend {
        let mut i_0: ptrdiff_t = 0;
        let mut a: uint32_t = 0;
        let mut b: uint16_t = 0;
        let mut c: uint8_t = 0;
        ai = mrb_gc_arena_save(mrb);
        i_0 = pc.wrapping_offset_from((*irep).iseq) as libc::c_long;
        next_file = mrb_debug_get_filename(mrb, irep, i_0);
        if !next_file.is_null() && file != next_file {
            printf(b"file: %s\n\x00" as *const u8 as *const libc::c_char,
                   next_file);
            file = next_file
        }
        print_header(mrb, irep, i_0);
        let fresh0 = pc;
        pc = pc.offset(1);
        ins = *fresh0;
        match ins as libc::c_int {
            0 => { current_block = 4153728562337695617; }
            1 => {
                let fresh1 = pc;
                pc = pc.offset(1);
                a = *fresh1 as uint32_t;
                let fresh2 = pc;
                pc = pc.offset(1);
                b = *fresh2 as uint16_t;
                current_block = 3345631096401374476;
            }
            2 => {
                let fresh3 = pc;
                pc = pc.offset(1);
                a = *fresh3 as uint32_t;
                let fresh4 = pc;
                pc = pc.offset(1);
                b = *fresh4 as uint16_t;
                current_block = 14904807116429336826;
            }
            3 => {
                let fresh5 = pc;
                pc = pc.offset(1);
                a = *fresh5 as uint32_t;
                let fresh6 = pc;
                pc = pc.offset(1);
                b = *fresh6 as uint16_t;
                current_block = 8703264613394043654;
            }
            4 => {
                let fresh7 = pc;
                pc = pc.offset(1);
                a = *fresh7 as uint32_t;
                let fresh8 = pc;
                pc = pc.offset(1);
                b = *fresh8 as uint16_t;
                current_block = 4843221280033581897;
            }
            5 => {
                let fresh9 = pc;
                pc = pc.offset(1);
                a = *fresh9 as uint32_t;
                current_block = 6276274620003476740;
            }
            6 => {
                let fresh10 = pc;
                pc = pc.offset(1);
                a = *fresh10 as uint32_t;
                current_block = 8096298871650922110;
            }
            7 => {
                let fresh11 = pc;
                pc = pc.offset(1);
                a = *fresh11 as uint32_t;
                current_block = 8096298871650922110;
            }
            8 => {
                let fresh12 = pc;
                pc = pc.offset(1);
                a = *fresh12 as uint32_t;
                current_block = 8096298871650922110;
            }
            9 => {
                let fresh13 = pc;
                pc = pc.offset(1);
                a = *fresh13 as uint32_t;
                current_block = 8096298871650922110;
            }
            10 => {
                let fresh14 = pc;
                pc = pc.offset(1);
                a = *fresh14 as uint32_t;
                current_block = 8096298871650922110;
            }
            11 => {
                let fresh15 = pc;
                pc = pc.offset(1);
                a = *fresh15 as uint32_t;
                current_block = 8096298871650922110;
            }
            12 => {
                let fresh16 = pc;
                pc = pc.offset(1);
                a = *fresh16 as uint32_t;
                current_block = 8096298871650922110;
            }
            13 => {
                let fresh17 = pc;
                pc = pc.offset(1);
                a = *fresh17 as uint32_t;
                current_block = 8096298871650922110;
            }
            14 => {
                let fresh18 = pc;
                pc = pc.offset(1);
                a = *fresh18 as uint32_t;
                let fresh19 = pc;
                pc = pc.offset(1);
                b = *fresh19 as uint16_t;
                current_block = 16157800273112125286;
            }
            15 => {
                let fresh20 = pc;
                pc = pc.offset(1);
                a = *fresh20 as uint32_t;
                current_block = 17940677572951814003;
            }
            16 => {
                let fresh21 = pc;
                pc = pc.offset(1);
                a = *fresh21 as uint32_t;
                current_block = 12915432016225878389;
            }
            17 => {
                let fresh22 = pc;
                pc = pc.offset(1);
                a = *fresh22 as uint32_t;
                current_block = 4255402116796946452;
            }
            18 => {
                let fresh23 = pc;
                pc = pc.offset(1);
                a = *fresh23 as uint32_t;
                current_block = 2779364454669598918;
            }
            19 => {
                let fresh24 = pc;
                pc = pc.offset(1);
                a = *fresh24 as uint32_t;
                let fresh25 = pc;
                pc = pc.offset(1);
                b = *fresh25 as uint16_t;
                current_block = 17386262169460468759;
            }
            20 => {
                let fresh26 = pc;
                pc = pc.offset(1);
                a = *fresh26 as uint32_t;
                let fresh27 = pc;
                pc = pc.offset(1);
                b = *fresh27 as uint16_t;
                current_block = 16200511029630275951;
            }
            21 => {
                let fresh28 = pc;
                pc = pc.offset(1);
                a = *fresh28 as uint32_t;
                let fresh29 = pc;
                pc = pc.offset(1);
                b = *fresh29 as uint16_t;
                current_block = 16758983331069857237;
            }
            22 => {
                let fresh30 = pc;
                pc = pc.offset(1);
                a = *fresh30 as uint32_t;
                let fresh31 = pc;
                pc = pc.offset(1);
                b = *fresh31 as uint16_t;
                current_block = 4481804007556508143;
            }
            27 => {
                let fresh32 = pc;
                pc = pc.offset(1);
                a = *fresh32 as uint32_t;
                let fresh33 = pc;
                pc = pc.offset(1);
                b = *fresh33 as uint16_t;
                current_block = 16034675043887719473;
            }
            28 => {
                let fresh34 = pc;
                pc = pc.offset(1);
                a = *fresh34 as uint32_t;
                let fresh35 = pc;
                pc = pc.offset(1);
                b = *fresh35 as uint16_t;
                current_block = 942271536672158721;
            }
            29 => {
                let fresh36 = pc;
                pc = pc.offset(1);
                a = *fresh36 as uint32_t;
                let fresh37 = pc;
                pc = pc.offset(1);
                b = *fresh37 as uint16_t;
                current_block = 1873199301525838551;
            }
            30 => {
                let fresh38 = pc;
                pc = pc.offset(1);
                a = *fresh38 as uint32_t;
                let fresh39 = pc;
                pc = pc.offset(1);
                b = *fresh39 as uint16_t;
                current_block = 9704183418455961811;
            }
            23 => {
                let fresh40 = pc;
                pc = pc.offset(1);
                a = *fresh40 as uint32_t;
                let fresh41 = pc;
                pc = pc.offset(1);
                b = *fresh41 as uint16_t;
                current_block = 7446123777345652171;
            }
            24 => {
                let fresh42 = pc;
                pc = pc.offset(1);
                a = *fresh42 as uint32_t;
                let fresh43 = pc;
                pc = pc.offset(1);
                b = *fresh43 as uint16_t;
                current_block = 7570639851576535195;
            }
            31 => {
                let fresh44 = pc;
                pc = pc.offset(1);
                a = *fresh44 as uint32_t;
                let fresh45 = pc;
                pc = pc.offset(1);
                b = *fresh45 as uint16_t;
                let fresh46 = pc;
                pc = pc.offset(1);
                c = *fresh46;
                current_block = 10547193261004346349;
            }
            32 => {
                let fresh47 = pc;
                pc = pc.offset(1);
                a = *fresh47 as uint32_t;
                let fresh48 = pc;
                pc = pc.offset(1);
                b = *fresh48 as uint16_t;
                let fresh49 = pc;
                pc = pc.offset(1);
                c = *fresh49;
                current_block = 9015561896112508021;
            }
            25 => {
                let fresh50 = pc;
                pc = pc.offset(1);
                a = *fresh50 as uint32_t;
                let fresh51 = pc;
                pc = pc.offset(1);
                b = *fresh51 as uint16_t;
                current_block = 1863480813282067938;
            }
            26 => {
                let fresh52 = pc;
                pc = pc.offset(1);
                a = *fresh52 as uint32_t;
                let fresh53 = pc;
                pc = pc.offset(1);
                b = *fresh53 as uint16_t;
                current_block = 7849445245878263537;
            }
            33 => {
                pc = pc.offset(2isize);
                a =
                    ((*pc.offset(-2isize).offset(0isize) as libc::c_int) <<
                         8i32 |
                         *pc.offset(-2isize).offset(1isize) as libc::c_int) as
                        uint32_t;
                current_block = 10458688773482078930;
            }
            34 => {
                let fresh54 = pc;
                pc = pc.offset(1);
                a = *fresh54 as uint32_t;
                pc = pc.offset(2isize);
                b =
                    ((*pc.offset(-2isize).offset(0isize) as libc::c_int) <<
                         8i32 |
                         *pc.offset(-2isize).offset(1isize) as libc::c_int) as
                        uint16_t;
                current_block = 9865490829578899964;
            }
            35 => {
                let fresh55 = pc;
                pc = pc.offset(1);
                a = *fresh55 as uint32_t;
                pc = pc.offset(2isize);
                b =
                    ((*pc.offset(-2isize).offset(0isize) as libc::c_int) <<
                         8i32 |
                         *pc.offset(-2isize).offset(1isize) as libc::c_int) as
                        uint16_t;
                current_block = 17639296353235453144;
            }
            36 => {
                let fresh56 = pc;
                pc = pc.offset(1);
                a = *fresh56 as uint32_t;
                pc = pc.offset(2isize);
                b =
                    ((*pc.offset(-2isize).offset(0isize) as libc::c_int) <<
                         8i32 |
                         *pc.offset(-2isize).offset(1isize) as libc::c_int) as
                        uint16_t;
                current_block = 12781735722417726073;
            }
            44 => {
                let fresh57 = pc;
                pc = pc.offset(1);
                a = *fresh57 as uint32_t;
                let fresh58 = pc;
                pc = pc.offset(1);
                b = *fresh58 as uint16_t;
                current_block = 12846811306833230447;
            }
            45 => {
                let fresh59 = pc;
                pc = pc.offset(1);
                a = *fresh59 as uint32_t;
                let fresh60 = pc;
                pc = pc.offset(1);
                b = *fresh60 as uint16_t;
                current_block = 10246497873717858537;
            }
            46 => {
                let fresh61 = pc;
                pc = pc.offset(1);
                a = *fresh61 as uint32_t;
                let fresh62 = pc;
                pc = pc.offset(1);
                b = *fresh62 as uint16_t;
                let fresh63 = pc;
                pc = pc.offset(1);
                c = *fresh63;
                current_block = 5602041749335279496;
            }
            47 => {
                let fresh64 = pc;
                pc = pc.offset(1);
                a = *fresh64 as uint32_t;
                let fresh65 = pc;
                pc = pc.offset(1);
                b = *fresh65 as uint16_t;
                let fresh66 = pc;
                pc = pc.offset(1);
                c = *fresh66;
                current_block = 3906616468301123675;
            }
            48 => { current_block = 11480039587962891479; }
            49 => {
                let fresh67 = pc;
                pc = pc.offset(1);
                a = *fresh67 as uint32_t;
                let fresh68 = pc;
                pc = pc.offset(1);
                b = *fresh68 as uint16_t;
                current_block = 5769007513321684282;
            }
            50 => {
                let fresh69 = pc;
                pc = pc.offset(1);
                a = *fresh69 as uint32_t;
                pc = pc.offset(2isize);
                b =
                    ((*pc.offset(-2isize).offset(0isize) as libc::c_int) <<
                         8i32 |
                         *pc.offset(-2isize).offset(1isize) as libc::c_int) as
                        uint16_t;
                current_block = 4063058694607173453;
            }
            51 => {
                pc = pc.offset(3isize);
                a =
                    ((*pc.offset(-3isize).offset(0isize) as libc::c_int) <<
                         16i32 |
                         (*pc.offset(-3isize).offset(1isize) as libc::c_int)
                             << 8i32 |
                         *pc.offset(-3isize).offset(2isize) as libc::c_int) as
                        uint32_t;
                current_block = 17974925641258900466;
            }
            52 => {
                let fresh70 = pc;
                pc = pc.offset(1);
                a = *fresh70 as uint32_t;
                let fresh71 = pc;
                pc = pc.offset(1);
                b = *fresh71 as uint16_t;
                current_block = 17792945831448561469;
            }
            53 => { current_block = 11667657158681136576; }
            54 => {
                let fresh72 = pc;
                pc = pc.offset(1);
                a = *fresh72 as uint32_t;
                let fresh73 = pc;
                pc = pc.offset(1);
                b = *fresh73 as uint16_t;
                current_block = 13480378041738100027;
            }
            55 => {
                let fresh74 = pc;
                pc = pc.offset(1);
                a = *fresh74 as uint32_t;
                current_block = 15446005597338724160;
            }
            56 => {
                let fresh75 = pc;
                pc = pc.offset(1);
                a = *fresh75 as uint32_t;
                current_block = 15158068222670743697;
            }
            57 => {
                let fresh76 = pc;
                pc = pc.offset(1);
                a = *fresh76 as uint32_t;
                current_block = 6968106866976024785;
            }
            58 => {
                let fresh77 = pc;
                pc = pc.offset(1);
                a = *fresh77 as uint32_t;
                pc = pc.offset(2isize);
                b =
                    ((*pc.offset(-2isize).offset(0isize) as libc::c_int) <<
                         8i32 |
                         *pc.offset(-2isize).offset(1isize) as libc::c_int) as
                        uint16_t;
                current_block = 5611126728812655874;
            }
            84 => {
                let fresh78 = pc;
                pc = pc.offset(1);
                a = *fresh78 as uint32_t;
                let fresh79 = pc;
                pc = pc.offset(1);
                b = *fresh79 as uint16_t;
                current_block = 3020430753860169586;
            }
            85 => {
                let fresh80 = pc;
                pc = pc.offset(1);
                a = *fresh80 as uint32_t;
                let fresh81 = pc;
                pc = pc.offset(1);
                b = *fresh81 as uint16_t;
                current_block = 11988973803779423327;
            }
            86 => {
                let fresh82 = pc;
                pc = pc.offset(1);
                a = *fresh82 as uint32_t;
                let fresh83 = pc;
                pc = pc.offset(1);
                b = *fresh83 as uint16_t;
                current_block = 10561944073379837955;
            }
            87 => {
                let fresh84 = pc;
                pc = pc.offset(1);
                a = *fresh84 as uint32_t;
                current_block = 9864403379770423142;
            }
            88 => {
                let fresh85 = pc;
                pc = pc.offset(1);
                a = *fresh85 as uint32_t;
                current_block = 8124361700674833375;
            }
            93 => {
                let fresh86 = pc;
                pc = pc.offset(1);
                a = *fresh86 as uint32_t;
                let fresh87 = pc;
                pc = pc.offset(1);
                b = *fresh87 as uint16_t;
                current_block = 8489059574810375089;
            }
            95 => {
                let fresh88 = pc;
                pc = pc.offset(1);
                a = *fresh88 as uint32_t;
                current_block = 12775348822761942368;
            }
            94 => {
                let fresh89 = pc;
                pc = pc.offset(1);
                a = *fresh89 as uint32_t;
                let fresh90 = pc;
                pc = pc.offset(1);
                b = *fresh90 as uint16_t;
                current_block = 12998570369541158573;
            }
            59 => {
                let fresh91 = pc;
                pc = pc.offset(1);
                a = *fresh91 as uint32_t;
                current_block = 328988732995390160;
            }
            60 => {
                let fresh92 = pc;
                pc = pc.offset(1);
                a = *fresh92 as uint32_t;
                let fresh93 = pc;
                pc = pc.offset(1);
                b = *fresh93 as uint16_t;
                current_block = 10541196509243133637;
            }
            61 => {
                let fresh94 = pc;
                pc = pc.offset(1);
                a = *fresh94 as uint32_t;
                current_block = 16989604180394837074;
            }
            62 => {
                let fresh95 = pc;
                pc = pc.offset(1);
                a = *fresh95 as uint32_t;
                let fresh96 = pc;
                pc = pc.offset(1);
                b = *fresh96 as uint16_t;
                current_block = 15326039268735274622;
            }
            63 => {
                let fresh97 = pc;
                pc = pc.offset(1);
                a = *fresh97 as uint32_t;
                current_block = 6846200202819738136;
            }
            64 => {
                let fresh98 = pc;
                pc = pc.offset(1);
                a = *fresh98 as uint32_t;
                current_block = 16948125840707872018;
            }
            66 => {
                let fresh99 = pc;
                pc = pc.offset(1);
                a = *fresh99 as uint32_t;
                current_block = 1975408140333322065;
            }
            67 => {
                let fresh100 = pc;
                pc = pc.offset(1);
                a = *fresh100 as uint32_t;
                current_block = 12696324532854772620;
            }
            68 => {
                let fresh101 = pc;
                pc = pc.offset(1);
                a = *fresh101 as uint32_t;
                current_block = 3433849069680490924;
            }
            69 => {
                let fresh102 = pc;
                pc = pc.offset(1);
                a = *fresh102 as uint32_t;
                current_block = 10072772881953407077;
            }
            65 => {
                let fresh103 = pc;
                pc = pc.offset(1);
                a = *fresh103 as uint32_t;
                current_block = 11372858789397587571;
            }
            70 => {
                let fresh104 = pc;
                pc = pc.offset(1);
                a = *fresh104 as uint32_t;
                let fresh105 = pc;
                pc = pc.offset(1);
                b = *fresh105 as uint16_t;
                current_block = 4850402844069062092;
            }
            71 => {
                let fresh106 = pc;
                pc = pc.offset(1);
                a = *fresh106 as uint32_t;
                let fresh107 = pc;
                pc = pc.offset(1);
                b = *fresh107 as uint16_t;
                let fresh108 = pc;
                pc = pc.offset(1);
                c = *fresh108;
                current_block = 1741746549607172898;
            }
            72 => {
                let fresh109 = pc;
                pc = pc.offset(1);
                a = *fresh109 as uint32_t;
                current_block = 6053772697959983332;
            }
            73 => {
                let fresh110 = pc;
                pc = pc.offset(1);
                a = *fresh110 as uint32_t;
                current_block = 18022542853155082387;
            }
            74 => {
                let fresh111 = pc;
                pc = pc.offset(1);
                a = *fresh111 as uint32_t;
                current_block = 1606585337627918712;
            }
            75 => {
                let fresh112 = pc;
                pc = pc.offset(1);
                a = *fresh112 as uint32_t;
                let fresh113 = pc;
                pc = pc.offset(1);
                b = *fresh113 as uint16_t;
                let fresh114 = pc;
                pc = pc.offset(1);
                c = *fresh114;
                current_block = 9599741646002396977;
            }
            76 => {
                let fresh115 = pc;
                pc = pc.offset(1);
                a = *fresh115 as uint32_t;
                let fresh116 = pc;
                pc = pc.offset(1);
                b = *fresh116 as uint16_t;
                let fresh117 = pc;
                pc = pc.offset(1);
                c = *fresh117;
                current_block = 11441496697127961056;
            }
            77 => {
                let fresh118 = pc;
                pc = pc.offset(1);
                a = *fresh118 as uint32_t;
                let fresh119 = pc;
                pc = pc.offset(1);
                b = *fresh119 as uint16_t;
                let fresh120 = pc;
                pc = pc.offset(1);
                c = *fresh120;
                current_block = 10671272775318305553;
            }
            78 => {
                let fresh121 = pc;
                pc = pc.offset(1);
                a = *fresh121 as uint32_t;
                current_block = 7127613333771968219;
            }
            79 => {
                let fresh122 = pc;
                pc = pc.offset(1);
                a = *fresh122 as uint32_t;
                let fresh123 = pc;
                pc = pc.offset(1);
                b = *fresh123 as uint16_t;
                current_block = 16677287981175421660;
            }
            80 => {
                let fresh124 = pc;
                pc = pc.offset(1);
                a = *fresh124 as uint32_t;
                current_block = 13433017329934546124;
            }
            81 => {
                let fresh125 = pc;
                pc = pc.offset(1);
                a = *fresh125 as uint32_t;
                let fresh126 = pc;
                pc = pc.offset(1);
                b = *fresh126 as uint16_t;
                current_block = 314002374245450226;
            }
            82 => {
                let fresh127 = pc;
                pc = pc.offset(1);
                a = *fresh127 as uint32_t;
                let fresh128 = pc;
                pc = pc.offset(1);
                b = *fresh128 as uint16_t;
                current_block = 5341326654522537507;
            }
            83 => {
                let fresh129 = pc;
                pc = pc.offset(1);
                a = *fresh129 as uint32_t;
                current_block = 14386882892843117929;
            }
            89 => {
                let fresh130 = pc;
                pc = pc.offset(1);
                a = *fresh130 as uint32_t;
                current_block = 2968065794317548828;
            }
            90 => {
                let fresh131 = pc;
                pc = pc.offset(1);
                a = *fresh131 as uint32_t;
                let fresh132 = pc;
                pc = pc.offset(1);
                b = *fresh132 as uint16_t;
                current_block = 3258098155381626199;
            }
            91 => {
                let fresh133 = pc;
                pc = pc.offset(1);
                a = *fresh133 as uint32_t;
                let fresh134 = pc;
                pc = pc.offset(1);
                b = *fresh134 as uint16_t;
                current_block = 7493503892657452062;
            }
            92 => {
                let fresh135 = pc;
                pc = pc.offset(1);
                a = *fresh135 as uint32_t;
                let fresh136 = pc;
                pc = pc.offset(1);
                b = *fresh136 as uint16_t;
                current_block = 2499490863002033965;
            }
            96 => {
                let fresh137 = pc;
                pc = pc.offset(1);
                a = *fresh137 as uint32_t;
                current_block = 355116111335096151;
            }
            97 => {
                let fresh138 = pc;
                pc = pc.offset(1);
                a = *fresh138 as uint32_t;
                current_block = 890063714143163519;
            }
            99 => {
                let fresh139 = pc;
                pc = pc.offset(1);
                a = *fresh139 as uint32_t;
                current_block = 7833862538762852372;
            }
            42 => {
                let fresh140 = pc;
                pc = pc.offset(1);
                a = *fresh140 as uint32_t;
                current_block = 1367206831037961075;
            }
            37 => {
                pc = pc.offset(2isize);
                a =
                    ((*pc.offset(-2isize).offset(0isize) as libc::c_int) <<
                         8i32 |
                         *pc.offset(-2isize).offset(1isize) as libc::c_int) as
                        uint32_t;
                current_block = 6898479631119937405;
            }
            38 => {
                let fresh141 = pc;
                pc = pc.offset(1);
                a = *fresh141 as uint32_t;
                current_block = 4275858079292575654;
            }
            39 => {
                let fresh142 = pc;
                pc = pc.offset(1);
                a = *fresh142 as uint32_t;
                let fresh143 = pc;
                pc = pc.offset(1);
                b = *fresh143 as uint16_t;
                current_block = 5105253204732552581;
            }
            41 => {
                let fresh144 = pc;
                pc = pc.offset(1);
                a = *fresh144 as uint32_t;
                current_block = 2384513197176890912;
            }
            40 => {
                let fresh145 = pc;
                pc = pc.offset(1);
                a = *fresh145 as uint32_t;
                current_block = 1459864638523139318;
            }
            43 => {
                let fresh146 = pc;
                pc = pc.offset(1);
                a = *fresh146 as uint32_t;
                current_block = 6367683481748506220;
            }
            98 => {
                let fresh147 = pc;
                pc = pc.offset(1);
                a = *fresh147 as uint32_t;
                let fresh148 = pc;
                pc = pc.offset(1);
                b = *fresh148 as uint16_t;
                let fresh149 = pc;
                pc = pc.offset(1);
                c = *fresh149;
                current_block = 998308582751651876;
            }
            103 => { current_block = 10942739336945693405; }
            100 => { current_block = 8667820544026455208; }
            101 => { current_block = 10575315580790317560; }
            102 => { current_block = 18239940301814930841; }
            _ => {
                printf(b"OP_unknown (0x%x)\n\x00" as *const u8 as
                           *const libc::c_char, ins as libc::c_int);
                current_block = 3935997918849240122;
            }
        }
        loop  {
            match current_block {
                3935997918849240122 => {
                    mrb_gc_arena_restore(mrb, ai);
                    break ;
                }
                10575315580790317560 => {
                    let fresh206 = pc;
                    pc = pc.offset(1);
                    ins = *fresh206;
                    printf(b"OP_EXT2\n\x00" as *const u8 as
                               *const libc::c_char);
                    print_header(mrb, irep,
                                 pc.wrapping_offset_from((*irep).iseq) as
                                     libc::c_long - 2i32 as libc::c_long);
                    match ins as libc::c_int {
                        0 => { current_block = 4153728562337695617; }
                        1 => {
                            let fresh207 = pc;
                            pc = pc.offset(1);
                            a = *fresh207 as uint32_t;
                            pc = pc.offset(2isize);
                            b =
                                ((*pc.offset(-2isize).offset(0isize) as
                                      libc::c_int) << 8i32 |
                                     *pc.offset(-2isize).offset(1isize) as
                                         libc::c_int) as uint16_t;
                            current_block = 3345631096401374476;
                        }
                        2 => {
                            let fresh208 = pc;
                            pc = pc.offset(1);
                            a = *fresh208 as uint32_t;
                            pc = pc.offset(2isize);
                            b =
                                ((*pc.offset(-2isize).offset(0isize) as
                                      libc::c_int) << 8i32 |
                                     *pc.offset(-2isize).offset(1isize) as
                                         libc::c_int) as uint16_t;
                            current_block = 14904807116429336826;
                        }
                        3 => {
                            let fresh209 = pc;
                            pc = pc.offset(1);
                            a = *fresh209 as uint32_t;
                            pc = pc.offset(2isize);
                            b =
                                ((*pc.offset(-2isize).offset(0isize) as
                                      libc::c_int) << 8i32 |
                                     *pc.offset(-2isize).offset(1isize) as
                                         libc::c_int) as uint16_t;
                            current_block = 8703264613394043654;
                        }
                        4 => {
                            let fresh210 = pc;
                            pc = pc.offset(1);
                            a = *fresh210 as uint32_t;
                            pc = pc.offset(2isize);
                            b =
                                ((*pc.offset(-2isize).offset(0isize) as
                                      libc::c_int) << 8i32 |
                                     *pc.offset(-2isize).offset(1isize) as
                                         libc::c_int) as uint16_t;
                            current_block = 4843221280033581897;
                        }
                        5 => {
                            let fresh211 = pc;
                            pc = pc.offset(1);
                            a = *fresh211 as uint32_t;
                            current_block = 6276274620003476740;
                        }
                        6 => {
                            let fresh212 = pc;
                            pc = pc.offset(1);
                            a = *fresh212 as uint32_t;
                            current_block = 8096298871650922110;
                        }
                        7 => {
                            let fresh213 = pc;
                            pc = pc.offset(1);
                            a = *fresh213 as uint32_t;
                            current_block = 8096298871650922110;
                        }
                        8 => {
                            let fresh214 = pc;
                            pc = pc.offset(1);
                            a = *fresh214 as uint32_t;
                            current_block = 8096298871650922110;
                        }
                        9 => {
                            let fresh215 = pc;
                            pc = pc.offset(1);
                            a = *fresh215 as uint32_t;
                            current_block = 8096298871650922110;
                        }
                        10 => {
                            let fresh216 = pc;
                            pc = pc.offset(1);
                            a = *fresh216 as uint32_t;
                            current_block = 8096298871650922110;
                        }
                        11 => {
                            let fresh217 = pc;
                            pc = pc.offset(1);
                            a = *fresh217 as uint32_t;
                            current_block = 8096298871650922110;
                        }
                        12 => {
                            let fresh218 = pc;
                            pc = pc.offset(1);
                            a = *fresh218 as uint32_t;
                            current_block = 8096298871650922110;
                        }
                        13 => {
                            let fresh219 = pc;
                            pc = pc.offset(1);
                            a = *fresh219 as uint32_t;
                            current_block = 8096298871650922110;
                        }
                        14 => {
                            let fresh220 = pc;
                            pc = pc.offset(1);
                            a = *fresh220 as uint32_t;
                            pc = pc.offset(2isize);
                            b =
                                ((*pc.offset(-2isize).offset(0isize) as
                                      libc::c_int) << 8i32 |
                                     *pc.offset(-2isize).offset(1isize) as
                                         libc::c_int) as uint16_t;
                            current_block = 16157800273112125286;
                        }
                        15 => {
                            let fresh221 = pc;
                            pc = pc.offset(1);
                            a = *fresh221 as uint32_t;
                            current_block = 17940677572951814003;
                        }
                        16 => {
                            let fresh222 = pc;
                            pc = pc.offset(1);
                            a = *fresh222 as uint32_t;
                            current_block = 12915432016225878389;
                        }
                        17 => {
                            let fresh223 = pc;
                            pc = pc.offset(1);
                            a = *fresh223 as uint32_t;
                            current_block = 4255402116796946452;
                        }
                        18 => {
                            let fresh224 = pc;
                            pc = pc.offset(1);
                            a = *fresh224 as uint32_t;
                            current_block = 2779364454669598918;
                        }
                        19 => {
                            let fresh225 = pc;
                            pc = pc.offset(1);
                            a = *fresh225 as uint32_t;
                            pc = pc.offset(2isize);
                            b =
                                ((*pc.offset(-2isize).offset(0isize) as
                                      libc::c_int) << 8i32 |
                                     *pc.offset(-2isize).offset(1isize) as
                                         libc::c_int) as uint16_t;
                            current_block = 17386262169460468759;
                        }
                        20 => {
                            let fresh226 = pc;
                            pc = pc.offset(1);
                            a = *fresh226 as uint32_t;
                            pc = pc.offset(2isize);
                            b =
                                ((*pc.offset(-2isize).offset(0isize) as
                                      libc::c_int) << 8i32 |
                                     *pc.offset(-2isize).offset(1isize) as
                                         libc::c_int) as uint16_t;
                            current_block = 16200511029630275951;
                        }
                        21 => {
                            let fresh227 = pc;
                            pc = pc.offset(1);
                            a = *fresh227 as uint32_t;
                            pc = pc.offset(2isize);
                            b =
                                ((*pc.offset(-2isize).offset(0isize) as
                                      libc::c_int) << 8i32 |
                                     *pc.offset(-2isize).offset(1isize) as
                                         libc::c_int) as uint16_t;
                            current_block = 16758983331069857237;
                        }
                        22 => {
                            let fresh228 = pc;
                            pc = pc.offset(1);
                            a = *fresh228 as uint32_t;
                            pc = pc.offset(2isize);
                            b =
                                ((*pc.offset(-2isize).offset(0isize) as
                                      libc::c_int) << 8i32 |
                                     *pc.offset(-2isize).offset(1isize) as
                                         libc::c_int) as uint16_t;
                            current_block = 4481804007556508143;
                        }
                        23 => {
                            let fresh229 = pc;
                            pc = pc.offset(1);
                            a = *fresh229 as uint32_t;
                            pc = pc.offset(2isize);
                            b =
                                ((*pc.offset(-2isize).offset(0isize) as
                                      libc::c_int) << 8i32 |
                                     *pc.offset(-2isize).offset(1isize) as
                                         libc::c_int) as uint16_t;
                            current_block = 7446123777345652171;
                        }
                        24 => {
                            let fresh230 = pc;
                            pc = pc.offset(1);
                            a = *fresh230 as uint32_t;
                            pc = pc.offset(2isize);
                            b =
                                ((*pc.offset(-2isize).offset(0isize) as
                                      libc::c_int) << 8i32 |
                                     *pc.offset(-2isize).offset(1isize) as
                                         libc::c_int) as uint16_t;
                            current_block = 7570639851576535195;
                        }
                        25 => {
                            let fresh231 = pc;
                            pc = pc.offset(1);
                            a = *fresh231 as uint32_t;
                            pc = pc.offset(2isize);
                            b =
                                ((*pc.offset(-2isize).offset(0isize) as
                                      libc::c_int) << 8i32 |
                                     *pc.offset(-2isize).offset(1isize) as
                                         libc::c_int) as uint16_t;
                            current_block = 1863480813282067938;
                        }
                        26 => {
                            let fresh232 = pc;
                            pc = pc.offset(1);
                            a = *fresh232 as uint32_t;
                            pc = pc.offset(2isize);
                            b =
                                ((*pc.offset(-2isize).offset(0isize) as
                                      libc::c_int) << 8i32 |
                                     *pc.offset(-2isize).offset(1isize) as
                                         libc::c_int) as uint16_t;
                            current_block = 7849445245878263537;
                        }
                        27 => {
                            let fresh233 = pc;
                            pc = pc.offset(1);
                            a = *fresh233 as uint32_t;
                            pc = pc.offset(2isize);
                            b =
                                ((*pc.offset(-2isize).offset(0isize) as
                                      libc::c_int) << 8i32 |
                                     *pc.offset(-2isize).offset(1isize) as
                                         libc::c_int) as uint16_t;
                            current_block = 16034675043887719473;
                        }
                        28 => {
                            let fresh234 = pc;
                            pc = pc.offset(1);
                            a = *fresh234 as uint32_t;
                            pc = pc.offset(2isize);
                            b =
                                ((*pc.offset(-2isize).offset(0isize) as
                                      libc::c_int) << 8i32 |
                                     *pc.offset(-2isize).offset(1isize) as
                                         libc::c_int) as uint16_t;
                            current_block = 942271536672158721;
                        }
                        29 => {
                            let fresh235 = pc;
                            pc = pc.offset(1);
                            a = *fresh235 as uint32_t;
                            pc = pc.offset(2isize);
                            b =
                                ((*pc.offset(-2isize).offset(0isize) as
                                      libc::c_int) << 8i32 |
                                     *pc.offset(-2isize).offset(1isize) as
                                         libc::c_int) as uint16_t;
                            current_block = 1873199301525838551;
                        }
                        30 => {
                            let fresh236 = pc;
                            pc = pc.offset(1);
                            a = *fresh236 as uint32_t;
                            pc = pc.offset(2isize);
                            b =
                                ((*pc.offset(-2isize).offset(0isize) as
                                      libc::c_int) << 8i32 |
                                     *pc.offset(-2isize).offset(1isize) as
                                         libc::c_int) as uint16_t;
                            current_block = 9704183418455961811;
                        }
                        31 => {
                            let fresh237 = pc;
                            pc = pc.offset(1);
                            a = *fresh237 as uint32_t;
                            pc = pc.offset(2isize);
                            b =
                                ((*pc.offset(-2isize).offset(0isize) as
                                      libc::c_int) << 8i32 |
                                     *pc.offset(-2isize).offset(1isize) as
                                         libc::c_int) as uint16_t;
                            let fresh238 = pc;
                            pc = pc.offset(1);
                            c = *fresh238;
                            current_block = 10547193261004346349;
                        }
                        32 => {
                            let fresh239 = pc;
                            pc = pc.offset(1);
                            a = *fresh239 as uint32_t;
                            pc = pc.offset(2isize);
                            b =
                                ((*pc.offset(-2isize).offset(0isize) as
                                      libc::c_int) << 8i32 |
                                     *pc.offset(-2isize).offset(1isize) as
                                         libc::c_int) as uint16_t;
                            let fresh240 = pc;
                            pc = pc.offset(1);
                            c = *fresh240;
                            current_block = 9015561896112508021;
                        }
                        33 => {
                            pc = pc.offset(2isize);
                            a =
                                ((*pc.offset(-2isize).offset(0isize) as
                                      libc::c_int) << 8i32 |
                                     *pc.offset(-2isize).offset(1isize) as
                                         libc::c_int) as uint32_t;
                            current_block = 10458688773482078930;
                        }
                        34 => {
                            let fresh241 = pc;
                            pc = pc.offset(1);
                            a = *fresh241 as uint32_t;
                            pc = pc.offset(2isize);
                            b =
                                ((*pc.offset(-2isize).offset(0isize) as
                                      libc::c_int) << 8i32 |
                                     *pc.offset(-2isize).offset(1isize) as
                                         libc::c_int) as uint16_t;
                            current_block = 9865490829578899964;
                        }
                        35 => {
                            let fresh242 = pc;
                            pc = pc.offset(1);
                            a = *fresh242 as uint32_t;
                            pc = pc.offset(2isize);
                            b =
                                ((*pc.offset(-2isize).offset(0isize) as
                                      libc::c_int) << 8i32 |
                                     *pc.offset(-2isize).offset(1isize) as
                                         libc::c_int) as uint16_t;
                            current_block = 17639296353235453144;
                        }
                        36 => {
                            let fresh243 = pc;
                            pc = pc.offset(1);
                            a = *fresh243 as uint32_t;
                            pc = pc.offset(2isize);
                            b =
                                ((*pc.offset(-2isize).offset(0isize) as
                                      libc::c_int) << 8i32 |
                                     *pc.offset(-2isize).offset(1isize) as
                                         libc::c_int) as uint16_t;
                            current_block = 12781735722417726073;
                        }
                        37 => {
                            pc = pc.offset(2isize);
                            a =
                                ((*pc.offset(-2isize).offset(0isize) as
                                      libc::c_int) << 8i32 |
                                     *pc.offset(-2isize).offset(1isize) as
                                         libc::c_int) as uint32_t;
                            current_block = 6898479631119937405;
                        }
                        38 => {
                            let fresh244 = pc;
                            pc = pc.offset(1);
                            a = *fresh244 as uint32_t;
                            current_block = 4275858079292575654;
                        }
                        39 => {
                            let fresh245 = pc;
                            pc = pc.offset(1);
                            a = *fresh245 as uint32_t;
                            pc = pc.offset(2isize);
                            b =
                                ((*pc.offset(-2isize).offset(0isize) as
                                      libc::c_int) << 8i32 |
                                     *pc.offset(-2isize).offset(1isize) as
                                         libc::c_int) as uint16_t;
                            current_block = 5105253204732552581;
                        }
                        40 => {
                            let fresh246 = pc;
                            pc = pc.offset(1);
                            a = *fresh246 as uint32_t;
                            current_block = 1459864638523139318;
                        }
                        41 => {
                            let fresh247 = pc;
                            pc = pc.offset(1);
                            a = *fresh247 as uint32_t;
                            current_block = 2384513197176890912;
                        }
                        42 => {
                            let fresh248 = pc;
                            pc = pc.offset(1);
                            a = *fresh248 as uint32_t;
                            current_block = 1367206831037961075;
                        }
                        43 => {
                            let fresh249 = pc;
                            pc = pc.offset(1);
                            a = *fresh249 as uint32_t;
                            current_block = 6367683481748506220;
                        }
                        44 => {
                            let fresh250 = pc;
                            pc = pc.offset(1);
                            a = *fresh250 as uint32_t;
                            pc = pc.offset(2isize);
                            b =
                                ((*pc.offset(-2isize).offset(0isize) as
                                      libc::c_int) << 8i32 |
                                     *pc.offset(-2isize).offset(1isize) as
                                         libc::c_int) as uint16_t;
                            current_block = 12846811306833230447;
                        }
                        45 => {
                            let fresh251 = pc;
                            pc = pc.offset(1);
                            a = *fresh251 as uint32_t;
                            pc = pc.offset(2isize);
                            b =
                                ((*pc.offset(-2isize).offset(0isize) as
                                      libc::c_int) << 8i32 |
                                     *pc.offset(-2isize).offset(1isize) as
                                         libc::c_int) as uint16_t;
                            current_block = 10246497873717858537;
                        }
                        46 => {
                            let fresh252 = pc;
                            pc = pc.offset(1);
                            a = *fresh252 as uint32_t;
                            pc = pc.offset(2isize);
                            b =
                                ((*pc.offset(-2isize).offset(0isize) as
                                      libc::c_int) << 8i32 |
                                     *pc.offset(-2isize).offset(1isize) as
                                         libc::c_int) as uint16_t;
                            let fresh253 = pc;
                            pc = pc.offset(1);
                            c = *fresh253;
                            current_block = 5602041749335279496;
                        }
                        47 => {
                            let fresh254 = pc;
                            pc = pc.offset(1);
                            a = *fresh254 as uint32_t;
                            pc = pc.offset(2isize);
                            b =
                                ((*pc.offset(-2isize).offset(0isize) as
                                      libc::c_int) << 8i32 |
                                     *pc.offset(-2isize).offset(1isize) as
                                         libc::c_int) as uint16_t;
                            let fresh255 = pc;
                            pc = pc.offset(1);
                            c = *fresh255;
                            current_block = 3906616468301123675;
                        }
                        48 => { current_block = 11480039587962891479; }
                        49 => {
                            let fresh256 = pc;
                            pc = pc.offset(1);
                            a = *fresh256 as uint32_t;
                            pc = pc.offset(2isize);
                            b =
                                ((*pc.offset(-2isize).offset(0isize) as
                                      libc::c_int) << 8i32 |
                                     *pc.offset(-2isize).offset(1isize) as
                                         libc::c_int) as uint16_t;
                            current_block = 5769007513321684282;
                        }
                        50 => {
                            let fresh257 = pc;
                            pc = pc.offset(1);
                            a = *fresh257 as uint32_t;
                            pc = pc.offset(2isize);
                            b =
                                ((*pc.offset(-2isize).offset(0isize) as
                                      libc::c_int) << 8i32 |
                                     *pc.offset(-2isize).offset(1isize) as
                                         libc::c_int) as uint16_t;
                            current_block = 4063058694607173453;
                        }
                        51 => {
                            pc = pc.offset(3isize);
                            a =
                                ((*pc.offset(-3isize).offset(0isize) as
                                      libc::c_int) << 16i32 |
                                     (*pc.offset(-3isize).offset(1isize) as
                                          libc::c_int) << 8i32 |
                                     *pc.offset(-3isize).offset(2isize) as
                                         libc::c_int) as uint32_t;
                            current_block = 17974925641258900466;
                        }
                        52 => {
                            let fresh258 = pc;
                            pc = pc.offset(1);
                            a = *fresh258 as uint32_t;
                            pc = pc.offset(2isize);
                            b =
                                ((*pc.offset(-2isize).offset(0isize) as
                                      libc::c_int) << 8i32 |
                                     *pc.offset(-2isize).offset(1isize) as
                                         libc::c_int) as uint16_t;
                            current_block = 17792945831448561469;
                        }
                        53 => { current_block = 11667657158681136576; }
                        54 => {
                            let fresh259 = pc;
                            pc = pc.offset(1);
                            a = *fresh259 as uint32_t;
                            pc = pc.offset(2isize);
                            b =
                                ((*pc.offset(-2isize).offset(0isize) as
                                      libc::c_int) << 8i32 |
                                     *pc.offset(-2isize).offset(1isize) as
                                         libc::c_int) as uint16_t;
                            current_block = 13480378041738100027;
                        }
                        55 => {
                            let fresh260 = pc;
                            pc = pc.offset(1);
                            a = *fresh260 as uint32_t;
                            current_block = 15446005597338724160;
                        }
                        56 => {
                            let fresh261 = pc;
                            pc = pc.offset(1);
                            a = *fresh261 as uint32_t;
                            current_block = 15158068222670743697;
                        }
                        57 => {
                            let fresh262 = pc;
                            pc = pc.offset(1);
                            a = *fresh262 as uint32_t;
                            current_block = 6968106866976024785;
                        }
                        58 => {
                            let fresh263 = pc;
                            pc = pc.offset(1);
                            a = *fresh263 as uint32_t;
                            pc = pc.offset(2isize);
                            b =
                                ((*pc.offset(-2isize).offset(0isize) as
                                      libc::c_int) << 8i32 |
                                     *pc.offset(-2isize).offset(1isize) as
                                         libc::c_int) as uint16_t;
                            current_block = 5611126728812655874;
                        }
                        59 => {
                            let fresh264 = pc;
                            pc = pc.offset(1);
                            a = *fresh264 as uint32_t;
                            current_block = 328988732995390160;
                        }
                        60 => {
                            let fresh265 = pc;
                            pc = pc.offset(1);
                            a = *fresh265 as uint32_t;
                            pc = pc.offset(2isize);
                            b =
                                ((*pc.offset(-2isize).offset(0isize) as
                                      libc::c_int) << 8i32 |
                                     *pc.offset(-2isize).offset(1isize) as
                                         libc::c_int) as uint16_t;
                            current_block = 10541196509243133637;
                        }
                        61 => {
                            let fresh266 = pc;
                            pc = pc.offset(1);
                            a = *fresh266 as uint32_t;
                            current_block = 16989604180394837074;
                        }
                        62 => {
                            let fresh267 = pc;
                            pc = pc.offset(1);
                            a = *fresh267 as uint32_t;
                            pc = pc.offset(2isize);
                            b =
                                ((*pc.offset(-2isize).offset(0isize) as
                                      libc::c_int) << 8i32 |
                                     *pc.offset(-2isize).offset(1isize) as
                                         libc::c_int) as uint16_t;
                            current_block = 15326039268735274622;
                        }
                        63 => {
                            let fresh268 = pc;
                            pc = pc.offset(1);
                            a = *fresh268 as uint32_t;
                            current_block = 6846200202819738136;
                        }
                        64 => {
                            let fresh269 = pc;
                            pc = pc.offset(1);
                            a = *fresh269 as uint32_t;
                            current_block = 16948125840707872018;
                        }
                        65 => {
                            let fresh270 = pc;
                            pc = pc.offset(1);
                            a = *fresh270 as uint32_t;
                            current_block = 11372858789397587571;
                        }
                        66 => {
                            let fresh271 = pc;
                            pc = pc.offset(1);
                            a = *fresh271 as uint32_t;
                            current_block = 1975408140333322065;
                        }
                        67 => {
                            let fresh272 = pc;
                            pc = pc.offset(1);
                            a = *fresh272 as uint32_t;
                            current_block = 12696324532854772620;
                        }
                        68 => {
                            let fresh273 = pc;
                            pc = pc.offset(1);
                            a = *fresh273 as uint32_t;
                            current_block = 3433849069680490924;
                        }
                        69 => {
                            let fresh274 = pc;
                            pc = pc.offset(1);
                            a = *fresh274 as uint32_t;
                            current_block = 10072772881953407077;
                        }
                        70 => {
                            let fresh275 = pc;
                            pc = pc.offset(1);
                            a = *fresh275 as uint32_t;
                            pc = pc.offset(2isize);
                            b =
                                ((*pc.offset(-2isize).offset(0isize) as
                                      libc::c_int) << 8i32 |
                                     *pc.offset(-2isize).offset(1isize) as
                                         libc::c_int) as uint16_t;
                            current_block = 4850402844069062092;
                        }
                        71 => {
                            let fresh276 = pc;
                            pc = pc.offset(1);
                            a = *fresh276 as uint32_t;
                            pc = pc.offset(2isize);
                            b =
                                ((*pc.offset(-2isize).offset(0isize) as
                                      libc::c_int) << 8i32 |
                                     *pc.offset(-2isize).offset(1isize) as
                                         libc::c_int) as uint16_t;
                            let fresh277 = pc;
                            pc = pc.offset(1);
                            c = *fresh277;
                            current_block = 1741746549607172898;
                        }
                        72 => {
                            let fresh278 = pc;
                            pc = pc.offset(1);
                            a = *fresh278 as uint32_t;
                            current_block = 6053772697959983332;
                        }
                        73 => {
                            let fresh279 = pc;
                            pc = pc.offset(1);
                            a = *fresh279 as uint32_t;
                            current_block = 18022542853155082387;
                        }
                        74 => {
                            let fresh280 = pc;
                            pc = pc.offset(1);
                            a = *fresh280 as uint32_t;
                            current_block = 1606585337627918712;
                        }
                        75 => {
                            let fresh281 = pc;
                            pc = pc.offset(1);
                            a = *fresh281 as uint32_t;
                            pc = pc.offset(2isize);
                            b =
                                ((*pc.offset(-2isize).offset(0isize) as
                                      libc::c_int) << 8i32 |
                                     *pc.offset(-2isize).offset(1isize) as
                                         libc::c_int) as uint16_t;
                            let fresh282 = pc;
                            pc = pc.offset(1);
                            c = *fresh282;
                            current_block = 9599741646002396977;
                        }
                        76 => {
                            let fresh283 = pc;
                            pc = pc.offset(1);
                            a = *fresh283 as uint32_t;
                            pc = pc.offset(2isize);
                            b =
                                ((*pc.offset(-2isize).offset(0isize) as
                                      libc::c_int) << 8i32 |
                                     *pc.offset(-2isize).offset(1isize) as
                                         libc::c_int) as uint16_t;
                            let fresh284 = pc;
                            pc = pc.offset(1);
                            c = *fresh284;
                            current_block = 11441496697127961056;
                        }
                        77 => {
                            let fresh285 = pc;
                            pc = pc.offset(1);
                            a = *fresh285 as uint32_t;
                            pc = pc.offset(2isize);
                            b =
                                ((*pc.offset(-2isize).offset(0isize) as
                                      libc::c_int) << 8i32 |
                                     *pc.offset(-2isize).offset(1isize) as
                                         libc::c_int) as uint16_t;
                            let fresh286 = pc;
                            pc = pc.offset(1);
                            c = *fresh286;
                            current_block = 10671272775318305553;
                        }
                        78 => {
                            let fresh287 = pc;
                            pc = pc.offset(1);
                            a = *fresh287 as uint32_t;
                            current_block = 7127613333771968219;
                        }
                        79 => {
                            let fresh288 = pc;
                            pc = pc.offset(1);
                            a = *fresh288 as uint32_t;
                            pc = pc.offset(2isize);
                            b =
                                ((*pc.offset(-2isize).offset(0isize) as
                                      libc::c_int) << 8i32 |
                                     *pc.offset(-2isize).offset(1isize) as
                                         libc::c_int) as uint16_t;
                            current_block = 16677287981175421660;
                        }
                        80 => {
                            let fresh289 = pc;
                            pc = pc.offset(1);
                            a = *fresh289 as uint32_t;
                            current_block = 13433017329934546124;
                        }
                        81 => {
                            let fresh290 = pc;
                            pc = pc.offset(1);
                            a = *fresh290 as uint32_t;
                            pc = pc.offset(2isize);
                            b =
                                ((*pc.offset(-2isize).offset(0isize) as
                                      libc::c_int) << 8i32 |
                                     *pc.offset(-2isize).offset(1isize) as
                                         libc::c_int) as uint16_t;
                            current_block = 314002374245450226;
                        }
                        82 => {
                            let fresh291 = pc;
                            pc = pc.offset(1);
                            a = *fresh291 as uint32_t;
                            pc = pc.offset(2isize);
                            b =
                                ((*pc.offset(-2isize).offset(0isize) as
                                      libc::c_int) << 8i32 |
                                     *pc.offset(-2isize).offset(1isize) as
                                         libc::c_int) as uint16_t;
                            current_block = 5341326654522537507;
                        }
                        83 => {
                            let fresh292 = pc;
                            pc = pc.offset(1);
                            a = *fresh292 as uint32_t;
                            current_block = 14386882892843117929;
                        }
                        84 => {
                            let fresh293 = pc;
                            pc = pc.offset(1);
                            a = *fresh293 as uint32_t;
                            pc = pc.offset(2isize);
                            b =
                                ((*pc.offset(-2isize).offset(0isize) as
                                      libc::c_int) << 8i32 |
                                     *pc.offset(-2isize).offset(1isize) as
                                         libc::c_int) as uint16_t;
                            current_block = 3020430753860169586;
                        }
                        85 => {
                            let fresh294 = pc;
                            pc = pc.offset(1);
                            a = *fresh294 as uint32_t;
                            pc = pc.offset(2isize);
                            b =
                                ((*pc.offset(-2isize).offset(0isize) as
                                      libc::c_int) << 8i32 |
                                     *pc.offset(-2isize).offset(1isize) as
                                         libc::c_int) as uint16_t;
                            current_block = 11988973803779423327;
                        }
                        86 => {
                            let fresh295 = pc;
                            pc = pc.offset(1);
                            a = *fresh295 as uint32_t;
                            pc = pc.offset(2isize);
                            b =
                                ((*pc.offset(-2isize).offset(0isize) as
                                      libc::c_int) << 8i32 |
                                     *pc.offset(-2isize).offset(1isize) as
                                         libc::c_int) as uint16_t;
                            current_block = 10561944073379837955;
                        }
                        87 => {
                            let fresh296 = pc;
                            pc = pc.offset(1);
                            a = *fresh296 as uint32_t;
                            current_block = 9864403379770423142;
                        }
                        88 => {
                            let fresh297 = pc;
                            pc = pc.offset(1);
                            a = *fresh297 as uint32_t;
                            current_block = 8124361700674833375;
                        }
                        89 => {
                            let fresh298 = pc;
                            pc = pc.offset(1);
                            a = *fresh298 as uint32_t;
                            current_block = 2968065794317548828;
                        }
                        90 => {
                            let fresh299 = pc;
                            pc = pc.offset(1);
                            a = *fresh299 as uint32_t;
                            pc = pc.offset(2isize);
                            b =
                                ((*pc.offset(-2isize).offset(0isize) as
                                      libc::c_int) << 8i32 |
                                     *pc.offset(-2isize).offset(1isize) as
                                         libc::c_int) as uint16_t;
                            current_block = 3258098155381626199;
                        }
                        91 => {
                            let fresh300 = pc;
                            pc = pc.offset(1);
                            a = *fresh300 as uint32_t;
                            pc = pc.offset(2isize);
                            b =
                                ((*pc.offset(-2isize).offset(0isize) as
                                      libc::c_int) << 8i32 |
                                     *pc.offset(-2isize).offset(1isize) as
                                         libc::c_int) as uint16_t;
                            current_block = 7493503892657452062;
                        }
                        92 => {
                            let fresh301 = pc;
                            pc = pc.offset(1);
                            a = *fresh301 as uint32_t;
                            pc = pc.offset(2isize);
                            b =
                                ((*pc.offset(-2isize).offset(0isize) as
                                      libc::c_int) << 8i32 |
                                     *pc.offset(-2isize).offset(1isize) as
                                         libc::c_int) as uint16_t;
                            current_block = 2499490863002033965;
                        }
                        93 => {
                            let fresh302 = pc;
                            pc = pc.offset(1);
                            a = *fresh302 as uint32_t;
                            pc = pc.offset(2isize);
                            b =
                                ((*pc.offset(-2isize).offset(0isize) as
                                      libc::c_int) << 8i32 |
                                     *pc.offset(-2isize).offset(1isize) as
                                         libc::c_int) as uint16_t;
                            current_block = 8489059574810375089;
                        }
                        94 => {
                            let fresh303 = pc;
                            pc = pc.offset(1);
                            a = *fresh303 as uint32_t;
                            pc = pc.offset(2isize);
                            b =
                                ((*pc.offset(-2isize).offset(0isize) as
                                      libc::c_int) << 8i32 |
                                     *pc.offset(-2isize).offset(1isize) as
                                         libc::c_int) as uint16_t;
                            current_block = 12998570369541158573;
                        }
                        95 => {
                            let fresh304 = pc;
                            pc = pc.offset(1);
                            a = *fresh304 as uint32_t;
                            current_block = 12775348822761942368;
                        }
                        96 => {
                            let fresh305 = pc;
                            pc = pc.offset(1);
                            a = *fresh305 as uint32_t;
                            current_block = 355116111335096151;
                        }
                        97 => {
                            let fresh306 = pc;
                            pc = pc.offset(1);
                            a = *fresh306 as uint32_t;
                            current_block = 890063714143163519;
                        }
                        98 => {
                            let fresh307 = pc;
                            pc = pc.offset(1);
                            a = *fresh307 as uint32_t;
                            pc = pc.offset(2isize);
                            b =
                                ((*pc.offset(-2isize).offset(0isize) as
                                      libc::c_int) << 8i32 |
                                     *pc.offset(-2isize).offset(1isize) as
                                         libc::c_int) as uint16_t;
                            let fresh308 = pc;
                            pc = pc.offset(1);
                            c = *fresh308;
                            current_block = 998308582751651876;
                        }
                        99 => {
                            let fresh309 = pc;
                            pc = pc.offset(1);
                            a = *fresh309 as uint32_t;
                            current_block = 7833862538762852372;
                        }
                        100 => { current_block = 8667820544026455208; }
                        101 => { current_block = 10575315580790317560; }
                        102 => { current_block = 18239940301814930841; }
                        103 => { current_block = 10942739336945693405; }
                        _ => { current_block = 3935997918849240122; }
                    }
                }
                8667820544026455208 => {
                    let fresh150 = pc;
                    pc = pc.offset(1);
                    ins = *fresh150;
                    printf(b"OP_EXT1\n\x00" as *const u8 as
                               *const libc::c_char);
                    print_header(mrb, irep,
                                 pc.wrapping_offset_from((*irep).iseq) as
                                     libc::c_long - 2i32 as libc::c_long);
                    match ins as libc::c_int {
                        0 => { current_block = 4153728562337695617; }
                        1 => {
                            pc = pc.offset(2isize);
                            a =
                                ((*pc.offset(-2isize).offset(0isize) as
                                      libc::c_int) << 8i32 |
                                     *pc.offset(-2isize).offset(1isize) as
                                         libc::c_int) as uint32_t;
                            let fresh151 = pc;
                            pc = pc.offset(1);
                            b = *fresh151 as uint16_t;
                            current_block = 3345631096401374476;
                        }
                        2 => {
                            pc = pc.offset(2isize);
                            a =
                                ((*pc.offset(-2isize).offset(0isize) as
                                      libc::c_int) << 8i32 |
                                     *pc.offset(-2isize).offset(1isize) as
                                         libc::c_int) as uint32_t;
                            let fresh152 = pc;
                            pc = pc.offset(1);
                            b = *fresh152 as uint16_t;
                            current_block = 14904807116429336826;
                        }
                        3 => {
                            pc = pc.offset(2isize);
                            a =
                                ((*pc.offset(-2isize).offset(0isize) as
                                      libc::c_int) << 8i32 |
                                     *pc.offset(-2isize).offset(1isize) as
                                         libc::c_int) as uint32_t;
                            let fresh153 = pc;
                            pc = pc.offset(1);
                            b = *fresh153 as uint16_t;
                            current_block = 8703264613394043654;
                        }
                        4 => {
                            pc = pc.offset(2isize);
                            a =
                                ((*pc.offset(-2isize).offset(0isize) as
                                      libc::c_int) << 8i32 |
                                     *pc.offset(-2isize).offset(1isize) as
                                         libc::c_int) as uint32_t;
                            let fresh154 = pc;
                            pc = pc.offset(1);
                            b = *fresh154 as uint16_t;
                            current_block = 4843221280033581897;
                        }
                        5 => {
                            pc = pc.offset(2isize);
                            a =
                                ((*pc.offset(-2isize).offset(0isize) as
                                      libc::c_int) << 8i32 |
                                     *pc.offset(-2isize).offset(1isize) as
                                         libc::c_int) as uint32_t;
                            current_block = 6276274620003476740;
                        }
                        6 => {
                            pc = pc.offset(2isize);
                            a =
                                ((*pc.offset(-2isize).offset(0isize) as
                                      libc::c_int) << 8i32 |
                                     *pc.offset(-2isize).offset(1isize) as
                                         libc::c_int) as uint32_t;
                            current_block = 8096298871650922110;
                        }
                        7 => {
                            pc = pc.offset(2isize);
                            a =
                                ((*pc.offset(-2isize).offset(0isize) as
                                      libc::c_int) << 8i32 |
                                     *pc.offset(-2isize).offset(1isize) as
                                         libc::c_int) as uint32_t;
                            current_block = 8096298871650922110;
                        }
                        8 => {
                            pc = pc.offset(2isize);
                            a =
                                ((*pc.offset(-2isize).offset(0isize) as
                                      libc::c_int) << 8i32 |
                                     *pc.offset(-2isize).offset(1isize) as
                                         libc::c_int) as uint32_t;
                            current_block = 8096298871650922110;
                        }
                        9 => {
                            pc = pc.offset(2isize);
                            a =
                                ((*pc.offset(-2isize).offset(0isize) as
                                      libc::c_int) << 8i32 |
                                     *pc.offset(-2isize).offset(1isize) as
                                         libc::c_int) as uint32_t;
                            current_block = 8096298871650922110;
                        }
                        10 => {
                            pc = pc.offset(2isize);
                            a =
                                ((*pc.offset(-2isize).offset(0isize) as
                                      libc::c_int) << 8i32 |
                                     *pc.offset(-2isize).offset(1isize) as
                                         libc::c_int) as uint32_t;
                            current_block = 8096298871650922110;
                        }
                        11 => {
                            pc = pc.offset(2isize);
                            a =
                                ((*pc.offset(-2isize).offset(0isize) as
                                      libc::c_int) << 8i32 |
                                     *pc.offset(-2isize).offset(1isize) as
                                         libc::c_int) as uint32_t;
                            current_block = 8096298871650922110;
                        }
                        12 => {
                            pc = pc.offset(2isize);
                            a =
                                ((*pc.offset(-2isize).offset(0isize) as
                                      libc::c_int) << 8i32 |
                                     *pc.offset(-2isize).offset(1isize) as
                                         libc::c_int) as uint32_t;
                            current_block = 8096298871650922110;
                        }
                        13 => {
                            pc = pc.offset(2isize);
                            a =
                                ((*pc.offset(-2isize).offset(0isize) as
                                      libc::c_int) << 8i32 |
                                     *pc.offset(-2isize).offset(1isize) as
                                         libc::c_int) as uint32_t;
                            current_block = 8096298871650922110;
                        }
                        14 => {
                            pc = pc.offset(2isize);
                            a =
                                ((*pc.offset(-2isize).offset(0isize) as
                                      libc::c_int) << 8i32 |
                                     *pc.offset(-2isize).offset(1isize) as
                                         libc::c_int) as uint32_t;
                            let fresh155 = pc;
                            pc = pc.offset(1);
                            b = *fresh155 as uint16_t;
                            current_block = 16157800273112125286;
                        }
                        15 => {
                            pc = pc.offset(2isize);
                            a =
                                ((*pc.offset(-2isize).offset(0isize) as
                                      libc::c_int) << 8i32 |
                                     *pc.offset(-2isize).offset(1isize) as
                                         libc::c_int) as uint32_t;
                            current_block = 17940677572951814003;
                        }
                        16 => {
                            pc = pc.offset(2isize);
                            a =
                                ((*pc.offset(-2isize).offset(0isize) as
                                      libc::c_int) << 8i32 |
                                     *pc.offset(-2isize).offset(1isize) as
                                         libc::c_int) as uint32_t;
                            current_block = 12915432016225878389;
                        }
                        17 => {
                            pc = pc.offset(2isize);
                            a =
                                ((*pc.offset(-2isize).offset(0isize) as
                                      libc::c_int) << 8i32 |
                                     *pc.offset(-2isize).offset(1isize) as
                                         libc::c_int) as uint32_t;
                            current_block = 4255402116796946452;
                        }
                        18 => {
                            pc = pc.offset(2isize);
                            a =
                                ((*pc.offset(-2isize).offset(0isize) as
                                      libc::c_int) << 8i32 |
                                     *pc.offset(-2isize).offset(1isize) as
                                         libc::c_int) as uint32_t;
                            current_block = 2779364454669598918;
                        }
                        19 => {
                            pc = pc.offset(2isize);
                            a =
                                ((*pc.offset(-2isize).offset(0isize) as
                                      libc::c_int) << 8i32 |
                                     *pc.offset(-2isize).offset(1isize) as
                                         libc::c_int) as uint32_t;
                            let fresh156 = pc;
                            pc = pc.offset(1);
                            b = *fresh156 as uint16_t;
                            current_block = 17386262169460468759;
                        }
                        20 => {
                            pc = pc.offset(2isize);
                            a =
                                ((*pc.offset(-2isize).offset(0isize) as
                                      libc::c_int) << 8i32 |
                                     *pc.offset(-2isize).offset(1isize) as
                                         libc::c_int) as uint32_t;
                            let fresh157 = pc;
                            pc = pc.offset(1);
                            b = *fresh157 as uint16_t;
                            current_block = 16200511029630275951;
                        }
                        21 => {
                            pc = pc.offset(2isize);
                            a =
                                ((*pc.offset(-2isize).offset(0isize) as
                                      libc::c_int) << 8i32 |
                                     *pc.offset(-2isize).offset(1isize) as
                                         libc::c_int) as uint32_t;
                            let fresh158 = pc;
                            pc = pc.offset(1);
                            b = *fresh158 as uint16_t;
                            current_block = 16758983331069857237;
                        }
                        22 => {
                            pc = pc.offset(2isize);
                            a =
                                ((*pc.offset(-2isize).offset(0isize) as
                                      libc::c_int) << 8i32 |
                                     *pc.offset(-2isize).offset(1isize) as
                                         libc::c_int) as uint32_t;
                            let fresh159 = pc;
                            pc = pc.offset(1);
                            b = *fresh159 as uint16_t;
                            current_block = 4481804007556508143;
                        }
                        23 => {
                            pc = pc.offset(2isize);
                            a =
                                ((*pc.offset(-2isize).offset(0isize) as
                                      libc::c_int) << 8i32 |
                                     *pc.offset(-2isize).offset(1isize) as
                                         libc::c_int) as uint32_t;
                            let fresh160 = pc;
                            pc = pc.offset(1);
                            b = *fresh160 as uint16_t;
                            current_block = 7446123777345652171;
                        }
                        24 => {
                            pc = pc.offset(2isize);
                            a =
                                ((*pc.offset(-2isize).offset(0isize) as
                                      libc::c_int) << 8i32 |
                                     *pc.offset(-2isize).offset(1isize) as
                                         libc::c_int) as uint32_t;
                            let fresh161 = pc;
                            pc = pc.offset(1);
                            b = *fresh161 as uint16_t;
                            current_block = 7570639851576535195;
                        }
                        25 => {
                            pc = pc.offset(2isize);
                            a =
                                ((*pc.offset(-2isize).offset(0isize) as
                                      libc::c_int) << 8i32 |
                                     *pc.offset(-2isize).offset(1isize) as
                                         libc::c_int) as uint32_t;
                            let fresh162 = pc;
                            pc = pc.offset(1);
                            b = *fresh162 as uint16_t;
                            current_block = 1863480813282067938;
                        }
                        26 => {
                            pc = pc.offset(2isize);
                            a =
                                ((*pc.offset(-2isize).offset(0isize) as
                                      libc::c_int) << 8i32 |
                                     *pc.offset(-2isize).offset(1isize) as
                                         libc::c_int) as uint32_t;
                            let fresh163 = pc;
                            pc = pc.offset(1);
                            b = *fresh163 as uint16_t;
                            current_block = 7849445245878263537;
                        }
                        27 => {
                            pc = pc.offset(2isize);
                            a =
                                ((*pc.offset(-2isize).offset(0isize) as
                                      libc::c_int) << 8i32 |
                                     *pc.offset(-2isize).offset(1isize) as
                                         libc::c_int) as uint32_t;
                            let fresh164 = pc;
                            pc = pc.offset(1);
                            b = *fresh164 as uint16_t;
                            current_block = 16034675043887719473;
                        }
                        28 => {
                            pc = pc.offset(2isize);
                            a =
                                ((*pc.offset(-2isize).offset(0isize) as
                                      libc::c_int) << 8i32 |
                                     *pc.offset(-2isize).offset(1isize) as
                                         libc::c_int) as uint32_t;
                            let fresh165 = pc;
                            pc = pc.offset(1);
                            b = *fresh165 as uint16_t;
                            current_block = 942271536672158721;
                        }
                        29 => {
                            pc = pc.offset(2isize);
                            a =
                                ((*pc.offset(-2isize).offset(0isize) as
                                      libc::c_int) << 8i32 |
                                     *pc.offset(-2isize).offset(1isize) as
                                         libc::c_int) as uint32_t;
                            let fresh166 = pc;
                            pc = pc.offset(1);
                            b = *fresh166 as uint16_t;
                            current_block = 1873199301525838551;
                        }
                        30 => {
                            pc = pc.offset(2isize);
                            a =
                                ((*pc.offset(-2isize).offset(0isize) as
                                      libc::c_int) << 8i32 |
                                     *pc.offset(-2isize).offset(1isize) as
                                         libc::c_int) as uint32_t;
                            let fresh167 = pc;
                            pc = pc.offset(1);
                            b = *fresh167 as uint16_t;
                            current_block = 9704183418455961811;
                        }
                        31 => {
                            pc = pc.offset(2isize);
                            a =
                                ((*pc.offset(-2isize).offset(0isize) as
                                      libc::c_int) << 8i32 |
                                     *pc.offset(-2isize).offset(1isize) as
                                         libc::c_int) as uint32_t;
                            let fresh168 = pc;
                            pc = pc.offset(1);
                            b = *fresh168 as uint16_t;
                            let fresh169 = pc;
                            pc = pc.offset(1);
                            c = *fresh169;
                            current_block = 10547193261004346349;
                        }
                        32 => {
                            pc = pc.offset(2isize);
                            a =
                                ((*pc.offset(-2isize).offset(0isize) as
                                      libc::c_int) << 8i32 |
                                     *pc.offset(-2isize).offset(1isize) as
                                         libc::c_int) as uint32_t;
                            let fresh170 = pc;
                            pc = pc.offset(1);
                            b = *fresh170 as uint16_t;
                            let fresh171 = pc;
                            pc = pc.offset(1);
                            c = *fresh171;
                            current_block = 9015561896112508021;
                        }
                        33 => {
                            pc = pc.offset(2isize);
                            a =
                                ((*pc.offset(-2isize).offset(0isize) as
                                      libc::c_int) << 8i32 |
                                     *pc.offset(-2isize).offset(1isize) as
                                         libc::c_int) as uint32_t;
                            current_block = 10458688773482078930;
                        }
                        34 => {
                            pc = pc.offset(2isize);
                            a =
                                ((*pc.offset(-2isize).offset(0isize) as
                                      libc::c_int) << 8i32 |
                                     *pc.offset(-2isize).offset(1isize) as
                                         libc::c_int) as uint32_t;
                            pc = pc.offset(2isize);
                            b =
                                ((*pc.offset(-2isize).offset(0isize) as
                                      libc::c_int) << 8i32 |
                                     *pc.offset(-2isize).offset(1isize) as
                                         libc::c_int) as uint16_t;
                            current_block = 9865490829578899964;
                        }
                        35 => {
                            pc = pc.offset(2isize);
                            a =
                                ((*pc.offset(-2isize).offset(0isize) as
                                      libc::c_int) << 8i32 |
                                     *pc.offset(-2isize).offset(1isize) as
                                         libc::c_int) as uint32_t;
                            pc = pc.offset(2isize);
                            b =
                                ((*pc.offset(-2isize).offset(0isize) as
                                      libc::c_int) << 8i32 |
                                     *pc.offset(-2isize).offset(1isize) as
                                         libc::c_int) as uint16_t;
                            current_block = 17639296353235453144;
                        }
                        36 => {
                            pc = pc.offset(2isize);
                            a =
                                ((*pc.offset(-2isize).offset(0isize) as
                                      libc::c_int) << 8i32 |
                                     *pc.offset(-2isize).offset(1isize) as
                                         libc::c_int) as uint32_t;
                            pc = pc.offset(2isize);
                            b =
                                ((*pc.offset(-2isize).offset(0isize) as
                                      libc::c_int) << 8i32 |
                                     *pc.offset(-2isize).offset(1isize) as
                                         libc::c_int) as uint16_t;
                            current_block = 12781735722417726073;
                        }
                        37 => {
                            pc = pc.offset(2isize);
                            a =
                                ((*pc.offset(-2isize).offset(0isize) as
                                      libc::c_int) << 8i32 |
                                     *pc.offset(-2isize).offset(1isize) as
                                         libc::c_int) as uint32_t;
                            current_block = 6898479631119937405;
                        }
                        38 => {
                            pc = pc.offset(2isize);
                            a =
                                ((*pc.offset(-2isize).offset(0isize) as
                                      libc::c_int) << 8i32 |
                                     *pc.offset(-2isize).offset(1isize) as
                                         libc::c_int) as uint32_t;
                            current_block = 4275858079292575654;
                        }
                        39 => {
                            pc = pc.offset(2isize);
                            a =
                                ((*pc.offset(-2isize).offset(0isize) as
                                      libc::c_int) << 8i32 |
                                     *pc.offset(-2isize).offset(1isize) as
                                         libc::c_int) as uint32_t;
                            let fresh172 = pc;
                            pc = pc.offset(1);
                            b = *fresh172 as uint16_t;
                            current_block = 5105253204732552581;
                        }
                        40 => {
                            pc = pc.offset(2isize);
                            a =
                                ((*pc.offset(-2isize).offset(0isize) as
                                      libc::c_int) << 8i32 |
                                     *pc.offset(-2isize).offset(1isize) as
                                         libc::c_int) as uint32_t;
                            current_block = 1459864638523139318;
                        }
                        41 => {
                            pc = pc.offset(2isize);
                            a =
                                ((*pc.offset(-2isize).offset(0isize) as
                                      libc::c_int) << 8i32 |
                                     *pc.offset(-2isize).offset(1isize) as
                                         libc::c_int) as uint32_t;
                            current_block = 2384513197176890912;
                        }
                        42 => {
                            pc = pc.offset(2isize);
                            a =
                                ((*pc.offset(-2isize).offset(0isize) as
                                      libc::c_int) << 8i32 |
                                     *pc.offset(-2isize).offset(1isize) as
                                         libc::c_int) as uint32_t;
                            current_block = 1367206831037961075;
                        }
                        43 => {
                            pc = pc.offset(2isize);
                            a =
                                ((*pc.offset(-2isize).offset(0isize) as
                                      libc::c_int) << 8i32 |
                                     *pc.offset(-2isize).offset(1isize) as
                                         libc::c_int) as uint32_t;
                            current_block = 6367683481748506220;
                        }
                        44 => {
                            pc = pc.offset(2isize);
                            a =
                                ((*pc.offset(-2isize).offset(0isize) as
                                      libc::c_int) << 8i32 |
                                     *pc.offset(-2isize).offset(1isize) as
                                         libc::c_int) as uint32_t;
                            let fresh173 = pc;
                            pc = pc.offset(1);
                            b = *fresh173 as uint16_t;
                            current_block = 12846811306833230447;
                        }
                        45 => {
                            pc = pc.offset(2isize);
                            a =
                                ((*pc.offset(-2isize).offset(0isize) as
                                      libc::c_int) << 8i32 |
                                     *pc.offset(-2isize).offset(1isize) as
                                         libc::c_int) as uint32_t;
                            let fresh174 = pc;
                            pc = pc.offset(1);
                            b = *fresh174 as uint16_t;
                            current_block = 10246497873717858537;
                        }
                        46 => {
                            pc = pc.offset(2isize);
                            a =
                                ((*pc.offset(-2isize).offset(0isize) as
                                      libc::c_int) << 8i32 |
                                     *pc.offset(-2isize).offset(1isize) as
                                         libc::c_int) as uint32_t;
                            let fresh175 = pc;
                            pc = pc.offset(1);
                            b = *fresh175 as uint16_t;
                            let fresh176 = pc;
                            pc = pc.offset(1);
                            c = *fresh176;
                            current_block = 5602041749335279496;
                        }
                        47 => {
                            pc = pc.offset(2isize);
                            a =
                                ((*pc.offset(-2isize).offset(0isize) as
                                      libc::c_int) << 8i32 |
                                     *pc.offset(-2isize).offset(1isize) as
                                         libc::c_int) as uint32_t;
                            let fresh177 = pc;
                            pc = pc.offset(1);
                            b = *fresh177 as uint16_t;
                            let fresh178 = pc;
                            pc = pc.offset(1);
                            c = *fresh178;
                            current_block = 3906616468301123675;
                        }
                        48 => { current_block = 11480039587962891479; }
                        49 => {
                            pc = pc.offset(2isize);
                            a =
                                ((*pc.offset(-2isize).offset(0isize) as
                                      libc::c_int) << 8i32 |
                                     *pc.offset(-2isize).offset(1isize) as
                                         libc::c_int) as uint32_t;
                            let fresh179 = pc;
                            pc = pc.offset(1);
                            b = *fresh179 as uint16_t;
                            current_block = 5769007513321684282;
                        }
                        50 => {
                            pc = pc.offset(2isize);
                            a =
                                ((*pc.offset(-2isize).offset(0isize) as
                                      libc::c_int) << 8i32 |
                                     *pc.offset(-2isize).offset(1isize) as
                                         libc::c_int) as uint32_t;
                            pc = pc.offset(2isize);
                            b =
                                ((*pc.offset(-2isize).offset(0isize) as
                                      libc::c_int) << 8i32 |
                                     *pc.offset(-2isize).offset(1isize) as
                                         libc::c_int) as uint16_t;
                            current_block = 4063058694607173453;
                        }
                        51 => {
                            pc = pc.offset(3isize);
                            a =
                                ((*pc.offset(-3isize).offset(0isize) as
                                      libc::c_int) << 16i32 |
                                     (*pc.offset(-3isize).offset(1isize) as
                                          libc::c_int) << 8i32 |
                                     *pc.offset(-3isize).offset(2isize) as
                                         libc::c_int) as uint32_t;
                            current_block = 17974925641258900466;
                        }
                        52 => {
                            pc = pc.offset(2isize);
                            a =
                                ((*pc.offset(-2isize).offset(0isize) as
                                      libc::c_int) << 8i32 |
                                     *pc.offset(-2isize).offset(1isize) as
                                         libc::c_int) as uint32_t;
                            let fresh180 = pc;
                            pc = pc.offset(1);
                            b = *fresh180 as uint16_t;
                            current_block = 17792945831448561469;
                        }
                        53 => { current_block = 11667657158681136576; }
                        54 => {
                            pc = pc.offset(2isize);
                            a =
                                ((*pc.offset(-2isize).offset(0isize) as
                                      libc::c_int) << 8i32 |
                                     *pc.offset(-2isize).offset(1isize) as
                                         libc::c_int) as uint32_t;
                            let fresh181 = pc;
                            pc = pc.offset(1);
                            b = *fresh181 as uint16_t;
                            current_block = 13480378041738100027;
                        }
                        55 => {
                            pc = pc.offset(2isize);
                            a =
                                ((*pc.offset(-2isize).offset(0isize) as
                                      libc::c_int) << 8i32 |
                                     *pc.offset(-2isize).offset(1isize) as
                                         libc::c_int) as uint32_t;
                            current_block = 15446005597338724160;
                        }
                        56 => {
                            pc = pc.offset(2isize);
                            a =
                                ((*pc.offset(-2isize).offset(0isize) as
                                      libc::c_int) << 8i32 |
                                     *pc.offset(-2isize).offset(1isize) as
                                         libc::c_int) as uint32_t;
                            current_block = 15158068222670743697;
                        }
                        57 => {
                            pc = pc.offset(2isize);
                            a =
                                ((*pc.offset(-2isize).offset(0isize) as
                                      libc::c_int) << 8i32 |
                                     *pc.offset(-2isize).offset(1isize) as
                                         libc::c_int) as uint32_t;
                            current_block = 6968106866976024785;
                        }
                        58 => {
                            pc = pc.offset(2isize);
                            a =
                                ((*pc.offset(-2isize).offset(0isize) as
                                      libc::c_int) << 8i32 |
                                     *pc.offset(-2isize).offset(1isize) as
                                         libc::c_int) as uint32_t;
                            pc = pc.offset(2isize);
                            b =
                                ((*pc.offset(-2isize).offset(0isize) as
                                      libc::c_int) << 8i32 |
                                     *pc.offset(-2isize).offset(1isize) as
                                         libc::c_int) as uint16_t;
                            current_block = 5611126728812655874;
                        }
                        59 => {
                            pc = pc.offset(2isize);
                            a =
                                ((*pc.offset(-2isize).offset(0isize) as
                                      libc::c_int) << 8i32 |
                                     *pc.offset(-2isize).offset(1isize) as
                                         libc::c_int) as uint32_t;
                            current_block = 328988732995390160;
                        }
                        60 => {
                            pc = pc.offset(2isize);
                            a =
                                ((*pc.offset(-2isize).offset(0isize) as
                                      libc::c_int) << 8i32 |
                                     *pc.offset(-2isize).offset(1isize) as
                                         libc::c_int) as uint32_t;
                            let fresh182 = pc;
                            pc = pc.offset(1);
                            b = *fresh182 as uint16_t;
                            current_block = 10541196509243133637;
                        }
                        61 => {
                            pc = pc.offset(2isize);
                            a =
                                ((*pc.offset(-2isize).offset(0isize) as
                                      libc::c_int) << 8i32 |
                                     *pc.offset(-2isize).offset(1isize) as
                                         libc::c_int) as uint32_t;
                            current_block = 16989604180394837074;
                        }
                        62 => {
                            pc = pc.offset(2isize);
                            a =
                                ((*pc.offset(-2isize).offset(0isize) as
                                      libc::c_int) << 8i32 |
                                     *pc.offset(-2isize).offset(1isize) as
                                         libc::c_int) as uint32_t;
                            let fresh183 = pc;
                            pc = pc.offset(1);
                            b = *fresh183 as uint16_t;
                            current_block = 15326039268735274622;
                        }
                        63 => {
                            pc = pc.offset(2isize);
                            a =
                                ((*pc.offset(-2isize).offset(0isize) as
                                      libc::c_int) << 8i32 |
                                     *pc.offset(-2isize).offset(1isize) as
                                         libc::c_int) as uint32_t;
                            current_block = 6846200202819738136;
                        }
                        64 => {
                            pc = pc.offset(2isize);
                            a =
                                ((*pc.offset(-2isize).offset(0isize) as
                                      libc::c_int) << 8i32 |
                                     *pc.offset(-2isize).offset(1isize) as
                                         libc::c_int) as uint32_t;
                            current_block = 16948125840707872018;
                        }
                        65 => {
                            pc = pc.offset(2isize);
                            a =
                                ((*pc.offset(-2isize).offset(0isize) as
                                      libc::c_int) << 8i32 |
                                     *pc.offset(-2isize).offset(1isize) as
                                         libc::c_int) as uint32_t;
                            current_block = 11372858789397587571;
                        }
                        66 => {
                            pc = pc.offset(2isize);
                            a =
                                ((*pc.offset(-2isize).offset(0isize) as
                                      libc::c_int) << 8i32 |
                                     *pc.offset(-2isize).offset(1isize) as
                                         libc::c_int) as uint32_t;
                            current_block = 1975408140333322065;
                        }
                        67 => {
                            pc = pc.offset(2isize);
                            a =
                                ((*pc.offset(-2isize).offset(0isize) as
                                      libc::c_int) << 8i32 |
                                     *pc.offset(-2isize).offset(1isize) as
                                         libc::c_int) as uint32_t;
                            current_block = 12696324532854772620;
                        }
                        68 => {
                            pc = pc.offset(2isize);
                            a =
                                ((*pc.offset(-2isize).offset(0isize) as
                                      libc::c_int) << 8i32 |
                                     *pc.offset(-2isize).offset(1isize) as
                                         libc::c_int) as uint32_t;
                            current_block = 3433849069680490924;
                        }
                        69 => {
                            pc = pc.offset(2isize);
                            a =
                                ((*pc.offset(-2isize).offset(0isize) as
                                      libc::c_int) << 8i32 |
                                     *pc.offset(-2isize).offset(1isize) as
                                         libc::c_int) as uint32_t;
                            current_block = 10072772881953407077;
                        }
                        70 => {
                            pc = pc.offset(2isize);
                            a =
                                ((*pc.offset(-2isize).offset(0isize) as
                                      libc::c_int) << 8i32 |
                                     *pc.offset(-2isize).offset(1isize) as
                                         libc::c_int) as uint32_t;
                            let fresh184 = pc;
                            pc = pc.offset(1);
                            b = *fresh184 as uint16_t;
                            current_block = 4850402844069062092;
                        }
                        71 => {
                            pc = pc.offset(2isize);
                            a =
                                ((*pc.offset(-2isize).offset(0isize) as
                                      libc::c_int) << 8i32 |
                                     *pc.offset(-2isize).offset(1isize) as
                                         libc::c_int) as uint32_t;
                            let fresh185 = pc;
                            pc = pc.offset(1);
                            b = *fresh185 as uint16_t;
                            let fresh186 = pc;
                            pc = pc.offset(1);
                            c = *fresh186;
                            current_block = 1741746549607172898;
                        }
                        72 => {
                            pc = pc.offset(2isize);
                            a =
                                ((*pc.offset(-2isize).offset(0isize) as
                                      libc::c_int) << 8i32 |
                                     *pc.offset(-2isize).offset(1isize) as
                                         libc::c_int) as uint32_t;
                            current_block = 6053772697959983332;
                        }
                        73 => {
                            pc = pc.offset(2isize);
                            a =
                                ((*pc.offset(-2isize).offset(0isize) as
                                      libc::c_int) << 8i32 |
                                     *pc.offset(-2isize).offset(1isize) as
                                         libc::c_int) as uint32_t;
                            current_block = 18022542853155082387;
                        }
                        74 => {
                            pc = pc.offset(2isize);
                            a =
                                ((*pc.offset(-2isize).offset(0isize) as
                                      libc::c_int) << 8i32 |
                                     *pc.offset(-2isize).offset(1isize) as
                                         libc::c_int) as uint32_t;
                            current_block = 1606585337627918712;
                        }
                        75 => {
                            pc = pc.offset(2isize);
                            a =
                                ((*pc.offset(-2isize).offset(0isize) as
                                      libc::c_int) << 8i32 |
                                     *pc.offset(-2isize).offset(1isize) as
                                         libc::c_int) as uint32_t;
                            let fresh187 = pc;
                            pc = pc.offset(1);
                            b = *fresh187 as uint16_t;
                            let fresh188 = pc;
                            pc = pc.offset(1);
                            c = *fresh188;
                            current_block = 9599741646002396977;
                        }
                        76 => {
                            pc = pc.offset(2isize);
                            a =
                                ((*pc.offset(-2isize).offset(0isize) as
                                      libc::c_int) << 8i32 |
                                     *pc.offset(-2isize).offset(1isize) as
                                         libc::c_int) as uint32_t;
                            let fresh189 = pc;
                            pc = pc.offset(1);
                            b = *fresh189 as uint16_t;
                            let fresh190 = pc;
                            pc = pc.offset(1);
                            c = *fresh190;
                            current_block = 11441496697127961056;
                        }
                        77 => {
                            pc = pc.offset(2isize);
                            a =
                                ((*pc.offset(-2isize).offset(0isize) as
                                      libc::c_int) << 8i32 |
                                     *pc.offset(-2isize).offset(1isize) as
                                         libc::c_int) as uint32_t;
                            let fresh191 = pc;
                            pc = pc.offset(1);
                            b = *fresh191 as uint16_t;
                            let fresh192 = pc;
                            pc = pc.offset(1);
                            c = *fresh192;
                            current_block = 10671272775318305553;
                        }
                        78 => {
                            pc = pc.offset(2isize);
                            a =
                                ((*pc.offset(-2isize).offset(0isize) as
                                      libc::c_int) << 8i32 |
                                     *pc.offset(-2isize).offset(1isize) as
                                         libc::c_int) as uint32_t;
                            current_block = 7127613333771968219;
                        }
                        79 => {
                            pc = pc.offset(2isize);
                            a =
                                ((*pc.offset(-2isize).offset(0isize) as
                                      libc::c_int) << 8i32 |
                                     *pc.offset(-2isize).offset(1isize) as
                                         libc::c_int) as uint32_t;
                            let fresh193 = pc;
                            pc = pc.offset(1);
                            b = *fresh193 as uint16_t;
                            current_block = 16677287981175421660;
                        }
                        80 => {
                            pc = pc.offset(2isize);
                            a =
                                ((*pc.offset(-2isize).offset(0isize) as
                                      libc::c_int) << 8i32 |
                                     *pc.offset(-2isize).offset(1isize) as
                                         libc::c_int) as uint32_t;
                            current_block = 13433017329934546124;
                        }
                        81 => {
                            pc = pc.offset(2isize);
                            a =
                                ((*pc.offset(-2isize).offset(0isize) as
                                      libc::c_int) << 8i32 |
                                     *pc.offset(-2isize).offset(1isize) as
                                         libc::c_int) as uint32_t;
                            let fresh194 = pc;
                            pc = pc.offset(1);
                            b = *fresh194 as uint16_t;
                            current_block = 314002374245450226;
                        }
                        82 => {
                            pc = pc.offset(2isize);
                            a =
                                ((*pc.offset(-2isize).offset(0isize) as
                                      libc::c_int) << 8i32 |
                                     *pc.offset(-2isize).offset(1isize) as
                                         libc::c_int) as uint32_t;
                            let fresh195 = pc;
                            pc = pc.offset(1);
                            b = *fresh195 as uint16_t;
                            current_block = 5341326654522537507;
                        }
                        83 => {
                            pc = pc.offset(2isize);
                            a =
                                ((*pc.offset(-2isize).offset(0isize) as
                                      libc::c_int) << 8i32 |
                                     *pc.offset(-2isize).offset(1isize) as
                                         libc::c_int) as uint32_t;
                            current_block = 14386882892843117929;
                        }
                        84 => {
                            pc = pc.offset(2isize);
                            a =
                                ((*pc.offset(-2isize).offset(0isize) as
                                      libc::c_int) << 8i32 |
                                     *pc.offset(-2isize).offset(1isize) as
                                         libc::c_int) as uint32_t;
                            let fresh196 = pc;
                            pc = pc.offset(1);
                            b = *fresh196 as uint16_t;
                            current_block = 3020430753860169586;
                        }
                        85 => {
                            pc = pc.offset(2isize);
                            a =
                                ((*pc.offset(-2isize).offset(0isize) as
                                      libc::c_int) << 8i32 |
                                     *pc.offset(-2isize).offset(1isize) as
                                         libc::c_int) as uint32_t;
                            let fresh197 = pc;
                            pc = pc.offset(1);
                            b = *fresh197 as uint16_t;
                            current_block = 11988973803779423327;
                        }
                        86 => {
                            pc = pc.offset(2isize);
                            a =
                                ((*pc.offset(-2isize).offset(0isize) as
                                      libc::c_int) << 8i32 |
                                     *pc.offset(-2isize).offset(1isize) as
                                         libc::c_int) as uint32_t;
                            let fresh198 = pc;
                            pc = pc.offset(1);
                            b = *fresh198 as uint16_t;
                            current_block = 10561944073379837955;
                        }
                        87 => {
                            pc = pc.offset(2isize);
                            a =
                                ((*pc.offset(-2isize).offset(0isize) as
                                      libc::c_int) << 8i32 |
                                     *pc.offset(-2isize).offset(1isize) as
                                         libc::c_int) as uint32_t;
                            current_block = 9864403379770423142;
                        }
                        88 => {
                            pc = pc.offset(2isize);
                            a =
                                ((*pc.offset(-2isize).offset(0isize) as
                                      libc::c_int) << 8i32 |
                                     *pc.offset(-2isize).offset(1isize) as
                                         libc::c_int) as uint32_t;
                            current_block = 8124361700674833375;
                        }
                        89 => {
                            pc = pc.offset(2isize);
                            a =
                                ((*pc.offset(-2isize).offset(0isize) as
                                      libc::c_int) << 8i32 |
                                     *pc.offset(-2isize).offset(1isize) as
                                         libc::c_int) as uint32_t;
                            current_block = 2968065794317548828;
                        }
                        90 => {
                            pc = pc.offset(2isize);
                            a =
                                ((*pc.offset(-2isize).offset(0isize) as
                                      libc::c_int) << 8i32 |
                                     *pc.offset(-2isize).offset(1isize) as
                                         libc::c_int) as uint32_t;
                            let fresh199 = pc;
                            pc = pc.offset(1);
                            b = *fresh199 as uint16_t;
                            current_block = 3258098155381626199;
                        }
                        91 => {
                            pc = pc.offset(2isize);
                            a =
                                ((*pc.offset(-2isize).offset(0isize) as
                                      libc::c_int) << 8i32 |
                                     *pc.offset(-2isize).offset(1isize) as
                                         libc::c_int) as uint32_t;
                            let fresh200 = pc;
                            pc = pc.offset(1);
                            b = *fresh200 as uint16_t;
                            current_block = 7493503892657452062;
                        }
                        92 => {
                            pc = pc.offset(2isize);
                            a =
                                ((*pc.offset(-2isize).offset(0isize) as
                                      libc::c_int) << 8i32 |
                                     *pc.offset(-2isize).offset(1isize) as
                                         libc::c_int) as uint32_t;
                            let fresh201 = pc;
                            pc = pc.offset(1);
                            b = *fresh201 as uint16_t;
                            current_block = 2499490863002033965;
                        }
                        93 => {
                            pc = pc.offset(2isize);
                            a =
                                ((*pc.offset(-2isize).offset(0isize) as
                                      libc::c_int) << 8i32 |
                                     *pc.offset(-2isize).offset(1isize) as
                                         libc::c_int) as uint32_t;
                            let fresh202 = pc;
                            pc = pc.offset(1);
                            b = *fresh202 as uint16_t;
                            current_block = 8489059574810375089;
                        }
                        94 => {
                            pc = pc.offset(2isize);
                            a =
                                ((*pc.offset(-2isize).offset(0isize) as
                                      libc::c_int) << 8i32 |
                                     *pc.offset(-2isize).offset(1isize) as
                                         libc::c_int) as uint32_t;
                            let fresh203 = pc;
                            pc = pc.offset(1);
                            b = *fresh203 as uint16_t;
                            current_block = 12998570369541158573;
                        }
                        95 => {
                            pc = pc.offset(2isize);
                            a =
                                ((*pc.offset(-2isize).offset(0isize) as
                                      libc::c_int) << 8i32 |
                                     *pc.offset(-2isize).offset(1isize) as
                                         libc::c_int) as uint32_t;
                            current_block = 12775348822761942368;
                        }
                        96 => {
                            pc = pc.offset(2isize);
                            a =
                                ((*pc.offset(-2isize).offset(0isize) as
                                      libc::c_int) << 8i32 |
                                     *pc.offset(-2isize).offset(1isize) as
                                         libc::c_int) as uint32_t;
                            current_block = 355116111335096151;
                        }
                        97 => {
                            pc = pc.offset(2isize);
                            a =
                                ((*pc.offset(-2isize).offset(0isize) as
                                      libc::c_int) << 8i32 |
                                     *pc.offset(-2isize).offset(1isize) as
                                         libc::c_int) as uint32_t;
                            current_block = 890063714143163519;
                        }
                        98 => {
                            pc = pc.offset(2isize);
                            a =
                                ((*pc.offset(-2isize).offset(0isize) as
                                      libc::c_int) << 8i32 |
                                     *pc.offset(-2isize).offset(1isize) as
                                         libc::c_int) as uint32_t;
                            let fresh204 = pc;
                            pc = pc.offset(1);
                            b = *fresh204 as uint16_t;
                            let fresh205 = pc;
                            pc = pc.offset(1);
                            c = *fresh205;
                            current_block = 998308582751651876;
                        }
                        99 => {
                            pc = pc.offset(2isize);
                            a =
                                ((*pc.offset(-2isize).offset(0isize) as
                                      libc::c_int) << 8i32 |
                                     *pc.offset(-2isize).offset(1isize) as
                                         libc::c_int) as uint32_t;
                            current_block = 7833862538762852372;
                        }
                        100 => { current_block = 8667820544026455208; }
                        101 => { current_block = 10575315580790317560; }
                        102 => { current_block = 18239940301814930841; }
                        103 => { current_block = 10942739336945693405; }
                        _ => { current_block = 3935997918849240122; }
                    }
                }
                2384513197176890912 => {
                    printf(b"OP_RAISE\tR%d\t\t\x00" as *const u8 as
                               *const libc::c_char, a);
                    print_lv_a(mrb, irep, a as uint16_t);
                    current_block = 3935997918849240122;
                }
                5105253204732552581 => {
                    printf(b"OP_RESCUE\tR%d\tR%d\x00" as *const u8 as
                               *const libc::c_char, a, b as libc::c_int);
                    print_lv_ab(mrb, irep, a as uint16_t, b);
                    current_block = 3935997918849240122;
                }
                4275858079292575654 => {
                    printf(b"OP_EXCEPT\tR%d\t\t\x00" as *const u8 as
                               *const libc::c_char, a);
                    print_lv_a(mrb, irep, a as uint16_t);
                    current_block = 3935997918849240122;
                }
                890063714143163519 => {
                    printf(b"OP_TCLASS\tR%d\t\t\x00" as *const u8 as
                               *const libc::c_char, a);
                    print_lv_a(mrb, irep, a as uint16_t);
                    current_block = 3935997918849240122;
                }
                355116111335096151 => {
                    printf(b"OP_SCLASS\tR%d\t\x00" as *const u8 as
                               *const libc::c_char, a);
                    print_lv_a(mrb, irep, a as uint16_t);
                    current_block = 3935997918849240122;
                }
                2499490863002033965 => {
                    printf(b"OP_EXEC\tR%d\tI(%d:%p)\x00" as *const u8 as
                               *const libc::c_char, a, b as libc::c_int,
                           *(*irep).reps.offset(b as isize));
                    print_lv_a(mrb, irep, a as uint16_t);
                    current_block = 3935997918849240122;
                }
                7493503892657452062 => {
                    printf(b"OP_MODULE\tR%d\t:%s\x00" as *const u8 as
                               *const libc::c_char, a,
                           mrb_sym2name(mrb,
                                        *(*irep).syms.offset(b as isize)));
                    print_lv_a(mrb, irep, a as uint16_t);
                    current_block = 3935997918849240122;
                }
                3258098155381626199 => {
                    printf(b"OP_CLASS\tR%d\t:%s\x00" as *const u8 as
                               *const libc::c_char, a,
                           mrb_sym2name(mrb,
                                        *(*irep).syms.offset(b as isize)));
                    print_lv_a(mrb, irep, a as uint16_t);
                    current_block = 3935997918849240122;
                }
                2968065794317548828 => {
                    printf(b"OP_OCLASS\tR%d\t\t\x00" as *const u8 as
                               *const libc::c_char, a);
                    print_lv_a(mrb, irep, a as uint16_t);
                    current_block = 3935997918849240122;
                }
                14386882892843117929 => {
                    printf(b"OP_HASHCAT\tR%d\t\x00" as *const u8 as
                               *const libc::c_char, a);
                    print_lv_a(mrb, irep, a as uint16_t);
                    current_block = 3935997918849240122;
                }
                5341326654522537507 => {
                    printf(b"OP_HASHADD\tR%d\t%d\t\x00" as *const u8 as
                               *const libc::c_char, a, b as libc::c_int);
                    print_lv_a(mrb, irep, a as uint16_t);
                    current_block = 3935997918849240122;
                }
                314002374245450226 => {
                    printf(b"OP_HASH\tR%d\t%d\t\x00" as *const u8 as
                               *const libc::c_char, a, b as libc::c_int);
                    print_lv_a(mrb, irep, a as uint16_t);
                    current_block = 3935997918849240122;
                }
                13433017329934546124 => {
                    printf(b"OP_STRCAT\tR%d\t\x00" as *const u8 as
                               *const libc::c_char, a);
                    print_lv_a(mrb, irep, a as uint16_t);
                    current_block = 3935997918849240122;
                }
                16677287981175421660 => {
                    let mut v_0: mrb_value = *(*irep).pool.offset(b as isize);
                    let mut s_1: mrb_value =
                        mrb_str_dump(mrb,
                                     mrb_str_new(mrb,
                                                 if 0 !=
                                                        (*(v_0.value.p as
                                                               *mut RString)).flags()
                                                            as libc::c_int &
                                                            32i32 {
                                                     (*(v_0.value.p as
                                                            *mut RString)).as_0.ary.as_mut_ptr()
                                                 } else {
                                                     (*(v_0.value.p as
                                                            *mut RString)).as_0.heap.ptr
                                                 },
                                                 (if 0 !=
                                                         (*(v_0.value.p as
                                                                *mut RString)).flags()
                                                             as libc::c_int &
                                                             32i32 {
                                                      (((*(v_0.value.p as
                                                               *mut RString)).flags()
                                                            as libc::c_int &
                                                            0x7c0i32) >> 6i32)
                                                          as mrb_int
                                                  } else {
                                                      (*(v_0.value.p as
                                                             *mut RString)).as_0.heap.len
                                                  }) as size_t));
                    printf(b"OP_STRING\tR%d\tL(%d)\t; %s\x00" as *const u8 as
                               *const libc::c_char, a, b as libc::c_int,
                           if 0 !=
                                  (*(s_1.value.p as *mut RString)).flags() as
                                      libc::c_int & 32i32 {
                               (*(s_1.value.p as
                                      *mut RString)).as_0.ary.as_mut_ptr()
                           } else {
                               (*(s_1.value.p as *mut RString)).as_0.heap.ptr
                           });
                    print_lv_a(mrb, irep, a as uint16_t);
                    current_block = 3935997918849240122;
                }
                7127613333771968219 => {
                    printf(b"OP_INTERN\tR%d\x00" as *const u8 as
                               *const libc::c_char, a);
                    print_lv_a(mrb, irep, a as uint16_t);
                    current_block = 3935997918849240122;
                }
                10671272775318305553 => {
                    printf(b"OP_APOST\tR%d\t%d\t%d\x00" as *const u8 as
                               *const libc::c_char, a, b as libc::c_int,
                           c as libc::c_int);
                    print_lv_a(mrb, irep, a as uint16_t);
                    current_block = 3935997918849240122;
                }
                11441496697127961056 => {
                    printf(b"OP_ASET\tR%d\tR%d\t%d\x00" as *const u8 as
                               *const libc::c_char, a, b as libc::c_int,
                           c as libc::c_int);
                    print_lv_ab(mrb, irep, a as uint16_t, b);
                    current_block = 3935997918849240122;
                }
                9599741646002396977 => {
                    printf(b"OP_AREF\tR%d\tR%d\t%d\x00" as *const u8 as
                               *const libc::c_char, a, b as libc::c_int,
                           c as libc::c_int);
                    print_lv_ab(mrb, irep, a as uint16_t, b);
                    current_block = 3935997918849240122;
                }
                1606585337627918712 => {
                    printf(b"OP_ARYDUP\tR%d\t\x00" as *const u8 as
                               *const libc::c_char, a);
                    print_lv_a(mrb, irep, a as uint16_t);
                    current_block = 3935997918849240122;
                }
                18022542853155082387 => {
                    printf(b"OP_ARYPUSH\tR%d\t\x00" as *const u8 as
                               *const libc::c_char, a);
                    print_lv_a(mrb, irep, a as uint16_t);
                    current_block = 3935997918849240122;
                }
                6053772697959983332 => {
                    printf(b"OP_ARYCAT\tR%d\t\x00" as *const u8 as
                               *const libc::c_char, a);
                    print_lv_a(mrb, irep, a as uint16_t);
                    current_block = 3935997918849240122;
                }
                1741746549607172898 => {
                    printf(b"OP_ARRAY\tR%d\tR%d\t%d\t\x00" as *const u8 as
                               *const libc::c_char, a, b as libc::c_int,
                           c as libc::c_int);
                    print_lv_ab(mrb, irep, a as uint16_t, b);
                    current_block = 3935997918849240122;
                }
                4850402844069062092 => {
                    printf(b"OP_ARRAY\tR%d\t%d\t\x00" as *const u8 as
                               *const libc::c_char, a, b as libc::c_int);
                    print_lv_a(mrb, irep, a as uint16_t);
                    current_block = 3935997918849240122;
                }
                5611126728812655874 => {
                    printf(b"OP_BLKPUSH\tR%d\t%d:%d:%d:%d (%d)\x00" as
                               *const u8 as *const libc::c_char, a,
                           b as libc::c_int >> 11i32 & 0x3fi32,
                           b as libc::c_int >> 10i32 & 0x1i32,
                           b as libc::c_int >> 5i32 & 0x1fi32,
                           b as libc::c_int >> 4i32 & 0x1i32,
                           b as libc::c_int >> 0i32 & 0xfi32);
                    print_lv_a(mrb, irep, a as uint16_t);
                    current_block = 3935997918849240122;
                }
                6968106866976024785 => {
                    printf(b"OP_BREAK\tR%d\t\t\x00" as *const u8 as
                               *const libc::c_char, a);
                    print_lv_a(mrb, irep, a as uint16_t);
                    current_block = 3935997918849240122;
                }
                15158068222670743697 => {
                    printf(b"OP_RETURN_BLK\tR%d\t\t\x00" as *const u8 as
                               *const libc::c_char, a);
                    print_lv_a(mrb, irep, a as uint16_t);
                    current_block = 3935997918849240122;
                }
                15446005597338724160 => {
                    printf(b"OP_RETURN\tR%d\t\t\x00" as *const u8 as
                               *const libc::c_char, a);
                    print_lv_a(mrb, irep, a as uint16_t);
                    current_block = 3935997918849240122;
                }
                13480378041738100027 => {
                    printf(b"OP_KARG\tR%d\t:%s\t\x00" as *const u8 as
                               *const libc::c_char, a,
                           mrb_sym2name(mrb,
                                        *(*irep).syms.offset(b as isize)));
                    print_lv_a(mrb, irep, a as uint16_t);
                    current_block = 3935997918849240122;
                }
                17792945831448561469 => {
                    printf(b"OP_KEY_P\tR%d\t:%s\t\x00" as *const u8 as
                               *const libc::c_char, a,
                           mrb_sym2name(mrb,
                                        *(*irep).syms.offset(b as isize)));
                    print_lv_a(mrb, irep, a as uint16_t);
                    current_block = 3935997918849240122;
                }
                4063058694607173453 => {
                    printf(b"OP_ARGARY\tR%d\t%d:%d:%d:%d (%d)\x00" as
                               *const u8 as *const libc::c_char, a,
                           b as libc::c_int >> 11i32 & 0x3fi32,
                           b as libc::c_int >> 10i32 & 0x1i32,
                           b as libc::c_int >> 5i32 & 0x1fi32,
                           b as libc::c_int >> 4i32 & 0x1i32,
                           b as libc::c_int >> 0i32 & 0xfi32);
                    print_lv_a(mrb, irep, a as uint16_t);
                    current_block = 3935997918849240122;
                }
                12781735722417726073 => {
                    printf(b"OP_JMPNIL\tR%d\t%03d\t\x00" as *const u8 as
                               *const libc::c_char, a, b as libc::c_int);
                    print_lv_a(mrb, irep, a as uint16_t);
                    current_block = 3935997918849240122;
                }
                17639296353235453144 => {
                    printf(b"OP_JMPNOT\tR%d\t%03d\t\x00" as *const u8 as
                               *const libc::c_char, a, b as libc::c_int);
                    print_lv_a(mrb, irep, a as uint16_t);
                    current_block = 3935997918849240122;
                }
                9865490829578899964 => {
                    printf(b"OP_JMPIF\tR%d\t%03d\t\x00" as *const u8 as
                               *const libc::c_char, a, b as libc::c_int);
                    print_lv_a(mrb, irep, a as uint16_t);
                    current_block = 3935997918849240122;
                }
                7849445245878263537 => {
                    printf(b"OP_SETCV\t%s\tR%d\x00" as *const u8 as
                               *const libc::c_char,
                           mrb_sym2name(mrb,
                                        *(*irep).syms.offset(b as isize)), a);
                    print_lv_a(mrb, irep, a as uint16_t);
                    current_block = 3935997918849240122;
                }
                1863480813282067938 => {
                    printf(b"OP_GETCV\tR%d\t%s\x00" as *const u8 as
                               *const libc::c_char, a,
                           mrb_sym2name(mrb,
                                        *(*irep).syms.offset(b as isize)));
                    print_lv_a(mrb, irep, a as uint16_t);
                    current_block = 3935997918849240122;
                }
                9015561896112508021 => {
                    printf(b"OP_SETUPVAR\tR%d\t%d\t%d\x00" as *const u8 as
                               *const libc::c_char, a, b as libc::c_int,
                           c as libc::c_int);
                    print_lv_a(mrb, irep, a as uint16_t);
                    current_block = 3935997918849240122;
                }
                10547193261004346349 => {
                    printf(b"OP_GETUPVAR\tR%d\t%d\t%d\x00" as *const u8 as
                               *const libc::c_char, a, b as libc::c_int,
                           c as libc::c_int);
                    print_lv_a(mrb, irep, a as uint16_t);
                    current_block = 3935997918849240122;
                }
                7570639851576535195 => {
                    printf(b"OP_SETIV\t%s\tR%d\x00" as *const u8 as
                               *const libc::c_char,
                           mrb_sym2name(mrb,
                                        *(*irep).syms.offset(b as isize)), a);
                    print_lv_a(mrb, irep, a as uint16_t);
                    current_block = 3935997918849240122;
                }
                7446123777345652171 => {
                    printf(b"OP_GETIV\tR%d\t%s\x00" as *const u8 as
                               *const libc::c_char, a,
                           mrb_sym2name(mrb,
                                        *(*irep).syms.offset(b as isize)));
                    print_lv_a(mrb, irep, a as uint16_t);
                    current_block = 3935997918849240122;
                }
                9704183418455961811 => {
                    printf(b"OP_SETMCNST\tR%d::%s\tR%d\x00" as *const u8 as
                               *const libc::c_char,
                           a.wrapping_add(1i32 as libc::c_uint),
                           mrb_sym2name(mrb,
                                        *(*irep).syms.offset(b as isize)), a);
                    print_lv_a(mrb, irep, a as uint16_t);
                    current_block = 3935997918849240122;
                }
                1873199301525838551 => {
                    printf(b"OP_GETMCNST\tR%d\tR%d::%s\x00" as *const u8 as
                               *const libc::c_char, a, a,
                           mrb_sym2name(mrb,
                                        *(*irep).syms.offset(b as isize)));
                    print_lv_a(mrb, irep, a as uint16_t);
                    current_block = 3935997918849240122;
                }
                942271536672158721 => {
                    printf(b"OP_SETCONST\t:%s\tR%d\x00" as *const u8 as
                               *const libc::c_char,
                           mrb_sym2name(mrb,
                                        *(*irep).syms.offset(b as isize)), a);
                    print_lv_a(mrb, irep, a as uint16_t);
                    current_block = 3935997918849240122;
                }
                16034675043887719473 => {
                    printf(b"OP_GETCONST\tR%d\t:%s\x00" as *const u8 as
                               *const libc::c_char, a,
                           mrb_sym2name(mrb,
                                        *(*irep).syms.offset(b as isize)));
                    print_lv_a(mrb, irep, a as uint16_t);
                    current_block = 3935997918849240122;
                }
                4481804007556508143 => {
                    printf(b"OP_SETSV\t:%s\tR%d\x00" as *const u8 as
                               *const libc::c_char,
                           mrb_sym2name(mrb,
                                        *(*irep).syms.offset(b as isize)), a);
                    print_lv_a(mrb, irep, a as uint16_t);
                    current_block = 3935997918849240122;
                }
                16758983331069857237 => {
                    printf(b"OP_GETSV\tR%d\t:%s\x00" as *const u8 as
                               *const libc::c_char, a,
                           mrb_sym2name(mrb,
                                        *(*irep).syms.offset(b as isize)));
                    print_lv_a(mrb, irep, a as uint16_t);
                    current_block = 3935997918849240122;
                }
                16200511029630275951 => {
                    printf(b"OP_SETGV\t:%s\tR%d\x00" as *const u8 as
                               *const libc::c_char,
                           mrb_sym2name(mrb,
                                        *(*irep).syms.offset(b as isize)), a);
                    print_lv_a(mrb, irep, a as uint16_t);
                    current_block = 3935997918849240122;
                }
                17386262169460468759 => {
                    printf(b"OP_GETGV\tR%d\t:%s\x00" as *const u8 as
                               *const libc::c_char, a,
                           mrb_sym2name(mrb,
                                        *(*irep).syms.offset(b as isize)));
                    print_lv_a(mrb, irep, a as uint16_t);
                    current_block = 3935997918849240122;
                }
                2779364454669598918 => {
                    printf(b"OP_LOADF\tR%d\t\t\x00" as *const u8 as
                               *const libc::c_char, a);
                    print_lv_a(mrb, irep, a as uint16_t);
                    current_block = 3935997918849240122;
                }
                4255402116796946452 => {
                    printf(b"OP_LOADT\tR%d\t\t\x00" as *const u8 as
                               *const libc::c_char, a);
                    print_lv_a(mrb, irep, a as uint16_t);
                    current_block = 3935997918849240122;
                }
                12915432016225878389 => {
                    printf(b"OP_LOADSELF\tR%d\t\t\x00" as *const u8 as
                               *const libc::c_char, a);
                    print_lv_a(mrb, irep, a as uint16_t);
                    current_block = 3935997918849240122;
                }
                17940677572951814003 => {
                    printf(b"OP_LOADNIL\tR%d\t\t\x00" as *const u8 as
                               *const libc::c_char, a);
                    print_lv_a(mrb, irep, a as uint16_t);
                    current_block = 3935997918849240122;
                }
                16157800273112125286 => {
                    printf(b"OP_LOADSYM\tR%d\t:%s\t\x00" as *const u8 as
                               *const libc::c_char, a,
                           mrb_sym2name(mrb,
                                        *(*irep).syms.offset(b as isize)));
                    print_lv_a(mrb, irep, a as uint16_t);
                    current_block = 3935997918849240122;
                }
                8096298871650922110 => {
                    printf(b"OP_LOADI_%d\tR%d\t\t\x00" as *const u8 as
                               *const libc::c_char,
                           ins as libc::c_int -
                               OP_LOADI_0 as libc::c_int as libc::c_int, a);
                    print_lv_a(mrb, irep, a as uint16_t);
                    current_block = 3935997918849240122;
                }
                6276274620003476740 => {
                    printf(b"OP_LOADI__1\tR%d\t\t\x00" as *const u8 as
                               *const libc::c_char, a);
                    print_lv_a(mrb, irep, a as uint16_t);
                    current_block = 3935997918849240122;
                }
                4843221280033581897 => {
                    printf(b"OP_LOADI\tR%d\t-%d\t\x00" as *const u8 as
                               *const libc::c_char, a, b as libc::c_int);
                    print_lv_a(mrb, irep, a as uint16_t);
                    current_block = 3935997918849240122;
                }
                8703264613394043654 => {
                    printf(b"OP_LOADI\tR%d\t%d\t\x00" as *const u8 as
                               *const libc::c_char, a, b as libc::c_int);
                    print_lv_a(mrb, irep, a as uint16_t);
                    current_block = 3935997918849240122;
                }
                14904807116429336826 => {
                    let mut v: mrb_value = *(*irep).pool.offset(b as isize);
                    let mut s_0: mrb_value = mrb_inspect(mrb, v);
                    printf(b"OP_LOADL\tR%d\tL(%d)\t; %s\x00" as *const u8 as
                               *const libc::c_char, a, b as libc::c_int,
                           if 0 !=
                                  (*(s_0.value.p as *mut RString)).flags() as
                                      libc::c_int & 32i32 {
                               (*(s_0.value.p as
                                      *mut RString)).as_0.ary.as_mut_ptr()
                           } else {
                               (*(s_0.value.p as *mut RString)).as_0.heap.ptr
                           });
                    print_lv_a(mrb, irep, a as uint16_t);
                    current_block = 3935997918849240122;
                }
                3345631096401374476 => {
                    printf(b"OP_MOVE\tR%d\tR%d\t\x00" as *const u8 as
                               *const libc::c_char, a, b as libc::c_int);
                    print_lv_ab(mrb, irep, a as uint16_t, b);
                    current_block = 3935997918849240122;
                }
                4153728562337695617 => {
                    printf(b"OP_NOP\n\x00" as *const u8 as
                               *const libc::c_char);
                    current_block = 3935997918849240122;
                }
                11480039587962891479 => {
                    printf(b"OP_CALL\n\x00" as *const u8 as
                               *const libc::c_char);
                    current_block = 3935997918849240122;
                }
                11667657158681136576 => {
                    printf(b"OP_KEYEND\n\x00" as *const u8 as
                               *const libc::c_char);
                    current_block = 3935997918849240122;
                }
                10942739336945693405 => {
                    printf(b"OP_STOP\n\x00" as *const u8 as
                               *const libc::c_char);
                    current_block = 3935997918849240122;
                }
                10458688773482078930 => {
                    printf(b"OP_JMP\t\t%03d\n\x00" as *const u8 as
                               *const libc::c_char, a);
                    current_block = 3935997918849240122;
                }
                12846811306833230447 => {
                    printf(b"OP_SENDV\tR%d\t:%s\n\x00" as *const u8 as
                               *const libc::c_char, a,
                           mrb_sym2name(mrb,
                                        *(*irep).syms.offset(b as isize)));
                    current_block = 3935997918849240122;
                }
                10246497873717858537 => {
                    printf(b"OP_SENDVB\tR%d\t:%s\n\x00" as *const u8 as
                               *const libc::c_char, a,
                           mrb_sym2name(mrb,
                                        *(*irep).syms.offset(b as isize)));
                    current_block = 3935997918849240122;
                }
                5602041749335279496 => {
                    printf(b"OP_SEND\tR%d\t:%s\t%d\n\x00" as *const u8 as
                               *const libc::c_char, a,
                           mrb_sym2name(mrb,
                                        *(*irep).syms.offset(b as isize)),
                           c as libc::c_int);
                    current_block = 3935997918849240122;
                }
                3906616468301123675 => {
                    printf(b"OP_SENDB\tR%d\t:%s\t%d\n\x00" as *const u8 as
                               *const libc::c_char, a,
                           mrb_sym2name(mrb,
                                        *(*irep).syms.offset(b as isize)),
                           c as libc::c_int);
                    current_block = 3935997918849240122;
                }
                5769007513321684282 => {
                    printf(b"OP_SUPER\tR%d\t%d\n\x00" as *const u8 as
                               *const libc::c_char, a, b as libc::c_int);
                    current_block = 3935997918849240122;
                }
                17974925641258900466 => {
                    printf(b"OP_ENTER\t%d:%d:%d:%d:%d:%d:%d\n\x00" as
                               *const u8 as *const libc::c_char,
                           a >> 18i32 & 0x1fi32 as libc::c_uint,
                           a >> 13i32 & 0x1fi32 as libc::c_uint,
                           a >> 12i32 & 0x1i32 as libc::c_uint,
                           a >> 7i32 & 0x1fi32 as libc::c_uint,
                           a >> 2i32 & 0x1fi32 as libc::c_uint,
                           a & (1i32 << 1i32) as libc::c_uint,
                           a & 1i32 as libc::c_uint);
                    current_block = 3935997918849240122;
                }
                3020430753860169586 => {
                    printf(b"OP_LAMBDA\tR%d\tI(%d:%p)\n\x00" as *const u8 as
                               *const libc::c_char, a, b as libc::c_int,
                           *(*irep).reps.offset(b as isize));
                    current_block = 3935997918849240122;
                }
                11988973803779423327 => {
                    printf(b"OP_BLOCK\tR%d\tI(%d:%p)\n\x00" as *const u8 as
                               *const libc::c_char, a, b as libc::c_int,
                           *(*irep).reps.offset(b as isize));
                    current_block = 3935997918849240122;
                }
                10561944073379837955 => {
                    printf(b"OP_METHOD\tR%d\tI(%d:%p)\n\x00" as *const u8 as
                               *const libc::c_char, a, b as libc::c_int,
                           *(*irep).reps.offset(b as isize));
                    current_block = 3935997918849240122;
                }
                9864403379770423142 => {
                    printf(b"OP_RANGE_INC\tR%d\n\x00" as *const u8 as
                               *const libc::c_char, a);
                    current_block = 3935997918849240122;
                }
                8124361700674833375 => {
                    printf(b"OP_RANGE_EXC\tR%d\n\x00" as *const u8 as
                               *const libc::c_char, a);
                    current_block = 3935997918849240122;
                }
                8489059574810375089 => {
                    printf(b"OP_DEF\tR%d\t:%s\n\x00" as *const u8 as
                               *const libc::c_char, a,
                           mrb_sym2name(mrb,
                                        *(*irep).syms.offset(b as isize)));
                    current_block = 3935997918849240122;
                }
                12775348822761942368 => {
                    printf(b"OP_UNDEF\t:%s\n\x00" as *const u8 as
                               *const libc::c_char,
                           mrb_sym2name(mrb,
                                        *(*irep).syms.offset(a as isize)));
                    current_block = 3935997918849240122;
                }
                12998570369541158573 => {
                    printf(b"OP_ALIAS\t:%s\t%s\n\x00" as *const u8 as
                               *const libc::c_char,
                           mrb_sym2name(mrb,
                                        *(*irep).syms.offset(a as isize)),
                           mrb_sym2name(mrb,
                                        *(*irep).syms.offset(b as isize)));
                    current_block = 3935997918849240122;
                }
                328988732995390160 => {
                    printf(b"OP_ADD\tR%d\t\n\x00" as *const u8 as
                               *const libc::c_char, a);
                    current_block = 3935997918849240122;
                }
                10541196509243133637 => {
                    printf(b"OP_ADDI\tR%d\t%d\n\x00" as *const u8 as
                               *const libc::c_char, a, b as libc::c_int);
                    current_block = 3935997918849240122;
                }
                16989604180394837074 => {
                    printf(b"OP_SUB\tR%d\t\n\x00" as *const u8 as
                               *const libc::c_char, a);
                    current_block = 3935997918849240122;
                }
                15326039268735274622 => {
                    printf(b"OP_SUBI\tR%d\t%d\n\x00" as *const u8 as
                               *const libc::c_char, a, b as libc::c_int);
                    current_block = 3935997918849240122;
                }
                6846200202819738136 => {
                    printf(b"OP_MUL\tR%d\t\n\x00" as *const u8 as
                               *const libc::c_char, a);
                    current_block = 3935997918849240122;
                }
                16948125840707872018 => {
                    printf(b"OP_DIV\tR%d\t\n\x00" as *const u8 as
                               *const libc::c_char, a);
                    current_block = 3935997918849240122;
                }
                1975408140333322065 => {
                    printf(b"OP_LT\t\tR%d\t\n\x00" as *const u8 as
                               *const libc::c_char, a);
                    current_block = 3935997918849240122;
                }
                12696324532854772620 => {
                    printf(b"OP_LE\t\tR%d\t\n\x00" as *const u8 as
                               *const libc::c_char, a);
                    current_block = 3935997918849240122;
                }
                3433849069680490924 => {
                    printf(b"OP_GT\t\tR%d\t\n\x00" as *const u8 as
                               *const libc::c_char, a);
                    current_block = 3935997918849240122;
                }
                10072772881953407077 => {
                    printf(b"OP_GE\t\tR%d\t\n\x00" as *const u8 as
                               *const libc::c_char, a);
                    current_block = 3935997918849240122;
                }
                11372858789397587571 => {
                    printf(b"OP_EQ\t\tR%d\t\n\x00" as *const u8 as
                               *const libc::c_char, a);
                    current_block = 3935997918849240122;
                }
                7833862538762852372 => {
                    let mut v_1: mrb_value = *(*irep).pool.offset(a as isize);
                    let mut s_2: mrb_value =
                        mrb_str_dump(mrb,
                                     mrb_str_new(mrb,
                                                 if 0 !=
                                                        (*(v_1.value.p as
                                                               *mut RString)).flags()
                                                            as libc::c_int &
                                                            32i32 {
                                                     (*(v_1.value.p as
                                                            *mut RString)).as_0.ary.as_mut_ptr()
                                                 } else {
                                                     (*(v_1.value.p as
                                                            *mut RString)).as_0.heap.ptr
                                                 },
                                                 (if 0 !=
                                                         (*(v_1.value.p as
                                                                *mut RString)).flags()
                                                             as libc::c_int &
                                                             32i32 {
                                                      (((*(v_1.value.p as
                                                               *mut RString)).flags()
                                                            as libc::c_int &
                                                            0x7c0i32) >> 6i32)
                                                          as mrb_int
                                                  } else {
                                                      (*(v_1.value.p as
                                                             *mut RString)).as_0.heap.len
                                                  }) as size_t));
                    printf(b"OP_ERR\t%s\n\x00" as *const u8 as
                               *const libc::c_char,
                           if 0 !=
                                  (*(s_2.value.p as *mut RString)).flags() as
                                      libc::c_int & 32i32 {
                               (*(s_2.value.p as
                                      *mut RString)).as_0.ary.as_mut_ptr()
                           } else {
                               (*(s_2.value.p as *mut RString)).as_0.heap.ptr
                           });
                    current_block = 3935997918849240122;
                }
                1367206831037961075 => {
                    printf(b"OP_EPUSH\t\t:I(%d:%p)\n\x00" as *const u8 as
                               *const libc::c_char, a,
                           *(*irep).reps.offset(a as isize));
                    current_block = 3935997918849240122;
                }
                6898479631119937405 => {
                    printf(b"OP_ONERR\t%03d\n\x00" as *const u8 as
                               *const libc::c_char, a);
                    current_block = 3935997918849240122;
                }
                1459864638523139318 => {
                    printf(b"OP_POPERR\t%d\t\t\n\x00" as *const u8 as
                               *const libc::c_char, a);
                    current_block = 3935997918849240122;
                }
                6367683481748506220 => {
                    printf(b"OP_EPOP\t%d\n\x00" as *const u8 as
                               *const libc::c_char, a);
                    current_block = 3935997918849240122;
                }
                998308582751651876 => {
                    printf(b"OP_DEBUG\t%d\t%d\t%d\n\x00" as *const u8 as
                               *const libc::c_char, a, b as libc::c_int,
                           c as libc::c_int);
                    current_block = 3935997918849240122;
                }
                _ => {
                    let fresh310 = pc;
                    pc = pc.offset(1);
                    ins = *fresh310;
                    printf(b"OP_EXT3\n\x00" as *const u8 as
                               *const libc::c_char);
                    print_header(mrb, irep,
                                 pc.wrapping_offset_from((*irep).iseq) as
                                     libc::c_long - 2i32 as libc::c_long);
                    match ins as libc::c_int {
                        0 => { current_block = 4153728562337695617; }
                        1 => {
                            pc = pc.offset(2isize);
                            a =
                                ((*pc.offset(-2isize).offset(0isize) as
                                      libc::c_int) << 8i32 |
                                     *pc.offset(-2isize).offset(1isize) as
                                         libc::c_int) as uint32_t;
                            pc = pc.offset(2isize);
                            b =
                                ((*pc.offset(-2isize).offset(0isize) as
                                      libc::c_int) << 8i32 |
                                     *pc.offset(-2isize).offset(1isize) as
                                         libc::c_int) as uint16_t;
                            current_block = 3345631096401374476;
                        }
                        2 => {
                            pc = pc.offset(2isize);
                            a =
                                ((*pc.offset(-2isize).offset(0isize) as
                                      libc::c_int) << 8i32 |
                                     *pc.offset(-2isize).offset(1isize) as
                                         libc::c_int) as uint32_t;
                            pc = pc.offset(2isize);
                            b =
                                ((*pc.offset(-2isize).offset(0isize) as
                                      libc::c_int) << 8i32 |
                                     *pc.offset(-2isize).offset(1isize) as
                                         libc::c_int) as uint16_t;
                            current_block = 14904807116429336826;
                        }
                        3 => {
                            pc = pc.offset(2isize);
                            a =
                                ((*pc.offset(-2isize).offset(0isize) as
                                      libc::c_int) << 8i32 |
                                     *pc.offset(-2isize).offset(1isize) as
                                         libc::c_int) as uint32_t;
                            pc = pc.offset(2isize);
                            b =
                                ((*pc.offset(-2isize).offset(0isize) as
                                      libc::c_int) << 8i32 |
                                     *pc.offset(-2isize).offset(1isize) as
                                         libc::c_int) as uint16_t;
                            current_block = 8703264613394043654;
                        }
                        4 => {
                            pc = pc.offset(2isize);
                            a =
                                ((*pc.offset(-2isize).offset(0isize) as
                                      libc::c_int) << 8i32 |
                                     *pc.offset(-2isize).offset(1isize) as
                                         libc::c_int) as uint32_t;
                            pc = pc.offset(2isize);
                            b =
                                ((*pc.offset(-2isize).offset(0isize) as
                                      libc::c_int) << 8i32 |
                                     *pc.offset(-2isize).offset(1isize) as
                                         libc::c_int) as uint16_t;
                            current_block = 4843221280033581897;
                        }
                        5 => {
                            let fresh311 = pc;
                            pc = pc.offset(1);
                            a = *fresh311 as uint32_t;
                            current_block = 6276274620003476740;
                        }
                        6 => {
                            let fresh312 = pc;
                            pc = pc.offset(1);
                            a = *fresh312 as uint32_t;
                            current_block = 8096298871650922110;
                        }
                        7 => {
                            let fresh313 = pc;
                            pc = pc.offset(1);
                            a = *fresh313 as uint32_t;
                            current_block = 8096298871650922110;
                        }
                        8 => {
                            let fresh314 = pc;
                            pc = pc.offset(1);
                            a = *fresh314 as uint32_t;
                            current_block = 8096298871650922110;
                        }
                        9 => {
                            let fresh315 = pc;
                            pc = pc.offset(1);
                            a = *fresh315 as uint32_t;
                            current_block = 8096298871650922110;
                        }
                        10 => {
                            let fresh316 = pc;
                            pc = pc.offset(1);
                            a = *fresh316 as uint32_t;
                            current_block = 8096298871650922110;
                        }
                        11 => {
                            let fresh317 = pc;
                            pc = pc.offset(1);
                            a = *fresh317 as uint32_t;
                            current_block = 8096298871650922110;
                        }
                        12 => {
                            let fresh318 = pc;
                            pc = pc.offset(1);
                            a = *fresh318 as uint32_t;
                            current_block = 8096298871650922110;
                        }
                        13 => {
                            let fresh319 = pc;
                            pc = pc.offset(1);
                            a = *fresh319 as uint32_t;
                            current_block = 8096298871650922110;
                        }
                        14 => {
                            pc = pc.offset(2isize);
                            a =
                                ((*pc.offset(-2isize).offset(0isize) as
                                      libc::c_int) << 8i32 |
                                     *pc.offset(-2isize).offset(1isize) as
                                         libc::c_int) as uint32_t;
                            pc = pc.offset(2isize);
                            b =
                                ((*pc.offset(-2isize).offset(0isize) as
                                      libc::c_int) << 8i32 |
                                     *pc.offset(-2isize).offset(1isize) as
                                         libc::c_int) as uint16_t;
                            current_block = 16157800273112125286;
                        }
                        15 => {
                            let fresh320 = pc;
                            pc = pc.offset(1);
                            a = *fresh320 as uint32_t;
                            current_block = 17940677572951814003;
                        }
                        16 => {
                            let fresh321 = pc;
                            pc = pc.offset(1);
                            a = *fresh321 as uint32_t;
                            current_block = 12915432016225878389;
                        }
                        17 => {
                            let fresh322 = pc;
                            pc = pc.offset(1);
                            a = *fresh322 as uint32_t;
                            current_block = 4255402116796946452;
                        }
                        18 => {
                            let fresh323 = pc;
                            pc = pc.offset(1);
                            a = *fresh323 as uint32_t;
                            current_block = 2779364454669598918;
                        }
                        19 => {
                            pc = pc.offset(2isize);
                            a =
                                ((*pc.offset(-2isize).offset(0isize) as
                                      libc::c_int) << 8i32 |
                                     *pc.offset(-2isize).offset(1isize) as
                                         libc::c_int) as uint32_t;
                            pc = pc.offset(2isize);
                            b =
                                ((*pc.offset(-2isize).offset(0isize) as
                                      libc::c_int) << 8i32 |
                                     *pc.offset(-2isize).offset(1isize) as
                                         libc::c_int) as uint16_t;
                            current_block = 17386262169460468759;
                        }
                        20 => {
                            pc = pc.offset(2isize);
                            a =
                                ((*pc.offset(-2isize).offset(0isize) as
                                      libc::c_int) << 8i32 |
                                     *pc.offset(-2isize).offset(1isize) as
                                         libc::c_int) as uint32_t;
                            pc = pc.offset(2isize);
                            b =
                                ((*pc.offset(-2isize).offset(0isize) as
                                      libc::c_int) << 8i32 |
                                     *pc.offset(-2isize).offset(1isize) as
                                         libc::c_int) as uint16_t;
                            current_block = 16200511029630275951;
                        }
                        21 => {
                            pc = pc.offset(2isize);
                            a =
                                ((*pc.offset(-2isize).offset(0isize) as
                                      libc::c_int) << 8i32 |
                                     *pc.offset(-2isize).offset(1isize) as
                                         libc::c_int) as uint32_t;
                            pc = pc.offset(2isize);
                            b =
                                ((*pc.offset(-2isize).offset(0isize) as
                                      libc::c_int) << 8i32 |
                                     *pc.offset(-2isize).offset(1isize) as
                                         libc::c_int) as uint16_t;
                            current_block = 16758983331069857237;
                        }
                        22 => {
                            pc = pc.offset(2isize);
                            a =
                                ((*pc.offset(-2isize).offset(0isize) as
                                      libc::c_int) << 8i32 |
                                     *pc.offset(-2isize).offset(1isize) as
                                         libc::c_int) as uint32_t;
                            pc = pc.offset(2isize);
                            b =
                                ((*pc.offset(-2isize).offset(0isize) as
                                      libc::c_int) << 8i32 |
                                     *pc.offset(-2isize).offset(1isize) as
                                         libc::c_int) as uint16_t;
                            current_block = 4481804007556508143;
                        }
                        23 => {
                            pc = pc.offset(2isize);
                            a =
                                ((*pc.offset(-2isize).offset(0isize) as
                                      libc::c_int) << 8i32 |
                                     *pc.offset(-2isize).offset(1isize) as
                                         libc::c_int) as uint32_t;
                            pc = pc.offset(2isize);
                            b =
                                ((*pc.offset(-2isize).offset(0isize) as
                                      libc::c_int) << 8i32 |
                                     *pc.offset(-2isize).offset(1isize) as
                                         libc::c_int) as uint16_t;
                            current_block = 7446123777345652171;
                        }
                        24 => {
                            pc = pc.offset(2isize);
                            a =
                                ((*pc.offset(-2isize).offset(0isize) as
                                      libc::c_int) << 8i32 |
                                     *pc.offset(-2isize).offset(1isize) as
                                         libc::c_int) as uint32_t;
                            pc = pc.offset(2isize);
                            b =
                                ((*pc.offset(-2isize).offset(0isize) as
                                      libc::c_int) << 8i32 |
                                     *pc.offset(-2isize).offset(1isize) as
                                         libc::c_int) as uint16_t;
                            current_block = 7570639851576535195;
                        }
                        25 => {
                            pc = pc.offset(2isize);
                            a =
                                ((*pc.offset(-2isize).offset(0isize) as
                                      libc::c_int) << 8i32 |
                                     *pc.offset(-2isize).offset(1isize) as
                                         libc::c_int) as uint32_t;
                            pc = pc.offset(2isize);
                            b =
                                ((*pc.offset(-2isize).offset(0isize) as
                                      libc::c_int) << 8i32 |
                                     *pc.offset(-2isize).offset(1isize) as
                                         libc::c_int) as uint16_t;
                            current_block = 1863480813282067938;
                        }
                        26 => {
                            pc = pc.offset(2isize);
                            a =
                                ((*pc.offset(-2isize).offset(0isize) as
                                      libc::c_int) << 8i32 |
                                     *pc.offset(-2isize).offset(1isize) as
                                         libc::c_int) as uint32_t;
                            pc = pc.offset(2isize);
                            b =
                                ((*pc.offset(-2isize).offset(0isize) as
                                      libc::c_int) << 8i32 |
                                     *pc.offset(-2isize).offset(1isize) as
                                         libc::c_int) as uint16_t;
                            current_block = 7849445245878263537;
                        }
                        27 => {
                            pc = pc.offset(2isize);
                            a =
                                ((*pc.offset(-2isize).offset(0isize) as
                                      libc::c_int) << 8i32 |
                                     *pc.offset(-2isize).offset(1isize) as
                                         libc::c_int) as uint32_t;
                            pc = pc.offset(2isize);
                            b =
                                ((*pc.offset(-2isize).offset(0isize) as
                                      libc::c_int) << 8i32 |
                                     *pc.offset(-2isize).offset(1isize) as
                                         libc::c_int) as uint16_t;
                            current_block = 16034675043887719473;
                        }
                        28 => {
                            pc = pc.offset(2isize);
                            a =
                                ((*pc.offset(-2isize).offset(0isize) as
                                      libc::c_int) << 8i32 |
                                     *pc.offset(-2isize).offset(1isize) as
                                         libc::c_int) as uint32_t;
                            pc = pc.offset(2isize);
                            b =
                                ((*pc.offset(-2isize).offset(0isize) as
                                      libc::c_int) << 8i32 |
                                     *pc.offset(-2isize).offset(1isize) as
                                         libc::c_int) as uint16_t;
                            current_block = 942271536672158721;
                        }
                        29 => {
                            pc = pc.offset(2isize);
                            a =
                                ((*pc.offset(-2isize).offset(0isize) as
                                      libc::c_int) << 8i32 |
                                     *pc.offset(-2isize).offset(1isize) as
                                         libc::c_int) as uint32_t;
                            pc = pc.offset(2isize);
                            b =
                                ((*pc.offset(-2isize).offset(0isize) as
                                      libc::c_int) << 8i32 |
                                     *pc.offset(-2isize).offset(1isize) as
                                         libc::c_int) as uint16_t;
                            current_block = 1873199301525838551;
                        }
                        30 => {
                            pc = pc.offset(2isize);
                            a =
                                ((*pc.offset(-2isize).offset(0isize) as
                                      libc::c_int) << 8i32 |
                                     *pc.offset(-2isize).offset(1isize) as
                                         libc::c_int) as uint32_t;
                            pc = pc.offset(2isize);
                            b =
                                ((*pc.offset(-2isize).offset(0isize) as
                                      libc::c_int) << 8i32 |
                                     *pc.offset(-2isize).offset(1isize) as
                                         libc::c_int) as uint16_t;
                            current_block = 9704183418455961811;
                        }
                        31 => {
                            pc = pc.offset(2isize);
                            a =
                                ((*pc.offset(-2isize).offset(0isize) as
                                      libc::c_int) << 8i32 |
                                     *pc.offset(-2isize).offset(1isize) as
                                         libc::c_int) as uint32_t;
                            pc = pc.offset(2isize);
                            b =
                                ((*pc.offset(-2isize).offset(0isize) as
                                      libc::c_int) << 8i32 |
                                     *pc.offset(-2isize).offset(1isize) as
                                         libc::c_int) as uint16_t;
                            let fresh324 = pc;
                            pc = pc.offset(1);
                            c = *fresh324;
                            current_block = 10547193261004346349;
                        }
                        32 => {
                            pc = pc.offset(2isize);
                            a =
                                ((*pc.offset(-2isize).offset(0isize) as
                                      libc::c_int) << 8i32 |
                                     *pc.offset(-2isize).offset(1isize) as
                                         libc::c_int) as uint32_t;
                            pc = pc.offset(2isize);
                            b =
                                ((*pc.offset(-2isize).offset(0isize) as
                                      libc::c_int) << 8i32 |
                                     *pc.offset(-2isize).offset(1isize) as
                                         libc::c_int) as uint16_t;
                            let fresh325 = pc;
                            pc = pc.offset(1);
                            c = *fresh325;
                            current_block = 9015561896112508021;
                        }
                        33 => {
                            pc = pc.offset(2isize);
                            a =
                                ((*pc.offset(-2isize).offset(0isize) as
                                      libc::c_int) << 8i32 |
                                     *pc.offset(-2isize).offset(1isize) as
                                         libc::c_int) as uint32_t;
                            current_block = 10458688773482078930;
                        }
                        34 => {
                            pc = pc.offset(2isize);
                            a =
                                ((*pc.offset(-2isize).offset(0isize) as
                                      libc::c_int) << 8i32 |
                                     *pc.offset(-2isize).offset(1isize) as
                                         libc::c_int) as uint32_t;
                            pc = pc.offset(2isize);
                            b =
                                ((*pc.offset(-2isize).offset(0isize) as
                                      libc::c_int) << 8i32 |
                                     *pc.offset(-2isize).offset(1isize) as
                                         libc::c_int) as uint16_t;
                            current_block = 9865490829578899964;
                        }
                        35 => {
                            pc = pc.offset(2isize);
                            a =
                                ((*pc.offset(-2isize).offset(0isize) as
                                      libc::c_int) << 8i32 |
                                     *pc.offset(-2isize).offset(1isize) as
                                         libc::c_int) as uint32_t;
                            pc = pc.offset(2isize);
                            b =
                                ((*pc.offset(-2isize).offset(0isize) as
                                      libc::c_int) << 8i32 |
                                     *pc.offset(-2isize).offset(1isize) as
                                         libc::c_int) as uint16_t;
                            current_block = 17639296353235453144;
                        }
                        36 => {
                            pc = pc.offset(2isize);
                            a =
                                ((*pc.offset(-2isize).offset(0isize) as
                                      libc::c_int) << 8i32 |
                                     *pc.offset(-2isize).offset(1isize) as
                                         libc::c_int) as uint32_t;
                            pc = pc.offset(2isize);
                            b =
                                ((*pc.offset(-2isize).offset(0isize) as
                                      libc::c_int) << 8i32 |
                                     *pc.offset(-2isize).offset(1isize) as
                                         libc::c_int) as uint16_t;
                            current_block = 12781735722417726073;
                        }
                        37 => {
                            pc = pc.offset(2isize);
                            a =
                                ((*pc.offset(-2isize).offset(0isize) as
                                      libc::c_int) << 8i32 |
                                     *pc.offset(-2isize).offset(1isize) as
                                         libc::c_int) as uint32_t;
                            current_block = 6898479631119937405;
                        }
                        38 => {
                            let fresh326 = pc;
                            pc = pc.offset(1);
                            a = *fresh326 as uint32_t;
                            current_block = 4275858079292575654;
                        }
                        39 => {
                            pc = pc.offset(2isize);
                            a =
                                ((*pc.offset(-2isize).offset(0isize) as
                                      libc::c_int) << 8i32 |
                                     *pc.offset(-2isize).offset(1isize) as
                                         libc::c_int) as uint32_t;
                            pc = pc.offset(2isize);
                            b =
                                ((*pc.offset(-2isize).offset(0isize) as
                                      libc::c_int) << 8i32 |
                                     *pc.offset(-2isize).offset(1isize) as
                                         libc::c_int) as uint16_t;
                            current_block = 5105253204732552581;
                        }
                        40 => {
                            let fresh327 = pc;
                            pc = pc.offset(1);
                            a = *fresh327 as uint32_t;
                            current_block = 1459864638523139318;
                        }
                        41 => {
                            let fresh328 = pc;
                            pc = pc.offset(1);
                            a = *fresh328 as uint32_t;
                            current_block = 2384513197176890912;
                        }
                        42 => {
                            let fresh329 = pc;
                            pc = pc.offset(1);
                            a = *fresh329 as uint32_t;
                            current_block = 1367206831037961075;
                        }
                        43 => {
                            let fresh330 = pc;
                            pc = pc.offset(1);
                            a = *fresh330 as uint32_t;
                            current_block = 6367683481748506220;
                        }
                        44 => {
                            pc = pc.offset(2isize);
                            a =
                                ((*pc.offset(-2isize).offset(0isize) as
                                      libc::c_int) << 8i32 |
                                     *pc.offset(-2isize).offset(1isize) as
                                         libc::c_int) as uint32_t;
                            pc = pc.offset(2isize);
                            b =
                                ((*pc.offset(-2isize).offset(0isize) as
                                      libc::c_int) << 8i32 |
                                     *pc.offset(-2isize).offset(1isize) as
                                         libc::c_int) as uint16_t;
                            current_block = 12846811306833230447;
                        }
                        45 => {
                            pc = pc.offset(2isize);
                            a =
                                ((*pc.offset(-2isize).offset(0isize) as
                                      libc::c_int) << 8i32 |
                                     *pc.offset(-2isize).offset(1isize) as
                                         libc::c_int) as uint32_t;
                            pc = pc.offset(2isize);
                            b =
                                ((*pc.offset(-2isize).offset(0isize) as
                                      libc::c_int) << 8i32 |
                                     *pc.offset(-2isize).offset(1isize) as
                                         libc::c_int) as uint16_t;
                            current_block = 10246497873717858537;
                        }
                        46 => {
                            pc = pc.offset(2isize);
                            a =
                                ((*pc.offset(-2isize).offset(0isize) as
                                      libc::c_int) << 8i32 |
                                     *pc.offset(-2isize).offset(1isize) as
                                         libc::c_int) as uint32_t;
                            pc = pc.offset(2isize);
                            b =
                                ((*pc.offset(-2isize).offset(0isize) as
                                      libc::c_int) << 8i32 |
                                     *pc.offset(-2isize).offset(1isize) as
                                         libc::c_int) as uint16_t;
                            let fresh331 = pc;
                            pc = pc.offset(1);
                            c = *fresh331;
                            current_block = 5602041749335279496;
                        }
                        47 => {
                            pc = pc.offset(2isize);
                            a =
                                ((*pc.offset(-2isize).offset(0isize) as
                                      libc::c_int) << 8i32 |
                                     *pc.offset(-2isize).offset(1isize) as
                                         libc::c_int) as uint32_t;
                            pc = pc.offset(2isize);
                            b =
                                ((*pc.offset(-2isize).offset(0isize) as
                                      libc::c_int) << 8i32 |
                                     *pc.offset(-2isize).offset(1isize) as
                                         libc::c_int) as uint16_t;
                            let fresh332 = pc;
                            pc = pc.offset(1);
                            c = *fresh332;
                            current_block = 3906616468301123675;
                        }
                        48 => { current_block = 11480039587962891479; }
                        49 => {
                            pc = pc.offset(2isize);
                            a =
                                ((*pc.offset(-2isize).offset(0isize) as
                                      libc::c_int) << 8i32 |
                                     *pc.offset(-2isize).offset(1isize) as
                                         libc::c_int) as uint32_t;
                            pc = pc.offset(2isize);
                            b =
                                ((*pc.offset(-2isize).offset(0isize) as
                                      libc::c_int) << 8i32 |
                                     *pc.offset(-2isize).offset(1isize) as
                                         libc::c_int) as uint16_t;
                            current_block = 5769007513321684282;
                        }
                        50 => {
                            pc = pc.offset(2isize);
                            a =
                                ((*pc.offset(-2isize).offset(0isize) as
                                      libc::c_int) << 8i32 |
                                     *pc.offset(-2isize).offset(1isize) as
                                         libc::c_int) as uint32_t;
                            pc = pc.offset(2isize);
                            b =
                                ((*pc.offset(-2isize).offset(0isize) as
                                      libc::c_int) << 8i32 |
                                     *pc.offset(-2isize).offset(1isize) as
                                         libc::c_int) as uint16_t;
                            current_block = 4063058694607173453;
                        }
                        51 => {
                            pc = pc.offset(3isize);
                            a =
                                ((*pc.offset(-3isize).offset(0isize) as
                                      libc::c_int) << 16i32 |
                                     (*pc.offset(-3isize).offset(1isize) as
                                          libc::c_int) << 8i32 |
                                     *pc.offset(-3isize).offset(2isize) as
                                         libc::c_int) as uint32_t;
                            current_block = 17974925641258900466;
                        }
                        52 => {
                            pc = pc.offset(2isize);
                            a =
                                ((*pc.offset(-2isize).offset(0isize) as
                                      libc::c_int) << 8i32 |
                                     *pc.offset(-2isize).offset(1isize) as
                                         libc::c_int) as uint32_t;
                            pc = pc.offset(2isize);
                            b =
                                ((*pc.offset(-2isize).offset(0isize) as
                                      libc::c_int) << 8i32 |
                                     *pc.offset(-2isize).offset(1isize) as
                                         libc::c_int) as uint16_t;
                            current_block = 17792945831448561469;
                        }
                        53 => { current_block = 11667657158681136576; }
                        54 => {
                            pc = pc.offset(2isize);
                            a =
                                ((*pc.offset(-2isize).offset(0isize) as
                                      libc::c_int) << 8i32 |
                                     *pc.offset(-2isize).offset(1isize) as
                                         libc::c_int) as uint32_t;
                            pc = pc.offset(2isize);
                            b =
                                ((*pc.offset(-2isize).offset(0isize) as
                                      libc::c_int) << 8i32 |
                                     *pc.offset(-2isize).offset(1isize) as
                                         libc::c_int) as uint16_t;
                            current_block = 13480378041738100027;
                        }
                        55 => {
                            let fresh333 = pc;
                            pc = pc.offset(1);
                            a = *fresh333 as uint32_t;
                            current_block = 15446005597338724160;
                        }
                        56 => {
                            let fresh334 = pc;
                            pc = pc.offset(1);
                            a = *fresh334 as uint32_t;
                            current_block = 15158068222670743697;
                        }
                        57 => {
                            let fresh335 = pc;
                            pc = pc.offset(1);
                            a = *fresh335 as uint32_t;
                            current_block = 6968106866976024785;
                        }
                        58 => {
                            pc = pc.offset(2isize);
                            a =
                                ((*pc.offset(-2isize).offset(0isize) as
                                      libc::c_int) << 8i32 |
                                     *pc.offset(-2isize).offset(1isize) as
                                         libc::c_int) as uint32_t;
                            pc = pc.offset(2isize);
                            b =
                                ((*pc.offset(-2isize).offset(0isize) as
                                      libc::c_int) << 8i32 |
                                     *pc.offset(-2isize).offset(1isize) as
                                         libc::c_int) as uint16_t;
                            current_block = 5611126728812655874;
                        }
                        59 => {
                            let fresh336 = pc;
                            pc = pc.offset(1);
                            a = *fresh336 as uint32_t;
                            current_block = 328988732995390160;
                        }
                        60 => {
                            pc = pc.offset(2isize);
                            a =
                                ((*pc.offset(-2isize).offset(0isize) as
                                      libc::c_int) << 8i32 |
                                     *pc.offset(-2isize).offset(1isize) as
                                         libc::c_int) as uint32_t;
                            pc = pc.offset(2isize);
                            b =
                                ((*pc.offset(-2isize).offset(0isize) as
                                      libc::c_int) << 8i32 |
                                     *pc.offset(-2isize).offset(1isize) as
                                         libc::c_int) as uint16_t;
                            current_block = 10541196509243133637;
                        }
                        61 => {
                            let fresh337 = pc;
                            pc = pc.offset(1);
                            a = *fresh337 as uint32_t;
                            current_block = 16989604180394837074;
                        }
                        62 => {
                            pc = pc.offset(2isize);
                            a =
                                ((*pc.offset(-2isize).offset(0isize) as
                                      libc::c_int) << 8i32 |
                                     *pc.offset(-2isize).offset(1isize) as
                                         libc::c_int) as uint32_t;
                            pc = pc.offset(2isize);
                            b =
                                ((*pc.offset(-2isize).offset(0isize) as
                                      libc::c_int) << 8i32 |
                                     *pc.offset(-2isize).offset(1isize) as
                                         libc::c_int) as uint16_t;
                            current_block = 15326039268735274622;
                        }
                        63 => {
                            let fresh338 = pc;
                            pc = pc.offset(1);
                            a = *fresh338 as uint32_t;
                            current_block = 6846200202819738136;
                        }
                        64 => {
                            let fresh339 = pc;
                            pc = pc.offset(1);
                            a = *fresh339 as uint32_t;
                            current_block = 16948125840707872018;
                        }
                        65 => {
                            let fresh340 = pc;
                            pc = pc.offset(1);
                            a = *fresh340 as uint32_t;
                            current_block = 11372858789397587571;
                        }
                        66 => {
                            let fresh341 = pc;
                            pc = pc.offset(1);
                            a = *fresh341 as uint32_t;
                            current_block = 1975408140333322065;
                        }
                        67 => {
                            let fresh342 = pc;
                            pc = pc.offset(1);
                            a = *fresh342 as uint32_t;
                            current_block = 12696324532854772620;
                        }
                        68 => {
                            let fresh343 = pc;
                            pc = pc.offset(1);
                            a = *fresh343 as uint32_t;
                            current_block = 3433849069680490924;
                        }
                        69 => {
                            let fresh344 = pc;
                            pc = pc.offset(1);
                            a = *fresh344 as uint32_t;
                            current_block = 10072772881953407077;
                        }
                        70 => {
                            pc = pc.offset(2isize);
                            a =
                                ((*pc.offset(-2isize).offset(0isize) as
                                      libc::c_int) << 8i32 |
                                     *pc.offset(-2isize).offset(1isize) as
                                         libc::c_int) as uint32_t;
                            pc = pc.offset(2isize);
                            b =
                                ((*pc.offset(-2isize).offset(0isize) as
                                      libc::c_int) << 8i32 |
                                     *pc.offset(-2isize).offset(1isize) as
                                         libc::c_int) as uint16_t;
                            current_block = 4850402844069062092;
                        }
                        71 => {
                            pc = pc.offset(2isize);
                            a =
                                ((*pc.offset(-2isize).offset(0isize) as
                                      libc::c_int) << 8i32 |
                                     *pc.offset(-2isize).offset(1isize) as
                                         libc::c_int) as uint32_t;
                            pc = pc.offset(2isize);
                            b =
                                ((*pc.offset(-2isize).offset(0isize) as
                                      libc::c_int) << 8i32 |
                                     *pc.offset(-2isize).offset(1isize) as
                                         libc::c_int) as uint16_t;
                            let fresh345 = pc;
                            pc = pc.offset(1);
                            c = *fresh345;
                            current_block = 1741746549607172898;
                        }
                        72 => {
                            let fresh346 = pc;
                            pc = pc.offset(1);
                            a = *fresh346 as uint32_t;
                            current_block = 6053772697959983332;
                        }
                        73 => {
                            let fresh347 = pc;
                            pc = pc.offset(1);
                            a = *fresh347 as uint32_t;
                            current_block = 18022542853155082387;
                        }
                        74 => {
                            let fresh348 = pc;
                            pc = pc.offset(1);
                            a = *fresh348 as uint32_t;
                            current_block = 1606585337627918712;
                        }
                        75 => {
                            pc = pc.offset(2isize);
                            a =
                                ((*pc.offset(-2isize).offset(0isize) as
                                      libc::c_int) << 8i32 |
                                     *pc.offset(-2isize).offset(1isize) as
                                         libc::c_int) as uint32_t;
                            pc = pc.offset(2isize);
                            b =
                                ((*pc.offset(-2isize).offset(0isize) as
                                      libc::c_int) << 8i32 |
                                     *pc.offset(-2isize).offset(1isize) as
                                         libc::c_int) as uint16_t;
                            let fresh349 = pc;
                            pc = pc.offset(1);
                            c = *fresh349;
                            current_block = 9599741646002396977;
                        }
                        76 => {
                            pc = pc.offset(2isize);
                            a =
                                ((*pc.offset(-2isize).offset(0isize) as
                                      libc::c_int) << 8i32 |
                                     *pc.offset(-2isize).offset(1isize) as
                                         libc::c_int) as uint32_t;
                            pc = pc.offset(2isize);
                            b =
                                ((*pc.offset(-2isize).offset(0isize) as
                                      libc::c_int) << 8i32 |
                                     *pc.offset(-2isize).offset(1isize) as
                                         libc::c_int) as uint16_t;
                            let fresh350 = pc;
                            pc = pc.offset(1);
                            c = *fresh350;
                            current_block = 11441496697127961056;
                        }
                        77 => {
                            pc = pc.offset(2isize);
                            a =
                                ((*pc.offset(-2isize).offset(0isize) as
                                      libc::c_int) << 8i32 |
                                     *pc.offset(-2isize).offset(1isize) as
                                         libc::c_int) as uint32_t;
                            pc = pc.offset(2isize);
                            b =
                                ((*pc.offset(-2isize).offset(0isize) as
                                      libc::c_int) << 8i32 |
                                     *pc.offset(-2isize).offset(1isize) as
                                         libc::c_int) as uint16_t;
                            let fresh351 = pc;
                            pc = pc.offset(1);
                            c = *fresh351;
                            current_block = 10671272775318305553;
                        }
                        78 => {
                            let fresh352 = pc;
                            pc = pc.offset(1);
                            a = *fresh352 as uint32_t;
                            current_block = 7127613333771968219;
                        }
                        79 => {
                            pc = pc.offset(2isize);
                            a =
                                ((*pc.offset(-2isize).offset(0isize) as
                                      libc::c_int) << 8i32 |
                                     *pc.offset(-2isize).offset(1isize) as
                                         libc::c_int) as uint32_t;
                            pc = pc.offset(2isize);
                            b =
                                ((*pc.offset(-2isize).offset(0isize) as
                                      libc::c_int) << 8i32 |
                                     *pc.offset(-2isize).offset(1isize) as
                                         libc::c_int) as uint16_t;
                            current_block = 16677287981175421660;
                        }
                        80 => {
                            let fresh353 = pc;
                            pc = pc.offset(1);
                            a = *fresh353 as uint32_t;
                            current_block = 13433017329934546124;
                        }
                        81 => {
                            pc = pc.offset(2isize);
                            a =
                                ((*pc.offset(-2isize).offset(0isize) as
                                      libc::c_int) << 8i32 |
                                     *pc.offset(-2isize).offset(1isize) as
                                         libc::c_int) as uint32_t;
                            pc = pc.offset(2isize);
                            b =
                                ((*pc.offset(-2isize).offset(0isize) as
                                      libc::c_int) << 8i32 |
                                     *pc.offset(-2isize).offset(1isize) as
                                         libc::c_int) as uint16_t;
                            current_block = 314002374245450226;
                        }
                        82 => {
                            pc = pc.offset(2isize);
                            a =
                                ((*pc.offset(-2isize).offset(0isize) as
                                      libc::c_int) << 8i32 |
                                     *pc.offset(-2isize).offset(1isize) as
                                         libc::c_int) as uint32_t;
                            pc = pc.offset(2isize);
                            b =
                                ((*pc.offset(-2isize).offset(0isize) as
                                      libc::c_int) << 8i32 |
                                     *pc.offset(-2isize).offset(1isize) as
                                         libc::c_int) as uint16_t;
                            current_block = 5341326654522537507;
                        }
                        83 => {
                            let fresh354 = pc;
                            pc = pc.offset(1);
                            a = *fresh354 as uint32_t;
                            current_block = 14386882892843117929;
                        }
                        84 => {
                            pc = pc.offset(2isize);
                            a =
                                ((*pc.offset(-2isize).offset(0isize) as
                                      libc::c_int) << 8i32 |
                                     *pc.offset(-2isize).offset(1isize) as
                                         libc::c_int) as uint32_t;
                            pc = pc.offset(2isize);
                            b =
                                ((*pc.offset(-2isize).offset(0isize) as
                                      libc::c_int) << 8i32 |
                                     *pc.offset(-2isize).offset(1isize) as
                                         libc::c_int) as uint16_t;
                            current_block = 3020430753860169586;
                        }
                        85 => {
                            pc = pc.offset(2isize);
                            a =
                                ((*pc.offset(-2isize).offset(0isize) as
                                      libc::c_int) << 8i32 |
                                     *pc.offset(-2isize).offset(1isize) as
                                         libc::c_int) as uint32_t;
                            pc = pc.offset(2isize);
                            b =
                                ((*pc.offset(-2isize).offset(0isize) as
                                      libc::c_int) << 8i32 |
                                     *pc.offset(-2isize).offset(1isize) as
                                         libc::c_int) as uint16_t;
                            current_block = 11988973803779423327;
                        }
                        86 => {
                            pc = pc.offset(2isize);
                            a =
                                ((*pc.offset(-2isize).offset(0isize) as
                                      libc::c_int) << 8i32 |
                                     *pc.offset(-2isize).offset(1isize) as
                                         libc::c_int) as uint32_t;
                            pc = pc.offset(2isize);
                            b =
                                ((*pc.offset(-2isize).offset(0isize) as
                                      libc::c_int) << 8i32 |
                                     *pc.offset(-2isize).offset(1isize) as
                                         libc::c_int) as uint16_t;
                            current_block = 10561944073379837955;
                        }
                        87 => {
                            let fresh355 = pc;
                            pc = pc.offset(1);
                            a = *fresh355 as uint32_t;
                            current_block = 9864403379770423142;
                        }
                        88 => {
                            let fresh356 = pc;
                            pc = pc.offset(1);
                            a = *fresh356 as uint32_t;
                            current_block = 8124361700674833375;
                        }
                        89 => {
                            let fresh357 = pc;
                            pc = pc.offset(1);
                            a = *fresh357 as uint32_t;
                            current_block = 2968065794317548828;
                        }
                        90 => {
                            pc = pc.offset(2isize);
                            a =
                                ((*pc.offset(-2isize).offset(0isize) as
                                      libc::c_int) << 8i32 |
                                     *pc.offset(-2isize).offset(1isize) as
                                         libc::c_int) as uint32_t;
                            pc = pc.offset(2isize);
                            b =
                                ((*pc.offset(-2isize).offset(0isize) as
                                      libc::c_int) << 8i32 |
                                     *pc.offset(-2isize).offset(1isize) as
                                         libc::c_int) as uint16_t;
                            current_block = 3258098155381626199;
                        }
                        91 => {
                            pc = pc.offset(2isize);
                            a =
                                ((*pc.offset(-2isize).offset(0isize) as
                                      libc::c_int) << 8i32 |
                                     *pc.offset(-2isize).offset(1isize) as
                                         libc::c_int) as uint32_t;
                            pc = pc.offset(2isize);
                            b =
                                ((*pc.offset(-2isize).offset(0isize) as
                                      libc::c_int) << 8i32 |
                                     *pc.offset(-2isize).offset(1isize) as
                                         libc::c_int) as uint16_t;
                            current_block = 7493503892657452062;
                        }
                        92 => {
                            pc = pc.offset(2isize);
                            a =
                                ((*pc.offset(-2isize).offset(0isize) as
                                      libc::c_int) << 8i32 |
                                     *pc.offset(-2isize).offset(1isize) as
                                         libc::c_int) as uint32_t;
                            pc = pc.offset(2isize);
                            b =
                                ((*pc.offset(-2isize).offset(0isize) as
                                      libc::c_int) << 8i32 |
                                     *pc.offset(-2isize).offset(1isize) as
                                         libc::c_int) as uint16_t;
                            current_block = 2499490863002033965;
                        }
                        93 => {
                            pc = pc.offset(2isize);
                            a =
                                ((*pc.offset(-2isize).offset(0isize) as
                                      libc::c_int) << 8i32 |
                                     *pc.offset(-2isize).offset(1isize) as
                                         libc::c_int) as uint32_t;
                            pc = pc.offset(2isize);
                            b =
                                ((*pc.offset(-2isize).offset(0isize) as
                                      libc::c_int) << 8i32 |
                                     *pc.offset(-2isize).offset(1isize) as
                                         libc::c_int) as uint16_t;
                            current_block = 8489059574810375089;
                        }
                        94 => {
                            pc = pc.offset(2isize);
                            a =
                                ((*pc.offset(-2isize).offset(0isize) as
                                      libc::c_int) << 8i32 |
                                     *pc.offset(-2isize).offset(1isize) as
                                         libc::c_int) as uint32_t;
                            pc = pc.offset(2isize);
                            b =
                                ((*pc.offset(-2isize).offset(0isize) as
                                      libc::c_int) << 8i32 |
                                     *pc.offset(-2isize).offset(1isize) as
                                         libc::c_int) as uint16_t;
                            current_block = 12998570369541158573;
                        }
                        95 => {
                            let fresh358 = pc;
                            pc = pc.offset(1);
                            a = *fresh358 as uint32_t;
                            current_block = 12775348822761942368;
                        }
                        96 => {
                            let fresh359 = pc;
                            pc = pc.offset(1);
                            a = *fresh359 as uint32_t;
                            current_block = 355116111335096151;
                        }
                        97 => {
                            let fresh360 = pc;
                            pc = pc.offset(1);
                            a = *fresh360 as uint32_t;
                            current_block = 890063714143163519;
                        }
                        98 => {
                            pc = pc.offset(2isize);
                            a =
                                ((*pc.offset(-2isize).offset(0isize) as
                                      libc::c_int) << 8i32 |
                                     *pc.offset(-2isize).offset(1isize) as
                                         libc::c_int) as uint32_t;
                            pc = pc.offset(2isize);
                            b =
                                ((*pc.offset(-2isize).offset(0isize) as
                                      libc::c_int) << 8i32 |
                                     *pc.offset(-2isize).offset(1isize) as
                                         libc::c_int) as uint16_t;
                            let fresh361 = pc;
                            pc = pc.offset(1);
                            c = *fresh361;
                            current_block = 998308582751651876;
                        }
                        99 => {
                            let fresh362 = pc;
                            pc = pc.offset(1);
                            a = *fresh362 as uint32_t;
                            current_block = 7833862538762852372;
                        }
                        100 => { current_block = 8667820544026455208; }
                        101 => { current_block = 10575315580790317560; }
                        102 => { current_block = 18239940301814930841; }
                        103 => { current_block = 10942739336945693405; }
                        _ => { current_block = 3935997918849240122; }
                    }
                }
            }
        }
    }
    printf(b"\n\x00" as *const u8 as *const libc::c_char);
}
unsafe extern "C" fn codedump_recur(mut mrb: *mut mrb_state,
                                    mut irep: *mut mrb_irep) {
    let mut i: libc::c_int = 0;
    codedump(mrb, irep);
    i = 0i32;
    while i < (*irep).rlen as libc::c_int {
        codedump_recur(mrb, *(*irep).reps.offset(i as isize));
        i += 1
    };
}
#[no_mangle]
pub unsafe extern "C" fn mrb_codedump_all(mut mrb: *mut mrb_state,
                                          mut proc_0: *mut RProc) {
    codedump_recur(mrb, (*proc_0).body.irep);
}