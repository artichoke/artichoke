# frozen_string_literal: true

class Thread
  def self.current
    @current ||= {}
  end
end

class Mutex
  # mruby interpreters are single threaded and are not `Send`. A `Mutex` can
  # never be contended, so synchronize just immediately yields.
  def synchronize
    yield
  end
end
