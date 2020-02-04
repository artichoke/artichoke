use std::borrow::Cow;
use std::io;
use std::path::Path;

use crate::ffi;
use crate::fs::RUBY_LOAD_PATH;
use crate::{Artichoke, ArtichokeError, File, LoadSources};

impl LoadSources for Artichoke {
    type Artichoke = Self;

    fn def_file_for_type<T>(&self, filename: &[u8]) -> Result<(), ArtichokeError>
    where
        T: File<Artichoke = Self>,
    {
        let api = self.0.borrow();
        let path = ffi::bytes_to_os_str(filename).map_err(|err| {
            ArtichokeError::Vfs(io::Error::new(io::ErrorKind::Other, err.to_string()))
        })?;
        let path = Path::new(&path);
        let path = if path.is_relative() {
            Path::new(RUBY_LOAD_PATH).join(path)
        } else {
            path.to_path_buf()
        };
        if let Some(parent) = path.parent() {
            api.vfs.create_dir_all(parent)?;
        }
        if !api.vfs.is_file(&path) {
            let contents = format!("# virtual source file -- {:?}", &path);
            api.vfs.write_file(&path, contents)?;
        }
        let mut metadata = api.vfs.metadata(&path).unwrap_or_default();
        metadata.require = Some(T::require);
        api.vfs.set_metadata(&path, metadata)?;
        trace!(
            "Added rust-backed ruby source file with require func -- {:?}",
            &path
        );
        Ok(())
    }

    fn def_rb_source_file<T>(&self, filename: &[u8], contents: T) -> Result<(), ArtichokeError>
    where
        T: Into<Cow<'static, [u8]>>,
    {
        let api = self.0.borrow();
        let path = ffi::bytes_to_os_str(filename).map_err(|err| {
            ArtichokeError::Vfs(io::Error::new(io::ErrorKind::Other, err.to_string()))
        })?;
        let path = Path::new(&path);
        let path = if path.is_relative() {
            Path::new(RUBY_LOAD_PATH).join(path)
        } else {
            path.to_path_buf()
        };
        if let Some(parent) = path.parent() {
            api.vfs.create_dir_all(parent)?;
        }
        api.vfs.write_file(&path, contents.into().as_ref())?;
        trace!("Added pure ruby source file -- {:?}", &path);
        Ok(())
    }
}
