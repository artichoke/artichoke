use mruby::gc::GarbageCollection;
use mruby::interpreter::Mrb;
use mruby::MrbError;
use nemesis::adapter::RackApp;
use nemesis::server::rocket::request::Request;
use nemesis::{self, handler, Error};
use ref_thread_local::RefThreadLocal;
use rocket::{get, Response};

use crate::foolsgold::RACKUP;

ref_thread_local! {
    static managed INTERPRETER: Result<Mrb, MrbError> = super::interpreter();
}

#[get("/fools-gold/prefork")]
#[allow(clippy::needless_pass_by_value)]
pub fn rack_app<'a>(req: Request) -> Result<Response<'a>, Error> {
    info!("Using prefork thread local mruby interpreter");
    match *INTERPRETER.borrow() {
        Ok(ref interp) => {
            let arena = interp.create_arena_savepoint();
            let adapter = RackApp::from_rackup(&interp, RACKUP)?;
            let response = handler::run(interp, &adapter, &req)?;
            arena.restore();
            interp.incremental_gc();
            Ok(response)
        }
        Err(_) => Err(Error::from(MrbError::New)),
    }
}
