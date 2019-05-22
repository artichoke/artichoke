use nemesis::handler::RackRequest;
use nemesis::{self, handler};
use rocket::http::Status;
use rocket::{get, Response};
use std::io::Cursor;

use crate::sources::rackup;

#[get("/fools-gold")]
#[allow(clippy::needless_pass_by_value)]
pub fn rack_app<'a>(req: RackRequest) -> Result<Response<'a>, Response<'a>> {
    info!("Initializing fresh shared nothing mruby interpreter");
    let interp = super::interpreter().map_err(|err| {
        Response::build()
            .status(Status::InternalServerError)
            .sized_body(Cursor::new(format!("{}", err)))
            .finalize()
    })?;
    let adapter = handler::adapter_from_rackup(&interp, rackup::rack_adapter()).map_err(|err| {
        Response::build()
            .status(Status::InternalServerError)
            .sized_body(Cursor::new(format!("{}", err)))
            .finalize()
    })?;
    // GC and managing the arena are unnecessary since we throw the interpreter
    // away at the end of the request.
    handler::run(&interp, &adapter, &req).map_err(|err| {
        Response::build()
            .status(Status::InternalServerError)
            .sized_body(Cursor::new(format!("{}", err)))
            .finalize()
    })
}
