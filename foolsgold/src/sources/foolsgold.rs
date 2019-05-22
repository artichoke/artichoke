use mruby::convert::TryFromMrb;
use mruby::def::{ClassLike, Define, Parent};
use mruby::eval::{EvalContext, MrbEval};
use mruby::file::MrbFile;
use mruby::interpreter::{Mrb, MrbApi};
use mruby::sys::{self, DescribeState};
use mruby::value::Value;
use mruby::{
    class_spec_or_raise, interpreter_or_raise, module_spec_or_raise, unwrap_value_or_raise,
};
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
        // TODO: Implement ruby sources with `MrbLoadSources::def_rb_source_file`
        let contents = Source::contents("foolsgold.rb");
        interp
            .eval_with_context(contents, EvalContext::new("foolsgold.rb"))
            .expect("foolsgold source");
        let contents = Source::contents("foolsgold/adapter/memory.rb");
        interp
            .eval_with_context(contents, EvalContext::new("foolsgold/adapter/memory.rb"))
            .expect("foolsgold source");

        {
            let mut api = interp.borrow_mut();
            api.def_module::<Self>("FoolsGold", None);
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
            // TODO: return Err instead of expects when require is fallible. See
            // GH-25.
            let spec = api.module_spec::<Metrics>().expect("Metrics not defined");
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
        spec.borrow().define(&interp).expect("class install");
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
            let spec = unsafe { class_spec_or_raise!(interp, Counter) };
            let rclass = spec.borrow().rclass(Rc::clone(&interp));
            if let Some(rclass) = rclass {
                let args = &[];
                unsafe { sys::mrb_obj_new(mrb, rclass, 0, args.as_ptr()) }
            } else {
                interp.nil().inner()
            }
        }

        let spec = {
            let mut api = interp.borrow_mut();
            // TODO: return Err instead of expects when require is fallible. See
            // GH-25.
            let spec = api.module_spec::<Lib>().expect("lib not defined");
            let parent = Parent::Module {
                spec: Rc::clone(&spec),
            };
            let spec = api.def_module::<Self>("Metrics", Some(parent));
            spec.borrow_mut()
                .add_method("total_requests", total_requests, sys::mrb_args_none());
            spec.borrow_mut().add_self_method(
                "total_requests",
                total_requests,
                sys::mrb_args_none(),
            );
            spec
        };
        spec.borrow().define(&interp).expect("module install");
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
            // TODO: return Err instead of expects when require is fallible. See
            // GH-25.
            let spec = api.module_spec::<Lib>().expect("lib not defined");
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
        spec.borrow().define(&interp).expect("class install");
    }
}
