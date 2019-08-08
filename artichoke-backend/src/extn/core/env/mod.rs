use crate::def::{ClassLike, Define};

use crate::eval::Eval;
use crate::sys;
use crate::Artichoke;
use crate::ArtichokeError;
use log::trace;

mod env_object;
mod errors;
use env_object::{Env, RubyEnvNativeApi};

pub fn patch(interp: &Artichoke) -> Result<(), ArtichokeError> {
    if interp.borrow().class_spec::<Env>().is_some() {
        return Ok(());
    }

    let env = interp.borrow_mut().def_class::<Env>("EnvClass", None, None);

    env.borrow_mut()
        .add_method("[]", Env::get, sys::mrb_args_req(1));
    env.borrow_mut()
        .add_method("[]=", Env::set, sys::mrb_args_req(2));

    env.borrow()
        .define(interp)
        .map_err(|_| ArtichokeError::New)?;

    interp.eval(include_str!("env.rb"))?;

    trace!("Patched ENV onto interpreter");

    Ok(())
}
