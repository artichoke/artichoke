use libc;
use c2rust_bitfields::{BitfieldStruct};
extern "C" {
    pub type iv_tbl;
    pub type kh_mt;
    /* debug info */
    pub type mrb_irep_debug_info;
    pub type symbol_name;
    pub type RProc;
    pub type REnv;
    pub type mrb_jmpbuf;
    #[no_mangle]
    fn __assert_rtn(_: *const libc::c_char, _: *const libc::c_char,
                    _: libc::c_int, _: *const libc::c_char) -> !;
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
    fn mrb_obj_alloc(_: *mut mrb_state, _: mrb_vtype, _: *mut RClass)
     -> *mut RBasic;
    /* *
 * Turns a C string into a Ruby string value.
 */
    #[no_mangle]
    fn mrb_str_new_cstr(_: *mut mrb_state, _: *const libc::c_char)
     -> mrb_value;
    /*
 * Returns a symbol from a passed in Ruby string.
 *
 * @param [mrb_state] mrb The current mruby state.
 * @param [mrb_value] self Ruby string.
 * @return [mrb_value] A symbol.
 */
    #[no_mangle]
    fn mrb_str_intern(mrb: *mut mrb_state, self_0: mrb_value) -> mrb_value;
    #[no_mangle]
    fn mrb_raisef(mrb: *mut mrb_state, c: *mut RClass,
                  fmt: *const libc::c_char, _: ...) -> !;
    #[no_mangle]
    fn mrb_inspect(mrb: *mut mrb_state, obj: mrb_value) -> mrb_value;
    #[no_mangle]
    fn mrb_check_string_type(mrb: *mut mrb_state, str: mrb_value)
     -> mrb_value;
    #[no_mangle]
    fn mrb_check_type(mrb: *mut mrb_state, x: mrb_value, t: mrb_vtype);
}
pub type int64_t = libc::c_longlong;
pub type uint8_t = libc::c_uchar;
pub type uint16_t = libc::c_ushort;
pub type uint32_t = libc::c_uint;
pub type __darwin_intptr_t = libc::c_long;
pub type __darwin_size_t = libc::c_ulong;
pub type intptr_t = __darwin_intptr_t;
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
    pub _pad: [u8; 4],
    pub c: *mut RClass,
    pub gcnext: *mut RBasic,
    pub iv: *mut iv_tbl,
    pub type_0: *const mrb_data_type,
    pub data: *mut libc::c_void,
}
#[inline]
unsafe extern "C" fn mrb_obj_value(mut p: *mut libc::c_void) -> mrb_value {
    let mut v: mrb_value =
        mrb_value{value: unnamed{f: 0.,}, tt: MRB_TT_FALSE,};
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
#[no_mangle]
pub unsafe extern "C" fn mrb_obj_id(mut obj: mrb_value) -> mrb_int {
    let mut tt: mrb_int = obj.tt as mrb_int;
    match tt {
        1 | 5 => {
            return (0i32 as intptr_t as libc::c_longlong ^ tt) as mrb_int
        }
        0 => {
            if obj.tt as libc::c_uint ==
                   MRB_TT_FALSE as libc::c_int as libc::c_uint &&
                   0 == obj.value.i {
                return (1i32 as intptr_t as libc::c_longlong ^ tt) as mrb_int
            }
            return (0i32 as intptr_t as libc::c_longlong ^ tt) as mrb_int
        }
        2 => { return (1i32 as intptr_t as libc::c_longlong ^ tt) as mrb_int }
        4 => {
            return (obj.value.sym as intptr_t as libc::c_longlong ^ tt) as
                       mrb_int
        }
        3 => {
            return (mrb_float_id(obj.value.i as mrb_float) as intptr_t ^
                        MRB_TT_FLOAT as libc::c_int as libc::c_long) as
                       mrb_int
        }
        6 => {
            return (mrb_float_id(obj.value.f) as intptr_t as libc::c_longlong
                        ^ tt) as mrb_int
        }
        16 | 8 | 9 | 10 | 11 | 12 | 13 | 14 | 15 | 17 | 18 | 19 | 21 | 23 | _
        => {
            return (obj.value.p as intptr_t as libc::c_longlong ^ tt) as
                       mrb_int
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn mrb_float_id(mut f: mrb_float) -> mrb_int {
    let mut p: *const libc::c_char =
        &mut f as *mut mrb_float as *const libc::c_char;
    let mut len: libc::c_int =
        ::std::mem::size_of::<mrb_float>() as libc::c_ulong as libc::c_int;
    let mut id: uint32_t = 0i32 as uint32_t;
    if f == 0i32 as libc::c_double { f = 0.0f64 }
    loop  {
        let fresh0 = len;
        len = len - 1;
        if !(0 != fresh0) { break ; }
        id =
            id.wrapping_mul(65599i32 as
                                libc::c_uint).wrapping_add(*p as
                                                               libc::c_uint);
        p = p.offset(1isize)
    }
    id = id.wrapping_add(id >> 5i32);
    return id as mrb_int;
}
#[no_mangle]
pub unsafe extern "C" fn mrb_obj_to_sym(mut mrb: *mut mrb_state,
                                        mut name: mrb_value) -> mrb_sym {
    let mut id: mrb_sym = 0;
    let mut current_block_6: u64;
    match name.tt as libc::c_uint {
        16 => { current_block_6 = 16640572375626650562; }
        4 => { current_block_6 = 12034546258976416165; }
        _ => {
            name = mrb_check_string_type(mrb, name);
            if name.tt as libc::c_uint ==
                   MRB_TT_FALSE as libc::c_int as libc::c_uint &&
                   0 == name.value.i {
                name = mrb_inspect(mrb, name);
                mrb_raisef(mrb,
                           mrb_exc_get(mrb,
                                       b"TypeError\x00" as *const u8 as
                                           *const libc::c_char),
                           b"%S is not a symbol\x00" as *const u8 as
                               *const libc::c_char, name);
            }
            /* not reached */
            /* fall through */
            current_block_6 = 16640572375626650562;
        }
    }
    match current_block_6 {
        16640572375626650562 => { name = mrb_str_intern(mrb, name) }
        _ => { }
    }
    id = name.value.sym;
    return id;
}
#[no_mangle]
pub unsafe extern "C" fn mrb_data_object_alloc(mut mrb: *mut mrb_state,
                                               mut klass: *mut RClass,
                                               mut ptr: *mut libc::c_void,
                                               mut type_0:
                                                   *const mrb_data_type)
 -> *mut RData {
    let mut data: *mut RData = 0 as *mut RData;
    data = mrb_obj_alloc(mrb, MRB_TT_DATA, klass) as *mut RData;
    (*data).data = ptr;
    (*data).type_0 = type_0;
    return data;
}
#[no_mangle]
pub unsafe extern "C" fn mrb_data_check_type(mut mrb: *mut mrb_state,
                                             mut obj: mrb_value,
                                             mut type_0:
                                                 *const mrb_data_type) {
    if obj.tt as libc::c_uint != MRB_TT_DATA as libc::c_int as libc::c_uint {
        mrb_check_type(mrb, obj, MRB_TT_DATA);
    }
    if (*(obj.value.p as *mut RData)).type_0 != type_0 {
        let mut t2: *const mrb_data_type =
            (*(obj.value.p as *mut RData)).type_0;
        if !t2.is_null() {
            mrb_raisef(mrb,
                       mrb_exc_get(mrb,
                                   b"TypeError\x00" as *const u8 as
                                       *const libc::c_char),
                       b"wrong argument type %S (expected %S)\x00" as
                           *const u8 as *const libc::c_char,
                       mrb_str_new_cstr(mrb, (*t2).struct_name),
                       mrb_str_new_cstr(mrb, (*type_0).struct_name));
        } else {
            let mut c: *mut RClass = mrb_class(mrb, obj);
            mrb_raisef(mrb,
                       mrb_exc_get(mrb,
                                   b"TypeError\x00" as *const u8 as
                                       *const libc::c_char),
                       b"uninitialized %S (expected %S)\x00" as *const u8 as
                           *const libc::c_char,
                       mrb_obj_value(c as *mut libc::c_void),
                       mrb_str_new_cstr(mrb, (*type_0).struct_name));
        }
    };
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
#[no_mangle]
pub unsafe extern "C" fn mrb_data_get_ptr(mut mrb: *mut mrb_state,
                                          mut obj: mrb_value,
                                          mut type_0: *const mrb_data_type)
 -> *mut libc::c_void {
    mrb_data_check_type(mrb, obj, type_0);
    return (*(obj.value.p as *mut RData)).data;
}
#[no_mangle]
pub unsafe extern "C" fn mrb_data_check_get_ptr(mut mrb: *mut mrb_state,
                                                mut obj: mrb_value,
                                                mut type_0:
                                                    *const mrb_data_type)
 -> *mut libc::c_void {
    if obj.tt as libc::c_uint != MRB_TT_DATA as libc::c_int as libc::c_uint {
        return 0 as *mut libc::c_void
    }
    if (*(obj.value.p as *mut RData)).type_0 != type_0 {
        return 0 as *mut libc::c_void
    }
    return (*(obj.value.p as *mut RData)).data;
}