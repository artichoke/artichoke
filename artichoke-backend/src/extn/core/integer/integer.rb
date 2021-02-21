# frozen_string_literal: true

class Integer
  include Comparable
  # mruby hack to get Integer#<=>
  include Integral

  def ceil
    self
  end

  def floor
    self
  end

  alias round floor
  alias truncate floor
end
