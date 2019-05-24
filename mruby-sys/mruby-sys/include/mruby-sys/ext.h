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
 * C extension bindings of mruby to make implementing the mruby-sys crate
 * easier. The functions defined in mruby-sys.h are limited to those that are
 * either not possible to implment in Rust (e.g. because the functions are
 * inlined) or are simpler to implement in C (e.g. any of the mrb_value
 * initializers).
 */

#include <mruby.h>
#include <mruby/array.h>
#include <mruby/class.h>
#include <mruby/data.h>
#include <mruby/error.h>
#include <mruby/proc.h>
#include <mruby/value.h>
#include <mruby/variable.h>

// Check whether `mrb_value` is nil, false, or true

_Bool mrb_sys_value_is_nil(mrb_value value);

_Bool mrb_sys_value_is_false(mrb_value value);

_Bool mrb_sys_value_is_true(mrb_value value);

// Extract pointers from `mrb_value`s

mrb_int mrb_sys_fixnum_to_cint(mrb_value value);

mrb_float mrb_sys_float_to_cdouble(mrb_value value);

struct RBasic *mrb_sys_basic_ptr(mrb_value value);

struct RObject *mrb_sys_obj_ptr(mrb_value value);

struct RProc *mrb_sys_proc_ptr(mrb_value value);

struct RClass *mrb_sys_class_ptr(mrb_value value);

struct RClass *mrb_sys_class_to_rclass(mrb_value value);

struct RClass *mrb_sys_class_of_value(struct mrb_state *mrb, mrb_value value);

// Construct `mrb_value`s

mrb_value mrb_sys_nil_value(void);

mrb_value mrb_sys_false_value(void);

mrb_value mrb_sys_true_value(void);

mrb_value mrb_sys_fixnum_value(mrb_int value);

mrb_value mrb_sys_float_value(struct mrb_state *mrb, mrb_float value);

mrb_value mrb_sys_obj_value(void *p);

mrb_value mrb_sys_class_value(struct RClass *klass);

mrb_value mrb_sys_module_value(struct RClass *module);

mrb_value mrb_sys_data_value(struct RData *data);

mrb_value mrb_sys_proc_value(struct mrb_state *mrb, struct RProc *proc);

// Manipulate `Symbol`s

const char *mrb_sys_symbol_name(struct mrb_state *mrb, mrb_value value);

mrb_value mrb_sys_new_symbol(struct mrb_state *mrb, const char *string,
                             size_t len);

// Manage Rust-backed `mrb_value`s

void mrb_sys_set_instance_tt(struct RClass *class, enum mrb_vtype type);

void mrb_sys_data_init(mrb_value *value, void *ptr, const mrb_data_type *type);

// Raise exceptions and debug info

mrb_noreturn void mrb_sys_raise(struct mrb_state *mrb, const char *eclass,
                                const char *msg);

void mrb_sys_raise_current_exception(struct mrb_state *mrb);

mrb_value mrb_sys_value_debug_str(struct mrb_state *mrb, mrb_value value);

// Manipulate Array `mrb_value`s

mrb_int mrb_sys_ary_len(mrb_value value);

// Manage the mruby garbage collector (GC)

/**
 * Set save point for garbage collection arena to recycle `mrb_value` objects
 * created with C function calls. Returns an index in the arena stack to restore
 * to when calling `mrb_sys_gc_arena_restore`.
 */
int mrb_sys_gc_arena_save(mrb_state *mrb);

/**
 * Restore save point for garbage collection arena to recycle `mrb_value`
 * objects created with C function calls.
 */
void mrb_sys_gc_arena_restore(mrb_state *mrb, int arena_index);

void mrb_sys_gc_disable(mrb_state *mrb);

void mrb_sys_gc_enable(mrb_state *mrb);

_Bool mrb_sys_value_is_dead(mrb_state *_mrb, mrb_value value);

int mrb_sys_gc_live_objects(mrb_state *mrb);
