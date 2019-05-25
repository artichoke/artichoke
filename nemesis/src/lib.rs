#[macro_use]
extern crate rust_embed;

use mruby::eval::MrbEval;
use mruby::interpreter::Mrb;
use mruby::MrbError;
use mruby_gems::rubygems::rack;

pub mod handler;
mod rubygems;

use rubygems::nemesis;

pub fn init(interp: &Mrb) -> Result<(), MrbError> {
    rack::init(interp)?;
    nemesis::init(interp)?;
    // The Rack module makes heavy use of `Module#autoload` for dynamically
    // resolving constants. For now, we don't care about this and manually
    // require the bits of Rack we need. Create a stub implementation of
    // `Module#autoload` that to allow the module to successfully eval on
    // require.
    //
    // TODO: remove this stub once GH-12 is resolved.
    interp.eval(r#"class Module; def autoload(*args); end; end"#)?;
    // Preload required gem sources
    interp.eval("require 'rack'")?;
    interp.eval("require 'rack/builder'")?;
    interp.eval("require 'nemesis/response'")?;
    Ok(())
}
