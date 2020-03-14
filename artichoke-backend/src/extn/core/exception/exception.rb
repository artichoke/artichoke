# frozen_string_literal: true

class KeyError
  attr_reader :key
  attr_reader :receiver

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
    super message, name
  end
end

class StopIteration
  attr_accessor :result
end

# Exception raised by a throw
class UncaughtThrowError
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
