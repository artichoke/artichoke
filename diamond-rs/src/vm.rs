use libc;
use c2rust_bitfields::BitfieldStruct;
extern "C" {
    pub type iv_tbl;
    /* debug info */
    pub type mrb_irep_debug_info;
    pub type symbol_name;
    #[no_mangle]
    fn mrb_singleton_class(_: *mut mrb_state, _: mrb_value) -> mrb_value;
    #[no_mangle]
    fn mrb_obj_eq(_: *mut mrb_state, _: mrb_value, _: mrb_value) -> mrb_bool;
    #[no_mangle]
    fn mrb_obj_alloc(_: *mut mrb_state, _: mrb_vtype, _: *mut RClass)
     -> *mut RBasic;
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
    fn mrb_intern_str(_: *mut mrb_state, _: mrb_value) -> mrb_sym;
    #[no_mangle]
    fn mrb_calloc(_: *mut mrb_state, _: size_t, _: size_t)
     -> *mut libc::c_void;
    #[no_mangle]
    fn mrb_realloc(_: *mut mrb_state, _: *mut libc::c_void, _: size_t)
     -> *mut libc::c_void;
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
    fn mrb_intern_static(_: *mut mrb_state, _: *const libc::c_char, _: size_t)
     -> mrb_sym;
    #[no_mangle]
    fn mrb_str_new_static(mrb: *mut mrb_state, p: *const libc::c_char,
                          len: size_t) -> mrb_value;
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
    fn mrb_undef_method_id(_: *mut mrb_state, _: *mut RClass, _: mrb_sym);
    #[no_mangle]
    fn mrb_malloc(_: *mut mrb_state, _: size_t) -> *mut libc::c_void;
    #[no_mangle]
    fn mrb_field_write_barrier(_: *mut mrb_state, _: *mut RBasic,
                               _: *mut RBasic);
    #[no_mangle]
    fn mrb_write_barrier(_: *mut mrb_state, _: *mut RBasic);
    #[no_mangle]
    fn mrb_convert_type(mrb: *mut mrb_state, val: mrb_value,
                        type_0: mrb_vtype, tname: *const libc::c_char,
                        method: *const libc::c_char) -> mrb_value;
    #[no_mangle]
    fn mrb_obj_is_kind_of(mrb: *mut mrb_state, obj: mrb_value, c: *mut RClass)
     -> mrb_bool;
    #[no_mangle]
    fn mrb_exc_raise(mrb: *mut mrb_state, exc: mrb_value);
    #[no_mangle]
    fn mrb_raise(mrb: *mut mrb_state, c: *mut RClass,
                 msg: *const libc::c_char);
    #[no_mangle]
    fn mrb_raisef(mrb: *mut mrb_state, c: *mut RClass,
                  fmt: *const libc::c_char, _: ...);
    /* mrb_gc_protect() leaves the object in the arena */
    #[no_mangle]
    fn mrb_gc_protect(mrb: *mut mrb_state, obj: mrb_value);
    #[no_mangle]
    fn mrb_format(mrb: *mut mrb_state, format: *const libc::c_char, _: ...)
     -> mrb_value;
    #[no_mangle]
    fn mrb_ary_new_capa(_: *mut mrb_state, _: mrb_int) -> mrb_value;
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
    /*
 * Concatenate two arrays. The target array will be modified
 *
 * Equivalent to:
 *      ary.concat(other)
 *
 * @param mrb The mruby state reference.
 * @param self The target array.
 * @param other The array that will be concatenated to self.
 */
    #[no_mangle]
    fn mrb_ary_concat(mrb: *mut mrb_state, self_0: mrb_value,
                      other: mrb_value);
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
    /*
 * Returns a reference to an element of the array on the given index.
 *
 * Equivalent to:
 *
 *      ary[n]
 *
 * @param mrb The mruby state reference.
 * @param ary The target array.
 * @param n The array index being referenced
 * @return The referenced value.
 */
    #[no_mangle]
    fn mrb_ary_ref(mrb: *mut mrb_state, ary: mrb_value, n: mrb_int)
     -> mrb_value;
    /*
 * Sets a value on an array at the given index
 *
 * Equivalent to:
 *
 *      ary[n] = val
 *
 * @param mrb The mruby state reference.
 * @param ary The target array.
 * @param n The array index being referenced.
 * @param val The value being setted.
 */
    #[no_mangle]
    fn mrb_ary_set(mrb: *mut mrb_state, ary: mrb_value, n: mrb_int,
                   val: mrb_value);
    /*
 * Unshift an element into the array
 *
 * Equivalent to:
 *
 *     ary.unshift(item)
 *
 * @param mrb The mruby state reference.
 * @param self The target array.
 * @param item The item to unshift.
 */
    #[no_mangle]
    fn mrb_ary_unshift(mrb: *mut mrb_state, self_0: mrb_value,
                       item: mrb_value) -> mrb_value;
    /*
 * Shifts the first element from the array.
 *
 * Equivalent to:
 *
 *      ary.shift
 *
 * @param mrb The mruby state reference.
 * @param self The array from which the value will be shifted.
 * @return The shifted value.
 */
    #[no_mangle]
    fn mrb_ary_shift(mrb: *mut mrb_state, self_0: mrb_value) -> mrb_value;
    #[no_mangle]
    fn mrb_vm_define_class(_: *mut mrb_state, _: mrb_value, _: mrb_value,
                           _: mrb_sym) -> *mut RClass;
    #[no_mangle]
    fn mrb_vm_define_module(_: *mut mrb_state, _: mrb_value, _: mrb_sym)
     -> *mut RClass;
    #[no_mangle]
    fn mrb_define_method_raw(_: *mut mrb_state, _: *mut RClass, _: mrb_sym,
                             _: mrb_method_t);
    #[no_mangle]
    fn mrb_alias_method(_: *mut mrb_state, c: *mut RClass, a: mrb_sym,
                        b: mrb_sym);
    #[no_mangle]
    fn mrb_method_search_vm(_: *mut mrb_state, _: *mut *mut RClass,
                            _: mrb_sym) -> mrb_method_t;
    #[no_mangle]
    fn mrb_hash_new_capa(_: *mut mrb_state, _: mrb_int) -> mrb_value;
    #[no_mangle]
    fn mrb_ensure_hash_type(mrb: *mut mrb_state, hash: mrb_value)
     -> mrb_value;
    /*
 * Initializes a new hash.
 *
 * Equivalent to:
 *
 *      Hash.new
 *
 * @param mrb The mruby state reference.
 * @return The initialized hash.
 */
    #[no_mangle]
    fn mrb_hash_new(mrb: *mut mrb_state) -> mrb_value;
    /*
 * Sets a keys and values to hashes.
 *
 * Equivalent to:
 *
 *      hash[key] = val
 *
 * @param mrb The mruby state reference.
 * @param hash The target hash.
 * @param key The key to set.
 * @param val The value to set.
 * @return The value.
 */
    #[no_mangle]
    fn mrb_hash_set(mrb: *mut mrb_state, hash: mrb_value, key: mrb_value,
                    val: mrb_value);
    /*
 * Gets a value from a key. If the key is not found, the default of the
 * hash is used.
 *
 * Equivalent to:
 *
 *     hash[key]
 *
 * @param mrb The mruby state reference.
 * @param hash The target hash.
 * @param key The key to get.
 * @return The found value.
 */
    #[no_mangle]
    fn mrb_hash_get(mrb: *mut mrb_state, hash: mrb_value, key: mrb_value)
     -> mrb_value;
    /*
 * Deletes hash key and value pair.
 *
 * Equivalent to:
 *
 *     hash.delete(key)
 *
 * @param mrb The mruby state reference.
 * @param hash The target hash.
 * @param key The key to delete.
 * @return The deleted value.
 */
    #[no_mangle]
    fn mrb_hash_delete_key(mrb: *mut mrb_state, hash: mrb_value,
                           key: mrb_value) -> mrb_value;
    /*
 * Gets an array of keys.
 *
 * Equivalent to:
 *
 *     hash.keys
 *
 * @param mrb The mruby state reference.
 * @param hash The target hash.
 * @return An array with the keys of the hash.
 */
    #[no_mangle]
    fn mrb_hash_keys(mrb: *mut mrb_state, hash: mrb_value) -> mrb_value;
    /*
 * Check if the hash has the key.
 *
 * Equivalent to:
 *
 *     hash.key?(key)
 *
 * @param mrb The mruby state reference.
 * @param hash The target hash.
 * @param key The key to check existence.
 * @return True if the hash has the key
 */
    #[no_mangle]
    fn mrb_hash_key_p(mrb: *mut mrb_state, hash: mrb_value, key: mrb_value)
     -> mrb_bool;
    /*
 * Check if the hash is empty
 *
 * Equivalent to:
 *
 *     hash.empty?
 *
 * @param mrb The mruby state reference.
 * @param self The target hash.
 * @return True if the hash is empty, false otherwise.
 */
    #[no_mangle]
    fn mrb_hash_empty_p(mrb: *mut mrb_state, self_0: mrb_value) -> mrb_bool;
    /*
 * Copies the hash.
 *
 *
 * @param mrb The mruby state reference.
 * @param hash The target hash.
 * @return The copy of the hash
 */
    #[no_mangle]
    fn mrb_hash_dup(mrb: *mut mrb_state, hash: mrb_value) -> mrb_value;
    /*
 * Merges two hashes. The first hash will be modified by the
 * second hash.
 *
 * @param mrb The mruby state reference.
 * @param hash1 The target hash.
 * @param hash2 Updating hash
 */
    #[no_mangle]
    fn mrb_hash_merge(mrb: *mut mrb_state, hash1: mrb_value,
                      hash2: mrb_value);
    #[no_mangle]
    fn mrb_proc_new(_: *mut mrb_state, _: *mut mrb_irep) -> *mut RProc;
    #[no_mangle]
    fn mrb_closure_new(_: *mut mrb_state, _: *mut mrb_irep) -> *mut RProc;
    /*
 * Initializes a Range.
 *
 * If the third parameter is FALSE then it includes the last value in the range.
 * If the third parameter is TRUE then it excludes the last value in the range.
 *
 * @param start the beginning value.
 * @param end the ending value.
 * @param exclude represents the inclusion or exclusion of the last value.
 */
    #[no_mangle]
    fn mrb_range_new(mrb: *mut mrb_state, start: mrb_value, end: mrb_value,
                     exclude: mrb_bool) -> mrb_value;
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
 * Adds two strings together.
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
 *       mrb_value a;
 *       mrb_value b;
 *       mrb_value c;
 *
 *       mrb_state *mrb = mrb_open();
 *       if (!mrb)
 *       {
 *          // handle error
 *       }
 *
 *       // Creates two Ruby strings from the passed in C strings.
 *       a = mrb_str_new_lit(mrb, "abc");
 *       b = mrb_str_new_lit(mrb, "def");
 *
 *       // Prints both C strings.
 *       mrb_p(mrb, a);
 *       mrb_p(mrb, b);
 *
 *       // Concatenates both Ruby strings.
 *       c = mrb_str_plus(mrb, a, b);
 *
 *      // Prints new Concatenated Ruby string.
 *      mrb_p(mrb, c);
 *
 *      mrb_close(mrb);
 *      return 0;
 *    }
 *
 *
 *  Result:
 *
 *     => "abc"  # First string
 *     => "def"  # Second string
 *     => "abcdef" # First & Second concatenated.
 *
 * @param [mrb_state] mrb The current mruby state.
 * @param [mrb_value] a First string to concatenate.
 * @param [mrb_value] b Second string to concatenate.
 * @return [mrb_value] Returns a new String containing a concatenated to b.
 */
    #[no_mangle]
    fn mrb_str_plus(_: *mut mrb_state, _: mrb_value, _: mrb_value)
     -> mrb_value;
    #[no_mangle]
    fn mrb_str_new_capa(mrb: *mut mrb_state, capa: size_t) -> mrb_value;
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
    #[no_mangle]
    fn mrb_vm_special_get(_: *mut mrb_state, _: mrb_sym) -> mrb_value;
    #[no_mangle]
    fn mrb_vm_special_set(_: *mut mrb_state, _: mrb_sym, _: mrb_value);
    #[no_mangle]
    fn mrb_vm_cv_get(_: *mut mrb_state, _: mrb_sym) -> mrb_value;
    #[no_mangle]
    fn mrb_vm_cv_set(_: *mut mrb_state, _: mrb_sym, _: mrb_value);
    #[no_mangle]
    fn mrb_vm_const_get(_: *mut mrb_state, _: mrb_sym) -> mrb_value;
    #[no_mangle]
    fn mrb_vm_const_set(_: *mut mrb_state, _: mrb_sym, _: mrb_value);
    #[no_mangle]
    fn mrb_const_get(_: *mut mrb_state, _: mrb_value, _: mrb_sym)
     -> mrb_value;
    #[no_mangle]
    fn mrb_const_set(_: *mut mrb_state, _: mrb_value, _: mrb_sym,
                     _: mrb_value);
    #[no_mangle]
    fn mrb_iv_get(mrb: *mut mrb_state, obj: mrb_value, sym: mrb_sym)
     -> mrb_value;
    #[no_mangle]
    fn mrb_iv_set(mrb: *mut mrb_state, obj: mrb_value, sym: mrb_sym,
                  v: mrb_value);
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
    fn mrb_gv_get(mrb: *mut mrb_state, sym: mrb_sym) -> mrb_value;
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
    fn mrb_gv_set(mrb: *mut mrb_state, sym: mrb_sym, val: mrb_value);
    #[no_mangle]
    fn mrb_exc_new_str(mrb: *mut mrb_state, c: *mut RClass, str: mrb_value)
     -> mrb_value;
    #[no_mangle]
    fn _longjmp(_: *mut libc::c_int, _: libc::c_int) -> !;
    #[no_mangle]
    fn _setjmp(_: *mut libc::c_int) -> libc::c_int;
    /*
** vm.c - virtual machine for mruby
**
** See Copyright Notice in mruby.h
*/
    #[no_mangle]
    fn abort() -> !;
    #[no_mangle]
    fn mrb_method_missing(mrb: *mut mrb_state, name: mrb_sym,
                          self_0: mrb_value, args: mrb_value);
    #[no_mangle]
    fn mrb_exc_set(mrb: *mut mrb_state, exc: mrb_value);
    #[no_mangle]
    fn mrb_hash_check_kdict(mrb: *mut mrb_state, self_0: mrb_value);
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
pub type __darwin_ptrdiff_t = libc::c_long;
pub type __darwin_size_t = libc::c_ulong;
pub type int64_t = libc::c_longlong;
pub type ptrdiff_t = __darwin_ptrdiff_t;
pub type size_t = __darwin_size_t;
pub type va_list = __builtin_va_list;
pub type uint8_t = libc::c_uchar;
pub type uint16_t = libc::c_ushort;
pub type uint32_t = libc::c_uint;
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
#[derive ( Copy , Clone )]
#[repr(C)]
pub struct C2RustUnnamed_3 {
    pub len: mrb_int,
    pub aux: C2RustUnnamed_4,
    pub ptr: *mut mrb_value,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub union C2RustUnnamed_4 {
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
pub union C2RustUnnamed_5 {
    pub heap: C2RustUnnamed_3,
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
    pub as_0: C2RustUnnamed_5,
}
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
pub type localjump_error_kind = libc::c_uint;
pub const LOCALJUMP_ERROR_YIELD: localjump_error_kind = 2;
pub const LOCALJUMP_ERROR_BREAK: localjump_error_kind = 1;
pub const LOCALJUMP_ERROR_RETURN: localjump_error_kind = 0;
/* R(a) = block (16=m5:r1:m5:d1:lv4) */
pub const OP_BLKPUSH: mrb_insn = 58;
#[derive ( BitfieldStruct , Clone , Copy )]
#[repr(C)]
pub struct RBreak {
    #[bitfield(name = "tt", ty = "mrb_vtype", bits = "0..=7")]
    #[bitfield(name = "color", ty = "uint32_t", bits = "8..=10")]
    #[bitfield(name = "flags", ty = "uint32_t", bits = "11..=31")]
    pub tt_color_flags: [u8; 4],
    #[bitfield(padding)]
    pub _pad: [u8; 4],
    pub c: *mut RClass,
    pub gcnext: *mut RBasic,
    pub proc_0: *mut RProc,
    pub val: mrb_value,
}
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
/* R(a) = mrb_int(0) */
pub const OP_LOADI_0: mrb_insn = 6;
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
/*
** mruby/opcode.h - RiteVM operation codes
**
** See Copyright Notice in mruby.h
*/
pub type mrb_insn = libc::c_uint;
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
unsafe extern "C" fn mrb_bool_value(mut boolean: mrb_bool) -> mrb_value {
    let mut v: mrb_value =
        mrb_value{value: C2RustUnnamed_0{f: 0.,}, tt: MRB_TT_FALSE,};
    v.tt =
        (if 0 != boolean as libc::c_int {
             MRB_TT_TRUE as libc::c_int
         } else { MRB_TT_FALSE as libc::c_int }) as mrb_vtype;
    v.value.i = 1i32 as mrb_int;
    return v;
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
unsafe extern "C" fn value_move(mut s1: *mut mrb_value,
                                mut s2: *const mrb_value, mut n: size_t) {
    if s1 > s2 as *mut mrb_value &&
           s1 < s2.offset(n as isize) as *mut mrb_value {
        s1 = s1.offset(n as isize);
        s2 = s2.offset(n as isize);
        loop  {
            let fresh6 = n;
            n = n.wrapping_sub(1);
            if !(fresh6 > 0i32 as libc::c_ulong) { break ; }
            s1 = s1.offset(-1isize);
            s2 = s2.offset(-1isize);
            *s1 = *s2
        }
    } else if s1 != s2 as *mut mrb_value {
        loop  {
            let fresh7 = n;
            n = n.wrapping_sub(1);
            if !(fresh7 > 0i32 as libc::c_ulong) { break ; }
            let fresh9 = s1;
            s1 = s1.offset(1);
            let fresh8 = s2;
            s2 = s2.offset(1);
            *fresh9 = *fresh8
        }
    };
}
unsafe extern "C" fn mrb_gc_arena_shrink(mut mrb: *mut mrb_state,
                                         mut idx: libc::c_int) {
    let mut gc: *mut mrb_gc = &mut (*mrb).gc;
    let mut capa: libc::c_int = (*gc).arena_capa;
    if idx < capa / 4i32 {
        capa >>= 2i32;
        if capa < 100i32 { capa = 100i32 }
        if capa != (*gc).arena_capa {
            (*gc).arena =
                mrb_realloc(mrb, (*gc).arena as *mut libc::c_void,
                            (::std::mem::size_of::<*mut RBasic>() as
                                 libc::c_ulong).wrapping_mul(capa as
                                                                 libc::c_ulong))
                    as *mut *mut RBasic;
            (*gc).arena_capa = capa
        }
    };
}
#[inline]
unsafe extern "C" fn stack_clear(mut from: *mut mrb_value,
                                 mut count: size_t) {
    let mrb_value_zero: mrb_value =
        mrb_value{value: C2RustUnnamed_0{f: 0i32 as mrb_float,},
                  tt: MRB_TT_FALSE,};
    loop  {
        let fresh10 = count;
        count = count.wrapping_sub(1);
        if !(fresh10 > 0i32 as libc::c_ulong) { break ; }
        let fresh11 = from;
        from = from.offset(1);
        *fresh11 = mrb_value_zero
    };
}
#[inline]
unsafe extern "C" fn stack_copy(mut dst: *mut mrb_value,
                                mut src: *const mrb_value, mut size: size_t) {
    loop  {
        let fresh12 = size;
        size = size.wrapping_sub(1);
        if !(fresh12 > 0i32 as libc::c_ulong) { break ; }
        let fresh14 = dst;
        dst = dst.offset(1);
        let fresh13 = src;
        src = src.offset(1);
        *fresh14 = *fresh13
    };
}
unsafe extern "C" fn stack_init(mut mrb: *mut mrb_state) {
    let mut c: *mut mrb_context = (*mrb).c;
    /* mrb_assert(mrb->stack == NULL); */
    (*c).stbase =
        mrb_calloc(mrb, 128i32 as size_t,
                   ::std::mem::size_of::<mrb_value>() as libc::c_ulong) as
            *mut mrb_value;
    (*c).stend = (*c).stbase.offset(128isize);
    (*c).stack = (*c).stbase;
    /* mrb_assert(ci == NULL); */
    (*c).cibase =
        mrb_calloc(mrb, 32i32 as size_t,
                   ::std::mem::size_of::<mrb_callinfo>() as libc::c_ulong) as
            *mut mrb_callinfo;
    (*c).ciend = (*c).cibase.offset(32isize);
    (*c).ci = (*c).cibase;
    (*(*c).ci).target_class = (*mrb).object_class;
    (*(*c).ci).stackent = (*c).stack;
}
#[inline]
unsafe extern "C" fn envadjust(mut mrb: *mut mrb_state,
                               mut oldbase: *mut mrb_value,
                               mut newbase: *mut mrb_value,
                               mut oldsize: size_t) {
    let mut ci: *mut mrb_callinfo = (*(*mrb).c).cibase;
    if newbase == oldbase { return }
    while ci <= (*(*mrb).c).ci {
        let mut e: *mut REnv = (*ci).env;
        let mut st: *mut mrb_value = 0 as *mut mrb_value;
        if !e.is_null() && (*e).flags() as libc::c_int & 1i32 << 20i32 == 0i32
               && { st = (*e).stack; !st.is_null() } && oldbase <= st &&
               st < oldbase.offset(oldsize as isize) {
            let mut off: ptrdiff_t =
                (*e).stack.wrapping_offset_from(oldbase) as libc::c_long;
            (*e).stack = newbase.offset(off as isize)
        }
        if !(*ci).proc_0.is_null() &&
               (*(*ci).proc_0).flags() as libc::c_int & 1024i32 != 0i32 &&
               (*ci).env !=
                   (if (*(*ci).proc_0).flags() as libc::c_int & 1024i32 !=
                           0i32 {
                        (*(*ci).proc_0).e.env
                    } else { 0 as *mut REnv }) {
            e =
                if (*(*ci).proc_0).flags() as libc::c_int & 1024i32 != 0i32 {
                    (*(*ci).proc_0).e.env
                } else { 0 as *mut REnv };
            if !e.is_null() &&
                   (*e).flags() as libc::c_int & 1i32 << 20i32 == 0i32 &&
                   { st = (*e).stack; !st.is_null() } && oldbase <= st &&
                   st < oldbase.offset(oldsize as isize) {
                let mut off_0: ptrdiff_t =
                    (*e).stack.wrapping_offset_from(oldbase) as libc::c_long;
                (*e).stack = newbase.offset(off_0 as isize)
            }
        }
        (*ci).stackent =
            newbase.offset((*ci).stackent.wrapping_offset_from(oldbase) as
                               libc::c_long as isize);
        ci = ci.offset(1isize)
    };
}
/* * def rec ; $deep =+ 1 ; if $deep > 1000 ; return 0 ; end ; rec ; end  */
unsafe extern "C" fn stack_extend_alloc(mut mrb: *mut mrb_state,
                                        mut room: mrb_int) {
    let mut oldbase: *mut mrb_value = (*(*mrb).c).stbase;
    let mut newstack: *mut mrb_value = 0 as *mut mrb_value;
    let mut oldsize: size_t =
        (*(*mrb).c).stend.wrapping_offset_from((*(*mrb).c).stbase) as
            libc::c_long as size_t;
    let mut size: size_t = oldsize;
    let mut off: size_t =
        (*(*mrb).c).stack.wrapping_offset_from((*(*mrb).c).stbase) as
            libc::c_long as size_t;
    if off > size { size = off }
    /* Use linear stack growth.
     It is slightly slower than doubling the stack space,
     but it saves memory on small devices. */
    if room <= 128i32 as libc::c_longlong {
        size =
            (size as libc::c_ulong).wrapping_add(128i32 as libc::c_ulong) as
                size_t as size_t
    } else {
        size =
            (size as
                 libc::c_ulonglong).wrapping_add(room as libc::c_ulonglong) as
                size_t as size_t
    }
    newstack =
        mrb_realloc(mrb, (*(*mrb).c).stbase as *mut libc::c_void,
                    (::std::mem::size_of::<mrb_value>() as
                         libc::c_ulong).wrapping_mul(size)) as *mut mrb_value;
    if newstack.is_null() {
        mrb_exc_raise(mrb,
                      mrb_obj_value((*mrb).stack_err as *mut libc::c_void));
    }
    stack_clear(&mut *newstack.offset(oldsize as isize),
                size.wrapping_sub(oldsize));
    envadjust(mrb, oldbase, newstack, oldsize);
    (*(*mrb).c).stbase = newstack;
    (*(*mrb).c).stack = (*(*mrb).c).stbase.offset(off as isize);
    (*(*mrb).c).stend = (*(*mrb).c).stbase.offset(size as isize);
    /* Raise an exception if the new stack size will be too large,
     to prevent infinite recursion. However, do this only after resizing the stack, so mrb_raise has stack space to work with. */
    if size > (0x40000i32 - 128i32) as libc::c_ulong {
        mrb_exc_raise(mrb,
                      mrb_obj_value((*mrb).stack_err as *mut libc::c_void));
    };
}
/*
 * FiberError reference
 *
 * @mrbgem mruby-fiber
 */
#[no_mangle]
pub unsafe extern "C" fn mrb_stack_extend(mut mrb: *mut mrb_state,
                                          mut room: mrb_int) {
    if (*(*mrb).c).stack.offset(room as isize) >= (*(*mrb).c).stend {
        stack_extend_alloc(mrb, room);
    };
}
#[inline]
unsafe extern "C" fn uvenv(mut mrb: *mut mrb_state, mut up: libc::c_int)
 -> *mut REnv {
    let mut proc_0: *mut RProc = (*(*(*mrb).c).ci).proc_0;
    let mut e: *mut REnv = 0 as *mut REnv;
    loop  {
        let fresh15 = up;
        up = up - 1;
        if !(0 != fresh15) { break ; }
        proc_0 = (*proc_0).upper;
        if proc_0.is_null() { return 0 as *mut REnv }
    }
    e =
        if (*proc_0).flags() as libc::c_int & 1024i32 != 0i32 {
            (*proc_0).e.env
        } else { 0 as *mut REnv };
    if !e.is_null() {
        /* proc has enclosed env */
        return e
    } else {
        let mut ci: *mut mrb_callinfo = (*(*mrb).c).ci;
        let mut cb: *mut mrb_callinfo = (*(*mrb).c).cibase;
        while cb <= ci {
            if (*ci).proc_0 == proc_0 { return (*ci).env }
            ci = ci.offset(-1isize)
        }
    }
    return 0 as *mut REnv;
}
#[inline]
unsafe extern "C" fn top_proc(mut mrb: *mut mrb_state, mut proc_0: *mut RProc)
 -> *mut RProc {
    while !(*proc_0).upper.is_null() {
        if (*proc_0).flags() as libc::c_int & 2048i32 != 0i32 ||
               (*proc_0).flags() as libc::c_int & 256i32 != 0i32 {
            return proc_0
        }
        proc_0 = (*proc_0).upper
    }
    return proc_0;
}
#[inline]
unsafe extern "C" fn cipush(mut mrb: *mut mrb_state) -> *mut mrb_callinfo {
    let mut c: *mut mrb_context = (*mrb).c;
    static mut ci_zero: mrb_callinfo =
        mrb_callinfo{mid: 0i32 as mrb_sym,
                     proc_0: 0 as *const RProc as *mut RProc,
                     stackent: 0 as *const mrb_value as *mut mrb_value,
                     ridx: 0,
                     epos: 0,
                     env: 0 as *const REnv as *mut REnv,
                     pc: 0 as *const mrb_code as *mut mrb_code,
                     err: 0 as *const mrb_code as *mut mrb_code,
                     argc: 0,
                     acc: 0,
                     target_class: 0 as *const RClass as *mut RClass,};
    let mut ci: *mut mrb_callinfo = (*c).ci;
    let mut ridx: libc::c_int = (*ci).ridx as libc::c_int;
    if ci.offset(1isize) == (*c).ciend {
        let mut size: ptrdiff_t =
            ci.wrapping_offset_from((*c).cibase) as libc::c_long;
        (*c).cibase =
            mrb_realloc(mrb, (*c).cibase as *mut libc::c_void,
                        (::std::mem::size_of::<mrb_callinfo>() as
                             libc::c_ulong).wrapping_mul(size as
                                                             libc::c_ulong).wrapping_mul(2i32
                                                                                             as
                                                                                             libc::c_ulong))
                as *mut mrb_callinfo;
        (*c).ci = (*c).cibase.offset(size as isize);
        (*c).ciend =
            (*c).cibase.offset((size * 2i32 as libc::c_long) as isize)
    }
    (*c).ci = (*c).ci.offset(1isize);
    ci = (*c).ci;
    *ci = ci_zero;
    (*ci).epos = (*(*mrb).c).eidx;
    (*ci).ridx = ridx as uint16_t;
    return ci;
}
#[no_mangle]
pub unsafe extern "C" fn mrb_env_unshare(mut mrb: *mut mrb_state,
                                         mut e: *mut REnv) {
    if e.is_null() {
        return
    } else {
        let mut len: size_t =
            ((*e).flags() as libc::c_int & 0x3ffi32) as mrb_int as size_t;
        let mut p: *mut mrb_value = 0 as *mut mrb_value;
        if !((*e).flags() as libc::c_int & 1i32 << 20i32 == 0i32) { return }
        if (*e).cxt != (*mrb).c { return }
        if e == (*(*(*mrb).c).cibase).env {
            /* for mirb */
            return
        }
        p =
            mrb_malloc(mrb,
                       (::std::mem::size_of::<mrb_value>() as
                            libc::c_ulong).wrapping_mul(len)) as
                *mut mrb_value;
        if len > 0i32 as libc::c_ulong { stack_copy(p, (*e).stack, len); }
        (*e).stack = p;
        (*e).set_flags((*e).flags() | (1i32 << 20i32) as uint32_t);
        mrb_write_barrier(mrb, e as *mut RBasic);
    };
}
#[inline]
unsafe extern "C" fn cipop(mut mrb: *mut mrb_state) {
    let mut c: *mut mrb_context = (*mrb).c;
    let mut env: *mut REnv = (*(*c).ci).env;
    (*c).ci = (*c).ci.offset(-1isize);
    if !env.is_null() { mrb_env_unshare(mrb, env); };
}
unsafe extern "C" fn ecall(mut mrb: *mut mrb_state) {
    let mut p: *mut RProc = 0 as *mut RProc;
    let mut c: *mut mrb_context = (*mrb).c;
    let mut ci: *mut mrb_callinfo = (*c).ci;
    let mut exc: *mut RObject = 0 as *mut RObject;
    let mut env: *mut REnv = 0 as *mut REnv;
    let mut cioff: ptrdiff_t = 0;
    let mut ai: libc::c_int = mrb_gc_arena_save(mrb);
    (*c).eidx = (*c).eidx.wrapping_sub(1);
    let mut i: uint16_t = (*c).eidx;
    let mut nregs: libc::c_int = 0;
    if (i as libc::c_int) < 0i32 { return }
    /* restrict total call depth of ecall() */
    (*mrb).ecall_nest = (*mrb).ecall_nest.wrapping_add(1);
    if (*mrb).ecall_nest as libc::c_int > 512i32 {
        mrb_exc_raise(mrb,
                      mrb_obj_value((*mrb).stack_err as *mut libc::c_void));
    }
    p = *(*c).ensure.offset(i as isize);
    if p.is_null() { return }
    let ref mut fresh16 = *(*c).ensure.offset(i as isize);
    *fresh16 = 0 as *mut RProc;
    nregs = (*(*(*p).upper).body.irep).nregs as libc::c_int;
    if !(*ci).proc_0.is_null() &&
           !((*(*ci).proc_0).flags() as libc::c_int & 128i32 != 0i32) &&
           (*(*(*ci).proc_0).body.irep).nregs as libc::c_int > nregs {
        nregs = (*(*(*ci).proc_0).body.irep).nregs as libc::c_int
    }
    cioff = ci.wrapping_offset_from((*c).cibase) as libc::c_long;
    ci = cipush(mrb);
    (*ci).stackent = (*(*mrb).c).stack;
    (*ci).mid = (*ci.offset(-1i32 as isize)).mid;
    (*ci).acc = -1i32;
    (*ci).argc = 0i32;
    (*ci).proc_0 = p;
    (*ci).target_class =
        if (*p).flags() as libc::c_int & 1024i32 != 0i32 {
            (*(*p).e.env).c
        } else { (*p).e.target_class };
    env =
        if (*p).flags() as libc::c_int & 1024i32 != 0i32 {
            (*p).e.env
        } else { 0 as *mut REnv };
    (*c).stack = (*c).stack.offset(nregs as isize);
    exc = (*mrb).exc;
    (*mrb).exc = 0 as *mut RObject;
    if !exc.is_null() {
        mrb_gc_protect(mrb, mrb_obj_value(exc as *mut libc::c_void));
    }
    if !(*(*mrb).c).fib.is_null() {
        mrb_gc_protect(mrb,
                       mrb_obj_value((*(*mrb).c).fib as *mut libc::c_void));
    }
    mrb_run(mrb, p, *(*env).stack.offset(0isize));
    (*mrb).c = c;
    (*c).ci = (*c).cibase.offset(cioff as isize);
    if (*mrb).exc.is_null() { (*mrb).exc = exc }
    mrb_gc_arena_restore(mrb, ai);
    (*mrb).ecall_nest = (*mrb).ecall_nest.wrapping_sub(1);
}
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
pub unsafe extern "C" fn mrb_funcall(mut mrb: *mut mrb_state,
                                     mut self_0: mrb_value,
                                     mut name: *const libc::c_char,
                                     mut argc: mrb_int, mut ap: ...)
 -> mrb_value {
    let mut argv: [mrb_value; 16] =
        [mrb_value{value: C2RustUnnamed_0{f: 0.,}, tt: MRB_TT_FALSE,}; 16];
    let mut i: mrb_int = 0;
    let mut mid: mrb_sym = mrb_intern_cstr(mrb, name);
    if argc > 16i32 as libc::c_longlong {
        mrb_raise(mrb,
                  mrb_exc_get(mrb,
                              b"ArgumentError\x00" as *const u8 as
                                  *const libc::c_char),
                  b"Too long arguments. (limit=16)\x00" as *const u8 as
                      *const libc::c_char);
    }
    i = 0i32 as mrb_int;
    while i < argc {
        argv[i as usize] = ap.as_va_list().arg::<mrb_value>();
        i += 1
    }
    return mrb_funcall_argv(mrb, self_0, mid, argc, argv.as_mut_ptr());
}
unsafe extern "C" fn ci_nregs(mut ci: *mut mrb_callinfo) -> libc::c_int {
    let mut p: *mut RProc = 0 as *mut RProc;
    let mut n: libc::c_int = 0i32;
    if ci.is_null() { return 3i32 }
    p = (*ci).proc_0;
    if p.is_null() {
        if (*ci).argc < 0i32 { return 3i32 }
        return (*ci).argc + 2i32
    }
    if !((*p).flags() as libc::c_int & 128i32 != 0i32) &&
           !(*p).body.irep.is_null() {
        n = (*(*p).body.irep).nregs as libc::c_int
    }
    if (*ci).argc < 0i32 {
        if n < 3i32 {
            /* self + args + blk */
            n = 3i32
        }
    }
    if (*ci).argc > n {
        /* self + blk */
        n = (*ci).argc + 2i32
    }
    return n;
}
/* *
 * Call existing ruby functions with a block.
 */
#[no_mangle]
pub unsafe extern "C" fn mrb_funcall_with_block(mut mrb: *mut mrb_state,
                                                mut self_0: mrb_value,
                                                mut mid: mrb_sym,
                                                mut argc: mrb_int,
                                                mut argv: *const mrb_value,
                                                mut blk: mrb_value)
 -> mrb_value {
    let mut val: mrb_value =
        mrb_value{value: C2RustUnnamed_0{f: 0.,}, tt: MRB_TT_FALSE,};
    if (*mrb).jmp.is_null() {
        let mut c_jmp: mrb_jmpbuf = mrb_jmpbuf{impl_0: [0; 37],};
        let mut nth_ci: ptrdiff_t =
            (*(*mrb).c).ci.wrapping_offset_from((*(*mrb).c).cibase) as
                libc::c_long;
        if _setjmp(c_jmp.impl_0.as_mut_ptr()) == 0i32 {
            (*mrb).jmp = &mut c_jmp;
            /* recursive call */
            val = mrb_funcall_with_block(mrb, self_0, mid, argc, argv, blk);
            (*mrb).jmp = 0 as *mut mrb_jmpbuf
        } else {
            /* error */
            while nth_ci <
                      (*(*mrb).c).ci.wrapping_offset_from((*(*mrb).c).cibase)
                          as libc::c_long {
                (*(*mrb).c).stack = (*(*(*mrb).c).ci).stackent;
                cipop(mrb);
            }
            (*mrb).jmp = 0 as *mut mrb_jmpbuf;
            val = mrb_obj_value((*mrb).exc as *mut libc::c_void)
        }
        (*mrb).jmp = 0 as *mut mrb_jmpbuf
    } else {
        let mut m: mrb_method_t =
            mrb_method_t{func_p: 0,
                         c2rust_unnamed:
                             C2RustUnnamed{proc_0: 0 as *mut RProc,},};
        let mut c: *mut RClass = 0 as *mut RClass;
        let mut ci: *mut mrb_callinfo = 0 as *mut mrb_callinfo;
        let mut n: libc::c_int = ci_nregs((*(*mrb).c).ci);
        let mut voff: ptrdiff_t = -1i32 as ptrdiff_t;
        if (*(*mrb).c).stack.is_null() { stack_init(mrb); }
        if argc < 0i32 as libc::c_longlong {
            mrb_raisef(mrb,
                       mrb_exc_get(mrb,
                                   b"ArgumentError\x00" as *const u8 as
                                       *const libc::c_char),
                       b"negative argc for funcall (%i)\x00" as *const u8 as
                           *const libc::c_char, argc);
        }
        c = mrb_class(mrb, self_0);
        m = mrb_method_search_vm(mrb, &mut c, mid);
        if m.c2rust_unnamed.proc_0.is_null() {
            let mut missing: mrb_sym =
                mrb_intern_static(mrb,
                                  b"method_missing\x00" as *const u8 as
                                      *const libc::c_char,
                                  (::std::mem::size_of::<[libc::c_char; 15]>()
                                       as
                                       libc::c_ulong).wrapping_sub(1i32 as
                                                                       libc::c_ulong));
            let mut args: mrb_value =
                mrb_ary_new_from_values(mrb, argc, argv);
            m = mrb_method_search_vm(mrb, &mut c, missing);
            if m.c2rust_unnamed.proc_0.is_null() {
                mrb_method_missing(mrb, mid, self_0, args);
            }
            mrb_ary_unshift(mrb, args, mrb_symbol_value(mid));
            mrb_stack_extend(mrb, (n + 2i32) as mrb_int);
            *(*(*mrb).c).stack.offset((n + 1i32) as isize) = args;
            argc = -1i32 as mrb_int
        }
        if (*(*mrb).c).ci.wrapping_offset_from((*(*mrb).c).cibase) as
               libc::c_long > 512i32 as libc::c_long {
            mrb_exc_raise(mrb,
                          mrb_obj_value((*mrb).stack_err as
                                            *mut libc::c_void));
        }
        ci = cipush(mrb);
        (*ci).mid = mid;
        (*ci).stackent = (*(*mrb).c).stack;
        (*ci).argc = argc as libc::c_int;
        (*ci).target_class = c;
        (*(*mrb).c).stack = (*(*mrb).c).stack.offset(n as isize);
        if argc < 0i32 as libc::c_longlong { argc = 1i32 as mrb_int }
        if (*(*mrb).c).stbase <= argv as *mut mrb_value &&
               argv < (*(*mrb).c).stend as *const mrb_value {
            voff =
                argv.wrapping_offset_from((*(*mrb).c).stbase) as libc::c_long
        }
        if argc >= 127i32 as libc::c_longlong {
            let mut args_0: mrb_value =
                mrb_ary_new_from_values(mrb, argc, argv);
            *(*(*mrb).c).stack.offset(1isize) = args_0;
            (*ci).argc = -1i32;
            argc = 1i32 as mrb_int
        }
        mrb_stack_extend(mrb, argc + 2i32 as libc::c_longlong);
        if 0 == m.func_p {
            let mut p: *mut RProc = m.c2rust_unnamed.proc_0;
            (*ci).proc_0 = p;
            if !((*p).flags() as libc::c_int & 128i32 != 0i32) {
                mrb_stack_extend(mrb,
                                 (*(*p).body.irep).nregs as libc::c_longlong +
                                     argc);
            }
        }
        if voff >= 0i32 as libc::c_long {
            argv = (*(*mrb).c).stbase.offset(voff as isize)
        }
        *(*(*mrb).c).stack.offset(0isize) = self_0;
        if (*ci).argc > 0i32 {
            stack_copy((*(*mrb).c).stack.offset(1isize), argv,
                       argc as size_t);
        }
        *(*(*mrb).c).stack.offset((argc + 1i32 as libc::c_longlong) as isize)
            = blk;
        if 0 !=
               if 0 != m.func_p as libc::c_int {
                   1i32
               } else if !m.c2rust_unnamed.proc_0.is_null() {
                   ((*m.c2rust_unnamed.proc_0).flags() as libc::c_int & 128i32
                        != 0i32) as libc::c_int
               } else { 0i32 } {
            let mut ai: libc::c_int = mrb_gc_arena_save(mrb);
            (*ci).acc = -2i32;
            val =
                if 0 != m.func_p as libc::c_int {
                    m.c2rust_unnamed.func
                } else if !m.c2rust_unnamed.proc_0.is_null() &&
                              (*m.c2rust_unnamed.proc_0).flags() as
                                  libc::c_int & 128i32 != 0i32 {
                    (*m.c2rust_unnamed.proc_0).body.func
                } else {
                    None
                }.expect("non-null function pointer")(mrb, self_0);
            (*(*mrb).c).stack = (*(*(*mrb).c).ci).stackent;
            cipop(mrb);
            mrb_gc_arena_restore(mrb, ai);
        } else {
            (*ci).acc = -1i32;
            val = mrb_run(mrb, m.c2rust_unnamed.proc_0, self_0)
        }
    }
    mrb_gc_protect(mrb, val);
    return val;
}
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
pub unsafe extern "C" fn mrb_funcall_argv(mut mrb: *mut mrb_state,
                                          mut self_0: mrb_value,
                                          mut mid: mrb_sym, mut argc: mrb_int,
                                          mut argv: *const mrb_value)
 -> mrb_value {
    return mrb_funcall_with_block(mrb, self_0, mid, argc, argv,
                                  mrb_nil_value());
}
#[no_mangle]
pub unsafe extern "C" fn mrb_exec_irep(mut mrb: *mut mrb_state,
                                       mut self_0: mrb_value,
                                       mut p: *mut RProc) -> mrb_value {
    let mut ci: *mut mrb_callinfo = (*(*mrb).c).ci;
    let mut keep: libc::c_int = 0;
    let mut nregs: libc::c_int = 0;
    *(*(*mrb).c).stack.offset(0isize) = self_0;
    (*ci).proc_0 = p;
    if (*p).flags() as libc::c_int & 128i32 != 0i32 {
        return (*p).body.func.expect("non-null function pointer")(mrb, self_0)
    }
    nregs = (*(*p).body.irep).nregs as libc::c_int;
    if (*ci).argc < 0i32 { keep = 3i32 } else { keep = (*ci).argc + 2i32 }
    if nregs < keep {
        mrb_stack_extend(mrb, keep as mrb_int);
    } else {
        mrb_stack_extend(mrb, nregs as mrb_int);
        stack_clear((*(*mrb).c).stack.offset(keep as isize),
                    (nregs - keep) as size_t);
    }
    ci = cipush(mrb);
    (*ci).target_class = 0 as *mut RClass;
    (*ci).pc = (*(*p).body.irep).iseq;
    (*ci).stackent = (*(*mrb).c).stack;
    (*ci).acc = 0i32;
    return self_0;
}
/* implementation of #send method */
/* 15.3.1.3.4  */
/* 15.3.1.3.44 */
/*
 *  call-seq:
 *     obj.send(symbol [, args...])        -> obj
 *     obj.__send__(symbol [, args...])      -> obj
 *
 *  Invokes the method identified by _symbol_, passing it any
 *  arguments specified. You can use <code>__send__</code> if the name
 *  +send+ clashes with an existing method in _obj_.
 *
 *     class Klass
 *       def hello(*args)
 *         "Hello " + args.join(' ')
 *       end
 *     end
 *     k = Klass.new
 *     k.send :hello, "gentle", "readers"   #=> "Hello gentle readers"
 */
#[no_mangle]
pub unsafe extern "C" fn mrb_f_send(mut mrb: *mut mrb_state,
                                    mut self_0: mrb_value) -> mrb_value {
    let mut name: mrb_sym = 0;
    let mut block: mrb_value =
        mrb_value{value: C2RustUnnamed_0{f: 0.,}, tt: MRB_TT_FALSE,};
    let mut argv: *mut mrb_value = 0 as *mut mrb_value;
    let mut regs: *mut mrb_value = 0 as *mut mrb_value;
    let mut argc: mrb_int = 0;
    let mut i: mrb_int = 0;
    let mut len: mrb_int = 0;
    let mut m: mrb_method_t =
        mrb_method_t{func_p: 0,
                     c2rust_unnamed:
                         C2RustUnnamed{proc_0: 0 as *mut RProc,},};
    let mut c: *mut RClass = 0 as *mut RClass;
    let mut ci: *mut mrb_callinfo = 0 as *mut mrb_callinfo;
    mrb_get_args(mrb, b"n*&\x00" as *const u8 as *const libc::c_char,
                 &mut name as *mut mrb_sym, &mut argv as *mut *mut mrb_value,
                 &mut argc as *mut mrb_int, &mut block as *mut mrb_value);
    ci = (*(*mrb).c).ci;
    if !((*ci).acc < 0i32) {
        c = mrb_class(mrb, self_0);
        m = mrb_method_search_vm(mrb, &mut c, name);
        if !m.c2rust_unnamed.proc_0.is_null() {
            /* call method_mising */
            (*ci).mid = name;
            (*ci).target_class = c;
            regs = (*(*mrb).c).stack.offset(1isize);
            /* remove first symbol from arguments */
            if (*ci).argc >= 0i32 {
                i = 0i32 as mrb_int;
                len = (*ci).argc as mrb_int;
                while i < len {
                    *regs.offset(i as isize) =
                        *regs.offset((i + 1i32 as libc::c_longlong) as isize);
                    i += 1
                }
                (*ci).argc -= 1
            } else {
                /* variable length arguments */
                mrb_ary_shift(mrb, *regs.offset(0isize));
            }
            if 0 !=
                   if 0 != m.func_p as libc::c_int {
                       1i32
                   } else if !m.c2rust_unnamed.proc_0.is_null() {
                       ((*m.c2rust_unnamed.proc_0).flags() as libc::c_int &
                            128i32 != 0i32) as libc::c_int
                   } else { 0i32 } {
                if 0 == m.func_p { (*ci).proc_0 = m.c2rust_unnamed.proc_0 }
                return if 0 != m.func_p as libc::c_int {
                           m.c2rust_unnamed.func
                       } else if !m.c2rust_unnamed.proc_0.is_null() &&
                                     (*m.c2rust_unnamed.proc_0).flags() as
                                         libc::c_int & 128i32 != 0i32 {
                           (*m.c2rust_unnamed.proc_0).body.func
                       } else {
                           None
                       }.expect("non-null function pointer")(mrb, self_0)
            }
            return mrb_exec_irep(mrb, self_0, m.c2rust_unnamed.proc_0)
        }
    }
    return mrb_funcall_with_block(mrb, self_0, name, argc, argv, block);
}
unsafe extern "C" fn eval_under(mut mrb: *mut mrb_state,
                                mut self_0: mrb_value, mut blk: mrb_value,
                                mut c: *mut RClass) -> mrb_value {
    let mut p: *mut RProc = 0 as *mut RProc;
    let mut ci: *mut mrb_callinfo = 0 as *mut mrb_callinfo;
    let mut nregs: libc::c_int = 0;
    if blk.tt as libc::c_uint == MRB_TT_FALSE as libc::c_int as libc::c_uint
           && 0 == blk.value.i {
        mrb_raise(mrb,
                  mrb_exc_get(mrb,
                              b"ArgumentError\x00" as *const u8 as
                                  *const libc::c_char),
                  b"no block given\x00" as *const u8 as *const libc::c_char);
    }
    ci = (*(*mrb).c).ci;
    if (*ci).acc == -2i32 {
        (*ci).target_class = c;
        return mrb_yield_cont(mrb, blk, self_0, 1i32 as mrb_int, &mut self_0)
    }
    (*ci).target_class = c;
    p = blk.value.p as *mut RProc;
    (*ci).proc_0 = p;
    (*ci).argc = 1i32;
    (*ci).mid = (*ci.offset(-1i32 as isize)).mid;
    if (*p).flags() as libc::c_int & 128i32 != 0i32 {
        mrb_stack_extend(mrb, 3i32 as mrb_int);
        *(*(*mrb).c).stack.offset(0isize) = self_0;
        *(*(*mrb).c).stack.offset(1isize) = self_0;
        *(*(*mrb).c).stack.offset(2isize) = mrb_nil_value();
        return (*p).body.func.expect("non-null function pointer")(mrb, self_0)
    }
    nregs = (*(*p).body.irep).nregs as libc::c_int;
    if nregs < 3i32 { nregs = 3i32 }
    mrb_stack_extend(mrb, nregs as mrb_int);
    *(*(*mrb).c).stack.offset(0isize) = self_0;
    *(*(*mrb).c).stack.offset(1isize) = self_0;
    stack_clear((*(*mrb).c).stack.offset(2isize), (nregs - 2i32) as size_t);
    ci = cipush(mrb);
    (*ci).target_class = 0 as *mut RClass;
    (*ci).pc = (*(*p).body.irep).iseq;
    (*ci).stackent = (*(*mrb).c).stack;
    (*ci).acc = 0i32;
    return self_0;
}
/* 15.2.2.4.35 */
/*
 *  call-seq:
 *     mod.class_eval {| | block }  -> obj
 *     mod.module_eval {| | block } -> obj
 *
 *  Evaluates block in the context of _mod_. This can
 *  be used to add methods to a class. <code>module_eval</code> returns
 *  the result of evaluating its argument.
 */
#[no_mangle]
pub unsafe extern "C" fn mrb_mod_module_eval(mut mrb: *mut mrb_state,
                                             mut mod_0: mrb_value)
 -> mrb_value {
    let mut a: mrb_value =
        mrb_value{value: C2RustUnnamed_0{f: 0.,}, tt: MRB_TT_FALSE,};
    let mut b: mrb_value =
        mrb_value{value: C2RustUnnamed_0{f: 0.,}, tt: MRB_TT_FALSE,};
    if mrb_get_args(mrb, b"|S&\x00" as *const u8 as *const libc::c_char,
                    &mut a as *mut mrb_value, &mut b as *mut mrb_value) ==
           1i32 as libc::c_longlong {
        mrb_raise(mrb,
                  mrb_exc_get(mrb,
                              b"NotImplementedError\x00" as *const u8 as
                                  *const libc::c_char),
                  b"module_eval/class_eval with string not implemented\x00" as
                      *const u8 as *const libc::c_char);
    }
    return eval_under(mrb, mod_0, b, mod_0.value.p as *mut RClass);
}
/* 15.3.1.3.18 */
/*
 *  call-seq:
 *     obj.instance_eval {| | block }                       -> obj
 *
 *  Evaluates the given block,within  the context of the receiver (_obj_).
 *  In order to set the context, the variable +self+ is set to _obj_ while
 *  the code is executing, giving the code access to _obj_'s
 *  instance variables. In the version of <code>instance_eval</code>
 *  that takes a +String+, the optional second and third
 *  parameters supply a filename and starting line number that are used
 *  when reporting compilation errors.
 *
 *     class KlassWithSecret
 *       def initialize
 *         @secret = 99
 *       end
 *     end
 *     k = KlassWithSecret.new
 *     k.instance_eval { @secret }   #=> 99
 */
#[no_mangle]
pub unsafe extern "C" fn mrb_obj_instance_eval(mut mrb: *mut mrb_state,
                                               mut self_0: mrb_value)
 -> mrb_value {
    let mut a: mrb_value =
        mrb_value{value: C2RustUnnamed_0{f: 0.,}, tt: MRB_TT_FALSE,};
    let mut b: mrb_value =
        mrb_value{value: C2RustUnnamed_0{f: 0.,}, tt: MRB_TT_FALSE,};
    let mut cv: mrb_value =
        mrb_value{value: C2RustUnnamed_0{f: 0.,}, tt: MRB_TT_FALSE,};
    let mut c: *mut RClass = 0 as *mut RClass;
    if mrb_get_args(mrb, b"|S&\x00" as *const u8 as *const libc::c_char,
                    &mut a as *mut mrb_value, &mut b as *mut mrb_value) ==
           1i32 as libc::c_longlong {
        mrb_raise(mrb,
                  mrb_exc_get(mrb,
                              b"NotImplementedError\x00" as *const u8 as
                                  *const libc::c_char),
                  b"instance_eval with string not implemented\x00" as
                      *const u8 as *const libc::c_char);
    }
    match self_0.tt as libc::c_uint {
        4 | 3 | 6 => { c = 0 as *mut RClass }
        _ => {
            cv = mrb_singleton_class(mrb, self_0);
            c = cv.value.p as *mut RClass
        }
    }
    return eval_under(mrb, self_0, b, c);
}
#[no_mangle]
pub unsafe extern "C" fn mrb_yield_with_class(mut mrb: *mut mrb_state,
                                              mut b: mrb_value,
                                              mut argc: mrb_int,
                                              mut argv: *const mrb_value,
                                              mut self_0: mrb_value,
                                              mut c: *mut RClass)
 -> mrb_value {
    let mut p: *mut RProc = 0 as *mut RProc;
    let mut mid: mrb_sym = (*(*(*mrb).c).ci).mid;
    let mut ci: *mut mrb_callinfo = 0 as *mut mrb_callinfo;
    let mut val: mrb_value =
        mrb_value{value: C2RustUnnamed_0{f: 0.,}, tt: MRB_TT_FALSE,};
    let mut n: libc::c_int = 0;
    if b.tt as libc::c_uint == MRB_TT_FALSE as libc::c_int as libc::c_uint &&
           0 == b.value.i {
        mrb_raise(mrb,
                  mrb_exc_get(mrb,
                              b"ArgumentError\x00" as *const u8 as
                                  *const libc::c_char),
                  b"no block given\x00" as *const u8 as *const libc::c_char);
    }
    ci = (*(*mrb).c).ci;
    n = ci_nregs(ci);
    if ci.wrapping_offset_from((*(*mrb).c).cibase) as libc::c_long >
           512i32 as libc::c_long {
        mrb_exc_raise(mrb,
                      mrb_obj_value((*mrb).stack_err as *mut libc::c_void));
    }
    p = b.value.p as *mut RProc;
    ci = cipush(mrb);
    (*ci).mid = mid;
    (*ci).proc_0 = p;
    (*ci).stackent = (*(*mrb).c).stack;
    (*ci).argc = argc as libc::c_int;
    (*ci).target_class = c;
    (*ci).acc = -1i32;
    n =
        if (*p).flags() as libc::c_int & 128i32 != 0i32 {
            (argc + 2i32 as libc::c_longlong) as libc::c_int
        } else { (*(*p).body.irep).nregs as libc::c_int };
    (*(*mrb).c).stack = (*(*mrb).c).stack.offset(n as isize);
    mrb_stack_extend(mrb, n as mrb_int);
    *(*(*mrb).c).stack.offset(0isize) = self_0;
    if argc > 0i32 as libc::c_longlong {
        stack_copy((*(*mrb).c).stack.offset(1isize), argv, argc as size_t);
    }
    *(*(*mrb).c).stack.offset((argc + 1i32 as libc::c_longlong) as isize) =
        mrb_nil_value();
    if (*p).flags() as libc::c_int & 128i32 != 0i32 {
        val = (*p).body.func.expect("non-null function pointer")(mrb, self_0);
        (*(*mrb).c).stack = (*(*(*mrb).c).ci).stackent;
        cipop(mrb);
    } else { val = mrb_run(mrb, p, self_0) }
    return val;
}
#[no_mangle]
pub unsafe extern "C" fn mrb_yield_argv(mut mrb: *mut mrb_state,
                                        mut b: mrb_value, mut argc: mrb_int,
                                        mut argv: *const mrb_value)
 -> mrb_value {
    let mut p: *mut RProc = b.value.p as *mut RProc;
    return mrb_yield_with_class(mrb, b, argc, argv,
                                *(*if (*p).flags() as libc::c_int & 1024i32 !=
                                          0i32 {
                                       (*p).e.env
                                   } else {
                                       0 as *mut REnv
                                   }).stack.offset(0isize),
                                if (*p).flags() as libc::c_int & 1024i32 !=
                                       0i32 {
                                    (*(*p).e.env).c
                                } else { (*p).e.target_class });
}
/* macros to get typical exception objects
   note:
   + those E_* macros requires mrb_state* variable named mrb.
   + exception objects obtained from those macros are local to mrb
*/
#[no_mangle]
pub unsafe extern "C" fn mrb_yield(mut mrb: *mut mrb_state, mut b: mrb_value,
                                   mut arg: mrb_value) -> mrb_value {
    let mut p: *mut RProc = b.value.p as *mut RProc;
    return mrb_yield_with_class(mrb, b, 1i32 as mrb_int, &mut arg,
                                *(*if (*p).flags() as libc::c_int & 1024i32 !=
                                          0i32 {
                                       (*p).e.env
                                   } else {
                                       0 as *mut REnv
                                   }).stack.offset(0isize),
                                if (*p).flags() as libc::c_int & 1024i32 !=
                                       0i32 {
                                    (*(*p).e.env).c
                                } else { (*p).e.target_class });
}
/* continue execution to the proc */
/* this function should always be called as the last function of a method */
/* e.g. return mrb_yield_cont(mrb, proc, self, argc, argv); */
#[no_mangle]
pub unsafe extern "C" fn mrb_yield_cont(mut mrb: *mut mrb_state,
                                        mut b: mrb_value,
                                        mut self_0: mrb_value,
                                        mut argc: mrb_int,
                                        mut argv: *const mrb_value)
 -> mrb_value {
    let mut p: *mut RProc = 0 as *mut RProc;
    let mut ci: *mut mrb_callinfo = 0 as *mut mrb_callinfo;
    if b.tt as libc::c_uint == MRB_TT_FALSE as libc::c_int as libc::c_uint &&
           0 == b.value.i {
        mrb_raise(mrb,
                  mrb_exc_get(mrb,
                              b"ArgumentError\x00" as *const u8 as
                                  *const libc::c_char),
                  b"no block given\x00" as *const u8 as *const libc::c_char);
    }
    if b.tt as libc::c_uint != MRB_TT_PROC as libc::c_int as libc::c_uint {
        mrb_raise(mrb,
                  mrb_exc_get(mrb,
                              b"TypeError\x00" as *const u8 as
                                  *const libc::c_char),
                  b"not a block\x00" as *const u8 as *const libc::c_char);
    }
    p = b.value.p as *mut RProc;
    ci = (*(*mrb).c).ci;
    mrb_stack_extend(mrb, 3i32 as mrb_int);
    *(*(*mrb).c).stack.offset(1isize) =
        mrb_ary_new_from_values(mrb, argc, argv);
    *(*(*mrb).c).stack.offset(2isize) = mrb_nil_value();
    (*ci).argc = -1i32;
    return mrb_exec_irep(mrb, self_0, p);
}
unsafe extern "C" fn break_new(mut mrb: *mut mrb_state, mut p: *mut RProc,
                               mut val: mrb_value) -> *mut RBreak {
    let mut brk: *mut RBreak = 0 as *mut RBreak;
    brk = mrb_obj_alloc(mrb, MRB_TT_BREAK, 0 as *mut RClass) as *mut RBreak;
    (*brk).proc_0 = p;
    (*brk).val = val;
    return brk;
}
unsafe extern "C" fn localjump_error(mut mrb: *mut mrb_state,
                                     mut kind: localjump_error_kind) {
    let mut kind_str: [[libc::c_char; 7]; 3] =
        [*::std::mem::transmute::<&[u8; 7],
                                  &mut [libc::c_char; 7]>(b"return\x00"),
         *::std::mem::transmute::<&[u8; 7],
                                  &mut [libc::c_char; 7]>(b"break\x00\x00"),
         *::std::mem::transmute::<&[u8; 7],
                                  &mut [libc::c_char; 7]>(b"yield\x00\x00")];
    let mut kind_str_len: [libc::c_char; 3] =
        [6i32 as libc::c_char, 5i32 as libc::c_char, 5i32 as libc::c_char];
    static mut lead: [libc::c_char; 12] =
        [117, 110, 101, 120, 112, 101, 99, 116, 101, 100, 32, 0];
    let mut msg: mrb_value =
        mrb_value{value: C2RustUnnamed_0{f: 0.,}, tt: MRB_TT_FALSE,};
    let mut exc: mrb_value =
        mrb_value{value: C2RustUnnamed_0{f: 0.,}, tt: MRB_TT_FALSE,};
    msg =
        mrb_str_new_capa(mrb,
                         (::std::mem::size_of::<[libc::c_char; 12]>() as
                              libc::c_ulong).wrapping_add(7i32 as
                                                              libc::c_ulong));
    mrb_str_cat(mrb, msg, lead.as_ptr(),
                (::std::mem::size_of::<[libc::c_char; 12]>() as
                     libc::c_ulong).wrapping_sub(1i32 as libc::c_ulong));
    mrb_str_cat(mrb, msg, kind_str[kind as usize].as_mut_ptr(),
                kind_str_len[kind as usize] as size_t);
    exc =
        mrb_exc_new_str(mrb,
                        mrb_exc_get(mrb,
                                    b"LocalJumpError\x00" as *const u8 as
                                        *const libc::c_char), msg);
    mrb_exc_set(mrb, exc);
}
unsafe extern "C" fn argnum_error(mut mrb: *mut mrb_state, mut num: mrb_int) {
    let mut exc: mrb_value =
        mrb_value{value: C2RustUnnamed_0{f: 0.,}, tt: MRB_TT_FALSE,};
    let mut str: mrb_value =
        mrb_value{value: C2RustUnnamed_0{f: 0.,}, tt: MRB_TT_FALSE,};
    let mut argc: mrb_int = (*(*(*mrb).c).ci).argc as mrb_int;
    if argc < 0i32 as libc::c_longlong {
        let mut args: mrb_value = *(*(*mrb).c).stack.offset(1isize);
        if args.tt as libc::c_uint ==
               MRB_TT_ARRAY as libc::c_int as libc::c_uint {
            argc =
                if 0 !=
                       (*(args.value.p as *mut RArray)).flags() as libc::c_int
                           & 7i32 {
                    (((*(args.value.p as *mut RArray)).flags() as libc::c_int
                          & 7i32) - 1i32) as mrb_int
                } else { (*(args.value.p as *mut RArray)).as_0.heap.len }
        }
    }
    if 0 != (*(*(*mrb).c).ci).mid {
        str =
            mrb_format(mrb,
                       b"\'%n\': wrong number of arguments (%i for %i)\x00" as
                           *const u8 as *const libc::c_char,
                       (*(*(*mrb).c).ci).mid, argc, num)
    } else {
        str =
            mrb_format(mrb,
                       b"wrong number of arguments (%i for %i)\x00" as
                           *const u8 as *const libc::c_char, argc, num)
    }
    exc =
        mrb_exc_new_str(mrb,
                        mrb_exc_get(mrb,
                                    b"ArgumentError\x00" as *const u8 as
                                        *const libc::c_char), str);
    mrb_exc_set(mrb, exc);
}
/* ifndef MRB_DISABLE_DIRECT_THREADING */
#[no_mangle]
pub unsafe extern "C" fn mrb_vm_run(mut mrb: *mut mrb_state,
                                    mut proc_0: *mut RProc,
                                    mut self_0: mrb_value,
                                    mut stack_keep: libc::c_uint)
 -> mrb_value {
    let mut irep: *mut mrb_irep = (*proc_0).body.irep;
    let mut result: mrb_value =
        mrb_value{value: C2RustUnnamed_0{f: 0.,}, tt: MRB_TT_FALSE,};
    let mut c: *mut mrb_context = (*mrb).c;
    let mut cioff: ptrdiff_t =
        (*c).ci.wrapping_offset_from((*c).cibase) as libc::c_long;
    let mut nregs: libc::c_uint = (*irep).nregs as libc::c_uint;
    if (*c).stack.is_null() { stack_init(mrb); }
    if stack_keep > nregs { nregs = stack_keep }
    mrb_stack_extend(mrb, nregs as mrb_int);
    stack_clear((*c).stack.offset(stack_keep as isize),
                nregs.wrapping_sub(stack_keep) as size_t);
    *(*c).stack.offset(0isize) = self_0;
    result = mrb_vm_exec(mrb, proc_0, (*irep).iseq);
    if (*mrb).c != c {
        if !(*(*mrb).c).fib.is_null() {
            mrb_write_barrier(mrb, (*(*mrb).c).fib as *mut RBasic);
        }
        (*mrb).c = c
    } else if (*c).ci.wrapping_offset_from((*c).cibase) as libc::c_long >
                  cioff {
        (*c).ci = (*c).cibase.offset(cioff as isize)
    }
    return result;
}
unsafe extern "C" fn check_target_class(mut mrb: *mut mrb_state) -> mrb_bool {
    if (*(*(*mrb).c).ci).target_class.is_null() {
        let mut exc: mrb_value =
            mrb_exc_new_str(mrb,
                            mrb_exc_get(mrb,
                                        b"TypeError\x00" as *const u8 as
                                            *const libc::c_char),
                            mrb_str_new_static(mrb,
                                               b"no target class or module\x00"
                                                   as *const u8 as
                                                   *const libc::c_char,
                                               (::std::mem::size_of::<[libc::c_char; 26]>()
                                                    as
                                                    libc::c_ulong).wrapping_sub(1i32
                                                                                    as
                                                                                    libc::c_ulong)));
        mrb_exc_set(mrb, exc);
        return 0i32 as mrb_bool
    }
    return 1i32 as mrb_bool;
}
#[no_mangle]
pub unsafe extern "C" fn mrb_vm_exec(mut mrb: *mut mrb_state,
                                     mut proc_0: *mut RProc,
                                     mut pc: *mut mrb_code) -> mrb_value {
    let mut ci_3: *mut mrb_callinfo = 0 as *mut mrb_callinfo;
    let mut acc: libc::c_int = 0;
    let mut v_0: mrb_value =
        mrb_value{value: C2RustUnnamed_0{f: 0.,}, tt: MRB_TT_FALSE,};
    let mut dst: *mut RProc = 0 as *mut RProc;
    let mut result_0: libc::c_int = 0;
    let mut k: mrb_value =
        mrb_value{value: C2RustUnnamed_0{f: 0.,}, tt: MRB_TT_FALSE,};
    let mut kdict_0: mrb_value =
        mrb_value{value: C2RustUnnamed_0{f: 0.,}, tt: MRB_TT_FALSE,};
    let mut m1_0: libc::c_int = 0;
    let mut o: libc::c_int = 0;
    let mut r_0: libc::c_int = 0;
    let mut m2_0: libc::c_int = 0;
    let mut kd_0: libc::c_int = 0;
    let mut argc_1: libc::c_int = 0;
    let mut argv: *mut mrb_value = 0 as *mut mrb_value;
    let mut argv0: *mut mrb_value = 0 as *mut mrb_value;
    let mut len_0: libc::c_int = 0;
    let mut blk_pos: libc::c_int = 0;
    let mut blk_1: *mut mrb_value = 0 as *mut mrb_value;
    let mut kdict: mrb_value =
        mrb_value{value: C2RustUnnamed_0{f: 0.,}, tt: MRB_TT_FALSE,};
    let mut kargs: libc::c_int = 0;
    let mut x_2: libc::c_double = 0.;
    let mut y_2: libc::c_double = 0.;
    let mut f: libc::c_double = 0.;
    let mut result_1: libc::c_int = 0;
    let mut m1: libc::c_int = 0;
    let mut r: libc::c_int = 0;
    let mut m2: libc::c_int = 0;
    let mut kd: libc::c_int = 0;
    let mut lv: libc::c_int = 0;
    let mut stack: *mut mrb_value = 0 as *mut mrb_value;
    let mut argc_0: libc::c_int = 0;
    let mut bidx_2: libc::c_int = 0;
    let mut m_1: mrb_method_t =
        mrb_method_t{func_p: 0,
                     c2rust_unnamed:
                         C2RustUnnamed{proc_0: 0 as *mut RProc,},};
    let mut cls_0: *mut RClass = 0 as *mut RClass;
    let mut ci_2: *mut mrb_callinfo = 0 as *mut mrb_callinfo;
    let mut recv_1: mrb_value =
        mrb_value{value: C2RustUnnamed_0{f: 0.,}, tt: MRB_TT_FALSE,};
    let mut blk_0: mrb_value =
        mrb_value{value: C2RustUnnamed_0{f: 0.,}, tt: MRB_TT_FALSE,};
    let mut mid_1: mrb_sym = 0;
    let mut target_class_0: *mut RClass = 0 as *mut RClass;
    let mut result_2: libc::c_int = 0;
    let mut result_3: libc::c_int = 0;
    let mut m1_1: libc::c_int = 0;
    let mut r_1: libc::c_int = 0;
    let mut m2_1: libc::c_int = 0;
    let mut kd_1: libc::c_int = 0;
    let mut lv_0: libc::c_int = 0;
    let mut stack_0: *mut mrb_value = 0 as *mut mrb_value;
    let mut result: libc::c_int = 0;
    let mut p: *mut RProc = 0 as *mut RProc;
    let mut exc_1: mrb_value =
        mrb_value{value: C2RustUnnamed_0{f: 0.,}, tt: MRB_TT_FALSE,};
    let mut e_1: mrb_value =
        mrb_value{value: C2RustUnnamed_0{f: 0.,}, tt: MRB_TT_FALSE,};
    let mut ec: *mut RClass = 0 as *mut RClass;
    let mut exc_7: mrb_value =
        mrb_value{value: C2RustUnnamed_0{f: 0.,}, tt: MRB_TT_FALSE,};
    let mut ci0: *mut mrb_callinfo = 0 as *mut mrb_callinfo;
    let mut exc_9: mrb_value =
        mrb_value{value: C2RustUnnamed_0{f: 0.,}, tt: MRB_TT_FALSE,};
    let mut current_block: u64;
    /* mrb_assert(MRB_PROC_CFUNC_P(proc)) */
    let mut pc0: *mut mrb_code = pc;
    let mut irep: *mut mrb_irep = (*proc_0).body.irep;
    let mut pool: *mut mrb_value = (*irep).pool;
    let mut syms: *mut mrb_sym = (*irep).syms;
    let mut insn: mrb_code = 0;
    let mut ai: libc::c_int = mrb_gc_arena_save(mrb);
    let mut prev_jmp: *mut mrb_jmpbuf = (*mrb).jmp;
    let mut c_jmp: mrb_jmpbuf = mrb_jmpbuf{impl_0: [0; 37],};
    let mut a: uint32_t = 0;
    let mut b: uint16_t = 0;
    let mut c: uint8_t = 0;
    let mut mid: mrb_sym = 0;
    let mut exc_catched: mrb_bool = 0i32 as mrb_bool;
    while !(_setjmp(c_jmp.impl_0.as_mut_ptr()) == 0i32) {
        exc_catched = 1i32 as mrb_bool
    }
    if 0 != exc_catched {
        exc_catched = 0i32 as mrb_bool;
        mrb_gc_arena_restore(mrb, ai);
        if !(*mrb).exc.is_null() &&
               (*(*mrb).exc).tt() as libc::c_int ==
                   MRB_TT_BREAK as libc::c_int {
            v_0 = (*((*mrb).exc as *mut RBreak)).val;
            proc_0 = (*((*mrb).exc as *mut RBreak)).proc_0;
            (*mrb).exc = 0 as *mut RObject;
            ci_3 = (*(*mrb).c).ci;
            current_block = 5718902055567931348;
        } else { current_block = 2687857153341325290; }
    } else {
        (*mrb).jmp = &mut c_jmp;
        (*(*(*mrb).c).ci).proc_0 = proc_0;
        current_block = 7149356873433890176;
    }
    'c_4688:
        loop  {
            match current_block {
                2687857153341325290 => {
                    ci_3 = (*(*mrb).c).ci;
                    ci0 = ci_3;
                    if ci_3 == (*(*mrb).c).cibase {
                        if (*ci_3).ridx as libc::c_int == 0i32 {
                            current_block = 5884177718142856409;
                        } else { current_block = 439497837856754218; }
                    } else {
                        loop  {
                            if !((*ci_3.offset(0isize)).ridx as libc::c_int ==
                                     (*ci_3.offset(-1i32 as isize)).ridx as
                                         libc::c_int) {
                                current_block = 439497837856754218;
                                break ;
                            }
                            cipop(mrb);
                            (*(*mrb).c).stack = (*ci_3).stackent;
                            if (*ci_3).acc == -1i32 && !prev_jmp.is_null() {
                                (*mrb).jmp = prev_jmp;
                                _longjmp((*prev_jmp).impl_0.as_mut_ptr(),
                                         1i32);
                            }
                            ci_3 = (*(*mrb).c).ci;
                            if ci_3 == (*(*mrb).c).cibase {
                                if (*ci_3).ridx as libc::c_int == 0i32 {
                                    current_block = 6860640994621533902;
                                    break ;
                                } else {
                                    current_block = 439497837856754218;
                                    break ;
                                }
                            } else if (*ci_3.offset(0isize)).ridx as
                                          libc::c_int ==
                                          (*ci_3.offset(-1i32 as isize)).ridx
                                              as libc::c_int {
                                while (*(*mrb).c).eidx as libc::c_int >
                                          (*ci_3).epos as libc::c_int {
                                    let mut cioff_0: ptrdiff_t =
                                        ci_3.wrapping_offset_from((*(*mrb).c).cibase)
                                            as libc::c_long;
                                    ecall(mrb);
                                    ci_3 =
                                        (*(*mrb).c).cibase.offset(cioff_0 as
                                                                      isize)
                                }
                            }
                        }
                        match current_block {
                            439497837856754218 => { }
                            _ => {
                                /* fiber top */
                                current_block = 5884177718142856409;
                            }
                        }
                    }
                    match current_block {
                        5884177718142856409 => {
                            if (*mrb).c == (*mrb).root_c {
                                (*(*mrb).c).stack = (*(*mrb).c).stbase;
                                current_block = 9425350357932430420;
                                break ;
                            } else {
                                let mut c_0: *mut mrb_context = (*mrb).c;
                                while (*c_0).eidx as libc::c_int >
                                          (*ci_3).epos as libc::c_int {
                                    let mut cioff: ptrdiff_t =
                                        ci_3.wrapping_offset_from((*(*mrb).c).cibase)
                                            as libc::c_long;
                                    ecall(mrb);
                                    ci_3 =
                                        (*(*mrb).c).cibase.offset(cioff as
                                                                      isize)
                                }
                                (*c_0).status = MRB_FIBER_TERMINATED;
                                (*mrb).c = (*c_0).prev;
                                (*c_0).prev = 0 as *mut mrb_context;
                                current_block = 2687857153341325290;
                                continue ;
                            }
                        }
                        _ => {
                            if (*ci_3).ridx as libc::c_int == 0i32 {
                                current_block = 9425350357932430420;
                                break ;
                            }
                            proc_0 = (*ci_3).proc_0;
                            irep = (*proc_0).body.irep;
                            pool = (*irep).pool;
                            syms = (*irep).syms;
                            if ci_3 < ci0 {
                                (*(*mrb).c).stack =
                                    (*ci_3.offset(1isize)).stackent
                            }
                            mrb_stack_extend(mrb, (*irep).nregs as mrb_int);
                            (*ci_3).ridx = (*ci_3).ridx.wrapping_sub(1);
                            pc =
                                (*irep).iseq.offset(*(*(*mrb).c).rescue.offset((*ci_3).ridx
                                                                                   as
                                                                                   isize)
                                                        as libc::c_int as
                                                        isize);
                            current_block = 7149356873433890176;
                            continue ;
                        }
                    }
                }
                5718902055567931348 => {
                    (*(*mrb).c).stack = (*ci_3).stackent;
                    proc_0 = (*proc_0).upper;
                    loop  {
                        if !((*(*mrb).c).cibase < ci_3 &&
                                 (*ci_3.offset(-1i32 as isize)).proc_0 !=
                                     proc_0) {
                            current_block = 14459795788234892187;
                            break ;
                        }
                        if (*ci_3.offset(-1i32 as isize)).acc == -1i32 {
                            while ci_3 < (*(*mrb).c).ci { cipop(mrb); }
                            current_block = 3814937494483549324;
                            break ;
                        } else { ci_3 = ci_3.offset(-1isize) }
                    }
                    match current_block {
                        3814937494483549324 => { }
                        _ => {
                            if ci_3 == (*(*mrb).c).cibase {
                                current_block = 3814937494483549324;
                            } else { current_block = 17785511045760650023; }
                        }
                    }
                }
                _ => {
                    insn = *pc;
                    match insn as libc::c_int {
                        0 => {
                            let fresh17 = pc;
                            pc = pc.offset(1);
                            pc0 = fresh17;
                            current_block = 14576567515993809846;
                        }
                        1 => {
                            let fresh18 = pc;
                            pc = pc.offset(1);
                            pc0 = fresh18;
                            let fresh19 = pc;
                            pc = pc.offset(1);
                            a = *fresh19 as uint32_t;
                            let fresh20 = pc;
                            pc = pc.offset(1);
                            b = *fresh20 as uint16_t;
                            current_block = 4488286894823169796;
                        }
                        2 => {
                            let fresh21 = pc;
                            pc = pc.offset(1);
                            pc0 = fresh21;
                            let fresh22 = pc;
                            pc = pc.offset(1);
                            a = *fresh22 as uint32_t;
                            let fresh23 = pc;
                            pc = pc.offset(1);
                            b = *fresh23 as uint16_t;
                            current_block = 8845338526596852646;
                        }
                        3 => {
                            let fresh24 = pc;
                            pc = pc.offset(1);
                            pc0 = fresh24;
                            let fresh25 = pc;
                            pc = pc.offset(1);
                            a = *fresh25 as uint32_t;
                            let fresh26 = pc;
                            pc = pc.offset(1);
                            b = *fresh26 as uint16_t;
                            current_block = 14447253356787937536;
                        }
                        4 => {
                            let fresh27 = pc;
                            pc = pc.offset(1);
                            pc0 = fresh27;
                            let fresh28 = pc;
                            pc = pc.offset(1);
                            a = *fresh28 as uint32_t;
                            let fresh29 = pc;
                            pc = pc.offset(1);
                            b = *fresh29 as uint16_t;
                            current_block = 7343950298149844727;
                        }
                        5 => {
                            let fresh30 = pc;
                            pc = pc.offset(1);
                            pc0 = fresh30;
                            let fresh31 = pc;
                            pc = pc.offset(1);
                            a = *fresh31 as uint32_t;
                            current_block = 14563621082360312968;
                        }
                        6 => {
                            let fresh32 = pc;
                            pc = pc.offset(1);
                            pc0 = fresh32;
                            let fresh33 = pc;
                            pc = pc.offset(1);
                            a = *fresh33 as uint32_t;
                            current_block = 14563621082360312968;
                        }
                        7 => {
                            let fresh34 = pc;
                            pc = pc.offset(1);
                            pc0 = fresh34;
                            let fresh35 = pc;
                            pc = pc.offset(1);
                            a = *fresh35 as uint32_t;
                            current_block = 14563621082360312968;
                        }
                        8 => {
                            let fresh36 = pc;
                            pc = pc.offset(1);
                            pc0 = fresh36;
                            let fresh37 = pc;
                            pc = pc.offset(1);
                            a = *fresh37 as uint32_t;
                            current_block = 14563621082360312968;
                        }
                        9 => {
                            let fresh38 = pc;
                            pc = pc.offset(1);
                            pc0 = fresh38;
                            let fresh39 = pc;
                            pc = pc.offset(1);
                            a = *fresh39 as uint32_t;
                            current_block = 14563621082360312968;
                        }
                        10 => {
                            let fresh40 = pc;
                            pc = pc.offset(1);
                            pc0 = fresh40;
                            let fresh41 = pc;
                            pc = pc.offset(1);
                            a = *fresh41 as uint32_t;
                            current_block = 14563621082360312968;
                        }
                        11 => {
                            let fresh42 = pc;
                            pc = pc.offset(1);
                            pc0 = fresh42;
                            let fresh43 = pc;
                            pc = pc.offset(1);
                            a = *fresh43 as uint32_t;
                            current_block = 14563621082360312968;
                        }
                        12 => {
                            let fresh44 = pc;
                            pc = pc.offset(1);
                            pc0 = fresh44;
                            let fresh45 = pc;
                            pc = pc.offset(1);
                            a = *fresh45 as uint32_t;
                            current_block = 14563621082360312968;
                        }
                        13 => {
                            let fresh46 = pc;
                            pc = pc.offset(1);
                            pc0 = fresh46;
                            let fresh47 = pc;
                            pc = pc.offset(1);
                            a = *fresh47 as uint32_t;
                            current_block = 14563621082360312968;
                        }
                        14 => {
                            let fresh48 = pc;
                            pc = pc.offset(1);
                            pc0 = fresh48;
                            let fresh49 = pc;
                            pc = pc.offset(1);
                            a = *fresh49 as uint32_t;
                            let fresh50 = pc;
                            pc = pc.offset(1);
                            b = *fresh50 as uint16_t;
                            current_block = 16314074004867283505;
                        }
                        15 => {
                            let fresh51 = pc;
                            pc = pc.offset(1);
                            pc0 = fresh51;
                            let fresh52 = pc;
                            pc = pc.offset(1);
                            a = *fresh52 as uint32_t;
                            current_block = 18201902862271706575;
                        }
                        16 => {
                            let fresh53 = pc;
                            pc = pc.offset(1);
                            pc0 = fresh53;
                            let fresh54 = pc;
                            pc = pc.offset(1);
                            a = *fresh54 as uint32_t;
                            current_block = 16974974966130203269;
                        }
                        17 => {
                            let fresh55 = pc;
                            pc = pc.offset(1);
                            pc0 = fresh55;
                            let fresh56 = pc;
                            pc = pc.offset(1);
                            a = *fresh56 as uint32_t;
                            current_block = 4976922244085895320;
                        }
                        18 => {
                            let fresh57 = pc;
                            pc = pc.offset(1);
                            pc0 = fresh57;
                            let fresh58 = pc;
                            pc = pc.offset(1);
                            a = *fresh58 as uint32_t;
                            current_block = 4459663504651627985;
                        }
                        19 => {
                            let fresh59 = pc;
                            pc = pc.offset(1);
                            pc0 = fresh59;
                            let fresh60 = pc;
                            pc = pc.offset(1);
                            a = *fresh60 as uint32_t;
                            let fresh61 = pc;
                            pc = pc.offset(1);
                            b = *fresh61 as uint16_t;
                            current_block = 13479157322803929894;
                        }
                        20 => {
                            let fresh62 = pc;
                            pc = pc.offset(1);
                            pc0 = fresh62;
                            let fresh63 = pc;
                            pc = pc.offset(1);
                            a = *fresh63 as uint32_t;
                            let fresh64 = pc;
                            pc = pc.offset(1);
                            b = *fresh64 as uint16_t;
                            current_block = 3151994457458062110;
                        }
                        21 => {
                            let fresh65 = pc;
                            pc = pc.offset(1);
                            pc0 = fresh65;
                            let fresh66 = pc;
                            pc = pc.offset(1);
                            a = *fresh66 as uint32_t;
                            let fresh67 = pc;
                            pc = pc.offset(1);
                            b = *fresh67 as uint16_t;
                            current_block = 14187386403465544025;
                        }
                        22 => {
                            let fresh68 = pc;
                            pc = pc.offset(1);
                            pc0 = fresh68;
                            let fresh69 = pc;
                            pc = pc.offset(1);
                            a = *fresh69 as uint32_t;
                            let fresh70 = pc;
                            pc = pc.offset(1);
                            b = *fresh70 as uint16_t;
                            current_block = 9974864727789713748;
                        }
                        23 => {
                            let fresh71 = pc;
                            pc = pc.offset(1);
                            pc0 = fresh71;
                            let fresh72 = pc;
                            pc = pc.offset(1);
                            a = *fresh72 as uint32_t;
                            let fresh73 = pc;
                            pc = pc.offset(1);
                            b = *fresh73 as uint16_t;
                            current_block = 4491581808830814586;
                        }
                        24 => {
                            let fresh74 = pc;
                            pc = pc.offset(1);
                            pc0 = fresh74;
                            let fresh75 = pc;
                            pc = pc.offset(1);
                            a = *fresh75 as uint32_t;
                            let fresh76 = pc;
                            pc = pc.offset(1);
                            b = *fresh76 as uint16_t;
                            current_block = 12788783625999190409;
                        }
                        25 => {
                            let fresh77 = pc;
                            pc = pc.offset(1);
                            pc0 = fresh77;
                            let fresh78 = pc;
                            pc = pc.offset(1);
                            a = *fresh78 as uint32_t;
                            let fresh79 = pc;
                            pc = pc.offset(1);
                            b = *fresh79 as uint16_t;
                            current_block = 3906822848181906220;
                        }
                        26 => {
                            let fresh80 = pc;
                            pc = pc.offset(1);
                            pc0 = fresh80;
                            let fresh81 = pc;
                            pc = pc.offset(1);
                            a = *fresh81 as uint32_t;
                            let fresh82 = pc;
                            pc = pc.offset(1);
                            b = *fresh82 as uint16_t;
                            current_block = 1105701036774490218;
                        }
                        27 => {
                            let fresh83 = pc;
                            pc = pc.offset(1);
                            pc0 = fresh83;
                            let fresh84 = pc;
                            pc = pc.offset(1);
                            a = *fresh84 as uint32_t;
                            let fresh85 = pc;
                            pc = pc.offset(1);
                            b = *fresh85 as uint16_t;
                            current_block = 13114814261106982490;
                        }
                        28 => {
                            let fresh86 = pc;
                            pc = pc.offset(1);
                            pc0 = fresh86;
                            let fresh87 = pc;
                            pc = pc.offset(1);
                            a = *fresh87 as uint32_t;
                            let fresh88 = pc;
                            pc = pc.offset(1);
                            b = *fresh88 as uint16_t;
                            current_block = 6813271534392596583;
                        }
                        29 => {
                            let fresh89 = pc;
                            pc = pc.offset(1);
                            pc0 = fresh89;
                            let fresh90 = pc;
                            pc = pc.offset(1);
                            a = *fresh90 as uint32_t;
                            let fresh91 = pc;
                            pc = pc.offset(1);
                            b = *fresh91 as uint16_t;
                            current_block = 4001239642700071046;
                        }
                        30 => {
                            let fresh92 = pc;
                            pc = pc.offset(1);
                            pc0 = fresh92;
                            let fresh93 = pc;
                            pc = pc.offset(1);
                            a = *fresh93 as uint32_t;
                            let fresh94 = pc;
                            pc = pc.offset(1);
                            b = *fresh94 as uint16_t;
                            current_block = 5323028055506826224;
                        }
                        31 => {
                            let fresh95 = pc;
                            pc = pc.offset(1);
                            pc0 = fresh95;
                            let fresh96 = pc;
                            pc = pc.offset(1);
                            a = *fresh96 as uint32_t;
                            let fresh97 = pc;
                            pc = pc.offset(1);
                            b = *fresh97 as uint16_t;
                            let fresh98 = pc;
                            pc = pc.offset(1);
                            c = *fresh98;
                            current_block = 16263365153914704257;
                        }
                        32 => {
                            let fresh99 = pc;
                            pc = pc.offset(1);
                            pc0 = fresh99;
                            let fresh100 = pc;
                            pc = pc.offset(1);
                            a = *fresh100 as uint32_t;
                            let fresh101 = pc;
                            pc = pc.offset(1);
                            b = *fresh101 as uint16_t;
                            let fresh102 = pc;
                            pc = pc.offset(1);
                            c = *fresh102;
                            current_block = 4485224281673279150;
                        }
                        33 => {
                            let fresh103 = pc;
                            pc = pc.offset(1);
                            pc0 = fresh103;
                            pc = pc.offset(2isize);
                            a =
                                ((*pc.offset(-2isize).offset(0isize) as
                                      libc::c_int) << 8i32 |
                                     *pc.offset(-2isize).offset(1isize) as
                                         libc::c_int) as uint32_t;
                            current_block = 9422951997864425805;
                        }
                        34 => {
                            let fresh104 = pc;
                            pc = pc.offset(1);
                            pc0 = fresh104;
                            let fresh105 = pc;
                            pc = pc.offset(1);
                            a = *fresh105 as uint32_t;
                            pc = pc.offset(2isize);
                            b =
                                ((*pc.offset(-2isize).offset(0isize) as
                                      libc::c_int) << 8i32 |
                                     *pc.offset(-2isize).offset(1isize) as
                                         libc::c_int) as uint16_t;
                            current_block = 1297461190301222800;
                        }
                        35 => {
                            let fresh106 = pc;
                            pc = pc.offset(1);
                            pc0 = fresh106;
                            let fresh107 = pc;
                            pc = pc.offset(1);
                            a = *fresh107 as uint32_t;
                            pc = pc.offset(2isize);
                            b =
                                ((*pc.offset(-2isize).offset(0isize) as
                                      libc::c_int) << 8i32 |
                                     *pc.offset(-2isize).offset(1isize) as
                                         libc::c_int) as uint16_t;
                            current_block = 11208766757666257413;
                        }
                        36 => {
                            let fresh108 = pc;
                            pc = pc.offset(1);
                            pc0 = fresh108;
                            let fresh109 = pc;
                            pc = pc.offset(1);
                            a = *fresh109 as uint32_t;
                            pc = pc.offset(2isize);
                            b =
                                ((*pc.offset(-2isize).offset(0isize) as
                                      libc::c_int) << 8i32 |
                                     *pc.offset(-2isize).offset(1isize) as
                                         libc::c_int) as uint16_t;
                            current_block = 14166554486324432560;
                        }
                        37 => {
                            let fresh110 = pc;
                            pc = pc.offset(1);
                            pc0 = fresh110;
                            pc = pc.offset(2isize);
                            a =
                                ((*pc.offset(-2isize).offset(0isize) as
                                      libc::c_int) << 8i32 |
                                     *pc.offset(-2isize).offset(1isize) as
                                         libc::c_int) as uint32_t;
                            current_block = 12913164539471242988;
                        }
                        38 => {
                            let fresh112 = pc;
                            pc = pc.offset(1);
                            pc0 = fresh112;
                            let fresh113 = pc;
                            pc = pc.offset(1);
                            a = *fresh113 as uint32_t;
                            current_block = 2722617303989997926;
                        }
                        39 => {
                            let fresh114 = pc;
                            pc = pc.offset(1);
                            pc0 = fresh114;
                            let fresh115 = pc;
                            pc = pc.offset(1);
                            a = *fresh115 as uint32_t;
                            let fresh116 = pc;
                            pc = pc.offset(1);
                            b = *fresh116 as uint16_t;
                            current_block = 16261244098005255386;
                        }
                        40 => {
                            let fresh117 = pc;
                            pc = pc.offset(1);
                            pc0 = fresh117;
                            let fresh118 = pc;
                            pc = pc.offset(1);
                            a = *fresh118 as uint32_t;
                            current_block = 14933967110489461578;
                        }
                        41 => {
                            let fresh119 = pc;
                            pc = pc.offset(1);
                            pc0 = fresh119;
                            let fresh120 = pc;
                            pc = pc.offset(1);
                            a = *fresh120 as uint32_t;
                            current_block = 85722159537708186;
                        }
                        42 => {
                            let fresh121 = pc;
                            pc = pc.offset(1);
                            pc0 = fresh121;
                            let fresh122 = pc;
                            pc = pc.offset(1);
                            a = *fresh122 as uint32_t;
                            current_block = 17178928522929307677;
                        }
                        43 => {
                            let fresh126 = pc;
                            pc = pc.offset(1);
                            pc0 = fresh126;
                            let fresh127 = pc;
                            pc = pc.offset(1);
                            a = *fresh127 as uint32_t;
                            current_block = 8592218650707875588;
                        }
                        44 => {
                            let fresh129 = pc;
                            pc = pc.offset(1);
                            pc0 = fresh129;
                            let fresh130 = pc;
                            pc = pc.offset(1);
                            a = *fresh130 as uint32_t;
                            let fresh131 = pc;
                            pc = pc.offset(1);
                            b = *fresh131 as uint16_t;
                            current_block = 12662816978892980002;
                        }
                        45 => {
                            let fresh132 = pc;
                            pc = pc.offset(1);
                            pc0 = fresh132;
                            let fresh133 = pc;
                            pc = pc.offset(1);
                            a = *fresh133 as uint32_t;
                            let fresh134 = pc;
                            pc = pc.offset(1);
                            b = *fresh134 as uint16_t;
                            current_block = 7460542724658431689;
                        }
                        46 => {
                            let fresh135 = pc;
                            pc = pc.offset(1);
                            pc0 = fresh135;
                            let fresh136 = pc;
                            pc = pc.offset(1);
                            a = *fresh136 as uint32_t;
                            let fresh137 = pc;
                            pc = pc.offset(1);
                            b = *fresh137 as uint16_t;
                            let fresh138 = pc;
                            pc = pc.offset(1);
                            c = *fresh138;
                            current_block = 7304850410330992871;
                        }
                        47 => {
                            let fresh139 = pc;
                            pc = pc.offset(1);
                            pc0 = fresh139;
                            let fresh140 = pc;
                            pc = pc.offset(1);
                            a = *fresh140 as uint32_t;
                            let fresh141 = pc;
                            pc = pc.offset(1);
                            b = *fresh141 as uint16_t;
                            let fresh142 = pc;
                            pc = pc.offset(1);
                            c = *fresh142;
                            current_block = 10790764747546649571;
                        }
                        48 => {
                            let fresh143 = pc;
                            pc = pc.offset(1);
                            pc0 = fresh143;
                            current_block = 13698748673986354976;
                        }
                        49 => {
                            let fresh144 = pc;
                            pc = pc.offset(1);
                            pc0 = fresh144;
                            let fresh145 = pc;
                            pc = pc.offset(1);
                            a = *fresh145 as uint32_t;
                            let fresh146 = pc;
                            pc = pc.offset(1);
                            b = *fresh146 as uint16_t;
                            current_block = 9607585333290054341;
                        }
                        50 => {
                            let fresh147 = pc;
                            pc = pc.offset(1);
                            pc0 = fresh147;
                            let fresh148 = pc;
                            pc = pc.offset(1);
                            a = *fresh148 as uint32_t;
                            pc = pc.offset(2isize);
                            b =
                                ((*pc.offset(-2isize).offset(0isize) as
                                      libc::c_int) << 8i32 |
                                     *pc.offset(-2isize).offset(1isize) as
                                         libc::c_int) as uint16_t;
                            current_block = 10763054988551342546;
                        }
                        51 => {
                            let fresh149 = pc;
                            pc = pc.offset(1);
                            pc0 = fresh149;
                            pc = pc.offset(3isize);
                            a =
                                ((*pc.offset(-3isize).offset(0isize) as
                                      libc::c_int) << 16i32 |
                                     (*pc.offset(-3isize).offset(1isize) as
                                          libc::c_int) << 8i32 |
                                     *pc.offset(-3isize).offset(2isize) as
                                         libc::c_int) as uint32_t;
                            current_block = 15803687294481920802;
                        }
                        54 => {
                            let fresh150 = pc;
                            pc = pc.offset(1);
                            pc0 = fresh150;
                            let fresh151 = pc;
                            pc = pc.offset(1);
                            a = *fresh151 as uint32_t;
                            let fresh152 = pc;
                            pc = pc.offset(1);
                            b = *fresh152 as uint16_t;
                            current_block = 14765652484648187635;
                        }
                        52 => {
                            let fresh153 = pc;
                            pc = pc.offset(1);
                            pc0 = fresh153;
                            let fresh154 = pc;
                            pc = pc.offset(1);
                            a = *fresh154 as uint32_t;
                            let fresh155 = pc;
                            pc = pc.offset(1);
                            b = *fresh155 as uint16_t;
                            current_block = 1525421113118994726;
                        }
                        53 => {
                            let fresh156 = pc;
                            pc = pc.offset(1);
                            pc0 = fresh156;
                            current_block = 4663630632216779857;
                        }
                        57 => {
                            let fresh157 = pc;
                            pc = pc.offset(1);
                            pc0 = fresh157;
                            let fresh158 = pc;
                            pc = pc.offset(1);
                            a = *fresh158 as uint32_t;
                            current_block = 12223856006808243146;
                        }
                        56 => {
                            let fresh159 = pc;
                            pc = pc.offset(1);
                            pc0 = fresh159;
                            let fresh160 = pc;
                            pc = pc.offset(1);
                            a = *fresh160 as uint32_t;
                            current_block = 13114312767422449834;
                        }
                        55 => {
                            let fresh161 = pc;
                            pc = pc.offset(1);
                            pc0 = fresh161;
                            let fresh162 = pc;
                            pc = pc.offset(1);
                            a = *fresh162 as uint32_t;
                            current_block = 14547135203946798640;
                        }
                        58 => {
                            let fresh163 = pc;
                            pc = pc.offset(1);
                            pc0 = fresh163;
                            let fresh164 = pc;
                            pc = pc.offset(1);
                            a = *fresh164 as uint32_t;
                            pc = pc.offset(2isize);
                            b =
                                ((*pc.offset(-2isize).offset(0isize) as
                                      libc::c_int) << 8i32 |
                                     *pc.offset(-2isize).offset(1isize) as
                                         libc::c_int) as uint16_t;
                            current_block = 2994422791481536047;
                        }
                        59 => {
                            let fresh165 = pc;
                            pc = pc.offset(1);
                            pc0 = fresh165;
                            let fresh166 = pc;
                            pc = pc.offset(1);
                            a = *fresh166 as uint32_t;
                            current_block = 16140473650108643456;
                        }
                        61 => {
                            let fresh167 = pc;
                            pc = pc.offset(1);
                            pc0 = fresh167;
                            let fresh168 = pc;
                            pc = pc.offset(1);
                            a = *fresh168 as uint32_t;
                            current_block = 10653245833193402520;
                        }
                        63 => {
                            let fresh169 = pc;
                            pc = pc.offset(1);
                            pc0 = fresh169;
                            let fresh170 = pc;
                            pc = pc.offset(1);
                            a = *fresh170 as uint32_t;
                            current_block = 2558910614333069445;
                        }
                        64 => {
                            let fresh171 = pc;
                            pc = pc.offset(1);
                            pc0 = fresh171;
                            let fresh172 = pc;
                            pc = pc.offset(1);
                            a = *fresh172 as uint32_t;
                            current_block = 15842299444801573850;
                        }
                        60 => {
                            let fresh173 = pc;
                            pc = pc.offset(1);
                            pc0 = fresh173;
                            let fresh174 = pc;
                            pc = pc.offset(1);
                            a = *fresh174 as uint32_t;
                            let fresh175 = pc;
                            pc = pc.offset(1);
                            b = *fresh175 as uint16_t;
                            current_block = 4396809944207673246;
                        }
                        62 => {
                            let fresh176 = pc;
                            pc = pc.offset(1);
                            pc0 = fresh176;
                            let fresh177 = pc;
                            pc = pc.offset(1);
                            a = *fresh177 as uint32_t;
                            let fresh178 = pc;
                            pc = pc.offset(1);
                            b = *fresh178 as uint16_t;
                            current_block = 1767406316152665813;
                        }
                        65 => {
                            let fresh179 = pc;
                            pc = pc.offset(1);
                            pc0 = fresh179;
                            let fresh180 = pc;
                            pc = pc.offset(1);
                            a = *fresh180 as uint32_t;
                            current_block = 10264557722339789830;
                        }
                        66 => {
                            let fresh181 = pc;
                            pc = pc.offset(1);
                            pc0 = fresh181;
                            let fresh182 = pc;
                            pc = pc.offset(1);
                            a = *fresh182 as uint32_t;
                            current_block = 11600479572347497286;
                        }
                        67 => {
                            let fresh183 = pc;
                            pc = pc.offset(1);
                            pc0 = fresh183;
                            let fresh184 = pc;
                            pc = pc.offset(1);
                            a = *fresh184 as uint32_t;
                            current_block = 6268356626658084969;
                        }
                        68 => {
                            let fresh185 = pc;
                            pc = pc.offset(1);
                            pc0 = fresh185;
                            let fresh186 = pc;
                            pc = pc.offset(1);
                            a = *fresh186 as uint32_t;
                            current_block = 14064792543554720178;
                        }
                        69 => {
                            let fresh187 = pc;
                            pc = pc.offset(1);
                            pc0 = fresh187;
                            let fresh188 = pc;
                            pc = pc.offset(1);
                            a = *fresh188 as uint32_t;
                            current_block = 12168003877939815570;
                        }
                        70 => {
                            let fresh189 = pc;
                            pc = pc.offset(1);
                            pc0 = fresh189;
                            let fresh190 = pc;
                            pc = pc.offset(1);
                            a = *fresh190 as uint32_t;
                            let fresh191 = pc;
                            pc = pc.offset(1);
                            b = *fresh191 as uint16_t;
                            current_block = 5334851501950231651;
                        }
                        71 => {
                            let fresh192 = pc;
                            pc = pc.offset(1);
                            pc0 = fresh192;
                            let fresh193 = pc;
                            pc = pc.offset(1);
                            a = *fresh193 as uint32_t;
                            let fresh194 = pc;
                            pc = pc.offset(1);
                            b = *fresh194 as uint16_t;
                            let fresh195 = pc;
                            pc = pc.offset(1);
                            c = *fresh195;
                            current_block = 15590445366052810487;
                        }
                        72 => {
                            let fresh196 = pc;
                            pc = pc.offset(1);
                            pc0 = fresh196;
                            let fresh197 = pc;
                            pc = pc.offset(1);
                            a = *fresh197 as uint32_t;
                            current_block = 15951340130516588981;
                        }
                        73 => {
                            let fresh198 = pc;
                            pc = pc.offset(1);
                            pc0 = fresh198;
                            let fresh199 = pc;
                            pc = pc.offset(1);
                            a = *fresh199 as uint32_t;
                            current_block = 17884754765720769523;
                        }
                        74 => {
                            let fresh200 = pc;
                            pc = pc.offset(1);
                            pc0 = fresh200;
                            let fresh201 = pc;
                            pc = pc.offset(1);
                            a = *fresh201 as uint32_t;
                            current_block = 6985669818981290717;
                        }
                        75 => {
                            let fresh202 = pc;
                            pc = pc.offset(1);
                            pc0 = fresh202;
                            let fresh203 = pc;
                            pc = pc.offset(1);
                            a = *fresh203 as uint32_t;
                            let fresh204 = pc;
                            pc = pc.offset(1);
                            b = *fresh204 as uint16_t;
                            let fresh205 = pc;
                            pc = pc.offset(1);
                            c = *fresh205;
                            current_block = 10370658347790963364;
                        }
                        76 => {
                            let fresh206 = pc;
                            pc = pc.offset(1);
                            pc0 = fresh206;
                            let fresh207 = pc;
                            pc = pc.offset(1);
                            a = *fresh207 as uint32_t;
                            let fresh208 = pc;
                            pc = pc.offset(1);
                            b = *fresh208 as uint16_t;
                            let fresh209 = pc;
                            pc = pc.offset(1);
                            c = *fresh209;
                            current_block = 4938714451943331475;
                        }
                        77 => {
                            let fresh210 = pc;
                            pc = pc.offset(1);
                            pc0 = fresh210;
                            let fresh211 = pc;
                            pc = pc.offset(1);
                            a = *fresh211 as uint32_t;
                            let fresh212 = pc;
                            pc = pc.offset(1);
                            b = *fresh212 as uint16_t;
                            let fresh213 = pc;
                            pc = pc.offset(1);
                            c = *fresh213;
                            current_block = 11125061941892892926;
                        }
                        78 => {
                            let fresh218 = pc;
                            pc = pc.offset(1);
                            pc0 = fresh218;
                            let fresh219 = pc;
                            pc = pc.offset(1);
                            a = *fresh219 as uint32_t;
                            current_block = 2896230971687472890;
                        }
                        79 => {
                            let fresh220 = pc;
                            pc = pc.offset(1);
                            pc0 = fresh220;
                            let fresh221 = pc;
                            pc = pc.offset(1);
                            a = *fresh221 as uint32_t;
                            let fresh222 = pc;
                            pc = pc.offset(1);
                            b = *fresh222 as uint16_t;
                            current_block = 14841647832940836303;
                        }
                        80 => {
                            let fresh223 = pc;
                            pc = pc.offset(1);
                            pc0 = fresh223;
                            let fresh224 = pc;
                            pc = pc.offset(1);
                            a = *fresh224 as uint32_t;
                            current_block = 4191712132543108294;
                        }
                        81 => {
                            let fresh225 = pc;
                            pc = pc.offset(1);
                            pc0 = fresh225;
                            let fresh226 = pc;
                            pc = pc.offset(1);
                            a = *fresh226 as uint32_t;
                            let fresh227 = pc;
                            pc = pc.offset(1);
                            b = *fresh227 as uint16_t;
                            current_block = 1686938961607716371;
                        }
                        82 => {
                            let fresh228 = pc;
                            pc = pc.offset(1);
                            pc0 = fresh228;
                            let fresh229 = pc;
                            pc = pc.offset(1);
                            a = *fresh229 as uint32_t;
                            let fresh230 = pc;
                            pc = pc.offset(1);
                            b = *fresh230 as uint16_t;
                            current_block = 17492450365994263002;
                        }
                        83 => {
                            let fresh231 = pc;
                            pc = pc.offset(1);
                            pc0 = fresh231;
                            let fresh232 = pc;
                            pc = pc.offset(1);
                            a = *fresh232 as uint32_t;
                            current_block = 15495526794150689403;
                        }
                        84 => {
                            let fresh233 = pc;
                            pc = pc.offset(1);
                            pc0 = fresh233;
                            let fresh234 = pc;
                            pc = pc.offset(1);
                            a = *fresh234 as uint32_t;
                            let fresh235 = pc;
                            pc = pc.offset(1);
                            b = *fresh235 as uint16_t;
                            current_block = 2702902889507381978;
                        }
                        85 => {
                            let fresh236 = pc;
                            pc = pc.offset(1);
                            pc0 = fresh236;
                            let fresh237 = pc;
                            pc = pc.offset(1);
                            a = *fresh237 as uint32_t;
                            let fresh238 = pc;
                            pc = pc.offset(1);
                            b = *fresh238 as uint16_t;
                            current_block = 1152971280698483565;
                        }
                        86 => {
                            let fresh239 = pc;
                            pc = pc.offset(1);
                            pc0 = fresh239;
                            let fresh240 = pc;
                            pc = pc.offset(1);
                            a = *fresh240 as uint32_t;
                            let fresh241 = pc;
                            pc = pc.offset(1);
                            b = *fresh241 as uint16_t;
                            current_block = 16224973743324382762;
                        }
                        87 => {
                            let fresh242 = pc;
                            pc = pc.offset(1);
                            pc0 = fresh242;
                            let fresh243 = pc;
                            pc = pc.offset(1);
                            a = *fresh243 as uint32_t;
                            current_block = 6571067991607217453;
                        }
                        88 => {
                            let fresh244 = pc;
                            pc = pc.offset(1);
                            pc0 = fresh244;
                            let fresh245 = pc;
                            pc = pc.offset(1);
                            a = *fresh245 as uint32_t;
                            current_block = 14502557129029694510;
                        }
                        89 => {
                            let fresh246 = pc;
                            pc = pc.offset(1);
                            pc0 = fresh246;
                            let fresh247 = pc;
                            pc = pc.offset(1);
                            a = *fresh247 as uint32_t;
                            current_block = 16196049249653527883;
                        }
                        90 => {
                            let fresh248 = pc;
                            pc = pc.offset(1);
                            pc0 = fresh248;
                            let fresh249 = pc;
                            pc = pc.offset(1);
                            a = *fresh249 as uint32_t;
                            let fresh250 = pc;
                            pc = pc.offset(1);
                            b = *fresh250 as uint16_t;
                            current_block = 16736254050004001558;
                        }
                        91 => {
                            let fresh251 = pc;
                            pc = pc.offset(1);
                            pc0 = fresh251;
                            let fresh252 = pc;
                            pc = pc.offset(1);
                            a = *fresh252 as uint32_t;
                            let fresh253 = pc;
                            pc = pc.offset(1);
                            b = *fresh253 as uint16_t;
                            current_block = 16407897916814827188;
                        }
                        92 => {
                            let fresh254 = pc;
                            pc = pc.offset(1);
                            pc0 = fresh254;
                            let fresh255 = pc;
                            pc = pc.offset(1);
                            a = *fresh255 as uint32_t;
                            let fresh256 = pc;
                            pc = pc.offset(1);
                            b = *fresh256 as uint16_t;
                            current_block = 9892557545450190862;
                        }
                        93 => {
                            let fresh257 = pc;
                            pc = pc.offset(1);
                            pc0 = fresh257;
                            let fresh258 = pc;
                            pc = pc.offset(1);
                            a = *fresh258 as uint32_t;
                            let fresh259 = pc;
                            pc = pc.offset(1);
                            b = *fresh259 as uint16_t;
                            current_block = 15928390647053518202;
                        }
                        96 => {
                            let fresh260 = pc;
                            pc = pc.offset(1);
                            pc0 = fresh260;
                            let fresh261 = pc;
                            pc = pc.offset(1);
                            a = *fresh261 as uint32_t;
                            current_block = 15212051862008919012;
                        }
                        97 => {
                            let fresh262 = pc;
                            pc = pc.offset(1);
                            pc0 = fresh262;
                            let fresh263 = pc;
                            pc = pc.offset(1);
                            a = *fresh263 as uint32_t;
                            current_block = 15561344248022367768;
                        }
                        94 => {
                            let fresh264 = pc;
                            pc = pc.offset(1);
                            pc0 = fresh264;
                            let fresh265 = pc;
                            pc = pc.offset(1);
                            a = *fresh265 as uint32_t;
                            let fresh266 = pc;
                            pc = pc.offset(1);
                            b = *fresh266 as uint16_t;
                            current_block = 1656069329997846098;
                        }
                        95 => {
                            let fresh267 = pc;
                            pc = pc.offset(1);
                            pc0 = fresh267;
                            let fresh268 = pc;
                            pc = pc.offset(1);
                            a = *fresh268 as uint32_t;
                            current_block = 5769344152980800111;
                        }
                        98 => {
                            let fresh269 = pc;
                            pc = pc.offset(1);
                            pc0 = fresh269;
                            current_block = 10071922767614739690;
                            break ;
                        }
                        99 => {
                            let fresh273 = pc;
                            pc = pc.offset(1);
                            pc0 = fresh273;
                            let fresh274 = pc;
                            pc = pc.offset(1);
                            a = *fresh274 as uint32_t;
                            current_block = 9217466010932042731;
                        }
                        100 => {
                            let fresh275 = pc;
                            pc = pc.offset(1);
                            pc0 = fresh275;
                            current_block = 16136094826069225138;
                        }
                        101 => {
                            let fresh332 = pc;
                            pc = pc.offset(1);
                            pc0 = fresh332;
                            current_block = 12853241758993110625;
                        }
                        102 => {
                            let fresh437 = pc;
                            pc = pc.offset(1);
                            pc0 = fresh437;
                            current_block = 3106153520265210690;
                        }
                        103 => {
                            let fresh491 = pc;
                            pc = pc.offset(1);
                            pc0 = fresh491;
                            current_block = 5128590269704079499;
                            break ;
                        }
                        _ => {
                            current_block = 7149356873433890176;
                            continue ;
                        }
                    }
                    loop  {
                        match current_block {
                            14576567515993809846 => {
                                /* do nothing */
                                current_block = 7149356873433890176;
                                continue 'c_4688 ;
                            }
                            5769344152980800111 => {
                                let mut target_1: *mut RClass =
                                    0 as *mut RClass;
                                if 0 == check_target_class(mrb) {
                                    current_block = 2687857153341325290;
                                    continue 'c_4688 ;
                                }
                                target_1 = (*(*(*mrb).c).ci).target_class;
                                mrb_undef_method_id(mrb, target_1,
                                                    *syms.offset(a as isize));
                                current_block = 7149356873433890176;
                                continue 'c_4688 ;
                            }
                            1656069329997846098 => {
                                let mut target_0: *mut RClass =
                                    0 as *mut RClass;
                                if 0 == check_target_class(mrb) {
                                    current_block = 2687857153341325290;
                                    continue 'c_4688 ;
                                }
                                target_0 = (*(*(*mrb).c).ci).target_class;
                                mrb_alias_method(mrb, target_0,
                                                 *syms.offset(a as isize),
                                                 *syms.offset(b as isize));
                                current_block = 7149356873433890176;
                                continue 'c_4688 ;
                            }
                            15928390647053518202 => {
                                let mut target: *mut RClass =
                                    (*(*(*mrb).c).stack.offset(a as
                                                                   isize)).value.p
                                        as *mut RClass;
                                let mut p_5: *mut RProc =
                                    (*(*(*mrb).c).stack.offset(a.wrapping_add(1i32
                                                                                  as
                                                                                  libc::c_uint)
                                                                   as
                                                                   isize)).value.p
                                        as *mut RProc;
                                let mut m_2: mrb_method_t =
                                    mrb_method_t{func_p: 0,
                                                 c2rust_unnamed:
                                                     C2RustUnnamed{proc_0:
                                                                       0 as
                                                                           *mut RProc,},};
                                m_2.func_p = 0i32 as mrb_bool;
                                m_2.c2rust_unnamed.proc_0 = p_5;
                                mrb_define_method_raw(mrb, target,
                                                      *syms.offset(b as
                                                                       isize),
                                                      m_2);
                                mrb_gc_arena_restore(mrb, ai);
                                current_block = 7149356873433890176;
                                continue 'c_4688 ;
                            }
                            9892557545450190862 => {
                                let mut ci_4: *mut mrb_callinfo =
                                    0 as *mut mrb_callinfo;
                                let mut recv_2: mrb_value =
                                    *(*(*mrb).c).stack.offset(a as isize);
                                let mut p_4: *mut RProc = 0 as *mut RProc;
                                let mut nirep_0: *mut mrb_irep =
                                    *(*irep).reps.offset(b as isize);
                                /* prepare closure */
                                p_4 = mrb_proc_new(mrb, nirep_0);
                                (*p_4).c = 0 as *mut RClass;
                                mrb_field_write_barrier(mrb,
                                                        p_4 as *mut RBasic,
                                                        proc_0 as
                                                            *mut RBasic);
                                if (*p_4).flags() as libc::c_int & 1024i32 !=
                                       0i32 {
                                    (*(*p_4).e.env).c =
                                        recv_2.value.p as *mut RClass;
                                    mrb_field_write_barrier(mrb,
                                                            (*p_4).e.env as
                                                                *mut RBasic,
                                                            recv_2.value.p as
                                                                *mut RClass as
                                                                *mut RBasic);
                                } else {
                                    (*p_4).e.target_class =
                                        recv_2.value.p as *mut RClass;
                                    mrb_field_write_barrier(mrb,
                                                            p_4 as
                                                                *mut RBasic,
                                                            recv_2.value.p as
                                                                *mut RClass as
                                                                *mut RBasic);
                                }
                                (*p_4).set_flags((*p_4).flags() |
                                                     2048i32 as uint32_t);
                                /* prepare call stack */
                                ci_4 = cipush(mrb);
                                (*ci_4).pc = pc;
                                (*ci_4).acc = a as libc::c_int;
                                (*ci_4).mid = 0i32 as mrb_sym;
                                (*ci_4).stackent = (*(*mrb).c).stack;
                                (*ci_4).argc = 0i32;
                                (*ci_4).target_class =
                                    recv_2.value.p as *mut RClass;
                                /* prepare stack */
                                (*(*mrb).c).stack =
                                    (*(*mrb).c).stack.offset(a as isize);
                                /* setup block to call */
                                (*ci_4).proc_0 = p_4;
                                irep = (*p_4).body.irep;
                                pool = (*irep).pool;
                                syms = (*irep).syms;
                                mrb_stack_extend(mrb,
                                                 (*irep).nregs as mrb_int);
                                stack_clear((*(*mrb).c).stack.offset(1isize),
                                            ((*irep).nregs as libc::c_int -
                                                 1i32) as size_t);
                                pc = (*irep).iseq;
                                current_block = 7149356873433890176;
                                continue 'c_4688 ;
                            }
                            16407897916814827188 => {
                                let mut cls_1: *mut RClass = 0 as *mut RClass;
                                let mut baseclass_0: *mut RClass =
                                    0 as *mut RClass;
                                let mut base_0: mrb_value =
                                    mrb_value{value: C2RustUnnamed_0{f: 0.,},
                                              tt: MRB_TT_FALSE,};
                                let mut id_0: mrb_sym =
                                    *syms.offset(b as isize);
                                base_0 =
                                    *(*(*mrb).c).stack.offset(a as isize);
                                if base_0.tt as libc::c_uint ==
                                       MRB_TT_FALSE as libc::c_int as
                                           libc::c_uint && 0 == base_0.value.i
                                   {
                                    baseclass_0 =
                                        if (*(*(*(*mrb).c).ci).proc_0).flags()
                                               as libc::c_int & 1024i32 !=
                                               0i32 {
                                            (*(*(*(*(*mrb).c).ci).proc_0).e.env).c
                                        } else {
                                            (*(*(*(*mrb).c).ci).proc_0).e.target_class
                                        };
                                    base_0 =
                                        mrb_obj_value(baseclass_0 as
                                                          *mut libc::c_void)
                                }
                                cls_1 =
                                    mrb_vm_define_module(mrb, base_0, id_0);
                                *(*(*mrb).c).stack.offset(a as isize) =
                                    mrb_obj_value(cls_1 as *mut libc::c_void);
                                mrb_gc_arena_restore(mrb, ai);
                                current_block = 7149356873433890176;
                                continue 'c_4688 ;
                            }
                            16736254050004001558 => {
                                let mut c_3: *mut RClass = 0 as *mut RClass;
                                let mut baseclass: *mut RClass =
                                    0 as *mut RClass;
                                let mut base: mrb_value =
                                    mrb_value{value: C2RustUnnamed_0{f: 0.,},
                                              tt: MRB_TT_FALSE,};
                                let mut super_0: mrb_value =
                                    mrb_value{value: C2RustUnnamed_0{f: 0.,},
                                              tt: MRB_TT_FALSE,};
                                let mut id: mrb_sym =
                                    *syms.offset(b as isize);
                                base = *(*(*mrb).c).stack.offset(a as isize);
                                super_0 =
                                    *(*(*mrb).c).stack.offset(a.wrapping_add(1i32
                                                                                 as
                                                                                 libc::c_uint)
                                                                  as isize);
                                if base.tt as libc::c_uint ==
                                       MRB_TT_FALSE as libc::c_int as
                                           libc::c_uint && 0 == base.value.i {
                                    baseclass =
                                        if (*(*(*(*mrb).c).ci).proc_0).flags()
                                               as libc::c_int & 1024i32 !=
                                               0i32 {
                                            (*(*(*(*(*mrb).c).ci).proc_0).e.env).c
                                        } else {
                                            (*(*(*(*mrb).c).ci).proc_0).e.target_class
                                        };
                                    base =
                                        mrb_obj_value(baseclass as
                                                          *mut libc::c_void)
                                }
                                c_3 =
                                    mrb_vm_define_class(mrb, base, super_0,
                                                        id);
                                *(*(*mrb).c).stack.offset(a as isize) =
                                    mrb_obj_value(c_3 as *mut libc::c_void);
                                mrb_gc_arena_restore(mrb, ai);
                                current_block = 7149356873433890176;
                                continue 'c_4688 ;
                            }
                            14502557129029694510 => {
                                let mut val_5: mrb_value =
                                    mrb_range_new(mrb,
                                                  *(*(*mrb).c).stack.offset(a
                                                                                as
                                                                                isize),
                                                  *(*(*mrb).c).stack.offset(a.wrapping_add(1i32
                                                                                               as
                                                                                               libc::c_uint)
                                                                                as
                                                                                isize),
                                                  1i32 as mrb_bool);
                                *(*(*mrb).c).stack.offset(a as isize) = val_5;
                                mrb_gc_arena_restore(mrb, ai);
                                current_block = 7149356873433890176;
                                continue 'c_4688 ;
                            }
                            6571067991607217453 => {
                                let mut val_4: mrb_value =
                                    mrb_range_new(mrb,
                                                  *(*(*mrb).c).stack.offset(a
                                                                                as
                                                                                isize),
                                                  *(*(*mrb).c).stack.offset(a.wrapping_add(1i32
                                                                                               as
                                                                                               libc::c_uint)
                                                                                as
                                                                                isize),
                                                  0i32 as mrb_bool);
                                *(*(*mrb).c).stack.offset(a as isize) = val_4;
                                mrb_gc_arena_restore(mrb, ai);
                                current_block = 7149356873433890176;
                                continue 'c_4688 ;
                            }
                            15495526794150689403 => {
                                let mut hash_1: mrb_value =
                                    mrb_ensure_hash_type(mrb,
                                                         *(*(*mrb).c).stack.offset(a
                                                                                       as
                                                                                       isize));
                                mrb_hash_merge(mrb, hash_1,
                                               *(*(*mrb).c).stack.offset(a.wrapping_add(1i32
                                                                                            as
                                                                                            libc::c_uint)
                                                                             as
                                                                             isize));
                                mrb_gc_arena_restore(mrb, ai);
                                current_block = 7149356873433890176;
                                continue 'c_4688 ;
                            }
                            17492450365994263002 => {
                                let mut hash_0: mrb_value =
                                    mrb_value{value: C2RustUnnamed_0{f: 0.,},
                                              tt: MRB_TT_FALSE,};
                                let mut i_0: libc::c_int = 0;
                                let mut lim_0: libc::c_int =
                                    a.wrapping_add((b as libc::c_int * 2i32)
                                                       as
                                                       libc::c_uint).wrapping_add(1i32
                                                                                      as
                                                                                      libc::c_uint)
                                        as libc::c_int;
                                hash_0 =
                                    mrb_ensure_hash_type(mrb,
                                                         *(*(*mrb).c).stack.offset(a
                                                                                       as
                                                                                       isize));
                                i_0 =
                                    a.wrapping_add(1i32 as libc::c_uint) as
                                        libc::c_int;
                                while i_0 < lim_0 {
                                    mrb_hash_set(mrb, hash_0,
                                                 *(*(*mrb).c).stack.offset(i_0
                                                                               as
                                                                               isize),
                                                 *(*(*mrb).c).stack.offset((i_0
                                                                                +
                                                                                1i32)
                                                                               as
                                                                               isize));
                                    i_0 += 2i32
                                }
                                mrb_gc_arena_restore(mrb, ai);
                                current_block = 7149356873433890176;
                                continue 'c_4688 ;
                            }
                            1686938961607716371 => {
                                let mut hash: mrb_value =
                                    mrb_hash_new_capa(mrb, b as mrb_int);
                                let mut i: libc::c_int = 0;
                                let mut lim: libc::c_int =
                                    a.wrapping_add((b as libc::c_int * 2i32)
                                                       as libc::c_uint) as
                                        libc::c_int;
                                i = a as libc::c_int;
                                while i < lim {
                                    mrb_hash_set(mrb, hash,
                                                 *(*(*mrb).c).stack.offset(i
                                                                               as
                                                                               isize),
                                                 *(*(*mrb).c).stack.offset((i
                                                                                +
                                                                                1i32)
                                                                               as
                                                                               isize));
                                    i += 2i32
                                }
                                *(*(*mrb).c).stack.offset(a as isize) = hash;
                                mrb_gc_arena_restore(mrb, ai);
                                current_block = 7149356873433890176;
                                continue 'c_4688 ;
                            }
                            14841647832940836303 => {
                                let mut str_1: mrb_value =
                                    mrb_str_dup(mrb,
                                                *pool.offset(b as isize));
                                *(*(*mrb).c).stack.offset(a as isize) = str_1;
                                mrb_gc_arena_restore(mrb, ai);
                                current_block = 7149356873433890176;
                                continue 'c_4688 ;
                            }
                            2896230971687472890 => {
                                let mut sym_0: mrb_sym =
                                    mrb_intern_str(mrb,
                                                   *(*(*mrb).c).stack.offset(a
                                                                                 as
                                                                                 isize));
                                *(*(*mrb).c).stack.offset(a as isize) =
                                    mrb_symbol_value(sym_0);
                                mrb_gc_arena_restore(mrb, ai);
                                current_block = 7149356873433890176;
                                continue 'c_4688 ;
                            }
                            11125061941892892926 => {
                                let mut v_4: mrb_value =
                                    *(*(*mrb).c).stack.offset(a as isize);
                                let mut pre: libc::c_int = b as libc::c_int;
                                let mut post: libc::c_int = c as libc::c_int;
                                let mut ary_3: *mut RArray = 0 as *mut RArray;
                                let mut len_1: libc::c_int = 0;
                                let mut idx: libc::c_int = 0;
                                if !(v_4.tt as libc::c_uint ==
                                         MRB_TT_ARRAY as libc::c_int as
                                             libc::c_uint) {
                                    v_4 =
                                        mrb_ary_new_from_values(mrb,
                                                                1i32 as
                                                                    mrb_int,
                                                                &mut *(*(*mrb).c).stack.offset(a
                                                                                                   as
                                                                                                   isize))
                                }
                                ary_3 = v_4.value.p as *mut RArray;
                                len_1 =
                                    (if 0 !=
                                            (*ary_3).flags() as libc::c_int &
                                                7i32 {
                                         (((*ary_3).flags() as libc::c_int &
                                               7i32) - 1i32) as mrb_int
                                     } else { (*ary_3).as_0.heap.len }) as
                                        libc::c_int;
                                if len_1 > pre + post {
                                    v_4 =
                                        mrb_ary_new_from_values(mrb,
                                                                (len_1 - pre -
                                                                     post) as
                                                                    mrb_int,
                                                                (if 0 !=
                                                                        (*ary_3).flags()
                                                                            as
                                                                            libc::c_int
                                                                            &
                                                                            7i32
                                                                    {
                                                                     &mut (*ary_3).as_0
                                                                         as
                                                                         *mut C2RustUnnamed_5
                                                                         as
                                                                         *mut mrb_value
                                                                 } else {
                                                                     (*ary_3).as_0.heap.ptr
                                                                 }).offset(pre
                                                                               as
                                                                               isize));
                                    let fresh214 = a;
                                    a = a.wrapping_add(1);
                                    *(*(*mrb).c).stack.offset(fresh214 as
                                                                  isize) =
                                        v_4;
                                    loop  {
                                        let fresh215 = post;
                                        post = post - 1;
                                        if !(0 != fresh215) { break ; }
                                        let fresh216 = a;
                                        a = a.wrapping_add(1);
                                        *(*(*mrb).c).stack.offset(fresh216 as
                                                                      isize) =
                                            *if 0 !=
                                                    (*ary_3).flags() as
                                                        libc::c_int & 7i32 {
                                                 &mut (*ary_3).as_0 as
                                                     *mut C2RustUnnamed_5 as
                                                     *mut mrb_value
                                             } else {
                                                 (*ary_3).as_0.heap.ptr
                                             }.offset((len_1 - post - 1i32) as
                                                          isize)
                                    }
                                } else {
                                    v_4 =
                                        mrb_ary_new_capa(mrb,
                                                         0i32 as mrb_int);
                                    let fresh217 = a;
                                    a = a.wrapping_add(1);
                                    *(*(*mrb).c).stack.offset(fresh217 as
                                                                  isize) =
                                        v_4;
                                    idx = 0i32;
                                    while idx + pre < len_1 {
                                        *(*(*mrb).c).stack.offset(a.wrapping_add(idx
                                                                                     as
                                                                                     libc::c_uint)
                                                                      as
                                                                      isize) =
                                            *if 0 !=
                                                    (*ary_3).flags() as
                                                        libc::c_int & 7i32 {
                                                 &mut (*ary_3).as_0 as
                                                     *mut C2RustUnnamed_5 as
                                                     *mut mrb_value
                                             } else {
                                                 (*ary_3).as_0.heap.ptr
                                             }.offset((pre + idx) as isize);
                                        idx += 1
                                    }
                                    while idx < post {
                                        (*(*(*mrb).c).stack.offset(a.wrapping_add(idx
                                                                                      as
                                                                                      libc::c_uint)
                                                                       as
                                                                       isize)).tt
                                            = MRB_TT_FALSE;
                                        (*(*(*mrb).c).stack.offset(a.wrapping_add(idx
                                                                                      as
                                                                                      libc::c_uint)
                                                                       as
                                                                       isize)).value.i
                                            = 0i32 as mrb_int;
                                        idx += 1
                                    }
                                }
                                mrb_gc_arena_restore(mrb, ai);
                                current_block = 7149356873433890176;
                                continue 'c_4688 ;
                            }
                            6985669818981290717 => {
                                let mut ary_2: mrb_value =
                                    *(*(*mrb).c).stack.offset(a as isize);
                                if ary_2.tt as libc::c_uint ==
                                       MRB_TT_ARRAY as libc::c_int as
                                           libc::c_uint {
                                    ary_2 =
                                        mrb_ary_new_from_values(mrb,
                                                                if 0 !=
                                                                       (*(ary_2.value.p
                                                                              as
                                                                              *mut RArray)).flags()
                                                                           as
                                                                           libc::c_int
                                                                           &
                                                                           7i32
                                                                   {
                                                                    (((*(ary_2.value.p
                                                                             as
                                                                             *mut RArray)).flags()
                                                                          as
                                                                          libc::c_int
                                                                          &
                                                                          7i32)
                                                                         -
                                                                         1i32)
                                                                        as
                                                                        mrb_int
                                                                } else {
                                                                    (*(ary_2.value.p
                                                                           as
                                                                           *mut RArray)).as_0.heap.len
                                                                },
                                                                if 0 !=
                                                                       (*(ary_2.value.p
                                                                              as
                                                                              *mut RArray)).flags()
                                                                           as
                                                                           libc::c_int
                                                                           &
                                                                           7i32
                                                                   {
                                                                    &mut (*(ary_2.value.p
                                                                                as
                                                                                *mut RArray)).as_0
                                                                        as
                                                                        *mut C2RustUnnamed_5
                                                                        as
                                                                        *mut mrb_value
                                                                } else {
                                                                    (*(ary_2.value.p
                                                                           as
                                                                           *mut RArray)).as_0.heap.ptr
                                                                })
                                } else {
                                    ary_2 =
                                        mrb_ary_new_from_values(mrb,
                                                                1i32 as
                                                                    mrb_int,
                                                                &mut ary_2)
                                }
                                *(*(*mrb).c).stack.offset(a as isize) = ary_2;
                                current_block = 7149356873433890176;
                                continue 'c_4688 ;
                            }
                            15951340130516588981 => {
                                let mut splat: mrb_value =
                                    mrb_ary_splat(mrb,
                                                  *(*(*mrb).c).stack.offset(a.wrapping_add(1i32
                                                                                               as
                                                                                               libc::c_uint)
                                                                                as
                                                                                isize));
                                mrb_ary_concat(mrb,
                                               *(*(*mrb).c).stack.offset(a as
                                                                             isize),
                                               splat);
                                mrb_gc_arena_restore(mrb, ai);
                                current_block = 7149356873433890176;
                                continue 'c_4688 ;
                            }
                            15590445366052810487 => {
                                let mut v_2: mrb_value =
                                    mrb_ary_new_from_values(mrb, c as mrb_int,
                                                            &mut *(*(*mrb).c).stack.offset(b
                                                                                               as
                                                                                               isize));
                                *(*(*mrb).c).stack.offset(a as isize) = v_2;
                                mrb_gc_arena_restore(mrb, ai);
                                current_block = 7149356873433890176;
                                continue 'c_4688 ;
                            }
                            5334851501950231651 => {
                                let mut v_1: mrb_value =
                                    mrb_ary_new_from_values(mrb, b as mrb_int,
                                                            &mut *(*(*mrb).c).stack.offset(a
                                                                                               as
                                                                                               isize));
                                *(*(*mrb).c).stack.offset(a as isize) = v_1;
                                mrb_gc_arena_restore(mrb, ai);
                                current_block = 7149356873433890176;
                                continue 'c_4688 ;
                            }
                            10264557722339789830 => {
                                if 0 !=
                                       mrb_obj_eq(mrb,
                                                  *(*(*mrb).c).stack.offset(a
                                                                                as
                                                                                isize),
                                                  *(*(*mrb).c).stack.offset(a.wrapping_add(1i32
                                                                                               as
                                                                                               libc::c_uint)
                                                                                as
                                                                                isize))
                                   {
                                    (*(*(*mrb).c).stack.offset(a as isize)).tt
                                        = MRB_TT_TRUE;
                                    (*(*(*mrb).c).stack.offset(a as
                                                                   isize)).value.i
                                        = 1i32 as mrb_int;
                                    current_block = 7149356873433890176;
                                    continue 'c_4688 ;
                                } else {
                                    result = 0;
                                    match ((*(*(*mrb).c).stack.offset(a as
                                                                          isize)).tt
                                               as uint16_t as libc::c_int) <<
                                              8i32 |
                                              (*(*(*mrb).c).stack.offset(a.wrapping_add(1i32
                                                                                            as
                                                                                            libc::c_uint)
                                                                             as
                                                                             isize)).tt
                                                  as uint16_t as libc::c_int &
                                                  0xffi32 {
                                        771 => {
                                            current_block =
                                                5381297608968995174;
                                            break ;
                                        }
                                        774 => {
                                            current_block =
                                                7679224362344455344;
                                            break ;
                                        }
                                        1539 => {
                                            current_block =
                                                15176476975632920360;
                                            break ;
                                        }
                                        1542 => {
                                            current_block =
                                                3733621521558651155;
                                            break ;
                                        }
                                        _ => {
                                            current_block =
                                                14455231216035570027;
                                            break ;
                                        }
                                    }
                                }
                            }
                            1767406316152665813 => {
                                match (*(*(*mrb).c).stack.offset(a as
                                                                     isize)).tt
                                          as libc::c_uint {
                                    3 => {
                                        let mut x_4: mrb_int =
                                            (*(*(*mrb).c).stack.offset(a as
                                                                           isize)).value.i;
                                        let mut y_4: mrb_int = b as mrb_int;
                                        let mut z_13: mrb_int = 0;
                                        if 0 !=
                                               mrb_int_sub_overflow(x_4, y_4,
                                                                    &mut z_13)
                                           {
                                            (*(*(*mrb).c).stack.offset(a as
                                                                           isize)).tt
                                                = MRB_TT_FLOAT;
                                            (*(*(*mrb).c).stack.offset(a as
                                                                           isize)).value.f
                                                =
                                                x_4 as mrb_float -
                                                    y_4 as mrb_float
                                        } else {
                                            (*(*(*mrb).c).stack.offset(a as
                                                                           isize)).tt
                                                = MRB_TT_FIXNUM;
                                            (*(*(*mrb).c).stack.offset(a as
                                                                           isize)).value.i
                                                = z_13
                                        }
                                        current_block = 7149356873433890176;
                                        continue 'c_4688 ;
                                    }
                                    6 => {
                                        let mut z_14: mrb_float =
                                            (*(*(*mrb).c).stack.offset(a as
                                                                           isize)).value.f
                                                -
                                                b as libc::c_int as
                                                    libc::c_double;
                                        (*(*(*mrb).c).stack.offset(a as
                                                                       isize)).tt
                                            = MRB_TT_FLOAT;
                                        (*(*(*mrb).c).stack.offset(a as
                                                                       isize)).value.f
                                            = z_14;
                                        current_block = 7149356873433890176;
                                        continue 'c_4688 ;
                                    }
                                    _ => {
                                        (*(*(*mrb).c).stack.offset(a.wrapping_add(1i32
                                                                                      as
                                                                                      libc::c_uint)
                                                                       as
                                                                       isize)).tt
                                            = MRB_TT_FIXNUM;
                                        (*(*(*mrb).c).stack.offset(a.wrapping_add(1i32
                                                                                      as
                                                                                      libc::c_uint)
                                                                       as
                                                                       isize)).value.i
                                            = b as mrb_int;
                                        c = 1i32 as uint8_t;
                                        mid =
                                            mrb_intern_static(mrb,
                                                              b"-\x00" as
                                                                  *const u8 as
                                                                  *const libc::c_char,
                                                              (::std::mem::size_of::<[libc::c_char; 2]>()
                                                                   as
                                                                   libc::c_ulong).wrapping_sub(1i32
                                                                                                   as
                                                                                                   libc::c_ulong));
                                        current_block = 4800884466390615302;
                                        break ;
                                    }
                                }
                            }
                            4396809944207673246 => {
                                match (*(*(*mrb).c).stack.offset(a as
                                                                     isize)).tt
                                          as libc::c_uint {
                                    3 => {
                                        let mut x_3: mrb_int =
                                            (*(*(*mrb).c).stack.offset(a as
                                                                           isize)).value.i;
                                        let mut y_3: mrb_int = b as mrb_int;
                                        let mut z_11: mrb_int = 0;
                                        if 0 !=
                                               mrb_int_add_overflow(x_3, y_3,
                                                                    &mut z_11)
                                           {
                                            (*(*(*mrb).c).stack.offset(a as
                                                                           isize)).tt
                                                = MRB_TT_FLOAT;
                                            (*(*(*mrb).c).stack.offset(a as
                                                                           isize)).value.f
                                                =
                                                x_3 as mrb_float +
                                                    y_3 as mrb_float
                                        } else {
                                            (*(*(*mrb).c).stack.offset(a as
                                                                           isize)).tt
                                                = MRB_TT_FIXNUM;
                                            (*(*(*mrb).c).stack.offset(a as
                                                                           isize)).value.i
                                                = z_11
                                        }
                                        current_block = 7149356873433890176;
                                        continue 'c_4688 ;
                                    }
                                    6 => {
                                        let mut z_12: mrb_float =
                                            (*(*(*mrb).c).stack.offset(a as
                                                                           isize)).value.f
                                                +
                                                b as libc::c_int as
                                                    libc::c_double;
                                        (*(*(*mrb).c).stack.offset(a as
                                                                       isize)).tt
                                            = MRB_TT_FLOAT;
                                        (*(*(*mrb).c).stack.offset(a as
                                                                       isize)).value.f
                                            = z_12;
                                        current_block = 7149356873433890176;
                                        continue 'c_4688 ;
                                    }
                                    _ => {
                                        (*(*(*mrb).c).stack.offset(a.wrapping_add(1i32
                                                                                      as
                                                                                      libc::c_uint)
                                                                       as
                                                                       isize)).tt
                                            = MRB_TT_FIXNUM;
                                        (*(*(*mrb).c).stack.offset(a.wrapping_add(1i32
                                                                                      as
                                                                                      libc::c_uint)
                                                                       as
                                                                       isize)).value.i
                                            = b as mrb_int;
                                        c = 1i32 as uint8_t;
                                        mid =
                                            mrb_intern_static(mrb,
                                                              b"+\x00" as
                                                                  *const u8 as
                                                                  *const libc::c_char,
                                                              (::std::mem::size_of::<[libc::c_char; 2]>()
                                                                   as
                                                                   libc::c_ulong).wrapping_sub(1i32
                                                                                                   as
                                                                                                   libc::c_ulong));
                                        current_block = 4800884466390615302;
                                        break ;
                                    }
                                }
                            }
                            2558910614333069445 => {
                                match ((*(*(*mrb).c).stack.offset(a as
                                                                      isize)).tt
                                           as uint16_t as libc::c_int) << 8i32
                                          |
                                          (*(*(*mrb).c).stack.offset(a.wrapping_add(1i32
                                                                                        as
                                                                                        libc::c_uint)
                                                                         as
                                                                         isize)).tt
                                              as uint16_t as libc::c_int &
                                              0xffi32 {
                                    771 => {
                                        let mut x_1: mrb_int =
                                            (*(*(*mrb).c).stack.offset(a as
                                                                           isize)).value.i;
                                        let mut y_1: mrb_int =
                                            (*(*(*mrb).c).stack.offset(a.wrapping_add(1i32
                                                                                          as
                                                                                          libc::c_uint)
                                                                           as
                                                                           isize)).value.i;
                                        let mut z_7: mrb_int = 0;
                                        if 0 !=
                                               mrb_int_mul_overflow(x_1, y_1,
                                                                    &mut z_7)
                                           {
                                            (*(*(*mrb).c).stack.offset(a as
                                                                           isize)).tt
                                                = MRB_TT_FLOAT;
                                            (*(*(*mrb).c).stack.offset(a as
                                                                           isize)).value.f
                                                =
                                                x_1 as mrb_float *
                                                    y_1 as mrb_float
                                        } else {
                                            (*(*(*mrb).c).stack.offset(a as
                                                                           isize)).tt
                                                = MRB_TT_FIXNUM;
                                            (*(*(*mrb).c).stack.offset(a as
                                                                           isize)).value.i
                                                = z_7
                                        }
                                        current_block = 7149356873433890176;
                                        continue 'c_4688 ;
                                    }
                                    774 => {
                                        let mut z_8: mrb_float =
                                            (*(*(*mrb).c).stack.offset(a as
                                                                           isize)).value.i
                                                as libc::c_double *
                                                (*(*(*mrb).c).stack.offset(a.wrapping_add(1i32
                                                                                              as
                                                                                              libc::c_uint)
                                                                               as
                                                                               isize)).value.f;
                                        (*(*(*mrb).c).stack.offset(a as
                                                                       isize)).tt
                                            = MRB_TT_FLOAT;
                                        (*(*(*mrb).c).stack.offset(a as
                                                                       isize)).value.f
                                            = z_8;
                                        current_block = 7149356873433890176;
                                        continue 'c_4688 ;
                                    }
                                    1539 => {
                                        let mut z_9: mrb_float =
                                            (*(*(*mrb).c).stack.offset(a as
                                                                           isize)).value.f
                                                *
                                                (*(*(*mrb).c).stack.offset(a.wrapping_add(1i32
                                                                                              as
                                                                                              libc::c_uint)
                                                                               as
                                                                               isize)).value.i
                                                    as libc::c_double;
                                        (*(*(*mrb).c).stack.offset(a as
                                                                       isize)).tt
                                            = MRB_TT_FLOAT;
                                        (*(*(*mrb).c).stack.offset(a as
                                                                       isize)).value.f
                                            = z_9;
                                        current_block = 7149356873433890176;
                                        continue 'c_4688 ;
                                    }
                                    1542 => {
                                        let mut z_10: mrb_float =
                                            (*(*(*mrb).c).stack.offset(a as
                                                                           isize)).value.f
                                                *
                                                (*(*(*mrb).c).stack.offset(a.wrapping_add(1i32
                                                                                              as
                                                                                              libc::c_uint)
                                                                               as
                                                                               isize)).value.f;
                                        (*(*(*mrb).c).stack.offset(a as
                                                                       isize)).tt
                                            = MRB_TT_FLOAT;
                                        (*(*(*mrb).c).stack.offset(a as
                                                                       isize)).value.f
                                            = z_10;
                                        current_block = 7149356873433890176;
                                        continue 'c_4688 ;
                                    }
                                    _ => {
                                        c = 1i32 as uint8_t;
                                        mid =
                                            mrb_intern_static(mrb,
                                                              b"*\x00" as
                                                                  *const u8 as
                                                                  *const libc::c_char,
                                                              (::std::mem::size_of::<[libc::c_char; 2]>()
                                                                   as
                                                                   libc::c_ulong).wrapping_sub(1i32
                                                                                                   as
                                                                                                   libc::c_ulong));
                                        current_block = 4800884466390615302;
                                        break ;
                                    }
                                }
                            }
                            10653245833193402520 => {
                                match ((*(*(*mrb).c).stack.offset(a as
                                                                      isize)).tt
                                           as uint16_t as libc::c_int) << 8i32
                                          |
                                          (*(*(*mrb).c).stack.offset(a.wrapping_add(1i32
                                                                                        as
                                                                                        libc::c_uint)
                                                                         as
                                                                         isize)).tt
                                              as uint16_t as libc::c_int &
                                              0xffi32 {
                                    771 => {
                                        let mut x_0: mrb_int =
                                            (*(*(*mrb).c).stack.offset(a as
                                                                           isize)).value.i;
                                        let mut y_0: mrb_int =
                                            (*(*(*mrb).c).stack.offset(a.wrapping_add(1i32
                                                                                          as
                                                                                          libc::c_uint)
                                                                           as
                                                                           isize)).value.i;
                                        let mut z_3: mrb_int = 0;
                                        if 0 !=
                                               mrb_int_sub_overflow(x_0, y_0,
                                                                    &mut z_3)
                                           {
                                            (*(*(*mrb).c).stack.offset(a as
                                                                           isize)).tt
                                                = MRB_TT_FLOAT;
                                            (*(*(*mrb).c).stack.offset(a as
                                                                           isize)).value.f
                                                =
                                                x_0 as mrb_float -
                                                    y_0 as mrb_float
                                        } else {
                                            (*(*(*mrb).c).stack.offset(a as
                                                                           isize)).tt
                                                = MRB_TT_FIXNUM;
                                            (*(*(*mrb).c).stack.offset(a as
                                                                           isize)).value.i
                                                = z_3
                                        }
                                        current_block = 7149356873433890176;
                                        continue 'c_4688 ;
                                    }
                                    774 => {
                                        let mut z_4: mrb_float =
                                            (*(*(*mrb).c).stack.offset(a as
                                                                           isize)).value.i
                                                as libc::c_double -
                                                (*(*(*mrb).c).stack.offset(a.wrapping_add(1i32
                                                                                              as
                                                                                              libc::c_uint)
                                                                               as
                                                                               isize)).value.f;
                                        (*(*(*mrb).c).stack.offset(a as
                                                                       isize)).tt
                                            = MRB_TT_FLOAT;
                                        (*(*(*mrb).c).stack.offset(a as
                                                                       isize)).value.f
                                            = z_4;
                                        current_block = 7149356873433890176;
                                        continue 'c_4688 ;
                                    }
                                    1539 => {
                                        let mut z_5: mrb_float =
                                            (*(*(*mrb).c).stack.offset(a as
                                                                           isize)).value.f
                                                -
                                                (*(*(*mrb).c).stack.offset(a.wrapping_add(1i32
                                                                                              as
                                                                                              libc::c_uint)
                                                                               as
                                                                               isize)).value.i
                                                    as libc::c_double;
                                        (*(*(*mrb).c).stack.offset(a as
                                                                       isize)).tt
                                            = MRB_TT_FLOAT;
                                        (*(*(*mrb).c).stack.offset(a as
                                                                       isize)).value.f
                                            = z_5;
                                        current_block = 7149356873433890176;
                                        continue 'c_4688 ;
                                    }
                                    1542 => {
                                        let mut z_6: mrb_float =
                                            (*(*(*mrb).c).stack.offset(a as
                                                                           isize)).value.f
                                                -
                                                (*(*(*mrb).c).stack.offset(a.wrapping_add(1i32
                                                                                              as
                                                                                              libc::c_uint)
                                                                               as
                                                                               isize)).value.f;
                                        (*(*(*mrb).c).stack.offset(a as
                                                                       isize)).tt
                                            = MRB_TT_FLOAT;
                                        (*(*(*mrb).c).stack.offset(a as
                                                                       isize)).value.f
                                            = z_6;
                                        current_block = 7149356873433890176;
                                        continue 'c_4688 ;
                                    }
                                    _ => {
                                        c = 1i32 as uint8_t;
                                        mid =
                                            mrb_intern_static(mrb,
                                                              b"-\x00" as
                                                                  *const u8 as
                                                                  *const libc::c_char,
                                                              (::std::mem::size_of::<[libc::c_char; 2]>()
                                                                   as
                                                                   libc::c_ulong).wrapping_sub(1i32
                                                                                                   as
                                                                                                   libc::c_ulong));
                                        current_block = 4800884466390615302;
                                        break ;
                                    }
                                }
                            }
                            16140473650108643456 => {
                                match ((*(*(*mrb).c).stack.offset(a as
                                                                      isize)).tt
                                           as uint16_t as libc::c_int) << 8i32
                                          |
                                          (*(*(*mrb).c).stack.offset(a.wrapping_add(1i32
                                                                                        as
                                                                                        libc::c_uint)
                                                                         as
                                                                         isize)).tt
                                              as uint16_t as libc::c_int &
                                              0xffi32 {
                                    771 => {
                                        let mut x: mrb_int =
                                            (*(*(*mrb).c).stack.offset(a as
                                                                           isize)).value.i;
                                        let mut y: mrb_int =
                                            (*(*(*mrb).c).stack.offset(a.wrapping_add(1i32
                                                                                          as
                                                                                          libc::c_uint)
                                                                           as
                                                                           isize)).value.i;
                                        let mut z: mrb_int = 0;
                                        if 0 !=
                                               mrb_int_add_overflow(x, y,
                                                                    &mut z) {
                                            (*(*(*mrb).c).stack.offset(a as
                                                                           isize)).tt
                                                = MRB_TT_FLOAT;
                                            (*(*(*mrb).c).stack.offset(a as
                                                                           isize)).value.f
                                                =
                                                x as mrb_float +
                                                    y as mrb_float
                                        } else {
                                            (*(*(*mrb).c).stack.offset(a as
                                                                           isize)).tt
                                                = MRB_TT_FIXNUM;
                                            (*(*(*mrb).c).stack.offset(a as
                                                                           isize)).value.i
                                                = z
                                        }
                                        current_block = 7149356873433890176;
                                        continue 'c_4688 ;
                                    }
                                    774 => {
                                        let mut z_0: mrb_float =
                                            (*(*(*mrb).c).stack.offset(a as
                                                                           isize)).value.i
                                                as libc::c_double +
                                                (*(*(*mrb).c).stack.offset(a.wrapping_add(1i32
                                                                                              as
                                                                                              libc::c_uint)
                                                                               as
                                                                               isize)).value.f;
                                        (*(*(*mrb).c).stack.offset(a as
                                                                       isize)).tt
                                            = MRB_TT_FLOAT;
                                        (*(*(*mrb).c).stack.offset(a as
                                                                       isize)).value.f
                                            = z_0;
                                        current_block = 7149356873433890176;
                                        continue 'c_4688 ;
                                    }
                                    1539 => {
                                        let mut z_1: mrb_float =
                                            (*(*(*mrb).c).stack.offset(a as
                                                                           isize)).value.f
                                                +
                                                (*(*(*mrb).c).stack.offset(a.wrapping_add(1i32
                                                                                              as
                                                                                              libc::c_uint)
                                                                               as
                                                                               isize)).value.i
                                                    as libc::c_double;
                                        (*(*(*mrb).c).stack.offset(a as
                                                                       isize)).tt
                                            = MRB_TT_FLOAT;
                                        (*(*(*mrb).c).stack.offset(a as
                                                                       isize)).value.f
                                            = z_1;
                                        current_block = 7149356873433890176;
                                        continue 'c_4688 ;
                                    }
                                    1542 => {
                                        let mut z_2: mrb_float =
                                            (*(*(*mrb).c).stack.offset(a as
                                                                           isize)).value.f
                                                +
                                                (*(*(*mrb).c).stack.offset(a.wrapping_add(1i32
                                                                                              as
                                                                                              libc::c_uint)
                                                                               as
                                                                               isize)).value.f;
                                        (*(*(*mrb).c).stack.offset(a as
                                                                       isize)).tt
                                            = MRB_TT_FLOAT;
                                        (*(*(*mrb).c).stack.offset(a as
                                                                       isize)).value.f
                                            = z_2;
                                        current_block = 7149356873433890176;
                                        continue 'c_4688 ;
                                    }
                                    4112 => {
                                        *(*(*mrb).c).stack.offset(a as isize)
                                            =
                                            mrb_str_plus(mrb,
                                                         *(*(*mrb).c).stack.offset(a
                                                                                       as
                                                                                       isize),
                                                         *(*(*mrb).c).stack.offset(a.wrapping_add(1i32
                                                                                                      as
                                                                                                      libc::c_uint)
                                                                                       as
                                                                                       isize));
                                        mrb_gc_arena_restore(mrb, ai);
                                        current_block = 7149356873433890176;
                                        continue 'c_4688 ;
                                    }
                                    _ => {
                                        c = 1i32 as uint8_t;
                                        mid =
                                            mrb_intern_static(mrb,
                                                              b"+\x00" as
                                                                  *const u8 as
                                                                  *const libc::c_char,
                                                              (::std::mem::size_of::<[libc::c_char; 2]>()
                                                                   as
                                                                   libc::c_ulong).wrapping_sub(1i32
                                                                                                   as
                                                                                                   libc::c_ulong));
                                        current_block = 4800884466390615302;
                                        break ;
                                    }
                                }
                            }
                            2994422791481536047 => {
                                m1_1 = b as libc::c_int >> 11i32 & 0x3fi32;
                                r_1 = b as libc::c_int >> 10i32 & 0x1i32;
                                m2_1 = b as libc::c_int >> 5i32 & 0x1fi32;
                                kd_1 = b as libc::c_int >> 4i32 & 0x1i32;
                                lv_0 = b as libc::c_int >> 0i32 & 0xfi32;
                                stack_0 = 0 as *mut mrb_value;
                                if lv_0 == 0i32 {
                                    current_block = 14979696880445242837;
                                    break ;
                                } else {
                                    current_block = 7975968915446637229;
                                    break ;
                                }
                            }
                            4663630632216779857 => {
                                let mut kdict_2: mrb_value =
                                    *(*(*mrb).c).stack.offset((*(*(*mrb).c).ci).argc
                                                                  as isize);
                                if !(kdict_2.tt as libc::c_uint ==
                                         MRB_TT_HASH as libc::c_int as
                                             libc::c_uint &&
                                         0 == mrb_hash_empty_p(mrb, kdict_2))
                                   {
                                    current_block = 7149356873433890176;
                                    continue 'c_4688 ;
                                }
                                let mut keys: mrb_value =
                                    mrb_hash_keys(mrb, kdict_2);
                                let mut key1: mrb_value =
                                    *if 0 !=
                                            (*(keys.value.p as
                                                   *mut RArray)).flags() as
                                                libc::c_int & 7i32 {
                                         &mut (*(keys.value.p as
                                                     *mut RArray)).as_0 as
                                             *mut C2RustUnnamed_5 as
                                             *mut mrb_value
                                     } else {
                                         (*(keys.value.p as
                                                *mut RArray)).as_0.heap.ptr
                                     }.offset(0isize);
                                let mut str_0: mrb_value =
                                    mrb_format(mrb,
                                               b"unknown keyword: %v\x00" as
                                                   *const u8 as
                                                   *const libc::c_char, key1);
                                mrb_exc_set(mrb,
                                            mrb_exc_new_str(mrb,
                                                            mrb_exc_get(mrb,
                                                                        b"ArgumentError\x00"
                                                                            as
                                                                            *const u8
                                                                            as
                                                                            *const libc::c_char),
                                                            str_0));
                                current_block = 2687857153341325290;
                                continue 'c_4688 ;
                            }
                            1525421113118994726 => {
                                let mut k_0: mrb_value =
                                    mrb_symbol_value(*syms.offset(b as
                                                                      isize));
                                let mut kdict_1: mrb_value =
                                    *(*(*mrb).c).stack.offset((*(*(*mrb).c).ci).argc
                                                                  as isize);
                                let mut key_p: mrb_bool = 0i32 as mrb_bool;
                                if kdict_1.tt as libc::c_uint ==
                                       MRB_TT_HASH as libc::c_int as
                                           libc::c_uint {
                                    key_p = mrb_hash_key_p(mrb, kdict_1, k_0)
                                }
                                *(*(*mrb).c).stack.offset(a as isize) =
                                    mrb_bool_value(key_p);
                                current_block = 7149356873433890176;
                                continue 'c_4688 ;
                            }
                            14765652484648187635 => {
                                k =
                                    mrb_symbol_value(*syms.offset(b as
                                                                      isize));
                                kdict_0 =
                                    *(*(*mrb).c).stack.offset((*(*(*mrb).c).ci).argc
                                                                  as isize);
                                if !(kdict_0.tt as libc::c_uint ==
                                         MRB_TT_HASH as libc::c_int as
                                             libc::c_uint) ||
                                       0 == mrb_hash_key_p(mrb, kdict_0, k) {
                                    current_block = 777853614834280397;
                                    break ;
                                } else {
                                    current_block = 14871291313029355442;
                                    break ;
                                }
                            }
                            15803687294481920802 => {
                                m1_0 =
                                    (a >> 18i32 & 0x1fi32 as libc::c_uint) as
                                        libc::c_int;
                                o =
                                    (a >> 13i32 & 0x1fi32 as libc::c_uint) as
                                        libc::c_int;
                                r_0 =
                                    (a >> 12i32 & 0x1i32 as libc::c_uint) as
                                        libc::c_int;
                                m2_0 =
                                    (a >> 7i32 & 0x1fi32 as libc::c_uint) as
                                        libc::c_int;
                                kd_0 =
                                    if a >> 2i32 & 0x1fi32 as libc::c_uint >
                                           0i32 as libc::c_uint ||
                                           0 !=
                                               a &
                                                   (1i32 << 1i32) as
                                                       libc::c_uint {
                                        1i32
                                    } else { 0i32 };
                                /* unused
      int b  = MRB_ASPEC_BLOCK(a);
      */
                                argc_1 = (*(*(*mrb).c).ci).argc;
                                argv = (*(*mrb).c).stack.offset(1isize);
                                argv0 = argv;
                                len_0 = m1_0 + o + r_0 + m2_0;
                                blk_pos = len_0 + kd_0 + 1i32;
                                blk_1 =
                                    &mut *argv.offset((if argc_1 < 0i32 {
                                                           1i32
                                                       } else { argc_1 }) as
                                                          isize) as
                                        *mut mrb_value;
                                kdict =
                                    mrb_value{value: C2RustUnnamed_0{f: 0.,},
                                              tt: MRB_TT_FALSE,};
                                kargs = kd_0;
                                /* arguments is passed with Array */
                                if argc_1 < 0i32 {
                                    let mut ary_0: *mut RArray =
                                        (*(*(*mrb).c).stack.offset(1isize)).value.p
                                            as *mut RArray;
                                    argv =
                                        if 0 !=
                                               (*ary_0).flags() as libc::c_int
                                                   & 7i32 {
                                            &mut (*ary_0).as_0 as
                                                *mut C2RustUnnamed_5 as
                                                *mut mrb_value
                                        } else { (*ary_0).as_0.heap.ptr };
                                    argc_1 =
                                        (if 0 !=
                                                (*ary_0).flags() as
                                                    libc::c_int & 7i32 {
                                             (((*ary_0).flags() as libc::c_int
                                                   & 7i32) - 1i32) as mrb_int
                                         } else { (*ary_0).as_0.heap.len }) as
                                            libc::c_int;
                                    mrb_gc_protect(mrb,
                                                   *(*(*mrb).c).stack.offset(1isize));
                                }
                                /* strict argument check */
                                if !(*(*(*mrb).c).ci).proc_0.is_null() &&
                                       (*(*(*(*mrb).c).ci).proc_0).flags() as
                                           libc::c_int & 256i32 != 0i32 {
                                    current_block = 14951089859709286127;
                                    break ;
                                } else {
                                    current_block = 1846749606198814734;
                                    break ;
                                }
                            }
                            10763054988551342546 => {
                                m1 = b as libc::c_int >> 11i32 & 0x3fi32;
                                r = b as libc::c_int >> 10i32 & 0x1i32;
                                m2 = b as libc::c_int >> 5i32 & 0x1fi32;
                                kd = b as libc::c_int >> 4i32 & 0x1i32;
                                lv = b as libc::c_int >> 0i32 & 0xfi32;
                                stack = 0 as *mut mrb_value;
                                if (*(*(*mrb).c).ci).mid ==
                                       0i32 as libc::c_uint ||
                                       (*(*(*mrb).c).ci).target_class.is_null()
                                   {
                                    current_block = 15236292558918277616;
                                    break ;
                                } else {
                                    current_block = 9640155021185875683;
                                    break ;
                                }
                            }
                            9607585333290054341 => {
                                argc_0 =
                                    if b as libc::c_int == 127i32 {
                                        -1i32
                                    } else { b as libc::c_int };
                                bidx_2 =
                                    (if argc_0 < 0i32 {
                                         a.wrapping_add(2i32 as libc::c_uint)
                                     } else {
                                         a.wrapping_add(b as
                                                            libc::c_uint).wrapping_add(1i32
                                                                                           as
                                                                                           libc::c_uint)
                                     }) as libc::c_int;
                                m_1 =
                                    mrb_method_t{func_p: 0,
                                                 c2rust_unnamed:
                                                     C2RustUnnamed{proc_0:
                                                                       0 as
                                                                           *mut RProc,},};
                                cls_0 = 0 as *mut RClass;
                                ci_2 = (*(*mrb).c).ci;
                                recv_1 =
                                    mrb_value{value: C2RustUnnamed_0{f: 0.,},
                                              tt: MRB_TT_FALSE,};
                                blk_0 =
                                    mrb_value{value: C2RustUnnamed_0{f: 0.,},
                                              tt: MRB_TT_FALSE,};
                                mid_1 = (*ci_2).mid;
                                target_class_0 =
                                    if (*(*ci_2).proc_0).flags() as
                                           libc::c_int & 1024i32 != 0i32 {
                                        (*(*(*ci_2).proc_0).e.env).c
                                    } else {
                                        (*(*ci_2).proc_0).e.target_class
                                    };
                                if mid_1 == 0i32 as libc::c_uint ||
                                       target_class_0.is_null() {
                                    current_block = 18074041116490187063;
                                    break ;
                                } else {
                                    current_block = 7296667520727038133;
                                    break ;
                                }
                            }
                            13698748673986354976 => {
                                let mut ci_1: *mut mrb_callinfo =
                                    0 as *mut mrb_callinfo;
                                let mut recv_0: mrb_value =
                                    *(*(*mrb).c).stack.offset(0isize);
                                let mut m_0: *mut RProc =
                                    recv_0.value.p as *mut RProc;
                                /* replace callinfo */
                                ci_1 = (*(*mrb).c).ci;
                                (*ci_1).target_class =
                                    if (*m_0).flags() as libc::c_int & 1024i32
                                           != 0i32 {
                                        (*(*m_0).e.env).c
                                    } else { (*m_0).e.target_class };
                                (*ci_1).proc_0 = m_0;
                                if (*m_0).flags() as libc::c_int & 1024i32 !=
                                       0i32 {
                                    let mut mid_0: mrb_sym = 0;
                                    let mut e_2: *mut REnv =
                                        if (*m_0).flags() as libc::c_int &
                                               1024i32 != 0i32 {
                                            (*m_0).e.env
                                        } else { 0 as *mut REnv };
                                    mid_0 = (*e_2).mid;
                                    if 0 != mid_0 { (*ci_1).mid = mid_0 }
                                    if (*e_2).stack.is_null() {
                                        (*e_2).stack = (*(*mrb).c).stack
                                    }
                                }
                                /* prepare stack */
                                if (*m_0).flags() as libc::c_int & 128i32 !=
                                       0i32 {
                                    recv_0 =
                                        (*m_0).body.func.expect("non-null function pointer")(mrb,
                                                                                             recv_0);
                                    mrb_gc_arena_restore(mrb, ai);
                                    mrb_gc_arena_shrink(mrb, ai);
                                    if !(*mrb).exc.is_null() {
                                        current_block = 2687857153341325290;
                                        continue 'c_4688 ;
                                    }
                                    /* pop stackpos */
                                    ci_1 = (*(*mrb).c).ci;
                                    (*(*mrb).c).stack = (*ci_1).stackent;
                                    *(*(*mrb).c).stack.offset((*ci_1).acc as
                                                                  isize) =
                                        recv_0;
                                    pc = (*ci_1).pc;
                                    cipop(mrb);
                                    irep =
                                        (*(*(*(*mrb).c).ci).proc_0).body.irep;
                                    pool = (*irep).pool;
                                    syms = (*irep).syms;
                                    current_block = 7149356873433890176;
                                    continue 'c_4688 ;
                                } else {
                                    /* setup environment for calling method */
                                    proc_0 = m_0;
                                    irep = (*m_0).body.irep;
                                    if irep.is_null() {
                                        *(*(*mrb).c).stack.offset(0isize) =
                                            mrb_nil_value();
                                        a = 0i32 as uint32_t;
                                        c = 0i32 as uint8_t;
                                        current_block = 14547135203946798640;
                                    } else {
                                        pool = (*irep).pool;
                                        syms = (*irep).syms;
                                        mrb_stack_extend(mrb,
                                                         (*irep).nregs as
                                                             mrb_int);
                                        if (*ci_1).argc < 0i32 {
                                            if (*irep).nregs as libc::c_int >
                                                   3i32 {
                                                stack_clear((*(*mrb).c).stack.offset(3isize),
                                                            ((*irep).nregs as
                                                                 libc::c_int -
                                                                 3i32) as
                                                                size_t);
                                            }
                                        } else if (*ci_1).argc + 2i32 <
                                                      (*irep).nregs as
                                                          libc::c_int {
                                            stack_clear((*(*mrb).c).stack.offset((*ci_1).argc
                                                                                     as
                                                                                     isize).offset(2isize),
                                                        ((*irep).nregs as
                                                             libc::c_int -
                                                             (*ci_1).argc -
                                                             2i32) as size_t);
                                        }
                                        if (*m_0).flags() as libc::c_int &
                                               1024i32 != 0i32 {
                                            *(*(*mrb).c).stack.offset(0isize)
                                                =
                                                *(*if (*m_0).flags() as
                                                          libc::c_int &
                                                          1024i32 != 0i32 {
                                                       (*m_0).e.env
                                                   } else {
                                                       0 as *mut REnv
                                                   }).stack.offset(0isize)
                                        }
                                        pc = (*irep).iseq;
                                        current_block = 7149356873433890176;
                                        continue 'c_4688 ;
                                    }
                                }
                            }
                            8592218650707875588 => {
                                let mut ci: *mut mrb_callinfo =
                                    (*(*mrb).c).ci;
                                let mut n: libc::c_uint = 0;
                                let mut epos: libc::c_uint =
                                    (*ci).epos as libc::c_uint;
                                let mut self_0: mrb_value =
                                    *(*(*mrb).c).stack.offset(0isize);
                                let mut target_class: *mut RClass =
                                    (*ci).target_class;
                                if (*(*mrb).c).eidx as libc::c_uint <= epos {
                                    current_block = 7149356873433890176;
                                    continue 'c_4688 ;
                                }
                                if a >
                                       ((*(*mrb).c).eidx as libc::c_int as
                                            libc::c_uint).wrapping_sub(epos) {
                                    a =
                                        ((*(*mrb).c).eidx as
                                             libc::c_uint).wrapping_sub(epos)
                                }
                                n = 0i32 as libc::c_uint;
                                while n < a {
                                    let mut nregs: libc::c_int =
                                        (*irep).nregs as libc::c_int;
                                    proc_0 =
                                        *(*(*mrb).c).ensure.offset(epos.wrapping_add(n)
                                                                       as
                                                                       isize);
                                    let ref mut fresh128 =
                                        *(*(*mrb).c).ensure.offset(epos.wrapping_add(n)
                                                                       as
                                                                       isize);
                                    *fresh128 = 0 as *mut RProc;
                                    if !proc_0.is_null() {
                                        irep = (*proc_0).body.irep;
                                        ci = cipush(mrb);
                                        (*ci).mid =
                                            (*ci.offset(-1i32 as isize)).mid;
                                        (*ci).argc = 0i32;
                                        (*ci).proc_0 = proc_0;
                                        (*ci).stackent = (*(*mrb).c).stack;
                                        (*ci).target_class = target_class;
                                        (*ci).pc = pc;
                                        (*ci).acc = nregs;
                                        (*(*mrb).c).stack =
                                            (*(*mrb).c).stack.offset((*ci).acc
                                                                         as
                                                                         isize);
                                        mrb_stack_extend(mrb,
                                                         (*irep).nregs as
                                                             mrb_int);
                                        *(*(*mrb).c).stack.offset(0isize) =
                                            self_0;
                                        pc = (*irep).iseq
                                    }
                                    n = n.wrapping_add(1)
                                }
                                pool = (*irep).pool;
                                syms = (*irep).syms;
                                (*(*mrb).c).eidx = epos as uint16_t;
                                current_block = 7149356873433890176;
                                continue 'c_4688 ;
                            }
                            17178928522929307677 => {
                                p = 0 as *mut RProc;
                                p =
                                    mrb_closure_new(mrb,
                                                    *(*irep).reps.offset(a as
                                                                             isize));
                                /* check ensure stack */
                                if (*(*mrb).c).eidx as libc::c_int ==
                                       65535i32 - 1i32 {
                                    current_block = 11870793248822581739;
                                    break ;
                                } else {
                                    current_block = 16020430568569951085;
                                    break ;
                                }
                            }
                            16261244098005255386 => {
                                /* exc on stack */
                                exc_1 = *(*(*mrb).c).stack.offset(a as isize);
                                e_1 = *(*(*mrb).c).stack.offset(b as isize);
                                ec = 0 as *mut RClass;
                                match e_1.tt as libc::c_uint {
                                    9 | 10 => {
                                        current_block = 11989315111553324117;
                                        break ;
                                    }
                                    _ => {
                                        current_block = 13561616326120682545;
                                        break ;
                                    }
                                }
                            }
                            2722617303989997926 => {
                                let mut exc_0: mrb_value =
                                    mrb_obj_value((*mrb).exc as
                                                      *mut libc::c_void);
                                (*mrb).exc = 0 as *mut RObject;
                                *(*(*mrb).c).stack.offset(a as isize) = exc_0;
                                current_block = 7149356873433890176;
                                continue 'c_4688 ;
                            }
                            12913164539471242988 => {
                                /* check rescue stack */
                                if (*(*(*mrb).c).ci).ridx as libc::c_int ==
                                       65535i32 - 1i32 {
                                    let mut exc: mrb_value =
                                        mrb_exc_new_str(mrb,
                                                        mrb_exc_get(mrb,
                                                                    b"RuntimeError\x00"
                                                                        as
                                                                        *const u8
                                                                        as
                                                                        *const libc::c_char),
                                                        mrb_str_new_static(mrb,
                                                                           b"too many nested rescues\x00"
                                                                               as
                                                                               *const u8
                                                                               as
                                                                               *const libc::c_char,
                                                                           (::std::mem::size_of::<[libc::c_char; 24]>()
                                                                                as
                                                                                libc::c_ulong).wrapping_sub(1i32
                                                                                                                as
                                                                                                                libc::c_ulong)));
                                    mrb_exc_set(mrb, exc);
                                    current_block = 2687857153341325290;
                                    continue 'c_4688 ;
                                } else {
                                    /* expand rescue stack */
                                    if (*(*mrb).c).rsize as libc::c_int <=
                                           (*(*(*mrb).c).ci).ridx as
                                               libc::c_int {
                                        if (*(*mrb).c).rsize as libc::c_int ==
                                               0i32 {
                                            (*(*mrb).c).rsize =
                                                16i32 as uint16_t
                                        } else {
                                            (*(*mrb).c).rsize =
                                                ((*(*mrb).c).rsize as
                                                     libc::c_int * 2i32) as
                                                    uint16_t;
                                            if (*(*mrb).c).rsize as
                                                   libc::c_int <=
                                                   (*(*(*mrb).c).ci).ridx as
                                                       libc::c_int {
                                                (*(*mrb).c).rsize =
                                                    65535i32 as uint16_t
                                            }
                                        }
                                        (*(*mrb).c).rescue =
                                            mrb_realloc(mrb,
                                                        (*(*mrb).c).rescue as
                                                            *mut libc::c_void,
                                                        (::std::mem::size_of::<uint16_t>()
                                                             as
                                                             libc::c_ulong).wrapping_mul((*(*mrb).c).rsize
                                                                                             as
                                                                                             libc::c_ulong))
                                                as *mut uint16_t
                                    }
                                    /* push rescue stack */
                                    let fresh111 = (*(*(*mrb).c).ci).ridx;
                                    (*(*(*mrb).c).ci).ridx =
                                        (*(*(*mrb).c).ci).ridx.wrapping_add(1);
                                    *(*(*mrb).c).rescue.offset(fresh111 as
                                                                   isize) =
                                        a as uint16_t;
                                    current_block = 7149356873433890176;
                                    continue 'c_4688 ;
                                }
                            }
                            16263365153914704257 => {
                                let mut regs_a: *mut mrb_value =
                                    (*(*mrb).c).stack.offset(a as isize);
                                let mut e: *mut REnv =
                                    uvenv(mrb, c as libc::c_int);
                                if !e.is_null() &&
                                       (b as libc::c_longlong) <
                                           ((*e).flags() as libc::c_int &
                                                0x3ffi32) as mrb_int {
                                    *regs_a = *(*e).stack.offset(b as isize)
                                } else { *regs_a = mrb_nil_value() }
                                current_block = 7149356873433890176;
                                continue 'c_4688 ;
                            }
                            4001239642700071046 => {
                                let mut val_3: mrb_value =
                                    mrb_value{value: C2RustUnnamed_0{f: 0.,},
                                              tt: MRB_TT_FALSE,};
                                (*(*(*mrb).c).ci).err = pc0;
                                val_3 =
                                    mrb_const_get(mrb,
                                                  *(*(*mrb).c).stack.offset(a
                                                                                as
                                                                                isize),
                                                  *syms.offset(b as isize));
                                (*(*(*mrb).c).ci).err = 0 as *mut mrb_code;
                                *(*(*mrb).c).stack.offset(a as isize) = val_3;
                                current_block = 7149356873433890176;
                                continue 'c_4688 ;
                            }
                            13114814261106982490 => {
                                let mut val_2: mrb_value =
                                    mrb_value{value: C2RustUnnamed_0{f: 0.,},
                                              tt: MRB_TT_FALSE,};
                                let mut sym: mrb_sym =
                                    *syms.offset(b as isize);
                                (*(*(*mrb).c).ci).err = pc0;
                                val_2 = mrb_vm_const_get(mrb, sym);
                                (*(*(*mrb).c).ci).err = 0 as *mut mrb_code;
                                *(*(*mrb).c).stack.offset(a as isize) = val_2;
                                current_block = 7149356873433890176;
                                continue 'c_4688 ;
                            }
                            3906822848181906220 => {
                                let mut val_1: mrb_value =
                                    mrb_value{value: C2RustUnnamed_0{f: 0.,},
                                              tt: MRB_TT_FALSE,};
                                (*(*(*mrb).c).ci).err = pc0;
                                val_1 =
                                    mrb_vm_cv_get(mrb,
                                                  *syms.offset(b as isize));
                                (*(*(*mrb).c).ci).err = 0 as *mut mrb_code;
                                *(*(*mrb).c).stack.offset(a as isize) = val_1;
                                current_block = 7149356873433890176;
                                continue 'c_4688 ;
                            }
                            3106153520265210690 => {
                                let fresh438 = pc;
                                pc = pc.offset(1);
                                let mut insn_0: uint8_t = *fresh438;
                                match insn_0 as libc::c_int {
                                    0 => {
                                        current_block = 14576567515993809846;
                                    }
                                    1 => {
                                        pc = pc.offset(2isize);
                                        a =
                                            ((*pc.offset(-2isize).offset(0isize)
                                                  as libc::c_int) << 8i32 |
                                                 *pc.offset(-2isize).offset(1isize)
                                                     as libc::c_int) as
                                                uint32_t;
                                        /* R(a) = R(b) */
                                        pc = pc.offset(2isize);
                                        b =
                                            ((*pc.offset(-2isize).offset(0isize)
                                                  as libc::c_int) << 8i32 |
                                                 *pc.offset(-2isize).offset(1isize)
                                                     as libc::c_int) as
                                                uint16_t;
                                        current_block = 4488286894823169796;
                                    }
                                    2 => {
                                        pc = pc.offset(2isize);
                                        a =
                                            ((*pc.offset(-2isize).offset(0isize)
                                                  as libc::c_int) << 8i32 |
                                                 *pc.offset(-2isize).offset(1isize)
                                                     as libc::c_int) as
                                                uint32_t;
                                        /* R(a) = Pool(b) */
                                        pc = pc.offset(2isize);
                                        b =
                                            ((*pc.offset(-2isize).offset(0isize)
                                                  as libc::c_int) << 8i32 |
                                                 *pc.offset(-2isize).offset(1isize)
                                                     as libc::c_int) as
                                                uint16_t;
                                        current_block = 8845338526596852646;
                                    }
                                    3 => {
                                        pc = pc.offset(2isize);
                                        a =
                                            ((*pc.offset(-2isize).offset(0isize)
                                                  as libc::c_int) << 8i32 |
                                                 *pc.offset(-2isize).offset(1isize)
                                                     as libc::c_int) as
                                                uint32_t;
                                        /* R(a) = mrb_int(b) */
                                        pc = pc.offset(2isize);
                                        b =
                                            ((*pc.offset(-2isize).offset(0isize)
                                                  as libc::c_int) << 8i32 |
                                                 *pc.offset(-2isize).offset(1isize)
                                                     as libc::c_int) as
                                                uint16_t;
                                        current_block = 14447253356787937536;
                                    }
                                    4 => {
                                        pc = pc.offset(2isize);
                                        a =
                                            ((*pc.offset(-2isize).offset(0isize)
                                                  as libc::c_int) << 8i32 |
                                                 *pc.offset(-2isize).offset(1isize)
                                                     as libc::c_int) as
                                                uint32_t;
                                        /* R(a) = mrb_int(-b) */
                                        pc = pc.offset(2isize);
                                        b =
                                            ((*pc.offset(-2isize).offset(0isize)
                                                  as libc::c_int) << 8i32 |
                                                 *pc.offset(-2isize).offset(1isize)
                                                     as libc::c_int) as
                                                uint16_t;
                                        current_block = 7343950298149844727;
                                    }
                                    5 => {
                                        /* R(a) = mrb_int(-1) */
                                        let fresh439 = pc;
                                        pc = pc.offset(1);
                                        a = *fresh439 as uint32_t;
                                        current_block = 14563621082360312968;
                                    }
                                    6 => {
                                        /* R(a) = mrb_int(0) */
                                        let fresh440 = pc;
                                        pc = pc.offset(1);
                                        a = *fresh440 as uint32_t;
                                        current_block = 14563621082360312968;
                                    }
                                    7 => {
                                        /* R(a) = mrb_int(1) */
                                        let fresh441 = pc;
                                        pc = pc.offset(1);
                                        a = *fresh441 as uint32_t;
                                        current_block = 14563621082360312968;
                                    }
                                    8 => {
                                        /* R(a) = mrb_int(2) */
                                        let fresh442 = pc;
                                        pc = pc.offset(1);
                                        a = *fresh442 as uint32_t;
                                        current_block = 14563621082360312968;
                                    }
                                    9 => {
                                        /* R(a) = mrb_int(3) */
                                        let fresh443 = pc;
                                        pc = pc.offset(1);
                                        a = *fresh443 as uint32_t;
                                        current_block = 14563621082360312968;
                                    }
                                    10 => {
                                        /* R(a) = mrb_int(4) */
                                        let fresh444 = pc;
                                        pc = pc.offset(1);
                                        a = *fresh444 as uint32_t;
                                        current_block = 14563621082360312968;
                                    }
                                    11 => {
                                        /* R(a) = mrb_int(5) */
                                        let fresh445 = pc;
                                        pc = pc.offset(1);
                                        a = *fresh445 as uint32_t;
                                        current_block = 14563621082360312968;
                                    }
                                    12 => {
                                        /* R(a) = mrb_int(6) */
                                        let fresh446 = pc;
                                        pc = pc.offset(1);
                                        a = *fresh446 as uint32_t;
                                        current_block = 14563621082360312968;
                                    }
                                    13 => {
                                        /* R(a) = mrb_int(7) */
                                        let fresh447 = pc;
                                        pc = pc.offset(1);
                                        a = *fresh447 as uint32_t;
                                        current_block = 14563621082360312968;
                                    }
                                    14 => {
                                        pc = pc.offset(2isize);
                                        a =
                                            ((*pc.offset(-2isize).offset(0isize)
                                                  as libc::c_int) << 8i32 |
                                                 *pc.offset(-2isize).offset(1isize)
                                                     as libc::c_int) as
                                                uint32_t;
                                        /* R(a) = Syms(b) */
                                        pc = pc.offset(2isize);
                                        b =
                                            ((*pc.offset(-2isize).offset(0isize)
                                                  as libc::c_int) << 8i32 |
                                                 *pc.offset(-2isize).offset(1isize)
                                                     as libc::c_int) as
                                                uint16_t;
                                        current_block = 16314074004867283505;
                                    }
                                    15 => {
                                        /* R(a) = nil */
                                        let fresh448 = pc;
                                        pc = pc.offset(1);
                                        a = *fresh448 as uint32_t;
                                        current_block = 18201902862271706575;
                                    }
                                    16 => {
                                        /* R(a) = self */
                                        let fresh449 = pc;
                                        pc = pc.offset(1);
                                        a = *fresh449 as uint32_t;
                                        current_block = 16974974966130203269;
                                    }
                                    17 => {
                                        /* R(a) = true */
                                        let fresh450 = pc;
                                        pc = pc.offset(1);
                                        a = *fresh450 as uint32_t;
                                        current_block = 4976922244085895320;
                                    }
                                    18 => {
                                        /* R(a) = false */
                                        let fresh451 = pc;
                                        pc = pc.offset(1);
                                        a = *fresh451 as uint32_t;
                                        current_block = 4459663504651627985;
                                    }
                                    19 => {
                                        pc = pc.offset(2isize);
                                        a =
                                            ((*pc.offset(-2isize).offset(0isize)
                                                  as libc::c_int) << 8i32 |
                                                 *pc.offset(-2isize).offset(1isize)
                                                     as libc::c_int) as
                                                uint32_t;
                                        /* R(a) = getglobal(Syms(b)) */
                                        pc = pc.offset(2isize);
                                        b =
                                            ((*pc.offset(-2isize).offset(0isize)
                                                  as libc::c_int) << 8i32 |
                                                 *pc.offset(-2isize).offset(1isize)
                                                     as libc::c_int) as
                                                uint16_t;
                                        current_block = 13479157322803929894;
                                    }
                                    20 => {
                                        pc = pc.offset(2isize);
                                        a =
                                            ((*pc.offset(-2isize).offset(0isize)
                                                  as libc::c_int) << 8i32 |
                                                 *pc.offset(-2isize).offset(1isize)
                                                     as libc::c_int) as
                                                uint32_t;
                                        /* setglobal(Syms(b), R(a)) */
                                        pc = pc.offset(2isize);
                                        b =
                                            ((*pc.offset(-2isize).offset(0isize)
                                                  as libc::c_int) << 8i32 |
                                                 *pc.offset(-2isize).offset(1isize)
                                                     as libc::c_int) as
                                                uint16_t;
                                        current_block = 3151994457458062110;
                                    }
                                    21 => {
                                        pc = pc.offset(2isize);
                                        a =
                                            ((*pc.offset(-2isize).offset(0isize)
                                                  as libc::c_int) << 8i32 |
                                                 *pc.offset(-2isize).offset(1isize)
                                                     as libc::c_int) as
                                                uint32_t;
                                        /* R(a) = Special[Syms(b)] */
                                        pc = pc.offset(2isize);
                                        b =
                                            ((*pc.offset(-2isize).offset(0isize)
                                                  as libc::c_int) << 8i32 |
                                                 *pc.offset(-2isize).offset(1isize)
                                                     as libc::c_int) as
                                                uint16_t;
                                        current_block = 14187386403465544025;
                                    }
                                    22 => {
                                        pc = pc.offset(2isize);
                                        a =
                                            ((*pc.offset(-2isize).offset(0isize)
                                                  as libc::c_int) << 8i32 |
                                                 *pc.offset(-2isize).offset(1isize)
                                                     as libc::c_int) as
                                                uint32_t;
                                        /* Special[Syms(b)] = R(a) */
                                        pc = pc.offset(2isize);
                                        b =
                                            ((*pc.offset(-2isize).offset(0isize)
                                                  as libc::c_int) << 8i32 |
                                                 *pc.offset(-2isize).offset(1isize)
                                                     as libc::c_int) as
                                                uint16_t;
                                        current_block = 9974864727789713748;
                                    }
                                    23 => {
                                        pc = pc.offset(2isize);
                                        a =
                                            ((*pc.offset(-2isize).offset(0isize)
                                                  as libc::c_int) << 8i32 |
                                                 *pc.offset(-2isize).offset(1isize)
                                                     as libc::c_int) as
                                                uint32_t;
                                        /* R(a) = ivget(Syms(b)) */
                                        pc = pc.offset(2isize);
                                        b =
                                            ((*pc.offset(-2isize).offset(0isize)
                                                  as libc::c_int) << 8i32 |
                                                 *pc.offset(-2isize).offset(1isize)
                                                     as libc::c_int) as
                                                uint16_t;
                                        current_block = 4491581808830814586;
                                    }
                                    24 => {
                                        pc = pc.offset(2isize);
                                        a =
                                            ((*pc.offset(-2isize).offset(0isize)
                                                  as libc::c_int) << 8i32 |
                                                 *pc.offset(-2isize).offset(1isize)
                                                     as libc::c_int) as
                                                uint32_t;
                                        /* ivset(Syms(b),R(a)) */
                                        pc = pc.offset(2isize);
                                        b =
                                            ((*pc.offset(-2isize).offset(0isize)
                                                  as libc::c_int) << 8i32 |
                                                 *pc.offset(-2isize).offset(1isize)
                                                     as libc::c_int) as
                                                uint16_t;
                                        current_block = 12788783625999190409;
                                    }
                                    25 => {
                                        pc = pc.offset(2isize);
                                        a =
                                            ((*pc.offset(-2isize).offset(0isize)
                                                  as libc::c_int) << 8i32 |
                                                 *pc.offset(-2isize).offset(1isize)
                                                     as libc::c_int) as
                                                uint32_t;
                                        /* R(a) = cvget(Syms(b)) */
                                        pc = pc.offset(2isize);
                                        b =
                                            ((*pc.offset(-2isize).offset(0isize)
                                                  as libc::c_int) << 8i32 |
                                                 *pc.offset(-2isize).offset(1isize)
                                                     as libc::c_int) as
                                                uint16_t;
                                        current_block = 3906822848181906220;
                                    }
                                    26 => {
                                        pc = pc.offset(2isize);
                                        a =
                                            ((*pc.offset(-2isize).offset(0isize)
                                                  as libc::c_int) << 8i32 |
                                                 *pc.offset(-2isize).offset(1isize)
                                                     as libc::c_int) as
                                                uint32_t;
                                        /* cvset(Syms(b),R(a)) */
                                        pc = pc.offset(2isize);
                                        b =
                                            ((*pc.offset(-2isize).offset(0isize)
                                                  as libc::c_int) << 8i32 |
                                                 *pc.offset(-2isize).offset(1isize)
                                                     as libc::c_int) as
                                                uint16_t;
                                        current_block = 1105701036774490218;
                                    }
                                    27 => {
                                        pc = pc.offset(2isize);
                                        a =
                                            ((*pc.offset(-2isize).offset(0isize)
                                                  as libc::c_int) << 8i32 |
                                                 *pc.offset(-2isize).offset(1isize)
                                                     as libc::c_int) as
                                                uint32_t;
                                        /* R(a) = constget(Syms(b)) */
                                        pc = pc.offset(2isize);
                                        b =
                                            ((*pc.offset(-2isize).offset(0isize)
                                                  as libc::c_int) << 8i32 |
                                                 *pc.offset(-2isize).offset(1isize)
                                                     as libc::c_int) as
                                                uint16_t;
                                        current_block = 13114814261106982490;
                                    }
                                    28 => {
                                        pc = pc.offset(2isize);
                                        a =
                                            ((*pc.offset(-2isize).offset(0isize)
                                                  as libc::c_int) << 8i32 |
                                                 *pc.offset(-2isize).offset(1isize)
                                                     as libc::c_int) as
                                                uint32_t;
                                        /* constset(Syms(b),R(a)) */
                                        pc = pc.offset(2isize);
                                        b =
                                            ((*pc.offset(-2isize).offset(0isize)
                                                  as libc::c_int) << 8i32 |
                                                 *pc.offset(-2isize).offset(1isize)
                                                     as libc::c_int) as
                                                uint16_t;
                                        current_block = 6813271534392596583;
                                    }
                                    29 => {
                                        pc = pc.offset(2isize);
                                        a =
                                            ((*pc.offset(-2isize).offset(0isize)
                                                  as libc::c_int) << 8i32 |
                                                 *pc.offset(-2isize).offset(1isize)
                                                     as libc::c_int) as
                                                uint32_t;
                                        /* R(a) = R(a)::Syms(b) */
                                        pc = pc.offset(2isize);
                                        b =
                                            ((*pc.offset(-2isize).offset(0isize)
                                                  as libc::c_int) << 8i32 |
                                                 *pc.offset(-2isize).offset(1isize)
                                                     as libc::c_int) as
                                                uint16_t;
                                        current_block = 4001239642700071046;
                                    }
                                    30 => {
                                        pc = pc.offset(2isize);
                                        a =
                                            ((*pc.offset(-2isize).offset(0isize)
                                                  as libc::c_int) << 8i32 |
                                                 *pc.offset(-2isize).offset(1isize)
                                                     as libc::c_int) as
                                                uint32_t;
                                        /* R(a+1)::Syms(b) = R(a) */
                                        pc = pc.offset(2isize);
                                        b =
                                            ((*pc.offset(-2isize).offset(0isize)
                                                  as libc::c_int) << 8i32 |
                                                 *pc.offset(-2isize).offset(1isize)
                                                     as libc::c_int) as
                                                uint16_t;
                                        current_block = 5323028055506826224;
                                    }
                                    31 => {
                                        pc = pc.offset(2isize);
                                        a =
                                            ((*pc.offset(-2isize).offset(0isize)
                                                  as libc::c_int) << 8i32 |
                                                 *pc.offset(-2isize).offset(1isize)
                                                     as libc::c_int) as
                                                uint32_t;
                                        pc = pc.offset(2isize);
                                        b =
                                            ((*pc.offset(-2isize).offset(0isize)
                                                  as libc::c_int) << 8i32 |
                                                 *pc.offset(-2isize).offset(1isize)
                                                     as libc::c_int) as
                                                uint16_t;
                                        /* R(a) = uvget(b,c) */
                                        let fresh452 = pc;
                                        pc = pc.offset(1);
                                        c = *fresh452;
                                        current_block = 16263365153914704257;
                                    }
                                    32 => {
                                        pc = pc.offset(2isize);
                                        a =
                                            ((*pc.offset(-2isize).offset(0isize)
                                                  as libc::c_int) << 8i32 |
                                                 *pc.offset(-2isize).offset(1isize)
                                                     as libc::c_int) as
                                                uint32_t;
                                        pc = pc.offset(2isize);
                                        b =
                                            ((*pc.offset(-2isize).offset(0isize)
                                                  as libc::c_int) << 8i32 |
                                                 *pc.offset(-2isize).offset(1isize)
                                                     as libc::c_int) as
                                                uint16_t;
                                        /* uvset(b,c,R(a)) */
                                        let fresh453 = pc;
                                        pc = pc.offset(1);
                                        c = *fresh453;
                                        current_block = 4485224281673279150;
                                    }
                                    33 => {
                                        /* pc=a */
                                        pc = pc.offset(2isize);
                                        a =
                                            ((*pc.offset(-2isize).offset(0isize)
                                                  as libc::c_int) << 8i32 |
                                                 *pc.offset(-2isize).offset(1isize)
                                                     as libc::c_int) as
                                                uint32_t;
                                        current_block = 9422951997864425805;
                                    }
                                    34 => {
                                        pc = pc.offset(2isize);
                                        a =
                                            ((*pc.offset(-2isize).offset(0isize)
                                                  as libc::c_int) << 8i32 |
                                                 *pc.offset(-2isize).offset(1isize)
                                                     as libc::c_int) as
                                                uint32_t;
                                        /* if R(b) pc=a */
                                        pc = pc.offset(2isize);
                                        b =
                                            ((*pc.offset(-2isize).offset(0isize)
                                                  as libc::c_int) << 8i32 |
                                                 *pc.offset(-2isize).offset(1isize)
                                                     as libc::c_int) as
                                                uint16_t;
                                        current_block = 1297461190301222800;
                                    }
                                    35 => {
                                        pc = pc.offset(2isize);
                                        a =
                                            ((*pc.offset(-2isize).offset(0isize)
                                                  as libc::c_int) << 8i32 |
                                                 *pc.offset(-2isize).offset(1isize)
                                                     as libc::c_int) as
                                                uint32_t;
                                        /* if !R(b) pc=a */
                                        pc = pc.offset(2isize);
                                        b =
                                            ((*pc.offset(-2isize).offset(0isize)
                                                  as libc::c_int) << 8i32 |
                                                 *pc.offset(-2isize).offset(1isize)
                                                     as libc::c_int) as
                                                uint16_t;
                                        current_block = 11208766757666257413;
                                    }
                                    36 => {
                                        pc = pc.offset(2isize);
                                        a =
                                            ((*pc.offset(-2isize).offset(0isize)
                                                  as libc::c_int) << 8i32 |
                                                 *pc.offset(-2isize).offset(1isize)
                                                     as libc::c_int) as
                                                uint32_t;
                                        /* if R(b)==nil pc=a */
                                        pc = pc.offset(2isize);
                                        b =
                                            ((*pc.offset(-2isize).offset(0isize)
                                                  as libc::c_int) << 8i32 |
                                                 *pc.offset(-2isize).offset(1isize)
                                                     as libc::c_int) as
                                                uint16_t;
                                        current_block = 14166554486324432560;
                                    }
                                    37 => {
                                        /* rescue_push(a) */
                                        pc = pc.offset(2isize);
                                        a =
                                            ((*pc.offset(-2isize).offset(0isize)
                                                  as libc::c_int) << 8i32 |
                                                 *pc.offset(-2isize).offset(1isize)
                                                     as libc::c_int) as
                                                uint32_t;
                                        current_block = 12913164539471242988;
                                    }
                                    38 => {
                                        /* R(a) = exc */
                                        let fresh454 = pc;
                                        pc = pc.offset(1);
                                        a = *fresh454 as uint32_t;
                                        current_block = 2722617303989997926;
                                    }
                                    39 => {
                                        pc = pc.offset(2isize);
                                        a =
                                            ((*pc.offset(-2isize).offset(0isize)
                                                  as libc::c_int) << 8i32 |
                                                 *pc.offset(-2isize).offset(1isize)
                                                     as libc::c_int) as
                                                uint32_t;
                                        /* R(b) = R(a).isa?(R(b)) */
                                        pc = pc.offset(2isize);
                                        b =
                                            ((*pc.offset(-2isize).offset(0isize)
                                                  as libc::c_int) << 8i32 |
                                                 *pc.offset(-2isize).offset(1isize)
                                                     as libc::c_int) as
                                                uint16_t;
                                        current_block = 16261244098005255386;
                                    }
                                    40 => {
                                        /* a.times{rescue_pop()} */
                                        let fresh455 = pc;
                                        pc = pc.offset(1);
                                        a = *fresh455 as uint32_t;
                                        current_block = 14933967110489461578;
                                    }
                                    41 => {
                                        /* raise(R(a)) */
                                        let fresh456 = pc;
                                        pc = pc.offset(1);
                                        a = *fresh456 as uint32_t;
                                        current_block = 85722159537708186;
                                    }
                                    42 => {
                                        /* ensure_push(SEQ[a]) */
                                        let fresh457 = pc;
                                        pc = pc.offset(1);
                                        a = *fresh457 as uint32_t;
                                        current_block = 17178928522929307677;
                                    }
                                    43 => {
                                        /* A.times{ensure_pop().call} */
                                        let fresh458 = pc;
                                        pc = pc.offset(1);
                                        a = *fresh458 as uint32_t;
                                        current_block = 8592218650707875588;
                                    }
                                    44 => {
                                        pc = pc.offset(2isize);
                                        a =
                                            ((*pc.offset(-2isize).offset(0isize)
                                                  as libc::c_int) << 8i32 |
                                                 *pc.offset(-2isize).offset(1isize)
                                                     as libc::c_int) as
                                                uint32_t;
                                        /* R(a) = call(R(a),Syms(b),*R(a+1)) */
                                        pc = pc.offset(2isize);
                                        b =
                                            ((*pc.offset(-2isize).offset(0isize)
                                                  as libc::c_int) << 8i32 |
                                                 *pc.offset(-2isize).offset(1isize)
                                                     as libc::c_int) as
                                                uint16_t;
                                        current_block = 12662816978892980002;
                                    }
                                    45 => {
                                        pc = pc.offset(2isize);
                                        a =
                                            ((*pc.offset(-2isize).offset(0isize)
                                                  as libc::c_int) << 8i32 |
                                                 *pc.offset(-2isize).offset(1isize)
                                                     as libc::c_int) as
                                                uint32_t;
                                        /* R(a) = call(R(a),Syms(b),*R(a+1),&R(a+2)) */
                                        pc = pc.offset(2isize);
                                        b =
                                            ((*pc.offset(-2isize).offset(0isize)
                                                  as libc::c_int) << 8i32 |
                                                 *pc.offset(-2isize).offset(1isize)
                                                     as libc::c_int) as
                                                uint16_t;
                                        current_block = 7460542724658431689;
                                    }
                                    46 => {
                                        pc = pc.offset(2isize);
                                        a =
                                            ((*pc.offset(-2isize).offset(0isize)
                                                  as libc::c_int) << 8i32 |
                                                 *pc.offset(-2isize).offset(1isize)
                                                     as libc::c_int) as
                                                uint32_t;
                                        pc = pc.offset(2isize);
                                        b =
                                            ((*pc.offset(-2isize).offset(0isize)
                                                  as libc::c_int) << 8i32 |
                                                 *pc.offset(-2isize).offset(1isize)
                                                     as libc::c_int) as
                                                uint16_t;
                                        /* R(a) = call(R(a),Syms(b),R(a+1),...,R(a+c)) */
                                        let fresh459 = pc;
                                        pc = pc.offset(1);
                                        c = *fresh459;
                                        current_block = 7304850410330992871;
                                    }
                                    47 => {
                                        pc = pc.offset(2isize);
                                        a =
                                            ((*pc.offset(-2isize).offset(0isize)
                                                  as libc::c_int) << 8i32 |
                                                 *pc.offset(-2isize).offset(1isize)
                                                     as libc::c_int) as
                                                uint32_t;
                                        pc = pc.offset(2isize);
                                        b =
                                            ((*pc.offset(-2isize).offset(0isize)
                                                  as libc::c_int) << 8i32 |
                                                 *pc.offset(-2isize).offset(1isize)
                                                     as libc::c_int) as
                                                uint16_t;
                                        /* R(a) = call(R(a),Syms(Bx),R(a+1),...,R(a+c),&R(a+c+1)) */
                                        let fresh460 = pc;
                                        pc = pc.offset(1);
                                        c = *fresh460;
                                        current_block = 10790764747546649571;
                                    }
                                    48 => {
                                        current_block = 13698748673986354976;
                                    }
                                    49 => {
                                        pc = pc.offset(2isize);
                                        a =
                                            ((*pc.offset(-2isize).offset(0isize)
                                                  as libc::c_int) << 8i32 |
                                                 *pc.offset(-2isize).offset(1isize)
                                                     as libc::c_int) as
                                                uint32_t;
                                        /* R(a) = super(R(a+1),... ,R(a+b+1)) */
                                        pc = pc.offset(2isize);
                                        b =
                                            ((*pc.offset(-2isize).offset(0isize)
                                                  as libc::c_int) << 8i32 |
                                                 *pc.offset(-2isize).offset(1isize)
                                                     as libc::c_int) as
                                                uint16_t;
                                        current_block = 9607585333290054341;
                                    }
                                    50 => {
                                        pc = pc.offset(2isize);
                                        a =
                                            ((*pc.offset(-2isize).offset(0isize)
                                                  as libc::c_int) << 8i32 |
                                                 *pc.offset(-2isize).offset(1isize)
                                                     as libc::c_int) as
                                                uint32_t;
                                        /* R(a) = argument array (16=m5:r1:m5:d1:lv4) */
                                        pc = pc.offset(2isize);
                                        b =
                                            ((*pc.offset(-2isize).offset(0isize)
                                                  as libc::c_int) << 8i32 |
                                                 *pc.offset(-2isize).offset(1isize)
                                                     as libc::c_int) as
                                                uint16_t;
                                        current_block = 10763054988551342546;
                                    }
                                    51 => {
                                        /* arg setup according to flags (23=m5:o5:r1:m5:k5:d1:b1) */
                                        pc = pc.offset(3isize);
                                        a =
                                            ((*pc.offset(-3isize).offset(0isize)
                                                  as libc::c_int) << 16i32 |
                                                 (*pc.offset(-3isize).offset(1isize)
                                                      as libc::c_int) << 8i32
                                                 |
                                                 *pc.offset(-3isize).offset(2isize)
                                                     as libc::c_int) as
                                                uint32_t;
                                        current_block = 15803687294481920802;
                                    }
                                    52 => {
                                        pc = pc.offset(2isize);
                                        a =
                                            ((*pc.offset(-2isize).offset(0isize)
                                                  as libc::c_int) << 8i32 |
                                                 *pc.offset(-2isize).offset(1isize)
                                                     as libc::c_int) as
                                                uint32_t;
                                        /* R(a) = kdict.key?(Syms(b))                      # todo */
                                        pc = pc.offset(2isize);
                                        b =
                                            ((*pc.offset(-2isize).offset(0isize)
                                                  as libc::c_int) << 8i32 |
                                                 *pc.offset(-2isize).offset(1isize)
                                                     as libc::c_int) as
                                                uint16_t;
                                        current_block = 1525421113118994726;
                                    }
                                    53 => {
                                        current_block = 4663630632216779857;
                                    }
                                    54 => {
                                        pc = pc.offset(2isize);
                                        a =
                                            ((*pc.offset(-2isize).offset(0isize)
                                                  as libc::c_int) << 8i32 |
                                                 *pc.offset(-2isize).offset(1isize)
                                                     as libc::c_int) as
                                                uint32_t;
                                        /* R(a) = kdict[Syms(b)]; kdict.delete(Syms(b))    # todo */
                                        pc = pc.offset(2isize);
                                        b =
                                            ((*pc.offset(-2isize).offset(0isize)
                                                  as libc::c_int) << 8i32 |
                                                 *pc.offset(-2isize).offset(1isize)
                                                     as libc::c_int) as
                                                uint16_t;
                                        current_block = 14765652484648187635;
                                    }
                                    55 => {
                                        /* return R(a) (normal) */
                                        let fresh461 = pc;
                                        pc = pc.offset(1);
                                        a = *fresh461 as uint32_t;
                                        current_block = 14547135203946798640;
                                    }
                                    56 => {
                                        /* return R(a) (in-block return) */
                                        let fresh462 = pc;
                                        pc = pc.offset(1);
                                        a = *fresh462 as uint32_t;
                                        current_block = 13114312767422449834;
                                    }
                                    57 => {
                                        /* break R(a) */
                                        let fresh463 = pc;
                                        pc = pc.offset(1);
                                        a = *fresh463 as uint32_t;
                                        current_block = 12223856006808243146;
                                    }
                                    58 => {
                                        pc = pc.offset(2isize);
                                        a =
                                            ((*pc.offset(-2isize).offset(0isize)
                                                  as libc::c_int) << 8i32 |
                                                 *pc.offset(-2isize).offset(1isize)
                                                     as libc::c_int) as
                                                uint32_t;
                                        /* R(a) = block (16=m5:r1:m5:d1:lv4) */
                                        pc = pc.offset(2isize);
                                        b =
                                            ((*pc.offset(-2isize).offset(0isize)
                                                  as libc::c_int) << 8i32 |
                                                 *pc.offset(-2isize).offset(1isize)
                                                     as libc::c_int) as
                                                uint16_t;
                                        current_block = 2994422791481536047;
                                    }
                                    59 => {
                                        /* R(a) = R(a)+R(a+1) */
                                        let fresh464 = pc;
                                        pc = pc.offset(1);
                                        a = *fresh464 as uint32_t;
                                        current_block = 16140473650108643456;
                                    }
                                    60 => {
                                        pc = pc.offset(2isize);
                                        a =
                                            ((*pc.offset(-2isize).offset(0isize)
                                                  as libc::c_int) << 8i32 |
                                                 *pc.offset(-2isize).offset(1isize)
                                                     as libc::c_int) as
                                                uint32_t;
                                        /* R(a) = R(a)+mrb_int(c)  */
                                        pc = pc.offset(2isize);
                                        b =
                                            ((*pc.offset(-2isize).offset(0isize)
                                                  as libc::c_int) << 8i32 |
                                                 *pc.offset(-2isize).offset(1isize)
                                                     as libc::c_int) as
                                                uint16_t;
                                        current_block = 4396809944207673246;
                                    }
                                    61 => {
                                        /* R(a) = R(a)-R(a+1) */
                                        let fresh465 = pc;
                                        pc = pc.offset(1);
                                        a = *fresh465 as uint32_t;
                                        current_block = 10653245833193402520;
                                    }
                                    62 => {
                                        pc = pc.offset(2isize);
                                        a =
                                            ((*pc.offset(-2isize).offset(0isize)
                                                  as libc::c_int) << 8i32 |
                                                 *pc.offset(-2isize).offset(1isize)
                                                     as libc::c_int) as
                                                uint32_t;
                                        /* R(a) = R(a)-C */
                                        pc = pc.offset(2isize);
                                        b =
                                            ((*pc.offset(-2isize).offset(0isize)
                                                  as libc::c_int) << 8i32 |
                                                 *pc.offset(-2isize).offset(1isize)
                                                     as libc::c_int) as
                                                uint16_t;
                                        current_block = 1767406316152665813;
                                    }
                                    63 => {
                                        /* R(a) = R(a)*R(a+1) */
                                        let fresh466 = pc;
                                        pc = pc.offset(1);
                                        a = *fresh466 as uint32_t;
                                        current_block = 2558910614333069445;
                                    }
                                    64 => {
                                        /* R(a) = R(a)/R(a+1) */
                                        let fresh467 = pc;
                                        pc = pc.offset(1);
                                        a = *fresh467 as uint32_t;
                                        current_block = 15842299444801573850;
                                    }
                                    65 => {
                                        /* R(a) = R(a)==R(a+1) */
                                        let fresh468 = pc;
                                        pc = pc.offset(1);
                                        a = *fresh468 as uint32_t;
                                        current_block = 10264557722339789830;
                                    }
                                    66 => {
                                        /* R(a) = R(a)<R(a+1) */
                                        let fresh469 = pc;
                                        pc = pc.offset(1);
                                        a = *fresh469 as uint32_t;
                                        current_block = 11600479572347497286;
                                    }
                                    67 => {
                                        /* R(a) = R(a)<=R(a+1) */
                                        let fresh470 = pc;
                                        pc = pc.offset(1);
                                        a = *fresh470 as uint32_t;
                                        current_block = 6268356626658084969;
                                    }
                                    68 => {
                                        /* R(a) = R(a)>R(a+1) */
                                        let fresh471 = pc;
                                        pc = pc.offset(1);
                                        a = *fresh471 as uint32_t;
                                        current_block = 14064792543554720178;
                                    }
                                    69 => {
                                        /* R(a) = R(a)>=R(a+1) */
                                        let fresh472 = pc;
                                        pc = pc.offset(1);
                                        a = *fresh472 as uint32_t;
                                        current_block = 12168003877939815570;
                                    }
                                    70 => {
                                        pc = pc.offset(2isize);
                                        a =
                                            ((*pc.offset(-2isize).offset(0isize)
                                                  as libc::c_int) << 8i32 |
                                                 *pc.offset(-2isize).offset(1isize)
                                                     as libc::c_int) as
                                                uint32_t;
                                        /* R(a) = ary_new(R(a),R(a+1)..R(a+b)) */
                                        pc = pc.offset(2isize);
                                        b =
                                            ((*pc.offset(-2isize).offset(0isize)
                                                  as libc::c_int) << 8i32 |
                                                 *pc.offset(-2isize).offset(1isize)
                                                     as libc::c_int) as
                                                uint16_t;
                                        current_block = 5334851501950231651;
                                    }
                                    71 => {
                                        pc = pc.offset(2isize);
                                        a =
                                            ((*pc.offset(-2isize).offset(0isize)
                                                  as libc::c_int) << 8i32 |
                                                 *pc.offset(-2isize).offset(1isize)
                                                     as libc::c_int) as
                                                uint32_t;
                                        pc = pc.offset(2isize);
                                        b =
                                            ((*pc.offset(-2isize).offset(0isize)
                                                  as libc::c_int) << 8i32 |
                                                 *pc.offset(-2isize).offset(1isize)
                                                     as libc::c_int) as
                                                uint16_t;
                                        /* R(a) = ary_new(R(b),R(b+1)..R(b+c)) */
                                        let fresh473 = pc;
                                        pc = pc.offset(1);
                                        c = *fresh473;
                                        current_block = 15590445366052810487;
                                    }
                                    72 => {
                                        /* ary_cat(R(a),R(a+1)) */
                                        let fresh474 = pc;
                                        pc = pc.offset(1);
                                        a = *fresh474 as uint32_t;
                                        current_block = 15951340130516588981;
                                    }
                                    73 => {
                                        /* ary_push(R(a),R(a+1)) */
                                        let fresh475 = pc;
                                        pc = pc.offset(1);
                                        a = *fresh475 as uint32_t;
                                        current_block = 17884754765720769523;
                                    }
                                    74 => {
                                        /* R(a) = ary_dup(R(a)) */
                                        let fresh476 = pc;
                                        pc = pc.offset(1);
                                        a = *fresh476 as uint32_t;
                                        current_block = 6985669818981290717;
                                    }
                                    75 => {
                                        pc = pc.offset(2isize);
                                        a =
                                            ((*pc.offset(-2isize).offset(0isize)
                                                  as libc::c_int) << 8i32 |
                                                 *pc.offset(-2isize).offset(1isize)
                                                     as libc::c_int) as
                                                uint32_t;
                                        pc = pc.offset(2isize);
                                        b =
                                            ((*pc.offset(-2isize).offset(0isize)
                                                  as libc::c_int) << 8i32 |
                                                 *pc.offset(-2isize).offset(1isize)
                                                     as libc::c_int) as
                                                uint16_t;
                                        /* R(a) = R(b)[c] */
                                        let fresh477 = pc;
                                        pc = pc.offset(1);
                                        c = *fresh477;
                                        current_block = 10370658347790963364;
                                    }
                                    76 => {
                                        pc = pc.offset(2isize);
                                        a =
                                            ((*pc.offset(-2isize).offset(0isize)
                                                  as libc::c_int) << 8i32 |
                                                 *pc.offset(-2isize).offset(1isize)
                                                     as libc::c_int) as
                                                uint32_t;
                                        pc = pc.offset(2isize);
                                        b =
                                            ((*pc.offset(-2isize).offset(0isize)
                                                  as libc::c_int) << 8i32 |
                                                 *pc.offset(-2isize).offset(1isize)
                                                     as libc::c_int) as
                                                uint16_t;
                                        /* R(a)[c] = R(b) */
                                        let fresh478 = pc;
                                        pc = pc.offset(1);
                                        c = *fresh478;
                                        current_block = 4938714451943331475;
                                    }
                                    77 => {
                                        pc = pc.offset(2isize);
                                        a =
                                            ((*pc.offset(-2isize).offset(0isize)
                                                  as libc::c_int) << 8i32 |
                                                 *pc.offset(-2isize).offset(1isize)
                                                     as libc::c_int) as
                                                uint32_t;
                                        pc = pc.offset(2isize);
                                        b =
                                            ((*pc.offset(-2isize).offset(0isize)
                                                  as libc::c_int) << 8i32 |
                                                 *pc.offset(-2isize).offset(1isize)
                                                     as libc::c_int) as
                                                uint16_t;
                                        /* *R(a),R(a+1)..R(a+c) = R(a)[b..] */
                                        let fresh479 = pc;
                                        pc = pc.offset(1);
                                        c = *fresh479;
                                        current_block = 11125061941892892926;
                                    }
                                    78 => {
                                        /* R(a) = intern(R(a)) */
                                        let fresh480 = pc;
                                        pc = pc.offset(1);
                                        a = *fresh480 as uint32_t;
                                        current_block = 2896230971687472890;
                                    }
                                    79 => {
                                        pc = pc.offset(2isize);
                                        a =
                                            ((*pc.offset(-2isize).offset(0isize)
                                                  as libc::c_int) << 8i32 |
                                                 *pc.offset(-2isize).offset(1isize)
                                                     as libc::c_int) as
                                                uint32_t;
                                        /* R(a) = str_dup(Lit(b)) */
                                        pc = pc.offset(2isize);
                                        b =
                                            ((*pc.offset(-2isize).offset(0isize)
                                                  as libc::c_int) << 8i32 |
                                                 *pc.offset(-2isize).offset(1isize)
                                                     as libc::c_int) as
                                                uint16_t;
                                        current_block = 14841647832940836303;
                                    }
                                    80 => {
                                        /* str_cat(R(a),R(a+1)) */
                                        let fresh481 = pc;
                                        pc = pc.offset(1);
                                        a = *fresh481 as uint32_t;
                                        current_block = 4191712132543108294;
                                    }
                                    81 => {
                                        pc = pc.offset(2isize);
                                        a =
                                            ((*pc.offset(-2isize).offset(0isize)
                                                  as libc::c_int) << 8i32 |
                                                 *pc.offset(-2isize).offset(1isize)
                                                     as libc::c_int) as
                                                uint32_t;
                                        /* R(a) = hash_new(R(a),R(a+1)..R(a+b)) */
                                        pc = pc.offset(2isize);
                                        b =
                                            ((*pc.offset(-2isize).offset(0isize)
                                                  as libc::c_int) << 8i32 |
                                                 *pc.offset(-2isize).offset(1isize)
                                                     as libc::c_int) as
                                                uint16_t;
                                        current_block = 1686938961607716371;
                                    }
                                    82 => {
                                        pc = pc.offset(2isize);
                                        a =
                                            ((*pc.offset(-2isize).offset(0isize)
                                                  as libc::c_int) << 8i32 |
                                                 *pc.offset(-2isize).offset(1isize)
                                                     as libc::c_int) as
                                                uint32_t;
                                        /* R(a) = hash_push(R(a),R(a+1)..R(a+b)) */
                                        pc = pc.offset(2isize);
                                        b =
                                            ((*pc.offset(-2isize).offset(0isize)
                                                  as libc::c_int) << 8i32 |
                                                 *pc.offset(-2isize).offset(1isize)
                                                     as libc::c_int) as
                                                uint16_t;
                                        current_block = 17492450365994263002;
                                    }
                                    83 => {
                                        /* R(a) = hash_cat(R(a),R(a+1)) */
                                        let fresh482 = pc;
                                        pc = pc.offset(1);
                                        a = *fresh482 as uint32_t;
                                        current_block = 15495526794150689403;
                                    }
                                    84 => {
                                        pc = pc.offset(2isize);
                                        a =
                                            ((*pc.offset(-2isize).offset(0isize)
                                                  as libc::c_int) << 8i32 |
                                                 *pc.offset(-2isize).offset(1isize)
                                                     as libc::c_int) as
                                                uint32_t;
                                        /* R(a) = lambda(SEQ[b],L_LAMBDA) */
                                        pc = pc.offset(2isize);
                                        b =
                                            ((*pc.offset(-2isize).offset(0isize)
                                                  as libc::c_int) << 8i32 |
                                                 *pc.offset(-2isize).offset(1isize)
                                                     as libc::c_int) as
                                                uint16_t;
                                        current_block = 2702902889507381978;
                                    }
                                    85 => {
                                        pc = pc.offset(2isize);
                                        a =
                                            ((*pc.offset(-2isize).offset(0isize)
                                                  as libc::c_int) << 8i32 |
                                                 *pc.offset(-2isize).offset(1isize)
                                                     as libc::c_int) as
                                                uint32_t;
                                        /* R(a) = lambda(SEQ[b],L_BLOCK) */
                                        pc = pc.offset(2isize);
                                        b =
                                            ((*pc.offset(-2isize).offset(0isize)
                                                  as libc::c_int) << 8i32 |
                                                 *pc.offset(-2isize).offset(1isize)
                                                     as libc::c_int) as
                                                uint16_t;
                                        current_block = 1152971280698483565;
                                    }
                                    86 => {
                                        pc = pc.offset(2isize);
                                        a =
                                            ((*pc.offset(-2isize).offset(0isize)
                                                  as libc::c_int) << 8i32 |
                                                 *pc.offset(-2isize).offset(1isize)
                                                     as libc::c_int) as
                                                uint32_t;
                                        /* R(a) = lambda(SEQ[b],L_METHOD) */
                                        pc = pc.offset(2isize);
                                        b =
                                            ((*pc.offset(-2isize).offset(0isize)
                                                  as libc::c_int) << 8i32 |
                                                 *pc.offset(-2isize).offset(1isize)
                                                     as libc::c_int) as
                                                uint16_t;
                                        current_block = 16224973743324382762;
                                    }
                                    87 => {
                                        /* R(a) = range_new(R(a),R(a+1),FALSE) */
                                        let fresh483 = pc;
                                        pc = pc.offset(1);
                                        a = *fresh483 as uint32_t;
                                        current_block = 6571067991607217453;
                                    }
                                    88 => {
                                        /* R(a) = range_new(R(a),R(a+1),TRUE) */
                                        let fresh484 = pc;
                                        pc = pc.offset(1);
                                        a = *fresh484 as uint32_t;
                                        current_block = 14502557129029694510;
                                    }
                                    89 => {
                                        /* R(a) = ::Object */
                                        let fresh485 = pc;
                                        pc = pc.offset(1);
                                        a = *fresh485 as uint32_t;
                                        current_block = 16196049249653527883;
                                    }
                                    90 => {
                                        pc = pc.offset(2isize);
                                        a =
                                            ((*pc.offset(-2isize).offset(0isize)
                                                  as libc::c_int) << 8i32 |
                                                 *pc.offset(-2isize).offset(1isize)
                                                     as libc::c_int) as
                                                uint32_t;
                                        /* R(a) = newclass(R(a),Syms(b),R(a+1)) */
                                        pc = pc.offset(2isize);
                                        b =
                                            ((*pc.offset(-2isize).offset(0isize)
                                                  as libc::c_int) << 8i32 |
                                                 *pc.offset(-2isize).offset(1isize)
                                                     as libc::c_int) as
                                                uint16_t;
                                        current_block = 16736254050004001558;
                                    }
                                    91 => {
                                        pc = pc.offset(2isize);
                                        a =
                                            ((*pc.offset(-2isize).offset(0isize)
                                                  as libc::c_int) << 8i32 |
                                                 *pc.offset(-2isize).offset(1isize)
                                                     as libc::c_int) as
                                                uint32_t;
                                        /* R(a) = newmodule(R(a),Syms(b)) */
                                        pc = pc.offset(2isize);
                                        b =
                                            ((*pc.offset(-2isize).offset(0isize)
                                                  as libc::c_int) << 8i32 |
                                                 *pc.offset(-2isize).offset(1isize)
                                                     as libc::c_int) as
                                                uint16_t;
                                        current_block = 16407897916814827188;
                                    }
                                    92 => {
                                        pc = pc.offset(2isize);
                                        a =
                                            ((*pc.offset(-2isize).offset(0isize)
                                                  as libc::c_int) << 8i32 |
                                                 *pc.offset(-2isize).offset(1isize)
                                                     as libc::c_int) as
                                                uint32_t;
                                        /* R(a) = blockexec(R(a),SEQ[b]) */
                                        pc = pc.offset(2isize);
                                        b =
                                            ((*pc.offset(-2isize).offset(0isize)
                                                  as libc::c_int) << 8i32 |
                                                 *pc.offset(-2isize).offset(1isize)
                                                     as libc::c_int) as
                                                uint16_t;
                                        current_block = 9892557545450190862;
                                    }
                                    93 => {
                                        pc = pc.offset(2isize);
                                        a =
                                            ((*pc.offset(-2isize).offset(0isize)
                                                  as libc::c_int) << 8i32 |
                                                 *pc.offset(-2isize).offset(1isize)
                                                     as libc::c_int) as
                                                uint32_t;
                                        /* R(a).newmethod(Syms(b),R(a+1)) */
                                        pc = pc.offset(2isize);
                                        b =
                                            ((*pc.offset(-2isize).offset(0isize)
                                                  as libc::c_int) << 8i32 |
                                                 *pc.offset(-2isize).offset(1isize)
                                                     as libc::c_int) as
                                                uint16_t;
                                        current_block = 15928390647053518202;
                                    }
                                    94 => {
                                        pc = pc.offset(2isize);
                                        a =
                                            ((*pc.offset(-2isize).offset(0isize)
                                                  as libc::c_int) << 8i32 |
                                                 *pc.offset(-2isize).offset(1isize)
                                                     as libc::c_int) as
                                                uint32_t;
                                        /* alias_method(target_class,Syms(a),Syms(b)) */
                                        pc = pc.offset(2isize);
                                        b =
                                            ((*pc.offset(-2isize).offset(0isize)
                                                  as libc::c_int) << 8i32 |
                                                 *pc.offset(-2isize).offset(1isize)
                                                     as libc::c_int) as
                                                uint16_t;
                                        current_block = 1656069329997846098;
                                    }
                                    95 => {
                                        /* undef_method(target_class,Syms(a)) */
                                        let fresh486 = pc;
                                        pc = pc.offset(1);
                                        a = *fresh486 as uint32_t;
                                        current_block = 5769344152980800111;
                                    }
                                    96 => {
                                        /* R(a) = R(a).singleton_class */
                                        let fresh487 = pc;
                                        pc = pc.offset(1);
                                        a = *fresh487 as uint32_t;
                                        current_block = 15212051862008919012;
                                    }
                                    97 => {
                                        /* R(a) = target_class */
                                        let fresh488 = pc;
                                        pc = pc.offset(1);
                                        a = *fresh488 as uint32_t;
                                        current_block = 15561344248022367768;
                                    }
                                    98 => {
                                        pc = pc.offset(2isize);
                                        a =
                                            ((*pc.offset(-2isize).offset(0isize)
                                                  as libc::c_int) << 8i32 |
                                                 *pc.offset(-2isize).offset(1isize)
                                                     as libc::c_int) as
                                                uint32_t;
                                        pc = pc.offset(2isize);
                                        b =
                                            ((*pc.offset(-2isize).offset(0isize)
                                                  as libc::c_int) << 8i32 |
                                                 *pc.offset(-2isize).offset(1isize)
                                                     as libc::c_int) as
                                                uint16_t;
                                        /* print a,b,c */
                                        let fresh489 = pc;
                                        pc = pc.offset(1);
                                        c = *fresh489;
                                        current_block = 10071922767614739690;
                                        break 'c_4688 ;
                                    }
                                    99 => {
                                        /* raise(LocalJumpError, Lit(a)) */
                                        let fresh490 = pc;
                                        pc = pc.offset(1);
                                        a = *fresh490 as uint32_t;
                                        current_block = 9217466010932042731;
                                    }
                                    100 => {
                                        current_block = 16136094826069225138;
                                    }
                                    101 => {
                                        current_block = 12853241758993110625;
                                    }
                                    102 => {
                                        current_block = 3106153520265210690;
                                    }
                                    103 => {
                                        current_block = 5128590269704079499;
                                        break 'c_4688 ;
                                    }
                                    _ => {
                                        pc = pc.offset(-1isize);
                                        current_block = 7149356873433890176;
                                        continue 'c_4688 ;
                                    }
                                }
                            }
                            12853241758993110625 => {
                                let fresh333 = pc;
                                pc = pc.offset(1);
                                insn = *fresh333;
                                match insn as libc::c_int {
                                    0 => {
                                        current_block = 14576567515993809846;
                                    }
                                    1 => {
                                        let fresh334 = pc;
                                        pc = pc.offset(1);
                                        a = *fresh334 as uint32_t;
                                        /* R(a) = R(b) */
                                        pc = pc.offset(2isize);
                                        b =
                                            ((*pc.offset(-2isize).offset(0isize)
                                                  as libc::c_int) << 8i32 |
                                                 *pc.offset(-2isize).offset(1isize)
                                                     as libc::c_int) as
                                                uint16_t;
                                        current_block = 4488286894823169796;
                                    }
                                    2 => {
                                        let fresh335 = pc;
                                        pc = pc.offset(1);
                                        a = *fresh335 as uint32_t;
                                        /* R(a) = Pool(b) */
                                        pc = pc.offset(2isize);
                                        b =
                                            ((*pc.offset(-2isize).offset(0isize)
                                                  as libc::c_int) << 8i32 |
                                                 *pc.offset(-2isize).offset(1isize)
                                                     as libc::c_int) as
                                                uint16_t;
                                        current_block = 8845338526596852646;
                                    }
                                    3 => {
                                        let fresh336 = pc;
                                        pc = pc.offset(1);
                                        a = *fresh336 as uint32_t;
                                        /* R(a) = mrb_int(b) */
                                        pc = pc.offset(2isize);
                                        b =
                                            ((*pc.offset(-2isize).offset(0isize)
                                                  as libc::c_int) << 8i32 |
                                                 *pc.offset(-2isize).offset(1isize)
                                                     as libc::c_int) as
                                                uint16_t;
                                        current_block = 14447253356787937536;
                                    }
                                    4 => {
                                        let fresh337 = pc;
                                        pc = pc.offset(1);
                                        a = *fresh337 as uint32_t;
                                        /* R(a) = mrb_int(-b) */
                                        pc = pc.offset(2isize);
                                        b =
                                            ((*pc.offset(-2isize).offset(0isize)
                                                  as libc::c_int) << 8i32 |
                                                 *pc.offset(-2isize).offset(1isize)
                                                     as libc::c_int) as
                                                uint16_t;
                                        current_block = 7343950298149844727;
                                    }
                                    5 => {
                                        /* R(a) = mrb_int(-1) */
                                        let fresh338 = pc;
                                        pc = pc.offset(1);
                                        a = *fresh338 as uint32_t;
                                        current_block = 14563621082360312968;
                                    }
                                    6 => {
                                        /* R(a) = mrb_int(0) */
                                        let fresh339 = pc;
                                        pc = pc.offset(1);
                                        a = *fresh339 as uint32_t;
                                        current_block = 14563621082360312968;
                                    }
                                    7 => {
                                        /* R(a) = mrb_int(1) */
                                        let fresh340 = pc;
                                        pc = pc.offset(1);
                                        a = *fresh340 as uint32_t;
                                        current_block = 14563621082360312968;
                                    }
                                    8 => {
                                        /* R(a) = mrb_int(2) */
                                        let fresh341 = pc;
                                        pc = pc.offset(1);
                                        a = *fresh341 as uint32_t;
                                        current_block = 14563621082360312968;
                                    }
                                    9 => {
                                        /* R(a) = mrb_int(3) */
                                        let fresh342 = pc;
                                        pc = pc.offset(1);
                                        a = *fresh342 as uint32_t;
                                        current_block = 14563621082360312968;
                                    }
                                    10 => {
                                        /* R(a) = mrb_int(4) */
                                        let fresh343 = pc;
                                        pc = pc.offset(1);
                                        a = *fresh343 as uint32_t;
                                        current_block = 14563621082360312968;
                                    }
                                    11 => {
                                        /* R(a) = mrb_int(5) */
                                        let fresh344 = pc;
                                        pc = pc.offset(1);
                                        a = *fresh344 as uint32_t;
                                        current_block = 14563621082360312968;
                                    }
                                    12 => {
                                        /* R(a) = mrb_int(6) */
                                        let fresh345 = pc;
                                        pc = pc.offset(1);
                                        a = *fresh345 as uint32_t;
                                        current_block = 14563621082360312968;
                                    }
                                    13 => {
                                        /* R(a) = mrb_int(7) */
                                        let fresh346 = pc;
                                        pc = pc.offset(1);
                                        a = *fresh346 as uint32_t;
                                        current_block = 14563621082360312968;
                                    }
                                    14 => {
                                        let fresh347 = pc;
                                        pc = pc.offset(1);
                                        a = *fresh347 as uint32_t;
                                        /* R(a) = Syms(b) */
                                        pc = pc.offset(2isize);
                                        b =
                                            ((*pc.offset(-2isize).offset(0isize)
                                                  as libc::c_int) << 8i32 |
                                                 *pc.offset(-2isize).offset(1isize)
                                                     as libc::c_int) as
                                                uint16_t;
                                        current_block = 16314074004867283505;
                                    }
                                    15 => {
                                        /* R(a) = nil */
                                        let fresh348 = pc;
                                        pc = pc.offset(1);
                                        a = *fresh348 as uint32_t;
                                        current_block = 18201902862271706575;
                                    }
                                    16 => {
                                        /* R(a) = self */
                                        let fresh349 = pc;
                                        pc = pc.offset(1);
                                        a = *fresh349 as uint32_t;
                                        current_block = 16974974966130203269;
                                    }
                                    17 => {
                                        /* R(a) = true */
                                        let fresh350 = pc;
                                        pc = pc.offset(1);
                                        a = *fresh350 as uint32_t;
                                        current_block = 4976922244085895320;
                                    }
                                    18 => {
                                        /* R(a) = false */
                                        let fresh351 = pc;
                                        pc = pc.offset(1);
                                        a = *fresh351 as uint32_t;
                                        current_block = 4459663504651627985;
                                    }
                                    19 => {
                                        let fresh352 = pc;
                                        pc = pc.offset(1);
                                        a = *fresh352 as uint32_t;
                                        /* R(a) = getglobal(Syms(b)) */
                                        pc = pc.offset(2isize);
                                        b =
                                            ((*pc.offset(-2isize).offset(0isize)
                                                  as libc::c_int) << 8i32 |
                                                 *pc.offset(-2isize).offset(1isize)
                                                     as libc::c_int) as
                                                uint16_t;
                                        current_block = 13479157322803929894;
                                    }
                                    20 => {
                                        let fresh353 = pc;
                                        pc = pc.offset(1);
                                        a = *fresh353 as uint32_t;
                                        /* setglobal(Syms(b), R(a)) */
                                        pc = pc.offset(2isize);
                                        b =
                                            ((*pc.offset(-2isize).offset(0isize)
                                                  as libc::c_int) << 8i32 |
                                                 *pc.offset(-2isize).offset(1isize)
                                                     as libc::c_int) as
                                                uint16_t;
                                        current_block = 3151994457458062110;
                                    }
                                    21 => {
                                        let fresh354 = pc;
                                        pc = pc.offset(1);
                                        a = *fresh354 as uint32_t;
                                        /* R(a) = Special[Syms(b)] */
                                        pc = pc.offset(2isize);
                                        b =
                                            ((*pc.offset(-2isize).offset(0isize)
                                                  as libc::c_int) << 8i32 |
                                                 *pc.offset(-2isize).offset(1isize)
                                                     as libc::c_int) as
                                                uint16_t;
                                        current_block = 14187386403465544025;
                                    }
                                    22 => {
                                        let fresh355 = pc;
                                        pc = pc.offset(1);
                                        a = *fresh355 as uint32_t;
                                        /* Special[Syms(b)] = R(a) */
                                        pc = pc.offset(2isize);
                                        b =
                                            ((*pc.offset(-2isize).offset(0isize)
                                                  as libc::c_int) << 8i32 |
                                                 *pc.offset(-2isize).offset(1isize)
                                                     as libc::c_int) as
                                                uint16_t;
                                        current_block = 9974864727789713748;
                                    }
                                    23 => {
                                        let fresh356 = pc;
                                        pc = pc.offset(1);
                                        a = *fresh356 as uint32_t;
                                        /* R(a) = ivget(Syms(b)) */
                                        pc = pc.offset(2isize);
                                        b =
                                            ((*pc.offset(-2isize).offset(0isize)
                                                  as libc::c_int) << 8i32 |
                                                 *pc.offset(-2isize).offset(1isize)
                                                     as libc::c_int) as
                                                uint16_t;
                                        current_block = 4491581808830814586;
                                    }
                                    24 => {
                                        let fresh357 = pc;
                                        pc = pc.offset(1);
                                        a = *fresh357 as uint32_t;
                                        /* ivset(Syms(b),R(a)) */
                                        pc = pc.offset(2isize);
                                        b =
                                            ((*pc.offset(-2isize).offset(0isize)
                                                  as libc::c_int) << 8i32 |
                                                 *pc.offset(-2isize).offset(1isize)
                                                     as libc::c_int) as
                                                uint16_t;
                                        current_block = 12788783625999190409;
                                    }
                                    25 => {
                                        let fresh358 = pc;
                                        pc = pc.offset(1);
                                        a = *fresh358 as uint32_t;
                                        /* R(a) = cvget(Syms(b)) */
                                        pc = pc.offset(2isize);
                                        b =
                                            ((*pc.offset(-2isize).offset(0isize)
                                                  as libc::c_int) << 8i32 |
                                                 *pc.offset(-2isize).offset(1isize)
                                                     as libc::c_int) as
                                                uint16_t;
                                        current_block = 3906822848181906220;
                                    }
                                    26 => {
                                        let fresh359 = pc;
                                        pc = pc.offset(1);
                                        a = *fresh359 as uint32_t;
                                        /* cvset(Syms(b),R(a)) */
                                        pc = pc.offset(2isize);
                                        b =
                                            ((*pc.offset(-2isize).offset(0isize)
                                                  as libc::c_int) << 8i32 |
                                                 *pc.offset(-2isize).offset(1isize)
                                                     as libc::c_int) as
                                                uint16_t;
                                        current_block = 1105701036774490218;
                                    }
                                    27 => {
                                        let fresh360 = pc;
                                        pc = pc.offset(1);
                                        a = *fresh360 as uint32_t;
                                        /* R(a) = constget(Syms(b)) */
                                        pc = pc.offset(2isize);
                                        b =
                                            ((*pc.offset(-2isize).offset(0isize)
                                                  as libc::c_int) << 8i32 |
                                                 *pc.offset(-2isize).offset(1isize)
                                                     as libc::c_int) as
                                                uint16_t;
                                        current_block = 13114814261106982490;
                                    }
                                    28 => {
                                        let fresh361 = pc;
                                        pc = pc.offset(1);
                                        a = *fresh361 as uint32_t;
                                        /* constset(Syms(b),R(a)) */
                                        pc = pc.offset(2isize);
                                        b =
                                            ((*pc.offset(-2isize).offset(0isize)
                                                  as libc::c_int) << 8i32 |
                                                 *pc.offset(-2isize).offset(1isize)
                                                     as libc::c_int) as
                                                uint16_t;
                                        current_block = 6813271534392596583;
                                    }
                                    29 => {
                                        let fresh362 = pc;
                                        pc = pc.offset(1);
                                        a = *fresh362 as uint32_t;
                                        /* R(a) = R(a)::Syms(b) */
                                        pc = pc.offset(2isize);
                                        b =
                                            ((*pc.offset(-2isize).offset(0isize)
                                                  as libc::c_int) << 8i32 |
                                                 *pc.offset(-2isize).offset(1isize)
                                                     as libc::c_int) as
                                                uint16_t;
                                        current_block = 4001239642700071046;
                                    }
                                    30 => {
                                        let fresh363 = pc;
                                        pc = pc.offset(1);
                                        a = *fresh363 as uint32_t;
                                        /* R(a+1)::Syms(b) = R(a) */
                                        pc = pc.offset(2isize);
                                        b =
                                            ((*pc.offset(-2isize).offset(0isize)
                                                  as libc::c_int) << 8i32 |
                                                 *pc.offset(-2isize).offset(1isize)
                                                     as libc::c_int) as
                                                uint16_t;
                                        current_block = 5323028055506826224;
                                    }
                                    31 => {
                                        let fresh364 = pc;
                                        pc = pc.offset(1);
                                        a = *fresh364 as uint32_t;
                                        pc = pc.offset(2isize);
                                        b =
                                            ((*pc.offset(-2isize).offset(0isize)
                                                  as libc::c_int) << 8i32 |
                                                 *pc.offset(-2isize).offset(1isize)
                                                     as libc::c_int) as
                                                uint16_t;
                                        /* R(a) = uvget(b,c) */
                                        let fresh365 = pc;
                                        pc = pc.offset(1);
                                        c = *fresh365;
                                        current_block = 16263365153914704257;
                                    }
                                    32 => {
                                        let fresh366 = pc;
                                        pc = pc.offset(1);
                                        a = *fresh366 as uint32_t;
                                        pc = pc.offset(2isize);
                                        b =
                                            ((*pc.offset(-2isize).offset(0isize)
                                                  as libc::c_int) << 8i32 |
                                                 *pc.offset(-2isize).offset(1isize)
                                                     as libc::c_int) as
                                                uint16_t;
                                        /* uvset(b,c,R(a)) */
                                        let fresh367 = pc;
                                        pc = pc.offset(1);
                                        c = *fresh367;
                                        current_block = 4485224281673279150;
                                    }
                                    33 => {
                                        /* pc=a */
                                        pc = pc.offset(2isize);
                                        a =
                                            ((*pc.offset(-2isize).offset(0isize)
                                                  as libc::c_int) << 8i32 |
                                                 *pc.offset(-2isize).offset(1isize)
                                                     as libc::c_int) as
                                                uint32_t;
                                        current_block = 9422951997864425805;
                                    }
                                    34 => {
                                        let fresh368 = pc;
                                        pc = pc.offset(1);
                                        a = *fresh368 as uint32_t;
                                        /* if R(b) pc=a */
                                        pc = pc.offset(2isize);
                                        b =
                                            ((*pc.offset(-2isize).offset(0isize)
                                                  as libc::c_int) << 8i32 |
                                                 *pc.offset(-2isize).offset(1isize)
                                                     as libc::c_int) as
                                                uint16_t;
                                        current_block = 1297461190301222800;
                                    }
                                    35 => {
                                        let fresh369 = pc;
                                        pc = pc.offset(1);
                                        a = *fresh369 as uint32_t;
                                        /* if !R(b) pc=a */
                                        pc = pc.offset(2isize);
                                        b =
                                            ((*pc.offset(-2isize).offset(0isize)
                                                  as libc::c_int) << 8i32 |
                                                 *pc.offset(-2isize).offset(1isize)
                                                     as libc::c_int) as
                                                uint16_t;
                                        current_block = 11208766757666257413;
                                    }
                                    36 => {
                                        let fresh370 = pc;
                                        pc = pc.offset(1);
                                        a = *fresh370 as uint32_t;
                                        /* if R(b)==nil pc=a */
                                        pc = pc.offset(2isize);
                                        b =
                                            ((*pc.offset(-2isize).offset(0isize)
                                                  as libc::c_int) << 8i32 |
                                                 *pc.offset(-2isize).offset(1isize)
                                                     as libc::c_int) as
                                                uint16_t;
                                        current_block = 14166554486324432560;
                                    }
                                    37 => {
                                        /* rescue_push(a) */
                                        pc = pc.offset(2isize);
                                        a =
                                            ((*pc.offset(-2isize).offset(0isize)
                                                  as libc::c_int) << 8i32 |
                                                 *pc.offset(-2isize).offset(1isize)
                                                     as libc::c_int) as
                                                uint32_t;
                                        current_block = 12913164539471242988;
                                    }
                                    38 => {
                                        /* R(a) = exc */
                                        let fresh371 = pc;
                                        pc = pc.offset(1);
                                        a = *fresh371 as uint32_t;
                                        current_block = 2722617303989997926;
                                    }
                                    39 => {
                                        let fresh372 = pc;
                                        pc = pc.offset(1);
                                        a = *fresh372 as uint32_t;
                                        /* R(b) = R(a).isa?(R(b)) */
                                        pc = pc.offset(2isize);
                                        b =
                                            ((*pc.offset(-2isize).offset(0isize)
                                                  as libc::c_int) << 8i32 |
                                                 *pc.offset(-2isize).offset(1isize)
                                                     as libc::c_int) as
                                                uint16_t;
                                        current_block = 16261244098005255386;
                                    }
                                    40 => {
                                        /* a.times{rescue_pop()} */
                                        let fresh373 = pc;
                                        pc = pc.offset(1);
                                        a = *fresh373 as uint32_t;
                                        current_block = 14933967110489461578;
                                    }
                                    41 => {
                                        /* raise(R(a)) */
                                        let fresh374 = pc;
                                        pc = pc.offset(1);
                                        a = *fresh374 as uint32_t;
                                        current_block = 85722159537708186;
                                    }
                                    42 => {
                                        /* ensure_push(SEQ[a]) */
                                        let fresh375 = pc;
                                        pc = pc.offset(1);
                                        a = *fresh375 as uint32_t;
                                        current_block = 17178928522929307677;
                                    }
                                    43 => {
                                        /* A.times{ensure_pop().call} */
                                        let fresh376 = pc;
                                        pc = pc.offset(1);
                                        a = *fresh376 as uint32_t;
                                        current_block = 8592218650707875588;
                                    }
                                    44 => {
                                        let fresh377 = pc;
                                        pc = pc.offset(1);
                                        a = *fresh377 as uint32_t;
                                        /* R(a) = call(R(a),Syms(b),*R(a+1)) */
                                        pc = pc.offset(2isize);
                                        b =
                                            ((*pc.offset(-2isize).offset(0isize)
                                                  as libc::c_int) << 8i32 |
                                                 *pc.offset(-2isize).offset(1isize)
                                                     as libc::c_int) as
                                                uint16_t;
                                        current_block = 12662816978892980002;
                                    }
                                    45 => {
                                        let fresh378 = pc;
                                        pc = pc.offset(1);
                                        a = *fresh378 as uint32_t;
                                        /* R(a) = call(R(a),Syms(b),*R(a+1),&R(a+2)) */
                                        pc = pc.offset(2isize);
                                        b =
                                            ((*pc.offset(-2isize).offset(0isize)
                                                  as libc::c_int) << 8i32 |
                                                 *pc.offset(-2isize).offset(1isize)
                                                     as libc::c_int) as
                                                uint16_t;
                                        current_block = 7460542724658431689;
                                    }
                                    46 => {
                                        let fresh379 = pc;
                                        pc = pc.offset(1);
                                        a = *fresh379 as uint32_t;
                                        pc = pc.offset(2isize);
                                        b =
                                            ((*pc.offset(-2isize).offset(0isize)
                                                  as libc::c_int) << 8i32 |
                                                 *pc.offset(-2isize).offset(1isize)
                                                     as libc::c_int) as
                                                uint16_t;
                                        /* R(a) = call(R(a),Syms(b),R(a+1),...,R(a+c)) */
                                        let fresh380 = pc;
                                        pc = pc.offset(1);
                                        c = *fresh380;
                                        current_block = 7304850410330992871;
                                    }
                                    47 => {
                                        let fresh381 = pc;
                                        pc = pc.offset(1);
                                        a = *fresh381 as uint32_t;
                                        pc = pc.offset(2isize);
                                        b =
                                            ((*pc.offset(-2isize).offset(0isize)
                                                  as libc::c_int) << 8i32 |
                                                 *pc.offset(-2isize).offset(1isize)
                                                     as libc::c_int) as
                                                uint16_t;
                                        /* R(a) = call(R(a),Syms(Bx),R(a+1),...,R(a+c),&R(a+c+1)) */
                                        let fresh382 = pc;
                                        pc = pc.offset(1);
                                        c = *fresh382;
                                        current_block = 10790764747546649571;
                                    }
                                    48 => {
                                        current_block = 13698748673986354976;
                                    }
                                    49 => {
                                        let fresh383 = pc;
                                        pc = pc.offset(1);
                                        a = *fresh383 as uint32_t;
                                        /* R(a) = super(R(a+1),... ,R(a+b+1)) */
                                        pc = pc.offset(2isize);
                                        b =
                                            ((*pc.offset(-2isize).offset(0isize)
                                                  as libc::c_int) << 8i32 |
                                                 *pc.offset(-2isize).offset(1isize)
                                                     as libc::c_int) as
                                                uint16_t;
                                        current_block = 9607585333290054341;
                                    }
                                    50 => {
                                        let fresh384 = pc;
                                        pc = pc.offset(1);
                                        a = *fresh384 as uint32_t;
                                        /* R(a) = argument array (16=m5:r1:m5:d1:lv4) */
                                        pc = pc.offset(2isize);
                                        b =
                                            ((*pc.offset(-2isize).offset(0isize)
                                                  as libc::c_int) << 8i32 |
                                                 *pc.offset(-2isize).offset(1isize)
                                                     as libc::c_int) as
                                                uint16_t;
                                        current_block = 10763054988551342546;
                                    }
                                    51 => {
                                        /* arg setup according to flags (23=m5:o5:r1:m5:k5:d1:b1) */
                                        pc = pc.offset(3isize);
                                        a =
                                            ((*pc.offset(-3isize).offset(0isize)
                                                  as libc::c_int) << 16i32 |
                                                 (*pc.offset(-3isize).offset(1isize)
                                                      as libc::c_int) << 8i32
                                                 |
                                                 *pc.offset(-3isize).offset(2isize)
                                                     as libc::c_int) as
                                                uint32_t;
                                        current_block = 15803687294481920802;
                                    }
                                    52 => {
                                        let fresh385 = pc;
                                        pc = pc.offset(1);
                                        a = *fresh385 as uint32_t;
                                        /* R(a) = kdict.key?(Syms(b))                      # todo */
                                        pc = pc.offset(2isize);
                                        b =
                                            ((*pc.offset(-2isize).offset(0isize)
                                                  as libc::c_int) << 8i32 |
                                                 *pc.offset(-2isize).offset(1isize)
                                                     as libc::c_int) as
                                                uint16_t;
                                        current_block = 1525421113118994726;
                                    }
                                    53 => {
                                        current_block = 4663630632216779857;
                                    }
                                    54 => {
                                        let fresh386 = pc;
                                        pc = pc.offset(1);
                                        a = *fresh386 as uint32_t;
                                        /* R(a) = kdict[Syms(b)]; kdict.delete(Syms(b))    # todo */
                                        pc = pc.offset(2isize);
                                        b =
                                            ((*pc.offset(-2isize).offset(0isize)
                                                  as libc::c_int) << 8i32 |
                                                 *pc.offset(-2isize).offset(1isize)
                                                     as libc::c_int) as
                                                uint16_t;
                                        current_block = 14765652484648187635;
                                    }
                                    55 => {
                                        /* return R(a) (normal) */
                                        let fresh387 = pc;
                                        pc = pc.offset(1);
                                        a = *fresh387 as uint32_t;
                                        current_block = 14547135203946798640;
                                    }
                                    56 => {
                                        /* return R(a) (in-block return) */
                                        let fresh388 = pc;
                                        pc = pc.offset(1);
                                        a = *fresh388 as uint32_t;
                                        current_block = 13114312767422449834;
                                    }
                                    57 => {
                                        /* break R(a) */
                                        let fresh389 = pc;
                                        pc = pc.offset(1);
                                        a = *fresh389 as uint32_t;
                                        current_block = 12223856006808243146;
                                    }
                                    58 => {
                                        let fresh390 = pc;
                                        pc = pc.offset(1);
                                        a = *fresh390 as uint32_t;
                                        /* R(a) = block (16=m5:r1:m5:d1:lv4) */
                                        pc = pc.offset(2isize);
                                        b =
                                            ((*pc.offset(-2isize).offset(0isize)
                                                  as libc::c_int) << 8i32 |
                                                 *pc.offset(-2isize).offset(1isize)
                                                     as libc::c_int) as
                                                uint16_t;
                                        current_block = 2994422791481536047;
                                    }
                                    59 => {
                                        /* R(a) = R(a)+R(a+1) */
                                        let fresh391 = pc;
                                        pc = pc.offset(1);
                                        a = *fresh391 as uint32_t;
                                        current_block = 16140473650108643456;
                                    }
                                    60 => {
                                        let fresh392 = pc;
                                        pc = pc.offset(1);
                                        a = *fresh392 as uint32_t;
                                        /* R(a) = R(a)+mrb_int(c)  */
                                        pc = pc.offset(2isize);
                                        b =
                                            ((*pc.offset(-2isize).offset(0isize)
                                                  as libc::c_int) << 8i32 |
                                                 *pc.offset(-2isize).offset(1isize)
                                                     as libc::c_int) as
                                                uint16_t;
                                        current_block = 4396809944207673246;
                                    }
                                    61 => {
                                        /* R(a) = R(a)-R(a+1) */
                                        let fresh393 = pc;
                                        pc = pc.offset(1);
                                        a = *fresh393 as uint32_t;
                                        current_block = 10653245833193402520;
                                    }
                                    62 => {
                                        let fresh394 = pc;
                                        pc = pc.offset(1);
                                        a = *fresh394 as uint32_t;
                                        /* R(a) = R(a)-C */
                                        pc = pc.offset(2isize);
                                        b =
                                            ((*pc.offset(-2isize).offset(0isize)
                                                  as libc::c_int) << 8i32 |
                                                 *pc.offset(-2isize).offset(1isize)
                                                     as libc::c_int) as
                                                uint16_t;
                                        current_block = 1767406316152665813;
                                    }
                                    63 => {
                                        /* R(a) = R(a)*R(a+1) */
                                        let fresh395 = pc;
                                        pc = pc.offset(1);
                                        a = *fresh395 as uint32_t;
                                        current_block = 2558910614333069445;
                                    }
                                    64 => {
                                        /* R(a) = R(a)/R(a+1) */
                                        let fresh396 = pc;
                                        pc = pc.offset(1);
                                        a = *fresh396 as uint32_t;
                                        current_block = 15842299444801573850;
                                    }
                                    65 => {
                                        /* R(a) = R(a)==R(a+1) */
                                        let fresh397 = pc;
                                        pc = pc.offset(1);
                                        a = *fresh397 as uint32_t;
                                        current_block = 10264557722339789830;
                                    }
                                    66 => {
                                        /* R(a) = R(a)<R(a+1) */
                                        let fresh398 = pc;
                                        pc = pc.offset(1);
                                        a = *fresh398 as uint32_t;
                                        current_block = 11600479572347497286;
                                    }
                                    67 => {
                                        /* R(a) = R(a)<=R(a+1) */
                                        let fresh399 = pc;
                                        pc = pc.offset(1);
                                        a = *fresh399 as uint32_t;
                                        current_block = 6268356626658084969;
                                    }
                                    68 => {
                                        /* R(a) = R(a)>R(a+1) */
                                        let fresh400 = pc;
                                        pc = pc.offset(1);
                                        a = *fresh400 as uint32_t;
                                        current_block = 14064792543554720178;
                                    }
                                    69 => {
                                        /* R(a) = R(a)>=R(a+1) */
                                        let fresh401 = pc;
                                        pc = pc.offset(1);
                                        a = *fresh401 as uint32_t;
                                        current_block = 12168003877939815570;
                                    }
                                    70 => {
                                        let fresh402 = pc;
                                        pc = pc.offset(1);
                                        a = *fresh402 as uint32_t;
                                        /* R(a) = ary_new(R(a),R(a+1)..R(a+b)) */
                                        pc = pc.offset(2isize);
                                        b =
                                            ((*pc.offset(-2isize).offset(0isize)
                                                  as libc::c_int) << 8i32 |
                                                 *pc.offset(-2isize).offset(1isize)
                                                     as libc::c_int) as
                                                uint16_t;
                                        current_block = 5334851501950231651;
                                    }
                                    71 => {
                                        let fresh403 = pc;
                                        pc = pc.offset(1);
                                        a = *fresh403 as uint32_t;
                                        pc = pc.offset(2isize);
                                        b =
                                            ((*pc.offset(-2isize).offset(0isize)
                                                  as libc::c_int) << 8i32 |
                                                 *pc.offset(-2isize).offset(1isize)
                                                     as libc::c_int) as
                                                uint16_t;
                                        /* R(a) = ary_new(R(b),R(b+1)..R(b+c)) */
                                        let fresh404 = pc;
                                        pc = pc.offset(1);
                                        c = *fresh404;
                                        current_block = 15590445366052810487;
                                    }
                                    72 => {
                                        /* ary_cat(R(a),R(a+1)) */
                                        let fresh405 = pc;
                                        pc = pc.offset(1);
                                        a = *fresh405 as uint32_t;
                                        current_block = 15951340130516588981;
                                    }
                                    73 => {
                                        /* ary_push(R(a),R(a+1)) */
                                        let fresh406 = pc;
                                        pc = pc.offset(1);
                                        a = *fresh406 as uint32_t;
                                        current_block = 17884754765720769523;
                                    }
                                    74 => {
                                        /* R(a) = ary_dup(R(a)) */
                                        let fresh407 = pc;
                                        pc = pc.offset(1);
                                        a = *fresh407 as uint32_t;
                                        current_block = 6985669818981290717;
                                    }
                                    75 => {
                                        let fresh408 = pc;
                                        pc = pc.offset(1);
                                        a = *fresh408 as uint32_t;
                                        pc = pc.offset(2isize);
                                        b =
                                            ((*pc.offset(-2isize).offset(0isize)
                                                  as libc::c_int) << 8i32 |
                                                 *pc.offset(-2isize).offset(1isize)
                                                     as libc::c_int) as
                                                uint16_t;
                                        /* R(a) = R(b)[c] */
                                        let fresh409 = pc;
                                        pc = pc.offset(1);
                                        c = *fresh409;
                                        current_block = 10370658347790963364;
                                    }
                                    76 => {
                                        let fresh410 = pc;
                                        pc = pc.offset(1);
                                        a = *fresh410 as uint32_t;
                                        pc = pc.offset(2isize);
                                        b =
                                            ((*pc.offset(-2isize).offset(0isize)
                                                  as libc::c_int) << 8i32 |
                                                 *pc.offset(-2isize).offset(1isize)
                                                     as libc::c_int) as
                                                uint16_t;
                                        /* R(a)[c] = R(b) */
                                        let fresh411 = pc;
                                        pc = pc.offset(1);
                                        c = *fresh411;
                                        current_block = 4938714451943331475;
                                    }
                                    77 => {
                                        let fresh412 = pc;
                                        pc = pc.offset(1);
                                        a = *fresh412 as uint32_t;
                                        pc = pc.offset(2isize);
                                        b =
                                            ((*pc.offset(-2isize).offset(0isize)
                                                  as libc::c_int) << 8i32 |
                                                 *pc.offset(-2isize).offset(1isize)
                                                     as libc::c_int) as
                                                uint16_t;
                                        /* *R(a),R(a+1)..R(a+c) = R(a)[b..] */
                                        let fresh413 = pc;
                                        pc = pc.offset(1);
                                        c = *fresh413;
                                        current_block = 11125061941892892926;
                                    }
                                    78 => {
                                        /* R(a) = intern(R(a)) */
                                        let fresh414 = pc;
                                        pc = pc.offset(1);
                                        a = *fresh414 as uint32_t;
                                        current_block = 2896230971687472890;
                                    }
                                    79 => {
                                        let fresh415 = pc;
                                        pc = pc.offset(1);
                                        a = *fresh415 as uint32_t;
                                        /* R(a) = str_dup(Lit(b)) */
                                        pc = pc.offset(2isize);
                                        b =
                                            ((*pc.offset(-2isize).offset(0isize)
                                                  as libc::c_int) << 8i32 |
                                                 *pc.offset(-2isize).offset(1isize)
                                                     as libc::c_int) as
                                                uint16_t;
                                        current_block = 14841647832940836303;
                                    }
                                    80 => {
                                        /* str_cat(R(a),R(a+1)) */
                                        let fresh416 = pc;
                                        pc = pc.offset(1);
                                        a = *fresh416 as uint32_t;
                                        current_block = 4191712132543108294;
                                    }
                                    81 => {
                                        let fresh417 = pc;
                                        pc = pc.offset(1);
                                        a = *fresh417 as uint32_t;
                                        /* R(a) = hash_new(R(a),R(a+1)..R(a+b)) */
                                        pc = pc.offset(2isize);
                                        b =
                                            ((*pc.offset(-2isize).offset(0isize)
                                                  as libc::c_int) << 8i32 |
                                                 *pc.offset(-2isize).offset(1isize)
                                                     as libc::c_int) as
                                                uint16_t;
                                        current_block = 1686938961607716371;
                                    }
                                    82 => {
                                        let fresh418 = pc;
                                        pc = pc.offset(1);
                                        a = *fresh418 as uint32_t;
                                        /* R(a) = hash_push(R(a),R(a+1)..R(a+b)) */
                                        pc = pc.offset(2isize);
                                        b =
                                            ((*pc.offset(-2isize).offset(0isize)
                                                  as libc::c_int) << 8i32 |
                                                 *pc.offset(-2isize).offset(1isize)
                                                     as libc::c_int) as
                                                uint16_t;
                                        current_block = 17492450365994263002;
                                    }
                                    83 => {
                                        /* R(a) = hash_cat(R(a),R(a+1)) */
                                        let fresh419 = pc;
                                        pc = pc.offset(1);
                                        a = *fresh419 as uint32_t;
                                        current_block = 15495526794150689403;
                                    }
                                    84 => {
                                        let fresh420 = pc;
                                        pc = pc.offset(1);
                                        a = *fresh420 as uint32_t;
                                        /* R(a) = lambda(SEQ[b],L_LAMBDA) */
                                        pc = pc.offset(2isize);
                                        b =
                                            ((*pc.offset(-2isize).offset(0isize)
                                                  as libc::c_int) << 8i32 |
                                                 *pc.offset(-2isize).offset(1isize)
                                                     as libc::c_int) as
                                                uint16_t;
                                        current_block = 2702902889507381978;
                                    }
                                    85 => {
                                        let fresh421 = pc;
                                        pc = pc.offset(1);
                                        a = *fresh421 as uint32_t;
                                        /* R(a) = lambda(SEQ[b],L_BLOCK) */
                                        pc = pc.offset(2isize);
                                        b =
                                            ((*pc.offset(-2isize).offset(0isize)
                                                  as libc::c_int) << 8i32 |
                                                 *pc.offset(-2isize).offset(1isize)
                                                     as libc::c_int) as
                                                uint16_t;
                                        current_block = 1152971280698483565;
                                    }
                                    86 => {
                                        let fresh422 = pc;
                                        pc = pc.offset(1);
                                        a = *fresh422 as uint32_t;
                                        /* R(a) = lambda(SEQ[b],L_METHOD) */
                                        pc = pc.offset(2isize);
                                        b =
                                            ((*pc.offset(-2isize).offset(0isize)
                                                  as libc::c_int) << 8i32 |
                                                 *pc.offset(-2isize).offset(1isize)
                                                     as libc::c_int) as
                                                uint16_t;
                                        current_block = 16224973743324382762;
                                    }
                                    87 => {
                                        /* R(a) = range_new(R(a),R(a+1),FALSE) */
                                        let fresh423 = pc;
                                        pc = pc.offset(1);
                                        a = *fresh423 as uint32_t;
                                        current_block = 6571067991607217453;
                                    }
                                    88 => {
                                        /* R(a) = range_new(R(a),R(a+1),TRUE) */
                                        let fresh424 = pc;
                                        pc = pc.offset(1);
                                        a = *fresh424 as uint32_t;
                                        current_block = 14502557129029694510;
                                    }
                                    89 => {
                                        /* R(a) = ::Object */
                                        let fresh425 = pc;
                                        pc = pc.offset(1);
                                        a = *fresh425 as uint32_t;
                                        current_block = 16196049249653527883;
                                    }
                                    90 => {
                                        let fresh426 = pc;
                                        pc = pc.offset(1);
                                        a = *fresh426 as uint32_t;
                                        /* R(a) = newclass(R(a),Syms(b),R(a+1)) */
                                        pc = pc.offset(2isize);
                                        b =
                                            ((*pc.offset(-2isize).offset(0isize)
                                                  as libc::c_int) << 8i32 |
                                                 *pc.offset(-2isize).offset(1isize)
                                                     as libc::c_int) as
                                                uint16_t;
                                        current_block = 16736254050004001558;
                                    }
                                    91 => {
                                        let fresh427 = pc;
                                        pc = pc.offset(1);
                                        a = *fresh427 as uint32_t;
                                        /* R(a) = newmodule(R(a),Syms(b)) */
                                        pc = pc.offset(2isize);
                                        b =
                                            ((*pc.offset(-2isize).offset(0isize)
                                                  as libc::c_int) << 8i32 |
                                                 *pc.offset(-2isize).offset(1isize)
                                                     as libc::c_int) as
                                                uint16_t;
                                        current_block = 16407897916814827188;
                                    }
                                    92 => {
                                        let fresh428 = pc;
                                        pc = pc.offset(1);
                                        a = *fresh428 as uint32_t;
                                        /* R(a) = blockexec(R(a),SEQ[b]) */
                                        pc = pc.offset(2isize);
                                        b =
                                            ((*pc.offset(-2isize).offset(0isize)
                                                  as libc::c_int) << 8i32 |
                                                 *pc.offset(-2isize).offset(1isize)
                                                     as libc::c_int) as
                                                uint16_t;
                                        current_block = 9892557545450190862;
                                    }
                                    93 => {
                                        let fresh429 = pc;
                                        pc = pc.offset(1);
                                        a = *fresh429 as uint32_t;
                                        /* R(a).newmethod(Syms(b),R(a+1)) */
                                        pc = pc.offset(2isize);
                                        b =
                                            ((*pc.offset(-2isize).offset(0isize)
                                                  as libc::c_int) << 8i32 |
                                                 *pc.offset(-2isize).offset(1isize)
                                                     as libc::c_int) as
                                                uint16_t;
                                        current_block = 15928390647053518202;
                                    }
                                    94 => {
                                        let fresh430 = pc;
                                        pc = pc.offset(1);
                                        a = *fresh430 as uint32_t;
                                        /* alias_method(target_class,Syms(a),Syms(b)) */
                                        pc = pc.offset(2isize);
                                        b =
                                            ((*pc.offset(-2isize).offset(0isize)
                                                  as libc::c_int) << 8i32 |
                                                 *pc.offset(-2isize).offset(1isize)
                                                     as libc::c_int) as
                                                uint16_t;
                                        current_block = 1656069329997846098;
                                    }
                                    95 => {
                                        /* undef_method(target_class,Syms(a)) */
                                        let fresh431 = pc;
                                        pc = pc.offset(1);
                                        a = *fresh431 as uint32_t;
                                        current_block = 5769344152980800111;
                                    }
                                    96 => {
                                        /* R(a) = R(a).singleton_class */
                                        let fresh432 = pc;
                                        pc = pc.offset(1);
                                        a = *fresh432 as uint32_t;
                                        current_block = 15212051862008919012;
                                    }
                                    97 => {
                                        /* R(a) = target_class */
                                        let fresh433 = pc;
                                        pc = pc.offset(1);
                                        a = *fresh433 as uint32_t;
                                        current_block = 15561344248022367768;
                                    }
                                    98 => {
                                        let fresh434 = pc;
                                        pc = pc.offset(1);
                                        a = *fresh434 as uint32_t;
                                        pc = pc.offset(2isize);
                                        b =
                                            ((*pc.offset(-2isize).offset(0isize)
                                                  as libc::c_int) << 8i32 |
                                                 *pc.offset(-2isize).offset(1isize)
                                                     as libc::c_int) as
                                                uint16_t;
                                        /* print a,b,c */
                                        let fresh435 = pc;
                                        pc = pc.offset(1);
                                        c = *fresh435;
                                        current_block = 10071922767614739690;
                                        break 'c_4688 ;
                                    }
                                    99 => {
                                        /* raise(LocalJumpError, Lit(a)) */
                                        let fresh436 = pc;
                                        pc = pc.offset(1);
                                        a = *fresh436 as uint32_t;
                                        current_block = 9217466010932042731;
                                    }
                                    100 => {
                                        current_block = 16136094826069225138;
                                    }
                                    101 => {
                                        current_block = 12853241758993110625;
                                    }
                                    102 => {
                                        current_block = 3106153520265210690;
                                    }
                                    103 => {
                                        current_block = 5128590269704079499;
                                        break 'c_4688 ;
                                    }
                                    _ => {
                                        pc = pc.offset(-1isize);
                                        current_block = 7149356873433890176;
                                        continue 'c_4688 ;
                                    }
                                }
                            }
                            16136094826069225138 => {
                                let fresh276 = pc;
                                pc = pc.offset(1);
                                insn = *fresh276;
                                match insn as libc::c_int {
                                    0 => {
                                        current_block = 14576567515993809846;
                                    }
                                    1 => {
                                        pc = pc.offset(2isize);
                                        a =
                                            ((*pc.offset(-2isize).offset(0isize)
                                                  as libc::c_int) << 8i32 |
                                                 *pc.offset(-2isize).offset(1isize)
                                                     as libc::c_int) as
                                                uint32_t;
                                        /* R(a) = R(b) */
                                        let fresh277 = pc;
                                        pc = pc.offset(1);
                                        b = *fresh277 as uint16_t;
                                        current_block = 4488286894823169796;
                                    }
                                    2 => {
                                        pc = pc.offset(2isize);
                                        a =
                                            ((*pc.offset(-2isize).offset(0isize)
                                                  as libc::c_int) << 8i32 |
                                                 *pc.offset(-2isize).offset(1isize)
                                                     as libc::c_int) as
                                                uint32_t;
                                        /* R(a) = Pool(b) */
                                        let fresh278 = pc;
                                        pc = pc.offset(1);
                                        b = *fresh278 as uint16_t;
                                        current_block = 8845338526596852646;
                                    }
                                    3 => {
                                        pc = pc.offset(2isize);
                                        a =
                                            ((*pc.offset(-2isize).offset(0isize)
                                                  as libc::c_int) << 8i32 |
                                                 *pc.offset(-2isize).offset(1isize)
                                                     as libc::c_int) as
                                                uint32_t;
                                        /* R(a) = mrb_int(b) */
                                        let fresh279 = pc;
                                        pc = pc.offset(1);
                                        b = *fresh279 as uint16_t;
                                        current_block = 14447253356787937536;
                                    }
                                    4 => {
                                        pc = pc.offset(2isize);
                                        a =
                                            ((*pc.offset(-2isize).offset(0isize)
                                                  as libc::c_int) << 8i32 |
                                                 *pc.offset(-2isize).offset(1isize)
                                                     as libc::c_int) as
                                                uint32_t;
                                        /* R(a) = mrb_int(-b) */
                                        let fresh280 = pc;
                                        pc = pc.offset(1);
                                        b = *fresh280 as uint16_t;
                                        current_block = 7343950298149844727;
                                    }
                                    5 => {
                                        /* R(a) = mrb_int(-1) */
                                        pc = pc.offset(2isize);
                                        a =
                                            ((*pc.offset(-2isize).offset(0isize)
                                                  as libc::c_int) << 8i32 |
                                                 *pc.offset(-2isize).offset(1isize)
                                                     as libc::c_int) as
                                                uint32_t;
                                        current_block = 14563621082360312968;
                                    }
                                    6 => {
                                        /* R(a) = mrb_int(0) */
                                        pc = pc.offset(2isize);
                                        a =
                                            ((*pc.offset(-2isize).offset(0isize)
                                                  as libc::c_int) << 8i32 |
                                                 *pc.offset(-2isize).offset(1isize)
                                                     as libc::c_int) as
                                                uint32_t;
                                        current_block = 14563621082360312968;
                                    }
                                    7 => {
                                        /* R(a) = mrb_int(1) */
                                        pc = pc.offset(2isize);
                                        a =
                                            ((*pc.offset(-2isize).offset(0isize)
                                                  as libc::c_int) << 8i32 |
                                                 *pc.offset(-2isize).offset(1isize)
                                                     as libc::c_int) as
                                                uint32_t;
                                        current_block = 14563621082360312968;
                                    }
                                    8 => {
                                        /* R(a) = mrb_int(2) */
                                        pc = pc.offset(2isize);
                                        a =
                                            ((*pc.offset(-2isize).offset(0isize)
                                                  as libc::c_int) << 8i32 |
                                                 *pc.offset(-2isize).offset(1isize)
                                                     as libc::c_int) as
                                                uint32_t;
                                        current_block = 14563621082360312968;
                                    }
                                    9 => {
                                        /* R(a) = mrb_int(3) */
                                        pc = pc.offset(2isize);
                                        a =
                                            ((*pc.offset(-2isize).offset(0isize)
                                                  as libc::c_int) << 8i32 |
                                                 *pc.offset(-2isize).offset(1isize)
                                                     as libc::c_int) as
                                                uint32_t;
                                        current_block = 14563621082360312968;
                                    }
                                    10 => {
                                        /* R(a) = mrb_int(4) */
                                        pc = pc.offset(2isize);
                                        a =
                                            ((*pc.offset(-2isize).offset(0isize)
                                                  as libc::c_int) << 8i32 |
                                                 *pc.offset(-2isize).offset(1isize)
                                                     as libc::c_int) as
                                                uint32_t;
                                        current_block = 14563621082360312968;
                                    }
                                    11 => {
                                        /* R(a) = mrb_int(5) */
                                        pc = pc.offset(2isize);
                                        a =
                                            ((*pc.offset(-2isize).offset(0isize)
                                                  as libc::c_int) << 8i32 |
                                                 *pc.offset(-2isize).offset(1isize)
                                                     as libc::c_int) as
                                                uint32_t;
                                        current_block = 14563621082360312968;
                                    }
                                    12 => {
                                        /* R(a) = mrb_int(6) */
                                        pc = pc.offset(2isize);
                                        a =
                                            ((*pc.offset(-2isize).offset(0isize)
                                                  as libc::c_int) << 8i32 |
                                                 *pc.offset(-2isize).offset(1isize)
                                                     as libc::c_int) as
                                                uint32_t;
                                        current_block = 14563621082360312968;
                                    }
                                    13 => {
                                        /* R(a) = mrb_int(7) */
                                        pc = pc.offset(2isize);
                                        a =
                                            ((*pc.offset(-2isize).offset(0isize)
                                                  as libc::c_int) << 8i32 |
                                                 *pc.offset(-2isize).offset(1isize)
                                                     as libc::c_int) as
                                                uint32_t;
                                        current_block = 14563621082360312968;
                                    }
                                    14 => {
                                        pc = pc.offset(2isize);
                                        a =
                                            ((*pc.offset(-2isize).offset(0isize)
                                                  as libc::c_int) << 8i32 |
                                                 *pc.offset(-2isize).offset(1isize)
                                                     as libc::c_int) as
                                                uint32_t;
                                        /* R(a) = Syms(b) */
                                        let fresh281 = pc;
                                        pc = pc.offset(1);
                                        b = *fresh281 as uint16_t;
                                        current_block = 16314074004867283505;
                                    }
                                    15 => {
                                        /* R(a) = nil */
                                        pc = pc.offset(2isize);
                                        a =
                                            ((*pc.offset(-2isize).offset(0isize)
                                                  as libc::c_int) << 8i32 |
                                                 *pc.offset(-2isize).offset(1isize)
                                                     as libc::c_int) as
                                                uint32_t;
                                        current_block = 18201902862271706575;
                                    }
                                    16 => {
                                        /* R(a) = self */
                                        pc = pc.offset(2isize);
                                        a =
                                            ((*pc.offset(-2isize).offset(0isize)
                                                  as libc::c_int) << 8i32 |
                                                 *pc.offset(-2isize).offset(1isize)
                                                     as libc::c_int) as
                                                uint32_t;
                                        current_block = 16974974966130203269;
                                    }
                                    17 => {
                                        /* R(a) = true */
                                        pc = pc.offset(2isize);
                                        a =
                                            ((*pc.offset(-2isize).offset(0isize)
                                                  as libc::c_int) << 8i32 |
                                                 *pc.offset(-2isize).offset(1isize)
                                                     as libc::c_int) as
                                                uint32_t;
                                        current_block = 4976922244085895320;
                                    }
                                    18 => {
                                        /* R(a) = false */
                                        pc = pc.offset(2isize);
                                        a =
                                            ((*pc.offset(-2isize).offset(0isize)
                                                  as libc::c_int) << 8i32 |
                                                 *pc.offset(-2isize).offset(1isize)
                                                     as libc::c_int) as
                                                uint32_t;
                                        current_block = 4459663504651627985;
                                    }
                                    19 => {
                                        pc = pc.offset(2isize);
                                        a =
                                            ((*pc.offset(-2isize).offset(0isize)
                                                  as libc::c_int) << 8i32 |
                                                 *pc.offset(-2isize).offset(1isize)
                                                     as libc::c_int) as
                                                uint32_t;
                                        /* R(a) = getglobal(Syms(b)) */
                                        let fresh282 = pc;
                                        pc = pc.offset(1);
                                        b = *fresh282 as uint16_t;
                                        current_block = 13479157322803929894;
                                    }
                                    20 => {
                                        pc = pc.offset(2isize);
                                        a =
                                            ((*pc.offset(-2isize).offset(0isize)
                                                  as libc::c_int) << 8i32 |
                                                 *pc.offset(-2isize).offset(1isize)
                                                     as libc::c_int) as
                                                uint32_t;
                                        /* setglobal(Syms(b), R(a)) */
                                        let fresh283 = pc;
                                        pc = pc.offset(1);
                                        b = *fresh283 as uint16_t;
                                        current_block = 3151994457458062110;
                                    }
                                    21 => {
                                        pc = pc.offset(2isize);
                                        a =
                                            ((*pc.offset(-2isize).offset(0isize)
                                                  as libc::c_int) << 8i32 |
                                                 *pc.offset(-2isize).offset(1isize)
                                                     as libc::c_int) as
                                                uint32_t;
                                        /* R(a) = Special[Syms(b)] */
                                        let fresh284 = pc;
                                        pc = pc.offset(1);
                                        b = *fresh284 as uint16_t;
                                        current_block = 14187386403465544025;
                                    }
                                    22 => {
                                        pc = pc.offset(2isize);
                                        a =
                                            ((*pc.offset(-2isize).offset(0isize)
                                                  as libc::c_int) << 8i32 |
                                                 *pc.offset(-2isize).offset(1isize)
                                                     as libc::c_int) as
                                                uint32_t;
                                        /* Special[Syms(b)] = R(a) */
                                        let fresh285 = pc;
                                        pc = pc.offset(1);
                                        b = *fresh285 as uint16_t;
                                        current_block = 9974864727789713748;
                                    }
                                    23 => {
                                        pc = pc.offset(2isize);
                                        a =
                                            ((*pc.offset(-2isize).offset(0isize)
                                                  as libc::c_int) << 8i32 |
                                                 *pc.offset(-2isize).offset(1isize)
                                                     as libc::c_int) as
                                                uint32_t;
                                        /* R(a) = ivget(Syms(b)) */
                                        let fresh286 = pc;
                                        pc = pc.offset(1);
                                        b = *fresh286 as uint16_t;
                                        current_block = 4491581808830814586;
                                    }
                                    24 => {
                                        pc = pc.offset(2isize);
                                        a =
                                            ((*pc.offset(-2isize).offset(0isize)
                                                  as libc::c_int) << 8i32 |
                                                 *pc.offset(-2isize).offset(1isize)
                                                     as libc::c_int) as
                                                uint32_t;
                                        /* ivset(Syms(b),R(a)) */
                                        let fresh287 = pc;
                                        pc = pc.offset(1);
                                        b = *fresh287 as uint16_t;
                                        current_block = 12788783625999190409;
                                    }
                                    25 => {
                                        pc = pc.offset(2isize);
                                        a =
                                            ((*pc.offset(-2isize).offset(0isize)
                                                  as libc::c_int) << 8i32 |
                                                 *pc.offset(-2isize).offset(1isize)
                                                     as libc::c_int) as
                                                uint32_t;
                                        /* R(a) = cvget(Syms(b)) */
                                        let fresh288 = pc;
                                        pc = pc.offset(1);
                                        b = *fresh288 as uint16_t;
                                        current_block = 3906822848181906220;
                                    }
                                    26 => {
                                        pc = pc.offset(2isize);
                                        a =
                                            ((*pc.offset(-2isize).offset(0isize)
                                                  as libc::c_int) << 8i32 |
                                                 *pc.offset(-2isize).offset(1isize)
                                                     as libc::c_int) as
                                                uint32_t;
                                        /* cvset(Syms(b),R(a)) */
                                        let fresh289 = pc;
                                        pc = pc.offset(1);
                                        b = *fresh289 as uint16_t;
                                        current_block = 1105701036774490218;
                                    }
                                    27 => {
                                        pc = pc.offset(2isize);
                                        a =
                                            ((*pc.offset(-2isize).offset(0isize)
                                                  as libc::c_int) << 8i32 |
                                                 *pc.offset(-2isize).offset(1isize)
                                                     as libc::c_int) as
                                                uint32_t;
                                        /* R(a) = constget(Syms(b)) */
                                        let fresh290 = pc;
                                        pc = pc.offset(1);
                                        b = *fresh290 as uint16_t;
                                        current_block = 13114814261106982490;
                                    }
                                    28 => {
                                        pc = pc.offset(2isize);
                                        a =
                                            ((*pc.offset(-2isize).offset(0isize)
                                                  as libc::c_int) << 8i32 |
                                                 *pc.offset(-2isize).offset(1isize)
                                                     as libc::c_int) as
                                                uint32_t;
                                        /* constset(Syms(b),R(a)) */
                                        let fresh291 = pc;
                                        pc = pc.offset(1);
                                        b = *fresh291 as uint16_t;
                                        current_block = 6813271534392596583;
                                    }
                                    29 => {
                                        pc = pc.offset(2isize);
                                        a =
                                            ((*pc.offset(-2isize).offset(0isize)
                                                  as libc::c_int) << 8i32 |
                                                 *pc.offset(-2isize).offset(1isize)
                                                     as libc::c_int) as
                                                uint32_t;
                                        /* R(a) = R(a)::Syms(b) */
                                        let fresh292 = pc;
                                        pc = pc.offset(1);
                                        b = *fresh292 as uint16_t;
                                        current_block = 4001239642700071046;
                                    }
                                    30 => {
                                        pc = pc.offset(2isize);
                                        a =
                                            ((*pc.offset(-2isize).offset(0isize)
                                                  as libc::c_int) << 8i32 |
                                                 *pc.offset(-2isize).offset(1isize)
                                                     as libc::c_int) as
                                                uint32_t;
                                        /* R(a+1)::Syms(b) = R(a) */
                                        let fresh293 = pc;
                                        pc = pc.offset(1);
                                        b = *fresh293 as uint16_t;
                                        current_block = 5323028055506826224;
                                    }
                                    31 => {
                                        pc = pc.offset(2isize);
                                        a =
                                            ((*pc.offset(-2isize).offset(0isize)
                                                  as libc::c_int) << 8i32 |
                                                 *pc.offset(-2isize).offset(1isize)
                                                     as libc::c_int) as
                                                uint32_t;
                                        let fresh294 = pc;
                                        pc = pc.offset(1);
                                        b = *fresh294 as uint16_t;
                                        /* R(a) = uvget(b,c) */
                                        let fresh295 = pc;
                                        pc = pc.offset(1);
                                        c = *fresh295;
                                        current_block = 16263365153914704257;
                                    }
                                    32 => {
                                        pc = pc.offset(2isize);
                                        a =
                                            ((*pc.offset(-2isize).offset(0isize)
                                                  as libc::c_int) << 8i32 |
                                                 *pc.offset(-2isize).offset(1isize)
                                                     as libc::c_int) as
                                                uint32_t;
                                        let fresh296 = pc;
                                        pc = pc.offset(1);
                                        b = *fresh296 as uint16_t;
                                        /* uvset(b,c,R(a)) */
                                        let fresh297 = pc;
                                        pc = pc.offset(1);
                                        c = *fresh297;
                                        current_block = 4485224281673279150;
                                    }
                                    33 => {
                                        /* pc=a */
                                        pc = pc.offset(2isize);
                                        a =
                                            ((*pc.offset(-2isize).offset(0isize)
                                                  as libc::c_int) << 8i32 |
                                                 *pc.offset(-2isize).offset(1isize)
                                                     as libc::c_int) as
                                                uint32_t;
                                        current_block = 9422951997864425805;
                                    }
                                    34 => {
                                        pc = pc.offset(2isize);
                                        a =
                                            ((*pc.offset(-2isize).offset(0isize)
                                                  as libc::c_int) << 8i32 |
                                                 *pc.offset(-2isize).offset(1isize)
                                                     as libc::c_int) as
                                                uint32_t;
                                        /* if R(b) pc=a */
                                        pc = pc.offset(2isize);
                                        b =
                                            ((*pc.offset(-2isize).offset(0isize)
                                                  as libc::c_int) << 8i32 |
                                                 *pc.offset(-2isize).offset(1isize)
                                                     as libc::c_int) as
                                                uint16_t;
                                        current_block = 1297461190301222800;
                                    }
                                    35 => {
                                        pc = pc.offset(2isize);
                                        a =
                                            ((*pc.offset(-2isize).offset(0isize)
                                                  as libc::c_int) << 8i32 |
                                                 *pc.offset(-2isize).offset(1isize)
                                                     as libc::c_int) as
                                                uint32_t;
                                        /* if !R(b) pc=a */
                                        pc = pc.offset(2isize);
                                        b =
                                            ((*pc.offset(-2isize).offset(0isize)
                                                  as libc::c_int) << 8i32 |
                                                 *pc.offset(-2isize).offset(1isize)
                                                     as libc::c_int) as
                                                uint16_t;
                                        current_block = 11208766757666257413;
                                    }
                                    36 => {
                                        pc = pc.offset(2isize);
                                        a =
                                            ((*pc.offset(-2isize).offset(0isize)
                                                  as libc::c_int) << 8i32 |
                                                 *pc.offset(-2isize).offset(1isize)
                                                     as libc::c_int) as
                                                uint32_t;
                                        /* if R(b)==nil pc=a */
                                        pc = pc.offset(2isize);
                                        b =
                                            ((*pc.offset(-2isize).offset(0isize)
                                                  as libc::c_int) << 8i32 |
                                                 *pc.offset(-2isize).offset(1isize)
                                                     as libc::c_int) as
                                                uint16_t;
                                        current_block = 14166554486324432560;
                                    }
                                    37 => {
                                        /* rescue_push(a) */
                                        pc = pc.offset(2isize);
                                        a =
                                            ((*pc.offset(-2isize).offset(0isize)
                                                  as libc::c_int) << 8i32 |
                                                 *pc.offset(-2isize).offset(1isize)
                                                     as libc::c_int) as
                                                uint32_t;
                                        current_block = 12913164539471242988;
                                    }
                                    38 => {
                                        /* R(a) = exc */
                                        pc = pc.offset(2isize);
                                        a =
                                            ((*pc.offset(-2isize).offset(0isize)
                                                  as libc::c_int) << 8i32 |
                                                 *pc.offset(-2isize).offset(1isize)
                                                     as libc::c_int) as
                                                uint32_t;
                                        current_block = 2722617303989997926;
                                    }
                                    39 => {
                                        pc = pc.offset(2isize);
                                        a =
                                            ((*pc.offset(-2isize).offset(0isize)
                                                  as libc::c_int) << 8i32 |
                                                 *pc.offset(-2isize).offset(1isize)
                                                     as libc::c_int) as
                                                uint32_t;
                                        /* R(b) = R(a).isa?(R(b)) */
                                        let fresh298 = pc;
                                        pc = pc.offset(1);
                                        b = *fresh298 as uint16_t;
                                        current_block = 16261244098005255386;
                                    }
                                    40 => {
                                        /* a.times{rescue_pop()} */
                                        pc = pc.offset(2isize);
                                        a =
                                            ((*pc.offset(-2isize).offset(0isize)
                                                  as libc::c_int) << 8i32 |
                                                 *pc.offset(-2isize).offset(1isize)
                                                     as libc::c_int) as
                                                uint32_t;
                                        current_block = 14933967110489461578;
                                    }
                                    41 => {
                                        /* raise(R(a)) */
                                        pc = pc.offset(2isize);
                                        a =
                                            ((*pc.offset(-2isize).offset(0isize)
                                                  as libc::c_int) << 8i32 |
                                                 *pc.offset(-2isize).offset(1isize)
                                                     as libc::c_int) as
                                                uint32_t;
                                        current_block = 85722159537708186;
                                    }
                                    42 => {
                                        /* ensure_push(SEQ[a]) */
                                        pc = pc.offset(2isize);
                                        a =
                                            ((*pc.offset(-2isize).offset(0isize)
                                                  as libc::c_int) << 8i32 |
                                                 *pc.offset(-2isize).offset(1isize)
                                                     as libc::c_int) as
                                                uint32_t;
                                        current_block = 17178928522929307677;
                                    }
                                    43 => {
                                        /* A.times{ensure_pop().call} */
                                        pc = pc.offset(2isize);
                                        a =
                                            ((*pc.offset(-2isize).offset(0isize)
                                                  as libc::c_int) << 8i32 |
                                                 *pc.offset(-2isize).offset(1isize)
                                                     as libc::c_int) as
                                                uint32_t;
                                        current_block = 8592218650707875588;
                                    }
                                    44 => {
                                        pc = pc.offset(2isize);
                                        a =
                                            ((*pc.offset(-2isize).offset(0isize)
                                                  as libc::c_int) << 8i32 |
                                                 *pc.offset(-2isize).offset(1isize)
                                                     as libc::c_int) as
                                                uint32_t;
                                        /* R(a) = call(R(a),Syms(b),*R(a+1)) */
                                        let fresh299 = pc;
                                        pc = pc.offset(1);
                                        b = *fresh299 as uint16_t;
                                        current_block = 12662816978892980002;
                                    }
                                    45 => {
                                        pc = pc.offset(2isize);
                                        a =
                                            ((*pc.offset(-2isize).offset(0isize)
                                                  as libc::c_int) << 8i32 |
                                                 *pc.offset(-2isize).offset(1isize)
                                                     as libc::c_int) as
                                                uint32_t;
                                        /* R(a) = call(R(a),Syms(b),*R(a+1),&R(a+2)) */
                                        let fresh300 = pc;
                                        pc = pc.offset(1);
                                        b = *fresh300 as uint16_t;
                                        current_block = 7460542724658431689;
                                    }
                                    46 => {
                                        pc = pc.offset(2isize);
                                        a =
                                            ((*pc.offset(-2isize).offset(0isize)
                                                  as libc::c_int) << 8i32 |
                                                 *pc.offset(-2isize).offset(1isize)
                                                     as libc::c_int) as
                                                uint32_t;
                                        let fresh301 = pc;
                                        pc = pc.offset(1);
                                        b = *fresh301 as uint16_t;
                                        /* R(a) = call(R(a),Syms(b),R(a+1),...,R(a+c)) */
                                        let fresh302 = pc;
                                        pc = pc.offset(1);
                                        c = *fresh302;
                                        current_block = 7304850410330992871;
                                    }
                                    47 => {
                                        pc = pc.offset(2isize);
                                        a =
                                            ((*pc.offset(-2isize).offset(0isize)
                                                  as libc::c_int) << 8i32 |
                                                 *pc.offset(-2isize).offset(1isize)
                                                     as libc::c_int) as
                                                uint32_t;
                                        let fresh303 = pc;
                                        pc = pc.offset(1);
                                        b = *fresh303 as uint16_t;
                                        /* R(a) = call(R(a),Syms(Bx),R(a+1),...,R(a+c),&R(a+c+1)) */
                                        let fresh304 = pc;
                                        pc = pc.offset(1);
                                        c = *fresh304;
                                        current_block = 10790764747546649571;
                                    }
                                    48 => {
                                        current_block = 13698748673986354976;
                                    }
                                    49 => {
                                        pc = pc.offset(2isize);
                                        a =
                                            ((*pc.offset(-2isize).offset(0isize)
                                                  as libc::c_int) << 8i32 |
                                                 *pc.offset(-2isize).offset(1isize)
                                                     as libc::c_int) as
                                                uint32_t;
                                        /* R(a) = super(R(a+1),... ,R(a+b+1)) */
                                        let fresh305 = pc;
                                        pc = pc.offset(1);
                                        b = *fresh305 as uint16_t;
                                        current_block = 9607585333290054341;
                                    }
                                    50 => {
                                        pc = pc.offset(2isize);
                                        a =
                                            ((*pc.offset(-2isize).offset(0isize)
                                                  as libc::c_int) << 8i32 |
                                                 *pc.offset(-2isize).offset(1isize)
                                                     as libc::c_int) as
                                                uint32_t;
                                        /* R(a) = argument array (16=m5:r1:m5:d1:lv4) */
                                        pc = pc.offset(2isize);
                                        b =
                                            ((*pc.offset(-2isize).offset(0isize)
                                                  as libc::c_int) << 8i32 |
                                                 *pc.offset(-2isize).offset(1isize)
                                                     as libc::c_int) as
                                                uint16_t;
                                        current_block = 10763054988551342546;
                                    }
                                    51 => {
                                        /* arg setup according to flags (23=m5:o5:r1:m5:k5:d1:b1) */
                                        pc = pc.offset(3isize);
                                        a =
                                            ((*pc.offset(-3isize).offset(0isize)
                                                  as libc::c_int) << 16i32 |
                                                 (*pc.offset(-3isize).offset(1isize)
                                                      as libc::c_int) << 8i32
                                                 |
                                                 *pc.offset(-3isize).offset(2isize)
                                                     as libc::c_int) as
                                                uint32_t;
                                        current_block = 15803687294481920802;
                                    }
                                    52 => {
                                        pc = pc.offset(2isize);
                                        a =
                                            ((*pc.offset(-2isize).offset(0isize)
                                                  as libc::c_int) << 8i32 |
                                                 *pc.offset(-2isize).offset(1isize)
                                                     as libc::c_int) as
                                                uint32_t;
                                        /* R(a) = kdict.key?(Syms(b))                      # todo */
                                        let fresh306 = pc;
                                        pc = pc.offset(1);
                                        b = *fresh306 as uint16_t;
                                        current_block = 1525421113118994726;
                                    }
                                    53 => {
                                        current_block = 4663630632216779857;
                                    }
                                    54 => {
                                        pc = pc.offset(2isize);
                                        a =
                                            ((*pc.offset(-2isize).offset(0isize)
                                                  as libc::c_int) << 8i32 |
                                                 *pc.offset(-2isize).offset(1isize)
                                                     as libc::c_int) as
                                                uint32_t;
                                        /* R(a) = kdict[Syms(b)]; kdict.delete(Syms(b))    # todo */
                                        let fresh307 = pc;
                                        pc = pc.offset(1);
                                        b = *fresh307 as uint16_t;
                                        current_block = 14765652484648187635;
                                    }
                                    55 => {
                                        /* return R(a) (normal) */
                                        pc = pc.offset(2isize);
                                        a =
                                            ((*pc.offset(-2isize).offset(0isize)
                                                  as libc::c_int) << 8i32 |
                                                 *pc.offset(-2isize).offset(1isize)
                                                     as libc::c_int) as
                                                uint32_t;
                                        current_block = 14547135203946798640;
                                    }
                                    56 => {
                                        /* return R(a) (in-block return) */
                                        pc = pc.offset(2isize);
                                        a =
                                            ((*pc.offset(-2isize).offset(0isize)
                                                  as libc::c_int) << 8i32 |
                                                 *pc.offset(-2isize).offset(1isize)
                                                     as libc::c_int) as
                                                uint32_t;
                                        current_block = 13114312767422449834;
                                    }
                                    57 => {
                                        /* break R(a) */
                                        pc = pc.offset(2isize);
                                        a =
                                            ((*pc.offset(-2isize).offset(0isize)
                                                  as libc::c_int) << 8i32 |
                                                 *pc.offset(-2isize).offset(1isize)
                                                     as libc::c_int) as
                                                uint32_t;
                                        current_block = 12223856006808243146;
                                    }
                                    58 => {
                                        pc = pc.offset(2isize);
                                        a =
                                            ((*pc.offset(-2isize).offset(0isize)
                                                  as libc::c_int) << 8i32 |
                                                 *pc.offset(-2isize).offset(1isize)
                                                     as libc::c_int) as
                                                uint32_t;
                                        /* R(a) = block (16=m5:r1:m5:d1:lv4) */
                                        pc = pc.offset(2isize);
                                        b =
                                            ((*pc.offset(-2isize).offset(0isize)
                                                  as libc::c_int) << 8i32 |
                                                 *pc.offset(-2isize).offset(1isize)
                                                     as libc::c_int) as
                                                uint16_t;
                                        current_block = 2994422791481536047;
                                    }
                                    59 => {
                                        /* R(a) = R(a)+R(a+1) */
                                        pc = pc.offset(2isize);
                                        a =
                                            ((*pc.offset(-2isize).offset(0isize)
                                                  as libc::c_int) << 8i32 |
                                                 *pc.offset(-2isize).offset(1isize)
                                                     as libc::c_int) as
                                                uint32_t;
                                        current_block = 16140473650108643456;
                                    }
                                    60 => {
                                        pc = pc.offset(2isize);
                                        a =
                                            ((*pc.offset(-2isize).offset(0isize)
                                                  as libc::c_int) << 8i32 |
                                                 *pc.offset(-2isize).offset(1isize)
                                                     as libc::c_int) as
                                                uint32_t;
                                        /* R(a) = R(a)+mrb_int(c)  */
                                        let fresh308 = pc;
                                        pc = pc.offset(1);
                                        b = *fresh308 as uint16_t;
                                        current_block = 4396809944207673246;
                                    }
                                    61 => {
                                        /* R(a) = R(a)-R(a+1) */
                                        pc = pc.offset(2isize);
                                        a =
                                            ((*pc.offset(-2isize).offset(0isize)
                                                  as libc::c_int) << 8i32 |
                                                 *pc.offset(-2isize).offset(1isize)
                                                     as libc::c_int) as
                                                uint32_t;
                                        current_block = 10653245833193402520;
                                    }
                                    62 => {
                                        pc = pc.offset(2isize);
                                        a =
                                            ((*pc.offset(-2isize).offset(0isize)
                                                  as libc::c_int) << 8i32 |
                                                 *pc.offset(-2isize).offset(1isize)
                                                     as libc::c_int) as
                                                uint32_t;
                                        /* R(a) = R(a)-C */
                                        let fresh309 = pc;
                                        pc = pc.offset(1);
                                        b = *fresh309 as uint16_t;
                                        current_block = 1767406316152665813;
                                    }
                                    63 => {
                                        /* R(a) = R(a)*R(a+1) */
                                        pc = pc.offset(2isize);
                                        a =
                                            ((*pc.offset(-2isize).offset(0isize)
                                                  as libc::c_int) << 8i32 |
                                                 *pc.offset(-2isize).offset(1isize)
                                                     as libc::c_int) as
                                                uint32_t;
                                        current_block = 2558910614333069445;
                                    }
                                    64 => {
                                        /* R(a) = R(a)/R(a+1) */
                                        pc = pc.offset(2isize);
                                        a =
                                            ((*pc.offset(-2isize).offset(0isize)
                                                  as libc::c_int) << 8i32 |
                                                 *pc.offset(-2isize).offset(1isize)
                                                     as libc::c_int) as
                                                uint32_t;
                                        current_block = 15842299444801573850;
                                    }
                                    65 => {
                                        /* R(a) = R(a)==R(a+1) */
                                        pc = pc.offset(2isize);
                                        a =
                                            ((*pc.offset(-2isize).offset(0isize)
                                                  as libc::c_int) << 8i32 |
                                                 *pc.offset(-2isize).offset(1isize)
                                                     as libc::c_int) as
                                                uint32_t;
                                        current_block = 10264557722339789830;
                                    }
                                    66 => {
                                        /* R(a) = R(a)<R(a+1) */
                                        pc = pc.offset(2isize);
                                        a =
                                            ((*pc.offset(-2isize).offset(0isize)
                                                  as libc::c_int) << 8i32 |
                                                 *pc.offset(-2isize).offset(1isize)
                                                     as libc::c_int) as
                                                uint32_t;
                                        current_block = 11600479572347497286;
                                    }
                                    67 => {
                                        /* R(a) = R(a)<=R(a+1) */
                                        pc = pc.offset(2isize);
                                        a =
                                            ((*pc.offset(-2isize).offset(0isize)
                                                  as libc::c_int) << 8i32 |
                                                 *pc.offset(-2isize).offset(1isize)
                                                     as libc::c_int) as
                                                uint32_t;
                                        current_block = 6268356626658084969;
                                    }
                                    68 => {
                                        /* R(a) = R(a)>R(a+1) */
                                        pc = pc.offset(2isize);
                                        a =
                                            ((*pc.offset(-2isize).offset(0isize)
                                                  as libc::c_int) << 8i32 |
                                                 *pc.offset(-2isize).offset(1isize)
                                                     as libc::c_int) as
                                                uint32_t;
                                        current_block = 14064792543554720178;
                                    }
                                    69 => {
                                        /* R(a) = R(a)>=R(a+1) */
                                        pc = pc.offset(2isize);
                                        a =
                                            ((*pc.offset(-2isize).offset(0isize)
                                                  as libc::c_int) << 8i32 |
                                                 *pc.offset(-2isize).offset(1isize)
                                                     as libc::c_int) as
                                                uint32_t;
                                        current_block = 12168003877939815570;
                                    }
                                    70 => {
                                        pc = pc.offset(2isize);
                                        a =
                                            ((*pc.offset(-2isize).offset(0isize)
                                                  as libc::c_int) << 8i32 |
                                                 *pc.offset(-2isize).offset(1isize)
                                                     as libc::c_int) as
                                                uint32_t;
                                        /* R(a) = ary_new(R(a),R(a+1)..R(a+b)) */
                                        let fresh310 = pc;
                                        pc = pc.offset(1);
                                        b = *fresh310 as uint16_t;
                                        current_block = 5334851501950231651;
                                    }
                                    71 => {
                                        pc = pc.offset(2isize);
                                        a =
                                            ((*pc.offset(-2isize).offset(0isize)
                                                  as libc::c_int) << 8i32 |
                                                 *pc.offset(-2isize).offset(1isize)
                                                     as libc::c_int) as
                                                uint32_t;
                                        let fresh311 = pc;
                                        pc = pc.offset(1);
                                        b = *fresh311 as uint16_t;
                                        /* R(a) = ary_new(R(b),R(b+1)..R(b+c)) */
                                        let fresh312 = pc;
                                        pc = pc.offset(1);
                                        c = *fresh312;
                                        current_block = 15590445366052810487;
                                    }
                                    72 => {
                                        /* ary_cat(R(a),R(a+1)) */
                                        pc = pc.offset(2isize);
                                        a =
                                            ((*pc.offset(-2isize).offset(0isize)
                                                  as libc::c_int) << 8i32 |
                                                 *pc.offset(-2isize).offset(1isize)
                                                     as libc::c_int) as
                                                uint32_t;
                                        current_block = 15951340130516588981;
                                    }
                                    73 => {
                                        /* ary_push(R(a),R(a+1)) */
                                        pc = pc.offset(2isize);
                                        a =
                                            ((*pc.offset(-2isize).offset(0isize)
                                                  as libc::c_int) << 8i32 |
                                                 *pc.offset(-2isize).offset(1isize)
                                                     as libc::c_int) as
                                                uint32_t;
                                        current_block = 17884754765720769523;
                                    }
                                    74 => {
                                        /* R(a) = ary_dup(R(a)) */
                                        pc = pc.offset(2isize);
                                        a =
                                            ((*pc.offset(-2isize).offset(0isize)
                                                  as libc::c_int) << 8i32 |
                                                 *pc.offset(-2isize).offset(1isize)
                                                     as libc::c_int) as
                                                uint32_t;
                                        current_block = 6985669818981290717;
                                    }
                                    75 => {
                                        pc = pc.offset(2isize);
                                        a =
                                            ((*pc.offset(-2isize).offset(0isize)
                                                  as libc::c_int) << 8i32 |
                                                 *pc.offset(-2isize).offset(1isize)
                                                     as libc::c_int) as
                                                uint32_t;
                                        let fresh313 = pc;
                                        pc = pc.offset(1);
                                        b = *fresh313 as uint16_t;
                                        /* R(a) = R(b)[c] */
                                        let fresh314 = pc;
                                        pc = pc.offset(1);
                                        c = *fresh314;
                                        current_block = 10370658347790963364;
                                    }
                                    76 => {
                                        pc = pc.offset(2isize);
                                        a =
                                            ((*pc.offset(-2isize).offset(0isize)
                                                  as libc::c_int) << 8i32 |
                                                 *pc.offset(-2isize).offset(1isize)
                                                     as libc::c_int) as
                                                uint32_t;
                                        let fresh315 = pc;
                                        pc = pc.offset(1);
                                        b = *fresh315 as uint16_t;
                                        /* R(a)[c] = R(b) */
                                        let fresh316 = pc;
                                        pc = pc.offset(1);
                                        c = *fresh316;
                                        current_block = 4938714451943331475;
                                    }
                                    77 => {
                                        pc = pc.offset(2isize);
                                        a =
                                            ((*pc.offset(-2isize).offset(0isize)
                                                  as libc::c_int) << 8i32 |
                                                 *pc.offset(-2isize).offset(1isize)
                                                     as libc::c_int) as
                                                uint32_t;
                                        let fresh317 = pc;
                                        pc = pc.offset(1);
                                        b = *fresh317 as uint16_t;
                                        /* *R(a),R(a+1)..R(a+c) = R(a)[b..] */
                                        let fresh318 = pc;
                                        pc = pc.offset(1);
                                        c = *fresh318;
                                        current_block = 11125061941892892926;
                                    }
                                    78 => {
                                        /* R(a) = intern(R(a)) */
                                        pc = pc.offset(2isize);
                                        a =
                                            ((*pc.offset(-2isize).offset(0isize)
                                                  as libc::c_int) << 8i32 |
                                                 *pc.offset(-2isize).offset(1isize)
                                                     as libc::c_int) as
                                                uint32_t;
                                        current_block = 2896230971687472890;
                                    }
                                    79 => {
                                        pc = pc.offset(2isize);
                                        a =
                                            ((*pc.offset(-2isize).offset(0isize)
                                                  as libc::c_int) << 8i32 |
                                                 *pc.offset(-2isize).offset(1isize)
                                                     as libc::c_int) as
                                                uint32_t;
                                        /* R(a) = str_dup(Lit(b)) */
                                        let fresh319 = pc;
                                        pc = pc.offset(1);
                                        b = *fresh319 as uint16_t;
                                        current_block = 14841647832940836303;
                                    }
                                    80 => {
                                        /* str_cat(R(a),R(a+1)) */
                                        pc = pc.offset(2isize);
                                        a =
                                            ((*pc.offset(-2isize).offset(0isize)
                                                  as libc::c_int) << 8i32 |
                                                 *pc.offset(-2isize).offset(1isize)
                                                     as libc::c_int) as
                                                uint32_t;
                                        current_block = 4191712132543108294;
                                    }
                                    81 => {
                                        pc = pc.offset(2isize);
                                        a =
                                            ((*pc.offset(-2isize).offset(0isize)
                                                  as libc::c_int) << 8i32 |
                                                 *pc.offset(-2isize).offset(1isize)
                                                     as libc::c_int) as
                                                uint32_t;
                                        /* R(a) = hash_new(R(a),R(a+1)..R(a+b)) */
                                        let fresh320 = pc;
                                        pc = pc.offset(1);
                                        b = *fresh320 as uint16_t;
                                        current_block = 1686938961607716371;
                                    }
                                    82 => {
                                        pc = pc.offset(2isize);
                                        a =
                                            ((*pc.offset(-2isize).offset(0isize)
                                                  as libc::c_int) << 8i32 |
                                                 *pc.offset(-2isize).offset(1isize)
                                                     as libc::c_int) as
                                                uint32_t;
                                        /* R(a) = hash_push(R(a),R(a+1)..R(a+b)) */
                                        let fresh321 = pc;
                                        pc = pc.offset(1);
                                        b = *fresh321 as uint16_t;
                                        current_block = 17492450365994263002;
                                    }
                                    83 => {
                                        /* R(a) = hash_cat(R(a),R(a+1)) */
                                        pc = pc.offset(2isize);
                                        a =
                                            ((*pc.offset(-2isize).offset(0isize)
                                                  as libc::c_int) << 8i32 |
                                                 *pc.offset(-2isize).offset(1isize)
                                                     as libc::c_int) as
                                                uint32_t;
                                        current_block = 15495526794150689403;
                                    }
                                    84 => {
                                        pc = pc.offset(2isize);
                                        a =
                                            ((*pc.offset(-2isize).offset(0isize)
                                                  as libc::c_int) << 8i32 |
                                                 *pc.offset(-2isize).offset(1isize)
                                                     as libc::c_int) as
                                                uint32_t;
                                        /* R(a) = lambda(SEQ[b],L_LAMBDA) */
                                        let fresh322 = pc;
                                        pc = pc.offset(1);
                                        b = *fresh322 as uint16_t;
                                        current_block = 2702902889507381978;
                                    }
                                    85 => {
                                        pc = pc.offset(2isize);
                                        a =
                                            ((*pc.offset(-2isize).offset(0isize)
                                                  as libc::c_int) << 8i32 |
                                                 *pc.offset(-2isize).offset(1isize)
                                                     as libc::c_int) as
                                                uint32_t;
                                        /* R(a) = lambda(SEQ[b],L_BLOCK) */
                                        let fresh323 = pc;
                                        pc = pc.offset(1);
                                        b = *fresh323 as uint16_t;
                                        current_block = 1152971280698483565;
                                    }
                                    86 => {
                                        pc = pc.offset(2isize);
                                        a =
                                            ((*pc.offset(-2isize).offset(0isize)
                                                  as libc::c_int) << 8i32 |
                                                 *pc.offset(-2isize).offset(1isize)
                                                     as libc::c_int) as
                                                uint32_t;
                                        /* R(a) = lambda(SEQ[b],L_METHOD) */
                                        let fresh324 = pc;
                                        pc = pc.offset(1);
                                        b = *fresh324 as uint16_t;
                                        current_block = 16224973743324382762;
                                    }
                                    87 => {
                                        /* R(a) = range_new(R(a),R(a+1),FALSE) */
                                        pc = pc.offset(2isize);
                                        a =
                                            ((*pc.offset(-2isize).offset(0isize)
                                                  as libc::c_int) << 8i32 |
                                                 *pc.offset(-2isize).offset(1isize)
                                                     as libc::c_int) as
                                                uint32_t;
                                        current_block = 6571067991607217453;
                                    }
                                    88 => {
                                        /* R(a) = range_new(R(a),R(a+1),TRUE) */
                                        pc = pc.offset(2isize);
                                        a =
                                            ((*pc.offset(-2isize).offset(0isize)
                                                  as libc::c_int) << 8i32 |
                                                 *pc.offset(-2isize).offset(1isize)
                                                     as libc::c_int) as
                                                uint32_t;
                                        current_block = 14502557129029694510;
                                    }
                                    89 => {
                                        /* R(a) = ::Object */
                                        pc = pc.offset(2isize);
                                        a =
                                            ((*pc.offset(-2isize).offset(0isize)
                                                  as libc::c_int) << 8i32 |
                                                 *pc.offset(-2isize).offset(1isize)
                                                     as libc::c_int) as
                                                uint32_t;
                                        current_block = 16196049249653527883;
                                    }
                                    90 => {
                                        pc = pc.offset(2isize);
                                        a =
                                            ((*pc.offset(-2isize).offset(0isize)
                                                  as libc::c_int) << 8i32 |
                                                 *pc.offset(-2isize).offset(1isize)
                                                     as libc::c_int) as
                                                uint32_t;
                                        /* R(a) = newclass(R(a),Syms(b),R(a+1)) */
                                        let fresh325 = pc;
                                        pc = pc.offset(1);
                                        b = *fresh325 as uint16_t;
                                        current_block = 16736254050004001558;
                                    }
                                    91 => {
                                        pc = pc.offset(2isize);
                                        a =
                                            ((*pc.offset(-2isize).offset(0isize)
                                                  as libc::c_int) << 8i32 |
                                                 *pc.offset(-2isize).offset(1isize)
                                                     as libc::c_int) as
                                                uint32_t;
                                        /* R(a) = newmodule(R(a),Syms(b)) */
                                        let fresh326 = pc;
                                        pc = pc.offset(1);
                                        b = *fresh326 as uint16_t;
                                        current_block = 16407897916814827188;
                                    }
                                    92 => {
                                        pc = pc.offset(2isize);
                                        a =
                                            ((*pc.offset(-2isize).offset(0isize)
                                                  as libc::c_int) << 8i32 |
                                                 *pc.offset(-2isize).offset(1isize)
                                                     as libc::c_int) as
                                                uint32_t;
                                        /* R(a) = blockexec(R(a),SEQ[b]) */
                                        let fresh327 = pc;
                                        pc = pc.offset(1);
                                        b = *fresh327 as uint16_t;
                                        current_block = 9892557545450190862;
                                    }
                                    93 => {
                                        pc = pc.offset(2isize);
                                        a =
                                            ((*pc.offset(-2isize).offset(0isize)
                                                  as libc::c_int) << 8i32 |
                                                 *pc.offset(-2isize).offset(1isize)
                                                     as libc::c_int) as
                                                uint32_t;
                                        /* R(a).newmethod(Syms(b),R(a+1)) */
                                        let fresh328 = pc;
                                        pc = pc.offset(1);
                                        b = *fresh328 as uint16_t;
                                        current_block = 15928390647053518202;
                                    }
                                    94 => {
                                        pc = pc.offset(2isize);
                                        a =
                                            ((*pc.offset(-2isize).offset(0isize)
                                                  as libc::c_int) << 8i32 |
                                                 *pc.offset(-2isize).offset(1isize)
                                                     as libc::c_int) as
                                                uint32_t;
                                        /* alias_method(target_class,Syms(a),Syms(b)) */
                                        let fresh329 = pc;
                                        pc = pc.offset(1);
                                        b = *fresh329 as uint16_t;
                                        current_block = 1656069329997846098;
                                    }
                                    95 => {
                                        /* undef_method(target_class,Syms(a)) */
                                        pc = pc.offset(2isize);
                                        a =
                                            ((*pc.offset(-2isize).offset(0isize)
                                                  as libc::c_int) << 8i32 |
                                                 *pc.offset(-2isize).offset(1isize)
                                                     as libc::c_int) as
                                                uint32_t;
                                        current_block = 5769344152980800111;
                                    }
                                    96 => {
                                        /* R(a) = R(a).singleton_class */
                                        pc = pc.offset(2isize);
                                        a =
                                            ((*pc.offset(-2isize).offset(0isize)
                                                  as libc::c_int) << 8i32 |
                                                 *pc.offset(-2isize).offset(1isize)
                                                     as libc::c_int) as
                                                uint32_t;
                                        current_block = 15212051862008919012;
                                    }
                                    97 => {
                                        /* R(a) = target_class */
                                        pc = pc.offset(2isize);
                                        a =
                                            ((*pc.offset(-2isize).offset(0isize)
                                                  as libc::c_int) << 8i32 |
                                                 *pc.offset(-2isize).offset(1isize)
                                                     as libc::c_int) as
                                                uint32_t;
                                        current_block = 15561344248022367768;
                                    }
                                    98 => {
                                        pc = pc.offset(2isize);
                                        a =
                                            ((*pc.offset(-2isize).offset(0isize)
                                                  as libc::c_int) << 8i32 |
                                                 *pc.offset(-2isize).offset(1isize)
                                                     as libc::c_int) as
                                                uint32_t;
                                        let fresh330 = pc;
                                        pc = pc.offset(1);
                                        b = *fresh330 as uint16_t;
                                        /* print a,b,c */
                                        let fresh331 = pc;
                                        pc = pc.offset(1);
                                        c = *fresh331;
                                        current_block = 10071922767614739690;
                                        break 'c_4688 ;
                                    }
                                    99 => {
                                        /* raise(LocalJumpError, Lit(a)) */
                                        pc = pc.offset(2isize);
                                        a =
                                            ((*pc.offset(-2isize).offset(0isize)
                                                  as libc::c_int) << 8i32 |
                                                 *pc.offset(-2isize).offset(1isize)
                                                     as libc::c_int) as
                                                uint32_t;
                                        current_block = 9217466010932042731;
                                    }
                                    100 => {
                                        current_block = 16136094826069225138;
                                    }
                                    101 => {
                                        current_block = 12853241758993110625;
                                    }
                                    102 => {
                                        current_block = 3106153520265210690;
                                    }
                                    103 => {
                                        current_block = 5128590269704079499;
                                        break 'c_4688 ;
                                    }
                                    _ => {
                                        pc = pc.offset(-1isize);
                                        current_block = 7149356873433890176;
                                        continue 'c_4688 ;
                                    }
                                }
                            }
                            15561344248022367768 => {
                                if 0 == check_target_class(mrb) {
                                    current_block = 2687857153341325290;
                                    continue 'c_4688 ;
                                }
                                *(*(*mrb).c).stack.offset(a as isize) =
                                    mrb_obj_value((*(*(*mrb).c).ci).target_class
                                                      as *mut libc::c_void);
                                current_block = 7149356873433890176;
                                continue 'c_4688 ;
                            }
                            15212051862008919012 => {
                                *(*(*mrb).c).stack.offset(a as isize) =
                                    mrb_singleton_class(mrb,
                                                        *(*(*mrb).c).stack.offset(a
                                                                                      as
                                                                                      isize));
                                mrb_gc_arena_restore(mrb, ai);
                                current_block = 7149356873433890176;
                                continue 'c_4688 ;
                            }
                            10370658347790963364 => {
                                let mut v_3: mrb_value =
                                    *(*(*mrb).c).stack.offset(b as isize);
                                if !(v_3.tt as libc::c_uint ==
                                         MRB_TT_ARRAY as libc::c_int as
                                             libc::c_uint) {
                                    if c as libc::c_int == 0i32 {
                                        *(*(*mrb).c).stack.offset(a as isize)
                                            = v_3
                                    } else {
                                        (*(*(*mrb).c).stack.offset(a as
                                                                       isize)).tt
                                            = MRB_TT_FALSE;
                                        (*(*(*mrb).c).stack.offset(a as
                                                                       isize)).value.i
                                            = 0i32 as mrb_int
                                    }
                                } else {
                                    v_3 = mrb_ary_ref(mrb, v_3, c as mrb_int);
                                    *(*(*mrb).c).stack.offset(a as isize) =
                                        v_3
                                }
                                current_block = 7149356873433890176;
                                continue 'c_4688 ;
                            }
                            12168003877939815570 => {
                                result_3 = 0;
                                match ((*(*(*mrb).c).stack.offset(a as
                                                                      isize)).tt
                                           as uint16_t as libc::c_int) << 8i32
                                          |
                                          (*(*(*mrb).c).stack.offset(a.wrapping_add(1i32
                                                                                        as
                                                                                        libc::c_uint)
                                                                         as
                                                                         isize)).tt
                                              as uint16_t as libc::c_int &
                                              0xffi32 {
                                    771 => {
                                        current_block = 17916663512757813856;
                                        break ;
                                    }
                                    774 => {
                                        current_block = 3348445690345083753;
                                        break ;
                                    }
                                    1539 => {
                                        current_block = 16076866197496778268;
                                        break ;
                                    }
                                    1542 => {
                                        current_block = 1314190040982086846;
                                        break ;
                                    }
                                    _ => {
                                        current_block = 4156951849113663992;
                                        break ;
                                    }
                                }
                            }
                            14064792543554720178 => {
                                result_2 = 0;
                                match ((*(*(*mrb).c).stack.offset(a as
                                                                      isize)).tt
                                           as uint16_t as libc::c_int) << 8i32
                                          |
                                          (*(*(*mrb).c).stack.offset(a.wrapping_add(1i32
                                                                                        as
                                                                                        libc::c_uint)
                                                                         as
                                                                         isize)).tt
                                              as uint16_t as libc::c_int &
                                              0xffi32 {
                                    771 => {
                                        current_block = 12909619691863992879;
                                        break ;
                                    }
                                    774 => {
                                        current_block = 16805217273881322446;
                                        break ;
                                    }
                                    1539 => {
                                        current_block = 678496273481856040;
                                        break ;
                                    }
                                    1542 => {
                                        current_block = 3978484480214358740;
                                        break ;
                                    }
                                    _ => {
                                        current_block = 12115052188632565762;
                                        break ;
                                    }
                                }
                            }
                            6268356626658084969 => {
                                result_1 = 0;
                                match ((*(*(*mrb).c).stack.offset(a as
                                                                      isize)).tt
                                           as uint16_t as libc::c_int) << 8i32
                                          |
                                          (*(*(*mrb).c).stack.offset(a.wrapping_add(1i32
                                                                                        as
                                                                                        libc::c_uint)
                                                                         as
                                                                         isize)).tt
                                              as uint16_t as libc::c_int &
                                              0xffi32 {
                                    771 => {
                                        current_block = 3527484645425325286;
                                        break ;
                                    }
                                    774 => {
                                        current_block = 3075363095485550289;
                                        break ;
                                    }
                                    1539 => {
                                        current_block = 8736935332486372157;
                                        break ;
                                    }
                                    1542 => {
                                        current_block = 7747793278649614952;
                                        break ;
                                    }
                                    _ => {
                                        current_block = 11695679538446617396;
                                        break ;
                                    }
                                }
                            }
                            11600479572347497286 => {
                                result_0 = 0;
                                match ((*(*(*mrb).c).stack.offset(a as
                                                                      isize)).tt
                                           as uint16_t as libc::c_int) << 8i32
                                          |
                                          (*(*(*mrb).c).stack.offset(a.wrapping_add(1i32
                                                                                        as
                                                                                        libc::c_uint)
                                                                         as
                                                                         isize)).tt
                                              as uint16_t as libc::c_int &
                                              0xffi32 {
                                    771 => {
                                        current_block = 3624932733755770481;
                                        break ;
                                    }
                                    774 => {
                                        current_block = 6802740918435369298;
                                        break ;
                                    }
                                    1539 => {
                                        current_block = 18056191432036799439;
                                        break ;
                                    }
                                    1542 => {
                                        current_block = 11951279088167397802;
                                        break ;
                                    }
                                    _ => {
                                        current_block = 3676359657258785160;
                                        break ;
                                    }
                                }
                            }
                            15842299444801573850 => {
                                x_2 = 0.;
                                y_2 = 0.;
                                f = 0.;
                                /* need to check if op is overridden */
                                match ((*(*(*mrb).c).stack.offset(a as
                                                                      isize)).tt
                                           as uint16_t as libc::c_int) << 8i32
                                          |
                                          (*(*(*mrb).c).stack.offset(a.wrapping_add(1i32
                                                                                        as
                                                                                        libc::c_uint)
                                                                         as
                                                                         isize)).tt
                                              as uint16_t as libc::c_int &
                                              0xffi32 {
                                    771 => {
                                        current_block = 8282156447772606522;
                                        break ;
                                    }
                                    774 => {
                                        current_block = 12887046741138131814;
                                        break ;
                                    }
                                    1539 => {
                                        current_block = 2808535963553584040;
                                        break ;
                                    }
                                    1542 => {
                                        current_block = 4739989325879360476;
                                        break ;
                                    }
                                    _ => {
                                        current_block = 6523494653378992949;
                                        break ;
                                    }
                                }
                            }
                            7304850410330992871 => {
                                /* push nil after arguments */
                                let mut bidx: libc::c_int =
                                    (if c as libc::c_int == 127i32 {
                                         a.wrapping_add(2i32 as libc::c_uint)
                                     } else {
                                         a.wrapping_add(c as
                                                            libc::c_uint).wrapping_add(1i32
                                                                                           as
                                                                                           libc::c_uint)
                                     }) as libc::c_int;
                                (*(*(*mrb).c).stack.offset(bidx as isize)).tt
                                    = MRB_TT_FALSE;
                                (*(*(*mrb).c).stack.offset(bidx as
                                                               isize)).value.i
                                    = 0i32 as mrb_int;
                                current_block = 10790764747546649571;
                            }
                            14166554486324432560 => {
                                if !((*(*(*mrb).c).stack.offset(a as
                                                                    isize)).tt
                                         as libc::c_uint ==
                                         MRB_TT_FALSE as libc::c_int as
                                             libc::c_uint &&
                                         0 ==
                                             (*(*(*mrb).c).stack.offset(a as
                                                                            isize)).value.i)
                                   {
                                    current_block = 7149356873433890176;
                                    continue 'c_4688 ;
                                }
                                pc =
                                    (*irep).iseq.offset(b as libc::c_int as
                                                            isize);
                                current_block = 7149356873433890176;
                                continue 'c_4688 ;
                            }
                            11208766757666257413 => {
                                if (*(*(*mrb).c).stack.offset(a as isize)).tt
                                       as libc::c_uint !=
                                       MRB_TT_FALSE as libc::c_int as
                                           libc::c_uint {
                                    current_block = 7149356873433890176;
                                    continue 'c_4688 ;
                                }
                                pc =
                                    (*irep).iseq.offset(b as libc::c_int as
                                                            isize);
                                current_block = 7149356873433890176;
                                continue 'c_4688 ;
                            }
                            1297461190301222800 => {
                                if !((*(*(*mrb).c).stack.offset(a as
                                                                    isize)).tt
                                         as libc::c_uint !=
                                         MRB_TT_FALSE as libc::c_int as
                                             libc::c_uint) {
                                    current_block = 7149356873433890176;
                                    continue 'c_4688 ;
                                }
                                pc =
                                    (*irep).iseq.offset(b as libc::c_int as
                                                            isize);
                                current_block = 7149356873433890176;
                                continue 'c_4688 ;
                            }
                            4485224281673279150 => {
                                let mut e_0: *mut REnv =
                                    uvenv(mrb, c as libc::c_int);
                                if !e_0.is_null() {
                                    let mut regs_a_0: *mut mrb_value =
                                        (*(*mrb).c).stack.offset(a as isize);
                                    if (b as libc::c_longlong) <
                                           ((*e_0).flags() as libc::c_int &
                                                0x3ffi32) as mrb_int {
                                        *(*e_0).stack.offset(b as isize) =
                                            *regs_a_0;
                                        mrb_write_barrier(mrb,
                                                          e_0 as *mut RBasic);
                                    }
                                }
                                current_block = 7149356873433890176;
                                continue 'c_4688 ;
                            }
                            14187386403465544025 => {
                                let mut val_0: mrb_value =
                                    mrb_vm_special_get(mrb, b as mrb_sym);
                                *(*(*mrb).c).stack.offset(a as isize) = val_0;
                                current_block = 7149356873433890176;
                                continue 'c_4688 ;
                            }
                            13479157322803929894 => {
                                let mut val: mrb_value =
                                    mrb_gv_get(mrb, *syms.offset(b as isize));
                                *(*(*mrb).c).stack.offset(a as isize) = val;
                                current_block = 7149356873433890176;
                                continue 'c_4688 ;
                            }
                            16196049249653527883 => {
                                *(*(*mrb).c).stack.offset(a as isize) =
                                    mrb_obj_value((*mrb).object_class as
                                                      *mut libc::c_void);
                                current_block = 7149356873433890176;
                                continue 'c_4688 ;
                            }
                            16224973743324382762 => {
                                c = 1i32 as uint8_t;
                                current_block = 11169057726334001291;
                                break ;
                            }
                            1152971280698483565 => {
                                c = 2i32 as uint8_t;
                                current_block = 11169057726334001291;
                                break ;
                            }
                            2702902889507381978 => {
                                c = (1i32 | 2i32) as uint8_t;
                                current_block = 11169057726334001291;
                                break ;
                            }
                            4191712132543108294 => {
                                mrb_str_concat(mrb,
                                               *(*(*mrb).c).stack.offset(a as
                                                                             isize),
                                               *(*(*mrb).c).stack.offset(a.wrapping_add(1i32
                                                                                            as
                                                                                            libc::c_uint)
                                                                             as
                                                                             isize));
                                current_block = 7149356873433890176;
                                continue 'c_4688 ;
                            }
                            4938714451943331475 => {
                                mrb_ary_set(mrb,
                                            *(*(*mrb).c).stack.offset(b as
                                                                          isize),
                                            c as mrb_int,
                                            *(*(*mrb).c).stack.offset(a as
                                                                          isize));
                                current_block = 7149356873433890176;
                                continue 'c_4688 ;
                            }
                            17884754765720769523 => {
                                mrb_ary_push(mrb,
                                             *(*(*mrb).c).stack.offset(a as
                                                                           isize),
                                             *(*(*mrb).c).stack.offset(a.wrapping_add(1i32
                                                                                          as
                                                                                          libc::c_uint)
                                                                           as
                                                                           isize));
                                current_block = 7149356873433890176;
                                continue 'c_4688 ;
                            }
                            14547135203946798640 => {
                                c = 0i32 as uint8_t;
                                current_block = 7628100855803483190;
                                break ;
                            }
                            13114312767422449834 => {
                                c = 2i32 as uint8_t;
                                current_block = 7628100855803483190;
                                break ;
                            }
                            12223856006808243146 => {
                                c = 1i32 as uint8_t;
                                current_block = 7628100855803483190;
                                break ;
                            }
                            10790764747546649571 => {
                                mid = *syms.offset(b as isize);
                                current_block = 6940122889508134762;
                                break ;
                            }
                            7460542724658431689 => {
                                c = 127i32 as uint8_t;
                                current_block = 10790764747546649571;
                            }
                            12662816978892980002 => {
                                c = 127i32 as uint8_t;
                                current_block = 7304850410330992871;
                            }
                            85722159537708186 => {
                                mrb_exc_set(mrb,
                                            *(*(*mrb).c).stack.offset(a as
                                                                          isize));
                                current_block = 2687857153341325290;
                                continue 'c_4688 ;
                            }
                            14933967110489461578 => {
                                (*(*(*mrb).c).ci).ridx =
                                    ((*(*(*mrb).c).ci).ridx as
                                         libc::c_uint).wrapping_sub(a) as
                                        uint16_t as uint16_t;
                                current_block = 7149356873433890176;
                                continue 'c_4688 ;
                            }
                            9422951997864425805 => {
                                pc = (*irep).iseq.offset(a as isize);
                                current_block = 7149356873433890176;
                                continue 'c_4688 ;
                            }
                            5323028055506826224 => {
                                mrb_const_set(mrb,
                                              *(*(*mrb).c).stack.offset(a.wrapping_add(1i32
                                                                                           as
                                                                                           libc::c_uint)
                                                                            as
                                                                            isize),
                                              *syms.offset(b as isize),
                                              *(*(*mrb).c).stack.offset(a as
                                                                            isize));
                                current_block = 7149356873433890176;
                                continue 'c_4688 ;
                            }
                            6813271534392596583 => {
                                mrb_vm_const_set(mrb,
                                                 *syms.offset(b as isize),
                                                 *(*(*mrb).c).stack.offset(a
                                                                               as
                                                                               isize));
                                current_block = 7149356873433890176;
                                continue 'c_4688 ;
                            }
                            1105701036774490218 => {
                                mrb_vm_cv_set(mrb, *syms.offset(b as isize),
                                              *(*(*mrb).c).stack.offset(a as
                                                                            isize));
                                current_block = 7149356873433890176;
                                continue 'c_4688 ;
                            }
                            12788783625999190409 => {
                                mrb_iv_set(mrb,
                                           *(*(*mrb).c).stack.offset(0isize),
                                           *syms.offset(b as isize),
                                           *(*(*mrb).c).stack.offset(a as
                                                                         isize));
                                current_block = 7149356873433890176;
                                continue 'c_4688 ;
                            }
                            4491581808830814586 => {
                                *(*(*mrb).c).stack.offset(a as isize) =
                                    mrb_iv_get(mrb,
                                               *(*(*mrb).c).stack.offset(0isize),
                                               *syms.offset(b as isize));
                                current_block = 7149356873433890176;
                                continue 'c_4688 ;
                            }
                            9974864727789713748 => {
                                mrb_vm_special_set(mrb, b as mrb_sym,
                                                   *(*(*mrb).c).stack.offset(a
                                                                                 as
                                                                                 isize));
                                current_block = 7149356873433890176;
                                continue 'c_4688 ;
                            }
                            3151994457458062110 => {
                                mrb_gv_set(mrb, *syms.offset(b as isize),
                                           *(*(*mrb).c).stack.offset(a as
                                                                         isize));
                                current_block = 7149356873433890176;
                                continue 'c_4688 ;
                            }
                            4459663504651627985 => {
                                (*(*(*mrb).c).stack.offset(a as isize)).tt =
                                    MRB_TT_FALSE;
                                (*(*(*mrb).c).stack.offset(a as
                                                               isize)).value.i
                                    = 1i32 as mrb_int;
                                current_block = 7149356873433890176;
                                continue 'c_4688 ;
                            }
                            4976922244085895320 => {
                                (*(*(*mrb).c).stack.offset(a as isize)).tt =
                                    MRB_TT_TRUE;
                                (*(*(*mrb).c).stack.offset(a as
                                                               isize)).value.i
                                    = 1i32 as mrb_int;
                                current_block = 7149356873433890176;
                                continue 'c_4688 ;
                            }
                            16974974966130203269 => {
                                *(*(*mrb).c).stack.offset(a as isize) =
                                    *(*(*mrb).c).stack.offset(0isize);
                                current_block = 7149356873433890176;
                                continue 'c_4688 ;
                            }
                            18201902862271706575 => {
                                (*(*(*mrb).c).stack.offset(a as isize)).tt =
                                    MRB_TT_FALSE;
                                (*(*(*mrb).c).stack.offset(a as
                                                               isize)).value.i
                                    = 0i32 as mrb_int;
                                current_block = 7149356873433890176;
                                continue 'c_4688 ;
                            }
                            16314074004867283505 => {
                                (*(*(*mrb).c).stack.offset(a as isize)).tt =
                                    MRB_TT_SYMBOL;
                                (*(*(*mrb).c).stack.offset(a as
                                                               isize)).value.sym
                                    = *syms.offset(b as isize);
                                current_block = 7149356873433890176;
                                continue 'c_4688 ;
                            }
                            14563621082360312968 => {
                                (*(*(*mrb).c).stack.offset(a as isize)).tt =
                                    MRB_TT_FIXNUM;
                                (*(*(*mrb).c).stack.offset(a as
                                                               isize)).value.i
                                    =
                                    insn as mrb_int -
                                        OP_LOADI_0 as libc::c_int as mrb_int;
                                current_block = 7149356873433890176;
                                continue 'c_4688 ;
                            }
                            7343950298149844727 => {
                                (*(*(*mrb).c).stack.offset(a as isize)).tt =
                                    MRB_TT_FIXNUM;
                                (*(*(*mrb).c).stack.offset(a as
                                                               isize)).value.i
                                    = -(b as libc::c_int) as mrb_int;
                                current_block = 7149356873433890176;
                                continue 'c_4688 ;
                            }
                            14447253356787937536 => {
                                (*(*(*mrb).c).stack.offset(a as isize)).tt =
                                    MRB_TT_FIXNUM;
                                (*(*(*mrb).c).stack.offset(a as
                                                               isize)).value.i
                                    = b as mrb_int;
                                current_block = 7149356873433890176;
                                continue 'c_4688 ;
                            }
                            8845338526596852646 => {
                                *(*(*mrb).c).stack.offset(a as isize) =
                                    *pool.offset(b as isize);
                                current_block = 7149356873433890176;
                                continue 'c_4688 ;
                            }
                            4488286894823169796 => {
                                *(*(*mrb).c).stack.offset(a as isize) =
                                    *(*(*mrb).c).stack.offset(b as isize);
                                current_block = 7149356873433890176;
                                continue 'c_4688 ;
                            }
                            _ => {
                                let mut msg: mrb_value =
                                    mrb_str_dup(mrb,
                                                *pool.offset(a as isize));
                                let mut exc_10: mrb_value =
                                    mrb_value{value: C2RustUnnamed_0{f: 0.,},
                                              tt: MRB_TT_FALSE,};
                                exc_10 =
                                    mrb_exc_new_str(mrb,
                                                    mrb_exc_get(mrb,
                                                                b"LocalJumpError\x00"
                                                                    as
                                                                    *const u8
                                                                    as
                                                                    *const libc::c_char),
                                                    msg);
                                (*(*(*mrb).c).ci).err = pc0;
                                mrb_exc_set(mrb, exc_10);
                                current_block = 2687857153341325290;
                                continue 'c_4688 ;
                            }
                        }
                    }
                    match current_block {
                        7628100855803483190 => {
                            ci_3 = 0 as *mut mrb_callinfo;
                            ci_3 = (*(*mrb).c).ci;
                            if 0 != (*ci_3).mid {
                                let mut blk_2: mrb_value =
                                    mrb_value{value: C2RustUnnamed_0{f: 0.,},
                                              tt: MRB_TT_FALSE,};
                                if (*ci_3).argc < 0i32 {
                                    blk_2 = *(*(*mrb).c).stack.offset(2isize)
                                } else {
                                    blk_2 =
                                        *(*(*mrb).c).stack.offset(((*ci_3).argc
                                                                       + 1i32)
                                                                      as
                                                                      isize)
                                }
                                if blk_2.tt as libc::c_uint ==
                                       MRB_TT_PROC as libc::c_int as
                                           libc::c_uint {
                                    let mut p_2: *mut RProc =
                                        blk_2.value.p as *mut RProc;
                                    if !((*p_2).flags() as libc::c_int &
                                             256i32 != 0i32) &&
                                           ci_3 > (*(*mrb).c).cibase &&
                                           (if (*p_2).flags() as libc::c_int &
                                                   1024i32 != 0i32 {
                                                (*p_2).e.env
                                            } else { 0 as *mut REnv }) ==
                                               (*ci_3.offset(-1i32 as
                                                                 isize)).env {
                                        (*p_2).set_flags((*p_2).flags() |
                                                             512i32 as
                                                                 uint32_t)
                                    }
                                }
                            }
                            if !(*mrb).exc.is_null() {
                                ci0 = 0 as *mut mrb_callinfo;
                                current_block = 2687857153341325290;
                                continue ;
                            } else {
                                acc = 0;
                                v_0 =
                                    mrb_value{value: C2RustUnnamed_0{f: 0.,},
                                              tt: MRB_TT_FALSE,};
                                dst = 0 as *mut RProc;
                                ci_3 = (*(*mrb).c).ci;
                                v_0 = *(*(*mrb).c).stack.offset(a as isize);
                                mrb_gc_protect(mrb, v_0);
                                match c as libc::c_int {
                                    2 => {
                                        /* Fall through to OP_R_NORMAL otherwise */
                                        if (*ci_3).acc >= 0i32 &&
                                               (*proc_0).flags() as
                                                   libc::c_int & 1024i32 !=
                                                   0i32 &&
                                               !((*proc_0).flags() as
                                                     libc::c_int & 256i32 !=
                                                     0i32) {
                                            let mut cibase:
                                                    *mut mrb_callinfo =
                                                (*(*mrb).c).cibase;
                                            dst = top_proc(mrb, proc_0);
                                            if (*dst).flags() as libc::c_int &
                                                   1024i32 != 0i32 {
                                                let mut e_4: *mut REnv =
                                                    if (*dst).flags() as
                                                           libc::c_int &
                                                           1024i32 != 0i32 {
                                                        (*dst).e.env
                                                    } else { 0 as *mut REnv };
                                                if !((*e_4).flags() as
                                                         libc::c_int &
                                                         1i32 << 20i32 ==
                                                         0i32) ||
                                                       (*e_4).cxt != (*mrb).c
                                                   {
                                                    localjump_error(mrb,
                                                                    LOCALJUMP_ERROR_RETURN);
                                                    current_block =
                                                        2687857153341325290;
                                                    continue ;
                                                }
                                            }
                                            while cibase <= ci_3 &&
                                                      (*ci_3).proc_0 != dst {
                                                if (*ci_3).acc < 0i32 {
                                                    localjump_error(mrb,
                                                                    LOCALJUMP_ERROR_RETURN);
                                                    current_block =
                                                        2687857153341325290;
                                                    continue 'c_4688 ;
                                                } else {
                                                    ci_3 =
                                                        ci_3.offset(-1isize)
                                                }
                                            }
                                            if ci_3 <= cibase {
                                                localjump_error(mrb,
                                                                LOCALJUMP_ERROR_RETURN);
                                                current_block =
                                                    2687857153341325290;
                                                continue ;
                                            } else {
                                                current_block =
                                                    17785511045760650023;
                                            }
                                        } else {
                                            /* fallthrough */
                                            current_block =
                                                10630567705211265669;
                                        }
                                    }
                                    0 => {
                                        current_block = 10630567705211265669;
                                    }
                                    1 => {
                                        if (*proc_0).flags() as libc::c_int &
                                               256i32 != 0i32 {
                                            current_block =
                                                10630567705211265669;
                                        } else {
                                            if (*proc_0).flags() as
                                                   libc::c_int & 512i32 !=
                                                   0i32 {
                                                exc_9 =
                                                    mrb_value{value:
                                                                  C2RustUnnamed_0{f:
                                                                                      0.,},
                                                              tt:
                                                                  MRB_TT_FALSE,};
                                            } else if !(!((*proc_0).flags() as
                                                              libc::c_int &
                                                              1024i32 != 0i32)
                                                            ||
                                                            !((*(if (*proc_0).flags()
                                                                        as
                                                                        libc::c_int
                                                                        &
                                                                        1024i32
                                                                        !=
                                                                        0i32 {
                                                                     (*proc_0).e.env
                                                                 } else {
                                                                     0 as
                                                                         *mut REnv
                                                                 })).flags()
                                                                  as
                                                                  libc::c_int
                                                                  &
                                                                  1i32 <<
                                                                      20i32 ==
                                                                  0i32)) {
                                                let mut e_5: *mut REnv =
                                                    if (*proc_0).flags() as
                                                           libc::c_int &
                                                           1024i32 != 0i32 {
                                                        (*proc_0).e.env
                                                    } else { 0 as *mut REnv };
                                                if !((*e_5).cxt != (*mrb).c) {
                                                    while (*(*mrb).c).eidx as
                                                              libc::c_int >
                                                              (*(*(*mrb).c).ci).epos
                                                                  as
                                                                  libc::c_int
                                                          {
                                                        let mut cioff_1:
                                                                ptrdiff_t =
                                                            ci_3.wrapping_offset_from((*(*mrb).c).cibase)
                                                                as
                                                                libc::c_long;
                                                        ecall(mrb);
                                                        ci_3 =
                                                            (*(*mrb).c).cibase.offset(cioff_1
                                                                                          as
                                                                                          isize)
                                                    }
                                                    /* break from fiber block */
                                                    if ci_3 ==
                                                           (*(*mrb).c).cibase
                                                           &&
                                                           !(*ci_3).pc.is_null()
                                                       {
                                                        let mut c_2:
                                                                *mut mrb_context =
                                                            (*mrb).c;
                                                        (*mrb).c =
                                                            (*c_2).prev;
                                                        (*c_2).prev =
                                                            0 as
                                                                *mut mrb_context;
                                                        ci_3 = (*(*mrb).c).ci
                                                    }
                                                    if (*ci_3).acc < 0i32 {
                                                        mrb_gc_arena_restore(mrb,
                                                                             ai);
                                                        (*(*mrb).c).vmexec =
                                                            0i32 as mrb_bool;
                                                        (*mrb).exc =
                                                            break_new(mrb,
                                                                      proc_0,
                                                                      v_0) as
                                                                *mut RObject;
                                                        (*mrb).jmp = prev_jmp;
                                                        _longjmp((*prev_jmp).impl_0.as_mut_ptr(),
                                                                 1i32);
                                                    }
                                                    current_block =
                                                        5718902055567931348;
                                                    continue ;
                                                }
                                            }
                                            current_block =
                                                3814937494483549324;
                                        }
                                    }
                                    _ => {
                                        /* cannot happen */
                                        current_block = 17785511045760650023;
                                    }
                                }
                                match current_block {
                                    3814937494483549324 => { }
                                    17785511045760650023 => { }
                                    _ => {
                                        if ci_3 == (*(*mrb).c).cibase {
                                            let mut c_1: *mut mrb_context =
                                                (*mrb).c;
                                            if (*c_1).prev.is_null() {
                                                /* toplevel return */
                                                *(*(*mrb).c).stack.offset((*irep).nlocals
                                                                              as
                                                                              isize)
                                                    = v_0;
                                                current_block =
                                                    9425350357932430420;
                                                break ;
                                            } else if (*(*c_1).prev).ci ==
                                                          (*(*c_1).prev).cibase
                                             {
                                                let mut exc_8: mrb_value =
                                                    mrb_exc_new_str(mrb,
                                                                    mrb_exc_get(mrb,
                                                                                b"FiberError\x00"
                                                                                    as
                                                                                    *const u8
                                                                                    as
                                                                                    *const libc::c_char),
                                                                    mrb_str_new_static(mrb,
                                                                                       b"double resume\x00"
                                                                                           as
                                                                                           *const u8
                                                                                           as
                                                                                           *const libc::c_char,
                                                                                       (::std::mem::size_of::<[libc::c_char; 14]>()
                                                                                            as
                                                                                            libc::c_ulong).wrapping_sub(1i32
                                                                                                                            as
                                                                                                                            libc::c_ulong)));
                                                mrb_exc_set(mrb, exc_8);
                                                current_block =
                                                    2687857153341325290;
                                                continue ;
                                            } else {
                                                while (*c_1).eidx as
                                                          libc::c_int > 0i32 {
                                                    ecall(mrb);
                                                }
                                                /* automatic yield at the end */
                                                (*c_1).status =
                                                    MRB_FIBER_TERMINATED;
                                                (*mrb).c = (*c_1).prev;
                                                (*c_1).prev =
                                                    0 as *mut mrb_context;
                                                (*(*mrb).c).status =
                                                    MRB_FIBER_RUNNING;
                                                ci_3 = (*(*mrb).c).ci
                                            }
                                            current_block =
                                                17785511045760650023;
                                        } else {
                                            current_block =
                                                17785511045760650023;
                                        }
                                    }
                                }
                            }
                        }
                        3676359657258785160 => {
                            c = 1i32 as uint8_t;
                            mid =
                                mrb_intern_static(mrb,
                                                  b"<\x00" as *const u8 as
                                                      *const libc::c_char,
                                                  (::std::mem::size_of::<[libc::c_char; 2]>()
                                                       as
                                                       libc::c_ulong).wrapping_sub(1i32
                                                                                       as
                                                                                       libc::c_ulong));
                            current_block = 4800884466390615302;
                        }
                        14871291313029355442 => {
                            *(*(*mrb).c).stack.offset(a as isize) =
                                mrb_hash_get(mrb, kdict_0, k);
                            mrb_hash_delete_key(mrb, kdict_0, k);
                            current_block = 7149356873433890176;
                            continue ;
                        }
                        777853614834280397 => {
                            let mut str: mrb_value =
                                mrb_format(mrb,
                                           b"missing keyword: %v\x00" as
                                               *const u8 as
                                               *const libc::c_char, k);
                            mrb_exc_set(mrb,
                                        mrb_exc_new_str(mrb,
                                                        mrb_exc_get(mrb,
                                                                    b"ArgumentError\x00"
                                                                        as
                                                                        *const u8
                                                                        as
                                                                        *const libc::c_char),
                                                        str));
                            current_block = 2687857153341325290;
                            continue ;
                        }
                        14951089859709286127 => {
                            if argc_1 < m1_0 + m2_0 ||
                                   r_0 == 0i32 && argc_1 > len_0 + kd_0 {
                                argnum_error(mrb, (m1_0 + m2_0) as mrb_int);
                                current_block = 2687857153341325290;
                                continue ;
                            } else { current_block = 4611351410291638544; }
                        }
                        6523494653378992949 => {
                            c = 1i32 as uint8_t;
                            mid =
                                mrb_intern_static(mrb,
                                                  b"/\x00" as *const u8 as
                                                      *const libc::c_char,
                                                  (::std::mem::size_of::<[libc::c_char; 2]>()
                                                       as
                                                       libc::c_ulong).wrapping_sub(1i32
                                                                                       as
                                                                                       libc::c_ulong));
                            current_block = 4800884466390615302;
                        }
                        4739989325879360476 => {
                            x_2 =
                                (*(*(*mrb).c).stack.offset(a as
                                                               isize)).value.f;
                            y_2 =
                                (*(*(*mrb).c).stack.offset(a.wrapping_add(1i32
                                                                              as
                                                                              libc::c_uint)
                                                               as
                                                               isize)).value.f;
                            current_block = 8137017873359662078;
                        }
                        2808535963553584040 => {
                            x_2 =
                                (*(*(*mrb).c).stack.offset(a as
                                                               isize)).value.f;
                            y_2 =
                                (*(*(*mrb).c).stack.offset(a.wrapping_add(1i32
                                                                              as
                                                                              libc::c_uint)
                                                               as
                                                               isize)).value.i
                                    as mrb_float;
                            current_block = 8137017873359662078;
                        }
                        12887046741138131814 => {
                            x_2 =
                                (*(*(*mrb).c).stack.offset(a as
                                                               isize)).value.i
                                    as mrb_float;
                            y_2 =
                                (*(*(*mrb).c).stack.offset(a.wrapping_add(1i32
                                                                              as
                                                                              libc::c_uint)
                                                               as
                                                               isize)).value.f;
                            current_block = 8137017873359662078;
                        }
                        8282156447772606522 => {
                            x_2 =
                                (*(*(*mrb).c).stack.offset(a as
                                                               isize)).value.i
                                    as mrb_float;
                            y_2 =
                                (*(*(*mrb).c).stack.offset(a.wrapping_add(1i32
                                                                              as
                                                                              libc::c_uint)
                                                               as
                                                               isize)).value.i
                                    as mrb_float;
                            current_block = 8137017873359662078;
                        }
                        11695679538446617396 => {
                            c = 1i32 as uint8_t;
                            mid =
                                mrb_intern_static(mrb,
                                                  b"<=\x00" as *const u8 as
                                                      *const libc::c_char,
                                                  (::std::mem::size_of::<[libc::c_char; 3]>()
                                                       as
                                                       libc::c_ulong).wrapping_sub(1i32
                                                                                       as
                                                                                       libc::c_ulong));
                            current_block = 4800884466390615302;
                        }
                        9640155021185875683 => {
                            if lv == 0i32 {
                                stack = (*(*mrb).c).stack.offset(1isize);
                                current_block = 6314952843983381816;
                            } else {
                                let mut e_3: *mut REnv =
                                    uvenv(mrb, lv - 1i32);
                                if e_3.is_null() {
                                    current_block = 14860301496645178774;
                                } else if ((*e_3).flags() as libc::c_int &
                                               0x3ffi32) as mrb_int <=
                                              (m1 + r + m2 + kd + 1i32) as
                                                  libc::c_longlong {
                                    current_block = 14860301496645178774;
                                } else {
                                    stack = (*e_3).stack.offset(1isize);
                                    current_block = 6314952843983381816;
                                }
                            }
                            match current_block {
                                14860301496645178774 => { }
                                _ => {
                                    if r == 0i32 {
                                        *(*(*mrb).c).stack.offset(a as isize)
                                            =
                                            mrb_ary_new_from_values(mrb,
                                                                    (m1 + m2 +
                                                                         kd)
                                                                        as
                                                                        mrb_int,
                                                                    stack)
                                    } else {
                                        let mut pp: *mut mrb_value =
                                            0 as *mut mrb_value;
                                        let mut rest: *mut RArray =
                                            0 as *mut RArray;
                                        let mut len: libc::c_int = 0i32;
                                        if (*stack.offset(m1 as isize)).tt as
                                               libc::c_uint ==
                                               MRB_TT_ARRAY as libc::c_int as
                                                   libc::c_uint {
                                            let mut ary: *mut RArray =
                                                (*stack.offset(m1 as
                                                                   isize)).value.p
                                                    as *mut RArray;
                                            pp =
                                                if 0 !=
                                                       (*ary).flags() as
                                                           libc::c_int & 7i32
                                                   {
                                                    &mut (*ary).as_0 as
                                                        *mut C2RustUnnamed_5
                                                        as *mut mrb_value
                                                } else {
                                                    (*ary).as_0.heap.ptr
                                                };
                                            len =
                                                (if 0 !=
                                                        (*ary).flags() as
                                                            libc::c_int & 7i32
                                                    {
                                                     (((*ary).flags() as
                                                           libc::c_int & 7i32)
                                                          - 1i32) as mrb_int
                                                 } else {
                                                     (*ary).as_0.heap.len
                                                 }) as libc::c_int
                                        }
                                        *(*(*mrb).c).stack.offset(a as isize)
                                            =
                                            mrb_ary_new_capa(mrb,
                                                             (m1 + len + m2 +
                                                                  kd) as
                                                                 mrb_int);
                                        rest =
                                            (*(*(*mrb).c).stack.offset(a as
                                                                           isize)).value.p
                                                as *mut RArray;
                                        if m1 > 0i32 {
                                            stack_copy(if 0 !=
                                                              (*rest).flags()
                                                                  as
                                                                  libc::c_int
                                                                  & 7i32 {
                                                           &mut (*rest).as_0
                                                               as
                                                               *mut C2RustUnnamed_5
                                                               as
                                                               *mut mrb_value
                                                       } else {
                                                           (*rest).as_0.heap.ptr
                                                       }, stack,
                                                       m1 as size_t);
                                        }
                                        if len > 0i32 {
                                            stack_copy((if 0 !=
                                                               (*rest).flags()
                                                                   as
                                                                   libc::c_int
                                                                   & 7i32 {
                                                            &mut (*rest).as_0
                                                                as
                                                                *mut C2RustUnnamed_5
                                                                as
                                                                *mut mrb_value
                                                        } else {
                                                            (*rest).as_0.heap.ptr
                                                        }).offset(m1 as
                                                                      isize),
                                                       pp, len as size_t);
                                        }
                                        if m2 > 0i32 {
                                            stack_copy((if 0 !=
                                                               (*rest).flags()
                                                                   as
                                                                   libc::c_int
                                                                   & 7i32 {
                                                            &mut (*rest).as_0
                                                                as
                                                                *mut C2RustUnnamed_5
                                                                as
                                                                *mut mrb_value
                                                        } else {
                                                            (*rest).as_0.heap.ptr
                                                        }).offset(m1 as
                                                                      isize).offset(len
                                                                                        as
                                                                                        isize),
                                                       stack.offset(m1 as
                                                                        isize).offset(1isize),
                                                       m2 as size_t);
                                        }
                                        if 0 != kd {
                                            stack_copy((if 0 !=
                                                               (*rest).flags()
                                                                   as
                                                                   libc::c_int
                                                                   & 7i32 {
                                                            &mut (*rest).as_0
                                                                as
                                                                *mut C2RustUnnamed_5
                                                                as
                                                                *mut mrb_value
                                                        } else {
                                                            (*rest).as_0.heap.ptr
                                                        }).offset(m1 as
                                                                      isize).offset(len
                                                                                        as
                                                                                        isize).offset(m2
                                                                                                          as
                                                                                                          isize),
                                                       stack.offset(m1 as
                                                                        isize).offset(m2
                                                                                          as
                                                                                          isize).offset(1isize),
                                                       kd as size_t);
                                        }
                                        if 0 !=
                                               (*rest).flags() as libc::c_int
                                                   & 7i32 {
                                            (*rest).set_flags(((*rest).flags()
                                                                   as
                                                                   libc::c_int
                                                                   & !7i32) as
                                                                  libc::c_uint
                                                                  |
                                                                  ((m1 + len +
                                                                        m2 +
                                                                        kd) as
                                                                       uint32_t).wrapping_add(1i32
                                                                                                  as
                                                                                                  libc::c_uint))
                                        } else {
                                            (*rest).as_0.heap.len =
                                                (m1 + len + m2 + kd) as
                                                    mrb_int
                                        }
                                    }
                                    *(*(*mrb).c).stack.offset(a.wrapping_add(1i32
                                                                                 as
                                                                                 libc::c_uint)
                                                                  as isize) =
                                        *stack.offset((m1 + r + m2) as isize);
                                    mrb_gc_arena_restore(mrb, ai);
                                    current_block = 7149356873433890176;
                                    continue ;
                                }
                            }
                        }
                        11169057726334001291 => {
                            let mut p_3: *mut RProc = 0 as *mut RProc;
                            let mut nirep: *mut mrb_irep =
                                *(*irep).reps.offset(b as isize);
                            if 0 != c as libc::c_int & 2i32 {
                                p_3 = mrb_closure_new(mrb, nirep)
                            } else {
                                p_3 = mrb_proc_new(mrb, nirep);
                                (*p_3).set_flags((*p_3).flags() |
                                                     2048i32 as uint32_t)
                            }
                            if 0 != c as libc::c_int & 1i32 {
                                (*p_3).set_flags((*p_3).flags() |
                                                     256i32 as uint32_t)
                            }
                            *(*(*mrb).c).stack.offset(a as isize) =
                                mrb_obj_value(p_3 as *mut libc::c_void);
                            mrb_gc_arena_restore(mrb, ai);
                            current_block = 7149356873433890176;
                            continue ;
                        }
                        7296667520727038133 => {
                            if (*target_class_0).tt() as libc::c_int ==
                                   MRB_TT_MODULE as libc::c_int {
                                target_class_0 = (*ci_2).target_class;
                                if (*target_class_0).tt() as libc::c_int !=
                                       MRB_TT_ICLASS as libc::c_int {
                                    let mut exc_5: mrb_value =
                                        mrb_exc_new_str(mrb,
                                                        mrb_exc_get(mrb,
                                                                    b"RuntimeError\x00"
                                                                        as
                                                                        *const u8
                                                                        as
                                                                        *const libc::c_char),
                                                        mrb_str_new_static(mrb,
                                                                           b"superclass info lost [mruby limitations]\x00"
                                                                               as
                                                                               *const u8
                                                                               as
                                                                               *const libc::c_char,
                                                                           (::std::mem::size_of::<[libc::c_char; 41]>()
                                                                                as
                                                                                libc::c_ulong).wrapping_sub(1i32
                                                                                                                as
                                                                                                                libc::c_ulong)));
                                    mrb_exc_set(mrb, exc_5);
                                    current_block = 2687857153341325290;
                                    continue ;
                                }
                            }
                            recv_1 = *(*(*mrb).c).stack.offset(0isize);
                            if 0 ==
                                   mrb_obj_is_kind_of(mrb, recv_1,
                                                      target_class_0) {
                                let mut exc_6: mrb_value =
                                    mrb_exc_new_str(mrb,
                                                    mrb_exc_get(mrb,
                                                                b"TypeError\x00"
                                                                    as
                                                                    *const u8
                                                                    as
                                                                    *const libc::c_char),
                                                    mrb_str_new_static(mrb,
                                                                       b"self has wrong type to call super in this context\x00"
                                                                           as
                                                                           *const u8
                                                                           as
                                                                           *const libc::c_char,
                                                                       (::std::mem::size_of::<[libc::c_char; 50]>()
                                                                            as
                                                                            libc::c_ulong).wrapping_sub(1i32
                                                                                                            as
                                                                                                            libc::c_ulong)));
                                mrb_exc_set(mrb, exc_6);
                                current_block = 2687857153341325290;
                                continue ;
                            } else {
                                blk_0 =
                                    *(*(*mrb).c).stack.offset(bidx_2 as
                                                                  isize);
                                if !(blk_0.tt as libc::c_uint ==
                                         MRB_TT_FALSE as libc::c_int as
                                             libc::c_uint &&
                                         0 == blk_0.value.i) &&
                                       blk_0.tt as libc::c_uint !=
                                           MRB_TT_PROC as libc::c_int as
                                               libc::c_uint {
                                    blk_0 =
                                        mrb_convert_type(mrb, blk_0,
                                                         MRB_TT_PROC,
                                                         b"Proc\x00" as
                                                             *const u8 as
                                                             *const libc::c_char,
                                                         b"to_proc\x00" as
                                                             *const u8 as
                                                             *const libc::c_char);
                                    /* The stack or ci stack might have been reallocated during
           mrb_convert_type(), see #3622 and #3784 */
                                    *(*(*mrb).c).stack.offset(bidx_2 as isize)
                                        = blk_0;
                                    ci_2 = (*(*mrb).c).ci
                                }
                                cls_0 = (*target_class_0).super_0;
                                m_1 =
                                    mrb_method_search_vm(mrb, &mut cls_0,
                                                         mid_1);
                                if m_1.c2rust_unnamed.proc_0.is_null() {
                                    let mut missing_0: mrb_sym =
                                        mrb_intern_static(mrb,
                                                          b"method_missing\x00"
                                                              as *const u8 as
                                                              *const libc::c_char,
                                                          (::std::mem::size_of::<[libc::c_char; 15]>()
                                                               as
                                                               libc::c_ulong).wrapping_sub(1i32
                                                                                               as
                                                                                               libc::c_ulong));
                                    if mid_1 != missing_0 {
                                        cls_0 = mrb_class(mrb, recv_1)
                                    }
                                    m_1 =
                                        mrb_method_search_vm(mrb, &mut cls_0,
                                                             missing_0);
                                    if m_1.c2rust_unnamed.proc_0.is_null() {
                                        let mut args_0: mrb_value =
                                            if argc_0 < 0i32 {
                                                *(*(*mrb).c).stack.offset(a.wrapping_add(1i32
                                                                                             as
                                                                                             libc::c_uint)
                                                                              as
                                                                              isize)
                                            } else {
                                                mrb_ary_new_from_values(mrb,
                                                                        b as
                                                                            mrb_int,
                                                                        (*(*mrb).c).stack.offset(a
                                                                                                     as
                                                                                                     isize).offset(1isize))
                                            };
                                        (*(*(*mrb).c).ci).err = pc0;
                                        mrb_method_missing(mrb, mid_1, recv_1,
                                                           args_0);
                                    }
                                    mid_1 = missing_0;
                                    if argc_0 >= 0i32 {
                                        if a.wrapping_add(2i32 as
                                                              libc::c_uint) >=
                                               (*irep).nregs as libc::c_uint {
                                            mrb_stack_extend(mrb,
                                                             a.wrapping_add(3i32
                                                                                as
                                                                                libc::c_uint)
                                                                 as mrb_int);
                                        }
                                        *(*(*mrb).c).stack.offset(a.wrapping_add(1i32
                                                                                     as
                                                                                     libc::c_uint)
                                                                      as
                                                                      isize) =
                                            mrb_ary_new_from_values(mrb,
                                                                    b as
                                                                        mrb_int,
                                                                    (*(*mrb).c).stack.offset(a
                                                                                                 as
                                                                                                 isize).offset(1isize));
                                        *(*(*mrb).c).stack.offset(a.wrapping_add(2i32
                                                                                     as
                                                                                     libc::c_uint)
                                                                      as
                                                                      isize) =
                                            blk_0;
                                        argc_0 = -1i32
                                    }
                                    mrb_ary_unshift(mrb,
                                                    *(*(*mrb).c).stack.offset(a.wrapping_add(1i32
                                                                                                 as
                                                                                                 libc::c_uint)
                                                                                  as
                                                                                  isize),
                                                    mrb_symbol_value((*ci_2).mid));
                                }
                                /* push callinfo */
                                ci_2 = cipush(mrb);
                                (*ci_2).mid = mid_1;
                                (*ci_2).stackent = (*(*mrb).c).stack;
                                (*ci_2).target_class = cls_0;
                                (*ci_2).pc = pc;
                                (*ci_2).argc = argc_0;
                                /* prepare stack */
                                (*(*mrb).c).stack =
                                    (*(*mrb).c).stack.offset(a as isize);
                                *(*(*mrb).c).stack.offset(0isize) = recv_1;
                                if 0 !=
                                       if 0 != m_1.func_p as libc::c_int {
                                           1i32
                                       } else if !m_1.c2rust_unnamed.proc_0.is_null()
                                        {
                                           ((*m_1.c2rust_unnamed.proc_0).flags()
                                                as libc::c_int & 128i32 !=
                                                0i32) as libc::c_int
                                       } else { 0i32 } {
                                    let mut v: mrb_value =
                                        mrb_value{value:
                                                      C2RustUnnamed_0{f: 0.,},
                                                  tt: MRB_TT_FALSE,};
                                    if 0 == m_1.func_p {
                                        (*ci_2).proc_0 =
                                            m_1.c2rust_unnamed.proc_0
                                    }
                                    v =
                                        if 0 != m_1.func_p as libc::c_int {
                                            m_1.c2rust_unnamed.func
                                        } else if !m_1.c2rust_unnamed.proc_0.is_null()
                                                      &&
                                                      (*m_1.c2rust_unnamed.proc_0).flags()
                                                          as libc::c_int &
                                                          128i32 != 0i32 {
                                            (*m_1.c2rust_unnamed.proc_0).body.func
                                        } else {
                                            None
                                        }.expect("non-null function pointer")(mrb,
                                                                              recv_1);
                                    mrb_gc_arena_restore(mrb, ai);
                                    if !(*mrb).exc.is_null() {
                                        current_block = 2687857153341325290;
                                        continue ;
                                    }
                                    ci_2 = (*(*mrb).c).ci;
                                    if (*ci_2).target_class.is_null() {
                                        /* return from context modifying method (resume/yield) */
                                        if (*ci_2).acc == -3i32 {
                                            (*mrb).jmp = prev_jmp;
                                            return v
                                        } else {
                                            proc_0 =
                                                (*ci_2.offset(-1i32 as
                                                                  isize)).proc_0;
                                            irep = (*proc_0).body.irep;
                                            pool = (*irep).pool;
                                            syms = (*irep).syms
                                        }
                                    }
                                    *(*(*mrb).c).stack.offset(0isize) = v;
                                    /* pop stackpos */
                                    (*(*mrb).c).stack = (*ci_2).stackent;
                                    pc = (*ci_2).pc;
                                    cipop(mrb);
                                    current_block = 7149356873433890176;
                                    continue ;
                                } else {
                                    /* fill callinfo */
                                    (*ci_2).acc = a as libc::c_int;
                                    /* setup environment for calling method */
                                    (*ci_2).proc_0 =
                                        m_1.c2rust_unnamed.proc_0;
                                    proc_0 = (*ci_2).proc_0;
                                    irep = (*proc_0).body.irep;
                                    pool = (*irep).pool;
                                    syms = (*irep).syms;
                                    mrb_stack_extend(mrb,
                                                     (if argc_0 < 0i32 &&
                                                             ((*irep).nregs as
                                                                  libc::c_int)
                                                                 < 3i32 {
                                                          3i32
                                                      } else {
                                                          (*irep).nregs as
                                                              libc::c_int
                                                      }) as mrb_int);
                                    pc = (*irep).iseq;
                                    current_block = 7149356873433890176;
                                    continue ;
                                }
                            }
                        }
                        18074041116490187063 => {
                            let mut exc_4: mrb_value =
                                mrb_exc_new_str(mrb,
                                                mrb_exc_get(mrb,
                                                            b"NoMethodError\x00"
                                                                as *const u8
                                                                as
                                                                *const libc::c_char),
                                                mrb_str_new_static(mrb,
                                                                   b"super called outside of method\x00"
                                                                       as
                                                                       *const u8
                                                                       as
                                                                       *const libc::c_char,
                                                                   (::std::mem::size_of::<[libc::c_char; 31]>()
                                                                        as
                                                                        libc::c_ulong).wrapping_sub(1i32
                                                                                                        as
                                                                                                        libc::c_ulong)));
                            mrb_exc_set(mrb, exc_4);
                            current_block = 2687857153341325290;
                            continue ;
                        }
                        12115052188632565762 => {
                            c = 1i32 as uint8_t;
                            mid =
                                mrb_intern_static(mrb,
                                                  b">\x00" as *const u8 as
                                                      *const libc::c_char,
                                                  (::std::mem::size_of::<[libc::c_char; 2]>()
                                                       as
                                                       libc::c_ulong).wrapping_sub(1i32
                                                                                       as
                                                                                       libc::c_ulong));
                            current_block = 4800884466390615302;
                        }
                        4156951849113663992 => {
                            c = 1i32 as uint8_t;
                            mid =
                                mrb_intern_static(mrb,
                                                  b">=\x00" as *const u8 as
                                                      *const libc::c_char,
                                                  (::std::mem::size_of::<[libc::c_char; 3]>()
                                                       as
                                                       libc::c_ulong).wrapping_sub(1i32
                                                                                       as
                                                                                       libc::c_ulong));
                            current_block = 4800884466390615302;
                        }
                        7975968915446637229 => {
                            let mut e_6: *mut REnv = uvenv(mrb, lv_0 - 1i32);
                            if e_6.is_null() ||
                                   !((*e_6).flags() as libc::c_int &
                                         1i32 << 20i32 == 0i32) &&
                                       (*e_6).mid == 0i32 as libc::c_uint ||
                                   ((*e_6).flags() as libc::c_int & 0x3ffi32)
                                       as mrb_int <=
                                       (m1_1 + r_1 + m2_1 + 1i32) as
                                           libc::c_longlong {
                                localjump_error(mrb, LOCALJUMP_ERROR_YIELD);
                                current_block = 2687857153341325290;
                                continue ;
                            } else { stack_0 = (*e_6).stack.offset(1isize) }
                            current_block = 5046807210318655509;
                        }
                        14455231216035570027 => {
                            c = 1i32 as uint8_t;
                            mid =
                                mrb_intern_static(mrb,
                                                  b"==\x00" as *const u8 as
                                                      *const libc::c_char,
                                                  (::std::mem::size_of::<[libc::c_char; 3]>()
                                                       as
                                                       libc::c_ulong).wrapping_sub(1i32
                                                                                       as
                                                                                       libc::c_ulong));
                            current_block = 4800884466390615302;
                        }
                        16020430568569951085 => {
                            /* expand ensure stack */
                            if (*(*mrb).c).esize as libc::c_int <=
                                   (*(*mrb).c).eidx as libc::c_int + 1i32 {
                                if (*(*mrb).c).esize as libc::c_int == 0i32 {
                                    (*(*mrb).c).esize = 16i32 as uint16_t
                                } else {
                                    (*(*mrb).c).esize =
                                        ((*(*mrb).c).esize as libc::c_int *
                                             2i32) as uint16_t;
                                    if (*(*mrb).c).esize as libc::c_int <=
                                           (*(*mrb).c).eidx as libc::c_int {
                                        (*(*mrb).c).esize =
                                            65535i32 as uint16_t
                                    }
                                }
                                (*(*mrb).c).ensure =
                                    mrb_realloc(mrb,
                                                (*(*mrb).c).ensure as
                                                    *mut libc::c_void,
                                                (::std::mem::size_of::<*mut RProc>()
                                                     as
                                                     libc::c_ulong).wrapping_mul((*(*mrb).c).esize
                                                                                     as
                                                                                     libc::c_ulong))
                                        as *mut *mut RProc
                            }
                            /* push ensure stack */
                            let fresh123 = (*(*mrb).c).eidx;
                            (*(*mrb).c).eidx =
                                (*(*mrb).c).eidx.wrapping_add(1);
                            let ref mut fresh124 =
                                *(*(*mrb).c).ensure.offset(fresh123 as isize);
                            *fresh124 = p;
                            let ref mut fresh125 =
                                *(*(*mrb).c).ensure.offset((*(*mrb).c).eidx as
                                                               isize);
                            *fresh125 = 0 as *mut RProc;
                            mrb_gc_arena_restore(mrb, ai);
                            current_block = 7149356873433890176;
                            continue ;
                        }
                        11870793248822581739 => {
                            let mut exc_3: mrb_value =
                                mrb_exc_new_str(mrb,
                                                mrb_exc_get(mrb,
                                                            b"RuntimeError\x00"
                                                                as *const u8
                                                                as
                                                                *const libc::c_char),
                                                mrb_str_new_static(mrb,
                                                                   b"too many nested ensures\x00"
                                                                       as
                                                                       *const u8
                                                                       as
                                                                       *const libc::c_char,
                                                                   (::std::mem::size_of::<[libc::c_char; 24]>()
                                                                        as
                                                                        libc::c_ulong).wrapping_sub(1i32
                                                                                                        as
                                                                                                        libc::c_ulong)));
                            mrb_exc_set(mrb, exc_3);
                            current_block = 2687857153341325290;
                            continue ;
                        }
                        11989315111553324117 => {
                            ec = e_1.value.p as *mut RClass;
                            *(*(*mrb).c).stack.offset(b as isize) =
                                mrb_bool_value(mrb_obj_is_kind_of(mrb, exc_1,
                                                                  ec));
                            current_block = 7149356873433890176;
                            continue ;
                        }
                        13561616326120682545 => {
                            let mut exc_2: mrb_value =
                                mrb_value{value: C2RustUnnamed_0{f: 0.,},
                                          tt: MRB_TT_FALSE,};
                            exc_2 =
                                mrb_exc_new_str(mrb,
                                                mrb_exc_get(mrb,
                                                            b"TypeError\x00"
                                                                as *const u8
                                                                as
                                                                *const libc::c_char),
                                                mrb_str_new_static(mrb,
                                                                   b"class or module required for rescue clause\x00"
                                                                       as
                                                                       *const u8
                                                                       as
                                                                       *const libc::c_char,
                                                                   (::std::mem::size_of::<[libc::c_char; 43]>()
                                                                        as
                                                                        libc::c_ulong).wrapping_sub(1i32
                                                                                                        as
                                                                                                        libc::c_ulong)));
                            mrb_exc_set(mrb, exc_2);
                            current_block = 2687857153341325290;
                            continue ;
                        }
                        1314190040982086846 => {
                            result_3 =
                                ((*(*(*mrb).c).stack.offset(a as
                                                                isize)).value.f
                                     >=
                                     (*(*(*mrb).c).stack.offset(a.wrapping_add(1i32
                                                                                   as
                                                                                   libc::c_uint)
                                                                    as
                                                                    isize)).value.f)
                                    as libc::c_int;
                            current_block = 13666743919129817623;
                        }
                        5381297608968995174 => {
                            result =
                                ((*(*(*mrb).c).stack.offset(a as
                                                                isize)).value.i
                                     ==
                                     (*(*(*mrb).c).stack.offset(a.wrapping_add(1i32
                                                                                   as
                                                                                   libc::c_uint)
                                                                    as
                                                                    isize)).value.i)
                                    as libc::c_int;
                            current_block = 18322908395017252093;
                        }
                        7679224362344455344 => {
                            result =
                                ((*(*(*mrb).c).stack.offset(a as
                                                                isize)).value.i
                                     as libc::c_double ==
                                     (*(*(*mrb).c).stack.offset(a.wrapping_add(1i32
                                                                                   as
                                                                                   libc::c_uint)
                                                                    as
                                                                    isize)).value.f)
                                    as libc::c_int;
                            current_block = 18322908395017252093;
                        }
                        15176476975632920360 => {
                            result =
                                ((*(*(*mrb).c).stack.offset(a as
                                                                isize)).value.f
                                     ==
                                     (*(*(*mrb).c).stack.offset(a.wrapping_add(1i32
                                                                                   as
                                                                                   libc::c_uint)
                                                                    as
                                                                    isize)).value.i
                                         as libc::c_double) as libc::c_int;
                            current_block = 18322908395017252093;
                        }
                        3733621521558651155 => {
                            result =
                                ((*(*(*mrb).c).stack.offset(a as
                                                                isize)).value.f
                                     ==
                                     (*(*(*mrb).c).stack.offset(a.wrapping_add(1i32
                                                                                   as
                                                                                   libc::c_uint)
                                                                    as
                                                                    isize)).value.f)
                                    as libc::c_int;
                            current_block = 18322908395017252093;
                        }
                        3624932733755770481 => {
                            result_0 =
                                ((*(*(*mrb).c).stack.offset(a as
                                                                isize)).value.i
                                     <
                                     (*(*(*mrb).c).stack.offset(a.wrapping_add(1i32
                                                                                   as
                                                                                   libc::c_uint)
                                                                    as
                                                                    isize)).value.i)
                                    as libc::c_int;
                            current_block = 15715503430877517545;
                        }
                        6802740918435369298 => {
                            result_0 =
                                (((*(*(*mrb).c).stack.offset(a as
                                                                 isize)).value.i
                                      as libc::c_double) <
                                     (*(*(*mrb).c).stack.offset(a.wrapping_add(1i32
                                                                                   as
                                                                                   libc::c_uint)
                                                                    as
                                                                    isize)).value.f)
                                    as libc::c_int;
                            current_block = 15715503430877517545;
                        }
                        18056191432036799439 => {
                            result_0 =
                                ((*(*(*mrb).c).stack.offset(a as
                                                                isize)).value.f
                                     <
                                     (*(*(*mrb).c).stack.offset(a.wrapping_add(1i32
                                                                                   as
                                                                                   libc::c_uint)
                                                                    as
                                                                    isize)).value.i
                                         as libc::c_double) as libc::c_int;
                            current_block = 15715503430877517545;
                        }
                        11951279088167397802 => {
                            result_0 =
                                ((*(*(*mrb).c).stack.offset(a as
                                                                isize)).value.f
                                     <
                                     (*(*(*mrb).c).stack.offset(a.wrapping_add(1i32
                                                                                   as
                                                                                   libc::c_uint)
                                                                    as
                                                                    isize)).value.f)
                                    as libc::c_int;
                            current_block = 15715503430877517545;
                        }
                        1846749606198814734 => {
                            /* extract first argument array to arguments */
                            if len_0 > 1i32 && argc_1 == 1i32 &&
                                   (*argv.offset(0isize)).tt as libc::c_uint
                                       ==
                                       MRB_TT_ARRAY as libc::c_int as
                                           libc::c_uint {
                                mrb_gc_protect(mrb, *argv.offset(0isize));
                                argc_1 =
                                    (if 0 !=
                                            (*((*argv.offset(0isize)).value.p
                                                   as *mut RArray)).flags() as
                                                libc::c_int & 7i32 {
                                         (((*((*argv.offset(0isize)).value.p
                                                  as *mut RArray)).flags() as
                                               libc::c_int & 7i32) - 1i32) as
                                             mrb_int
                                     } else {
                                         (*((*argv.offset(0isize)).value.p as
                                                *mut RArray)).as_0.heap.len
                                     }) as libc::c_int;
                                argv =
                                    if 0 !=
                                           (*((*argv.offset(0isize)).value.p
                                                  as *mut RArray)).flags() as
                                               libc::c_int & 7i32 {
                                        &mut (*((*argv.offset(0isize)).value.p
                                                    as *mut RArray)).as_0 as
                                            *mut C2RustUnnamed_5 as
                                            *mut mrb_value
                                    } else {
                                        (*((*argv.offset(0isize)).value.p as
                                               *mut RArray)).as_0.heap.ptr
                                    }
                            }
                            current_block = 4611351410291638544;
                        }
                        14979696880445242837 => {
                            stack_0 = (*(*mrb).c).stack.offset(1isize);
                            current_block = 5046807210318655509;
                        }
                        15236292558918277616 => {
                            exc_7 =
                                mrb_value{value: C2RustUnnamed_0{f: 0.,},
                                          tt: MRB_TT_FALSE,};
                            current_block = 14860301496645178774;
                        }
                        3527484645425325286 => {
                            result_1 =
                                ((*(*(*mrb).c).stack.offset(a as
                                                                isize)).value.i
                                     <=
                                     (*(*(*mrb).c).stack.offset(a.wrapping_add(1i32
                                                                                   as
                                                                                   libc::c_uint)
                                                                    as
                                                                    isize)).value.i)
                                    as libc::c_int;
                            current_block = 8728320426208223650;
                        }
                        3075363095485550289 => {
                            result_1 =
                                ((*(*(*mrb).c).stack.offset(a as
                                                                isize)).value.i
                                     as libc::c_double <=
                                     (*(*(*mrb).c).stack.offset(a.wrapping_add(1i32
                                                                                   as
                                                                                   libc::c_uint)
                                                                    as
                                                                    isize)).value.f)
                                    as libc::c_int;
                            current_block = 8728320426208223650;
                        }
                        8736935332486372157 => {
                            result_1 =
                                ((*(*(*mrb).c).stack.offset(a as
                                                                isize)).value.f
                                     <=
                                     (*(*(*mrb).c).stack.offset(a.wrapping_add(1i32
                                                                                   as
                                                                                   libc::c_uint)
                                                                    as
                                                                    isize)).value.i
                                         as libc::c_double) as libc::c_int;
                            current_block = 8728320426208223650;
                        }
                        7747793278649614952 => {
                            result_1 =
                                ((*(*(*mrb).c).stack.offset(a as
                                                                isize)).value.f
                                     <=
                                     (*(*(*mrb).c).stack.offset(a.wrapping_add(1i32
                                                                                   as
                                                                                   libc::c_uint)
                                                                    as
                                                                    isize)).value.f)
                                    as libc::c_int;
                            current_block = 8728320426208223650;
                        }
                        12909619691863992879 => {
                            result_2 =
                                ((*(*(*mrb).c).stack.offset(a as
                                                                isize)).value.i
                                     >
                                     (*(*(*mrb).c).stack.offset(a.wrapping_add(1i32
                                                                                   as
                                                                                   libc::c_uint)
                                                                    as
                                                                    isize)).value.i)
                                    as libc::c_int;
                            current_block = 8359968701383644501;
                        }
                        16805217273881322446 => {
                            result_2 =
                                ((*(*(*mrb).c).stack.offset(a as
                                                                isize)).value.i
                                     as libc::c_double >
                                     (*(*(*mrb).c).stack.offset(a.wrapping_add(1i32
                                                                                   as
                                                                                   libc::c_uint)
                                                                    as
                                                                    isize)).value.f)
                                    as libc::c_int;
                            current_block = 8359968701383644501;
                        }
                        678496273481856040 => {
                            result_2 =
                                ((*(*(*mrb).c).stack.offset(a as
                                                                isize)).value.f
                                     >
                                     (*(*(*mrb).c).stack.offset(a.wrapping_add(1i32
                                                                                   as
                                                                                   libc::c_uint)
                                                                    as
                                                                    isize)).value.i
                                         as libc::c_double) as libc::c_int;
                            current_block = 8359968701383644501;
                        }
                        3978484480214358740 => {
                            result_2 =
                                ((*(*(*mrb).c).stack.offset(a as
                                                                isize)).value.f
                                     >
                                     (*(*(*mrb).c).stack.offset(a.wrapping_add(1i32
                                                                                   as
                                                                                   libc::c_uint)
                                                                    as
                                                                    isize)).value.f)
                                    as libc::c_int;
                            current_block = 8359968701383644501;
                        }
                        17916663512757813856 => {
                            result_3 =
                                ((*(*(*mrb).c).stack.offset(a as
                                                                isize)).value.i
                                     >=
                                     (*(*(*mrb).c).stack.offset(a.wrapping_add(1i32
                                                                                   as
                                                                                   libc::c_uint)
                                                                    as
                                                                    isize)).value.i)
                                    as libc::c_int;
                            current_block = 13666743919129817623;
                        }
                        3348445690345083753 => {
                            result_3 =
                                ((*(*(*mrb).c).stack.offset(a as
                                                                isize)).value.i
                                     as libc::c_double >=
                                     (*(*(*mrb).c).stack.offset(a.wrapping_add(1i32
                                                                                   as
                                                                                   libc::c_uint)
                                                                    as
                                                                    isize)).value.f)
                                    as libc::c_int;
                            current_block = 13666743919129817623;
                        }
                        16076866197496778268 => {
                            result_3 =
                                ((*(*(*mrb).c).stack.offset(a as
                                                                isize)).value.f
                                     >=
                                     (*(*(*mrb).c).stack.offset(a.wrapping_add(1i32
                                                                                   as
                                                                                   libc::c_uint)
                                                                    as
                                                                    isize)).value.i
                                         as libc::c_double) as libc::c_int;
                            current_block = 13666743919129817623;
                        }
                        _ => { }
                    }
                    match current_block {
                        17785511045760650023 => { }
                        3814937494483549324 => { }
                        _ => {
                            match current_block {
                                8137017873359662078 => {
                                    if y_2 == 0i32 as libc::c_double {
                                        if x_2 > 0i32 as libc::c_double {
                                            f =
                                                ::std::f32::INFINITY as
                                                    libc::c_double
                                        } else if x_2 < 0i32 as libc::c_double
                                         {
                                            f =
                                                -::std::f32::INFINITY as
                                                    libc::c_double
                                        } else {
                                            f =
                                                ::std::f32::NAN as
                                                    libc::c_double
                                        }
                                    } else { f = x_2 / y_2 }
                                    (*(*(*mrb).c).stack.offset(a as isize)).tt
                                        = MRB_TT_FLOAT;
                                    (*(*(*mrb).c).stack.offset(a as
                                                                   isize)).value.f
                                        = f;
                                    current_block = 7149356873433890176;
                                    continue ;
                                }
                                4611351410291638544 => {
                                    if 0 != kd_0 {
                                        /* check last arguments is hash if method takes keyword arguments */
                                        if argc_1 == m1_0 + m2_0 {
                                            kdict = mrb_hash_new(mrb);
                                            kargs = 0i32
                                        } else {
                                            if !argv.is_null() &&
                                                   argc_1 > 0i32 &&
                                                   (*argv.offset((argc_1 -
                                                                      1i32) as
                                                                     isize)).tt
                                                       as libc::c_uint ==
                                                       MRB_TT_HASH as
                                                           libc::c_int as
                                                           libc::c_uint {
                                                kdict =
                                                    *argv.offset((argc_1 -
                                                                      1i32) as
                                                                     isize);
                                                mrb_hash_check_kdict(mrb,
                                                                     kdict);
                                            } else if 0 != r_0 ||
                                                          argc_1 <=
                                                              m1_0 + m2_0 + o
                                                          ||
                                                          !(!(*(*(*mrb).c).ci).proc_0.is_null()
                                                                &&
                                                                (*(*(*(*mrb).c).ci).proc_0).flags()
                                                                    as
                                                                    libc::c_int
                                                                    & 256i32
                                                                    != 0i32) {
                                                kdict = mrb_hash_new(mrb);
                                                kargs = 0i32
                                            } else {
                                                argnum_error(mrb,
                                                             (m1_0 + m2_0) as
                                                                 mrb_int);
                                                current_block =
                                                    2687857153341325290;
                                                continue ;
                                            }
                                            if a >> 2i32 &
                                                   0x1fi32 as libc::c_uint >
                                                   0i32 as libc::c_uint {
                                                kdict =
                                                    mrb_hash_dup(mrb, kdict)
                                            }
                                        }
                                    }
                                    /* no rest arguments */
                                    if argc_1 - kargs < len_0 {
                                        let mut mlen: libc::c_int = m2_0;
                                        if argc_1 < m1_0 + m2_0 {
                                            mlen =
                                                if m1_0 < argc_1 {
                                                    argc_1 - m1_0
                                                } else { 0i32 }
                                        }
                                        /* move block */
                                        *(*(*mrb).c).stack.offset(blk_pos as
                                                                      isize) =
                                            *blk_1;
                                        if 0 != kd_0 {
                                            *(*(*mrb).c).stack.offset((len_0 +
                                                                           1i32)
                                                                          as
                                                                          isize)
                                                = kdict
                                        }
                                        /* copy mandatory and optional arguments */
                                        if argv0 != argv {
                                            /* m1 + o */
                                            value_move(&mut *(*(*mrb).c).stack.offset(1isize),
                                                       argv,
                                                       (argc_1 - mlen) as
                                                           size_t);
                                        }
                                        if argc_1 < m1_0 {
                                            stack_clear(&mut *(*(*mrb).c).stack.offset((argc_1
                                                                                            +
                                                                                            1i32)
                                                                                           as
                                                                                           isize),
                                                        (m1_0 - argc_1) as
                                                            size_t);
                                        }
                                        /* copy post mandatory arguments */
                                        if 0 != mlen {
                                            value_move(&mut *(*(*mrb).c).stack.offset((len_0
                                                                                           -
                                                                                           m2_0
                                                                                           +
                                                                                           1i32)
                                                                                          as
                                                                                          isize),
                                                       &mut *argv.offset((argc_1
                                                                              -
                                                                              mlen)
                                                                             as
                                                                             isize),
                                                       mlen as size_t);
                                        }
                                        if mlen < m2_0 {
                                            stack_clear(&mut *(*(*mrb).c).stack.offset((len_0
                                                                                            -
                                                                                            m2_0
                                                                                            +
                                                                                            mlen
                                                                                            +
                                                                                            1i32)
                                                                                           as
                                                                                           isize),
                                                        (m2_0 - mlen) as
                                                            size_t);
                                        }
                                        /* initalize rest arguments with empty Array */
                                        if 0 != r_0 {
                                            *(*(*mrb).c).stack.offset((m1_0 +
                                                                           o +
                                                                           1i32)
                                                                          as
                                                                          isize)
                                                =
                                                mrb_ary_new_capa(mrb,
                                                                 0i32 as
                                                                     mrb_int)
                                        }
                                        /* skip initailizer of passed arguments */
                                        if o > 0i32 &&
                                               argc_1 - kargs > m1_0 + m2_0 {
                                            pc =
                                                pc.offset(((argc_1 - kargs -
                                                                m1_0 - m2_0) *
                                                               3i32) as isize)
                                        }
                                    } else {
                                        let mut rnum: libc::c_int = 0i32;
                                        if argv0 != argv {
                                            /* move block */
                                            *(*(*mrb).c).stack.offset(blk_pos
                                                                          as
                                                                          isize)
                                                = *blk_1;
                                            if 0 != kd_0 {
                                                *(*(*mrb).c).stack.offset((len_0
                                                                               +
                                                                               1i32)
                                                                              as
                                                                              isize)
                                                    = kdict
                                            }
                                            value_move(&mut *(*(*mrb).c).stack.offset(1isize),
                                                       argv,
                                                       (m1_0 + o) as size_t);
                                        }
                                        if 0 != r_0 {
                                            let mut ary_1: mrb_value =
                                                mrb_value{value:
                                                              C2RustUnnamed_0{f:
                                                                                  0.,},
                                                          tt: MRB_TT_FALSE,};
                                            rnum =
                                                argc_1 - m1_0 - o - m2_0 -
                                                    kargs;
                                            ary_1 =
                                                mrb_ary_new_from_values(mrb,
                                                                        rnum
                                                                            as
                                                                            mrb_int,
                                                                        argv.offset(m1_0
                                                                                        as
                                                                                        isize).offset(o
                                                                                                          as
                                                                                                          isize));
                                            *(*(*mrb).c).stack.offset((m1_0 +
                                                                           o +
                                                                           1i32)
                                                                          as
                                                                          isize)
                                                = ary_1
                                        }
                                        if 0 != m2_0 {
                                            if argc_1 - m2_0 > m1_0 {
                                                value_move(&mut *(*(*mrb).c).stack.offset((m1_0
                                                                                               +
                                                                                               o
                                                                                               +
                                                                                               r_0
                                                                                               +
                                                                                               1i32)
                                                                                              as
                                                                                              isize),
                                                           &mut *argv.offset((m1_0
                                                                                  +
                                                                                  o
                                                                                  +
                                                                                  rnum)
                                                                                 as
                                                                                 isize),
                                                           m2_0 as size_t);
                                            }
                                        }
                                        if argv0 == argv {
                                            /* move block */
                                            *(*(*mrb).c).stack.offset(blk_pos
                                                                          as
                                                                          isize)
                                                = *blk_1;
                                            if 0 != kd_0 {
                                                *(*(*mrb).c).stack.offset((len_0
                                                                               +
                                                                               1i32)
                                                                              as
                                                                              isize)
                                                    = kdict
                                            }
                                        }
                                        pc = pc.offset((o * 3i32) as isize)
                                    }
                                    /* format arguments for generated code */
                                    (*(*(*mrb).c).ci).argc = len_0 + kd_0;
                                    /* clear local (but non-argument) variables */
                                    if (*irep).nlocals as libc::c_int -
                                           blk_pos - 1i32 > 0i32 {
                                        stack_clear(&mut *(*(*mrb).c).stack.offset((blk_pos
                                                                                        +
                                                                                        1i32)
                                                                                       as
                                                                                       isize),
                                                    ((*irep).nlocals as
                                                         libc::c_int - blk_pos
                                                         - 1i32) as size_t);
                                    }
                                    current_block = 7149356873433890176;
                                    continue ;
                                }
                                14860301496645178774 => {
                                    exc_7 =
                                        mrb_exc_new_str(mrb,
                                                        mrb_exc_get(mrb,
                                                                    b"NoMethodError\x00"
                                                                        as
                                                                        *const u8
                                                                        as
                                                                        *const libc::c_char),
                                                        mrb_str_new_static(mrb,
                                                                           b"super called outside of method\x00"
                                                                               as
                                                                               *const u8
                                                                               as
                                                                               *const libc::c_char,
                                                                           (::std::mem::size_of::<[libc::c_char; 31]>()
                                                                                as
                                                                                libc::c_ulong).wrapping_sub(1i32
                                                                                                                as
                                                                                                                libc::c_ulong)));
                                    mrb_exc_set(mrb, exc_7);
                                    current_block = 2687857153341325290;
                                    continue ;
                                }
                                5046807210318655509 => {
                                    if (*stack_0.offset((m1_1 + r_1 + m2_1) as
                                                            isize)).tt as
                                           libc::c_uint ==
                                           MRB_TT_FALSE as libc::c_int as
                                               libc::c_uint &&
                                           0 ==
                                               (*stack_0.offset((m1_1 + r_1 +
                                                                     m2_1) as
                                                                    isize)).value.i
                                       {
                                        localjump_error(mrb,
                                                        LOCALJUMP_ERROR_YIELD);
                                        current_block = 2687857153341325290;
                                        continue ;
                                    } else {
                                        *(*(*mrb).c).stack.offset(a as isize)
                                            =
                                            *stack_0.offset((m1_1 + r_1 + m2_1
                                                                 + kd_1) as
                                                                isize);
                                        current_block = 7149356873433890176;
                                        continue ;
                                    }
                                }
                                4800884466390615302 => {
                                    /* push nil after arguments */
                                    let mut bidx_0: libc::c_int =
                                        (if c as libc::c_int == 127i32 {
                                             a.wrapping_add(2i32 as
                                                                libc::c_uint)
                                         } else {
                                             a.wrapping_add(c as
                                                                libc::c_uint).wrapping_add(1i32
                                                                                               as
                                                                                               libc::c_uint)
                                         }) as libc::c_int;
                                    (*(*(*mrb).c).stack.offset(bidx_0 as
                                                                   isize)).tt
                                        = MRB_TT_FALSE;
                                    (*(*(*mrb).c).stack.offset(bidx_0 as
                                                                   isize)).value.i
                                        = 0i32 as mrb_int
                                }
                                13666743919129817623 => {
                                    if 0 != result_3 {
                                        (*(*(*mrb).c).stack.offset(a as
                                                                       isize)).tt
                                            = MRB_TT_TRUE;
                                        (*(*(*mrb).c).stack.offset(a as
                                                                       isize)).value.i
                                            = 1i32 as mrb_int
                                    } else {
                                        (*(*(*mrb).c).stack.offset(a as
                                                                       isize)).tt
                                            = MRB_TT_FALSE;
                                        (*(*(*mrb).c).stack.offset(a as
                                                                       isize)).value.i
                                            = 1i32 as mrb_int
                                    }
                                    current_block = 7149356873433890176;
                                    continue ;
                                }
                                18322908395017252093 => {
                                    if 0 != result {
                                        (*(*(*mrb).c).stack.offset(a as
                                                                       isize)).tt
                                            = MRB_TT_TRUE;
                                        (*(*(*mrb).c).stack.offset(a as
                                                                       isize)).value.i
                                            = 1i32 as mrb_int
                                    } else {
                                        (*(*(*mrb).c).stack.offset(a as
                                                                       isize)).tt
                                            = MRB_TT_FALSE;
                                        (*(*(*mrb).c).stack.offset(a as
                                                                       isize)).value.i
                                            = 1i32 as mrb_int
                                    }
                                    current_block = 7149356873433890176;
                                    continue ;
                                }
                                15715503430877517545 => {
                                    if 0 != result_0 {
                                        (*(*(*mrb).c).stack.offset(a as
                                                                       isize)).tt
                                            = MRB_TT_TRUE;
                                        (*(*(*mrb).c).stack.offset(a as
                                                                       isize)).value.i
                                            = 1i32 as mrb_int
                                    } else {
                                        (*(*(*mrb).c).stack.offset(a as
                                                                       isize)).tt
                                            = MRB_TT_FALSE;
                                        (*(*(*mrb).c).stack.offset(a as
                                                                       isize)).value.i
                                            = 1i32 as mrb_int
                                    }
                                    current_block = 7149356873433890176;
                                    continue ;
                                }
                                8728320426208223650 => {
                                    if 0 != result_1 {
                                        (*(*(*mrb).c).stack.offset(a as
                                                                       isize)).tt
                                            = MRB_TT_TRUE;
                                        (*(*(*mrb).c).stack.offset(a as
                                                                       isize)).value.i
                                            = 1i32 as mrb_int
                                    } else {
                                        (*(*(*mrb).c).stack.offset(a as
                                                                       isize)).tt
                                            = MRB_TT_FALSE;
                                        (*(*(*mrb).c).stack.offset(a as
                                                                       isize)).value.i
                                            = 1i32 as mrb_int
                                    }
                                    current_block = 7149356873433890176;
                                    continue ;
                                }
                                8359968701383644501 => {
                                    if 0 != result_2 {
                                        (*(*(*mrb).c).stack.offset(a as
                                                                       isize)).tt
                                            = MRB_TT_TRUE;
                                        (*(*(*mrb).c).stack.offset(a as
                                                                       isize)).value.i
                                            = 1i32 as mrb_int
                                    } else {
                                        (*(*(*mrb).c).stack.offset(a as
                                                                       isize)).tt
                                            = MRB_TT_FALSE;
                                        (*(*(*mrb).c).stack.offset(a as
                                                                       isize)).value.i
                                            = 1i32 as mrb_int
                                    }
                                    current_block = 7149356873433890176;
                                    continue ;
                                }
                                _ => { }
                            }
                            let mut argc: libc::c_int =
                                if c as libc::c_int == 127i32 {
                                    -1i32
                                } else { c as libc::c_int };
                            let mut bidx_1: libc::c_int =
                                (if argc < 0i32 {
                                     a.wrapping_add(2i32 as libc::c_uint)
                                 } else {
                                     a.wrapping_add(c as
                                                        libc::c_uint).wrapping_add(1i32
                                                                                       as
                                                                                       libc::c_uint)
                                 }) as libc::c_int;
                            let mut m: mrb_method_t =
                                mrb_method_t{func_p: 0,
                                             c2rust_unnamed:
                                                 C2RustUnnamed{proc_0:
                                                                   0 as
                                                                       *mut RProc,},};
                            let mut cls: *mut RClass = 0 as *mut RClass;
                            let mut ci_0: *mut mrb_callinfo = (*(*mrb).c).ci;
                            let mut recv: mrb_value =
                                mrb_value{value: C2RustUnnamed_0{f: 0.,},
                                          tt: MRB_TT_FALSE,};
                            let mut blk: mrb_value =
                                mrb_value{value: C2RustUnnamed_0{f: 0.,},
                                          tt: MRB_TT_FALSE,};
                            recv = *(*(*mrb).c).stack.offset(a as isize);
                            blk = *(*(*mrb).c).stack.offset(bidx_1 as isize);
                            if !(blk.tt as libc::c_uint ==
                                     MRB_TT_FALSE as libc::c_int as
                                         libc::c_uint && 0 == blk.value.i) &&
                                   blk.tt as libc::c_uint !=
                                       MRB_TT_PROC as libc::c_int as
                                           libc::c_uint {
                                blk =
                                    mrb_convert_type(mrb, blk, MRB_TT_PROC,
                                                     b"Proc\x00" as *const u8
                                                         as
                                                         *const libc::c_char,
                                                     b"to_proc\x00" as
                                                         *const u8 as
                                                         *const libc::c_char);
                                /* The stack might have been reallocated during mrb_convert_type(),
           see #3622 */
                                *(*(*mrb).c).stack.offset(bidx_1 as isize) =
                                    blk
                            }
                            cls = mrb_class(mrb, recv);
                            m = mrb_method_search_vm(mrb, &mut cls, mid);
                            if m.c2rust_unnamed.proc_0.is_null() {
                                let mut missing: mrb_sym =
                                    mrb_intern_static(mrb,
                                                      b"method_missing\x00" as
                                                          *const u8 as
                                                          *const libc::c_char,
                                                      (::std::mem::size_of::<[libc::c_char; 15]>()
                                                           as
                                                           libc::c_ulong).wrapping_sub(1i32
                                                                                           as
                                                                                           libc::c_ulong));
                                m =
                                    mrb_method_search_vm(mrb, &mut cls,
                                                         missing);
                                if m.c2rust_unnamed.proc_0.is_null() ||
                                       missing == (*(*(*mrb).c).ci).mid &&
                                           0 !=
                                               mrb_obj_eq(mrb,
                                                          *(*(*mrb).c).stack.offset(0isize),
                                                          recv) as libc::c_int
                                   {
                                    let mut args: mrb_value =
                                        if argc < 0i32 {
                                            *(*(*mrb).c).stack.offset(a.wrapping_add(1i32
                                                                                         as
                                                                                         libc::c_uint)
                                                                          as
                                                                          isize)
                                        } else {
                                            mrb_ary_new_from_values(mrb,
                                                                    c as
                                                                        mrb_int,
                                                                    (*(*mrb).c).stack.offset(a
                                                                                                 as
                                                                                                 isize).offset(1isize))
                                        };
                                    (*(*(*mrb).c).ci).err = pc0;
                                    mrb_method_missing(mrb, mid, recv, args);
                                }
                                if argc >= 0i32 {
                                    if a.wrapping_add(2i32 as libc::c_uint) >=
                                           (*irep).nregs as libc::c_uint {
                                        mrb_stack_extend(mrb,
                                                         a.wrapping_add(3i32
                                                                            as
                                                                            libc::c_uint)
                                                             as mrb_int);
                                    }
                                    *(*(*mrb).c).stack.offset(a.wrapping_add(1i32
                                                                                 as
                                                                                 libc::c_uint)
                                                                  as isize) =
                                        mrb_ary_new_from_values(mrb,
                                                                c as mrb_int,
                                                                (*(*mrb).c).stack.offset(a
                                                                                             as
                                                                                             isize).offset(1isize));
                                    *(*(*mrb).c).stack.offset(a.wrapping_add(2i32
                                                                                 as
                                                                                 libc::c_uint)
                                                                  as isize) =
                                        blk;
                                    argc = -1i32
                                }
                                mrb_ary_unshift(mrb,
                                                *(*(*mrb).c).stack.offset(a.wrapping_add(1i32
                                                                                             as
                                                                                             libc::c_uint)
                                                                              as
                                                                              isize),
                                                mrb_symbol_value(mid));
                                mid = missing
                            }
                            /* push callinfo */
                            ci_0 = cipush(mrb);
                            (*ci_0).mid = mid;
                            (*ci_0).stackent = (*(*mrb).c).stack;
                            (*ci_0).target_class = cls;
                            (*ci_0).argc = argc;
                            (*ci_0).pc = pc;
                            (*ci_0).acc = a as libc::c_int;
                            /* prepare stack */
                            (*(*mrb).c).stack =
                                (*(*mrb).c).stack.offset(a as isize);
                            if 0 !=
                                   if 0 != m.func_p as libc::c_int {
                                       1i32
                                   } else if !m.c2rust_unnamed.proc_0.is_null()
                                    {
                                       ((*m.c2rust_unnamed.proc_0).flags() as
                                            libc::c_int & 128i32 != 0i32) as
                                           libc::c_int
                                   } else { 0i32 } {
                                if 0 == m.func_p {
                                    let mut p_0: *mut RProc =
                                        m.c2rust_unnamed.proc_0;
                                    (*ci_0).proc_0 = p_0;
                                    recv =
                                        (*p_0).body.func.expect("non-null function pointer")(mrb,
                                                                                             recv)
                                } else {
                                    recv =
                                        m.c2rust_unnamed.func.expect("non-null function pointer")(mrb,
                                                                                                  recv)
                                }
                                mrb_gc_arena_restore(mrb, ai);
                                mrb_gc_arena_shrink(mrb, ai);
                                if !(*mrb).exc.is_null() {
                                    current_block = 2687857153341325290;
                                    continue ;
                                }
                                ci_0 = (*(*mrb).c).ci;
                                if blk.tt as libc::c_uint ==
                                       MRB_TT_PROC as libc::c_int as
                                           libc::c_uint {
                                    let mut p_1: *mut RProc =
                                        blk.value.p as *mut RProc;
                                    if !p_1.is_null() &&
                                           !((*p_1).flags() as libc::c_int &
                                                 256i32 != 0i32) &&
                                           (if (*p_1).flags() as libc::c_int &
                                                   1024i32 != 0i32 {
                                                (*p_1).e.env
                                            } else { 0 as *mut REnv }) ==
                                               (*ci_0.offset(-1i32 as
                                                                 isize)).env {
                                        (*p_1).set_flags((*p_1).flags() |
                                                             512i32 as
                                                                 uint32_t)
                                    }
                                }
                                if (*ci_0).target_class.is_null() {
                                    /* return from context modifying method (resume/yield) */
                                    if (*ci_0).acc == -3i32 {
                                        (*mrb).jmp = prev_jmp;
                                        return recv
                                    } else {
                                        proc_0 =
                                            (*ci_0.offset(-1i32 as
                                                              isize)).proc_0;
                                        irep = (*proc_0).body.irep;
                                        pool = (*irep).pool;
                                        syms = (*irep).syms
                                    }
                                }
                                *(*(*mrb).c).stack.offset(0isize) = recv;
                                /* pop stackpos */
                                (*(*mrb).c).stack = (*ci_0).stackent;
                                pc = (*ci_0).pc;
                                cipop(mrb);
                                current_block = 7149356873433890176;
                                continue ;
                            } else {
                                /* setup environment for calling method */
                                (*ci_0).proc_0 = m.c2rust_unnamed.proc_0;
                                proc_0 = (*ci_0).proc_0;
                                irep = (*proc_0).body.irep;
                                pool = (*irep).pool;
                                syms = (*irep).syms;
                                mrb_stack_extend(mrb,
                                                 (if argc < 0i32 &&
                                                         ((*irep).nregs as
                                                              libc::c_int) <
                                                             3i32 {
                                                      3i32
                                                  } else {
                                                      (*irep).nregs as
                                                          libc::c_int
                                                  }) as mrb_int);
                                pc = (*irep).iseq;
                                current_block = 7149356873433890176;
                                continue ;
                            }
                        }
                    }
                }
            }
            match current_block {
                3814937494483549324 => {
                    exc_9 =
                        mrb_exc_new_str(mrb,
                                        mrb_exc_get(mrb,
                                                    b"LocalJumpError\x00" as
                                                        *const u8 as
                                                        *const libc::c_char),
                                        mrb_str_new_static(mrb,
                                                           b"break from proc-closure\x00"
                                                               as *const u8 as
                                                               *const libc::c_char,
                                                           (::std::mem::size_of::<[libc::c_char; 24]>()
                                                                as
                                                                libc::c_ulong).wrapping_sub(1i32
                                                                                                as
                                                                                                libc::c_ulong)));
                    mrb_exc_set(mrb, exc_9);
                    current_block = 2687857153341325290;
                }
                _ => {
                    while ci_3 < (*(*mrb).c).ci { cipop(mrb); }
                    (*ci_3.offset(0isize)).ridx =
                        (*ci_3.offset(-1i32 as isize)).ridx;
                    while (*(*mrb).c).eidx as libc::c_int >
                              (*ci_3).epos as libc::c_int {
                        let mut cioff_2: ptrdiff_t =
                            ci_3.wrapping_offset_from((*(*mrb).c).cibase) as
                                libc::c_long;
                        ecall(mrb);
                        ci_3 = (*(*mrb).c).cibase.offset(cioff_2 as isize)
                    }
                    if 0 != (*(*mrb).c).vmexec as libc::c_int &&
                           (*ci_3).target_class.is_null() {
                        mrb_gc_arena_restore(mrb, ai);
                        (*(*mrb).c).vmexec = 0i32 as mrb_bool;
                        (*mrb).jmp = prev_jmp;
                        return v_0
                    }
                    acc = (*ci_3).acc;
                    (*(*mrb).c).stack = (*ci_3).stackent;
                    cipop(mrb);
                    if acc == -1i32 || acc == -2i32 {
                        mrb_gc_arena_restore(mrb, ai);
                        (*mrb).jmp = prev_jmp;
                        return v_0
                    }
                    pc = (*ci_3).pc;
                    ci_3 = (*(*mrb).c).ci;
                    proc_0 = (*(*(*mrb).c).ci).proc_0;
                    irep = (*proc_0).body.irep;
                    pool = (*irep).pool;
                    syms = (*irep).syms;
                    *(*(*mrb).c).stack.offset(acc as isize) = v_0;
                    mrb_gc_arena_restore(mrb, ai);
                    current_block = 7149356873433890176;
                }
            }
        }
    match current_block {
        10071922767614739690 => {
            let fresh270 = pc;
            pc = pc.offset(1);
            a = *fresh270 as uint32_t;
            let fresh271 = pc;
            pc = pc.offset(1);
            b = *fresh271 as uint16_t;
            let fresh272 = pc;
            pc = pc.offset(1);
            c = *fresh272;
            abort();
        }
        5128590269704079499 => { }
        _ => { }
    }
    /*        stop VM */
    while (*(*mrb).c).eidx as libc::c_int > 0i32 { ecall(mrb); }
    (*(*(*mrb).c).cibase).ridx = 0i32 as uint16_t;
    (*(*(*mrb).c).ci).err = 0 as *mut mrb_code;
    (*mrb).jmp = prev_jmp;
    if !(*mrb).exc.is_null() {
        return mrb_obj_value((*mrb).exc as *mut libc::c_void)
    }
    return *(*(*mrb).c).stack.offset((*irep).nlocals as isize);
}
#[no_mangle]
pub unsafe extern "C" fn mrb_run(mut mrb: *mut mrb_state,
                                 mut proc_0: *mut RProc,
                                 mut self_0: mrb_value) -> mrb_value {
    if (*(*(*mrb).c).ci).argc < 0i32 {
        /* receiver, args and block) */
        return mrb_vm_run(mrb, proc_0, self_0, 3i32 as libc::c_uint)
    } else {
        return mrb_vm_run(mrb, proc_0, self_0,
                          ((*(*(*mrb).c).ci).argc + 2i32) as libc::c_uint)
    };
}
#[no_mangle]
pub unsafe extern "C" fn mrb_top_run(mut mrb: *mut mrb_state,
                                     mut proc_0: *mut RProc,
                                     mut self_0: mrb_value,
                                     mut stack_keep: libc::c_uint)
 -> mrb_value {
    let mut ci: *mut mrb_callinfo = 0 as *mut mrb_callinfo;
    let mut v: mrb_value =
        mrb_value{value: C2RustUnnamed_0{f: 0.,}, tt: MRB_TT_FALSE,};
    if (*(*mrb).c).cibase.is_null() {
        return mrb_vm_run(mrb, proc_0, self_0, stack_keep)
    }
    if (*(*mrb).c).ci == (*(*mrb).c).cibase {
        (*(*(*mrb).c).ci).env = 0 as *mut REnv;
        return mrb_vm_run(mrb, proc_0, self_0, stack_keep)
    }
    ci = cipush(mrb);
    (*ci).stackent = (*(*mrb).c).stack;
    (*ci).mid = 0i32 as mrb_sym;
    (*ci).acc = -1i32;
    (*ci).target_class = (*mrb).object_class;
    v = mrb_vm_run(mrb, proc_0, self_0, stack_keep);
    return v;
}