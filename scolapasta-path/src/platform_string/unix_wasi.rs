#![allow(clippy::unnecessary_wraps)]

use std::ffi::{OsStr, OsString};
#[cfg(unix)]
use std::os::unix::ffi::{OsStrExt, OsStringExt};
#[cfg(target_os = "wasi")]
use std::os::wasi::ffi::{OsStrExt, OsStringExt};
#[cfg(not(any(unix, target_os = "wasi")))]
error!("Internal: This module can only be compiled on unix and wasi");

use super::ConvertBytesError;

pub fn bytes_to_os_str(bytes: &[u8]) -> Result<&OsStr, ConvertBytesError> {
    Ok(OsStr::from_bytes(bytes))
}

pub fn bytes_to_os_string(bytes: Vec<u8>) -> Result<OsString, ConvertBytesError> {
    Ok(OsString::from_vec(bytes))
}

pub fn os_str_to_bytes(os_str: &OsStr) -> Result<&[u8], ConvertBytesError> {
    Ok(os_str.as_bytes())
}

pub fn os_string_to_bytes(os_string: OsString) -> Result<Vec<u8>, ConvertBytesError> {
    Ok(os_string.into_vec())
}

#[cfg(test)]
mod tests {
    use std::ffi::OsString;
    #[cfg(unix)]
    use std::os::unix::ffi::OsStringExt;
    #[cfg(target_os = "wasi")]
    use std::os::wasi::ffi::OsStringExt;
    use std::str;

    use super::{bytes_to_os_str, bytes_to_os_string, os_str_to_bytes, os_string_to_bytes};

    #[test]
    fn utf8_bytes_convert_to_os_str() {
        let test_cases: &[&[u8]] = &[b"", b"abc", b"abc\0", b"\0abc", b"abc/xyz"];
        for &bytes in test_cases {
            bytes_to_os_str(bytes).unwrap();
            bytes_to_os_string(bytes.to_vec()).unwrap();
        }
    }

    #[test]
    fn invalid_utf8_bytes_convert_to_os_str() {
        let test_cases: &[&[u8]] = &[b"\xFF", b"\xFE", b"abc/\xFF/\xFE", b"\0\xFF", b"\xFF\0"];
        for &bytes in test_cases {
            bytes_to_os_str(bytes).unwrap();
            bytes_to_os_string(bytes.to_vec()).unwrap();
        }
    }

    #[test]
    fn bytes_os_str_round_trip() {
        let test_cases: &[&[u8]] = &[b"", b"abc", b"abc\0", b"\0abc", b"abc/xyz"];
        for &bytes in test_cases {
            let os_str = bytes_to_os_str(bytes).unwrap();
            assert_eq!(os_str_to_bytes(os_str).unwrap(), bytes);

            let os_string = bytes_to_os_string(bytes.to_vec()).unwrap();
            assert_eq!(os_string_to_bytes(os_string).unwrap(), bytes);
        }
    }

    #[test]
    fn invalid_utf8_bytes_os_str_round_trip() {
        let test_cases: &[&[u8]] = &[b"\xFF", b"\xFE", b"abc/\xFF/\xFE", b"\0\xFF", b"\xFF\0"];
        for &bytes in test_cases {
            let os_str = bytes_to_os_str(bytes).unwrap();
            assert_eq!(os_str_to_bytes(os_str).unwrap(), bytes);

            let os_string = bytes_to_os_string(bytes.to_vec()).unwrap();
            assert_eq!(os_string_to_bytes(os_string).unwrap(), bytes);
        }
    }

    #[test]
    fn owned_utf8_bytes_convert_to_os_string() {
        let test_cases: &[&[u8]] = &[b"", b"abc", b"abc\0", b"\0abc", b"abc/xyz"];
        for &bytes in test_cases {
            let s = str::from_utf8(bytes).unwrap();
            let os_string = OsString::from(s.to_owned());
            os_string_to_bytes(os_string).unwrap();
        }
    }

    #[test]
    fn owned_invalid_utf8_bytes_convert_to_os_string() {
        let test_cases: &[&[u8]] = &[b"\xFF", b"\xFE", b"abc/\xFF/\xFE", b"\0\xFF", b"\xFF\0"];
        for &bytes in test_cases {
            let os_string = OsString::from_vec(bytes.to_owned());
            os_string_to_bytes(os_string).unwrap();
        }
    }

    #[test]
    fn bytes_os_string_round_trip() {
        let test_cases: &[&[u8]] = &[b"", b"abc", b"abc\0", b"\0abc", b"abc/xyz"];
        for &bytes in test_cases {
            let os_string = bytes_to_os_string(bytes.to_owned()).unwrap();
            assert_eq!(os_string_to_bytes(os_string).unwrap(), bytes);
        }
    }

    #[test]
    fn invalid_utf8_bytes_os_string_round_trip() {
        let test_cases: &[&[u8]] = &[b"\xFF", b"\xFE", b"abc/\xFF/\xFE", b"\0\xFF", b"\xFF\0"];
        for &bytes in test_cases {
            let os_string = bytes_to_os_string(bytes.to_owned()).unwrap();
            assert_eq!(os_string_to_bytes(os_string).unwrap(), bytes);
        }
    }
}
