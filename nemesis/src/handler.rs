//! Run a Rack app with an environment derived from the request.

use mruby::gc::GarbageCollection;
use mruby::interpreter::Mrb;
use mruby::value::{Value, ValueLike};
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

impl From<request::Error> for Error {
    fn from(error: request::Error) -> Self {
        Error::Request(error)
    }
}

impl From<response::Error> for Error {
    fn from(error: response::Error) -> Self {
        Error::Response(error)
    }
}

pub fn run<'a>(
    interp: &Mrb,
    app: &Value,
    request: &Request,
) -> Result<rocket::Response<'a>, Error> {
    let _arena = interp.create_arena_savepoint();
    let args = &[request.to_env(interp)?];
    let response = app
        .funcall::<Vec<Value>, _, _>("call", args)
        .map_err(response::Error::Mrb)?;
    let response = Response::from(interp, response)?;
    Ok(response.into_rocket())
}
