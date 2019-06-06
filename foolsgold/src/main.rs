#![feature(integer_atomics)]
#![feature(proc_macro_hygiene, decl_macro)]
#![deny(warnings, intra_doc_link_resolution_failure)]
#![deny(clippy::all, clippy::pedantic)]

#[macro_use]
extern crate log;
#[macro_use]
extern crate mruby;
#[macro_use]
extern crate rust_embed;

use mruby::eval::MrbEval;
use mruby::interpreter::Mrb;
use mruby::MrbError;
use nemesis::{Builder, Error, Mount};

mod assets;
mod foolsgold;

use assets::Assets;

pub fn main() -> Result<(), i32> {
    env_logger::Builder::from_env("FOOLSGOLD_LOG").init();
    if let Err(err) = spawn() {
        error!("Failed to launch nemesis: {}", err);
        eprintln!("ERR: {}", err);
        Err(1)
    } else {
        Ok(())
    }
}

pub fn spawn() -> Result<(), Error> {
    Builder::default()
        .add_mount(
            Mount::from_rackup("foolsgold", foolsgold::RACKUP, "/fools-gold/shared-nothing")
                .with_init(Box::new(interp_init)),
        )
        .add_mount(
            Mount::from_rackup("foolsgold", foolsgold::RACKUP, "/fools-gold/prefork")
                .with_init(Box::new(interp_init))
                .with_shared_interpreter(Some(150)),
        )
        .add_static_assets(Assets::all()?)
        .serve()
}

fn interp_init(interp: &Mrb) -> Result<(), MrbError> {
    foolsgold::init(interp)?;
    // preload foolsgold sources
    interp.eval("require 'foolsgold'")?;
    Ok(())
}
