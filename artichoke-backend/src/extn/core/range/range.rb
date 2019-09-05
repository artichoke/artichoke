# frozen_string_literal: true

class Range
  def cover?(*args)
    # Failed tests:
    # Range#cover? compares values using <=>
    raise ArgumentError, "wrong number of arguments (given #{args.length}, expected 1)" unless args.length == 1

    # when range is empty
    return false if exclude_end? && (self.begin <=> self.end) == 0

    val = args[0]

    # TODO: optimize with memoization
    if val.is_a?(Range)
      val_begin = val.begin
      val_end = val.end

      cmp_begin = self.begin <=> val_begin
      # when not same type
      # val_begin <=> self.begin would break "returns false if types are not comparable" test
      return false if cmp_begin.nil?

      # begin check
      return false if cmp_begin > 0

      cmp_end = self.end <=> val_end

      if exclude_end? == val.exclude_end?
        return cmp_end >= 0
      elsif exclude_end?
        return cmp_end > 0
      elsif cmp_end >= 0
        return true
      end

      val_max = val.max rescue nil
      return false if val_max.nil?

      return (self.end <=> val_max) >= 0
    end

    # there is a difference between val <=> self.begin and self.begin <=> val
    cmp_begin = val <=> self.begin

    # comparison of different types return false
    return false if cmp_begin.nil?

    # begin check
    return false if cmp_begin < 0

    # self.max raise TypeError for non-Integer when exclude_end?
    max = self.max rescue nil
    cmp_max = val <=> max
    cmp_end = val <=> self.end

    if exclude_end?
      return cmp_end < 0
    elsif cmp_end <= 0
      return true
    end

    return cmp_max <= 0

    false
  end

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
