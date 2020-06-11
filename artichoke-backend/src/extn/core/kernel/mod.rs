pub mod integer;
pub mod mruby;
pub mod require;
pub mod trampoline;

#[derive(Debug)]
pub struct Kernel;

#[cfg(test)]
mod tests {
    use crate::test::prelude::*;

    #[test]
    fn integration_test() {
        let mut interp = crate::interpreter().unwrap();
        let _ = interp.eval(&include_bytes!("kernel_test.rb")[..]).unwrap();
        let result = interp.eval(b"spec");
        let result = result.unwrap().try_into::<bool>(&interp).unwrap();
        assert!(result);
    }

    mod require {
        use crate::test::prelude::*;

        #[derive(Debug)]
        struct IntegrationTest;

        impl File for IntegrationTest {
            type Artichoke = Artichoke;

            type Error = Exception;

            fn require(interp: &mut Artichoke) -> Result<(), Self::Error> {
                let _ = interp.eval(b"@i = 255").unwrap();
                Ok(())
            }
        }

        #[derive(Debug)]
        struct HybridRustAndRuby;

        impl File for HybridRustAndRuby {
            type Artichoke = Artichoke;

            type Error = Exception;

            fn require(interp: &mut Artichoke) -> Result<(), Self::Error> {
                let _ = interp.eval(b"module Foo; RUST = 7; end").unwrap();
                Ok(())
            }
        }

        // Integration test for `Kernel::require`:
        //
        // - require side effects (e.g. ivar set or class def) effect the interpreter
        // - Successful first require returns `true`.
        // - Second require returns `false`.
        // - Second require does not cause require side effects.
        // - Require non-existing file raises and returns `nil`.
        #[test]
        fn integration_test() {
            let mut interp = crate::interpreter().unwrap();
            interp
                .def_file_for_type::<_, IntegrationTest>("file.rb")
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
                .def_file_for_type::<_, HybridRustAndRuby>("foo.rb")
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
                .def_file_for_type::<_, HybridRustAndRuby>("foo.rb")
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
}
