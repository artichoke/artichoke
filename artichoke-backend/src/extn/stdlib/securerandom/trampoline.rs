use crate::extn::prelude::*;
use crate::extn::stdlib::securerandom;

#[inline]
pub fn alphanumeric(interp: &mut Artichoke, len: Option<Value>) -> Result<Value, Exception> {
    let alpha = securerandom::alphanumeric(interp, len)?;
    Ok(interp.convert_mut(alpha))
}

#[inline]
pub fn base64(interp: &mut Artichoke, len: Option<Value>) -> Result<Value, Exception> {
    let base64 = securerandom::base64(interp, len)?;
    Ok(interp.convert_mut(base64))
}

#[inline]
pub fn hex(interp: &mut Artichoke, len: Option<Value>) -> Result<Value, Exception> {
    let hex = securerandom::hex(interp, len)?;
    Ok(interp.convert_mut(hex))
}

#[inline]
pub fn random_bytes(interp: &mut Artichoke, len: Option<Value>) -> Result<Value, Exception> {
    let bytes = securerandom::random_bytes(interp, len)?;
    Ok(interp.convert_mut(bytes))
}

#[inline]
pub fn random_number(interp: &mut Artichoke, max: Option<Value>) -> Result<Value, Exception> {
    let max = interp.try_convert(max)?;
    let num = securerandom::random_number(interp, max)?;
    Ok(interp.convert_mut(num))
}

#[inline]
pub fn uuid(interp: &mut Artichoke) -> Result<Value, Exception> {
    let uuid = securerandom::uuid(interp);
    Ok(interp.convert_mut(uuid))
}
