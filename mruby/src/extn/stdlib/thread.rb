# frozen_string_literal: true

class Thread
  def self.current
    @current ||= {}
  end
end

class ThreadError < StandardError; end

class Mutex
  def initialize
    @owner = nil
  end

  def lock
    raise ThreadError, 'locked by current Thread' if owned?

    @owner = Thread.current
    self
  end

  def locked?
    !@owner.nil?
  end

  def owned?
    @owner == Thread.current
  end

  # This method does not actually sleep
  def sleep(timeout = nil)
    unlock
    lock
    timeout
  end

  # mruby interpreters are single threaded and are not `Send`. A `Mutex` can
  # never be contended, so synchronize just immediately yields.
  def synchronize
    lock
    yield
  ensure
    unlock
  end

  def try_lock
    lock
    locked? && owned?
  end

  def unlock
    raise ThreadError, 'not locked by current Thread' unless owned?

    @owner = nil
    self
  end
end
