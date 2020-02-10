use crate::extn::prelude::*;

pub fn init(interp: &mut Artichoke) -> InitializeResult<()> {
    let spec = class::Spec::new("OpenStruct", None, None)?;
    interp.0.borrow_mut().def_class::<OpenStruct>(spec);
    interp.def_rb_source_file(b"ostruct.rb", &include_bytes!("ostruct.rb")[..])?;
    Ok(())
}

pub struct OpenStruct;
