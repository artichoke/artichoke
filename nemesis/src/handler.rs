//! Run a Rack app with an environment derived from the request.

use mruby::gc::GarbageCollection;
use mruby::interpreter::Mrb;

use crate::adapter::RackApp;
use crate::request::Request;
use crate::response::Response;
use crate::Error;

pub fn run<'a, T: Request>(
    interp: &Mrb,
    app: &RackApp,
    request: &T,
) -> Result<rocket::Response<'a>, Error> {
    let _arena = interp.create_arena_savepoint();
    app.call(request).map(Response::into_rocket)
}
