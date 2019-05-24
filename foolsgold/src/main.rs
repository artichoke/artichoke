#![feature(integer_atomics)]
#![feature(proc_macro_hygiene, decl_macro)]
#![deny(warnings, intra_doc_link_resolution_failure)]
#![deny(clippy::all, clippy::pedantic)]

#[macro_use]
extern crate log;
#[macro_use]
extern crate mruby;
#[macro_use]
extern crate ref_thread_local;
#[macro_use]
extern crate rust_embed;

use rocket::routes;

mod assets;
mod execmodel;
mod foolsgold;

pub fn main() -> Result<(), i32> {
    env_logger::Builder::from_env("FOOLSGOLD_LOG").init();
    if let Err(err) = spawn() {
        eprintln!("ERR: {}", err);
        Err(1)
    } else {
        Ok(())
    }
}

pub fn spawn() -> Result<(), String> {
    let err = rocket::ignite()
        .mount(
            "/",
            routes![
                assets::index,
                execmodel::shared_nothing::rack_app,
                execmodel::prefork::rack_app
            ],
        )
        .mount("/img", routes![assets::pyrite, assets::resf])
        .launch();
    // This log is only reachable if Rocket has an error during startup,
    // otherwise `rocket::ignite().launch()` blocks forever.
    warn!("Failed to launch rocket: {}", err);
    Err(err.to_string())
}
