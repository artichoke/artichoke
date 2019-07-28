use std::borrow::Cow;

use artichoke_backend::convert::FromMrb;
use artichoke_backend::eval::MrbEval;
use artichoke_backend::load::MrbLoadSources;
use artichoke_backend::top_self::MrbTopSelf;
use artichoke_backend::value::{Value, ValueLike};
use artichoke_backend::{Mrb, MrbError};

pub fn init(interp: &Mrb) -> Result<(), MrbError> {
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
    interp: Mrb,
    enforce: bool,
}

impl Runner {
    pub fn new(interp: Mrb) -> Self {
        Self {
            specs: vec![],
            interp,
            enforce: true,
        }
    }

    pub fn add_spec<T: AsRef<[u8]>>(&mut self, source: &str, contents: T) -> Result<(), MrbError> {
        if !source.contains("/fixtures/") && !source.contains("/shared/") {
            self.specs.push(source.to_owned());
        }
        self.interp.def_rb_source_file(source, contents.as_ref())
    }

    pub fn run(self) -> Result<bool, MrbError> {
        init(&self.interp).unwrap();
        self.interp.def_rb_source_file("/src/spec_helper.rb", "")?;
        self.interp
            .def_rb_source_file("/src/test/spec_runner", include_str!("spec_runner.rb"))?;
        if let Err(err) = self.interp.eval("require '/src/test/spec_runner'") {
            println!("{}", err);
            assert!(!self.enforce);
        }
        let specs = Value::from_mrb(&self.interp, self.specs);
        self.interp
            .top_self()
            .funcall::<bool, _, _>("run_specs", &[specs])
    }
}

#[cfg(test)]
mod tests {
    use crate::mspec::Runner;

    #[test]
    fn mspec_framework_loads() {
        let interp = mruby::interpreter().expect("mrb init");
        // should not panic
        assert_eq!(Runner::new(interp).run(), Ok(true));
    }
}
