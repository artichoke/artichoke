use libc;
use c2rust_bitfields::{BitfieldStruct};
extern "C" {
    pub type iv_tbl;
    pub type kh_mt;
    pub type symbol_name;
    pub type RProc;
    pub type REnv;
    pub type mrb_jmpbuf;
    pub type mrb_shared_string;
    #[no_mangle]
    fn pow(_: libc::c_double, _: libc::c_double) -> libc::c_double;
    #[no_mangle]
    fn ceil(_: libc::c_double) -> libc::c_double;
    #[no_mangle]
    fn floor(_: libc::c_double) -> libc::c_double;
    #[no_mangle]
    fn round(_: libc::c_double) -> libc::c_double;
    #[no_mangle]
    fn fmod(_: libc::c_double, _: libc::c_double) -> libc::c_double;
    #[no_mangle]
    fn memcpy(_: *mut libc::c_void, _: *const libc::c_void, _: libc::c_ulong)
     -> *mut libc::c_void;
    #[no_mangle]
    fn memmove(_: *mut libc::c_void, _: *const libc::c_void, _: libc::c_ulong)
     -> *mut libc::c_void;
    /* *
 * Defines a new class.
 *
 * If you're creating a gem it may look something like this:
 *
 *      !!!c
 *      void mrb_example_gem_init(mrb_state* mrb) {
 *          struct RClass *example_class;
 *          example_class = mrb_define_class(mrb, "Example_Class", mrb->object_class);
 *      }
 *
 *      void mrb_example_gem_final(mrb_state* mrb) {
 *          //free(TheAnimals);
 *      }
 *
 * @param [mrb_state *] mrb The current mruby state.
 * @param [const char *] name The name of the defined class.
 * @param [struct RClass *] super The new class parent.
 * @return [struct RClass *] Reference to the newly defined class.
 * @see mrb_define_class_under
 */
    #[no_mangle]
    fn mrb_define_class(mrb: *mut mrb_state, name: *const libc::c_char,
                        super_0: *mut RClass) -> *mut RClass;
    /* *
 * Defines a new module.
 *
 * @param [mrb_state *] mrb_state* The current mruby state.
 * @param [const char *] char* The name of the module.
 * @return [struct RClass *] Reference to the newly defined module.
 */
    #[no_mangle]
    fn mrb_define_module(_: *mut mrb_state, _: *const libc::c_char)
     -> *mut RClass;
    /* *
 * Include a module in another class or module.
 * Equivalent to:
 *
 *   module B
 *     include A
 *   end
 * @param [mrb_state *] mrb_state* The current mruby state.
 * @param [struct RClass *] RClass* A reference to module or a class.
 * @param [struct RClass *] RClass* A reference to the module to be included.
 */
    #[no_mangle]
    fn mrb_include_module(_: *mut mrb_state, _: *mut RClass, _: *mut RClass);
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
    fn mrb_define_const(_: *mut mrb_state, _: *mut RClass,
                        name: *const libc::c_char, _: mrb_value);
    /* *
 * Undefine a class method.
 * Example:
 *
 *      # Ruby style
 *      class ExampleClass
 *        def self.example_method
 *          "example"
 *        end
 *      end
 *
 *     ExampleClass.example_method
 *
 *     // C style
 *     #include <stdio.h>
 *     #include <mruby.h>
 *
 *     mrb_value
 *     mrb_example_method(mrb_state *mrb){
 *       return mrb_str_new_lit(mrb, "example");
 *     }
 *
 *     void
 *     mrb_example_gem_init(mrb_state* mrb){
 *       struct RClass *example_class;
 *       example_class = mrb_define_class(mrb, "ExampleClass", mrb->object_class);
 *       mrb_define_class_method(mrb, example_class, "example_method", mrb_example_method, MRB_ARGS_NONE());
 *       mrb_undef_class_method(mrb, example_class, "example_method");
 *      }
 *
 *      void
 *      mrb_example_gem_final(mrb_state* mrb){
 *      }
 * @param [mrb_state*] mrb_state* The mruby state reference.
 * @param [RClass*] RClass* A class the class method will be undefined from.
 * @param [const char*] const char* The name of the class method to be undefined.
 */
    #[no_mangle]
    fn mrb_undef_class_method(_: *mut mrb_state, _: *mut RClass,
                              _: *const libc::c_char);
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
    /* `strlen` for character string literals (use with caution or `strlen` instead)
    Adjacent string literals are concatenated in C/C++ in translation phase 6.
    If `lit` is not one, the compiler will report a syntax error:
     MSVC: "error C2143: syntax error : missing ')' before 'string'"
     GCC:  "error: expected ')' before string constant"
*/
    /* *
 * Call existing ruby functions.
 *
 *      #include <stdio.h>
 *      #include <mruby.h>
 *      #include "mruby/compile.h"
 *
 *      int
 *      main()
 *      {
 *        mrb_int i = 99;
 *        mrb_state *mrb = mrb_open();
 *
 *        if (!mrb) { }
 *        FILE *fp = fopen("test.rb","r");
 *        mrb_value obj = mrb_load_file(mrb,fp);
 *        mrb_funcall(mrb, obj, "method_name", 1, mrb_fixnum_value(i));
 *        fclose(fp);
 *        mrb_close(mrb);
 *       }
 * @param [mrb_state*] mrb_state* The current mruby state.
 * @param [mrb_value] mrb_value A reference to an mruby value.
 * @param [const char*] const char* The name of the method.
 * @param [mrb_int] mrb_int The number of arguments the method has.
 * @param [...] ... Variadic values(not type safe!).
 * @return [mrb_value] mrb_value mruby function value.
 */
    #[no_mangle]
    fn mrb_funcall(_: *mut mrb_state, _: mrb_value, _: *const libc::c_char,
                   _: mrb_int, _: ...) -> mrb_value;
    #[no_mangle]
    fn mrb_str_new(mrb: *mut mrb_state, p: *const libc::c_char, len: size_t)
     -> mrb_value;
    #[no_mangle]
    fn mrb_str_new_static(mrb: *mut mrb_state, p: *const libc::c_char,
                          len: size_t) -> mrb_value;
    #[no_mangle]
    fn mrb_raise(mrb: *mut mrb_state, c: *mut RClass,
                 msg: *const libc::c_char) -> !;
    #[no_mangle]
    fn mrb_raisef(mrb: *mut mrb_state, c: *mut RClass,
                  fmt: *const libc::c_char, _: ...) -> !;
    /*
 * Initializes a new array with two initial values
 *
 * Equivalent to:
 *
 *      Array[car, cdr]
 *
 * @param mrb The mruby state reference.
 * @param car The first value.
 * @param cdr The second value.
 * @return The initialized array.
 */
    #[no_mangle]
    fn mrb_assoc_new(mrb: *mut mrb_state, car: mrb_value, cdr: mrb_value)
     -> mrb_value;
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
    /* ArgumentError if format string doesn't match /%(\.[0-9]+)?[aAeEfFgG]/ */
    #[no_mangle]
    fn mrb_float_to_str(mrb: *mut mrb_state, x: mrb_value,
                        fmt: *const libc::c_char) -> mrb_value;
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
pub type int64_t = libc::c_longlong;
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
/* *
 * Required arguments signature type.
 */
pub type mrb_aspec = uint32_t;
/* default method cache size: 128 */
/* cache size needs to be power of 2 */
pub type mrb_func_t
    =
    Option<unsafe extern "C" fn(_: *mut mrb_state, _: mrb_value)
               -> mrb_value>;
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
    pub as_0: unnamed_0,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub union unnamed_0 {
    pub heap: unnamed_1,
    pub ary: [libc::c_char; 24],
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
#[inline(always)]
unsafe extern "C" fn __inline_isnanf(mut __x: libc::c_float) -> libc::c_int {
    return (__x != __x) as libc::c_int;
}
#[inline(always)]
unsafe extern "C" fn __inline_isnand(mut __x: libc::c_double) -> libc::c_int {
    return (__x != __x) as libc::c_int;
}
#[inline(always)]
unsafe extern "C" fn __inline_isnanl(mut __x: f128::f128) -> libc::c_int {
    return (__x != __x) as libc::c_int;
}
/*
 * Returns a float in Ruby.
 */
#[inline]
unsafe extern "C" fn mrb_float_value(mut mrb: *mut mrb_state,
                                     mut f: mrb_float) -> mrb_value {
    let mut v: mrb_value =
        mrb_value{value: unnamed{f: 0.,}, tt: MRB_TT_FALSE,};
    v.tt = MRB_TT_FLOAT;
    v.value.f = f;
    return v;
}
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
unsafe extern "C" fn mrb_obj_value(mut p: *mut libc::c_void) -> mrb_value {
    let mut v: mrb_value =
        mrb_value{value: unnamed{f: 0.,}, tt: MRB_TT_FALSE,};
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
        mrb_value{value: unnamed{f: 0.,}, tt: MRB_TT_FALSE,};
    v.tt = MRB_TT_FALSE;
    v.value.i = 0i32 as mrb_int;
    return v;
}
/*
 * Returns false in Ruby.
 */
#[inline]
unsafe extern "C" fn mrb_false_value() -> mrb_value {
    let mut v: mrb_value =
        mrb_value{value: unnamed{f: 0.,}, tt: MRB_TT_FALSE,};
    v.tt = MRB_TT_FALSE;
    v.value.i = 1i32 as mrb_int;
    return v;
}
/*
 * Returns true in Ruby.
 */
#[inline]
unsafe extern "C" fn mrb_true_value() -> mrb_value {
    let mut v: mrb_value =
        mrb_value{value: unnamed{f: 0.,}, tt: MRB_TT_FALSE,};
    v.tt = MRB_TT_TRUE;
    v.value.i = 1i32 as mrb_int;
    return v;
}
#[inline]
unsafe extern "C" fn mrb_bool_value(mut boolean: mrb_bool) -> mrb_value {
    let mut v: mrb_value =
        mrb_value{value: unnamed{f: 0.,}, tt: MRB_TT_FALSE,};
    v.tt =
        (if 0 != boolean as libc::c_int {
             MRB_TT_TRUE as libc::c_int
         } else { MRB_TT_FALSE as libc::c_int }) as mrb_vtype;
    v.value.i = 1i32 as mrb_int;
    return v;
}
/*
** mruby/numeric.h - Numeric, Integer, Float, Fixnum class
**
** See Copyright Notice in mruby.h
*/
/* *
 * Numeric class and it's sub-classes.
 *
 * Integer, Float and Fixnum
 */
#[no_mangle]
pub unsafe extern "C" fn mrb_flo_to_fixnum(mut mrb: *mut mrb_state,
                                           mut x: mrb_value) -> mrb_value {
    let mut z: mrb_int = 0i32 as mrb_int;
    if !(x.tt as libc::c_uint == MRB_TT_FLOAT as libc::c_int as libc::c_uint)
       {
        mrb_raise(mrb,
                  mrb_exc_get(mrb,
                              b"TypeError\x00" as *const u8 as
                                  *const libc::c_char),
                  b"non float value\x00" as *const u8 as *const libc::c_char);
    } else {
        let mut d: mrb_float = x.value.f;
        mrb_check_num_exact(mrb, d);
        if d <= (9223372036854775807i64 >> 0i32) as mrb_float &&
               d >=
                   (-9223372036854775807i64 - 1i32 as libc::c_longlong >>
                        0i32) as mrb_float {
            z = d as mrb_int
        } else {
            mrb_raisef(mrb,
                       mrb_exc_get(mrb,
                                   b"RangeError\x00" as *const u8 as
                                       *const libc::c_char),
                       b"number (%S) too big for integer\x00" as *const u8 as
                           *const libc::c_char, x);
        }
    }
    return mrb_fixnum_value(z);
}
#[no_mangle]
pub unsafe extern "C" fn mrb_check_num_exact(mut mrb: *mut mrb_state,
                                             mut num: mrb_float) {
    if 0 !=
           if ::std::mem::size_of::<mrb_float>() as libc::c_ulong ==
                  ::std::mem::size_of::<libc::c_float>() as libc::c_ulong {
               __inline_isinff(num as libc::c_float)
           } else if ::std::mem::size_of::<mrb_float>() as libc::c_ulong ==
                         ::std::mem::size_of::<libc::c_double>() as
                             libc::c_ulong {
               __inline_isinfd(num as libc::c_double)
           } else { __inline_isinfl(f128::f128::new(num)) } {
        mrb_raise(mrb,
                  mrb_exc_get(mrb,
                              b"FloatDomainError\x00" as *const u8 as
                                  *const libc::c_char),
                  if num < 0i32 as libc::c_double {
                      b"-Infinity\x00" as *const u8 as *const libc::c_char
                  } else {
                      b"Infinity\x00" as *const u8 as *const libc::c_char
                  });
    }
    if 0 !=
           if ::std::mem::size_of::<mrb_float>() as libc::c_ulong ==
                  ::std::mem::size_of::<libc::c_float>() as libc::c_ulong {
               __inline_isnanf(num as libc::c_float)
           } else if ::std::mem::size_of::<mrb_float>() as libc::c_ulong ==
                         ::std::mem::size_of::<libc::c_double>() as
                             libc::c_ulong {
               __inline_isnand(num as libc::c_double)
           } else { __inline_isnanl(f128::f128::new(num)) } {
        mrb_raise(mrb,
                  mrb_exc_get(mrb,
                              b"FloatDomainError\x00" as *const u8 as
                                  *const libc::c_char),
                  b"NaN\x00" as *const u8 as *const libc::c_char);
    };
}
#[no_mangle]
pub unsafe extern "C" fn mrb_fixnum_to_str(mut mrb: *mut mrb_state,
                                           mut x: mrb_value,
                                           mut base: mrb_int) -> mrb_value {
    let mut buf: [libc::c_char; 65] = [0; 65];
    let mut b: *mut libc::c_char =
        buf.as_mut_ptr().offset(::std::mem::size_of::<[libc::c_char; 65]>() as
                                    libc::c_ulong as isize);
    let mut val: mrb_int = x.value.i;
    if base < 2i32 as libc::c_longlong || (36i32 as libc::c_longlong) < base {
        mrb_raisef(mrb,
                   mrb_exc_get(mrb,
                               b"ArgumentError\x00" as *const u8 as
                                   *const libc::c_char),
                   b"invalid radix %S\x00" as *const u8 as
                       *const libc::c_char, mrb_fixnum_value(base));
    }
    if val == 0i32 as libc::c_longlong {
        b = b.offset(-1isize);
        *b = '0' as i32 as libc::c_char
    } else if val < 0i32 as libc::c_longlong {
        loop  {
            b = b.offset(-1isize);
            *b = *mrb_digitmap.as_ptr().offset(-(val % base) as isize);
            val /= base;
            if !(0 != val) { break ; }
        }
        b = b.offset(-1isize);
        *b = '-' as i32 as libc::c_char
    } else {
        loop  {
            b = b.offset(-1isize);
            *b =
                *mrb_digitmap.as_ptr().offset((val % base) as libc::c_int as
                                                  isize);
            val /= base;
            if !(0 != val) { break ; }
        }
    }
    return mrb_str_new(mrb, b,
                       buf.as_mut_ptr().offset(::std::mem::size_of::<[libc::c_char; 65]>()
                                                   as libc::c_ulong as
                                                   isize).wrapping_offset_from(b)
                           as libc::c_long as size_t);
}
#[no_mangle]
pub unsafe extern "C" fn mrb_to_flo(mut mrb: *mut mrb_state,
                                    mut val: mrb_value) -> mrb_float {
    match val.tt as libc::c_uint {
        3 => { return val.value.i as mrb_float }
        6 => { }
        _ => {
            mrb_raise(mrb,
                      mrb_exc_get(mrb,
                                  b"TypeError\x00" as *const u8 as
                                      *const libc::c_char),
                      b"non float value\x00" as *const u8 as
                          *const libc::c_char);
        }
    }
    return val.value.f;
}
#[no_mangle]
pub unsafe extern "C" fn mrb_int_value(mut mrb: *mut mrb_state,
                                       mut f: mrb_float) -> mrb_value {
    if f <= (9223372036854775807i64 >> 0i32) as mrb_float &&
           f >=
               (-9223372036854775807i64 - 1i32 as libc::c_longlong >> 0i32) as
                   mrb_float {
        return mrb_fixnum_value(f as mrb_int)
    }
    return mrb_float_value(mrb, f);
}
#[no_mangle]
pub unsafe extern "C" fn mrb_num_plus(mut mrb: *mut mrb_state,
                                      mut x: mrb_value, mut y: mrb_value)
 -> mrb_value {
    if x.tt as libc::c_uint == MRB_TT_FIXNUM as libc::c_int as libc::c_uint {
        return fixnum_plus(mrb, x, y)
    }
    if x.tt as libc::c_uint == MRB_TT_FLOAT as libc::c_int as libc::c_uint {
        return mrb_float_value(mrb, x.value.f + mrb_to_flo(mrb, y))
    }
    mrb_raise(mrb,
              mrb_exc_get(mrb,
                          b"TypeError\x00" as *const u8 as
                              *const libc::c_char),
              b"no number addition\x00" as *const u8 as *const libc::c_char);
}
unsafe extern "C" fn fixnum_plus(mut mrb: *mut mrb_state, mut x: mrb_value,
                                 mut y: mrb_value) -> mrb_value {
    let mut a: mrb_int = 0;
    a = x.value.i;
    if y.tt as libc::c_uint == MRB_TT_FIXNUM as libc::c_int as libc::c_uint {
        let mut b: mrb_int = 0;
        let mut c: mrb_int = 0;
        if a == 0i32 as libc::c_longlong { return y }
        b = y.value.i;
        if 0 != mrb_int_add_overflow(a, b, &mut c) {
            return mrb_float_value(mrb, a as mrb_float + b as mrb_float)
        }
        return mrb_fixnum_value(c)
    }
    return mrb_float_value(mrb, a as mrb_float + mrb_to_flo(mrb, y));
}
/*
// Clang 3.8 and 3.9 have problem compiling mruby in 32-bit mode, when MRB_INT64 is set
// because of missing __mulodi4 and similar functions in its runtime. We need to use custom
// implementation for them.
*/
#[inline]
unsafe extern "C" fn mrb_int_add_overflow(mut augend: mrb_int,
                                          mut addend: mrb_int,
                                          mut sum: *mut mrb_int) -> mrb_bool {
    let (fresh0, fresh1) = augend.overflowing_add(addend);
    *sum = fresh0;
    return (0 != fresh1 as libc::c_int || 0 != 0i32) as libc::c_int as
               mrb_bool;
}
#[no_mangle]
pub unsafe extern "C" fn mrb_num_minus(mut mrb: *mut mrb_state,
                                       mut x: mrb_value, mut y: mrb_value)
 -> mrb_value {
    if x.tt as libc::c_uint == MRB_TT_FIXNUM as libc::c_int as libc::c_uint {
        return fixnum_minus(mrb, x, y)
    }
    if x.tt as libc::c_uint == MRB_TT_FLOAT as libc::c_int as libc::c_uint {
        return mrb_float_value(mrb, x.value.f - mrb_to_flo(mrb, y))
    }
    mrb_raise(mrb,
              mrb_exc_get(mrb,
                          b"TypeError\x00" as *const u8 as
                              *const libc::c_char),
              b"no number subtraction\x00" as *const u8 as
                  *const libc::c_char);
}
unsafe extern "C" fn fixnum_minus(mut mrb: *mut mrb_state, mut x: mrb_value,
                                  mut y: mrb_value) -> mrb_value {
    let mut a: mrb_int = 0;
    a = x.value.i;
    if y.tt as libc::c_uint == MRB_TT_FIXNUM as libc::c_int as libc::c_uint {
        let mut b: mrb_int = 0;
        let mut c: mrb_int = 0;
        b = y.value.i;
        if 0 != mrb_int_sub_overflow(a, b, &mut c) {
            return mrb_float_value(mrb, a as mrb_float - b as mrb_float)
        }
        return mrb_fixnum_value(c)
    }
    return mrb_float_value(mrb, a as mrb_float - mrb_to_flo(mrb, y));
}
#[inline]
unsafe extern "C" fn mrb_int_sub_overflow(mut minuend: mrb_int,
                                          mut subtrahend: mrb_int,
                                          mut difference: *mut mrb_int)
 -> mrb_bool {
    let (fresh2, fresh3) = minuend.overflowing_sub(subtrahend);
    *difference = fresh2;
    return (0 != fresh3 as libc::c_int || 0 != 0i32) as libc::c_int as
               mrb_bool;
}
#[no_mangle]
pub unsafe extern "C" fn mrb_num_mul(mut mrb: *mut mrb_state,
                                     mut x: mrb_value, mut y: mrb_value)
 -> mrb_value {
    if x.tt as libc::c_uint == MRB_TT_FIXNUM as libc::c_int as libc::c_uint {
        return fixnum_mul(mrb, x, y)
    }
    if x.tt as libc::c_uint == MRB_TT_FLOAT as libc::c_int as libc::c_uint {
        return mrb_float_value(mrb, x.value.f * mrb_to_flo(mrb, y))
    }
    mrb_raise(mrb,
              mrb_exc_get(mrb,
                          b"TypeError\x00" as *const u8 as
                              *const libc::c_char),
              b"no number multiply\x00" as *const u8 as *const libc::c_char);
}
unsafe extern "C" fn fixnum_mul(mut mrb: *mut mrb_state, mut x: mrb_value,
                                mut y: mrb_value) -> mrb_value {
    let mut a: mrb_int = 0;
    a = x.value.i;
    if y.tt as libc::c_uint == MRB_TT_FIXNUM as libc::c_int as libc::c_uint {
        let mut b: mrb_int = 0;
        let mut c: mrb_int = 0;
        if a == 0i32 as libc::c_longlong { return x }
        b = y.value.i;
        if 0 != mrb_int_mul_overflow(a, b, &mut c) {
            return mrb_float_value(mrb, a as mrb_float * b as mrb_float)
        }
        return mrb_fixnum_value(c)
    }
    return mrb_float_value(mrb, a as mrb_float * mrb_to_flo(mrb, y));
}
#[inline]
unsafe extern "C" fn mrb_int_mul_overflow(mut multiplier: mrb_int,
                                          mut multiplicand: mrb_int,
                                          mut product: *mut mrb_int)
 -> mrb_bool {
    let (fresh4, fresh5) = multiplier.overflowing_mul(multiplicand);
    *product = fresh4;
    return (0 != fresh5 as libc::c_int || 0 != 0i32) as libc::c_int as
               mrb_bool;
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
/*
 * call-seq:
 *
 *  num ** other  ->  num
 *
 * Raises <code>num</code> the <code>other</code> power.
 *
 *    2.0**3      #=> 8.0
 */
unsafe extern "C" fn integral_pow(mut mrb: *mut mrb_state, mut x: mrb_value)
 -> mrb_value {
    let mut current_block: u64;
    let mut y: mrb_value =
        mrb_value{value: unnamed{f: 0.,}, tt: MRB_TT_FALSE,};
    let mut d: mrb_float = 0.;
    mrb_get_args(mrb, b"o\x00" as *const u8 as *const libc::c_char,
                 &mut y as *mut mrb_value);
    if x.tt as libc::c_uint == MRB_TT_FIXNUM as libc::c_int as libc::c_uint &&
           y.tt as libc::c_uint ==
               MRB_TT_FIXNUM as libc::c_int as libc::c_uint {
        /* try ipow() */
        let mut base: mrb_int = x.value.i;
        let mut exp: mrb_int = y.value.i;
        let mut result: mrb_int = 1i32 as mrb_int;
        if !(exp < 0i32 as libc::c_longlong) {
            loop  {
                if 0 != exp & 1i32 as libc::c_longlong {
                    if 0 != mrb_int_mul_overflow(result, base, &mut result) {
                        current_block = 8078319602943683960;
                        break ;
                    }
                }
                exp >>= 1i32;
                if exp == 0i32 as libc::c_longlong {
                    current_block = 17860125682698302841;
                    break ;
                }
                if 0 != mrb_int_mul_overflow(base, base, &mut base) {
                    current_block = 8078319602943683960;
                    break ;
                }
            }
            match current_block {
                8078319602943683960 => { }
                _ => { return mrb_fixnum_value(result) }
            }
        }
    }
    d = pow(mrb_to_flo(mrb, x), mrb_to_flo(mrb, y));
    return mrb_float_value(mrb, d);
}
unsafe extern "C" fn integral_idiv(mut mrb: *mut mrb_state, mut x: mrb_value)
 -> mrb_value {
    let mut y: mrb_float = 0.;
    mrb_get_args(mrb, b"f\x00" as *const u8 as *const libc::c_char,
                 &mut y as *mut mrb_float);
    return mrb_int_value(mrb, mrb_to_flo(mrb, x) / y);
}
/* 15.2.8.3.4  */
/* 15.2.9.3.4  */
/*
 * call-seq:
 *   num / other  ->  num
 *
 * Performs division: the class of the resulting object depends on
 * the class of <code>num</code> and on the magnitude of the
 * result.
 */
/* 15.2.9.3.19(x) */
/*
 *  call-seq:
 *     num.quo(numeric)  ->  real
 *
 *  Returns most exact division.
 */
unsafe extern "C" fn integral_div(mut mrb: *mut mrb_state, mut x: mrb_value)
 -> mrb_value {
    let mut y: mrb_float = 0.;
    mrb_get_args(mrb, b"f\x00" as *const u8 as *const libc::c_char,
                 &mut y as *mut mrb_float);
    return mrb_float_value(mrb, mrb_to_flo(mrb, x) / y);
}
unsafe extern "C" fn integral_coerce_step_counter(mut mrb: *mut mrb_state,
                                                  mut self_0: mrb_value)
 -> mrb_value {
    let mut counter: mrb_value = self_0;
    let mut num: mrb_value =
        mrb_value{value: unnamed{f: 0.,}, tt: MRB_TT_FALSE,};
    let mut step: mrb_value =
        mrb_value{value: unnamed{f: 0.,}, tt: MRB_TT_FALSE,};
    mrb_get_args(mrb, b"oo\x00" as *const u8 as *const libc::c_char,
                 &mut num as *mut mrb_value, &mut step as *mut mrb_value);
    if self_0.tt as libc::c_uint ==
           MRB_TT_FLOAT as libc::c_int as libc::c_uint ||
           num.tt as libc::c_uint ==
               MRB_TT_FLOAT as libc::c_int as libc::c_uint ||
           step.tt as libc::c_uint ==
               MRB_TT_FLOAT as libc::c_int as libc::c_uint {
        counter =
            mrb_funcall(mrb, counter,
                        b"to_f\x00" as *const u8 as *const libc::c_char,
                        0i32 as mrb_int)
    }
    return counter;
}
/* *******************************************************************
 *
 * Document-class: Float
 *
 *  <code>Float</code> objects represent inexact real numbers using
 *  the native architecture's double-precision floating point
 *  representation.
 */
/* 15.2.9.3.16(x) */
/*
 *  call-seq:
 *     flt.to_s  ->  string
 *
 *  Returns a string containing a representation of self. As well as a
 *  fixed or exponential form of the number, the call may return
 *  "<code>NaN</code>", "<code>Infinity</code>", and
 *  "<code>-Infinity</code>".
 */
unsafe extern "C" fn flo_to_s(mut mrb: *mut mrb_state, mut flt: mrb_value)
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
                                      b"-Infinity\x00" as *const u8 as
                                          *const libc::c_char,
                                      (::std::mem::size_of::<[libc::c_char; 10]>()
                                           as
                                           libc::c_ulong).wrapping_sub(1i32 as
                                                                           libc::c_ulong))
               } else {
                   mrb_str_new_static(mrb,
                                      b"Infinity\x00" as *const u8 as
                                          *const libc::c_char,
                                      (::std::mem::size_of::<[libc::c_char; 9]>()
                                           as
                                           libc::c_ulong).wrapping_sub(1i32 as
                                                                           libc::c_ulong))
               }
    } else if 0 !=
                  if ::std::mem::size_of::<mrb_float>() as libc::c_ulong ==
                         ::std::mem::size_of::<libc::c_float>() as
                             libc::c_ulong {
                      __inline_isnanf(f as libc::c_float)
                  } else if ::std::mem::size_of::<mrb_float>() as
                                libc::c_ulong ==
                                ::std::mem::size_of::<libc::c_double>() as
                                    libc::c_ulong {
                      __inline_isnand(f as libc::c_double)
                  } else { __inline_isnanl(f128::f128::new(f)) } {
        return mrb_str_new_static(mrb,
                                  b"NaN\x00" as *const u8 as
                                      *const libc::c_char,
                                  (::std::mem::size_of::<[libc::c_char; 4]>()
                                       as
                                       libc::c_ulong).wrapping_sub(1i32 as
                                                                       libc::c_ulong))
    } else {
        let mut fmt: [libc::c_char; 6] =
            *::std::mem::transmute::<&[u8; 6],
                                     &mut [libc::c_char; 6]>(b"%.16g\x00");
        let mut str: mrb_value = mrb_float_to_str(mrb, flt, fmt.as_mut_ptr());
        let mut len: mrb_int = 0;
        let mut begp: *mut libc::c_char = 0 as *mut libc::c_char;
        let mut p: *mut libc::c_char = 0 as *mut libc::c_char;
        let mut endp: *mut libc::c_char = 0 as *mut libc::c_char;
        loop  {
            begp =
                if 0 !=
                       (*(str.value.p as *mut RString)).flags() as libc::c_int
                           & 32i32 {
                    (*(str.value.p as *mut RString)).as_0.ary.as_mut_ptr()
                } else { (*(str.value.p as *mut RString)).as_0.heap.ptr };
            len =
                if 0 !=
                       (*(str.value.p as *mut RString)).flags() as libc::c_int
                           & 32i32 {
                    (((*(str.value.p as *mut RString)).flags() as libc::c_int
                          & 0x7c0i32) >> 6i32) as mrb_int
                } else { (*(str.value.p as *mut RString)).as_0.heap.len };
            p = begp;
            endp = p.offset(len as isize);
            while p < endp {
                if *p as libc::c_int == '.' as i32 {
                    return str
                } else {
                    if *p as libc::c_int == 'e' as i32 {
                        let mut e_pos: ptrdiff_t =
                            p.wrapping_offset_from(begp) as libc::c_long;
                        mrb_str_cat(mrb, str,
                                    b".0\x00" as *const u8 as
                                        *const libc::c_char, 2i32 as size_t);
                        p =
                            if 0 !=
                                   (*(str.value.p as *mut RString)).flags() as
                                       libc::c_int & 32i32 {
                                (*(str.value.p as
                                       *mut RString)).as_0.ary.as_mut_ptr()
                            } else {
                                (*(str.value.p as *mut RString)).as_0.heap.ptr
                            }.offset(e_pos as isize);
                        memmove(p.offset(2isize) as *mut libc::c_void,
                                p as *const libc::c_void,
                                (len - e_pos as libc::c_longlong) as
                                    libc::c_ulong);
                        memcpy(p as *mut libc::c_void,
                               b".0\x00" as *const u8 as *const libc::c_char
                                   as *const libc::c_void,
                               2i32 as libc::c_ulong);
                        return str
                    }
                }
                p = p.offset(1isize)
            }
            if !((16i32 +
                      (*begp.offset(0isize) as libc::c_int == '-' as i32) as
                          libc::c_int) as libc::c_longlong <= len) {
                break ;
            }
            fmt[(::std::mem::size_of::<[libc::c_char; 6]>() as
                     libc::c_ulong).wrapping_sub(3i32 as libc::c_ulong) as
                    usize] -= 1;
            str = mrb_float_to_str(mrb, flt, fmt.as_mut_ptr())
        }
        mrb_str_cat(mrb, str, b".0\x00" as *const u8 as *const libc::c_char,
                    2i32 as size_t);
        return str
    };
}
/* 15.2.9.3.2  */
/*
 * call-seq:
 *   float - other  ->  float
 *
 * Returns a new float which is the difference of <code>float</code>
 * and <code>other</code>.
 */
