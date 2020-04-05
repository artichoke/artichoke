use crate::extn::prelude::*;

pub fn init(interp: &mut Artichoke) -> InitializeResult<()> {
    let spec = module::Spec::new(interp, "JSON", None)?;
    interp.0.borrow_mut().def_module::<Json>(spec);
    // NOTE(lopopolo): This setup of the JSON gem in the vfs does not include
    // any of the `json/add` sources for serializing "extra" types like `Time`
    // and `BigDecimal`, not all of which Artichoke supports.
    interp.def_rb_source_file(b"json.rb", &include_bytes!("vendor/json.rb")[..])?;
    interp.def_rb_source_file(
        b"json/common.rb",
        &include_bytes!("vendor/json/common.rb")[..],
    )?;
    interp.def_rb_source_file(
        b"json/generic_object.rb",
        &include_bytes!("vendor/json/generic_object.rb")[..],
    )?;
    interp.def_rb_source_file(
        b"json/version.rb",
        &include_bytes!("vendor/json/version.rb")[..],
    )?;
    interp.def_rb_source_file(b"json/pure.rb", &include_bytes!("vendor/json/pure.rb")[..])?;
    interp.def_rb_source_file(
        b"json/pure/generator.rb",
        &include_bytes!("vendor/json/pure/generator.rb")[..],
    )?;
    interp.def_rb_source_file(
        b"json/pure/parser.rb",
        &include_bytes!("vendor/json/pure/parser.rb")[..],
    )?;
    Ok(())
}

#[derive(Debug)]
pub struct Json;

#[cfg(test)]
mod tests {
    use crate::test::prelude::*;

    #[test]
    // TODO(GH-528): fix failing tests on Windows.
    #[cfg_attr(target_os = "windows", should_panic)]
    fn integration_test() {
        let mut interp = crate::interpreter().unwrap();
        let _ = interp.eval(&include_bytes!("json_test.rb")[..]).unwrap();
        let result = interp.eval(b"spec");
        let result = result.unwrap().try_into::<bool>(&mut interp).unwrap();
        assert!(result);
    }
}
