use libc;
use c2rust_bitfields::BitfieldStruct;
use num_traits::ToPrimitive;
extern "C" {
    pub type iv_tbl;
    pub type RClass;
    pub type symbol_name;
    pub type RProc;
    pub type REnv;
    pub type mrb_jmpbuf;
    #[no_mangle]
    fn memset(_: *mut libc::c_void, _: libc::c_int, _: libc::c_ulong)
     -> *mut libc::c_void;
    #[no_mangle]
    fn frexp(_: libc::c_double, _: *mut libc::c_int) -> libc::c_double;
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
    fn mrb_raise(mrb: *mut mrb_state, c: *mut RClass,
                 msg: *const libc::c_char);
    #[no_mangle]
    fn mrb_str_new_capa(mrb: *mut mrb_state, capa: size_t) -> mrb_value;
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
}
pub type __darwin_ptrdiff_t = libc::c_long;
pub type __darwin_size_t = libc::c_ulong;
pub type size_t = __darwin_size_t;
pub type int64_t = libc::c_longlong;
pub type uint8_t = libc::c_uchar;
pub type uint16_t = libc::c_ushort;
pub type uint32_t = libc::c_uint;
pub type uint64_t = libc::c_ulonglong;
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
    pub value: C2RustUnnamed_3,
    pub tt: mrb_vtype,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub union C2RustUnnamed_3 {
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
/*

Most code in this file originates from musl (src/stdio/vfprintf.c)
which, just like mruby itself, is licensed under the MIT license.

Copyright (c) 2005-2014 Rich Felker, et al.

Permission is hereby granted, free of charge, to any person obtaining
a copy of this software and associated documentation files (the
"Software"), to deal in the Software without restriction, including
without limitation the rights to use, copy, modify, merge, publish,
distribute, sublicense, and/or sell copies of the Software, and to
permit persons to whom the Software is furnished to do so, subject to
the following conditions:

The above copyright notice and this permission notice shall be
included in all copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND,
EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF
MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT.
IN NO EVENT SHALL THE AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY
CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT,
TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN CONNECTION WITH THE
SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE.

*/
#[derive ( Copy , Clone )]
#[repr(C)]
pub struct fmt_args {
    pub mrb: *mut mrb_state,
    pub str_0: mrb_value,
}
#[inline(always)]
unsafe extern "C" fn __inline_isfinitef(mut __x: libc::c_float)
 -> libc::c_int {
    return (__x == __x && __x.abs() != ::std::f32::INFINITY) as libc::c_int;
}
#[inline(always)]
unsafe extern "C" fn __inline_isfinited(mut __x: libc::c_double)
 -> libc::c_int {
    return (__x == __x && __x.abs() != ::std::f64::INFINITY) as libc::c_int;
}
#[inline(always)]
unsafe extern "C" fn __inline_isfinitel(mut __x: f128::f128) -> libc::c_int {
    return (__x == __x && __x.abs() != ::std::f64::INFINITY) as libc::c_int;
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
unsafe extern "C" fn out(mut f: *mut fmt_args, mut s: *const libc::c_char,
                         mut l: size_t) {
    mrb_str_cat((*f).mrb, (*f).str_0, s, l);
}
unsafe extern "C" fn pad(mut f: *mut fmt_args, mut c: libc::c_char,
                         mut w: ptrdiff_t, mut l: ptrdiff_t,
                         mut fl: uint8_t) {
    let mut pad_0: [libc::c_char; 256] = [0; 256];
    if 0 !=
           fl as libc::c_uint &
               (1u32 << '-' as i32 - ' ' as i32 |
                    1u32 << '0' as i32 - ' ' as i32) || l >= w {
        return
    }
    l = w - l;
    memset(pad_0.as_mut_ptr() as *mut libc::c_void, c as libc::c_int,
           (if l > 256i32 as libc::c_long {
                256i32 as libc::c_long
            } else { l }) as libc::c_ulong);
    while l >= 256i32 as libc::c_long {
        out(f, pad_0.as_mut_ptr(), 256i32 as size_t);
        l -= 256i32 as libc::c_long
    }
    out(f, pad_0.as_mut_ptr(), l as size_t);
}
static mut xdigits: [libc::c_char; 16] =
    ['0' as i32 as libc::c_char, '1' as i32 as libc::c_char,
     '2' as i32 as libc::c_char, '3' as i32 as libc::c_char,
     '4' as i32 as libc::c_char, '5' as i32 as libc::c_char,
     '6' as i32 as libc::c_char, '7' as i32 as libc::c_char,
     '8' as i32 as libc::c_char, '9' as i32 as libc::c_char,
     'A' as i32 as libc::c_char, 'B' as i32 as libc::c_char,
     'C' as i32 as libc::c_char, 'D' as i32 as libc::c_char,
     'E' as i32 as libc::c_char, 'F' as i32 as libc::c_char];
unsafe extern "C" fn fmt_u(mut x: uint32_t, mut s: *mut libc::c_char)
 -> *mut libc::c_char {
    while 0 != x {
        s = s.offset(-1isize);
        *s =
            ('0' as i32 as
                 libc::c_uint).wrapping_add(x.wrapping_rem(10i32 as
                                                               libc::c_uint))
                as libc::c_char;
        x =
            (x as libc::c_uint).wrapping_div(10i32 as libc::c_uint) as
                uint32_t as uint32_t
    }
    return s;
}
/* Do not override this check. The floating point printing code below
 * depends on the float.h constants being right. If they are wrong, it
 * may overflow the stack. */
unsafe extern "C" fn fmt_fp(mut f: *mut fmt_args, mut y: f128::f128,
                            mut p: ptrdiff_t, mut fl: uint8_t,
                            mut t: libc::c_int) -> libc::c_int {
    // mantissa expansion
    let mut big: [uint32_t; 1835] = [0; 1835];
    // exponent expansion
    let mut a: *mut uint32_t = 0 as *mut uint32_t;
    let mut d: *mut uint32_t = 0 as *mut uint32_t;
    let mut r: *mut uint32_t = 0 as *mut uint32_t;
    let mut z: *mut uint32_t = 0 as *mut uint32_t;
    let mut i: uint32_t = 0;
    let mut e2: libc::c_int = 0i32;
    let mut e: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut l: ptrdiff_t = 0;
    let mut buf: [libc::c_char; 25] = [0; 25];
    let mut s: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut prefix: *const libc::c_char =
        b"-0X+0X 0X-0x+0x 0x\x00" as *const u8 as *const libc::c_char;
    let mut pl: ptrdiff_t = 0;
    let mut ebuf0: [libc::c_char; 12] = [0; 12];
    let mut ebuf: *mut libc::c_char =
        &mut *ebuf0.as_mut_ptr().offset((3i32 as
                                             libc::c_ulong).wrapping_mul(::std::mem::size_of::<libc::c_int>()
                                                                             as
                                                                             libc::c_ulong)
                                            as isize) as *mut libc::c_char;
    let mut estr: *mut libc::c_char = 0 as *mut libc::c_char;
    pl = 1i32 as ptrdiff_t;
    if 0 !=
           if ::std::mem::size_of::<f128::f128>() as libc::c_ulong ==
                  ::std::mem::size_of::<libc::c_float>() as libc::c_ulong {
               __inline_signbitf(y.to_f32().unwrap())
           } else if ::std::mem::size_of::<f128::f128>() as libc::c_ulong ==
                         ::std::mem::size_of::<libc::c_double>() as
                             libc::c_ulong {
               __inline_signbitd(y.to_f64().unwrap())
           } else { __inline_signbitl(y) } {
        y = -y
    } else if 0 != fl as libc::c_uint & 1u32 << '+' as i32 - ' ' as i32 {
        prefix = prefix.offset(3isize)
    } else if 0 != fl as libc::c_uint & 1u32 << ' ' as i32 - ' ' as i32 {
        prefix = prefix.offset(6isize)
    } else { prefix = prefix.offset(1isize); pl = 0i32 as ptrdiff_t }
    if 0 ==
           if ::std::mem::size_of::<f128::f128>() as libc::c_ulong ==
                  ::std::mem::size_of::<libc::c_float>() as libc::c_ulong {
               __inline_isfinitef(y.to_f32().unwrap())
           } else if ::std::mem::size_of::<f128::f128>() as libc::c_ulong ==
                         ::std::mem::size_of::<libc::c_double>() as
                             libc::c_ulong {
               __inline_isfinited(y.to_f64().unwrap())
           } else { __inline_isfinitel(y) } {
        let mut ss: *const libc::c_char =
            if 0 != t & 32i32 {
                b"inf\x00" as *const u8 as *const libc::c_char
            } else { b"INF\x00" as *const u8 as *const libc::c_char };
        if y != y {
            ss =
                if 0 != t & 32i32 {
                    b"nan\x00" as *const u8 as *const libc::c_char
                } else { b"NAN\x00" as *const u8 as *const libc::c_char }
        }
        pad(f, ' ' as i32 as libc::c_char, 0i32 as ptrdiff_t,
            3i32 as libc::c_long + pl,
            (fl as libc::c_uint & !(1u32 << '0' as i32 - ' ' as i32)) as
                uint8_t);
        out(f, prefix, pl as size_t);
        out(f, ss, 3i32 as size_t);
        pad(f, ' ' as i32 as libc::c_char, 0i32 as ptrdiff_t,
            3i32 as libc::c_long + pl,
            (fl as libc::c_uint ^ 1u32 << '-' as i32 - ' ' as i32) as
                uint8_t);
        return 3i32 + pl as libc::c_int
    }
    y =
        f128::f128::new(frexp(y.to_f64().unwrap(), &mut e2) *
                            2i32 as libc::c_double);
    if 0. != y { e2 -= 1 }
    if t | 32i32 == 'a' as i32 {
        let mut round: f128::f128 = f128::f128::new(8.0f64);
        let mut re: ptrdiff_t = 0;
        if 0 != t & 32i32 { prefix = prefix.offset(9isize) }
        pl += 2i32 as libc::c_long;
        if p < 0i32 as libc::c_long ||
               p >= (64i32 / 4i32 - 1i32) as libc::c_long {
            re = 0i32 as ptrdiff_t
        } else { re = (64i32 / 4i32 - 1i32) as libc::c_long - p }
        if 0 != re {
            loop  {
                let fresh0 = re;
                re = re - 1;
                if !(0 != fresh0) { break ; }
                round *= f128::f128::new(16i32)
            }
            if *prefix as libc::c_int == '-' as i32 {
                y = -y;
                y -= round;
                y += round;
                y = -y
            } else { y += round; y -= round }
        }
        estr = fmt_u((if e2 < 0i32 { -e2 } else { e2 }) as uint32_t, ebuf);
        if estr == ebuf {
            estr = estr.offset(-1isize);
            *estr = '0' as i32 as libc::c_char
        }
        estr = estr.offset(-1isize);
        *estr =
            (if e2 < 0i32 { '-' as i32 } else { '+' as i32 }) as libc::c_char;
        estr = estr.offset(-1isize);
        *estr = (t + ('p' as i32 - 'a' as i32)) as libc::c_char;
        s = buf.as_mut_ptr();
        loop  {
            let mut x: libc::c_int = y.to_i32().unwrap();
            let fresh1 = s;
            s = s.offset(1);
            *fresh1 =
                (xdigits[x as usize] as libc::c_int | t & 32i32) as
                    libc::c_char;
            y = f128::f128::new(16i32) * (y - f128::f128::new(x));
            if s.wrapping_offset_from(buf.as_mut_ptr()) as libc::c_long ==
                   1i32 as libc::c_long &&
                   (0. != y || p > 0i32 as libc::c_long ||
                        0 !=
                            fl as libc::c_uint &
                                1u32 << '#' as i32 - ' ' as i32) {
                let fresh2 = s;
                s = s.offset(1);
                *fresh2 = '.' as i32 as libc::c_char
            }
            if !(0. != y) { break ; }
        }
        if 0 != p &&
               (s.wrapping_offset_from(buf.as_mut_ptr()) as libc::c_long -
                    2i32 as libc::c_long) < p {
            l =
                p + 2i32 as libc::c_long +
                    ebuf.wrapping_offset_from(estr) as libc::c_long
        } else {
            l =
                s.wrapping_offset_from(buf.as_mut_ptr()) as libc::c_long +
                    ebuf.wrapping_offset_from(estr) as libc::c_long
        }
        pad(f, ' ' as i32 as libc::c_char, 0i32 as ptrdiff_t, pl + l, fl);
        out(f, prefix, pl as size_t);
        pad(f, '0' as i32 as libc::c_char, 0i32 as ptrdiff_t, pl + l,
            (fl as libc::c_uint ^ 1u32 << '0' as i32 - ' ' as i32) as
                uint8_t);
        out(f, buf.as_mut_ptr(),
            s.wrapping_offset_from(buf.as_mut_ptr()) as libc::c_long as
                size_t);
        pad(f, '0' as i32 as libc::c_char,
            l - ebuf.wrapping_offset_from(estr) as libc::c_long -
                s.wrapping_offset_from(buf.as_mut_ptr()) as libc::c_long,
            0i32 as ptrdiff_t, 0i32 as uint8_t);
        out(f, estr,
            ebuf.wrapping_offset_from(estr) as libc::c_long as size_t);
        pad(f, ' ' as i32 as libc::c_char, 0i32 as ptrdiff_t, pl + l,
            (fl as libc::c_uint ^ 1u32 << '-' as i32 - ' ' as i32) as
                uint8_t);
        return pl as libc::c_int + l as libc::c_int
    }
    if p < 0i32 as libc::c_long { p = 6i32 as ptrdiff_t }
    if 0. != y { y *= f128::f128::new(268435456.0f64); e2 -= 28i32 }
    if e2 < 0i32 {
        z = big.as_mut_ptr();
        r = z;
        a = r
    } else {
        z =
            big.as_mut_ptr().offset((::std::mem::size_of::<[uint32_t; 1835]>()
                                         as
                                         libc::c_ulong).wrapping_div(::std::mem::size_of::<uint32_t>()
                                                                         as
                                                                         libc::c_ulong)
                                        as
                                        isize).offset(-64isize).offset(-1isize);
        r = z;
        a = r
    }
    loop  {
        *z = y.to_u32().unwrap();
        let fresh3 = z;
        z = z.offset(1);
        y = f128::f128::new(1000000000i32) * (y - f128::f128::new(*fresh3));
        if !(0. != y) { break ; }
    }
    while e2 > 0i32 {
        let mut carry: uint32_t = 0i32 as uint32_t;
        let mut sh: libc::c_int = if 29i32 < e2 { 29i32 } else { e2 };
        d = z.offset(-1isize);
        while d >= a {
            let mut x_0: uint64_t =
                ((*d as uint64_t) <<
                     sh).wrapping_add(carry as libc::c_ulonglong);
            *d =
                x_0.wrapping_rem(1000000000i32 as libc::c_ulonglong) as
                    uint32_t;
            carry =
                x_0.wrapping_div(1000000000i32 as libc::c_ulonglong) as
                    uint32_t;
            d = d.offset(-1isize)
        }
        if 0 != carry { a = a.offset(-1isize); *a = carry }
        while z > a && 0 == *z.offset(-1i32 as isize) {
            z = z.offset(-1isize)
        }
        e2 -= sh
    }
    while e2 < 0i32 {
        let mut carry_0: uint32_t = 0i32 as uint32_t;
        let mut b: *mut uint32_t = 0 as *mut uint32_t;
        let mut sh_0: libc::c_int = if 9i32 < -e2 { 9i32 } else { -e2 };
        let mut need: libc::c_int =
            1i32 + (p as libc::c_int + 64i32 / 3i32 + 8i32) / 9i32;
        d = a;
        while d < z {
            let mut rm: uint32_t =
                *d & ((1i32 << sh_0) - 1i32) as libc::c_uint;
            *d = (*d >> sh_0).wrapping_add(carry_0);
            carry_0 =
                ((1000000000i32 >> sh_0) as libc::c_uint).wrapping_mul(rm);
            d = d.offset(1isize)
        }
        if 0 == *a { a = a.offset(1isize) }
        if 0 != carry_0 { let fresh4 = z; z = z.offset(1); *fresh4 = carry_0 }
        /* Avoid (slow!) computation past requested precision */
        b = if t | 32i32 == 'f' as i32 { r } else { a };
        if z.wrapping_offset_from(b) as libc::c_long > need as libc::c_long {
            z = b.offset(need as isize)
        }
        e2 += sh_0
    }
    if a < z {
        i = 10i32 as uint32_t;
        e = 9i32 * r.wrapping_offset_from(a) as libc::c_long as libc::c_int;
        while *a >= i {
            i =
                (i as libc::c_uint).wrapping_mul(10i32 as libc::c_uint) as
                    uint32_t as uint32_t;
            e += 1
        }
    } else { e = 0i32 }
    /* Perform rounding: j is precision after the radix (possibly neg) */
    j =
        p as libc::c_int - (t | 32i32 != 'f' as i32) as libc::c_int * e -
            (t | 32i32 == 'g' as i32 && 0 != p) as libc::c_int;
    if (j as libc::c_long) <
           9i32 as libc::c_long *
               (z.wrapping_offset_from(r) as libc::c_long -
                    1i32 as libc::c_long) {
        let mut x_1: uint32_t = 0;
        /* We avoid C's broken division of negative numbers */
        d =
            r.offset(1isize).offset(((j + 9i32 * 16384i32) / 9i32 - 16384i32)
                                        as isize);
        j += 9i32 * 16384i32;
        j %= 9i32;
        i = 10i32 as uint32_t;
        j += 1;
        while j < 9i32 {
            i =
                (i as libc::c_uint).wrapping_mul(10i32 as libc::c_uint) as
                    uint32_t as uint32_t;
            j += 1
        }
        x_1 = (*d).wrapping_rem(i);
        /* Are there any significant digits past j? */
        if 0 != x_1 || d.offset(1isize) != z {
            let mut round_0: f128::f128 =
                f128::f128::new(2i32) /
                    f128::f128::new(1.08420217248550443401e-19);
            let mut small: f128::f128 = f128::f128::ZERO;
            if 0 != (*d).wrapping_div(i) & 1i32 as libc::c_uint {
                round_0 += f128::f128::new(2i32)
            }
            if x_1 < i.wrapping_div(2i32 as libc::c_uint) {
                small = f128::f128::new(0.5f64)
            } else if x_1 == i.wrapping_div(2i32 as libc::c_uint) &&
                          d.offset(1isize) == z {
                small = f128::f128::new(1.0f64)
            } else { small = f128::f128::new(1.5f64) }
            if 0 != pl && *prefix as libc::c_int == '-' as i32 {
                round_0 *= f128::f128::new(-1i32);
                small *= f128::f128::new(-1i32)
            }
            *d =
                (*d as libc::c_uint).wrapping_sub(x_1) as uint32_t as
                    uint32_t;
            /* Decide whether to round by probing round+small */
            if round_0 + small != round_0 {
                *d = (*d).wrapping_add(i);
                while *d > 999999999i32 as libc::c_uint {
                    let fresh5 = d;
                    d = d.offset(-1);
                    *fresh5 = 0i32 as uint32_t;
                    if d < a { a = a.offset(-1isize); *a = 0i32 as uint32_t }
                    *d = (*d).wrapping_add(1)
                }
                i = 10i32 as uint32_t;
                e =
                    9i32 *
                        r.wrapping_offset_from(a) as libc::c_long as
                            libc::c_int;
                while *a >= i {
                    i =
                        (i as
                             libc::c_uint).wrapping_mul(10i32 as libc::c_uint)
                            as uint32_t as uint32_t;
                    e += 1
                }
            }
        }
        if z > d.offset(1isize) { z = d.offset(1isize) }
    }
    while z > a && 0 == *z.offset(-1i32 as isize) { z = z.offset(-1isize) }
    if t | 32i32 == 'g' as i32 {
        if 0 == p { p += 1 }
        if p > e as libc::c_long && e >= -4i32 {
            t -= 1;
            p -= (e + 1i32) as libc::c_long
        } else { t -= 2i32; p -= 1 }
        if 0 == fl as libc::c_uint & 1u32 << '#' as i32 - ' ' as i32 {
            /* Count trailing zeros in last place */
            if z > a && 0 != *z.offset(-1i32 as isize) {
                i = 10i32 as uint32_t;
                j = 0i32;
                while (*z.offset(-1i32 as isize)).wrapping_rem(i) ==
                          0i32 as libc::c_uint {
                    i =
                        (i as
                             libc::c_uint).wrapping_mul(10i32 as libc::c_uint)
                            as uint32_t as uint32_t;
                    j += 1
                }
            } else { j = 9i32 }
            if t | 32i32 == 'f' as i32 {
                p =
                    if p <
                           (if 0i32 as libc::c_long >
                                   9i32 as libc::c_long *
                                       (z.wrapping_offset_from(r) as
                                            libc::c_long -
                                            1i32 as libc::c_long) -
                                       j as libc::c_long {
                                0i32 as libc::c_long
                            } else {
                                9i32 as libc::c_long *
                                    (z.wrapping_offset_from(r) as libc::c_long
                                         - 1i32 as libc::c_long) -
                                    j as libc::c_long
                            }) {
                        p
                    } else if 0i32 as libc::c_long >
                                  9i32 as libc::c_long *
                                      (z.wrapping_offset_from(r) as
                                           libc::c_long -
                                           1i32 as libc::c_long) -
                                      j as libc::c_long {
                        0i32 as libc::c_long
                    } else {
                        9i32 as libc::c_long *
                            (z.wrapping_offset_from(r) as libc::c_long -
                                 1i32 as libc::c_long) - j as libc::c_long
                    }
            } else {
                p =
                    if p <
                           (if 0i32 as libc::c_long >
                                   9i32 as libc::c_long *
                                       (z.wrapping_offset_from(r) as
                                            libc::c_long -
                                            1i32 as libc::c_long) +
                                       e as libc::c_long - j as libc::c_long {
                                0i32 as libc::c_long
                            } else {
                                9i32 as libc::c_long *
                                    (z.wrapping_offset_from(r) as libc::c_long
                                         - 1i32 as libc::c_long) +
                                    e as libc::c_long - j as libc::c_long
                            }) {
                        p
                    } else if 0i32 as libc::c_long >
                                  9i32 as libc::c_long *
                                      (z.wrapping_offset_from(r) as
                                           libc::c_long -
                                           1i32 as libc::c_long) +
                                      e as libc::c_long - j as libc::c_long {
                        0i32 as libc::c_long
                    } else {
                        9i32 as libc::c_long *
                            (z.wrapping_offset_from(r) as libc::c_long -
                                 1i32 as libc::c_long) + e as libc::c_long -
                            j as libc::c_long
                    }
            }
        }
    }
    l =
        1i32 as libc::c_long + p +
            (0 != p ||
                 0 != fl as libc::c_uint & 1u32 << '#' as i32 - ' ' as i32) as
                libc::c_int as libc::c_long;
    if t | 32i32 == 'f' as i32 {
        if e > 0i32 { l += e as libc::c_long }
    } else {
        estr = fmt_u((if e < 0i32 { -e } else { e }) as uint32_t, ebuf);
        while (ebuf.wrapping_offset_from(estr) as libc::c_long) <
                  2i32 as libc::c_long {
            estr = estr.offset(-1isize);
            *estr = '0' as i32 as libc::c_char
        }
        estr = estr.offset(-1isize);
        *estr =
            (if e < 0i32 { '-' as i32 } else { '+' as i32 }) as libc::c_char;
        estr = estr.offset(-1isize);
        *estr = t as libc::c_char;
        l += ebuf.wrapping_offset_from(estr) as libc::c_long
    }
    pad(f, ' ' as i32 as libc::c_char, 0i32 as ptrdiff_t, pl + l, fl);
    out(f, prefix, pl as size_t);
    pad(f, '0' as i32 as libc::c_char, 0i32 as ptrdiff_t, pl + l,
        (fl as libc::c_uint ^ 1u32 << '0' as i32 - ' ' as i32) as uint8_t);
    if t | 32i32 == 'f' as i32 {
        if a > r { a = r }
        d = a;
        while d <= r {
            let mut ss_0: *mut libc::c_char =
                fmt_u(*d, buf.as_mut_ptr().offset(9isize));
            if d != a {
                while ss_0 > buf.as_mut_ptr() {
                    ss_0 = ss_0.offset(-1isize);
                    *ss_0 = '0' as i32 as libc::c_char
                }
            } else if ss_0 == buf.as_mut_ptr().offset(9isize) {
                ss_0 = ss_0.offset(-1isize);
                *ss_0 = '0' as i32 as libc::c_char
            }
            out(f, ss_0,
                buf.as_mut_ptr().offset(9isize).wrapping_offset_from(ss_0) as
                    libc::c_long as size_t);
            d = d.offset(1isize)
        }
        if 0 != p || 0 != fl as libc::c_uint & 1u32 << '#' as i32 - ' ' as i32
           {
            out(f, b".\x00" as *const u8 as *const libc::c_char,
                1i32 as size_t);
        }
        while d < z && p > 0i32 as libc::c_long {
            let mut ss_1: *mut libc::c_char =
                fmt_u(*d, buf.as_mut_ptr().offset(9isize));
            while ss_1 > buf.as_mut_ptr() {
                ss_1 = ss_1.offset(-1isize);
                *ss_1 = '0' as i32 as libc::c_char
            }
            out(f, ss_1,
                (if (9i32 as libc::c_long) < p {
                     9i32 as libc::c_long
                 } else { p }) as size_t);
            d = d.offset(1isize);
            p -= 9i32 as libc::c_long
        }
        pad(f, '0' as i32 as libc::c_char, p + 9i32 as libc::c_long,
            9i32 as ptrdiff_t, 0i32 as uint8_t);
    } else {
        if z <= a { z = a.offset(1isize) }
        d = a;
        while d < z && p >= 0i32 as libc::c_long {
            let mut ss_2: *mut libc::c_char =
                fmt_u(*d, buf.as_mut_ptr().offset(9isize));
            if ss_2 == buf.as_mut_ptr().offset(9isize) {
                ss_2 = ss_2.offset(-1isize);
                *ss_2 = '0' as i32 as libc::c_char
            }
            if d != a {
                while ss_2 > buf.as_mut_ptr() {
                    ss_2 = ss_2.offset(-1isize);
                    *ss_2 = '0' as i32 as libc::c_char
                }
            } else {
                let fresh6 = ss_2;
                ss_2 = ss_2.offset(1);
                out(f, fresh6, 1i32 as size_t);
                if p > 0i32 as libc::c_long ||
                       0 !=
                           fl as libc::c_uint &
                               1u32 << '#' as i32 - ' ' as i32 {
                    out(f, b".\x00" as *const u8 as *const libc::c_char,
                        1i32 as size_t);
                }
            }
            out(f, ss_2,
                (if (buf.as_mut_ptr().offset(9isize).wrapping_offset_from(ss_2)
                         as libc::c_long) < p {
                     buf.as_mut_ptr().offset(9isize).wrapping_offset_from(ss_2)
                         as libc::c_long
                 } else { p }) as size_t);
            p -=
                buf.as_mut_ptr().offset(9isize).wrapping_offset_from(ss_2) as
                    libc::c_long as libc::c_int as libc::c_long;
            d = d.offset(1isize)
        }
        pad(f, '0' as i32 as libc::c_char, p + 18i32 as libc::c_long,
            18i32 as ptrdiff_t, 0i32 as uint8_t);
        out(f, estr,
            ebuf.wrapping_offset_from(estr) as libc::c_long as size_t);
    }
    pad(f, ' ' as i32 as libc::c_char, 0i32 as ptrdiff_t, pl + l,
        (fl as libc::c_uint ^ 1u32 << '-' as i32 - ' ' as i32) as uint8_t);
    return pl as libc::c_int + l as libc::c_int;
}
unsafe extern "C" fn fmt_core(mut f: *mut fmt_args,
                              mut fmt: *const libc::c_char,
                              mut flo: mrb_float) -> libc::c_int {
    let mut p: ptrdiff_t = 0;
    if *fmt as libc::c_int != '%' as i32 { return -1i32 }
    fmt = fmt.offset(1isize);
    if *fmt as libc::c_int == '.' as i32 {
        fmt = fmt.offset(1isize);
        p = 0i32 as ptrdiff_t;
        while (*fmt as libc::c_uint).wrapping_sub('0' as i32 as libc::c_uint)
                  < 10i32 as libc::c_uint {
            p =
                10i32 as libc::c_long * p +
                    (*fmt as libc::c_int - '0' as i32) as libc::c_long;
            fmt = fmt.offset(1isize)
        }
    } else { p = -1i32 as ptrdiff_t }
    match *fmt as libc::c_int {
        101 | 102 | 103 | 97 | 69 | 70 | 71 | 65 => {
            return fmt_fp(f, f128::f128::new(flo), p, 0i32 as uint8_t,
                          *fmt as libc::c_int)
        }
        _ => { return -1i32 }
    };
}
#[no_mangle]
pub unsafe extern "C" fn mrb_float_to_str(mut mrb: *mut mrb_state,
                                          mut flo: mrb_value,
                                          mut fmt: *const libc::c_char)
 -> mrb_value {
    let mut f: fmt_args =
        fmt_args{mrb: 0 as *mut mrb_state,
                 str_0:
                     mrb_value{value: C2RustUnnamed_3{f: 0.,},
                               tt: MRB_TT_FALSE,},};
    f.mrb = mrb;
    f.str_0 = mrb_str_new_capa(mrb, 24i32 as size_t);
    if fmt_core(&mut f, fmt, flo.value.f) < 0i32 {
        mrb_raise(mrb,
                  mrb_exc_get(mrb,
                              b"ArgumentError\x00" as *const u8 as
                                  *const libc::c_char),
                  b"invalid format string\x00" as *const u8 as
                      *const libc::c_char);
    }
    return f.str_0;
}