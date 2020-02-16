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

pub fn init(interp: &mut Artichoke, backend_name: &str) -> InitializeResult<()> {
    let copyright = interp.convert_mut(RUBY_COPYRIGHT);
    interp.define_global_constant("RUBY_COPYRIGHT", copyright)?;

    let description = interp.convert_mut(RUBY_DESCRIPTION);
    interp.define_global_constant("RUBY_DESCRIPTION", description)?;

    let mut engine = String::from(RUBY_ENGINE);
    engine.push('-');
    engine.push_str(backend_name);
    let engine = interp.convert_mut(engine);
    interp.define_global_constant("RUBY_ENGINE", engine)?;

    let engine_version = interp.convert_mut(RUBY_ENGINE_VERSION);
    interp.define_global_constant("RUBY_ENGINE_VERSION", engine_version)?;

    let patchlevel = RUBY_PATCHLEVEL
        .parse::<Int>()
        .map_err(|_| NotDefinedError::global_constant("RUBY_PATCHLEVEL"))?;
    let patchlevel = interp.convert(patchlevel);
    interp.define_global_constant("RUBY_PATCHLEVEL", patchlevel)?;

    let platform = interp.convert_mut(RUBY_PLATFORM);
    interp.define_global_constant("RUBY_PLATFORM", platform)?;

    let release_date = interp.convert_mut(RUBY_RELEASE_DATE);
    interp.define_global_constant("RUBY_RELEASE_DATE", release_date)?;

    let revision = RUBY_REVISION
        .parse::<Int>()
        .map_err(|_| NotDefinedError::global_constant("RUBY_REVISION"))?;
    let revision = interp.convert(revision);
    interp.define_global_constant("RUBY_REVISION", revision)?;

    let version = interp.convert_mut(RUBY_VERSION);
    interp.define_global_constant("RUBY_VERSION", version)?;

    let compiler_version = interp.convert_mut(ARTICHOKE_COMPILER_VERSION);
    interp.define_global_constant("ARTICHOKE_COMPILER_VERSION", compiler_version)?;

    core::init(interp)?;
    stdlib::init(interp)?;
    Ok(())
}
