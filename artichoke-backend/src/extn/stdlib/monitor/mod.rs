use crate::extn::prelude::*;

pub fn init(interp: &mut Artichoke) -> InitializeResult<()> {
    let spec = class::Spec::new("Monitor", None, None)?;
    interp.0.borrow_mut().def_class::<Monitor>(spec);
    interp.def_rb_source_file(b"monitor.rb", &include_bytes!("vendor/monitor.rb")[..])?;
    Ok(())
}

#[derive(Debug)]
pub struct Monitor;

#[cfg(test)]
mod tests {
    use crate::test::prelude::*;

    #[test]
    fn integration_test() {
        let mut interp = crate::interpreter().unwrap();
        let _ = interp.eval(&include_bytes!("monitor_test.rb")[..]).unwrap();
        let result = interp.eval(b"spec");
        let result = result.unwrap().try_into::<bool>(&interp).unwrap();
        assert!(result);
    }
}
