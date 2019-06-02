use mruby::convert::FromMrb;
use mruby::eval::MrbEval;
use mruby::interpreter::Mrb;
use mruby::sys;
use mruby::value::{Value, ValueLike};
use mruby::MrbError;
use std::convert::AsRef;
use std::rc::Rc;

use crate::handler;
use crate::request::Request;
use crate::response::{self, Response};

pub struct RackApp {
    interp: Mrb,
    app: Value,
}

impl RackApp {
    /// Create a Rack app by wrapping the supplied rackup source in a
    /// `Rack::Builder`. The returned [`Value`] has a call method and is
    /// suitable for passing to [`handler::run`](crate::handler::run).
    pub fn from_rackup<T: AsRef<str>>(
        interp: &Mrb,
        builder_script: T,
    ) -> Result<RackApp, MrbError> {
        let builder = interp.eval("Rack::Builder")?;
        let app = builder.funcall::<Value, _, _>(
            "new_from_string",
            &[Value::from_mrb(interp, builder_script.as_ref())],
        )?;
        Ok(Self {
            interp: Rc::clone(interp),
            app,
        })
    }

    pub fn call<T: Request>(&self, req: &T) -> Result<Response, handler::Error> {
        let env = req.to_env(&self.interp)?;
        let response = self
            .funcall::<Vec<Value>, _, _>("call", &[env])
            .map_err(response::Error::Mrb)?;
        let response = Response::from(&self.interp, response)?;
        Ok(response)
    }
}

impl ValueLike for RackApp {
    fn inner(&self) -> sys::mrb_value {
        self.app.inner()
    }

    fn interp(&self) -> &Mrb {
        &self.interp
    }
}
