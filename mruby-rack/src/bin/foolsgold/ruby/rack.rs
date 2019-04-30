use mruby::{sys, Mrb, MrbFile};

use crate::ruby::Source;

/// [`RackBuilder`] is an empty struct that implements `MrbFile`. Requiring
/// [`RackBuilder`] on an [`MrbApi`] exposes the Ruby class
/// [`Rack::Builder`](https://github.com/rack/rack/blob/2.0.7/lib/rack/builder.rb).
///
/// `Rack::Builder` can generate a Rack-compatible app from a `config.ru`
/// rackup file.
pub struct Builder;

impl MrbFile for Builder {
    fn require(interp: Mrb) {
        let builder = Source::contents("rack/builder.rb");
        let mrb = interp.borrow().mrb();
        let ctx = interp.borrow().ctx();
        unsafe {
            // load Rack::Builder by evaling the source file on the mruby
            // interpreter.
            sys::mrb_load_nstring_cxt(mrb, builder.as_ptr() as *const i8, builder.len(), ctx);
        }
    }
}
