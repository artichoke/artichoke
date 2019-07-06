use libc;
use c2rust_bitfields::{BitfieldStruct};
extern "C" {
    pub type iv_tbl;
    pub type RClass;
    pub type symbol_name;
    pub type RProc;
    pub type REnv;
    pub type mrb_jmpbuf;
    #[no_mangle]
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    #[no_mangle]
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    #[no_mangle]
    fn mrb_intern(_: *mut mrb_state, _: *const libc::c_char, _: size_t)
     -> mrb_sym;
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
}
pub type __darwin_ptrdiff_t = libc::c_long;
pub type __darwin_size_t = libc::c_ulong;
pub type size_t = __darwin_size_t;
pub type int32_t = libc::c_int;
pub type int64_t = libc::c_longlong;
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
 * get line from irep's debug info and program counter
 * @return returns NULL if not found
 */
#[no_mangle]
pub unsafe extern "C" fn mrb_debug_get_filename(mut mrb: *mut mrb_state,
                                                mut irep: *mut mrb_irep,
                                                mut pc: ptrdiff_t)
 -> *const libc::c_char {
    if !irep.is_null() && pc >= 0i32 as libc::c_long &&
           pc < (*irep).ilen as libc::c_long {
        let mut f: *mut mrb_irep_debug_info_file =
            0 as *mut mrb_irep_debug_info_file;
        if (*irep).debug_info.is_null() {
            return 0 as *const libc::c_char
        } else {
            f = get_file((*irep).debug_info, pc as uint32_t);
            if !f.is_null() {
                return mrb_sym2name_len(mrb, (*f).filename_sym,
                                        0 as *mut mrb_int)
            }
        }
    }
    return 0 as *const libc::c_char;
}
unsafe extern "C" fn get_file(mut info: *mut mrb_irep_debug_info,
                              mut pc: uint32_t)
 -> *mut mrb_irep_debug_info_file {
    let mut ret: *mut *mut mrb_irep_debug_info_file =
        0 as *mut *mut mrb_irep_debug_info_file;
    let mut count: int32_t = 0;
    if pc >= (*info).pc_count { return 0 as *mut mrb_irep_debug_info_file }
    ret = (*info).files;
    count = (*info).flen as int32_t;
    while count > 0i32 {
        let mut step: int32_t = count / 2i32;
        let mut it: *mut *mut mrb_irep_debug_info_file =
            ret.offset(step as isize);
        if !(pc < (**it).start_pos) {
            ret = it.offset(1isize);
            count -= step + 1i32
        } else { count = step }
    }
    ret = ret.offset(-1isize);
    return *ret;
}
/*
 * get line from irep's debug info and program counter
 * @return returns -1 if not found
 */
