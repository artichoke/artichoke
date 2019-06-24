use std::borrow::Cow;

use mruby::convert::FromMrb;
use mruby::eval::MrbEval;
use mruby::interpreter::{Mrb, MrbApi};
use mruby::load::MrbLoadSources;
use mruby::value::{Value, ValueLike};
use mruby::MrbError;

pub fn init(interp: &Mrb) -> Result<(), MrbError> {
    interp.def_rb_source_file("mspec.rb", include_str!("mspec.rb"))?;
    for source in Sources::iter() {
        let content = Sources::get(&source).map(Cow::into_owned).unwrap();
        interp.def_rb_source_file(format!("mspec/{}", source), content)?;
    }
    Ok(())
}

#[derive(RustEmbed)]
#[folder = "spec-runner/spec/mspec/"]
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
        self.specs.push(source.to_owned());
        self.interp.def_rb_source_file(source, contents.as_ref())
    }

    pub fn run(self) -> Result<bool, MrbError> {
        init(&self.interp).unwrap();
        self.interp
            .def_rb_source_file("/src/spec_helper.rb", "")
            .unwrap();
        self.interp
            .def_rb_source_file("/src/test/spec_runner", include_str!("spec_runner.rb"))
            .unwrap();
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
    use mruby::extn::test::mspec::MSpec;
    use mruby::interpreter::Interpreter;

    #[test]
    fn mspec_framework_loads() {
        let interp = Interpreter::create().expect("mrb init");
        // should not panic
        assert_eq!(MSpec::runner(interp).run(), Ok(true));
    }
}
