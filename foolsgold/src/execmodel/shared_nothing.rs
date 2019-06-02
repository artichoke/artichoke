use nemesis::server::rocket::request::Request;
use nemesis::{self, adapter, handler};
use rocket::{get, Response};

use crate::execmodel::Error;
use crate::foolsgold::RACKUP;

#[get("/fools-gold")]
#[allow(clippy::needless_pass_by_value)]
pub fn rack_app<'a>(req: Request) -> Result<Response<'a>, Error> {
    info!("Initializing fresh shared nothing mruby interpreter");
    let interp = super::interpreter()?;
    let adapter = adapter::from_rackup(&interp, RACKUP)?;
    // GC and managing the arena are unnecessary since we throw the interpreter
    // away at the end of the request.
    let response = handler::run(&interp, &adapter, &req)?;
    Ok(response)
}