unsafe extern "C" fn flo_minus(mut mrb: *mut mrb_state, mut x: mrb_value)
 -> mrb_value {
    let mut y: mrb_value =
        mrb_value{value: unnamed{f: 0.,}, tt: MRB_TT_FALSE,};
    mrb_get_args(mrb, b"o\x00" as *const u8 as *const libc::c_char,
                 &mut y as *mut mrb_value);
    return mrb_float_value(mrb, x.value.f - mrb_to_flo(mrb, y));
}
/* 15.2.9.3.3  */
/*
 * call-seq:
 *   float * other  ->  float
 *
 * Returns a new float which is the product of <code>float</code>
 * and <code>other</code>.
 */
unsafe extern "C" fn flo_mul(mut mrb: *mut mrb_state, mut x: mrb_value)
 -> mrb_value {
    let mut y: mrb_value =
        mrb_value{value: unnamed{f: 0.,}, tt: MRB_TT_FALSE,};
    mrb_get_args(mrb, b"o\x00" as *const u8 as *const libc::c_char,
                 &mut y as *mut mrb_value);
    return mrb_float_value(mrb, x.value.f * mrb_to_flo(mrb, y));
}
unsafe extern "C" fn flodivmod(mut mrb: *mut mrb_state, mut x: libc::c_double,
                               mut y: libc::c_double,
                               mut divp: *mut mrb_float,
                               mut modp: *mut mrb_float) {
    let mut div: libc::c_double = 0.;
    let mut mod_0: libc::c_double = 0.;
    if 0 !=
           if ::std::mem::size_of::<libc::c_double>() as libc::c_ulong ==
                  ::std::mem::size_of::<libc::c_float>() as libc::c_ulong {
               __inline_isnanf(y as libc::c_float)
           } else if ::std::mem::size_of::<libc::c_double>() as libc::c_ulong
                         ==
                         ::std::mem::size_of::<libc::c_double>() as
                             libc::c_ulong {
               __inline_isnand(y as libc::c_double)
           } else { __inline_isnanl(f128::f128::new(y)) } {
        mod_0 = y;
        div = mod_0
    } else if y == 0.0f64 {
        if x == 0i32 as libc::c_double {
            div = ::std::f32::NAN as libc::c_double
        } else if x > 0.0f64 {
            div = ::std::f32::INFINITY as libc::c_double
        } else { div = -::std::f32::INFINITY as libc::c_double }
        mod_0 = ::std::f32::NAN as libc::c_double
    } else {
        if x == 0.0f64 ||
               0 !=
                   if ::std::mem::size_of::<libc::c_double>() as libc::c_ulong
                          ==
                          ::std::mem::size_of::<libc::c_float>() as
                              libc::c_ulong {
                       __inline_isinff(y as libc::c_float)
                   } else if ::std::mem::size_of::<libc::c_double>() as
                                 libc::c_ulong ==
                                 ::std::mem::size_of::<libc::c_double>() as
                                     libc::c_ulong {
                       __inline_isinfd(y as libc::c_double)
                   } else { __inline_isinfl(f128::f128::new(y)) } &&
                   0 ==
                       if ::std::mem::size_of::<libc::c_double>() as
                              libc::c_ulong ==
                              ::std::mem::size_of::<libc::c_float>() as
                                  libc::c_ulong {
                           __inline_isinff(x as libc::c_float)
                       } else if ::std::mem::size_of::<libc::c_double>() as
                                     libc::c_ulong ==
                                     ::std::mem::size_of::<libc::c_double>()
                                         as libc::c_ulong {
                           __inline_isinfd(x as libc::c_double)
                       } else { __inline_isinfl(f128::f128::new(x)) } {
            mod_0 = x
        } else { mod_0 = fmod(x, y) }
        if 0 !=
               if ::std::mem::size_of::<libc::c_double>() as libc::c_ulong ==
                      ::std::mem::size_of::<libc::c_float>() as libc::c_ulong
                  {
                   __inline_isinff(x as libc::c_float)
               } else if ::std::mem::size_of::<libc::c_double>() as
                             libc::c_ulong ==
                             ::std::mem::size_of::<libc::c_double>() as
                                 libc::c_ulong {
                   __inline_isinfd(x as libc::c_double)
               } else { __inline_isinfl(f128::f128::new(x)) } &&
               0 ==
                   if ::std::mem::size_of::<libc::c_double>() as libc::c_ulong
                          ==
                          ::std::mem::size_of::<libc::c_float>() as
                              libc::c_ulong {
                       __inline_isinff(y as libc::c_float)
                   } else if ::std::mem::size_of::<libc::c_double>() as
                                 libc::c_ulong ==
                                 ::std::mem::size_of::<libc::c_double>() as
                                     libc::c_ulong {
                       __inline_isinfd(y as libc::c_double)
                   } else { __inline_isinfl(f128::f128::new(y)) } {
            div = x
        } else {
            div = (x - mod_0) / y;
            if !modp.is_null() && !divp.is_null() { div = round(div) }
        }
        if y * mod_0 < 0i32 as libc::c_double { mod_0 += y; div -= 1.0f64 }
    }
    if !modp.is_null() { *modp = mod_0 }
    if !divp.is_null() { *divp = div };
}
/* 15.2.9.3.5  */
/*
 *  call-seq:
 *     flt % other        ->  float
 *     flt.modulo(other)  ->  float
 *
 *  Return the modulo after division of <code>flt</code> by <code>other</code>.
 *
 *     6543.21.modulo(137)      #=> 104.21
 *     6543.21.modulo(137.24)   #=> 92.9299999999996
 */
