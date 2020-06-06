//! Embedded `MSpec` framework.

use std::path::Path;

use artichoke::prelude::core::*;
use artichoke::prelude::*;

/// Load `MSpec` sources into the Artichoke virtual filesystem.
///
/// # Errors
///
/// If an exception is raised on the Artichoke interpreter, it is returned.
pub fn init(interp: &mut Artichoke) -> Result<(), Exception> {
    for source in Sources::iter() {
        if let Some(content) = Sources::get(&source) {
            interp.def_rb_source_file(source.as_ref(), content)?;
        }
    }
    Ok(())
}

/// `MSpec` source code.
#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord, RustEmbed)]
#[folder = "vendor/mspec/lib"]
pub struct Sources;

/// Load the Artichoke `MSpec` entry point end execute the specs.
///
/// # Errors
///
/// If an exception is raised on the Artichoke interpreter, it is returned.
pub fn run<'a, T>(interp: &mut Artichoke, specs: T) -> Result<bool, Exception>
where
    T: IntoIterator<Item = &'a str>,
{
    interp.def_rb_source_file("/src/lib/spec_helper.rb", &b""[..])?;
    interp.def_rb_source_file(
        "/src/lib/test/spec_runner",
        &include_bytes!("spec_runner.rb")[..],
    )?;
    interp.eval_file(Path::new("/src/lib/test/spec_runner"))?;
    let specs = interp.try_convert_mut(specs.into_iter().collect::<Vec<_>>())?;
    let result = interp
        .top_self()
        .funcall(interp, "run_specs", &[specs], None)?;
    interp.try_convert(result)
}

#[cfg(test)]
mod tests {
    #[test]
    // TODO(GH-528): fix failing tests on Windows.
    #[cfg_attr(target_os = "windows", should_panic)]
    fn mspec_framework_loads() {
        let mut interp = artichoke::interpreter().unwrap();
        super::init(&mut interp).unwrap();
        // should not panic
        assert!(super::run(&mut interp, vec![]).unwrap());
    }
}
