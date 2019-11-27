use artichoke_backend::convert::Convert;
use artichoke_backend::top_self::TopSelf;
use artichoke_backend::{Artichoke, ArtichokeError};
use artichoke_core::eval::Eval;
use artichoke_core::load::LoadSources;
use artichoke_core::value::Value;
use std::borrow::Cow;

pub fn init(interp: &Artichoke) -> Result<(), ArtichokeError> {
    for source in Sources::iter() {
        let content = Sources::get(&source).unwrap();
        interp.def_rb_source_file(source.as_bytes(), content)?;
    }
    Ok(())
}

#[derive(RustEmbed)]
#[folder = "$OUT_DIR/mspec/lib"]
struct Sources;

#[derive(Debug)]
pub struct Runner {
    specs: Vec<String>,
    interp: Artichoke,
    enforce: bool,
}

impl Runner {
    pub fn new(interp: Artichoke) -> Self {
        Self {
            specs: vec![],
            interp,
            enforce: true,
        }
    }

    pub fn add_spec<T>(&mut self, source: &str, contents: T) -> Result<(), ArtichokeError>
    where
        T: Into<Cow<'static, [u8]>>,
    {
        if !source.contains("/fixtures/") && !source.contains("/shared/") {
            self.specs.push(source.to_owned());
        }
        self.interp.def_rb_source_file(source.as_bytes(), contents)
    }

    pub fn run(self) -> Result<bool, ArtichokeError> {
        init(&self.interp).unwrap();
        self.interp
            .def_rb_source_file(b"/src/spec_helper.rb", &b""[..])?;
        self.interp
            .def_rb_source_file(b"/src/lib/spec_helper.rb", &b""[..])?;
        self.interp.def_rb_source_file(
            b"/src/test/spec_runner",
            &include_bytes!("spec_runner.rb")[..],
        )?;
        if let Err(err) = self.interp.eval(b"require '/src/test/spec_runner'") {
            eprintln!("{}", err);
            assert!(!self.enforce);
        }
        let specs = self.interp.convert(self.specs);
        self.interp
            .top_self()
            .funcall::<bool>("run_specs", &[specs], None)
    }
}

#[cfg(test)]
mod tests {
    use crate::mspec::Runner;

    #[test]
    fn mspec_framework_loads() {
        let interp = artichoke_backend::interpreter().expect("init");
        // should not panic
        assert_eq!(Runner::new(interp).run(), Ok(true));
    }
}
