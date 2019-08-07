# frozen_string_literal: true

class ENV
  attr_reader :_env
  def initialize()
    @_env = initialize_internal()
  end

  def [](name)
    '234'
  end
end
