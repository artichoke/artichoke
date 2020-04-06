use crate::extn::prelude::*;

pub fn init(interp: &mut Artichoke) -> InitializeResult<()> {
    if interp.0.borrow().class_spec::<Thread>().is_some() {
        return Ok(());
    }
    if interp.0.borrow().class_spec::<Mutex>().is_some() {
        return Ok(());
    }
    let spec = class::Spec::new("Thread", None, None)?;
    interp.0.borrow_mut().def_class::<Thread>(spec);
    let spec = class::Spec::new("Mutex", None, None)?;
    interp.0.borrow_mut().def_class::<Mutex>(spec);
    interp.def_rb_source_file(b"thread.rb", &include_bytes!("thread.rb")[..])?;
    // Thread is loaded by default, so eval it on interpreter initialization
    // https://www.rubydoc.info/gems/rubocop/RuboCop/Cop/Lint/UnneededRequireStatement
    let _ = interp.eval(&b"require 'thread'"[..])?;
    trace!("Patched Thread onto interpreter");
    trace!("Patched Mutex onto interpreter");
    Ok(())
}

#[derive(Debug)]
pub struct Thread;

#[derive(Debug)]
pub struct Mutex;

#[cfg(test)]
mod tests {
    use crate::test::prelude::*;

    #[test]
    fn integration_test() {
        let mut interp = crate::interpreter().unwrap();
        let _ = interp.eval(&include_bytes!("thread_test.rb")[..]).unwrap();
        let result = interp.eval(b"spec");
        let result = result.unwrap().try_into::<bool>(&interp).unwrap();
        assert!(result);
    }
}
