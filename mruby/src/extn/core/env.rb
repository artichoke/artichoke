# frozen_string_literal: true

ENV = Object.new

class << ENV
  def [](_name)
    nil
  end
end
