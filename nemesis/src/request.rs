//! Convert a [`rocket::Request`] to a
//! [Rack environment](https://www.rubydoc.info/github/rack/rack/file/SPEC#label-The+Environment).
//!
//! Based on
//! [`Rack::Handler::Webrick`](https://github.com/rack/rack/blob/2.0.7/lib/rack/handler/webrick.rb).

use mruby::convert::FromMrb;
use mruby::value::Value;
use mruby::Mrb;

use crate::Error;

pub trait Request {
    fn origin(&self) -> String;

    fn http_version(&self) -> Option<String>;

    fn request_method(&self) -> String;

    fn script_name(&self) -> String;

    fn path_info(&self) -> String;

    fn query_string(&self) -> String;

    fn server_name(&self) -> String;

    fn server_port(&self) -> u16;

    fn url_scheme(&self) -> String;

    /// Convert a `Request` into a Rack Environment.
    ///
    /// The
    /// [Rack specification](https://www.rubydoc.info/github/rack/rack/file/SPEC#label-The+Environment)
    /// enumerates the required keys. This implementation is based on
    /// [`Rack::Handler::Webrick`](https://github.com/rack/rack/blob/2.0.7/lib/rack/handler/webrick.rb).
    fn to_env(&self, interp: &Mrb) -> Result<Value, Error> {
        let mut env = vec![];
        env.push(("rack.version", Value::from_mrb(interp, vec![1_i64, 3_i64])));
        if let Some(version) = self.http_version() {
            env.push(("HTTP_VERSION", Value::from_mrb(interp, version)));
        }
        env.push((
            "REQUEST_METHOD",
            Value::from_mrb(interp, self.request_method()),
        ));
        env.push(("SCRIPT_NAME", Value::from_mrb(interp, self.script_name())));
        env.push(("PATH_INFO", Value::from_mrb(interp, self.path_info())));
        env.push(("QUERY_STRING", Value::from_mrb(interp, self.query_string())));
        env.push(("SERVER_NAME", Value::from_mrb(interp, self.server_name())));
        env.push((
            "SERVER_PORT",
            Value::from_mrb(interp, self.server_port().to_string()),
        ));
        env.push((
            "rack.url_scheme",
            Value::from_mrb(interp, self.url_scheme()),
        ));

        // TODO: implement Rack IO, see GH-9.
        env.push(("rack.input", Value::from_mrb(interp, None::<Value>)));
        env.push(("rack.errors", Value::from_mrb(interp, None::<Value>)));

        env.push(("rack.multithread", Value::from_mrb(interp, false)));
        env.push(("rack.multiprocess", Value::from_mrb(interp, false)));
        env.push(("rack.run_once", Value::from_mrb(interp, false)));

        Ok(Value::from_mrb(interp, env))
    }
}

#[cfg(test)]
mod tests {
    use mruby::eval::MrbEval;
    use mruby::interpreter::{Interpreter, MrbApi};
    use mruby::value::ValueLike;
    use mruby_gems::rubygems::rack;

    // This module hard codes Rack constant names to avoid retrieving them via
    // the mruby VM. This test ensures that the inlined constants map to the
    // real ones in Ruby source.
    #[test]
    fn rack_constants_match() {
        let interp = Interpreter::create().expect("mrb init");
        rack::init(&interp).unwrap();
        let rack = interp.eval("require 'rack'; Rack").unwrap();
        assert_eq!(
            rack.funcall::<String, _, _>("const_get", &[interp.string("RACK_VERSION")]),
            Ok("rack.version".to_owned())
        );
        assert_eq!(
            rack.funcall::<Vec<i64>, _, _>("const_get", &[interp.string("VERSION")]),
            Ok(vec![1, 3])
        );
        assert_eq!(
            rack.funcall::<String, _, _>("const_get", &[interp.string("HTTP_VERSION")]),
            Ok("HTTP_VERSION".to_owned())
        );
        assert_eq!(
            rack.funcall::<String, _, _>("const_get", &[interp.string("REQUEST_METHOD")]),
            Ok("REQUEST_METHOD".to_owned())
        );
        assert_eq!(
            rack.funcall::<String, _, _>("const_get", &[interp.string("SCRIPT_NAME")]),
            Ok("SCRIPT_NAME".to_owned())
        );
        assert_eq!(
            rack.funcall::<String, _, _>("const_get", &[interp.string("PATH_INFO")]),
            Ok("PATH_INFO".to_owned())
        );
        assert_eq!(
            rack.funcall::<String, _, _>("const_get", &[interp.string("QUERY_STRING")]),
            Ok("QUERY_STRING".to_owned())
        );
        assert_eq!(
            rack.funcall::<String, _, _>("const_get", &[interp.string("SERVER_NAME")]),
            Ok("SERVER_NAME".to_owned())
        );
        assert_eq!(
            rack.funcall::<String, _, _>("const_get", &[interp.string("SERVER_PORT")]),
            Ok("SERVER_PORT".to_owned())
        );
        assert_eq!(
            rack.funcall::<String, _, _>("const_get", &[interp.string("RACK_URL_SCHEME")]),
            Ok("rack.url_scheme".to_owned())
        );
        assert_eq!(
            rack.funcall::<String, _, _>("const_get", &[interp.string("RACK_INPUT")]),
            Ok("rack.input".to_owned())
        );
        assert_eq!(
            rack.funcall::<String, _, _>("const_get", &[interp.string("RACK_ERRORS")]),
            Ok("rack.errors".to_owned())
        );
        assert_eq!(
            rack.funcall::<String, _, _>("const_get", &[interp.string("RACK_MULTITHREAD")]),
            Ok("rack.multithread".to_owned())
        );
        assert_eq!(
            rack.funcall::<String, _, _>("const_get", &[interp.string("RACK_MULTIPROCESS")]),
            Ok("rack.multiprocess".to_owned())
        );
        assert_eq!(
            rack.funcall::<String, _, _>("const_get", &[interp.string("RACK_RUNONCE")]),
            Ok("rack.run_once".to_owned())
        );
    }
}
