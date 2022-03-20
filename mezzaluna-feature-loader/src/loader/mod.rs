use std::ffi::OsStr;
use std::io;
use std::path::{self, Path};

use bstr::ByteSlice;

#[cfg(feature = "disk")]
mod disk;

#[cfg(feature = "disk")]
pub use disk::Disk;

#[cfg(feature = "rubylib")]
mod rubylib;

#[cfg(feature = "rubylib")]
pub use rubylib::Rubylib;

use crate::loaded_features::LoadedFeatures;

/// Directory at which Ruby sources and extensions are stored in the virtual
/// file system.
#[must_use]
pub fn memory_loader_ruby_load_path() -> &'static OsStr {
    if cfg!(windows) {
        OsStr::new("c:/artichoke/virtual_root/src/lib")
    } else {
        OsStr::new("/artichoke/virtual_root/src/lib")
    }
}

/// Return whether the given path starts with an explicit relative path.
///
/// Explicit relative paths start with `.` or `..` followed immediately by a
/// directory separator.
///
/// [`Disk`] loaders have special handling for explicit relative paths: they are
/// resolved relative to the process's [current working directory] rather than
/// the load path.
///
/// # Examples
///
/// ```
/// # use std::path::Path;
/// # use mezzaluna_feature_loader::is_explicit_relative;
/// assert!(is_explicit_relative(Path::new("./test/loader")));
/// assert!(is_explicit_relative(Path::new("../rake/test_task")));
///
/// assert!(!is_explicit_relative(Path::new("json/pure")));
/// assert!(!is_explicit_relative(Path::new("/artichoke/src/json/pure")));
/// ```
///
/// # MRI C Declaration
///
/// ```c
/// static int
/// is_explicit_relative(const char *path)
/// {
///     if (*path++ != '.') return 0;
///     if (*path == '.') path++;
///     return isdirsep(*path);
/// }
/// ```
///
/// [current working directory]: std::env::current_dir
#[must_use]
pub fn is_explicit_relative(path: &Path) -> bool {
    let bytes = if let Some(bytes) = <[u8]>::from_path(path) {
        bytes
    } else {
        return false;
    };
    // Implementation based on MRI:
    //
    // https://github.com/artichoke/artichoke/blob/7c845ddfe709658ad6f66be00b2514af05b2619a/artichoke-backend/vendor/ruby/file.c#L6005-L6011
    match bytes {
        [b'.', b'.', x, ..] if path::is_separator((*x).into()) => true,
        [b'.', x, ..] if path::is_separator((*x).into()) => true,
        _ => false,
    }
}

#[derive(Debug)]
pub struct Loader {
    #[cfg(feature = "rubylib")]
    rubylib: Rubylib,
    #[allow(dead_code)]
    loaded_features: LoadedFeatures,
}

impl Loader {
    #[must_use]
    pub fn new() -> Option<Self> {
        #[cfg(feature = "rubylib")]
        let rubylib = Rubylib::new()?;
        let loaded_features = LoadedFeatures::new();
        Some(Self {
            #[cfg(feature = "rubylib")]
            rubylib,
            loaded_features,
        })
    }

    #[must_use]
    #[cfg(feature = "rubylib")]
    pub fn with_rubylib(rubylib: &OsStr) -> Option<Self> {
        let rubylib = Rubylib::with_rubylib(rubylib)?;
        let loaded_features = LoadedFeatures::new();
        Some(Self {
            rubylib,
            loaded_features,
        })
    }

    #[must_use]
    #[cfg(feature = "rubylib")]
    pub fn with_rubylib_and_cwd(rubylib: &OsStr, cwd: &Path) -> Option<Self> {
        #[cfg(feature = "rubylib")]
        let rubylib = Rubylib::with_rubylib_and_cwd(rubylib, cwd)?;
        let loaded_features = LoadedFeatures::new();
        Some(Self {
            rubylib,
            loaded_features,
        })
    }