#[no_mangle]
pub unsafe extern "C" fn mrb_debug_get_line(mut mrb: *mut mrb_state,
                                            mut irep: *mut mrb_irep,
                                            mut pc: ptrdiff_t) -> int32_t {
    if !irep.is_null() && pc >= 0i32 as libc::c_long &&
           pc < (*irep).ilen as libc::c_long {
        let mut f: *mut mrb_irep_debug_info_file =
            0 as *mut mrb_irep_debug_info_file;
        if (*irep).debug_info.is_null() {
            return -1i32
        } else {
            f = get_file((*irep).debug_info, pc as uint32_t);
            if !f.is_null() {
                match (*f).line_type as libc::c_uint {
                    0 => {
                        return *(*f).lines.ary.offset((pc -
                                                           (*f).start_pos as
                                                               libc::c_long)
                                                          as isize) as int32_t
                    }
                    1 => {
                        let mut ret: *mut mrb_irep_debug_info_line =
                            (*f).lines.flat_map;
                        let mut count: uint32_t = (*f).line_entry_count;
                        while count > 0i32 as libc::c_uint {
                            let mut step: int32_t =
                                count.wrapping_div(2i32 as libc::c_uint) as
                                    int32_t;
                            let mut it: *mut mrb_irep_debug_info_line =
                                ret.offset(step as isize);
                            if !(pc < (*it).start_pos as libc::c_long) {
                                ret = it.offset(1isize);
                                count =
                                    (count as
                                         libc::c_uint).wrapping_sub((step +
                                                                         1i32)
                                                                        as
                                                                        libc::c_uint)
                                        as uint32_t as uint32_t
                            } else { count = step as uint32_t }
                        }
                        ret = ret.offset(-1isize);
                        return (*ret).line as int32_t
                    }
                    _ => { }
                }
            }
        }
    }
    return -1i32;
}
#[no_mangle]
pub unsafe extern "C" fn mrb_debug_info_alloc(mut mrb: *mut mrb_state,
                                              mut irep: *mut mrb_irep)
 -> *mut mrb_irep_debug_info {
    static mut initial: mrb_irep_debug_info =
        mrb_irep_debug_info{pc_count: 0i32 as uint32_t,
                            flen: 0i32 as uint16_t,
                            files:
                                0 as *const *mut mrb_irep_debug_info_file as
                                    *mut *mut mrb_irep_debug_info_file,};
    let mut ret: *mut mrb_irep_debug_info = 0 as *mut mrb_irep_debug_info;
    ret =
        mrb_malloc(mrb,
                   ::std::mem::size_of::<mrb_irep_debug_info>() as
                       libc::c_ulong) as *mut mrb_irep_debug_info;
    *ret = initial;
    (*irep).debug_info = ret;
    return ret;
}
#[no_mangle]
pub unsafe extern "C" fn mrb_debug_info_append_file(mut mrb: *mut mrb_state,
                                                    mut d:
                                                        *mut mrb_irep_debug_info,
                                                    mut filename:
                                                        *const libc::c_char,
                                                    mut lines: *mut uint16_t,
                                                    mut start_pos: uint32_t,
                                                    mut end_pos: uint32_t)
 -> *mut mrb_irep_debug_info_file {
    let mut f: *mut mrb_irep_debug_info_file =
        0 as *mut mrb_irep_debug_info_file;
    let mut file_pc_count: uint32_t = 0;
    let mut fn_len: size_t = 0;
    let mut i: uint32_t = 0;
    if d.is_null() { return 0 as *mut mrb_irep_debug_info_file }
    if start_pos == end_pos { return 0 as *mut mrb_irep_debug_info_file }
    if (*d).flen as libc::c_int > 0i32 {
        let mut fn_0: *const libc::c_char =
            mrb_sym2name_len(mrb,
                             (**(*d).files.offset(((*d).flen as libc::c_int -
                                                       1i32) as
                                                      isize)).filename_sym,
                             0 as *mut mrb_int);
        if strcmp(filename, fn_0) == 0i32 {
            return 0 as *mut mrb_irep_debug_info_file
        }
    }
    f =
        mrb_malloc(mrb,
                   ::std::mem::size_of::<mrb_irep_debug_info_file>() as
                       libc::c_ulong) as *mut mrb_irep_debug_info_file;
    (*d).files =
        (if !(*d).files.is_null() {
             mrb_realloc(mrb, (*d).files as *mut libc::c_void,
                         (::std::mem::size_of::<*mut mrb_irep_debug_info_file>()
                              as
                              libc::c_ulong).wrapping_mul(((*d).flen as
                                                               libc::c_int +
                                                               1i32) as
                                                              libc::c_ulong))
         } else {
             mrb_malloc(mrb,
                        ::std::mem::size_of::<*mut mrb_irep_debug_info_file>()
                            as libc::c_ulong)
         }) as *mut *mut mrb_irep_debug_info_file;
    let fresh0 = (*d).flen;
    (*d).flen = (*d).flen.wrapping_add(1);
    let ref mut fresh1 = *(*d).files.offset(fresh0 as isize);
    *fresh1 = f;
    file_pc_count = end_pos.wrapping_sub(start_pos);
    (*f).start_pos = start_pos;
    (*d).pc_count = end_pos;
    fn_len = strlen(filename);
    (*f).filename_sym = mrb_intern(mrb, filename, fn_len);
    (*f).line_type =
        select_line_type(lines.offset(start_pos as isize),
                         end_pos.wrapping_sub(start_pos) as size_t);
    (*f).lines.ptr = 0 as *mut libc::c_void;
    match (*f).line_type as libc::c_uint {
        0 => {
            (*f).line_entry_count = file_pc_count;
            (*f).lines.ary =
                mrb_malloc(mrb,
                           (::std::mem::size_of::<uint16_t>() as
                                libc::c_ulong).wrapping_mul(file_pc_count as
                                                                libc::c_ulong))
                    as *mut uint16_t;
            i = 0i32 as uint32_t;
            while i < file_pc_count {
                *(*f).lines.ary.offset(i as isize) =
                    *lines.offset(start_pos.wrapping_add(i) as isize);
                i = i.wrapping_add(1)
            }
        }
        1 => {
            let mut prev_line: uint16_t = 0i32 as uint16_t;
            let mut m: mrb_irep_debug_info_line =
                mrb_irep_debug_info_line{start_pos: 0, line: 0,};
            (*f).lines.flat_map =
                mrb_malloc(mrb,
                           (::std::mem::size_of::<mrb_irep_debug_info_line>()
                                as
                                libc::c_ulong).wrapping_mul(1i32 as
                                                                libc::c_ulong))
                    as *mut mrb_irep_debug_info_line;
            (*f).line_entry_count = 0i32 as uint32_t;
            i = 0i32 as uint32_t;
            while i < file_pc_count {
                if !(*lines.offset(start_pos.wrapping_add(i) as isize) as
                         libc::c_int == prev_line as libc::c_int) {
                    (*f).lines.flat_map =
                        mrb_realloc(mrb,
                                    (*f).lines.flat_map as *mut libc::c_void,
                                    (::std::mem::size_of::<mrb_irep_debug_info_line>()
                                         as
                                         libc::c_ulong).wrapping_mul((*f).line_entry_count.wrapping_add(1i32
                                                                                                            as
                                                                                                            libc::c_uint)
                                                                         as
                                                                         libc::c_ulong))
                            as *mut mrb_irep_debug_info_line;
                    m.start_pos = start_pos.wrapping_add(i);
                    m.line =
                        *lines.offset(start_pos.wrapping_add(i) as isize);
                    *(*f).lines.flat_map.offset((*f).line_entry_count as
                                                    isize) = m;
                    (*f).line_entry_count =
                        (*f).line_entry_count.wrapping_add(1);
                    prev_line =
                        *lines.offset(start_pos.wrapping_add(i) as isize)
                }
                i = i.wrapping_add(1)
            }
        }
        _ => { }
    }
    return f;
}
unsafe extern "C" fn select_line_type(mut lines: *const uint16_t,
                                      mut lines_len: size_t)
 -> mrb_debug_line_type {
    let mut line_count: size_t = 0i32 as size_t;
    let mut prev_line: libc::c_int = -1i32;
    let mut i: size_t = 0;
    i = 0i32 as size_t;
    while i < lines_len {
        if *lines.offset(i as isize) as libc::c_int != prev_line {
            line_count = line_count.wrapping_add(1)
        }
        i = i.wrapping_add(1)
    }
    return (if (::std::mem::size_of::<uint16_t>() as
                    libc::c_ulong).wrapping_mul(lines_len) <=
                   (::std::mem::size_of::<mrb_irep_debug_info_line>() as
                        libc::c_ulong).wrapping_mul(line_count) {
                mrb_debug_line_ary as libc::c_int
            } else { mrb_debug_line_flat_map as libc::c_int }) as
               mrb_debug_line_type;
}
#[no_mangle]
pub unsafe extern "C" fn mrb_debug_info_free(mut mrb: *mut mrb_state,
                                             mut d:
                                                 *mut mrb_irep_debug_info) {
    let mut i: uint32_t = 0;
    if d.is_null() { return }
    i = 0i32 as uint32_t;
    while i < (*d).flen as libc::c_uint {
        mrb_free(mrb, (**(*d).files.offset(i as isize)).lines.ptr);
        mrb_free(mrb, *(*d).files.offset(i as isize) as *mut libc::c_void);
        i = i.wrapping_add(1)
    }
    mrb_free(mrb, (*d).files as *mut libc::c_void);
    mrb_free(mrb, d as *mut libc::c_void);
}