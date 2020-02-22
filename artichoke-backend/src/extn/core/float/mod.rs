use crate::extn::prelude::*;
use crate::types;

pub fn init(interp: &mut Artichoke) -> InitializeResult<()> {
    if interp.0.borrow().class_spec::<Float>().is_some() {
        return Ok(());
    }
    let spec = class::Spec::new("Float", None, None)?;
    interp.0.borrow_mut().def_class::<Float>(spec);
    let _ = interp.eval(&include_bytes!("float.rb")[..])?;
    let epsilon = interp.convert_mut(Float::EPSILON);
    interp.define_class_constant::<Float>("EPSILON", epsilon)?;
    trace!("Patched Float onto interpreter");
    Ok(())
}

#[derive(Debug)]
pub struct Float;

impl Float {
    pub const EPSILON: types::Float = std::f64::EPSILON;
    pub const INFINITY: types::Float = std::f64::INFINITY;
    pub const NEG_INFINITY: types::Float = std::f64::NEG_INFINITY;
    pub const NAN: types::Float = std::f64::NAN;
}
