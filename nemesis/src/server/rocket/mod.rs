//! Nemesis server based on [Rocket](rocket).

use rocket::http::Status;
use rocket::request::Request;
use rocket::response::{self, Responder};
use std::io::Cursor;

use crate::server::Builder;
use crate::Error;

pub mod request;
pub mod routes;

pub fn launcher(builder: Builder) -> Result<(), Error> {
    let mut launcher = rocket::ignite();
    for (path, mount) in builder.mounts.0 {
        launcher = launcher.mount(path.as_str(), routes::RackHandler::routes(&mount));
    }
    for path in builder.assets.0.keys() {
        launcher = launcher.mount(path.as_str(), routes![routes::static_asset]);
    }
    launcher = launcher.manage(builder.assets);
    let err = launcher.launch();
    // This log is only reachable if Rocket has an error during startup,
    // otherwise `rocket::ignite().launch()` blocks forever.
    error!("Failed to launch rocket: {}", err);
    Err(Error::FailedLaunch(err.to_string()))
}

impl<'r> Responder<'r> for Error {
    fn respond_to(self, _: &Request) -> response::Result<'r> {
        response::Response::build()
            .status(Status::InternalServerError)
            .sized_body(Cursor::new(format!("{}", self)))
            .ok()
    }
}
