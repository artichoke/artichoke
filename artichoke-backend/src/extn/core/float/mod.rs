use artichoke_core::eval::Eval;

use crate::class;
use crate::types;
use crate::{Artichoke, BootError};

pub fn init(interp: &Artichoke) -> Result<(), BootError> {
    if interp.0.borrow().class_spec::<Float>().is_some() {
        return Ok(());
    }
    let spec = class::Spec::new("Float", None, None)?;
    interp.0.borrow_mut().def_class::<Float>(spec);
    let _ = interp.eval(&include_bytes!("float.rb")[..])?;
    // TODO: Add proper constant defs to class::Spec, see GH-158.
    let _ = interp.eval(format!("class Float; EPSILON={} end", Float::EPSILON).as_bytes())?;
    trace!("Patched Float onto interpreter");
    Ok(())
}

pub struct Float;

impl Float {
    pub const EPSILON: types::Float = std::f64::EPSILON;
    pub const INFINITY: types::Float = std::f64::INFINITY;
    pub const NEG_INFINITY: types::Float = std::f64::NEG_INFINITY;
    pub const NAN: types::Float = std::f64::NAN;
}
