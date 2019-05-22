use mruby::gc::GarbageCollection;
use mruby::interpreter::Mrb;
use mruby::MrbError;
use nemesis::handler::RackRequest;
use nemesis::{self, handler};
use ref_thread_local::RefThreadLocal;
use rocket::http::Status;
use rocket::{get, Response};
use std::io::Cursor;

use crate::foolsgold::RACKUP;

ref_thread_local! {
    static managed INTERPRETER: Result<Mrb, MrbError> = super::interpreter();
}

#[get("/fools-gold/prefork")]
#[allow(clippy::needless_pass_by_value)]
pub fn rack_app<'a>(req: RackRequest) -> Result<Response<'a>, Response<'a>> {
    info!("Using prefork thread local mruby interpreter");
    match *INTERPRETER.borrow() {
        Ok(ref interp) => {
            let adapter =
                handler::adapter_from_rackup(interp, RACKUP).map_err(|err| {
                    Response::build()
                        .status(Status::InternalServerError)
                        .sized_body(Cursor::new(format!("{}", err)))
                        .finalize()
                })?;
            let arena = interp.create_arena_savepoint();
            let response = handler::run(interp, &adapter, &req).map_err(|err| {
                Response::build()
                    .status(Status::InternalServerError)
                    .sized_body(Cursor::new(format!("{}", err)))
                    .finalize()
            })?;
            arena.restore();
            interp.incremental_gc();
            Ok(response)
        }
        Err(ref err) => Err(Response::build()
            .status(Status::InternalServerError)
            .sized_body(Cursor::new(format!("{}", err)))
            .finalize()),
    }
}
