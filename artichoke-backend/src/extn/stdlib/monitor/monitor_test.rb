# frozen_string_literal: true

require 'monitor'

def spec
  monitor_mixin

  true
end

def monitor_mixin
  cls = Class.new do
    include MonitorMixin

    def initialize(*array)
      mon_initialize
      @array = array
    end

    def to_a
      synchronize { @array.dup }
    end

    def initialize_copy(other)
      mon_initialize

      synchronize do
        @array = other.to_a
      end
    end
  end

  instance = cls.new(1, 2, 3)
  copy = instance.dup
  raise if copy == instance
end

spec if $PROGRAM_NAME == __FILE__
