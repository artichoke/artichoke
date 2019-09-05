# frozen_string_literal: true

class Range
  def cover?(*args)
    raise ArgumentError, "wrong number of arguments (given #{args.length}, expected 1)" unless args.length == 1

    range_begin = self.begin
    range_end = self.end

    return false if exclude_end? && (range_begin <=> range_end).zero?

    val = args[0]

    if val.is_a?(Range)
      val_begin = val.begin
      val_end = val.end

      cmp_begin = range_begin <=> val_begin
      return false if cmp_begin.nil? || cmp_begin.positive?

      cmp_end = range_end <=> val_end
      return cmp_end >= 0 if exclude_end? == val.exclude_end?
      return cmp_end.positive? if exclude_end?
      return true if cmp_end >= 0

      val_max = begin
                  val.max
                rescue StandardError
                  nil
                end
      return false if val_max.nil?

      return (range_end <=> val_max) >= 0
    end

    cmp_begin = val <=> range_begin
    return false if cmp_begin.nil? || cmp_begin.negative?

    max = begin
            self.max
          rescue StandardError
            nil
          end
    cmp_max = val <=> max
    cmp_end = val <=> range_end

    return cmp_end.negative? if exclude_end?
    return true if cmp_end <= 0

    cmp_max <= 0
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
