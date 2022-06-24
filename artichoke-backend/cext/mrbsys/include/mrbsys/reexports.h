#include <mruby.h>
#include <mruby/common.h>

#ifdef __cplusplus
extern "C" {
#endif

// Expose mrbgems subsystem initializer
MRB_API void mrb_init_mrbgems(mrb_state *mrb);

#ifdef __cplusplus
}  /* extern "C" { */
#endif
