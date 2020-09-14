#![allow(clippy::needless_pass_by_value)]

use crate::release_metadata::ReleaseMetadata;

pub mod core;
pub mod prelude;
pub mod stdlib;

use prelude::*;

pub const INPUT_RECORD_SEPARATOR: &str = "\n";

pub fn init(interp: &mut Artichoke, config: ReleaseMetadata<'_>) -> InitializeResult<()> {
    let copyright = interp.convert_mut(config.ruby_copyright());
    interp.define_global_constant("RUBY_COPYRIGHT", copyright)?;

    let description = interp.convert_mut(config.ruby_description());
    interp.define_global_constant("RUBY_DESCRIPTION", description)?;

    let engine = interp.convert_mut(config.ruby_engine());
    interp.define_global_constant("RUBY_ENGINE", engine)?;

    let engine_version = interp.convert_mut(config.ruby_engine_version());
    interp.define_global_constant("RUBY_ENGINE_VERSION", engine_version)?;

    let patchlevel = config
        .ruby_patchlevel()
        .parse::<Int>()
        .map_err(|_| NotDefinedError::global_constant("RUBY_PATCHLEVEL"))?;
    let patchlevel = interp.convert(patchlevel);
    interp.define_global_constant("RUBY_PATCHLEVEL", patchlevel)?;

    let platform = interp.convert_mut(config.ruby_platform());
    interp.define_global_constant("RUBY_PLATFORM", platform)?;

    let release_date = interp.convert_mut(config.ruby_release_date());
    interp.define_global_constant("RUBY_RELEASE_DATE", release_date)?;

    let revision = config
        .ruby_revision()
        .parse::<Int>()
        .map_err(|_| NotDefinedError::global_constant("RUBY_REVISION"))?;
    let revision = interp.convert(revision);
    interp.define_global_constant("RUBY_REVISION", revision)?;

    let version = interp.convert_mut(config.ruby_version());
    interp.define_global_constant("RUBY_VERSION", version)?;

    let compiler_version = interp.convert_mut(config.artichoke_compiler_version());
    interp.define_global_constant("ARTICHOKE_COMPILER_VERSION", compiler_version)?;

    core::init(interp)?;
    stdlib::init(interp)?;
    Ok(())
}
