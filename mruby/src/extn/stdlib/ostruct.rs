use crate::interpreter::Mrb;
use crate::load::MrbLoadSources;
use crate::MrbError;

pub fn init(interp: &Mrb) -> Result<(), MrbError> {
    interp
        .borrow_mut()
        .def_class::<OpenStruct>("OpenStruct", None, None);
    interp.def_rb_source_file("ostruct.rb", include_str!("ostruct.rb"))?;
    Ok(())
}

pub struct OpenStruct;

// OpenStruct tests from Ruby stdlib docs
// https://ruby-doc.org/stdlib-2.6.3/libdoc/ostruct/rdoc/OpenStruct.html
#[cfg(test)]
mod tests {
    use crate::eval::MrbEval;
    use crate::interpreter::{Interpreter, MrbApi};
    use crate::value::{Value, ValueLike};

    #[test]
    fn ostruct() {
        let interp = Interpreter::create().expect("mrb init");
        interp.eval("require 'ostruct'").expect("require");
        let person = interp.eval("person = OpenStruct.new").unwrap();
        person
            .funcall::<(), _, _>("name=", &[interp.string("John Smith")])
            .unwrap();
        person
            .funcall::<(), _, _>("age=", &[interp.fixnum(70)])
            .unwrap();
        let name = person.funcall::<String, _, _>("name", &[]).unwrap();
        assert_eq!(&name, "John Smith");
        let age = person.funcall::<i64, _, _>("age", &[]).unwrap();
        assert_eq!(age, 70);
        let address = person
            .funcall::<Option<Value>, _, _>("address", &[])
            .unwrap();
        assert!(address.is_none());
    }
}
