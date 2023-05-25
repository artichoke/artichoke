#include <mrbsys/ext.h>

#include <mruby/array.h>
#include <mruby/numeric.h>
#include <mruby/presym.h>
#include <mruby/range.h>
#include <mruby/string.h>
#include <mruby/internal.h>

#ifdef __cplusplus
extern "C" {
#endif

const uint8_t mrblib_irep[] = {0};

const char mrb_digitmap[] = "0123456789abcdefghijklmnopqrstuvwxyz";

// VM method table twiddling

MRB_API bool
mrb_sys_value_has_method(mrb_state *mrb, mrb_value value, mrb_sym method)
{
  struct RClass *class_pointer = mrb_sys_class_of_value(mrb, value);
  mrb_method_t m = mrb_method_search_vm(mrb, &class_pointer, method);
  return !MRB_METHOD_UNDEF_P(m);
}

// Check whether `mrb_value` is nil, false, or true

MRB_API bool
mrb_sys_value_is_nil(mrb_value value)
{
  return mrb_nil_p(value);
}

MRB_API bool
mrb_sys_value_is_false(mrb_value value)
{
  return mrb_false_p(value);
}

MRB_API bool
mrb_sys_value_is_true(mrb_value value)
{
  return mrb_true_p(value);
}

MRB_API bool
mrb_sys_range_excl(mrb_state *mrb, mrb_value value)
{
  return mrb_range_excl_p(mrb, value);
}

MRB_API bool
mrb_sys_obj_frozen(mrb_state *mrb, mrb_value value)
{
  (void)(mrb);
  return mrb_immediate_p(value) || MRB_FROZEN_P(mrb_basic_ptr(value));
}

// Extract pointers from `mrb_value`s

MRB_API mrb_int
mrb_sys_fixnum_to_cint(mrb_value value)
{
  return mrb_fixnum(value);
}

MRB_API mrb_float
mrb_sys_float_to_cdouble(mrb_value value)
{
  return mrb_float(value);
}

MRB_API void *
mrb_sys_cptr_ptr(mrb_value value)
{
  return mrb_cptr(value);
}

MRB_API struct RBasic *
mrb_sys_basic_ptr(mrb_value value)
{
  return mrb_basic_ptr(value);
}

MRB_API struct RObject *
mrb_sys_obj_ptr(mrb_value value)
{
  return mrb_obj_ptr(value);
}

MRB_API struct RProc *
mrb_sys_proc_ptr(mrb_value value)
{
  return mrb_proc_ptr(value);
}

MRB_API struct RClass *
mrb_sys_class_ptr(mrb_value value)
{
  return mrb_class_ptr(value);
}

MRB_API struct RClass *
mrb_sys_class_to_rclass(mrb_value value)
{
  return mrb_class_ptr(value);
}

MRB_API struct RClass *
mrb_sys_class_of_value(struct mrb_state *mrb, mrb_value value)
{
  return mrb_class(mrb, value);
}

// Construct `mrb_value`s

MRB_API mrb_value
mrb_sys_nil_value(void)
{
  return mrb_nil_value();
}

MRB_API mrb_value
mrb_sys_false_value(void)
{
  return mrb_false_value();
}

MRB_API mrb_value
mrb_sys_true_value(void)
{
  return mrb_true_value();
}

MRB_API mrb_value
mrb_sys_fixnum_value(mrb_int value)
{
  return mrb_fixnum_value(value);
}

MRB_API mrb_value
mrb_sys_float_value(struct mrb_state *mrb, mrb_float value)
{
  return mrb_float_value(mrb, value);
}

MRB_API mrb_value
mrb_sys_cptr_value(struct mrb_state *mrb, void *ptr)
{
  mrb_value value;
  (void)(mrb);

  SET_CPTR_VALUE(mrb, value, ptr);

  return value;
}

MRB_API mrb_value
mrb_sys_obj_value(void *p)
{
  return mrb_obj_value(p);
}

MRB_API mrb_value
mrb_sys_class_value(struct RClass *klass)
{
  mrb_value value;

  value.value.p = klass;
  value.tt = MRB_TT_CLASS;

  return value;
}

MRB_API mrb_value
mrb_sys_module_value(struct RClass *module)
{
  mrb_value value;

  value.value.p = module;
  value.tt = MRB_TT_MODULE;

  return value;
}

MRB_API mrb_value
mrb_sys_data_value(struct RData *data)
{
  mrb_value value;

  value.value.p = data;
  value.tt = MRB_TT_DATA;

  return value;
}

MRB_API mrb_value
mrb_sys_proc_value(struct mrb_state *mrb, struct RProc *proc)
{
  mrb_value value = mrb_cptr_value(mrb, proc);

  value.tt = MRB_TT_PROC;

  return value;
}

// Manipulate `Symbol`s

MRB_API mrb_value
mrb_sys_new_symbol(mrb_sym id)
{
  mrb_value value;
  mrb_symbol(value) = id;
  value.tt = MRB_TT_SYMBOL;

  return value;
}

// Manage Rust-backed `mrb_value`s

MRB_API void
mrb_sys_set_instance_tt(struct RClass *klass, enum mrb_vtype type)
{
  MRB_SET_INSTANCE_TT(klass, type);
}

MRB_API void
mrb_sys_data_init(mrb_value *value, void *ptr, const mrb_data_type *type)
{
  mrb_data_init(*value, ptr, type);
}

// Raise exceptions and debug info

MRB_API mrb_noreturn void
mrb_sys_raise(struct mrb_state *mrb, const char *eklass, const char *msg)
{
  mrb_raise(mrb, mrb_class_get(mrb, eklass), msg);
}

MRB_API void
mrb_sys_raise_current_exception(struct mrb_state *mrb)
{
  if (mrb->exc) {
    mrb_exc_raise(mrb, mrb_obj_value(mrb->exc));
  }
}

// Manipulate Array `mrb_value`s

MRB_API mrb_value
mrb_sys_alloc_rarray(struct mrb_state *mrb, mrb_value *ptr, mrb_int len, mrb_int capa)
{
  struct RArray *a = NULL;

  a = (struct RArray *)mrb_obj_alloc(mrb, MRB_TT_ARRAY, mrb->array_class);

  a->as.heap.ptr = ptr;
  a->as.heap.len = len;
  a->as.heap.aux.capa = capa;

  return mrb_obj_value(a);
}

MRB_API void
mrb_sys_repack_into_rarray(mrb_value *ptr, mrb_int len, mrb_int capa, mrb_value into)
{
  struct RArray *a = RARRAY(into);

  a->as.heap.ptr = ptr;
  a->as.heap.len = len;
  a->as.heap.aux.capa = capa;
}

MRB_API mrb_value
mrb_ary_entry(mrb_value ary, mrb_int offset)
{
  if (offset < 0) {
    offset += RARRAY_LEN(ary);
  }
  if (offset < 0 || RARRAY_LEN(ary) <= offset) {
    return mrb_nil_value();
  }
  return RARRAY_PTR(ary)[offset];
}

static void
ary_modify_check(mrb_state *mrb, struct RArray *a)
{
  mrb_check_frozen(mrb, a);
}

static void
ary_modify(mrb_state *mrb, struct RArray *a)
{
  ary_modify_check(mrb, a);
}

MRB_API void
mrb_ary_modify(mrb_state *mrb, struct RArray *a)
{
  mrb_write_barrier(mrb, (struct RBasic *)a);
  ary_modify(mrb, a);
}

mrb_value
mrb_ary_subseq(mrb_state *mrb, mrb_value ary, mrb_int beg, mrb_int len)
{
  struct RArray *a = mrb_ary_ptr(ary);
  return mrb_ary_new_from_values(mrb, len, ARY_PTR(a) + beg);
}

// Manipulate String `mrb_value`s

MRB_API mrb_value
mrb_sys_alloc_rstring(struct mrb_state *mrb, char *ptr, mrb_int len, mrb_int capa)
{
  struct RString *s = NULL;

  s = (struct RString *)mrb_obj_alloc(mrb, MRB_TT_STRING, mrb->string_class);

  s->as.heap.ptr = ptr;
  s->as.heap.len = len;
  s->as.heap.aux.capa = capa;

  return mrb_obj_value(s);
}

MRB_API struct RString *
mrb_sys_repack_into_rstring(char *ptr, mrb_int len, mrb_int capa, mrb_value into)
{
  struct RString *s = RSTRING(into);

  s->as.heap.ptr = ptr;
  s->as.heap.len = len;
  s->as.heap.aux.capa = capa;

  return s;
}

MRB_API void
mrb_str_modify_keep_ascii(mrb_state *mrb, struct RString *s)
{
  mrb_check_frozen(mrb, s);
}

MRB_API void
mrb_str_modify(mrb_state *mrb, struct RString *s)
{
  mrb_str_modify_keep_ascii(mrb, s);
}

MRB_API void
mrb_str_concat(mrb_state *mrb, mrb_value self, mrb_value other)
{
  other = mrb_obj_as_string(mrb, other);
  mrb_str_cat_str(mrb, self, other);
}

MRB_API mrb_value
mrb_str_intern(mrb_state *mrb, mrb_value self)
{
  return mrb_symbol_value(mrb_intern_str(mrb, self));
}

MRB_API mrb_value
mrb_obj_as_string(mrb_state *mrb, mrb_value obj)
{
  switch (mrb_type(obj)) {
    case MRB_TT_STRING:
      return obj;
    case MRB_TT_SYMBOL:
      return mrb_sym_str(mrb, mrb_symbol(obj));
    case MRB_TT_INTEGER:
      // NOLINTNEXTLINE(cppcoreguidelines-avoid-magic-numbers, readability-magic-numbers): base 10
      return mrb_integer_to_str(mrb, obj, 10);
    case MRB_TT_SCLASS:
    case MRB_TT_CLASS:
    case MRB_TT_MODULE:
      return mrb_mod_to_s(mrb, obj);
    default:
      return mrb_type_convert(mrb, obj, MRB_TT_STRING, MRB_SYM(to_s));
  }
}

MRB_API mrb_value
mrb_str_cat_cstr(mrb_state *mrb, mrb_value str, const char *ptr)
{
  return mrb_str_cat(mrb, str, ptr, ptr ? strlen(ptr) : 0);
}

MRB_API mrb_value
mrb_str_cat_str(mrb_state *mrb, mrb_value str, mrb_value str2)
{
  if (mrb_str_ptr(str) == mrb_str_ptr(str2)) {
    mrb_str_modify(mrb, mrb_str_ptr(str));
  }
  return mrb_str_cat(mrb, str, RSTRING_PTR(str2), RSTRING_LEN(str2));
}

MRB_API mrb_value
mrb_str_append(mrb_state *mrb, mrb_value str1, mrb_value str2)
{
  mrb_to_str(mrb, str2);
  return mrb_str_cat_str(mrb, str1, str2);
}

// Manage the mruby garbage collector (GC)

MRB_API int
mrb_sys_gc_arena_save(mrb_state *mrb)
{
  return mrb_gc_arena_save(mrb);
}

MRB_API void
mrb_sys_gc_arena_restore(mrb_state *mrb, int arena_index)
{
  mrb_gc_arena_restore(mrb, arena_index);
}

MRB_API bool
mrb_sys_gc_disable(mrb_state *mrb)
{
  mrb_gc *gc = &mrb->gc;
  bool was_enabled = !gc->disabled;
  gc->disabled = 1;
  return was_enabled;
}

MRB_API bool
mrb_sys_gc_enable(mrb_state *mrb)
{
  mrb_gc *gc = &mrb->gc;
  bool was_enabled = !gc->disabled;
  gc->disabled = 0;
  return was_enabled;
}

MRB_API bool
mrb_sys_value_is_dead(mrb_state *mrb, mrb_value value)
{
  // immediate values such as Fixnums and Symbols are never garbage
  // collected, so they are never dead. See `mrb_gc_protect` in gc.c.
  if (mrb_immediate_p(value)) {
    return false;
  }

  struct RBasic *ptr = mrb_basic_ptr(value);

  if (ptr == NULL) {
    return true;
  }

  return mrb_object_dead_p(mrb, ptr);
}

MRB_API size_t
mrb_sys_gc_live_objects(mrb_state *mrb)
{
  mrb_gc *gc = &mrb->gc;
  return gc->live;
}

MRB_API void
mrb_sys_safe_gc_mark(mrb_state *mrb, mrb_value value)
{
  if (!mrb_immediate_p(value)) {
    mrb_gc_mark(mrb, mrb_basic_ptr(value));
  }
}

#ifdef __cplusplus
} /* extern "C" { */
#endif
