#![deny(warnings, intra_doc_link_resolution_failure)]

#[macro_use]
extern crate rust_embed;

use artichoke_backend::Mrb;
use artichoke_backend::MrbError;

pub mod rubygems;

/// Define a Rubygem that can be installed into an [`Mrb`] interpreter.
pub trait Gem {
    /// Initialize a gem in the [`Mrb`] interpreter.
    fn init(interp: &Mrb) -> Result<(), MrbError>;
}

#[cfg(test)]
mod tests {
    use mruby::def::EnclosingRubyScope;
    use mruby::eval::MrbEval;
    use mruby::file::MrbFile;
    use mruby::load::MrbLoadSources;
    use mruby::{Mrb, MrbError};

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
            let scope = interp
                .borrow()
                .module_spec::<Foo>()
                .map(EnclosingRubyScope::module)
                .ok_or_else(|| MrbError::NotDefined("Foo".to_owned()))?;
            interp
                .borrow_mut()
                .def_class::<Self>("Bar", Some(scope), None);
            Ok(())
        }
    }

    #[test]
    fn require_mrbfile_before_sources() {
        let interp = mruby::interpreter().expect("mrb init");
        Foo::init(&interp).expect("gem init");
        assert_eq!(interp.eval("require 'foo'").map(|_| ()), Ok(()));
    }
}
