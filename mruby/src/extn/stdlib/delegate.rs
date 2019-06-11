use crate::interpreter::Mrb;
use crate::load::MrbLoadSources;
use crate::MrbError;

pub fn init(interp: &Mrb) -> Result<(), MrbError> {
    interp
        .borrow_mut()
        .def_class::<Delegator>("Delegator", None, None);
    interp
        .borrow_mut()
        .def_class::<SimpleDelegator>("SimpleDelegator", None, None);
    interp.def_rb_source_file("delegate.rb", include_str!("delegate.rb"))?;
    Ok(())
}

pub struct Delegator;
pub struct SimpleDelegator;

// Delegate tests from Ruby stdlib docs
// https://ruby-doc.org/stdlib-2.6.3/libdoc/delegate/rdoc/Delegator.html
// https://ruby-doc.org/stdlib-2.6.3/libdoc/delegate/rdoc/Object.html
// https://ruby-doc.org/stdlib-2.6.3/libdoc/delegate/rdoc/SimpleDelegator.html
#[cfg(test)]
mod tests {
    use crate::convert::FromMrb;
    use crate::eval::MrbEval;
    use crate::interpreter::{Interpreter, MrbApi};
    use crate::value::{Value, ValueLike};

    #[test]
    fn simple_delegator() {
        let interp = Interpreter::create().expect("mrb init");
        interp.eval("require 'delegate'").expect("require");
        // Hack because Date is not yet implemented
        interp
            .eval("Date = Struct.new(:year, :month, :day)")
            .expect("date");
        let value = interp
            .eval(
                r#"
class User
  def born_on
    Date.new(1989, 9, 10)
  end
end

class UserDecorator < SimpleDelegator
  def birth_year
    born_on.year
  end
end

decorated_user = UserDecorator.new(User.new)
                "#,
            )
            .unwrap();
        assert_eq!(value.funcall::<i64, _, _>("birth_year", &[]), Ok(1989));
        let user = value.funcall::<Value, _, _>("__getobj__", &[]).unwrap();
        let class = user.funcall::<Value, _, _>("class", &[]).unwrap();
        assert_eq!(&class.funcall::<String, _, _>("name", &[]).unwrap(), "User");
    }

    #[test]
    fn simple_delegator_super() {
        let interp = Interpreter::create().expect("mrb init");
        interp.eval("require 'delegate'").expect("require");
        let value = interp
            .eval(
                r#"
class SuperArray < SimpleDelegator
  def [](*args)
    super + 1
  end
end

SuperArray.new([1])
                "#,
            )
            .unwrap();
        assert_eq!(value.funcall::<i64, _, _>("[]", &[interp.fixnum(0)]), Ok(2));
    }

    #[test]
    fn simple_delegator_change_delegation_obj() {
        let interp = Interpreter::create().expect("mrb init");
        interp.eval("require 'delegate'").expect("require");
        // Hack because Date is not yet implemented
        interp
            .eval("Date = Struct.new(:year, :month, :day)")
            .expect("date");
        let value = interp
            .eval(
                r#"
class Stats
  def initialize
    @source = SimpleDelegator.new([])
  end

  def stats(records)
    @source.__setobj__(records)

    # [elements, non-nil, unique]
    [@source.size, @source.compact.size, @source.uniq.size]
  end
end

s = Stats.new
# puts s.stats(%w{James Edward Gray II})
# puts
# puts s.stats([1, 2, 3, nil, 4, 5, 1, 2])
                "#,
            )
            .unwrap();
        assert_eq!(
            value.funcall::<Vec<i64>, _, _>(
                "stats",
                &[Value::from_mrb(
                    &interp,
                    vec![
                        interp.string("James"),
                        interp.string("Edward"),
                        interp.string("Gray"),
                        interp.string("II")
                    ]
                )]
            ),
            Ok(vec![4, 4, 4])
        );
        assert_eq!(
            value.funcall::<Vec<i64>, _, _>(
                "stats",
                &[Value::from_mrb(
                    &interp,
                    vec![
                        interp.fixnum(1),
                        interp.fixnum(2),
                        interp.fixnum(3),
                        interp.nil(),
                        interp.fixnum(4),
                        interp.fixnum(5),
                        interp.fixnum(1),
                        interp.fixnum(2),
                    ]
                )]
            ),
            Ok(vec![8, 7, 6])
        );
    }

    #[test]
    fn delegate_class() {
        let interp = Interpreter::create().expect("mrb init");
        interp.eval("require 'delegate'").expect("require");
        let value = interp.eval(
            r#"
class MyClass < DelegateClass(Array) # Step 1
  def initialize
    super([])                        # Step 2
  end
end

MyClass.instance_methods
MyClass.new
                "#,
        );
        assert!(value.is_ok());
    }
}
