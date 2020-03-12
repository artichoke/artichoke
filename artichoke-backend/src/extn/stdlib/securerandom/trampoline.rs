use crate::extn::prelude::*;
use crate::extn::stdlib::securerandom;

pub fn alphanumeric(interp: &mut Artichoke, len: Option<Value>) -> Result<Value, Exception> {
    securerandom::alphanumeric(interp, len).map(|bytes| interp.convert_mut(bytes))
}

pub fn base64(interp: &mut Artichoke, len: Option<Value>) -> Result<Value, Exception> {
    securerandom::base64(interp, len).map(|bytes| interp.convert_mut(bytes))
}

pub fn hex(interp: &mut Artichoke, len: Option<Value>) -> Result<Value, Exception> {
    securerandom::hex(interp, len).map(|bytes| interp.convert_mut(bytes))
}

pub fn random_bytes(interp: &mut Artichoke, len: Option<Value>) -> Result<Value, Exception> {
    securerandom::random_bytes(interp, len).map(|bytes| interp.convert_mut(bytes))
}

pub fn random_number(interp: &mut Artichoke, max: Option<Value>) -> Result<Value, Exception> {
    securerandom::random_number(interp, max)
}

pub fn uuid(interp: &mut Artichoke) -> Result<Value, Exception> {
    let uuid = securerandom::uuid(interp);
    Ok(interp.convert_mut(uuid))
}
