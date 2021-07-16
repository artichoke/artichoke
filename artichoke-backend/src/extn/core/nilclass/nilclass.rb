# frozen_string_literal: true

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
