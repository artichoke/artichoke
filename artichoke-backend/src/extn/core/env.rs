use log::trace;

use crate::def::{ClassLike, Define};
use crate::eval::Eval;
use crate::sys;
use crate::Artichoke;
use crate::ArtichokeError;
use std::collections::HashMap;

pub fn patch(interp: &Artichoke) -> Result<(), ArtichokeError> {
    if interp.borrow().class_spec::<Env>().is_some() {
        return Ok(());
    }

    let env = interp.borrow_mut().def_class::<Env>("ENV", None, None);
    env.borrow_mut().add_method(
        "initialize_internal",
        Env::initialize_internal,
        sys::mrb_args_none(),
    );

    interp.eval(include_str!("env.rb"))?;

    env.borrow()
        .define(interp)
        .map_err(|_| ArtichokeError::New)?;
    trace!("Patched ENV onto interpreter");

    Ok(())
}

#[derive(Debug, Clone, Default)]
pub struct Env {
    env: HashMap<String, String>,
}

impl Env {
    pub fn new(env: HashMap<String, String>) -> Self {
        Env { env }
    }

    unsafe extern "C" fn initialize_internal(
        mrb: *mut sys::mrb_state,
        _slf: sys::mrb_value,
    ) -> sys::mrb_value {
        let _interp = unwrap_interpreter!(mrb);

        // fill the ENV here

        println!("Initialize called");
        sys::mrb_sys_nil_value()
    }
}

#[cfg(test)]
mod tests {
    use crate::eval::Eval;
    use crate::extn::core::env;

    #[test]
    fn test_env_initialized() {
        let interp = crate::interpreter().expect("init");
        env::patch(&interp).expect("env init");
    }

    #[test]
    #[allow(non_snake_case)]
    fn test_env_has_PATH() {
        let interp = crate::interpreter().expect("init");
        env::patch(&interp).expect("env init");

        let PATH_variable_value: String = (&interp)
            .eval(r"ENV['PATH']")
            .unwrap()
            .try_into::<String>()
            .unwrap();

        assert_eq!(PATH_variable_value.is_empty(), false);
    }
}