unsafe extern "C" fn flo_mod(mut mrb: *mut mrb_state, mut x: mrb_value)
 -> mrb_value {
    let mut y: mrb_value =
        mrb_value{value: unnamed{f: 0.,}, tt: MRB_TT_FALSE,};
    let mut mod_0: mrb_float = 0.;
    mrb_get_args(mrb, b"o\x00" as *const u8 as *const libc::c_char,
                 &mut y as *mut mrb_value);
    flodivmod(mrb, x.value.f, mrb_to_flo(mrb, y), 0 as *mut mrb_float,
              &mut mod_0);
    return mrb_float_value(mrb, mod_0);
}
/* 15.2.8.3.16 */
/*
 *  call-seq:
 *     num.eql?(numeric)  ->  true or false
 *
 *  Returns <code>true</code> if <i>num</i> and <i>numeric</i> are the
 *  same type and have equal values.
 *
 *     1 == 1.0          #=> true
 *     1.eql?(1.0)       #=> false
 *     (1.0).eql?(1.0)   #=> true
 */
unsafe extern "C" fn fix_eql(mut mrb: *mut mrb_state, mut x: mrb_value)
 -> mrb_value {
    let mut y: mrb_value =
        mrb_value{value: unnamed{f: 0.,}, tt: MRB_TT_FALSE,};
    mrb_get_args(mrb, b"o\x00" as *const u8 as *const libc::c_char,
                 &mut y as *mut mrb_value);
    if !(y.tt as libc::c_uint == MRB_TT_FIXNUM as libc::c_int as libc::c_uint)
       {
        return mrb_false_value()
    }
    return mrb_bool_value((x.value.i == y.value.i) as libc::c_int as
                              mrb_bool);
}
unsafe extern "C" fn flo_eql(mut mrb: *mut mrb_state, mut x: mrb_value)
 -> mrb_value {
    let mut y: mrb_value =
        mrb_value{value: unnamed{f: 0.,}, tt: MRB_TT_FALSE,};
    mrb_get_args(mrb, b"o\x00" as *const u8 as *const libc::c_char,
                 &mut y as *mut mrb_value);
    if !(y.tt as libc::c_uint == MRB_TT_FLOAT as libc::c_int as libc::c_uint)
       {
        return mrb_false_value()
    }
    return mrb_bool_value((x.value.f == y.value.f) as libc::c_int as
                              mrb_bool);
}
/* 15.2.9.3.7  */
/*
 *  call-seq:
 *     flt == obj  ->  true or false
 *
 *  Returns <code>true</code> only if <i>obj</i> has the same value
 *  as <i>flt</i>. Contrast this with <code>Float#eql?</code>, which
 *  requires <i>obj</i> to be a <code>Float</code>.
 *
 *     1.0 == 1   #=> true
 *
 */
