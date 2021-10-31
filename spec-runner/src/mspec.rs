//! Embedded `MSpec` framework.

use std::path::Path;
use std::str::FromStr;

use artichoke::backend::load_path::RUBY_LOAD_PATH;
use artichoke::prelude::*;

/// Load `MSpec` sources into the Artichoke virtual file system.
///
/// # Errors
///
/// If an exception is raised on the Artichoke interpreter, it is returned.
pub fn init(interp: &mut Artichoke) -> Result<(), Error> {
    for source in Sources::iter() {
        if let Some(content) = Sources::get(&source) {
            interp.def_rb_source_file(&*source, content.data)?;
        }
    }
    Ok(())
}

/// `MSpec` source code.
#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord, RustEmbed)]
#[folder = "vendor/mspec/lib"]
pub struct Sources;

/// `MSpec` formatter strategy.
#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub enum Formatter {
    // Artichoke's `RSpec`-style dot and it block format.
    Artichoke,
    /// Output exceptions and summary information in plaintext readable format.
    Summary,
    /// `MSpec` tagging mode.
    Tagger,
    /// Output exceptions and spec summary information in YAML format.
    Yaml,
}

impl Default for Formatter {
    fn default() -> Self {
        Self::Artichoke
    }
}

impl FromStr for Formatter {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            _ if s.eq_ignore_ascii_case("Artichoke") => Ok(Self::Artichoke),
            _ if s.eq_ignore_ascii_case("Summary") => Ok(Self::Summary),
            _ if s.eq_ignore_ascii_case("Tagger") => Ok(Self::Tagger),
            _ if s.eq_ignore_ascii_case("Yaml") => Ok(Self::Yaml),
            _ => Err("invalid formatter"),
        }
    }
}

impl Formatter {
    fn into_ruby_class(self) -> &'static str {
        match self {
            Self::Artichoke => "Artichoke::Spec::Formatter::Artichoke",
            Self::Summary => "Artichoke::Spec::Formatter::Summary",
            Self::Tagger => "Artichoke::Spec::Formatter::Tagger",
            Self::Yaml => "Artichoke::Spec::Formatter::Yaml",
        }
    }
}

/// Load the Artichoke `MSpec` entry point end execute the specs.
///
/// # Errors
///
/// If an exception is raised on the Artichoke interpreter, it is returned.
pub fn run<'a, T>(interp: &mut Artichoke, formatter: Formatter, specs: T) -> Result<bool, Error>
where
    T: IntoIterator<Item = &'a str>,
{
    let virtual_root = Path::new(RUBY_LOAD_PATH);

    interp.def_rb_source_file("spec_helper.rb", &b""[..])?;
    interp.def_rb_source_file("spec_runner.rb", &include_bytes!("spec_runner.rb")[..])?;

    interp.eval_file(&virtual_root.join("spec_runner.rb"))?;

    let artichoke_spec_formatter = interp.eval(formatter.into_ruby_class().as_bytes())?;

    let specs = interp.try_convert_mut(specs.into_iter().collect::<Vec<_>>())?;

    let result = artichoke_spec_formatter.funcall(interp, "run_specs", &[specs], None)?;
    interp.try_convert(result)
}

#[cfg(test)]
mod tests {
    use super::{init, run, Formatter};

    #[test]
    fn mspec_framework_loads() {
        let mut interp = artichoke::interpreter().unwrap();
        init(&mut interp).unwrap();
        // should not panic
        assert!(run(&mut interp, Formatter::default(), vec![]).unwrap());
        interp.close();
    }

    #[test]
    fn artichoke_formatter_succeeds() {
        let mut interp = artichoke::interpreter().unwrap();
        init(&mut interp).unwrap();
        // should not panic
        assert!(run(&mut interp, Formatter::Artichoke, vec![]).unwrap());
        interp.close();
    }

    #[test]
    fn summary_formatter_succeeds() {
        let mut interp = artichoke::interpreter().unwrap();
        init(&mut interp).unwrap();
        // should not panic
        assert!(run(&mut interp, Formatter::Summary, vec![]).unwrap());
        interp.close();
    }

    #[test]
    fn tagger_formatter_succeeds() {
        let mut interp = artichoke::interpreter().unwrap();
        init(&mut interp).unwrap();
        // should not panic
        assert!(run(&mut interp, Formatter::Tagger, vec![]).unwrap());
        interp.close();
    }

    #[test]
    fn yaml_formatter_succeeds() {
        let mut interp = artichoke::interpreter().unwrap();
        init(&mut interp).unwrap();
        // should not panic
        assert!(run(&mut interp, Formatter::Yaml, vec![]).unwrap());
        interp.close();
    }
}
