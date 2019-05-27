#![deny(warnings, intra_doc_link_resolution_failure)]
#![deny(clippy::all, clippy::pedantic)]
#![forbid(unsafe_code)]

#[macro_use]
extern crate rust_embed;

use mruby::eval::MrbEval;
use mruby::interpreter::Mrb;
use mruby::MrbError;
use mruby_gems::rubygems::rack;

pub mod handler;
pub mod request;
pub mod response;
mod rubygems;

use rubygems::nemesis;

pub fn init(interp: &Mrb) -> Result<(), MrbError> {
    rack::init(interp)?;
    nemesis::init(interp)?;
    // Preload required gem sources
    interp.eval("require 'rack'")?;
    interp.eval("require 'nemesis'")?;
    interp.eval("require 'nemesis/response'")?;
    Ok(())
}

pub mod adapter {
    use mruby::eval::MrbEval;
    use mruby::interpreter::Mrb;
    use mruby::value::Value;
    use mruby::MrbError;
    use std::convert::AsRef;

    /// Create a Rack app by wrapping the supplied rackup source in a
    /// `Rack::Builder`. The returned [`Value`] has a call method and is
    /// suitable for passing to [`handler::run`](crate::handler::run).
    pub fn from_rackup<T: AsRef<str>>(interp: &Mrb, rackup: T) -> Result<Value, MrbError> {
        interp.eval(format!(
            r#"
            Rack::Builder.new do
              {rackup}
            end
            "#,
            rackup = rackup.as_ref()
        ))
    }
}