unsafe extern "C" fn flo_eq(mut mrb: *mut mrb_state, mut x: mrb_value)
 -> mrb_value {
    let mut y: mrb_value =
        mrb_value{value: unnamed{f: 0.,}, tt: MRB_TT_FALSE,};
    mrb_get_args(mrb, b"o\x00" as *const u8 as *const libc::c_char,
                 &mut y as *mut mrb_value);
    match y.tt as libc::c_uint {
        3 => {
            return mrb_bool_value((x.value.f == y.value.i as mrb_float) as
                                      libc::c_int as mrb_bool)
        }
        6 => {
            return mrb_bool_value((x.value.f == y.value.f) as libc::c_int as
                                      mrb_bool)
        }
        _ => { return mrb_false_value() }
    };
}
unsafe extern "C" fn value_int64(mut mrb: *mut mrb_state, mut x: mrb_value)
 -> int64_t {
    match x.tt as libc::c_uint {
        3 => { return x.value.i as int64_t }
        6 => { return x.value.f as int64_t }
        _ => {
            mrb_raise(mrb,
                      mrb_exc_get(mrb,
                                  b"TypeError\x00" as *const u8 as
                                      *const libc::c_char),
                      b"cannot convert to Integer\x00" as *const u8 as
                          *const libc::c_char);
        }
    };
}
unsafe extern "C" fn int64_value(mut mrb: *mut mrb_state, mut v: int64_t)
 -> mrb_value {
    if v <= (9223372036854775807i64 >> 0i32) as int64_t &&
           v >=
               (-9223372036854775807i64 - 1i32 as libc::c_longlong >> 0i32) as
                   int64_t {
        return mrb_fixnum_value(v as mrb_int)
    }
    return mrb_float_value(mrb, v as mrb_float);
}
unsafe extern "C" fn flo_rev(mut mrb: *mut mrb_state, mut x: mrb_value)
 -> mrb_value {
    let mut v1: int64_t = 0;
    mrb_get_args(mrb, b"\x00" as *const u8 as *const libc::c_char);
    v1 = x.value.f as int64_t;
    return int64_value(mrb, !v1);
}
unsafe extern "C" fn flo_and(mut mrb: *mut mrb_state, mut x: mrb_value)
 -> mrb_value {
    let mut y: mrb_value =
        mrb_value{value: unnamed{f: 0.,}, tt: MRB_TT_FALSE,};
    let mut v1: int64_t = 0;
    let mut v2: int64_t = 0;
    mrb_get_args(mrb, b"o\x00" as *const u8 as *const libc::c_char,
                 &mut y as *mut mrb_value);
    v1 = x.value.f as int64_t;
    v2 = value_int64(mrb, y);
    return int64_value(mrb, v1 & v2);
}
unsafe extern "C" fn flo_or(mut mrb: *mut mrb_state, mut x: mrb_value)
 -> mrb_value {
    let mut y: mrb_value =
        mrb_value{value: unnamed{f: 0.,}, tt: MRB_TT_FALSE,};
    let mut v1: int64_t = 0;
    let mut v2: int64_t = 0;
    mrb_get_args(mrb, b"o\x00" as *const u8 as *const libc::c_char,
                 &mut y as *mut mrb_value);
    v1 = x.value.f as int64_t;
    v2 = value_int64(mrb, y);
    return int64_value(mrb, v1 | v2);
}
unsafe extern "C" fn flo_xor(mut mrb: *mut mrb_state, mut x: mrb_value)
 -> mrb_value {
    let mut y: mrb_value =
        mrb_value{value: unnamed{f: 0.,}, tt: MRB_TT_FALSE,};
    let mut v1: int64_t = 0;
    let mut v2: int64_t = 0;
    mrb_get_args(mrb, b"o\x00" as *const u8 as *const libc::c_char,
                 &mut y as *mut mrb_value);
    v1 = x.value.f as int64_t;
    v2 = value_int64(mrb, y);
    return int64_value(mrb, v1 ^ v2);
}
unsafe extern "C" fn flo_shift(mut mrb: *mut mrb_state, mut x: mrb_value,
                               mut width: mrb_int) -> mrb_value {
    let mut val: mrb_float = 0.;
    if width == 0i32 as libc::c_longlong { return x }
    val = x.value.f;
    if width < 0i32 as libc::c_longlong {
        loop  {
            let fresh6 = width;
            width = width + 1;
            if !(0 != fresh6) { break ; }
            val /= 2i32 as libc::c_double;
            if !(val < 1.0f64) { continue ; }
            val = 0i32 as mrb_float;
            break ;
        }
        if val > 0i32 as libc::c_double {
            val = floor(val)
        } else { val = ceil(val) }
        if val == 0i32 as libc::c_double && x.value.f < 0i32 as libc::c_double
           {
            return mrb_fixnum_value(-1i32 as mrb_int)
        }
    } else {
        loop  {
            let fresh7 = width;
            width = width - 1;
            if !(0 != fresh7) { break ; }
            val *= 2i32 as libc::c_double
        }
    }
    return mrb_int_value(mrb, val);
}
unsafe extern "C" fn flo_rshift(mut mrb: *mut mrb_state, mut x: mrb_value)
 -> mrb_value {
    let mut width: mrb_int = 0;
    mrb_get_args(mrb, b"i\x00" as *const u8 as *const libc::c_char,
                 &mut width as *mut mrb_int);
    return flo_shift(mrb, x, -width);
}
unsafe extern "C" fn flo_lshift(mut mrb: *mut mrb_state, mut x: mrb_value)
 -> mrb_value {
    let mut width: mrb_int = 0;
    mrb_get_args(mrb, b"i\x00" as *const u8 as *const libc::c_char,
                 &mut width as *mut mrb_int);
    return flo_shift(mrb, x, width);
}
/* 15.2.9.3.13 */
/*
 * call-seq:
 *   flt.to_f  ->  self
 *
 * As <code>flt</code> is already a float, returns +self+.
 */
unsafe extern "C" fn flo_to_f(mut mrb: *mut mrb_state, mut num: mrb_value)
 -> mrb_value {
    return num;
}
/* 15.2.9.3.11 */
/*
 *  call-seq:
 *     flt.infinite?  ->  nil, -1, +1
 *
 *  Returns <code>nil</code>, -1, or +1 depending on whether <i>flt</i>
 *  is finite, -infinity, or +infinity.
 *
 *     (0.0).infinite?        #=> nil
 *     (-1.0/0.0).infinite?   #=> -1
 *     (+1.0/0.0).infinite?   #=> 1
 */
