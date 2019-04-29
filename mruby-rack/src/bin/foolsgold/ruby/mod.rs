use mruby::{sys, Interpreter, Mrb, TryFromMrb, Value};
use ref_thread_local::RefThreadLocal;
use rocket::http::Status;
use rocket::{get, Response};
use std::borrow::Cow;
use std::convert::{AsRef, TryFrom};
use std::io::Cursor;

pub mod foolsgold;
pub mod rack;
pub mod rackup;

use foolsgold::FoolsGold;

ref_thread_local! {
    static managed INTERPRETER: Mrb = {
        let interp = Interpreter::create().expect("interp");
        {
            let mut api = interp.borrow_mut();
            api.def_file_for_type::<_, rack::Builder>("rack/builder");
            api.def_file_for_type::<_, FoolsGold>("foolsgold");
        }
        interp
    };
}

// TODO: resolve path relative to CARGO_MANIFEST_DIR
// https://github.com/pyros2097/rust-embed/pull/59
#[derive(RustEmbed)]
#[folder = "mruby-rack/ruby/lib"]
pub struct Source;

impl Source {
    fn contents<T: AsRef<str>>(path: T) -> Vec<u8> {
        let path = path.as_ref();
        Self::get(path).map(Cow::into_owned).expect(path)
    }
}

#[get("/fools-gold/prefork")]
pub fn threaded_rack_app<'a>() -> Result<Response<'a>, Status> {
    let interp = INTERPRETER.borrow();
    let code = rackup::rack_adapter();
    let (status, body) = unsafe {
        let (mrb, ctx) = {
            let api = interp.borrow();
            (api.mrb(), api.ctx())
        };
        let rack_response = Value::new(sys::mrb_load_nstring_cxt(
            mrb,
            code.as_ptr() as *const i8,
            code.len(),
            ctx,
        ));
        let api = interp.borrow();
        // if an exception was thrown on the interpreter during code execution,
        // return an HTTP 500 with the exception stack as the message.
        let exception = sys::mrb_sys_get_current_exception(api.mrb());
        if let Ok(Some(exc)) = <Option<String>>::try_from_mrb(&api, Value::new(exception)) {
            warn!("mruby exception: {}", exc);
            let response = Response::build()
                .status(Status::InternalServerError)
                .sized_body(Cursor::new(exc))
                .finalize();
            return Ok(response);
        }

        let mut rack_response = <Vec<Value>>::try_from_mrb(&api, rack_response).expect("convert");
        rack_response.reverse();
        let status_value = rack_response.pop().expect("rack status return");
        let status = i64::try_from_mrb(&api, status_value).expect("convert");
        let _ = rack_response.pop().expect("rack headers return");
        let body_value = rack_response.pop().expect("rack body return");
        let mut body_array = <Vec<Value>>::try_from_mrb(&api, body_value).expect("convert");
        let body_inner_value = body_array.pop().expect("rack body inner");
        let body = String::try_from_mrb(&api, body_inner_value).expect("convert");
        (status, body)
    };
    drop(interp);
    let response_code = u16::try_from(status)
        .ok()
        .and_then(Status::from_code)
        .expect("HTTP status code");
    let response = Response::build()
        .status(response_code)
        .sized_body(Cursor::new(body))
        .finalize();
    Ok(response)
}

#[get("/fools-gold")]
pub fn shared_nothing_rack_app<'a>() -> Result<Response<'a>, Status> {
    debug!("Initializing mruby interpreter");
    let interp = Interpreter::create().map_err(|_| Status::InternalServerError)?;
    {
        let mut api = interp.borrow_mut();
        api.def_file_for_type::<_, rack::Builder>("rack/builder");
        api.def_file_for_type::<_, FoolsGold>("foolsgold");
    }
    let code = rackup::rack_adapter();
    let (status, body) = unsafe {
        let (mrb, ctx) = {
            let api = interp.borrow();
            (api.mrb(), api.ctx())
        };
        let rack_response = Value::new(sys::mrb_load_nstring_cxt(
            mrb,
            code.as_ptr() as *const i8,
            code.len(),
            ctx,
        ));
        let api = interp.borrow();
        // if an exception was thrown on the interpreter during code execution,
        // return an HTTP 500 with the exception stack as the message.
        let exception = sys::mrb_sys_get_current_exception(api.mrb());
        if let Ok(Some(exc)) = <Option<String>>::try_from_mrb(&api, Value::new(exception)) {
            warn!("mruby exception: {}", exc);
            let response = Response::build()
                .status(Status::InternalServerError)
                .sized_body(Cursor::new(exc))
                .finalize();
            return Ok(response);
        }

        let mut rack_response = <Vec<Value>>::try_from_mrb(&api, rack_response).expect("convert");
        rack_response.reverse();
        let status_value = rack_response.pop().expect("rack status return");
        let status = i64::try_from_mrb(&api, status_value).expect("convert");
        let _ = rack_response.pop().expect("rack headers return");
        let body_value = rack_response.pop().expect("rack body return");
        let mut body_array = <Vec<Value>>::try_from_mrb(&api, body_value).expect("convert");
        let body_inner_value = body_array.pop().expect("rack body inner");
        let body = String::try_from_mrb(&api, body_inner_value).expect("convert");
        (status, body)
    };
    drop(interp);
    let response_code = u16::try_from(status)
        .ok()
        .and_then(Status::from_code)
        .expect("HTTP status code");
    let response = Response::build()
        .status(response_code)
        .sized_body(Cursor::new(body))
        .finalize();
    Ok(response)
}
