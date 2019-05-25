//! Convert a
//! [`Nemesis::Response`](https://github.com/lopopolo/ferrocarril/blob/master/nemesis/ruby/lib/nemesis/response.rb)
//! Rack response to a [`rocket::Response`].
//!
//! Based on
//! [`Rack::Response`](https://github.com/rack/rack/blob/2.0.7/lib/rack/response.rb).

use log::warn;
use mruby::class;
use mruby::convert::TryFromMrb;
use mruby::def::{ClassLike, Parent};
use mruby::interpreter::Mrb;
use mruby::module;
use mruby::sys;
use mruby::value::Value;
use mruby::MrbError;
use rocket::http::Status;
use std::cell::RefCell;
use std::collections::HashMap;
use std::convert::TryFrom;
use std::error;
use std::fmt;
use std::rc::Rc;

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

#[derive(Debug)]
struct Response {
    status: Status,
    headers: HashMap<String, String>,
    body: Vec<u8>,
}

impl Response {
    const RACK_RESPONSE_TUPLE_LEN: usize = 3;

    /// Convert from a Rack `[status, headers, body]` response tuple to a Rust
    /// representation. This code converts a response tuple using the Ruby class
    /// `Nemesis::Response`.
    pub fn from(interp: &Mrb, value: Value) -> Result<Self, Error> {
        let response = unsafe { <Vec<Value>>::try_from_mrb(interp, value) }
            .map_err(MrbError::ConvertToRust)
            .map_err(Error::Mrb)?;
        if response.len() != Self::RACK_RESPONSE_TUPLE_LEN {
            warn!(
                "malformed rack response: {:?}",
                response.iter().map(Value::to_s_debug).collect::<Vec<_>>()
            );
            return Err(Error::RackResponse);
        }
        let nemesis = module::Spec::new("Nemesis", None);
        let parent = Parent::Module {
            spec: Rc::new(RefCell::new(nemesis)),
        };
        let class = class::Spec::new("Response", Some(parent), None);
        let rclass = class
            .rclass(Rc::clone(interp))
            .ok_or_else(|| Error::Mrb(MrbError::NotDefined(class.fqname())))?;
        let args = response.iter().map(Value::inner).collect::<Vec<_>>();
        // Nemesis::Response.new(status, headers, body)
        let response = unsafe { sys::mrb_obj_new(interp.borrow().mrb, rclass, 3, args.as_ptr()) };
        let response = Value::new(interp, response);
        Ok(Self {
            status: Self::status(interp, response)?,
            headers: Self::headers(interp, response)?,
            body: Self::body(interp, response)?,
        })
    }

    fn status(interp: &Mrb, response: Value) -> Result<Status, Error> {
        let accessor = "status";
        let args = &[];
        let status = unsafe {
            let sym = sys::mrb_intern(
                interp.borrow().mrb,
                accessor.as_ptr() as *const i8,
                accessor.len(),
            );
            // response.status
            let value =
                sys::mrb_funcall_argv(interp.borrow().mrb, response.inner(), sym, 0, args.as_ptr());
            Value::new(interp, value)
        };
        let status = unsafe { i64::try_from_mrb(interp, status) }
            .map_err(MrbError::ConvertToRust)
            .map_err(Error::Mrb)?;
        let status = u16::try_from(status).map_err(|_| Error::Status)?;
        Status::from_code(status).ok_or(Error::Status)
    }

    fn headers(interp: &Mrb, response: Value) -> Result<HashMap<String, String>, Error> {
        let accessor = "headers";
        let args = &[];
        let headers = unsafe {
            let sym = sys::mrb_intern(
                interp.borrow().mrb,
                accessor.as_ptr() as *const i8,
                accessor.len(),
            );
            // response.headers
            let value =
                sys::mrb_funcall_argv(interp.borrow().mrb, response.inner(), sym, 0, args.as_ptr());
            Value::new(interp, value)
        };
        let pairs = unsafe { <Vec<(Value, Value)>>::try_from_mrb(interp, headers) }
            .map_err(MrbError::ConvertToRust)
            .map_err(Error::Mrb)?;
        let mut headers = HashMap::new();
        for (key, value) in pairs {
            let key = unsafe { String::try_from_mrb(&interp, key) }
                .map_err(MrbError::ConvertToRust)
                .map_err(Error::Mrb)?;
            let value = unsafe { String::try_from_mrb(&interp, value) }
                .map_err(MrbError::ConvertToRust)
                .map_err(Error::Mrb)?;
            headers.insert(key, value);
        }
        Ok(headers)
    }

    fn body(interp: &Mrb, response: Value) -> Result<Vec<u8>, Error> {
        let accessor = "body_bytes";
        let args = &[];
        let body = unsafe {
            let sym = sys::mrb_intern(
                interp.borrow().mrb,
                accessor.as_ptr() as *const i8,
                accessor.len(),
            );
            // response.body_bytes
            let value =
                sys::mrb_funcall_argv(interp.borrow().mrb, response.inner(), sym, 0, args.as_ptr());
            Value::new(interp, value)
        };
        unsafe { <Vec<u8>>::try_from_mrb(interp, body) }
            .map_err(MrbError::ConvertToRust)
            .map_err(Error::Mrb)
    }
}
