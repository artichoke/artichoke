#include <stdlib.h>
#include <mruby.h>
#include <mruby/proc.h>

#ifdef __cplusplus
extern "C" {
#endif

void mrb_mruby_class_ext_gem_init(mrb_state *mrb);
void mrb_mruby_class_ext_gem_final(mrb_state *mrb);
void mrb_mruby_error_gem_init(mrb_state *mrb);
void mrb_mruby_error_gem_final(mrb_state *mrb);
void mrb_mruby_eval_gem_init(mrb_state *mrb);
void mrb_mruby_eval_gem_final(mrb_state *mrb);
void mrb_mruby_fiber_gem_init(mrb_state *mrb);
void mrb_mruby_fiber_gem_final(mrb_state *mrb);
void mrb_mruby_metaprog_gem_init(mrb_state *mrb);
void mrb_mruby_metaprog_gem_final(mrb_state *mrb);
void mrb_mruby_method_gem_init(mrb_state *mrb);
void mrb_mruby_method_gem_final(mrb_state *mrb);
void mrb_mruby_pack_gem_init(mrb_state *mrb);
void mrb_mruby_pack_gem_final(mrb_state *mrb);
void mrb_mruby_proc_ext_gem_init(mrb_state *mrb);
void mrb_mruby_proc_ext_gem_final(mrb_state *mrb);
void mrb_mruby_sprintf_gem_init(mrb_state *mrb);
void mrb_mruby_sprintf_gem_final(mrb_state *mrb);
void mrb_mruby_toplevel_ext_gem_init(mrb_state *mrb);
void mrb_mruby_toplevel_ext_gem_final(mrb_state *mrb);

void
artichoke_mrbgem_mruby_class_ext_gem_init(mrb_state *mrb)
{
  int ai = mrb_gc_arena_save(mrb);
  mrb_mruby_class_ext_gem_init(mrb);
  mrb_gc_arena_restore(mrb, ai);
}

void
artichoke_mrbgem_mruby_class_ext_gem_final(mrb_state *mrb)
{
  mrb_mruby_class_ext_gem_final(mrb);
}

void
artichoke_mrbgem_mruby_error_gem_init(mrb_state *mrb)
{
  int ai = mrb_gc_arena_save(mrb);
  mrb_mruby_error_gem_init(mrb);
  mrb_gc_arena_restore(mrb, ai);
}

void
artichoke_mrbgem_mruby_error_gem_final(mrb_state *mrb)
{
  mrb_mruby_error_gem_final(mrb);
}

void
artichoke_mrbgem_mruby_eval_gem_init(mrb_state *mrb)
{
  int ai = mrb_gc_arena_save(mrb);
  mrb_mruby_eval_gem_init(mrb);
  mrb_gc_arena_restore(mrb, ai);
}

void
artichoke_mrbgem_mruby_eval_gem_final(mrb_state *mrb)
{
  mrb_mruby_eval_gem_final(mrb);
}

void
artichoke_mrbgem_mruby_fiber_gem_init(mrb_state *mrb)
{
  int ai = mrb_gc_arena_save(mrb);
  mrb_mruby_fiber_gem_init(mrb);
  mrb_gc_arena_restore(mrb, ai);
}

void
artichoke_mrbgem_mruby_fiber_gem_final(mrb_state *mrb)
{
  mrb_mruby_fiber_gem_final(mrb);
}

void
artichoke_mrbgem_mruby_metaprog_gem_init(mrb_state *mrb)
{
  int ai = mrb_gc_arena_save(mrb);
  mrb_mruby_metaprog_gem_init(mrb);
  mrb_gc_arena_restore(mrb, ai);
}

void
artichoke_mrbgem_mruby_metaprog_gem_final(mrb_state *mrb)
{
  mrb_mruby_metaprog_gem_final(mrb);
}

void
artichoke_mrbgem_mruby_method_gem_init(mrb_state *mrb)
{
  int ai = mrb_gc_arena_save(mrb);
  mrb_mruby_method_gem_init(mrb);
  mrb_gc_arena_restore(mrb, ai);
}

void
artichoke_mrbgem_mruby_method_gem_final(mrb_state *mrb)
{
  mrb_mruby_method_gem_final(mrb);
}

void
artichoke_mrbgem_mruby_pack_gem_init(mrb_state *mrb)
{
  int ai = mrb_gc_arena_save(mrb);
  mrb_mruby_pack_gem_init(mrb);
  mrb_gc_arena_restore(mrb, ai);
}

void
artichoke_mrbgem_mruby_pack_gem_final(mrb_state *mrb)
{
  mrb_mruby_pack_gem_final(mrb);
}

void
artichoke_mrbgem_mruby_proc_ext_gem_init(mrb_state *mrb)
{
  int ai = mrb_gc_arena_save(mrb);
  mrb_mruby_proc_ext_gem_init(mrb);
  mrb_gc_arena_restore(mrb, ai);
}

void
artichoke_mrbgem_mruby_proc_ext_gem_final(mrb_state *mrb)
{
  mrb_mruby_proc_ext_gem_final(mrb);
}

void
artichoke_mrbgem_mruby_sprintf_gem_init(mrb_state *mrb)
{
  int ai = mrb_gc_arena_save(mrb);
  mrb_mruby_sprintf_gem_init(mrb);
  mrb_gc_arena_restore(mrb, ai);
}

void
artichoke_mrbgem_mruby_sprintf_gem_final(mrb_state *mrb)
{
  mrb_mruby_sprintf_gem_final(mrb);
}

void
artichoke_mrbgem_mruby_toplevel_ext_gem_init(mrb_state *mrb)
{
  int ai = mrb_gc_arena_save(mrb);
  mrb_gc_arena_restore(mrb, ai);
}

void
artichoke_mrbgem_mruby_toplevel_ext_gem_final(mrb_state *mrb)
{
}

#ifdef __cplusplus
} /* extern "C" { */
#endif
