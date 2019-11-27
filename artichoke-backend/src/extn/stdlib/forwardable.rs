use crate::load::LoadSources;
use crate::{Artichoke, ArtichokeError};

pub fn init(interp: &Artichoke) -> Result<(), ArtichokeError> {
    interp
        .0
        .borrow_mut()
        .def_module::<Forwardable>("Forwardable", None);
    interp.def_rb_source_file("forwardable.rb", include_str!("forwardable.rb"))?;
    interp.def_rb_source_file("forwardable/impl.rb", include_str!("forwardable/impl.rb"))?;
    Ok(())
}

pub struct Forwardable;

// Forwardable tests from Ruby stdlib docs
// https://ruby-doc.org/stdlib-2.6.3/libdoc/forwardable/rdoc/Forwardable.html
#[cfg(test)]
mod tests {
    use artichoke_core::eval::Eval;
    use artichoke_core::value::Value as _;

    #[test]
    #[allow(clippy::shadow_unrelated)]
    fn forwardable() {
        let interp = crate::interpreter().expect("init");
        interp
            .eval(
                br#"
require 'forwardable'

class RecordCollection
  attr_accessor :records
  extend Forwardable
  def_delegator :@records, :[], :record_number
end
                "#,
            )
            .unwrap();
        let result = interp
            .eval(
                br#"
r = RecordCollection.new
r.records = [4,5,6]
r.record_number(0)
                "#,
            )
            .unwrap()
            .try_into::<i64>()
            .unwrap();
        assert_eq!(result, 4);
        interp
            .eval(
                br#"
class RecordCollection # re-open RecordCollection class
  def_delegators :@records, :size, :<<, :map
end
                "#,
            )
            .unwrap();
        let result = interp
            .eval(
                br#"
r = RecordCollection.new
r.records = [1,2,3]
r.record_number(0)
                "#,
            )
            .unwrap()
            .try_into::<i64>()
            .unwrap();
        assert_eq!(result, 1);
        let result = interp.eval(b"r.size").unwrap().try_into::<i64>().unwrap();
        assert_eq!(result, 3);
        let result = interp
            .eval(b"r << 4")
            .unwrap()
            .try_into::<Vec<i64>>()
            .unwrap();
        assert_eq!(result, vec![1, 2, 3, 4]);
        let result = interp
            .eval(b"r.map { |x| x * 2 }")
            .unwrap()
            .try_into::<Vec<i64>>()
            .unwrap();
        assert_eq!(result, vec![2, 4, 6, 8]);
    }

    #[test]
    fn forwardable_another_example() {
        let interp = crate::interpreter().expect("init");
        let result = interp
            .eval(
                br#"
require 'forwardable'

class Queue
  extend Forwardable

  def initialize
    @q = [ ]    # prepare delegate object
  end

  # setup preferred interface, enq() and deq()...
  def_delegator :@q, :push, :enq
  def_delegator :@q, :shift, :deq

  # support some general Array methods that fit Queues well
  def_delegators :@q, :clear, :first, :push, :shift, :size
end

out = []

q = Queue.new
q.enq 1, 2, 3, 4, 5
q.push 6

q.shift    # => 1
while q.size > 0
  out << q.deq.to_s
end

q.enq "Ruby", "Perl", "Python"
out << q.first
q.clear
out << q.first
                "#,
            )
            .unwrap()
            .try_into::<Vec<Option<String>>>()
            .unwrap();
        assert_eq!(
            result,
            vec![
                Some("2".to_owned()),
                Some("3".to_owned()),
                Some("4".to_owned()),
                Some("5".to_owned()),
                Some("6".to_owned()),
                Some("Ruby".to_owned()),
                None
            ]
        );
    }

    #[test]
    fn forwardable_def_instance_delegator() {
        let interp = crate::interpreter().expect("init");
        let result = interp
            .eval(
                br#"
require 'forwardable'

class MyQueue
  extend Forwardable
  attr_reader :queue
  def initialize
    @queue = []
  end

  def_delegator :@queue, :push, :mypush
end

q = MyQueue.new
q.mypush 42
# q.queue    #=> [42]
raise 'fail' unless q.queue == [42]
# q.push 23  #=> NoMethodError
begin
  q.push 23
  false
rescue NoMethodError
  true
end
                "#,
            )
            .unwrap()
            .try_into::<bool>()
            .unwrap();
        assert!(result);
    }
}
