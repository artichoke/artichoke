#![feature(proc_macro_hygiene, decl_macro)]
#![deny(warnings, intra_doc_link_resolution_failure)]
#![deny(clippy::all, clippy::pedantic)]
#![forbid(unsafe_code)]

#[macro_use]
extern crate log;
#[macro_use]
extern crate rocket;
#[macro_use]
extern crate rust_embed;

use mruby::eval::MrbEval;
use mruby::interpreter::Mrb;
use mruby::MrbError;
use mruby_gems::rubygems::rack;
use std::error;
use std::fmt;

pub mod adapter;
pub mod handler;
pub mod interpreter;
pub mod request;
pub mod response;
mod rubygems;
pub mod server;

use rubygems::nemesis;

#[derive(Debug)]
pub enum Error {
    CannotCreateApp,
    FailedLaunch(String),
    Mrb(MrbError),
    NoRoute,
    RackResponse,
    Status,
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Error::CannotCreateApp => write!(f, "cannot create Rack app"),
            Error::FailedLaunch(err) => write!(f, "failed to launch server: {}", err),
            Error::Mrb(err) => write!(f, "{}", err),
            Error::NoRoute => write!(f, "no matching route"),
            Error::RackResponse => write!(f, "malformed Rack response tuple"),
            Error::Status => write!(f, "status is not a valid HTTP status code"),
        }
    }
}

impl error::Error for Error {
    fn description(&self) -> &str {
        "nemesis error"
    }

    fn cause(&self) -> Option<&dyn error::Error> {
        match self {
            Error::Mrb(inner) => Some(inner),
            _ => None,
        }
    }
}

impl From<MrbError> for Error {
    fn from(error: MrbError) -> Self {
        Error::Mrb(error)
    }
}

pub fn init(interp: &Mrb) -> Result<(), MrbError> {
    rack::init(interp)?;
    nemesis::init(interp)?;
    // Preload required gem sources
    interp.eval("require 'rack'")?;
    interp.eval("require 'nemesis'")?;
    interp.eval("require 'nemesis/response'")?;
    Ok(())
}
