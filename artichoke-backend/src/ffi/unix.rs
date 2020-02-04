use std::borrow::Cow;
use std::ffi::OsStr;
use std::os::unix::ffi::OsStrExt;

use crate::ffi::ConvertBytesError;

pub fn os_str_to_bytes(value: &OsStr) -> Result<Cow<'_, [u8]>, ConvertBytesError> {
    Ok(value.as_bytes().into())
}

pub fn bytes_to_os_str(value: &[u8]) -> Result<Cow<'_, OsStr>, ConvertBytesError> {
    Ok(OsStr::from_bytes(value).into())
}
