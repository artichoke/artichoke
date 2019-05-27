# frozen_string_literal: true

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
    return const_missing_without_autoload(const) if path.nil?

    require path
    if const_defined?(const)
      const_get(const)
    else
      const_missing_without_autoload(const)
    end
  end

  alias const_missing_without_autoload const_missing
  alias const_missing const_missing_with_autoload
end
