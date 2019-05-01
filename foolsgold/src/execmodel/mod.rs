use mruby::{sys, Error, Ruby, Rust, TryFromMrb, Value};
use rocket::http::Status;
use rocket::Response;
use std::convert::{AsRef, TryFrom};
use std::io::Cursor;

pub mod prefork;
pub mod shared_nothing;

pub trait Interpreter {
    fn mrb(&self) -> *mut sys::mrb_state;
    fn ctx(&self) -> *mut sys::mrbc_context;
    fn try_value<T>(&self, value: Value) -> Result<T, Error<Ruby, Rust>>
    where
        T: TryFromMrb<Value, From = Ruby, To = Rust>;
}

pub fn exec<'a, T, S>(interp: &T, code: S) -> Result<Response<'a>, Status>
where
    T: Interpreter,
    S: AsRef<str>,
{
    let code = code.as_ref();
    let (status, body) = unsafe {
        let rack_response = Value::new(sys::mrb_load_nstring_cxt(
            interp.mrb(),
            code.as_ptr() as *const i8,
            code.len(),
            interp.ctx(),
        ));
        // if an exception was thrown on the interpreter during code execution,
        // return an HTTP 500 with the exception stack as the message.
        let exception = sys::mrb_sys_get_current_exception(interp.mrb());
        if let Ok(Some(exc)) = interp.try_value::<Option<String>>(Value::new(exception)) {
            warn!("mruby exception: {}", exc);
            let response = Response::build()
                .status(Status::InternalServerError)
                .sized_body(Cursor::new(exc))
                .finalize();
            return Ok(response);
        }

        let mut rack_response = interp
            .try_value::<Vec<Value>>(rack_response)
            .expect("convert");
        rack_response.reverse();
        let status_value = rack_response.pop().expect("rack status return");
        let status = interp.try_value::<i64>(status_value).expect("convert");
        let _ = rack_response.pop().expect("rack headers return");
        let body_value = rack_response.pop().expect("rack body return");
        let mut body_array = interp.try_value::<Vec<Value>>(body_value).expect("convert");
        let body_inner_value = body_array.pop().expect("rack body inner");
        let body = interp
            .try_value::<String>(body_inner_value)
            .expect("convert");
        (status, body)
    };
    let response_code = u16::try_from(status)
        .ok()
        .and_then(Status::from_code)
        .ok_or(Status::InternalServerError)?;
    let response = Response::build()
        .status(response_code)
        .sized_body(Cursor::new(body))
        .finalize();
    Ok(response)
}
