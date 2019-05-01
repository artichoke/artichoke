use mruby::{sys, Error, Mrb, Ruby, Rust, TryFromMrb, Value};
use rocket::http::Status;
use rocket::{get, Response};

use crate::execmodel::{exec, Interpreter};
use crate::sources::{foolsgold, rackup};

impl Interpreter for Mrb {
    fn mrb(&self) -> *mut sys::mrb_state {
        self.borrow().mrb()
    }

    fn ctx(&self) -> *mut sys::mrbc_context {
        self.borrow().ctx()
    }

    fn try_value<T>(&self, value: Value) -> Result<T, Error<Ruby, Rust>>
    where
        T: TryFromMrb<Value, From = Ruby, To = Rust>,
    {
        unsafe { <T>::try_from_mrb(&self.borrow(), value) }
    }
}

#[get("/fools-gold")]
pub fn rack_app<'a>() -> Result<Response<'a>, Status> {
    info!("Initializing fresh shared nothing mruby interpreter");
    let interp = mruby::Interpreter::create().map_err(|_| Status::InternalServerError)?;
    {
        let mut api = interp.borrow_mut();
        api.def_file_for_type::<_, mruby_rack::Builder>("rack/builder");
        api.def_file_for_type::<_, foolsgold::Lib>("foolsgold");
    }
    exec(&interp, rackup::rack_adapter())
}
