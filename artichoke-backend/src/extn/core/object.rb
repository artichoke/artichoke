# frozen_string_literal: true

class BasicObject
  # rubocop:disable Style/RedundantConditional
  # rubocop:disable Style/IfWithBooleanLiteralBranches
  def !=(other)
    if self == other
      false
    else
      true
    end
  end
  # rubocop:enable Style/IfWithBooleanLiteralBranches
  # rubocop:enable Style/RedundantConditional
end

class NilClass
  def <=>(other)
    return 0 if other.instance_of?(NilClass)

    nil
  end

  def dup
    self
  end

  def to_a
    []
  end

  def to_f
    0.0
  end

  def to_h
    {}
  end

  def to_i
    0
  end
end

class TrueClass
  def <=>(other)
    return nil unless other.equal?(true) || other.equal?(false)
    return 0 if self == other

    1
  end

  def dup
    self
  end
end

class FalseClass
  def <=>(other)
    return nil unless other.equal?(true) || other.equal?(false)
    return 0 if self == other

    -1
  end

  def dup
    self
  end
end

class Integer
  def dup
    self
  end

  def to_int
    self
  end
end

class Symbol
  def dup
    self
  end
end
