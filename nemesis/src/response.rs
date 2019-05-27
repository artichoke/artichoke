//! Convert a
//! [`Nemesis::Response`](https://github.com/lopopolo/ferrocarril/blob/master/nemesis/ruby/lib/nemesis/response.rb)
//! Rack response to a [`rocket::Response`].
//!
//! Based on
//! [`Rack::Response`](https://github.com/rack/rack/blob/2.0.7/lib/rack/response.rb).

use log::warn;
use mruby::def::ClassLike;
use mruby::interpreter::Mrb;
use mruby::sys;
use mruby::value::{Value, ValueLike};
use mruby::MrbError;
use rocket::http::Status;
use std::collections::HashMap;
use std::convert::{self, TryFrom};
use std::error;
use std::fmt;
use std::io::Cursor;

use crate::nemesis;

#[derive(Debug)]
pub enum Error {
    Mrb(MrbError),
    RackResponse,
    Status,
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Error::Mrb(inner) => write!(f, "{}", inner),
            Error::RackResponse => write!(f, "malformed Rack response tuple"),
            Error::Status => write!(f, "status is not a valid HTTP status code"),
        }
    }
}

impl error::Error for Error {
    fn description(&self) -> &str {
        "nemesis rack response error"
    }

    fn cause(&self) -> Option<&error::Error> {
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

#[derive(Debug)]
pub struct Response {
    status: Status,
    headers: HashMap<String, String>,
    body: Vec<u8>,
}

impl Response {
    const RACK_RESPONSE_TUPLE_LEN: usize = 3;

    pub fn into_rocket<'a>(self) -> rocket::Response<'a> {
        let mut response = rocket::Response::build();
        response.status(self.status);
        response.sized_body(Cursor::new(self.body));
        for (key, value) in self.headers {
            response.raw_header(key, value);
        }
        response.finalize()
    }

    /// Convert from a Rack `[status, headers, body]` response tuple to a Rust
    /// representation. This code converts a response tuple using the Ruby class
    /// `Nemesis::Response`.
    pub fn from(interp: &Mrb, response: Vec<Value>) -> Result<Self, Error> {
        if response.len() != Self::RACK_RESPONSE_TUPLE_LEN {
            warn!(
                "malformed rack response: {:?}",
                response.iter().map(Value::to_s_debug).collect::<Vec<_>>()
            );
            return Err(Error::RackResponse);
        }
        let classptr = interp
            .borrow()
            .class_spec::<nemesis::Response>()
            .and_then(|spec| spec.borrow().rclass(interp))
            .ok_or_else(|| Error::Mrb(MrbError::NotDefined("Nemesis::Response".to_owned())))?;
        let class = unsafe { sys::mrb_sys_class_value(classptr) };
        let class = Value::new(interp, class);
        let response = class.funcall::<Value, _, _>("new", response)?;
        Ok(Self {
            status: Self::status(&response)?,
            headers: Self::headers(&response)?,
            body: Self::body(&response)?,
        })
    }

    fn status(response: &Value) -> Result<Status, Error> {
        let status = response.funcall::<i64, _, _>("status", &[])?;
        let status = u16::try_from(status).map_err(|_| Error::Status)?;
        Status::from_code(status).ok_or(Error::Status)
    }

    fn headers(response: &Value) -> Result<HashMap<String, String>, Error> {
        let headers = response.funcall::<HashMap<String, String>, _, _>("headers", &[])?;
        Ok(headers)
    }

    fn body(response: &Value) -> Result<Vec<u8>, Error> {
        let parts = response.funcall::<Vec<Vec<u8>>, _, _>("body", &[])?;
        let bytes = parts
            .into_iter()
            .flat_map(convert::identity)
            .collect::<Vec<_>>();
        Ok(bytes)
    }
}
