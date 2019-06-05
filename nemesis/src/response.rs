//! Convert a
//! [`Nemesis::Response`](https://github.com/lopopolo/ferrocarril/blob/master/nemesis/ruby/lib/nemesis/response.rb)
//! Rack response to a [`rocket::Response`].
//!
//! Based on
//! [`Rack::Response`](https://github.com/rack/rack/blob/2.0.7/lib/rack/response.rb).

use log::warn;
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
            warn!(
                "malformed rack response: {:?}",
                response.iter().map(Value::to_s_debug).collect::<Vec<_>>()
            );
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
            headers: Self::headers(&response)?,
            body: Self::body(&response)?,
        })
    }

    fn status(response: &Value) -> Result<u16, Error> {
        let status = response.funcall::<i64, _, _>("status", &[])?;
        u16::try_from(status).map_err(|_| Error::Status)
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
