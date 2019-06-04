//! Nemesis server implementations.

use mruby::interpreter::Mrb;
use mruby::MrbError;
use std::collections::HashMap;
use std::sync::Mutex;

use crate::adapter::RackApp;
use crate::interpreter::ExecMode;
use crate::Error;

pub mod rocket;

/// Server implementation backend.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Backend {
    /// A [Rocket](::rocket)-based server implementation.
    Rocket,
}

pub struct Builder {
    mounts: MountRegistry,
    backend: Backend,
}

pub struct MountRegistry(HashMap<String, Mount>);

impl Builder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn add_mount(mut self, mount: Mount) -> Self {
        self.mounts.0.insert(mount.path.clone(), mount);
        self
    }

    pub fn set_backend(mut self, backend: Backend) -> Self {
        self.backend = backend;
        self
    }

    pub fn serve(self) -> Result<(), Error> {
        let Backend::Rocket = self.backend;
        let mut launcher = ::rocket::ignite();
        for (_path, mount) in &self.mounts.0 {
            launcher = launcher.mount(mount.path.as_str(), routes![rocket::routes::route_get]);
        }
        launcher = launcher.manage(self.mounts.0);
        let err = launcher.launch();
        // This log is only reachable if Rocket has an error during startup,
        // otherwise `rocket::ignite().launch()` blocks forever.
        warn!("Failed to launch rocket: {}", err);
        Err(Error::FailedLaunch(err.to_string()))
    }
}

impl Default for Builder {
    fn default() -> Self {
        Self {
            mounts: MountRegistry(HashMap::default()),
            backend: Backend::Rocket,
        }
    }
}

pub struct Mount {
    path: String,
    app: Mutex<Box<dyn Fn(&Mrb) -> Result<RackApp, MrbError> + Send>>,
    exec_mode: ExecMode,
}
