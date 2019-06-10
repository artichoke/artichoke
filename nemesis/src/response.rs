//! Convert a
//! [`Nemesis::Response`](https://github.com/lopopolo/ferrocarril/blob/master/nemesis/ruby/lib/nemesis/response.rb)
//! Rack response to a [`rocket::Response`].
//!
//! Based on
//! [`Rack::Response`](https://github.com/rack/rack/blob/2.0.7/lib/rack/response.rb).

use log::warn;
use mruby::convert::FromMrb;
use mruby::interpreter::Mrb;
use mruby::value::{Value, ValueLike};
use mruby::MrbError;
use std::collections::HashMap;
use std::convert::{self, TryFrom};

use crate::nemesis;
use crate::Error;

#[derive(Debug)]
pub struct Response {
    pub status: u16,
    pub headers: HashMap<String, String>,
    pub body: Vec<u8>,
}

impl Response {
    const RACK_RESPONSE_TUPLE_LEN: usize = 3;

    /// Convert from a Rack `[status, headers, body]` response tuple to a Rust
    /// representation. This code converts a response tuple using the Ruby class
    /// `Nemesis::Response`.
    pub fn from_rack_tuple(interp: &Mrb, response: Vec<Value>) -> Result<Self, Error> {
        if response.len() != Self::RACK_RESPONSE_TUPLE_LEN {
            warn!("malformed rack response: {:?}", response);
            return Err(Error::RackResponse);
        }
        let class = interp
            .borrow()
            .class_spec::<nemesis::Response>()
            .and_then(|spec| spec.borrow().value(interp))
            .ok_or_else(|| Error::Mrb(MrbError::NotDefined("Nemesis::Response".to_owned())))?;
        let response = class.funcall::<Value, _, _>("new", response)?;
        Ok(Self {
            status: Self::status(&response)?,
            headers: Self::headers(&response, interp)?,
            body: Self::body(&response, interp)?,
        })
    }

    fn status(response: &Value) -> Result<u16, Error> {
        let status = response.funcall::<i64, _, _>("status", &[])?;
        u16::try_from(status).map_err(|_| Error::Status)
    }

    fn headers(response: &Value, interp: &Mrb) -> Result<HashMap<String, String>, Error> {
        let itself = Value::from_mrb(interp, "itself")
            .funcall::<Value, _, _>("to_sym", &[])?
            .funcall::<Value, _, _>("to_proc", &[])?;
        // The header must respond to `each`, and yield values of key and value.
        let headers = response
            .funcall::<Value, _, _>("headers", &[])?
            .funcall::<Value, _, _>("each", &[])?
            .funcall_with_block::<Value, _, _>("map", &[], itself)?
            .funcall::<HashMap<String, String>, _, _>("to_h", &[])?;

        let headers = headers
            .into_iter()
            // Special headers starting “rack.” are for communicating with
            // the server, and must not be sent back to the client.
            .filter(|(k, _v)| !k.starts_with("rack."))
            .collect::<HashMap<_, _>>();
        Ok(headers)
    }

    fn body(response: &Value, interp: &Mrb) -> Result<Vec<u8>, Error> {
        let itself = Value::from_mrb(interp, "itself")
            .funcall::<Value, _, _>("to_sym", &[])?
            .funcall::<Value, _, _>("to_proc", &[])?;
        // The Body must respond to each and must only yield String values. The
        // Body itself should not be an instance of String, as this will break
        // in Ruby 1.9.
        let body = response.funcall::<Value, _, _>("body", &[])?;
        let parts = body
            .funcall::<Value, _, _>("each", &[])?
            .funcall_with_block::<Vec<Vec<u8>>, _, _>("map", &[], itself)?;
        let bytes = parts
            .into_iter()
            .flat_map(convert::identity)
            .collect::<Vec<_>>();
        // If the Body responds to `close`, it will be called after iteration.
        // If the body is replaced by a middleware after action, the original
        // body must be closed first, if it responds to close.
        if body.respond_to("close")? {
            body.funcall::<(), _, _>("close", &[])?;
        }
        Ok(bytes)
    }
}
