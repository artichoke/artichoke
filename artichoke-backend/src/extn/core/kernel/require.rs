//! [`Kernel#require`](https://ruby-doc.org/core-3.1.2/Kernel.html#method-i-require)

use std::path::{Path, PathBuf};

use artichoke_core::load::{Loaded, Required};
use bstr::ByteSlice;

use crate::convert::implicitly_convert_to_string;
use crate::extn::prelude::*;
use crate::platform_string::bytes_to_os_str;
use crate::state::parser::Context;

pub fn load(interp: &mut Artichoke, mut filename: Value) -> Result<Loaded, Error> {
    // SAFETY: The extracted byte slice is converted to an owned `Vec<u8>`
    // before the interp is used again which protects against a garbage
    // collection invalidating the pointer.
    let filename = unsafe { implicitly_convert_to_string(interp, &mut filename)? };
    if filename.find_byte(b'\0').is_some() {
        return Err(ArgumentError::with_message("path name contains null byte").into());
    }
    let filename = filename.to_vec();
    let file = bytes_to_os_str(&filename)?;
    let path = Path::new(file);

    if let Some(mut context) = interp.resolve_source_path(&path)? {
        for byte in &mut context {
            if *byte == b'\\' {
                *byte = b'/';
            }
        }
        let context =
            Context::new(context).ok_or_else(|| ArgumentError::with_message("path name contains null byte"))?;
        interp.push_context(context)?;
        let result = interp.load_source(&path);
        interp.pop_context()?;
        return result;
    }
    let mut message = b"cannot load such file -- ".to_vec();
    message.extend(filename);
    Err(LoadError::from(message).into())
}

pub fn require(interp: &mut Artichoke, mut filename: Value) -> Result<Required, Error> {
    // SAFETY: The extracted byte slice is converted to an owned `Vec<u8>`
    // before the interp is used again which protects against a garbage
    // collection invalidating the pointer.
    let filename = unsafe { implicitly_convert_to_string(interp, &mut filename)? };
    if filename.find_byte(b'\0').is_some() {
        return Err(ArgumentError::with_message("path name contains null byte").into());
    }
    let filename = filename.to_vec();
    let file = bytes_to_os_str(&filename)?;
    let path = Path::new(file);

    if let Some(mut context) = interp.resolve_source_path(&path)? {
        for byte in &mut context {
            if *byte == b'\\' {
                *byte = b'/';
            }
        }
        let context =
            Context::new(context).ok_or_else(|| ArgumentError::with_message("path name contains null byte"))?;
        interp.push_context(context)?;
        let result = interp.require_source(&path);
        interp.pop_context()?;
        return result;
    }
    let mut message = b"cannot load such file -- ".to_vec();
    message.extend(filename);
    Err(LoadError::from(message).into())
}

