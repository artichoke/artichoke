#![feature(integer_atomics)]
#![feature(proc_macro_hygiene, decl_macro)]
#![deny(clippy::all, clippy::pedantic)]

use log::{debug, info, trace, warn};
use mruby::*;
use rocket::http::{ContentType, Status};
use rocket::{get, routes, Response};
use std::cell::RefCell;
use std::convert::TryFrom;
use std::ffi::CString;
use std::io::Cursor;
use std::rc::Rc;
use std::sync::atomic::{AtomicI64, Ordering};
use uuid::Uuid;

static SEEN_REQUESTS_COUNTER: AtomicI64 = AtomicI64::new(0);
// Ruby sources
const RACK_BUILDER_SOURCE: &str = include_str!("../ruby/rack/builder.rb");
const FOOLS_GOLD_SOURCE: &str = include_str!("../ruby/fools-gold/adapter/in_memory.rb");
const RACKUP_SOURCE: &str = include_str!("../ruby/config.ru");
const REQUIRE_PREAMBLE: &str = r#"
require 'rack/builder'
require 'fools-gold'
"#;

// static resources
const INDEX: &str = include_str!("../static/index.html");
const PYRITE: &[u8] = include_bytes!("../static/pyrite.jpg");
const RESF: &[u8] = include_bytes!("../static/resf.png");

struct RackBuilder;

impl MrbFile for RackBuilder {
    fn require(interp: Mrb) {
        unsafe {
            let mrb = interp.borrow().mrb();
            let ctx = interp.borrow().ctx();
            sys::mrb_load_nstring_cxt(
                mrb,
                RACK_BUILDER_SOURCE.as_ptr() as *const i8,
                RACK_BUILDER_SOURCE.len(),
                ctx,
            );
        }
    }
}

struct FoolsGold;

impl MrbFile for FoolsGold {
    fn require(interp: Mrb) {
        unsafe {
            let mrb = interp.borrow().mrb();
            let ctx = interp.borrow().ctx();
            sys::mrb_load_nstring_cxt(
                mrb,
                FOOLS_GOLD_SOURCE.as_ptr() as *const i8,
                FOOLS_GOLD_SOURCE.len(),
                ctx,
            );
            RequestStats::require(interp);
        }
    }
}

struct RequestStats {
    id: Uuid,
}

impl MrbFile for RequestStats {
    fn require(interp: Mrb) {
        extern "C" fn free(_mrb: *mut sys::mrb_state, data: *mut ::std::ffi::c_void) {
            unsafe {
                debug!("preparing to free RequestStats instance");
                // Implictly dropped by going out of scope
                let inner =
                    std::mem::transmute::<*mut std::ffi::c_void, Rc<RefCell<RequestStats>>>(data);
                debug!(
                    "freeing RequestStats instance with id {}",
                    inner.borrow().id
                );
            }
        }

        extern "C" fn initialize(
            mrb: *mut sys::mrb_state,
            mut slf: sys::mrb_value,
        ) -> sys::mrb_value {
            unsafe {
                let request_id = Uuid::new_v4();
                let data = RequestStats { id: request_id };
                let data = Rc::new(RefCell::new(data));
                debug!(
                    "Storing smart pointer to RequestStats on mrb_value {:p} in interpreter {:p}",
                    &slf, mrb
                );
                let ptr =
                    std::mem::transmute::<Rc<RefCell<RequestStats>>, *mut std::ffi::c_void>(data);

                let interp = Interpreter::from_user_data(mrb).expect("interpreter");
                let mut api = interp.borrow_mut();
                let data_type = api.get_or_create_data_type("RequestStats", Some(free));
                sys::mrb_sys_data_init(&mut slf, ptr, data_type);

                info!("initialized RequestStats with uuid {}", request_id);
                slf
            }
        }

        extern "C" fn req_start(mrb: *mut sys::mrb_state, slf: sys::mrb_value) -> sys::mrb_value {
            unsafe {
                let interp = Interpreter::from_user_data(mrb).expect("interpreter");
                let mut api = interp.borrow_mut();
                let mrb = api.mrb();

                let data_type = api.get_or_create_data_type("RequestStats", Some(free));
                let ptr = sys::mrb_data_get_ptr(mrb, slf, data_type);
                let data =
                    std::mem::transmute::<*mut std::ffi::c_void, Rc<RefCell<RequestStats>>>(ptr);
                info!(
                    "Started request with id {} in interpreter {:p}",
                    data.borrow().id,
                    mrb
                );
                let id = data.borrow().id;
                std::mem::forget(data);
                match Value::try_from_mrb(&api, id.to_string()) {
                    Ok(value) => value.inner(),
                    Err(err) => {
                        // could not convert V4 UUID string to mrb_value.
                        // This should be unreachable since request_id is a
                        // Rust String which cannot contain NUL bytes.
                        let eclass = CString::new("RuntimeError").expect("eclass");
                        let message = CString::new(format!("{}", err)).expect("message");
                        sys::mrb_sys_raise(mrb, eclass.as_ptr(), message.as_ptr());
                        api.nil().expect("nil").inner()
                    }
                }
            }
        }

        extern "C" fn seen_count(mrb: *mut sys::mrb_state, slf: sys::mrb_value) -> sys::mrb_value {
            unsafe {
                let interp = Interpreter::from_user_data(mrb).expect("interpreter");
                let mut api = interp.borrow_mut();
                let mrb = api.mrb();

                let data_type = api.get_or_create_data_type("RequestStats", Some(free));
                let ptr = sys::mrb_data_get_ptr(mrb, slf, data_type);
                let data =
                    std::mem::transmute::<*mut std::ffi::c_void, Rc<RefCell<RequestStats>>>(ptr);
                info!(
                    "Retrieving seen count for request with id {} in interpreter {:p}",
                    data.borrow().id,
                    mrb
                );
                std::mem::forget(data);
                let seen_count = SEEN_REQUESTS_COUNTER.load(Ordering::SeqCst);
                match Value::try_from_mrb(&api, seen_count) {
                    Ok(value) => value.inner(),
                    Err(err) => {
                        // could not convert req counter i64 to mrb_value.
                        // This should be unreachable since convering from i64
                        // to mrb_value is infallible.
                        let eclass = CString::new("RuntimeError").expect("eclass");
                        let message = CString::new(format!("{}", err)).expect("message");
                        sys::mrb_sys_raise(mrb, eclass.as_ptr(), message.as_ptr());
                        api.nil().expect("nil").inner()
                    }
                }
            }
        }

        extern "C" fn req_finalize(
            mrb: *mut sys::mrb_state,
            slf: sys::mrb_value,
        ) -> sys::mrb_value {
            unsafe {
                let interp = Interpreter::from_user_data(mrb).expect("interpreter");
                let mut api = interp.borrow_mut();
                let mrb = api.mrb();

                let data_type = api.get_or_create_data_type("RequestStats", Some(free));
                let ptr = sys::mrb_data_get_ptr(mrb, slf, data_type);
                let data =
                    std::mem::transmute::<*mut std::ffi::c_void, Rc<RefCell<RequestStats>>>(ptr);
                info!(
                    "Finalizing request with id {} in interpreter {:p}",
                    data.borrow().id,
                    mrb
                );
                std::mem::forget(data);
                SEEN_REQUESTS_COUNTER.fetch_add(1, Ordering::SeqCst);
                api.nil().expect("nil").inner()
            }
        }

        unsafe {
            let mrb = { interp.borrow().mrb() };
            let fools_gold = CString::new("FoolsGold").expect("Kernel module");
            let fools_gold_module = sys::mrb_module_get(mrb, fools_gold.as_ptr());
            // this `CString` needs to stay in scope for the life of the mruby
            // interpreter, otherwise `mrb_close` will segfault.
            let class = CString::new("RequestStats").expect("RequestStats class");
            let mrb_class = sys::mrb_define_class_under(
                mrb,
                fools_gold_module,
                class.as_ptr(),
                (*mrb).object_class,
            );
            sys::mrb_sys_set_instance_tt(mrb_class, sys::mrb_vtype_MRB_TT_DATA);

            let initialize_method = CString::new("initialize").expect("method");
            sys::mrb_define_method(
                mrb,
                mrb_class,
                initialize_method.as_ptr(),
                Some(initialize),
                sys::mrb_args_none(),
            );

            let req_start_method = CString::new("req_start").expect("method");
            sys::mrb_define_method(
                mrb,
                mrb_class,
                req_start_method.as_ptr(),
                Some(req_start),
                sys::mrb_args_none(),
            );

            let seen_count_method = CString::new("seen_count").expect("method");
            sys::mrb_define_method(
                mrb,
                mrb_class,
                seen_count_method.as_ptr(),
                Some(seen_count),
                sys::mrb_args_none(),
            );

            let req_finalize_method = CString::new("req_finalize").expect("method");
            sys::mrb_define_method(
                mrb,
                mrb_class,
                req_finalize_method.as_ptr(),
                Some(req_finalize),
                sys::mrb_args_none(),
            );
        }
    }
}

