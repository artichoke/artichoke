//! Nemesis server implementations.

use mruby::gc::GarbageCollection;
use rocket::handler;
use rocket::http::{ContentType, Method, Status};
use rocket::request::FromRequest;
use rocket::{self, Data, Handler, Outcome, Route, State};
use std::ffi::OsStr;
use std::io::Cursor;
use std::path::Path;

use crate::request::Request;
use crate::response::Response;
use crate::server::rocket::request;
use crate::server::{AssetRegistry, Mount};
use crate::Error;

#[get("/")]
#[allow(clippy::needless_pass_by_value)]
pub fn static_asset<'a>(
    req: request::Request,
    assets: State<AssetRegistry>,
) -> Result<rocket::Response<'a>, Status> {
    let content_type = Path::new(&req.origin())
        .extension()
        .and_then(OsStr::to_str)
        .and_then(ContentType::from_extension);
    let content = assets
        .0
        .get(&req.origin())
        .map(Clone::clone)
        .ok_or(Status::NotFound)?;
    if let Some(content_type) = content_type {
        rocket::Response::build()
            .sized_body(Cursor::new(content))
            .header(content_type)
            .ok()
    } else {
        rocket::Response::build()
            .sized_body(Cursor::new(content))
            .ok()
    }
}

#[derive(Clone)]
pub struct RackHandler {
    mount: Mount,
}

impl RackHandler {
    fn new(mount: &Mount) -> Self {
        Self {
            mount: mount.clone(),
        }
    }

    pub fn routes(mount: &Mount) -> Vec<Route> {
        vec![
            Route::new(Method::Get, "/", Self::new(mount)),
            Route::new(Method::Get, "/<path..>", Self::new(mount)),
            Route::new(Method::Put, "/", Self::new(mount)),
            Route::new(Method::Put, "/<path..>", Self::new(mount)),
            Route::new(Method::Post, "/", Self::new(mount)),
            Route::new(Method::Post, "/<path..>", Self::new(mount)),
            Route::new(Method::Delete, "/", Self::new(mount)),
            Route::new(Method::Delete, "/<path..>", Self::new(mount)),
            Route::new(Method::Options, "/", Self::new(mount)),
            Route::new(Method::Options, "/<path..>", Self::new(mount)),
            Route::new(Method::Head, "/", Self::new(mount)),
            Route::new(Method::Head, "/<path..>", Self::new(mount)),
            Route::new(Method::Trace, "/", Self::new(mount)),
            Route::new(Method::Trace, "/<path..>", Self::new(mount)),
            Route::new(Method::Connect, "/", Self::new(mount)),
            Route::new(Method::Connect, "/<path..>", Self::new(mount)),
            Route::new(Method::Patch, "/", Self::new(mount)),
            Route::new(Method::Patch, "/<path..>", Self::new(mount)),
        ]
    }
}

impl Handler for RackHandler {
    fn handle<'r>(&self, req: &'r rocket::Request, _: Data) -> handler::Outcome<'r> {
        match request::Request::from_request(req) {
            Outcome::Success(nemreq) => Outcome::from(req, app(&nemreq, &self.mount)),
            _ => Outcome::failure(Status::InternalServerError),
        }
    }
}

pub fn app<'a>(req: &request::Request, mount: &Mount) -> Result<rocket::Response<'a>, Error> {
    let interp = mount.exec_mode.interpreter(&mount.interp_init)?;
    let _arena = interp.create_arena_savepoint();
    let lock = mount.app.lock().map_err(|_| Error::CannotCreateApp)?;
    let app = lock(&interp)?;
    debug!(
        "Matched Rack adapter: app={} base={} route={}",
        app.name(),
        req.script_name(),
        req.path_info()
    );
    let response = app.call(req).map(Response::into_rocket)?;
    mount.exec_mode.gc(&interp);
    Ok(response)
}
