//! [`Kernel#require`](https://ruby-doc.org/core-2.6.3/Kernel.html#method-i-require)

use bstr::ByteSlice;
use std::path::{Path, PathBuf};

use crate::extn::prelude::*;
use crate::ffi;
use crate::fs::RUBY_LOAD_PATH;
use crate::state::parser::Context;

const RUBY_EXTENSION: &str = "rb";

pub fn load(interp: &mut Artichoke, mut filename: Value) -> Result<bool, Exception> {
    let filename = filename.implicitly_convert_to_string(interp)?;
    if filename.find_byte(b'\0').is_some() {
        return Err(ArgumentError::from("path name contains null byte").into());
    }
    let file = ffi::bytes_to_os_str(filename)?;
    let pathbuf;
    let mut path = Path::new(file);
    if path.is_relative() {
        pathbuf = Path::new(RUBY_LOAD_PATH).join(file);
        path = pathbuf.as_path();
    }
    if !interp.source_is_file(path)? {
        let mut message = b"cannot load such file -- ".to_vec();
        message.extend_from_slice(filename);
        return Err(LoadError::from(message).into());
    }
    let context = Context::new(ffi::os_str_to_bytes(path.as_os_str())?.to_vec())
        .ok_or_else(|| ArgumentError::from("path name contains null byte"))?;
    interp.push_context(context)?;
    let result = interp.load_source(path);
    let _ = interp.pop_context()?;
    result
}

pub fn require(
    interp: &mut Artichoke,
    mut filename: Value,
    base: Option<RelativePath>,
) -> Result<bool, Exception> {
    let filename = filename.implicitly_convert_to_string(interp)?;
    if filename.find_byte(b'\0').is_some() {
        return Err(ArgumentError::from("path name contains null byte").into());
    }
    let file = ffi::bytes_to_os_str(filename)?;
    let path = Path::new(file);

    let (path, alternate) = if path.is_relative() {
        let mut path = if let Some(ref base) = base {
            base.join(path)
        } else {
            Path::new(RUBY_LOAD_PATH).join(path)
        };
        let is_rb = path
            .extension()
            .filter(|ext| ext == &RUBY_EXTENSION)
            .is_some();
        if is_rb {
            (path, None)
        } else {
            let alternate = path.clone();
            path.set_extension(RUBY_EXTENSION);
            (path, Some(alternate))
        }
    } else {
        (path.to_owned(), None)
    };
    if interp.source_is_file(&path)? {
        let context = Context::new(ffi::os_str_to_bytes(path.as_os_str())?.to_vec())
            .ok_or_else(|| ArgumentError::from("path name contains null byte"))?;
        interp.push_context(context)?;
        let result = interp.require_source(&path);
        let _ = interp.pop_context()?;
        return result;
    }
    if let Some(path) = alternate {
        if interp.source_is_file(&path)? {
            let context = Context::new(ffi::os_str_to_bytes(path.as_os_str())?.to_vec())
                .ok_or_else(|| ArgumentError::from("path name contains null byte"))?;
            interp.push_context(context)?;
            let result = interp.require_source(&path);
            let _ = interp.pop_context()?;
            return result;
        }
    }
    let mut message = b"cannot load such file -- ".to_vec();
    message.extend_from_slice(filename);
    Err(LoadError::from(message).into())
}

#[derive(Default, Debug, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct RelativePath(PathBuf);

impl From<PathBuf> for RelativePath {
    fn from(path: PathBuf) -> Self {
        Self(path)
    }
}

impl From<&Path> for RelativePath {
    fn from(path: &Path) -> Self {
        Self(path.into())
    }
}

impl From<String> for RelativePath {
    fn from(path: String) -> Self {
        Self(path.into())
    }
}

impl From<&str> for RelativePath {
    fn from(path: &str) -> Self {
        Self(path.into())
    }
}

impl RelativePath {
    #[must_use]
    pub fn new() -> Self {
        Self::from(RUBY_LOAD_PATH)
    }

    pub fn join<P: AsRef<Path>>(&self, path: P) -> PathBuf {
        self.0.join(path.as_ref())
    }

    pub fn try_from_interp(interp: &mut Artichoke) -> Result<Self, Exception> {
        let context = interp
            .peek_context()?
            .ok_or_else(|| Fatal::from("relative require with no context stack"))?;
        let path = ffi::bytes_to_os_str(context.filename())?;
        let path = Path::new(path);
        if let Some(base) = path.parent() {
            Ok(Self::from(base))
        } else {
            Ok(Self::from("/"))
        }
    }
}

#[cfg(test)]
mod test {
    use crate::test::prelude::*;

    #[derive(Debug)]
    struct MockSourceFile;

    impl File for MockSourceFile {
        type Artichoke = Artichoke;

        type Error = Exception;

        fn require(interp: &mut Artichoke) -> Result<(), Self::Error> {
            let _ = interp.eval(b"@i = 255").unwrap();
            Ok(())
        }
    }

    #[derive(Debug)]
    struct MockExtensionAndSourceFile;

