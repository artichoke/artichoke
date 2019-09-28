# frozen_string_literal: true

class ArgumentError < StandardError; end

class ScriptError < Exception; end # rubocop:disable Lint/InheritException

class LocalJumpError < ScriptError; end # rubocop:disable Lint/InheritException

class RangeError < StandardError; end

class FloatDomainError < RangeError; end

class RegexpError < StandardError; end

class TypeError < StandardError; end

class NameError < StandardError
  attr_accessor :name

  def initialize(message = nil, name = nil)
    @name = name
    super(message)
  end
end

class NoMethodError < NameError
  attr_reader :args

  def initialize(message = nil, name = nil, args = nil)
    @args = args
    super message, name
  end
end

class IndexError < StandardError; end

class KeyError < IndexError; end

class NotImplementedError < ScriptError; end # rubocop:disable Lint/InheritException

class FrozenError < RuntimeError; end

class StopIteration < IndexError
  attr_accessor :result
end

# Exception raised by a throw
class UncaughtThrowError < ArgumentError
  # @!attribute [r] tag
  #   @return [Symbol] tag object, mostly a Symbol
  attr_reader :tag
  # @!attribute [r] value
  #   @return [Array] extra parameters passed in
  attr_reader :value

  # @param [Symbol] tag  object to throw
  # @param [Object] value  any object to return to the catch block
  def initialize(tag, value = nil)
    @tag = tag
    @value = value
    super "uncaught throw #{tag}"
  end
end
