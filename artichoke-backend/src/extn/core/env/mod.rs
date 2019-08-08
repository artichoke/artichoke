use crate::def::{ClassLike, Define};

use crate::eval::Eval;
use crate::sys;
use crate::Artichoke;
use crate::ArtichokeError;
use log::trace;

mod backends;
mod env_object;
mod errors;
use backends::EnvStdBackend;
use env_object::{Env, RubyEnvNativeApi};

pub fn patch(interp: &Artichoke) -> Result<(), ArtichokeError> {
    if interp.borrow().class_spec::<Env<EnvStdBackend>>().is_some() {
        return Ok(());
    }

    let env = interp
        .borrow_mut()
        .def_class::<Env<EnvStdBackend>>("EnvClass", None, None);

    env.borrow_mut()
        .add_method("[]", Env::<EnvStdBackend>::get, sys::mrb_args_req(1));
    env.borrow_mut()
        .add_method("[]=", Env::<EnvStdBackend>::set, sys::mrb_args_req(2));

    env.borrow()
        .define(interp)
        .map_err(|_| ArtichokeError::New)?;

    interp.eval(include_str!("env.rb"))?;

    trace!("Patched ENV onto interpreter");

    Ok(())
}
