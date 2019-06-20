use std::borrow::Cow;
use std::env;

use crate::convert::FromMrb;
use crate::eval::MrbEval;
use crate::interpreter::{Mrb, MrbApi};
use crate::load::MrbLoadSources;
use crate::value::{Value, ValueLike};
use crate::MrbError;

pub const ENFORCE_RUBY_SPECS: &str = "MRUBY_ENFORCE_RUBY_SPECS";

pub fn init(interp: &Mrb) -> Result<(), MrbError> {
    interp.def_rb_source_file("mspec.rb", include_str!("mspec.rb"))?;
    for source in MSpec::iter() {
        let content = MSpec::get(&source).map(Cow::into_owned).unwrap();
        interp.def_rb_source_file(format!("mspec/{}", source), content)?;
    }
    Ok(())
}

#[derive(RustEmbed)]
#[folder = "mruby/src/extn/test/mspec/"]
pub struct MSpec;

impl MSpec {
    pub fn runner(interp: Mrb) -> MSpecRunner {
        MSpecRunner {
            specs: vec![],
            interp,
            enforce: true,
        }
    }
}

#[derive(Debug)]
pub struct MSpecRunner {
    specs: Vec<String>,
    interp: Mrb,
    enforce: bool,
}

impl MSpecRunner {
    pub fn add_spec<T: AsRef<[u8]>>(&mut self, source: &str, contents: T) -> Result<(), MrbError> {
        self.specs.push(source.to_owned());
        self.interp.def_rb_source_file(source, contents.as_ref())
    }

    pub fn mark_known_failing(&mut self) {
        self.enforce = env::var(ENFORCE_RUBY_SPECS).is_ok();
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
    use crate::extn::test::mspec::MSpec;
    use crate::interpreter::Interpreter;

    #[test]
    fn mspec_framework_loads() {
        let interp = Interpreter::create().expect("mrb init");
        // should not panic
        MSpec::runner(interp).run();
    }
}
