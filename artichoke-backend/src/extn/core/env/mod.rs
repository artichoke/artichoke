use crate::def::{ClassLike, Define};

use crate::eval::Eval;
use crate::sys;
use crate::Artichoke;
use crate::ArtichokeError;
use log::trace;

mod backends;
mod env_object;
mod errors;
use backends::{EnvBackend, EnvStdBackend};
use env_object::{Env, RubyEnvNativeApi};

pub fn patch(interp: &Artichoke) -> Result<(), ArtichokeError> {
    patch_internal_with_backend::<EnvStdBackend>(interp)
}

fn patch_internal_with_backend<T: EnvBackend>(interp: &Artichoke) -> Result<(), ArtichokeError>
where
    T: 'static,
{
    if interp.borrow().class_spec::<Env<T>>().is_some() {
        return Ok(());
    }

    let env = interp
        .borrow_mut()
        .def_class::<Env<T>>("EnvClass", None, None);

    env.borrow_mut()
        .add_method("[]", Env::<T>::get, sys::mrb_args_req(1));
    env.borrow_mut()
        .add_method("[]=", Env::<T>::set, sys::mrb_args_req(2));
    env.borrow_mut()
        .add_method("to_h", Env::<T>::to_h, sys::mrb_args_none());

    env.borrow()
        .define(interp)
        .map_err(|_| ArtichokeError::New)?;

    interp.eval(include_str!("env.rb"))?;

    trace!("Patched ENV onto interpreter");

    Ok(())
}
