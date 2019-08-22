# frozen_string_literal: true

class Numeric
  def zero?
    self == 0 # rubocop:disable Style/NumericPredicate
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

  def negative?
    self < 0 # rubocop:disable Style/NumericPredicate
  end
end
