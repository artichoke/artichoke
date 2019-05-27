#![deny(warnings, intra_doc_link_resolution_failure)]

#[macro_use]
extern crate rust_embed;

use mruby::interpreter::Mrb;
use mruby::MrbError;

pub mod rubygems;

/// Define a Rubygem that can be installed into an [`Mrb`] interpreter.
pub trait Gem {
    /// Initialize a gem in the [`Mrb`] interpreter.
    fn init(interp: &Mrb) -> Result<(), MrbError>;
}

#[cfg(test)]
mod tests {
    use mruby::def::Parent;
    use mruby::eval::MrbEval;
    use mruby::file::MrbFile;
    use mruby::interpreter::{Interpreter, Mrb};
    use mruby::load::MrbLoadSources;
    use mruby::MrbError;
    use std::rc::Rc;

    use crate::Gem;

    struct Foo;

    impl Gem for Foo {
        fn init(interp: &Mrb) -> Result<(), MrbError> {
            interp
                .def_rb_source_file("foo.rb", "require 'foo/bar'; module Foo; CONST = 10; end")?;
            interp
                .def_rb_source_file("foo/bar.rb", "module Foo; module Bar; CONST = 10; end; end")?;
            interp.def_file_for_type::<_, Foo>("foo.rb")?;
            interp.def_file_for_type::<_, Bar>("foo/bar.rb")?;
            Ok(())
        }
    }

    impl MrbFile for Foo {
        fn require(interp: Mrb) -> Result<(), MrbError> {
            interp.borrow_mut().def_module::<Self>("Foo", None);
            Ok(())
        }
    }

    struct Bar;

    impl MrbFile for Bar {
        fn require(interp: Mrb) -> Result<(), MrbError> {
            let parent = interp
                .borrow()
                .module_spec::<Foo>()
                .ok_or(MrbError::NotDefined("Foo".to_owned()))?;
            let parent = Parent::Module {
                spec: Rc::clone(&parent),
            };
            interp
                .borrow_mut()
                .def_class::<Self>("Bar", Some(parent), None);
            Ok(())
        }
    }

    #[test]
    fn require_mrbfile_before_sources() {
        let interp = Interpreter::create().expect("mrb init");
        Foo::init(&interp).expect("gem init");
        assert_eq!(interp.eval("require 'foo'").map(|_| ()), Ok(()));
    }
}
