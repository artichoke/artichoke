use std::borrow::Cow;
use std::collections::HashSet;
use std::env;
use std::fs;
use std::io;
use std::path::{Path, PathBuf};

use crate::fs::{absolutize_relative_to, ExtensionHook, Filesystem};

#[derive(Default, Debug, PartialEq, Eq)]
pub struct Native {
    loaded_features: HashSet<PathBuf>,
}

impl Native {
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }
}

impl Filesystem for Native {
    fn is_file(&self, path: &Path) -> bool {
        if let Ok(metadata) = fs::metadata(path) {
            !metadata.is_dir()
        } else {
            false
        }
    }

    fn read_file(&self, path: &Path) -> io::Result<Cow<'_, [u8]>> {
        Ok(fs::read(path)?.into())
    }

    fn write_file(&mut self, path: &Path, buf: Cow<'static, [u8]>) -> io::Result<()> {
        fs::write(path, buf)
    }

    fn get_extension(&self, path: &Path) -> Option<ExtensionHook> {
        let _ = path;
        None
    }

    fn register_extension(&mut self, path: &Path, extension: ExtensionHook) -> io::Result<()> {
        let _ = path;
        let _ = extension;
        Ok(())
    }

    fn is_required(&self, path: &Path) -> bool {
        if let Ok(cwd) = env::current_dir() {
            let path = absolutize_relative_to(path, &cwd);
            self.loaded_features.contains(&path)
        } else {
            false
        }
    }

    fn mark_required(&mut self, path: &Path) -> io::Result<()> {
        let cwd = env::current_dir()?;
        let path = absolutize_relative_to(path, &cwd);
        self.loaded_features.insert(path);
        Ok(())
    }
}