unsafe extern "C" fn flo_infinite_p(mut mrb: *mut mrb_state,
                                    mut num: mrb_value) -> mrb_value {
    let mut value: mrb_float = num.value.f;
    if 0 !=
           if ::std::mem::size_of::<mrb_float>() as libc::c_ulong ==
                  ::std::mem::size_of::<libc::c_float>() as libc::c_ulong {
               __inline_isinff(value as libc::c_float)
           } else if ::std::mem::size_of::<mrb_float>() as libc::c_ulong ==
                         ::std::mem::size_of::<libc::c_double>() as
                             libc::c_ulong {
               __inline_isinfd(value as libc::c_double)
           } else { __inline_isinfl(f128::f128::new(value)) } {
        return mrb_fixnum_value((if value < 0i32 as libc::c_double {
                                     -1i32
                                 } else { 1i32 }) as mrb_int)
    }
    return mrb_nil_value();
}
/* 15.2.9.3.9  */
/*
 *  call-seq:
 *     flt.finite?  ->  true or false
 *
 *  Returns <code>true</code> if <i>flt</i> is a valid IEEE floating
 *  point number (it is not infinite, and <code>nan?</code> is
 *  <code>false</code>).
 *
 */
unsafe extern "C" fn flo_finite_p(mut mrb: *mut mrb_state, mut num: mrb_value)
 -> mrb_value {
    return mrb_bool_value((if ::std::mem::size_of::<mrb_float>() as
                                  libc::c_ulong ==
                                  ::std::mem::size_of::<libc::c_float>() as
                                      libc::c_ulong {
                               __inline_isfinitef(num.value.f as
                                                      libc::c_float)
                           } else if ::std::mem::size_of::<mrb_float>() as
                                         libc::c_ulong ==
                                         ::std::mem::size_of::<libc::c_double>()
                                             as libc::c_ulong {
                               __inline_isfinited(num.value.f as
                                                      libc::c_double)
                           } else {
                               __inline_isfinitel(f128::f128::new(num.value.f))
                           }) as mrb_bool);
}
/* 15.2.9.3.10 */
/*
 *  call-seq:
 *     flt.floor  ->  integer
 *
 *  Returns the largest integer less than or equal to <i>flt</i>.
 *
 *     1.2.floor      #=> 1
 *     2.0.floor      #=> 2
 *     (-1.2).floor   #=> -2
 *     (-2.0).floor   #=> -2
 */
unsafe extern "C" fn flo_floor(mut mrb: *mut mrb_state, mut num: mrb_value)
 -> mrb_value {
    let mut f: mrb_float = floor(num.value.f);
    mrb_check_num_exact(mrb, f);
    return mrb_int_value(mrb, f);
}
/* 15.2.9.3.8  */
/*
 *  call-seq:
 *     flt.ceil  ->  integer
 *
 *  Returns the smallest <code>Integer</code> greater than or equal to
 *  <i>flt</i>.
 *
 *     1.2.ceil      #=> 2
 *     2.0.ceil      #=> 2
 *     (-1.2).ceil   #=> -1
 *     (-2.0).ceil   #=> -2
 */
unsafe extern "C" fn flo_ceil(mut mrb: *mut mrb_state, mut num: mrb_value)
 -> mrb_value {
    let mut f: mrb_float = ceil(num.value.f);
    mrb_check_num_exact(mrb, f);
    return mrb_int_value(mrb, f);
}
/* 15.2.9.3.12 */
/*
 *  call-seq:
 *     flt.round([ndigits])  ->  integer or float
 *
 *  Rounds <i>flt</i> to a given precision in decimal digits (default 0 digits).
 *  Precision may be negative.  Returns a floating point number when ndigits
 *  is more than zero.
 *
 *     1.4.round      #=> 1
 *     1.5.round      #=> 2
 *     1.6.round      #=> 2
 *     (-1.5).round   #=> -2
 *
 *     1.234567.round(2)  #=> 1.23
 *     1.234567.round(3)  #=> 1.235
 *     1.234567.round(4)  #=> 1.2346
 *     1.234567.round(5)  #=> 1.23457
 *
 *     34567.89.round(-5) #=> 0
 *     34567.89.round(-4) #=> 30000
 *     34567.89.round(-3) #=> 35000
 *     34567.89.round(-2) #=> 34600
 *     34567.89.round(-1) #=> 34570
 *     34567.89.round(0)  #=> 34568
 *     34567.89.round(1)  #=> 34567.9
 *     34567.89.round(2)  #=> 34567.89
 *     34567.89.round(3)  #=> 34567.89
 *
 */
unsafe extern "C" fn flo_round(mut mrb: *mut mrb_state, mut num: mrb_value)
 -> mrb_value {
    let mut number: libc::c_double = 0.;
    let mut f: libc::c_double = 0.;
    let mut ndigits: mrb_int = 0i32 as mrb_int;
    let mut i: mrb_int = 0;
    mrb_get_args(mrb, b"|i\x00" as *const u8 as *const libc::c_char,
                 &mut ndigits as *mut mrb_int);
    number = num.value.f;
    if (0i32 as libc::c_longlong) < ndigits &&
           (0 !=
                if ::std::mem::size_of::<libc::c_double>() as libc::c_ulong ==
                       ::std::mem::size_of::<libc::c_float>() as libc::c_ulong
                   {
                    __inline_isinff(number as libc::c_float)
                } else if ::std::mem::size_of::<libc::c_double>() as
                              libc::c_ulong ==
                              ::std::mem::size_of::<libc::c_double>() as
                                  libc::c_ulong {
                    __inline_isinfd(number as libc::c_double)
                } else { __inline_isinfl(f128::f128::new(number)) } ||
                0 !=
                    if ::std::mem::size_of::<libc::c_double>() as
                           libc::c_ulong ==
                           ::std::mem::size_of::<libc::c_float>() as
                               libc::c_ulong {
                        __inline_isnanf(number as libc::c_float)
                    } else if ::std::mem::size_of::<libc::c_double>() as
                                  libc::c_ulong ==
                                  ::std::mem::size_of::<libc::c_double>() as
                                      libc::c_ulong {
                        __inline_isnand(number as libc::c_double)
                    } else { __inline_isnanl(f128::f128::new(number)) }) {
        return num
    }
    mrb_check_num_exact(mrb, number);
    f = 1.0f64;
    i = if ndigits >= 0i32 as libc::c_longlong { ndigits } else { -ndigits };
    loop  {
        i -= 1;
        if !(i >= 0i32 as libc::c_longlong) { break ; }
        f = f * 10.0f64
    }
    if 0 !=
           if ::std::mem::size_of::<libc::c_double>() as libc::c_ulong ==
                  ::std::mem::size_of::<libc::c_float>() as libc::c_ulong {
               __inline_isinff(f as libc::c_float)
           } else if ::std::mem::size_of::<libc::c_double>() as libc::c_ulong
                         ==
                         ::std::mem::size_of::<libc::c_double>() as
                             libc::c_ulong {
               __inline_isinfd(f as libc::c_double)
           } else { __inline_isinfl(f128::f128::new(f)) } {
        if ndigits < 0i32 as libc::c_longlong {
            number = 0i32 as libc::c_double
        }
    } else {
        let mut d: libc::c_double = 0.;
        if ndigits < 0i32 as libc::c_longlong {
            number /= f
        } else { number *= f }
        if number > 0.0f64 {
            d = floor(number);
            number =
                d + (number - d >= 0.5f64) as libc::c_int as libc::c_double
        } else if number < 0.0f64 {
            d = ceil(number);
            number =
                d - (d - number >= 0.5f64) as libc::c_int as libc::c_double
        }
        if ndigits < 0i32 as libc::c_longlong {
            number *= f
        } else { number /= f }
    }
    if ndigits > 0i32 as libc::c_longlong {
        if 0 ==
               if ::std::mem::size_of::<libc::c_double>() as libc::c_ulong ==
                      ::std::mem::size_of::<libc::c_float>() as libc::c_ulong
                  {
                   __inline_isfinitef(number as libc::c_float)
               } else if ::std::mem::size_of::<libc::c_double>() as
                             libc::c_ulong ==
                             ::std::mem::size_of::<libc::c_double>() as
                                 libc::c_ulong {
                   __inline_isfinited(number as libc::c_double)
               } else { __inline_isfinitel(f128::f128::new(number)) } {
            return num
        }
        return mrb_float_value(mrb, number)
    }
    return mrb_int_value(mrb, number);
}
/* 15.2.9.3.14 */
/* 15.2.9.3.15 */
/*
 *  call-seq:
 *     flt.to_i      ->  integer
 *     flt.truncate  ->  integer
 *
 *  Returns <i>flt</i> truncated to an <code>Integer</code>.
 */
unsafe extern "C" fn flo_truncate(mut mrb: *mut mrb_state, mut num: mrb_value)
 -> mrb_value {
    let mut f: mrb_float = num.value.f;
    if f > 0.0f64 { f = floor(f) }
    if f < 0.0f64 { f = ceil(f) }
    mrb_check_num_exact(mrb, f);
    return mrb_int_value(mrb, f);
}
unsafe extern "C" fn flo_nan_p(mut mrb: *mut mrb_state, mut num: mrb_value)
 -> mrb_value {
    return mrb_bool_value((if ::std::mem::size_of::<mrb_float>() as
                                  libc::c_ulong ==
                                  ::std::mem::size_of::<libc::c_float>() as
                                      libc::c_ulong {
                               __inline_isnanf(num.value.f as libc::c_float)
                           } else if ::std::mem::size_of::<mrb_float>() as
                                         libc::c_ulong ==
                                         ::std::mem::size_of::<libc::c_double>()
                                             as libc::c_ulong {
                               __inline_isnand(num.value.f as libc::c_double)
                           } else {
                               __inline_isnanl(f128::f128::new(num.value.f))
                           }) as mrb_bool);
}
/*
 * Document-class: Integer
 *
 *  <code>Integer</code> is the basis for the two concrete classes that
 *  hold whole numbers, <code>Bignum</code> and <code>Fixnum</code>.
 *
 */
/*
 *  call-seq:
 *     int.to_i      ->  integer
 *
 *  As <i>int</i> is already an <code>Integer</code>, all these
 *  methods simply return the receiver.
 */
unsafe extern "C" fn int_to_i(mut mrb: *mut mrb_state, mut num: mrb_value)
 -> mrb_value {
    return num;
}
/* 15.2.8.3.3  */
/*
 * call-seq:
 *   fix * numeric  ->  numeric_result
 *
 * Performs multiplication: the class of the resulting object depends on
 * the class of <code>numeric</code> and on the magnitude of the
 * result.
 */
unsafe extern "C" fn fix_mul(mut mrb: *mut mrb_state, mut x: mrb_value)
 -> mrb_value {
    let mut y: mrb_value =
        mrb_value{value: unnamed{f: 0.,}, tt: MRB_TT_FALSE,};
    mrb_get_args(mrb, b"o\x00" as *const u8 as *const libc::c_char,
                 &mut y as *mut mrb_value);
    return fixnum_mul(mrb, x, y);
}
unsafe extern "C" fn fixdivmod(mut mrb: *mut mrb_state, mut x: mrb_int,
                               mut y: mrb_int, mut divp: *mut mrb_int,
                               mut modp: *mut mrb_int) {
    let mut div: mrb_int = 0;
    let mut mod_0: mrb_int = 0;
    if y < 0i32 as libc::c_longlong {
        if x < 0i32 as libc::c_longlong {
            div = -x / -y
        } else { div = -(x / -y) }
    } else if x < 0i32 as libc::c_longlong {
        div = -(-x / y)
    } else { div = x / y }
    mod_0 = x - div * y;
    if mod_0 < 0i32 as libc::c_longlong && y > 0i32 as libc::c_longlong ||
           mod_0 > 0i32 as libc::c_longlong && y < 0i32 as libc::c_longlong {
        mod_0 += y;
        div -= 1i32 as libc::c_longlong
    }
    if !divp.is_null() { *divp = div }
    if !modp.is_null() { *modp = mod_0 };
}
/* 15.2.8.3.5  */
/*
 *  call-seq:
 *    fix % other        ->  real
 *    fix.modulo(other)  ->  real
 *
 *  Returns <code>fix</code> modulo <code>other</code>.
 *  See <code>numeric.divmod</code> for more information.
 */
