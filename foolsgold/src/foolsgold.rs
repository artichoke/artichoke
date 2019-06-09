use mruby::convert::{RustBackedValue, TryFromMrb};
use mruby::def::{rust_data_free, ClassLike, Define, EnclosingRubyScope};
use mruby::eval::MrbEval;
use mruby::file::MrbFile;
use mruby::interpreter::{Mrb, MrbApi};
use mruby::load::MrbLoadSources;
use mruby::sys::{self, DescribeState};
use mruby::value::Value;
use mruby::MrbError;
use std::borrow::Cow;
use std::mem;
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
        let module = interp.borrow_mut().def_module::<Self>("FoolsGold", None);
        module.borrow().define(&interp)?;
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

impl Counter {
    unsafe extern "C" fn get(mrb: *mut sys::mrb_state, _slf: sys::mrb_value) -> sys::mrb_value {
        let interp = interpreter_or_raise!(mrb);

        // We can probably relax the ordering constraint.
        let value = SEEN_REQUESTS_COUNTER.load(Ordering::SeqCst);
        unwrap_value_or_raise!(interp, Value::try_from_mrb(&interp, value))
    }

    unsafe extern "C" fn inc(mrb: *mut sys::mrb_state, _slf: sys::mrb_value) -> sys::mrb_value {
        let interp = interpreter_or_raise!(mrb);
        let total_requests = SEEN_REQUESTS_COUNTER.fetch_add(1, Ordering::SeqCst);
        debug!(
            "Logged request number {} in {}",
            total_requests,
            mrb.debug()
        );
        interp.nil().inner()
    }
}

impl MrbFile for Counter {
    fn require(interp: Mrb) -> Result<(), MrbError> {
        // We do not need to define an initialize method since there is no need
        // to store any state on the `mrb_value`. The counter state is in a
        // static `AtomicI64`.
        let spec = {
            let mut api = interp.borrow_mut();
            let scope = api
                .module_spec::<FoolsGold>()
                .map(EnclosingRubyScope::module)
                .ok_or(MrbError::NotDefined("FoolsGold".to_owned()))?;
            // We do not need to define a free method since we are not storing
            // any data in the `mrb_value`.
            let spec = api.def_class::<Self>("Counter", Some(scope), None);
            spec.borrow_mut()
                .add_method("get", Self::get, sys::mrb_args_none());
            spec.borrow_mut()
                .add_method("inc", Self::inc, sys::mrb_args_none());
            spec
        };
        spec.borrow().define(&interp)?;
        Ok(())
    }
}

struct Metrics;

impl MrbFile for Metrics {
    fn require(interp: Mrb) -> Result<(), MrbError> {
        let scope = interp
            .borrow()
            .module_spec::<FoolsGold>()
            .map(EnclosingRubyScope::module)
            .ok_or(MrbError::NotDefined("FoolsGold".to_owned()))?;
        let spec = interp
            .borrow_mut()
            .def_module::<Self>("Metrics", Some(scope));
        spec.borrow().define(&interp)?;
        Ok(())
    }
}

struct RequestContext {
    trace_id: Uuid,
}

impl RequestContext {
    unsafe extern "C" fn initialize(
        mrb: *mut sys::mrb_state,
        slf: sys::mrb_value,
    ) -> sys::mrb_value {
        let interp = interpreter_or_raise!(mrb);

        let request_id = Uuid::new_v4();
        let data = Self {
            trace_id: request_id,
        };
        info!("initialized RequestContext with trace id {}", request_id);
        unwrap_value_or_raise!(interp, data.try_into_ruby(&interp, Some(slf)))
    }

    unsafe extern "C" fn trace_id(mrb: *mut sys::mrb_state, slf: sys::mrb_value) -> sys::mrb_value {
        let interp = interpreter_or_raise!(mrb);

        let data = unwrap_or_raise!(
            interp,
            Self::try_from_ruby(&interp, &Value::new(&interp, slf)),
            interp.nil().inner()
        );
        let trace_id = data.borrow().trace_id;
        info!("Retrieved trace id {} in {:?}", trace_id, interp);
        mem::forget(data);
        unwrap_value_or_raise!(interp, Value::try_from_mrb(&interp, trace_id.to_string()))
    }

    unsafe extern "C" fn metrics(mrb: *mut sys::mrb_state, _slf: sys::mrb_value) -> sys::mrb_value {
        let interp = interpreter_or_raise!(mrb);
        let spec = module_spec_or_raise!(interp, Metrics);
        let borrow = spec.borrow();
        borrow
            .value(&interp)
            .unwrap_or_else(|| interp.nil())
            .inner()
    }
}

impl RustBackedValue for RequestContext {}

impl MrbFile for RequestContext {
    fn require(interp: Mrb) -> Result<(), MrbError> {
        let spec = {
            let mut api = interp.borrow_mut();
            let scope = api
                .module_spec::<FoolsGold>()
                .map(EnclosingRubyScope::module)
                .ok_or(MrbError::NotDefined("FoolsGold".to_owned()))?;
            let spec =
                api.def_class::<Self>("RequestContext", Some(scope), Some(rust_data_free::<Self>));
            spec.borrow_mut().mrb_value_is_rust_backed(true);
            spec.borrow_mut()
                .add_method("initialize", Self::initialize, sys::mrb_args_none());
            spec.borrow_mut()
                .add_method("trace_id", Self::trace_id, sys::mrb_args_none());
            spec.borrow_mut()
                .add_method("metrics", Self::metrics, sys::mrb_args_none());
            spec
        };
        spec.borrow().define(&interp)?;
        Ok(())
    }
}
