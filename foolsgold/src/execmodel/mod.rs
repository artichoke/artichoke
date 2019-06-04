use mruby::eval::MrbEval;
use mruby::interpreter::{Interpreter, Mrb};
use mruby::MrbError;
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
pub enum Error {
    Mrb(MrbError),
    Nemesis(nemesis::Error),
}

impl From<MrbError> for Error {
    fn from(error: MrbError) -> Self {
        Error::Mrb(error)
    }
}

impl From<nemesis::Error> for Error {
    fn from(error: nemesis::Error) -> Self {
        Error::Nemesis(error)
    }
}

impl<'r> Responder<'r> for Error {
    fn respond_to(self, _: &Request) -> response::Result<'r> {
        match self {
            Error::Nemesis(inner) => Response::build()
                .status(Status::InternalServerError)
                .sized_body(Cursor::new(format!("{}", inner)))
                .ok(),
            Error::Mrb(inner) => Response::build()
                .status(Status::InternalServerError)
                .sized_body(Cursor::new(format!("{}", inner)))
                .ok(),
        }
    }
}
