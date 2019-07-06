use libc;
use c2rust_bitfields::{BitfieldStruct};
extern "C" {
    pub type iv_tbl;
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
    fn mrb_obj_iv_set(mrb: *mut mrb_state, obj: *mut RObject, sym: mrb_sym,
                      v: mrb_value);
    #[no_mangle]
    fn mrb_intern_static(_: *mut mrb_state, _: *const libc::c_char, _: size_t)
     -> mrb_sym;
    #[no_mangle]
    fn mrb_obj_iv_set_force(mrb: *mut mrb_state, obj: *mut RObject,
                            sym: mrb_sym, v: mrb_value);
    #[no_mangle]
    fn mrb_sym2name(_: *mut mrb_state, _: mrb_sym) -> *const libc::c_char;
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
    #[no_mangle]
    fn mrb_sym2str(_: *mut mrb_state, _: mrb_sym) -> mrb_value;
    #[no_mangle]
    fn mrb_class_find_path(_: *mut mrb_state, _: *mut RClass) -> mrb_value;
    #[no_mangle]
    fn mrb_obj_iv_get(mrb: *mut mrb_state, obj: *mut RObject, sym: mrb_sym)
     -> mrb_value;
    #[no_mangle]
    fn mrb_obj_iv_defined(mrb: *mut mrb_state, obj: *mut RObject,
                          sym: mrb_sym) -> mrb_bool;
    #[no_mangle]
    fn mrb_field_write_barrier(_: *mut mrb_state, _: *mut RBasic,
                               _: *mut RBasic);
    #[no_mangle]
    fn mrb_calloc(_: *mut mrb_state, _: size_t, _: size_t)
     -> *mut libc::c_void;
    #[no_mangle]
    fn mrb_malloc(_: *mut mrb_state, _: size_t) -> *mut libc::c_void;
    #[no_mangle]
    fn mrb_obj_alloc(_: *mut mrb_state, _: mrb_vtype, _: *mut RClass)
     -> *mut RBasic;
    #[no_mangle]
    fn mrb_const_get(_: *mut mrb_state, _: mrb_value, _: mrb_sym)
     -> mrb_value;
    #[no_mangle]
    fn mrb_raise(mrb: *mut mrb_state, c: *mut RClass,
                 msg: *const libc::c_char) -> !;
    #[no_mangle]
    fn mrb_raisef(mrb: *mut mrb_state, c: *mut RClass,
                  fmt: *const libc::c_char, _: ...) -> !;
    #[no_mangle]
    fn mrb_check_type(mrb: *mut mrb_state, x: mrb_value, t: mrb_vtype);
    #[no_mangle]
    fn mrb_const_defined_at(mrb: *mut mrb_state, mod_0: mrb_value,
                            id: mrb_sym) -> mrb_bool;
    #[no_mangle]
    fn mrb_warn(mrb: *mut mrb_state, fmt: *const libc::c_char, _: ...);
    #[no_mangle]
    fn mrb_free(_: *mut mrb_state, _: *mut libc::c_void);
    #[no_mangle]
    fn mrb_frozen_error(mrb: *mut mrb_state, frozen_obj: *mut libc::c_void)
     -> !;
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
    #[no_mangle]
    fn mrb_name_error(mrb: *mut mrb_state, id: mrb_sym,
                      fmt: *const libc::c_char, _: ...) -> !;
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
    fn mrb_func_basic_p(mrb: *mut mrb_state, obj: mrb_value, mid: mrb_sym,
                        func: mrb_func_t) -> mrb_bool;
    #[no_mangle]
    fn mrb_check_intern_cstr(_: *mut mrb_state, _: *const libc::c_char)
     -> mrb_value;
    #[no_mangle]
    fn mrb_const_defined(_: *mut mrb_state, _: mrb_value, _: mrb_sym)
     -> mrb_bool;
    #[no_mangle]
    fn mrb_str_new(mrb: *mut mrb_state, p: *const libc::c_char, len: size_t)
     -> mrb_value;
    /*
 * Initializes a new array with initial values
 *
 * Equivalent to:
 *
 *      Array[value1, value2, ...]
 *
 * @param mrb The mruby state reference.
 * @param size The numer of values.
 * @param vals The actual values.
 * @return The initialized array.
 */
    #[no_mangle]
    fn mrb_ary_new_from_values(mrb: *mut mrb_state, size: mrb_int,
                               vals: *const mrb_value) -> mrb_value;
    #[no_mangle]
    fn mrb_data_get_ptr(mrb: *mut mrb_state, _: mrb_value,
                        _: *const mrb_data_type) -> *mut libc::c_void;
    #[no_mangle]
    fn mrb_obj_to_sym(mrb: *mut mrb_state, name: mrb_value) -> mrb_sym;
    #[no_mangle]
    fn mrb_to_int(mrb: *mut mrb_state, val: mrb_value) -> mrb_value;
    #[no_mangle]
    fn mrb_to_flo(mrb: *mut mrb_state, x: mrb_value) -> mrb_float;
    #[no_mangle]
    fn mrb_str_new_static(mrb: *mut mrb_state, p: *const libc::c_char,
                          len: size_t) -> mrb_value;
    #[no_mangle]
    fn mrb_string_value_cstr(mrb: *mut mrb_state, ptr: *mut mrb_value)
     -> *const libc::c_char;
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
 * Call existing ruby functions with a block.
 */
    #[no_mangle]
    fn mrb_funcall_with_block(_: *mut mrb_state, _: mrb_value, _: mrb_sym,
                              _: mrb_int, _: *const mrb_value, _: mrb_value)
     -> mrb_value;
    #[no_mangle]
    fn mrb_intern(_: *mut mrb_state, _: *const libc::c_char, _: size_t)
     -> mrb_sym;
    #[no_mangle]
    fn mrb_intern_str(_: *mut mrb_state, _: mrb_value) -> mrb_sym;
    #[no_mangle]
    fn mrb_sym2name_len(_: *mut mrb_state, _: mrb_sym, _: *mut mrb_int)
     -> *const libc::c_char;
    #[no_mangle]
    fn mrb_obj_equal(_: *mut mrb_state, _: mrb_value, _: mrb_value)
     -> mrb_bool;
    #[no_mangle]
    fn mrb_equal(mrb: *mut mrb_state, obj1: mrb_value, obj2: mrb_value)
     -> mrb_bool;
    #[no_mangle]
    fn mrb_inspect(mrb: *mut mrb_state, obj: mrb_value) -> mrb_value;
    #[no_mangle]
    fn mrb_gc_mark(_: *mut mrb_state, _: *mut RBasic);
    #[no_mangle]
    fn mrb_any_to_s(mrb: *mut mrb_state, obj: mrb_value) -> mrb_value;
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
    #[no_mangle]
    fn mrb_obj_is_kind_of(mrb: *mut mrb_state, obj: mrb_value, c: *mut RClass)
     -> mrb_bool;
    #[no_mangle]
    fn mrb_yield_with_class(mrb: *mut mrb_state, b: mrb_value, argc: mrb_int,
                            argv: *const mrb_value, self_0: mrb_value,
                            c: *mut RClass) -> mrb_value;
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
    fn mrb_ident_p(s: *const libc::c_char, len: mrb_int) -> mrb_bool;
    #[no_mangle]
    fn mrb_proc_copy(a: *mut RProc, b: *mut RProc);
    /* implementation of #send method */
    #[no_mangle]
    fn mrb_f_send(mrb: *mut mrb_state, self_0: mrb_value) -> mrb_value;
    /* following functions are defined in mruby-proc-ext so please include it when using */
    #[no_mangle]
    fn mrb_proc_new_cfunc_with_env(_: *mut mrb_state, _: mrb_func_t,
                                   _: mrb_int, _: *const mrb_value)
     -> *mut RProc;
    #[no_mangle]
    fn mrb_proc_cfunc_env_get(_: *mut mrb_state, _: mrb_int) -> mrb_value;
    /*
 * Finds the index of a substring in a string
 */
    #[no_mangle]
    fn mrb_str_index(_: *mut mrb_state, _: mrb_value, _: *const libc::c_char,
                     _: mrb_int, _: mrb_int) -> mrb_int;
    /*
 * Returns a Ruby string type.
 *
 *
 * @param [mrb_state] mrb The current mruby state.
 * @param [mrb_value] str Ruby string.
 * @return [mrb_value] A Ruby string.
 */
    #[no_mangle]
    fn mrb_ensure_string_type(mrb: *mut mrb_state, str: mrb_value)
     -> mrb_value;
    #[no_mangle]
    fn mrb_str_new_capa(mrb: *mut mrb_state, capa: size_t) -> mrb_value;
    #[no_mangle]
    fn mrb_str_cat_str(mrb: *mut mrb_state, str: mrb_value, str2: mrb_value)
     -> mrb_value;
    #[no_mangle]
    fn mrb_const_set(_: *mut mrb_state, _: mrb_value, _: mrb_sym,
                     _: mrb_value);
    #[no_mangle]
    fn mrb_iv_name_sym_check(mrb: *mut mrb_state, sym: mrb_sym);
    #[no_mangle]
    fn mrb_iv_get(mrb: *mut mrb_state, obj: mrb_value, sym: mrb_sym)
     -> mrb_value;
    #[no_mangle]
    fn mrb_iv_set(mrb: *mut mrb_state, obj: mrb_value, sym: mrb_sym,
                  v: mrb_value);
    #[no_mangle]
    fn mrb_iv_remove(mrb: *mut mrb_state, obj: mrb_value, sym: mrb_sym)
     -> mrb_value;
    /* implementation of module_eval/class_eval */
    #[no_mangle]
    fn mrb_mod_module_eval(_: *mut mrb_state, _: mrb_value) -> mrb_value;
    /* implementation of __id__ */
    #[no_mangle]
    fn mrb_obj_id_m(mrb: *mut mrb_state, self_0: mrb_value) -> mrb_value;
    /* implementation of instance_eval */
    #[no_mangle]
    fn mrb_obj_instance_eval(_: *mut mrb_state, _: mrb_value) -> mrb_value;
}
pub type __builtin_va_list = [__va_list_tag; 1];
#[derive ( Copy , Clone )]
#[repr(C)]
pub struct __va_list_tag {
    pub gp_offset: libc::c_uint,
    pub fp_offset: libc::c_uint,
    pub overflow_arg_area: *mut libc::c_void,
    pub reg_save_area: *mut libc::c_void,
}
pub type va_list = __builtin_va_list;
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
pub type kh_mt_t = kh_mt;
pub type khiter_t = khint_t;
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
    pub ptr: *mut mrb_value,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub union unnamed_4 {
    pub capa: mrb_int,
    pub shared: *mut mrb_shared_array,
}
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
#[derive ( Copy , Clone )]
#[repr ( C )]
pub union unnamed_5 {
    pub heap: unnamed_3,
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
    pub as_0: unnamed_5,
}
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
#[derive ( Copy , Clone )]
#[repr(C)]
pub struct unnamed_6 {
    pub len: mrb_int,
    pub aux: unnamed_7,
    pub ptr: *mut libc::c_char,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub union unnamed_7 {
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
    pub as_0: unnamed_8,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub union unnamed_8 {
    pub heap: unnamed_6,
    pub ary: [libc::c_char; 24],
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
#[inline]
unsafe extern "C" fn mrb_undef_value() -> mrb_value {
    let mut v: mrb_value =
        mrb_value{value: unnamed_0{f: 0.,}, tt: MRB_TT_FALSE,};
    v.tt = MRB_TT_UNDEF;
    v.value.i = 0i32 as mrb_int;
    return v;
}
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
pub unsafe extern "C" fn mrb_define_class(mut mrb: *mut mrb_state,
                                          mut name: *const libc::c_char,
                                          mut super_0: *mut RClass)
 -> *mut RClass {
    return mrb_define_class_id(mrb, mrb_intern_cstr(mrb, name), super_0);
}
/* flags:
   20: frozen
   19: is_prepended
   18: is_origin
   17: is_inherited (used by method cache)
   16: unused
   0-15: instance type
*/
#[no_mangle]
pub unsafe extern "C" fn mrb_define_class_id(mut mrb: *mut mrb_state,
                                             mut name: mrb_sym,
                                             mut super_0: *mut RClass)
 -> *mut RClass {
    if super_0.is_null() {
        mrb_warn(mrb,
                 b"no super class for \'%S\', Object assumed\x00" as *const u8
                     as *const libc::c_char, mrb_sym2str(mrb, name));
    }
    return define_class(mrb, name, super_0, (*mrb).object_class);
}
unsafe extern "C" fn define_class(mut mrb: *mut mrb_state, mut name: mrb_sym,
                                  mut super_0: *mut RClass,
                                  mut outer: *mut RClass) -> *mut RClass {
    let mut c: *mut RClass = 0 as *mut RClass;
    if 0 !=
           mrb_const_defined_at(mrb,
                                mrb_obj_value(outer as *mut libc::c_void),
                                name) {
        c = class_from_sym(mrb, outer, name);
        if 0 != (*c).flags() as libc::c_int & 1i32 << 19i32 {
            c = (*c).super_0;
            while 0 == (*c).flags() as libc::c_int & 1i32 << 18i32 {
                c = (*c).super_0
            }
        }
        if !super_0.is_null() && mrb_class_real((*c).super_0) != super_0 {
            mrb_raisef(mrb,
                       mrb_exc_get(mrb,
                                   b"TypeError\x00" as *const u8 as
                                       *const libc::c_char),
                       b"superclass mismatch for Class %S (%S not %S)\x00" as
                           *const u8 as *const libc::c_char,
                       mrb_sym2str(mrb, name),
                       mrb_obj_value((*c).super_0 as *mut libc::c_void),
                       mrb_obj_value(super_0 as *mut libc::c_void));
        }
        return c
    }
    c = mrb_class_new(mrb, super_0);
    setup_class(mrb, outer, c, name);
    return c;
}
unsafe extern "C" fn setup_class(mut mrb: *mut mrb_state,
                                 mut outer: *mut RClass, mut c: *mut RClass,
                                 mut id: mrb_sym) {
    mrb_class_name_class(mrb, outer, c, id);
    mrb_obj_iv_set(mrb, outer as *mut RObject, id,
                   mrb_obj_value(c as *mut libc::c_void));
}
#[no_mangle]
pub unsafe extern "C" fn mrb_class_name_class(mut mrb: *mut mrb_state,
                                              mut outer: *mut RClass,
                                              mut c: *mut RClass,
                                              mut id: mrb_sym) {
    let mut name: mrb_value =
        mrb_value{value: unnamed_0{f: 0.,}, tt: MRB_TT_FALSE,};
    let mut nsym: mrb_sym =
        mrb_intern_static(mrb,
                          b"__classname__\x00" as *const u8 as
                              *const libc::c_char,
                          (::std::mem::size_of::<[libc::c_char; 14]>() as
                               libc::c_ulong).wrapping_sub(1i32 as
                                                               libc::c_ulong));
    if 0 != mrb_obj_iv_defined(mrb, c as *mut RObject, nsym) { return }
    if outer.is_null() || outer == (*mrb).object_class {
        name = mrb_symbol_value(id)
    } else {
        name = mrb_class_path(mrb, outer);
        if name.tt as libc::c_uint ==
               MRB_TT_FALSE as libc::c_int as libc::c_uint &&
               0 == name.value.i {
            if outer != (*mrb).object_class && outer != c {
                mrb_obj_iv_set_force(mrb, c as *mut RObject,
                                     mrb_intern_static(mrb,
                                                       b"__outer__\x00" as
                                                           *const u8 as
                                                           *const libc::c_char,
                                                       (::std::mem::size_of::<[libc::c_char; 10]>()
                                                            as
                                                            libc::c_ulong).wrapping_sub(1i32
                                                                                            as
                                                                                            libc::c_ulong)),
                                     mrb_obj_value(outer as
                                                       *mut libc::c_void));
            }
            return
        }
        mrb_str_cat_cstr(mrb, name,
                         b"::\x00" as *const u8 as *const libc::c_char);
        mrb_str_cat_cstr(mrb, name, mrb_sym2name(mrb, id));
    }
    mrb_obj_iv_set_force(mrb, c as *mut RObject, nsym, name);
}
#[no_mangle]
pub unsafe extern "C" fn mrb_class_path(mut mrb: *mut mrb_state,
                                        mut c: *mut RClass) -> mrb_value {
    let mut path: mrb_value =
        mrb_value{value: unnamed_0{f: 0.,}, tt: MRB_TT_FALSE,};
    let mut nsym: mrb_sym =
        mrb_intern_static(mrb,
                          b"__classname__\x00" as *const u8 as
                              *const libc::c_char,
                          (::std::mem::size_of::<[libc::c_char; 14]>() as
                               libc::c_ulong).wrapping_sub(1i32 as
                                                               libc::c_ulong));
    path = mrb_obj_iv_get(mrb, c as *mut RObject, nsym);
    if path.tt as libc::c_uint == MRB_TT_FALSE as libc::c_int as libc::c_uint
           && 0 == path.value.i {
        return mrb_class_find_path(mrb, c)
    } else {
        if path.tt as libc::c_uint ==
               MRB_TT_SYMBOL as libc::c_int as libc::c_uint {
            return mrb_sym2str(mrb, path.value.sym)
        }
    }
    return mrb_str_dup(mrb, path);
}
/* *
 * Creates a new instance of Class, Class.
 *
 * Example:
 *
 *      void
 *      mrb_example_gem_init(mrb_state* mrb) {
 *        struct RClass *example_class;
 *
 *        mrb_value obj;
 *        example_class = mrb_class_new(mrb, mrb->object_class);
 *        obj = mrb_obj_new(mrb, example_class, 0, NULL); // => #<#<Class:0x9a945b8>:0x9a94588>
 *        mrb_p(mrb, obj); // => Kernel#p
 *       }
 *
 * @param [mrb_state*] mrb The current mruby state.
 * @param [struct RClass *] super The super class or parent.
 * @return [struct RClass *] Reference to the new class.
 */
#[no_mangle]
pub unsafe extern "C" fn mrb_class_new(mut mrb: *mut mrb_state,
                                       mut super_0: *mut RClass)
 -> *mut RClass {
    let mut c: *mut RClass = 0 as *mut RClass;
    if !super_0.is_null() { mrb_check_inheritable(mrb, super_0); }
    c = boot_defclass(mrb, super_0);
    if !super_0.is_null() {
        (*c).set_flags(((*c).flags() as libc::c_int & !0xffi32 |
                            ((*super_0).flags() as libc::c_int & 0xffi32) as
                                mrb_vtype as libc::c_char as libc::c_int) as
                           uint32_t)
    }
    prepare_singleton_class(mrb, c as *mut RBasic);
    return c;
}
unsafe extern "C" fn prepare_singleton_class(mut mrb: *mut mrb_state,
                                             mut o: *mut RBasic) {
    let mut sc: *mut RClass = 0 as *mut RClass;
    let mut c: *mut RClass = 0 as *mut RClass;
    if (*(*o).c).tt() as libc::c_int == MRB_TT_SCLASS as libc::c_int {
        return
    }
    sc = mrb_obj_alloc(mrb, MRB_TT_SCLASS, (*mrb).class_class) as *mut RClass;
    (*sc).set_flags((*sc).flags() | (1i32 << 17i32) as uint32_t);
    (*sc).mt = kh_init_mt(mrb);
    (*sc).iv = 0 as *mut iv_tbl;
    if (*o).tt() as libc::c_int == MRB_TT_CLASS as libc::c_int {
        c = o as *mut RClass;
        if (*c).super_0.is_null() {
            (*sc).super_0 = (*mrb).class_class
        } else { (*sc).super_0 = (*(*c).super_0).c }
    } else if (*o).tt() as libc::c_int == MRB_TT_SCLASS as libc::c_int {
        c = o as *mut RClass;
        while (*(*c).super_0).tt() as libc::c_int ==
                  MRB_TT_ICLASS as libc::c_int {
            c = (*c).super_0
        }
        prepare_singleton_class(mrb, (*c).super_0 as *mut RBasic);
        (*sc).super_0 = (*(*c).super_0).c
    } else {
        (*sc).super_0 = (*o).c;
        prepare_singleton_class(mrb, sc as *mut RBasic);
    }
    (*o).c = sc;
    mrb_field_write_barrier(mrb, o as *mut RBasic, sc as *mut RBasic);
    mrb_field_write_barrier(mrb, sc as *mut RBasic, o as *mut RBasic);
    mrb_obj_iv_set(mrb, sc as *mut RObject,
                   mrb_intern_static(mrb,
                                     b"__attached__\x00" as *const u8 as
                                         *const libc::c_char,
                                     (::std::mem::size_of::<[libc::c_char; 13]>()
                                          as
                                          libc::c_ulong).wrapping_sub(1i32 as
                                                                          libc::c_ulong)),
                   mrb_obj_value(o as *mut libc::c_void));
    (*sc).set_flags((*sc).flags() |
                        ((*o).flags() as libc::c_int & 1i32 << 20i32) as
                            uint32_t);
}
#[no_mangle]
pub unsafe extern "C" fn kh_init_mt(mut mrb: *mut mrb_state) -> *mut kh_mt_t {
    return kh_init_mt_size(mrb, 32i32 as khint_t);
}
#[no_mangle]
pub unsafe extern "C" fn kh_init_mt_size(mut mrb: *mut mrb_state,
                                         mut size: khint_t) -> *mut kh_mt_t {
    let mut h: *mut kh_mt_t =
        mrb_calloc(mrb, 1i32 as size_t,
                   ::std::mem::size_of::<kh_mt_t>() as libc::c_ulong) as
            *mut kh_mt_t;
    if size < 8i32 as libc::c_uint { size = 8i32 as khint_t }
    size = size.wrapping_sub(1);
    size |= size >> 1i32;
    size |= size >> 2i32;
    size |= size >> 4i32;
    size |= size >> 8i32;
    size |= size >> 16i32;
    size = size.wrapping_add(1);
    (*h).n_buckets = size;
    kh_alloc_mt(mrb, h);
    return h;
}
#[no_mangle]
pub unsafe extern "C" fn kh_alloc_mt(mut mrb: *mut mrb_state,
                                     mut h: *mut kh_mt_t) {
    let mut sz: khint_t = (*h).n_buckets;
    let mut len: size_t =
        (::std::mem::size_of::<mrb_sym>() as
             libc::c_ulong).wrapping_add(if 0 != 1i32 {
                                             ::std::mem::size_of::<mrb_method_t>()
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
        if 0 != 1i32 {
            p.offset((::std::mem::size_of::<mrb_sym>() as
                          libc::c_ulong).wrapping_mul(sz as libc::c_ulong) as
                         isize) as *mut mrb_method_t
        } else { 0 as *mut mrb_method_t };
    (*h).ed_flags = p.offset(len.wrapping_mul(sz as libc::c_ulong) as isize);
    kh_fill_flags((*h).ed_flags, 0xaai32 as uint8_t,
                  sz.wrapping_div(4i32 as libc::c_uint) as size_t);
}
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
unsafe extern "C" fn boot_defclass(mut mrb: *mut mrb_state,
                                   mut super_0: *mut RClass) -> *mut RClass {
    let mut c: *mut RClass = 0 as *mut RClass;
    c = mrb_obj_alloc(mrb, MRB_TT_CLASS, (*mrb).class_class) as *mut RClass;
    if !super_0.is_null() {
        (*c).super_0 = super_0;
        mrb_field_write_barrier(mrb, c as *mut RBasic,
                                super_0 as *mut RBasic);
    } else { (*c).super_0 = (*mrb).object_class }
    (*c).mt = kh_init_mt(mrb);
    return c;
}
/* !
 * Ensures a class can be derived from super.
 *
 * \param super a reference to an object.
 * \exception TypeError if \a super is not a Class or \a super is a singleton class.
 */
unsafe extern "C" fn mrb_check_inheritable(mut mrb: *mut mrb_state,
                                           mut super_0: *mut RClass) {
    if (*super_0).tt() as libc::c_int != MRB_TT_CLASS as libc::c_int {
        mrb_raisef(mrb,
                   mrb_exc_get(mrb,
                               b"TypeError\x00" as *const u8 as
                                   *const libc::c_char),
                   b"superclass must be a Class (%S given)\x00" as *const u8
                       as *const libc::c_char,
                   mrb_obj_value(super_0 as *mut libc::c_void));
    }
    if (*super_0).tt() as libc::c_int == MRB_TT_SCLASS as libc::c_int {
        mrb_raise(mrb,
                  mrb_exc_get(mrb,
                              b"TypeError\x00" as *const u8 as
                                  *const libc::c_char),
                  b"can\'t make subclass of singleton class\x00" as *const u8
                      as *const libc::c_char);
    }
    if super_0 == (*mrb).class_class {
        mrb_raise(mrb,
                  mrb_exc_get(mrb,
                              b"TypeError\x00" as *const u8 as
                                  *const libc::c_char),
                  b"can\'t make subclass of Class\x00" as *const u8 as
                      *const libc::c_char);
    };
}
/* *
 * Gets a exception class.
 * @param [mrb_state*] mrb The current mruby state.
 * @param [const char *] name The name of the class.
 * @return [struct RClass *] A reference to the class.
*/
#[no_mangle]
pub unsafe extern "C" fn mrb_exc_get(mut mrb: *mut mrb_state,
                                     mut name: *const libc::c_char)
 -> *mut RClass {
    let mut exc: *mut RClass = 0 as *mut RClass;
    let mut e: *mut RClass = 0 as *mut RClass;
    let mut c: mrb_value =
        mrb_const_get(mrb,
                      mrb_obj_value((*mrb).object_class as *mut libc::c_void),
                      mrb_intern_cstr(mrb, name));
    if c.tt as libc::c_uint != MRB_TT_CLASS as libc::c_int as libc::c_uint {
        mrb_raise(mrb, (*mrb).eException_class,
                  b"exception corrupted\x00" as *const u8 as
                      *const libc::c_char);
    }
    e = c.value.p as *mut RClass;
    exc = e;
    while !e.is_null() {
        if e == (*mrb).eException_class { return exc }
        e = (*e).super_0
    }
    return (*mrb).eException_class;
}
#[no_mangle]
pub unsafe extern "C" fn mrb_class_real(mut cl: *mut RClass) -> *mut RClass {
    if cl.is_null() { return 0 as *mut RClass }
    while (*cl).tt() as libc::c_int == MRB_TT_SCLASS as libc::c_int ||
              (*cl).tt() as libc::c_int == MRB_TT_ICLASS as libc::c_int {
        cl = (*cl).super_0;
        if cl.is_null() { return 0 as *mut RClass }
    }
    return cl;
}
unsafe extern "C" fn class_from_sym(mut mrb: *mut mrb_state,
                                    mut klass: *mut RClass, mut id: mrb_sym)
 -> *mut RClass {
    let mut c: mrb_value =
        mrb_const_get(mrb, mrb_obj_value(klass as *mut libc::c_void), id);
    mrb_check_type(mrb, c, MRB_TT_CLASS);
    return c.value.p as *mut RClass;
}
/* *
 * Defines a new module.
 *
 * @param [mrb_state *] mrb_state* The current mruby state.
 * @param [const char *] char* The name of the module.
 * @return [struct RClass *] Reference to the newly defined module.
 */
#[no_mangle]
pub unsafe extern "C" fn mrb_define_module(mut mrb: *mut mrb_state,
                                           mut name: *const libc::c_char)
 -> *mut RClass {
    return define_module(mrb, mrb_intern_cstr(mrb, name),
                         (*mrb).object_class);
}
unsafe extern "C" fn define_module(mut mrb: *mut mrb_state, mut name: mrb_sym,
                                   mut outer: *mut RClass) -> *mut RClass {
    let mut m: *mut RClass = 0 as *mut RClass;
    if 0 !=
           mrb_const_defined_at(mrb,
                                mrb_obj_value(outer as *mut libc::c_void),
                                name) {
        return module_from_sym(mrb, outer, name)
    }
    m = mrb_module_new(mrb);
    setup_class(mrb, outer, m, name);
    return m;
}
/* *
 * Creates a new module, Module.
 *
 * Example:
 *      void
 *      mrb_example_gem_init(mrb_state* mrb) {
 *        struct RClass *example_module;
 *
 *        example_module = mrb_module_new(mrb);
 *      }
 *
 * @param [mrb_state*] mrb The current mruby state.
 * @return [struct RClass *] Reference to the new module.
 */
#[no_mangle]
pub unsafe extern "C" fn mrb_module_new(mut mrb: *mut mrb_state)
 -> *mut RClass {
    let mut m: *mut RClass =
        mrb_obj_alloc(mrb, MRB_TT_MODULE, (*mrb).module_class) as *mut RClass;
    boot_initmod(mrb, m);
    return m;
}
unsafe extern "C" fn boot_initmod(mut mrb: *mut mrb_state,
                                  mut mod_0: *mut RClass) {
    if (*mod_0).mt.is_null() { (*mod_0).mt = kh_init_mt(mrb) };
}
unsafe extern "C" fn module_from_sym(mut mrb: *mut mrb_state,
                                     mut klass: *mut RClass, mut id: mrb_sym)
 -> *mut RClass {
    let mut c: mrb_value =
        mrb_const_get(mrb, mrb_obj_value(klass as *mut libc::c_void), id);
    mrb_check_type(mrb, c, MRB_TT_MODULE);
    return c.value.p as *mut RClass;
}
#[no_mangle]
pub unsafe extern "C" fn mrb_singleton_class(mut mrb: *mut mrb_state,
                                             mut v: mrb_value) -> mrb_value {
    let mut obj: *mut RBasic = 0 as *mut RBasic;
    match v.tt as libc::c_uint {
        0 => {
            if v.tt as libc::c_uint ==
                   MRB_TT_FALSE as libc::c_int as libc::c_uint &&
                   0 == v.value.i {
                return mrb_obj_value((*mrb).nil_class as *mut libc::c_void)
            }
            return mrb_obj_value((*mrb).false_class as *mut libc::c_void)
        }
        2 => { return mrb_obj_value((*mrb).true_class as *mut libc::c_void) }
        7 => {
            return mrb_obj_value((*mrb).object_class as *mut libc::c_void)
        }
        4 | 3 | 6 => {
            mrb_raise(mrb,
                      mrb_exc_get(mrb,
                                  b"TypeError\x00" as *const u8 as
                                      *const libc::c_char),
                      b"can\'t define singleton\x00" as *const u8 as
                          *const libc::c_char);
        }
        _ => { }
    }
    obj = v.value.p as *mut RBasic;
    prepare_singleton_class(mrb, obj);
    return mrb_obj_value((*obj).c as *mut libc::c_void);
}
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
pub unsafe extern "C" fn mrb_include_module(mut mrb: *mut mrb_state,
                                            mut c: *mut RClass,
                                            mut m: *mut RClass) {
    let mut changed: libc::c_int =
        include_module_at(mrb, c, find_origin(c), m, 1i32);
    if changed < 0i32 {
        mrb_raise(mrb,
                  mrb_exc_get(mrb,
                              b"ArgumentError\x00" as *const u8 as
                                  *const libc::c_char),
                  b"cyclic include detected\x00" as *const u8 as
                      *const libc::c_char);
    };
}
unsafe extern "C" fn find_origin(mut c: *mut RClass) -> *mut RClass {
    if 0 != (*c).flags() as libc::c_int & 1i32 << 19i32 {
        c = (*c).super_0;
        while 0 == (*c).flags() as libc::c_int & 1i32 << 18i32 {
            c = (*c).super_0
        }
    }
    return c;
}
unsafe extern "C" fn include_module_at(mut mrb: *mut mrb_state,
                                       mut c: *mut RClass,
                                       mut ins_pos: *mut RClass,
                                       mut m: *mut RClass,
                                       mut search_super: libc::c_int)
 -> libc::c_int {
    let mut current_block: u64;
    let mut p: *mut RClass = 0 as *mut RClass;
    let mut ic: *mut RClass = 0 as *mut RClass;
    let mut klass_mt: *mut libc::c_void =
        (*find_origin(c)).mt as *mut libc::c_void;
    while !m.is_null() {
        let mut superclass_seen: libc::c_int = 0i32;
        if !(0 != (*m).flags() as libc::c_int & 1i32 << 19i32) {
            if !klass_mt.is_null() && klass_mt == (*m).mt as *mut libc::c_void
               {
                return -1i32
            }
            p = (*c).super_0;
            loop  {
                if p.is_null() {
                    current_block = 15904375183555213903;
                    break ;
                }
                if (*p).tt() as libc::c_int == MRB_TT_ICLASS as libc::c_int {
                    if (*p).mt == (*m).mt {
                        if 0 == superclass_seen { ins_pos = p }
                        current_block = 12402295122174744493;
                        break ;
                    }
                } else if (*p).tt() as libc::c_int ==
                              MRB_TT_CLASS as libc::c_int {
                    if 0 == search_super {
                        current_block = 15904375183555213903;
                        break ;
                    }
                    superclass_seen = 1i32
                }
                p = (*p).super_0
            }
            match current_block {
                12402295122174744493 => { }
                _ => {
                    ic = include_class_new(mrb, m, (*ins_pos).super_0);
                    (*m).set_flags((*m).flags() |
                                       (1i32 << 17i32) as uint32_t);
                    (*ins_pos).super_0 = ic;
                    mrb_field_write_barrier(mrb, ins_pos as *mut RBasic,
                                            ic as *mut RBasic);
                    ins_pos = ic
                }
            }
        }
        m = (*m).super_0
    }
    return 0i32;
}
unsafe extern "C" fn include_class_new(mut mrb: *mut mrb_state,
                                       mut m: *mut RClass,
                                       mut super_0: *mut RClass)
 -> *mut RClass {
    let mut ic: *mut RClass =
        mrb_obj_alloc(mrb, MRB_TT_ICLASS, (*mrb).class_class) as *mut RClass;
    if (*m).tt() as libc::c_int == MRB_TT_ICLASS as libc::c_int { m = (*m).c }
    if 0 != (*m).flags() as libc::c_int & 1i32 << 19i32 {
        m = (*m).super_0;
        while 0 == (*m).flags() as libc::c_int & 1i32 << 18i32 {
            m = (*m).super_0
        }
    }
    (*ic).iv = (*m).iv;
    (*ic).mt = (*m).mt;
    (*ic).super_0 = super_0;
    if (*m).tt() as libc::c_int == MRB_TT_ICLASS as libc::c_int {
        (*ic).c = (*m).c
    } else { (*ic).c = m }
    return ic;
}
/* *
 * Prepends a module in another class or module.
 *
 * Equivalent to:
 *  module B
 *    prepend A
 *  end
 * @param [mrb_state *] mrb_state* The current mruby state.
 * @param [struct RClass *] RClass* A reference to module or a class.
 * @param [struct RClass *] RClass* A reference to the module to be prepended.
 */
#[no_mangle]
pub unsafe extern "C" fn mrb_prepend_module(mut mrb: *mut mrb_state,
                                            mut c: *mut RClass,
                                            mut m: *mut RClass) {
    let mut origin: *mut RClass = 0 as *mut RClass;
    let mut changed: libc::c_int = 0i32;
    if 0 == (*c).flags() as libc::c_int & 1i32 << 19i32 {
        origin = mrb_obj_alloc(mrb, MRB_TT_ICLASS, c) as *mut RClass;
        (*origin).set_flags((*origin).flags() |
                                (1i32 << 18i32 | 1i32 << 17i32) as uint32_t);
        (*origin).super_0 = (*c).super_0;
        (*c).super_0 = origin;
        (*origin).mt = (*c).mt;
        (*c).mt = kh_init_mt(mrb);
        mrb_field_write_barrier(mrb, c as *mut RBasic, origin as *mut RBasic);
        (*c).set_flags((*c).flags() | (1i32 << 19i32) as uint32_t)
    }
    changed = include_module_at(mrb, c, c, m, 0i32);
    if changed < 0i32 {
        mrb_raise(mrb,
                  mrb_exc_get(mrb,
                              b"ArgumentError\x00" as *const u8 as
                                  *const libc::c_char),
                  b"cyclic prepend detected\x00" as *const u8 as
                      *const libc::c_char);
    };
}
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
pub unsafe extern "C" fn mrb_define_method(mut mrb: *mut mrb_state,
                                           mut c: *mut RClass,
                                           mut name: *const libc::c_char,
                                           mut func: mrb_func_t,
                                           mut aspec: mrb_aspec) {
    mrb_define_method_id(mrb, c, mrb_intern_cstr(mrb, name), func, aspec);
}
#[no_mangle]
pub unsafe extern "C" fn mrb_define_method_id(mut mrb: *mut mrb_state,
                                              mut c: *mut RClass,
                                              mut mid: mrb_sym,
                                              mut func: mrb_func_t,
                                              mut aspec: mrb_aspec) {
    let mut m: mrb_method_t =
        mrb_method_t{func_p: 0, unnamed: unnamed{proc_0: 0 as *mut RProc,},};
    let mut ai: libc::c_int = mrb_gc_arena_save(mrb);
    m.func_p = 1i32 as mrb_bool;
    m.unnamed.func = func;
    mrb_define_method_raw(mrb, c, mid, m);
    mrb_gc_arena_restore(mrb, ai);
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
#[no_mangle]
pub unsafe extern "C" fn mrb_define_method_raw(mut mrb: *mut mrb_state,
                                               mut c: *mut RClass,
                                               mut mid: mrb_sym,
                                               mut m: mrb_method_t) {
    let mut h: *mut kh_mt_t = 0 as *mut kh_mt_t;
    let mut k: khiter_t = 0;
    if 0 != (*c).flags() as libc::c_int & 1i32 << 19i32 {
        c = (*c).super_0;
        while 0 == (*c).flags() as libc::c_int & 1i32 << 18i32 {
            c = (*c).super_0
        }
    }
    h = (*c).mt;
    mrb_check_frozen(mrb, c as *mut libc::c_void);
    if h.is_null() { (*c).mt = kh_init_mt(mrb); h = (*c).mt }
    k = kh_put_mt(mrb, h, mid, 0 as *mut libc::c_int);
    *(*h).vals.offset(k as isize) = m;
    if 0 == m.func_p && !m.unnamed.proc_0.is_null() {
        let mut p: *mut RProc = m.unnamed.proc_0;
        (*p).set_flags((*p).flags() | 2048i32 as uint32_t);
        (*p).c = 0 as *mut RClass;
        mrb_field_write_barrier(mrb, c as *mut RBasic, p as *mut RBasic);
        if !((*p).flags() as libc::c_int & 1024i32 != 0i32) {
            if (*p).flags() as libc::c_int & 1024i32 != 0i32 {
                (*(*p).e.env).c = c;
                mrb_field_write_barrier(mrb, (*p).e.env as *mut RBasic,
                                        c as *mut RBasic);
            } else {
                (*p).e.target_class = c;
                mrb_field_write_barrier(mrb, p as *mut RBasic,
                                        c as *mut RBasic);
            }
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn kh_put_mt(mut mrb: *mut mrb_state,
                                   mut h: *mut kh_mt_t, mut key: mrb_sym,
                                   mut ret: *mut libc::c_int) -> khint_t {
    let mut k: khint_t = 0;
    let mut del_k: khint_t = 0;
    let mut step: khint_t = 0i32 as khint_t;
    if (*h).n_occupied >= (*h).n_buckets >> 2i32 | (*h).n_buckets >> 1i32 {
        kh_resize_mt(mrb, h,
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
        let ref mut fresh2 =
            *(*h).ed_flags.offset(del_k.wrapping_div(4i32 as libc::c_uint) as
                                      isize);
        *fresh2 =
            (*fresh2 as libc::c_int &
                 !(__m_del[del_k.wrapping_rem(4i32 as libc::c_uint) as usize]
                       as libc::c_int)) as uint8_t;
        (*h).size = (*h).size.wrapping_add(1);
        if !ret.is_null() { *ret = 2i32 }
        return del_k
    } else {
        *(*h).keys.offset(k as isize) = key;
        let ref mut fresh3 =
            *(*h).ed_flags.offset(k.wrapping_div(4i32 as libc::c_uint) as
                                      isize);
        *fresh3 =
            (*fresh3 as libc::c_int &
                 !(__m_empty[k.wrapping_rem(4i32 as libc::c_uint) as usize] as
                       libc::c_int)) as uint8_t;
        (*h).size = (*h).size.wrapping_add(1);
        (*h).n_occupied = (*h).n_occupied.wrapping_add(1);
        if !ret.is_null() { *ret = 1i32 }
        return k
    };
}
/* extern uint8_t __m[]; */
/* mask for flags */
static mut __m_empty: [uint8_t; 4] =
    [0x2i32 as uint8_t, 0x8i32 as uint8_t, 0x20i32 as uint8_t,
     0x80i32 as uint8_t];
static mut __m_del: [uint8_t; 4] =
    [0x1i32 as uint8_t, 0x4i32 as uint8_t, 0x10i32 as uint8_t,
     0x40i32 as uint8_t];
#[no_mangle]
pub unsafe extern "C" fn kh_resize_mt(mut mrb: *mut mrb_state,
                                      mut h: *mut kh_mt_t,
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
    let mut hh: kh_mt_t =
        kh_mt{n_buckets: 0,
              size: 0,
              n_occupied: 0,
              ed_flags: 0 as *mut uint8_t,
              keys: 0 as *mut mrb_sym,
              vals: 0 as *mut mrb_method_t,};
    let mut old_ed_flags: *mut uint8_t = (*h).ed_flags;
    let mut old_keys: *mut mrb_sym = (*h).keys;
    let mut old_vals: *mut mrb_method_t = (*h).vals;
    let mut old_n_buckets: khint_t = (*h).n_buckets;
    let mut i: khint_t = 0;
    hh.n_buckets = new_n_buckets;
    kh_alloc_mt(mrb, &mut hh);
    i = 0i32 as khint_t;
    while i < old_n_buckets {
        if 0 ==
               *old_ed_flags.offset(i.wrapping_div(4i32 as libc::c_uint) as
                                        isize) as libc::c_int &
                   __m_either[i.wrapping_rem(4i32 as libc::c_uint) as usize]
                       as libc::c_int {
            let mut k: khint_t =
                kh_put_mt(mrb, &mut hh, *old_keys.offset(i as isize),
                          0 as *mut libc::c_int);
            *hh.vals.offset(k as isize) = *old_vals.offset(i as isize)
        }
        i = i.wrapping_add(1)
    }
    *h = hh;
    mrb_free(mrb, old_keys as *mut libc::c_void);
}
static mut __m_either: [uint8_t; 4] =
    [0x3i32 as uint8_t, 0xci32 as uint8_t, 0x30i32 as uint8_t,
     0xc0i32 as uint8_t];
#[inline]
unsafe extern "C" fn mrb_check_frozen(mut mrb: *mut mrb_state,
                                      mut o: *mut libc::c_void) {
    if 0 != (*(o as *mut RBasic)).flags() as libc::c_int & 1i32 << 20i32 {
        mrb_frozen_error(mrb, o);
    };
}
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
pub unsafe extern "C" fn mrb_define_class_method(mut mrb: *mut mrb_state,
                                                 mut c: *mut RClass,
                                                 mut name:
                                                     *const libc::c_char,
                                                 mut func: mrb_func_t,
                                                 mut aspec: mrb_aspec) {
    mrb_define_singleton_method(mrb, c as *mut RObject, name, func, aspec);
}
#[no_mangle]
pub unsafe extern "C" fn mrb_define_singleton_method(mut mrb: *mut mrb_state,
                                                     mut o: *mut RObject,
                                                     mut name:
                                                         *const libc::c_char,
                                                     mut func: mrb_func_t,
                                                     mut aspec: mrb_aspec) {
    prepare_singleton_class(mrb, o as *mut RBasic);
    mrb_define_method_id(mrb, (*o).c, mrb_intern_cstr(mrb, name), func,
                         aspec);
}
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
pub unsafe extern "C" fn mrb_define_module_function(mut mrb: *mut mrb_state,
                                                    mut c: *mut RClass,
                                                    mut name:
                                                        *const libc::c_char,
                                                    mut func: mrb_func_t,
                                                    mut aspec: mrb_aspec) {
    mrb_define_class_method(mrb, c, name, func, aspec);
    mrb_define_method(mrb, c, name, func, aspec);
}
/* *
 * Undefines a method.
 *
 * Example:
 *
 *     # Ruby style
 *
 *     class ExampleClassA
 *       def example_method
 *         "example"
 *       end
 *     end
 *     ExampleClassA.new.example_method # => example
 *
 *     class ExampleClassB < ExampleClassA
 *       undef_method :example_method
 *     end
 *
 *     ExampleClassB.new.example_method # => undefined method 'example_method' for ExampleClassB (NoMethodError)
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
 *       struct RClass *example_class_a;
 *       struct RClass *example_class_b;
 *       struct RClass *example_class_c;
 *
 *       example_class_a = mrb_define_class(mrb, "ExampleClassA", mrb->object_class);
 *       mrb_define_method(mrb, example_class_a, "example_method", mrb_example_method, MRB_ARGS_NONE());
 *       example_class_b = mrb_define_class(mrb, "ExampleClassB", example_class_a);
 *       example_class_c = mrb_define_class(mrb, "ExampleClassC", example_class_b);
 *       mrb_undef_method(mrb, example_class_c, "example_method");
 *     }
 *
 *     mrb_example_gem_final(mrb_state* mrb){
 *     }
 * @param [mrb_state*] mrb_state* The mruby state reference.
 * @param [struct RClass*] RClass* A class the method will be undefined from.
 * @param [const char*] const char* The name of the method to be undefined.
 */
#[no_mangle]
pub unsafe extern "C" fn mrb_undef_method(mut mrb: *mut mrb_state,
                                          mut c: *mut RClass,
                                          mut name: *const libc::c_char) {
    mrb_undef_method_id(mrb, c, mrb_intern_cstr(mrb, name));
}
#[no_mangle]
pub unsafe extern "C" fn mrb_undef_method_id(mut mrb: *mut mrb_state,
                                             mut c: *mut RClass,
                                             mut a: mrb_sym) {
    if 0 == mrb_obj_respond_to(mrb, c, a) {
        mrb_name_error(mrb, a,
                       b"undefined method \'%S\' for class \'%S\'\x00" as
                           *const u8 as *const libc::c_char,
                       mrb_sym2str(mrb, a),
                       mrb_obj_value(c as *mut libc::c_void));
    } else {
        let mut m: mrb_method_t =
            mrb_method_t{func_p: 0,
                         unnamed: unnamed{proc_0: 0 as *mut RProc,},};
        m.func_p = 0i32 as mrb_bool;
        m.unnamed.proc_0 = 0 as *mut RProc;
        mrb_define_method_raw(mrb, c, a, m);
    };
}
/* *
 * Returns true if obj responds to the given method. If the method was defined for that
 * class it returns true, it returns false otherwise.
 *
 *      Example:
 *      # Ruby style
 *      class ExampleClass
 *        def example_method
 *        end
 *      end
 *
 *      ExampleClass.new.respond_to?(:example_method) # => true
 *
 *      // C style
 *      void
 *      mrb_example_gem_init(mrb_state* mrb) {
 *        struct RClass *example_class;
 *        mrb_sym mid;
 *        mrb_bool obj_resp;
 *
 *        example_class = mrb_define_class(mrb, "ExampleClass", mrb->object_class);
 *        mrb_define_method(mrb, example_class, "example_method", exampleMethod, MRB_ARGS_NONE());
 *        mid = mrb_intern_str(mrb, mrb_str_new_lit(mrb, "example_method" ));
 *        obj_resp = mrb_obj_respond_to(mrb, example_class, mid); // => 1(true in Ruby world)
 *
 *        // If mrb_obj_respond_to returns 1 then puts "True"
 *        // If mrb_obj_respond_to returns 0 then puts "False"
 *        if (obj_resp == 1) {
 *          puts("True");
 *        }
 *        else if (obj_resp == 0) {
 *          puts("False");
 *        }
 *      }
 *
 * @param [mrb_state*] mrb The current mruby state.
 * @param [struct RClass *] c A reference to a class.
 * @param [mrb_sym] mid A symbol referencing a method id.
 * @return [mrb_bool] A boolean value.
 */
#[no_mangle]
pub unsafe extern "C" fn mrb_obj_respond_to(mut mrb: *mut mrb_state,
                                            mut c: *mut RClass,
                                            mut mid: mrb_sym) -> mrb_bool {
    let mut m: mrb_method_t =
        mrb_method_t{func_p: 0, unnamed: unnamed{proc_0: 0 as *mut RProc,},};
    m = mrb_method_search_vm(mrb, &mut c, mid);
    if m.unnamed.proc_0.is_null() { return 0i32 as mrb_bool }
    return 1i32 as mrb_bool;
}
#[no_mangle]
pub unsafe extern "C" fn mrb_method_search_vm(mut mrb: *mut mrb_state,
                                              mut cp: *mut *mut RClass,
                                              mut mid: mrb_sym)
 -> mrb_method_t {
    let mut k: khiter_t = 0;
    let mut m: mrb_method_t =
        mrb_method_t{func_p: 0, unnamed: unnamed{proc_0: 0 as *mut RProc,},};
    let mut c: *mut RClass = *cp;
    while !c.is_null() {
        let mut h: *mut kh_mt_t = (*c).mt;
        if !h.is_null() {
            k = kh_get_mt(mrb, h, mid);
            if k != (*h).n_buckets {
                m = *(*h).vals.offset(k as isize);
                if m.unnamed.proc_0.is_null() { break ; }
                *cp = c;
                return m
            }
        }
        c = (*c).super_0
    }
    m.func_p = 0i32 as mrb_bool;
    m.unnamed.proc_0 = 0 as *mut RProc;
    return m;
}
#[no_mangle]
pub unsafe extern "C" fn kh_get_mt(mut mrb: *mut mrb_state,
                                   mut h: *mut kh_mt_t, mut key: mrb_sym)
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
pub unsafe extern "C" fn mrb_undef_class_method(mut mrb: *mut mrb_state,
                                                mut c: *mut RClass,
                                                mut name:
                                                    *const libc::c_char) {
    mrb_undef_method(mrb,
                     mrb_singleton_class(mrb,
                                         mrb_obj_value(c as
                                                           *mut libc::c_void)).value.p
                         as *mut RClass, name);
}
/* *
 * Initialize a new object instance of c class.
 *
 * Example:
 *
 *     # Ruby style
 *     class ExampleClass
 *     end
 *
 *     p ExampleClass # => #<ExampleClass:0x9958588>
 *     // C style
 *     #include <stdio.h>
 *     #include <mruby.h>
 *
 *     void
 *     mrb_example_gem_init(mrb_state* mrb) {
 *       struct RClass *example_class;
 *       mrb_value obj;
 *       example_class = mrb_define_class(mrb, "ExampleClass", mrb->object_class); # => class ExampleClass; end
 *       obj = mrb_obj_new(mrb, example_class, 0, NULL); # => ExampleClass.new
 *       mrb_p(mrb, obj); // => Kernel#p
 *      }
 * @param [mrb_state*] mrb The current mruby state.
 * @param [RClass*] c Reference to the class of the new object.
 * @param [mrb_int] argc Number of arguments in argv
 * @param [const mrb_value *] argv Array of mrb_value to initialize the object
 * @return [mrb_value] The newly initialized object
 */
#[no_mangle]
pub unsafe extern "C" fn mrb_obj_new(mut mrb: *mut mrb_state,
                                     mut c: *mut RClass, mut argc: mrb_int,
                                     mut argv: *const mrb_value)
 -> mrb_value {
    let mut obj: mrb_value =
        mrb_value{value: unnamed_0{f: 0.,}, tt: MRB_TT_FALSE,};
    let mut mid: mrb_sym = 0;
    obj = mrb_instance_alloc(mrb, mrb_obj_value(c as *mut libc::c_void));
    mid =
        mrb_intern_static(mrb,
                          b"initialize\x00" as *const u8 as
                              *const libc::c_char,
                          (::std::mem::size_of::<[libc::c_char; 11]>() as
                               libc::c_ulong).wrapping_sub(1i32 as
                                                               libc::c_ulong));
    if 0 == mrb_func_basic_p(mrb, obj, mid, Some(mrb_bob_init)) {
        mrb_funcall_argv(mrb, obj, mid, argc, argv);
    }
    return obj;
}
unsafe extern "C" fn mrb_bob_init(mut mrb: *mut mrb_state, mut cv: mrb_value)
 -> mrb_value {
    return mrb_nil_value();
}
unsafe extern "C" fn mrb_instance_alloc(mut mrb: *mut mrb_state,
                                        mut cv: mrb_value) -> mrb_value {
    let mut c: *mut RClass = cv.value.p as *mut RClass;
    let mut o: *mut RObject = 0 as *mut RObject;
    let mut ttype: mrb_vtype =
        ((*c).flags() as libc::c_int & 0xffi32) as mrb_vtype;
    if (*c).tt() as libc::c_int == MRB_TT_SCLASS as libc::c_int {
        mrb_raise(mrb,
                  mrb_exc_get(mrb,
                              b"TypeError\x00" as *const u8 as
                                  *const libc::c_char),
                  b"can\'t create instance of singleton class\x00" as
                      *const u8 as *const libc::c_char);
    }
    if ttype as libc::c_uint == 0i32 as libc::c_uint { ttype = MRB_TT_OBJECT }
    if ttype as libc::c_uint <= MRB_TT_CPTR as libc::c_int as libc::c_uint {
        mrb_raisef(mrb,
                   mrb_exc_get(mrb,
                               b"TypeError\x00" as *const u8 as
                                   *const libc::c_char),
                   b"can\'t create instance of %S\x00" as *const u8 as
                       *const libc::c_char, cv);
    }
    o = mrb_obj_alloc(mrb, ttype, c) as *mut RObject;
    return mrb_obj_value(o as *mut libc::c_void);
}
/* *
 * Returns an mrb_bool. True if class was defined, and false if the class was not defined.
 *
 * Example:
 *     void
 *     mrb_example_gem_init(mrb_state* mrb) {
 *       struct RClass *example_class;
 *       mrb_bool cd;
 *
 *       example_class = mrb_define_class(mrb, "ExampleClass", mrb->object_class);
 *       cd = mrb_class_defined(mrb, "ExampleClass");
 *
 *       // If mrb_class_defined returns 1 then puts "True"
 *       // If mrb_class_defined returns 0 then puts "False"
 *       if (cd == 1){
 *         puts("True");
 *       }
 *       else {
 *         puts("False");
 *       }
 *      }
 *
 * @param [mrb_state*] mrb The current mruby state.
 * @param [const char *] name A string representing the name of the class.
 * @return [mrb_bool] A boolean value.
 */
#[no_mangle]
pub unsafe extern "C" fn mrb_class_defined(mut mrb: *mut mrb_state,
                                           mut name: *const libc::c_char)
 -> mrb_bool {
    let mut sym: mrb_value = mrb_check_intern_cstr(mrb, name);
    if sym.tt as libc::c_uint == MRB_TT_FALSE as libc::c_int as libc::c_uint
           && 0 == sym.value.i {
        return 0i32 as mrb_bool
    }
    return mrb_const_defined(mrb,
                             mrb_obj_value((*mrb).object_class as
                                               *mut libc::c_void),
                             sym.value.sym);
}
/* *
 * Gets a class.
 * @param [mrb_state*] mrb The current mruby state.
 * @param [const char *] name The name of the class.
 * @return [struct RClass *] A reference to the class.
*/
#[no_mangle]
pub unsafe extern "C" fn mrb_class_get(mut mrb: *mut mrb_state,
                                       mut name: *const libc::c_char)
 -> *mut RClass {
    return mrb_class_get_under(mrb, (*mrb).object_class, name);
}
/* *
 * Gets a child class.
 * @param [mrb_state*] mrb The current mruby state.
 * @param [struct RClass *] outer The name of the parent class.
 * @param [const char *] name The name of the class.
 * @return [struct RClass *] A reference to the class.
*/
#[no_mangle]
pub unsafe extern "C" fn mrb_class_get_under(mut mrb: *mut mrb_state,
                                             mut outer: *mut RClass,
                                             mut name: *const libc::c_char)
 -> *mut RClass {
    return class_from_sym(mrb, outer, mrb_intern_cstr(mrb, name));
}
/* *
 * Returns an mrb_bool. True if inner class was defined, and false if the inner class was not defined.
 *
 * Example:
 *     void
 *     mrb_example_gem_init(mrb_state* mrb) {
 *       struct RClass *example_outer, *example_inner;
 *       mrb_bool cd;
 *
 *       example_outer = mrb_define_module(mrb, "ExampleOuter");
 *
 *       example_inner = mrb_define_class_under(mrb, example_outer, "ExampleInner", mrb->object_class);
 *       cd = mrb_class_defined_under(mrb, example_outer, "ExampleInner");
 *
 *       // If mrb_class_defined_under returns 1 then puts "True"
 *       // If mrb_class_defined_under returns 0 then puts "False"
 *       if (cd == 1){
 *         puts("True");
 *       }
 *       else {
 *         puts("False");
 *       }
 *      }
 *
 * @param [mrb_state*] mrb The current mruby state.
 * @param [struct RClass *] outer The name of the outer class.
 * @param [const char *] name A string representing the name of the inner class.
 * @return [mrb_bool] A boolean value.
 */
#[no_mangle]
pub unsafe extern "C" fn mrb_class_defined_under(mut mrb: *mut mrb_state,
                                                 mut outer: *mut RClass,
                                                 mut name:
                                                     *const libc::c_char)
 -> mrb_bool {
    let mut sym: mrb_value = mrb_check_intern_cstr(mrb, name);
    if sym.tt as libc::c_uint == MRB_TT_FALSE as libc::c_int as libc::c_uint
           && 0 == sym.value.i {
        return 0i32 as mrb_bool
    }
    return mrb_const_defined_at(mrb,
                                mrb_obj_value(outer as *mut libc::c_void),
                                sym.value.sym);
}
/* *
 * Gets a module.
 * @param [mrb_state*] mrb The current mruby state.
 * @param [const char *] name The name of the module.
 * @return [struct RClass *] A reference to the module.
*/
#[no_mangle]
pub unsafe extern "C" fn mrb_module_get(mut mrb: *mut mrb_state,
                                        mut name: *const libc::c_char)
 -> *mut RClass {
    return mrb_module_get_under(mrb, (*mrb).object_class, name);
}
/* *
 * Gets a module defined under another module.
 * @param [mrb_state*] mrb The current mruby state.
 * @param [struct RClass *] outer The name of the outer module.
 * @param [const char *] name The name of the module.
 * @return [struct RClass *] A reference to the module.
*/
#[no_mangle]
pub unsafe extern "C" fn mrb_module_get_under(mut mrb: *mut mrb_state,
                                              mut outer: *mut RClass,
                                              mut name: *const libc::c_char)
 -> *mut RClass {
    return module_from_sym(mrb, outer, mrb_intern_cstr(mrb, name));
}
/* a function to raise NotImplementedError with current method name */
#[no_mangle]
pub unsafe extern "C" fn mrb_notimplement(mut mrb: *mut mrb_state) {
    let mut ci: *mut mrb_callinfo = (*(*mrb).c).ci;
    if 0 != (*ci).mid {
        let mut str: mrb_value = mrb_sym2str(mrb, (*ci).mid);
        mrb_raisef(mrb,
                   mrb_exc_get(mrb,
                               b"NotImplementedError\x00" as *const u8 as
                                   *const libc::c_char),
                   b"%S() function is unimplemented on this machine\x00" as
                       *const u8 as *const libc::c_char, str);
    };
}
/* a function to be replacement of unimplemented method */
#[no_mangle]
pub unsafe extern "C" fn mrb_notimplement_m(mut mrb: *mut mrb_state,
                                            mut self_0: mrb_value)
 -> mrb_value {
    mrb_notimplement(mrb);
    return mrb_nil_value();
}
/* *
 * Defines a new class under a given module
 *
 * @param [mrb_state*] mrb The current mruby state.
 * @param [struct RClass *] outer Reference to the module under which the new class will be defined
 * @param [const char *] name The name of the defined class
 * @param [struct RClass *] super The new class parent
 * @return [struct RClass *] Reference to the newly defined class
 * @see mrb_define_class
 */
#[no_mangle]
pub unsafe extern "C" fn mrb_define_class_under(mut mrb: *mut mrb_state,
                                                mut outer: *mut RClass,
                                                mut name: *const libc::c_char,
                                                mut super_0: *mut RClass)
 -> *mut RClass {
    let mut id: mrb_sym = mrb_intern_cstr(mrb, name);
    let mut c: *mut RClass = 0 as *mut RClass;
    c = define_class(mrb, id, super_0, outer);
    setup_class(mrb, outer, c, id);
    return c;
}
#[no_mangle]
pub unsafe extern "C" fn mrb_define_module_under(mut mrb: *mut mrb_state,
                                                 mut outer: *mut RClass,
                                                 mut name:
                                                     *const libc::c_char)
 -> *mut RClass {
    let mut id: mrb_sym = mrb_intern_cstr(mrb, name);
    let mut c: *mut RClass = define_module(mrb, id, outer);
    setup_class(mrb, outer, c, id);
    return c;
}
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
pub unsafe extern "C" fn mrb_get_args(mut mrb: *mut mrb_state,
                                      mut format: *const libc::c_char,
                                      mut ap: ...) -> mrb_int {
    let mut fmt: *const libc::c_char = format;
    let mut c: libc::c_char = 0;
    let mut i: mrb_int = 0i32 as mrb_int;
    let mut argc: mrb_int = mrb_get_argc(mrb);
    let mut arg_i: mrb_int = 0i32 as mrb_int;
    let mut array_argv: *mut mrb_value = mrb_get_argv(mrb);
    let mut opt: mrb_bool = 0i32 as mrb_bool;
    let mut opt_skip: mrb_bool = 1i32 as mrb_bool;
    let mut given: mrb_bool = 1i32 as mrb_bool;
    loop  {
        let fresh4 = fmt;
        fmt = fmt.offset(1);
        c = *fresh4;
        if !(0 != c) { break ; }
        match c as libc::c_int {
            124 => { opt = 1i32 as mrb_bool }
            42 => { opt_skip = 0i32 as mrb_bool; break ; }
            38 | 63 => { if 0 != opt { opt_skip = 0i32 as mrb_bool } }
            33 | _ => { }
        }
    }
    opt = 0i32 as mrb_bool;
    i = 0i32 as mrb_int;
    loop  {
        let fresh5 = format;
        format = format.offset(1);
        c = *fresh5;
        if !(0 != c) { break ; }
        match c as libc::c_int {
            124 | 42 | 38 | 63 => { }
            _ => {
                if argc <= i {
                    if 0 != opt {
                        given = 0i32 as mrb_bool
                    } else {
                        mrb_raise(mrb,
                                  mrb_exc_get(mrb,
                                              b"ArgumentError\x00" as
                                                  *const u8 as
                                                  *const libc::c_char),
                                  b"wrong number of arguments\x00" as
                                      *const u8 as *const libc::c_char);
                    }
                }
            }
        }
        let mut current_block_182: u64;
        match c as libc::c_int {
            111 => {
                let mut p: *mut mrb_value = 0 as *mut mrb_value;
                p = ap.arg::<*mut mrb_value>();
                if i < argc {
                    let fresh6 = arg_i;
                    arg_i = arg_i + 1;
                    *p =
                        *if !array_argv.is_null() {
                             array_argv
                         } else {
                             (*(*mrb).c).stack.offset(1isize)
                         }.offset(fresh6 as isize);
                    i += 1
                }
            }
            67 => {
                let mut p_0: *mut mrb_value = 0 as *mut mrb_value;
                p_0 = ap.arg::<*mut mrb_value>();
                if i < argc {
                    let mut ss: mrb_value =
                        mrb_value{value: unnamed_0{f: 0.,},
                                  tt: MRB_TT_FALSE,};
                    let fresh7 = arg_i;
                    arg_i = arg_i + 1;
                    ss =
                        *if !array_argv.is_null() {
                             array_argv
                         } else {
                             (*(*mrb).c).stack.offset(1isize)
                         }.offset(fresh7 as isize);
                    if 0 == class_ptr_p(ss) {
                        mrb_raisef(mrb,
                                   mrb_exc_get(mrb,
                                               b"TypeError\x00" as *const u8
                                                   as *const libc::c_char),
                                   b"%S is not class/module\x00" as *const u8
                                       as *const libc::c_char, ss);
                    }
                    *p_0 = ss;
                    i += 1
                }
            }
            83 => {
                let mut p_1: *mut mrb_value = 0 as *mut mrb_value;
                p_1 = ap.arg::<*mut mrb_value>();
                if *format as libc::c_int == '!' as i32 {
                    format = format.offset(1isize);
                    if i < argc &&
                           ((*if !array_argv.is_null() {
                                  array_argv
                              } else {
                                  (*(*mrb).c).stack.offset(1isize)
                              }.offset(arg_i as isize)).tt as libc::c_uint ==
                                MRB_TT_FALSE as libc::c_int as libc::c_uint &&
                                0 ==
                                    (*if !array_argv.is_null() {
                                          array_argv
                                      } else {
                                          (*(*mrb).c).stack.offset(1isize)
                                      }.offset(arg_i as isize)).value.i) {
                        let fresh8 = arg_i;
                        arg_i = arg_i + 1;
                        *p_1 =
                            *if !array_argv.is_null() {
                                 array_argv
                             } else {
                                 (*(*mrb).c).stack.offset(1isize)
                             }.offset(fresh8 as isize);
                        i += 1;
                        current_block_182 = 15115612870595956036;
                    } else { current_block_182 = 10753070352654377903; }
                } else { current_block_182 = 10753070352654377903; }
                match current_block_182 {
                    15115612870595956036 => { }
                    _ => {
                        if i < argc {
                            let fresh9 = arg_i;
                            arg_i = arg_i + 1;
                            *p_1 =
                                to_str(mrb,
                                       *if !array_argv.is_null() {
                                            array_argv
                                        } else {
                                            (*(*mrb).c).stack.offset(1isize)
                                        }.offset(fresh9 as isize));
                            i += 1
                        }
                    }
                }
            }
            65 => {
                let mut p_2: *mut mrb_value = 0 as *mut mrb_value;
                p_2 = ap.arg::<*mut mrb_value>();
                if *format as libc::c_int == '!' as i32 {
                    format = format.offset(1isize);
                    if i < argc &&
                           ((*if !array_argv.is_null() {
                                  array_argv
                              } else {
                                  (*(*mrb).c).stack.offset(1isize)
                              }.offset(arg_i as isize)).tt as libc::c_uint ==
                                MRB_TT_FALSE as libc::c_int as libc::c_uint &&
                                0 ==
                                    (*if !array_argv.is_null() {
                                          array_argv
                                      } else {
                                          (*(*mrb).c).stack.offset(1isize)
                                      }.offset(arg_i as isize)).value.i) {
                        let fresh10 = arg_i;
                        arg_i = arg_i + 1;
                        *p_2 =
                            *if !array_argv.is_null() {
                                 array_argv
                             } else {
                                 (*(*mrb).c).stack.offset(1isize)
                             }.offset(fresh10 as isize);
                        i += 1;
                        current_block_182 = 15115612870595956036;
                    } else { current_block_182 = 10067844863897285902; }
                } else { current_block_182 = 10067844863897285902; }
                match current_block_182 {
                    15115612870595956036 => { }
                    _ => {
                        if i < argc {
                            let fresh11 = arg_i;
                            arg_i = arg_i + 1;
                            *p_2 =
                                to_ary(mrb,
                                       *if !array_argv.is_null() {
                                            array_argv
                                        } else {
                                            (*(*mrb).c).stack.offset(1isize)
                                        }.offset(fresh11 as isize));
                            i += 1
                        }
                    }
                }
            }
            72 => {
                let mut p_3: *mut mrb_value = 0 as *mut mrb_value;
                p_3 = ap.arg::<*mut mrb_value>();
                if *format as libc::c_int == '!' as i32 {
                    format = format.offset(1isize);
                    if i < argc &&
                           ((*if !array_argv.is_null() {
                                  array_argv
                              } else {
                                  (*(*mrb).c).stack.offset(1isize)
                              }.offset(arg_i as isize)).tt as libc::c_uint ==
                                MRB_TT_FALSE as libc::c_int as libc::c_uint &&
                                0 ==
                                    (*if !array_argv.is_null() {
                                          array_argv
                                      } else {
                                          (*(*mrb).c).stack.offset(1isize)
                                      }.offset(arg_i as isize)).value.i) {
                        let fresh12 = arg_i;
                        arg_i = arg_i + 1;
                        *p_3 =
                            *if !array_argv.is_null() {
                                 array_argv
                             } else {
                                 (*(*mrb).c).stack.offset(1isize)
                             }.offset(fresh12 as isize);
                        i += 1;
                        current_block_182 = 15115612870595956036;
                    } else { current_block_182 = 993425571616822999; }
                } else { current_block_182 = 993425571616822999; }
                match current_block_182 {
                    15115612870595956036 => { }
                    _ => {
                        if i < argc {
                            let fresh13 = arg_i;
                            arg_i = arg_i + 1;
                            *p_3 =
                                to_hash(mrb,
                                        *if !array_argv.is_null() {
                                             array_argv
                                         } else {
                                             (*(*mrb).c).stack.offset(1isize)
                                         }.offset(fresh13 as isize));
                            i += 1
                        }
                    }
                }
            }
            115 => {
                let mut ss_0: mrb_value =
                    mrb_value{value: unnamed_0{f: 0.,}, tt: MRB_TT_FALSE,};
                let mut ps: *mut *mut libc::c_char =
                    0 as *mut *mut libc::c_char;
                let mut pl: *mut mrb_int = 0 as *mut mrb_int;
                ps = ap.arg::<*mut *mut libc::c_char>();
                pl = ap.arg::<*mut mrb_int>();
                if *format as libc::c_int == '!' as i32 {
                    format = format.offset(1isize);
                    if i < argc &&
                           ((*if !array_argv.is_null() {
                                  array_argv
                              } else {
                                  (*(*mrb).c).stack.offset(1isize)
                              }.offset(arg_i as isize)).tt as libc::c_uint ==
                                MRB_TT_FALSE as libc::c_int as libc::c_uint &&
                                0 ==
                                    (*if !array_argv.is_null() {
                                          array_argv
                                      } else {
                                          (*(*mrb).c).stack.offset(1isize)
                                      }.offset(arg_i as isize)).value.i) {
                        *ps = 0 as *mut libc::c_char;
                        *pl = 0i32 as mrb_int;
                        i += 1;
                        arg_i += 1;
                        current_block_182 = 15115612870595956036;
                    } else { current_block_182 = 17958840340921835115; }
                } else { current_block_182 = 17958840340921835115; }
                match current_block_182 {
                    15115612870595956036 => { }
                    _ => {
                        if i < argc {
                            let fresh14 = arg_i;
                            arg_i = arg_i + 1;
                            ss_0 =
                                to_str(mrb,
                                       *if !array_argv.is_null() {
                                            array_argv
                                        } else {
                                            (*(*mrb).c).stack.offset(1isize)
                                        }.offset(fresh14 as isize));
                            *ps =
                                if 0 !=
                                       (*(ss_0.value.p as
                                              *mut RString)).flags() as
                                           libc::c_int & 32i32 {
                                    (*(ss_0.value.p as
                                           *mut RString)).as_0.ary.as_mut_ptr()
                                } else {
                                    (*(ss_0.value.p as
                                           *mut RString)).as_0.heap.ptr
                                };
                            *pl =
                                if 0 !=
                                       (*(ss_0.value.p as
                                              *mut RString)).flags() as
                                           libc::c_int & 32i32 {
                                    (((*(ss_0.value.p as
                                             *mut RString)).flags() as
                                          libc::c_int & 0x7c0i32) >> 6i32) as
                                        mrb_int
                                } else {
                                    (*(ss_0.value.p as
                                           *mut RString)).as_0.heap.len
                                };
                            i += 1
                        }
                    }
                }
            }
            122 => {
                let mut ss_1: mrb_value =
                    mrb_value{value: unnamed_0{f: 0.,}, tt: MRB_TT_FALSE,};
                let mut ps_0: *mut *const libc::c_char =
                    0 as *mut *const libc::c_char;
                ps_0 = ap.arg::<*mut *const libc::c_char>();
                if *format as libc::c_int == '!' as i32 {
                    format = format.offset(1isize);
                    if i < argc &&
                           ((*if !array_argv.is_null() {
                                  array_argv
                              } else {
                                  (*(*mrb).c).stack.offset(1isize)
                              }.offset(arg_i as isize)).tt as libc::c_uint ==
                                MRB_TT_FALSE as libc::c_int as libc::c_uint &&
                                0 ==
                                    (*if !array_argv.is_null() {
                                          array_argv
                                      } else {
                                          (*(*mrb).c).stack.offset(1isize)
                                      }.offset(arg_i as isize)).value.i) {
                        *ps_0 = 0 as *const libc::c_char;
                        i += 1;
                        arg_i += 1;
                        current_block_182 = 15115612870595956036;
                    } else { current_block_182 = 10109057886293123569; }
                } else { current_block_182 = 10109057886293123569; }
                match current_block_182 {
                    15115612870595956036 => { }
                    _ => {
                        if i < argc {
                            let fresh15 = arg_i;
                            arg_i = arg_i + 1;
                            ss_1 =
                                to_str(mrb,
                                       *if !array_argv.is_null() {
                                            array_argv
                                        } else {
                                            (*(*mrb).c).stack.offset(1isize)
                                        }.offset(fresh15 as isize));
                            *ps_0 = mrb_string_value_cstr(mrb, &mut ss_1);
                            i += 1
                        }
                    }
                }
            }
            97 => {
                let mut aa: mrb_value =
                    mrb_value{value: unnamed_0{f: 0.,}, tt: MRB_TT_FALSE,};
                let mut a: *mut RArray = 0 as *mut RArray;
                let mut pb: *mut *mut mrb_value = 0 as *mut *mut mrb_value;
                let mut pl_0: *mut mrb_int = 0 as *mut mrb_int;
                pb = ap.arg::<*mut *mut mrb_value>();
                pl_0 = ap.arg::<*mut mrb_int>();
                if *format as libc::c_int == '!' as i32 {
                    format = format.offset(1isize);
                    if i < argc &&
                           ((*if !array_argv.is_null() {
                                  array_argv
                              } else {
                                  (*(*mrb).c).stack.offset(1isize)
                              }.offset(arg_i as isize)).tt as libc::c_uint ==
                                MRB_TT_FALSE as libc::c_int as libc::c_uint &&
                                0 ==
                                    (*if !array_argv.is_null() {
                                          array_argv
                                      } else {
                                          (*(*mrb).c).stack.offset(1isize)
                                      }.offset(arg_i as isize)).value.i) {
                        *pb = 0 as *mut mrb_value;
                        *pl_0 = 0i32 as mrb_int;
                        i += 1;
                        arg_i += 1;
                        current_block_182 = 15115612870595956036;
                    } else { current_block_182 = 10468276026569382870; }
                } else { current_block_182 = 10468276026569382870; }
                match current_block_182 {
                    15115612870595956036 => { }
                    _ => {
                        if i < argc {
                            let fresh16 = arg_i;
                            arg_i = arg_i + 1;
                            aa =
                                to_ary(mrb,
                                       *if !array_argv.is_null() {
                                            array_argv
                                        } else {
                                            (*(*mrb).c).stack.offset(1isize)
                                        }.offset(fresh16 as isize));
                            a = aa.value.p as *mut RArray;
                            *pb =
                                if 0 != (*a).flags() as libc::c_int & 7i32 {
                                    &mut (*a).as_0 as *mut unnamed_5 as
                                        *mut mrb_value
                                } else { (*a).as_0.heap.ptr };
                            *pl_0 =
                                if 0 != (*a).flags() as libc::c_int & 7i32 {
                                    (((*a).flags() as libc::c_int & 7i32) -
                                         1i32) as mrb_int
                                } else { (*a).as_0.heap.len };
                            i += 1
                        }
                    }
                }
            }
            73 => {
                let mut p_4: *mut *mut libc::c_void =
                    0 as *mut *mut libc::c_void;
                let mut ss_2: mrb_value =
                    mrb_value{value: unnamed_0{f: 0.,}, tt: MRB_TT_FALSE,};
                p_4 = ap.arg::<*mut *mut libc::c_void>();
                if i < argc {
                    ss_2 =
                        *if !array_argv.is_null() {
                             array_argv
                         } else {
                             (*(*mrb).c).stack.offset(1isize)
                         }.offset(arg_i as isize);
                    if ss_2.tt as libc::c_uint !=
                           MRB_TT_ISTRUCT as libc::c_int as libc::c_uint {
                        mrb_raisef(mrb,
                                   mrb_exc_get(mrb,
                                               b"TypeError\x00" as *const u8
                                                   as *const libc::c_char),
                                   b"%S is not inline struct\x00" as *const u8
                                       as *const libc::c_char, ss_2);
                    }
                    *p_4 = mrb_istruct_ptr(ss_2);
                    arg_i += 1;
                    i += 1
                }
            }
            102 => {
                let mut p_5: *mut mrb_float = 0 as *mut mrb_float;
                p_5 = ap.arg::<*mut mrb_float>();
                if i < argc {
                    *p_5 =
                        mrb_to_flo(mrb,
                                   *if !array_argv.is_null() {
                                        array_argv
                                    } else {
                                        (*(*mrb).c).stack.offset(1isize)
                                    }.offset(arg_i as isize));
                    arg_i += 1;
                    i += 1
                }
            }
            105 => {
                let mut p_6: *mut mrb_int = 0 as *mut mrb_int;
                p_6 = ap.arg::<*mut mrb_int>();
                if i < argc {
                    *p_6 =
                        mrb_to_int(mrb,
                                   *if !array_argv.is_null() {
                                        array_argv
                                    } else {
                                        (*(*mrb).c).stack.offset(1isize)
                                    }.offset(arg_i as isize)).value.i;
                    arg_i += 1;
                    i += 1
                }
            }
            98 => {
                let mut boolp: *mut mrb_bool = ap.arg::<*mut mrb_bool>();
                if i < argc {
                    let fresh17 = arg_i;
                    arg_i = arg_i + 1;
                    let mut b: mrb_value =
                        *if !array_argv.is_null() {
                             array_argv
                         } else {
                             (*(*mrb).c).stack.offset(1isize)
                         }.offset(fresh17 as isize);
                    *boolp =
                        (b.tt as libc::c_uint !=
                             MRB_TT_FALSE as libc::c_int as libc::c_uint) as
                            libc::c_int as mrb_bool;
                    i += 1
                }
            }
            110 => {
                let mut symp: *mut mrb_sym = 0 as *mut mrb_sym;
                symp = ap.arg::<*mut mrb_sym>();
                if i < argc {
                    let mut ss_3: mrb_value =
                        mrb_value{value: unnamed_0{f: 0.,},
                                  tt: MRB_TT_FALSE,};
                    let fresh18 = arg_i;
                    arg_i = arg_i + 1;
                    ss_3 =
                        *if !array_argv.is_null() {
                             array_argv
                         } else {
                             (*(*mrb).c).stack.offset(1isize)
                         }.offset(fresh18 as isize);
                    *symp = mrb_obj_to_sym(mrb, ss_3);
                    i += 1
                }
            }
            100 => {
                let mut datap: *mut *mut libc::c_void =
                    0 as *mut *mut libc::c_void;
                let mut type_0: *const mrb_data_type =
                    0 as *const mrb_data_type;
                datap = ap.arg::<*mut *mut libc::c_void>();
                type_0 = ap.arg::<*const mrb_data_type>();
                if *format as libc::c_int == '!' as i32 {
                    format = format.offset(1isize);
                    if i < argc &&
                           ((*if !array_argv.is_null() {
                                  array_argv
                              } else {
                                  (*(*mrb).c).stack.offset(1isize)
                              }.offset(arg_i as isize)).tt as libc::c_uint ==
                                MRB_TT_FALSE as libc::c_int as libc::c_uint &&
                                0 ==
                                    (*if !array_argv.is_null() {
                                          array_argv
                                      } else {
                                          (*(*mrb).c).stack.offset(1isize)
                                      }.offset(arg_i as isize)).value.i) {
                        *datap = 0 as *mut libc::c_void;
                        i += 1;
                        arg_i += 1;
                        current_block_182 = 15115612870595956036;
                    } else { current_block_182 = 12299212226970775842; }
                } else { current_block_182 = 12299212226970775842; }
                match current_block_182 {
                    15115612870595956036 => { }
                    _ => {
                        if i < argc {
                            let fresh19 = arg_i;
                            arg_i = arg_i + 1;
                            *datap =
                                mrb_data_get_ptr(mrb,
                                                 *if !array_argv.is_null() {
                                                      array_argv
                                                  } else {
                                                      (*(*mrb).c).stack.offset(1isize)
                                                  }.offset(fresh19 as isize),
                                                 type_0);
                            i += 1
                        }
                    }
                }
            }
            38 => {
                let mut p_7: *mut mrb_value = 0 as *mut mrb_value;
                let mut bp: *mut mrb_value = 0 as *mut mrb_value;
                p_7 = ap.arg::<*mut mrb_value>();
                if (*(*(*mrb).c).ci).argc < 0i32 {
                    bp = (*(*mrb).c).stack.offset(2isize)
                } else {
                    bp =
                        (*(*mrb).c).stack.offset((*(*(*mrb).c).ci).argc as
                                                     isize).offset(1isize)
                }
                if *format as libc::c_int == '!' as i32 {
                    format = format.offset(1isize);
                    if (*bp).tt as libc::c_uint ==
                           MRB_TT_FALSE as libc::c_int as libc::c_uint &&
                           0 == (*bp).value.i {
                        mrb_raise(mrb,
                                  mrb_exc_get(mrb,
                                              b"ArgumentError\x00" as
                                                  *const u8 as
                                                  *const libc::c_char),
                                  b"no block given\x00" as *const u8 as
                                      *const libc::c_char);
                    }
                }
                *p_7 = *bp
            }
            124 => {
                if 0 != opt_skip as libc::c_int && i == argc { return argc }
                opt = 1i32 as mrb_bool
            }
            63 => {
                let mut p_8: *mut mrb_bool = 0 as *mut mrb_bool;
                p_8 = ap.arg::<*mut mrb_bool>();
                *p_8 = given
            }
            42 => {
                let mut var: *mut *mut mrb_value = 0 as *mut *mut mrb_value;
                let mut pl_1: *mut mrb_int = 0 as *mut mrb_int;
                let mut nocopy: mrb_bool =
                    (if !array_argv.is_null() { 1i32 } else { 0i32 }) as
                        mrb_bool;
                if *format as libc::c_int == '!' as i32 {
                    format = format.offset(1isize);
                    nocopy = 1i32 as mrb_bool
                }
                var = ap.arg::<*mut *mut mrb_value>();
                pl_1 = ap.arg::<*mut mrb_int>();
                if argc > i {
                    *pl_1 = argc - i;
                    if *pl_1 > 0i32 as libc::c_longlong {
                        if 0 != nocopy {
                            *var =
                                if !array_argv.is_null() {
                                    array_argv
                                } else {
                                    (*(*mrb).c).stack.offset(1isize)
                                }.offset(arg_i as isize)
                        } else {
                            let mut args: mrb_value =
                                mrb_ary_new_from_values(mrb, *pl_1,
                                                        if !array_argv.is_null()
                                                           {
                                                            array_argv
                                                        } else {
                                                            (*(*mrb).c).stack.offset(1isize)
                                                        }.offset(arg_i as
                                                                     isize));
                            let ref mut fresh20 =
                                (*(args.value.p as *mut RArray)).c;
                            *fresh20 = 0 as *mut RClass;
                            *var =
                                if 0 !=
                                       (*(args.value.p as
                                              *mut RArray)).flags() as
                                           libc::c_int & 7i32 {
                                    &mut (*(args.value.p as *mut RArray)).as_0
                                        as *mut unnamed_5 as *mut mrb_value
                                } else {
                                    (*(args.value.p as
                                           *mut RArray)).as_0.heap.ptr
                                }
                        }
                    }
                    i = argc;
                    arg_i += *pl_1
                } else { *pl_1 = 0i32 as mrb_int; *var = 0 as *mut mrb_value }
            }
            _ => {
                mrb_raisef(mrb,
                           mrb_exc_get(mrb,
                                       b"ArgumentError\x00" as *const u8 as
                                           *const libc::c_char),
                           b"invalid argument specifier %S\x00" as *const u8
                               as *const libc::c_char,
                           mrb_str_new(mrb, &mut c, 1i32 as size_t));
            }
        }
    }
    if 0 == c && argc > i {
        mrb_raise(mrb,
                  mrb_exc_get(mrb,
                              b"ArgumentError\x00" as *const u8 as
                                  *const libc::c_char),
                  b"wrong number of arguments\x00" as *const u8 as
                      *const libc::c_char);
    }
    return i;
}
/* *
 * Retrieve number of arguments from mrb_state.
 *
 * Correctly handles *splat arguments.
 */
#[no_mangle]
pub unsafe extern "C" fn mrb_get_argc(mut mrb: *mut mrb_state) -> mrb_int {
    let mut argc: mrb_int = (*(*(*mrb).c).ci).argc as mrb_int;
    if argc < 0i32 as libc::c_longlong {
        let mut a: *mut RArray =
            (*(*(*mrb).c).stack.offset(1isize)).value.p as *mut RArray;
        argc =
            if 0 != (*a).flags() as libc::c_int & 7i32 {
                (((*a).flags() as libc::c_int & 7i32) - 1i32) as mrb_int
            } else { (*a).as_0.heap.len }
    }
    return argc;
}
#[no_mangle]
pub unsafe extern "C" fn mrb_get_argv(mut mrb: *mut mrb_state)
 -> *mut mrb_value {
    let mut argc: mrb_int = (*(*(*mrb).c).ci).argc as mrb_int;
    let mut array_argv: *mut mrb_value = 0 as *mut mrb_value;
    if argc < 0i32 as libc::c_longlong {
        let mut a: *mut RArray =
            (*(*(*mrb).c).stack.offset(1isize)).value.p as *mut RArray;
        array_argv =
            if 0 != (*a).flags() as libc::c_int & 7i32 {
                &mut (*a).as_0 as *mut unnamed_5 as *mut mrb_value
            } else { (*a).as_0.heap.ptr }
    } else { array_argv = 0 as *mut mrb_value }
    return array_argv;
}
#[inline]
unsafe extern "C" fn mrb_istruct_ptr(mut object: mrb_value)
 -> *mut libc::c_void {
    return (*(object.value.p as *mut RIStruct)).inline_data.as_mut_ptr() as
               *mut libc::c_void;
}
unsafe extern "C" fn to_ary(mut mrb: *mut mrb_state, mut val: mrb_value)
 -> mrb_value {
    if val.tt as libc::c_uint != MRB_TT_ARRAY as libc::c_int as libc::c_uint {
        mrb_raisef(mrb,
                   mrb_exc_get(mrb,
                               b"TypeError\x00" as *const u8 as
                                   *const libc::c_char),
                   b"expected %S\x00" as *const u8 as *const libc::c_char,
                   mrb_str_new_static(mrb,
                                      b"Array\x00" as *const u8 as
                                          *const libc::c_char,
                                      (::std::mem::size_of::<[libc::c_char; 6]>()
                                           as
                                           libc::c_ulong).wrapping_sub(1i32 as
                                                                           libc::c_ulong)));
    }
    return val;
}
unsafe extern "C" fn to_str(mut mrb: *mut mrb_state, mut val: mrb_value)
 -> mrb_value {
    if val.tt as libc::c_uint != MRB_TT_STRING as libc::c_int as libc::c_uint
       {
        mrb_raisef(mrb,
                   mrb_exc_get(mrb,
                               b"TypeError\x00" as *const u8 as
                                   *const libc::c_char),
                   b"expected %S\x00" as *const u8 as *const libc::c_char,
                   mrb_str_new_static(mrb,
                                      b"String\x00" as *const u8 as
                                          *const libc::c_char,
                                      (::std::mem::size_of::<[libc::c_char; 7]>()
                                           as
                                           libc::c_ulong).wrapping_sub(1i32 as
                                                                           libc::c_ulong)));
    }
    return val;
}
unsafe extern "C" fn to_hash(mut mrb: *mut mrb_state, mut val: mrb_value)
 -> mrb_value {
    if val.tt as libc::c_uint != MRB_TT_HASH as libc::c_int as libc::c_uint {
        mrb_raisef(mrb,
                   mrb_exc_get(mrb,
                               b"TypeError\x00" as *const u8 as
                                   *const libc::c_char),
                   b"expected %S\x00" as *const u8 as *const libc::c_char,
                   mrb_str_new_static(mrb,
                                      b"Hash\x00" as *const u8 as
                                          *const libc::c_char,
                                      (::std::mem::size_of::<[libc::c_char; 5]>()
                                           as
                                           libc::c_ulong).wrapping_sub(1i32 as
                                                                           libc::c_ulong)));
    }
    return val;
}
unsafe extern "C" fn class_ptr_p(mut obj: mrb_value) -> mrb_bool {
    match obj.tt as libc::c_uint {
        9 | 12 | 10 => { return 1i32 as mrb_bool }
        _ => { return 0i32 as mrb_bool }
    };
}
#[no_mangle]
pub unsafe extern "C" fn mrb_obj_classname(mut mrb: *mut mrb_state,
                                           mut obj: mrb_value)
 -> *const libc::c_char {
    return mrb_class_name(mrb, mrb_obj_class(mrb, obj));
}
#[no_mangle]
pub unsafe extern "C" fn mrb_obj_class(mut mrb: *mut mrb_state,
                                       mut obj: mrb_value) -> *mut RClass {
    return mrb_class_real(mrb_class(mrb, obj));
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
pub unsafe extern "C" fn mrb_class_name(mut mrb: *mut mrb_state,
                                        mut c: *mut RClass)
 -> *const libc::c_char {
    let mut name: mrb_value = class_name_str(mrb, c);
    return if 0 !=
                  (*(name.value.p as *mut RString)).flags() as libc::c_int &
                      32i32 {
               (*(name.value.p as *mut RString)).as_0.ary.as_mut_ptr()
           } else { (*(name.value.p as *mut RString)).as_0.heap.ptr };
}
unsafe extern "C" fn class_name_str(mut mrb: *mut mrb_state,
                                    mut c: *mut RClass) -> mrb_value {
    let mut path: mrb_value = mrb_class_path(mrb, c);
    if path.tt as libc::c_uint == MRB_TT_FALSE as libc::c_int as libc::c_uint
           && 0 == path.value.i {
        path =
            if (*c).tt() as libc::c_int == MRB_TT_MODULE as libc::c_int {
                mrb_str_new_static(mrb,
                                   b"#<Module:\x00" as *const u8 as
                                       *const libc::c_char,
                                   (::std::mem::size_of::<[libc::c_char; 10]>()
                                        as
                                        libc::c_ulong).wrapping_sub(1i32 as
                                                                        libc::c_ulong))
            } else {
                mrb_str_new_static(mrb,
                                   b"#<Class:\x00" as *const u8 as
                                       *const libc::c_char,
                                   (::std::mem::size_of::<[libc::c_char; 9]>()
                                        as
                                        libc::c_ulong).wrapping_sub(1i32 as
                                                                        libc::c_ulong))
            };
        mrb_str_concat(mrb, path,
                       mrb_ptr_to_str(mrb, c as *mut libc::c_void));
        mrb_str_cat(mrb, path, b">\x00" as *const u8 as *const libc::c_char,
                    (::std::mem::size_of::<[libc::c_char; 2]>() as
                         libc::c_ulong).wrapping_sub(1i32 as libc::c_ulong));
    }
    return path;
}
#[no_mangle]
pub unsafe extern "C" fn mrb_define_alias(mut mrb: *mut mrb_state,
                                          mut klass: *mut RClass,
                                          mut name1: *const libc::c_char,
                                          mut name2: *const libc::c_char) {
    mrb_alias_method(mrb, klass, mrb_intern_cstr(mrb, name1),
                     mrb_intern_cstr(mrb, name2));
}
#[no_mangle]
pub unsafe extern "C" fn mrb_alias_method(mut mrb: *mut mrb_state,
                                          mut c: *mut RClass, mut a: mrb_sym,
                                          mut b: mrb_sym) {
    let mut m: mrb_method_t = mrb_method_search(mrb, c, b);
    mrb_define_method_raw(mrb, c, a, m);
}
#[no_mangle]
pub unsafe extern "C" fn mrb_method_search(mut mrb: *mut mrb_state,
                                           mut c: *mut RClass,
                                           mut mid: mrb_sym) -> mrb_method_t {
    let mut m: mrb_method_t =
        mrb_method_t{func_p: 0, unnamed: unnamed{proc_0: 0 as *mut RProc,},};
    m = mrb_method_search_vm(mrb, &mut c, mid);
    if m.unnamed.proc_0.is_null() {
        let mut inspect: mrb_value =
            mrb_funcall(mrb, mrb_obj_value(c as *mut libc::c_void),
                        b"inspect\x00" as *const u8 as *const libc::c_char,
                        0i32 as mrb_int);
        if inspect.tt as libc::c_uint ==
               MRB_TT_STRING as libc::c_int as libc::c_uint &&
               if 0 !=
                      (*(inspect.value.p as *mut RString)).flags() as
                          libc::c_int & 32i32 {
                   (((*(inspect.value.p as *mut RString)).flags() as
                         libc::c_int & 0x7c0i32) >> 6i32) as mrb_int
               } else { (*(inspect.value.p as *mut RString)).as_0.heap.len } >
                   64i32 as libc::c_longlong {
            inspect = mrb_any_to_s(mrb, mrb_obj_value(c as *mut libc::c_void))
        }
        mrb_name_error(mrb, mid,
                       b"undefined method \'%S\' for class %S\x00" as
                           *const u8 as *const libc::c_char,
                       mrb_sym2str(mrb, mid), inspect);
    }
    return m;
}
#[no_mangle]
pub unsafe extern "C" fn mrb_respond_to(mut mrb: *mut mrb_state,
                                        mut obj: mrb_value, mut mid: mrb_sym)
 -> mrb_bool {
    return mrb_obj_respond_to(mrb, mrb_class(mrb, obj), mid);
}
#[no_mangle]
pub unsafe extern "C" fn mrb_define_module_id(mut mrb: *mut mrb_state,
                                              mut name: mrb_sym)
 -> *mut RClass {
    return define_module(mrb, name, (*mrb).object_class);
}
#[no_mangle]
pub unsafe extern "C" fn mrb_vm_define_class(mut mrb: *mut mrb_state,
                                             mut outer: mrb_value,
                                             mut super_0: mrb_value,
                                             mut id: mrb_sym) -> *mut RClass {
    let mut s: *mut RClass = 0 as *mut RClass;
    let mut c: *mut RClass = 0 as *mut RClass;
    if !(super_0.tt as libc::c_uint ==
             MRB_TT_FALSE as libc::c_int as libc::c_uint &&
             0 == super_0.value.i) {
        if super_0.tt as libc::c_uint !=
               MRB_TT_CLASS as libc::c_int as libc::c_uint {
            mrb_raisef(mrb,
                       mrb_exc_get(mrb,
                                   b"TypeError\x00" as *const u8 as
                                       *const libc::c_char),
                       b"superclass must be a Class (%S given)\x00" as
                           *const u8 as *const libc::c_char,
                       mrb_inspect(mrb, super_0));
        }
        s = super_0.value.p as *mut RClass
    } else { s = 0 as *mut RClass }
    check_if_class_or_module(mrb, outer);
    if 0 != mrb_const_defined_at(mrb, outer, id) {
        let mut old: mrb_value = mrb_const_get(mrb, outer, id);
        if old.tt as libc::c_uint !=
               MRB_TT_CLASS as libc::c_int as libc::c_uint {
            mrb_raisef(mrb,
                       mrb_exc_get(mrb,
                                   b"TypeError\x00" as *const u8 as
                                       *const libc::c_char),
                       b"%S is not a class\x00" as *const u8 as
                           *const libc::c_char, mrb_inspect(mrb, old));
        }
        c = old.value.p as *mut RClass;
        if !s.is_null() {
            if mrb_class_real((*c).super_0) != s {
                mrb_raisef(mrb,
                           mrb_exc_get(mrb,
                                       b"TypeError\x00" as *const u8 as
                                           *const libc::c_char),
                           b"superclass mismatch for class %S\x00" as
                               *const u8 as *const libc::c_char, old);
            }
        }
        return c
    }
    c = define_class(mrb, id, s, outer.value.p as *mut RClass);
    mrb_class_inherited(mrb, mrb_class_real((*c).super_0), c);
    return c;
}
unsafe extern "C" fn mrb_class_inherited(mut mrb: *mut mrb_state,
                                         mut super_0: *mut RClass,
                                         mut klass: *mut RClass) {
    let mut s: mrb_value =
        mrb_value{value: unnamed_0{f: 0.,}, tt: MRB_TT_FALSE,};
    let mut mid: mrb_sym = 0;
    if super_0.is_null() { super_0 = (*mrb).object_class }
    (*super_0).set_flags((*super_0).flags() | (1i32 << 17i32) as uint32_t);
    s = mrb_obj_value(super_0 as *mut libc::c_void);
    mid =
        mrb_intern_static(mrb,
                          b"inherited\x00" as *const u8 as
                              *const libc::c_char,
                          (::std::mem::size_of::<[libc::c_char; 10]>() as
                               libc::c_ulong).wrapping_sub(1i32 as
                                                               libc::c_ulong));
    if 0 == mrb_func_basic_p(mrb, s, mid, Some(mrb_bob_init)) {
        let mut c: mrb_value = mrb_obj_value(klass as *mut libc::c_void);
        mrb_funcall_argv(mrb, s, mid, 1i32 as mrb_int, &mut c);
    };
}
unsafe extern "C" fn check_if_class_or_module(mut mrb: *mut mrb_state,
                                              mut obj: mrb_value) {
    if 0 == class_ptr_p(obj) {
        mrb_raisef(mrb,
                   mrb_exc_get(mrb,
                               b"TypeError\x00" as *const u8 as
                                   *const libc::c_char),
                   b"%S is not a class/module\x00" as *const u8 as
                       *const libc::c_char, mrb_inspect(mrb, obj));
    };
}
#[no_mangle]
pub unsafe extern "C" fn mrb_vm_define_module(mut mrb: *mut mrb_state,
                                              mut outer: mrb_value,
                                              mut id: mrb_sym)
 -> *mut RClass {
    check_if_class_or_module(mrb, outer);
    if 0 != mrb_const_defined_at(mrb, outer, id) {
        let mut old: mrb_value = mrb_const_get(mrb, outer, id);
        if old.tt as libc::c_uint !=
               MRB_TT_MODULE as libc::c_int as libc::c_uint {
            mrb_raisef(mrb,
                       mrb_exc_get(mrb,
                                   b"TypeError\x00" as *const u8 as
                                       *const libc::c_char),
                       b"%S is not a module\x00" as *const u8 as
                           *const libc::c_char, mrb_inspect(mrb, old));
        }
        return old.value.p as *mut RClass
    }
    return define_module(mrb, id, outer.value.p as *mut RClass);
}
#[no_mangle]
pub unsafe extern "C" fn mrb_instance_new(mut mrb: *mut mrb_state,
                                          mut cv: mrb_value) -> mrb_value {
    let mut obj: mrb_value =
        mrb_value{value: unnamed_0{f: 0.,}, tt: MRB_TT_FALSE,};
    let mut blk: mrb_value =
        mrb_value{value: unnamed_0{f: 0.,}, tt: MRB_TT_FALSE,};
    let mut argv: *mut mrb_value = 0 as *mut mrb_value;
    let mut argc: mrb_int = 0;
    let mut init: mrb_sym = 0;
    mrb_get_args(mrb, b"*&\x00" as *const u8 as *const libc::c_char,
                 &mut argv as *mut *mut mrb_value, &mut argc as *mut mrb_int,
                 &mut blk as *mut mrb_value);
    obj = mrb_instance_alloc(mrb, cv);
    init =
        mrb_intern_static(mrb,
                          b"initialize\x00" as *const u8 as
                              *const libc::c_char,
                          (::std::mem::size_of::<[libc::c_char; 11]>() as
                               libc::c_ulong).wrapping_sub(1i32 as
                                                               libc::c_ulong));
    if 0 == mrb_func_basic_p(mrb, obj, init, Some(mrb_bob_init)) {
        mrb_funcall_with_block(mrb, obj, init, argc, argv, blk);
    }
    return obj;
}
#[no_mangle]
pub unsafe extern "C" fn mrb_const_name_p(mut mrb: *mut mrb_state,
                                          mut name: *const libc::c_char,
                                          mut len: mrb_int) -> mrb_bool {
    return (len > 0i32 as libc::c_longlong &&
                (*name.offset(0isize) as
                     libc::c_uint).wrapping_sub('A' as i32 as libc::c_uint) <
                    26i32 as libc::c_uint &&
                0 !=
                    mrb_ident_p(name.offset(1isize),
                                len - 1i32 as libc::c_longlong) as
                        libc::c_int) as libc::c_int as mrb_bool;
}
#[no_mangle]
pub unsafe extern "C" fn mrb_gc_mark_mt(mut mrb: *mut mrb_state,
                                        mut c: *mut RClass) {
    let mut k: khiter_t = 0;
    let mut h: *mut kh_mt_t = (*c).mt;
    if h.is_null() { return }
    k = 0i32 as khint_t;
    while k != (*h).n_buckets {
        if 0 ==
               *(*h).ed_flags.offset(k.wrapping_div(4i32 as libc::c_uint) as
                                         isize) as libc::c_int &
                   __m_either[k.wrapping_rem(4i32 as libc::c_uint) as usize]
                       as libc::c_int {
            let mut m: mrb_method_t = *(*h).vals.offset(k as isize);
            if 0 == m.func_p {
                let mut p: *mut RProc = m.unnamed.proc_0;
                mrb_gc_mark(mrb, p as *mut RBasic);
            }
        }
        k = k.wrapping_add(1)
    };
}
#[no_mangle]
pub unsafe extern "C" fn mrb_gc_mark_mt_size(mut mrb: *mut mrb_state,
                                             mut c: *mut RClass) -> size_t {
    let mut h: *mut kh_mt_t = (*c).mt;
    if h.is_null() { return 0i32 as size_t }
    return (*h).size as size_t;
}
#[no_mangle]
pub unsafe extern "C" fn mrb_gc_free_mt(mut mrb: *mut mrb_state,
                                        mut c: *mut RClass) {
    kh_destroy_mt(mrb, (*c).mt);
}
#[no_mangle]
pub unsafe extern "C" fn kh_destroy_mt(mut mrb: *mut mrb_state,
                                       mut h: *mut kh_mt_t) {
    if !h.is_null() {
        mrb_free(mrb, (*h).keys as *mut libc::c_void);
        mrb_free(mrb, h as *mut libc::c_void);
    };
}
#[no_mangle]
pub unsafe extern "C" fn kh_clear_mt(mut mrb: *mut mrb_state,
                                     mut h: *mut kh_mt_t) {
    if !h.is_null() && !(*h).ed_flags.is_null() {
        kh_fill_flags((*h).ed_flags, 0xaai32 as uint8_t,
                      (*h).n_buckets.wrapping_div(4i32 as libc::c_uint) as
                          size_t);
        (*h).n_occupied = 0i32 as khint_t;
        (*h).size = (*h).n_occupied
    };
}
#[no_mangle]
pub unsafe extern "C" fn kh_del_mt(mut mrb: *mut mrb_state,
                                   mut h: *mut kh_mt_t, mut x: khint_t) {
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
                                               &[libc::c_char; 10]>(b"kh_del_mt\x00")).as_ptr(),
                     b"src/class.c\x00" as *const u8 as *const libc::c_char,
                     19i32,
                     b"x != h->n_buckets && !(h->ed_flags[(x)/4]&__m_either[(x)%4])\x00"
                         as *const u8 as *const libc::c_char);
    } else { };
    let ref mut fresh21 =
        *(*h).ed_flags.offset(x.wrapping_div(4i32 as libc::c_uint) as isize);
    *fresh21 =
        (*fresh21 as libc::c_int |
             __m_del[x.wrapping_rem(4i32 as libc::c_uint) as usize] as
                 libc::c_int) as uint8_t;
    (*h).size = (*h).size.wrapping_sub(1);
}
#[no_mangle]
pub unsafe extern "C" fn kh_copy_mt(mut mrb: *mut mrb_state,
                                    mut h: *mut kh_mt_t) -> *mut kh_mt_t {
    let mut h2: *mut kh_mt_t = 0 as *mut kh_mt_t;
    let mut k: khiter_t = 0;
    let mut k2: khiter_t = 0;
    h2 = kh_init_mt(mrb);
    k = 0i32 as khint_t;
    while k != (*h).n_buckets {
        if 0 ==
               *(*h).ed_flags.offset(k.wrapping_div(4i32 as libc::c_uint) as
                                         isize) as libc::c_int &
                   __m_either[k.wrapping_rem(4i32 as libc::c_uint) as usize]
                       as libc::c_int {
            k2 =
                kh_put_mt(mrb, h2, *(*h).keys.offset(k as isize),
                          0 as *mut libc::c_int);
            *(*h2).vals.offset(k2 as isize) = *(*h).vals.offset(k as isize)
        }
        k = k.wrapping_add(1)
    }
    return h2;
}
unsafe extern "C" fn mrb_mod_prepend_features(mut mrb: *mut mrb_state,
                                              mut mod_0: mrb_value)
 -> mrb_value {
    let mut klass: mrb_value =
        mrb_value{value: unnamed_0{f: 0.,}, tt: MRB_TT_FALSE,};
    mrb_check_type(mrb, mod_0, MRB_TT_MODULE);
    mrb_get_args(mrb, b"C\x00" as *const u8 as *const libc::c_char,
                 &mut klass as *mut mrb_value);
    mrb_prepend_module(mrb, klass.value.p as *mut RClass,
                       mod_0.value.p as *mut RClass);
    return mod_0;
}
unsafe extern "C" fn mrb_mod_append_features(mut mrb: *mut mrb_state,
                                             mut mod_0: mrb_value)
 -> mrb_value {
    let mut klass: mrb_value =
        mrb_value{value: unnamed_0{f: 0.,}, tt: MRB_TT_FALSE,};
    mrb_check_type(mrb, mod_0, MRB_TT_MODULE);
    mrb_get_args(mrb, b"C\x00" as *const u8 as *const libc::c_char,
                 &mut klass as *mut mrb_value);
    mrb_include_module(mrb, klass.value.p as *mut RClass,
                       mod_0.value.p as *mut RClass);
    return mod_0;
}
/* 15.2.2.4.28 */
/*
 *  call-seq:
 *     mod.include?(module)    -> true or false
 *
 *  Returns <code>true</code> if <i>module</i> is included in
 *  <i>mod</i> or one of <i>mod</i>'s ancestors.
 *
 *     module A
 *     end
 *     class B
 *       include A
 *     end
 *     class C < B
 *     end
 *     B.include?(A)   #=> true
 *     C.include?(A)   #=> true
 *     A.include?(A)   #=> false
 */
unsafe extern "C" fn mrb_mod_include_p(mut mrb: *mut mrb_state,
                                       mut mod_0: mrb_value) -> mrb_value {
    let mut mod2: mrb_value =
        mrb_value{value: unnamed_0{f: 0.,}, tt: MRB_TT_FALSE,};
    let mut c: *mut RClass = mod_0.value.p as *mut RClass;
    mrb_get_args(mrb, b"C\x00" as *const u8 as *const libc::c_char,
                 &mut mod2 as *mut mrb_value);
    mrb_check_type(mrb, mod2, MRB_TT_MODULE);
    while !c.is_null() {
        if (*c).tt() as libc::c_int == MRB_TT_ICLASS as libc::c_int {
            if (*c).c == mod2.value.p as *mut RClass {
                return mrb_true_value()
            }
        }
        c = (*c).super_0
    }
    return mrb_false_value();
}
unsafe extern "C" fn mrb_mod_ancestors(mut mrb: *mut mrb_state,
                                       mut self_0: mrb_value) -> mrb_value {
    let mut result: mrb_value =
        mrb_value{value: unnamed_0{f: 0.,}, tt: MRB_TT_FALSE,};
    let mut c: *mut RClass = self_0.value.p as *mut RClass;
    result = mrb_ary_new(mrb);
    while !c.is_null() {
        if (*c).tt() as libc::c_int == MRB_TT_ICLASS as libc::c_int {
            mrb_ary_push(mrb, result,
                         mrb_obj_value((*c).c as *mut libc::c_void));
        } else if 0 == (*c).flags() as libc::c_int & 1i32 << 19i32 {
            mrb_ary_push(mrb, result, mrb_obj_value(c as *mut libc::c_void));
        }
        c = (*c).super_0
    }
    return result;
}
unsafe extern "C" fn mrb_mod_extend_object(mut mrb: *mut mrb_state,
                                           mut mod_0: mrb_value)
 -> mrb_value {
    let mut obj: mrb_value =
        mrb_value{value: unnamed_0{f: 0.,}, tt: MRB_TT_FALSE,};
    mrb_check_type(mrb, mod_0, MRB_TT_MODULE);
    mrb_get_args(mrb, b"o\x00" as *const u8 as *const libc::c_char,
                 &mut obj as *mut mrb_value);
    mrb_include_module(mrb,
                       mrb_singleton_class(mrb, obj).value.p as *mut RClass,
                       mod_0.value.p as *mut RClass);
    return mod_0;
}
unsafe extern "C" fn mrb_mod_initialize(mut mrb: *mut mrb_state,
                                        mut mod_0: mrb_value) -> mrb_value {
    let mut b: mrb_value =
        mrb_value{value: unnamed_0{f: 0.,}, tt: MRB_TT_FALSE,};
    let mut m: *mut RClass = mod_0.value.p as *mut RClass;
    boot_initmod(mrb, m);
    mrb_get_args(mrb, b"|&\x00" as *const u8 as *const libc::c_char,
                 &mut b as *mut mrb_value);
    if !(b.tt as libc::c_uint == MRB_TT_FALSE as libc::c_int as libc::c_uint
             && 0 == b.value.i) {
        mrb_yield_with_class(mrb, b, 1i32 as mrb_int, &mut mod_0, mod_0, m);
    }
    return mod_0;
}
unsafe extern "C" fn mrb_mod_dummy_visibility(mut mrb: *mut mrb_state,
                                              mut mod_0: mrb_value)
 -> mrb_value {
    return mod_0;
}
unsafe extern "C" fn attr_reader(mut mrb: *mut mrb_state, mut obj: mrb_value)
 -> mrb_value {
    let mut name: mrb_value = mrb_proc_cfunc_env_get(mrb, 0i32 as mrb_int);
    return mrb_iv_get(mrb, obj, mrb_obj_to_sym(mrb, name));
}
unsafe extern "C" fn mrb_mod_attr_reader(mut mrb: *mut mrb_state,
                                         mut mod_0: mrb_value) -> mrb_value {
    let mut c: *mut RClass = mod_0.value.p as *mut RClass;
    let mut argv: *mut mrb_value = 0 as *mut mrb_value;
    let mut argc: mrb_int = 0;
    let mut i: mrb_int = 0;
    let mut ai: libc::c_int = 0;
    mrb_get_args(mrb, b"*\x00" as *const u8 as *const libc::c_char,
                 &mut argv as *mut *mut mrb_value, &mut argc as *mut mrb_int);
    ai = mrb_gc_arena_save(mrb);
    i = 0i32 as mrb_int;
    while i < argc {
        let mut name: mrb_value =
            mrb_value{value: unnamed_0{f: 0.,}, tt: MRB_TT_FALSE,};
        let mut str: mrb_value =
            mrb_value{value: unnamed_0{f: 0.,}, tt: MRB_TT_FALSE,};
        let mut method: mrb_sym = 0;
        let mut sym: mrb_sym = 0;
        let mut p: *mut RProc = 0 as *mut RProc;
        let mut m: mrb_method_t =
            mrb_method_t{func_p: 0,
                         unnamed: unnamed{proc_0: 0 as *mut RProc,},};
        method = mrb_obj_to_sym(mrb, *argv.offset(i as isize));
        name = mrb_sym2str(mrb, method);
        str =
            mrb_str_new_capa(mrb,
                             (if 0 !=
                                     (*(name.value.p as *mut RString)).flags()
                                         as libc::c_int & 32i32 {
                                  (((*(name.value.p as *mut RString)).flags()
                                        as libc::c_int & 0x7c0i32) >> 6i32) as
                                      mrb_int
                              } else {
                                  (*(name.value.p as
                                         *mut RString)).as_0.heap.len
                              } + 1i32 as libc::c_longlong) as size_t);
        mrb_str_cat(mrb, str, b"@\x00" as *const u8 as *const libc::c_char,
                    (::std::mem::size_of::<[libc::c_char; 2]>() as
                         libc::c_ulong).wrapping_sub(1i32 as libc::c_ulong));
        mrb_str_cat_str(mrb, str, name);
        sym = mrb_intern_str(mrb, str);
        mrb_iv_name_sym_check(mrb, sym);
        name = mrb_symbol_value(sym);
        p =
            mrb_proc_new_cfunc_with_env(mrb, Some(attr_reader),
                                        1i32 as mrb_int, &mut name);
        m.func_p = 0i32 as mrb_bool;
        m.unnamed.proc_0 = p;
        mrb_define_method_raw(mrb, c, method, m);
        mrb_gc_arena_restore(mrb, ai);
        i += 1
    }
    return mrb_nil_value();
}
unsafe extern "C" fn attr_writer(mut mrb: *mut mrb_state, mut obj: mrb_value)
 -> mrb_value {
    let mut name: mrb_value = mrb_proc_cfunc_env_get(mrb, 0i32 as mrb_int);
    let mut val: mrb_value =
        mrb_value{value: unnamed_0{f: 0.,}, tt: MRB_TT_FALSE,};
    mrb_get_args(mrb, b"o\x00" as *const u8 as *const libc::c_char,
                 &mut val as *mut mrb_value);
    mrb_iv_set(mrb, obj, mrb_obj_to_sym(mrb, name), val);
    return val;
}
unsafe extern "C" fn mrb_mod_attr_writer(mut mrb: *mut mrb_state,
                                         mut mod_0: mrb_value) -> mrb_value {
    let mut c: *mut RClass = mod_0.value.p as *mut RClass;
    let mut argv: *mut mrb_value = 0 as *mut mrb_value;
    let mut argc: mrb_int = 0;
    let mut i: mrb_int = 0;
    let mut ai: libc::c_int = 0;
    mrb_get_args(mrb, b"*\x00" as *const u8 as *const libc::c_char,
                 &mut argv as *mut *mut mrb_value, &mut argc as *mut mrb_int);
    ai = mrb_gc_arena_save(mrb);
    i = 0i32 as mrb_int;
    while i < argc {
        let mut name: mrb_value =
            mrb_value{value: unnamed_0{f: 0.,}, tt: MRB_TT_FALSE,};
        let mut str: mrb_value =
            mrb_value{value: unnamed_0{f: 0.,}, tt: MRB_TT_FALSE,};
        let mut attr: mrb_value =
            mrb_value{value: unnamed_0{f: 0.,}, tt: MRB_TT_FALSE,};
        let mut method: mrb_sym = 0;
        let mut sym: mrb_sym = 0;
        let mut p: *mut RProc = 0 as *mut RProc;
        let mut m: mrb_method_t =
            mrb_method_t{func_p: 0,
                         unnamed: unnamed{proc_0: 0 as *mut RProc,},};
        method = mrb_obj_to_sym(mrb, *argv.offset(i as isize));
        name = mrb_sym2str(mrb, method);
        str =
            mrb_str_new_capa(mrb,
                             (if 0 !=
                                     (*(name.value.p as *mut RString)).flags()
                                         as libc::c_int & 32i32 {
                                  (((*(name.value.p as *mut RString)).flags()
                                        as libc::c_int & 0x7c0i32) >> 6i32) as
                                      mrb_int
                              } else {
                                  (*(name.value.p as
                                         *mut RString)).as_0.heap.len
                              } + 1i32 as libc::c_longlong) as size_t);
        mrb_str_cat(mrb, str, b"@\x00" as *const u8 as *const libc::c_char,
                    (::std::mem::size_of::<[libc::c_char; 2]>() as
                         libc::c_ulong).wrapping_sub(1i32 as libc::c_ulong));
        mrb_str_cat_str(mrb, str, name);
        sym = mrb_intern_str(mrb, str);
        mrb_iv_name_sym_check(mrb, sym);
        attr = mrb_symbol_value(sym);
        str =
            mrb_str_new_capa(mrb,
                             (if 0 !=
                                     (*(str.value.p as *mut RString)).flags()
                                         as libc::c_int & 32i32 {
                                  (((*(str.value.p as *mut RString)).flags()
                                        as libc::c_int & 0x7c0i32) >> 6i32) as
                                      mrb_int
                              } else {
                                  (*(str.value.p as
                                         *mut RString)).as_0.heap.len
                              }) as size_t);
        mrb_str_cat_str(mrb, str, name);
        mrb_str_cat(mrb, str, b"=\x00" as *const u8 as *const libc::c_char,
                    (::std::mem::size_of::<[libc::c_char; 2]>() as
                         libc::c_ulong).wrapping_sub(1i32 as libc::c_ulong));
        method = mrb_intern_str(mrb, str);
        p =
            mrb_proc_new_cfunc_with_env(mrb, Some(attr_writer),
                                        1i32 as mrb_int, &mut attr);
        m.func_p = 0i32 as mrb_bool;
        m.unnamed.proc_0 = p;
        mrb_define_method_raw(mrb, c, method, m);
        mrb_gc_arena_restore(mrb, ai);
        i += 1
    }
    return mrb_nil_value();
}
unsafe extern "C" fn mrb_class_initialize(mut mrb: *mut mrb_state,
                                          mut c: mrb_value) -> mrb_value {
    let mut a: mrb_value =
        mrb_value{value: unnamed_0{f: 0.,}, tt: MRB_TT_FALSE,};
    let mut b: mrb_value =
        mrb_value{value: unnamed_0{f: 0.,}, tt: MRB_TT_FALSE,};
    mrb_get_args(mrb, b"|C&\x00" as *const u8 as *const libc::c_char,
                 &mut a as *mut mrb_value, &mut b as *mut mrb_value);
    if !(b.tt as libc::c_uint == MRB_TT_FALSE as libc::c_int as libc::c_uint
             && 0 == b.value.i) {
        mrb_yield_with_class(mrb, b, 1i32 as mrb_int, &mut c, c,
                             c.value.p as *mut RClass);
    }
    return c;
}
unsafe extern "C" fn mrb_class_new_class(mut mrb: *mut mrb_state,
                                         mut cv: mrb_value) -> mrb_value {
    let mut n: mrb_int = 0;
    let mut super_0: mrb_value =
        mrb_value{value: unnamed_0{f: 0.,}, tt: MRB_TT_FALSE,};
    let mut blk: mrb_value =
        mrb_value{value: unnamed_0{f: 0.,}, tt: MRB_TT_FALSE,};
    let mut new_class: mrb_value =
        mrb_value{value: unnamed_0{f: 0.,}, tt: MRB_TT_FALSE,};
    let mut mid: mrb_sym = 0;
    n =
        mrb_get_args(mrb, b"|C&\x00" as *const u8 as *const libc::c_char,
                     &mut super_0 as *mut mrb_value,
                     &mut blk as *mut mrb_value);
    if n == 0i32 as libc::c_longlong {
        super_0 = mrb_obj_value((*mrb).object_class as *mut libc::c_void)
    }
    new_class =
        mrb_obj_value(mrb_class_new(mrb, super_0.value.p as *mut RClass) as
                          *mut libc::c_void);
    mid =
        mrb_intern_static(mrb,
                          b"initialize\x00" as *const u8 as
                              *const libc::c_char,
                          (::std::mem::size_of::<[libc::c_char; 11]>() as
                               libc::c_ulong).wrapping_sub(1i32 as
                                                               libc::c_ulong));
    if 0 == mrb_func_basic_p(mrb, new_class, mid, Some(mrb_bob_init)) {
        mrb_funcall_with_block(mrb, new_class, mid, n, &mut super_0, blk);
    }
    mrb_class_inherited(mrb, super_0.value.p as *mut RClass,
                        new_class.value.p as *mut RClass);
    return new_class;
}
unsafe extern "C" fn mrb_class_superclass(mut mrb: *mut mrb_state,
                                          mut klass: mrb_value) -> mrb_value {
    let mut c: *mut RClass = 0 as *mut RClass;
    c = klass.value.p as *mut RClass;
    c = (*find_origin(c)).super_0;
    while !c.is_null() &&
              (*c).tt() as libc::c_int == MRB_TT_ICLASS as libc::c_int {
        c = (*find_origin(c)).super_0
    }
    if c.is_null() { return mrb_nil_value() }
    return mrb_obj_value(c as *mut libc::c_void);
}
unsafe extern "C" fn mrb_bob_not(mut mrb: *mut mrb_state, mut cv: mrb_value)
 -> mrb_value {
    return mrb_bool_value(!(cv.tt as libc::c_uint !=
                                MRB_TT_FALSE as libc::c_int as libc::c_uint)
                              as libc::c_int as mrb_bool);
}
/* 15.3.1.3.1  */
/* 15.3.1.3.10 */
/* 15.3.1.3.11 */
/*
 *  call-seq:
 *     obj == other        -> true or false
 *     obj.equal?(other)   -> true or false
 *     obj.eql?(other)     -> true or false
 *
 *  Equality---At the <code>Object</code> level, <code>==</code> returns
 *  <code>true</code> only if <i>obj</i> and <i>other</i> are the
 *  same object. Typically, this method is overridden in descendant
 *  classes to provide class-specific meaning.
 *
 *  Unlike <code>==</code>, the <code>equal?</code> method should never be
 *  overridden by subclasses: it is used to determine object identity
 *  (that is, <code>a.equal?(b)</code> iff <code>a</code> is the same
 *  object as <code>b</code>).
 *
 *  The <code>eql?</code> method returns <code>true</code> if
 *  <i>obj</i> and <i>anObject</i> have the same value. Used by
 *  <code>Hash</code> to test members for equality.  For objects of
 *  class <code>Object</code>, <code>eql?</code> is synonymous with
 *  <code>==</code>. Subclasses normally continue this tradition, but
 *  there are exceptions. <code>Numeric</code> types, for example,
 *  perform type conversion across <code>==</code>, but not across
 *  <code>eql?</code>, so:
 *
 *     1 == 1.0     #=> true
 *     1.eql? 1.0   #=> false
 */
#[no_mangle]
pub unsafe extern "C" fn mrb_obj_equal_m(mut mrb: *mut mrb_state,
                                         mut self_0: mrb_value) -> mrb_value {
    let mut arg: mrb_value =
        mrb_value{value: unnamed_0{f: 0.,}, tt: MRB_TT_FALSE,};
    mrb_get_args(mrb, b"o\x00" as *const u8 as *const libc::c_char,
                 &mut arg as *mut mrb_value);
    return mrb_bool_value(mrb_obj_equal(mrb, self_0, arg));
}
unsafe extern "C" fn mrb_obj_not_equal_m(mut mrb: *mut mrb_state,
                                         mut self_0: mrb_value) -> mrb_value {
    let mut arg: mrb_value =
        mrb_value{value: unnamed_0{f: 0.,}, tt: MRB_TT_FALSE,};
    mrb_get_args(mrb, b"o\x00" as *const u8 as *const libc::c_char,
                 &mut arg as *mut mrb_value);
    return mrb_bool_value((0 == mrb_equal(mrb, self_0, arg)) as libc::c_int as
                              mrb_bool);
}
/*
 * call-seq:
 *   mod.to_s   -> string
 *
 * Return a string representing this module or class. For basic
 * classes and modules, this is the name. For singletons, we
 * show information on the thing we're attached to as well.
 */
#[no_mangle]
pub unsafe extern "C" fn mrb_mod_to_s(mut mrb: *mut mrb_state,
                                      mut klass: mrb_value) -> mrb_value {
    if klass.tt as libc::c_uint ==
           MRB_TT_SCLASS as libc::c_int as libc::c_uint {
        let mut v: mrb_value =
            mrb_iv_get(mrb, klass,
                       mrb_intern_static(mrb,
                                         b"__attached__\x00" as *const u8 as
                                             *const libc::c_char,
                                         (::std::mem::size_of::<[libc::c_char; 13]>()
                                              as
                                              libc::c_ulong).wrapping_sub(1i32
                                                                              as
                                                                              libc::c_ulong)));
        let mut str: mrb_value =
            mrb_str_new_static(mrb,
                               b"#<Class:\x00" as *const u8 as
                                   *const libc::c_char,
                               (::std::mem::size_of::<[libc::c_char; 9]>() as
                                    libc::c_ulong).wrapping_sub(1i32 as
                                                                    libc::c_ulong));
        if 0 != class_ptr_p(v) {
            mrb_str_cat_str(mrb, str, mrb_inspect(mrb, v));
        } else { mrb_str_cat_str(mrb, str, mrb_any_to_s(mrb, v)); }
        return mrb_str_cat(mrb, str,
                           b">\x00" as *const u8 as *const libc::c_char,
                           (::std::mem::size_of::<[libc::c_char; 2]>() as
                                libc::c_ulong).wrapping_sub(1i32 as
                                                                libc::c_ulong))
    } else { return class_name_str(mrb, klass.value.p as *mut RClass) };
}
unsafe extern "C" fn mrb_mod_alias(mut mrb: *mut mrb_state,
                                   mut mod_0: mrb_value) -> mrb_value {
    let mut c: *mut RClass = mod_0.value.p as *mut RClass;
    let mut new_name: mrb_sym = 0;
    let mut old_name: mrb_sym = 0;
    mrb_get_args(mrb, b"nn\x00" as *const u8 as *const libc::c_char,
                 &mut new_name as *mut mrb_sym,
                 &mut old_name as *mut mrb_sym);
    mrb_alias_method(mrb, c, new_name, old_name);
    return mod_0;
}
unsafe extern "C" fn mrb_mod_undef(mut mrb: *mut mrb_state,
                                   mut mod_0: mrb_value) -> mrb_value {
    let mut c: *mut RClass = mod_0.value.p as *mut RClass;
    let mut argc: mrb_int = 0;
    let mut argv: *mut mrb_value = 0 as *mut mrb_value;
    mrb_get_args(mrb, b"*\x00" as *const u8 as *const libc::c_char,
                 &mut argv as *mut *mut mrb_value, &mut argc as *mut mrb_int);
    loop  {
        let fresh22 = argc;
        argc = argc - 1;
        if !(0 != fresh22) { break ; }
        mrb_undef_method_id(mrb, c, mrb_obj_to_sym(mrb, *argv));
        argv = argv.offset(1isize)
    }
    return mrb_nil_value();
}
unsafe extern "C" fn check_const_name_sym(mut mrb: *mut mrb_state,
                                          mut id: mrb_sym) {
    let mut len: mrb_int = 0;
    let mut name: *const libc::c_char = mrb_sym2name_len(mrb, id, &mut len);
    if 0 == mrb_const_name_p(mrb, name, len) {
        mrb_name_error(mrb, id,
                       b"wrong constant name %S\x00" as *const u8 as
                           *const libc::c_char, mrb_sym2str(mrb, id));
    };
}
unsafe extern "C" fn mrb_mod_const_defined(mut mrb: *mut mrb_state,
                                           mut mod_0: mrb_value)
 -> mrb_value {
    let mut id: mrb_sym = 0;
    let mut inherit: mrb_bool = 1i32 as mrb_bool;
    mrb_get_args(mrb, b"n|b\x00" as *const u8 as *const libc::c_char,
                 &mut id as *mut mrb_sym, &mut inherit as *mut mrb_bool);
    check_const_name_sym(mrb, id);
    if 0 != inherit {
        return mrb_bool_value(mrb_const_defined(mrb, mod_0, id))
    }
    return mrb_bool_value(mrb_const_defined_at(mrb, mod_0, id));
}
unsafe extern "C" fn mrb_const_get_sym(mut mrb: *mut mrb_state,
                                       mut mod_0: mrb_value, mut id: mrb_sym)
 -> mrb_value {
    check_const_name_sym(mrb, id);
    return mrb_const_get(mrb, mod_0, id);
}
unsafe extern "C" fn mrb_mod_const_get(mut mrb: *mut mrb_state,
                                       mut mod_0: mrb_value) -> mrb_value {
    let mut path: mrb_value =
        mrb_value{value: unnamed_0{f: 0.,}, tt: MRB_TT_FALSE,};
    let mut id: mrb_sym = 0;
    let mut ptr: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut off: mrb_int = 0;
    let mut end: mrb_int = 0;
    let mut len: mrb_int = 0;
    mrb_get_args(mrb, b"o\x00" as *const u8 as *const libc::c_char,
                 &mut path as *mut mrb_value);
    if path.tt as libc::c_uint == MRB_TT_SYMBOL as libc::c_int as libc::c_uint
       {
        id = path.value.sym;
        return mrb_const_get_sym(mrb, mod_0, id)
    }
    path = mrb_ensure_string_type(mrb, path);
    ptr =
        if 0 !=
               (*(path.value.p as *mut RString)).flags() as libc::c_int &
                   32i32 {
            (*(path.value.p as *mut RString)).as_0.ary.as_mut_ptr()
        } else { (*(path.value.p as *mut RString)).as_0.heap.ptr };
    len =
        if 0 !=
               (*(path.value.p as *mut RString)).flags() as libc::c_int &
                   32i32 {
            (((*(path.value.p as *mut RString)).flags() as libc::c_int &
                  0x7c0i32) >> 6i32) as mrb_int
        } else { (*(path.value.p as *mut RString)).as_0.heap.len };
    off = 0i32 as mrb_int;
    while off < len {
        end =
            mrb_str_index(mrb, path,
                          b"::\x00" as *const u8 as *const libc::c_char,
                          (::std::mem::size_of::<[libc::c_char; 3]>() as
                               libc::c_ulong).wrapping_sub(1i32 as
                                                               libc::c_ulong)
                              as mrb_int, off);
        end = if end == -1i32 as libc::c_longlong { len } else { end };
        id = mrb_intern(mrb, ptr.offset(off as isize), (end - off) as size_t);
        mod_0 = mrb_const_get_sym(mrb, mod_0, id);
        if end == len {
            off = end
        } else {
            off = end + 2i32 as libc::c_longlong;
            if off == len {
                mrb_name_error(mrb, id,
                               b"wrong constant name \'%S\'\x00" as *const u8
                                   as *const libc::c_char, path);
            }
        }
    }
    return mod_0;
}
unsafe extern "C" fn mrb_mod_const_set(mut mrb: *mut mrb_state,
                                       mut mod_0: mrb_value) -> mrb_value {
    let mut id: mrb_sym = 0;
    let mut value: mrb_value =
        mrb_value{value: unnamed_0{f: 0.,}, tt: MRB_TT_FALSE,};
    mrb_get_args(mrb, b"no\x00" as *const u8 as *const libc::c_char,
                 &mut id as *mut mrb_sym, &mut value as *mut mrb_value);
    check_const_name_sym(mrb, id);
    mrb_const_set(mrb, mod_0, id, value);
    return value;
}
unsafe extern "C" fn mrb_mod_remove_const(mut mrb: *mut mrb_state,
                                          mut mod_0: mrb_value) -> mrb_value {
    let mut id: mrb_sym = 0;
    let mut val: mrb_value =
        mrb_value{value: unnamed_0{f: 0.,}, tt: MRB_TT_FALSE,};
    mrb_get_args(mrb, b"n\x00" as *const u8 as *const libc::c_char,
                 &mut id as *mut mrb_sym);
    check_const_name_sym(mrb, id);
    val = mrb_iv_remove(mrb, mod_0, id);
    if val.tt as libc::c_uint == MRB_TT_UNDEF as libc::c_int as libc::c_uint {
        mrb_name_error(mrb, id,
                       b"constant %S not defined\x00" as *const u8 as
                           *const libc::c_char, mrb_sym2str(mrb, id));
    }
    return val;
}
unsafe extern "C" fn mrb_mod_const_missing(mut mrb: *mut mrb_state,
                                           mut mod_0: mrb_value)
 -> mrb_value {
    let mut sym: mrb_sym = 0;
    mrb_get_args(mrb, b"n\x00" as *const u8 as *const libc::c_char,
                 &mut sym as *mut mrb_sym);
    if mrb_class_real(mod_0.value.p as *mut RClass) != (*mrb).object_class {
        mrb_name_error(mrb, sym,
                       b"uninitialized constant %S::%S\x00" as *const u8 as
                           *const libc::c_char, mod_0, mrb_sym2str(mrb, sym));
    } else {
        mrb_name_error(mrb, sym,
                       b"uninitialized constant %S\x00" as *const u8 as
                           *const libc::c_char, mrb_sym2str(mrb, sym));
    };
}
/* 15.2.2.4.34 */
/*
 *  call-seq:
 *     mod.method_defined?(symbol)    -> true or false
 *
 *  Returns +true+ if the named method is defined by
 *  _mod_ (or its included modules and, if _mod_ is a class,
 *  its ancestors). Public and protected methods are matched.
 *
 *     module A
 *       def method1()  end
 *     end
 *     class B
 *       def method2()  end
 *     end
 *     class C < B
 *       include A
 *       def method3()  end
 *     end
 *
 *     A.method_defined? :method1    #=> true
 *     C.method_defined? "method1"   #=> true
 *     C.method_defined? "method2"   #=> true
 *     C.method_defined? "method3"   #=> true
 *     C.method_defined? "method4"   #=> false
 */
unsafe extern "C" fn mrb_mod_method_defined(mut mrb: *mut mrb_state,
                                            mut mod_0: mrb_value)
 -> mrb_value {
    let mut id: mrb_sym = 0;
    mrb_get_args(mrb, b"n\x00" as *const u8 as *const libc::c_char,
                 &mut id as *mut mrb_sym);
    return mrb_bool_value(mrb_obj_respond_to(mrb,
                                             mod_0.value.p as *mut RClass,
                                             id));
}
unsafe extern "C" fn mod_define_method(mut mrb: *mut mrb_state,
                                       mut self_0: mrb_value) -> mrb_value {
    let mut c: *mut RClass = self_0.value.p as *mut RClass;
    let mut p: *mut RProc = 0 as *mut RProc;
    let mut m: mrb_method_t =
        mrb_method_t{func_p: 0, unnamed: unnamed{proc_0: 0 as *mut RProc,},};
    let mut mid: mrb_sym = 0;
    let mut proc_0: mrb_value = mrb_undef_value();
    let mut blk: mrb_value =
        mrb_value{value: unnamed_0{f: 0.,}, tt: MRB_TT_FALSE,};
    mrb_get_args(mrb, b"n|o&\x00" as *const u8 as *const libc::c_char,
                 &mut mid as *mut mrb_sym, &mut proc_0 as *mut mrb_value,
                 &mut blk as *mut mrb_value);
    match proc_0.tt as libc::c_uint {
        13 => { blk = proc_0 }
        5 => { }
        _ => {
            /* ignored */
            mrb_raisef(mrb,
                       mrb_exc_get(mrb,
                                   b"TypeError\x00" as *const u8 as
                                       *const libc::c_char),
                       b"wrong argument type %S (expected Proc)\x00" as
                           *const u8 as *const libc::c_char,
                       mrb_obj_value(mrb_obj_class(mrb, proc_0) as
                                         *mut libc::c_void));
        }
    }
    if blk.tt as libc::c_uint == MRB_TT_FALSE as libc::c_int as libc::c_uint
           && 0 == blk.value.i {
        mrb_raise(mrb,
                  mrb_exc_get(mrb,
                              b"ArgumentError\x00" as *const u8 as
                                  *const libc::c_char),
                  b"no block given\x00" as *const u8 as *const libc::c_char);
    }
    p = mrb_obj_alloc(mrb, MRB_TT_PROC, (*mrb).proc_class) as *mut RProc;
    mrb_proc_copy(p, blk.value.p as *mut RProc);
    (*p).set_flags((*p).flags() | 256i32 as uint32_t);
    m.func_p = 0i32 as mrb_bool;
    m.unnamed.proc_0 = p;
    mrb_define_method_raw(mrb, c, mid, m);
    return mrb_symbol_value(mid);
}
unsafe extern "C" fn top_define_method(mut mrb: *mut mrb_state,
                                       mut self_0: mrb_value) -> mrb_value {
    return mod_define_method(mrb,
                             mrb_obj_value((*mrb).object_class as
                                               *mut libc::c_void));
}
unsafe extern "C" fn mrb_mod_eqq(mut mrb: *mut mrb_state,
                                 mut mod_0: mrb_value) -> mrb_value {
    let mut obj: mrb_value =
        mrb_value{value: unnamed_0{f: 0.,}, tt: MRB_TT_FALSE,};
    let mut eqq: mrb_bool = 0;
    mrb_get_args(mrb, b"o\x00" as *const u8 as *const libc::c_char,
                 &mut obj as *mut mrb_value);
    eqq = mrb_obj_is_kind_of(mrb, obj, mod_0.value.p as *mut RClass);
    return mrb_bool_value(eqq);
}
unsafe extern "C" fn mrb_mod_module_function(mut mrb: *mut mrb_state,
                                             mut mod_0: mrb_value)
 -> mrb_value {
    let mut argv: *mut mrb_value = 0 as *mut mrb_value;
    let mut argc: mrb_int = 0;
    let mut i: mrb_int = 0;
    let mut mid: mrb_sym = 0;
    let mut m: mrb_method_t =
        mrb_method_t{func_p: 0, unnamed: unnamed{proc_0: 0 as *mut RProc,},};
    let mut rclass: *mut RClass = 0 as *mut RClass;
    let mut ai: libc::c_int = 0;
    mrb_check_type(mrb, mod_0, MRB_TT_MODULE);
    mrb_get_args(mrb, b"*\x00" as *const u8 as *const libc::c_char,
                 &mut argv as *mut *mut mrb_value, &mut argc as *mut mrb_int);
    if argc == 0i32 as libc::c_longlong { return mod_0 }
    i = 0i32 as mrb_int;
    while i < argc {
        mrb_check_type(mrb, *argv.offset(i as isize), MRB_TT_SYMBOL);
        mid = (*argv.offset(i as isize)).value.sym;
        rclass = mod_0.value.p as *mut RClass;
        m = mrb_method_search(mrb, rclass, mid);
        prepare_singleton_class(mrb, rclass as *mut RBasic);
        ai = mrb_gc_arena_save(mrb);
        mrb_define_method_raw(mrb, (*rclass).c, mid, m);
        mrb_gc_arena_restore(mrb, ai);
        i += 1
    }
    return mod_0;
}
unsafe extern "C" fn inspect_main(mut mrb: *mut mrb_state,
                                  mut mod_0: mrb_value) -> mrb_value {
    return mrb_str_new_static(mrb,
                              b"main\x00" as *const u8 as *const libc::c_char,
                              (::std::mem::size_of::<[libc::c_char; 5]>() as
                                   libc::c_ulong).wrapping_sub(1i32 as
                                                                   libc::c_ulong));
}
#[no_mangle]
pub unsafe extern "C" fn mrb_init_class(mut mrb: *mut mrb_state) {
    /* BasicObject */
    let mut bob: *mut RClass = 0 as *mut RClass;
    /* Object */
    let mut obj: *mut RClass = 0 as *mut RClass;
    /* Module */
    let mut mod_0: *mut RClass = 0 as *mut RClass;
    /* Class */
    let mut cls: *mut RClass = 0 as *mut RClass;
    bob = boot_defclass(mrb, 0 as *mut RClass);
    obj = boot_defclass(mrb, bob);
    (*mrb).object_class = obj;
    mod_0 = boot_defclass(mrb, obj);
    (*mrb).module_class = mod_0;
    cls = boot_defclass(mrb, mod_0);
    (*mrb).class_class = cls;
    (*cls).c = cls;
    (*mod_0).c = (*cls).c;
    (*obj).c = (*mod_0).c;
    (*bob).c = (*obj).c;
    prepare_singleton_class(mrb, bob as *mut RBasic);
    prepare_singleton_class(mrb, obj as *mut RBasic);
    prepare_singleton_class(mrb, mod_0 as *mut RBasic);
    prepare_singleton_class(mrb, cls as *mut RBasic);
    mrb_define_const(mrb, bob,
                     b"BasicObject\x00" as *const u8 as *const libc::c_char,
                     mrb_obj_value(bob as *mut libc::c_void));
    mrb_define_const(mrb, obj,
                     b"BasicObject\x00" as *const u8 as *const libc::c_char,
                     mrb_obj_value(bob as *mut libc::c_void));
    mrb_define_const(mrb, obj,
                     b"Object\x00" as *const u8 as *const libc::c_char,
                     mrb_obj_value(obj as *mut libc::c_void));
    mrb_define_const(mrb, obj,
                     b"Module\x00" as *const u8 as *const libc::c_char,
                     mrb_obj_value(mod_0 as *mut libc::c_void));
    mrb_define_const(mrb, obj,
                     b"Class\x00" as *const u8 as *const libc::c_char,
                     mrb_obj_value(cls as *mut libc::c_void));
    mrb_class_name_class(mrb, 0 as *mut RClass, bob,
                         mrb_intern_static(mrb,
                                           b"BasicObject\x00" as *const u8 as
                                               *const libc::c_char,
                                           (::std::mem::size_of::<[libc::c_char; 12]>()
                                                as
                                                libc::c_ulong).wrapping_sub(1i32
                                                                                as
                                                                                libc::c_ulong)));
    mrb_class_name_class(mrb, 0 as *mut RClass, obj,
                         mrb_intern_static(mrb,
                                           b"Object\x00" as *const u8 as
                                               *const libc::c_char,
                                           (::std::mem::size_of::<[libc::c_char; 7]>()
                                                as
                                                libc::c_ulong).wrapping_sub(1i32
                                                                                as
                                                                                libc::c_ulong)));
    mrb_class_name_class(mrb, 0 as *mut RClass, mod_0,
                         mrb_intern_static(mrb,
                                           b"Module\x00" as *const u8 as
                                               *const libc::c_char,
                                           (::std::mem::size_of::<[libc::c_char; 7]>()
                                                as
                                                libc::c_ulong).wrapping_sub(1i32
                                                                                as
                                                                                libc::c_ulong)));
    mrb_class_name_class(mrb, 0 as *mut RClass, cls,
                         mrb_intern_static(mrb,
                                           b"Class\x00" as *const u8 as
                                               *const libc::c_char,
                                           (::std::mem::size_of::<[libc::c_char; 6]>()
                                                as
                                                libc::c_ulong).wrapping_sub(1i32
                                                                                as
                                                                                libc::c_ulong)));
    (*mrb).proc_class =
        mrb_define_class(mrb, b"Proc\x00" as *const u8 as *const libc::c_char,
                         (*mrb).object_class);
    (*(*mrb).proc_class).set_flags(((*(*mrb).proc_class).flags() as
                                        libc::c_int & !0xffi32 |
                                        MRB_TT_PROC as libc::c_int as
                                            libc::c_char as libc::c_int) as
                                       uint32_t);
    (*cls).set_flags(((*cls).flags() as libc::c_int & !0xffi32 |
                          MRB_TT_CLASS as libc::c_int as libc::c_char as
                              libc::c_int) as uint32_t);
    mrb_define_method(mrb, bob,
                      b"initialize\x00" as *const u8 as *const libc::c_char,
                      Some(mrb_bob_init), 0i32 as mrb_aspec);
    mrb_define_method(mrb, bob, b"!\x00" as *const u8 as *const libc::c_char,
                      Some(mrb_bob_not), 0i32 as mrb_aspec);
    mrb_define_method(mrb, bob, b"==\x00" as *const u8 as *const libc::c_char,
                      Some(mrb_obj_equal_m),
                      ((1i32 & 0x1fi32) as mrb_aspec) << 18i32);
    mrb_define_method(mrb, bob, b"!=\x00" as *const u8 as *const libc::c_char,
                      Some(mrb_obj_not_equal_m),
                      ((1i32 & 0x1fi32) as mrb_aspec) << 18i32);
    mrb_define_method(mrb, bob,
                      b"__id__\x00" as *const u8 as *const libc::c_char,
                      Some(mrb_obj_id_m), 0i32 as mrb_aspec);
    mrb_define_method(mrb, bob,
                      b"__send__\x00" as *const u8 as *const libc::c_char,
                      Some(mrb_f_send), (1i32 << 12i32) as mrb_aspec);
    mrb_define_method(mrb, bob,
                      b"equal?\x00" as *const u8 as *const libc::c_char,
                      Some(mrb_obj_equal_m),
                      ((1i32 & 0x1fi32) as mrb_aspec) << 18i32);
    mrb_define_method(mrb, bob,
                      b"instance_eval\x00" as *const u8 as
                          *const libc::c_char, Some(mrb_obj_instance_eval),
                      (1i32 << 12i32) as mrb_aspec);
    mrb_define_class_method(mrb, cls,
                            b"new\x00" as *const u8 as *const libc::c_char,
                            Some(mrb_class_new_class),
                            ((1i32 & 0x1fi32) as mrb_aspec) << 13i32);
    mrb_define_method(mrb, cls,
                      b"superclass\x00" as *const u8 as *const libc::c_char,
                      Some(mrb_class_superclass), 0i32 as mrb_aspec);
    mrb_define_method(mrb, cls,
                      b"new\x00" as *const u8 as *const libc::c_char,
                      Some(mrb_instance_new), (1i32 << 12i32) as mrb_aspec);
    mrb_define_method(mrb, cls,
                      b"initialize\x00" as *const u8 as *const libc::c_char,
                      Some(mrb_class_initialize),
                      ((1i32 & 0x1fi32) as mrb_aspec) << 13i32);
    mrb_define_method(mrb, cls,
                      b"inherited\x00" as *const u8 as *const libc::c_char,
                      Some(mrb_bob_init),
                      ((1i32 & 0x1fi32) as mrb_aspec) << 18i32);
    (*mod_0).set_flags(((*mod_0).flags() as libc::c_int & !0xffi32 |
                            MRB_TT_MODULE as libc::c_int as libc::c_char as
                                libc::c_int) as uint32_t);
    mrb_define_method(mrb, mod_0,
                      b"extend_object\x00" as *const u8 as
                          *const libc::c_char, Some(mrb_mod_extend_object),
                      ((1i32 & 0x1fi32) as mrb_aspec) << 18i32);
    mrb_define_method(mrb, mod_0,
                      b"extended\x00" as *const u8 as *const libc::c_char,
                      Some(mrb_bob_init),
                      ((1i32 & 0x1fi32) as mrb_aspec) << 18i32);
    mrb_define_method(mrb, mod_0,
                      b"prepended\x00" as *const u8 as *const libc::c_char,
                      Some(mrb_bob_init),
                      ((1i32 & 0x1fi32) as mrb_aspec) << 18i32);
    mrb_define_method(mrb, mod_0,
                      b"prepend_features\x00" as *const u8 as
                          *const libc::c_char, Some(mrb_mod_prepend_features),
                      ((1i32 & 0x1fi32) as mrb_aspec) << 18i32);
    mrb_define_method(mrb, mod_0,
                      b"include?\x00" as *const u8 as *const libc::c_char,
                      Some(mrb_mod_include_p),
                      ((1i32 & 0x1fi32) as mrb_aspec) << 18i32);
    mrb_define_method(mrb, mod_0,
                      b"append_features\x00" as *const u8 as
                          *const libc::c_char, Some(mrb_mod_append_features),
                      ((1i32 & 0x1fi32) as mrb_aspec) << 18i32);
    mrb_define_method(mrb, mod_0,
                      b"class_eval\x00" as *const u8 as *const libc::c_char,
                      Some(mrb_mod_module_eval),
                      (1i32 << 12i32) as mrb_aspec);
    mrb_define_method(mrb, mod_0,
                      b"included\x00" as *const u8 as *const libc::c_char,
                      Some(mrb_bob_init),
                      ((1i32 & 0x1fi32) as mrb_aspec) << 18i32);
    mrb_define_method(mrb, mod_0,
                      b"initialize\x00" as *const u8 as *const libc::c_char,
                      Some(mrb_mod_initialize), 0i32 as mrb_aspec);
    mrb_define_method(mrb, mod_0,
                      b"module_eval\x00" as *const u8 as *const libc::c_char,
                      Some(mrb_mod_module_eval),
                      (1i32 << 12i32) as mrb_aspec);
    mrb_define_method(mrb, mod_0,
                      b"module_function\x00" as *const u8 as
                          *const libc::c_char, Some(mrb_mod_module_function),
                      (1i32 << 12i32) as mrb_aspec);
    mrb_define_method(mrb, mod_0,
                      b"private\x00" as *const u8 as *const libc::c_char,
                      Some(mrb_mod_dummy_visibility),
                      (1i32 << 12i32) as mrb_aspec);
    mrb_define_method(mrb, mod_0,
                      b"protected\x00" as *const u8 as *const libc::c_char,
                      Some(mrb_mod_dummy_visibility),
                      (1i32 << 12i32) as mrb_aspec);
    mrb_define_method(mrb, mod_0,
                      b"public\x00" as *const u8 as *const libc::c_char,
                      Some(mrb_mod_dummy_visibility),
                      (1i32 << 12i32) as mrb_aspec);
    mrb_define_method(mrb, mod_0,
                      b"attr_reader\x00" as *const u8 as *const libc::c_char,
                      Some(mrb_mod_attr_reader),
                      (1i32 << 12i32) as mrb_aspec);
    mrb_define_method(mrb, mod_0,
                      b"attr_writer\x00" as *const u8 as *const libc::c_char,
                      Some(mrb_mod_attr_writer),
                      (1i32 << 12i32) as mrb_aspec);
    mrb_define_method(mrb, mod_0,
                      b"to_s\x00" as *const u8 as *const libc::c_char,
                      Some(mrb_mod_to_s), 0i32 as mrb_aspec);
    mrb_define_method(mrb, mod_0,
                      b"inspect\x00" as *const u8 as *const libc::c_char,
                      Some(mrb_mod_to_s), 0i32 as mrb_aspec);
    mrb_define_method(mrb, mod_0,
                      b"alias_method\x00" as *const u8 as *const libc::c_char,
                      Some(mrb_mod_alias), (1i32 << 12i32) as mrb_aspec);
    mrb_define_method(mrb, mod_0,
                      b"ancestors\x00" as *const u8 as *const libc::c_char,
                      Some(mrb_mod_ancestors), 0i32 as mrb_aspec);
    mrb_define_method(mrb, mod_0,
                      b"undef_method\x00" as *const u8 as *const libc::c_char,
                      Some(mrb_mod_undef), (1i32 << 12i32) as mrb_aspec);
    mrb_define_method(mrb, mod_0,
                      b"const_defined?\x00" as *const u8 as
                          *const libc::c_char, Some(mrb_mod_const_defined),
                      ((1i32 & 0x1fi32) as mrb_aspec) << 18i32 |
                          ((1i32 & 0x1fi32) as mrb_aspec) << 13i32);
    mrb_define_method(mrb, mod_0,
                      b"const_get\x00" as *const u8 as *const libc::c_char,
                      Some(mrb_mod_const_get),
                      ((1i32 & 0x1fi32) as mrb_aspec) << 18i32);
    mrb_define_method(mrb, mod_0,
                      b"const_set\x00" as *const u8 as *const libc::c_char,
                      Some(mrb_mod_const_set),
                      ((2i32 & 0x1fi32) as mrb_aspec) << 18i32);
    mrb_define_method(mrb, mod_0,
                      b"remove_const\x00" as *const u8 as *const libc::c_char,
                      Some(mrb_mod_remove_const),
                      ((1i32 & 0x1fi32) as mrb_aspec) << 18i32);
    mrb_define_method(mrb, mod_0,
                      b"const_missing\x00" as *const u8 as
                          *const libc::c_char, Some(mrb_mod_const_missing),
                      ((1i32 & 0x1fi32) as mrb_aspec) << 18i32);
    mrb_define_method(mrb, mod_0,
                      b"method_defined?\x00" as *const u8 as
                          *const libc::c_char, Some(mrb_mod_method_defined),
                      ((1i32 & 0x1fi32) as mrb_aspec) << 18i32);
    mrb_define_method(mrb, mod_0,
                      b"define_method\x00" as *const u8 as
                          *const libc::c_char, Some(mod_define_method),
                      ((1i32 & 0x1fi32) as mrb_aspec) << 18i32 |
                          ((1i32 & 0x1fi32) as mrb_aspec) << 13i32);
    mrb_define_method(mrb, mod_0,
                      b"===\x00" as *const u8 as *const libc::c_char,
                      Some(mrb_mod_eqq),
                      ((1i32 & 0x1fi32) as mrb_aspec) << 18i32);
    mrb_undef_method(mrb, cls,
                     b"append_features\x00" as *const u8 as
                         *const libc::c_char);
    mrb_undef_method(mrb, cls,
                     b"extend_object\x00" as *const u8 as
                         *const libc::c_char);
    (*mrb).top_self =
        mrb_obj_alloc(mrb, MRB_TT_OBJECT, (*mrb).object_class) as
            *mut RObject;
    mrb_define_singleton_method(mrb, (*mrb).top_self,
                                b"inspect\x00" as *const u8 as
                                    *const libc::c_char, Some(inspect_main),
                                0i32 as mrb_aspec);
    mrb_define_singleton_method(mrb, (*mrb).top_self,
                                b"to_s\x00" as *const u8 as
                                    *const libc::c_char, Some(inspect_main),
                                0i32 as mrb_aspec);
    mrb_define_singleton_method(mrb, (*mrb).top_self,
                                b"define_method\x00" as *const u8 as
                                    *const libc::c_char,
                                Some(top_define_method),
                                ((1i32 & 0x1fi32) as mrb_aspec) << 18i32 |
                                    ((1i32 & 0x1fi32) as mrb_aspec) << 13i32);
}