#[allow(clippy::module_name_repetitions)]
pub fn require_relative(interp: &mut Artichoke, mut filename: Value, base: RelativePath) -> Result<Required, Error> {
    // SAFETY: The extracted byte slice is converted to an owned `Vec<u8>`
    // before the interp is used again which protects against a garbage
    // collection invalidating the pointer.
    let filename = unsafe { implicitly_convert_to_string(interp, &mut filename)? };
    if filename.find_byte(b'\0').is_some() {
        return Err(ArgumentError::with_message("path name contains null byte").into());
    }
    let filename = filename.to_vec();
    let file = bytes_to_os_str(&filename)?;
    let path = base.join(Path::new(file));

    if let Some(mut context) = interp.resolve_source_path(&path)? {
        for byte in &mut context {
            if *byte == b'\\' {
                *byte = b'/';
            }
        }
        let context =
            Context::new(context).ok_or_else(|| ArgumentError::with_message("path name contains null byte"))?;
        interp.push_context(context)?;
        let result = interp.require_source(&path);
        interp.pop_context()?;
        return result;
    }
    let mut message = b"cannot load such file -- ".to_vec();
    message.extend(filename);
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
    pub fn join<P: AsRef<Path>>(&self, path: P) -> PathBuf {
        self.0.join(path.as_ref())
    }

    pub fn try_from_interp(interp: &mut Artichoke) -> Result<Self, Error> {
        let context = interp
            .peek_context()?
            .ok_or_else(|| Fatal::from("relative require with no context stack"))?;
        let path = bytes_to_os_str(context.filename())?;
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
    use bstr::ByteSlice;

    use crate::test::prelude::*;

    #[derive(Debug)]
    struct MockSourceFile;

    impl File for MockSourceFile {
        type Artichoke = Artichoke;

        type Error = Error;

        fn require(interp: &mut Artichoke) -> Result<(), Self::Error> {
            interp.eval(b"@i = 255").unwrap();
            Ok(())
        }
    }

    #[derive(Debug)]
    struct MockExtensionAndSourceFile;

    impl File for MockExtensionAndSourceFile {
        type Artichoke = Artichoke;

        type Error = Error;

        fn require(interp: &mut Artichoke) -> Result<(), Self::Error> {
            interp.eval(b"module Foo; RUST = 7; end").unwrap();
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
        let mut interp = interpreter();
        interp.def_file_for_type::<_, MockSourceFile>("file.rb").unwrap();
        let result = interp.eval(b"require 'file'").unwrap();
        let require_result = result.try_convert_into::<bool>(&interp).unwrap();
        assert!(require_result);
        let result = interp.eval(b"@i").unwrap();
        let i_result = result.try_convert_into::<i64>(&interp).unwrap();
        assert_eq!(i_result, 255);
        let result = interp.eval(b"@i = 1000; require 'file'").unwrap();
        let second_require_result = result.try_convert_into::<bool>(&interp).unwrap();
        assert!(!second_require_result);
        let result = interp.eval(b"@i").unwrap();
        let second_i_result = result.try_convert_into::<i64>(&interp).unwrap();
        assert_eq!(second_i_result, 1000);
        let err = interp.eval(b"require 'non-existent-source'").unwrap_err();
        assert_eq!(
            b"cannot load such file -- non-existent-source".as_bstr(),
            err.message().as_ref().as_bstr()
        );
        let expected_backtrace = b"(eval):1:in require\n(eval):1".to_vec();
        let actual_backtrace = bstr::join("\n", err.vm_backtrace(&mut interp).unwrap());
        assert_eq!(expected_backtrace.as_bstr(), actual_backtrace.as_bstr());
    }

    #[test]
    fn absolute_path() {
        let mut interp = interpreter();
        let (path, require_code) = if cfg!(windows) {
            (
                "c:/artichoke/virtual_root/src/lib/foo/bar/source.rb",
                &b"require 'c:/artichoke/virtual_root/src/lib/foo/bar/source.rb'"[..],
            )
        } else {
            (
                "/artichoke/virtual_root/src/lib/foo/bar/source.rb",
                &b"require '/artichoke/virtual_root/src/lib/foo/bar/source.rb'"[..],
            )
        };

        interp.def_rb_source_file(path, &b"# a source file"[..]).unwrap();
        let result = interp.eval(require_code).unwrap();
        assert!(result.try_convert_into::<bool>(&interp).unwrap());
        let result = interp.eval(require_code).unwrap();
        assert!(!result.try_convert_into::<bool>(&interp).unwrap());
    }

    #[test]
    fn relative_with_dotted_path() {
        let mut interp = interpreter();
        if cfg!(windows) {
            interp
                .def_rb_source_file(
                    "c:/artichoke/virtual_root/src/lib/foo/bar/source.rb",
                    &b"require_relative '../bar.rb'"[..],
                )
                .unwrap();
            interp
                .def_rb_source_file("c:/artichoke/virtual_root/src/lib/foo/bar.rb", &b"# a source file"[..])
                .unwrap();
            let result = interp
                .eval(b"require 'c:/artichoke/virtual_root/src/lib/foo/bar/source.rb'")
                .unwrap();
            assert!(result.try_convert_into::<bool>(&interp).unwrap());
            let result = interp
                .eval(b"require 'c:/artichoke/virtual_root/src/lib/foo/bar.rb'")
                .unwrap();
            assert!(!result.try_convert_into::<bool>(&interp).unwrap());
        } else {
            interp
                .def_rb_source_file(
                    "/artichoke/virtual_root/src/lib/foo/bar/source.rb",
                    &b"require_relative '../bar.rb'"[..],
                )
                .unwrap();
            interp
                .def_rb_source_file("/artichoke/virtual_root/src/lib/foo/bar.rb", &b"# a source file"[..])
                .unwrap();
            let result = interp
                .eval(b"require '/artichoke/virtual_root/src/lib/foo/bar/source.rb'")
                .unwrap();
            assert!(result.try_convert_into::<bool>(&interp).unwrap());
            let result = interp
                .eval(b"require '/artichoke/virtual_root/src/lib/foo/bar.rb'")
                .unwrap();
            assert!(!result.try_convert_into::<bool>(&interp).unwrap());
        };
    }

    #[test]
    fn directory_err() {
        let mut interp = interpreter();
        let err = interp.eval(b"require '/src'").unwrap_err();
        assert_eq!(
            b"cannot load such file -- /src".as_bstr(),
            err.message().as_ref().as_bstr()
        );
        let expected_backtrace = b"(eval):1:in require\n(eval):1".to_vec();
        let actual_backtrace = bstr::join("\n", err.vm_backtrace(&mut interp).unwrap());
        assert_eq!(expected_backtrace.as_bstr(), actual_backtrace.as_bstr());
    }

    #[test]
    fn path_defined_as_source_then_extension_file() {
        let mut interp = interpreter();
        interp
            .def_rb_source_file("foo.rb", &b"module Foo; RUBY = 3; end"[..])
            .unwrap();
        interp
            .def_file_for_type::<_, MockExtensionAndSourceFile>("foo.rb")
            .unwrap();
        let result = interp.eval(b"require 'foo'").unwrap();
        let result = result.try_convert_into::<bool>(&interp).unwrap();
        assert!(result, "successfully required foo.rb");
        let result = interp.eval(b"Foo::RUBY + Foo::RUST").unwrap();
        let result = result.try_convert_into::<i64>(&interp).unwrap();
        assert_eq!(result, 10, "defined Ruby and Rust sources from single require");
    }

    #[test]
    fn path_defined_as_extension_file_then_source() {
        let mut interp = interpreter();
        interp
            .def_file_for_type::<_, MockExtensionAndSourceFile>("foo.rb")
            .unwrap();
        interp
            .def_rb_source_file("foo.rb", &b"module Foo; RUBY = 3; end"[..])
            .unwrap();
        let result = interp.eval(b"require 'foo'").unwrap();
        let result = result.try_convert_into::<bool>(&interp).unwrap();
        assert!(result, "successfully required foo.rb");
        let result = interp.eval(b"Foo::RUBY + Foo::RUST").unwrap();
        let result = result.try_convert_into::<i64>(&interp).unwrap();
        assert_eq!(result, 10, "defined Ruby and Rust sources from single require");
    }
}
