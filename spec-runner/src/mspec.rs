use std::borrow::Cow;

use artichoke_backend::convert::Convert;
use artichoke_backend::eval::Eval;
use artichoke_backend::load::LoadSources;
use artichoke_backend::top_self::TopSelf;
use artichoke_backend::value::ValueLike;
use artichoke_backend::{Artichoke, ArtichokeError};

pub fn init(interp: &Artichoke) -> Result<(), ArtichokeError> {
    interp.def_rb_source_file("mspec.rb", include_str!("mspec.rb"))?;
    for source in Sources::iter() {
        let content = Sources::get(&source).map(Cow::into_owned).unwrap();
        interp.def_rb_source_file(source, content)?;
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

    pub fn add_spec<T: AsRef<[u8]>>(
        &mut self,
        source: &str,
        contents: T,
    ) -> Result<(), ArtichokeError> {
        if !source.contains("/fixtures/") && !source.contains("/shared/") {
            self.specs.push(source.to_owned());
        }
        self.interp.def_rb_source_file(source, contents.as_ref())
    }

    pub fn run(self) -> Result<bool, ArtichokeError> {
        init(&self.interp).unwrap();
        self.interp.def_rb_source_file("/src/spec_helper.rb", "")?;
        self.interp
            .def_rb_source_file("/src/test/spec_runner", include_str!("spec_runner.rb"))?;
        if let Err(err) = self.interp.eval("require '/src/test/spec_runner'") {
            println!("{}", err);
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
