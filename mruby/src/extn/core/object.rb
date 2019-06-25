# frozen_string_literal: true

class NilClass
  def dup
    self
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

class Integer
  def dup
    self
  end
end
