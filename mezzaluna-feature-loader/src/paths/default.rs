use std::ffi::OsStr;
use std::path;

#[allow(dead_code)]
pub fn is_explicit_relative<P: AsRef<OsStr>>(path: P) -> bool {
    let path = path.as_ref();
    let bytes = if let Some(path) = path.to_str() {
        path.as_bytes()
    } else {
        return false;
    };
    is_explicit_relative_bytes(bytes)
}

#[allow(dead_code)]
pub fn is_explicit_relative_bytes<P: AsRef<[u8]>>(path: P) -> bool {
    let bytes = path.as_ref();
    // See the reference implementation based on MRI:
    //
    // https://github.com/artichoke/ruby/blob/v3_0_2/file.c#L6287-L6293
    match bytes {
        [b'.', b'.', x, ..] if path::is_separator((*x).into()) => true,
        [b'.', x, ..] if path::is_separator((*x).into()) => true,
        _ => false,
    }
}

#[cfg(test)]
mod tests {
    use super::{is_explicit_relative, is_explicit_relative_bytes};

    #[test]
    fn empty() {
        assert!(!is_explicit_relative(""));
        assert!(!is_explicit_relative_bytes(""));
    }

    #[test]
    fn single_char() {
        assert!(!is_explicit_relative("a"));
        assert!(!is_explicit_relative_bytes("a"));
    }

    #[test]
    fn single_dot() {
        assert!(!is_explicit_relative("."));
        assert!(!is_explicit_relative_bytes("."));
    }

    #[test]
    fn double_dot() {
        assert!(!is_explicit_relative(".."));
        assert!(!is_explicit_relative_bytes(".."));
    }

    #[test]
    fn triple_dot() {
        assert!(!is_explicit_relative("..."));
        assert!(!is_explicit_relative_bytes("..."));
    }

    #[test]
    fn single_dot_slash() {
        assert!(is_explicit_relative("./"));
        assert!(is_explicit_relative_bytes("./"));
    }

    #[test]
    fn double_dot_slash() {
        assert!(is_explicit_relative("../"));
        assert!(is_explicit_relative_bytes("../"));
    }

    #[test]
    fn absolute() {
        let test_cases = [r"/bin", r"/home/artichoke"];
        for path in test_cases {
            assert!(
                !is_explicit_relative(path),
                "expected absolute path '{}' to NOT be explicit relative path",
                path
            );
            assert!(
                !is_explicit_relative_bytes(path),
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
                !is_explicit_relative(path),
                "expected relative path '{}' to NOT be explicit relative path",
                path
            );
            assert!(
                !is_explicit_relative_bytes(path),
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
                is_explicit_relative(path),
                "expected relative path '{}' to be explicit relative path",
                path
            );
            assert!(
                is_explicit_relative_bytes(path),
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
                !is_explicit_relative(path),
                "expected path '{}' to NOT be explicit relative path",
                path
            );
            assert!(
                !is_explicit_relative_bytes(path),
                "expected path '{}' to NOT be explicit relative path",
                path
            );
        }
    }

    #[test]
    fn invalid_utf8_explicit_relative_bytes() {
        let test_cases: [&[u8]; 4] = [b"./\xFF", b"../\xFF", b"./\xFF\xFE", b"../\xFF\xFE"];
        for path in test_cases {
            assert!(
                is_explicit_relative_bytes(path),
                "expected invalid UTF-8 relative path '{:?}' to be explicit relative path",
                path
            );
        }
    }

    #[test]
    fn invalid_utf8_not_explicit_relative_bytes() {
        let test_cases: [&[u8]; 4] = [b"/\xFF", b"\xFF", b"/\xFF\xFE", b"\xFF\xFE"];
        for path in test_cases {
            assert!(
                !is_explicit_relative_bytes(path),
                "expected invalid UTF-8 path '{:?}' to NOT be explicit relative path",
                path
            );
        }
    }
}
