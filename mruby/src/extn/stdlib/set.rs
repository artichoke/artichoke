use crate::interpreter::Mrb;
use crate::load::MrbLoadSources;
use crate::MrbError;

pub fn init(interp: &Mrb) -> Result<(), MrbError> {
    interp.borrow_mut().def_class::<Set>("Set", None, None);
    interp
        .borrow_mut()
        .def_class::<SortedSet>("SortedSet", None, None);
    interp.def_rb_source_file("set.rb", include_str!("set.rb"))?;
    Ok(())
}

pub struct Set;
#[allow(clippy::module_name_repetitions)]
pub struct SortedSet;

// Set tests from Ruby stdlib docs
// https://ruby-doc.org/stdlib-2.6.3/libdoc/set/rdoc/Set.html
// https://ruby-doc.org/stdlib-2.6.3/libdoc/set/rdoc/SortedSet.html
#[cfg(test)]
mod tests {
    use crate::convert::FromMrb;
    use crate::eval::MrbEval;
    use crate::interpreter::{Interpreter, MrbApi};
    use crate::value::{Value, ValueLike};

    #[test]
    fn set() {
        let interp = Interpreter::create().expect("mrb init");
        interp.eval("require 'set'").expect("require");
        let s1 = interp.eval("s1 = Set[1, 2]").unwrap();
        let s2 = interp.eval("s2 = [1, 2].to_set").unwrap();
        assert_eq!(s1.funcall::<bool, _, _>("==", &[s2]), Ok(true));
        let s1 = interp.eval("s1 = Set[1, 2]").unwrap();
        let s1 = s1
            .funcall::<Value, _, _>("add", &[interp.string("foo")])
            .unwrap();
        let s1 = s1
            .funcall::<Value, _, _>(
                "merge",
                &[Value::from_mrb(
                    &interp,
                    vec![interp.fixnum(2), interp.fixnum(6)],
                )],
            )
            .unwrap();
        let s2 = interp.eval("s2 = [1, 2].to_set").unwrap();
        assert_eq!(s1.funcall::<bool, _, _>("subset?", &[s2]), Ok(false));
        let s2 = interp.eval("s2 = [1, 2].to_set").unwrap();
        assert_eq!(s2.funcall::<bool, _, _>("subset?", &[s1]), Ok(true));
    }

    #[test]
    fn sorted_set() {
        let interp = Interpreter::create().expect("mrb init");
        interp.eval("require 'set'").expect("require");
        let s1 = interp.eval("s1 = Set[1, 2]").unwrap();
        let s2 = interp.eval("s2 = [1, 2].to_set").unwrap();
        assert_eq!(s1.funcall::<bool, _, _>("==", &[s2]), Ok(true));
        let result = interp
            .eval(
                "
set = SortedSet.new([2, 1, 5, 6, 4, 5, 3, 3, 3])

set.each_with_object([]) do |obj, ary|
  ary << obj
end
                ",
            )
            .unwrap();
        let result = result.funcall::<Vec<i64>, _, _>("itself", &[]);
        assert_eq!(result, Ok(vec![1, 2, 3, 4, 5, 6]));
    }
}
