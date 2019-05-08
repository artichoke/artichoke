use mruby::convert::{Error, TryFromMrb};
use mruby::interpreter::MrbError;
use mruby::value::types::{Ruby, Rust};
use mruby::value::Value;
use rocket::http::Status;
use rocket::Response;
use std::convert::{AsRef, TryFrom};
use std::io::Cursor;

pub mod prefork;
pub mod shared_nothing;

pub trait Interpreter {
    fn eval<T>(&self, code: T) -> Result<Value, MrbError>
    where
        T: AsRef<[u8]>;

    fn try_value<T>(&self, value: Value) -> Result<T, Error<Ruby, Rust>>
    where
        T: TryFromMrb<Value, From = Ruby, To = Rust>;
}

pub fn exec<'a, T, S>(interp: &T, code: S) -> Result<Response<'a>, Status>
where
    T: Interpreter,
    S: AsRef<str>,
{
    let response = match interp.eval(code.as_ref()) {
        Ok(response) => response,
        Err(MrbError::Exec(backtrace)) => {
            // if an exception was thrown on the interpreter during code
            // execution, return an HTTP 500 with the exception stack as the
            // message.
            warn!("mruby exception: {}", backtrace);
            let response = Response::build()
                .status(Status::InternalServerError)
                .sized_body(Cursor::new(backtrace))
                .finalize();
            return Ok(response);
        }
        Err(_) => return Err(Status::InternalServerError),
    };
    let response = interp.try_value::<Vec<Value>>(response);
    let mut response = match response {
        Ok(response) => response,
        Err(err) => {
            warn!("rack response type error: {}", err);
            let response = Response::build()
                .status(Status::InternalServerError)
                .sized_body(Cursor::new(format!("{}", err)))
                .finalize();
            return Ok(response);
        }
    };
    response.reverse();
    let status = if let Some(status) = response.pop() {
        status
    } else {
        warn!("rack response unexpected length: missing status");
        return Err(Status::InternalServerError);
    };
    let status = interp
        .try_value::<i64>(status)
        .map_err(|_| Status::InternalServerError)?;
    if response.pop().is_none() {
        warn!("rack response unexpected length: missing headers");
        return Err(Status::InternalServerError);
    }
    let body = if let Some(body) = response.pop() {
        body
    } else {
        warn!("rack response unexpected length: missing body");
        return Err(Status::InternalServerError);
    };
    let body = match interp.try_value::<Vec<Value>>(body) {
        Ok(body) => body,
        Err(err) => {
            warn!("rack response type error: {}", err);
            return Err(Status::InternalServerError);
        }
    };
    let body: String = body.into_iter().map(|part| part.to_s()).collect();
    let status = u16::try_from(status)
        .ok()
        .and_then(Status::from_code)
        .ok_or(Status::InternalServerError)?;
    let response = Response::build()
        .status(status)
        .sized_body(Cursor::new(body))
        .finalize();
    Ok(response)
}
