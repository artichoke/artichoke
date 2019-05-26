//! Run a Rack app with an environment derived from the request.

use mruby::eval::MrbEval;
use mruby::interpreter::Mrb;
use mruby::sys;
use mruby::value::Value;
use mruby::MrbError;
use std::convert::AsRef;
use std::error;
use std::fmt;

use crate::request::{self, Request};
use crate::response::{self, Response};

#[derive(Debug)]
pub enum Error {
    Request(request::Error),
    Response(response::Error),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Error::Request(inner) => write!(f, "{}", inner),
            Error::Response(inner) => write!(f, "{}", inner),
        }
    }
}

impl error::Error for Error {
    fn description(&self) -> &str {
        "nemesis rack error"
    }

    fn cause(&self) -> Option<&error::Error> {
        match self {
            Error::Request(inner) => Some(inner),
            Error::Response(inner) => Some(inner),
        }
    }
}

pub fn adapter_from_rackup<T>(interp: &Mrb, source: T) -> Result<Value, MrbError>
where
    T: AsRef<str>,
{
    interp.eval(format!(
        r#"
        Rack::Builder.new do
          {rackup}
        end
        "#,
        rackup = source.as_ref()
    ))
}

pub fn run<'a>(
    interp: &Mrb,
    app: &Value,
    request: &Request,
) -> Result<rocket::Response<'a>, Error> {
    let fun = "call";
    // build env hash that is passed to app.call
    let args = &[request.to_env(interp).map_err(Error::Request)?.inner()];
    let response = unsafe {
        let sym = sys::mrb_intern(interp.borrow().mrb, fun.as_ptr() as *const i8, fun.len());
        // app.call(env)
        sys::mrb_funcall_argv(interp.borrow().mrb, app.inner(), sym, 1, args.as_ptr())
    };
    let response = Response::from(interp, Value::new(interp, response)).map_err(Error::Response)?;
    Ok(response.into_rocket())
}
