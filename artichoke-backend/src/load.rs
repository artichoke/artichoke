use std::borrow::Cow;
use std::ffi::OsStr;
use std::path::Path;

use crate::core::{Eval, File, LoadSources};
use crate::error::Error;
use crate::ffi::InterpreterExtractError;
use crate::fs::RUBY_LOAD_PATH;
use crate::Artichoke;

const RUBY_EXTENSION: &str = "rb";

impl LoadSources for Artichoke {
    type Artichoke = Self;
    type Error = Error;
    type Exception = Error;

    fn def_file_for_type<P, T>(&mut self, path: P) -> Result<(), Self::Error>
    where
        P: AsRef<Path>,
        T: File<Artichoke = Self::Artichoke, Error = Self::Exception>,
    {
        let state = self.state.as_deref_mut().ok_or_else(InterpreterExtractError::new)?;
        let mut path = path.as_ref();
        let absolute_path;
        if path.is_relative() {
            // All extension hooks added to the interpreter are placed in the
            // default Ruby load path. This ensures that alternate load paths,
            // such as those defined by the `RUBYLIB` environment variable are
            // not written to.
            absolute_path = Path::new(RUBY_LOAD_PATH).join(path);
            path = &absolute_path;
        }
        state.vfs.register_extension(path, T::require)?;
        trace!("Added Rust extension to interpreter filesystem -- {}", path.display());
        Ok(())
    }

    fn def_rb_source_file<P, T>(&mut self, path: P, contents: T) -> Result<(), Self::Error>
    where
        P: AsRef<Path>,
        T: Into<Cow<'static, [u8]>>,
    {
        let state = self.state.as_deref_mut().ok_or_else(InterpreterExtractError::new)?;
        let mut path = path.as_ref();
        let absolute_path;
        if path.is_relative() {
            // All source files added to the interpreter are placed in the
            // default Ruby load path. This ensures that alternate load paths,
            // such as those defined by the `RUBYLIB` environment variable are
            // not written to.
            absolute_path = Path::new(RUBY_LOAD_PATH).join(path);
            path = &absolute_path;
        }
        state.vfs.write_file(path, contents.into())?;
        trace!("Added Ruby source to interpreter filesystem -- {}", path.display());
        Ok(())
    }

    fn resolve_source_path<P>(&self, path: P) -> Result<Option<Vec<u8>>, Self::Error>
    where
        P: AsRef<Path>,
    {
        let state = self.state.as_deref().ok_or_else(InterpreterExtractError::new)?;
        let path = path.as_ref();
        if let Some(path) = state.vfs.resolve_file(path) {
            return Ok(Some(path));
        }
        // If the given path did not end in `.rb`, try again with a `.rb` file
        // extension.
        if !matches!(path.extension(), Some(ext) if *ext == *OsStr::new(RUBY_EXTENSION)) {
            let mut path = path.to_owned();
            path.set_extension(RUBY_EXTENSION);
            return Ok(state.vfs.resolve_file(&path));
        }
        Ok(None)
    }

    fn source_is_file<P>(&self, path: P) -> Result<bool, Self::Error>
    where
        P: AsRef<Path>,
    {
        let state = self.state.as_deref().ok_or_else(InterpreterExtractError::new)?;
        let path = path.as_ref();
        if state.vfs.is_file(path) {
            return Ok(true);
        }
        // If the given path did not end in `.rb`, try again with a `.rb` file
        // extension.
        if !matches!(path.extension(), Some(ext) if *ext == *OsStr::new(RUBY_EXTENSION)) {
            let mut path = path.to_owned();
            path.set_extension(RUBY_EXTENSION);
            if state.vfs.is_file(&path) {
                return Ok(true);
            }
        }
        Ok(false)
    }

