#include <mruby.h>
#include <mruby/array.h>
#include <mruby/boxing_no.h>
#include <mruby/class.h>
#include <mruby/common.h>
#include <mruby/data.h>
#include <mruby/error.h>
#include <mruby/proc.h>
#include <mruby/value.h>
#include <mruby/variable.h>

// Array overrides
MRB_API mrb_value artichoke_assoc_new(mrb_state *mrb, mrb_value car,
                                      mrb_value cdr);
MRB_API mrb_value artichoke_value_to_ary(mrb_state *mrb, mrb_value value);
MRB_API mrb_value artichoke_ary_new(mrb_state *mrb);
MRB_API mrb_value artichoke_ary_new_capa(mrb_state *, mrb_int);
MRB_API mrb_value artichoke_ary_new_from_values(mrb_state *mrb, mrb_int size,
                                                const mrb_value *vals);
MRB_API mrb_value artichoke_ary_splat(mrb_state *mrb, mrb_value value);
MRB_API mrb_value artichoke_ary_clone(mrb_state *mrb, mrb_value value);
MRB_API mrb_value artichoke_ary_clear(mrb_state *mrb, mrb_value self);
MRB_API mrb_value artichoke_ary_entry(mrb_value ary, mrb_int offset);
MRB_API mrb_value artichoke_ary_ref(mrb_state *mrb, mrb_value ary, mrb_int n);
MRB_API mrb_value artichoke_ary_entry(mrb_value ary, mrb_int offset);
MRB_API mrb_value artichoke_ary_ref(mrb_state *mrb, mrb_value ary, mrb_int n);
MRB_API mrb_value artichoke_ary_join(mrb_state *mrb, mrb_value ary,
                                     mrb_value sep);
MRB_API mrb_value artichoke_ary_pop(mrb_state *mrb, mrb_value ary);
MRB_API mrb_value artichoke_ary_resize(mrb_state *mrb, mrb_value ary,
                                       mrb_int new_len);
MRB_API mrb_value artichoke_ary_shift(mrb_state *mrb, mrb_value self);
MRB_API mrb_value artichoke_ary_splice(mrb_state *mrb, mrb_value self,
                                       mrb_int head, mrb_int len,
                                       mrb_value rpl);
MRB_API mrb_value artichoke_ary_shift(mrb_state *mrb, mrb_value self);
MRB_API mrb_value artichoke_ary_unshift(mrb_state *mrb, mrb_value self,
                                        mrb_value item);
MRB_API mrb_int artichoke_ary_len(mrb_state *mrb, mrb_value self);
MRB_API void artichoke_ary_concat(mrb_state *mrb, mrb_value self,
                                  mrb_value other);
MRB_API void artichoke_ary_push(mrb_state *mrb, mrb_value array,
                                mrb_value value);
MRB_API void artichoke_ary_replace(mrb_state *mrb, mrb_value self,
                                   mrb_value other);
MRB_API void artichoke_ary_set(mrb_state *mrb, mrb_value ary, mrb_int n,
                               mrb_value val);
MRB_API mrb_bool artichoke_ary_check(mrb_state *mrb, mrb_value ary);

// Expose mrbgems subsystem initializer
MRB_API void mrb_init_mrbgems(mrb_state *mrb);