unsafe extern "C" fn fix_mod(mut mrb: *mut mrb_state, mut x: mrb_value)
 -> mrb_value {
    let mut y: mrb_value =
        mrb_value{value: unnamed{f: 0.,}, tt: MRB_TT_FALSE,};
    let mut a: mrb_int = 0;
    mrb_get_args(mrb, b"o\x00" as *const u8 as *const libc::c_char,
                 &mut y as *mut mrb_value);
    a = x.value.i;
    if y.tt as libc::c_uint == MRB_TT_FIXNUM as libc::c_int as libc::c_uint {
        let mut b: mrb_int = 0;
        let mut mod_0: mrb_int = 0;
        b = y.value.i;
        if b == 0i32 as libc::c_longlong {
            return mrb_float_value(mrb, ::std::f32::NAN as mrb_float)
        }
        fixdivmod(mrb, a, b, 0 as *mut mrb_int, &mut mod_0);
        return mrb_fixnum_value(mod_0)
    } else {
        let mut mod_1: mrb_float = 0.;
        flodivmod(mrb, a as mrb_float, mrb_to_flo(mrb, y),
                  0 as *mut mrb_float, &mut mod_1);
        return mrb_float_value(mrb, mod_1)
    };
}
/*
 *  call-seq:
 *     fix.divmod(numeric)  ->  array
 *
 *  See <code>Numeric#divmod</code>.
 */
unsafe extern "C" fn fix_divmod(mut mrb: *mut mrb_state, mut x: mrb_value)
 -> mrb_value {
    let mut y: mrb_value =
        mrb_value{value: unnamed{f: 0.,}, tt: MRB_TT_FALSE,};
    mrb_get_args(mrb, b"o\x00" as *const u8 as *const libc::c_char,
                 &mut y as *mut mrb_value);
    if y.tt as libc::c_uint == MRB_TT_FIXNUM as libc::c_int as libc::c_uint {
        let mut div: mrb_int = 0;
        let mut mod_0: mrb_int = 0;
        if y.value.i == 0i32 as libc::c_longlong {
            return mrb_assoc_new(mrb,
                                 if x.value.i == 0i32 as libc::c_longlong {
                                     mrb_float_value(mrb,
                                                     ::std::f32::NAN as
                                                         mrb_float)
                                 } else {
                                     mrb_float_value(mrb,
                                                     ::std::f32::INFINITY as
                                                         mrb_float)
                                 },
                                 mrb_float_value(mrb,
                                                 ::std::f32::NAN as
                                                     mrb_float))
        }
        fixdivmod(mrb, x.value.i, y.value.i, &mut div, &mut mod_0);
        return mrb_assoc_new(mrb, mrb_fixnum_value(div),
                             mrb_fixnum_value(mod_0))
    } else {
        let mut div_0: mrb_float = 0.;
        let mut mod_1: mrb_float = 0.;
        let mut a: mrb_value =
            mrb_value{value: unnamed{f: 0.,}, tt: MRB_TT_FALSE,};
        let mut b: mrb_value =
            mrb_value{value: unnamed{f: 0.,}, tt: MRB_TT_FALSE,};
        flodivmod(mrb, x.value.i as mrb_float, mrb_to_flo(mrb, y), &mut div_0,
                  &mut mod_1);
        a = mrb_int_value(mrb, div_0);
        b = mrb_float_value(mrb, mod_1);
        return mrb_assoc_new(mrb, a, b)
    };
}
unsafe extern "C" fn flo_divmod(mut mrb: *mut mrb_state, mut x: mrb_value)
 -> mrb_value {
    let mut y: mrb_value =
        mrb_value{value: unnamed{f: 0.,}, tt: MRB_TT_FALSE,};
    let mut div: mrb_float = 0.;
    let mut mod_0: mrb_float = 0.;
    let mut a: mrb_value =
        mrb_value{value: unnamed{f: 0.,}, tt: MRB_TT_FALSE,};
    let mut b: mrb_value =
        mrb_value{value: unnamed{f: 0.,}, tt: MRB_TT_FALSE,};
    mrb_get_args(mrb, b"o\x00" as *const u8 as *const libc::c_char,
                 &mut y as *mut mrb_value);
    flodivmod(mrb, x.value.f, mrb_to_flo(mrb, y), &mut div, &mut mod_0);
    a = mrb_int_value(mrb, div);
    b = mrb_float_value(mrb, mod_0);
    return mrb_assoc_new(mrb, a, b);
}
/* 15.2.8.3.7  */
/*
 * call-seq:
 *   fix == other  ->  true or false
 *
 * Return <code>true</code> if <code>fix</code> equals <code>other</code>
 * numerically.
 *
 *   1 == 2      #=> false
 *   1 == 1.0    #=> true
 */
unsafe extern "C" fn fix_equal(mut mrb: *mut mrb_state, mut x: mrb_value)
 -> mrb_value {
    let mut y: mrb_value =
        mrb_value{value: unnamed{f: 0.,}, tt: MRB_TT_FALSE,};
    mrb_get_args(mrb, b"o\x00" as *const u8 as *const libc::c_char,
                 &mut y as *mut mrb_value);
    match y.tt as libc::c_uint {
        3 => {
            return mrb_bool_value((x.value.i == y.value.i) as libc::c_int as
                                      mrb_bool)
        }
        6 => {
            return mrb_bool_value((x.value.i as mrb_float == y.value.f) as
                                      libc::c_int as mrb_bool)
        }
        _ => { return mrb_false_value() }
    };
}
/* 15.2.8.3.8  */
/*
 * call-seq:
 *   ~fix  ->  integer
 *
 * One's complement: returns a number where each bit is flipped.
 *   ex.0---00001 (1)-> 1---11110 (-2)
 *   ex.0---00010 (2)-> 1---11101 (-3)
 *   ex.0---00100 (4)-> 1---11011 (-5)
 */
unsafe extern "C" fn fix_rev(mut mrb: *mut mrb_state, mut num: mrb_value)
 -> mrb_value {
    let mut val: mrb_int = num.value.i;
    return mrb_fixnum_value(!val);
}
/* 15.2.8.3.9  */
/*
 * call-seq:
 *   fix & integer  ->  integer_result
 *
 * Bitwise AND.
 */
unsafe extern "C" fn fix_and(mut mrb: *mut mrb_state, mut x: mrb_value)
 -> mrb_value {
    let mut y: mrb_value =
        mrb_value{value: unnamed{f: 0.,}, tt: MRB_TT_FALSE,};
    mrb_get_args(mrb, b"o\x00" as *const u8 as *const libc::c_char,
                 &mut y as *mut mrb_value);
    if y.tt as libc::c_uint == MRB_TT_FIXNUM as libc::c_int as libc::c_uint {
        return mrb_fixnum_value(x.value.i & y.value.i)
    }
    return flo_and(mrb, mrb_float_value(mrb, x.value.i as mrb_float));
}
/* 15.2.8.3.10 */
/*
 * call-seq:
 *   fix | integer  ->  integer_result
 *
 * Bitwise OR.
 */
unsafe extern "C" fn fix_or(mut mrb: *mut mrb_state, mut x: mrb_value)
 -> mrb_value {
    let mut y: mrb_value =
        mrb_value{value: unnamed{f: 0.,}, tt: MRB_TT_FALSE,};
    mrb_get_args(mrb, b"o\x00" as *const u8 as *const libc::c_char,
                 &mut y as *mut mrb_value);
    if y.tt as libc::c_uint == MRB_TT_FIXNUM as libc::c_int as libc::c_uint {
        return mrb_fixnum_value(x.value.i | y.value.i)
    }
    return flo_or(mrb, mrb_float_value(mrb, x.value.i as mrb_float));
}
/* 15.2.8.3.11 */
/*
 * call-seq:
 *   fix ^ integer  ->  integer_result
 *
 * Bitwise EXCLUSIVE OR.
 */
unsafe extern "C" fn fix_xor(mut mrb: *mut mrb_state, mut x: mrb_value)
 -> mrb_value {
    let mut y: mrb_value =
        mrb_value{value: unnamed{f: 0.,}, tt: MRB_TT_FALSE,};
    mrb_get_args(mrb, b"o\x00" as *const u8 as *const libc::c_char,
                 &mut y as *mut mrb_value);
    if y.tt as libc::c_uint == MRB_TT_FIXNUM as libc::c_int as libc::c_uint {
        return mrb_fixnum_value(x.value.i ^ y.value.i)
    }
    return flo_or(mrb, mrb_float_value(mrb, x.value.i as mrb_float));
}
unsafe extern "C" fn lshift(mut mrb: *mut mrb_state, mut val: mrb_int,
                            mut width: mrb_int) -> mrb_value {
    if width < 0i32 as libc::c_longlong {
        return mrb_float_value(mrb, ::std::f32::INFINITY as mrb_float)
    }
    if val > 0i32 as libc::c_longlong {
        if !(width > (64i32 - 1i32) as libc::c_longlong ||
                 val > 9223372036854775807i64 >> 0i32 >> width) {
            return mrb_fixnum_value(val << width)
        }
    } else if !(width > (64i32 - 1i32) as libc::c_longlong ||
                    val <
                        -9223372036854775807i64 - 1i32 as libc::c_longlong >>
                            0i32 >> width) {
        return mrb_fixnum_value(val * ((1i32 as mrb_int) << width))
    }
    let mut f: mrb_float = val as mrb_float;
    loop  {
        let fresh8 = width;
        width = width - 1;
        if !(0 != fresh8) { break ; }
        f *= 2i32 as libc::c_double
    }
    return mrb_float_value(mrb, f);
}
unsafe extern "C" fn rshift(mut val: mrb_int, mut width: mrb_int)
 -> mrb_value {
    if width < 0i32 as libc::c_longlong {
        return mrb_fixnum_value(0i32 as mrb_int)
    }
    if width >= (64i32 - 1i32) as libc::c_longlong {
        if val < 0i32 as libc::c_longlong {
            return mrb_fixnum_value(-1i32 as mrb_int)
        }
        return mrb_fixnum_value(0i32 as mrb_int)
    }
    return mrb_fixnum_value(val >> width);
}
/* 15.2.8.3.12 */
/*
 * call-seq:
 *   fix << count  ->  integer or float
 *
 * Shifts _fix_ left _count_ positions (right if _count_ is negative).
 */
unsafe extern "C" fn fix_lshift(mut mrb: *mut mrb_state, mut x: mrb_value)
 -> mrb_value {
    let mut width: mrb_int = 0;
    let mut val: mrb_int = 0;
    mrb_get_args(mrb, b"i\x00" as *const u8 as *const libc::c_char,
                 &mut width as *mut mrb_int);
    if width == 0i32 as libc::c_longlong { return x }
    val = x.value.i;
    if val == 0i32 as libc::c_longlong { return x }
    if width < 0i32 as libc::c_longlong { return rshift(val, -width) }
    return lshift(mrb, val, width);
}
/* 15.2.8.3.13 */
/*
 * call-seq:
 *   fix >> count  ->  integer or float
 *
 * Shifts _fix_ right _count_ positions (left if _count_ is negative).
 */
unsafe extern "C" fn fix_rshift(mut mrb: *mut mrb_state, mut x: mrb_value)
 -> mrb_value {
    let mut width: mrb_int = 0;
    let mut val: mrb_int = 0;
    mrb_get_args(mrb, b"i\x00" as *const u8 as *const libc::c_char,
                 &mut width as *mut mrb_int);
    if width == 0i32 as libc::c_longlong { return x }
    val = x.value.i;
    if val == 0i32 as libc::c_longlong { return x }
    if width < 0i32 as libc::c_longlong { return lshift(mrb, val, -width) }
    return rshift(val, width);
}
/* 15.2.8.3.23 */
/*
 *  call-seq:
 *     fix.to_f  ->  float
 *
 *  Converts <i>fix</i> to a <code>Float</code>.
 *
 */
