#[macro_use]
extern crate rust_embed;

use mruby::eval::MrbEval;
use mruby::interpreter::Mrb;
use mruby::MrbError;
use mruby_gems::rubygems::rack;

mod gem;
pub mod handler;

pub fn init(interp: &mut Mrb) -> Result<(), MrbError> {
    rack::init(interp)?;
    gem::init(interp)?;
    // TODO: properly implement Module#autoload and remove this hack to allow
    // access to Rack constants defined in rack.rb.
    interp.eval(r#"class Module; def autoload(*args); end; end"#)?;
    // Preload required gems
    interp.eval("require 'rack'")?;
    interp.eval("require 'rack/builder'")?;
    interp.eval("require 'nemesis/response'")?;
    Ok(())
}
