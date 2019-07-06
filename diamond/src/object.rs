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
    fn mrb_str_new(mrb: *mut mrb_state, p: *const libc::c_char, len: size_t)
     -> mrb_value;
    /* *
 * Turns a C string into a Ruby string value.
 */
    #[no_mangle]
    fn mrb_str_new_cstr(_: *mut mrb_state, _: *const libc::c_char)
     -> mrb_value;
    #[no_mangle]
    fn mrb_str_new_static(mrb: *mut mrb_state, p: *const libc::c_char,
                          len: size_t) -> mrb_value;
    #[no_mangle]
    fn mrb_raisef(mrb: *mut mrb_state, c: *mut RClass,
                  fmt: *const libc::c_char, _: ...) -> !;
    #[no_mangle]
    fn mrb_obj_classname(mrb: *mut mrb_state, obj: mrb_value)
     -> *const libc::c_char;
    /*
 * Returns an object as a Ruby string.
 *
 * @param [mrb_state] mrb The current mruby state.
 * @param [mrb_value] obj An object to return as a Ruby string.
 * @return [mrb_value] An object as a Ruby string.
 */
    #[no_mangle]
    fn mrb_obj_as_string(mrb: *mut mrb_state, obj: mrb_value) -> mrb_value;
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
    fn mrb_flo_to_fixnum(mrb: *mut mrb_state, val: mrb_value) -> mrb_value;
    #[no_mangle]
    fn mrb_raise(mrb: *mut mrb_state, c: *mut RClass,
                 msg: *const libc::c_char) -> !;
    #[no_mangle]
    fn mrb_str_to_inum(mrb: *mut mrb_state, str: mrb_value, base: mrb_int,
                       badcheck: mrb_bool) -> mrb_value;
    #[no_mangle]
    fn mrb_respond_to(mrb: *mut mrb_state, obj: mrb_value, mid: mrb_sym)
     -> mrb_bool;
    #[no_mangle]
    fn mrb_str_to_dbl(mrb: *mut mrb_state, str: mrb_value, badcheck: mrb_bool)
     -> libc::c_double;
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
    /*
 * Converts pointer into a Ruby string.
 *
 * @param [mrb_state] mrb The current mruby state.
 * @param [void*] p The pointer to convert to Ruby string.
 * @return [mrb_value] Returns a new Ruby String.
 */
    #[no_mangle]
    fn mrb_ptr_to_str(_: *mut mrb_state, _: *mut libc::c_void) -> mrb_value;
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
#[derive ( Copy , Clone )]
#[repr(C)]
pub struct types {
    pub type_0: libc::c_uchar,
    pub name: *const libc::c_char,
}
#[derive ( Copy , Clone )]
#[repr(C)]
pub struct unnamed_0 {
    pub len: mrb_int,
    pub aux: unnamed_1,
    pub ptr: *mut libc::c_char,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub union unnamed_1 {
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
    pub as_0: unnamed_2,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub union unnamed_2 {
    pub heap: unnamed_0,
    pub ary: [libc::c_char; 24],
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
#[no_mangle]
pub unsafe extern "C" fn mrb_obj_eq(mut mrb: *mut mrb_state,
                                    mut v1: mrb_value, mut v2: mrb_value)
 -> mrb_bool {
    if v1.tt as libc::c_uint != v2.tt as libc::c_uint {
        return 0i32 as mrb_bool
    }
    match v1.tt as libc::c_uint {
        2 => { return 1i32 as mrb_bool }
        0 | 3 => {
            return (v1.value.i == v2.value.i) as libc::c_int as mrb_bool
        }
        4 => {
            return (v1.value.sym == v2.value.sym) as libc::c_int as mrb_bool
        }
        6 => { return (v1.value.f == v2.value.f) as libc::c_int as mrb_bool }
        _ => { return (v1.value.p == v2.value.p) as libc::c_int as mrb_bool }
    };
}
#[no_mangle]
pub unsafe extern "C" fn mrb_obj_equal(mut mrb: *mut mrb_state,
                                       mut v1: mrb_value, mut v2: mrb_value)
 -> mrb_bool {
    return mrb_obj_eq(mrb, v1, v2);
}
#[no_mangle]
pub unsafe extern "C" fn mrb_equal(mut mrb: *mut mrb_state,
                                   mut obj1: mrb_value, mut obj2: mrb_value)
 -> mrb_bool {
    let mut result: mrb_value =
        mrb_value{value: unnamed{f: 0.,}, tt: MRB_TT_FALSE,};
    if 0 != mrb_obj_eq(mrb, obj1, obj2) { return 1i32 as mrb_bool }
    result =
        mrb_funcall(mrb, obj1, b"==\x00" as *const u8 as *const libc::c_char,
                    1i32 as mrb_int, obj2);
    if result.tt as libc::c_uint !=
           MRB_TT_FALSE as libc::c_int as libc::c_uint {
        return 1i32 as mrb_bool
    }
    return 0i32 as mrb_bool;
}
#[no_mangle]
pub unsafe extern "C" fn mrb_convert_to_integer(mut mrb: *mut mrb_state,
                                                mut val: mrb_value,
                                                mut base: mrb_int)
 -> mrb_value {
    let mut current_block: u64;
    let mut tmp: mrb_value =
        mrb_value{value: unnamed{f: 0.,}, tt: MRB_TT_FALSE,};
    if val.tt as libc::c_uint == MRB_TT_FALSE as libc::c_int as libc::c_uint
           && 0 == val.value.i {
        if !(base != 0i32 as libc::c_longlong) {
            mrb_raise(mrb,
                      mrb_exc_get(mrb,
                                  b"TypeError\x00" as *const u8 as
                                      *const libc::c_char),
                      b"can\'t convert nil into Integer\x00" as *const u8 as
                          *const libc::c_char);
        }
    } else {
        match val.tt as libc::c_uint {
            6 => {
                if base != 0i32 as libc::c_longlong {
                    current_block = 12478305818333681747;
                } else { return mrb_flo_to_fixnum(mrb, val) }
            }
            3 => {
                if base != 0i32 as libc::c_longlong {
                    current_block = 12478305818333681747;
                } else { return val }
            }
            16 => { current_block = 6897396512028333200; }
            _ => {
                if base != 0i32 as libc::c_longlong {
                    tmp = mrb_check_string_type(mrb, val);
                    if !(tmp.tt as libc::c_uint ==
                             MRB_TT_FALSE as libc::c_int as libc::c_uint &&
                             0 == tmp.value.i) {
                        val = tmp;
                        current_block = 6897396512028333200;
                    } else { current_block = 12478305818333681747; }
                } else { return mrb_to_int(mrb, val) }
            }
        }
        match current_block {
            12478305818333681747 => { }
            _ => { return mrb_str_to_inum(mrb, val, base, 1i32 as mrb_bool) }
        }
    }
    mrb_raise(mrb,
              mrb_exc_get(mrb,
                          b"ArgumentError\x00" as *const u8 as
                              *const libc::c_char),
              b"base specified for non string value\x00" as *const u8 as
                  *const libc::c_char);
}
#[no_mangle]
pub unsafe extern "C" fn mrb_to_int(mut mrb: *mut mrb_state,
                                    mut val: mrb_value) -> mrb_value {
    if !(val.tt as libc::c_uint ==
             MRB_TT_FIXNUM as libc::c_int as libc::c_uint) {
        let mut type_0: mrb_value =
            mrb_value{value: unnamed{f: 0.,}, tt: MRB_TT_FALSE,};
        if val.tt as libc::c_uint ==
               MRB_TT_FLOAT as libc::c_int as libc::c_uint {
            return mrb_flo_to_fixnum(mrb, val)
        }
        type_0 = inspect_type(mrb, val);
        mrb_raisef(mrb,
                   mrb_exc_get(mrb,
                               b"TypeError\x00" as *const u8 as
                                   *const libc::c_char),
                   b"can\'t convert %S to Integer\x00" as *const u8 as
                       *const libc::c_char, type_0);
    }
    return val;
}
unsafe extern "C" fn inspect_type(mut mrb: *mut mrb_state, mut val: mrb_value)
 -> mrb_value {
    if val.tt as libc::c_uint == MRB_TT_FALSE as libc::c_int as libc::c_uint
           ||
           val.tt as libc::c_uint ==
               MRB_TT_TRUE as libc::c_int as libc::c_uint {
        return mrb_inspect(mrb, val)
    } else { return mrb_str_new_cstr(mrb, mrb_obj_classname(mrb, val)) };
}
#[no_mangle]
pub unsafe extern "C" fn mrb_inspect(mut mrb: *mut mrb_state,
                                     mut obj: mrb_value) -> mrb_value {
    return mrb_obj_as_string(mrb,
                             mrb_funcall(mrb, obj,
                                         b"inspect\x00" as *const u8 as
                                             *const libc::c_char,
                                         0i32 as mrb_int));
}
#[no_mangle]
pub unsafe extern "C" fn mrb_check_string_type(mut mrb: *mut mrb_state,
                                               mut str: mrb_value)
 -> mrb_value {
    if !(str.tt as libc::c_uint ==
             MRB_TT_STRING as libc::c_int as libc::c_uint) {
        return mrb_nil_value()
    }
    return str;
}
#[no_mangle]
pub unsafe extern "C" fn mrb_Integer(mut mrb: *mut mrb_state,
                                     mut val: mrb_value) -> mrb_value {
    return mrb_convert_to_integer(mrb, val, 0i32 as mrb_int);
}
#[no_mangle]
pub unsafe extern "C" fn mrb_Float(mut mrb: *mut mrb_state,
                                   mut val: mrb_value) -> mrb_value {
    if val.tt as libc::c_uint == MRB_TT_FALSE as libc::c_int as libc::c_uint
           && 0 == val.value.i {
        mrb_raise(mrb,
                  mrb_exc_get(mrb,
                              b"TypeError\x00" as *const u8 as
                                  *const libc::c_char),
                  b"can\'t convert nil into Float\x00" as *const u8 as
                      *const libc::c_char);
    }
    match val.tt as libc::c_uint {
        3 => { return mrb_float_value(mrb, val.value.i as mrb_float) }
        6 => { return val }
        16 => {
            return mrb_float_value(mrb,
                                   mrb_str_to_dbl(mrb, val, 1i32 as mrb_bool))
        }
        _ => {
            return mrb_convert_type(mrb, val, MRB_TT_FLOAT,
                                    b"Float\x00" as *const u8 as
                                        *const libc::c_char,
                                    b"to_f\x00" as *const u8 as
                                        *const libc::c_char)
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn mrb_convert_type(mut mrb: *mut mrb_state,
                                          mut val: mrb_value,
                                          mut type_0: mrb_vtype,
                                          mut tname: *const libc::c_char,
                                          mut method: *const libc::c_char)
 -> mrb_value {
    let mut v: mrb_value =
        mrb_value{value: unnamed{f: 0.,}, tt: MRB_TT_FALSE,};
    if val.tt as libc::c_uint == type_0 as libc::c_uint { return val }
    v = convert_type(mrb, val, tname, method, 1i32 as mrb_bool);
    if v.tt as libc::c_uint != type_0 as libc::c_uint {
        mrb_raisef(mrb,
                   mrb_exc_get(mrb,
                               b"TypeError\x00" as *const u8 as
                                   *const libc::c_char),
                   b"%S cannot be converted to %S by #%S\x00" as *const u8 as
                       *const libc::c_char, val, mrb_str_new_cstr(mrb, tname),
                   mrb_str_new_cstr(mrb, method));
    }
    return v;
}
unsafe extern "C" fn convert_type(mut mrb: *mut mrb_state, mut val: mrb_value,
                                  mut tname: *const libc::c_char,
                                  mut method: *const libc::c_char,
                                  mut raise: mrb_bool) -> mrb_value {
    let mut m: mrb_sym = 0i32 as mrb_sym;
    m = mrb_intern_cstr(mrb, method);
    if 0 == mrb_respond_to(mrb, val, m) {
        if 0 != raise {
            mrb_raisef(mrb,
                       mrb_exc_get(mrb,
                                   b"TypeError\x00" as *const u8 as
                                       *const libc::c_char),
                       b"can\'t convert %S into %S\x00" as *const u8 as
                           *const libc::c_char, inspect_type(mrb, val),
                       mrb_str_new_cstr(mrb, tname));
        }
        return mrb_nil_value()
    }
    return mrb_funcall_argv(mrb, val, m, 0i32 as mrb_int,
                            0 as *const mrb_value);
}
#[no_mangle]
pub unsafe extern "C" fn mrb_eql(mut mrb: *mut mrb_state, mut obj1: mrb_value,
                                 mut obj2: mrb_value) -> mrb_bool {
    if 0 != mrb_obj_eq(mrb, obj1, obj2) { return 1i32 as mrb_bool }
    return (mrb_funcall(mrb, obj1,
                        b"eql?\x00" as *const u8 as *const libc::c_char,
                        1i32 as mrb_int, obj2).tt as libc::c_uint !=
                MRB_TT_FALSE as libc::c_int as libc::c_uint) as libc::c_int as
               mrb_bool;
}
#[no_mangle]
pub unsafe extern "C" fn mrb_check_convert_type(mut mrb: *mut mrb_state,
                                                mut val: mrb_value,
                                                mut type_0: mrb_vtype,
                                                mut tname:
                                                    *const libc::c_char,
                                                mut method:
                                                    *const libc::c_char)
 -> mrb_value {
    let mut v: mrb_value =
        mrb_value{value: unnamed{f: 0.,}, tt: MRB_TT_FALSE,};
    if val.tt as libc::c_uint == type_0 as libc::c_uint &&
           type_0 as libc::c_uint !=
               MRB_TT_DATA as libc::c_int as libc::c_uint &&
           type_0 as libc::c_uint !=
               MRB_TT_ISTRUCT as libc::c_int as libc::c_uint {
        return val
    }
    v = convert_type(mrb, val, tname, method, 0i32 as mrb_bool);
    if v.tt as libc::c_uint == MRB_TT_FALSE as libc::c_int as libc::c_uint &&
           0 == v.value.i || v.tt as libc::c_uint != type_0 as libc::c_uint {
        return mrb_nil_value()
    }
    return v;
}
#[no_mangle]
pub unsafe extern "C" fn mrb_any_to_s(mut mrb: *mut mrb_state,
                                      mut obj: mrb_value) -> mrb_value {
    let mut str: mrb_value = mrb_str_new_capa(mrb, 20i32 as size_t);
    let mut cname: *const libc::c_char = mrb_obj_classname(mrb, obj);
    mrb_str_cat(mrb, str, b"#<\x00" as *const u8 as *const libc::c_char,
                (::std::mem::size_of::<[libc::c_char; 3]>() as
                     libc::c_ulong).wrapping_sub(1i32 as libc::c_ulong));
    mrb_str_cat_cstr(mrb, str, cname);
    if !((obj.tt as libc::c_uint) <
             MRB_TT_OBJECT as libc::c_int as libc::c_uint) {
        mrb_str_cat(mrb, str, b":\x00" as *const u8 as *const libc::c_char,
                    (::std::mem::size_of::<[libc::c_char; 2]>() as
                         libc::c_ulong).wrapping_sub(1i32 as libc::c_ulong));
        mrb_str_concat(mrb, str, mrb_ptr_to_str(mrb, obj.value.p));
    }
    mrb_str_cat(mrb, str, b">\x00" as *const u8 as *const libc::c_char,
                (::std::mem::size_of::<[libc::c_char; 2]>() as
                     libc::c_ulong).wrapping_sub(1i32 as libc::c_ulong));
    return str;
}
#[no_mangle]
pub unsafe extern "C" fn mrb_obj_is_kind_of(mut mrb: *mut mrb_state,
                                            mut obj: mrb_value,
                                            mut c: *mut RClass) -> mrb_bool {
    let mut cl: *mut RClass = mrb_class(mrb, obj);
    match (*c).tt() as libc::c_int {
        10 | 9 | 11 | 12 => { }
        _ => {
            mrb_raise(mrb,
                      mrb_exc_get(mrb,
                                  b"TypeError\x00" as *const u8 as
                                      *const libc::c_char),
                      b"class or module required\x00" as *const u8 as
                          *const libc::c_char);
        }
    }
    if 0 != (*c).flags() as libc::c_int & 1i32 << 19i32 {
        c = (*c).super_0;
        while 0 == (*c).flags() as libc::c_int & 1i32 << 18i32 {
            c = (*c).super_0
        }
    }
    while !cl.is_null() {
        if cl == c || (*cl).mt == (*c).mt { return 1i32 as mrb_bool }
        cl = (*cl).super_0
    }
    return 0i32 as mrb_bool;
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
pub unsafe extern "C" fn mrb_to_str(mut mrb: *mut mrb_state,
                                    mut val: mrb_value) -> mrb_value {
    return mrb_ensure_string_type(mrb, val);
}
/*
 * Returns a Ruby string type.
 *
 *
 * @param [mrb_state] mrb The current mruby state.
 * @param [mrb_value] str Ruby string.
 * @return [mrb_value] A Ruby string.
 */
#[no_mangle]
pub unsafe extern "C" fn mrb_ensure_string_type(mut mrb: *mut mrb_state,
                                                mut str: mrb_value)
 -> mrb_value {
    if !(str.tt as libc::c_uint ==
             MRB_TT_STRING as libc::c_int as libc::c_uint) {
        mrb_raisef(mrb,
                   mrb_exc_get(mrb,
                               b"TypeError\x00" as *const u8 as
                                   *const libc::c_char),
                   b"%S cannot be converted to String\x00" as *const u8 as
                       *const libc::c_char, inspect_type(mrb, str));
    }
    return str;
}
#[no_mangle]
pub unsafe extern "C" fn mrb_check_type(mut mrb: *mut mrb_state,
                                        mut x: mrb_value, mut t: mrb_vtype) {
    let mut type_0: *const types = builtin_types.as_ptr();
    let mut xt: mrb_vtype = MRB_TT_FALSE;
    xt = x.tt;
    if xt as libc::c_uint != t as libc::c_uint ||
           xt as libc::c_uint == MRB_TT_DATA as libc::c_int as libc::c_uint ||
           xt as libc::c_uint == MRB_TT_ISTRUCT as libc::c_int as libc::c_uint
       {
        while ((*type_0).type_0 as libc::c_int) <
                  MRB_TT_MAXDEFINE as libc::c_int {
            if (*type_0).type_0 as libc::c_uint == t as libc::c_uint {
                let mut etype: *const libc::c_char = 0 as *const libc::c_char;
                if x.tt as libc::c_uint ==
                       MRB_TT_FALSE as libc::c_int as libc::c_uint &&
                       0 == x.value.i {
                    etype = b"nil\x00" as *const u8 as *const libc::c_char
                } else if x.tt as libc::c_uint ==
                              MRB_TT_FIXNUM as libc::c_int as libc::c_uint {
                    etype = b"Fixnum\x00" as *const u8 as *const libc::c_char
                } else if x.tt as libc::c_uint ==
                              MRB_TT_SYMBOL as libc::c_int as libc::c_uint {
                    etype = b"Symbol\x00" as *const u8 as *const libc::c_char
                } else if (x.tt as libc::c_uint) <
                              MRB_TT_OBJECT as libc::c_int as libc::c_uint {
                    etype =
                        if 0 !=
                               (*(mrb_obj_as_string(mrb, x).value.p as
                                      *mut RString)).flags() as libc::c_int &
                                   32i32 {
                            (*(mrb_obj_as_string(mrb, x).value.p as
                                   *mut RString)).as_0.ary.as_mut_ptr()
                        } else {
                            (*(mrb_obj_as_string(mrb, x).value.p as
                                   *mut RString)).as_0.heap.ptr
                        }
                } else { etype = mrb_obj_classname(mrb, x) }
                mrb_raisef(mrb,
                           mrb_exc_get(mrb,
                                       b"TypeError\x00" as *const u8 as
                                           *const libc::c_char),
                           b"wrong argument type %S (expected %S)\x00" as
                               *const u8 as *const libc::c_char,
                           mrb_str_new_cstr(mrb, etype),
                           mrb_str_new_cstr(mrb, (*type_0).name));
            }
            type_0 = type_0.offset(1isize)
        }
        mrb_raisef(mrb,
                   mrb_exc_get(mrb,
                               b"TypeError\x00" as *const u8 as
                                   *const libc::c_char),
                   b"unknown type %S (%S given)\x00" as *const u8 as
                       *const libc::c_char, mrb_fixnum_value(t as mrb_int),
                   mrb_fixnum_value(x.tt as mrb_int));
    };
}
static mut builtin_types: [types; 18] =
    [types{type_0: MRB_TT_FALSE as libc::c_int as libc::c_uchar,
           name: b"false\x00" as *const u8 as *const libc::c_char,},
     types{type_0: MRB_TT_TRUE as libc::c_int as libc::c_uchar,
           name: b"true\x00" as *const u8 as *const libc::c_char,},
     types{type_0: MRB_TT_FIXNUM as libc::c_int as libc::c_uchar,
           name: b"Fixnum\x00" as *const u8 as *const libc::c_char,},
     types{type_0: MRB_TT_SYMBOL as libc::c_int as libc::c_uchar,
           name: b"Symbol\x00" as *const u8 as *const libc::c_char,},
     types{type_0: MRB_TT_MODULE as libc::c_int as libc::c_uchar,
           name: b"Module\x00" as *const u8 as *const libc::c_char,},
     types{type_0: MRB_TT_OBJECT as libc::c_int as libc::c_uchar,
           name: b"Object\x00" as *const u8 as *const libc::c_char,},
     types{type_0: MRB_TT_CLASS as libc::c_int as libc::c_uchar,
           name: b"Class\x00" as *const u8 as *const libc::c_char,},
     types{type_0: MRB_TT_ICLASS as libc::c_int as libc::c_uchar,
           name: b"iClass\x00" as *const u8 as *const libc::c_char,},
     types{type_0: MRB_TT_SCLASS as libc::c_int as libc::c_uchar,
           name: b"SClass\x00" as *const u8 as *const libc::c_char,},
     types{type_0: MRB_TT_PROC as libc::c_int as libc::c_uchar,
           name: b"Proc\x00" as *const u8 as *const libc::c_char,},
     types{type_0: MRB_TT_FLOAT as libc::c_int as libc::c_uchar,
           name: b"Float\x00" as *const u8 as *const libc::c_char,},
     types{type_0: MRB_TT_ARRAY as libc::c_int as libc::c_uchar,
           name: b"Array\x00" as *const u8 as *const libc::c_char,},
     types{type_0: MRB_TT_HASH as libc::c_int as libc::c_uchar,
           name: b"Hash\x00" as *const u8 as *const libc::c_char,},
     types{type_0: MRB_TT_STRING as libc::c_int as libc::c_uchar,
           name: b"String\x00" as *const u8 as *const libc::c_char,},
     types{type_0: MRB_TT_RANGE as libc::c_int as libc::c_uchar,
           name: b"Range\x00" as *const u8 as *const libc::c_char,},
     types{type_0: MRB_TT_FILE as libc::c_int as libc::c_uchar,
           name: b"File\x00" as *const u8 as *const libc::c_char,},
     types{type_0: MRB_TT_DATA as libc::c_int as libc::c_uchar,
           name: b"Data\x00" as *const u8 as *const libc::c_char,},
     types{type_0: MRB_TT_MAXDEFINE as libc::c_int as libc::c_uchar,
           name: 0 as *const libc::c_char,}];
/* obsolete: use mrb_ensure_string_type() instead */
#[no_mangle]
pub unsafe extern "C" fn mrb_string_type(mut mrb: *mut mrb_state,
                                         mut str: mrb_value) -> mrb_value {
    return mrb_ensure_string_type(mrb, str);
}
/*
 * Document-class: NilClass
 *
 *  The class of the singleton object <code>nil</code>.
 */
/* 15.2.4.3.4  */
/*
 * call_seq:
 *   nil.nil?               -> true
 *
 * Only the object <i>nil</i> responds <code>true</code> to <code>nil?</code>.
 */
unsafe extern "C" fn mrb_true(mut mrb: *mut mrb_state, mut obj: mrb_value)
 -> mrb_value {
    return mrb_true_value();
}
/* 15.2.4.3.5  */
/*
 *  call-seq:
 *     nil.to_s    -> ""
 *
 *  Always returns the empty string.
 */
unsafe extern "C" fn nil_to_s(mut mrb: *mut mrb_state, mut obj: mrb_value)
 -> mrb_value {
    return mrb_str_new(mrb, 0 as *const libc::c_char, 0i32 as size_t);
}
unsafe extern "C" fn nil_inspect(mut mrb: *mut mrb_state, mut obj: mrb_value)
 -> mrb_value {
    return mrb_str_new_static(mrb,
                              b"nil\x00" as *const u8 as *const libc::c_char,
                              (::std::mem::size_of::<[libc::c_char; 4]>() as
                                   libc::c_ulong).wrapping_sub(1i32 as
                                                                   libc::c_ulong));
}
/* **********************************************************************
 *  Document-class: TrueClass
 *
 *  The global value <code>true</code> is the only instance of class
 *  <code>TrueClass</code> and represents a logically true value in
 *  boolean expressions. The class provides operators allowing
 *  <code>true</code> to be used in logical expressions.
 */
/* 15.2.5.3.1  */
/*
 *  call-seq:
 *     true & obj    -> true or false
 *
 *  And---Returns <code>false</code> if <i>obj</i> is
 *  <code>nil</code> or <code>false</code>, <code>true</code> otherwise.
 */
unsafe extern "C" fn true_and(mut mrb: *mut mrb_state, mut obj: mrb_value)
 -> mrb_value {
    let mut obj2: mrb_bool = 0;
    mrb_get_args(mrb, b"b\x00" as *const u8 as *const libc::c_char,
                 &mut obj2 as *mut mrb_bool);
    return mrb_bool_value(obj2);
}
/* 15.2.5.3.2  */
/*
 *  call-seq:
 *     true ^ obj   -> !obj
 *
 *  Exclusive Or---Returns <code>true</code> if <i>obj</i> is
 *  <code>nil</code> or <code>false</code>, <code>false</code>
 *  otherwise.
 */
unsafe extern "C" fn true_xor(mut mrb: *mut mrb_state, mut obj: mrb_value)
 -> mrb_value {
    let mut obj2: mrb_bool = 0;
    mrb_get_args(mrb, b"b\x00" as *const u8 as *const libc::c_char,
                 &mut obj2 as *mut mrb_bool);
    return mrb_bool_value((0 == obj2) as libc::c_int as mrb_bool);
}
/* 15.2.5.3.3  */
/*
 * call-seq:
 *   true.to_s   ->  "true"
 *
 * The string representation of <code>true</code> is "true".
 */
unsafe extern "C" fn true_to_s(mut mrb: *mut mrb_state, mut obj: mrb_value)
 -> mrb_value {
    return mrb_str_new_static(mrb,
                              b"true\x00" as *const u8 as *const libc::c_char,
                              (::std::mem::size_of::<[libc::c_char; 5]>() as
                                   libc::c_ulong).wrapping_sub(1i32 as
                                                                   libc::c_ulong));
}
/* 15.2.5.3.4  */
/*
 *  call-seq:
 *     true | obj   -> true
 *
 *  Or---Returns <code>true</code>. As <i>anObject</i> is an argument to
 *  a method call, it is always evaluated; there is no short-circuit
 *  evaluation in this case.
 *
 *     true |  puts("or")
 *     true || puts("logical or")
 *
 *  <em>produces:</em>
 *
 *     or
 */
unsafe extern "C" fn true_or(mut mrb: *mut mrb_state, mut obj: mrb_value)
 -> mrb_value {
    return mrb_true_value();
}
/*
 *  Document-class: FalseClass
 *
 *  The global value <code>false</code> is the only instance of class
 *  <code>FalseClass</code> and represents a logically false value in
 *  boolean expressions. The class provides operators allowing
 *  <code>false</code> to participate correctly in logical expressions.
 *
 */
/* 15.2.4.3.1  */
/* 15.2.6.3.1  */
/*
 *  call-seq:
 *     false & obj   -> false
 *     nil & obj     -> false
 *
 *  And---Returns <code>false</code>. <i>obj</i> is always
 *  evaluated as it is the argument to a method call---there is no
 *  short-circuit evaluation in this case.
 */
unsafe extern "C" fn false_and(mut mrb: *mut mrb_state, mut obj: mrb_value)
 -> mrb_value {
    return mrb_false_value();
}
/* 15.2.4.3.2  */
/* 15.2.6.3.2  */
/*
 *  call-seq:
 *     false ^ obj    -> true or false
 *     nil   ^ obj    -> true or false
 *
 *  Exclusive Or---If <i>obj</i> is <code>nil</code> or
 *  <code>false</code>, returns <code>false</code>; otherwise, returns
 *  <code>true</code>.
 *
 */
unsafe extern "C" fn false_xor(mut mrb: *mut mrb_state, mut obj: mrb_value)
 -> mrb_value {
    let mut obj2: mrb_bool = 0;
    mrb_get_args(mrb, b"b\x00" as *const u8 as *const libc::c_char,
                 &mut obj2 as *mut mrb_bool);
    return mrb_bool_value(obj2);
}
/* 15.2.4.3.3  */
/* 15.2.6.3.4  */
/*
 *  call-seq:
 *     false | obj   ->   true or false
 *     nil   | obj   ->   true or false
 *
 *  Or---Returns <code>false</code> if <i>obj</i> is
 *  <code>nil</code> or <code>false</code>; <code>true</code> otherwise.
 */
unsafe extern "C" fn false_or(mut mrb: *mut mrb_state, mut obj: mrb_value)
 -> mrb_value {
    let mut obj2: mrb_bool = 0;
    mrb_get_args(mrb, b"b\x00" as *const u8 as *const libc::c_char,
                 &mut obj2 as *mut mrb_bool);
    return mrb_bool_value(obj2);
}
/* 15.2.6.3.3  */
/*
 * call-seq:
 *   false.to_s   ->  "false"
 *
 * 'nuf said...
 */
unsafe extern "C" fn false_to_s(mut mrb: *mut mrb_state, mut obj: mrb_value)
 -> mrb_value {
    return mrb_str_new_static(mrb,
                              b"false\x00" as *const u8 as
                                  *const libc::c_char,
                              (::std::mem::size_of::<[libc::c_char; 6]>() as
                                   libc::c_ulong).wrapping_sub(1i32 as
                                                                   libc::c_ulong));
}
#[no_mangle]
pub unsafe extern "C" fn mrb_init_object(mut mrb: *mut mrb_state) {
    let mut n: *mut RClass = 0 as *mut RClass;
    let mut t: *mut RClass = 0 as *mut RClass;
    let mut f: *mut RClass = 0 as *mut RClass;
    n =
        mrb_define_class(mrb,
                         b"NilClass\x00" as *const u8 as *const libc::c_char,
                         (*mrb).object_class);
    (*mrb).nil_class = n;
    (*n).set_flags(((*n).flags() as libc::c_int & !0xffi32 |
                        MRB_TT_TRUE as libc::c_int as libc::c_char as
                            libc::c_int) as uint32_t);
    mrb_undef_class_method(mrb, n,
                           b"new\x00" as *const u8 as *const libc::c_char);
    mrb_define_method(mrb, n, b"&\x00" as *const u8 as *const libc::c_char,
                      Some(false_and),
                      ((1i32 & 0x1fi32) as mrb_aspec) << 18i32);
    mrb_define_method(mrb, n, b"^\x00" as *const u8 as *const libc::c_char,
                      Some(false_xor),
                      ((1i32 & 0x1fi32) as mrb_aspec) << 18i32);
    mrb_define_method(mrb, n, b"|\x00" as *const u8 as *const libc::c_char,
                      Some(false_or),
                      ((1i32 & 0x1fi32) as mrb_aspec) << 18i32);
    mrb_define_method(mrb, n, b"nil?\x00" as *const u8 as *const libc::c_char,
                      Some(mrb_true), 0i32 as mrb_aspec);
    mrb_define_method(mrb, n, b"to_s\x00" as *const u8 as *const libc::c_char,
                      Some(nil_to_s), 0i32 as mrb_aspec);
    mrb_define_method(mrb, n,
                      b"inspect\x00" as *const u8 as *const libc::c_char,
                      Some(nil_inspect), 0i32 as mrb_aspec);
    t =
        mrb_define_class(mrb,
                         b"TrueClass\x00" as *const u8 as *const libc::c_char,
                         (*mrb).object_class);
    (*mrb).true_class = t;
    (*t).set_flags(((*t).flags() as libc::c_int & !0xffi32 |
                        MRB_TT_TRUE as libc::c_int as libc::c_char as
                            libc::c_int) as uint32_t);
    mrb_undef_class_method(mrb, t,
                           b"new\x00" as *const u8 as *const libc::c_char);
    mrb_define_method(mrb, t, b"&\x00" as *const u8 as *const libc::c_char,
                      Some(true_and),
                      ((1i32 & 0x1fi32) as mrb_aspec) << 18i32);
    mrb_define_method(mrb, t, b"^\x00" as *const u8 as *const libc::c_char,
                      Some(true_xor),
                      ((1i32 & 0x1fi32) as mrb_aspec) << 18i32);
    mrb_define_method(mrb, t, b"to_s\x00" as *const u8 as *const libc::c_char,
                      Some(true_to_s), 0i32 as mrb_aspec);
    mrb_define_method(mrb, t, b"|\x00" as *const u8 as *const libc::c_char,
                      Some(true_or),
                      ((1i32 & 0x1fi32) as mrb_aspec) << 18i32);
    mrb_define_method(mrb, t,
                      b"inspect\x00" as *const u8 as *const libc::c_char,
                      Some(true_to_s), 0i32 as mrb_aspec);
    f =
        mrb_define_class(mrb,
                         b"FalseClass\x00" as *const u8 as
                             *const libc::c_char, (*mrb).object_class);
    (*mrb).false_class = f;
    (*f).set_flags(((*f).flags() as libc::c_int & !0xffi32 |
                        MRB_TT_TRUE as libc::c_int as libc::c_char as
                            libc::c_int) as uint32_t);
    mrb_undef_class_method(mrb, f,
                           b"new\x00" as *const u8 as *const libc::c_char);
    mrb_define_method(mrb, f, b"&\x00" as *const u8 as *const libc::c_char,
                      Some(false_and),
                      ((1i32 & 0x1fi32) as mrb_aspec) << 18i32);
    mrb_define_method(mrb, f, b"^\x00" as *const u8 as *const libc::c_char,
                      Some(false_xor),
                      ((1i32 & 0x1fi32) as mrb_aspec) << 18i32);
    mrb_define_method(mrb, f, b"to_s\x00" as *const u8 as *const libc::c_char,
                      Some(false_to_s), 0i32 as mrb_aspec);
    mrb_define_method(mrb, f, b"|\x00" as *const u8 as *const libc::c_char,
                      Some(false_or),
                      ((1i32 & 0x1fi32) as mrb_aspec) << 18i32);
    mrb_define_method(mrb, f,
                      b"inspect\x00" as *const u8 as *const libc::c_char,
                      Some(false_to_s), 0i32 as mrb_aspec);
}
#[no_mangle]
pub unsafe extern "C" fn mrb_ensure_array_type(mut mrb: *mut mrb_state,
                                               mut ary: mrb_value)
 -> mrb_value {
    if !(ary.tt as libc::c_uint ==
             MRB_TT_ARRAY as libc::c_int as libc::c_uint) {
        mrb_raisef(mrb,
                   mrb_exc_get(mrb,
                               b"TypeError\x00" as *const u8 as
                                   *const libc::c_char),
                   b"%S cannot be converted to Array\x00" as *const u8 as
                       *const libc::c_char, inspect_type(mrb, ary));
    }
    return ary;
}
#[no_mangle]
pub unsafe extern "C" fn mrb_check_array_type(mut mrb: *mut mrb_state,
                                              mut ary: mrb_value)
 -> mrb_value {
    if !(ary.tt as libc::c_uint ==
             MRB_TT_ARRAY as libc::c_int as libc::c_uint) {
        return mrb_nil_value()
    }
    return ary;
}
#[no_mangle]
pub unsafe extern "C" fn mrb_ensure_hash_type(mut mrb: *mut mrb_state,
                                              mut hash: mrb_value)
 -> mrb_value {
    if !(hash.tt as libc::c_uint ==
             MRB_TT_HASH as libc::c_int as libc::c_uint) {
        mrb_raisef(mrb,
                   mrb_exc_get(mrb,
                               b"TypeError\x00" as *const u8 as
                                   *const libc::c_char),
                   b"%S cannot be converted to Hash\x00" as *const u8 as
                       *const libc::c_char, inspect_type(mrb, hash));
    }
    return hash;
}
#[no_mangle]
pub unsafe extern "C" fn mrb_check_hash_type(mut mrb: *mut mrb_state,
                                             mut hash: mrb_value)
 -> mrb_value {
    if !(hash.tt as libc::c_uint ==
             MRB_TT_HASH as libc::c_int as libc::c_uint) {
        return mrb_nil_value()
    }
    return hash;
}