unsafe extern "C" fn fix_to_f(mut mrb: *mut mrb_state, mut num: mrb_value)
 -> mrb_value {
    return mrb_float_value(mrb, num.value.i as mrb_float);
}
/* 15.2.8.3.1  */
/*
 * call-seq:
 *   fix + numeric  ->  numeric_result
 *
 * Performs addition: the class of the resulting object depends on
 * the class of <code>numeric</code> and on the magnitude of the
 * result.
 */
unsafe extern "C" fn fix_plus(mut mrb: *mut mrb_state, mut self_0: mrb_value)
 -> mrb_value {
    let mut other: mrb_value =
        mrb_value{value: unnamed{f: 0.,}, tt: MRB_TT_FALSE,};
    mrb_get_args(mrb, b"o\x00" as *const u8 as *const libc::c_char,
                 &mut other as *mut mrb_value);
    return fixnum_plus(mrb, self_0, other);
}
/* 15.2.8.3.2  */
/* 15.2.8.3.16 */
/*
 * call-seq:
 *   fix - numeric  ->  numeric_result
 *
 * Performs subtraction: the class of the resulting object depends on
 * the class of <code>numeric</code> and on the magnitude of the
 * result.
 */
unsafe extern "C" fn fix_minus(mut mrb: *mut mrb_state, mut self_0: mrb_value)
 -> mrb_value {
    let mut other: mrb_value =
        mrb_value{value: unnamed{f: 0.,}, tt: MRB_TT_FALSE,};
    mrb_get_args(mrb, b"o\x00" as *const u8 as *const libc::c_char,
                 &mut other as *mut mrb_value);
    return fixnum_minus(mrb, self_0, other);
}
/* 15.2.8.3.25 */
/*
 *  call-seq:
 *     fix.to_s(base=10)  ->  string
 *
 *  Returns a string containing the representation of <i>fix</i> radix
 *  <i>base</i> (between 2 and 36).
 *
 *     12345.to_s       #=> "12345"
 *     12345.to_s(2)    #=> "11000000111001"
 *     12345.to_s(8)    #=> "30071"
 *     12345.to_s(10)   #=> "12345"
 *     12345.to_s(16)   #=> "3039"
 *     12345.to_s(36)   #=> "9ix"
 *
 */
unsafe extern "C" fn fix_to_s(mut mrb: *mut mrb_state, mut self_0: mrb_value)
 -> mrb_value {
    let mut base: mrb_int = 10i32 as mrb_int;
    mrb_get_args(mrb, b"|i\x00" as *const u8 as *const libc::c_char,
                 &mut base as *mut mrb_int);
    return mrb_fixnum_to_str(mrb, self_0, base);
}
/* compare two numbers: (1:0:-1; -2 for error) */
unsafe extern "C" fn cmpnum(mut mrb: *mut mrb_state, mut v1: mrb_value,
                            mut v2: mrb_value) -> mrb_int {
    let mut x: mrb_float = 0.;
    let mut y: mrb_float = 0.;
    x = mrb_to_flo(mrb, v1);
    match v2.tt as libc::c_uint {
        3 => { y = v2.value.i as mrb_float }
        6 => { y = v2.value.f }
        _ => { return -2i32 as mrb_int }
    }
    if x > y {
        return 1i32 as mrb_int
    } else { if x < y { return -1i32 as mrb_int } return 0i32 as mrb_int };
}
/* 15.2.9.3.6  */
/*
 * call-seq:
 *     self.f <=> other.f    => -1, 0, +1
 *             <  => -1
 *             =  =>  0
 *             >  => +1
 *  Comparison---Returns -1, 0, or +1 depending on whether <i>fix</i> is
 *  less than, equal to, or greater than <i>numeric</i>. This is the
 *  basis for the tests in <code>Comparable</code>.
 */
unsafe extern "C" fn integral_cmp(mut mrb: *mut mrb_state,
                                  mut self_0: mrb_value) -> mrb_value {
    let mut other: mrb_value =
        mrb_value{value: unnamed{f: 0.,}, tt: MRB_TT_FALSE,};
    let mut n: mrb_int = 0;
    mrb_get_args(mrb, b"o\x00" as *const u8 as *const libc::c_char,
                 &mut other as *mut mrb_value);
    n = cmpnum(mrb, self_0, other);
    if n == -2i32 as libc::c_longlong { return mrb_nil_value() }
    return mrb_fixnum_value(n);
}
unsafe extern "C" fn cmperr(mut mrb: *mut mrb_state, mut v1: mrb_value,
                            mut v2: mrb_value) {
    mrb_raisef(mrb,
               mrb_exc_get(mrb,
                           b"ArgumentError\x00" as *const u8 as
                               *const libc::c_char),
               b"comparison of %S with %S failed\x00" as *const u8 as
                   *const libc::c_char,
               mrb_obj_value(mrb_class(mrb, v1) as *mut libc::c_void),
               mrb_obj_value(mrb_class(mrb, v2) as *mut libc::c_void));
}
unsafe extern "C" fn integral_lt(mut mrb: *mut mrb_state,
                                 mut self_0: mrb_value) -> mrb_value {
    let mut other: mrb_value =
        mrb_value{value: unnamed{f: 0.,}, tt: MRB_TT_FALSE,};
    let mut n: mrb_int = 0;
    mrb_get_args(mrb, b"o\x00" as *const u8 as *const libc::c_char,
                 &mut other as *mut mrb_value);
    n = cmpnum(mrb, self_0, other);
    if n == -2i32 as libc::c_longlong { cmperr(mrb, self_0, other); }
    if n < 0i32 as libc::c_longlong { return mrb_true_value() }
    return mrb_false_value();
}
unsafe extern "C" fn integral_le(mut mrb: *mut mrb_state,
                                 mut self_0: mrb_value) -> mrb_value {
    let mut other: mrb_value =
        mrb_value{value: unnamed{f: 0.,}, tt: MRB_TT_FALSE,};
    let mut n: mrb_int = 0;
    mrb_get_args(mrb, b"o\x00" as *const u8 as *const libc::c_char,
                 &mut other as *mut mrb_value);
    n = cmpnum(mrb, self_0, other);
    if n == -2i32 as libc::c_longlong { cmperr(mrb, self_0, other); }
    if n <= 0i32 as libc::c_longlong { return mrb_true_value() }
    return mrb_false_value();
}
unsafe extern "C" fn integral_gt(mut mrb: *mut mrb_state,
                                 mut self_0: mrb_value) -> mrb_value {
    let mut other: mrb_value =
        mrb_value{value: unnamed{f: 0.,}, tt: MRB_TT_FALSE,};
    let mut n: mrb_int = 0;
    mrb_get_args(mrb, b"o\x00" as *const u8 as *const libc::c_char,
                 &mut other as *mut mrb_value);
    n = cmpnum(mrb, self_0, other);
    if n == -2i32 as libc::c_longlong { cmperr(mrb, self_0, other); }
    if n > 0i32 as libc::c_longlong { return mrb_true_value() }
    return mrb_false_value();
}
unsafe extern "C" fn integral_ge(mut mrb: *mut mrb_state,
                                 mut self_0: mrb_value) -> mrb_value {
    let mut other: mrb_value =
        mrb_value{value: unnamed{f: 0.,}, tt: MRB_TT_FALSE,};
    let mut n: mrb_int = 0;
    mrb_get_args(mrb, b"o\x00" as *const u8 as *const libc::c_char,
                 &mut other as *mut mrb_value);
    n = cmpnum(mrb, self_0, other);
    if n == -2i32 as libc::c_longlong { cmperr(mrb, self_0, other); }
    if n >= 0i32 as libc::c_longlong { return mrb_true_value() }
    return mrb_false_value();
}
unsafe extern "C" fn num_finite_p(mut mrb: *mut mrb_state,
                                  mut self_0: mrb_value) -> mrb_value {
    mrb_get_args(mrb, b"\x00" as *const u8 as *const libc::c_char);
    return mrb_true_value();
}
unsafe extern "C" fn num_infinite_p(mut mrb: *mut mrb_state,
                                    mut self_0: mrb_value) -> mrb_value {
    mrb_get_args(mrb, b"\x00" as *const u8 as *const libc::c_char);
    return mrb_false_value();
}
/* 15.2.9.3.1  */
/*
 * call-seq:
 *   float + other  ->  float
 *
 * Returns a new float which is the sum of <code>float</code>
 * and <code>other</code>.
 */
