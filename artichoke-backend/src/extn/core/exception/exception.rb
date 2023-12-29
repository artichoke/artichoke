# frozen_string_literal: true

class Exception
  ##
  # call-seq:
  #   exception.message   ->  string
  #
  # Returns the result of invoking <code>exception.to_s</code>.
  # Normally this returns the exception's message or name.
  #
  def message
    to_s
  end
end

class KeyError
  attr_reader :key, :receiver

  def initialize(message = nil, receiver: nil, key: nil)
    @receiver = receiver
    @key = key
    super(message)
  end
end

class NameError
  attr_reader :name

  def initialize(message = nil, name = nil)
    @name = name
    super(message)
  end
end

class NoMethodError
  attr_reader :args

  def initialize(message = nil, name = nil, args = nil)
    @args = args
    super(message, name)
  end
end

class StopIteration
  attr_accessor :result
end

# Exception raised by a throw
class UncaughtThrowError
  # @!attribute [r] tag
  #   @return [Symbol] tag object, mostly a Symbol
  #
  # @!attribute [r] value
  #   @return [Array] extra parameters passed in
  attr_reader :tag, :value

  # @param [Symbol] tag  object to throw
  # @param [Object] value  any object to return to the catch block
  def initialize(tag, value = nil)
    @tag = tag
    @value = value
    super("uncaught throw #{tag}")
  end
end
