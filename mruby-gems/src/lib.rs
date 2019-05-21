#![deny(warnings, intra_doc_link_resolution_failure)]

#[macro_use]
extern crate rust_embed;

use mruby::interpreter::Mrb;
use mruby::MrbError;

pub mod rubygems;

pub trait Gem {
    fn install(interp: &mut Mrb) -> Result<(), MrbError>;
}
