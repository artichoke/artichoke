//! Convert a [`rocket::Request`] to a
//! [Rack environment](https://www.rubydoc.info/github/rack/rack/file/SPEC#label-The+Environment).
//!
//! Based on
//! [`Rack::Handler::Webrick`](https://github.com/rack/rack/blob/2.0.7/lib/rack/handler/webrick.rb).

use mruby::convert::FromMrb;
use mruby::eval::MrbEval;
use mruby::interpreter::Mrb;
use mruby::value::{Value, ValueLike};

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
        let env = interp.eval("{ Rack::RACK_VERSION => Rack::VERSION }")?;

        if let Some(version) = self.http_version() {
            let key = interp.eval("Rack::HTTP_VERSION")?;
            env.funcall::<(), _, _>("[]=", &[key, Value::from_mrb(interp, version)])?;
        }

        let key = interp.eval("Rack::REQUEST_METHOD")?;
        env.funcall::<(), _, _>(
            "[]=",
            &[key, Value::from_mrb(interp, self.request_method())],
        )?;

        let key = interp.eval("Rack::SCRIPT_NAME")?;
        env.funcall::<(), _, _>("[]=", &[key, Value::from_mrb(interp, self.script_name())])?;

        let key = interp.eval("Rack::PATH_INFO")?;
        env.funcall::<(), _, _>("[]=", &[key, Value::from_mrb(interp, self.path_info())])?;

        let key = interp.eval("Rack::QUERY_STRING")?;
        env.funcall::<(), _, _>("[]=", &[key, Value::from_mrb(interp, self.query_string())])?;

        let key = interp.eval("Rack::SERVER_NAME")?;
        env.funcall::<(), _, _>("[]=", &[key, Value::from_mrb(interp, self.server_name())])?;

        let key = interp.eval("Rack::SERVER_PORT")?;
        env.funcall::<(), _, _>("[]=", &[key, Value::from_mrb(interp, self.server_port())])?;

        let key = interp.eval("Rack::RACK_URL_SCHEME")?;
        env.funcall::<(), _, _>("[]=", &[key, Value::from_mrb(interp, self.url_scheme())])?;

        // TODO: implement Rack IO, see GH-9.
        let key = interp.eval("Rack::RACK_INPUT")?;
        env.funcall::<(), _, _>("[]=", &[key, Value::from_mrb(interp, None::<Value>)])?;
        let key = interp.eval("Rack::RACK_ERRORS")?;
        env.funcall::<(), _, _>("[]=", &[key, Value::from_mrb(interp, None::<Value>)])?;

        let key = interp.eval("Rack::RACK_MULTITHREAD")?;
        env.funcall::<(), _, _>("[]=", &[key, Value::from_mrb(interp, false)])?;
        let key = interp.eval("Rack::RACK_MULTIPROCESS")?;
        env.funcall::<(), _, _>("[]=", &[key, Value::from_mrb(interp, false)])?;
        // TODO: Set RUNONCE based on whether nemesis is in shared nothing or
        // prefork mode.
        let key = interp.eval("Rack::RACK_RUNONCE")?;
        env.funcall::<(), _, _>("[]=", &[key, Value::from_mrb(interp, false)])?;

        Ok(env)
    }
}
