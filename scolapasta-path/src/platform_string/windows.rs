use std::ffi::{OsStr, OsString};
use std::str;

use super::ConvertBytesError;

pub fn bytes_to_os_str(bytes: &[u8]) -> Result<&OsStr, ConvertBytesError> {
    let s = str::from_utf8(bytes).map_err(|_| ConvertBytesError::new())?;
    Ok(OsStr::new(s))
}

pub fn bytes_to_os_string(bytes: Vec<u8>) -> Result<OsString, ConvertBytesError> {
    let s = String::from_utf8(bytes).map_err(|_| ConvertBytesError::new())?;
    Ok(OsString::from(s))
}

pub fn os_str_to_bytes(os_str: &OsStr) -> Result<&[u8], ConvertBytesError> {
    let s = os_str.to_str().ok_or_else(ConvertBytesError::new)?;
    Ok(s.as_bytes())
}

pub fn os_string_to_bytes(os_string: OsString) -> Result<Vec<u8>, ConvertBytesError> {
    let s = os_string.into_string().map_err(|_| ConvertBytesError::new())?;
    Ok(s.into_bytes())
}

#[cfg(test)]
mod tests {
    use std::ffi::OsString;
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
    fn invalid_utf8_bytes_do_not_convert_to_os_str() {
        let test_cases: &[&[u8]] = &[b"\xFF", b"\xFE", b"abc/\xFF/\xFE", b"\0\xFF", b"\xFF\0"];
        for &bytes in test_cases {
            bytes_to_os_str(bytes).unwrap_err();
            bytes_to_os_string(bytes.to_vec()).unwrap_err();
        }
    }

    #[test]
    fn bytes_os_str_round_trip() {
        let test_cases: &[&[u8]] = &[b"", b"abc", b"abc\0", b"\0abc", b"abc/xyz"];
        for &bytes in test_cases {
            let os_str = bytes_to_os_str(bytes).unwrap();
            assert_eq!(os_str_to_bytes(os_str).unwrap(), bytes);
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
    fn bytes_os_string_round_trip() {
        let test_cases: &[&[u8]] = &[b"", b"abc", b"abc\0", b"\0abc", b"abc/xyz"];
        for &bytes in test_cases {
            let os_string = bytes_to_os_string(bytes.to_owned()).unwrap();
            assert_eq!(os_string_to_bytes(os_string).unwrap(), bytes);
        }
    }
}
