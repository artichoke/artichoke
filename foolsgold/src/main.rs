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
use nemesis::adapter::RackApp;
use nemesis::interpreter::ExecMode;
use nemesis::server::{Builder, Mount};
use nemesis::Error;
use std::sync::{Arc, Mutex};

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
        .add_mount(Mount {
            path: "/fools-gold".to_owned(),
            // TODO: make this API nicer, it should probably just take a func
            app: Arc::new(Mutex::new(Box::new(|interp: &Mrb| {
                RackApp::from_rackup(interp, foolsgold::RACKUP)
            }))),
            // TODO: make this API nicer, it should probably just take a func
            interp_init: Some(Arc::new(Mutex::new(Box::new(|interp: &Mrb| {
                foolsgold::init(&interp)?;
                // preload foolsgold sources
                interp.eval("require 'foolsgold'")?;
                Ok(())
            })))),
            exec_mode: ExecMode::SingleUse,
        })
        .add_static_asset(
            "/",
            Assets::get("index.html").expect("missing static asset"),
        )
        .add_static_asset(
            "/img/pyrite.jpg",
            Assets::get("pyrite.jpg").expect("missing static asset"),
        )
        .add_static_asset(
            "/img/resf.png",
            Assets::get("resf.png").expect("missing static asset"),
        )
        .serve()
}
