use std::borrow::Cow;
use std::error;
use std::fmt;
use std::path::Path;

use crate::core::load::State;
use crate::exception::{Exception, RubyException};
use crate::extn::core::exception::Fatal;
use crate::fs::{ExtensionHook, RUBY_LOAD_PATH};
use crate::sys;
use crate::{Artichoke, ConvertMut, File, LoadSources};

impl LoadSources for Artichoke {
    type Artichoke = Self;

    type Error = Exception;

    type Exception = Exception;

    type Extension = ExtensionHook;

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

    fn source_is_file<P>(&self, path: P) -> Result<bool, Self::Error>
    where
        P: AsRef<Path>,
    {
        let is_file = self.0.borrow().vfs.is_file(path.as_ref());
        Ok(is_file)
    }

    fn source_require_state<P>(&self, path: P) -> Result<State, Self::Error>
    where
        P: AsRef<Path>,
    {
        let is_required = self.0.borrow().vfs.is_required(path.as_ref());
        if is_required {
            Ok(State::Required)
        } else {
            Ok(State::Default)
        }
    }

    fn set_source_require_state<P>(&mut self, path: P, next: State) -> Result<(), Self::Error>
    where
        P: AsRef<Path>,
    {
        let current = self.source_require_state(path.as_ref())?;
        match (current, next) {
            (State::Default, State::Default) | (State::Required, State::Required) => {
                // no transition. this is a no-op.
            }
            (State::Default, State::Required) => {
                self.0.borrow_mut().vfs.mark_required(path.as_ref())?;
            }
            _ => return Err(StateTransitionError::new(current, next).into()),
        }
        Ok(())
    }

    fn source_extension_hook<P>(&self, path: P) -> Result<Option<Self::Extension>, Self::Error>
    where
        P: AsRef<Path>,
    {
        let hook = self.0.borrow().vfs.get_extension(path.as_ref());
        Ok(hook)
    }

    fn read_source_file<P>(&self, path: P) -> Result<Cow<'_, [u8]>, Self::Error>
    where
        P: AsRef<Path>,
    {
        let borrow = self.0.borrow();
        let contents = borrow.vfs.read_file(path.as_ref())?;
        Ok(contents.to_vec().into())
    }
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub struct StateTransitionError {
    pub current: State,
    pub next: State,
}

impl StateTransitionError {
    pub fn new(current: State, next: State) -> Self {
        Self { current, next }
    }
}

impl fmt::Display for StateTransitionError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Invalid state transition: cannot un-require a source file"
        )
    }
}

impl error::Error for StateTransitionError {}

impl RubyException for StateTransitionError {
    fn message(&self) -> &[u8] {
        &b"Invalid state transition: cannot un-require a source file"[..]
    }

    fn name(&self) -> String {
        String::from("fatal")
    }

    fn vm_backtrace(&self, interp: &mut Artichoke) -> Option<Vec<Vec<u8>>> {
        let _ = interp;
        None
    }

    fn as_mrb_value(&self, interp: &mut Artichoke) -> Option<sys::mrb_value> {
        let message = interp.convert_mut(self.message());
        let borrow = interp.0.borrow();
        let spec = borrow.class_spec::<Fatal>()?;
        let value = spec.new_instance(interp, &[message])?;
        Some(value.inner())
    }
}

impl From<StateTransitionError> for Exception {
    fn from(exception: StateTransitionError) -> Self {
        Self::from(Box::<dyn RubyException>::from(exception))
    }
}

impl From<Box<StateTransitionError>> for Exception {
    fn from(exception: Box<StateTransitionError>) -> Self {
        Self::from(Box::<dyn RubyException>::from(exception))
    }
}

impl From<StateTransitionError> for Box<dyn RubyException> {
    fn from(exception: StateTransitionError) -> Box<dyn RubyException> {
        Box::new(exception)
    }
}

impl From<Box<StateTransitionError>> for Box<dyn RubyException> {
    fn from(exception: Box<StateTransitionError>) -> Box<dyn RubyException> {
        exception
    }
}
