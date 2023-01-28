use std::ffi::{OsStr, OsString};
use std::path::{self, PathBuf};

#[allow(dead_code)]
pub fn is_explicit_relative(path: &OsStr) -> bool {
    let bytes = if let Some(path) = path.to_str() {
        path.as_bytes()
    } else {
        return false;
    };
    is_explicit_relative_bytes(bytes)
}

#[allow(dead_code)]
pub fn is_explicit_relative_bytes(path: &[u8]) -> bool {
    // See the reference implementation based on MRI:
    //
    // https://github.com/artichoke/ruby/blob/v3_0_2/file.c#L6287-L6293
    match path {
        [b'.', b'.', x, ..] if path::is_separator((*x).into()) => true,
        [b'.', x, ..] if path::is_separator((*x).into()) => true,
        _ => false,
    }
}

#[allow(dead_code)]
pub fn normalize_slashes(path: PathBuf) -> Result<Vec<u8>, PathBuf> {
    match OsString::from(path).into_string() {
        Ok(s) => Ok(s.into()),
        Err(buf) => Err(buf.into()),
    }
}

#[cfg(test)]
mod tests {
    use std::ffi::OsStr;
    use std::path::PathBuf;

    use super::{is_explicit_relative, is_explicit_relative_bytes, normalize_slashes};

    #[test]
    fn empty() {
        assert!(!is_explicit_relative(OsStr::new("")));
        assert!(!is_explicit_relative_bytes("".as_bytes()));
    }

    #[test]
    fn single_char() {
        assert!(!is_explicit_relative(OsStr::new("a")));
        assert!(!is_explicit_relative_bytes("a".as_bytes()));
    }

    #[test]
    fn single_dot() {
        assert!(!is_explicit_relative(OsStr::new(".")));
        assert!(!is_explicit_relative_bytes(".".as_bytes()));
    }

    #[test]
    fn double_dot() {
        assert!(!is_explicit_relative(OsStr::new("..")));
        assert!(!is_explicit_relative_bytes("..".as_bytes()));
    }

    #[test]
    fn triple_dot() {
        assert!(!is_explicit_relative(OsStr::new("...")));
        assert!(!is_explicit_relative_bytes("...".as_bytes()));
    }

    #[test]
    fn single_dot_slash() {
        assert!(is_explicit_relative(OsStr::new("./")));
        assert!(is_explicit_relative_bytes("./".as_bytes()));
    }

    #[test]
    fn double_dot_slash() {
        assert!(is_explicit_relative(OsStr::new("../")));
        assert!(is_explicit_relative_bytes("../".as_bytes()));
    }

    #[test]
    fn absolute() {
        let test_cases = [r"/bin", r"/home/artichoke"];
        for path in test_cases {
            assert!(
                !is_explicit_relative(OsStr::new(path)),
                "expected absolute path '{path}' to NOT be explicit relative path"
            );
            assert!(
                !is_explicit_relative_bytes(path.as_bytes()),
                "expected absolute path '{path}' to NOT be explicit relative path"
            );
        }
    }

    #[test]
    fn relative() {
        let test_cases = [r"temp", r"temp/../var"];
        for path in test_cases {
            assert!(
                !is_explicit_relative(OsStr::new(path)),
                "expected relative path '{path}' to NOT be explicit relative path"
            );
            assert!(
                !is_explicit_relative_bytes(path.as_bytes()),
                "expected relative path '{path}' to NOT be explicit relative path"
            );
        }
    }

    #[test]
    fn explicit_relative() {
        let test_cases = [r"./cache", r"../cache", r"./.git", r"../.git"];
        for path in test_cases {
            assert!(
                is_explicit_relative(OsStr::new(path)),
                "expected relative path '{path}' to be explicit relative path"
            );
            assert!(
                is_explicit_relative_bytes(path.as_bytes()),
                "expected relative path '{path}' to be explicit relative path"
            );
        }
    }

    #[test]
    fn not_explicit_relative() {
        let test_cases = [r"...\var", r".../var", r"\var", r"/var"];
        for path in test_cases {
            assert!(
                !is_explicit_relative(OsStr::new(path)),
                "expected path '{path}' to NOT be explicit relative path"
            );
            assert!(
                !is_explicit_relative_bytes(path.as_bytes()),
                "expected path '{path}' to NOT be explicit relative path"
            );
        }
    }

    #[test]
    fn invalid_utf8_explicit_relative_bytes() {
        let test_cases: [&[u8]; 4] = [b"./\xFF", b"../\xFF", b"./\xFF\xFE", b"../\xFF\xFE"];
        for path in test_cases {
            assert!(
                is_explicit_relative_bytes(path),
                "expected invalid UTF-8 relative path '{path:?}' to be explicit relative path"
            );
        }
    }

    #[test]
    fn invalid_utf8_not_explicit_relative_bytes() {
        let test_cases: [&[u8]; 4] = [b"/\xFF", b"\xFF", b"/\xFF\xFE", b"\xFF\xFE"];
        for path in test_cases {
            assert!(
                !is_explicit_relative_bytes(path),
                "expected invalid UTF-8 path '{path:?}' to NOT be explicit relative path"
            );
        }
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
}
