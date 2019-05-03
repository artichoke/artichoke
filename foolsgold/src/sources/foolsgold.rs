use mruby::{interpreter_or_raise, sys, unwrap_or_raise, Mrb, MrbApi, MrbFile, TryFromMrb, Value};
use std::cell::RefCell;
use std::ffi::{c_void, CString};
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

        // Rust sources
        RequestContext::require(Rc::clone(&interp));
        Metrics::require(Rc::clone(&interp));
        Counter::require(Rc::clone(&interp));
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

        unsafe {
            let foolsgold = CString::new("FoolsGold").expect("FoolsGold");
            let foolsgold_module = sys::mrb_module_get(interp.borrow().mrb, foolsgold.as_ptr());
            let metrics = CString::new("Metrics").expect("Metrics");
            let metrics_module =
                sys::mrb_module_get_under(interp.borrow().mrb, foolsgold_module, metrics.as_ptr());
            let class = CString::new("Counter").expect("Counter class");
            let mrb_class = sys::mrb_define_class_under(
                interp.borrow().mrb,
                metrics_module,
                class.as_ptr(),
                (*interp.borrow().mrb).object_class,
            );

            let get_method = CString::new("get").expect("method");
            sys::mrb_define_method(
                interp.borrow().mrb,
                mrb_class,
                get_method.as_ptr(),
                Some(get),
                sys::mrb_args_none(),
            );

            let inc_method = CString::new("inc").expect("method");
            sys::mrb_define_method(
                interp.borrow().mrb,
                mrb_class,
                inc_method.as_ptr(),
                Some(inc),
                sys::mrb_args_none(),
            );
        }
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
            unsafe {
                let foolsgold = CString::new("FoolsGold").expect("FoolsGold");
                let foolsgold_module = sys::mrb_module_get(mrb, foolsgold.as_ptr());
                let metrics = CString::new("Metrics").expect("Metrics");
                let metrics_module =
                    sys::mrb_module_get_under(mrb, foolsgold_module, metrics.as_ptr());
                let counter = CString::new("Counter").expect("FoolsGold::Metrics::Counter");
                let counter_class = sys::mrb_class_get_under(mrb, metrics_module, counter.as_ptr());
                sys::mrb_obj_new(mrb, counter_class, 0, std::ptr::null())
            }
        }

        unsafe {
            let foolsgold = CString::new("FoolsGold").expect("FoolsGold module");
            let foolsgold_module = sys::mrb_module_get(interp.borrow().mrb, foolsgold.as_ptr());
            let module = CString::new("Metrics").expect("Metrics module");
            let mrb_module = sys::mrb_define_module_under(
                interp.borrow().mrb,
                foolsgold_module,
                module.as_ptr(),
            );

            let total_requests_method = CString::new("total_requests").expect("method");
            sys::mrb_define_method(
                interp.borrow().mrb,
                mrb_module,
                total_requests_method.as_ptr(),
                Some(total_requests),
                sys::mrb_args_none(),
            );

            sys::mrb_define_class_method(
                interp.borrow().mrb,
                mrb_module,
                total_requests_method.as_ptr(),
                Some(total_requests),
                sys::mrb_args_none(),
            );
        }
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
                    let mut api = interp.borrow_mut();
                    let data_type = api.get_or_create_data_type("RequestContext", Some(free));
                    sys::mrb_sys_data_init(&mut slf, ptr, data_type);
                };

                info!("initialized RequestContext with trace id {}", request_id);
                slf
            }
        }

        extern "C" fn trace_id(mrb: *mut sys::mrb_state, slf: sys::mrb_value) -> sys::mrb_value {
            unsafe {
                let interp = interpreter_or_raise!(mrb);

                let ptr = {
                    let mut api = interp.borrow_mut();
                    let data_type = api.get_or_create_data_type("RequestContext", Some(free));
                    sys::mrb_data_get_ptr(mrb, slf, data_type)
                };
                let data = mem::transmute::<*mut c_void, Rc<RefCell<RequestContext>>>(ptr);
                let trace_id = data.borrow().trace_id;
                info!("Retrieved trace id {} in interpreter {:p}", trace_id, mrb);
                mem::forget(data);
                unwrap_or_raise!(interp, Value::try_from_mrb(&interp, trace_id.to_string()))
            }
        }

        extern "C" fn metrics(mrb: *mut sys::mrb_state, _slf: sys::mrb_value) -> sys::mrb_value {
            unsafe {
                let foolsgold = CString::new("FoolsGold").expect("FoolsGold module");
                let foolsgold_module = sys::mrb_module_get(mrb, foolsgold.as_ptr());
                let module = CString::new("Metrics").expect("Metrics module");
                let metrics_module =
                    sys::mrb_define_module_under(mrb, foolsgold_module, module.as_ptr());
                sys::mrb_sys_class_value(metrics_module)
            }
        }

        unsafe {
            let foolsgold = CString::new("FoolsGold").expect("FoolsGold module");
            let foolsgold_module = sys::mrb_module_get(interp.borrow().mrb, foolsgold.as_ptr());
            let class = CString::new("RequestContext").expect("RequestContext class");
            let mrb_class = sys::mrb_define_class_under(
                interp.borrow().mrb,
                foolsgold_module,
                class.as_ptr(),
                (*interp.borrow().mrb).object_class,
            );
            sys::mrb_sys_set_instance_tt(mrb_class, sys::mrb_vtype::MRB_TT_DATA);

            let initialize_method = CString::new("initialize").expect("method");
            sys::mrb_define_method(
                interp.borrow().mrb,
                mrb_class,
                initialize_method.as_ptr(),
                Some(initialize),
                sys::mrb_args_none(),
            );

            let trace_id_method = CString::new("trace_id").expect("method");
            sys::mrb_define_method(
                interp.borrow().mrb,
                mrb_class,
                trace_id_method.as_ptr(),
                Some(trace_id),
                sys::mrb_args_none(),
            );

            let metrics_method = CString::new("metrics").expect("method");
            sys::mrb_define_method(
                interp.borrow().mrb,
                mrb_class,
                metrics_method.as_ptr(),
                Some(metrics),
                sys::mrb_args_none(),
            );
        }
    }
}
