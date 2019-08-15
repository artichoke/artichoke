use libc;
use c2rust_bitfields::BitfieldStruct;
extern "C" {
    pub type iv_tbl;
    /* debug info */
    pub type mrb_irep_debug_info;
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
    #[no_mangle]
    fn mrb_singleton_class(_: *mut mrb_state, _: mrb_value) -> mrb_value;
    /* *
 * Defines a global function in ruby.
 *
 * If you're creating a gem it may look something like this
 *
 * Example:
 *
 *     !!!c
 *     mrb_value example_method(mrb_state* mrb, mrb_value self)
 *     {
 *          puts("Executing example command!");
 *          return self;
 *     }
 *
 *     void mrb_example_gem_init(mrb_state* mrb)
 *     {
 *           mrb_define_method(mrb, mrb->kernel_module, "example_method", example_method, MRB_ARGS_NONE());
 *     }
 *
 * @param [mrb_state *] mrb The MRuby state reference.
 * @param [struct RClass *] cla The class pointer where the method will be defined.
 * @param [const char *] name The name of the method being defined.
 * @param [mrb_func_t] func The function pointer to the method definition.
 * @param [mrb_aspec] aspec The method parameters declaration.
 */
    #[no_mangle]
    fn mrb_define_method(mrb: *mut mrb_state, cla: *mut RClass,
                         name: *const libc::c_char, func: mrb_func_t,
                         aspec: mrb_aspec);
    /* *
 *  Defines a module function.
 *
 * Example:
 *
 *        # Ruby style
 *        module Foo
 *          def Foo.bar
 *          end
 *        end
 *        // C style
 *        mrb_value bar_method(mrb_state* mrb, mrb_value self){
 *          return mrb_nil_value();
 *        }
 *        void mrb_example_gem_init(mrb_state* mrb){
 *          struct RClass *foo;
 *          foo = mrb_define_module(mrb, "Foo");
 *          mrb_define_module_function(mrb, foo, "bar", bar_method, MRB_ARGS_NONE());
 *        }
 *  @param [mrb_state *] mrb_state* The MRuby state reference.
 *  @param [struct RClass *] RClass* The module where the module function will be defined.
 *  @param [const char *] char* The name of the module function being defined.
 *  @param [mrb_func_t] mrb_func_t The function pointer to the module function definition.
 *  @param [mrb_aspec] mrb_aspec The method parameters declaration.
 */
    #[no_mangle]
    fn mrb_define_module_function(_: *mut mrb_state, _: *mut RClass,
                                  _: *const libc::c_char, _: mrb_func_t,
                                  _: mrb_aspec);
    /* *
 * Gets a class.
 * @param [mrb_state*] mrb The current mruby state.
 * @param [const char *] name The name of the class.
 * @return [struct RClass *] A reference to the class.
*/
    #[no_mangle]
    fn mrb_class_get(mrb: *mut mrb_state, name: *const libc::c_char)
     -> *mut RClass;
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
    #[no_mangle]
    fn mrb_obj_alloc(_: *mut mrb_state, _: mrb_vtype, _: *mut RClass)
     -> *mut RBasic;
    #[no_mangle]
    fn mrb_top_run(_: *mut mrb_state, _: *mut RProc, _: mrb_value,
                   _: libc::c_uint) -> mrb_value;
    #[no_mangle]
    fn mrb_field_write_barrier(_: *mut mrb_state, _: *mut RBasic,
                               _: *mut RBasic);
    #[no_mangle]
    fn mrb_exc_raise(mrb: *mut mrb_state, exc: mrb_value);
    #[no_mangle]
    fn mrb_raise(mrb: *mut mrb_state, c: *mut RClass,
                 msg: *const libc::c_char);
    #[no_mangle]
    fn mrb_format(mrb: *mut mrb_state, format: *const libc::c_char, _: ...)
     -> mrb_value;
    #[no_mangle]
    fn mrbc_context_new(mrb: *mut mrb_state) -> *mut mrbc_context;
    #[no_mangle]
    fn mrbc_context_free(mrb: *mut mrb_state, cxt: *mut mrbc_context);
    #[no_mangle]
    fn mrbc_filename(mrb: *mut mrb_state, c: *mut mrbc_context,
                     s: *const libc::c_char) -> *const libc::c_char;
    #[no_mangle]
    fn mrb_parser_free(_: *mut mrb_parser_state);
    #[no_mangle]
    fn mrb_parse_nstring(_: *mut mrb_state, _: *const libc::c_char, _: size_t,
                         _: *mut mrbc_context) -> *mut mrb_parser_state;
    #[no_mangle]
    fn mrb_generate_code(_: *mut mrb_state, _: *mut mrb_parser_state)
     -> *mut RProc;
    #[no_mangle]
    fn mrb_exc_new_str(mrb: *mut mrb_state, c: *mut RClass, str: mrb_value)
     -> mrb_value;
    #[no_mangle]
    fn mrb_exec_irep(mrb: *mut mrb_state, self_0: mrb_value, p: *mut RProc)
     -> mrb_value;
    #[no_mangle]
    fn mrb_obj_instance_eval(mrb: *mut mrb_state, self_0: mrb_value)
     -> mrb_value;
    #[no_mangle]
    static mut mrb_insn_size: [uint8_t; 0];
    #[no_mangle]
    static mut mrb_insn_size1: [uint8_t; 0];
    #[no_mangle]
    static mut mrb_insn_size2: [uint8_t; 0];
    #[no_mangle]
    static mut mrb_insn_size3: [uint8_t; 0];
}
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
/* *
 * Required arguments signature type.
 */
