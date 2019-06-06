//! Create or retrieve an interpreter for a request.

use mruby::eval::MrbEval;
use mruby::gc::GarbageCollection;
use mruby::interpreter::{Interpreter, Mrb};
use mruby::MrbError;
use mruby_gems::rubygems::rack;
use ref_thread_local::RefThreadLocal;
use std::collections::HashMap;
use std::rc::Rc;
use std::sync::Arc;

use crate::adapter::RackApp;
use crate::rubygems::nemesis;
use crate::Error;

pub type InitFunc = Box<dyn Fn(&Mrb) -> Result<(), MrbError> + Send + Sync>;

ref_thread_local! {
    static managed STORAGE: HashMap<Key, (usize, Mrb)> = HashMap::default();
}

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
enum Key {
    PerWorker { app: Option<String> },
}

/// Execution mode of an interpreter for a given mount.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ExecMode {
    /// A single interpreter will be used for a worker executing the rack app.
    PerAppPerWorker {
        /// After `max_requests`, close the interpreter and lazily initialize a
        /// new one.
        ///
        /// If `max_requests` is `0`, the interpreter is never recycled.
        max_requests: usize,
    },
    /// A new interpreter will be initialized for each request and closed at the
    /// end of the request.
    SingleUse,
}

impl ExecMode {
    pub fn interpreter(
        &self,
        mount_path: &str,
        init: &Option<Arc<InitFunc>>,
    ) -> Result<Mrb, Error> {
        match self {
            ExecMode::SingleUse => {
                info!(
                    "Initializing single use interpreter for app at {}",
                    mount_path
                );
                let interp = Interpreter::create()?;
                rack::init(&interp)?;
                nemesis::init(&interp)?;
                // Preload required gem sources
                interp.eval("require 'rack'")?;
                interp.eval("require 'nemesis'")?;
                interp.eval("require 'nemesis/response'")?;
                if let Some(init) = init {
                    init(&interp)?;
                }
                Ok(interp)
            }
            ExecMode::PerAppPerWorker { .. } => {
                let key = Key::PerWorker {
                    app: Some(mount_path.to_owned()),
                };
                let interp = {
                    let borrow = STORAGE.borrow();
                    borrow.get(&key).map(|(_, interp)| Rc::clone(interp))
                };
                if let Some(interp) = interp {
                    Ok(interp)
                } else {
                    info!(
                        "Initializing thread local interpreter for app at {}",
                        mount_path
                    );
                    let interp = Interpreter::create()?;
                    rack::init(&interp)?;
                    nemesis::init(&interp)?;
                    // Preload required gem sources
                    interp.eval("require 'rack'")?;
                    interp.eval("require 'nemesis'")?;
                    interp.eval("require 'nemesis/response'")?;
                    if let Some(init) = init {
                        init(&interp)?;
                    };
                    STORAGE.borrow_mut().insert(key, (0, Rc::clone(&interp)));
                    Ok(interp)
                }
            }
        }
    }

    /// Finalize a request on the interpeter for `app`.
    ///
    /// Keep track of the request count for an interpreter and potentially tear
    /// it down if it has served too many requests.
    ///
    /// Maybe execute a garbage collection on the interpreter. Returns true if
    /// a GC was performed, false otherwise.
    pub fn finalize(&self, interp: &Mrb, app: &RackApp) -> bool {
        match self {
            ExecMode::SingleUse => false,
            ExecMode::PerAppPerWorker { max_requests } => {
                let key = Key::PerWorker {
                    app: Some(app.mount_path().to_owned()),
                };
                {
                    let mut borrow = STORAGE.borrow_mut();
                    let counter = borrow
                        .get_mut(&key)
                        .map(|record| {
                            let counter = record.0;
                            record.0 += 1;
                            counter
                        })
                        .unwrap_or_default();
                    info!(
                        "Finalizing request {} for app at {}",
                        counter,
                        app.mount_path()
                    );
                    if *max_requests > 0 && counter > 0 && counter % max_requests == 0 {
                        // Recycle the interpreter if it has been used for
                        // `max_requests` app invocations.
                        borrow.remove(&key);
                        info!(
                            "Recycling interpreter at {} after {} requests",
                            app.mount_path(),
                            counter
                        );
                        false
                    } else {
                        interp.incremental_gc();
                        true
                    }
                }
            }
        }
    }
}

impl Default for ExecMode {
    fn default() -> Self {
        ExecMode::SingleUse
    }
}
