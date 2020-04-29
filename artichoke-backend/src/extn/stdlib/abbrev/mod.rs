use crate::extn::prelude::*;

pub fn init(interp: &mut Artichoke) -> InitializeResult<()> {
    let spec = module::Spec::new(interp, "Abbrev", None)?;
    interp.0.borrow_mut().def_module::<Abbrev>(spec);
    interp.def_rb_source_file("abbrev.rb", &include_bytes!("vendor/abbrev.rb")[..])?;
    Ok(())
}

#[derive(Debug)]
pub struct Abbrev;

#[cfg(test)]
mod tests {
    use crate::test::prelude::*;

    #[test]
    fn integration_test() {
        let mut interp = crate::interpreter().unwrap();
        let _ = interp.eval(&include_bytes!("abbrev_test.rb")[..]).unwrap();
        let result = interp.eval(b"spec");
        let result = result.unwrap().try_into::<bool>(&interp).unwrap();
        assert!(result);
    }
}
