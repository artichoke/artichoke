# frozen_string_literal: true

class Integer
  include Comparable

  def ceil
    self
  end

  def floor
    self
  end

  def dup
    self
  end

  def to_int
    self
  end

  alias round floor
  alias truncate floor
end
