use std::ffi::CStr;

use crate::extn::core::float::Float;
use crate::extn::prelude::*;

const FLOAT_CSTR: &CStr = cstr::cstr!("Float");
static FLOAT_RUBY_SOURCE: &[u8] = include_bytes!("float.rb");

pub fn init(interp: &mut Artichoke) -> InitializeResult<()> {
    if interp.is_class_defined::<Float>() {
        return Ok(());
    }

    let spec = class::Spec::new("Float", FLOAT_CSTR, None, None)?;
    interp.def_class::<Float>(spec)?;
    interp.eval(FLOAT_RUBY_SOURCE)?;

    let dig = interp.convert(Float::DIG);
    interp.define_class_constant::<Float>("DIG", dig)?;
    let epsilon = interp.convert_mut(Float::EPSILON);
    interp.define_class_constant::<Float>("EPSILON", epsilon)?;
    let infinity = interp.convert_mut(Float::INFINITY);
    interp.define_class_constant::<Float>("INFINITY", infinity)?;
    let mant_dig = interp.convert(Float::MANT_DIG);
    interp.define_class_constant::<Float>("MANT_DIG", mant_dig)?;
    let max = interp.convert_mut(Float::MAX);
    interp.define_class_constant::<Float>("MAX", max)?;
    let max_10_exp = interp.convert(Float::MAX_10_EXP);
    interp.define_class_constant::<Float>("MAX_10_EXP", max_10_exp)?;
    let max_exp = interp.convert(Float::MAX_EXP);
    interp.define_class_constant::<Float>("MAX_EXP", max_exp)?;
    let min = interp.convert_mut(Float::MIN);
    interp.define_class_constant::<Float>("MIN", min)?;
    let min_10_exp = interp.convert(Float::MIN_10_EXP);
    interp.define_class_constant::<Float>("MIN_10_EXP", min_10_exp)?;
    let min_exp = interp.convert(Float::MIN_EXP);
    interp.define_class_constant::<Float>("MIN_EXP", min_exp)?;
    let nan = interp.convert_mut(Float::NAN);
    interp.define_class_constant::<Float>("NAN", nan)?;
    let radix = interp.convert(Float::RADIX);
    interp.define_class_constant::<Float>("RADIX", radix)?;
    let rounds = interp.convert(Float::ROUNDS);
    interp.define_class_constant::<Float>("ROUNDS", rounds)?;

    Ok(())
}
