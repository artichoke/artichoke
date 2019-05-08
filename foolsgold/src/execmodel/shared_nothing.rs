use mruby::convert::{Error, TryFromMrb};
use mruby::interpreter::{self, Mrb, MrbApi, MrbError};
use mruby::value::types::{Ruby, Rust};
use mruby::value::Value;
use rocket::http::Status;
use rocket::{get, Response};

use crate::execmodel::{exec, Interpreter};
use crate::sources::{foolsgold, rackup};

impl Interpreter for Mrb {
    fn eval<T>(&self, code: T) -> Result<Value, MrbError>
    where
        T: AsRef<[u8]>,
    {
        MrbApi::eval(self, code.as_ref())
    }

    fn try_value<T>(&self, value: Value) -> Result<T, Error<Ruby, Rust>>
    where
        T: TryFromMrb<Value, From = Ruby, To = Rust>,
    {
        unsafe { <T>::try_from_mrb(self, value) }
    }
}

#[get("/fools-gold")]
pub fn rack_app<'a>() -> Result<Response<'a>, Status> {
    info!("Initializing fresh shared nothing mruby interpreter");
    let mut interp = interpreter::Interpreter::create().map_err(|_| Status::InternalServerError)?;
    interp.def_file_for_type::<_, mruby_rack::Builder>("rack/builder");
    interp.def_file_for_type::<_, foolsgold::Lib>("foolsgold");
    exec(&interp, rackup::rack_adapter())
}