    fn load_source<P>(&mut self, path: P) -> Result<bool, Self::Error>
    where
        P: AsRef<Path>,
    {
        let path = path.as_ref();
        let mut alternate_path;
        let path = {
            let state = self.state.as_deref_mut().ok_or_else(InterpreterExtractError::new)?;
            // If a file is already required, short circuit.
            if state.vfs.is_required(path) {
                return Ok(false);
            }
            // Require Rust `File` first because an File may define classes and
            // modules with `LoadSources` and Ruby files can require arbitrary
            // other files, including some child sources that may depend on these
            // module definitions.
            match state.vfs.get_extension(path) {
                Some(hook) => {
                    // dynamic, Rust-backed `File` require
                    hook(self)?;
                    path
                }
                None if matches!(path.extension(), Some(ext) if *ext == *OsStr::new(RUBY_EXTENSION)) => path,
                None => {
                    alternate_path = path.to_owned();
                    alternate_path.set_extension(RUBY_EXTENSION);
                    // If a file is already required, short circuit.
                    if state.vfs.is_required(&alternate_path) {
                        return Ok(false);
                    }
                    if let Some(hook) = state.vfs.get_extension(&alternate_path) {
                        // dynamic, Rust-backed `File` require
                        hook(self)?;
                        // This ensures that if we load the hook at an alternate
                        // path, we use that alternate path to load the Ruby
                        // source.
                        &alternate_path
                    } else {
                        path
                    }
                }
            }
        };
        let contents = self.read_source_file_contents(path)?.into_owned();
        self.eval(contents.as_ref())?;
        trace!("Successful load of {}", path.display());
        Ok(true)
    }

    fn require_source<P>(&mut self, path: P) -> Result<bool, Self::Error>
    where
        P: AsRef<Path>,
    {
        let path = path.as_ref();
        let mut alternate_path;
        let path = {
            let state = self.state.as_deref_mut().ok_or_else(InterpreterExtractError::new)?;
            // If a file is already required, short circuit.
            if state.vfs.is_required(path) {
                return Ok(false);
            }
            // Require Rust `File` first because an File may define classes and
            // modules with `LoadSources` and Ruby files can require arbitrary
            // other files, including some child sources that may depend on these
            // module definitions.
            match state.vfs.get_extension(path) {
                Some(hook) => {
                    // dynamic, Rust-backed `File` require
                    hook(self)?;
                    path
                }
                None if matches!(path.extension(), Some(ext) if *ext == *OsStr::new(RUBY_EXTENSION)) => path,
                None => {
                    alternate_path = path.to_owned();
                    alternate_path.set_extension(RUBY_EXTENSION);
                    // If a file is already required, short circuit.
                    if state.vfs.is_required(&alternate_path) {
                        return Ok(false);
                    }
                    if let Some(hook) = state.vfs.get_extension(&alternate_path) {
                        // dynamic, Rust-backed `File` require
                        hook(self)?;
                        // This ensures that if we load the hook at an alternate
                        // path, we use that alternate path to load the Ruby
                        // source.
                        &alternate_path
                    } else {
                        path
                    }
                }
            }
        };
        let contents = self.read_source_file_contents(path)?.into_owned();
        self.eval(contents.as_ref())?;
        let state = self.state.as_deref_mut().ok_or_else(InterpreterExtractError::new)?;
        state.vfs.mark_required(path.as_ref())?;
        trace!("Successful require of {}", path.display());
        Ok(true)
    }

    fn read_source_file_contents<P>(&self, path: P) -> Result<Cow<'_, [u8]>, Self::Error>
    where
        P: AsRef<Path>,
    {
        let state = self.state.as_deref().ok_or_else(InterpreterExtractError::new)?;
        let path = path.as_ref();
        let contents = match state.vfs.read_file(path) {
            Ok(contents) => contents,
            // If we failed to read the source file and it already has a `.rb`
            // extension, return the error since there are no alternative paths
            // we can try.
            Err(err) if matches!(path.extension(), Some(ext) if *ext == *OsStr::new(RUBY_EXTENSION)) => {
                return Err(err.into());
            }
            // Retry the read with an alternative file path that has a `.rb`
            // extension.
            Err(_) => {
                let mut path = path.to_owned();
                path.set_extension(RUBY_EXTENSION);
                state.vfs.read_file(&path)?
            }
        };
        Ok(contents.to_vec().into())
    }
}
