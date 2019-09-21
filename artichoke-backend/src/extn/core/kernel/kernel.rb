# frozen_string_literal: true

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

    raise TypeError, "can't convert #{classname} to Array (#{classname}#to_ary gives #{ret.class})" unless ret.nil? || ret.is_a?(Array)

    ret = arg.to_a if ret.nil? && arg.respond_to?(:to_a)

    raise TypeError, "can't convert #{classname} to Array (#{classname}#to_a gives #{ret.class})" unless ret.nil? || ret.is_a?(Array)

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

  def Integer(arg, base = 0) # rubocop:disable Naming/MethodName
    raise ArgumentError, 'base specified for non string value' if base.positive? && arg.is_a?(Numeric)

    classname = arg.class
    classname = arg.inspect if arg.nil? || arg.equal?(false) || arg.equal?(true)

    if arg&.respond_to?(:to_int)
      ret = arg.to_int

      # uncritically return the value of to_int even if it is not an Integer
      return ret if ret.is_a?(Numeric)
      return ret if ret&.scan(/\D/)&.empty?

      ret = arg.to_i

      return ret if ret.is_a?(Numeric)

      raise TypeError, "can't convert #{classname} to Integer (#{arg.class}#to_i gives #{ret.class})"
    elsif !arg.is_a?(String) && arg && arg.respond_to?(:to_i)
      ret = arg.to_i

      return ret if ret.is_a?(Numeric)

      raise TypeError, "can't convert #{classname} to Integer (#{arg.class}#to_i gives #{ret.class})"
    elsif arg.is_a?(String)
      i = 0

      # squeeze preceding spaces
      i += 1 while arg[i] == ' '

      # handle sign
      sign = 1
      if arg[i] == '+'
        i += 1
      elsif arg[i] == '-'
        i += 1
        sign = -1
      end

      # no multiple leading +/-, i.e. "++1", "---1"
      # no space between -/+ and next digit, i.e. "+ 1"
      # no leading _
      # TODO: no leading null byte \0
      raise ArgumentError, "invalid value for Integer(): \"#{arg}\"" if arg[i] == '+' || arg[i] == '-' || arg[i] == ' ' || arg[i] == '_'

      # no multiple embedded _
      # TODO: no null bytel \0
      j = i
      while j < arg.length
        raise ArgumentError, "invalid value for Integer(): \"#{arg}\"" if arg[j] == '_' && arg[j + 1] == '_'

        j += 1
      end

      # no trailing -, +, i.e. "1-", "1++"
      # no trailing _
      # TODO: no trailing null byte \0
      k = arg.length - 1
      # squeeze trailing spaces
      k += 1 while arg[k] == ' '
      raise ArgumentError, "invalid value for Integer(): \"#{arg}\"" if arg[k] == '-' || arg[k] == '+' || arg[k] == '_'

      # when base is not specified or zero, check the base letter
      if base <= 0
        if arg[i] == '0'
          # TODO: if start with 1 zero and not follow by b,o,d,x, then it's octal
          case arg[i + 1]
          when 'b', 'B'
            base = 2
          when 'o', 'O'
            base = 8
          when 'd', 'D'
            base = 10
          when 'x', 'X'
            base = 16
          else
            base = 8
          end
        elsif base < -1
          base = -base
        else
          base = 10
        end
      end

      if arg[i] == '0'
        invalid_value = false
        case arg[i + 1]
        when 'b', 'B'
          return 0 if i + 2 >= arg.length

          invalid_value = true if base != 2
          (i + 2...arg.length).each do |x|
            c = arg[x]
            unless %w[0 1].include?(c)
              invalid_value = true
              break
            end
          end
        when 'o', 'O'
          return 0 if i + 2 >= arg.length

          invalid_value = true if base != 8
          (i + 2...arg.length).each do |x|
            c = arg[x]
            if c < '0' || c > '7'
              invalid_value = true
              break
            end
          end
        when 'd', 'D'
          return 0 if i + 2 >= arg.length

          invalid_value = true if base != 10
          (i + 2...arg.length).each do |x|
            c = arg[x]
            unless c >= '0' && c <= '9'
              invalid_value = true
              break
            end
          end
        when 'x', 'X'
          return 0 if i + 2 >= arg.length

          invalid_value = true if base != 16
          (i + 2...arg.length).each do |x|
            c = arg[x]
            unless (c >= '0' && c <= '9') || (c >= 'a' && c <= 'f') || (c >= 'A' && c <= 'F')
              invalid_value = true
              break
            end
          end
        else
          return 0 if i + 1 >= arg.length

          (i + 1...arg.length).each do |x|
            c = arg[x]
            if c < '0' || c > '7'
              invalid_value = true
              break
            end
          end
        end
        raise ArgumentError, "invalid value for Integer(): \"#{arg}\"" if invalid_value
      end

      case base
      when 2
        i += 2 if arg[i] == '0' && (arg[i + 1] == 'b' || arg[i + 1] == 'B')
      when 8
        i += 2 if arg[i] == '0' && (arg[i + 1] == 'o' || arg[i + 1] == 'O')
      when 10
        i += 2 if arg[i] == '0' && (arg[i + 1] == 'd' || arg[i + 1] == 'D')
      when 16
        i += 2 if arg[i] == '0' && (arg[i + 1] == 'x' || arg[i + 1] == 'X')
      else
        raise ArgumentError, "invalid radix #{base}" if base < 2 || base > 36
      end

      # raise when empty string
      raise ArgumentError, "invalid value for Integer(): \"#{arg}\"" if i >= arg.length

      # no digits out of range of radix
      char_to_digit_mapping = '0123456789abcdefghijklmnopqrstuvwxyz'
      (i...arg.length).each do |x|
        ch = arg[x].downcase
        digit = char_to_digit_mapping.index(ch)
        raise ArgumentError, "invalid value for Integer(): \"#{arg}\"" if digit.nil? || digit > base
      end

      return sign * arg[i..-1].to_i(base)
    end

    raise TypeError, "can't convert #{classname} to Integer"
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
end
