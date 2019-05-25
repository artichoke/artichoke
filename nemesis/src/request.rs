//! Convert a [`rocket::Request`] to a
//! [Rack environment](https://www.rubydoc.info/github/rack/rack/file/SPEC#label-The+Environment).
//!
//! Based on
//! [`Rack::Handler::Webrick`](https://github.com/rack/rack/blob/2.0.7/lib/rack/handler/webrick.rb).

use mruby::eval::MrbEval;
use mruby::interpreter::Mrb;
use mruby::value::Value;
use mruby::MrbError;
use rocket::http::uri::Origin;
use rocket::http::Method;
use rocket::request::{self, FromRequest};
use rocket::Outcome;
use std::error;
use std::fmt;

#[derive(Debug)]
pub enum Error {
    Mrb(MrbError),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Error::Mrb(inner) => write!(f, "{}", inner),
        }
    }
}

impl error::Error for Error {
    fn description(&self) -> &str {
        "nemesis rack environment error"
    }

    fn cause(&self) -> Option<&error::Error> {
        match self {
            Error::Mrb(inner) => Some(inner),
        }
    }
}

pub struct Request<'a> {
    method: Method,
    origin: Origin<'a>,
}

impl<'a> Request<'a> {
    pub fn to_env(&self, interp: &Mrb) -> Result<Value, Error> {
        // The keys in the environment hash are required by the Rack spec:
        // https://www.rubydoc.info/github/rack/rack/file/SPEC#label-The+Environment
        interp
            .eval(format!(
                r#"
                {{
                    Rack::REQUEST_METHOD => '{method}',
                    Rack::SCRIPT_NAME => '', # TODO: Rocket mount path
                    Rack::PATH_INFO => '{path}',
                    Rack::QUERY_STRING => '{query}',
                    Rack::SERVER_NAME => 'localhost', # TODO: set this correctly
                    Rack::SERVER_PORT => 8000, # TODO: set this correctly
                    Rack::HTTP_VERSION => '1.1', # TODO: set this correctly
                    Rack::RACK_VERSION => Rack::VERSION,
                    Rack::RACK_URL_SCHEME => 'http', # TODO: set this correctly
                    Rack::RACK_INPUT => nil, # TODO: implement IO
                    Rack::RACK_ERRORS => nil, # TODO: implement IO
                    Rack::RACK_MULTITHREAD => false,
                    Rack::RACK_MULTIPROCESS => false,
                    Rack::RACK_RUNONCE => false,
                }}
                "#,
                method = self.method,
                path = self.origin.path(),
                query = self.origin.query().unwrap_or_else(|| "")
            ))
            .map_err(Error::Mrb)
    }
}

impl<'a, 'r> FromRequest<'a, 'r> for Request<'a> {
    type Error = ();

    fn from_request(request: &'a request::Request<'r>) -> request::Outcome<Self, Self::Error> {
        Outcome::Success(Request {
            method: request.method(),
            origin: request.uri().clone(),
        })
    }
}
