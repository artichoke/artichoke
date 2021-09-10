use std::ffi::CStr;

use crate::extn::prelude::*;

const HASH_CSTR: &CStr = cstr::cstr!("Hash");

pub fn init(interp: &mut Artichoke) -> InitializeResult<()> {
    if interp.is_class_defined::<Hash>() {
        return Ok(());
    }
    let spec = class::Spec::new("Hash", HASH_CSTR, None, None)?;
    interp.def_class::<Hash>(spec)?;
    interp.eval(&include_bytes!("hash.rb")[..])?;
    trace!("Patched Hash onto interpreter");
    Ok(())
}

#[derive(Debug, Clone, Copy)]
pub struct Hash;

#[cfg(test)]
mod tests {
    use crate::test::prelude::*;

    #[test]
    fn regression_github_1099() {
        let mut interp = interpreter().unwrap();
        let inspect = interp.eval(b"{ a: 'GH-1099' }.inspect").unwrap();
        let inspect = inspect.try_convert_into_mut::<&str>(&mut interp).unwrap();
        assert_eq!(inspect, r#"{:a=>"GH-1099"}"#);
    }
}
