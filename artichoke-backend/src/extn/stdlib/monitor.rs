use crate::load::LoadSources;
use crate::{Artichoke, ArtichokeError};

pub fn init(interp: &Artichoke) -> Result<(), ArtichokeError> {
    interp
        .0
        .borrow_mut()
        .def_class::<Monitor>("Monitor", None, None);
    interp.def_rb_source_file("monitor.rb", include_str!("monitor.rb"))?;
    Ok(())
}

pub struct Monitor;

// Monitor tests from ruby/spec
// https://github.com/ruby/spec/tree/master/library/monitor
#[cfg(test)]
mod tests {
    use crate::eval::Eval;
    use crate::value::ValueLike;

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
        let interp = crate::interpreter().expect("init");
        interp.eval("require 'monitor'").expect("require");
        let result = interp.eval(spec).expect("spec");
        assert!(result.try_into::<bool>().expect("convert"));
    }
}