pub fn main() -> Result<(), i32> {
    env_logger::Builder::from_env("FOOLS_GOLD_LOG").init();
    if let Err(err) = spawn() {
        eprintln!("ERR: {}", err);
        Err(1)
    } else {
        Ok(())
    }
}

pub fn spawn() -> Result<(), String> {
    let err = rocket::ignite()
        .mount("/", routes![index, fools_gold])
        .mount("/img", routes![pyrite, resf])
        .launch();
    // This log is only reachable is Rocket has an error during startup,
    // otherwise `rocket::ignite().launch()` blocks forever.
    warn!("Failed to launch rocket: {}", err);
    Err(err.to_string())
}

#[get("/")]
fn index<'a>() -> Response<'a> {
    Response::build()
        .sized_body(Cursor::new(INDEX.to_owned()))
        .header(ContentType::HTML)
        .finalize()
}

#[get("/pyrite.jpg")]
fn pyrite() -> Vec<u8> {
    PYRITE.to_vec()
}

#[get("/resf.png")]
fn resf() -> Vec<u8> {
    RESF.to_vec()
}

#[get("/fools-gold")]
fn fools_gold<'a>() -> Response<'a> {
    debug!("Initializing mruby interpreter");
    let interp = Interpreter::create().expect("interpreter");
    {
        let mut api = interp.borrow_mut();
        api.def_file_for_type::<_, RackBuilder>("rack/builder");
        api.def_file_for_type::<_, FoolsGold>("fools-gold");
    }
    let code = format!(
        "{}\nFoolsGold::Adapter::InMemory.new(Rack::Builder.new {{ {} }}).call({{}})",
        REQUIRE_PREAMBLE, RACKUP_SOURCE
    );
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
        let exception = sys::mrb_sys_get_current_exception(api.mrb());
        if let Ok(Some(exc)) = <Option<String>>::try_from_mrb(&api, Value::new(exception)) {
            warn!("mruby exception: {}", exc);
            return Response::build()
                .status(Status::from_code(500).expect("500"))
                .sized_body(Cursor::new(exc))
                .finalize();
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
    trace!("done with interp: refcount = {}", Rc::strong_count(&interp));
    drop(interp);
    let response_code = u16::try_from(status)
        .ok()
        .and_then(Status::from_code)
        .expect("HTTP status code");
    Response::build()
        .status(response_code)
        .sized_body(Cursor::new(body))
        .finalize()
}
