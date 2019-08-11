use log::trace;
use std::error;
use std::fmt;

use crate::def::{ClassLike, Define};
use crate::eval::Eval;
use crate::sys;
use crate::{Artichoke, ArtichokeError};

mod backends;
mod env_object;

use backends::{EnvBackend, EnvStdBackend};
use env_object::{Env, RubyEnvNativeApi};

pub fn init(interp: &Artichoke) -> Result<(), ArtichokeError> {
    init_internal_with_backend::<EnvStdBackend>(interp)
}

fn init_internal_with_backend<T: EnvBackend>(interp: &Artichoke) -> Result<(), ArtichokeError>
where
    T: 'static,
{
    if interp.borrow().class_spec::<Env<T>>().is_some() {
        return Ok(());
    }

    let env = interp
        .borrow_mut()
        .def_class::<Env<T>>("EnvClass", None, None);

    env.borrow_mut().mrb_value_is_rust_backed(true);

    env.borrow_mut()
        .add_method("initialize", Env::<T>::initialize, sys::mrb_args_none());
    env.borrow_mut()
        .add_method("[]", Env::<T>::get, sys::mrb_args_req(1));
    env.borrow_mut()
        .add_method("[]=", Env::<T>::set, sys::mrb_args_req(2));
    env.borrow_mut()
        .add_method("to_h", Env::<T>::env_to_h, sys::mrb_args_none());

    env.borrow()
        .define(interp)
        .map_err(|_| ArtichokeError::New)?;

    interp.eval(include_str!("env.rb"))?;

    trace!("Patched ENV onto interpreter");

    Ok(())
}

#[derive(Clone, Eq, PartialEq, Hash)]
pub enum Error {
    Fatal,
    NameContainsNullByte,
    Os(String), // should this be a `Value`?
    ValueContainsNullByte,
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Error::Fatal => write!(f, "fatal ENV error"),
            Error::NameContainsNullByte => {
                write!(f, "bad environment variable name: contains null byte")
            }
            Error::Os(arg) => write!(f, "Errno::EINVAL (Invalid argument - setenv({}))", arg),
            Error::ValueContainsNullByte => {
                write!(f, "bad environment variable value: contains null byte")
            }
        }
    }
}

impl fmt::Debug for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Error::Fatal => write!(f, "RuntimeError: {}", self),
            Error::NameContainsNullByte | Error::ValueContainsNullByte => {
                write!(f, "ArgumentError: {}", self)
            }
            Error::Os(arg) => write!(f, "Errno::EINVAL (Invalid argument - setenv({}))", arg),
        }
    }
}

impl error::Error for Error {
    fn description(&self) -> &str {
        "ENV error"
    }

    fn cause(&self) -> Option<&dyn error::Error> {
        None
    }
}
