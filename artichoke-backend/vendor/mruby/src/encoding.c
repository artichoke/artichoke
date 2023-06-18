/*
** encoding.c - Encoding class
**
** See Copyright Notice in mruby.h
*/

#include <mruby.h>
#include <mruby/class.h>

void
mrb_init_encoding(mrb_state *mrb)
{
  struct RClass *enc;

  mrb->encoding_class = mrb_define_class(mrb, "Encoding", mrb->object_class);  /* 15.2.11 */
  MRB_SET_INSTANCE_TT(enc, MRB_TT_ENCODING);
  mrb_undef_class_method(mrb,  enc, "new");
}
