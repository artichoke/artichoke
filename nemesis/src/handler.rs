//! A Rack handler that glues together a [`rocket::Request`] and a Rack app.
//!
//! Based on `Rack::Handler::Webrick`:
//! <https://github.com/rack/rack/blob/2.0.7/lib/rack/handler/cgi.rb>

use mruby::class;
use mruby::convert::TryFromMrb;
use mruby::def::{ClassLike, Parent};
use mruby::eval::MrbEval;
use mruby::interpreter::Mrb;
use mruby::module;
use mruby::sys;
use mruby::value::Value;
use mruby::MrbError;
use rocket::http::uri::Origin;
use rocket::http::Status;
use rocket::request::{self, FromRequest, Request};
use rocket::{Outcome, Response};
use std::cell::RefCell;
use std::collections::HashMap;
use std::convert::{AsRef, TryFrom};
use std::error;
use std::fmt;
use std::io::Cursor;
use std::rc::Rc;

pub struct RackRequest<'a> {
    origin: Origin<'a>,
}

impl<'a, 'r> FromRequest<'a, 'r> for RackRequest<'a> {
    type Error = ();

    fn from_request(request: &'a Request<'r>) -> request::Outcome<Self, Self::Error> {
        Outcome::Success(RackRequest {
            origin: request.uri().clone(),
        })
    }
}

#[derive(Debug)]
struct RackResponse {
    status: Status,
    headers: HashMap<String, String>,
    body: Vec<u8>,
}

#[derive(Debug)]
pub enum ResponseError {
    InvalidStatus,
    Mrb(MrbError),
    RackResponseNot3Tuple,
}

impl fmt::Display for ResponseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ResponseError::InvalidStatus => write!(f, "status is not a valid HTTP status code"),
            ResponseError::Mrb(inner) => write!(f, "{}", inner),
            ResponseError::RackResponseNot3Tuple => write!(f, "malformed Rack response tuple"),
        }
    }
}

impl error::Error for ResponseError {
    fn description(&self) -> &str {
        "nemesis rack error"
    }

    fn cause(&self) -> Option<&error::Error> {
        match self {
            ResponseError::Mrb(inner) => Some(inner),
            _ => None,
        }
    }
}

impl RackResponse {
    fn from(interp: &Mrb, value: Value) -> Result<Self, ResponseError> {
        let response = unsafe { <Vec<Value>>::try_from_mrb(interp, value) }
            .map_err(|_| ResponseError::RackResponseNot3Tuple)?;
        if response.len() != 3 {
            return Err(ResponseError::RackResponseNot3Tuple);
        }
        let nemesis = module::Spec::new("Nemesis", None);
        let parent = Parent::Module {
            spec: Rc::new(RefCell::new(nemesis)),
        };
        let class = class::Spec::new("Response", Some(parent), None);
        let rclass = class
            .rclass(Rc::clone(interp))
            .ok_or_else(|| ResponseError::Mrb(MrbError::NotDefined(class.fqname())))?;
        let args = response.iter().map(Value::inner).collect::<Vec<_>>();
        // Nemesis::Response.new(status, headers, body)
        let response = unsafe { sys::mrb_obj_new(interp.borrow().mrb, rclass, 3, args.as_ptr()) };
        let status = unsafe {
            let accessor = "status";
            let sym = sys::mrb_intern(
                interp.borrow().mrb,
                accessor.as_ptr() as *const i8,
                accessor.len(),
            );
            let args = &[];
            // response.status
            let value = sys::mrb_funcall_argv(interp.borrow().mrb, response, sym, 0, args.as_ptr());
            Value::new(Rc::clone(interp), value)
        };
        let status = unsafe { i64::try_from_mrb(interp, status) }
            .map_err(MrbError::ConvertToRust)
            .map_err(ResponseError::Mrb)?;
        let status = u16::try_from(status).map_err(|_| ResponseError::InvalidStatus)?;
        let status = Status::from_code(status).ok_or_else(|| ResponseError::InvalidStatus)?;
        let headers = unsafe {
            let accessor = "headers";
            let sym = sys::mrb_intern(
                interp.borrow().mrb,
                accessor.as_ptr() as *const i8,
                accessor.len(),
            );
            let args = &[];
            // response.headers
            let value = sys::mrb_funcall_argv(interp.borrow().mrb, response, sym, 0, args.as_ptr());
            Value::new(Rc::clone(interp), value)
        };
        let header_pairs = unsafe { <Vec<(Value, Value)>>::try_from_mrb(interp, headers) }
            .map_err(MrbError::ConvertToRust)
            .map_err(ResponseError::Mrb)?;
        let mut headers = HashMap::new();
        for (key, value) in header_pairs {
            let key = unsafe { String::try_from_mrb(&interp, key) }
                .map_err(MrbError::ConvertToRust)
                .map_err(ResponseError::Mrb)?;
            let value = unsafe { String::try_from_mrb(&interp, value) }
                .map_err(MrbError::ConvertToRust)
                .map_err(ResponseError::Mrb)?;
            headers.insert(key, value);
        }
        let body = unsafe {
            let accessor = "body_bytes";
            let sym = sys::mrb_intern(
                interp.borrow().mrb,
                accessor.as_ptr() as *const i8,
                accessor.len(),
            );
            let args = &[];
            // response.body_bytes
            let value = sys::mrb_funcall_argv(interp.borrow().mrb, response, sym, 0, args.as_ptr());
            Value::new(Rc::clone(interp), value)
        };
        let body = unsafe { <Vec<u8>>::try_from_mrb(interp, body) }
            .map_err(MrbError::ConvertToRust)
            .map_err(ResponseError::Mrb)?;
        Ok(Self {
            status,
            headers,
            body,
        })
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
    request: &RackRequest,
) -> Result<Response<'a>, ResponseError> {
    // build env hash that is passed to app.call
    let env = interp
        .eval(format!(
            r#"
            {{
                Rack::RACK_VERSION => Rack::VERSION,
                Rack::RACK_INPUT => nil,
                Rack::RACK_ERRORS => nil,
                Rack::RACK_MULTITHREAD => false,
                Rack::RACK_MULTIPROCESS => true,
                Rack::RACK_RUNONCE => false,
                Rack::RACK_URL_SCHEME => 'http', # TODO: set this correctly
                Rack::HTTP_VERSION => '1.1', # TODO: set this correctly
                Rack::PATH_INFO => '',
                Rack::SCRIPT_NAME => '{script}',
                Rack::QUERY_STRING => '{query}',
            }}
            "#,
            script = request.origin.path(),
            query = request.origin.query().unwrap_or_else(|| "")
        ))
        .map_err(ResponseError::Mrb)?;
    let args = &[env.inner()];
    let response = unsafe {
        let call_str = "call";
        let call_sym = sys::mrb_intern(
            interp.borrow().mrb,
            call_str.as_ptr() as *const i8,
            call_str.len(),
        );
        // app.call(env)
        sys::mrb_funcall_argv(interp.borrow().mrb, app.inner(), call_sym, 1, args.as_ptr())
    };
    let response = RackResponse::from(interp, Value::new(Rc::clone(interp), response))?;
    let mut build = Response::build();
    build.status(response.status);
    build.sized_body(Cursor::new(response.body));
    for (key, value) in &response.headers {
        build.raw_header(key.clone(), value.clone());
    }
    Ok(build.finalize())
}
