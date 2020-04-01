pub mod integer;
pub mod mruby;
pub mod require;
pub mod trampoline;

#[derive(Debug)]
pub struct Kernel;

#[cfg(test)]
mod tests {
    use crate::test::prelude::*;

    // Integration test for `Kernel::require`:
    //
    // - require side effects (e.g. ivar set or class def) effect the interpreter
    // - Successful first require returns `true`.
    // - Second require returns `false`.
    // - Second require does not cause require side effects.
    // - Require non-existing file raises and returns `nil`.
    #[test]
    fn require() {
        struct TestFile;

        impl File for TestFile {
            type Artichoke = Artichoke;

            type Error = Exception;

            fn require(interp: &mut Artichoke) -> Result<(), Self::Error> {
                let _ = interp.eval(b"@i = 255").unwrap();
                Ok(())
            }
        }

        let mut interp = crate::interpreter().expect("init");
        interp
            .def_file_for_type::<TestFile>(b"file.rb")
            .expect("def file");
        let result = interp.eval(b"require 'file'").expect("eval");
        let require_result = result.try_into::<bool>().unwrap();
        assert_eq!(require_result, true);
        let result = interp.eval(b"@i").expect("eval");
        let i_result = result.try_into::<i64>().unwrap();
        assert_eq!(i_result, 255);
        let result = interp.eval(b"@i = 1000; require 'file'").expect("eval");
        let second_require_result = result.try_into::<bool>().unwrap();
        assert_eq!(second_require_result, false);
        let result = interp.eval(b"@i").expect("eval");
        let second_i_result = result.try_into::<i64>().unwrap();
        assert_eq!(second_i_result, 1000);
        let err = interp.eval(b"require 'non-existent-source'").unwrap_err();
        assert_eq!(
            &b"cannot load such file -- non-existent-source"[..],
            err.message()
        );
        let expected = vec![Vec::from(&b"(eval):1"[..])];
        assert_eq!(Some(expected), err.vm_backtrace(&mut interp),);
    }

    #[test]
    fn require_absolute_path() {
        let mut interp = crate::interpreter().expect("init");
        interp
            .def_rb_source_file(b"/foo/bar/source.rb", &b"# a source file"[..])
            .expect("def file");
        let result = interp.eval(b"require '/foo/bar/source.rb'").expect("value");
        assert!(result.try_into::<bool>().expect("convert"));
        let result = interp.eval(b"require '/foo/bar/source.rb'").expect("value");
        assert!(!result.try_into::<bool>().expect("convert"));
    }

    #[test]
    fn require_relative_with_dotted_path() {
        let mut interp = crate::interpreter().expect("init");
        interp
            .def_rb_source_file(b"/foo/bar/source.rb", &b"require_relative '../bar.rb'"[..])
            .expect("def file");
        interp
            .def_rb_source_file(b"/foo/bar.rb", &b"# a source file"[..])
            .expect("def file");
        let result = interp.eval(b"require '/foo/bar/source.rb'").expect("value");
        assert!(result.try_into::<bool>().expect("convert"));
    }

    #[test]
    fn require_directory() {
        let mut interp = crate::interpreter().expect("init");
        let err = interp.eval(b"require '/src'").unwrap_err();
        assert_eq!(&b"cannot load such file -- /src"[..], err.message());
        let expected = vec![Vec::from(&b"(eval):1"[..])];
        assert_eq!(Some(expected), err.vm_backtrace(&mut interp),);
    }

    #[test]
    fn require_path_defined_as_source_then_mrbfile() {
        struct Foo;

        impl File for Foo {
            type Artichoke = Artichoke;

            type Error = Exception;

            fn require(interp: &mut Artichoke) -> Result<(), Self::Error> {
                let _ = interp.eval(b"module Foo; RUST = 7; end").unwrap();
                Ok(())
            }
        }
        let mut interp = crate::interpreter().expect("init");
        interp
            .def_rb_source_file(b"foo.rb", &b"module Foo; RUBY = 3; end"[..])
            .expect("def");
        interp.def_file_for_type::<Foo>(b"foo.rb").expect("def");
        let result = interp.eval(b"require 'foo'").expect("eval");
        let result = result.try_into::<bool>().expect("convert");
        assert!(result, "successfully required foo.rb");
        let result = interp.eval(b"Foo::RUBY + Foo::RUST").expect("eval");
        let result = result.try_into::<i64>().expect("convert");
        assert_eq!(
            result, 10,
            "defined Ruby and Rust sources from single require"
        );
    }

    #[test]
    fn require_path_defined_as_mrbfile_then_source() {
        struct Foo;

        impl File for Foo {
            type Artichoke = Artichoke;

            type Error = Exception;

            fn require(interp: &mut Artichoke) -> Result<(), Self::Error> {
                let _ = interp.eval(b"module Foo; RUST = 7; end").unwrap();
                Ok(())
            }
        }
        let mut interp = crate::interpreter().expect("init");
        interp.def_file_for_type::<Foo>(b"foo.rb").expect("def");
        interp
            .def_rb_source_file(b"foo.rb", &b"module Foo; RUBY = 3; end"[..])
            .expect("def");
        let result = interp.eval(b"require 'foo'").expect("eval");
        let result = result.try_into::<bool>().expect("convert");
        assert!(result, "successfully required foo.rb");
        let result = interp.eval(b"Foo::RUBY + Foo::RUST").expect("eval");
        let result = result.try_into::<i64>().expect("convert");
        assert_eq!(
            result, 10,
            "defined Ruby and Rust sources from single require"
        );
    }

    #[test]
    #[allow(clippy::shadow_unrelated)]
    fn kernel_throw_catch() {
        // https://ruby-doc.org/core-2.6.3/Kernel.html#method-i-catch
        let mut interp = crate::interpreter().expect("init");
        let result = interp
            .eval(b"catch(1) { 123 }")
            .unwrap()
            .try_into::<i64>()
            .unwrap();
        assert_eq!(result, 123);
        let result = interp
            .eval(b"catch(1) { throw(1, 456) }")
            .unwrap()
            .try_into::<i64>()
            .unwrap();
        assert_eq!(result, 456);
        let result = interp
            .eval(b"catch(1) { throw(1) }")
            .unwrap()
            .try_into::<Option<i64>>()
            .unwrap();
        assert_eq!(result, None);
        let result = interp
            .eval(b"catch(1) {|x| x + 2 }")
            .unwrap()
            .try_into::<i64>()
            .unwrap();
        assert_eq!(result, 3);

        let result = interp
            .eval(
                br#"
catch do |obj_A|
  catch do |obj_B|
    throw(obj_B, 123)
    # puts "This puts is not reached"
  end

  # puts "This puts is displayed"
  456
end
            "#,
            )
            .unwrap()
            .try_into::<i64>()
            .unwrap();
        assert_eq!(result, 456);
        let result = interp
            .eval(
                br#"
catch do |obj_A|
  catch do |obj_B|
    throw(obj_A, 123)
    # puts "This puts is still not reached"
  end

  # puts "Now this puts is also not reached"
  456
end
            "#,
            )
            .unwrap()
            .try_into::<i64>()
            .unwrap();
        assert_eq!(result, 123);
    }
}
