use crate::interpreter::Mrb;
use crate::load::MrbLoadSources;
use crate::MrbError;

pub fn init(interp: &Mrb) -> Result<(), MrbError> {
    interp
        .borrow_mut()
        .def_class::<Monitor>("Monitor", None, None);
    interp.def_rb_source_file("monitor.rb", include_str!("monitor.rb"))
}

pub struct Monitor;

// Monitor tests from ruby/spec
// https://github.com/ruby/spec/tree/master/library/monitor
#[cfg(test)]
mod tests {
    use crate::convert::TryFromMrb;
    use crate::eval::MrbEval;
    use crate::interpreter::Interpreter;

    #[test]
    fn mon_initialize() {
        let spec = r#"
cls = Class.new do
  include MonitorMixin

  def initialize(*array)
    mon_initialize
    @array = array
  end

  def to_a
    synchronize { @array.dup }
  end

  def initialize_copy(other)
    mon_initialize

    synchronize do
      @array = other.to_a
    end
  end
end


instance = cls.new(1, 2, 3)
copy = instance.dup
copy != instance
# The below requires mspec
# copy.should_not equal(instance)
"#;
        let interp = Interpreter::create().expect("mrb init");
        interp.eval("require 'monitor'").expect("require");
        let result = interp.eval(spec).expect("spec");
        assert!(unsafe { bool::try_from_mrb(&interp, result) }.expect("convert"));
    }
}
