use std::borrow::Cow;
use std::ffi::OsStr;
use std::str;

use crate::ffi::ConvertBytesError;

pub fn os_str_to_bytes(value: &OsStr) -> Result<Cow<'_, [u8]>, ConvertBytesError> {
    value
        .to_str()
        .map(str::as_bytes)
        .map(Into::into)
        .ok_or(ConvertBytesError)
}

pub fn bytes_to_os_str(value: &[u8]) -> Result<Cow<'_, OsStr>, ConvertBytesError> {
    str::from_utf8(value)
        .map(OsStr::new)
        .map(Into::into)
        .map_err(|_| ConvertBytesError)
}
