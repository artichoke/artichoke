use crate::extn::prelude::*;

pub fn init(interp: &mut Artichoke) -> InitializeResult<()> {
    
    
    let spec = crate::module::Spec::new(interp, "Racc", None)?;
    interp.0.borrow_mut().def_module::<Racc>(spec);
    
    
    
    let spec = crate::class::Spec::new("ParseError", None, None)?;
    interp.0.borrow_mut().def_class::<ParseError>(spec);
    
    
    
    interp.def_rb_source_file(
        b"racc/parser.rb",
        &include_bytes!(concat!(env!("OUT_DIR"), "/src/generated/racc/parser.rb"))[..]
    )?;
    
    Ok(())
}

#[derive(Debug)]
pub struct Racc;


#[derive(Debug)]
pub struct ParseError;


