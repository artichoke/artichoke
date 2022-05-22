# frozen_string_literal: true

class Range
  include Enumerable

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

  def each(&block)
    return to_enum :each unless block

    val = first
    last = self.last

    if val.is_a?(Integer) && last.is_a?(Integer) # fixnums are special
      lim = last
      lim += 1 unless exclude_end?
      i = val
      while i < lim
        block.call(i)
        i += 1
      end
      return self
    end

    if val.is_a?(String) && last.is_a?(String) # strings are special
      return val.upto(last, exclude_end?, &block) if val.respond_to? :upto

      str_each = true
    end

    raise TypeError, "can't iterate" unless val.respond_to? :succ

    return self if (val <=> last) > 0 # rubocop:disable Style/NumericPredicate

    while (val <=> last) < 0 # rubocop:disable Style/NumericPredicate
      block.call(val)
      val = val.succ
      next unless str_each
      break if val.size > last.size
    end

    block.call(val) if !exclude_end? && (val <=> last) == 0 # rubocop:disable Style/NumericPredicate
    self
  end

  def first(*args)
    return self.begin if args.empty?

    raise ArgumentError, "wrong number of arguments (given #{args.length}, expected 1)" unless args.length == 1

    arg = args[0]
    classname = arg.class
    classname = arg.inspect if arg.nil? || arg.equal?(false) || arg.equal?(true)
    raise TypeError, "no implicit conversion of #{classname} into Integer" unless arg.respond_to?(:to_int)

    n = arg.to_int
    unless n.is_a?(Integer)
      raise TypeError, "can't convert #{arg.class} to Integer (#{arg.class}#to_int gives #{n.class})"
    end
    raise ArgumentError, 'negative array size (or size too big)' if n.negative?

    each.take(n).to_a
  end

  def hash
    suffix = 0
    suffix = 1 if exclude_end?
    [first, last, suffix].hash
  end

  def last(*args)
    return self.end if args.empty?

    raise ArgumentError, "wrong number of arguments (given #{args.length}, expected 0..1)" if args.length > 1

    arg = args[0]
    classname = arg.class
    classname = arg.inspect if arg.nil? || arg.equal?(false) || arg.equal?(true)
    raise TypeError, "no implicit conversion of #{classname} into Integer" unless arg.respond_to?(:to_int)

    n = arg.to_int
    unless n.is_a?(Integer)
      raise TypeError, "can't convert #{arg.class} to Integer (#{arg.class}#to_int gives #{n.class})"
    end

    array = to_a
    array.last(n)
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

  def size
    range_begin = self.begin
    range_end = self.end

    return Float::INFINITY if range_end.nil?

    if range_begin.is_a?(Integer) && range_end.is_a?(Integer)
      delta = range_end - range_begin
      delta -= 1 if exclude_end?

      return 0 if delta.negative?

      delta + 1
    elsif range_begin.is_a?(Float) || range_end.is_a?(Float)
      epsilon = Float::EPSILON
      delta = range_end - range_begin

      return Float::INFINITY if range_end > range_begin && delta.abs.infinite?

      err = (range_begin.abs + range_end.abs + (range_end - range_begin).abs) * epsilon
      err = 0.5 if err > 0.5

      if exclude_end?
        return 0 if delta <= 0.0

        delta = if delta < 1.0
                  0
                else
                  (delta - err).floor
                end
      else
        return 0 if delta < 0.0

        delta = (delta + err).floor
      end

      delta + 1
    elsif range_begin.respond_to?(:succ) && range_end.respond_to?(:succ)
      # TODO: implement Range#size for object that responds to :succ
      raise NotImplementedError
    end
  end

  def to_a
    raise RangeError, 'cannot convert endless range to an array' if self.end.nil?

    super
  end
end