pub type mrb_aspec = uint32_t;
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
 * | `s`  | {String}       | char *, {mrb_int} | Receive two arguments; `s!` gives (`NULL`,`0`) for `nil`        |
 * | `z`  | {String}       | char *            | `NULL` terminated string; `z!` gives `NULL` for `nil`           |
 * | `a`  | {Array}        | {mrb_value} *, {mrb_int} | Receive two arguments; `a!` gives (`NULL`,`0`) for `nil` |
 * | `f`  | {Fixnum}/{Float} | {mrb_float}       |                                                    |
 * | `i`  | {Fixnum}/{Float} | {mrb_int}         |                                                    |
 * | `b`  | boolean        | {mrb_bool}        |                                                    |
 * | `n`  | {String}/{Symbol} | {mrb_sym}         |                                                    |
 * | `d`  | data           | void *, {mrb_data_type} const | 2nd argument will be used to check data type so it won't be modified; when `!` follows, the value may be `nil` |
 * | `I`  | inline struct  | void *          |                                                    |
 * | `&`  | block          | {mrb_value}       | &! raises exception if no block given.             |
 * | `*`  | rest arguments | {mrb_value} *, {mrb_int} | Receive the rest of arguments as an array; `*!` avoid copy of the stack.  |
 * | &vert; | optional     |                   | After this spec following specs would be optional. |
 * | `?`  | optional given | {mrb_bool}        | `TRUE` if preceding argument is given. Used to check optional argument is given. |
 *
 * @see mrb_get_args
 */
