// ext derived from mrusty @ 1.0.0
// <https://github.com/anima-engine/mrusty/tree/v1.0.0>

// mrusty. mruby safe bindings for Rust
// Copyright (C) 2016  Dragoș Tiselice
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

#include <mruby-sys/ext.h>

mrb_int mrb_sys_fixnum_to_cint(mrb_value value) { return mrb_fixnum(value); }

mrb_float mrb_sys_float_to_cdouble(mrb_value value) { return mrb_float(value); }

_Bool mrb_sys_value_is_nil(mrb_value value) {
  mrb_value test = mrb_nil_value();
  return mrb_type(value) == mrb_type(test) && value.value.i == test.value.i;
}

_Bool mrb_sys_value_is_false(mrb_value value) {
  mrb_value test = mrb_false_value();
  return mrb_type(value) == mrb_type(test) && value.value.i == test.value.i;
}

_Bool mrb_sys_value_is_true(mrb_value value) {
  mrb_value test = mrb_true_value();
  return mrb_type(value) == mrb_type(test) && value.value.i == test.value.i;
}

struct RClass *mrb_sys_class_to_rclass(mrb_value value) {
  return (struct RClass *)value.value.p;
}

mrb_value mrb_sys_nil_value(void) {
  // `mrb_nil_value` is defined as `MRB_INLINE` which means bindgen won't find
  // it, so we wrap it here so we can link to it.
  return mrb_nil_value();
}

mrb_value mrb_sys_false_value(void) {
  // `mrb_false_value` is defined as `MRB_INLINE` which means bindgen won't
  // find it, so we wrap it here so we can link to it.
  return mrb_false_value();
}

mrb_value mrb_sys_true_value(void) {
  // `mrb_true_value` is defined as `MRB_INLINE` which means bindgen won't find
  // it, so we wrap it here so we can link to it.
  return mrb_true_value();
}

mrb_value mrb_sys_fixnum_value(mrb_int value) {
  // `mrb_fixnum_value` is defined as `MRB_INLINE` which means bindgen won't
  // find it, so we wrap it here so we can link to it.
  return mrb_fixnum_value(value);
}

mrb_value mrb_sys_float_value(struct mrb_state *mrb, mrb_float value) {
  // `mrb_float_value` is defined as `MRB_INLINE` which means bindgen won't
  // find it, so we wrap it here so we can link to it.
  return mrb_float_value(mrb, value);
}

mrb_value mrb_sys_proc_value(struct mrb_state *mrb, struct RProc *proc) {
  mrb_value value = mrb_cptr_value(mrb, proc);

  value.tt = MRB_TT_PROC;

  return value;
}

mrb_value mrb_sys_class_value(struct RClass *klass) {
  mrb_value value;

  value.value.p = klass;
  value.tt = MRB_TT_CLASS;

  return value;
}

mrb_value mrb_sys_module_value(struct RClass *module) {
  mrb_value value;

  value.value.p = module;
  value.tt = MRB_TT_MODULE;

  return value;
}

mrb_value mrb_sys_data_value(struct RData *data) {
  mrb_value value;

  value.value.p = data;
  value.tt = MRB_TT_DATA;

  return value;
}

const char *mrb_sys_symbol_name(struct mrb_state *mrb, mrb_value value) {
  return mrb_sym2name(mrb, mrb_symbol(value));
}

mrb_value mrb_sys_new_symbol(struct mrb_state *mrb, const char *string,
                             size_t len) {
  mrb_value value;

  mrb_symbol(value) = mrb_intern(mrb, string, len);

  value.tt = MRB_TT_SYMBOL;

  return value;
}

void mrb_sys_data_init(mrb_value *value, void *ptr, const mrb_data_type *type) {
  // `mrb_data_init` is defined as `inline` which means bindgen won't find it,
  // so we wrap it here so we can link to it.
  mrb_data_init(*value, ptr, type);
}

mrb_value mrb_sys_get_current_exception(struct mrb_state *mrb) {
  if (!mrb->exc) {
    return mrb_nil_value();
  }

  // exception_info = exc.inspect
  // backtrace = exc.backtrace
  // backtrace.unshift(exception_info)
  // backtrace.join("\n")
  mrb_value exc = mrb_funcall(mrb, mrb_obj_value(mrb->exc), "inspect", 0);
  mrb_value backtrace = mrb_exc_backtrace(mrb, mrb_obj_value(mrb->exc));

  mrb_funcall(mrb, backtrace, "unshift", 1, exc);

  // clear exception from interpreter
  mrb->exc = NULL;

  return mrb_funcall(mrb, backtrace, "join", 1, mrb_str_new_cstr(mrb, "\n"));
}

void mrb_sys_raise_current_exception(struct mrb_state *mrb) {
  if (mrb->exc) {
    mrb_exc_raise(mrb, mrb_obj_value(mrb->exc));
  }
}

// TODO: implement this debug function in Rust
mrb_value mrb_sys_value_debug_str(struct mrb_state *mrb, mrb_value value) {
  return mrb_funcall(mrb, value, "inspect", 0);
}

mrb_noreturn void mrb_sys_raise(struct mrb_state *mrb, const char *eclass,
                                const char *msg) {
  mrb_raise(mrb, mrb_class_get(mrb, eclass), msg);
}

// TODO: implement this utility function in Rust
mrb_bool mrb_sys_class_defined_under(struct mrb_state *mrb,
                                     struct RClass *outer, const char *name) {
  mrb_value sym = mrb_check_intern_cstr(mrb, name);

  if (mrb_nil_p(sym))
    return FALSE;

  return mrb_const_defined(mrb, mrb_obj_value(outer), mrb_symbol(sym));
}

struct RClass *mrb_sys_class_of_value(struct mrb_state *mrb, mrb_value value) {
  // `mrb_class` is defined as `inline` which means bindgen won't find it, so
  // we wrap it here so we can link to it.
  return mrb_class(mrb, value);
}

mrb_int mrb_sys_ary_len(mrb_value value) {
  // `ARY_LEN` is defined as a macro which means bindgen won't find it, so we
  // wrap it here so we can link to it.
  return ARY_LEN(mrb_ary_ptr(value));
}
