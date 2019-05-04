// ext derived from mrusty @ 1.0.0
// <https://github.com/anima-engine/mrusty/tree/v1.0.0>

// mrusty. mruby safe bindings for Rust
// Copyright (C) 2016  Dragoș Tiselice
//
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

/**
 * Extract the integer value from a Fixnum `mrb_value`
 */
mrb_int mrb_sys_fixnum_to_cint(mrb_value value);

/**
 * Extract the float value from a Float `mrb_value`
 */
mrb_float mrb_sys_float_to_cdouble(mrb_value value);

/**
 * Test if an `mrb_value` is a Ruby `nil`
 */
_Bool mrb_sys_value_is_nil(mrb_value value);

/**
 * Test if an `mrb_value` is a Ruby `false`
 */
_Bool mrb_sys_value_is_false(mrb_value value);

/**
 * Test if an `mrb_value` is a Ruby `true`
 */
_Bool mrb_sys_value_is_true(mrb_value value);

/**
 * Extract the `RClass` from a Class `mrb_value`
 */
struct RClass *mrb_sys_class_to_rclass(mrb_value value);

/**
 * Create an `mrb_value` representing `nil`
 */
mrb_value mrb_sys_nil_value(void);

/**
 * Create an `mrb_value` representing `false`
 */
mrb_value mrb_sys_false_value(void);

/**
 * Create an `mrb_value` representing `true`
 */
mrb_value mrb_sys_true_value(void);

/**
 * Create an `mrb_value` representing an integer (a `Fixnum`)
 */
mrb_value mrb_sys_fixnum_value(mrb_int value);

/**
 * Create an `mrb_value` representing a float
 */
mrb_value mrb_sys_float_value(struct mrb_state *mrb, mrb_float value);

/**
 * Create an `mrb_value` from an `RProc`
 */
mrb_value mrb_sys_proc_value(struct mrb_state *mrb, struct RProc *proc);

/**
 * Create a `Class` `mrb_value` from an `RClass`
 */
mrb_value mrb_sys_class_value(struct RClass *klass);

/**
 * Create a `Module` `mrb_value` from an `RClass`
 */
mrb_value mrb_sys_module_value(struct RClass *module);

/**
 * Create an `mrb_value` from an `RData`
 */
mrb_value mrb_sys_data_value(struct RData *data);

/**
 * Create an `mrb_value` from a `void *`
 */
mrb_value mrb_sys_obj_value(void *p);

/**
 * Set instance type tag
 */
void mrb_sys_set_instance_tt(struct RClass *class, enum mrb_vtype type);

/**
 * Get a C string with the name of the symbol identified by an `mrb_value`
 */
const char *mrb_sys_symbol_name(struct mrb_state *mrb, mrb_value value);

/**
 * Create a new symbol from a C string
 */
mrb_value mrb_sys_new_symbol(struct mrb_state *mrb, const char *string,
                             size_t len);

// TODO: document purpose
void mrb_sys_data_init(mrb_value *value, void *ptr, const mrb_data_type *type);

/**
 * Raise the most recent thrown exception on `mrb_state`
 */
void mrb_sys_raise_current_exception(struct mrb_state *mrb);

/**
 * Generate a String `mrb_value` from a value suitable for debug logging
 */
mrb_value mrb_sys_value_debug_str(struct mrb_state *mrb, mrb_value value);

/**
 * Raise an exception class with a message
 */
mrb_noreturn void mrb_sys_raise(struct mrb_state *mrb, const char *eclass,
                                const char *msg);

/**
 * Check if a class is defined under another class or module
 */
mrb_bool mrb_sys_class_defined_under(struct mrb_state *mrb,
                                     struct RClass *outer, const char *name);

/**
 * Get the `RClass` representing the `Class` of an `mrb_value`
 */
struct RClass *mrb_sys_class_of_value(struct mrb_state *mrb, mrb_value value);

/**
 * Get length of an `Array`
 */
mrb_int mrb_sys_ary_len(mrb_value value);

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

/**
 * Get an `RBasic` pointer to an mruby object.
 */
struct RBasic *mrb_sys_basic_ptr(mrb_value value);
