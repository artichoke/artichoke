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
end

def self.autoload(const, path); end

def self.autoload?(const); end
