// ext is partially derived from mrusty @ 1.0.0
// <https://github.com/anima-engine/mrusty/tree/v1.0.0>
//
// Copyright (C) 2016  Dragoș Tiselice
// Licensed under the Mozilla Public License 2.0

// ext is partially derived from go-mruby @ cd6a04a
// <https://github.com/mitchellh/go-mruby/tree/cd6a04a>
//
// Copyright (c) 2017 Mitchell Hashimoto
// Licensed under the MIT License

// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

#include <mruby-sys/ext.h>

// Check whether `mrb_value` is nil, false, or true

_Bool mrb_sys_value_is_nil(mrb_value value) { return mrb_nil_p(value); }

_Bool mrb_sys_value_is_false(mrb_value value) { return mrb_false_p(value); }

_Bool mrb_sys_value_is_true(mrb_value value) { return mrb_true_p(value); }

// Extract pointers from `mrb_value`s

mrb_int mrb_sys_fixnum_to_cint(mrb_value value) { return mrb_fixnum(value); }

mrb_float mrb_sys_float_to_cdouble(mrb_value value) { return mrb_float(value); }

struct RBasic *mrb_sys_basic_ptr(mrb_value value) {
  return mrb_basic_ptr(value);
}

struct RObject *mrb_sys_obj_ptr(mrb_value value) {
  return mrb_obj_ptr(value);
}

struct RProc *mrb_sys_proc_ptr(mrb_value value) {
  return mrb_proc_ptr(value);
}

struct RClass *mrb_sys_class_ptr(mrb_value value) {
  return mrb_class_ptr(value);
}

struct RClass *mrb_sys_class_to_rclass(mrb_value value) {
  return (struct RClass *)value.value.p;
}

struct RClass *mrb_sys_class_of_value(struct mrb_state *mrb, mrb_value value) {
  return mrb_class(mrb, value);
}

// Construct `mrb_value`s

mrb_value mrb_sys_nil_value(void) { return mrb_nil_value(); }

mrb_value mrb_sys_false_value(void) { return mrb_false_value(); }

mrb_value mrb_sys_true_value(void) { return mrb_true_value(); }

mrb_value mrb_sys_fixnum_value(mrb_int value) {
  return mrb_fixnum_value(value);
}

mrb_value mrb_sys_float_value(struct mrb_state *mrb, mrb_float value) {
  return mrb_float_value(mrb, value);
}

mrb_value mrb_sys_obj_value(void *p) { return mrb_obj_value(p); }

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

mrb_value mrb_sys_proc_value(struct mrb_state *mrb, struct RProc *proc) {
  mrb_value value = mrb_cptr_value(mrb, proc);

  value.tt = MRB_TT_PROC;

  return value;
}

// Manipulate `Symbol`s

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

// Manage Rust-backed `mrb_value`s

void mrb_sys_set_instance_tt(struct RClass *class, enum mrb_vtype type) {
  MRB_SET_INSTANCE_TT(class, type);
}

void mrb_sys_data_init(mrb_value *value, void *ptr, const mrb_data_type *type) {
  mrb_data_init(*value, ptr, type);
}

// Raise exceptions and debug info

mrb_noreturn void mrb_sys_raise(struct mrb_state *mrb, const char *eclass,
                                const char *msg) {
  mrb_raise(mrb, mrb_class_get(mrb, eclass), msg);
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

// Manipulate Array `mrb_value`s

mrb_int mrb_sys_ary_len(mrb_value value) { return ARY_LEN(mrb_ary_ptr(value)); }

// Manage the mruby garbage collector (GC)

int mrb_sys_gc_arena_save(mrb_state *mrb) { return mrb_gc_arena_save(mrb); }

void mrb_sys_gc_arena_restore(mrb_state *mrb, int arena_index) {
  mrb_gc_arena_restore(mrb, arena_index);
}

void mrb_sys_gc_disable(mrb_state *mrb) {
  mrb_gc *gc = &mrb->gc;
  gc->disabled = 1;
}

void mrb_sys_gc_enable(mrb_state *mrb) {
  mrb_gc *gc = &mrb->gc;
  gc->disabled = 0;
}

_Bool mrb_sys_value_is_dead(mrb_state *mrb, mrb_value value) {
  // immediate values such as Fixnums and Symbols are never garbage
  // collected, so they are never dead. See `mrb_gc_protect` in gc.c.
  if (mrb_immediate_p(value)) {
    return FALSE;
  }

  struct RBasic *ptr = mrb_basic_ptr(value);

  if (ptr == NULL) {
    return TRUE;
  }

  return mrb_object_dead_p(mrb, ptr);
}

int mrb_sys_gc_live_objects(mrb_state *mrb) {
  mrb_gc *gc = &mrb->gc;
  return gc->live;
}
