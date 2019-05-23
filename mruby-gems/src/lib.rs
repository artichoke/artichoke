#![deny(warnings, intra_doc_link_resolution_failure)]

#[macro_use]
extern crate rust_embed;

use mruby::interpreter::Mrb;
use mruby::MrbError;

pub mod rubygems;

/// Define a Rubygem that can be installed into an [`Mrb`] interpreter.
pub trait Gem {
    /// Initialize a gem in the [`Mrb`] interpreter.
    fn init(interp: &Mrb) -> Result<(), MrbError>;
}
