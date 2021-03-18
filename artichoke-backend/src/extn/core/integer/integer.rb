# frozen_string_literal: true

class Integer
  include Comparable

  def ceil
    self
  end

  def floor
    self
  end

  alias round floor
  alias truncate floor
end
