#![deny(warnings, intra_doc_link_resolution_failure)]
#![deny(clippy::all, clippy::pedantic)]

#[macro_use]
extern crate log;

use mruby::eval::{EvalContext, MrbEval};
use mruby::interpreter::Mrb;
use mruby::MrbError;
use mruby_gems::rubygems;
use nemesis::{Builder, Error, Mount};

const APP: &str = include_str!("config.ru");

pub fn main() -> Result<(), i32> {
    env_logger::Builder::from_env("HUBRIS_LOG").init();
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
            Mount::from_rackup("echo", APP, "/")
                .with_init(Box::new(interp_init))
                .with_shared_interpreter(Some(150)),
        )
        .serve()
}

fn interp_init(interp: &Mrb) -> Result<(), MrbError> {
    rubygems::mustermann::init(&interp)?;
    rubygems::rack::init(&interp)?;
    rubygems::rack_protection::init(&interp)?;
    rubygems::sinatra::init(&interp)?;
    rubygems::tilt::init(&interp)?;
    interp.eval_with_context(
        include_str!("echo_server.rb"),
        EvalContext::new("/src/lib/echo_server.rb"),
    )?;
    Ok(())
}
