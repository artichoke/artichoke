/*
** compar.c - Comparable module
**
** See Copyright Notice in mruby.h
*/

#include <mruby.h>

#ifdef __cplusplus
extern "C" {
#endif

void
mrb_init_comparable(mrb_state *mrb)
{
  mrb_define_module(mrb, "Comparable");  /* 15.3.3 */
}

#ifdef __cplusplus
} /* extern "C" { */
#endif
