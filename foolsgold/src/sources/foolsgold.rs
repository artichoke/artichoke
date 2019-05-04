use mruby::def::{ClassLike, Define, Parent};
use mruby::{interpreter_or_raise, sys, unwrap_or_raise, Mrb, MrbApi, MrbFile, TryFromMrb, Value};
use std::cell::RefCell;
use std::ffi::c_void;
use std::mem;
use std::rc::Rc;
use std::sync::atomic::{AtomicI64, Ordering};
use uuid::Uuid;

use crate::sources::Source;

static SEEN_REQUESTS_COUNTER: AtomicI64 = AtomicI64::new(0);

pub struct Lib;

impl MrbFile for Lib {
    fn require(interp: Mrb) {
        // Ruby sources
        let lib = Source::contents("foolsgold.rb");
        let adapter = Source::contents("foolsgold/adapter/memory.rb");
        interp.eval(lib).expect("foolsgold.rb");
        interp.eval(adapter).expect("foolsgold/adapter/memory.rb");

        {
            let mut api = interp.borrow_mut();
            api.def_module::<Lib>("FoolsGold", None);
        }
        // Rust sources
        Metrics::require(Rc::clone(&interp));
        Counter::require(Rc::clone(&interp));
        RequestContext::require(Rc::clone(&interp));
    }
}

struct Counter;

impl MrbFile for Counter {
    fn require(interp: Mrb) {
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
                unwrap_or_raise!(interp, Value::try_from_mrb(&interp, value))
            }
        }

        extern "C" fn inc(mrb: *mut sys::mrb_state, _slf: sys::mrb_value) -> sys::mrb_value {
            unsafe {
                let total_requests = SEEN_REQUESTS_COUNTER.fetch_add(1, Ordering::SeqCst);
                debug!(
                    "Logged request number {} in interpreter {:p}",
                    total_requests, mrb
                );
                sys::mrb_sys_nil_value()
            }
        }

        {
            let mut api = interp.borrow_mut();
            let spec = api.module_spec::<Metrics>();
            let parent = Parent::Module {
                spec: Rc::clone(&spec),
            };
            api.def_class::<Self>("Counter", Some(parent), None);
            let spec = api.class_spec_mut::<Self>();
            spec.add_method("get", get, sys::mrb_args_none());
            spec.add_method("inc", inc, sys::mrb_args_none());
        }
        let spec = interp.borrow().class_spec::<Self>();
        spec.define(Rc::clone(&interp)).expect("class install");
    }
}

struct Metrics;

impl MrbFile for Metrics {
    fn require(interp: Mrb) {
        // We do not need to define a free method since we are not storing any
        // data in the `mrb_value`.

        // We do not need to define an initialize method since there is no need
        // to store any state on the `mrb_value` since counters are stateless.
        // We can just create a new Counter instance every time it is accessed.

        extern "C" fn total_requests(
            mrb: *mut sys::mrb_state,
            _slf: sys::mrb_value,
        ) -> sys::mrb_value {
            let interp = unsafe { interpreter_or_raise!(mrb) };
            let api = interp.borrow();
            let spec = api.class_spec::<Counter>();
            let rclass = spec.rclass(Rc::clone(&interp));
            unsafe { sys::mrb_obj_new(mrb, rclass, 0, std::ptr::null()) }
        }

        {
            let mut api = interp.borrow_mut();
            let spec = api.module_spec::<Lib>();
            let parent = Parent::Module {
                spec: Rc::clone(&spec),
            };
            api.def_module::<Self>("Metrics", Some(parent));
            let spec = api.module_spec_mut::<Self>();
            spec.add_method("total_requests", total_requests, sys::mrb_args_none());
            spec.add_self_method("total_requests", total_requests, sys::mrb_args_none());
        }
        let spec = interp.borrow().module_spec::<Self>();
        spec.define(Rc::clone(&interp)).expect("module install");
    }
}

struct RequestContext {
    trace_id: Uuid,
}

impl MrbFile for RequestContext {
    fn require(interp: Mrb) {
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
                    let api = interp.borrow();
                    let spec = api.class_spec::<RequestContext>();
                    sys::mrb_sys_data_init(&mut slf, ptr, spec.data_type());
                };

                info!("initialized RequestContext with trace id {}", request_id);
                slf
            }
        }

        extern "C" fn trace_id(mrb: *mut sys::mrb_state, slf: sys::mrb_value) -> sys::mrb_value {
            unsafe {
                let interp = interpreter_or_raise!(mrb);

                let ptr = {
                    let api = interp.borrow();
                    let spec = api.class_spec::<RequestContext>();
                    sys::mrb_data_get_ptr(mrb, slf, spec.data_type())
                };
                let data = mem::transmute::<*mut c_void, Rc<RefCell<RequestContext>>>(ptr);
                let trace_id = data.borrow().trace_id;
                info!("Retrieved trace id {} in interpreter {:p}", trace_id, mrb);
                mem::forget(data);
                unwrap_or_raise!(interp, Value::try_from_mrb(&interp, trace_id.to_string()))
            }
        }

        extern "C" fn metrics(mrb: *mut sys::mrb_state, _slf: sys::mrb_value) -> sys::mrb_value {
            let interp = unsafe { interpreter_or_raise!(mrb) };
            let api = interp.borrow();
            let spec = api.module_spec::<Metrics>();
            let rclass = spec.rclass(Rc::clone(&interp));
            unsafe { sys::mrb_sys_class_value(rclass) }
        }

        {
            let mut api = interp.borrow_mut();
            let spec = api.module_spec::<Lib>();
            let parent = Parent::Module {
                spec: Rc::clone(&spec),
            };
            api.def_class::<Self>("RequestContext", Some(parent), Some(free));
            let spec = api.class_spec_mut::<Self>();
            spec.mrb_value_is_rust_backed(true);
            spec.add_method("initialize", initialize, sys::mrb_args_none());
            spec.add_method("trace_id", trace_id, sys::mrb_args_none());
            spec.add_method("metrics", metrics, sys::mrb_args_none());
        }
        let api = interp.borrow();
        let spec = api.class_spec::<Self>();
        spec.define(Rc::clone(&interp)).expect("class install");
    }
}
