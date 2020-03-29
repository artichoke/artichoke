use crate::extn::prelude::*;

pub fn init(interp: &mut Artichoke) -> InitializeResult<()> {
    let spec = module::Spec::new(interp, "Forwardable", None)?;
    interp.0.borrow_mut().def_module::<Forwardable>(spec);
    interp.def_rb_source_file(b"forwardable.rb", &include_bytes!("forwardable.rb")[..])?;
    interp.def_rb_source_file(
        b"forwardable/impl.rb",
        &include_bytes!("forwardable/impl.rb")[..],
    )?;
    Ok(())
}

#[derive(Debug)]
pub struct Forwardable;

// Forwardable tests from Ruby stdlib docs
// https://ruby-doc.org/stdlib-2.6.3/libdoc/forwardable/rdoc/Forwardable.html
// TODO: Move these tests to a dedicated Ruby file like the inline buffer tests.
#[cfg(test)]
mod tests {
    use crate::test::prelude::*;

    #[test]
    #[allow(clippy::shadow_unrelated)]
    // TODO(GH-528): fix failing tests on Windows.
    #[cfg_attr(target_os = "windows", should_panic)]
    fn forwardable() {
        let mut interp = crate::interpreter().expect("init");
        let _ = interp
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
        let _ = interp
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
    // TODO(GH-528): fix failing tests on Windows.
    #[cfg_attr(target_os = "windows", should_panic)]
    fn forwardable_another_example() {
        let mut interp = crate::interpreter().expect("init");
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
            .try_into::<Vec<Option<&str>>>()
            .unwrap();
        assert_eq!(
            result,
            vec![
                Some("2"),
                Some("3"),
                Some("4"),
                Some("5"),
                Some("6"),
                Some("Ruby"),
                None
            ]
        );
    }

    #[test]
    // TODO(GH-528): fix failing tests on Windows.
    #[cfg_attr(target_os = "windows", should_panic)]
    fn forwardable_def_instance_delegator() {
        let mut interp = crate::interpreter().expect("init");
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
