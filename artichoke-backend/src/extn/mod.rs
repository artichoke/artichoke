#![allow(clippy::needless_pass_by_value)]

pub mod core;
pub mod prelude;
pub mod stdlib;

use prelude::*;

pub const RUBY_COPYRIGHT: &str = env!("RUBY_COPYRIGHT");
pub const RUBY_DESCRIPTION: &str = env!("RUBY_DESCRIPTION");
pub const RUBY_ENGINE: &str = "artichoke";
pub const RUBY_ENGINE_VERSION: &str = env!("CARGO_PKG_VERSION");
pub const RUBY_PATCHLEVEL: &str = "0";
pub const RUBY_PLATFORM: &str = env!("RUBY_PLATFORM");
pub const RUBY_RELEASE_DATE: &str = env!("RUBY_RELEASE_DATE");
pub const RUBY_REVISION: &str = env!("RUBY_REVISION");
pub const RUBY_VERSION: &str = "2.6.3";

pub const ARTICHOKE_COMPILER_VERSION: &str = env!("ARTICHOKE_COMPILER_VERSION");

pub const INPUT_RECORD_SEPARATOR: &str = "\n";

macro_rules! global_const {
    ($interp:expr, $constant:ident) => {{
        let mrb = $interp.0.borrow().mrb;
        unsafe {
            sys::mrb_define_global_const(
                mrb,
                concat!(stringify!($constant), "\0").as_ptr() as *const i8,
                $interp.convert($constant).inner(),
            );
        }
    }};
    ($interp:expr, $constant:ident, $value:expr) => {{
        let mrb = $interp.0.borrow().mrb;
        unsafe {
            sys::mrb_define_global_const(
                mrb,
                concat!(stringify!($constant), "\0").as_ptr() as *const i8,
                $interp.convert($value).inner(),
            );
        }
    }};
    ($interp:expr, $constant:ident as Int) => {{
        let mrb = $interp.0.borrow().mrb;
        let constant = $constant.parse::<Int>().map_err(|_| ArtichokeError::New)?;
        unsafe {
            sys::mrb_define_global_const(
                mrb,
                concat!(stringify!($constant), "\0").as_ptr() as *const i8,
                $interp.convert(constant).inner(),
            );
        }
    }};
}

pub fn init(interp: &Artichoke, backend_name: &str) -> InitializeResult<()> {
    let mut engine = String::from(RUBY_ENGINE);
    engine.push('-');
    engine.push_str(backend_name);

    global_const!(interp, RUBY_COPYRIGHT);
    global_const!(interp, RUBY_DESCRIPTION);
    global_const!(interp, RUBY_ENGINE, engine);
    global_const!(interp, RUBY_ENGINE_VERSION);
    global_const!(interp, RUBY_PATCHLEVEL as Int);
    global_const!(interp, RUBY_PLATFORM);
    global_const!(interp, RUBY_RELEASE_DATE);
    global_const!(interp, RUBY_REVISION as Int);
    global_const!(interp, RUBY_VERSION);

    global_const!(interp, ARTICHOKE_COMPILER_VERSION);

    core::init(interp)?;
    stdlib::init(interp)?;
    Ok(())
}
