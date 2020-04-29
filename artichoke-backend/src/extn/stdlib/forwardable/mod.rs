use crate::extn::prelude::*;

pub fn init(interp: &mut Artichoke) -> InitializeResult<()> {
    let spec = module::Spec::new(interp, "Forwardable", None)?;
    interp.0.borrow_mut().def_module::<Forwardable>(spec);
    interp.def_rb_source_file(
        "forwardable.rb",
        &include_bytes!("vendor/forwardable.rb")[..],
    )?;
    interp.def_rb_source_file(
        "forwardable/impl.rb",
        &include_bytes!("vendor/forwardable/impl.rb")[..],
    )?;
    Ok(())
}

#[derive(Debug)]
pub struct Forwardable;

#[cfg(test)]
mod tests {
    use crate::test::prelude::*;

    #[test]
    // TODO(GH-528): fix failing tests on Windows.
    #[cfg_attr(target_os = "windows", should_panic)]
    fn integration_test() {
        let mut interp = crate::interpreter().unwrap();
        let _ = interp
            .eval(&include_bytes!("forwardable_test.rb")[..])
            .unwrap();
        let result = interp.eval(b"spec");
        let result = result.unwrap().try_into::<bool>(&interp).unwrap();
        assert!(result);
    }
}
