use mruby::eval::MrbEval;
use mruby::interpreter::{Interpreter, Mrb};
use mruby::MrbError;
use nemesis::handler::ResponseError;
use rocket::http::Status;
use rocket::request::Request;
use rocket::response::{self, Responder, Response};
use std::io::Cursor;

use crate::foolsgold;

pub mod prefork;
pub mod shared_nothing;

fn interpreter() -> Result<Mrb, MrbError> {
    let interp = Interpreter::create()?;
    nemesis::init(&interp)?;
    foolsgold::init(&interp)?;
    // preload foolsgold sources
    interp.eval("require 'foolsgold'")?;
    Ok(interp)
}

#[derive(Debug)]
pub struct Error(ResponseError);

impl From<MrbError> for Error {
    fn from(error: MrbError) -> Self {
        Self(ResponseError::Mrb(error))
    }
}

impl From<ResponseError> for Error {
    fn from(error: ResponseError) -> Self {
        Self(error)
    }
}

impl<'r> Responder<'r> for Error {
    fn respond_to(self, _: &Request) -> response::Result<'r> {
        Response::build()
            .status(Status::InternalServerError)
            .sized_body(Cursor::new(format!("{}", self.0)))
            .ok()
    }
}
