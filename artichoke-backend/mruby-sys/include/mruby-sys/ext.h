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

/**
 * C extension bindings of mruby to make implementing the artichoke-backend
 * crate easier. The functions defined in mruby-sys.h are limited to those that
 * are either not possible to implment in Rust (e.g. because the functions are
 * inlined) or are simpler to implement in C (e.g. any of the `mrb_value`
 * initializers).
 */

#include <mruby/class.h>
#include <mruby/common.h>
#include <mruby/data.h>
#include <mruby/error.h>
#include <mruby/proc.h>
#include <mruby/value.h>
#include <mruby/variable.h>

// Check whether `mrb_value` is nil, false, or true

MRB_API _Bool mrb_sys_value_is_nil(mrb_value value);

MRB_API _Bool mrb_sys_value_is_false(mrb_value value);

MRB_API _Bool mrb_sys_value_is_true(mrb_value value);

MRB_API _Bool mrb_sys_range_excl(mrb_state *mrb, mrb_value value);

MRB_API _Bool mrb_sys_obj_frozen(mrb_state *mrb, mrb_value value);

// Extract pointers from `mrb_value`s

MRB_API mrb_int mrb_sys_fixnum_to_cint(mrb_value value);

MRB_API mrb_float mrb_sys_float_to_cdouble(mrb_value value);

MRB_API void *mrb_sys_cptr_ptr(mrb_value value);

MRB_API struct RBasic *mrb_sys_basic_ptr(mrb_value value);

MRB_API struct RObject *mrb_sys_obj_ptr(mrb_value value);

MRB_API struct RProc *mrb_sys_proc_ptr(mrb_value value);

MRB_API struct RClass *mrb_sys_class_ptr(mrb_value value);

MRB_API struct RClass *mrb_sys_class_to_rclass(mrb_value value);

MRB_API struct RClass *mrb_sys_class_of_value(struct mrb_state *mrb, mrb_value value);

// Construct `mrb_value`s

MRB_API mrb_value mrb_sys_nil_value(void);

MRB_API mrb_value mrb_sys_false_value(void);

MRB_API mrb_value mrb_sys_true_value(void);

MRB_API mrb_value mrb_sys_fixnum_value(mrb_int value);

MRB_API mrb_value mrb_sys_float_value(struct mrb_state *mrb, mrb_float value);

MRB_API mrb_value mrb_sys_cptr_value(struct mrb_state *mrb, void *ptr);

MRB_API mrb_value mrb_sys_obj_value(void *p);

MRB_API mrb_value mrb_sys_class_value(struct RClass *klass);

MRB_API mrb_value mrb_sys_module_value(struct RClass *module);

MRB_API mrb_value mrb_sys_data_value(struct RData *data);

MRB_API mrb_value mrb_sys_proc_value(struct mrb_state *mrb, struct RProc *proc);

// Manipulate `Symbol`s

MRB_API mrb_value mrb_sys_new_symbol(mrb_sym id);

// Manage Rust-backed `mrb_value`s

MRB_API void mrb_sys_set_instance_tt(struct RClass *class, enum mrb_vtype type);

MRB_API void mrb_sys_data_init(mrb_value *value, void *ptr, const mrb_data_type *type);

// Raise exceptions and debug info

MRB_API mrb_noreturn void mrb_sys_raise(struct mrb_state *mrb, const char *eclass, const char *msg);

MRB_API void mrb_sys_raise_current_exception(struct mrb_state *mrb);

// Manipulate Array `mrb_value`s

MRB_API mrb_value mrb_sys_alloc_rarray(struct mrb_state *mrb, mrb_value *ptr, mrb_int len,
                                       mrb_int capa);

MRB_API void mrb_sys_repack_into_rarray(mrb_value *ptr, mrb_int len, mrb_int capa, mrb_value into);

// Manipulate String `mrb_value`s

MRB_API mrb_value mrb_sys_alloc_rstring(struct mrb_state *mrb, char *ptr, mrb_int len,
                                        mrb_int capa);

MRB_API struct RString *mrb_sys_repack_into_rstring(char *ptr, mrb_int len, mrb_int capa,
                                                    mrb_value into);

// Manage the mruby garbage collector (GC)

/**
 * Set save point for garbage collection arena to recycle `mrb_value` objects
 * created with C function calls. Returns an index in the arena stack to restore
 * to when calling `mrb_sys_gc_arena_restore`.
 */
MRB_API int mrb_sys_gc_arena_save(mrb_state *mrb);

/**
 * Restore save point for garbage collection arena to recycle `mrb_value`
 * objects created with C function calls.
 */
MRB_API void mrb_sys_gc_arena_restore(mrb_state *mrb, int arena_index);

/**
 * Disable GC. Returns previous enabled state.
 */
MRB_API _Bool mrb_sys_gc_disable(mrb_state *mrb);

/**
 * Enable GC. Returns previous enabled state.
 */
MRB_API _Bool mrb_sys_gc_enable(mrb_state *mrb);

MRB_API _Bool mrb_sys_value_is_dead(mrb_state *_mrb, mrb_value value);

MRB_API int mrb_sys_gc_live_objects(mrb_state *mrb);

MRB_API void mrb_sys_safe_gc_mark(mrb_state *mrb, mrb_value value);
