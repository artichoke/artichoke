use std::borrow::Cow;
use std::path::Path;

use crate::exception::Exception;
use crate::fs::RUBY_LOAD_PATH;
use crate::{Artichoke, File, LoadSources};

impl LoadSources for Artichoke {
    type Artichoke = Self;

    type Error = Exception;

    type Exception = Exception;

    fn def_file_for_type<P, T>(&mut self, path: P) -> Result<(), Self::Error>
    where
        P: AsRef<Path>,
        T: File<Artichoke = Self::Artichoke, Error = Self::Exception>,
    {
        let mut path = path.as_ref();
        let absolute_path;
        if path.is_relative() {
            absolute_path = Path::new(RUBY_LOAD_PATH).join(path);
            path = &absolute_path;
        }
        self.0
            .borrow_mut()
            .vfs
            .register_extension(path, T::require)?;
        trace!(
            "Added Rust extension to interpreter filesystem -- {}",
            path.display()
        );
        Ok(())
    }

    fn def_rb_source_file<P, T>(&mut self, path: P, contents: T) -> Result<(), Self::Error>
    where
        P: AsRef<Path>,
        T: Into<Cow<'static, [u8]>>,
    {
        let mut path = path.as_ref();
        let absolute_path;
        if path.is_relative() {
            absolute_path = Path::new(RUBY_LOAD_PATH).join(path);
            path = &absolute_path;
        }
        self.0.borrow_mut().vfs.write_file(path, contents)?;
        trace!(
            "Added Ruby source to interpreter filesystem -- {}",
            path.display()
        );
        Ok(())
    }
}
