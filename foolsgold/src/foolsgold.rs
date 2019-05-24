use mruby::convert::TryFromMrb;
use mruby::def::{ClassLike, Define, Parent};
use mruby::eval::MrbEval;
use mruby::file::MrbFile;
use mruby::interpreter::{Mrb, MrbApi};
use mruby::load::MrbLoadSources;
use mruby::sys::{self, DescribeState};
use mruby::value::Value;
use mruby::MrbError;
use std::borrow::Cow;
use std::cell::RefCell;
use std::convert::AsRef;
use std::ffi::c_void;
use std::mem;
use std::rc::Rc;
use std::sync::atomic::{AtomicI64, Ordering};
use uuid::Uuid;

use mruby_gems::Gem;

pub const RACKUP: &str = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/ruby/config.ru"));

static SEEN_REQUESTS_COUNTER: AtomicI64 = AtomicI64::new(0);

pub fn init(interp: &Mrb) -> Result<(), MrbError> {
    FoolsGold::init(interp)
}

#[derive(RustEmbed)]
// TODO: resolve path relative to CARGO_MANIFEST_DIR
// https://github.com/pyros2097/rust-embed/pull/59
#[folder = "foolsgold/ruby/lib"]
struct FoolsGold;

impl FoolsGold {
    fn contents<T: AsRef<str>>(path: T) -> Result<Vec<u8>, MrbError> {
        let path = path.as_ref();
        Self::get(path)
            .map(Cow::into_owned)
            .ok_or_else(|| MrbError::SourceNotFound(path.to_owned()))
    }
}

impl MrbFile for FoolsGold {
    fn require(interp: Mrb) -> Result<(), MrbError> {
        interp.borrow_mut().def_module::<Self>("FoolsGold", None);
        interp.eval("require 'foolsgold/ext/stats'")?;
        interp.eval("require 'foolsgold/metrics'")?;
        interp.eval("require 'foolsgold/ext/counter'")?;
        interp.eval("require 'foolsgold/middleware/request'")?;
        Ok(())
    }
}

impl Gem for FoolsGold {
    fn init(interp: &Mrb) -> Result<(), MrbError> {
        for source in Self::iter() {
            let contents = Self::contents(&source)?;
            interp.def_rb_source_file(source, contents)?;
        }
        // Rust and Ruby backed sources
        interp.def_file_for_type::<_, Self>("foolsgold.rb")?;
        interp.def_file_for_type::<_, Metrics>("foolsgold/metrics.rb")?;
        // Pure Rust sources
        interp.def_file_for_type::<_, RequestContext>("foolsgold/ext/stats.rb")?;
        interp.def_file_for_type::<_, Counter>("foolsgold/ext/counter.rb")?;
        Ok(())
    }
}

struct Counter;

impl MrbFile for Counter {
    fn require(interp: Mrb) -> Result<(), MrbError> {
        // We do not need to define a free method since we are not storing any
        // data in the `mrb_value`.

        // We do not need to define an initialize method since there is no need
        // to store any state on the `mrb_value`. The counter state is in a
        // static `AtomicI64`.

        extern "C" fn get(mrb: *mut sys::mrb_state, _slf: sys::mrb_value) -> sys::mrb_value {
            unsafe {
                let interp = interpreter_or_raise!(mrb);

                // We can probably relax the ordering constraint.
                let value = SEEN_REQUESTS_COUNTER.load(Ordering::SeqCst);
                unwrap_value_or_raise!(interp, Value::try_from_mrb(&interp, value))
            }
        }

        extern "C" fn inc(mrb: *mut sys::mrb_state, _slf: sys::mrb_value) -> sys::mrb_value {
            unsafe {
                let total_requests = SEEN_REQUESTS_COUNTER.fetch_add(1, Ordering::SeqCst);
                debug!(
                    "Logged request number {} in {}",
                    total_requests,
                    mrb.debug()
                );
                sys::mrb_sys_nil_value()
            }
        }

        let spec = {
            let mut api = interp.borrow_mut();
            let spec = api.module_spec::<FoolsGold>().expect("Metrics not defined");
            let parent = Parent::Module {
                spec: Rc::clone(&spec),
            };
            let spec = api.def_class::<Self>("Counter", Some(parent), None);
            spec.borrow_mut()
                .add_method("get", get, sys::mrb_args_none());
            spec.borrow_mut()
                .add_method("inc", inc, sys::mrb_args_none());
            spec
        };
        spec.borrow().define(&interp)?;
        Ok(())
    }
}

