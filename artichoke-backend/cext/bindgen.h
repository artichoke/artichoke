/**
 * `bindgen.h` includes all headers for the mruby, mrbgems, and mrbsys C
 * extensions which are linked to the `artichoke-backend` crate.
 *
 * This file is parsed by `bindgen` in `build.rs` to generate Rust bindings for
 * the C functions and types defined in these the mruby and mrbsys C headers.
 * These bindings are exported in the `artichoke_backend::sys` module.
 */

#include <mruby.h>
#include <mruby/array.h>
#include <mruby/boxing_no.h>
#include <mruby/class.h>
#include <mruby/common.h>
#include <mruby/compile.h>
#include <mruby/data.h>
#include <mruby/dump.h>
#include <mruby/error.h>
#include <mruby/gc.h>
#include <mruby/hash.h>
#include <mruby/irep.h>
#include <mruby/istruct.h>
#include <mruby/khash.h>
#include <mruby/numeric.h>
#include <mruby/object.h>
#include <mruby/opcode.h>
#include <mruby/proc.h>
#include <mruby/range.h>
#include <mruby/re.h>
#include <mruby/string.h>
#include <mruby/throw.h>
#include <mruby/value.h>
#include <mruby/variable.h>
#include <mruby/version.h>

#include <mrbsys/ext.h>
#ifdef ARTICHOKE
#include <mrbsys/artichoke.h>
#endif

// Expose mrbgems subsystem initializer
MRB_API void mrb_init_mrbgems(mrb_state *mrb);
