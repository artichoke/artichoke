use std::borrow::Cow;
use std::path::Path;

use crate::exception::Exception;
use crate::ffi;
use crate::fs::RUBY_LOAD_PATH;
use crate::{Artichoke, File, LoadSources};

impl LoadSources for Artichoke {
    type Artichoke = Self;

    type Error = Exception;

    type Exception = Exception;

    fn def_file_for_type<T>(&mut self, filename: &[u8]) -> Result<(), Self::Error>
    where
        T: File<Artichoke = Self::Artichoke, Error = Self::Exception>,
    {
        let path = ffi::bytes_to_os_str(filename)?;
        let path = Path::new(&path);
        let path = if path.is_relative() {
            Path::new(RUBY_LOAD_PATH).join(path)
        } else {
            path.to_path_buf()
        };
        self.0
            .borrow_mut()
            .vfs
            .register_extension(&path, T::require)?;
        trace!(
            "Added Rust extension to interpreter filesystem -- {:?}",
            &path
        );
        Ok(())
    }

    fn def_rb_source_file<T>(&mut self, filename: &[u8], contents: T) -> Result<(), Self::Error>
    where
        T: Into<Cow<'static, [u8]>>,
    {
        let path = ffi::bytes_to_os_str(filename)?;
        let path = Path::new(&path);
        let path = if path.is_relative() {
            Path::new(RUBY_LOAD_PATH).join(path)
        } else {
            path.to_path_buf()
        };
        self.0.borrow_mut().vfs.write_file(&path, contents)?;
        trace!("Added Ruby source to interpreter filesystem -- {:?}", &path);
        Ok(())
    }
}
