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
    assets: AssetRegistry,
    html: HtmlAssetRegistry,
    mounts: MountRegistry,
    backend: Backend,
}

pub struct AssetRegistry(HashMap<String, Vec<u8>>);

pub struct HtmlAssetRegistry(HashMap<String, Vec<u8>>);

pub struct MountRegistry(HashMap<String, Mount>);

impl Builder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn add_mount(mut self, mount: Mount) -> Self {
        self.mounts.0.insert(mount.path.clone(), mount);
        self
    }

    pub fn add_html_asset<S: AsRef<str>, T: AsRef<[u8]>>(mut self, path: S, asset: T) -> Self {
        self.html
            .0
            .insert(path.as_ref().to_owned(), asset.as_ref().to_owned());
        self
    }


    pub fn add_static_asset<S: AsRef<str>, T: AsRef<[u8]>>(mut self, path: S, asset: T) -> Self {
        self.assets
            .0
            .insert(path.as_ref().to_owned(), asset.as_ref().to_owned());
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

impl Default for Builder {
    fn default() -> Self {
        Self {
            assets: AssetRegistry(Default::default()),
            html: HtmlAssetRegistry(Default::default()),
            mounts: MountRegistry(Default::default()),
            backend: Backend::Rocket,
        }
    }
}

pub struct Mount {
    pub path: String,
    pub app: Mutex<Box<dyn Fn(&Mrb) -> Result<RackApp, MrbError> + Send>>,
    pub interp_init: Option<Mutex<Box<dyn Fn(&Mrb) -> Result<(), MrbError> + Send>>>,
    pub exec_mode: ExecMode,
}
