# frozen_string_literal: true

class Random
  def self.bytes(size)
    DEFAULT.bytes(size)
  end

  def self.rand(max = (not_set = true))
    if not_set
      DEFAULT.rand
    else
      DEFAULT.rand(max)
    end
  end
end
