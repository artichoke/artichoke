# frozen_string_literal: true

module Kernel
  # Setup a catch block and wait for an object to be thrown, the
  # catch end without catching anything.
  #
  # @param [Symbol] tag  tag to catch
  # @return [void]
  #
  # @example
  #   catch :thing do
  #     __do_stuff__
  #     throw :thing
  #   end
  def catch(tag = nil)
    tag = Object.new if tag.nil?

    yield tag
  rescue UncaughtThrowError => e
    raise e unless e.tag == tag

    e.value
  end

  def Array(arg) # rubocop:disable Naming/MethodName
    return arg if arg.is_a?(Array)

    ret = nil
    ret = arg.to_ary if arg.respond_to?(:to_ary)
    classname = arg.class

    unless ret.nil? || ret.is_a?(Array)
      raise TypeError, "can't convert #{classname} to Array (#{classname}#to_ary gives #{ret.class})"
    end

    ret = arg.to_a if ret.nil? && arg.respond_to?(:to_a)

    unless ret.nil? || ret.is_a?(Array)
      raise TypeError, "can't convert #{classname} to Array (#{classname}#to_a gives #{ret.class})"
    end

    ret.nil? ? [arg] : ret
  end

  def Hash(arg) # rubocop:disable Naming/MethodName
    return arg if arg.is_a?(Hash)
    return {} if arg.nil? || arg == []

    classname = arg.class

    if arg.respond_to?(:to_hash)
      ret = arg.to_hash

      return ret if ret.is_a?(Hash)

      raise TypeError, "can't convert #{classname} into Hash" if ret.nil?

      raise TypeError, "can't convert #{classname} to Hash (#{classname}#to_hash gives #{ret.class})"
    end

    raise TypeError, "can't convert #{classname} into Hash"
  end

  def Integer(arg, base = (not_set = true), exception: true) # rubocop:disable Naming/MethodName
    if not_set == true
      ::Artichoke::Kernel.Integer(arg)
    else
      ::Artichoke::Kernel.Integer(arg, base)
    end
  rescue StandardError => e
    return nil if exception.equal?(false)

    raise e
  end

  def String(arg) # rubocop:disable Naming/MethodName
    return arg if arg.is_a?(String)

    # TODO: after defined? keyword is implemented
    # if `to_s` is not defined, raise TypeError even if respond_to?(:to_s) return true
    if arg.respond_to?(:to_s)
      ret = arg.to_s
      return ret if ret.is_a?(String)
    end

    raise TypeError, "can't convert #{arg.class} into String"
  end

  def loop(&block)
    return to_enum :loop unless block

    # RuboCop's `Style/InfiniteLoop` lint says:
    #     Use `Kernel#loop` for infinite loops.
    # Disable this lint since we are _implementing_ `Kernel#loop`.
    #
    # rubocop:disable Style/InfiniteLoop
    yield while true
    # rubocop:enable Style/InfiniteLoop
  rescue StopIteration => e
    e.result
  end

  def rand(max = 0)
    if max == 0 # rubocop:disable Style/NumericPredicate
      Random.rand
    else
      Random.rand(max)
    end
  end

  def srand(number = Random.new_seed)
    Random.srand(number)
  end

  # 11.4.4 Step c)
  def !~(other)
    !(self =~ other) # rubocop:disable Style/InverseMethods
  end

  def singleton_method(name)
    m = method(name)
    sc = (class << self; self; end)
    raise NameError, "undefined method '#{name}' for class '#{sc}'" if m.owner != sc

    m
  end

  # Throws an object, uncaught throws will bubble up through other catch blocks.
  #
  # @param [Symbol] tag  tag being thrown
  # @param [Object] value  a value to return to the catch block
  # @raises [UncaughtThrowError]
  # @return [void] it will never return normally.
  #
  # @example
  #   catch :ball do
  #     pitcher.wind_up
  #     throw :ball
  #   end
  def throw(tag, value = nil)
    raise UncaughtThrowError.new(tag, value)
  end

  def warn(*msg)
    msg.each do |warning|
      warning = warning.to_s
      warning << "\n" unless warning[-1] == "\n"
      # TODO: This should call `Warning.warn` but due to method visibility
      # limitations of the mruby VM, we cannot shadow the warn method there.
      out = $stderr || $stdout || self
      out.print(warning)
    end
    nil
  end
end
