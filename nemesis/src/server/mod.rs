//! Nemesis server implementations.

use mruby::interpreter::Mrb;
use std::collections::HashMap;
use std::sync::Arc;

use crate::adapter::{AppFactory, RackApp};
use crate::interpreter::{ExecMode, InitFunc};
use crate::Error;

mod rocket;

/// Server implementation backend.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Backend {
    /// A [Rocket](::rocket)-based server implementation.
    Rocket,
}

impl Default for Backend {
    fn default() -> Self {
        Backend::Rocket
    }
}

#[derive(Default, Debug, Clone)]
struct AssetRegistry(HashMap<String, Vec<u8>>);

#[derive(Default, Clone)]
struct MountRegistry(HashMap<String, Mount>);

#[derive(Default, Clone)]
pub struct Builder {
    assets: AssetRegistry,
    mounts: MountRegistry,
    backend: Backend,
}

impl Builder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn add_mount(mut self, mount: Mount) -> Self {
        self.mounts.0.insert(mount.path.clone(), mount);
        self
    }

    pub fn add_static_asset<S: AsRef<str>, T: AsRef<[u8]>>(mut self, path: S, asset: T) -> Self {
        self.assets
            .0
            .insert(path.as_ref().to_owned(), asset.as_ref().to_owned());
        self
    }

    pub fn add_static_assets(mut self, assets: HashMap<String, Vec<u8>>) -> Self {
        self.assets.0.extend(assets);
        self
    }

    pub fn set_backend(mut self, backend: Backend) -> Self {
        self.backend = backend;
        self
    }

    pub fn serve(self) -> Result<(), Error> {
        let Backend::Rocket = self.backend;
        rocket::launcher(self)
    }
}

#[derive(Clone)]
pub struct Mount {
    path: String,
    app: Arc<AppFactory>,
    interp_init: Option<Arc<InitFunc>>,
    exec_mode: ExecMode,
}

impl Mount {
    pub fn from_rackup(name: &str, rackup: &str, mount_path: &str) -> Self {
        let name = name.to_owned();
        let path = mount_path.to_owned();
        let rackup = rackup.to_owned();
        let app = move |interp: &Mrb| {
            RackApp::from_rackup(interp, &rackup.clone(), &name.clone(), &path.clone())
        };
        Self {
            path: mount_path.to_owned(),
            app: Arc::new(Box::new(app)),
            interp_init: None,
            exec_mode: ExecMode::default(),
        }
    }

    pub fn with_init(self, init: InitFunc) -> Self {
        Self {
            path: self.path,
            app: self.app,
            interp_init: Some(Arc::new(init)),
            exec_mode: self.exec_mode,
        }
    }

    pub fn with_shared_interpreter(self, max_requests: Option<usize>) -> Self {
        Self {
            path: self.path,
            app: self.app,
            interp_init: self.interp_init,
            exec_mode: ExecMode::PerAppPerWorker {
                max_requests: max_requests.unwrap_or_default(),
            },
        }
    }
}
