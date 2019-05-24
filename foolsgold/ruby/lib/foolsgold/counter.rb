# frozen_string_literal: true

module FoolsGold
  class Counter
    def initialize
      @counter = 0
      @mutex = Mutex.new
    end

    def get
      @mutex.synchronize do
        @counter
      end
    end

    def inc(by = 1)
      @mutex.synchronize do
        @counter += by
      end
      nil
    end
  end
end