    impl File for MockExtensionAndSourceFile {
        type Artichoke = Artichoke;

        type Error = Exception;

        fn require(interp: &mut Artichoke) -> Result<(), Self::Error> {
            let _ = interp.eval(b"module Foo; RUST = 7; end").unwrap();
            Ok(())
        }
    }

    // Functional test for `Kernel::require`:
    //
    // - require side effects (e.g. ivar set or class def) effect the interpreter
    // - Successful first require returns `true`.
    // - Second require returns `false`.
    // - Second require does not cause require side effects.
    // - Require non-existing file raises and returns `nil`.
    #[test]
    fn functional() {
        let mut interp = crate::interpreter().unwrap();
        interp
            .def_file_for_type::<_, MockSourceFile>("file.rb")
            .unwrap();
        let result = interp.eval(b"require 'file'").unwrap();
        let require_result = result.try_into::<bool>(&interp).unwrap();
        assert!(require_result);
        let result = interp.eval(b"@i").unwrap();
        let i_result = result.try_into::<i64>(&interp).unwrap();
        assert_eq!(i_result, 255);
        let result = interp.eval(b"@i = 1000; require 'file'").unwrap();
        let second_require_result = result.try_into::<bool>(&interp).unwrap();
        assert!(!second_require_result);
        let result = interp.eval(b"@i").unwrap();
        let second_i_result = result.try_into::<i64>(&interp).unwrap();
        assert_eq!(second_i_result, 1000);
        let err = interp.eval(b"require 'non-existent-source'").unwrap_err();
        assert_eq!(
            &b"cannot load such file -- non-existent-source"[..],
            err.message().as_ref()
        );
        let expected = vec![Vec::from(&b"(eval):1"[..])];
        assert_eq!(Some(expected), err.vm_backtrace(&mut interp),);
    }

    #[test]
    fn absolute_path() {
        let mut interp = crate::interpreter().unwrap();
        interp
            .def_rb_source_file("/foo/bar/source.rb", &b"# a source file"[..])
            .unwrap();
        let result = interp.eval(b"require '/foo/bar/source.rb'").unwrap();
        assert!(result.try_into::<bool>(&interp).unwrap());
        let result = interp.eval(b"require '/foo/bar/source.rb'").unwrap();
        assert!(!result.try_into::<bool>(&interp).unwrap());
    }

    #[test]
    fn relative_with_dotted_path() {
        let mut interp = crate::interpreter().unwrap();
        interp
            .def_rb_source_file("/foo/bar/source.rb", &b"require_relative '../bar.rb'"[..])
            .unwrap();
        interp
            .def_rb_source_file("/foo/bar.rb", &b"# a source file"[..])
            .unwrap();
        let result = interp.eval(b"require '/foo/bar/source.rb'").unwrap();
        assert!(result.try_into::<bool>(&interp).unwrap());
        let result = interp.eval(b"require '/foo/bar.rb'").unwrap();
        assert!(!result.try_into::<bool>(&interp).unwrap());
    }

    #[test]
    fn directory_err() {
        let mut interp = crate::interpreter().unwrap();
        let err = interp.eval(b"require '/src'").unwrap_err();
        assert_eq!(
            &b"cannot load such file -- /src"[..],
            err.message().as_ref()
        );
        let expected = vec![Vec::from(&b"(eval):1"[..])];
        assert_eq!(Some(expected), err.vm_backtrace(&mut interp));
    }

    #[test]
    fn path_defined_as_source_then_extension_file() {
        let mut interp = crate::interpreter().unwrap();
        interp
            .def_rb_source_file("foo.rb", &b"module Foo; RUBY = 3; end"[..])
            .unwrap();
        interp
            .def_file_for_type::<_, MockExtensionAndSourceFile>("foo.rb")
            .unwrap();
        let result = interp.eval(b"require 'foo'").unwrap();
        let result = result.try_into::<bool>(&interp).unwrap();
        assert!(result, "successfully required foo.rb");
        let result = interp.eval(b"Foo::RUBY + Foo::RUST").unwrap();
        let result = result.try_into::<i64>(&interp).unwrap();
        assert_eq!(
            result, 10,
            "defined Ruby and Rust sources from single require"
        );
    }

    #[test]
    fn path_defined_as_extension_file_then_source() {
        let mut interp = crate::interpreter().unwrap();
        interp
            .def_file_for_type::<_, MockExtensionAndSourceFile>("foo.rb")
            .unwrap();
        interp
            .def_rb_source_file("foo.rb", &b"module Foo; RUBY = 3; end"[..])
            .unwrap();
        let result = interp.eval(b"require 'foo'").unwrap();
        let result = result.try_into::<bool>(&interp).unwrap();
        assert!(result, "successfully required foo.rb");
        let result = interp.eval(b"Foo::RUBY + Foo::RUST").unwrap();
        let result = result.try_into::<i64>(&interp).unwrap();
        assert_eq!(
            result, 10,
            "defined Ruby and Rust sources from single require"
        );
    }
}
