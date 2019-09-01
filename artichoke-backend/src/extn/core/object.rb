# frozen_string_literal: true

class NilClass
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
  def dup
    self
  end
end

class FalseClass
  def dup
    self
  end
end

class Float
  def to_int
    floor
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
