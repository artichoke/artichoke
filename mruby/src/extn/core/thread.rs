use crate::eval::{EvalContext, MrbEval};
use crate::interpreter::Mrb;
use crate::MrbError;

pub fn init(interp: &Mrb) -> Result<(), MrbError> {
    interp
        .borrow_mut()
        .def_class::<Thread>("Thread", None, None);
    interp.borrow_mut().def_class::<Mutex>("Mutex", None, None);
    // Thread is loaded by default, so eval it on interpreter initialization
    // https://www.rubydoc.info/gems/rubocop/RuboCop/Cop/Lint/UnneededRequireStatement
    interp.eval_with_context(
        include_str!("thread.rb"),
        EvalContext::new("/src/lib/thread.rb"),
    )?;
    Ok(())
}

pub struct Thread;
pub struct Mutex;

#[cfg(test)]
mod tests {
    #![allow(clippy::shadow_unrelated)]

    use crate::convert::TryFromMrb;
    use crate::eval::MrbEval;
    use crate::interpreter::Interpreter;
    use crate::MrbError;

    #[test]
    fn thread_required_by_default() {
        let interp = Interpreter::create().expect("mrb init");
        let result = interp
            .eval("Object.const_defined?(:Thread)")
            .expect("thread");
        assert!(unsafe { bool::try_from_mrb(&interp, result) }.expect("convert"));
    }

    #[test]
    fn thread_current_is_main() {
        let interp = Interpreter::create().expect("mrb init");
        let spec = "Thread.current == Thread.main";
        let result = interp.eval(spec).expect("spec");
        assert!(unsafe { bool::try_from_mrb(&interp, result) }.expect("convert"));
        let spec = "Thread.new { Thread.current == Thread.main }.join.value == false";
        let result = interp.eval(spec).expect("spec");
        assert!(unsafe { bool::try_from_mrb(&interp, result) }.expect("convert"));
    }

    #[test]
    fn thread_join_value() {
        let interp = Interpreter::create().expect("mrb init");
        let spec = "Thread.new { 2 + 3 }.join.value == 5";
        let result = interp.eval(spec).expect("spec");
        assert!(unsafe { bool::try_from_mrb(&interp, result) }.expect("convert"));
        let spec = "Thread.new { 2 + Thread.new { 3 }.join.value }.join.value == 5";
        let result = interp.eval(spec).expect("spec");
        assert!(unsafe { bool::try_from_mrb(&interp, result) }.expect("convert"));
    }

    #[test]
    fn thread_main_is_running() {
        let interp = Interpreter::create().expect("mrb init");
        let spec = "Thread.current.status == 'run'";
        let result = interp.eval(spec).expect("spec");
        assert!(unsafe { bool::try_from_mrb(&interp, result) }.expect("convert"));
        let spec = "Thread.current.alive?";
        let result = interp.eval(spec).expect("spec");
        assert!(unsafe { bool::try_from_mrb(&interp, result) }.expect("convert"));
    }

    #[test]
    fn thread_spawn() {
        let interp = Interpreter::create().expect("mrb init");
        let spec = "Thread.new { Thread.current }.join.value != Thread.current";
        let result = interp.eval(spec).expect("spec");
        assert!(unsafe { bool::try_from_mrb(&interp, result) }.expect("convert"));
        let spec = "Thread.new { Thread.current.name }.join.value != Thread.current.name";
        let result = interp.eval(spec).expect("spec");
        assert!(unsafe { bool::try_from_mrb(&interp, result) }.expect("convert"));
        let spec = "Thread.new { Thread.current }.join.value.alive? == false";
        let result = interp.eval(spec).expect("spec");
        assert!(unsafe { bool::try_from_mrb(&interp, result) }.expect("convert"));
        let spec = "Thread.new { Thread.current }.join.value.status == false";
        let result = interp.eval(spec).expect("spec");
        assert!(unsafe { bool::try_from_mrb(&interp, result) }.expect("convert"));
    }

    #[test]
    fn thread_locals() {
        let interp = Interpreter::create().expect("mrb init");
        let spec = r#"
Thread.current[:local] = 42
Thread.new { Thread.current.keys.empty? }.join.value
"#;
        let result = interp.eval(spec).expect("spec");
        assert!(unsafe { bool::try_from_mrb(&interp, result) }.expect("convert"));
        let spec = r#"
Thread.current[:local] = 42
Thread.new { Thread.current[:local] = 96 }.join
Thread.current[:local] == 42
"#;
        let result = interp.eval(spec).expect("spec");
        assert!(unsafe { bool::try_from_mrb(&interp, result) }.expect("convert"));
        let spec = r#"
Thread.current.thread_variable_set(:local, 42)
Thread.new { Thread.current.thread_variables.empty? }.join.value
"#;
        let result = interp.eval(spec).expect("spec");
        assert!(unsafe { bool::try_from_mrb(&interp, result) }.expect("convert"));
        let spec = r#"
Thread.current.thread_variable_set(:local, 42)
Thread.new { Thread.current.thread_variable_set(:local, 96) }.join
Thread.current.thread_variable_get(:local) == 42
"#;
        let result = interp.eval(spec).expect("spec");
        assert!(unsafe { bool::try_from_mrb(&interp, result) }.expect("convert"));
    }

    #[test]
    fn thread_abort_on_exception() {
        let interp = Interpreter::create().expect("mrb init");
        let spec = r#"
Thread.abort_on_exception = true
Thread.new { raise 'failboat' }.join
"#;
        let result = interp.eval(spec);
        assert!(result.is_err());
        let spec = r#"
Thread.abort_on_exception = true
Thread.new do
  begin
    Thread.new { raise 'failboat' }.join
  rescue StandardError
    # swallow errors
  end
end.join
"#;
        let result = interp.eval(spec);
        assert!(result.is_err());
        let spec = r#"
Thread.abort_on_exception = false
Thread.new do
  begin
    Thread.new do
      Thread.current.abort_on_exception = true
      raise 'failboat'
    end.join
  rescue StandardError
    # swallow errors
  end
end.join
"#;
        let result = interp.eval(spec);
        assert!(result.is_err());
        let spec = r#"
Thread.abort_on_exception = false
Thread.new do
  begin
    Thread.new do
      begin
        Thread.new do
          Thread.current.abort_on_exception = true
          raise 'inner'
        end.join
        raise 'outer'
      rescue StandardError
        # swallow errors
      end
    end.join
    raise 'failboat'
  rescue StandardError
    # swallow errors
  end
end.join
"#;
        let result = interp.eval(spec.trim()).map(|_| ());
        let expected_backtrace = r#"
(eval):8: inner (RuntimeError)
(eval):8:in call
/src/lib/thread.rb:118:in initialize
(eval):6:in call
/src/lib/thread.rb:118:in initialize
(eval):4:in call
/src/lib/thread.rb:118:in initialize
(eval):2
"#;
        assert_eq!(
            result,
            Err(MrbError::Exec(expected_backtrace.trim().to_owned()))
        );
    }
}
