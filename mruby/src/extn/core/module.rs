use crate::eval::MrbEval;
use crate::interpreter::Mrb;
use crate::MrbError;

const AUTOLOAD_PATCH: &str = r#"
class Module
  def autoload(const, path)
    @__autoload_registry ||= {}
    @__autoload_registry[const] = path
  end

  def autoload?(const)
    @__autoload_registry[const]
  end

  def const_missing_with_autoload(const)
    @__autoload_registry ||= {}
    path = @__autoload_registry[const]
    if path.nil?
      return const_missing_without_autoload(const)
    end
    require path
    if self.const_defined?(const)
      self.const_get(const)
    else
      raise #{const} not defined in rust autoloader"
      const_missing_without_autoload(const)
    end
  end

  alias_method :const_missing_without_autoload, :const_missing
  alias_method :const_missing, :const_missing_with_autoload
end
"#;

pub fn patch(interp: &Mrb) -> Result<(), MrbError> {
    interp.borrow_mut().def_module::<Module>("Module", None);
    interp.eval(AUTOLOAD_PATCH)?;
    Ok(())
}

pub struct Module;
