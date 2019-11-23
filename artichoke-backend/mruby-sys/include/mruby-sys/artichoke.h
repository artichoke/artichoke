#include <mruby/boxing_no.h>
#include <mruby/common.h>
#include <mruby/value.h>

// Array overrides
mrb_value artichoke_ary_new(mrb_state *mrb);
mrb_value artichoke_ary_new_capa(mrb_state *, mrb_int);
mrb_value artichoke_ary_new_from_values(mrb_state *mrb, mrb_int size,
                                        const mrb_value *vals);
mrb_value artichoke_ary_splat(mrb_state *mrb, mrb_value value);
void artichoke_ary_concat(mrb_state *mrb, mrb_value self, mrb_value other);
mrb_value artichoke_ary_pop(mrb_state *mrb, mrb_value ary);
void artichoke_ary_push(mrb_state *mrb, mrb_value array, mrb_value value);
mrb_value artichoke_ary_ref(mrb_state *mrb, mrb_value ary, mrb_int n);
void artichoke_ary_set(mrb_state *mrb, mrb_value ary, mrb_int n, mrb_value val);
mrb_value artichoke_ary_shift(mrb_state *mrb, mrb_value self);
mrb_value artichoke_ary_unshift(mrb_state *mrb, mrb_value self, mrb_value item);
mrb_int artichoke_ary_len(mrb_state *mrb, mrb_value self);
void artichoke_ary_set_len(mrb_state *mrb, mrb_value self, mrb_int len);
mrb_value *artichoke_ary_ptr(mrb_state *mrb, mrb_value self);
mrb_bool artichoke_ary_check(mrb_state *mrb, mrb_value ary);

// GC
void artichoke_gc_mark_ary(mrb_state *mrb, mrb_value ary);
size_t artichoke_gc_mark_ary_size(mrb_state *mrb, mrb_value ary);
