// This pragma is needed to allow passing `Value` by value in all the mruby
// and Rust trampolines.
#![allow(clippy::needless_pass_by_value)]

use crate::release_metadata::ReleaseMetadata;

pub mod core;
pub mod prelude;
pub mod stdlib;

use prelude::*;

pub const INPUT_RECORD_SEPARATOR: &str = "\n";

pub fn init(interp: &mut Artichoke, config: ReleaseMetadata<'_>) -> InitializeResult<()> {
    let mut copyright = interp.try_convert_mut(config.ruby_copyright())?;
    copyright.freeze(interp)?;
    interp.define_global_constant("RUBY_COPYRIGHT", copyright)?;

    let mut description = interp.try_convert_mut(config.ruby_description())?;
    description.freeze(interp)?;
    interp.define_global_constant("RUBY_DESCRIPTION", description)?;

    let mut engine = interp.try_convert_mut(config.ruby_engine())?;
    engine.freeze(interp)?;
    interp.define_global_constant("RUBY_ENGINE", engine)?;

    let mut engine_version = interp.try_convert_mut(config.ruby_engine_version())?;
    engine_version.freeze(interp)?;
    interp.define_global_constant("RUBY_ENGINE_VERSION", engine_version)?;

    let patchlevel = config
        .ruby_patchlevel()
        .parse::<i64>()
        .map_err(|_| NotDefinedError::global_constant("RUBY_PATCHLEVEL"))?;
    let patchlevel = interp.convert(patchlevel);
    interp.define_global_constant("RUBY_PATCHLEVEL", patchlevel)?;

    let mut platform = interp.try_convert_mut(config.ruby_platform())?;
    platform.freeze(interp)?;
    interp.define_global_constant("RUBY_PLATFORM", platform)?;

    let mut release_date = interp.try_convert_mut(config.ruby_release_date())?;
    release_date.freeze(interp)?;
    interp.define_global_constant("RUBY_RELEASE_DATE", release_date)?;

    let revision = config
        .ruby_revision()
        .parse::<i64>()
        .map_err(|_| NotDefinedError::global_constant("RUBY_REVISION"))?;
    let revision = interp.convert(revision);
    interp.define_global_constant("RUBY_REVISION", revision)?;

    let mut version = interp.try_convert_mut(config.ruby_version())?;
    version.freeze(interp)?;
    interp.define_global_constant("RUBY_VERSION", version)?;

    let mut compiler_version = interp.try_convert_mut(config.artichoke_compiler_version())?;
    compiler_version.freeze(interp)?;
    interp.define_global_constant("ARTICHOKE_COMPILER_VERSION", compiler_version)?;

    core::init(interp)?;
    stdlib::init(interp)?;
    Ok(())
}
