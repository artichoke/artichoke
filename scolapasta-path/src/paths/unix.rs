use std::ffi::{OsStr, OsString};
use std::os::unix::ffi::{OsStrExt, OsStringExt};
use std::path::PathBuf;

use super::is_explicit_relative_bytes;

pub fn is_explicit_relative(path: &OsStr) -> bool {
    let bytes = path.as_bytes();

    is_explicit_relative_bytes(bytes)
}

#[allow(clippy::unnecessary_wraps)]
pub fn normalize_slashes(path: PathBuf) -> Result<Vec<u8>, PathBuf> {
    Ok(OsString::from(path).into_vec())
}

#[cfg(test)]
mod tests {
    use std::ffi::{OsStr, OsString};
    use std::os::unix::ffi::{OsStrExt, OsStringExt};
    use std::path::PathBuf;

    use super::{is_explicit_relative, normalize_slashes};

    #[test]
    fn empty() {
        assert!(!is_explicit_relative(OsStr::new("")));
    }

    #[test]
    fn single_char() {
        assert!(!is_explicit_relative(OsStr::new("a")));
    }

    #[test]
    fn single_dot() {
        assert!(!is_explicit_relative(OsStr::new(".")));
    }

    #[test]
    fn double_dot() {
        assert!(!is_explicit_relative(OsStr::new("..")));
    }

    #[test]
    fn triple_dot() {
        assert!(!is_explicit_relative(OsStr::new("...")));
    }

    #[test]
    fn single_dot_slash() {
        assert!(is_explicit_relative(OsStr::new("./")));
    }

    #[test]
    fn double_dot_slash() {
        assert!(is_explicit_relative(OsStr::new("../")));
    }

    #[test]
    fn absolute() {
        let test_cases = [r"/bin", r"/home/artichoke"];
        for path in test_cases {
            assert!(
                !is_explicit_relative(OsStr::new(path)),
                "expected absolute path '{}' to NOT be explicit relative path",
                path
            );
        }
    }

    #[test]
    fn relative() {
        let test_cases = [r"temp", r"temp/../var"];
        for path in test_cases {
            assert!(
                !is_explicit_relative(OsStr::new(path)),
                "expected relative path '{}' to NOT be explicit relative path",
                path
            );
        }
    }

    #[test]
    fn explicit_relative() {
        let test_cases = [r"./cache", r"../cache", r"./.git", r"../.git"];
        for path in test_cases {
            assert!(
                is_explicit_relative(OsStr::new(path)),
                "expected relative path '{}' to be explicit relative path",
                path
            );
        }
    }

    #[test]
    fn not_explicit_relative() {
        let test_cases = [r"...\var", r".../var", r"\var", r"/var"];
        for path in test_cases {
            assert!(
                !is_explicit_relative(OsStr::new(path)),
                "expected path '{}' to NOT be explicit relative path",
                path
            );
        }
    }

    #[test]
    fn forward_slash() {
        let test_cases = [
            r".\cache",
            r"..\cache",
            r".\.git",
            r"..\.git",
            r"...\var",
            r".../var",
            r"\var",
            r"/var",
        ];
        for path in test_cases {
            assert!(
                !is_explicit_relative(OsStr::new(path)),
                "expected relative path '{}' to NOT be explicit relative path",
                path
            );
        }
    }

    #[test]
    fn invalid_utf8_dot_dot_slash() {
        let path = OsStr::from_bytes(b"../\xFF\xFE");
        assert!(is_explicit_relative(path));
    }

    #[test]
    fn invalid_utf8_dot_slash() {
        let path = OsStr::from_bytes(b"./\xFF\xFE");
        assert!(is_explicit_relative(path));
    }

    #[test]
    fn invalid_utf8_absolute() {
        let path = OsStr::from_bytes(b"/\xFF\xFE");
        assert!(!is_explicit_relative(path));
    }

    #[test]
    fn invalid_utf8_relative() {
        let path = OsStr::from_bytes(b"\xFF\xFE");
        assert!(!is_explicit_relative(path));
    }

    #[test]
    fn normalize_slashes_no_backslash() {
        let path = PathBuf::from(r"abcxyz".to_string());
        assert_eq!(normalize_slashes(path).unwrap(), b"abcxyz".to_vec());

        let path = PathBuf::from(r"abc/xyz".to_string());
        assert_eq!(normalize_slashes(path).unwrap(), b"abc/xyz".to_vec());
    }

    #[test]
    fn normalize_slashes_backslash_noop() {
        let path = PathBuf::from(r"abc\xyz".to_string());
        assert_eq!(normalize_slashes(path).unwrap(), br"abc\xyz".to_vec());

        let path = PathBuf::from(r"abc\xyz\123".to_string());
        assert_eq!(normalize_slashes(path).unwrap(), br"abc\xyz\123".to_vec());
    }

    #[test]
    fn normalize_slashes_invalid_utf8() {
        let path = OsString::from_vec(b"abcxyz\xFF".to_vec());
        assert_eq!(normalize_slashes(path.into()).unwrap(), b"abcxyz\xFF".to_vec());

        let path = OsString::from_vec(b"abc/xyz\xFF".to_vec());
        assert_eq!(normalize_slashes(path.into()).unwrap(), b"abc/xyz\xFF".to_vec());

        let path = OsString::from_vec(b"abc\\xyz\xFF".to_vec());
        assert_eq!(normalize_slashes(path.into()).unwrap(), b"abc\\xyz\xFF".to_vec());
    }
}
