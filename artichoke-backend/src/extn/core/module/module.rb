# frozen_string_literal: true

class Module
  def <(other)
    if equal?(other)
      false
    else
      self <= other
    end
  end

  def <=(other)
    raise TypeError, 'compared with non class/module' unless other.is_a?(Module)

    return true if ancestors.include?(other)
    return false if other.ancestors.include?(self)

    nil
  end

  def >(other)
    if equal?(other)
      false
    else
      self >= other
    end
  end

  def >=(other)
    raise TypeError, 'compared with non class/module' unless other.is_a?(Module)

    other < self
  end

  def <=>(other)
    return 0 if equal?(other)
    return nil unless other.is_a?(Module)

    cmp = self < other
    return -1 if cmp
    return 1 unless cmp.nil?

    nil
  end

  def attr_accessor(*names)
    attr_reader(*names)
    attr_writer(*names)
  end

  alias attr attr_reader

  def include(*args)
    args.reverse!
    args.each do |m|
      m.append_features(self)
      m.included(self)
    end
    self
  end

  def prepend(*args)
    args.reverse!
    args.each do |m|
      m.prepend_features(self)
      m.prepended(self)
    end
    self
  end

  alias attr attr_reader
end

def self.autoload(const, path); end

def self.autoload?(const); end

def self.include(*modules)
  self.class.include(*modules)
end

def self.private(*methods); end

def self.protected(*methods); end

def self.public(*methods); end
