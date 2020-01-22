use crate::extn::core::artichoke;
use crate::extn::prelude::*;

pub mod integer;
pub mod require;

pub fn init(interp: &mut Artichoke) -> InitializeResult<()> {
    if interp.state().module_spec::<Kernel>().is_some() {
        return Ok(());
    }
    let spec = module::Spec::new("Kernel", None)?;
    module::Builder::for_spec(interp, &spec)
        .add_method("require", Kernel::require, sys::mrb_args_rest())?
        .add_method(
            "require_relative",
            Kernel::require_relative,
            sys::mrb_args_rest(),
        )?
        .add_method("load", Kernel::load, sys::mrb_args_rest())?
        .add_method("print", Kernel::print, sys::mrb_args_rest())?
        .add_method("puts", Kernel::puts, sys::mrb_args_rest())?
        .define()?;
    interp.state_mut().def_module::<Kernel>(spec);
    let _ = interp.eval(&include_bytes!("kernel.rb")[..])?;
    trace!("Patched Kernel onto interpreter");
    let scope = interp
        .state()
        .module_spec::<artichoke::Artichoke>()
        .map(EnclosingRubyScope::module)
        .ok_or(ArtichokeError::New)?;
    let spec = module::Spec::new("Kernel", Some(scope))?;
    module::Builder::for_spec(interp, &spec)
        .add_method("Integer", Kernel::integer, sys::mrb_args_req_and_opt(1, 1))?
        .add_self_method("Integer", Kernel::integer, sys::mrb_args_req_and_opt(1, 1))?
        .define()?;
    interp.state_mut().def_module::<artichoke::Kernel>(spec);
    trace!("Patched Artichoke::Kernel onto interpreter");
    Ok(())
}

pub struct Kernel;

impl Kernel {
    unsafe extern "C" fn integer(mrb: *mut sys::mrb_state, _slf: sys::mrb_value) -> sys::mrb_value {
        let (arg, base) = mrb_get_args!(mrb, required = 1, optional = 1);
        let interp = unwrap_interpreter!(mrb);
        let result = integer::method(
            &mut interp,
            Value::new(&interp, arg),
            base.map(|base| Value::new(&interp, base)),
        );
        match result {
            Ok(value) => value.inner(),
            Err(exception) => exception::raise(interp, exception),
        }
    }

    unsafe extern "C" fn load(mrb: *mut sys::mrb_state, _slf: sys::mrb_value) -> sys::mrb_value {
        let file = mrb_get_args!(mrb, required = 1);
        let interp = unwrap_interpreter!(mrb);
        let file = Value::new(&interp, file);
        let result = require::load(&mut interp, file);
        match result {
            Ok(value) => value.inner(),
            Err(exception) => exception::raise(interp, exception),
        }
    }

    unsafe extern "C" fn print(mrb: *mut sys::mrb_state, _slf: sys::mrb_value) -> sys::mrb_value {
        let args = mrb_get_args!(mrb, *args);
        let interp = unwrap_interpreter!(mrb);

        let mut buf = vec![];
        for value in args.iter().copied() {
            let to_s = Value::new(&interp, value).to_s(&mut interp);
            buf.extend(to_s);
        }
        interp.state_mut().print(buf.as_slice());
        sys::mrb_sys_nil_value()
    }

    unsafe extern "C" fn puts(mrb: *mut sys::mrb_state, _slf: sys::mrb_value) -> sys::mrb_value {
        fn do_puts(interp: &mut Artichoke, value: &Value, buf: &mut Vec<u8>) {
            if let Ok(array) = value.try_into::<Vec<Value>>(interp) {
                for value in array {
                    do_puts(interp, &value, buf);
                }
            } else {
                buf.extend(value.to_s(interp));
                buf.push(b'\n');
            }
        }

        let args = mrb_get_args!(mrb, *args);
        let interp = unwrap_interpreter!(mrb);
        if args.is_empty() {
            interp.state_mut().puts(&[]);
        } else {
            let mut buf = vec![];
            for value in args.iter().copied() {
                do_puts(&mut interp, &Value::new(&interp, value), &mut buf);
            }
            interp.state_mut().print(buf.as_slice());
        }
        sys::mrb_sys_nil_value()
    }

    unsafe extern "C" fn require(mrb: *mut sys::mrb_state, _slf: sys::mrb_value) -> sys::mrb_value {
        let file = mrb_get_args!(mrb, required = 1);
        let interp = unwrap_interpreter!(mrb);
        let file = Value::new(&interp, file);
        let result = require::require(&mut interp, file, None);
        match result {
            Ok(value) => value.inner(),
            Err(exception) => exception::raise(interp, exception),
        }
    }

    unsafe extern "C" fn require_relative(
        mrb: *mut sys::mrb_state,
        _slf: sys::mrb_value,
    ) -> sys::mrb_value {
        let file = mrb_get_args!(mrb, required = 1);
        let interp = unwrap_interpreter!(mrb);
        let file = Value::new(&interp, file);
        let result = require::require_relative(&mut interp, file);
        match result {
            Ok(value) => value.inner(),
            Err(exception) => exception::raise(interp, exception),
        }
    }
}

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

            fn require(interp: &Artichoke) -> Result<(), ArtichokeError> {
                let _ = interp.eval(b"@i = 255").unwrap();
                Ok(())
            }
        }

        let interp = crate::interpreter().expect("init");
        interp
            .def_file_for_type::<TestFile>(b"file.rb")
            .expect("def file");
        let result = interp.eval(b"require 'file'").expect("eval");
        let require_result = result.try_into::<bool>();
        assert_eq!(require_result, Ok(true));
        let result = interp.eval(b"@i").expect("eval");
        let i_result = result.try_into::<i64>();
        assert_eq!(i_result, Ok(255));
        let result = interp.eval(b"@i = 1000; require 'file'").expect("eval");
        let second_require_result = result.try_into::<bool>();
        assert_eq!(second_require_result, Ok(false));
        let result = interp.eval(b"@i").expect("eval");
        let second_i_result = result.try_into::<i64>();
        assert_eq!(second_i_result, Ok(1000));
        let err = interp.eval(b"require 'non-existent-source'").unwrap_err();
        assert_eq!(
            &b"cannot load such file -- non-existent-source"[..],
            err.message()
        );
        let expected = vec![Vec::from(&b"(eval):1"[..])];
        assert_eq!(Some(expected), err.backtrace(&interp),);
    }

    #[test]
    fn require_absolute_path() {
        let interp = crate::interpreter().expect("init");
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
        let interp = crate::interpreter().expect("init");
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
        let interp = crate::interpreter().expect("init");
        let err = interp.eval(b"require '/src'").unwrap_err();
        assert_eq!(&b"cannot load such file -- /src"[..], err.message());
        let expected = vec![Vec::from(&b"(eval):1"[..])];
        assert_eq!(Some(expected), err.backtrace(&interp),);
    }

    #[test]
    fn require_path_defined_as_source_then_mrbfile() {
        struct Foo;

        impl File for Foo {
            type Artichoke = Artichoke;

            fn require(interp: &Artichoke) -> Result<(), ArtichokeError> {
                let _ = interp.eval(b"module Foo; RUST = 7; end").unwrap();
                Ok(())
            }
        }
        let interp = crate::interpreter().expect("init");
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

            fn require(interp: &Artichoke) -> Result<(), ArtichokeError> {
                let _ = interp.eval(b"module Foo; RUST = 7; end").unwrap();
                Ok(())
            }
        }
        let interp = crate::interpreter().expect("init");
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
        let interp = crate::interpreter().expect("init");
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