struct Metrics;

impl MrbFile for Metrics {
    fn require(interp: Mrb) -> Result<(), MrbError> {
        let parent = interp
            .borrow()
            .module_spec::<FoolsGold>()
            .ok_or(MrbError::NotDefined("FoolsGold".to_owned()))?;
        let parent = Parent::Module {
            spec: Rc::clone(&parent),
        };
        interp
            .borrow_mut()
            .def_module::<Self>("Metrics", Some(parent));
        Ok(())
    }
}

struct RequestContext {
    trace_id: Uuid,
}

impl MrbFile for RequestContext {
    fn require(interp: Mrb) -> Result<(), MrbError> {
        extern "C" fn free(_mrb: *mut sys::mrb_state, data: *mut c_void) {
            unsafe {
                // Implictly dropped by going out of scope
                let _ = mem::transmute::<*mut c_void, Rc<RefCell<RequestContext>>>(data);
            }
        }

        extern "C" fn initialize(
            mrb: *mut sys::mrb_state,
            mut slf: sys::mrb_value,
        ) -> sys::mrb_value {
            unsafe {
                let request_id = Uuid::new_v4();
                let data = RequestContext {
                    trace_id: request_id,
                };
                let data = Rc::new(RefCell::new(data));
                let ptr = mem::transmute::<Rc<RefCell<RequestContext>>, *mut c_void>(data);

                let interp = interpreter_or_raise!(mrb);
                {
                    let spec = class_spec_or_raise!(interp, RequestContext);
                    let borrow = spec.borrow();
                    sys::mrb_sys_data_init(&mut slf, ptr, borrow.data_type());
                };

                info!("initialized RequestContext with trace id {}", request_id);
                slf
            }
        }

        extern "C" fn trace_id(mrb: *mut sys::mrb_state, slf: sys::mrb_value) -> sys::mrb_value {
            unsafe {
                let interp = interpreter_or_raise!(mrb);

                let ptr = {
                    let spec = class_spec_or_raise!(interp, RequestContext);
                    let borrow = spec.borrow();
                    sys::mrb_data_get_ptr(mrb, slf, borrow.data_type())
                };
                let data = mem::transmute::<*mut c_void, Rc<RefCell<RequestContext>>>(ptr);
                let trace_id = data.borrow().trace_id;
                info!("Retrieved trace id {} in {:?}", trace_id, interp);
                mem::forget(data);
                unwrap_value_or_raise!(interp, Value::try_from_mrb(&interp, trace_id.to_string()))
            }
        }

        extern "C" fn metrics(mrb: *mut sys::mrb_state, _slf: sys::mrb_value) -> sys::mrb_value {
            let interp = unsafe { interpreter_or_raise!(mrb) };
            let spec = unsafe { module_spec_or_raise!(interp, Metrics) };
            let rclass = spec.borrow().rclass(Rc::clone(&interp));
            unsafe { rclass.map(|cls| sys::mrb_sys_class_value(cls)) }
                .unwrap_or_else(|| interp.nil().inner())
        }

        let spec = {
            let mut api = interp.borrow_mut();
            let spec = api
                .module_spec::<FoolsGold>()
                .ok_or(MrbError::NotDefined("FoolsGold".to_owned()))?;
            let parent = Parent::Module {
                spec: Rc::clone(&spec),
            };
            let spec = api.def_class::<Self>("RequestContext", Some(parent), Some(free));
            spec.borrow_mut().mrb_value_is_rust_backed(true);
            spec.borrow_mut()
                .add_method("initialize", initialize, sys::mrb_args_none());
            spec.borrow_mut()
                .add_method("trace_id", trace_id, sys::mrb_args_none());
            spec.borrow_mut()
                .add_method("metrics", metrics, sys::mrb_args_none());
            spec
        };
        spec.borrow().define(&interp)?;
        Ok(())
    }
}
