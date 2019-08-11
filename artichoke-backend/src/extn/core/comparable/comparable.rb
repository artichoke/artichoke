# frozen_string_literal: true

module Comparable
  def <(other)
    return true if (self <=> other).negative?

    false
  end

  def <=(other)
    return false if (self <=> other).positive?

    true
  end

  def ==(other)
    return true if (self <=> other).zero?

    false
  end

  def >(other)
    return true if (self <=> other).positive?

    false
  end

  def >=(other)
    return false if (self <=> other).negative?

    true
  end

  def between?(min, max)
    return false if (min <=> max).positive?

    c = self <=> min
    return false if c.negative?

    c = self <=> max
    return false if c.positive?

    true
  end

  def clamp(min, max)
    raise ArgumentError, 'min argument must be smaller than max argument' if (min <=> max).positive?

    c = self <=> min
    return self if c.zero?
    return min if c.negative?

    c = self <=> max
    return max if c.positive?

    self
  end
end
