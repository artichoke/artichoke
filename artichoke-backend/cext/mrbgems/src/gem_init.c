#include <mruby.h>

#ifdef __cplusplus
extern "C" {
#endif

void artichoke_mrbgem_mruby_eval_gem_init(mrb_state *);
void artichoke_mrbgem_mruby_eval_gem_final(mrb_state *);
void artichoke_mrbgem_mruby_metaprog_gem_init(mrb_state *);
void artichoke_mrbgem_mruby_metaprog_gem_final(mrb_state *);
void artichoke_mrbgem_mruby_proc_ext_gem_init(mrb_state *);
void artichoke_mrbgem_mruby_proc_ext_gem_final(mrb_state *);
void artichoke_mrbgem_mruby_method_gem_init(mrb_state *);
void artichoke_mrbgem_mruby_method_gem_final(mrb_state *);
void artichoke_mrbgem_mruby_toplevel_ext_gem_init(mrb_state *);
void artichoke_mrbgem_mruby_toplevel_ext_gem_final(mrb_state *);
void artichoke_mrbgem_mruby_fiber_gem_init(mrb_state *);
void artichoke_mrbgem_mruby_fiber_gem_final(mrb_state *);
void artichoke_mrbgem_mruby_pack_gem_init(mrb_state *);
void artichoke_mrbgem_mruby_pack_gem_final(mrb_state *);
void artichoke_mrbgem_mruby_sprintf_gem_init(mrb_state *);
void artichoke_mrbgem_mruby_sprintf_gem_final(mrb_state *);
void artichoke_mrbgem_mruby_class_ext_gem_init(mrb_state *);
void artichoke_mrbgem_mruby_class_ext_gem_final(mrb_state *);

static void
mrb_final_mrbgems(mrb_state *mrb)
{
  artichoke_mrbgem_mruby_class_ext_gem_final(mrb);
  artichoke_mrbgem_mruby_sprintf_gem_final(mrb);
  artichoke_mrbgem_mruby_pack_gem_final(mrb);
  artichoke_mrbgem_mruby_fiber_gem_final(mrb);
  artichoke_mrbgem_mruby_toplevel_ext_gem_final(mrb);
  artichoke_mrbgem_mruby_method_gem_final(mrb);
  artichoke_mrbgem_mruby_proc_ext_gem_final(mrb);
  artichoke_mrbgem_mruby_metaprog_gem_final(mrb);
  artichoke_mrbgem_mruby_eval_gem_final(mrb);
}

void
mrb_init_mrbgems(mrb_state *mrb)
{
  artichoke_mrbgem_mruby_eval_gem_init(mrb);
  artichoke_mrbgem_mruby_metaprog_gem_init(mrb);
  artichoke_mrbgem_mruby_proc_ext_gem_init(mrb);
  artichoke_mrbgem_mruby_method_gem_init(mrb);
  artichoke_mrbgem_mruby_toplevel_ext_gem_init(mrb);
  artichoke_mrbgem_mruby_fiber_gem_init(mrb);
  artichoke_mrbgem_mruby_pack_gem_init(mrb);
  artichoke_mrbgem_mruby_sprintf_gem_init(mrb);
  artichoke_mrbgem_mruby_class_ext_gem_init(mrb);
  mrb_state_atexit(mrb, mrb_final_mrbgems);
}

#ifdef __cplusplus
} /* extern "C" { */
#endif