    #[allow(clippy::missing_errors_doc)]
    #[allow(clippy::missing_panics_doc)]
    pub fn read<T>(&self, path: &Path) -> io::Result<Vec<u8>> {
        #[cfg(feature = "rubylib")]
        {
            use std::io::Read;

            if let Some(handle) = self.rubylib.resolve_file(path) {
                let file = handle.as_file();
                // Allocate one extra byte so the buffer doesn't need to grow before the
                // final `read` call at the end of the file.  Don't worry about `usize`
                // overflow because reading will fail regardless in that case.
                #[allow(clippy::cast_possible_truncation)]
                let initial_buffer_size = file.metadata().map(|m| m.len() as usize + 1).unwrap_or(0);
                let mut buf = Vec::with_capacity(initial_buffer_size);
                handle.as_file().read_to_end(&mut buf)?;
                return Ok(buf);
            }
        }
        let _ = path;
        unimplemented!("implement Loader::read");
    }
}

#[cfg(test)]
mod tests {
    use std::path::Path;

    use super::is_explicit_relative;

    #[test]
    #[cfg(windows)]
    fn windows_explicit_relative() {
        let absolute: &[&Path] = &[
            Path::new(r"c:\windows"),
            Path::new(r"c:/windows"),
            Path::new(r"\\.\COM1"),
            Path::new(r"\\?\C:\windows"),
        ];
        for &path in absolute {
            assert!(
                !is_explicit_relative(path),
                "expected absolute path '{}' to NOT be explicit relative path",
                path.display()
            );
        }

        let relative: &[&Path] = &[
            Path::new(r"c:temp"),
            Path::new(r"temp"),
            Path::new(r"\temp"),
            Path::new(r"/temp"),
        ];
        for &path in relative {
            assert!(
                !is_explicit_relative(path),
                "expected relative path '{}' to NOT be explicit relative path",
                path.display()
            );
        }

        let explicit_relative: &[&Path] = &[
            Path::new(r".\windows"),
            Path::new(r"./windows"),
            Path::new(r"..\windows"),
            Path::new(r"../windows"),
            Path::new(r".\.git"),
            Path::new(r"./.git"),
            Path::new(r"..\.git"),
            Path::new(r"../.git"),
        ];
        for &path in explicit_relative {
            assert!(is_explicit_relative(path));
            assert!(
                is_explicit_relative(path),
                "expected relative path '{}' to be explicit relative path",
                path.display()
            );
        }

        let not_explicit_relative: &[&Path] = &[
            Path::new(r"...\windows"),
            Path::new(r".../windows"),
            Path::new(r"\windows"),
            Path::new(r"/windows"),
        ];
        for &path in not_explicit_relative {
            assert!(
                !is_explicit_relative(path),
                "expected relative path '{}' to NOT be explicit relative path",
                path.display()
            );
        }
    }

    #[test]
    #[cfg(not(windows))]
    fn explicit_relative() {
        let absolute: &[&Path] = &[Path::new(r"/bin"), Path::new(r"/home/artichoke")];
        for &path in absolute {
            assert!(
                !is_explicit_relative(path),
                "expected absolute path '{}' to NOT be explicit relative path",
                path.display()
            );
        }

        let relative: &[&Path] = &[Path::new(r"temp"), Path::new(r"temp/../var")];
        for &path in relative {
            assert!(
                !is_explicit_relative(path),
                "expected relative path '{}' to NOT be explicit relative path",
                path.display()
            );
        }

        let explicit_relative: &[&Path] = &[
            Path::new(r"./cache"),
            Path::new(r"../cache"),
            Path::new(r"./.git"),
            Path::new(r"../.git"),
        ];
        for &path in explicit_relative {
            assert!(
                is_explicit_relative(path),
                "expected relative path '{}' to be explicit relative path",
                path.display()
            );
        }

        let not_explicit_relative: &[&Path] = &[
            Path::new(r".\cache"),
            Path::new(r"..\cache"),
            Path::new(r".\.git"),
            Path::new(r"..\.git"),
            Path::new(r"...\var"),
            Path::new(r".../var"),
            Path::new(r"\var"),
            Path::new(r"/var"),
        ];
        for &path in not_explicit_relative {
            assert!(
                !is_explicit_relative(path),
                "expected relative path '{}' to NOT be explicit relative path",
                path.display()
            );
        }
    }
}
