# frozen_string_literal: true

class Range
  def first(*args)
    return self.begin if args.empty?

    raise ArgumentError, "wrong number of arguments (given #{args.length}, expected 1)" unless args.length == 1

    n = args[0].to_i
    raise ArgumentError, 'negative array size (or size too big)' unless n >= 0

    ary = []
    each do |i|
      break if n <= 0

      ary.push(i)
      n -= 1
    end
    ary
  end

  def last(*args)
    return self.end if args.empty?

    raise ArgumentError, "wrong number of arguments (given #{args.length}, expected 1)" unless args.length == 1

    arg = args[0]
    raise TypeError, "no implicit conversion of #{arg.class} into Integer" unless arg.respond_to?(:to_int)

    n = arg.to_int
    raise TypeError, "can't convert #{arg.class} to Integer (#{arg.class}#to_int gives #{n.class})" unless n.is_a?(Integer)

    array = to_a
    array.last(n.to_int)
  end

  def max(&block)
    val = first
    last = self.last
    return super if block

    # fast path for numerics
    if val.is_a?(Numeric) && last.is_a?(Numeric)
      raise TypeError if exclude_end? && !last.is_a?(Integer)
      return nil if val > last
      return nil if val == last && exclude_end?

      max = last
      max -= 1 if exclude_end?
      return max
    end

    # delegate to Enumerable
    super
  end

  def min(&block)
    val = first
    last = self.last
    return super if block

    # fast path for numerics
    if val.is_a?(Numeric) && last.is_a?(Numeric)
      return nil if val > last
      return nil if val == last && exclude_end?

      min = val
      return min
    end

    # delegate to Enumerable
    super
  end
end
