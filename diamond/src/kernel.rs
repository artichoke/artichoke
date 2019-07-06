use libc;
use c2rust_bitfields::{BitfieldStruct};
extern "C" {
    pub type iv_tbl;
    /* debug info */
    pub type mrb_irep_debug_info;
    pub type symbol_name;
    pub type mrb_jmpbuf;
    #[no_mangle]
    fn __assert_rtn(_: *const libc::c_char, _: *const libc::c_char,
                    _: libc::c_int, _: *const libc::c_char) -> !;
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
 * Defines a class method.
 *
 * Example:
 *
 *     # Ruby style
 *     class Foo
 *       def Foo.bar
 *       end
 *     end
 *     // C style
 *     mrb_value bar_method(mrb_state* mrb, mrb_value self){
 *       return mrb_nil_value();
 *     }
 *     void mrb_example_gem_init(mrb_state* mrb){
 *       struct RClass *foo;
 *       foo = mrb_define_class(mrb, "Foo", mrb->object_class);
 *       mrb_define_class_method(mrb, foo, "bar", bar_method, MRB_ARGS_NONE());
 *     }
 * @param [mrb_state *] mrb_state* The MRuby state reference.
 * @param [struct RClass *] RClass* The class where the class method will be defined.
 * @param [const char *] char* The name of the class method being defined.
 * @param [mrb_func_t] mrb_func_t The function pointer to the class method definition.
 * @param [mrb_aspec] mrb_aspec The method parameters declaration.
 */
    #[no_mangle]
    fn mrb_define_class_method(_: *mut mrb_state, _: *mut RClass,
                               _: *const libc::c_char, _: mrb_func_t,
                               _: mrb_aspec);
    /* *
 * Gets a exception class.
 * @param [mrb_state*] mrb The current mruby state.
 * @param [const char *] name The name of the class.
 * @return [struct RClass *] A reference to the class.
*/
    #[no_mangle]
    fn mrb_exc_get(mrb: *mut mrb_state, name: *const libc::c_char)
     -> *mut RClass;
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
    fn memcpy(_: *mut libc::c_void, _: *const libc::c_void, _: libc::c_ulong)
     -> *mut libc::c_void;
    #[no_mangle]
    fn mrb_iv_copy(mrb: *mut mrb_state, dst: mrb_value, src: mrb_value);
    #[no_mangle]
    fn mrb_intern_static(_: *mut mrb_state, _: *const libc::c_char, _: size_t)
     -> mrb_sym;
    #[no_mangle]
    fn mrb_iv_remove(mrb: *mut mrb_state, obj: mrb_value, sym: mrb_sym)
     -> mrb_value;
    #[no_mangle]
    fn kh_init_mt(mrb: *mut mrb_state) -> *mut kh_mt_t;
    #[no_mangle]
    fn kh_copy_mt(mrb: *mut mrb_state, h: *mut kh_mt_t) -> *mut kh_mt_t;
    #[no_mangle]
    fn mrb_obj_class(mrb: *mut mrb_state, obj: mrb_value) -> *mut RClass;
    #[no_mangle]
    fn mrb_obj_alloc(_: *mut mrb_state, _: mrb_vtype, _: *mut RClass)
     -> *mut RBasic;
    #[no_mangle]
    fn mrb_raise(mrb: *mut mrb_state, c: *mut RClass,
                 msg: *const libc::c_char) -> !;
    #[no_mangle]
    fn mrb_raisef(mrb: *mut mrb_state, c: *mut RClass,
                  fmt: *const libc::c_char, _: ...) -> !;
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
    fn mrb_sym2str(_: *mut mrb_state, _: mrb_sym) -> mrb_value;
    #[no_mangle]
    fn mrb_malloc(_: *mut mrb_state, _: size_t) -> *mut libc::c_void;
    #[no_mangle]
    fn mrb_calloc(_: *mut mrb_state, _: size_t, _: size_t)
     -> *mut libc::c_void;
    #[no_mangle]
    fn mrb_free(_: *mut mrb_state, _: *mut libc::c_void);
    #[no_mangle]
    fn mrb_obj_id(obj: mrb_value) -> mrb_int;
    #[no_mangle]
    fn mrb_obj_equal(_: *mut mrb_state, _: mrb_value, _: mrb_value)
     -> mrb_bool;
    #[no_mangle]
    fn mrb_equal(mrb: *mut mrb_state, obj1: mrb_value, obj2: mrb_value)
     -> mrb_bool;
    #[no_mangle]
    fn mrb_field_write_barrier(_: *mut mrb_state, _: *mut RBasic,
                               _: *mut RBasic);
    #[no_mangle]
    fn mrb_any_to_s(mrb: *mut mrb_state, obj: mrb_value) -> mrb_value;
    #[no_mangle]
    fn mrb_obj_is_kind_of(mrb: *mut mrb_state, obj: mrb_value, c: *mut RClass)
     -> mrb_bool;
    #[no_mangle]
    fn mrb_obj_iv_inspect(_: *mut mrb_state, _: *mut RObject) -> mrb_value;
    #[no_mangle]
    fn mrb_method_search_vm(_: *mut mrb_state, _: *mut *mut RClass,
                            _: mrb_sym) -> mrb_method_t;
    #[no_mangle]
    fn mrb_obj_iv_set(mrb: *mut mrb_state, obj: *mut RObject, sym: mrb_sym,
                      v: mrb_value);
    #[no_mangle]
    fn mrb_exc_raise(mrb: *mut mrb_state, exc: mrb_value) -> !;
    #[no_mangle]
    fn mrb_name_error(mrb: *mut mrb_state, id: mrb_sym,
                      fmt: *const libc::c_char, _: ...) -> !;
    #[no_mangle]
    fn mrb_to_int(mrb: *mut mrb_state, val: mrb_value) -> mrb_value;
    #[no_mangle]
    fn mrb_to_str(mrb: *mut mrb_state, val: mrb_value) -> mrb_value;
    #[no_mangle]
    fn mrb_check_type(mrb: *mut mrb_state, x: mrb_value, t: mrb_vtype);
    #[no_mangle]
    fn mrb_define_alias(mrb: *mut mrb_state, c: *mut RClass,
                        a: *const libc::c_char, b: *const libc::c_char);
    #[no_mangle]
    fn mrb_respond_to(mrb: *mut mrb_state, obj: mrb_value, mid: mrb_sym)
     -> mrb_bool;
    /*
 * Create an array from the input. It tries calling to_a on the
 * value. If value does not respond to that, it creates a new
 * array with just this value.
 *
 * @param mrb The mruby state reference.
 * @param value The value to change into an array.
 * @return An array representation of value.
 */
    #[no_mangle]
    fn mrb_ary_splat(mrb: *mut mrb_state, value: mrb_value) -> mrb_value;
    /*
 * Get nth element in the array
 *
 * Equivalent to:
 *
 *     ary[offset]
 *
 * @param ary The target array.
 * @param offset The element position (negative counts from the tail).
 */
    #[no_mangle]
    fn mrb_ary_entry(ary: mrb_value, offset: mrb_int) -> mrb_value;
    #[no_mangle]
    fn mrb_iv_name_sym_check(mrb: *mut mrb_state, sym: mrb_sym);
    #[no_mangle]
    fn mrb_make_exception(mrb: *mut mrb_state, argc: mrb_int,
                          argv: *const mrb_value) -> mrb_value;
    #[no_mangle]
    fn mrb_no_method_error(mrb: *mut mrb_state, id: mrb_sym, args: mrb_value,
                           fmt: *const libc::c_char, _: ...) -> !;
    #[no_mangle]
    fn mrb_obj_equal_m(mrb: *mut mrb_state, _: mrb_value) -> mrb_value;
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
    pub unnamed: unnamed,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub union unnamed {
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
    pub value: unnamed_0,
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
pub union unnamed_0 {
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
 * Required arguments signature type.
 */
pub type mrb_aspec = uint32_t;
/*
** mruby/istruct.h - Inline structures
**
** See Copyright Notice in mruby.h
*/
/* *
 * Inline structures that fit in RVALUE
 *
 * They cannot have finalizer, and cannot have instance variables.
 */
#[derive ( BitfieldStruct , Clone , Copy )]
#[repr(C)]
pub struct RIStruct {
    #[bitfield(name = "tt", ty = "mrb_vtype", bits = "0..=7")]
    #[bitfield(name = "color", ty = "uint32_t", bits = "8..=10")]
    #[bitfield(name = "flags", ty = "uint32_t", bits = "11..=31")]
    pub tt_color_flags: [u8; 4],
    pub _pad: [u8; 4],
    pub c: *mut RClass,
    pub gcnext: *mut RBasic,
    pub inline_data: [libc::c_char; 24],
}
pub type kh_mt_t = kh_mt;
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
/*
** mruby/array.h - Array class
**
** See Copyright Notice in mruby.h
*/
/*
 * Array class
 */
#[derive ( Copy , Clone )]
#[repr(C)]
pub struct mrb_shared_array {
    pub refcnt: libc::c_int,
    pub len: mrb_int,
    pub ptr: *mut mrb_value,
}
#[derive ( BitfieldStruct , Clone , Copy )]
#[repr(C)]
pub struct RArray {
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
}
#[derive ( Copy , Clone )]
#[repr(C)]
pub struct unnamed_4 {
    pub len: mrb_int,
    pub aux: unnamed_5,
    pub ptr: *mut mrb_value,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub union unnamed_5 {
    pub capa: mrb_int,
    pub shared: *mut mrb_shared_array,
}
pub type khiter_t = khint_t;
#[derive ( Copy , Clone )]
#[repr(C)]
pub struct kh_st {
    pub n_buckets: khint_t,
    pub size: khint_t,
    pub n_occupied: khint_t,
    pub ed_flags: *mut uint8_t,
    pub keys: *mut mrb_sym,
    pub vals: *mut libc::c_char,
}
pub type kh_st_t = kh_st;
/*
 * Returns a fixnum in Ruby.
 */
#[inline]
unsafe extern "C" fn mrb_fixnum_value(mut i: mrb_int) -> mrb_value {
    let mut v: mrb_value =
        mrb_value{value: unnamed_0{f: 0.,}, tt: MRB_TT_FALSE,};
    v.tt = MRB_TT_FIXNUM;
    v.value.i = i;
    return v;
}
#[inline]
unsafe extern "C" fn mrb_symbol_value(mut i: mrb_sym) -> mrb_value {
    let mut v: mrb_value =
        mrb_value{value: unnamed_0{f: 0.,}, tt: MRB_TT_FALSE,};
    v.tt = MRB_TT_SYMBOL;
    v.value.sym = i;
    return v;
}
#[inline]
unsafe extern "C" fn mrb_obj_value(mut p: *mut libc::c_void) -> mrb_value {
    let mut v: mrb_value =
        mrb_value{value: unnamed_0{f: 0.,}, tt: MRB_TT_FALSE,};
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
        mrb_value{value: unnamed_0{f: 0.,}, tt: MRB_TT_FALSE,};
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
        mrb_value{value: unnamed_0{f: 0.,}, tt: MRB_TT_FALSE,};
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
        mrb_value{value: unnamed_0{f: 0.,}, tt: MRB_TT_FALSE,};
    v.tt = MRB_TT_TRUE;
    v.value.i = 1i32 as mrb_int;
    return v;
}
#[inline]
unsafe extern "C" fn mrb_bool_value(mut boolean: mrb_bool) -> mrb_value {
    let mut v: mrb_value =
        mrb_value{value: unnamed_0{f: 0.,}, tt: MRB_TT_FALSE,};
    v.tt =
        (if 0 != boolean as libc::c_int {
             MRB_TT_TRUE as libc::c_int
         } else { MRB_TT_FALSE as libc::c_int }) as mrb_vtype;
    v.value.i = 1i32 as mrb_int;
    return v;
}
/* *
 * Duplicate an object.
 *
 * Equivalent to:
 *   Object#dup
 * @param [mrb_state*] mrb The current mruby state.
 * @param [mrb_value] obj Object to be duplicate.
 * @return [mrb_value] The newly duplicated object.
 */
#[no_mangle]
pub unsafe extern "C" fn mrb_obj_dup(mut mrb: *mut mrb_state,
                                     mut obj: mrb_value) -> mrb_value {
    let mut p: *mut RBasic = 0 as *mut RBasic;
    let mut dup: mrb_value =
        mrb_value{value: unnamed_0{f: 0.,}, tt: MRB_TT_FALSE,};
    if (obj.tt as libc::c_uint) < MRB_TT_OBJECT as libc::c_int as libc::c_uint
       {
        mrb_raisef(mrb,
                   mrb_exc_get(mrb,
                               b"TypeError\x00" as *const u8 as
                                   *const libc::c_char),
                   b"can\'t dup %S\x00" as *const u8 as *const libc::c_char,
                   obj);
    }
    if obj.tt as libc::c_uint == MRB_TT_SCLASS as libc::c_int as libc::c_uint
       {
        mrb_raise(mrb,
                  mrb_exc_get(mrb,
                              b"TypeError\x00" as *const u8 as
                                  *const libc::c_char),
                  b"can\'t dup singleton class\x00" as *const u8 as
                      *const libc::c_char);
    }
    p = mrb_obj_alloc(mrb, obj.tt, mrb_obj_class(mrb, obj));
    dup = mrb_obj_value(p as *mut libc::c_void);
    init_copy(mrb, dup, obj);
    return dup;
}
unsafe extern "C" fn init_copy(mut mrb: *mut mrb_state, mut dest: mrb_value,
                               mut obj: mrb_value) {
    match obj.tt as libc::c_uint {
        11 => { copy_class(mrb, dest, obj); return }
        9 | 10 => {
            copy_class(mrb, dest, obj);
            mrb_iv_copy(mrb, dest, obj);
            mrb_iv_remove(mrb, dest,
                          mrb_intern_static(mrb,
                                            b"__classname__\x00" as *const u8
                                                as *const libc::c_char,
                                            (::std::mem::size_of::<[libc::c_char; 14]>()
                                                 as
                                                 libc::c_ulong).wrapping_sub(1i32
                                                                                 as
                                                                                 libc::c_ulong)));
        }
        8 | 12 | 15 | 21 | 18 => { mrb_iv_copy(mrb, dest, obj); }
        23 => { mrb_istruct_copy(dest, obj); }
        _ => { }
    }
    mrb_funcall(mrb, dest,
                b"initialize_copy\x00" as *const u8 as *const libc::c_char,
                1i32 as mrb_int, obj);
}
#[inline]
unsafe extern "C" fn mrb_istruct_copy(mut dest: mrb_value,
                                      mut src: mrb_value) {
    memcpy((*(dest.value.p as *mut RIStruct)).inline_data.as_mut_ptr() as
               *mut libc::c_void,
           (*(src.value.p as *mut RIStruct)).inline_data.as_mut_ptr() as
               *const libc::c_void,
           (::std::mem::size_of::<*mut libc::c_void>() as
                libc::c_ulong).wrapping_mul(3i32 as libc::c_ulong));
}
unsafe extern "C" fn copy_class(mut mrb: *mut mrb_state, mut dst: mrb_value,
                                mut src: mrb_value) {
    let mut dc: *mut RClass = dst.value.p as *mut RClass;
    let mut sc: *mut RClass = src.value.p as *mut RClass;
    if 0 != (*sc).flags() as libc::c_int & 1i32 << 19i32 {
        let mut c0: *mut RClass = (*sc).super_0;
        let mut c1: *mut RClass = dc;
        while 0 == (*c0).flags() as libc::c_int & 1i32 << 18i32 {
            (*c1).super_0 =
                mrb_obj_dup(mrb,
                            mrb_obj_value(c0 as *mut libc::c_void)).value.p as
                    *mut RClass;
            c1 = (*c1).super_0;
            c0 = (*c0).super_0
        }
        (*c1).super_0 =
            mrb_obj_dup(mrb, mrb_obj_value(c0 as *mut libc::c_void)).value.p
                as *mut RClass;
        (*(*c1).super_0).set_flags((*(*c1).super_0).flags() |
                                       (1i32 << 18i32) as uint32_t)
    }
    if !(*sc).mt.is_null() {
        (*dc).mt = kh_copy_mt(mrb, (*sc).mt)
    } else { (*dc).mt = kh_init_mt(mrb) }
    (*dc).super_0 = (*sc).super_0;
    (*dc).set_flags(((*dc).flags() as libc::c_int & !0xffi32 |
                         ((*sc).flags() as libc::c_int & 0xffi32) as mrb_vtype
                             as libc::c_char as libc::c_int) as uint32_t);
}
#[no_mangle]
pub unsafe extern "C" fn mrb_obj_inspect(mut mrb: *mut mrb_state,
                                         mut obj: mrb_value) -> mrb_value {
    if obj.tt as libc::c_uint == MRB_TT_OBJECT as libc::c_int as libc::c_uint
           && 0 != mrb_obj_basic_to_s_p(mrb, obj) as libc::c_int {
        return mrb_obj_iv_inspect(mrb, obj.value.p as *mut RObject)
    }
    return mrb_any_to_s(mrb, obj);
}
unsafe extern "C" fn mrb_obj_basic_to_s_p(mut mrb: *mut mrb_state,
                                          mut obj: mrb_value) -> mrb_bool {
    return mrb_func_basic_p(mrb, obj,
                            mrb_intern_static(mrb,
                                              b"to_s\x00" as *const u8 as
                                                  *const libc::c_char,
                                              (::std::mem::size_of::<[libc::c_char; 5]>()
                                                   as
                                                   libc::c_ulong).wrapping_sub(1i32
                                                                                   as
                                                                                   libc::c_ulong)),
                            Some(mrb_any_to_s));
}
#[no_mangle]
pub unsafe extern "C" fn mrb_func_basic_p(mut mrb: *mut mrb_state,
                                          mut obj: mrb_value,
                                          mut mid: mrb_sym,
                                          mut func: mrb_func_t) -> mrb_bool {
    let mut c: *mut RClass = mrb_class(mrb, obj);
    let mut m: mrb_method_t = mrb_method_search_vm(mrb, &mut c, mid);
    let mut p: *mut RProc = 0 as *mut RProc;
    if m.unnamed.proc_0.is_null() { return 0i32 as mrb_bool }
    if 0 != m.func_p {
        return (m.unnamed.func == func) as libc::c_int as mrb_bool
    }
    p = m.unnamed.proc_0;
    if (*p).flags() as libc::c_int & 128i32 != 0i32 && (*p).body.func == func
       {
        return 1i32 as mrb_bool
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
pub unsafe extern "C" fn mrb_obj_clone(mut mrb: *mut mrb_state,
                                       mut self_0: mrb_value) -> mrb_value {
    let mut p: *mut RObject = 0 as *mut RObject;
    let mut clone: mrb_value =
        mrb_value{value: unnamed_0{f: 0.,}, tt: MRB_TT_FALSE,};
    if (self_0.tt as libc::c_uint) <
           MRB_TT_OBJECT as libc::c_int as libc::c_uint {
        mrb_raisef(mrb,
                   mrb_exc_get(mrb,
                               b"TypeError\x00" as *const u8 as
                                   *const libc::c_char),
                   b"can\'t clone %S\x00" as *const u8 as *const libc::c_char,
                   self_0);
    }
    if self_0.tt as libc::c_uint ==
           MRB_TT_SCLASS as libc::c_int as libc::c_uint {
        mrb_raise(mrb,
                  mrb_exc_get(mrb,
                              b"TypeError\x00" as *const u8 as
                                  *const libc::c_char),
                  b"can\'t clone singleton class\x00" as *const u8 as
                      *const libc::c_char);
    }
    p =
        mrb_obj_alloc(mrb, self_0.tt, mrb_obj_class(mrb, self_0)) as
            *mut RObject;
    (*p).c = mrb_singleton_class_clone(mrb, self_0);
    mrb_field_write_barrier(mrb, p as *mut RBasic, (*p).c as *mut RBasic);
    clone = mrb_obj_value(p as *mut libc::c_void);
    init_copy(mrb, clone, self_0);
    (*p).set_flags((*p).flags() |
                       ((*(self_0.value.p as *mut RObject)).flags() as
                            libc::c_int & 1i32 << 20i32) as uint32_t);
    return clone;
}
unsafe extern "C" fn mrb_singleton_class_clone(mut mrb: *mut mrb_state,
                                               mut obj: mrb_value)
 -> *mut RClass {
    let mut klass: *mut RClass = (*(obj.value.p as *mut RBasic)).c;
    if (*klass).tt() as libc::c_int != MRB_TT_SCLASS as libc::c_int {
        return klass
    } else {
        let mut clone: *mut RClass =
            mrb_obj_alloc(mrb, (*klass).tt(), (*mrb).class_class) as
                *mut RClass;
        match obj.tt as libc::c_uint {
            9 | 12 => { }
            _ => {
                (*clone).c =
                    mrb_singleton_class_clone(mrb,
                                              mrb_obj_value(klass as
                                                                *mut libc::c_void))
            }
        }
        (*clone).super_0 = (*klass).super_0;
        if !(*klass).iv.is_null() {
            mrb_iv_copy(mrb, mrb_obj_value(clone as *mut libc::c_void),
                        mrb_obj_value(klass as *mut libc::c_void));
            mrb_obj_iv_set(mrb, clone as *mut RObject,
                           mrb_intern_static(mrb,
                                             b"__attached__\x00" as *const u8
                                                 as *const libc::c_char,
                                             (::std::mem::size_of::<[libc::c_char; 13]>()
                                                  as
                                                  libc::c_ulong).wrapping_sub(1i32
                                                                                  as
                                                                                  libc::c_ulong)),
                           obj);
        }
        if !(*klass).mt.is_null() {
            (*clone).mt = kh_copy_mt(mrb, (*klass).mt)
        } else { (*clone).mt = kh_init_mt(mrb) }
        (*clone).set_tt(MRB_TT_SCLASS);
        return clone
    };
}
#[no_mangle]
pub unsafe extern "C" fn mrb_obj_is_instance_of(mut mrb: *mut mrb_state,
                                                mut obj: mrb_value,
                                                mut c: *mut RClass)
 -> mrb_bool {
    if mrb_obj_class(mrb, obj) == c { return 1i32 as mrb_bool }
    return 0i32 as mrb_bool;
}
/* extern uint8_t __m[]; */
/* mask for flags */
static mut __m_empty: [uint8_t; 4] =
    [0x2i32 as uint8_t, 0x8i32 as uint8_t, 0x20i32 as uint8_t,
     0x80i32 as uint8_t];
static mut __m_del: [uint8_t; 4] =
    [0x1i32 as uint8_t, 0x4i32 as uint8_t, 0x10i32 as uint8_t,
     0x40i32 as uint8_t];
static mut __m_either: [uint8_t; 4] =
    [0x3i32 as uint8_t, 0xci32 as uint8_t, 0x30i32 as uint8_t,
     0xc0i32 as uint8_t];
/* declare struct kh_xxx and kh_xxx_funcs

   name: hash name
   khkey_t: key data type
   khval_t: value data type
   kh_is_map: (0: hash set / 1: hash map)
*/
#[inline]
unsafe extern "C" fn kh_fill_flags(mut p: *mut uint8_t, mut c: uint8_t,
                                   mut len: size_t) {
    loop  {
        let fresh0 = len;
        len = len.wrapping_sub(1);
        if !(fresh0 > 0i32 as libc::c_ulong) { break ; }
        let fresh1 = p;
        p = p.offset(1);
        *fresh1 = c
    };
}
/* declaration for `fail` method */
#[no_mangle]
pub unsafe extern "C" fn mrb_f_raise(mut mrb: *mut mrb_state,
                                     mut self_0: mrb_value) -> mrb_value {
    let mut a: [mrb_value; 2] =
        [mrb_value{value: unnamed_0{f: 0.,}, tt: MRB_TT_FALSE,}; 2];
    let mut exc: mrb_value =
        mrb_value{value: unnamed_0{f: 0.,}, tt: MRB_TT_FALSE,};
    let mut argc: mrb_int = 0;
    argc =
        mrb_get_args(mrb, b"|oo\x00" as *const u8 as *const libc::c_char,
                     &mut *a.as_mut_ptr().offset(0isize) as *mut mrb_value,
                     &mut *a.as_mut_ptr().offset(1isize) as *mut mrb_value);
    match argc {
        0 => {
            mrb_raise(mrb,
                      mrb_exc_get(mrb,
                                  b"RuntimeError\x00" as *const u8 as
                                      *const libc::c_char),
                      b"\x00" as *const u8 as *const libc::c_char);
        }
        1 => {
            if a[0usize].tt as libc::c_uint ==
                   MRB_TT_STRING as libc::c_int as libc::c_uint {
                a[1usize] = a[0usize];
                argc = 2i32 as mrb_int;
                a[0usize] =
                    mrb_obj_value(mrb_exc_get(mrb,
                                              b"RuntimeError\x00" as *const u8
                                                  as *const libc::c_char) as
                                      *mut libc::c_void)
            }
        }
        _ => { }
    }
    exc = mrb_make_exception(mrb, argc, a.as_mut_ptr());
    mrb_exc_raise(mrb, exc);
}
/* 15.3.1.3.2  */
/*
 *  call-seq:
 *     obj === other   -> true or false
 *
 *  Case Equality---For class <code>Object</code>, effectively the same
 *  as calling  <code>#==</code>, but typically overridden by descendants
 *  to provide meaningful semantics in <code>case</code> statements.
 */
unsafe extern "C" fn mrb_equal_m(mut mrb: *mut mrb_state,
                                 mut self_0: mrb_value) -> mrb_value {
    let mut arg: mrb_value =
        mrb_value{value: unnamed_0{f: 0.,}, tt: MRB_TT_FALSE,};
    mrb_get_args(mrb, b"o\x00" as *const u8 as *const libc::c_char,
                 &mut arg as *mut mrb_value);
    return mrb_bool_value(mrb_equal(mrb, self_0, arg));
}
/* 15.3.1.3.3  */
/* 15.3.1.3.33 */
/*
 *  Document-method: __id__
 *  Document-method: object_id
 *
 *  call-seq:
 *     obj.__id__       -> fixnum
 *     obj.object_id    -> fixnum
 *
 *  Returns an integer identifier for <i>obj</i>. The same number will
 *  be returned on all calls to <code>id</code> for a given object, and
 *  no two active objects will share an id.
 *  <code>Object#object_id</code> is a different concept from the
 *  <code>:name</code> notation, which returns the symbol id of
 *  <code>name</code>. Replaces the deprecated <code>Object#id</code>.
 */
#[no_mangle]
pub unsafe extern "C" fn mrb_obj_id_m(mut mrb: *mut mrb_state,
                                      mut self_0: mrb_value) -> mrb_value {
    return mrb_fixnum_value(mrb_obj_id(self_0));
}
/* 15.3.1.2.2  */
/* 15.3.1.2.5  */
/* 15.3.1.3.6  */
/* 15.3.1.3.25 */
/*
 *  call-seq:
 *     block_given?   -> true or false
 *     iterator?      -> true or false
 *
 *  Returns <code>true</code> if <code>yield</code> would execute a
 *  block in the current context. The <code>iterator?</code> form
 *  is mildly deprecated.
 *
 *     def try
 *       if block_given?
 *         yield
 *       else
 *         "no block"
 *       end
 *     end
 *     try                  #=> "no block"
 *     try { "hello" }      #=> "hello"
 *     try do "hello" end   #=> "hello"
 */
unsafe extern "C" fn mrb_f_block_given_p_m(mut mrb: *mut mrb_state,
                                           mut self_0: mrb_value)
 -> mrb_value {
    let mut ci: *mut mrb_callinfo =
        &mut *(*(*mrb).c).ci.offset(-1i32 as isize) as *mut mrb_callinfo;
    let mut cibase: *mut mrb_callinfo = (*(*mrb).c).cibase;
    let mut bp: *mut mrb_value = 0 as *mut mrb_value;
    let mut p: *mut RProc = 0 as *mut RProc;
    if ci <= cibase { return mrb_false_value() }
    p = (*ci).proc_0;
    while !p.is_null() {
        if (*p).flags() as libc::c_int & 2048i32 != 0i32 { break ; }
        p = (*p).upper
    }
    if p.is_null() { return mrb_false_value() }
    while cibase < ci {
        if (*ci).proc_0 == p { break ; }
        ci = ci.offset(-1isize)
    }
    if ci == cibase {
        return mrb_false_value()
    } else {
        if !(*ci).env.is_null() {
            let mut e: *mut REnv = (*ci).env;
            let mut bidx: libc::c_int = 0;
            if (*e).stack == (*(*mrb).c).stbase { return mrb_false_value() }
            bidx = (*e).flags() as libc::c_int >> 10i32 & 0x3ffi32;
            if bidx as libc::c_longlong >=
                   ((*e).flags() as libc::c_int & 0x3ffi32) as mrb_int {
                return mrb_false_value()
            }
            bp = &mut *(*e).stack.offset(bidx as isize) as *mut mrb_value
        } else {
            bp = (*ci.offset(1isize)).stackent.offset(1isize);
            if (*ci).argc >= 0i32 {
                bp = bp.offset((*ci).argc as isize)
            } else { bp = bp.offset(1isize) }
        }
    }
    if (*bp).tt as libc::c_uint == MRB_TT_FALSE as libc::c_int as libc::c_uint
           && 0 == (*bp).value.i {
        return mrb_false_value()
    }
    return mrb_true_value();
}
/* 15.3.1.3.7  */
/*
 *  call-seq:
 *     obj.class    -> class
 *
 *  Returns the class of <i>obj</i>. This method must always be
 *  called with an explicit receiver, as <code>class</code> is also a
 *  reserved word in Ruby.
 *
 *     1.class      #=> Fixnum
 *     self.class   #=> Object
 */
unsafe extern "C" fn mrb_obj_class_m(mut mrb: *mut mrb_state,
                                     mut self_0: mrb_value) -> mrb_value {
    return mrb_obj_value(mrb_obj_class(mrb, self_0) as *mut libc::c_void);
}
unsafe extern "C" fn mrb_obj_extend(mut mrb: *mut mrb_state,
                                    mut argc: mrb_int,
                                    mut argv: *mut mrb_value,
                                    mut obj: mrb_value) -> mrb_value {
    let mut i: mrb_int = 0;
    if argc == 0i32 as libc::c_longlong {
        mrb_raise(mrb,
                  mrb_exc_get(mrb,
                              b"ArgumentError\x00" as *const u8 as
                                  *const libc::c_char),
                  b"wrong number of arguments (at least 1)\x00" as *const u8
                      as *const libc::c_char);
    }
    i = 0i32 as mrb_int;
    while i < argc {
        mrb_check_type(mrb, *argv.offset(i as isize), MRB_TT_MODULE);
        i += 1
    }
    loop  {
        let fresh2 = argc;
        argc = argc - 1;
        if !(0 != fresh2) { break ; }
        mrb_funcall(mrb, *argv.offset(argc as isize),
                    b"extend_object\x00" as *const u8 as *const libc::c_char,
                    1i32 as mrb_int, obj);
        mrb_funcall(mrb, *argv.offset(argc as isize),
                    b"extended\x00" as *const u8 as *const libc::c_char,
                    1i32 as mrb_int, obj);
    }
    return obj;
}
/* 15.3.1.3.13 */
/*
 *  call-seq:
 *     obj.extend(module, ...)    -> obj
 *
 *  Adds to _obj_ the instance methods from each module given as a
 *  parameter.
 *
 *     module Mod
 *       def hello
 *         "Hello from Mod.\n"
 *       end
 *     end
 *
 *     class Klass
 *       def hello
 *         "Hello from Klass.\n"
 *       end
 *     end
 *
 *     k = Klass.new
 *     k.hello         #=> "Hello from Klass.\n"
 *     k.extend(Mod)   #=> #<Klass:0x401b3bc8>
 *     k.hello         #=> "Hello from Mod.\n"
 */
unsafe extern "C" fn mrb_obj_extend_m(mut mrb: *mut mrb_state,
                                      mut self_0: mrb_value) -> mrb_value {
    let mut argv: *mut mrb_value = 0 as *mut mrb_value;
    let mut argc: mrb_int = 0;
    mrb_get_args(mrb, b"*\x00" as *const u8 as *const libc::c_char,
                 &mut argv as *mut *mut mrb_value, &mut argc as *mut mrb_int);
    return mrb_obj_extend(mrb, argc, argv, self_0);
}
unsafe extern "C" fn mrb_obj_freeze(mut mrb: *mut mrb_state,
                                    mut self_0: mrb_value) -> mrb_value {
    if !((self_0.tt as libc::c_uint) <
             MRB_TT_OBJECT as libc::c_int as libc::c_uint) {
        let mut b: *mut RBasic = self_0.value.p as *mut RBasic;
        if 0 == (*b).flags() as libc::c_int & 1i32 << 20i32 {
            (*b).set_flags((*b).flags() | (1i32 << 20i32) as uint32_t);
            if (*(*b).c).tt() as libc::c_int == MRB_TT_SCLASS as libc::c_int {
                (*(*b).c).set_flags((*(*b).c).flags() |
                                        (1i32 << 20i32) as uint32_t)
            }
        }
    }
    return self_0;
}
unsafe extern "C" fn mrb_obj_frozen(mut mrb: *mut mrb_state,
                                    mut self_0: mrb_value) -> mrb_value {
    return mrb_bool_value(((self_0.tt as libc::c_uint) <
                               MRB_TT_OBJECT as libc::c_int as libc::c_uint ||
                               0 !=
                                   (*(self_0.value.p as *mut RBasic)).flags()
                                       as libc::c_int & 1i32 << 20i32) as
                              libc::c_int as mrb_bool);
}
/* 15.3.1.3.15 */
/*
 *  call-seq:
 *     obj.hash    -> fixnum
 *
 *  Generates a <code>Fixnum</code> hash value for this object. This
 *  function must have the property that <code>a.eql?(b)</code> implies
 *  <code>a.hash == b.hash</code>. The hash value is used by class
 *  <code>Hash</code>. Any hash value that exceeds the capacity of a
 *  <code>Fixnum</code> will be truncated before being used.
 */
unsafe extern "C" fn mrb_obj_hash(mut mrb: *mut mrb_state,
                                  mut self_0: mrb_value) -> mrb_value {
    return mrb_fixnum_value(mrb_obj_id(self_0));
}
/* 15.3.1.3.16 */
unsafe extern "C" fn mrb_obj_init_copy(mut mrb: *mut mrb_state,
                                       mut self_0: mrb_value) -> mrb_value {
    let mut orig: mrb_value =
        mrb_value{value: unnamed_0{f: 0.,}, tt: MRB_TT_FALSE,};
    mrb_get_args(mrb, b"o\x00" as *const u8 as *const libc::c_char,
                 &mut orig as *mut mrb_value);
    if 0 != mrb_obj_equal(mrb, self_0, orig) { return self_0 }
    if self_0.tt as libc::c_uint != orig.tt as libc::c_uint ||
           mrb_obj_class(mrb, self_0) != mrb_obj_class(mrb, orig) {
        mrb_raise(mrb,
                  mrb_exc_get(mrb,
                              b"TypeError\x00" as *const u8 as
                                  *const libc::c_char),
                  b"initialize_copy should take same class object\x00" as
                      *const u8 as *const libc::c_char);
    }
    return self_0;
}
/* 15.3.1.3.19 */
/*
 *  call-seq:
 *     obj.instance_of?(class)    -> true or false
 *
 *  Returns <code>true</code> if <i>obj</i> is an instance of the given
 *  class. See also <code>Object#kind_of?</code>.
 */
unsafe extern "C" fn obj_is_instance_of(mut mrb: *mut mrb_state,
                                        mut self_0: mrb_value) -> mrb_value {
    let mut arg: mrb_value =
        mrb_value{value: unnamed_0{f: 0.,}, tt: MRB_TT_FALSE,};
    mrb_get_args(mrb, b"C\x00" as *const u8 as *const libc::c_char,
                 &mut arg as *mut mrb_value);
    return mrb_bool_value(mrb_obj_is_instance_of(mrb, self_0,
                                                 arg.value.p as *mut RClass));
}
/* 15.3.1.3.24 */
/* 15.3.1.3.26 */
/*
 *  call-seq:
 *     obj.is_a?(class)       -> true or false
 *     obj.kind_of?(class)    -> true or false
 *
 *  Returns <code>true</code> if <i>class</i> is the class of
 *  <i>obj</i>, or if <i>class</i> is one of the superclasses of
 *  <i>obj</i> or modules included in <i>obj</i>.
 *
 *     module M;    end
 *     class A
 *       include M
 *     end
 *     class B < A; end
 *     class C < B; end
 *     b = B.new
 *     b.instance_of? A   #=> false
 *     b.instance_of? B   #=> true
 *     b.instance_of? C   #=> false
 *     b.instance_of? M   #=> false
 *     b.kind_of? A       #=> true
 *     b.kind_of? B       #=> true
 *     b.kind_of? C       #=> false
 *     b.kind_of? M       #=> true
 */
unsafe extern "C" fn mrb_obj_is_kind_of_m(mut mrb: *mut mrb_state,
                                          mut self_0: mrb_value)
 -> mrb_value {
    let mut arg: mrb_value =
        mrb_value{value: unnamed_0{f: 0.,}, tt: MRB_TT_FALSE,};
    mrb_get_args(mrb, b"C\x00" as *const u8 as *const libc::c_char,
                 &mut arg as *mut mrb_value);
    return mrb_bool_value(mrb_obj_is_kind_of(mrb, self_0,
                                             arg.value.p as *mut RClass));
}
#[no_mangle]
pub unsafe extern "C" fn kh_alloc_st(mut mrb: *mut mrb_state,
                                     mut h: *mut kh_st_t) {
    let mut sz: khint_t = (*h).n_buckets;
    let mut len: size_t =
        (::std::mem::size_of::<mrb_sym>() as
             libc::c_ulong).wrapping_add(if 0 != 0i32 {
                                             ::std::mem::size_of::<libc::c_char>()
                                                 as libc::c_ulong
                                         } else { 0i32 as libc::c_ulong });
    let mut p: *mut uint8_t =
        mrb_malloc(mrb,
                   (::std::mem::size_of::<uint8_t>() as
                        libc::c_ulong).wrapping_mul(sz as
                                                        libc::c_ulong).wrapping_div(4i32
                                                                                        as
                                                                                        libc::c_ulong).wrapping_add(len.wrapping_mul(sz
                                                                                                                                         as
                                                                                                                                         libc::c_ulong)))
            as *mut uint8_t;
    (*h).n_occupied = 0i32 as khint_t;
    (*h).size = (*h).n_occupied;
    (*h).keys = p as *mut mrb_sym;
    (*h).vals =
        if 0 != 0i32 {
            p.offset((::std::mem::size_of::<mrb_sym>() as
                          libc::c_ulong).wrapping_mul(sz as libc::c_ulong) as
                         isize) as *mut libc::c_char
        } else { 0 as *mut libc::c_char };
    (*h).ed_flags = p.offset(len.wrapping_mul(sz as libc::c_ulong) as isize);
    kh_fill_flags((*h).ed_flags, 0xaai32 as uint8_t,
                  sz.wrapping_div(4i32 as libc::c_uint) as size_t);
}
#[no_mangle]
pub unsafe extern "C" fn kh_init_st_size(mut mrb: *mut mrb_state,
                                         mut size: khint_t) -> *mut kh_st_t {
    let mut h: *mut kh_st_t =
        mrb_calloc(mrb, 1i32 as size_t,
                   ::std::mem::size_of::<kh_st_t>() as libc::c_ulong) as
            *mut kh_st_t;
    if size < 8i32 as libc::c_uint { size = 8i32 as khint_t }
    size = size.wrapping_sub(1);
    size |= size >> 1i32;
    size |= size >> 2i32;
    size |= size >> 4i32;
    size |= size >> 8i32;
    size |= size >> 16i32;
    size = size.wrapping_add(1);
    (*h).n_buckets = size;
    kh_alloc_st(mrb, h);
    return h;
}
#[no_mangle]
pub unsafe extern "C" fn kh_init_st(mut mrb: *mut mrb_state) -> *mut kh_st_t {
    return kh_init_st_size(mrb, 32i32 as khint_t);
}
#[no_mangle]
pub unsafe extern "C" fn kh_destroy_st(mut mrb: *mut mrb_state,
                                       mut h: *mut kh_st_t) {
    if !h.is_null() {
        mrb_free(mrb, (*h).keys as *mut libc::c_void);
        mrb_free(mrb, h as *mut libc::c_void);
    };
}
#[no_mangle]
pub unsafe extern "C" fn kh_clear_st(mut mrb: *mut mrb_state,
                                     mut h: *mut kh_st_t) {
    if !h.is_null() && !(*h).ed_flags.is_null() {
        kh_fill_flags((*h).ed_flags, 0xaai32 as uint8_t,
                      (*h).n_buckets.wrapping_div(4i32 as libc::c_uint) as
                          size_t);
        (*h).n_occupied = 0i32 as khint_t;
        (*h).size = (*h).n_occupied
    };
}
#[no_mangle]
pub unsafe extern "C" fn kh_get_st(mut mrb: *mut mrb_state,
                                   mut h: *mut kh_st_t, mut key: mrb_sym)
 -> khint_t {
    let mut k: khint_t =
        (key ^ key << 2i32 ^ key >> 2i32) as khint_t &
            (*h).n_buckets.wrapping_sub(1i32 as libc::c_uint);
    let mut step: khint_t = 0i32 as khint_t;
    while 0 ==
              *(*h).ed_flags.offset(k.wrapping_div(4i32 as libc::c_uint) as
                                        isize) as libc::c_int &
                  __m_empty[k.wrapping_rem(4i32 as libc::c_uint) as usize] as
                      libc::c_int {
        if 0 ==
               *(*h).ed_flags.offset(k.wrapping_div(4i32 as libc::c_uint) as
                                         isize) as libc::c_int &
                   __m_del[k.wrapping_rem(4i32 as libc::c_uint) as usize] as
                       libc::c_int {
            if *(*h).keys.offset(k as isize) == key { return k }
        }
        step = step.wrapping_add(1);
        k =
            k.wrapping_add(step) &
                (*h).n_buckets.wrapping_sub(1i32 as libc::c_uint)
    }
    return (*h).n_buckets;
}
#[no_mangle]
pub unsafe extern "C" fn kh_put_st(mut mrb: *mut mrb_state,
                                   mut h: *mut kh_st_t, mut key: mrb_sym,
                                   mut ret: *mut libc::c_int) -> khint_t {
    let mut k: khint_t = 0;
    let mut del_k: khint_t = 0;
    let mut step: khint_t = 0i32 as khint_t;
    if (*h).n_occupied >= (*h).n_buckets >> 2i32 | (*h).n_buckets >> 1i32 {
        kh_resize_st(mrb, h,
                     (*h).n_buckets.wrapping_mul(2i32 as libc::c_uint));
    }
    k =
        (key ^ key << 2i32 ^ key >> 2i32) as khint_t &
            (*h).n_buckets.wrapping_sub(1i32 as libc::c_uint);
    del_k = (*h).n_buckets;
    while 0 ==
              *(*h).ed_flags.offset(k.wrapping_div(4i32 as libc::c_uint) as
                                        isize) as libc::c_int &
                  __m_empty[k.wrapping_rem(4i32 as libc::c_uint) as usize] as
                      libc::c_int {
        if 0 ==
               *(*h).ed_flags.offset(k.wrapping_div(4i32 as libc::c_uint) as
                                         isize) as libc::c_int &
                   __m_del[k.wrapping_rem(4i32 as libc::c_uint) as usize] as
                       libc::c_int {
            if *(*h).keys.offset(k as isize) == key {
                if !ret.is_null() { *ret = 0i32 }
                return k
            }
        } else if del_k == (*h).n_buckets { del_k = k }
        step = step.wrapping_add(1);
        k =
            k.wrapping_add(step) &
                (*h).n_buckets.wrapping_sub(1i32 as libc::c_uint)
    }
    if del_k != (*h).n_buckets {
        *(*h).keys.offset(del_k as isize) = key;
        let ref mut fresh3 =
            *(*h).ed_flags.offset(del_k.wrapping_div(4i32 as libc::c_uint) as
                                      isize);
        *fresh3 =
            (*fresh3 as libc::c_int &
                 !(__m_del[del_k.wrapping_rem(4i32 as libc::c_uint) as usize]
                       as libc::c_int)) as uint8_t;
        (*h).size = (*h).size.wrapping_add(1);
        if !ret.is_null() { *ret = 2i32 }
        return del_k
    } else {
        *(*h).keys.offset(k as isize) = key;
        let ref mut fresh4 =
            *(*h).ed_flags.offset(k.wrapping_div(4i32 as libc::c_uint) as
                                      isize);
        *fresh4 =
            (*fresh4 as libc::c_int &
                 !(__m_empty[k.wrapping_rem(4i32 as libc::c_uint) as usize] as
                       libc::c_int)) as uint8_t;
        (*h).size = (*h).size.wrapping_add(1);
        (*h).n_occupied = (*h).n_occupied.wrapping_add(1);
        if !ret.is_null() { *ret = 1i32 }
        return k
    };
}
#[no_mangle]
pub unsafe extern "C" fn kh_resize_st(mut mrb: *mut mrb_state,
                                      mut h: *mut kh_st_t,
                                      mut new_n_buckets: khint_t) {
    if new_n_buckets < 8i32 as libc::c_uint {
        new_n_buckets = 8i32 as khint_t
    }
    new_n_buckets = new_n_buckets.wrapping_sub(1);
    new_n_buckets |= new_n_buckets >> 1i32;
    new_n_buckets |= new_n_buckets >> 2i32;
    new_n_buckets |= new_n_buckets >> 4i32;
    new_n_buckets |= new_n_buckets >> 8i32;
    new_n_buckets |= new_n_buckets >> 16i32;
    new_n_buckets = new_n_buckets.wrapping_add(1);
    let mut hh: kh_st_t =
        kh_st{n_buckets: 0,
              size: 0,
              n_occupied: 0,
              ed_flags: 0 as *mut uint8_t,
              keys: 0 as *mut mrb_sym,
              vals: 0 as *mut libc::c_char,};
    let mut old_ed_flags: *mut uint8_t = (*h).ed_flags;
    let mut old_keys: *mut mrb_sym = (*h).keys;
    let mut old_vals: *mut libc::c_char = (*h).vals;
    let mut old_n_buckets: khint_t = (*h).n_buckets;
    let mut i: khint_t = 0;
    hh.n_buckets = new_n_buckets;
    kh_alloc_st(mrb, &mut hh);
    i = 0i32 as khint_t;
    while i < old_n_buckets {
        if 0 ==
               *old_ed_flags.offset(i.wrapping_div(4i32 as libc::c_uint) as
                                        isize) as libc::c_int &
                   __m_either[i.wrapping_rem(4i32 as libc::c_uint) as usize]
                       as libc::c_int {
            let mut k: khint_t =
                kh_put_st(mrb, &mut hh, *old_keys.offset(i as isize),
                          0 as *mut libc::c_int);
        }
        i = i.wrapping_add(1)
    }
    *h = hh;
    mrb_free(mrb, old_keys as *mut libc::c_void);
}
#[no_mangle]
pub unsafe extern "C" fn kh_del_st(mut mrb: *mut mrb_state,
                                   mut h: *mut kh_st_t, mut x: khint_t) {
    if 0 !=
           !(x != (*h).n_buckets &&
                 0 ==
                     *(*h).ed_flags.offset(x.wrapping_div(4i32 as
                                                              libc::c_uint) as
                                               isize) as libc::c_int &
                         __m_either[x.wrapping_rem(4i32 as libc::c_uint) as
                                        usize] as libc::c_int) as libc::c_int
               as libc::c_long {
        __assert_rtn((*::std::mem::transmute::<&[u8; 10],
                                               &[libc::c_char; 10]>(b"kh_del_st\x00")).as_ptr(),
                     b"src/kernel.c\x00" as *const u8 as *const libc::c_char,
                     548i32,
                     b"x != h->n_buckets && !(h->ed_flags[(x)/4]&__m_either[(x)%4])\x00"
                         as *const u8 as *const libc::c_char);
    } else { };
    let ref mut fresh5 =
        *(*h).ed_flags.offset(x.wrapping_div(4i32 as libc::c_uint) as isize);
    *fresh5 =
        (*fresh5 as libc::c_int |
             __m_del[x.wrapping_rem(4i32 as libc::c_uint) as usize] as
                 libc::c_int) as uint8_t;
    (*h).size = (*h).size.wrapping_sub(1);
}
#[no_mangle]
pub unsafe extern "C" fn kh_copy_st(mut mrb: *mut mrb_state,
                                    mut h: *mut kh_st_t) -> *mut kh_st_t {
    let mut h2: *mut kh_st_t = 0 as *mut kh_st_t;
    let mut k: khiter_t = 0;
    let mut k2: khiter_t = 0;
    h2 = kh_init_st(mrb);
    k = 0i32 as khint_t;
    while k != (*h).n_buckets {
        if 0 ==
               *(*h).ed_flags.offset(k.wrapping_div(4i32 as libc::c_uint) as
                                         isize) as libc::c_int &
                   __m_either[k.wrapping_rem(4i32 as libc::c_uint) as usize]
                       as libc::c_int {
            k2 =
                kh_put_st(mrb, h2, *(*h).keys.offset(k as isize),
                          0 as *mut libc::c_int)
        }
        k = k.wrapping_add(1)
    }
    return h2;
}
/* 15.3.1.3.32 */
/*
 * call_seq:
 *   nil.nil?               -> true
 *   <anything_else>.nil?   -> false
 *
 * Only the object <i>nil</i> responds <code>true</code> to <code>nil?</code>.
 */
unsafe extern "C" fn mrb_false(mut mrb: *mut mrb_state, mut self_0: mrb_value)
 -> mrb_value {
    return mrb_false_value();
}
/* 15.3.1.3.41 */
/*
 *  call-seq:
 *     obj.remove_instance_variable(symbol)    -> obj
 *
 *  Removes the named instance variable from <i>obj</i>, returning that
 *  variable's value.
 *
 *     class Dummy
 *       attr_reader :var
 *       def initialize
 *         @var = 99
 *       end
 *       def remove
 *         remove_instance_variable(:@var)
 *       end
 *     end
 *     d = Dummy.new
 *     d.var      #=> 99
 *     d.remove   #=> 99
 *     d.var      #=> nil
 */
unsafe extern "C" fn mrb_obj_remove_instance_variable(mut mrb: *mut mrb_state,
                                                      mut self_0: mrb_value)
 -> mrb_value {
    let mut sym: mrb_sym = 0;
    let mut val: mrb_value =
        mrb_value{value: unnamed_0{f: 0.,}, tt: MRB_TT_FALSE,};
    mrb_get_args(mrb, b"n\x00" as *const u8 as *const libc::c_char,
                 &mut sym as *mut mrb_sym);
    mrb_iv_name_sym_check(mrb, sym);
    val = mrb_iv_remove(mrb, self_0, sym);
    if val.tt as libc::c_uint == MRB_TT_UNDEF as libc::c_int as libc::c_uint {
        mrb_name_error(mrb, sym,
                       b"instance variable %S not defined\x00" as *const u8 as
                           *const libc::c_char, mrb_sym2str(mrb, sym));
    }
    return val;
}
#[no_mangle]
pub unsafe extern "C" fn mrb_method_missing(mut mrb: *mut mrb_state,
                                            mut name: mrb_sym,
                                            mut self_0: mrb_value,
                                            mut args: mrb_value) {
    mrb_no_method_error(mrb, name, args,
                        b"undefined method \'%S\'\x00" as *const u8 as
                            *const libc::c_char, mrb_sym2str(mrb, name));
}
/* 15.3.1.3.30 */
/*
 *  call-seq:
 *     obj.method_missing(symbol [, *args] )   -> result
 *
 *  Invoked by Ruby when <i>obj</i> is sent a message it cannot handle.
 *  <i>symbol</i> is the symbol for the method called, and <i>args</i>
 *  are any arguments that were passed to it. By default, the interpreter
 *  raises an error when this method is called. However, it is possible
 *  to override the method to provide more dynamic behavior.
 *  If it is decided that a particular method should not be handled, then
 *  <i>super</i> should be called, so that ancestors can pick up the
 *  missing method.
 *  The example below creates
 *  a class <code>Roman</code>, which responds to methods with names
 *  consisting of roman numerals, returning the corresponding integer
 *  values.
 *
 *     class Roman
 *       def romanToInt(str)
 *         # ...
 *       end
 *       def method_missing(methId)
 *         str = methId.id2name
 *         romanToInt(str)
 *       end
 *     end
 *
 *     r = Roman.new
 *     r.iv      #=> 4
 *     r.xxiii   #=> 23
 *     r.mm      #=> 2000
 */
#[inline]
unsafe extern "C" fn basic_obj_respond_to(mut mrb: *mut mrb_state,
                                          mut obj: mrb_value, mut id: mrb_sym,
                                          mut pub_0: libc::c_int)
 -> mrb_bool {
    return mrb_respond_to(mrb, obj, id);
}
/* 15.3.1.3.43 */
/*
 *  call-seq:
 *     obj.respond_to?(symbol, include_private=false) -> true or false
 *
 *  Returns +true+ if _obj_ responds to the given
 *  method. Private methods are included in the search only if the
 *  optional second parameter evaluates to +true+.
 *
 *  If the method is not implemented,
 *  as Process.fork on Windows, File.lchmod on GNU/Linux, etc.,
 *  false is returned.
 *
 *  If the method is not defined, <code>respond_to_missing?</code>
 *  method is called and the result is returned.
 */
unsafe extern "C" fn obj_respond_to(mut mrb: *mut mrb_state,
                                    mut self_0: mrb_value) -> mrb_value {
    let mut id: mrb_sym = 0;
    let mut rtm_id: mrb_sym = 0;
    let mut priv_0: mrb_bool = 0i32 as mrb_bool;
    let mut respond_to_p: mrb_bool = 0;
    mrb_get_args(mrb, b"n|b\x00" as *const u8 as *const libc::c_char,
                 &mut id as *mut mrb_sym, &mut priv_0 as *mut mrb_bool);
    respond_to_p =
        basic_obj_respond_to(mrb, self_0, id, (0 == priv_0) as libc::c_int);
    if 0 == respond_to_p {
        rtm_id =
            mrb_intern_static(mrb,
                              b"respond_to_missing?\x00" as *const u8 as
                                  *const libc::c_char,
                              (::std::mem::size_of::<[libc::c_char; 20]>() as
                                   libc::c_ulong).wrapping_sub(1i32 as
                                                                   libc::c_ulong));
        if 0 !=
               basic_obj_respond_to(mrb, self_0, rtm_id,
                                    (0 == priv_0) as libc::c_int) {
            let mut args: [mrb_value; 2] =
                [mrb_value{value: unnamed_0{f: 0.,}, tt: MRB_TT_FALSE,}; 2];
            let mut v: mrb_value =
                mrb_value{value: unnamed_0{f: 0.,}, tt: MRB_TT_FALSE,};
            args[0usize] = mrb_symbol_value(id);
            args[1usize] = mrb_bool_value(priv_0);
            v =
                mrb_funcall_argv(mrb, self_0, rtm_id, 2i32 as mrb_int,
                                 args.as_mut_ptr());
            return mrb_bool_value((v.tt as libc::c_uint !=
                                       MRB_TT_FALSE as libc::c_int as
                                           libc::c_uint) as libc::c_int as
                                      mrb_bool)
        }
    }
    return mrb_bool_value(respond_to_p);
}
unsafe extern "C" fn mrb_obj_ceqq(mut mrb: *mut mrb_state,
                                  mut self_0: mrb_value) -> mrb_value {
    let mut v: mrb_value =
        mrb_value{value: unnamed_0{f: 0.,}, tt: MRB_TT_FALSE,};
    let mut i: mrb_int = 0;
    let mut len: mrb_int = 0;
    let mut eqq: mrb_sym =
        mrb_intern_static(mrb, b"===\x00" as *const u8 as *const libc::c_char,
                          (::std::mem::size_of::<[libc::c_char; 4]>() as
                               libc::c_ulong).wrapping_sub(1i32 as
                                                               libc::c_ulong));
    let mut ary: mrb_value = mrb_ary_splat(mrb, self_0);
    mrb_get_args(mrb, b"o\x00" as *const u8 as *const libc::c_char,
                 &mut v as *mut mrb_value);
    len =
        if 0 != (*(ary.value.p as *mut RArray)).flags() as libc::c_int & 7i32
           {
            (((*(ary.value.p as *mut RArray)).flags() as libc::c_int & 7i32) -
                 1i32) as mrb_int
        } else { (*(ary.value.p as *mut RArray)).as_0.heap.len };
    i = 0i32 as mrb_int;
    while i < len {
        let mut c: mrb_value =
            mrb_funcall_argv(mrb, mrb_ary_entry(ary, i), eqq, 1i32 as mrb_int,
                             &mut v);
        if c.tt as libc::c_uint != MRB_TT_FALSE as libc::c_int as libc::c_uint
           {
            return mrb_true_value()
        }
        i += 1
    }
    return mrb_false_value();
}
#[no_mangle]
pub unsafe extern "C" fn mrb_init_kernel(mut mrb: *mut mrb_state) {
    let mut krn: *mut RClass = 0 as *mut RClass;
    krn =
        mrb_define_module(mrb,
                          b"Kernel\x00" as *const u8 as *const libc::c_char);
    (*mrb).kernel_module = krn;
    mrb_define_class_method(mrb, krn,
                            b"block_given?\x00" as *const u8 as
                                *const libc::c_char,
                            Some(mrb_f_block_given_p_m), 0i32 as mrb_aspec);
    mrb_define_class_method(mrb, krn,
                            b"iterator?\x00" as *const u8 as
                                *const libc::c_char,
                            Some(mrb_f_block_given_p_m), 0i32 as mrb_aspec);
    mrb_define_class_method(mrb, krn,
                            b"raise\x00" as *const u8 as *const libc::c_char,
                            Some(mrb_f_raise),
                            ((2i32 & 0x1fi32) as mrb_aspec) << 13i32);
    mrb_define_method(mrb, krn,
                      b"===\x00" as *const u8 as *const libc::c_char,
                      Some(mrb_equal_m),
                      ((1i32 & 0x1fi32) as mrb_aspec) << 18i32);
    mrb_define_method(mrb, krn,
                      b"block_given?\x00" as *const u8 as *const libc::c_char,
                      Some(mrb_f_block_given_p_m), 0i32 as mrb_aspec);
    mrb_define_method(mrb, krn,
                      b"class\x00" as *const u8 as *const libc::c_char,
                      Some(mrb_obj_class_m), 0i32 as mrb_aspec);
    mrb_define_method(mrb, krn,
                      b"clone\x00" as *const u8 as *const libc::c_char,
                      Some(mrb_obj_clone), 0i32 as mrb_aspec);
    mrb_define_method(mrb, krn,
                      b"dup\x00" as *const u8 as *const libc::c_char,
                      Some(mrb_obj_dup), 0i32 as mrb_aspec);
    mrb_define_method(mrb, krn,
                      b"eql?\x00" as *const u8 as *const libc::c_char,
                      Some(mrb_obj_equal_m),
                      ((1i32 & 0x1fi32) as mrb_aspec) << 18i32);
    mrb_define_method(mrb, krn,
                      b"extend\x00" as *const u8 as *const libc::c_char,
                      Some(mrb_obj_extend_m), (1i32 << 12i32) as mrb_aspec);
    mrb_define_method(mrb, krn,
                      b"freeze\x00" as *const u8 as *const libc::c_char,
                      Some(mrb_obj_freeze), 0i32 as mrb_aspec);
    mrb_define_method(mrb, krn,
                      b"frozen?\x00" as *const u8 as *const libc::c_char,
                      Some(mrb_obj_frozen), 0i32 as mrb_aspec);
    mrb_define_method(mrb, krn,
                      b"hash\x00" as *const u8 as *const libc::c_char,
                      Some(mrb_obj_hash), 0i32 as mrb_aspec);
    mrb_define_method(mrb, krn,
                      b"initialize_copy\x00" as *const u8 as
                          *const libc::c_char, Some(mrb_obj_init_copy),
                      ((1i32 & 0x1fi32) as mrb_aspec) << 18i32);
    mrb_define_method(mrb, krn,
                      b"inspect\x00" as *const u8 as *const libc::c_char,
                      Some(mrb_obj_inspect), 0i32 as mrb_aspec);
    mrb_define_method(mrb, krn,
                      b"instance_of?\x00" as *const u8 as *const libc::c_char,
                      Some(obj_is_instance_of),
                      ((1i32 & 0x1fi32) as mrb_aspec) << 18i32);
    mrb_define_method(mrb, krn,
                      b"is_a?\x00" as *const u8 as *const libc::c_char,
                      Some(mrb_obj_is_kind_of_m),
                      ((1i32 & 0x1fi32) as mrb_aspec) << 18i32);
    mrb_define_method(mrb, krn,
                      b"iterator?\x00" as *const u8 as *const libc::c_char,
                      Some(mrb_f_block_given_p_m), 0i32 as mrb_aspec);
    mrb_define_method(mrb, krn,
                      b"kind_of?\x00" as *const u8 as *const libc::c_char,
                      Some(mrb_obj_is_kind_of_m),
                      ((1i32 & 0x1fi32) as mrb_aspec) << 18i32);
    mrb_define_method(mrb, krn,
                      b"nil?\x00" as *const u8 as *const libc::c_char,
                      Some(mrb_false), 0i32 as mrb_aspec);
    mrb_define_method(mrb, krn,
                      b"object_id\x00" as *const u8 as *const libc::c_char,
                      Some(mrb_obj_id_m), 0i32 as mrb_aspec);
    mrb_define_method(mrb, krn,
                      b"raise\x00" as *const u8 as *const libc::c_char,
                      Some(mrb_f_raise), (1i32 << 12i32) as mrb_aspec);
    mrb_define_method(mrb, krn,
                      b"remove_instance_variable\x00" as *const u8 as
                          *const libc::c_char,
                      Some(mrb_obj_remove_instance_variable),
                      ((1i32 & 0x1fi32) as mrb_aspec) << 18i32);
    mrb_define_method(mrb, krn,
                      b"respond_to?\x00" as *const u8 as *const libc::c_char,
                      Some(obj_respond_to), (1i32 << 12i32) as mrb_aspec);
    mrb_define_method(mrb, krn,
                      b"to_s\x00" as *const u8 as *const libc::c_char,
                      Some(mrb_any_to_s), 0i32 as mrb_aspec);
    mrb_define_method(mrb, krn,
                      b"__case_eqq\x00" as *const u8 as *const libc::c_char,
                      Some(mrb_obj_ceqq),
                      ((1i32 & 0x1fi32) as mrb_aspec) << 18i32);
    mrb_define_method(mrb, krn,
                      b"__to_int\x00" as *const u8 as *const libc::c_char,
                      Some(mrb_to_int), 0i32 as mrb_aspec);
    mrb_define_method(mrb, krn,
                      b"__to_str\x00" as *const u8 as *const libc::c_char,
                      Some(mrb_to_str), 0i32 as mrb_aspec);
    mrb_include_module(mrb, (*mrb).object_class, (*mrb).kernel_module);
    mrb_define_alias(mrb, (*mrb).module_class,
                     b"dup\x00" as *const u8 as *const libc::c_char,
                     b"clone\x00" as *const u8 as *const libc::c_char);
}