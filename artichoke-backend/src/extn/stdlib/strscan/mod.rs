use crate::extn::prelude::*;

pub fn init(interp: &mut Artichoke) -> InitializeResult<()> {
    let spec = class::Spec::new("StringScanner", None, None)?;
    interp.def_class::<StringScanner>(spec)?;
    interp.def_rb_source_file("strscan.rb", &include_bytes!("strscan.rb")[..])?;
    Ok(())
}

#[derive(Debug)]
pub struct StringScanner;

#[cfg(test)]
mod tests {
    use crate::test::prelude::*;

    #[test]
    fn integration_test() {
        let mut interp = crate::interpreter().expect("init");
        interp.eval(&include_bytes!("strscan_test.rb")[..]).unwrap();
        let result = interp.eval(b"spec");
        let result = result.unwrap().try_into::<bool>(&interp).unwrap();
        assert!(result);
    }
}
