use mruby::convert::{Error, TryFromMrb};
use mruby::gc::GarbageCollection;
use mruby::interpreter::{self, Mrb, MrbApi, MrbError};
use mruby::load::MrbLoadSources;
use mruby::value::types::{Ruby, Rust};
use mruby::value::Value;
use ref_thread_local::RefThreadLocal;
use rocket::http::Status;
use rocket::{get, Response};

use crate::execmodel::{exec, Interpreter};
use crate::sources::{foolsgold, rackup};

ref_thread_local! {
    static managed INTERPRETER: Mrb = {
        let mut interp = interpreter::Interpreter::create().expect("mrb interpreter");
        interp.def_file_for_type::<_, mruby_rack::Builder>("rack/builder").expect("def rack");
        interp.def_file_for_type::<_, foolsgold::Lib>("foolsgold").expect("def foolsgold");
        interp
    };
}

impl Interpreter for &INTERPRETER {
    fn eval<T>(&self, code: T) -> Result<Value, MrbError>
    where
        T: AsRef<[u8]>,
    {
        let arena = GarbageCollection::create_arena_savepoint(&*self.borrow());
        let result = MrbApi::eval(&*self.borrow(), code.as_ref());
        arena.restore();
        GarbageCollection::incremental_gc(&*self.borrow());
        result
    }

    fn try_value<T>(&self, value: Value) -> Result<T, Error<Ruby, Rust>>
    where
        T: TryFromMrb<Value, From = Ruby, To = Rust>,
    {
        unsafe { <T>::try_from_mrb(&self.borrow(), value) }
    }
}

#[get("/fools-gold/prefork")]
pub fn rack_app<'a>() -> Result<Response<'a>, Status> {
    info!("Using prefork thread local mruby interpreter");
    let interp = &INTERPRETER;
    exec(&interp, rackup::rack_adapter())
}
