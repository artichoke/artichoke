#![deny(warnings, intra_doc_link_resolution_failure)]
#![deny(clippy::all, clippy::pedantic)]
#![forbid(unsafe_code)]

#[macro_use]
extern crate rust_embed;

use mruby::eval::MrbEval;
use mruby::interpreter::Mrb;
use mruby::MrbError;
use mruby_gems::rubygems::rack;

pub mod adapter;
pub mod handler;
pub mod request;
pub mod response;
mod rubygems;
pub mod server;

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
