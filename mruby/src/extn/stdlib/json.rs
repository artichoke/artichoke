use crate::interpreter::Mrb;
use crate::load::MrbLoadSources;
use crate::MrbError;

pub fn init(interp: &Mrb) -> Result<(), MrbError> {
    interp.def_rb_source_file("json.rb", include_str!("json.rb"))?;
    interp.def_rb_source_file("json/common.rb", include_str!("json/common.rb"))?;
    interp.def_rb_source_file(
        "json/generic_object.rb",
        include_str!("json/generic_object.rb"),
    )?;
    interp.def_rb_source_file("json/version.rb", include_str!("json/version.rb"))?;
    interp.def_rb_source_file("json/pure.rb", include_str!("json/pure.rb"))?;
    interp.def_rb_source_file(
        "json/pure/generator.rb",
        include_str!("json/pure/generator.rb"),
    )?;
    interp.def_rb_source_file("json/pure/parser.rb", include_str!("json/pure/parser.rb"))?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::extn::test::mspec::MSpec;
    use crate::interpreter::Interpreter;

    #[test]
    fn json_doc_spec() {
        let interp = Interpreter::create().expect("mrb init");
        let mut runner = MSpec::runner(interp);
        runner
            .add_spec(
                "json/json_doc_spec.rb",
                include_str!("spec/json_doc_spec.rb"),
            )
            .unwrap();
        assert_eq!(runner.run(), Ok(true));
    }
}