pub type mrb_args_format = *const libc::c_char;
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
unsafe extern "C" fn get_closure_irep(mut mrb: *mut mrb_state,
                                      mut level: libc::c_int)
 -> *mut mrb_irep {
    let mut proc_0: *mut RProc =
        (*(*(*mrb).c).ci.offset(-1i32 as isize)).proc_0;
    loop  {
        let fresh0 = level;
        level = level - 1;
        if !(0 != fresh0) { break ; }
        if proc_0.is_null() { return 0 as *mut mrb_irep }
        proc_0 = (*proc_0).upper
    }
    if proc_0.is_null() { return 0 as *mut mrb_irep }
    if (*proc_0).flags() as libc::c_int & 128i32 != 0i32 {
        return 0 as *mut mrb_irep
    }
    return (*proc_0).body.irep;
}
/* search for irep lev above the bottom */
unsafe extern "C" fn search_irep(mut top: *mut mrb_irep,
                                 mut bnest: libc::c_int, mut lev: libc::c_int,
                                 mut bottom: *mut mrb_irep) -> *mut mrb_irep {
    let mut i: libc::c_int = 0;
    i = 0i32;
    while i < (*top).rlen as libc::c_int {
        let mut tmp: *mut mrb_irep = *(*top).reps.offset(i as isize);
        if tmp == bottom { return top }
        tmp = search_irep(tmp, bnest - 1i32, lev, bottom);
        if !tmp.is_null() { if bnest == lev { return top } return tmp }
        i += 1
    }
    return 0 as *mut mrb_irep;
}
unsafe extern "C" fn search_variable(mut mrb: *mut mrb_state,
                                     mut vsym: mrb_sym,
                                     mut bnest: libc::c_int) -> uint16_t {
    let mut virep: *mut mrb_irep = 0 as *mut mrb_irep;
    let mut level: libc::c_int = 0;
    let mut pos: libc::c_int = 0;
    level = 0i32;
    loop  {
        virep = get_closure_irep(mrb, level);
        if virep.is_null() { break ; }
        if !(*virep).lv.is_null() {
            pos = 0i32;
            while pos < (*virep).nlocals as libc::c_int - 1i32 {
                if vsym == (*(*virep).lv.offset(pos as isize)).name {
                    return (pos + 1i32 << 8i32 | level + bnest) as uint16_t
                }
                pos += 1
            }
        }
        level += 1
    }
    return 0i32 as uint16_t;
}
unsafe extern "C" fn irep_argc(mut irep: *mut mrb_irep) -> libc::c_int {
    let mut c: mrb_code = 0;
    c = *(*irep).iseq.offset(0isize);
    if c as libc::c_int == OP_ENTER as libc::c_int {
        let mut ax: mrb_aspec =
            ((*(*irep).iseq.offset(1isize).offset(0isize) as libc::c_int) <<
                 16i32 |
                 (*(*irep).iseq.offset(1isize).offset(1isize) as libc::c_int)
                     << 8i32 |
                 *(*irep).iseq.offset(1isize).offset(2isize) as libc::c_int)
                as mrb_aspec;
        /* extra 1 means a slot for block */
        return (ax >> 18i32 &
                    0x1fi32 as
                        libc::c_uint).wrapping_add(ax >> 13i32 &
                                                       0x1fi32 as
                                                           libc::c_uint).wrapping_add(ax
                                                                                          >>
                                                                                          12i32
                                                                                          &
                                                                                          0x1i32
                                                                                              as
                                                                                              libc::c_uint).wrapping_add(ax
                                                                                                                             >>
                                                                                                                             7i32
                                                                                                                             &
                                                                                                                             0x1fi32
                                                                                                                                 as
                                                                                                                                 libc::c_uint).wrapping_add(1i32
                                                                                                                                                                as
                                                                                                                                                                libc::c_uint)
                   as libc::c_int
    }
    return 0i32;
}
unsafe extern "C" fn potential_upvar_p(mut lv: *mut mrb_locals,
                                       mut v: uint16_t, mut argc: libc::c_int,
                                       mut nlocals: uint16_t) -> mrb_bool {
    if v as libc::c_int >= nlocals as libc::c_int { return 0i32 as mrb_bool }
    /* skip arguments  */
    if (v as libc::c_int) < argc + 1i32 { return 0i32 as mrb_bool }
    return 1i32 as mrb_bool;
}
unsafe extern "C" fn patch_irep(mut mrb: *mut mrb_state,
                                mut irep: *mut mrb_irep,
                                mut bnest: libc::c_int,
                                mut top: *mut mrb_irep) {
    let mut i: libc::c_int = 0;
    let mut a: uint32_t = 0;
    let mut b: uint16_t = 0;
    let mut c: uint8_t = 0;
    let mut insn: mrb_code = 0;
    let mut argc: libc::c_int = irep_argc(irep);
    i = 0i32;
    while i < (*irep).ilen as libc::c_int {
        insn = *(*irep).iseq.offset(i as isize);
        match insn as libc::c_int {
            42 => {
                b =
                    ((*(*irep).iseq.offset(i as
                                               isize).offset(1isize).offset(0isize)
                          as libc::c_int) << 8i32 |
                         *(*irep).iseq.offset(i as
                                                  isize).offset(1isize).offset(1isize)
                             as libc::c_int) as uint16_t;
                patch_irep(mrb, *(*irep).reps.offset(b as isize),
                           bnest + 1i32, top);
            }
            84 | 85 => {
                a =
                    *(*irep).iseq.offset(i as isize).offset(1isize) as
                        uint32_t;
                b =
                    *(*irep).iseq.offset(i as isize).offset(2isize) as
                        uint16_t;
                patch_irep(mrb, *(*irep).reps.offset(b as isize),
                           bnest + 1i32, top);
            }
            46 => {
                b =
                    *(*irep).iseq.offset(i as isize).offset(2isize) as
                        uint16_t;
                c = *(*irep).iseq.offset(i as isize).offset(3isize);
                if !(c as libc::c_int != 0i32) {
                    let mut arg: uint16_t =
                        search_variable(mrb, *(*irep).syms.offset(b as isize),
                                        bnest);
                    if arg as libc::c_int != 0i32 {
                        /* must replace */
                        *(*irep).iseq.offset(i as isize) =
                            OP_GETUPVAR as libc::c_int as mrb_code;
                        *(*irep).iseq.offset((i + 2i32) as isize) =
                            (arg as libc::c_int >> 8i32) as mrb_code;
                        *(*irep).iseq.offset((i + 3i32) as isize) =
                            (arg as libc::c_int & 0xffi32) as mrb_code
                    }
                }
            }
            1 => {
                a =
                    *(*irep).iseq.offset(i as isize).offset(1isize) as
                        uint32_t;
                b =
                    *(*irep).iseq.offset(i as isize).offset(2isize) as
                        uint16_t;
                /* src part */
                if 0 !=
                       potential_upvar_p((*irep).lv, b, argc, (*irep).nlocals)
                   {
                    let mut arg_0: uint16_t =
                        search_variable(mrb,
                                        (*(*irep).lv.offset((b as libc::c_int
                                                                 - 1i32) as
                                                                isize)).name,
                                        bnest);
                    if arg_0 as libc::c_int != 0i32 {
                        /* must replace */
                        insn = OP_GETUPVAR as libc::c_int as mrb_code;
                        *(*irep).iseq.offset(i as isize) = insn;
                        *(*irep).iseq.offset((i + 2i32) as isize) =
                            (arg_0 as libc::c_int >> 8i32) as mrb_code;
                        *(*irep).iseq.offset((i + 3i32) as isize) =
                            (arg_0 as libc::c_int & 0xffi32) as mrb_code
                    }
                }
                /* dst part */
                if 0 !=
                       potential_upvar_p((*irep).lv, a as uint16_t, argc,
                                         (*irep).nlocals) {
                    let mut arg_1: uint16_t =
                        search_variable(mrb,
                                        (*(*irep).lv.offset(a.wrapping_sub(1i32
                                                                               as
                                                                               libc::c_uint)
                                                                as
                                                                isize)).name,
                                        bnest);
                    if arg_1 as libc::c_int != 0i32 {
                        /* must replace */
                        insn = OP_SETUPVAR as libc::c_int as mrb_code;
                        *(*irep).iseq.offset(i as isize) = insn;
                        *(*irep).iseq.offset((i + 1i32) as isize) =
                            b as mrb_code;
                        *(*irep).iseq.offset((i + 2i32) as isize) =
                            (arg_1 as libc::c_int >> 8i32) as mrb_code;
                        *(*irep).iseq.offset((i + 3i32) as isize) =
                            (arg_1 as libc::c_int & 0xffi32) as mrb_code
                    }
                }
            }
            31 => {
                a =
                    *(*irep).iseq.offset(i as isize).offset(1isize) as
                        uint32_t;
                b =
                    *(*irep).iseq.offset(i as isize).offset(2isize) as
                        uint16_t;
                c = *(*irep).iseq.offset(i as isize).offset(3isize);
                let mut lev: libc::c_int = c as libc::c_int + 1i32;
                let mut tmp: *mut mrb_irep =
                    search_irep(top, bnest, lev, irep);
                if 0 !=
                       potential_upvar_p((*tmp).lv, b, irep_argc(tmp),
                                         (*tmp).nlocals) {
                    let mut arg_2: uint16_t =
                        search_variable(mrb,
                                        (*(*tmp).lv.offset((b as libc::c_int -
                                                                1i32) as
                                                               isize)).name,
                                        bnest);
                    if arg_2 as libc::c_int != 0i32 {
                        /* must replace */
                        *(*irep).iseq.offset(i as isize) =
                            OP_GETUPVAR as libc::c_int as mrb_code;
                        *(*irep).iseq.offset((i + 2i32) as isize) =
                            (arg_2 as libc::c_int >> 8i32) as mrb_code;
                        *(*irep).iseq.offset((i + 3i32) as isize) =
                            (arg_2 as libc::c_int & 0xffi32) as mrb_code
                    }
                }
            }
            32 => {
                a =
                    *(*irep).iseq.offset(i as isize).offset(1isize) as
                        uint32_t;
                b =
                    *(*irep).iseq.offset(i as isize).offset(2isize) as
                        uint16_t;
                c = *(*irep).iseq.offset(i as isize).offset(3isize);
                let mut lev_0: libc::c_int = c as libc::c_int + 1i32;
                let mut tmp_0: *mut mrb_irep =
                    search_irep(top, bnest, lev_0, irep);
                if 0 !=
                       potential_upvar_p((*tmp_0).lv, b, irep_argc(tmp_0),
                                         (*tmp_0).nlocals) {
                    let mut arg_3: uint16_t =
                        search_variable(mrb,
                                        (*(*tmp_0).lv.offset((b as libc::c_int
                                                                  - 1i32) as
                                                                 isize)).name,
                                        bnest);
                    if arg_3 as libc::c_int != 0i32 {
                        /* must replace */
                        *(*irep).iseq.offset(i as isize) =
                            OP_SETUPVAR as libc::c_int as mrb_code;
                        *(*irep).iseq.offset((i + 1i32) as isize) =
                            a as mrb_code;
                        *(*irep).iseq.offset((i + 2i32) as isize) =
                            (arg_3 as libc::c_int >> 8i32) as mrb_code;
                        *(*irep).iseq.offset((i + 3i32) as isize) =
                            (arg_3 as libc::c_int & 0xffi32) as mrb_code
                    }
                }
            }
            100 => {
                insn = *(*irep).iseq.offset(i as isize).offset(1isize);
                i +=
                    *mrb_insn_size1.as_mut_ptr().offset(insn as isize) as
                        libc::c_int + 1i32;
                continue ;
            }
            101 => {
                insn = *(*irep).iseq.offset(i as isize).offset(1isize);
                i +=
                    *mrb_insn_size2.as_mut_ptr().offset(insn as isize) as
                        libc::c_int + 1i32;
                continue ;
            }
            102 => {
                insn = *(*irep).iseq.offset(i as isize).offset(1isize);
                i +=
                    *mrb_insn_size3.as_mut_ptr().offset(insn as isize) as
                        libc::c_int + 1i32;
                continue ;
            }
            _ => { }
        }
        i += *mrb_insn_size.as_mut_ptr().offset(insn as isize) as libc::c_int
    };
}
unsafe extern "C" fn create_proc_from_string(mut mrb: *mut mrb_state,
                                             mut s: *mut libc::c_char,
                                             mut len: mrb_int,
                                             mut binding: mrb_value,
                                             mut file: *const libc::c_char,
                                             mut line: mrb_int)
 -> *mut RProc {
    let mut cxt: *mut mrbc_context = 0 as *mut mrbc_context;
    let mut p: *mut mrb_parser_state = 0 as *mut mrb_parser_state;
    let mut proc_0: *mut RProc = 0 as *mut RProc;
    let mut e: *mut REnv = 0 as *mut REnv;
    /* callinfo of eval caller */
    let mut ci: *mut mrb_callinfo = 0 as *mut mrb_callinfo;
    let mut target_class: *mut RClass = 0 as *mut RClass;
    let mut bidx: libc::c_int = 0;
    if !(binding.tt as libc::c_uint ==
             MRB_TT_FALSE as libc::c_int as libc::c_uint &&
             0 == binding.value.i) {
        mrb_raise(mrb,
                  mrb_exc_get(mrb,
                              b"ArgumentError\x00" as *const u8 as
                                  *const libc::c_char),
                  b"Binding of eval must be nil.\x00" as *const u8 as
                      *const libc::c_char);
    }
    cxt = mrbc_context_new(mrb);
    (*cxt).lineno = line as uint16_t;
    mrbc_filename(mrb, cxt,
                  if !file.is_null() {
                      file
                  } else {
                      b"(eval)\x00" as *const u8 as *const libc::c_char
                  });
    (*cxt).set_capture_errors(1i32 as mrb_bool);
    (*cxt).set_no_optimize(1i32 as mrb_bool);
    (*cxt).set_on_eval(1i32 as mrb_bool);
    p = mrb_parse_nstring(mrb, s, len as size_t, cxt);
    /* only occur when memory ran out */
    if p.is_null() {
        mrb_raise(mrb,
                  mrb_exc_get(mrb,
                              b"RuntimeError\x00" as *const u8 as
                                  *const libc::c_char),
                  b"Failed to create parser state.\x00" as *const u8 as
                      *const libc::c_char);
    }
    if (0i32 as libc::c_ulong) < (*p).nerr {
        /* parse error */
        let mut str: mrb_value =
            mrb_value{value: C2RustUnnamed_0{f: 0.,}, tt: MRB_TT_FALSE,};
        if !file.is_null() {
            str =
                mrb_format(mrb,
                           b"file %s line %d: %s\x00" as *const u8 as
                               *const libc::c_char, file,
                           (*p).error_buffer[0usize].lineno as libc::c_int,
                           (*p).error_buffer[0usize].message)
        } else {
            str =
                mrb_format(mrb,
                           b"line %d: %s\x00" as *const u8 as
                               *const libc::c_char,
                           (*p).error_buffer[0usize].lineno as libc::c_int,
                           (*p).error_buffer[0usize].message)
        }
        mrb_parser_free(p);
        mrbc_context_free(mrb, cxt);
        mrb_exc_raise(mrb,
                      mrb_exc_new_str(mrb,
                                      mrb_exc_get(mrb,
                                                  b"SyntaxError\x00" as
                                                      *const u8 as
                                                      *const libc::c_char),
                                      str));
    }
    proc_0 = mrb_generate_code(mrb, p);
    if proc_0.is_null() {
        /* codegen error */
        mrb_parser_free(p);
        mrbc_context_free(mrb, cxt);
        mrb_raise(mrb,
                  mrb_exc_get(mrb,
                              b"ScriptError\x00" as *const u8 as
                                  *const libc::c_char),
                  b"codegen error\x00" as *const u8 as *const libc::c_char);
    }
    if (*(*mrb).c).ci > (*(*mrb).c).cibase {
        ci = &mut *(*(*mrb).c).ci.offset(-1i32 as isize) as *mut mrb_callinfo
    } else { ci = (*(*mrb).c).cibase }
    if !(*ci).proc_0.is_null() {
        target_class =
            if (*(*ci).proc_0).flags() as libc::c_int & 1024i32 != 0i32 {
                (*(*(*ci).proc_0).e.env).c
            } else { (*(*ci).proc_0).e.target_class }
    }
    if !(*ci).proc_0.is_null() &&
           !((*(*ci).proc_0).flags() as libc::c_int & 128i32 != 0i32) {
        if !(*ci).env.is_null() {
            e = (*ci).env
        } else {
            e = mrb_obj_alloc(mrb, MRB_TT_ENV, target_class) as *mut REnv;
            (*e).mid = (*ci).mid;
            (*e).stack = (*ci.offset(1isize)).stackent;
            (*e).cxt = (*mrb).c;
            (*e).set_flags(((*e).flags() as libc::c_int & !0x3ffi32) as
                               libc::c_uint |
                               (*(*(*ci).proc_0).body.irep).nlocals as
                                   libc::c_uint & 0x3ffi32 as libc::c_uint);
            bidx = (*ci).argc;
            if (*ci).argc < 0i32 { bidx = 2i32 } else { bidx += 1i32 }
            (*e).set_flags(((*e).flags() as libc::c_int &
                                !(0x3ffi32 << 10i32)) as libc::c_uint |
                               (bidx as libc::c_uint &
                                    0x3ffi32 as libc::c_uint) << 10i32);
            (*ci).env = e
        }
        (*proc_0).e.env = e;
        (*proc_0).set_flags((*proc_0).flags() | 1024i32 as uint32_t);
        mrb_field_write_barrier(mrb, proc_0 as *mut RBasic, e as *mut RBasic);
    }
    (*proc_0).upper = (*ci).proc_0;
    (*(*(*mrb).c).ci).target_class = target_class;
    patch_irep(mrb, (*proc_0).body.irep, 0i32, (*proc_0).body.irep);
    /* mrb_codedump_all(mrb, proc); */
    mrb_parser_free(p);
    mrbc_context_free(mrb, cxt);
    return proc_0;
}
unsafe extern "C" fn exec_irep(mut mrb: *mut mrb_state, mut self_0: mrb_value,
                               mut proc_0: *mut RProc) -> mrb_value {
    /* no argument passed from eval() */
    (*(*(*mrb).c).ci).argc = 0i32;
    if (*(*(*mrb).c).ci).acc < 0i32 {
        let mut cioff: ptrdiff_t =
            (*(*mrb).c).ci.wrapping_offset_from((*(*mrb).c).cibase) as
                libc::c_long;
        let mut ret: mrb_value =
            mrb_top_run(mrb, proc_0, self_0, 0i32 as libc::c_uint);
        if !(*mrb).exc.is_null() {
            mrb_exc_raise(mrb,
                          mrb_obj_value((*mrb).exc as *mut libc::c_void));
        }
        (*(*mrb).c).ci = (*(*mrb).c).cibase.offset(cioff as isize);
        return ret
    }
    /* clear block */
    *(*(*mrb).c).stack.offset(1isize) = mrb_nil_value();
    return mrb_exec_irep(mrb, self_0, proc_0);
}
unsafe extern "C" fn f_eval(mut mrb: *mut mrb_state, mut self_0: mrb_value)
 -> mrb_value {
    let mut s: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut len: mrb_int = 0;
    let mut binding: mrb_value = mrb_nil_value();
    let mut file: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut line: mrb_int = 1i32 as mrb_int;
    let mut proc_0: *mut RProc = 0 as *mut RProc;
    mrb_get_args(mrb, b"s|ozi\x00" as *const u8 as *const libc::c_char,
                 &mut s as *mut *mut libc::c_char, &mut len as *mut mrb_int,
                 &mut binding as *mut mrb_value,
                 &mut file as *mut *mut libc::c_char,
                 &mut line as *mut mrb_int);
    proc_0 = create_proc_from_string(mrb, s, len, binding, file, line);
    return exec_irep(mrb, self_0, proc_0);
}
unsafe extern "C" fn f_instance_eval(mut mrb: *mut mrb_state,
                                     mut self_0: mrb_value) -> mrb_value {
    let mut b: mrb_value =
        mrb_value{value: C2RustUnnamed_0{f: 0.,}, tt: MRB_TT_FALSE,};
    let mut argc: mrb_int = 0;
    let mut argv: *mut mrb_value = 0 as *mut mrb_value;
    mrb_get_args(mrb, b"*!&\x00" as *const u8 as *const libc::c_char,
                 &mut argv as *mut *mut mrb_value, &mut argc as *mut mrb_int,
                 &mut b as *mut mrb_value);
    if b.tt as libc::c_uint == MRB_TT_FALSE as libc::c_int as libc::c_uint &&
           0 == b.value.i {
        let mut s: *mut libc::c_char = 0 as *mut libc::c_char;
        let mut len: mrb_int = 0;
        let mut file: *mut libc::c_char = 0 as *mut libc::c_char;
        let mut line: mrb_int = 1i32 as mrb_int;
        let mut cv: mrb_value =
            mrb_value{value: C2RustUnnamed_0{f: 0.,}, tt: MRB_TT_FALSE,};
        let mut proc_0: *mut RProc = 0 as *mut RProc;
        mrb_get_args(mrb, b"s|zi\x00" as *const u8 as *const libc::c_char,
                     &mut s as *mut *mut libc::c_char,
                     &mut len as *mut mrb_int,
                     &mut file as *mut *mut libc::c_char,
                     &mut line as *mut mrb_int);
        cv = mrb_singleton_class(mrb, self_0);
        proc_0 =
            create_proc_from_string(mrb, s, len, mrb_nil_value(), file, line);
        if (*proc_0).flags() as libc::c_int & 1024i32 != 0i32 {
            (*(*proc_0).e.env).c = cv.value.p as *mut RClass;
            mrb_field_write_barrier(mrb, (*proc_0).e.env as *mut RBasic,
                                    cv.value.p as *mut RClass as *mut RBasic);
        } else {
            (*proc_0).e.target_class = cv.value.p as *mut RClass;
            mrb_field_write_barrier(mrb, proc_0 as *mut RBasic,
                                    cv.value.p as *mut RClass as *mut RBasic);
        }
        (*(*(*mrb).c).ci).target_class = cv.value.p as *mut RClass;
        return exec_irep(mrb, self_0, proc_0)
    } else {
        mrb_get_args(mrb, b"&\x00" as *const u8 as *const libc::c_char,
                     &mut b as *mut mrb_value);
        return mrb_obj_instance_eval(mrb, self_0)
    };
}
#[no_mangle]
pub unsafe extern "C" fn mrb_mruby_eval_gem_init(mut mrb: *mut mrb_state) {
    mrb_define_module_function(mrb, (*mrb).kernel_module,
                               b"eval\x00" as *const u8 as
                                   *const libc::c_char,
                               Some(f_eval as
                                        unsafe extern "C" fn(_:
                                                                 *mut mrb_state,
                                                             _: mrb_value)
                                            -> mrb_value),
                               ((1i32 & 0x1fi32) as mrb_aspec) << 18i32 |
                                   ((3i32 & 0x1fi32) as mrb_aspec) << 13i32);
    mrb_define_method(mrb,
                      mrb_class_get(mrb,
                                    b"BasicObject\x00" as *const u8 as
                                        *const libc::c_char),
                      b"instance_eval\x00" as *const u8 as
                          *const libc::c_char,
                      Some(f_instance_eval as
                               unsafe extern "C" fn(_: *mut mrb_state,
                                                    _: mrb_value)
                                   -> mrb_value),
                      ((1i32 & 0x1fi32) as mrb_aspec) << 18i32 |
                          ((2i32 & 0x1fi32) as mrb_aspec) << 13i32);
}
#[no_mangle]
pub unsafe extern "C" fn mrb_mruby_eval_gem_final(mut mrb: *mut mrb_state) { }