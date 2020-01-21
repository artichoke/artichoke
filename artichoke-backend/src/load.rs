use artichoke_core::file::File;
use artichoke_core::load::LoadSources;
use std::borrow::Cow;
use std::io;
use std::path::Path;

use crate::fs::{self, RUBY_LOAD_PATH};
use crate::{Artichoke, ArtichokeError};

impl LoadSources for Artichoke {
    type Artichoke = Self;

    fn def_file_for_type<T>(&mut self, filename: &[u8]) -> Result<(), ArtichokeError>
    where
        T: File<Artichoke = Self>,
    {
        let path = fs::bytes_to_osstr(self, filename).map_err(|err| {
            ArtichokeError::Vfs(io::Error::new(io::ErrorKind::Other, err.to_string()))
        })?;
        let path = Path::new(path);
        let path = if path.is_relative() {
            Path::new(RUBY_LOAD_PATH).join(path)
        } else {
            path.to_path_buf()
        };
        if let Some(parent) = path.parent() {
            self.vfs_mut().create_dir_all(parent)?;
        }
        if !self.vfs().is_file(&path) {
            let contents = format!("# virtual source file -- {:?}", &path);
            self.vfs_mut().write_file(&path, contents)?;
        }
        let mut metadata = self.vfs().metadata(&path).unwrap_or_default();
        metadata.require = Some(T::require);
        self.vfs_mut().set_metadata(&path, metadata)?;
        trace!(
            "Added rust-backed ruby source file with require func -- {:?}",
            &path
        );
        Ok(())
    }

    fn def_rb_source_file<T>(&mut self, filename: &[u8], contents: T) -> Result<(), ArtichokeError>
    where
        T: Into<Cow<'static, [u8]>>,
    {
        let path = fs::bytes_to_osstr(self, filename).map_err(|err| {
            ArtichokeError::Vfs(io::Error::new(io::ErrorKind::Other, err.to_string()))
        })?;
        let path = Path::new(path);
        let path = if path.is_relative() {
            Path::new(RUBY_LOAD_PATH).join(path)
        } else {
            path.to_path_buf()
        };
        if let Some(parent) = path.parent() {
            self.vfs_mut().create_dir_all(parent)?;
        }
        self.vfs_mut().write_file(&path, contents.into().as_ref())?;
        trace!("Added pure ruby source file -- {:?}", &path);
        Ok(())
    }
}
