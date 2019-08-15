use libc;
use c2rust_bitfields::BitfieldStruct;
extern "C" {
    pub type iv_tbl;
    pub type symbol_name;
    pub type mrb_shared_string;
    #[no_mangle]
    fn __error() -> *mut libc::c_int;
    #[no_mangle]
    fn abort() -> !;
    #[no_mangle]
    fn exit(_: libc::c_int) -> !;
    /* compatibility macros */
    #[no_mangle]
    fn mrb_p(_: *mut mrb_state, _: mrb_value);
    #[no_mangle]
    fn mrb_str_new_static(mrb: *mut mrb_state, p: *const libc::c_char,
                          len: size_t) -> mrb_value;
    /* *
 * Turns a C string into a Ruby string value.
 */
    #[no_mangle]
    fn mrb_str_new_cstr(_: *mut mrb_state, _: *const libc::c_char)
     -> mrb_value;
    #[no_mangle]
    fn mrb_str_new(mrb: *mut mrb_state, p: *const libc::c_char, len: size_t)
     -> mrb_value;
    #[no_mangle]
    fn mrb_intern_static(_: *mut mrb_state, _: *const libc::c_char, _: size_t)
     -> mrb_sym;
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
    fn mrb_obj_new(mrb: *mut mrb_state, c: *mut RClass, argc: mrb_int,
                   argv: *const mrb_value) -> mrb_value;
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
    fn mrb_class_defined(mrb: *mut mrb_state, name: *const libc::c_char)
     -> mrb_bool;
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
    /* string type checking (contrary to the name, it doesn't convert) */
    #[no_mangle]
    fn mrb_to_str(mrb: *mut mrb_state, val: mrb_value) -> mrb_value;
    #[no_mangle]
    fn mrb_obj_clone(mrb: *mut mrb_state, self_0: mrb_value) -> mrb_value;
    #[no_mangle]
    fn mrb_obj_is_kind_of(mrb: *mut mrb_state, obj: mrb_value, c: *mut RClass)
     -> mrb_bool;
    #[no_mangle]
    fn mrb_obj_class(mrb: *mut mrb_state, obj: mrb_value) -> *mut RClass;
    #[no_mangle]
    fn mrb_obj_classname(mrb: *mut mrb_state, obj: mrb_value)
     -> *const libc::c_char;
    #[no_mangle]
    fn mrb_obj_equal(_: *mut mrb_state, _: mrb_value, _: mrb_value)
     -> mrb_bool;
    #[no_mangle]
    fn mrb_inspect(mrb: *mut mrb_state, obj: mrb_value) -> mrb_value;
    #[no_mangle]
    fn mrb_attr_get(mrb: *mut mrb_state, obj: mrb_value, id: mrb_sym)
     -> mrb_value;
    #[no_mangle]
    fn mrb_respond_to(mrb: *mut mrb_state, obj: mrb_value, mid: mrb_sym)
     -> mrb_bool;
    #[no_mangle]
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    /*
 * Returns an object as a Ruby string.
 *
 * @param [mrb_state] mrb The current mruby state.
 * @param [mrb_value] obj An object to return as a Ruby string.
 * @return [mrb_value] An object as a Ruby string.
 */
    #[no_mangle]
    fn mrb_obj_as_string(mrb: *mut mrb_state, obj: mrb_value) -> mrb_value;
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
    #[no_mangle]
    fn mrb_obj_iv_set(mrb: *mut mrb_state, obj: *mut RObject, sym: mrb_sym,
                      v: mrb_value);
    #[no_mangle]
    fn mrb_obj_iv_defined(mrb: *mut mrb_state, obj: *mut RObject,
                          sym: mrb_sym) -> mrb_bool;
    #[no_mangle]
    fn mrb_iv_set(mrb: *mut mrb_state, obj: mrb_value, sym: mrb_sym,
                  v: mrb_value);
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
    fn mrb_exc_backtrace(mrb: *mut mrb_state, exc: mrb_value) -> mrb_value;
    #[no_mangle]
    fn mrb_instance_new(mrb: *mut mrb_state, cv: mrb_value) -> mrb_value;
    #[no_mangle]
    fn _longjmp(_: *mut libc::c_int, _: libc::c_int) -> !;
    #[no_mangle]
    fn mrb_keep_backtrace(mrb: *mut mrb_state, exc: mrb_value);
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
pub type __darwin_ptrdiff_t = libc::c_long;
pub type __darwin_size_t = libc::c_ulong;
pub type int32_t = libc::c_int;
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
    pub lines: C2RustUnnamed_3,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub union C2RustUnnamed_3 {
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
/*
** mruby/compile.h - mruby parser
**
** See Copyright Notice in mruby.h
*/
/* *
 * MRuby Compiler
 */
#[derive ( Copy , Clone )]
#[repr(C)]
pub struct mrb_jmpbuf {
    pub impl_0: jmp_buf,
}
pub type jmp_buf = [libc::c_int; 37];
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
#[derive ( Copy , Clone )]
#[repr(C)]
pub struct C2RustUnnamed_4 {
    pub len: mrb_int,
    pub aux: C2RustUnnamed_5,
    pub ptr: *mut libc::c_char,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub union C2RustUnnamed_5 {
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
    #[bitfield(padding)]
    pub _pad: [u8; 4],
    pub c: *mut RClass,
    pub gcnext: *mut RBasic,
    pub as_0: C2RustUnnamed_6,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub union C2RustUnnamed_6 {
    pub heap: C2RustUnnamed_4,
    pub ary: [libc::c_char; 24],
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
#[derive ( BitfieldStruct , Clone , Copy )]
#[repr(C)]
pub struct RArray {
    #[bitfield(name = "tt", ty = "mrb_vtype", bits = "0..=7")]
    #[bitfield(name = "color", ty = "uint32_t", bits = "8..=10")]
    #[bitfield(name = "flags", ty = "uint32_t", bits = "11..=31")]
    pub tt_color_flags: [u8; 4],
    #[bitfield(padding)]
    pub _pad: [u8; 4],
    pub c: *mut RClass,
    pub gcnext: *mut RBasic,
    pub as_0: C2RustUnnamed_7,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub union C2RustUnnamed_7 {
    pub heap: C2RustUnnamed_8,
}
#[derive ( Copy , Clone )]
#[repr(C)]
pub struct C2RustUnnamed_8 {
    pub len: mrb_int,
    pub aux: C2RustUnnamed_9,
    pub ptr: *mut mrb_value,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub union C2RustUnnamed_9 {
    pub capa: mrb_int,
    pub shared: *mut mrb_shared_array,
}
/*
 * Returns a float in Ruby.
 */
#[inline]
unsafe extern "C" fn mrb_float_value(mut mrb: *mut mrb_state,
                                     mut f: mrb_float) -> mrb_value {
    let mut v: mrb_value =
        mrb_value{value: C2RustUnnamed_0{f: 0.,}, tt: MRB_TT_FALSE,};
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
        mrb_value{value: C2RustUnnamed_0{f: 0.,}, tt: MRB_TT_FALSE,};
    v.tt = MRB_TT_FIXNUM;
    v.value.i = i;
    return v;
}
#[inline]
unsafe extern "C" fn mrb_symbol_value(mut i: mrb_sym) -> mrb_value {
    let mut v: mrb_value =
        mrb_value{value: C2RustUnnamed_0{f: 0.,}, tt: MRB_TT_FALSE,};
    v.tt = MRB_TT_SYMBOL;
    v.value.sym = i;
    return v;
}
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
#[inline]
unsafe extern "C" fn mrb_gc_arena_restore(mut mrb: *mut mrb_state,
                                          mut idx: libc::c_int) {
    (*mrb).gc.arena_idx = idx;
}
#[inline]
unsafe extern "C" fn mrb_gc_arena_save(mut mrb: *mut mrb_state)
 -> libc::c_int {
    return (*mrb).gc.arena_idx;
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
** error.c - Exception class
**
** See Copyright Notice in mruby.h
*/
#[no_mangle]
pub unsafe extern "C" fn mrb_exc_new(mut mrb: *mut mrb_state,
                                     mut c: *mut RClass,
                                     mut ptr: *const libc::c_char,
                                     mut len: size_t) -> mrb_value {
    let mut arg: mrb_value = mrb_str_new(mrb, ptr, len);
    return mrb_obj_new(mrb, c, 1i32 as mrb_int, &mut arg);
}
#[no_mangle]
pub unsafe extern "C" fn mrb_exc_new_str(mut mrb: *mut mrb_state,
                                         mut c: *mut RClass,
                                         mut str: mrb_value) -> mrb_value {
    mrb_to_str(mrb, str);
    return mrb_obj_new(mrb, c, 1i32 as mrb_int, &mut str);
}
/*
 * call-seq:
 *    Exception.new(msg = nil)   ->  exception
 *
 *  Construct a new Exception object, optionally passing in
 *  a message.
 */
unsafe extern "C" fn exc_initialize(mut mrb: *mut mrb_state,
                                    mut exc: mrb_value) -> mrb_value {
    let mut mesg: mrb_value =
        mrb_value{value: C2RustUnnamed_0{f: 0.,}, tt: MRB_TT_FALSE,};
    let mut argc: mrb_int = 0;
    let mut argv: *mut mrb_value = 0 as *mut mrb_value;
    if mrb_get_args(mrb, b"|o*!\x00" as *const u8 as *const libc::c_char,
                    &mut mesg as *mut mrb_value,
                    &mut argv as *mut *mut mrb_value,
                    &mut argc as *mut mrb_int) >= 1i32 as libc::c_longlong {
        mrb_iv_set(mrb, exc,
                   mrb_intern_static(mrb,
                                     b"mesg\x00" as *const u8 as
                                         *const libc::c_char,
                                     (::std::mem::size_of::<[libc::c_char; 5]>()
                                          as
                                          libc::c_ulong).wrapping_sub(1i32 as
                                                                          libc::c_ulong)),
                   mesg);
    }
    return exc;
}
/*
 *  Document-method: exception
 *
 *  call-seq:
 *     exc.exception(string)  ->  an_exception or exc
 *
 *  With no argument, or if the argument is the same as the receiver,
 *  return the receiver. Otherwise, create a new
 *  exception object of the same class as the receiver, but with a
 *  message equal to <code>string</code>.
 *
 */
unsafe extern "C" fn exc_exception(mut mrb: *mut mrb_state,
                                   mut self_0: mrb_value) -> mrb_value {
    let mut exc: mrb_value =
        mrb_value{value: C2RustUnnamed_0{f: 0.,}, tt: MRB_TT_FALSE,};
    let mut a: mrb_value =
        mrb_value{value: C2RustUnnamed_0{f: 0.,}, tt: MRB_TT_FALSE,};
    let mut argc: mrb_int = 0;
    argc =
        mrb_get_args(mrb, b"|o\x00" as *const u8 as *const libc::c_char,
                     &mut a as *mut mrb_value);
    if argc == 0i32 as libc::c_longlong { return self_0 }
    if 0 != mrb_obj_equal(mrb, self_0, a) { return self_0 }
    exc = mrb_obj_clone(mrb, self_0);
    mrb_iv_set(mrb, exc,
               mrb_intern_static(mrb,
                                 b"mesg\x00" as *const u8 as
                                     *const libc::c_char,
                                 (::std::mem::size_of::<[libc::c_char; 5]>()
                                      as
                                      libc::c_ulong).wrapping_sub(1i32 as
                                                                      libc::c_ulong)),
               a);
    return exc;
}
/*
 * call-seq:
 *   exception.to_s   ->  string
 *
 * Returns exception's message (or the name of the exception if
 * no message is set).
 */
unsafe extern "C" fn exc_to_s(mut mrb: *mut mrb_state, mut exc: mrb_value)
 -> mrb_value {
    let mut mesg: mrb_value =
        mrb_attr_get(mrb, exc,
                     mrb_intern_static(mrb,
                                       b"mesg\x00" as *const u8 as
                                           *const libc::c_char,
                                       (::std::mem::size_of::<[libc::c_char; 5]>()
                                            as
                                            libc::c_ulong).wrapping_sub(1i32
                                                                            as
                                                                            libc::c_ulong)));
    let mut p: *mut RObject = 0 as *mut RObject;
    if !(mesg.tt as libc::c_uint ==
             MRB_TT_STRING as libc::c_int as libc::c_uint) {
        return mrb_str_new_cstr(mrb, mrb_obj_classname(mrb, exc))
    }
    p = mesg.value.p as *mut RObject;
    if (*p).c.is_null() { (*p).c = (*mrb).string_class }
    return mesg;
}
/*
 * call-seq:
 *   exception.message   ->  string
 *
 * Returns the result of invoking <code>exception.to_s</code>.
 * Normally this returns the exception's message or name.
 */
unsafe extern "C" fn exc_message(mut mrb: *mut mrb_state, mut exc: mrb_value)
 -> mrb_value {
    return mrb_funcall(mrb, exc,
                       b"to_s\x00" as *const u8 as *const libc::c_char,
                       0i32 as mrb_int);
}
/*
 * call-seq:
 *   exception.inspect   -> string
 *
 * Returns this exception's file name, line number,
 * message and class name.
 * If file name or line number is not set,
 * returns message and class name.
 */
unsafe extern "C" fn exc_inspect(mut mrb: *mut mrb_state, mut exc: mrb_value)
 -> mrb_value {
    let mut str: mrb_value =
        mrb_value{value: C2RustUnnamed_0{f: 0.,}, tt: MRB_TT_FALSE,};
    let mut mesg: mrb_value =
        mrb_value{value: C2RustUnnamed_0{f: 0.,}, tt: MRB_TT_FALSE,};
    let mut file: mrb_value =
        mrb_value{value: C2RustUnnamed_0{f: 0.,}, tt: MRB_TT_FALSE,};
    let mut line: mrb_value =
        mrb_value{value: C2RustUnnamed_0{f: 0.,}, tt: MRB_TT_FALSE,};
    let mut append_mesg: mrb_bool = 0;
    let mut cname: *const libc::c_char = 0 as *const libc::c_char;
    mesg =
        mrb_attr_get(mrb, exc,
                     mrb_intern_static(mrb,
                                       b"mesg\x00" as *const u8 as
                                           *const libc::c_char,
                                       (::std::mem::size_of::<[libc::c_char; 5]>()
                                            as
                                            libc::c_ulong).wrapping_sub(1i32
                                                                            as
                                                                            libc::c_ulong)));
    file =
        mrb_attr_get(mrb, exc,
                     mrb_intern_static(mrb,
                                       b"file\x00" as *const u8 as
                                           *const libc::c_char,
                                       (::std::mem::size_of::<[libc::c_char; 5]>()
                                            as
                                            libc::c_ulong).wrapping_sub(1i32
                                                                            as
                                                                            libc::c_ulong)));
    line =
        mrb_attr_get(mrb, exc,
                     mrb_intern_static(mrb,
                                       b"line\x00" as *const u8 as
                                           *const libc::c_char,
                                       (::std::mem::size_of::<[libc::c_char; 5]>()
                                            as
                                            libc::c_ulong).wrapping_sub(1i32
                                                                            as
                                                                            libc::c_ulong)));
    append_mesg =
        !(mesg.tt as libc::c_uint ==
              MRB_TT_FALSE as libc::c_int as libc::c_uint &&
              0 == mesg.value.i) as libc::c_int as mrb_bool;
    if 0 != append_mesg {
        mesg = mrb_obj_as_string(mrb, mesg);
        append_mesg =
            ((if 0 !=
                     (*(mesg.value.p as *mut RString)).flags() as libc::c_int
                         & 32i32 {
                  (((*(mesg.value.p as *mut RString)).flags() as libc::c_int &
                        0x7c0i32) >> 6i32) as mrb_int
              } else { (*(mesg.value.p as *mut RString)).as_0.heap.len }) >
                 0i32 as libc::c_longlong) as libc::c_int as mrb_bool
    }
    cname = mrb_obj_classname(mrb, exc);
    str = mrb_str_new_cstr(mrb, cname);
    if file.tt as libc::c_uint == MRB_TT_STRING as libc::c_int as libc::c_uint
           &&
           line.tt as libc::c_uint ==
               MRB_TT_FIXNUM as libc::c_int as libc::c_uint {
        if 0 != append_mesg {
            str =
                mrb_format(mrb,
                           b"%v:%v: %v (%v)\x00" as *const u8 as
                               *const libc::c_char, file, line, mesg, str)
        } else {
            str =
                mrb_format(mrb,
                           b"%v:%v: %v\x00" as *const u8 as
                               *const libc::c_char, file, line, str)
        }
    } else if 0 != append_mesg {
        str =
            mrb_format(mrb, b"%v: %v\x00" as *const u8 as *const libc::c_char,
                       str, mesg)
    }
    return str;
}
unsafe extern "C" fn set_backtrace(mut mrb: *mut mrb_state,
                                   mut exc: mrb_value,
                                   mut backtrace: mrb_value) {
    let mut current_block_1: u64;
    if !(backtrace.tt as libc::c_uint ==
             MRB_TT_ARRAY as libc::c_int as libc::c_uint) {
        current_block_1 = 2646805700453042256;
    } else {
        let mut p: *const mrb_value =
            if 0 !=
                   (*(backtrace.value.p as *mut RArray)).flags() as
                       libc::c_int & 7i32 {
                &mut (*(backtrace.value.p as *mut RArray)).as_0 as
                    *mut C2RustUnnamed_7 as *mut mrb_value
            } else { (*(backtrace.value.p as *mut RArray)).as_0.heap.ptr };
        let mut pend: *const mrb_value =
            p.offset((if 0 !=
                             (*(backtrace.value.p as *mut RArray)).flags() as
                                 libc::c_int & 7i32 {
                          (((*(backtrace.value.p as *mut RArray)).flags() as
                                libc::c_int & 7i32) - 1i32) as mrb_int
                      } else {
                          (*(backtrace.value.p as *mut RArray)).as_0.heap.len
                      }) as isize);
        loop  {
            if !(p < pend) { current_block_1 = 14523784380283086299; break ; }
            if !((*p).tt as libc::c_uint ==
                     MRB_TT_STRING as libc::c_int as libc::c_uint) {
                current_block_1 = 2646805700453042256;
                break ;
            }
            p = p.offset(1isize)
        }
    }
    match current_block_1 {
        2646805700453042256 => {
            mrb_raise(mrb,
                      mrb_exc_get(mrb,
                                  b"TypeError\x00" as *const u8 as
                                      *const libc::c_char),
                      b"backtrace must be Array of String\x00" as *const u8 as
                          *const libc::c_char);
        }
        _ => { }
    }
    mrb_iv_set(mrb, exc,
               mrb_intern_static(mrb,
                                 b"backtrace\x00" as *const u8 as
                                     *const libc::c_char,
                                 (::std::mem::size_of::<[libc::c_char; 10]>()
                                      as
                                      libc::c_ulong).wrapping_sub(1i32 as
                                                                      libc::c_ulong)),
               backtrace);
}
unsafe extern "C" fn exc_set_backtrace(mut mrb: *mut mrb_state,
                                       mut exc: mrb_value) -> mrb_value {
    let mut backtrace: mrb_value =
        mrb_value{value: C2RustUnnamed_0{f: 0.,}, tt: MRB_TT_FALSE,};
    mrb_get_args(mrb, b"o\x00" as *const u8 as *const libc::c_char,
                 &mut backtrace as *mut mrb_value);
    set_backtrace(mrb, exc, backtrace);
    return backtrace;
}
unsafe extern "C" fn exc_debug_info(mut mrb: *mut mrb_state,
                                    mut exc: *mut RObject) {
    let mut ci: *mut mrb_callinfo = (*(*mrb).c).ci;
    let mut pc: *mut mrb_code = (*ci).pc;
    if 0 !=
           mrb_obj_iv_defined(mrb, exc,
                              mrb_intern_static(mrb,
                                                b"file\x00" as *const u8 as
                                                    *const libc::c_char,
                                                (::std::mem::size_of::<[libc::c_char; 5]>()
                                                     as
                                                     libc::c_ulong).wrapping_sub(1i32
                                                                                     as
                                                                                     libc::c_ulong)))
       {
        return
    }
    while ci >= (*(*mrb).c).cibase {
        let mut err: *mut mrb_code = (*ci).err;
        if err.is_null() && !pc.is_null() { err = pc.offset(-1isize) }
        if !err.is_null() && !(*ci).proc_0.is_null() &&
               !((*(*ci).proc_0).flags() as libc::c_int & 128i32 != 0i32) {
            let mut irep: *mut mrb_irep = (*(*ci).proc_0).body.irep;
            let line: int32_t =
                mrb_debug_get_line(mrb, irep,
                                   err.wrapping_offset_from((*irep).iseq) as
                                       libc::c_long);
            let mut file: *const libc::c_char =
                mrb_debug_get_filename(mrb, irep,
                                       err.wrapping_offset_from((*irep).iseq)
                                           as libc::c_long);
            if line != -1i32 && !file.is_null() {
                mrb_obj_iv_set(mrb, exc,
                               mrb_intern_static(mrb,
                                                 b"file\x00" as *const u8 as
                                                     *const libc::c_char,
                                                 (::std::mem::size_of::<[libc::c_char; 5]>()
                                                      as
                                                      libc::c_ulong).wrapping_sub(1i32
                                                                                      as
                                                                                      libc::c_ulong)),
                               mrb_str_new_cstr(mrb, file));
                mrb_obj_iv_set(mrb, exc,
                               mrb_intern_static(mrb,
                                                 b"line\x00" as *const u8 as
                                                     *const libc::c_char,
                                                 (::std::mem::size_of::<[libc::c_char; 5]>()
                                                      as
                                                      libc::c_ulong).wrapping_sub(1i32
                                                                                      as
                                                                                      libc::c_ulong)),
                               mrb_fixnum_value(line as mrb_int));
                return
            }
        }
        pc = (*ci).pc;
        ci = ci.offset(-1isize)
    };
}
#[no_mangle]
pub unsafe extern "C" fn mrb_exc_set(mut mrb: *mut mrb_state,
                                     mut exc: mrb_value) {
    if exc.tt as libc::c_uint == MRB_TT_FALSE as libc::c_int as libc::c_uint
           && 0 == exc.value.i {
        (*mrb).exc = 0 as *mut RObject
    } else {
        (*mrb).exc = exc.value.p as *mut RObject;
        if (*mrb).gc.arena_idx > 0i32 &&
               (*mrb).exc as *mut RBasic ==
                   *(*mrb).gc.arena.offset(((*mrb).gc.arena_idx - 1i32) as
                                               isize) {
            (*mrb).gc.arena_idx -= 1
        }
        if 0 == (*mrb).gc.out_of_memory() &&
               0 == (*(*mrb).exc).flags() as libc::c_int & 1i32 << 20i32 {
            exc_debug_info(mrb, (*mrb).exc);
            mrb_keep_backtrace(mrb, exc);
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn mrb_exc_raise(mut mrb: *mut mrb_state,
                                       mut exc: mrb_value) {
    if 0 == mrb_obj_is_kind_of(mrb, exc, (*mrb).eException_class) {
        mrb_raise(mrb,
                  mrb_exc_get(mrb,
                              b"TypeError\x00" as *const u8 as
                                  *const libc::c_char),
                  b"exception object expected\x00" as *const u8 as
                      *const libc::c_char);
    }
    mrb_exc_set(mrb, exc);
    if (*mrb).jmp.is_null() { mrb_p(mrb, exc); abort(); }
    _longjmp((*(*mrb).jmp).impl_0.as_mut_ptr(), 1i32);
}
#[no_mangle]
pub unsafe extern "C" fn mrb_raise(mut mrb: *mut mrb_state,
                                   mut c: *mut RClass,
                                   mut msg: *const libc::c_char) {
    mrb_exc_raise(mrb, mrb_exc_new_str(mrb, c, mrb_str_new_cstr(mrb, msg)));
}
/* function for `raisef` formatting */
/*
 * <code>vsprintf</code> like formatting.
 *
 * The syntax of a format sequence is as follows.
 *
 *   %[modifier]specifier
 *
 * The modifiers are:
 *
 *   ----------+------------------------------------------------------------
 *   Modifier  | Meaning
 *   ----------+------------------------------------------------------------
 *       !     | Convert to string by corresponding `inspect` instead of
 *             | corresponding `to_s`.
 *   ----------+------------------------------------------------------------
 *
 * The specifiers are:
 *
 *   ----------+----------------+--------------------------------------------
 *   Specifier | Argument Type  | Note
 *   ----------+----------------+--------------------------------------------
 *       c     | char           |
 *       d     | int            |
 *       f     | mrb_float      |
 *       i     | mrb_int        |
 *       l     | char*, size_t  | Arguments are string and length.
 *       n     | mrb_sym        |
 *       s     | char*          | Argument is NUL terminated string.
 *       t     | mrb_value      | Convert to type (class) of object.
 *      v,S    | mrb_value      |
 *       C     | struct RClass* |
 *       T     | mrb_value      | Convert to real type (class) of object.
 *       Y     | mrb_value      | Same as `!v` if argument is `true`, `false`
 *             |                | or `nil`, otherwise same as `T`.
 *       %     | -              | Convert to percent sign itself (no argument
 *             |                | taken).
 *   ----------+----------------+--------------------------------------------
 */
#[no_mangle]
pub unsafe extern "C" fn mrb_vformat(mut mrb: *mut mrb_state,
                                     mut format: *const libc::c_char,
                                     mut ap: ::std::ffi::VaList)
 -> mrb_value {
    let mut current_block: u64;
    let mut chars: *const libc::c_char = 0 as *const libc::c_char;
    let mut p: *const libc::c_char = format;
    let mut b: *const libc::c_char = format;
    let mut e: *const libc::c_char = 0 as *const libc::c_char;
    let mut ch: libc::c_char = 0;
    let mut len: size_t = 0;
    let mut i: mrb_int = 0;
    let mut cls: *mut RClass = 0 as *mut RClass;
    let mut inspect: mrb_bool = 0i32 as mrb_bool;
    let mut result: mrb_value = mrb_str_new_capa(mrb, 128i32 as size_t);
    let mut obj: mrb_value =
        mrb_value{value: C2RustUnnamed_0{f: 0.,}, tt: MRB_TT_FALSE,};
    let mut str: mrb_value =
        mrb_value{value: C2RustUnnamed_0{f: 0.,}, tt: MRB_TT_FALSE,};
    let mut ai: libc::c_int = mrb_gc_arena_save(mrb);
    while 0 != *p {
        let fresh0 = p;
        p = p.offset(1);
        let c: libc::c_char = *fresh0;
        e = p;
        if c as libc::c_int == '%' as i32 {
            if *p as libc::c_int == '!' as i32 {
                inspect = 1i32 as mrb_bool;
                p = p.offset(1isize)
            }
            if 0 == *p { break ; }
            match *p as libc::c_int {
                99 => {
                    current_block = 17541379661316378737;
                    match current_block {
                        7094039868041363793 => {
                            mrb_raisef(mrb,
                                       mrb_exc_get(mrb,
                                                   b"ArgumentError\x00" as
                                                       *const u8 as
                                                       *const libc::c_char),
                                       b"malformed format string - %%%c\x00"
                                           as *const u8 as
                                           *const libc::c_char,
                                       *p as libc::c_int);
                            continue ;
                        }
                        7200958511956080679 => {
                            chars =
                                ap.as_va_list().arg::<*mut libc::c_char>();
                            len = ap.as_va_list().arg::<size_t>();
                            current_block = 1606585337627918712;
                        }
                        17483139890018647811 => {
                            i =
                                if *p as libc::c_int == 'd' as i32 {
                                    ap.as_va_list().arg::<libc::c_int>() as
                                        mrb_int
                                } else { ap.as_va_list().arg::<mrb_int>() };
                            obj = mrb_fixnum_value(i);
                            current_block = 7216536457578139154;
                        }
                        17541379661316378737 => {
                            ch =
                                ap.as_va_list().arg::<libc::c_int>() as
                                    libc::c_char;
                            chars = &mut ch;
                            len = 1i32 as size_t;
                            current_block = 1606585337627918712;
                        }
                        1447766087013959760 => {
                            obj = ap.as_va_list().arg::<mrb_value>();
                            if !(obj.tt as libc::c_uint !=
                                     MRB_TT_FALSE as libc::c_int as
                                         libc::c_uint) ||
                                   obj.tt as libc::c_uint ==
                                       MRB_TT_TRUE as libc::c_int as
                                           libc::c_uint {
                                inspect = 1i32 as mrb_bool;
                                current_block = 7216536457578139154;
                            } else { current_block = 1774679666800629700; }
                        }
                        18022542853155082387 => {
                            obj =
                                mrb_float_value(mrb,
                                                ap.as_va_list().arg::<libc::c_double>());
                            current_block = 7216536457578139154;
                        }
                        14414354668014553667 => {
                            obj =
                                mrb_symbol_value(ap.as_va_list().arg::<mrb_sym>());
                            current_block = 7216536457578139154;
                        }
                        4187025628485394788 => {
                            cls =
                                mrb_class(mrb,
                                          ap.as_va_list().arg::<mrb_value>());
                            current_block = 16677287981175421660;
                        }
                        8801382743313005935 => {
                            obj = ap.as_va_list().arg::<mrb_value>();
                            current_block = 7216536457578139154;
                        }
                        3092744430171703683 => {
                            cls = ap.as_va_list().arg::<*mut RClass>();
                            current_block = 16677287981175421660;
                        }
                        5435597673222852903 => {
                            obj = ap.as_va_list().arg::<mrb_value>();
                            current_block = 1774679666800629700;
                        }
                        _ => {
                            chars =
                                ap.as_va_list().arg::<*mut libc::c_char>();
                            len = strlen(chars);
                            current_block = 1606585337627918712;
                        }
                    }
                    match current_block {
                        1606585337627918712 => { }
                        7216536457578139154 => { }
                        _ => {
                            match current_block {
                                1774679666800629700 => {
                                    cls = mrb_obj_class(mrb, obj)
                                }
                                _ => { }
                            }
                            obj = mrb_obj_value(cls as *mut libc::c_void);
                            current_block = 7216536457578139154;
                        }
                    }
                }
                100 | 105 => {
                    current_block = 17483139890018647811;
                    match current_block {
                        7094039868041363793 => {
                            mrb_raisef(mrb,
                                       mrb_exc_get(mrb,
                                                   b"ArgumentError\x00" as
                                                       *const u8 as
                                                       *const libc::c_char),
                                       b"malformed format string - %%%c\x00"
                                           as *const u8 as
                                           *const libc::c_char,
                                       *p as libc::c_int);
                            continue ;
                        }
                        7200958511956080679 => {
                            chars =
                                ap.as_va_list().arg::<*mut libc::c_char>();
                            len = ap.as_va_list().arg::<size_t>();
                            current_block = 1606585337627918712;
                        }
                        17483139890018647811 => {
                            i =
                                if *p as libc::c_int == 'd' as i32 {
                                    ap.as_va_list().arg::<libc::c_int>() as
                                        mrb_int
                                } else { ap.as_va_list().arg::<mrb_int>() };
                            obj = mrb_fixnum_value(i);
                            current_block = 7216536457578139154;
                        }
                        17541379661316378737 => {
                            ch =
                                ap.as_va_list().arg::<libc::c_int>() as
                                    libc::c_char;
                            chars = &mut ch;
                            len = 1i32 as size_t;
                            current_block = 1606585337627918712;
                        }
                        1447766087013959760 => {
                            obj = ap.as_va_list().arg::<mrb_value>();
                            if !(obj.tt as libc::c_uint !=
                                     MRB_TT_FALSE as libc::c_int as
                                         libc::c_uint) ||
                                   obj.tt as libc::c_uint ==
                                       MRB_TT_TRUE as libc::c_int as
                                           libc::c_uint {
                                inspect = 1i32 as mrb_bool;
                                current_block = 7216536457578139154;
                            } else { current_block = 1774679666800629700; }
                        }
                        18022542853155082387 => {
                            obj =
                                mrb_float_value(mrb,
                                                ap.as_va_list().arg::<libc::c_double>());
                            current_block = 7216536457578139154;
                        }
                        14414354668014553667 => {
                            obj =
                                mrb_symbol_value(ap.as_va_list().arg::<mrb_sym>());
                            current_block = 7216536457578139154;
                        }
                        4187025628485394788 => {
                            cls =
                                mrb_class(mrb,
                                          ap.as_va_list().arg::<mrb_value>());
                            current_block = 16677287981175421660;
                        }
                        8801382743313005935 => {
                            obj = ap.as_va_list().arg::<mrb_value>();
                            current_block = 7216536457578139154;
                        }
                        3092744430171703683 => {
                            cls = ap.as_va_list().arg::<*mut RClass>();
                            current_block = 16677287981175421660;
                        }
                        5435597673222852903 => {
                            obj = ap.as_va_list().arg::<mrb_value>();
                            current_block = 1774679666800629700;
                        }
                        _ => {
                            chars =
                                ap.as_va_list().arg::<*mut libc::c_char>();
                            len = strlen(chars);
                            current_block = 1606585337627918712;
                        }
                    }
                    match current_block {
                        1606585337627918712 => { }
                        7216536457578139154 => { }
                        _ => {
                            match current_block {
                                1774679666800629700 => {
                                    cls = mrb_obj_class(mrb, obj)
                                }
                                _ => { }
                            }
                            obj = mrb_obj_value(cls as *mut libc::c_void);
                            current_block = 7216536457578139154;
                        }
                    }
                }
                102 => {
                    current_block = 18022542853155082387;
                    match current_block {
                        7094039868041363793 => {
                            mrb_raisef(mrb,
                                       mrb_exc_get(mrb,
                                                   b"ArgumentError\x00" as
                                                       *const u8 as
                                                       *const libc::c_char),
                                       b"malformed format string - %%%c\x00"
                                           as *const u8 as
                                           *const libc::c_char,
                                       *p as libc::c_int);
                            continue ;
                        }
                        7200958511956080679 => {
                            chars =
                                ap.as_va_list().arg::<*mut libc::c_char>();
                            len = ap.as_va_list().arg::<size_t>();
                            current_block = 1606585337627918712;
                        }
                        17483139890018647811 => {
                            i =
                                if *p as libc::c_int == 'd' as i32 {
                                    ap.as_va_list().arg::<libc::c_int>() as
                                        mrb_int
                                } else { ap.as_va_list().arg::<mrb_int>() };
                            obj = mrb_fixnum_value(i);
                            current_block = 7216536457578139154;
                        }
                        17541379661316378737 => {
                            ch =
                                ap.as_va_list().arg::<libc::c_int>() as
                                    libc::c_char;
                            chars = &mut ch;
                            len = 1i32 as size_t;
                            current_block = 1606585337627918712;
                        }
                        1447766087013959760 => {
                            obj = ap.as_va_list().arg::<mrb_value>();
                            if !(obj.tt as libc::c_uint !=
                                     MRB_TT_FALSE as libc::c_int as
                                         libc::c_uint) ||
                                   obj.tt as libc::c_uint ==
                                       MRB_TT_TRUE as libc::c_int as
                                           libc::c_uint {
                                inspect = 1i32 as mrb_bool;
                                current_block = 7216536457578139154;
                            } else { current_block = 1774679666800629700; }
                        }
                        18022542853155082387 => {
                            obj =
                                mrb_float_value(mrb,
                                                ap.as_va_list().arg::<libc::c_double>());
                            current_block = 7216536457578139154;
                        }
                        14414354668014553667 => {
                            obj =
                                mrb_symbol_value(ap.as_va_list().arg::<mrb_sym>());
                            current_block = 7216536457578139154;
                        }
                        4187025628485394788 => {
                            cls =
                                mrb_class(mrb,
                                          ap.as_va_list().arg::<mrb_value>());
                            current_block = 16677287981175421660;
                        }
                        8801382743313005935 => {
                            obj = ap.as_va_list().arg::<mrb_value>();
                            current_block = 7216536457578139154;
                        }
                        3092744430171703683 => {
                            cls = ap.as_va_list().arg::<*mut RClass>();
                            current_block = 16677287981175421660;
                        }
                        5435597673222852903 => {
                            obj = ap.as_va_list().arg::<mrb_value>();
                            current_block = 1774679666800629700;
                        }
                        _ => {
                            chars =
                                ap.as_va_list().arg::<*mut libc::c_char>();
                            len = strlen(chars);
                            current_block = 1606585337627918712;
                        }
                    }
                    match current_block {
                        1606585337627918712 => { }
                        7216536457578139154 => { }
                        _ => {
                            match current_block {
                                1774679666800629700 => {
                                    cls = mrb_obj_class(mrb, obj)
                                }
                                _ => { }
                            }
                            obj = mrb_obj_value(cls as *mut libc::c_void);
                            current_block = 7216536457578139154;
                        }
                    }
                }
                108 => {
                    current_block = 7200958511956080679;
                    match current_block {
                        7094039868041363793 => {
                            mrb_raisef(mrb,
                                       mrb_exc_get(mrb,
                                                   b"ArgumentError\x00" as
                                                       *const u8 as
                                                       *const libc::c_char),
                                       b"malformed format string - %%%c\x00"
                                           as *const u8 as
                                           *const libc::c_char,
                                       *p as libc::c_int);
                            continue ;
                        }
                        7200958511956080679 => {
                            chars =
                                ap.as_va_list().arg::<*mut libc::c_char>();
                            len = ap.as_va_list().arg::<size_t>();
                            current_block = 1606585337627918712;
                        }
                        17483139890018647811 => {
                            i =
                                if *p as libc::c_int == 'd' as i32 {
                                    ap.as_va_list().arg::<libc::c_int>() as
                                        mrb_int
                                } else { ap.as_va_list().arg::<mrb_int>() };
                            obj = mrb_fixnum_value(i);
                            current_block = 7216536457578139154;
                        }
                        17541379661316378737 => {
                            ch =
                                ap.as_va_list().arg::<libc::c_int>() as
                                    libc::c_char;
                            chars = &mut ch;
                            len = 1i32 as size_t;
                            current_block = 1606585337627918712;
                        }
                        1447766087013959760 => {
                            obj = ap.as_va_list().arg::<mrb_value>();
                            if !(obj.tt as libc::c_uint !=
                                     MRB_TT_FALSE as libc::c_int as
                                         libc::c_uint) ||
                                   obj.tt as libc::c_uint ==
                                       MRB_TT_TRUE as libc::c_int as
                                           libc::c_uint {
                                inspect = 1i32 as mrb_bool;
                                current_block = 7216536457578139154;
                            } else { current_block = 1774679666800629700; }
                        }
                        18022542853155082387 => {
                            obj =
                                mrb_float_value(mrb,
                                                ap.as_va_list().arg::<libc::c_double>());
                            current_block = 7216536457578139154;
                        }
                        14414354668014553667 => {
                            obj =
                                mrb_symbol_value(ap.as_va_list().arg::<mrb_sym>());
                            current_block = 7216536457578139154;
                        }
                        4187025628485394788 => {
                            cls =
                                mrb_class(mrb,
                                          ap.as_va_list().arg::<mrb_value>());
                            current_block = 16677287981175421660;
                        }
                        8801382743313005935 => {
                            obj = ap.as_va_list().arg::<mrb_value>();
                            current_block = 7216536457578139154;
                        }
                        3092744430171703683 => {
                            cls = ap.as_va_list().arg::<*mut RClass>();
                            current_block = 16677287981175421660;
                        }
                        5435597673222852903 => {
                            obj = ap.as_va_list().arg::<mrb_value>();
                            current_block = 1774679666800629700;
                        }
                        _ => {
                            chars =
                                ap.as_va_list().arg::<*mut libc::c_char>();
                            len = strlen(chars);
                            current_block = 1606585337627918712;
                        }
                    }
                    match current_block {
                        1606585337627918712 => { }
                        7216536457578139154 => { }
                        _ => {
                            match current_block {
                                1774679666800629700 => {
                                    cls = mrb_obj_class(mrb, obj)
                                }
                                _ => { }
                            }
                            obj = mrb_obj_value(cls as *mut libc::c_void);
                            current_block = 7216536457578139154;
                        }
                    }
                }
                110 => {
                    current_block = 14414354668014553667;
                    match current_block {
                        7094039868041363793 => {
                            mrb_raisef(mrb,
                                       mrb_exc_get(mrb,
                                                   b"ArgumentError\x00" as
                                                       *const u8 as
                                                       *const libc::c_char),
                                       b"malformed format string - %%%c\x00"
                                           as *const u8 as
                                           *const libc::c_char,
                                       *p as libc::c_int);
                            continue ;
                        }
                        7200958511956080679 => {
                            chars =
                                ap.as_va_list().arg::<*mut libc::c_char>();
                            len = ap.as_va_list().arg::<size_t>();
                            current_block = 1606585337627918712;
                        }
                        17483139890018647811 => {
                            i =
                                if *p as libc::c_int == 'd' as i32 {
                                    ap.as_va_list().arg::<libc::c_int>() as
                                        mrb_int
                                } else { ap.as_va_list().arg::<mrb_int>() };
                            obj = mrb_fixnum_value(i);
                            current_block = 7216536457578139154;
                        }
                        17541379661316378737 => {
                            ch =
                                ap.as_va_list().arg::<libc::c_int>() as
                                    libc::c_char;
                            chars = &mut ch;
                            len = 1i32 as size_t;
                            current_block = 1606585337627918712;
                        }
                        1447766087013959760 => {
                            obj = ap.as_va_list().arg::<mrb_value>();
                            if !(obj.tt as libc::c_uint !=
                                     MRB_TT_FALSE as libc::c_int as
                                         libc::c_uint) ||
                                   obj.tt as libc::c_uint ==
                                       MRB_TT_TRUE as libc::c_int as
                                           libc::c_uint {
                                inspect = 1i32 as mrb_bool;
                                current_block = 7216536457578139154;
                            } else { current_block = 1774679666800629700; }
                        }
                        18022542853155082387 => {
                            obj =
                                mrb_float_value(mrb,
                                                ap.as_va_list().arg::<libc::c_double>());
                            current_block = 7216536457578139154;
                        }
                        14414354668014553667 => {
                            obj =
                                mrb_symbol_value(ap.as_va_list().arg::<mrb_sym>());
                            current_block = 7216536457578139154;
                        }
                        4187025628485394788 => {
                            cls =
                                mrb_class(mrb,
                                          ap.as_va_list().arg::<mrb_value>());
                            current_block = 16677287981175421660;
                        }
                        8801382743313005935 => {
                            obj = ap.as_va_list().arg::<mrb_value>();
                            current_block = 7216536457578139154;
                        }
                        3092744430171703683 => {
                            cls = ap.as_va_list().arg::<*mut RClass>();
                            current_block = 16677287981175421660;
                        }
                        5435597673222852903 => {
                            obj = ap.as_va_list().arg::<mrb_value>();
                            current_block = 1774679666800629700;
                        }
                        _ => {
                            chars =
                                ap.as_va_list().arg::<*mut libc::c_char>();
                            len = strlen(chars);
                            current_block = 1606585337627918712;
                        }
                    }
                    match current_block {
                        1606585337627918712 => { }
                        7216536457578139154 => { }
                        _ => {
                            match current_block {
                                1774679666800629700 => {
                                    cls = mrb_obj_class(mrb, obj)
                                }
                                _ => { }
                            }
                            obj = mrb_obj_value(cls as *mut libc::c_void);
                            current_block = 7216536457578139154;
                        }
                    }
                }
                115 => {
                    current_block = 11441496697127961056;
                    match current_block {
                        7094039868041363793 => {
                            mrb_raisef(mrb,
                                       mrb_exc_get(mrb,
                                                   b"ArgumentError\x00" as
                                                       *const u8 as
                                                       *const libc::c_char),
                                       b"malformed format string - %%%c\x00"
                                           as *const u8 as
                                           *const libc::c_char,
                                       *p as libc::c_int);
                            continue ;
                        }
                        7200958511956080679 => {
                            chars =
                                ap.as_va_list().arg::<*mut libc::c_char>();
                            len = ap.as_va_list().arg::<size_t>();
                            current_block = 1606585337627918712;
                        }
                        17483139890018647811 => {
                            i =
                                if *p as libc::c_int == 'd' as i32 {
                                    ap.as_va_list().arg::<libc::c_int>() as
                                        mrb_int
                                } else { ap.as_va_list().arg::<mrb_int>() };
                            obj = mrb_fixnum_value(i);
                            current_block = 7216536457578139154;
                        }
                        17541379661316378737 => {
                            ch =
                                ap.as_va_list().arg::<libc::c_int>() as
                                    libc::c_char;
                            chars = &mut ch;
                            len = 1i32 as size_t;
                            current_block = 1606585337627918712;
                        }
                        1447766087013959760 => {
                            obj = ap.as_va_list().arg::<mrb_value>();
                            if !(obj.tt as libc::c_uint !=
                                     MRB_TT_FALSE as libc::c_int as
                                         libc::c_uint) ||
                                   obj.tt as libc::c_uint ==
                                       MRB_TT_TRUE as libc::c_int as
                                           libc::c_uint {
                                inspect = 1i32 as mrb_bool;
                                current_block = 7216536457578139154;
                            } else { current_block = 1774679666800629700; }
                        }
                        18022542853155082387 => {
                            obj =
                                mrb_float_value(mrb,
                                                ap.as_va_list().arg::<libc::c_double>());
                            current_block = 7216536457578139154;
                        }
                        14414354668014553667 => {
                            obj =
                                mrb_symbol_value(ap.as_va_list().arg::<mrb_sym>());
                            current_block = 7216536457578139154;
                        }
                        4187025628485394788 => {
                            cls =
                                mrb_class(mrb,
                                          ap.as_va_list().arg::<mrb_value>());
                            current_block = 16677287981175421660;
                        }
                        8801382743313005935 => {
                            obj = ap.as_va_list().arg::<mrb_value>();
                            current_block = 7216536457578139154;
                        }
                        3092744430171703683 => {
                            cls = ap.as_va_list().arg::<*mut RClass>();
                            current_block = 16677287981175421660;
                        }
                        5435597673222852903 => {
                            obj = ap.as_va_list().arg::<mrb_value>();
                            current_block = 1774679666800629700;
                        }
                        _ => {
                            chars =
                                ap.as_va_list().arg::<*mut libc::c_char>();
                            len = strlen(chars);
                            current_block = 1606585337627918712;
                        }
                    }
                    match current_block {
                        1606585337627918712 => { }
                        7216536457578139154 => { }
                        _ => {
                            match current_block {
                                1774679666800629700 => {
                                    cls = mrb_obj_class(mrb, obj)
                                }
                                _ => { }
                            }
                            obj = mrb_obj_value(cls as *mut libc::c_void);
                            current_block = 7216536457578139154;
                        }
                    }
                }
                116 => {
                    current_block = 4187025628485394788;
                    match current_block {
                        7094039868041363793 => {
                            mrb_raisef(mrb,
                                       mrb_exc_get(mrb,
                                                   b"ArgumentError\x00" as
                                                       *const u8 as
                                                       *const libc::c_char),
                                       b"malformed format string - %%%c\x00"
                                           as *const u8 as
                                           *const libc::c_char,
                                       *p as libc::c_int);
                            continue ;
                        }
                        7200958511956080679 => {
                            chars =
                                ap.as_va_list().arg::<*mut libc::c_char>();
                            len = ap.as_va_list().arg::<size_t>();
                            current_block = 1606585337627918712;
                        }
                        17483139890018647811 => {
                            i =
                                if *p as libc::c_int == 'd' as i32 {
                                    ap.as_va_list().arg::<libc::c_int>() as
                                        mrb_int
                                } else { ap.as_va_list().arg::<mrb_int>() };
                            obj = mrb_fixnum_value(i);
                            current_block = 7216536457578139154;
                        }
                        17541379661316378737 => {
                            ch =
                                ap.as_va_list().arg::<libc::c_int>() as
                                    libc::c_char;
                            chars = &mut ch;
                            len = 1i32 as size_t;
                            current_block = 1606585337627918712;
                        }
                        1447766087013959760 => {
                            obj = ap.as_va_list().arg::<mrb_value>();
                            if !(obj.tt as libc::c_uint !=
                                     MRB_TT_FALSE as libc::c_int as
                                         libc::c_uint) ||
                                   obj.tt as libc::c_uint ==
                                       MRB_TT_TRUE as libc::c_int as
                                           libc::c_uint {
                                inspect = 1i32 as mrb_bool;
                                current_block = 7216536457578139154;
                            } else { current_block = 1774679666800629700; }
                        }
                        18022542853155082387 => {
                            obj =
                                mrb_float_value(mrb,
                                                ap.as_va_list().arg::<libc::c_double>());
                            current_block = 7216536457578139154;
                        }
                        14414354668014553667 => {
                            obj =
                                mrb_symbol_value(ap.as_va_list().arg::<mrb_sym>());
                            current_block = 7216536457578139154;
                        }
                        4187025628485394788 => {
                            cls =
                                mrb_class(mrb,
                                          ap.as_va_list().arg::<mrb_value>());
                            current_block = 16677287981175421660;
                        }
                        8801382743313005935 => {
                            obj = ap.as_va_list().arg::<mrb_value>();
                            current_block = 7216536457578139154;
                        }
                        3092744430171703683 => {
                            cls = ap.as_va_list().arg::<*mut RClass>();
                            current_block = 16677287981175421660;
                        }
                        5435597673222852903 => {
                            obj = ap.as_va_list().arg::<mrb_value>();
                            current_block = 1774679666800629700;
                        }
                        _ => {
                            chars =
                                ap.as_va_list().arg::<*mut libc::c_char>();
                            len = strlen(chars);
                            current_block = 1606585337627918712;
                        }
                    }
                    match current_block {
                        1606585337627918712 => { }
                        7216536457578139154 => { }
                        _ => {
                            match current_block {
                                1774679666800629700 => {
                                    cls = mrb_obj_class(mrb, obj)
                                }
                                _ => { }
                            }
                            obj = mrb_obj_value(cls as *mut libc::c_void);
                            current_block = 7216536457578139154;
                        }
                    }
                }
                118 | 83 => {
                    current_block = 8801382743313005935;
                    match current_block {
                        7094039868041363793 => {
                            mrb_raisef(mrb,
                                       mrb_exc_get(mrb,
                                                   b"ArgumentError\x00" as
                                                       *const u8 as
                                                       *const libc::c_char),
                                       b"malformed format string - %%%c\x00"
                                           as *const u8 as
                                           *const libc::c_char,
                                       *p as libc::c_int);
                            continue ;
                        }
                        7200958511956080679 => {
                            chars =
                                ap.as_va_list().arg::<*mut libc::c_char>();
                            len = ap.as_va_list().arg::<size_t>();
                            current_block = 1606585337627918712;
                        }
                        17483139890018647811 => {
                            i =
                                if *p as libc::c_int == 'd' as i32 {
                                    ap.as_va_list().arg::<libc::c_int>() as
                                        mrb_int
                                } else { ap.as_va_list().arg::<mrb_int>() };
                            obj = mrb_fixnum_value(i);
                            current_block = 7216536457578139154;
                        }
                        17541379661316378737 => {
                            ch =
                                ap.as_va_list().arg::<libc::c_int>() as
                                    libc::c_char;
                            chars = &mut ch;
                            len = 1i32 as size_t;
                            current_block = 1606585337627918712;
                        }
                        1447766087013959760 => {
                            obj = ap.as_va_list().arg::<mrb_value>();
                            if !(obj.tt as libc::c_uint !=
                                     MRB_TT_FALSE as libc::c_int as
                                         libc::c_uint) ||
                                   obj.tt as libc::c_uint ==
                                       MRB_TT_TRUE as libc::c_int as
                                           libc::c_uint {
                                inspect = 1i32 as mrb_bool;
                                current_block = 7216536457578139154;
                            } else { current_block = 1774679666800629700; }
                        }
                        18022542853155082387 => {
                            obj =
                                mrb_float_value(mrb,
                                                ap.as_va_list().arg::<libc::c_double>());
                            current_block = 7216536457578139154;
                        }
                        14414354668014553667 => {
                            obj =
                                mrb_symbol_value(ap.as_va_list().arg::<mrb_sym>());
                            current_block = 7216536457578139154;
                        }
                        4187025628485394788 => {
                            cls =
                                mrb_class(mrb,
                                          ap.as_va_list().arg::<mrb_value>());
                            current_block = 16677287981175421660;
                        }
                        8801382743313005935 => {
                            obj = ap.as_va_list().arg::<mrb_value>();
                            current_block = 7216536457578139154;
                        }
                        3092744430171703683 => {
                            cls = ap.as_va_list().arg::<*mut RClass>();
                            current_block = 16677287981175421660;
                        }
                        5435597673222852903 => {
                            obj = ap.as_va_list().arg::<mrb_value>();
                            current_block = 1774679666800629700;
                        }
                        _ => {
                            chars =
                                ap.as_va_list().arg::<*mut libc::c_char>();
                            len = strlen(chars);
                            current_block = 1606585337627918712;
                        }
                    }
                    match current_block {
                        1606585337627918712 => { }
                        7216536457578139154 => { }
                        _ => {
                            match current_block {
                                1774679666800629700 => {
                                    cls = mrb_obj_class(mrb, obj)
                                }
                                _ => { }
                            }
                            obj = mrb_obj_value(cls as *mut libc::c_void);
                            current_block = 7216536457578139154;
                        }
                    }
                }
                67 => {
                    current_block = 3092744430171703683;
                    match current_block {
                        7094039868041363793 => {
                            mrb_raisef(mrb,
                                       mrb_exc_get(mrb,
                                                   b"ArgumentError\x00" as
                                                       *const u8 as
                                                       *const libc::c_char),
                                       b"malformed format string - %%%c\x00"
                                           as *const u8 as
                                           *const libc::c_char,
                                       *p as libc::c_int);
                            continue ;
                        }
                        7200958511956080679 => {
                            chars =
                                ap.as_va_list().arg::<*mut libc::c_char>();
                            len = ap.as_va_list().arg::<size_t>();
                            current_block = 1606585337627918712;
                        }
                        17483139890018647811 => {
                            i =
                                if *p as libc::c_int == 'd' as i32 {
                                    ap.as_va_list().arg::<libc::c_int>() as
                                        mrb_int
                                } else { ap.as_va_list().arg::<mrb_int>() };
                            obj = mrb_fixnum_value(i);
                            current_block = 7216536457578139154;
                        }
                        17541379661316378737 => {
                            ch =
                                ap.as_va_list().arg::<libc::c_int>() as
                                    libc::c_char;
                            chars = &mut ch;
                            len = 1i32 as size_t;
                            current_block = 1606585337627918712;
                        }
                        1447766087013959760 => {
                            obj = ap.as_va_list().arg::<mrb_value>();
                            if !(obj.tt as libc::c_uint !=
                                     MRB_TT_FALSE as libc::c_int as
                                         libc::c_uint) ||
                                   obj.tt as libc::c_uint ==
                                       MRB_TT_TRUE as libc::c_int as
                                           libc::c_uint {
                                inspect = 1i32 as mrb_bool;
                                current_block = 7216536457578139154;
                            } else { current_block = 1774679666800629700; }
                        }
                        18022542853155082387 => {
                            obj =
                                mrb_float_value(mrb,
                                                ap.as_va_list().arg::<libc::c_double>());
                            current_block = 7216536457578139154;
                        }
                        14414354668014553667 => {
                            obj =
                                mrb_symbol_value(ap.as_va_list().arg::<mrb_sym>());
                            current_block = 7216536457578139154;
                        }
                        4187025628485394788 => {
                            cls =
                                mrb_class(mrb,
                                          ap.as_va_list().arg::<mrb_value>());
                            current_block = 16677287981175421660;
                        }
                        8801382743313005935 => {
                            obj = ap.as_va_list().arg::<mrb_value>();
                            current_block = 7216536457578139154;
                        }
                        3092744430171703683 => {
                            cls = ap.as_va_list().arg::<*mut RClass>();
                            current_block = 16677287981175421660;
                        }
                        5435597673222852903 => {
                            obj = ap.as_va_list().arg::<mrb_value>();
                            current_block = 1774679666800629700;
                        }
                        _ => {
                            chars =
                                ap.as_va_list().arg::<*mut libc::c_char>();
                            len = strlen(chars);
                            current_block = 1606585337627918712;
                        }
                    }
                    match current_block {
                        1606585337627918712 => { }
                        7216536457578139154 => { }
                        _ => {
                            match current_block {
                                1774679666800629700 => {
                                    cls = mrb_obj_class(mrb, obj)
                                }
                                _ => { }
                            }
                            obj = mrb_obj_value(cls as *mut libc::c_void);
                            current_block = 7216536457578139154;
                        }
                    }
                }
                84 => {
                    current_block = 5435597673222852903;
                    match current_block {
                        7094039868041363793 => {
                            mrb_raisef(mrb,
                                       mrb_exc_get(mrb,
                                                   b"ArgumentError\x00" as
                                                       *const u8 as
                                                       *const libc::c_char),
                                       b"malformed format string - %%%c\x00"
                                           as *const u8 as
                                           *const libc::c_char,
                                       *p as libc::c_int);
                            continue ;
                        }
                        7200958511956080679 => {
                            chars =
                                ap.as_va_list().arg::<*mut libc::c_char>();
                            len = ap.as_va_list().arg::<size_t>();
                            current_block = 1606585337627918712;
                        }
                        17483139890018647811 => {
                            i =
                                if *p as libc::c_int == 'd' as i32 {
                                    ap.as_va_list().arg::<libc::c_int>() as
                                        mrb_int
                                } else { ap.as_va_list().arg::<mrb_int>() };
                            obj = mrb_fixnum_value(i);
                            current_block = 7216536457578139154;
                        }
                        17541379661316378737 => {
                            ch =
                                ap.as_va_list().arg::<libc::c_int>() as
                                    libc::c_char;
                            chars = &mut ch;
                            len = 1i32 as size_t;
                            current_block = 1606585337627918712;
                        }
                        1447766087013959760 => {
                            obj = ap.as_va_list().arg::<mrb_value>();
                            if !(obj.tt as libc::c_uint !=
                                     MRB_TT_FALSE as libc::c_int as
                                         libc::c_uint) ||
                                   obj.tt as libc::c_uint ==
                                       MRB_TT_TRUE as libc::c_int as
                                           libc::c_uint {
                                inspect = 1i32 as mrb_bool;
                                current_block = 7216536457578139154;
                            } else { current_block = 1774679666800629700; }
                        }
                        18022542853155082387 => {
                            obj =
                                mrb_float_value(mrb,
                                                ap.as_va_list().arg::<libc::c_double>());
                            current_block = 7216536457578139154;
                        }
                        14414354668014553667 => {
                            obj =
                                mrb_symbol_value(ap.as_va_list().arg::<mrb_sym>());
                            current_block = 7216536457578139154;
                        }
                        4187025628485394788 => {
                            cls =
                                mrb_class(mrb,
                                          ap.as_va_list().arg::<mrb_value>());
                            current_block = 16677287981175421660;
                        }
                        8801382743313005935 => {
                            obj = ap.as_va_list().arg::<mrb_value>();
                            current_block = 7216536457578139154;
                        }
                        3092744430171703683 => {
                            cls = ap.as_va_list().arg::<*mut RClass>();
                            current_block = 16677287981175421660;
                        }
                        5435597673222852903 => {
                            obj = ap.as_va_list().arg::<mrb_value>();
                            current_block = 1774679666800629700;
                        }
                        _ => {
                            chars =
                                ap.as_va_list().arg::<*mut libc::c_char>();
                            len = strlen(chars);
                            current_block = 1606585337627918712;
                        }
                    }
                    match current_block {
                        1606585337627918712 => { }
                        7216536457578139154 => { }
                        _ => {
                            match current_block {
                                1774679666800629700 => {
                                    cls = mrb_obj_class(mrb, obj)
                                }
                                _ => { }
                            }
                            obj = mrb_obj_value(cls as *mut libc::c_void);
                            current_block = 7216536457578139154;
                        }
                    }
                }
                89 => {
                    current_block = 1447766087013959760;
                    match current_block {
                        7094039868041363793 => {
                            mrb_raisef(mrb,
                                       mrb_exc_get(mrb,
                                                   b"ArgumentError\x00" as
                                                       *const u8 as
                                                       *const libc::c_char),
                                       b"malformed format string - %%%c\x00"
                                           as *const u8 as
                                           *const libc::c_char,
                                       *p as libc::c_int);
                            continue ;
                        }
                        7200958511956080679 => {
                            chars =
                                ap.as_va_list().arg::<*mut libc::c_char>();
                            len = ap.as_va_list().arg::<size_t>();
                            current_block = 1606585337627918712;
                        }
                        17483139890018647811 => {
                            i =
                                if *p as libc::c_int == 'd' as i32 {
                                    ap.as_va_list().arg::<libc::c_int>() as
                                        mrb_int
                                } else { ap.as_va_list().arg::<mrb_int>() };
                            obj = mrb_fixnum_value(i);
                            current_block = 7216536457578139154;
                        }
                        17541379661316378737 => {
                            ch =
                                ap.as_va_list().arg::<libc::c_int>() as
                                    libc::c_char;
                            chars = &mut ch;
                            len = 1i32 as size_t;
                            current_block = 1606585337627918712;
                        }
                        1447766087013959760 => {
                            obj = ap.as_va_list().arg::<mrb_value>();
                            if !(obj.tt as libc::c_uint !=
                                     MRB_TT_FALSE as libc::c_int as
                                         libc::c_uint) ||
                                   obj.tt as libc::c_uint ==
                                       MRB_TT_TRUE as libc::c_int as
                                           libc::c_uint {
                                inspect = 1i32 as mrb_bool;
                                current_block = 7216536457578139154;
                            } else { current_block = 1774679666800629700; }
                        }
                        18022542853155082387 => {
                            obj =
                                mrb_float_value(mrb,
                                                ap.as_va_list().arg::<libc::c_double>());
                            current_block = 7216536457578139154;
                        }
                        14414354668014553667 => {
                            obj =
                                mrb_symbol_value(ap.as_va_list().arg::<mrb_sym>());
                            current_block = 7216536457578139154;
                        }
                        4187025628485394788 => {
                            cls =
                                mrb_class(mrb,
                                          ap.as_va_list().arg::<mrb_value>());
                            current_block = 16677287981175421660;
                        }
                        8801382743313005935 => {
                            obj = ap.as_va_list().arg::<mrb_value>();
                            current_block = 7216536457578139154;
                        }
                        3092744430171703683 => {
                            cls = ap.as_va_list().arg::<*mut RClass>();
                            current_block = 16677287981175421660;
                        }
                        5435597673222852903 => {
                            obj = ap.as_va_list().arg::<mrb_value>();
                            current_block = 1774679666800629700;
                        }
                        _ => {
                            chars =
                                ap.as_va_list().arg::<*mut libc::c_char>();
                            len = strlen(chars);
                            current_block = 1606585337627918712;
                        }
                    }
                    match current_block {
                        1606585337627918712 => { }
                        7216536457578139154 => { }
                        _ => {
                            match current_block {
                                1774679666800629700 => {
                                    cls = mrb_obj_class(mrb, obj)
                                }
                                _ => { }
                            }
                            obj = mrb_obj_value(cls as *mut libc::c_void);
                            current_block = 7216536457578139154;
                        }
                    }
                }
                37 => { current_block = 14066111679330324295; }
                _ => {
                    current_block = 7094039868041363793;
                    match current_block {
                        7094039868041363793 => {
                            mrb_raisef(mrb,
                                       mrb_exc_get(mrb,
                                                   b"ArgumentError\x00" as
                                                       *const u8 as
                                                       *const libc::c_char),
                                       b"malformed format string - %%%c\x00"
                                           as *const u8 as
                                           *const libc::c_char,
                                       *p as libc::c_int);
                            continue ;
                        }
                        7200958511956080679 => {
                            chars =
                                ap.as_va_list().arg::<*mut libc::c_char>();
                            len = ap.as_va_list().arg::<size_t>();
                            current_block = 1606585337627918712;
                        }
                        17483139890018647811 => {
                            i =
                                if *p as libc::c_int == 'd' as i32 {
                                    ap.as_va_list().arg::<libc::c_int>() as
                                        mrb_int
                                } else { ap.as_va_list().arg::<mrb_int>() };
                            obj = mrb_fixnum_value(i);
                            current_block = 7216536457578139154;
                        }
                        17541379661316378737 => {
                            ch =
                                ap.as_va_list().arg::<libc::c_int>() as
                                    libc::c_char;
                            chars = &mut ch;
                            len = 1i32 as size_t;
                            current_block = 1606585337627918712;
                        }
                        1447766087013959760 => {
                            obj = ap.as_va_list().arg::<mrb_value>();
                            if !(obj.tt as libc::c_uint !=
                                     MRB_TT_FALSE as libc::c_int as
                                         libc::c_uint) ||
                                   obj.tt as libc::c_uint ==
                                       MRB_TT_TRUE as libc::c_int as
                                           libc::c_uint {
                                inspect = 1i32 as mrb_bool;
                                current_block = 7216536457578139154;
                            } else { current_block = 1774679666800629700; }
                        }
                        18022542853155082387 => {
                            obj =
                                mrb_float_value(mrb,
                                                ap.as_va_list().arg::<libc::c_double>());
                            current_block = 7216536457578139154;
                        }
                        14414354668014553667 => {
                            obj =
                                mrb_symbol_value(ap.as_va_list().arg::<mrb_sym>());
                            current_block = 7216536457578139154;
                        }
                        4187025628485394788 => {
                            cls =
                                mrb_class(mrb,
                                          ap.as_va_list().arg::<mrb_value>());
                            current_block = 16677287981175421660;
                        }
                        8801382743313005935 => {
                            obj = ap.as_va_list().arg::<mrb_value>();
                            current_block = 7216536457578139154;
                        }
                        3092744430171703683 => {
                            cls = ap.as_va_list().arg::<*mut RClass>();
                            current_block = 16677287981175421660;
                        }
                        5435597673222852903 => {
                            obj = ap.as_va_list().arg::<mrb_value>();
                            current_block = 1774679666800629700;
                        }
                        _ => {
                            chars =
                                ap.as_va_list().arg::<*mut libc::c_char>();
                            len = strlen(chars);
                            current_block = 1606585337627918712;
                        }
                    }
                    match current_block {
                        1606585337627918712 => { }
                        7216536457578139154 => { }
                        _ => {
                            match current_block {
                                1774679666800629700 => {
                                    cls = mrb_obj_class(mrb, obj)
                                }
                                _ => { }
                            }
                            obj = mrb_obj_value(cls as *mut libc::c_void);
                            current_block = 7216536457578139154;
                        }
                    }
                }
            }
        } else {
            if !(c as libc::c_int == '\\' as i32) { continue ; }
            if 0 == *p { break ; }
            current_block = 14066111679330324295;
        }
        match current_block {
            14066111679330324295 => {
                chars = p;
                len = 1i32 as size_t;
                current_block = 1606585337627918712;
            }
            _ => { }
        }
        loop  {
            match current_block {
                1606585337627918712 => {
                    if !(0 != inspect) { break ; }
                    obj = mrb_str_new(mrb, chars, len);
                    current_block = 7216536457578139154;
                }
                _ => {
                    str =
                        if 0 != inspect as libc::c_int {
                            Some(mrb_inspect as
                                     unsafe extern "C" fn(_: *mut mrb_state,
                                                          _: mrb_value)
                                         -> mrb_value)
                        } else {
                            Some(mrb_obj_as_string as
                                     unsafe extern "C" fn(_: *mut mrb_state,
                                                          _: mrb_value)
                                         -> mrb_value)
                        }.expect("non-null function pointer")(mrb, obj);
                    chars =
                        if 0 !=
                               (*(str.value.p as *mut RString)).flags() as
                                   libc::c_int & 32i32 {
                            (*(str.value.p as
                                   *mut RString)).as_0.ary.as_mut_ptr()
                        } else {
                            (*(str.value.p as *mut RString)).as_0.heap.ptr
                        };
                    len =
                        (if 0 !=
                                (*(str.value.p as *mut RString)).flags() as
                                    libc::c_int & 32i32 {
                             (((*(str.value.p as *mut RString)).flags() as
                                   libc::c_int & 0x7c0i32) >> 6i32) as mrb_int
                         } else {
                             (*(str.value.p as *mut RString)).as_0.heap.len
                         }) as size_t;
                    inspect = 0i32 as mrb_bool;
                    current_block = 1606585337627918712;
                }
            }
        }
        mrb_str_cat(mrb, result, b,
                    (e.wrapping_offset_from(b) as libc::c_long -
                         1i32 as libc::c_long) as size_t);
        mrb_str_cat(mrb, result, chars, len);
        p = p.offset(1isize);
        b = p;
        mrb_gc_arena_restore(mrb, ai);
    }
    mrb_str_cat(mrb, result, b,
                p.wrapping_offset_from(b) as libc::c_long as size_t);
    return result;
}
#[no_mangle]
pub unsafe extern "C" fn mrb_format(mut mrb: *mut mrb_state,
                                    mut format: *const libc::c_char,
                                    mut ap: ...) -> mrb_value {
    let mut str: mrb_value =
        mrb_value{value: C2RustUnnamed_0{f: 0.,}, tt: MRB_TT_FALSE,};
    str = mrb_vformat(mrb, format, ap.as_va_list());
    return str;
}
unsafe extern "C" fn raise_va(mut mrb: *mut mrb_state, mut c: *mut RClass,
                              mut fmt: *const libc::c_char,
                              mut ap_0: ::std::ffi::VaList,
                              mut argc: libc::c_int,
                              mut argv: *mut mrb_value) {
    let mut mesg: mrb_value =
        mrb_value{value: C2RustUnnamed_0{f: 0.,}, tt: MRB_TT_FALSE,};
    mesg = mrb_vformat(mrb, fmt, ap_0.as_va_list());
    if argv.is_null() {
        argv = &mut mesg
    } else { *argv.offset(0isize) = mesg }
    mrb_exc_raise(mrb, mrb_obj_new(mrb, c, (argc + 1i32) as mrb_int, argv));
}
#[no_mangle]
pub unsafe extern "C" fn mrb_raisef(mut mrb: *mut mrb_state,
                                    mut c: *mut RClass,
                                    mut fmt: *const libc::c_char,
                                    mut args: ...) {
    raise_va(mrb, c, fmt, args.as_va_list(), 0i32, 0 as *mut mrb_value);
}
#[no_mangle]
pub unsafe extern "C" fn mrb_name_error(mut mrb: *mut mrb_state,
                                        mut id: mrb_sym,
                                        mut fmt: *const libc::c_char,
                                        mut args_0: ...) {
    let mut argv: [mrb_value; 2] =
        [mrb_value{value: C2RustUnnamed_0{f: 0.,}, tt: MRB_TT_FALSE,}; 2];
    argv[1usize] = mrb_symbol_value(id);
    raise_va(mrb,
             mrb_exc_get(mrb,
                         b"NameError\x00" as *const u8 as
                             *const libc::c_char), fmt, args_0.as_va_list(),
             1i32, argv.as_mut_ptr());
}
#[no_mangle]
pub unsafe extern "C" fn mrb_warn(mut mrb: *mut mrb_state,
                                  mut fmt: *const libc::c_char, _: ...) {
}
#[no_mangle]
pub unsafe extern "C" fn mrb_bug(mut mrb: *mut mrb_state,
                                 mut fmt: *const libc::c_char, _: ...) {
    exit(1i32);
}
#[no_mangle]
pub unsafe extern "C" fn mrb_make_exception(mut mrb: *mut mrb_state,
                                            mut argc: mrb_int,
                                            mut argv: *const mrb_value)
 -> mrb_value {
    let mut mesg: mrb_value =
        mrb_value{value: C2RustUnnamed_0{f: 0.,}, tt: MRB_TT_FALSE,};
    let mut n: libc::c_int = 0;
    mesg = mrb_nil_value();
    let mut current_block_10: u64;
    match argc {
        0 => { current_block_10 = 13242334135786603907; }
        1 => {
            if (*argv.offset(0isize)).tt as libc::c_uint ==
                   MRB_TT_FALSE as libc::c_int as libc::c_uint &&
                   0 == (*argv.offset(0isize)).value.i {
                current_block_10 = 13242334135786603907;
            } else if (*argv.offset(0isize)).tt as libc::c_uint ==
                          MRB_TT_STRING as libc::c_int as libc::c_uint {
                mesg =
                    mrb_exc_new_str(mrb,
                                    mrb_exc_get(mrb,
                                                b"RuntimeError\x00" as
                                                    *const u8 as
                                                    *const libc::c_char),
                                    *argv.offset(0isize));
                current_block_10 = 13242334135786603907;
            } else { n = 0i32; current_block_10 = 13366683415698734266; }
        }
        2 | 3 => { n = 1i32; current_block_10 = 13366683415698734266; }
        _ => {
            mrb_raisef(mrb,
                       mrb_exc_get(mrb,
                                   b"ArgumentError\x00" as *const u8 as
                                       *const libc::c_char),
                       b"wrong number of arguments (%i for 0..3)\x00" as
                           *const u8 as *const libc::c_char, argc);
            current_block_10 = 13242334135786603907;
        }
    }
    match current_block_10 {
        13366683415698734266 => {
            let mut exc: mrb_sym =
                mrb_intern_static(mrb,
                                  b"exception\x00" as *const u8 as
                                      *const libc::c_char,
                                  (::std::mem::size_of::<[libc::c_char; 10]>()
                                       as
                                       libc::c_ulong).wrapping_sub(1i32 as
                                                                       libc::c_ulong));
            if 0 != mrb_respond_to(mrb, *argv.offset(0isize), exc) {
                mesg =
                    mrb_funcall_argv(mrb, *argv.offset(0isize), exc,
                                     n as mrb_int, argv.offset(1isize))
            } else {
                /* undef */
                mrb_raise(mrb,
                          mrb_exc_get(mrb,
                                      b"TypeError\x00" as *const u8 as
                                          *const libc::c_char),
                          b"exception class/object expected\x00" as *const u8
                              as *const libc::c_char);
            }
        }
        _ => { }
    }
    if argc > 0i32 as libc::c_longlong {
        if 0 == mrb_obj_is_kind_of(mrb, mesg, (*mrb).eException_class) {
            mrb_raise(mrb, (*mrb).eException_class,
                      b"exception object expected\x00" as *const u8 as
                          *const libc::c_char);
        }
        if argc > 2i32 as libc::c_longlong {
            set_backtrace(mrb, mesg, *argv.offset(2isize));
        }
    }
    return mesg;
}
#[no_mangle]
pub unsafe extern "C" fn mrb_sys_fail(mut mrb: *mut mrb_state,
                                      mut mesg: *const libc::c_char) {
    let mut sce: *mut RClass = 0 as *mut RClass;
    let mut no: mrb_int = 0;
    no = *__error() as mrb_int;
    if 0 !=
           mrb_class_defined(mrb,
                             b"SystemCallError\x00" as *const u8 as
                                 *const libc::c_char) {
        sce =
            mrb_class_get(mrb,
                          b"SystemCallError\x00" as *const u8 as
                              *const libc::c_char);
        if !mesg.is_null() {
            mrb_funcall(mrb, mrb_obj_value(sce as *mut libc::c_void),
                        b"_sys_fail\x00" as *const u8 as *const libc::c_char,
                        2i32 as mrb_int, mrb_fixnum_value(no),
                        mrb_str_new_cstr(mrb, mesg));
        } else {
            mrb_funcall(mrb, mrb_obj_value(sce as *mut libc::c_void),
                        b"_sys_fail\x00" as *const u8 as *const libc::c_char,
                        1i32 as mrb_int, mrb_fixnum_value(no));
        }
    } else {
        mrb_raise(mrb,
                  mrb_exc_get(mrb,
                              b"RuntimeError\x00" as *const u8 as
                                  *const libc::c_char), mesg);
    };
}
#[no_mangle]
pub unsafe extern "C" fn mrb_no_method_error(mut mrb: *mut mrb_state,
                                             mut id: mrb_sym,
                                             mut args_1: mrb_value,
                                             mut fmt: *const libc::c_char,
                                             mut ap_0: ...) {
    let mut exc: mrb_value =
        mrb_value{value: C2RustUnnamed_0{f: 0.,}, tt: MRB_TT_FALSE,};
    let mut argv: [mrb_value; 3] =
        [mrb_value{value: C2RustUnnamed_0{f: 0.,}, tt: MRB_TT_FALSE,}; 3];
    argv[0usize] = mrb_vformat(mrb, fmt, ap_0.as_va_list());
    argv[1usize] = mrb_symbol_value(id);
    argv[2usize] = args_1;
    exc =
        mrb_obj_new(mrb,
                    mrb_exc_get(mrb,
                                b"NoMethodError\x00" as *const u8 as
                                    *const libc::c_char), 3i32 as mrb_int,
                    argv.as_mut_ptr());
    mrb_exc_raise(mrb, exc);
}
#[no_mangle]
pub unsafe extern "C" fn mrb_frozen_error(mut mrb: *mut mrb_state,
                                          mut frozen_obj: *mut libc::c_void) {
    mrb_raisef(mrb,
               mrb_exc_get(mrb,
                           b"FrozenError\x00" as *const u8 as
                               *const libc::c_char),
               b"can\'t modify frozen %t\x00" as *const u8 as
                   *const libc::c_char, mrb_obj_value(frozen_obj));
}
#[no_mangle]
pub unsafe extern "C" fn mrb_init_exception(mut mrb: *mut mrb_state) {
    let mut exception: *mut RClass = 0 as *mut RClass;
    let mut script_error: *mut RClass = 0 as *mut RClass;
    let mut stack_error: *mut RClass = 0 as *mut RClass;
    let mut nomem_error: *mut RClass = 0 as *mut RClass;
    /* 15.2.22 */
    exception =
        mrb_define_class(mrb,
                         b"Exception\x00" as *const u8 as *const libc::c_char,
                         (*mrb).object_class);
    (*mrb).eException_class = exception;
    (*exception).set_flags(((*exception).flags() as libc::c_int & !0xffi32 |
                                MRB_TT_EXCEPTION as libc::c_int as
                                    libc::c_char as libc::c_int) as uint32_t);
    mrb_define_class_method(mrb, exception,
                            b"exception\x00" as *const u8 as
                                *const libc::c_char,
                            Some(mrb_instance_new as
                                     unsafe extern "C" fn(_: *mut mrb_state,
                                                          _: mrb_value)
                                         -> mrb_value),
                            (1i32 << 12i32) as mrb_aspec);
    mrb_define_method(mrb, exception,
                      b"exception\x00" as *const u8 as *const libc::c_char,
                      Some(exc_exception as
                               unsafe extern "C" fn(_: *mut mrb_state,
                                                    _: mrb_value)
                                   -> mrb_value),
                      (1i32 << 12i32) as mrb_aspec);
    mrb_define_method(mrb, exception,
                      b"initialize\x00" as *const u8 as *const libc::c_char,
                      Some(exc_initialize as
                               unsafe extern "C" fn(_: *mut mrb_state,
                                                    _: mrb_value)
                                   -> mrb_value),
                      (1i32 << 12i32) as mrb_aspec);
    mrb_define_method(mrb, exception,
                      b"to_s\x00" as *const u8 as *const libc::c_char,
                      Some(exc_to_s as
                               unsafe extern "C" fn(_: *mut mrb_state,
                                                    _: mrb_value)
                                   -> mrb_value), 0i32 as mrb_aspec);
    mrb_define_method(mrb, exception,
                      b"message\x00" as *const u8 as *const libc::c_char,
                      Some(exc_message as
                               unsafe extern "C" fn(_: *mut mrb_state,
                                                    _: mrb_value)
                                   -> mrb_value), 0i32 as mrb_aspec);
    mrb_define_method(mrb, exception,
                      b"inspect\x00" as *const u8 as *const libc::c_char,
                      Some(exc_inspect as
                               unsafe extern "C" fn(_: *mut mrb_state,
                                                    _: mrb_value)
                                   -> mrb_value), 0i32 as mrb_aspec);
    mrb_define_method(mrb, exception,
                      b"backtrace\x00" as *const u8 as *const libc::c_char,
                      Some(mrb_exc_backtrace as
                               unsafe extern "C" fn(_: *mut mrb_state,
                                                    _: mrb_value)
                                   -> mrb_value), 0i32 as mrb_aspec);
    mrb_define_method(mrb, exception,
                      b"set_backtrace\x00" as *const u8 as
                          *const libc::c_char,
                      Some(exc_set_backtrace as
                               unsafe extern "C" fn(_: *mut mrb_state,
                                                    _: mrb_value)
                                   -> mrb_value),
                      ((1i32 & 0x1fi32) as mrb_aspec) << 18i32);
    /* 15.2.23 */
    (*mrb).eStandardError_class =
        mrb_define_class(mrb,
                         b"StandardError\x00" as *const u8 as
                             *const libc::c_char, (*mrb).eException_class);
    /* 15.2.28 */
    mrb_define_class(mrb,
                     b"RuntimeError\x00" as *const u8 as *const libc::c_char,
                     (*mrb).eStandardError_class);
    /* 15.2.37 */
    script_error =
        mrb_define_class(mrb,
                         b"ScriptError\x00" as *const u8 as
                             *const libc::c_char, (*mrb).eException_class);
    /* 15.2.38 */
    mrb_define_class(mrb,
                     b"SyntaxError\x00" as *const u8 as *const libc::c_char,
                     script_error);
    stack_error =
        mrb_define_class(mrb,
                         b"SystemStackError\x00" as *const u8 as
                             *const libc::c_char, exception);
    (*mrb).stack_err =
        mrb_exc_new_str(mrb, stack_error,
                        mrb_str_new_static(mrb,
                                           b"stack level too deep\x00" as
                                               *const u8 as
                                               *const libc::c_char,
                                           (::std::mem::size_of::<[libc::c_char; 21]>()
                                                as
                                                libc::c_ulong).wrapping_sub(1i32
                                                                                as
                                                                                libc::c_ulong))).value.p
            as *mut RObject;
    nomem_error =
        mrb_define_class(mrb,
                         b"NoMemoryError\x00" as *const u8 as
                             *const libc::c_char, exception);
    (*mrb).nomem_err =
        mrb_exc_new_str(mrb, nomem_error,
                        mrb_str_new_static(mrb,
                                           b"Out of memory\x00" as *const u8
                                               as *const libc::c_char,
                                           (::std::mem::size_of::<[libc::c_char; 14]>()
                                                as
                                                libc::c_ulong).wrapping_sub(1i32
                                                                                as
                                                                                libc::c_ulong))).value.p
            as *mut RObject;
}