use libc;
use c2rust_bitfields::{BitfieldStruct};
extern "C" {
    /* debug info */
    pub type mrb_irep_debug_info;
    pub type symbol_name;
    pub type mrb_jmpbuf;
    pub type mrb_shared_string;
    #[no_mangle]
    fn __assert_rtn(_: *const libc::c_char, _: *const libc::c_char,
                    _: libc::c_int, _: *const libc::c_char) -> !;
    /* *
 * Create a symbol
 *
 *     # Ruby style:
 *     :pizza # => :pizza
 *
 *     // C style:
 *     mrb_sym m_sym = mrb_intern_lit(mrb, "pizza"); //  => :pizza
 * @param [mrb_state*] mrb_state* The current mruby state.
 * @param [const char*] const char* The name of the method.
 * @return [mrb_sym] mrb_sym A symbol.
 */
    #[no_mangle]
    fn mrb_intern_cstr(_: *mut mrb_state, _: *const libc::c_char) -> mrb_sym;
    #[no_mangle]
    fn mrb_write_barrier(_: *mut mrb_state, _: *mut RBasic);
    #[no_mangle]
    fn mrb_malloc(_: *mut mrb_state, _: size_t) -> *mut libc::c_void;
    #[no_mangle]
    fn mrb_intern_static(_: *mut mrb_state, _: *const libc::c_char, _: size_t)
     -> mrb_sym;
    #[no_mangle]
    fn mrb_sym2name(_: *mut mrb_state, _: mrb_sym) -> *const libc::c_char;
    #[no_mangle]
    fn mrb_frozen_error(mrb: *mut mrb_state, frozen_obj: *mut libc::c_void)
     -> !;
    /* *
 * Gets a exception class.
 * @param [mrb_state*] mrb The current mruby state.
 * @param [const char *] name The name of the class.
 * @return [struct RClass *] A reference to the class.
*/
    #[no_mangle]
    fn mrb_exc_get(mrb: *mut mrb_state, name: *const libc::c_char)
     -> *mut RClass;
    /* *
 * Retrieve arguments from mrb_state.
 *
 * @param mrb The current MRuby state.
 * @param format [mrb_args_format] is a list of format specifiers
 * @param ... The passing variadic arguments must be a pointer of retrieving type.
 * @return the number of arguments retrieved.
 * @see mrb_args_format
 */
    #[no_mangle]
    fn mrb_get_args(mrb: *mut mrb_state, format: mrb_args_format, _: ...)
     -> mrb_int;
    /* *
 * Call existing ruby functions. This is basically the type safe version of mrb_funcall.
 *
 *      #include <stdio.h>
 *      #include <mruby.h>
 *      #include "mruby/compile.h"
 *      int
 *      main()
 *      {
 *        mrb_int i = 99;
 *        mrb_state *mrb = mrb_open();
 *
 *        if (!mrb) { }
 *        mrb_sym m_sym = mrb_intern_lit(mrb, "method_name"); // Symbol for method.
 *
 *        FILE *fp = fopen("test.rb","r");
 *        mrb_value obj = mrb_load_file(mrb,fp);
 *        mrb_funcall_argv(mrb, obj, m_sym, 1, &obj); // Calling ruby function from test.rb.
 *        fclose(fp);
 *        mrb_close(mrb);
 *       }
 * @param [mrb_state*] mrb_state* The current mruby state.
 * @param [mrb_value] mrb_value A reference to an mruby value.
 * @param [mrb_sym] mrb_sym The symbol representing the method.
 * @param [mrb_int] mrb_int The number of arguments the method has.
 * @param [const mrb_value*] mrb_value* Pointer to the object.
 * @return [mrb_value] mrb_value mruby function value.
 * @see mrb_funcall
 */
    #[no_mangle]
    fn mrb_funcall_argv(_: *mut mrb_state, _: mrb_value, _: mrb_sym,
                        _: mrb_int, _: *const mrb_value) -> mrb_value;
    #[no_mangle]
    fn mrb_sym2name_len(_: *mut mrb_state, _: mrb_sym, _: *mut mrb_int)
     -> *const libc::c_char;
    #[no_mangle]
    fn mrb_sym2str(_: *mut mrb_state, _: mrb_sym) -> mrb_value;
    #[no_mangle]
    fn mrb_free(_: *mut mrb_state, _: *mut libc::c_void);
    #[no_mangle]
    fn mrb_inspect(mrb: *mut mrb_state, obj: mrb_value) -> mrb_value;
    #[no_mangle]
    fn mrb_gc_mark(_: *mut mrb_state, _: *mut RBasic);
    #[no_mangle]
    fn mrb_field_write_barrier(_: *mut mrb_state, _: *mut RBasic,
                               _: *mut RBasic);
    #[no_mangle]
    fn mrb_any_to_s(mrb: *mut mrb_state, obj: mrb_value) -> mrb_value;
    #[no_mangle]
    fn mrb_obj_classname(mrb: *mut mrb_state, obj: mrb_value)
     -> *const libc::c_char;
    #[no_mangle]
    fn mrb_raise(mrb: *mut mrb_state, c: *mut RClass,
                 msg: *const libc::c_char) -> !;
    #[no_mangle]
    fn mrb_name_error(mrb: *mut mrb_state, id: mrb_sym,
                      fmt: *const libc::c_char, _: ...) -> !;
    #[no_mangle]
    fn mrb_class_name(mrb: *mut mrb_state, klass: *mut RClass)
     -> *const libc::c_char;
    /*
 * Initializes a new array.
 *
 * Equivalent to:
 *
 *      Array.new
 *
 * @param mrb The mruby state reference.
 * @return The initialized array.
 */
    #[no_mangle]
    fn mrb_ary_new(mrb: *mut mrb_state) -> mrb_value;
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
    #[no_mangle]
    fn mrb_class_name_class(_: *mut mrb_state, _: *mut RClass, _: *mut RClass,
                            _: mrb_sym);
    /*
 * Duplicates a string object.
 *
 *
 * @param [mrb_state] mrb The current mruby state.
 * @param [mrb_value] str Ruby string.
 * @return [mrb_value] Duplicated Ruby string.
 */
    #[no_mangle]
    fn mrb_str_dup(mrb: *mut mrb_state, str: mrb_value) -> mrb_value;
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
    #[no_mangle]
    fn mrb_str_new_capa(mrb: *mut mrb_state, capa: size_t) -> mrb_value;
    /*
 * Appends self to other. Returns self as a concatenated string.
 *
 *
 *  Example:
 *
 *     !!!c
 *     int
 *     main(int argc,
 *          char **argv)
 *     {
 *       // Variable declarations.
 *       mrb_value str1;
 *       mrb_value str2;
 *
 *       mrb_state *mrb = mrb_open();
 *       if (!mrb)
 *       {
 *          // handle error
 *       }
 *
 *       // Creates new Ruby strings.
 *       str1 = mrb_str_new_lit(mrb, "abc");
 *       str2 = mrb_str_new_lit(mrb, "def");
 *
 *       // Concatenates str2 to str1.
 *       mrb_str_concat(mrb, str1, str2);
 *
 *      // Prints new Concatenated Ruby string.
 *      mrb_p(mrb, str1);
 *
 *      mrb_close(mrb);
 *      return 0;
 *    }
 *
 *
 *  Result:
 *
 *     => "abcdef"
 *
 * @param [mrb_state] mrb The current mruby state.
 * @param [mrb_value] self String to concatenate.
 * @param [mrb_value] other String to append to self.
 * @return [mrb_value] Returns a new String appending other to self.
 */
    #[no_mangle]
    fn mrb_str_concat(_: *mut mrb_state, _: mrb_value, _: mrb_value);
    /*
 * Converts pointer into a Ruby string.
 *
 * @param [mrb_state] mrb The current mruby state.
 * @param [void*] p The pointer to convert to Ruby string.
 * @return [mrb_value] Returns a new Ruby String.
 */
    #[no_mangle]
    fn mrb_ptr_to_str(_: *mut mrb_state, _: *mut libc::c_void) -> mrb_value;
    #[no_mangle]
    fn mrb_str_cat_str(mrb: *mut mrb_state, str: mrb_value, str2: mrb_value)
     -> mrb_value;
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
/* Instance variable table structure */
#[derive ( Copy , Clone )]
#[repr(C)]
pub struct iv_tbl {
    pub rootseg: *mut segment,
    pub size: size_t,
    pub last_len: size_t,
}
/*
** variable.c - mruby variables
**
** See Copyright Notice in mruby.h
*/
#[derive ( Copy , Clone )]
#[repr(C)]
pub struct segment {
    pub key: [mrb_sym; 4],
    pub val: [mrb_value; 4],
    pub next: *mut segment,
}
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
pub union unnamed {
    pub f: mrb_float,
    pub p: *mut libc::c_void,
    pub i: mrb_int,
    pub sym: mrb_sym,
}
pub type mrb_int = int64_t;
pub type mrb_float = libc::c_double;
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
    pub unnamed: unnamed_0,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub union unnamed_0 {
    pub proc_0: *mut RProc,
    pub func: mrb_func_t,
}
/* default method cache size: 128 */
/* cache size needs to be power of 2 */
pub type mrb_func_t
    =
    Option<unsafe extern "C" fn(_: *mut mrb_state, _: mrb_value)
               -> mrb_value>;
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
    pub body: unnamed_2,
    pub upper: *mut RProc,
    pub e: unnamed_1,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub union unnamed_1 {
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
#[derive ( Copy , Clone )]
#[repr ( C )]
pub union unnamed_2 {
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
/* *
 * Function requires n arguments.
 *
 * @param n
 *      The number of required arguments.
 */
/* *
 * Function takes n optional arguments
 *
 * @param n
 *      The number of optional arguments.
 */
/* *
 * Function takes n1 mandatory arguments and n2 optional arguments
 *
 * @param n1
 *      The number of required arguments.
 * @param n2
 *      The number of optional arguments.
 */
/* * rest argument */
/* * required arguments after rest */
/* * keyword arguments (n of keys, kdict) */
/* *
 * Function takes a block argument
 */
/* *
 * Function accepts any number of arguments
 */
/* *
 * Function accepts no arguments
 */
/* *
 * Format specifiers for {mrb_get_args} function
 *
 * Must be a C string composed of the following format specifiers:
 *
 * | char | Ruby type      | C types           | Notes                                              |
 * |:----:|----------------|-------------------|----------------------------------------------------|
 * | `o`  | {Object}       | {mrb_value}       | Could be used to retrieve any type of argument     |
 * | `C`  | {Class}/{Module} | {mrb_value}     |                                                    |
 * | `S`  | {String}       | {mrb_value}       | when `!` follows, the value may be `nil`           |
 * | `A`  | {Array}        | {mrb_value}       | when `!` follows, the value may be `nil`           |
 * | `H`  | {Hash}         | {mrb_value}       | when `!` follows, the value may be `nil`           |
 * | `s`  | {String}       | char *, {mrb_int} | Receive two arguments; `s!` gives (`NULL`,`0`) for `nil`       |
 * | `z`  | {String}       | char *            | `NULL` terminated string; `z!` gives `NULL` for `nil`           |
 * | `a`  | {Array}        | {mrb_value} *, {mrb_int} | Receive two arguments; `a!` gives (`NULL`,`0`) for `nil` |
 * | `f`  | {Float}        | {mrb_float}       |                                                    |
 * | `i`  | {Integer}      | {mrb_int}         |                                                    |
 * | `b`  | boolean        | {mrb_bool}        |                                                    |
 * | `n`  | {Symbol}       | {mrb_sym}         |                                                    |
 * | `&`  | block          | {mrb_value}       | &! raises exception if no block given.             |
 * | `*`  | rest arguments | {mrb_value} *, {mrb_int} | Receive the rest of arguments as an array; *! avoid copy of the stack.  |
 * | &vert; | optional     |                   | After this spec following specs would be optional. |
 * | `?`  | optional given | {mrb_bool}        | `TRUE` if preceding argument is given. Used to check optional argument is given. |
 *
 * @see mrb_get_args
 */
pub type mrb_args_format = *const libc::c_char;
#[derive ( Copy , Clone )]
#[repr(C)]
pub struct unnamed_3 {
    pub len: mrb_int,
    pub aux: unnamed_4,
    pub ptr: *mut libc::c_char,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub union unnamed_4 {
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
    pub as_0: unnamed_5,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub union unnamed_5 {
    pub heap: unnamed_3,
    pub ary: [libc::c_char; 24],
}
#[derive ( Copy , Clone )]
#[repr(C)]
pub struct csym_arg {
    pub c: *mut RClass,
    pub sym: mrb_sym,
}
/* return non zero to break the loop */
pub type mrb_iv_foreach_func
    =
    unsafe extern "C" fn(_: *mut mrb_state, _: mrb_sym, _: mrb_value,
                         _: *mut libc::c_void) -> libc::c_int;
/*
 * Returns a fixnum in Ruby.
 */
#[inline]
unsafe extern "C" fn mrb_fixnum_value(mut i: mrb_int) -> mrb_value {
    let mut v: mrb_value =
        mrb_value{value: unnamed{f: 0.,}, tt: MRB_TT_FALSE,};
    v.tt = MRB_TT_FIXNUM;
    v.value.i = i;
    return v;
}
#[inline]
unsafe extern "C" fn mrb_symbol_value(mut i: mrb_sym) -> mrb_value {
    let mut v: mrb_value =
        mrb_value{value: unnamed{f: 0.,}, tt: MRB_TT_FALSE,};
    v.tt = MRB_TT_SYMBOL;
    v.value.sym = i;
    return v;
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
/*
 * Get a nil mrb_value object.
 *
 * @return
 *      nil mrb_value object reference.
 */
#[inline]
unsafe extern "C" fn mrb_nil_value() -> mrb_value {
    let mut v: mrb_value =
        mrb_value{value: unnamed{f: 0.,}, tt: MRB_TT_FALSE,};
    v.tt = MRB_TT_FALSE;
    v.value.i = 0i32 as mrb_int;
    return v;
}
#[inline]
unsafe extern "C" fn mrb_undef_value() -> mrb_value {
    let mut v: mrb_value =
        mrb_value{value: unnamed{f: 0.,}, tt: MRB_TT_FALSE,};
    v.tt = MRB_TT_UNDEF;
    v.value.i = 0i32 as mrb_int;
    return v;
}
/* *
 *  Defines a constant.
 *
 * Example:
 *
 *          # Ruby style
 *          class ExampleClass
 *            AGE = 22
 *          end
 *          // C style
 *          #include <stdio.h>
 *          #include <mruby.h>
 *
 *          void
 *          mrb_example_gem_init(mrb_state* mrb){
 *            mrb_define_const(mrb, mrb->kernel_module, "AGE", mrb_fixnum_value(22));
 *          }
 *
 *          mrb_value
 *          mrb_example_gem_final(mrb_state* mrb){
 *          }
 *  @param [mrb_state *] mrb_state* The MRuby state reference.
 *  @param [struct RClass *] RClass* A class or module the constant is defined in.
 *  @param [const char *] name The name of the constant being defined.
 *  @param [mrb_value] mrb_value The value for the constant.
 */
#[no_mangle]
pub unsafe extern "C" fn mrb_define_const(mut mrb: *mut mrb_state,
                                          mut mod_0: *mut RClass,
                                          mut name: *const libc::c_char,
                                          mut v: mrb_value) {
    mrb_obj_iv_set(mrb, mod_0 as *mut RObject, mrb_intern_cstr(mrb, name), v);
}
#[no_mangle]
pub unsafe extern "C" fn mrb_obj_iv_set(mut mrb: *mut mrb_state,
                                        mut obj: *mut RObject,
                                        mut sym: mrb_sym, mut v: mrb_value) {
    mrb_check_frozen(mrb, obj as *mut libc::c_void);
    mrb_obj_iv_set_force(mrb, obj, sym, v);
}
#[no_mangle]
pub unsafe extern "C" fn mrb_obj_iv_set_force(mut mrb: *mut mrb_state,
                                              mut obj: *mut RObject,
                                              mut sym: mrb_sym,
                                              mut v: mrb_value) {
    assign_class_name(mrb, obj, sym, v);
    if (*obj).iv.is_null() { (*obj).iv = iv_new(mrb) }
    iv_put(mrb, (*obj).iv, sym, v);
    mrb_write_barrier(mrb, obj as *mut RBasic);
}
/* Set the value for the symbol in the instance variable table. */
unsafe extern "C" fn iv_put(mut mrb: *mut mrb_state, mut t: *mut iv_tbl,
                            mut sym: mrb_sym, mut val: mrb_value) {
    let mut seg: *mut segment = 0 as *mut segment;
    let mut prev: *mut segment = 0 as *mut segment;
    let mut matched_seg: *mut segment = 0 as *mut segment;
    let mut matched_idx: size_t = 0i32 as size_t;
    let mut i: size_t = 0;
    if t.is_null() { return }
    seg = (*t).rootseg;
    while !seg.is_null() {
        i = 0i32 as size_t;
        while i < 4i32 as libc::c_ulong {
            let mut key: mrb_sym = (*seg).key[i as usize];
            if (*seg).next.is_null() && i >= (*t).last_len {
                (*seg).key[i as usize] = sym;
                (*seg).val[i as usize] = val;
                (*t).last_len = i.wrapping_add(1i32 as libc::c_ulong);
                (*t).size = (*t).size.wrapping_add(1);
                return
            }
            if matched_seg.is_null() && key == 0i32 as libc::c_uint {
                matched_seg = seg;
                matched_idx = i
            } else if key == sym { (*seg).val[i as usize] = val; return }
            i = i.wrapping_add(1)
        }
        prev = seg;
        seg = (*seg).next
    }
    if !matched_seg.is_null() {
        (*matched_seg).key[matched_idx as usize] = sym;
        (*matched_seg).val[matched_idx as usize] = val;
        (*t).size = (*t).size.wrapping_add(1);
        return
    }
    seg =
        mrb_malloc(mrb, ::std::mem::size_of::<segment>() as libc::c_ulong) as
            *mut segment;
    (*seg).next = 0 as *mut segment;
    (*seg).key[0usize] = sym;
    (*seg).val[0usize] = val;
    (*t).last_len = 1i32 as size_t;
    (*t).size = (*t).size.wrapping_add(1);
    if !prev.is_null() { (*prev).next = seg } else { (*t).rootseg = seg };
}
/* Creates the instance variable table. */
unsafe extern "C" fn iv_new(mut mrb: *mut mrb_state) -> *mut iv_tbl {
    let mut t: *mut iv_tbl = 0 as *mut iv_tbl;
    t =
        mrb_malloc(mrb, ::std::mem::size_of::<iv_tbl>() as libc::c_ulong) as
            *mut iv_tbl;
    (*t).size = 0i32 as size_t;
    (*t).rootseg = 0 as *mut segment;
    (*t).last_len = 0i32 as size_t;
    return t;
}
#[inline]
unsafe extern "C" fn assign_class_name(mut mrb: *mut mrb_state,
                                       mut obj: *mut RObject,
                                       mut sym: mrb_sym, mut v: mrb_value) {
    if 0 != namespace_p((*obj).tt()) as libc::c_int &&
           0 != namespace_p(v.tt) as libc::c_int {
        let mut c: *mut RObject = v.value.p as *mut RObject;
        if obj != c &&
               (*mrb_sym2name(mrb, sym).offset(0isize) as
                    libc::c_uint).wrapping_sub('A' as i32 as libc::c_uint) <
                   26i32 as libc::c_uint {
            let mut id_classname: mrb_sym =
                mrb_intern_static(mrb,
                                  b"__classname__\x00" as *const u8 as
                                      *const libc::c_char,
                                  (::std::mem::size_of::<[libc::c_char; 14]>()
                                       as
                                       libc::c_ulong).wrapping_sub(1i32 as
                                                                       libc::c_ulong));
            let mut o: mrb_value = mrb_obj_iv_get(mrb, c, id_classname);
            if o.tt as libc::c_uint ==
                   MRB_TT_FALSE as libc::c_int as libc::c_uint &&
                   0 == o.value.i {
                let mut id_outer: mrb_sym =
                    mrb_intern_static(mrb,
                                      b"__outer__\x00" as *const u8 as
                                          *const libc::c_char,
                                      (::std::mem::size_of::<[libc::c_char; 10]>()
                                           as
                                           libc::c_ulong).wrapping_sub(1i32 as
                                                                           libc::c_ulong));
                o = mrb_obj_iv_get(mrb, c, id_outer);
                if o.tt as libc::c_uint ==
                       MRB_TT_FALSE as libc::c_int as libc::c_uint &&
                       0 == o.value.i {
                    if obj as *mut RClass == (*mrb).object_class {
                        mrb_obj_iv_set_force(mrb, c, id_classname,
                                             mrb_symbol_value(sym));
                    } else {
                        mrb_obj_iv_set_force(mrb, c, id_outer,
                                             mrb_obj_value(obj as
                                                               *mut libc::c_void));
                    }
                }
            }
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn mrb_obj_iv_get(mut mrb: *mut mrb_state,
                                        mut obj: *mut RObject,
                                        mut sym: mrb_sym) -> mrb_value {
    let mut v: mrb_value =
        mrb_value{value: unnamed{f: 0.,}, tt: MRB_TT_FALSE,};
    if !(*obj).iv.is_null() &&
           0 != iv_get(mrb, (*obj).iv, sym, &mut v) as libc::c_int {
        return v
    }
    return mrb_nil_value();
}
/* Get a value for a symbol from the instance variable table. */
unsafe extern "C" fn iv_get(mut mrb: *mut mrb_state, mut t: *mut iv_tbl,
                            mut sym: mrb_sym, mut vp: *mut mrb_value)
 -> mrb_bool {
    let mut seg: *mut segment = 0 as *mut segment;
    let mut i: size_t = 0;
    if t.is_null() { return 0i32 as mrb_bool }
    seg = (*t).rootseg;
    while !seg.is_null() {
        i = 0i32 as size_t;
        while i < 4i32 as libc::c_ulong {
            let mut key: mrb_sym = (*seg).key[i as usize];
            if (*seg).next.is_null() && i >= (*t).last_len {
                return 0i32 as mrb_bool
            }
            if key == sym {
                if !vp.is_null() { *vp = (*seg).val[i as usize] }
                return 1i32 as mrb_bool
            }
            i = i.wrapping_add(1)
        }
        seg = (*seg).next
    }
    return 0i32 as mrb_bool;
}
#[inline]
unsafe extern "C" fn namespace_p(mut tt: mrb_vtype) -> mrb_bool {
    return (if tt as libc::c_uint ==
                   MRB_TT_CLASS as libc::c_int as libc::c_uint ||
                   tt as libc::c_uint ==
                       MRB_TT_MODULE as libc::c_int as libc::c_uint {
                1i32
            } else { 0i32 }) as mrb_bool;
}
#[inline]
unsafe extern "C" fn mrb_check_frozen(mut mrb: *mut mrb_state,
                                      mut o: *mut libc::c_void) {
    if 0 != (*(o as *mut RBasic)).flags() as libc::c_int & 1i32 << 20i32 {
        mrb_frozen_error(mrb, o);
    };
}
#[no_mangle]
pub unsafe extern "C" fn mrb_define_global_const(mut mrb: *mut mrb_state,
                                                 mut name:
                                                     *const libc::c_char,
                                                 mut val: mrb_value) {
    mrb_define_const(mrb, (*mrb).object_class, name, val);
}
#[no_mangle]
pub unsafe extern "C" fn mrb_attr_get(mut mrb: *mut mrb_state,
                                      mut obj: mrb_value, mut id: mrb_sym)
 -> mrb_value {
    return mrb_iv_get(mrb, obj, id);
}
#[no_mangle]
pub unsafe extern "C" fn mrb_iv_get(mut mrb: *mut mrb_state,
                                    mut obj: mrb_value, mut sym: mrb_sym)
 -> mrb_value {
    if 0 != obj_iv_p(obj) {
        return mrb_obj_iv_get(mrb, obj.value.p as *mut RObject, sym)
    }
    return mrb_nil_value();
}
unsafe extern "C" fn obj_iv_p(mut obj: mrb_value) -> mrb_bool {
    match obj.tt as libc::c_uint {
        8 | 9 | 10 | 12 | 15 | 21 | 18 => { return 1i32 as mrb_bool }
        _ => { return 0i32 as mrb_bool }
    };
}
#[no_mangle]
pub unsafe extern "C" fn mrb_class_find_path(mut mrb: *mut mrb_state,
                                             mut c: *mut RClass)
 -> mrb_value {
    let mut outer: *mut RClass = 0 as *mut RClass;
    let mut path: mrb_value =
        mrb_value{value: unnamed{f: 0.,}, tt: MRB_TT_FALSE,};
    let mut name: mrb_sym = 0;
    let mut str: *const libc::c_char = 0 as *const libc::c_char;
    let mut len: mrb_int = 0;
    if 0 != detect_outer_loop(mrb, c) { return mrb_nil_value() }
    outer = outer_class(mrb, c);
    if outer.is_null() { return mrb_nil_value() }
    name = find_class_sym(mrb, outer, c);
    if name == 0i32 as libc::c_uint { return mrb_nil_value() }
    str = mrb_class_name(mrb, outer);
    path = mrb_str_new_capa(mrb, 40i32 as size_t);
    mrb_str_cat_cstr(mrb, path, str);
    mrb_str_cat_cstr(mrb, path,
                     b"::\x00" as *const u8 as *const libc::c_char);
    str = mrb_sym2name_len(mrb, name, &mut len);
    mrb_str_cat(mrb, path, str, len as size_t);
    if *if 0 !=
               (*(path.value.p as *mut RString)).flags() as libc::c_int &
                   32i32 {
            (*(path.value.p as *mut RString)).as_0.ary.as_mut_ptr()
        } else {
            (*(path.value.p as *mut RString)).as_0.heap.ptr
        }.offset(0isize) as libc::c_int != '#' as i32 {
        iv_del(mrb, (*c).iv,
               mrb_intern_static(mrb,
                                 b"__outer__\x00" as *const u8 as
                                     *const libc::c_char,
                                 (::std::mem::size_of::<[libc::c_char; 10]>()
                                      as
                                      libc::c_ulong).wrapping_sub(1i32 as
                                                                      libc::c_ulong)),
               0 as *mut mrb_value);
        iv_put(mrb, (*c).iv,
               mrb_intern_static(mrb,
                                 b"__classname__\x00" as *const u8 as
                                     *const libc::c_char,
                                 (::std::mem::size_of::<[libc::c_char; 14]>()
                                      as
                                      libc::c_ulong).wrapping_sub(1i32 as
                                                                      libc::c_ulong)),
               path);
        if !((path.tt as libc::c_uint) <
                 MRB_TT_OBJECT as libc::c_int as libc::c_uint) {
            mrb_field_write_barrier(mrb, c as *mut RBasic,
                                    path.value.p as *mut RBasic);
        }
        path = mrb_str_dup(mrb, path)
    }
    return path;
}
/* Deletes the value for the symbol from the instance variable table. */
unsafe extern "C" fn iv_del(mut mrb: *mut mrb_state, mut t: *mut iv_tbl,
                            mut sym: mrb_sym, mut vp: *mut mrb_value)
 -> mrb_bool {
    let mut seg: *mut segment = 0 as *mut segment;
    let mut i: size_t = 0;
    if t.is_null() { return 0i32 as mrb_bool }
    seg = (*t).rootseg;
    while !seg.is_null() {
        i = 0i32 as size_t;
        while i < 4i32 as libc::c_ulong {
            let mut key: mrb_sym = (*seg).key[i as usize];
            if (*seg).next.is_null() && i >= (*t).last_len {
                return 0i32 as mrb_bool
            }
            if key == sym {
                (*t).size = (*t).size.wrapping_sub(1);
                (*seg).key[i as usize] = 0i32 as mrb_sym;
                if !vp.is_null() { *vp = (*seg).val[i as usize] }
                return 1i32 as mrb_bool
            }
            i = i.wrapping_add(1)
        }
        seg = (*seg).next
    }
    return 0i32 as mrb_bool;
}
unsafe extern "C" fn find_class_sym(mut mrb: *mut mrb_state,
                                    mut outer: *mut RClass,
                                    mut c: *mut RClass) -> mrb_sym {
    let mut arg: csym_arg = csym_arg{c: 0 as *mut RClass, sym: 0,};
    if outer.is_null() { return 0i32 as mrb_sym }
    if outer == c { return 0i32 as mrb_sym }
    arg.c = c;
    arg.sym = 0i32 as mrb_sym;
    iv_foreach(mrb, (*outer).iv, Some(csym_i),
               &mut arg as *mut csym_arg as *mut libc::c_void);
    return arg.sym;
}
unsafe extern "C" fn csym_i(mut mrb: *mut mrb_state, mut sym: mrb_sym,
                            mut v: mrb_value, mut p: *mut libc::c_void)
 -> libc::c_int {
    let mut a: *mut csym_arg = p as *mut csym_arg;
    let mut c: *mut RClass = (*a).c;
    if v.tt as libc::c_uint == (*c).tt() as libc::c_uint &&
           v.value.p as *mut RClass == c {
        (*a).sym = sym;
        return 1i32
    }
    return 0i32;
}
/* Iterates over the instance variable table. */
unsafe extern "C" fn iv_foreach(mut mrb: *mut mrb_state, mut t: *mut iv_tbl,
                                mut func:
                                    Option<unsafe extern "C" fn(_:
                                                                    *mut mrb_state,
                                                                _: mrb_sym,
                                                                _: mrb_value,
                                                                _:
                                                                    *mut libc::c_void)
                                               -> libc::c_int>,
                                mut p: *mut libc::c_void) {
    let mut seg: *mut segment = 0 as *mut segment;
    let mut i: size_t = 0;
    if t.is_null() { return }
    seg = (*t).rootseg;
    while !seg.is_null() {
        i = 0i32 as size_t;
        while i < 4i32 as libc::c_ulong {
            let mut key: mrb_sym = (*seg).key[i as usize];
            if (*seg).next.is_null() && i >= (*t).last_len { return }
            if key != 0i32 as libc::c_uint {
                if func.expect("non-null function pointer")(mrb, key,
                                                            (*seg).val[i as
                                                                           usize],
                                                            p) != 0i32 {
                    return
                }
            }
            i = i.wrapping_add(1)
        }
        seg = (*seg).next
    };
}
unsafe extern "C" fn outer_class(mut mrb: *mut mrb_state, mut c: *mut RClass)
 -> *mut RClass {
    let mut ov: mrb_value =
        mrb_value{value: unnamed{f: 0.,}, tt: MRB_TT_FALSE,};
    ov =
        mrb_obj_iv_get(mrb, c as *mut RObject,
                       mrb_intern_static(mrb,
                                         b"__outer__\x00" as *const u8 as
                                             *const libc::c_char,
                                         (::std::mem::size_of::<[libc::c_char; 10]>()
                                              as
                                              libc::c_ulong).wrapping_sub(1i32
                                                                              as
                                                                              libc::c_ulong)));
    if ov.tt as libc::c_uint == MRB_TT_FALSE as libc::c_int as libc::c_uint &&
           0 == ov.value.i {
        return 0 as *mut RClass
    }
    match ov.tt as libc::c_uint {
        9 | 10 => { return ov.value.p as *mut RClass }
        _ => { }
    }
    return 0 as *mut RClass;
}
unsafe extern "C" fn detect_outer_loop(mut mrb: *mut mrb_state,
                                       mut c: *mut RClass) -> mrb_bool {
    /* tortoise */
    let mut t: *mut RClass = c;
    /* hare */
    let mut h: *mut RClass = c;
    loop  {
        if h.is_null() { return 0i32 as mrb_bool }
        h = outer_class(mrb, h);
        if h.is_null() { return 0i32 as mrb_bool }
        h = outer_class(mrb, h);
        t = outer_class(mrb, t);
        if t == h { return 1i32 as mrb_bool }
    };
}
#[no_mangle]
pub unsafe extern "C" fn mrb_vm_special_get(mut mrb: *mut mrb_state,
                                            mut i: mrb_sym) -> mrb_value {
    return mrb_fixnum_value(0i32 as mrb_int);
}
#[no_mangle]
pub unsafe extern "C" fn mrb_vm_special_set(mut mrb: *mut mrb_state,
                                            mut i: mrb_sym,
                                            mut v: mrb_value) {
}
#[no_mangle]
pub unsafe extern "C" fn mrb_vm_cv_get(mut mrb: *mut mrb_state,
                                       mut sym: mrb_sym) -> mrb_value {
    let mut c: *mut RClass = 0 as *mut RClass;
    c =
        if (*(*(*(*mrb).c).ci).proc_0).flags() as libc::c_int & 1024i32 !=
               0i32 {
            (*(*(*(*(*mrb).c).ci).proc_0).e.env).c
        } else { (*(*(*(*mrb).c).ci).proc_0).e.target_class };
    return mrb_mod_cv_get(mrb, c, sym);
}
#[no_mangle]
pub unsafe extern "C" fn mrb_mod_cv_get(mut mrb: *mut mrb_state,
                                        mut c: *mut RClass, mut sym: mrb_sym)
 -> mrb_value {
    let mut cls: *mut RClass = c;
    let mut v: mrb_value =
        mrb_value{value: unnamed{f: 0.,}, tt: MRB_TT_FALSE,};
    let mut given: libc::c_int = 0i32;
    while !c.is_null() {
        if !(*c).iv.is_null() &&
               0 != iv_get(mrb, (*c).iv, sym, &mut v) as libc::c_int {
            given = 1i32
        }
        c = (*c).super_0
    }
    if 0 != given { return v }
    if !cls.is_null() &&
           (*cls).tt() as libc::c_int == MRB_TT_SCLASS as libc::c_int {
        let mut klass: mrb_value =
            mrb_value{value: unnamed{f: 0.,}, tt: MRB_TT_FALSE,};
        klass =
            mrb_obj_iv_get(mrb, cls as *mut RObject,
                           mrb_intern_static(mrb,
                                             b"__attached__\x00" as *const u8
                                                 as *const libc::c_char,
                                             (::std::mem::size_of::<[libc::c_char; 13]>()
                                                  as
                                                  libc::c_ulong).wrapping_sub(1i32
                                                                                  as
                                                                                  libc::c_ulong)));
        c = klass.value.p as *mut RClass;
        if (*c).tt() as libc::c_int == MRB_TT_CLASS as libc::c_int ||
               (*c).tt() as libc::c_int == MRB_TT_MODULE as libc::c_int {
            given = 0i32;
            while !c.is_null() {
                if !(*c).iv.is_null() &&
                       0 != iv_get(mrb, (*c).iv, sym, &mut v) as libc::c_int {
                    given = 1i32
                }
                c = (*c).super_0
            }
            if 0 != given { return v }
        }
    }
    mrb_name_error(mrb, sym,
                   b"uninitialized class variable %S in %S\x00" as *const u8
                       as *const libc::c_char, mrb_sym2str(mrb, sym),
                   mrb_obj_value(cls as *mut libc::c_void));
}
#[no_mangle]
pub unsafe extern "C" fn mrb_vm_cv_set(mut mrb: *mut mrb_state,
                                       mut sym: mrb_sym, mut v: mrb_value) {
    let mut c: *mut RClass = 0 as *mut RClass;
    c =
        if (*(*(*(*mrb).c).ci).proc_0).flags() as libc::c_int & 1024i32 !=
               0i32 {
            (*(*(*(*(*mrb).c).ci).proc_0).e.env).c
        } else { (*(*(*(*mrb).c).ci).proc_0).e.target_class };
    mrb_mod_cv_set(mrb, c, sym, v);
}
#[no_mangle]
pub unsafe extern "C" fn mrb_mod_cv_set(mut mrb: *mut mrb_state,
                                        mut c: *mut RClass, mut sym: mrb_sym,
                                        mut v: mrb_value) {
    let mut cls: *mut RClass = c;
    while !c.is_null() {
        let mut t: *mut iv_tbl = (*c).iv;
        if 0 != iv_get(mrb, t, sym, 0 as *mut mrb_value) {
            mrb_check_frozen(mrb, c as *mut libc::c_void);
            iv_put(mrb, t, sym, v);
            mrb_write_barrier(mrb, c as *mut RBasic);
            return
        }
        c = (*c).super_0
    }
    if !cls.is_null() &&
           (*cls).tt() as libc::c_int == MRB_TT_SCLASS as libc::c_int {
        let mut klass: mrb_value =
            mrb_value{value: unnamed{f: 0.,}, tt: MRB_TT_FALSE,};
        klass =
            mrb_obj_iv_get(mrb, cls as *mut RObject,
                           mrb_intern_static(mrb,
                                             b"__attached__\x00" as *const u8
                                                 as *const libc::c_char,
                                             (::std::mem::size_of::<[libc::c_char; 13]>()
                                                  as
                                                  libc::c_ulong).wrapping_sub(1i32
                                                                                  as
                                                                                  libc::c_ulong)));
        match klass.tt as libc::c_uint {
            9 | 10 | 12 => { c = klass.value.p as *mut RClass }
            _ => { c = cls }
        }
    } else { c = cls }
    mrb_check_frozen(mrb, c as *mut libc::c_void);
    if (*c).iv.is_null() { (*c).iv = iv_new(mrb) }
    iv_put(mrb, (*c).iv, sym, v);
    mrb_write_barrier(mrb, c as *mut RBasic);
}
#[no_mangle]
pub unsafe extern "C" fn mrb_vm_const_get(mut mrb: *mut mrb_state,
                                          mut sym: mrb_sym) -> mrb_value {
    let mut c: *mut RClass = 0 as *mut RClass;
    let mut c2: *mut RClass = 0 as *mut RClass;
    let mut v: mrb_value =
        mrb_value{value: unnamed{f: 0.,}, tt: MRB_TT_FALSE,};
    let mut proc_0: *mut RProc = 0 as *mut RProc;
    c =
        if (*(*(*(*mrb).c).ci).proc_0).flags() as libc::c_int & 1024i32 !=
               0i32 {
            (*(*(*(*(*mrb).c).ci).proc_0).e.env).c
        } else { (*(*(*(*mrb).c).ci).proc_0).e.target_class };
    if 0 != iv_get(mrb, (*c).iv, sym, &mut v) { return v }
    c2 = c;
    while !c2.is_null() &&
              (*c2).tt() as libc::c_int == MRB_TT_SCLASS as libc::c_int {
        let mut klass: mrb_value =
            mrb_value{value: unnamed{f: 0.,}, tt: MRB_TT_FALSE,};
        if 0 ==
               iv_get(mrb, (*c2).iv,
                      mrb_intern_static(mrb,
                                        b"__attached__\x00" as *const u8 as
                                            *const libc::c_char,
                                        (::std::mem::size_of::<[libc::c_char; 13]>()
                                             as
                                             libc::c_ulong).wrapping_sub(1i32
                                                                             as
                                                                             libc::c_ulong)),
                      &mut klass) {
            c2 = 0 as *mut RClass;
            break ;
        } else { c2 = klass.value.p as *mut RClass }
    }
    if !c2.is_null() &&
           ((*c2).tt() as libc::c_int == MRB_TT_CLASS as libc::c_int ||
                (*c2).tt() as libc::c_int == MRB_TT_MODULE as libc::c_int) {
        c = c2
    }
    if 0 !=
           ((*(*(*(*mrb).c).ci).proc_0).flags() as libc::c_int & 128i32 !=
                0i32) as libc::c_int as libc::c_long {
        __assert_rtn((*::std::mem::transmute::<&[u8; 17],
                                               &[libc::c_char; 17]>(b"mrb_vm_const_get\x00")).as_ptr(),
                     b"src/variable.c\x00" as *const u8 as
                         *const libc::c_char, 826i32,
                     b"!(((mrb->c->ci->proc)->flags & 128) != 0)\x00" as
                         *const u8 as *const libc::c_char);
    } else { };
    proc_0 = (*(*(*mrb).c).ci).proc_0;
    while !proc_0.is_null() {
        c2 =
            if (*proc_0).flags() as libc::c_int & 1024i32 != 0i32 {
                (*(*proc_0).e.env).c
            } else { (*proc_0).e.target_class };
        if !c2.is_null() &&
               0 != iv_get(mrb, (*c2).iv, sym, &mut v) as libc::c_int {
            return v
        }
        proc_0 = (*proc_0).upper
    }
    return const_get(mrb, c, sym);
}
unsafe extern "C" fn const_get(mut mrb: *mut mrb_state, mut base: *mut RClass,
                               mut sym: mrb_sym) -> mrb_value {
    let mut c: *mut RClass = base;
    let mut v: mrb_value =
        mrb_value{value: unnamed{f: 0.,}, tt: MRB_TT_FALSE,};
    let mut retry: mrb_bool = 0i32 as mrb_bool;
    let mut name: mrb_value =
        mrb_value{value: unnamed{f: 0.,}, tt: MRB_TT_FALSE,};
    loop  {
        while !c.is_null() {
            if !(*c).iv.is_null() {
                if 0 != iv_get(mrb, (*c).iv, sym, &mut v) { return v }
            }
            c = (*c).super_0
        }
        if !(0 == retry &&
                 (*base).tt() as libc::c_int == MRB_TT_MODULE as libc::c_int)
           {
            break ;
        }
        c = (*mrb).object_class;
        retry = 1i32 as mrb_bool
    }
    name = mrb_symbol_value(sym);
    return mrb_funcall_argv(mrb, mrb_obj_value(base as *mut libc::c_void),
                            mrb_intern_static(mrb,
                                              b"const_missing\x00" as
                                                  *const u8 as
                                                  *const libc::c_char,
                                              (::std::mem::size_of::<[libc::c_char; 14]>()
                                                   as
                                                   libc::c_ulong).wrapping_sub(1i32
                                                                                   as
                                                                                   libc::c_ulong)),
                            1i32 as mrb_int, &mut name);
}
#[no_mangle]
pub unsafe extern "C" fn mrb_vm_const_set(mut mrb: *mut mrb_state,
                                          mut sym: mrb_sym,
                                          mut v: mrb_value) {
    let mut c: *mut RClass = 0 as *mut RClass;
    c =
        if (*(*(*(*mrb).c).ci).proc_0).flags() as libc::c_int & 1024i32 !=
               0i32 {
            (*(*(*(*(*mrb).c).ci).proc_0).e.env).c
        } else { (*(*(*(*mrb).c).ci).proc_0).e.target_class };
    mrb_obj_iv_set(mrb, c as *mut RObject, sym, v);
}
#[no_mangle]
pub unsafe extern "C" fn mrb_const_get(mut mrb: *mut mrb_state,
                                       mut mod_0: mrb_value, mut sym: mrb_sym)
 -> mrb_value {
    mod_const_check(mrb, mod_0);
    return const_get(mrb, mod_0.value.p as *mut RClass, sym);
}
unsafe extern "C" fn mod_const_check(mut mrb: *mut mrb_state,
                                     mut mod_0: mrb_value) {
    match mod_0.tt as libc::c_uint {
        9 | 10 | 12 => { }
        _ => {
            mrb_raise(mrb,
                      mrb_exc_get(mrb,
                                  b"TypeError\x00" as *const u8 as
                                      *const libc::c_char),
                      b"constant look-up for non class/module\x00" as
                          *const u8 as *const libc::c_char);
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn mrb_const_set(mut mrb: *mut mrb_state,
                                       mut mod_0: mrb_value, mut sym: mrb_sym,
                                       mut v: mrb_value) {
    mod_const_check(mrb, mod_0);
    if v.tt as libc::c_uint == MRB_TT_CLASS as libc::c_int as libc::c_uint ||
           v.tt as libc::c_uint ==
               MRB_TT_MODULE as libc::c_int as libc::c_uint {
        mrb_class_name_class(mrb, mod_0.value.p as *mut RClass,
                             v.value.p as *mut RClass, sym);
    }
    mrb_iv_set(mrb, mod_0, sym, v);
}
#[no_mangle]
pub unsafe extern "C" fn mrb_iv_set(mut mrb: *mut mrb_state,
                                    mut obj: mrb_value, mut sym: mrb_sym,
                                    mut v: mrb_value) {
    if 0 != obj_iv_p(obj) {
        mrb_obj_iv_set(mrb, obj.value.p as *mut RObject, sym, v);
    } else {
        mrb_raise(mrb,
                  mrb_exc_get(mrb,
                              b"ArgumentError\x00" as *const u8 as
                                  *const libc::c_char),
                  b"cannot set instance variable\x00" as *const u8 as
                      *const libc::c_char);
    };
}
#[no_mangle]
pub unsafe extern "C" fn mrb_const_defined(mut mrb: *mut mrb_state,
                                           mut mod_0: mrb_value,
                                           mut id: mrb_sym) -> mrb_bool {
    return mrb_const_defined_0(mrb, mod_0, id, 1i32 as mrb_bool,
                               1i32 as mrb_bool);
}
unsafe extern "C" fn mrb_const_defined_0(mut mrb: *mut mrb_state,
                                         mut mod_0: mrb_value,
                                         mut id: mrb_sym,
                                         mut exclude: mrb_bool,
                                         mut recurse: mrb_bool) -> mrb_bool {
    let mut klass: *mut RClass = mod_0.value.p as *mut RClass;
    let mut tmp: *mut RClass = 0 as *mut RClass;
    let mut mod_retry: mrb_bool = 0i32 as mrb_bool;
    tmp = klass;
    loop  {
        while !tmp.is_null() {
            if 0 != iv_get(mrb, (*tmp).iv, id, 0 as *mut mrb_value) {
                return 1i32 as mrb_bool
            }
            if 0 == recurse && klass != (*mrb).object_class { break ; }
            tmp = (*tmp).super_0
        }
        if !(0 == exclude && 0 == mod_retry &&
                 (*klass).tt() as libc::c_int == MRB_TT_MODULE as libc::c_int)
           {
            break ;
        }
        mod_retry = 1i32 as mrb_bool;
        tmp = (*mrb).object_class
    }
    return 0i32 as mrb_bool;
}
#[no_mangle]
pub unsafe extern "C" fn mrb_const_remove(mut mrb: *mut mrb_state,
                                          mut mod_0: mrb_value,
                                          mut sym: mrb_sym) {
    mod_const_check(mrb, mod_0);
    mrb_iv_remove(mrb, mod_0, sym);
}
#[no_mangle]
pub unsafe extern "C" fn mrb_iv_remove(mut mrb: *mut mrb_state,
                                       mut obj: mrb_value, mut sym: mrb_sym)
 -> mrb_value {
    mrb_check_frozen(mrb, obj.value.p as *mut RObject as *mut libc::c_void);
    if 0 != obj_iv_p(obj) {
        let mut t: *mut iv_tbl = (*(obj.value.p as *mut RObject)).iv;
        let mut val: mrb_value =
            mrb_value{value: unnamed{f: 0.,}, tt: MRB_TT_FALSE,};
        if 0 != iv_del(mrb, t, sym, &mut val) { return val }
    }
    return mrb_undef_value();
}
#[no_mangle]
pub unsafe extern "C" fn mrb_iv_name_sym_p(mut mrb: *mut mrb_state,
                                           mut iv_name: mrb_sym) -> mrb_bool {
    let mut s: *const libc::c_char = 0 as *const libc::c_char;
    let mut len: mrb_int = 0;
    s = mrb_sym2name_len(mrb, iv_name, &mut len);
    if len < 2i32 as libc::c_longlong { return 0i32 as mrb_bool }
    if *s.offset(0isize) as libc::c_int != '@' as i32 {
        return 0i32 as mrb_bool
    }
    if (*s.offset(1isize) as
            libc::c_uint).wrapping_sub('0' as i32 as libc::c_uint) <
           10i32 as libc::c_uint {
        return 0i32 as mrb_bool
    }
    return mrb_ident_p(s.offset(1isize), len - 1i32 as libc::c_longlong);
}
#[no_mangle]
pub unsafe extern "C" fn mrb_ident_p(mut s: *const libc::c_char,
                                     mut len: mrb_int) -> mrb_bool {
    let mut i: mrb_int = 0;
    i = 0i32 as mrb_int;
    while i < len {
        if !((*s.offset(i as isize) as libc::c_uint |
                  0x20i32 as
                      libc::c_uint).wrapping_sub('a' as i32 as libc::c_uint) <
                 26i32 as libc::c_uint ||
                 (*s.offset(i as isize) as
                      libc::c_uint).wrapping_sub('0' as i32 as libc::c_uint) <
                     10i32 as libc::c_uint ||
                 *s.offset(i as isize) as libc::c_int == '_' as i32 ||
                 !(*s.offset(i as isize) as libc::c_uint <=
                       0x7fi32 as libc::c_uint)) {
            return 0i32 as mrb_bool
        }
        i += 1
    }
    return 1i32 as mrb_bool;
}
#[no_mangle]
pub unsafe extern "C" fn mrb_iv_name_sym_check(mut mrb: *mut mrb_state,
                                               mut iv_name: mrb_sym) {
    if 0 == mrb_iv_name_sym_p(mrb, iv_name) {
        mrb_name_error(mrb, iv_name,
                       b"\'%S\' is not allowed as an instance variable name\x00"
                           as *const u8 as *const libc::c_char,
                       mrb_sym2str(mrb, iv_name));
    };
}
#[no_mangle]
pub unsafe extern "C" fn mrb_obj_iv_defined(mut mrb: *mut mrb_state,
                                            mut obj: *mut RObject,
                                            mut sym: mrb_sym) -> mrb_bool {
    let mut t: *mut iv_tbl = 0 as *mut iv_tbl;
    t = (*obj).iv;
    if !t.is_null() { return iv_get(mrb, t, sym, 0 as *mut mrb_value) }
    return 0i32 as mrb_bool;
}
#[no_mangle]
pub unsafe extern "C" fn mrb_iv_defined(mut mrb: *mut mrb_state,
                                        mut obj: mrb_value, mut sym: mrb_sym)
 -> mrb_bool {
    if 0 == obj_iv_p(obj) { return 0i32 as mrb_bool }
    return mrb_obj_iv_defined(mrb, obj.value.p as *mut RObject, sym);
}
#[no_mangle]
pub unsafe extern "C" fn mrb_iv_copy(mut mrb: *mut mrb_state,
                                     mut dest: mrb_value,
                                     mut src: mrb_value) {
    let mut d: *mut RObject = dest.value.p as *mut RObject;
    let mut s: *mut RObject = src.value.p as *mut RObject;
    if !(*d).iv.is_null() {
        iv_free(mrb, (*d).iv);
        (*d).iv = 0 as *mut iv_tbl
    }
    if !(*s).iv.is_null() {
        mrb_write_barrier(mrb, d as *mut RBasic);
        (*d).iv = iv_copy(mrb, (*s).iv)
    };
}
/* Copy the instance variable table. */
unsafe extern "C" fn iv_copy(mut mrb: *mut mrb_state, mut t: *mut iv_tbl)
 -> *mut iv_tbl {
    let mut seg: *mut segment = 0 as *mut segment;
    let mut t2: *mut iv_tbl = 0 as *mut iv_tbl;
    let mut i: size_t = 0;
    seg = (*t).rootseg;
    t2 = iv_new(mrb);
    while !seg.is_null() {
        i = 0i32 as size_t;
        while i < 4i32 as libc::c_ulong {
            let mut key: mrb_sym = (*seg).key[i as usize];
            let mut val: mrb_value = (*seg).val[i as usize];
            if (*seg).next.is_null() && i >= (*t).last_len { return t2 }
            iv_put(mrb, t2, key, val);
            i = i.wrapping_add(1)
        }
        seg = (*seg).next
    }
    return t2;
}
/* Free memory of the instance variable table. */
unsafe extern "C" fn iv_free(mut mrb: *mut mrb_state, mut t: *mut iv_tbl) {
    let mut seg: *mut segment = 0 as *mut segment;
    seg = (*t).rootseg;
    while !seg.is_null() {
        let mut p: *mut segment = seg;
        seg = (*seg).next;
        mrb_free(mrb, p as *mut libc::c_void);
    }
    mrb_free(mrb, t as *mut libc::c_void);
}
#[no_mangle]
pub unsafe extern "C" fn mrb_const_defined_at(mut mrb: *mut mrb_state,
                                              mut mod_0: mrb_value,
                                              mut id: mrb_sym) -> mrb_bool {
    return mrb_const_defined_0(mrb, mod_0, id, 1i32 as mrb_bool,
                               0i32 as mrb_bool);
}
/* *
 * Get a global variable. Will return nil if the var does not exist
 *
 * Example:
 *
 *     !!!ruby
 *     # Ruby style
 *     var = $value
 *
 *     !!!c
 *     // C style
 *     mrb_sym sym = mrb_intern_lit(mrb, "$value");
 *     mrb_value var = mrb_gv_get(mrb, sym);
 *
 * @param mrb The mruby state reference
 * @param sym The name of the global variable
 * @return The value of that global variable. May be nil
 */
#[no_mangle]
pub unsafe extern "C" fn mrb_gv_get(mut mrb: *mut mrb_state, mut sym: mrb_sym)
 -> mrb_value {
    let mut v: mrb_value =
        mrb_value{value: unnamed{f: 0.,}, tt: MRB_TT_FALSE,};
    if 0 != iv_get(mrb, (*mrb).globals, sym, &mut v) { return v }
    return mrb_nil_value();
}
/* *
 * Set a global variable
 *
 * Example:
 *
 *     !!!ruby
 *     # Ruby style
 *     $value = "foo"
 *
 *     !!!c
 *     // C style
 *     mrb_sym sym = mrb_intern_lit(mrb, "$value");
 *     mrb_gv_set(mrb, sym, mrb_str_new_lit("foo"));
 *
 * @param mrb The mruby state reference
 * @param sym The name of the global variable
 * @param val The value of the global variable
 */
#[no_mangle]
pub unsafe extern "C" fn mrb_gv_set(mut mrb: *mut mrb_state, mut sym: mrb_sym,
                                    mut v: mrb_value) {
    let mut t: *mut iv_tbl = 0 as *mut iv_tbl;
    if (*mrb).globals.is_null() { (*mrb).globals = iv_new(mrb) }
    t = (*mrb).globals;
    iv_put(mrb, t, sym, v);
}
/* *
 * Remove a global variable.
 *
 * Example:
 *
 *     !!!ruby
 *     # Ruby style
 *     $value = nil
 *
 *     !!!c
 *     // C style
 *     mrb_sym sym = mrb_intern_lit(mrb, "$value");
 *     mrb_gv_remove(mrb, sym);
 *
 * @param mrb The mruby state reference
 * @param sym The name of the global variable
 * @param val The value of the global variable
 */
#[no_mangle]
pub unsafe extern "C" fn mrb_gv_remove(mut mrb: *mut mrb_state,
                                       mut sym: mrb_sym) {
    iv_del(mrb, (*mrb).globals, sym, 0 as *mut mrb_value);
}
#[no_mangle]
pub unsafe extern "C" fn mrb_cv_get(mut mrb: *mut mrb_state,
                                    mut mod_0: mrb_value, mut sym: mrb_sym)
 -> mrb_value {
    return mrb_mod_cv_get(mrb, mod_0.value.p as *mut RClass, sym);
}
#[no_mangle]
pub unsafe extern "C" fn mrb_cv_set(mut mrb: *mut mrb_state,
                                    mut mod_0: mrb_value, mut sym: mrb_sym,
                                    mut v: mrb_value) {
    mrb_mod_cv_set(mrb, mod_0.value.p as *mut RClass, sym, v);
}
#[no_mangle]
pub unsafe extern "C" fn mrb_cv_defined(mut mrb: *mut mrb_state,
                                        mut mod_0: mrb_value,
                                        mut sym: mrb_sym) -> mrb_bool {
    return mrb_mod_cv_defined(mrb, mod_0.value.p as *mut RClass, sym);
}
#[no_mangle]
pub unsafe extern "C" fn mrb_mod_cv_defined(mut mrb: *mut mrb_state,
                                            mut c: *mut RClass,
                                            mut sym: mrb_sym) -> mrb_bool {
    while !c.is_null() {
        let mut t: *mut iv_tbl = (*c).iv;
        if 0 != iv_get(mrb, t, sym, 0 as *mut mrb_value) {
            return 1i32 as mrb_bool
        }
        c = (*c).super_0
    }
    return 0i32 as mrb_bool;
}
#[no_mangle]
pub unsafe extern "C" fn mrb_obj_iv_inspect(mut mrb: *mut mrb_state,
                                            mut obj: *mut RObject)
 -> mrb_value {
    let mut t: *mut iv_tbl = (*obj).iv;
    let mut len: size_t = iv_size(mrb, t);
    if len > 0i32 as libc::c_ulong {
        let mut cn: *const libc::c_char =
            mrb_obj_classname(mrb, mrb_obj_value(obj as *mut libc::c_void));
        let mut str: mrb_value = mrb_str_new_capa(mrb, 30i32 as size_t);
        mrb_str_cat(mrb, str, b"-<\x00" as *const u8 as *const libc::c_char,
                    (::std::mem::size_of::<[libc::c_char; 3]>() as
                         libc::c_ulong).wrapping_sub(1i32 as libc::c_ulong));
        mrb_str_cat_cstr(mrb, str, cn);
        mrb_str_cat(mrb, str, b":\x00" as *const u8 as *const libc::c_char,
                    (::std::mem::size_of::<[libc::c_char; 2]>() as
                         libc::c_ulong).wrapping_sub(1i32 as libc::c_ulong));
        mrb_str_concat(mrb, str,
                       mrb_ptr_to_str(mrb, obj as *mut libc::c_void));
        iv_foreach(mrb, t, Some(inspect_i),
                   &mut str as *mut mrb_value as *mut libc::c_void);
        mrb_str_cat(mrb, str, b">\x00" as *const u8 as *const libc::c_char,
                    (::std::mem::size_of::<[libc::c_char; 2]>() as
                         libc::c_ulong).wrapping_sub(1i32 as libc::c_ulong));
        return str
    }
    return mrb_any_to_s(mrb, mrb_obj_value(obj as *mut libc::c_void));
}
unsafe extern "C" fn inspect_i(mut mrb: *mut mrb_state, mut sym: mrb_sym,
                               mut v: mrb_value, mut p: *mut libc::c_void)
 -> libc::c_int {
    let mut str: mrb_value = *(p as *mut mrb_value);
    let mut s: *const libc::c_char = 0 as *const libc::c_char;
    let mut len: mrb_int = 0;
    let mut ins: mrb_value =
        mrb_value{value: unnamed{f: 0.,}, tt: MRB_TT_FALSE,};
    let mut sp: *mut libc::c_char =
        if 0 !=
               (*(str.value.p as *mut RString)).flags() as libc::c_int & 32i32
           {
            (*(str.value.p as *mut RString)).as_0.ary.as_mut_ptr()
        } else { (*(str.value.p as *mut RString)).as_0.heap.ptr };
    if *sp.offset(0isize) as libc::c_int == '-' as i32 {
        *sp.offset(0isize) = '#' as i32 as libc::c_char;
        mrb_str_cat(mrb, str, b" \x00" as *const u8 as *const libc::c_char,
                    (::std::mem::size_of::<[libc::c_char; 2]>() as
                         libc::c_ulong).wrapping_sub(1i32 as libc::c_ulong));
    } else {
        mrb_str_cat(mrb, str, b", \x00" as *const u8 as *const libc::c_char,
                    (::std::mem::size_of::<[libc::c_char; 3]>() as
                         libc::c_ulong).wrapping_sub(1i32 as libc::c_ulong));
    }
    s = mrb_sym2name_len(mrb, sym, &mut len);
    mrb_str_cat(mrb, str, s, len as size_t);
    mrb_str_cat(mrb, str, b"=\x00" as *const u8 as *const libc::c_char,
                (::std::mem::size_of::<[libc::c_char; 2]>() as
                     libc::c_ulong).wrapping_sub(1i32 as libc::c_ulong));
    if v.tt as libc::c_uint == MRB_TT_OBJECT as libc::c_int as libc::c_uint {
        ins = mrb_any_to_s(mrb, v)
    } else { ins = mrb_inspect(mrb, v) }
    mrb_str_cat_str(mrb, str, ins);
    return 0i32;
}
/* Get the size of the instance variable table. */
unsafe extern "C" fn iv_size(mut mrb: *mut mrb_state, mut t: *mut iv_tbl)
 -> size_t {
    let mut seg: *mut segment = 0 as *mut segment;
    let mut size: size_t = 0i32 as size_t;
    if t.is_null() { return 0i32 as size_t }
    if (*t).size > 0i32 as libc::c_ulong { return (*t).size }
    seg = (*t).rootseg;
    while !seg.is_null() {
        if (*seg).next.is_null() {
            size =
                (size as libc::c_ulong).wrapping_add((*t).last_len) as size_t
                    as size_t;
            return size
        }
        seg = (*seg).next;
        size =
            (size as libc::c_ulong).wrapping_add(4i32 as libc::c_ulong) as
                size_t as size_t
    }
    return 0i32 as size_t;
}
#[no_mangle]
pub unsafe extern "C" fn mrb_mod_constants(mut mrb: *mut mrb_state,
                                           mut mod_0: mrb_value)
 -> mrb_value {
    let mut ary: mrb_value =
        mrb_value{value: unnamed{f: 0.,}, tt: MRB_TT_FALSE,};
    let mut inherit: mrb_bool = 1i32 as mrb_bool;
    let mut c: *mut RClass = mod_0.value.p as *mut RClass;
    mrb_get_args(mrb, b"|b\x00" as *const u8 as *const libc::c_char,
                 &mut inherit as *mut mrb_bool);
    ary = mrb_ary_new(mrb);
    while !c.is_null() {
        iv_foreach(mrb, (*c).iv, Some(const_i),
                   &mut ary as *mut mrb_value as *mut libc::c_void);
        if 0 == inherit { break ; }
        c = (*c).super_0;
        if c == (*mrb).object_class { break ; }
    }
    return ary;
}
unsafe extern "C" fn const_i(mut mrb: *mut mrb_state, mut sym: mrb_sym,
                             mut v: mrb_value, mut p: *mut libc::c_void)
 -> libc::c_int {
    let mut ary: mrb_value =
        mrb_value{value: unnamed{f: 0.,}, tt: MRB_TT_FALSE,};
    let mut s: *const libc::c_char = 0 as *const libc::c_char;
    let mut len: mrb_int = 0;
    ary = *(p as *mut mrb_value);
    s = mrb_sym2name_len(mrb, sym, &mut len);
    if len >= 1i32 as libc::c_longlong &&
           (*s.offset(0isize) as
                libc::c_uint).wrapping_sub('A' as i32 as libc::c_uint) <
               26i32 as libc::c_uint {
        mrb_ary_push(mrb, ary, mrb_symbol_value(sym));
    }
    return 0i32;
}
#[no_mangle]
pub unsafe extern "C" fn mrb_f_global_variables(mut mrb: *mut mrb_state,
                                                mut self_0: mrb_value)
 -> mrb_value {
    let mut t: *mut iv_tbl = (*mrb).globals;
    let mut ary: mrb_value = mrb_ary_new(mrb);
    iv_foreach(mrb, t, Some(gv_i),
               &mut ary as *mut mrb_value as *mut libc::c_void);
    return ary;
}
unsafe extern "C" fn gv_i(mut mrb: *mut mrb_state, mut sym: mrb_sym,
                          mut v: mrb_value, mut p: *mut libc::c_void)
 -> libc::c_int {
    let mut ary: mrb_value =
        mrb_value{value: unnamed{f: 0.,}, tt: MRB_TT_FALSE,};
    ary = *(p as *mut mrb_value);
    mrb_ary_push(mrb, ary, mrb_symbol_value(sym));
    return 0i32;
}
#[no_mangle]
pub unsafe extern "C" fn mrb_obj_instance_variables(mut mrb: *mut mrb_state,
                                                    mut self_0: mrb_value)
 -> mrb_value {
    let mut ary: mrb_value =
        mrb_value{value: unnamed{f: 0.,}, tt: MRB_TT_FALSE,};
    ary = mrb_ary_new(mrb);
    if 0 != obj_iv_p(self_0) {
        iv_foreach(mrb, (*(self_0.value.p as *mut RObject)).iv, Some(iv_i),
                   &mut ary as *mut mrb_value as *mut libc::c_void);
    }
    return ary;
}
unsafe extern "C" fn iv_i(mut mrb: *mut mrb_state, mut sym: mrb_sym,
                          mut v: mrb_value, mut p: *mut libc::c_void)
 -> libc::c_int {
    let mut ary: mrb_value =
        mrb_value{value: unnamed{f: 0.,}, tt: MRB_TT_FALSE,};
    let mut s: *const libc::c_char = 0 as *const libc::c_char;
    let mut len: mrb_int = 0;
    ary = *(p as *mut mrb_value);
    s = mrb_sym2name_len(mrb, sym, &mut len);
    if len > 1i32 as libc::c_longlong &&
           *s.offset(0isize) as libc::c_int == '@' as i32 &&
           *s.offset(1isize) as libc::c_int != '@' as i32 {
        mrb_ary_push(mrb, ary, mrb_symbol_value(sym));
    }
    return 0i32;
}
#[no_mangle]
pub unsafe extern "C" fn mrb_mod_class_variables(mut mrb: *mut mrb_state,
                                                 mut mod_0: mrb_value)
 -> mrb_value {
    let mut ary: mrb_value =
        mrb_value{value: unnamed{f: 0.,}, tt: MRB_TT_FALSE,};
    let mut c: *mut RClass = 0 as *mut RClass;
    ary = mrb_ary_new(mrb);
    c = mod_0.value.p as *mut RClass;
    while !c.is_null() {
        iv_foreach(mrb, (*c).iv, Some(cv_i),
                   &mut ary as *mut mrb_value as *mut libc::c_void);
        c = (*c).super_0
    }
    return ary;
}
unsafe extern "C" fn cv_i(mut mrb: *mut mrb_state, mut sym: mrb_sym,
                          mut v: mrb_value, mut p: *mut libc::c_void)
 -> libc::c_int {
    let mut ary: mrb_value =
        mrb_value{value: unnamed{f: 0.,}, tt: MRB_TT_FALSE,};
    let mut s: *const libc::c_char = 0 as *const libc::c_char;
    let mut len: mrb_int = 0;
    ary = *(p as *mut mrb_value);
    s = mrb_sym2name_len(mrb, sym, &mut len);
    if len > 2i32 as libc::c_longlong &&
           *s.offset(0isize) as libc::c_int == '@' as i32 &&
           *s.offset(1isize) as libc::c_int == '@' as i32 {
        mrb_ary_push(mrb, ary, mrb_symbol_value(sym));
    }
    return 0i32;
}
/* GC functions */
#[no_mangle]
pub unsafe extern "C" fn mrb_gc_mark_gv(mut mrb: *mut mrb_state) {
    mark_tbl(mrb, (*mrb).globals);
}
unsafe extern "C" fn mark_tbl(mut mrb: *mut mrb_state, mut t: *mut iv_tbl) {
    iv_foreach(mrb, t, Some(iv_mark_i), 0 as *mut libc::c_void);
}
unsafe extern "C" fn iv_mark_i(mut mrb: *mut mrb_state, mut sym: mrb_sym,
                               mut v: mrb_value, mut p: *mut libc::c_void)
 -> libc::c_int {
    if !((v.tt as libc::c_uint) <
             MRB_TT_OBJECT as libc::c_int as libc::c_uint) {
        mrb_gc_mark(mrb, v.value.p as *mut RBasic);
    }
    return 0i32;
}
#[no_mangle]
pub unsafe extern "C" fn mrb_gc_free_gv(mut mrb: *mut mrb_state) {
    if !(*mrb).globals.is_null() { iv_free(mrb, (*mrb).globals); };
}
#[no_mangle]
pub unsafe extern "C" fn mrb_gc_mark_iv(mut mrb: *mut mrb_state,
                                        mut obj: *mut RObject) {
    mark_tbl(mrb, (*obj).iv);
}
#[no_mangle]
pub unsafe extern "C" fn mrb_gc_mark_iv_size(mut mrb: *mut mrb_state,
                                             mut obj: *mut RObject)
 -> size_t {
    return iv_size(mrb, (*obj).iv);
}
#[no_mangle]
pub unsafe extern "C" fn mrb_gc_free_iv(mut mrb: *mut mrb_state,
                                        mut obj: *mut RObject) {
    if !(*obj).iv.is_null() { iv_free(mrb, (*obj).iv); };
}
#[no_mangle]
pub unsafe extern "C" fn mrb_iv_foreach(mut mrb: *mut mrb_state,
                                        mut obj: mrb_value,
                                        mut func:
                                            Option<unsafe extern "C" fn(_:
                                                                            *mut mrb_state,
                                                                        _:
                                                                            mrb_sym,
                                                                        _:
                                                                            mrb_value,
                                                                        _:
                                                                            *mut libc::c_void)
                                                       -> libc::c_int>,
                                        mut p: *mut libc::c_void) {
    if 0 == obj_iv_p(obj) { return }
    iv_foreach(mrb, (*(obj.value.p as *mut RObject)).iv, func, p);
}