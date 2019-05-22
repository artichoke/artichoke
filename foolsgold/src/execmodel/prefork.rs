use mruby::gc::GarbageCollection;
use mruby::interpreter::Mrb;
use mruby::MrbError;
use nemesis::handler::RackRequest;
use nemesis::{self, handler};
use ref_thread_local::RefThreadLocal;
use rocket::{get, Response};

use crate::execmodel::Error;
use crate::foolsgold::RACKUP;

ref_thread_local! {
    static managed INTERPRETER: Result<Mrb, MrbError> = super::interpreter();
}

#[get("/fools-gold/prefork")]
#[allow(clippy::needless_pass_by_value)]
pub fn rack_app<'a>(req: RackRequest) -> Result<Response<'a>, Error> {
    info!("Using prefork thread local mruby interpreter");
    match *INTERPRETER.borrow() {
        Ok(ref interp) => {
            let adapter = handler::adapter_from_rackup(interp, RACKUP)?;
            let arena = interp.create_arena_savepoint();
            let response = handler::run(interp, &adapter, &req)?;
            arena.restore();
            interp.incremental_gc();
            Ok(response)
        }
        Err(_) => Err(Error::from(MrbError::New)),
    }
}
