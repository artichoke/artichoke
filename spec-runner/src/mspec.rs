use artichoke_backend::{Artichoke, BootError, ConvertMut, Eval, LoadSources, TopSelf, ValueLike};
use std::borrow::Cow;

pub fn init(interp: &mut Artichoke) -> Result<(), BootError> {
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

    pub fn add_spec<T>(&mut self, source: &str, contents: T) -> Result<(), BootError>
    where
        T: Into<Cow<'static, [u8]>>,
    {
        if !source.contains("/fixtures/") && !source.contains("/shared/") {
            self.specs.push(source.to_owned());
        }
        self.interp
            .def_rb_source_file(source.as_bytes(), contents)?;
        Ok(())
    }

    pub fn run(mut self) -> Result<bool, BootError> {
        init(&mut self.interp).unwrap();
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
        let specs = self.interp.convert_mut(self.specs);
        let result = self
            .interp
            .top_self()
            .funcall::<bool>("run_specs", &[specs], None)?;
        Ok(result)
    }
}

#[cfg(test)]
mod tests {
    use crate::mspec::Runner;

    #[test]
    fn mspec_framework_loads() {
        let interp = artichoke_backend::interpreter().expect("init");
        // should not panic
        assert!(Runner::new(interp).run().unwrap());
    }
}
