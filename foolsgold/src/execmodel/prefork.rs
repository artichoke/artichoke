use mruby::{sys, Error, Mrb, Ruby, Rust, TryFromMrb, Value};
use ref_thread_local::RefThreadLocal;
use rocket::http::Status;
use rocket::{get, Response};

use crate::execmodel::{exec, Interpreter};
use crate::sources::{foolsgold, rackup};

ref_thread_local! {
    static managed INTERPRETER: Mrb = {
        let interp = mruby::Interpreter::create().expect("mrb interpreter");
        {
            let mut api = interp.borrow_mut();
            api.def_file_for_type::<_, mruby_rack::Builder>("rack/builder");
            api.def_file_for_type::<_, foolsgold::Lib>("foolsgold");
        }
        interp
    };
}

impl Interpreter for &INTERPRETER {
    fn mrb(&self) -> *mut sys::mrb_state {
        let interp = self.borrow();
        let api = interp.borrow();
        api.mrb()
    }

    fn ctx(&self) -> *mut sys::mrbc_context {
        let interp = self.borrow();
        let api = interp.borrow();
        api.ctx()
    }

    fn try_value<T>(&self, value: Value) -> Result<T, Error<Ruby, Rust>>
    where
        T: TryFromMrb<Value, From = Ruby, To = Rust>,
    {
        let interp = self.borrow();
        let api = interp.borrow();
        unsafe { <T>::try_from_mrb(&api, value) }
    }
}

#[get("/fools-gold/prefork")]
pub fn rack_app<'a>() -> Result<Response<'a>, Status> {
    info!("Using prefork thread local mruby interpreter");
    let interp = &INTERPRETER;
    exec(&interp, rackup::rack_adapter())
}
