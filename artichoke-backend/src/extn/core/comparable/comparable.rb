# frozen_string_literal: true

module Comparable
  def <(other)
    cmp = (self <=> other)

    if cmp.nil?
      classname = other.class
      if other.nil? || other.equal?(false) || other.equal?(true) || other.is_a?(Numeric)
        classname = other.inspect
      end
      raise ArgumentError, "Comparison of #{self.class} with #{classname} failed"
    end
    unless cmp.is_a?(Numeric)
      classname = other.class
      if other.nil? || other.equal?(false) || other.equal?(true) || other.is_a?(Numeric)
        classname = other.inspect
      end
      raise ArgumentError, "Comparison of #{self.class} with #{classname} failed"
    end

    return true if cmp.negative?

    false
  end

  def <=(other)
    cmp = (self <=> other)

    if cmp.nil?
      classname = other.class
      if other.nil? || other.equal?(false) || other.equal?(true) || other.is_a?(Numeric)
        classname = other.inspect
      end
      raise ArgumentError, "Comparison of #{self.class} with #{classname} failed"
    end
    unless cmp.is_a?(Numeric)
      classname = other.class
      if other.nil? || other.equal?(false) || other.equal?(true) || other.is_a?(Numeric)
        classname = other.inspect
      end
      raise ArgumentError, "Comparison of #{self.class} with #{classname} failed"
    end

    return false if cmp.positive?

    true
  end

  def ==(other)
    return true if equal?(other)
    return false unless respond_to?(:<=>)

    cmp = (self <=> other)
    return false if cmp.nil?

    unless cmp.is_a?(Numeric)
      classname = other.class
      if other.nil? || other.equal?(false) || other.equal?(true) || other.is_a?(Numeric)
        classname = other.inspect
      end
      raise ArgumentError, "Comparison of #{self.class} with #{classname} failed"
    end

    return true if cmp.zero?

    false
  rescue NoMethodError
    false
  end

  def >(other)
    cmp = (self <=> other)

    if cmp.nil?
      classname = other.class
      if other.nil? || other.equal?(false) || other.equal?(true) || other.is_a?(Numeric)
        classname = other.inspect
      end
      raise ArgumentError, "Comparison of #{self.class} with #{classname} failed"
    end
    unless cmp.is_a?(Numeric)
      classname = other.class
      if other.nil? || other.equal?(false) || other.equal?(true) || other.is_a?(Numeric)
        classname = other.inspect
      end
      raise ArgumentError, "Comparison of #{self.class} with #{classname} failed"
    end

    return true if cmp.positive?

    false
  end

  def >=(other)
    cmp = (self <=> other)

    if cmp.nil?
      classname = other.class
      if other.nil? || other.equal?(false) || other.equal?(true) || other.is_a?(Numeric)
        classname = other.inspect
      end
      raise ArgumentError, "Comparison of #{self.class} with #{classname} failed"
    end
    unless cmp.is_a?(Numeric)
      classname = other.class
      if other.nil? || other.equal?(false) || other.equal?(true) || other.is_a?(Numeric)
        classname = other.inspect
      end
      raise ArgumentError, "Comparison of #{self.class} with #{classname} failed"
    end

    return false if cmp.negative?

    true
  end

  def between?(min, max)
    cmp = (self <=> min)
    if cmp.nil?
      classname = other.class
      if other.nil? || other.equal?(false) || other.equal?(true) || other.is_a?(Numeric)
        classname = other.inspect
      end
      raise ArgumentError, "Comparison of #{self.class} with #{classname} failed"
    end
    unless cmp.is_a?(Numeric)
      classname = other.class
      if other.nil? || other.equal?(false) || other.equal?(true) || other.is_a?(Numeric)
        classname = other.inspect
      end
      raise ArgumentError, "Comparison of #{self.class} with #{classname} failed"
    end

    return false if cmp.negative?

    cmp = (self <=> max)
    if cmp.nil?
      classname = other.class
      if other.nil? || other.equal?(false) || other.equal?(true) || other.is_a?(Numeric)
        classname = other.inspect
      end
      raise ArgumentError, "Comparison of #{self.class} with #{classname} failed"
    end
    unless cmp.is_a?(Numeric)
      classname = other.class
      if other.nil? || other.equal?(false) || other.equal?(true) || other.is_a?(Numeric)
        classname = other.inspect
      end
      raise ArgumentError, "Comparison of #{self.class} with #{classname} failed"
    end

    return false if cmp.positive?

    true
  end

  def clamp(min, max)
    paramcmp = (min <=> max)
    if paramcmp.nil? || paramcmp > 0
      raise ArgumentError, 'min argument must be smaller than max argument'
    end

    cmp = (self <=> min)
    if cmp.nil?
      classname = other.class
      if other.nil? || other.equal?(false) || other.equal?(true) || other.is_a?(Numeric)
        classname = other.inspect
      end
      raise ArgumentError, "Comparison of #{self.class} with #{classname} failed"
    end
    unless cmp.is_a?(Numeric)
      classname = other.class
      if other.nil? || other.equal?(false) || other.equal?(true) || other.is_a?(Numeric)
        classname = other.inspect
      end
      raise ArgumentError, "Comparison of #{self.class} with #{classname} failed"
    end

    return min if cmp < 0 # rubocop:disable Style/NumericPredicate

    cmp = (self <=> max)
    if cmp.nil?
      classname = other.class
      if other.nil? || other.equal?(false) || other.equal?(true) || other.is_a?(Numeric)
        classname = other.inspect
      end
      raise ArgumentError, "Comparison of #{self.class} with #{classname} failed"
    end
    unless cmp.is_a?(Numeric)
      classname = other.class
      if other.nil? || other.equal?(false) || other.equal?(true) || other.is_a?(Numeric)
        classname = other.inspect
      end
      raise ArgumentError, "Comparison of #{self.class} with #{classname} failed"
    end

    return max if cmp > 0 # rubocop:disable Style/NumericPredicate

    self
  end
end
