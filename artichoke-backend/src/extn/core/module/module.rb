# frozen_string_literal: true

class Module
  def attr_accessor(*names)
    attr_reader(*names)
    attr_writer(*names)
  end

  def include(*args)
    args.reverse.each do |m|
      m.append_features(self)
      m.included(self)
    end
    self
  end

  def prepend(*args)
    args.reverse.each do |m|
      m.prepend_features(self)
      m.prepended(self)
    end
    self
  end

  class Autoloaders
    class << self
      attr_accessor :top_self
    end

    def self.all
      @all ||= {}
    end

    def self.[](obj)
      all[obj]
    end

    def self.register(obj)
      all[obj] = Autoloader.new(obj) unless all.key?(obj)
      all[obj]
    end
  end

  class Autoloader
    attr_reader :lookup

    def initialize(const_lookup_base)
      @registry = {}
      @lookup = const_lookup_base
      @lookup = Object if const_lookup_base == Module::Autoloaders.top_self
    end

    def autoload(const, path)
      @registry[const] = path
      nil
    end

    def autoload?(const)
      @registry[const]
    end
  end

  def autoload(const, path)
    Module::Autoloaders.register(self).autoload(const, path)
  end

  def autoload?(const)
    Module::Autoloaders.register(self).autoload?(const)
  end

  def const_missing_with_autoload(const)
    autoloader = Module::Autoloaders.register(self)
    path = autoloader.autoload?(const)
    if path.nil? && !Module::Autoloaders.top_self.nil?
      autoloader = Module::Autoloaders.register(Module::Autoloaders.top_self)
      path = autoloader.autoload?(const)
    end
    return const_missing_without_autoload(const) if path.nil?

    require path
    if autoloader.lookup.const_defined?(const)
      autoloader.lookup.const_get(const)
    else
      const_missing_without_autoload(const)
    end
  end

  alias attr attr_reader
  # alias const_missing_without_autoload const_missing
  # alias const_missing const_missing_with_autoload
end

def self.autoload(const, path)
  Module::Autoloaders.top_self = self
  Module::Autoloaders.register(self).autoload(const, path)
end

def self.autoload?(const)
  Module::Autoloaders.top_self = self
  Module::Autoloaders.register(self).autoload?(const)
end
