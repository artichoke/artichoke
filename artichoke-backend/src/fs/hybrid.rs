use std::borrow::Cow;
use std::io;
use std::path::Path;

use crate::fs::memory::Memory;
use crate::fs::native::Native;
use crate::fs::{ExtensionHook, Filesystem, RUBY_LOAD_PATH};

#[derive(Default, Debug)]
pub struct Hybrid {
    memory: Memory,
    native: Native,
}

impl Hybrid {
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }
}

impl Filesystem for Hybrid {
    fn is_file(&self, path: &Path) -> bool {
        self.memory.is_file(path) || self.native.is_file(path)
    }

    fn read_file(&self, path: &Path) -> io::Result<Cow<'_, [u8]>> {
        self.memory.read_file(path).or_else(|_| self.native.read_file(path))
    }

    fn write_file(&mut self, path: &Path, buf: Cow<'static, [u8]>) -> io::Result<()> {
        if path.starts_with(RUBY_LOAD_PATH) {
            self.memory.write_file(path, buf)
        } else {
            self.native.write_file(path, buf)
        }
    }

    fn get_extension(&self, path: &Path) -> Option<ExtensionHook> {
        self.memory.get_extension(path)
    }

    fn register_extension(&mut self, path: &Path, extension: ExtensionHook) -> io::Result<()> {
        self.memory.register_extension(path, extension)
    }

    fn is_required(&self, path: &Path) -> bool {
        self.memory.is_required(path) || self.native.is_required(path)
    }

    fn mark_required(&mut self, path: &Path) -> io::Result<()> {
        if path.starts_with(RUBY_LOAD_PATH) {
            self.memory.mark_required(path)
        } else {
            self.native.mark_required(path)
        }
    }
}
