# frozen_string_literal: true

class Numeric
  include Comparable

  def +@
    self
  end

  def -@
    0 - self
  end

  def abs
    if negative?
      -self
    else
      self
    end
  end

  def downto(num, &block)
    return to_enum(:downto, num) unless block

    i = self
    while i >= num
      block.call(i)
      i -= 1
    end
    self
  end

  def negative?
    self < 0 # rubocop:disable Style/NumericPredicate
  end

  def next
    self + 1
  end

  def nonzero?
    if self == 0 # rubocop:disable Style/NumericPredicate
      nil
    else
      self
    end
  end

  def positive?
    self > 0 # rubocop:disable Style/NumericPredicate
  end

  def times &block
    return to_enum :times unless block

    i = 0
    while i < self
      block.call i
      i += 1
    end
    self
  end

  def upto(num, &block)
    return to_enum(:upto, num) unless block

    i = self
    while i <= num
      block.call(i)
      i += 1
    end
    self
  end

  def zero?
    self == 0 # rubocop:disable Style/NumericPredicate
  end

  alias succ next
end