unsafe extern "C" fn flo_plus(mut mrb: *mut mrb_state, mut x: mrb_value)
 -> mrb_value {
    let mut y: mrb_value =
        mrb_value{value: unnamed{f: 0.,}, tt: MRB_TT_FALSE,};
    mrb_get_args(mrb, b"o\x00" as *const u8 as *const libc::c_char,
                 &mut y as *mut mrb_value);
    return mrb_float_value(mrb, x.value.f + mrb_to_flo(mrb, y));
}
/* ------------------------------------------------------------------------*/
#[no_mangle]
pub unsafe extern "C" fn mrb_init_numeric(mut mrb: *mut mrb_state) {
    let mut numeric: *mut RClass = 0 as *mut RClass;
    let mut integer: *mut RClass = 0 as *mut RClass;
    let mut fixnum: *mut RClass = 0 as *mut RClass;
    let mut integral: *mut RClass = 0 as *mut RClass;
    let mut fl: *mut RClass = 0 as *mut RClass;
    integral =
        mrb_define_module(mrb,
                          b"Integral\x00" as *const u8 as
                              *const libc::c_char);
    mrb_define_method(mrb, integral,
                      b"**\x00" as *const u8 as *const libc::c_char,
                      Some(integral_pow),
                      ((1i32 & 0x1fi32) as mrb_aspec) << 18i32);
    mrb_define_method(mrb, integral,
                      b"/\x00" as *const u8 as *const libc::c_char,
                      Some(integral_div),
                      ((1i32 & 0x1fi32) as mrb_aspec) << 18i32);
    mrb_define_method(mrb, integral,
                      b"quo\x00" as *const u8 as *const libc::c_char,
                      Some(integral_div),
                      ((1i32 & 0x1fi32) as mrb_aspec) << 18i32);
    mrb_define_method(mrb, integral,
                      b"div\x00" as *const u8 as *const libc::c_char,
                      Some(integral_idiv),
                      ((1i32 & 0x1fi32) as mrb_aspec) << 18i32);
    mrb_define_method(mrb, integral,
                      b"<=>\x00" as *const u8 as *const libc::c_char,
                      Some(integral_cmp),
                      ((1i32 & 0x1fi32) as mrb_aspec) << 18i32);
    mrb_define_method(mrb, integral,
                      b"<\x00" as *const u8 as *const libc::c_char,
                      Some(integral_lt),
                      ((1i32 & 0x1fi32) as mrb_aspec) << 18i32);
    mrb_define_method(mrb, integral,
                      b"<=\x00" as *const u8 as *const libc::c_char,
                      Some(integral_le),
                      ((1i32 & 0x1fi32) as mrb_aspec) << 18i32);
    mrb_define_method(mrb, integral,
                      b">\x00" as *const u8 as *const libc::c_char,
                      Some(integral_gt),
                      ((1i32 & 0x1fi32) as mrb_aspec) << 18i32);
    mrb_define_method(mrb, integral,
                      b">=\x00" as *const u8 as *const libc::c_char,
                      Some(integral_ge),
                      ((1i32 & 0x1fi32) as mrb_aspec) << 18i32);
    mrb_define_method(mrb, integral,
                      b"__coerce_step_counter\x00" as *const u8 as
                          *const libc::c_char,
                      Some(integral_coerce_step_counter),
                      ((2i32 & 0x1fi32) as mrb_aspec) << 18i32);
    numeric =
        mrb_define_class(mrb,
                         b"Numeric\x00" as *const u8 as *const libc::c_char,
                         (*mrb).object_class);
    mrb_define_method(mrb, numeric,
                      b"finite?\x00" as *const u8 as *const libc::c_char,
                      Some(num_finite_p), 0i32 as mrb_aspec);
    mrb_define_method(mrb, numeric,
                      b"infinite?\x00" as *const u8 as *const libc::c_char,
                      Some(num_infinite_p), 0i32 as mrb_aspec);
    integer =
        mrb_define_class(mrb,
                         b"Integer\x00" as *const u8 as *const libc::c_char,
                         numeric);
    (*integer).set_flags(((*integer).flags() as libc::c_int & !0xffi32 |
                              MRB_TT_FIXNUM as libc::c_int as libc::c_char as
                                  libc::c_int) as uint32_t);
    mrb_undef_class_method(mrb, integer,
                           b"new\x00" as *const u8 as *const libc::c_char);
    mrb_define_method(mrb, integer,
                      b"to_i\x00" as *const u8 as *const libc::c_char,
                      Some(int_to_i), 0i32 as mrb_aspec);
    mrb_define_method(mrb, integer,
                      b"to_int\x00" as *const u8 as *const libc::c_char,
                      Some(int_to_i), 0i32 as mrb_aspec);
    mrb_define_method(mrb, integer,
                      b"ceil\x00" as *const u8 as *const libc::c_char,
                      Some(int_to_i),
                      ((1i32 & 0x1fi32) as mrb_aspec) << 18i32);
    mrb_define_method(mrb, integer,
                      b"floor\x00" as *const u8 as *const libc::c_char,
                      Some(int_to_i),
                      ((1i32 & 0x1fi32) as mrb_aspec) << 18i32);
    mrb_define_method(mrb, integer,
                      b"round\x00" as *const u8 as *const libc::c_char,
                      Some(int_to_i),
                      ((1i32 & 0x1fi32) as mrb_aspec) << 18i32);
    mrb_define_method(mrb, integer,
                      b"truncate\x00" as *const u8 as *const libc::c_char,
                      Some(int_to_i),
                      ((1i32 & 0x1fi32) as mrb_aspec) << 18i32);
    fixnum =
        mrb_define_class(mrb,
                         b"Fixnum\x00" as *const u8 as *const libc::c_char,
                         integer);
    (*mrb).fixnum_class = fixnum;
    mrb_define_method(mrb, fixnum,
                      b"+\x00" as *const u8 as *const libc::c_char,
                      Some(fix_plus),
                      ((1i32 & 0x1fi32) as mrb_aspec) << 18i32);
    mrb_define_method(mrb, fixnum,
                      b"-\x00" as *const u8 as *const libc::c_char,
                      Some(fix_minus),
                      ((1i32 & 0x1fi32) as mrb_aspec) << 18i32);
    mrb_define_method(mrb, fixnum,
                      b"*\x00" as *const u8 as *const libc::c_char,
                      Some(fix_mul),
                      ((1i32 & 0x1fi32) as mrb_aspec) << 18i32);
    mrb_define_method(mrb, fixnum,
                      b"%\x00" as *const u8 as *const libc::c_char,
                      Some(fix_mod),
                      ((1i32 & 0x1fi32) as mrb_aspec) << 18i32);
    mrb_define_method(mrb, fixnum,
                      b"==\x00" as *const u8 as *const libc::c_char,
                      Some(fix_equal),
                      ((1i32 & 0x1fi32) as mrb_aspec) << 18i32);
    mrb_define_method(mrb, fixnum,
                      b"~\x00" as *const u8 as *const libc::c_char,
                      Some(fix_rev), 0i32 as mrb_aspec);
    mrb_define_method(mrb, fixnum,
                      b"&\x00" as *const u8 as *const libc::c_char,
                      Some(fix_and),
                      ((1i32 & 0x1fi32) as mrb_aspec) << 18i32);
    mrb_define_method(mrb, fixnum,
                      b"|\x00" as *const u8 as *const libc::c_char,
                      Some(fix_or), ((1i32 & 0x1fi32) as mrb_aspec) << 18i32);
    mrb_define_method(mrb, fixnum,
                      b"^\x00" as *const u8 as *const libc::c_char,
                      Some(fix_xor),
                      ((1i32 & 0x1fi32) as mrb_aspec) << 18i32);
    mrb_define_method(mrb, fixnum,
                      b"<<\x00" as *const u8 as *const libc::c_char,
                      Some(fix_lshift),
                      ((1i32 & 0x1fi32) as mrb_aspec) << 18i32);
    mrb_define_method(mrb, fixnum,
                      b">>\x00" as *const u8 as *const libc::c_char,
                      Some(fix_rshift),
                      ((1i32 & 0x1fi32) as mrb_aspec) << 18i32);
    mrb_define_method(mrb, fixnum,
                      b"eql?\x00" as *const u8 as *const libc::c_char,
                      Some(fix_eql),
                      ((1i32 & 0x1fi32) as mrb_aspec) << 18i32);
    mrb_define_method(mrb, fixnum,
                      b"to_f\x00" as *const u8 as *const libc::c_char,
                      Some(fix_to_f), 0i32 as mrb_aspec);
    mrb_define_method(mrb, fixnum,
                      b"to_s\x00" as *const u8 as *const libc::c_char,
                      Some(fix_to_s), 0i32 as mrb_aspec);
    mrb_define_method(mrb, fixnum,
                      b"inspect\x00" as *const u8 as *const libc::c_char,
                      Some(fix_to_s), 0i32 as mrb_aspec);
    mrb_define_method(mrb, fixnum,
                      b"divmod\x00" as *const u8 as *const libc::c_char,
                      Some(fix_divmod),
                      ((1i32 & 0x1fi32) as mrb_aspec) << 18i32);
    fl =
        mrb_define_class(mrb,
                         b"Float\x00" as *const u8 as *const libc::c_char,
                         numeric);
    (*mrb).float_class = fl;
    (*fl).set_flags(((*fl).flags() as libc::c_int & !0xffi32 |
                         MRB_TT_FLOAT as libc::c_int as libc::c_char as
                             libc::c_int) as uint32_t);
    mrb_undef_class_method(mrb, fl,
                           b"new\x00" as *const u8 as *const libc::c_char);
    mrb_define_method(mrb, fl, b"+\x00" as *const u8 as *const libc::c_char,
                      Some(flo_plus),
                      ((1i32 & 0x1fi32) as mrb_aspec) << 18i32);
    mrb_define_method(mrb, fl, b"-\x00" as *const u8 as *const libc::c_char,
                      Some(flo_minus),
                      ((1i32 & 0x1fi32) as mrb_aspec) << 18i32);
    mrb_define_method(mrb, fl, b"*\x00" as *const u8 as *const libc::c_char,
                      Some(flo_mul),
                      ((1i32 & 0x1fi32) as mrb_aspec) << 18i32);
    mrb_define_method(mrb, fl, b"%\x00" as *const u8 as *const libc::c_char,
                      Some(flo_mod),
                      ((1i32 & 0x1fi32) as mrb_aspec) << 18i32);
    mrb_define_method(mrb, fl, b"==\x00" as *const u8 as *const libc::c_char,
                      Some(flo_eq), ((1i32 & 0x1fi32) as mrb_aspec) << 18i32);
    mrb_define_method(mrb, fl, b"~\x00" as *const u8 as *const libc::c_char,
                      Some(flo_rev), 0i32 as mrb_aspec);
    mrb_define_method(mrb, fl, b"&\x00" as *const u8 as *const libc::c_char,
                      Some(flo_and),
                      ((1i32 & 0x1fi32) as mrb_aspec) << 18i32);
    mrb_define_method(mrb, fl, b"|\x00" as *const u8 as *const libc::c_char,
                      Some(flo_or), ((1i32 & 0x1fi32) as mrb_aspec) << 18i32);
    mrb_define_method(mrb, fl, b"^\x00" as *const u8 as *const libc::c_char,
                      Some(flo_xor),
                      ((1i32 & 0x1fi32) as mrb_aspec) << 18i32);
    mrb_define_method(mrb, fl, b">>\x00" as *const u8 as *const libc::c_char,
                      Some(flo_rshift),
                      ((1i32 & 0x1fi32) as mrb_aspec) << 18i32);
    mrb_define_method(mrb, fl, b"<<\x00" as *const u8 as *const libc::c_char,
                      Some(flo_lshift),
                      ((1i32 & 0x1fi32) as mrb_aspec) << 18i32);
    mrb_define_method(mrb, fl,
                      b"ceil\x00" as *const u8 as *const libc::c_char,
                      Some(flo_ceil), 0i32 as mrb_aspec);
    mrb_define_method(mrb, fl,
                      b"finite?\x00" as *const u8 as *const libc::c_char,
                      Some(flo_finite_p), 0i32 as mrb_aspec);
    mrb_define_method(mrb, fl,
                      b"floor\x00" as *const u8 as *const libc::c_char,
                      Some(flo_floor), 0i32 as mrb_aspec);
    mrb_define_method(mrb, fl,
                      b"infinite?\x00" as *const u8 as *const libc::c_char,
                      Some(flo_infinite_p), 0i32 as mrb_aspec);
    mrb_define_method(mrb, fl,
                      b"round\x00" as *const u8 as *const libc::c_char,
                      Some(flo_round),
                      ((1i32 & 0x1fi32) as mrb_aspec) << 13i32);
    mrb_define_method(mrb, fl,
                      b"to_f\x00" as *const u8 as *const libc::c_char,
                      Some(flo_to_f), 0i32 as mrb_aspec);
    mrb_define_method(mrb, fl,
                      b"to_i\x00" as *const u8 as *const libc::c_char,
                      Some(flo_truncate), 0i32 as mrb_aspec);
    mrb_define_method(mrb, fl,
                      b"to_int\x00" as *const u8 as *const libc::c_char,
                      Some(flo_truncate), 0i32 as mrb_aspec);
    mrb_define_method(mrb, fl,
                      b"truncate\x00" as *const u8 as *const libc::c_char,
                      Some(flo_truncate), 0i32 as mrb_aspec);
    mrb_define_method(mrb, fl,
                      b"divmod\x00" as *const u8 as *const libc::c_char,
                      Some(flo_divmod),
                      ((1i32 & 0x1fi32) as mrb_aspec) << 18i32);
    mrb_define_method(mrb, fl,
                      b"eql?\x00" as *const u8 as *const libc::c_char,
                      Some(flo_eql),
                      ((1i32 & 0x1fi32) as mrb_aspec) << 18i32);
    mrb_define_method(mrb, fl,
                      b"to_s\x00" as *const u8 as *const libc::c_char,
                      Some(flo_to_s), 0i32 as mrb_aspec);
    mrb_define_method(mrb, fl,
                      b"inspect\x00" as *const u8 as *const libc::c_char,
                      Some(flo_to_s), 0i32 as mrb_aspec);
    mrb_define_method(mrb, fl,
                      b"nan?\x00" as *const u8 as *const libc::c_char,
                      Some(flo_nan_p), 0i32 as mrb_aspec);
    mrb_define_const(mrb, fl,
                     b"INFINITY\x00" as *const u8 as *const libc::c_char,
                     mrb_float_value(mrb, ::std::f32::INFINITY as mrb_float));
    mrb_define_const(mrb, fl, b"NAN\x00" as *const u8 as *const libc::c_char,
                     mrb_float_value(mrb, ::std::f32::NAN as mrb_float));
    mrb_include_module(mrb, fl, integral);
}