# frozen_string_literal: true

# Single-threaded implementation of Thread and Mutex.
#
# Threads are executed synchronously and support "spawning" new threads that
# get pushed onto the stack and executed synchronously.
#
# A special "root" thread that never terminates is created on require.
#
# Mutex immediately acquires locks and yields since there is no possibility of
# contention in a single-threaded environement.

class Thread
  attr_accessor :abort_on_exception
  attr_accessor :name
  attr_accessor :report_on_exception

  def self.current
    @@current.last
  end

  # To simulate concurrent execution, Thread maintains a stack of Threads.
  @@current = [] # rubocop:disable Style/ClassVars

  def initialize(root = false)
    @priority = 0
    @priority = self.class.current.priority unless self.class.current.nil?

    @@current.push(self)
    @fiber_locals = {}
    @thread_locals = {}
    @abort_on_exception = false
    @name = "thread-#{@@current.length}"
    @report_on_exception = false
    @terminated_with_exception = nil
    # mruby is not multi-threaded. Threads are executed synchronously.
    @alive = true
    @value = yield if block_given?
  rescue StandardError => e
    @terminated_with_exception = true
    @value = e
  ensure
    @alive = false unless root
    @@current.pop unless root
  end

  def [](sym)
    @fiber_locals[sym]
  end

  alias thr []

  def []=(sym, obj)
    @fiber_locals[sym] = obj
  end

  alias thr= []=

  def add_trace_func(func)
    # TODO: not implemented
  end

  def alive?
    @alive
  end

  def backtrace
    # TODO: implement this in Rust using the C API
    []
  end

  def backtrace_locations(*_args)
    # TODO: not implemented
    nil
  end

  def exit
    # Because mruby Thread instances are run synchronously on initialize, this
    # method is a noop.
    nil
  end

  alias kill exit
  alias terminate exit

  def fetch(*args)
    Kernel.raise ArgumentError, 'Thread#fetch requires 1 or 2 arguments' unless [1, 2].include?(args.length)

    key = args[0]
    if @fiber_locals.key?(key)
      @fiber_locals[key]
    elsif block_given?
      # block supersedes default argument
      yield key
    elsif args.length == 2
      args[1]
    else
      Kernel.raise "key '#{key}' not found in Thread locals"
    end
  end

  def group
    nil
  end

  alias inspect to_s

  def join(*_args)
    # Because mruby Thread instances are run synchronously on initialize, this
    # method is a noop.
    self
  end

  def key?(sym)
    @fiber_locals.key?(sym)
  end

  def keys
    @fiber_locals.keys.map(&:to_sym)
  end

  def pending_interrupt?(_error = nil)
    false
  end

  attr_reader :priority

  def priority=(pri)
    @priority = pri
    self # rubocop:disable Lint/Void
  end

  def raise(*args)
    exc, message, array = *args
    case args.length
    when 0 then Kernel.raise
    when 1 then Kernel.raise(exc)
    when 2 then Kernel.raise(exc, message)
    when 3 then Kernel.raise(exc, message, array)
    else Kernel.raise ArgumentError
    end
  end

  def run(*_args)
    # Because mruby Thread instances are run synchronously on initialize, this
    # method is a noop.
    self
  end

  def safe_level
    # TODO: not implemented
    0
  end

  def set_trace_fun(func) # rubocop:disable Naming/AccessorMethodName
    # TODO: not implemented
  end

  def status
    if alive?
      'run'
    elsif @terminated_with_exception
      nil
    else
      false
    end
  end

  def stop?
    !alive?
  end

  def thread_variable?(key)
    @thread_locals.key?(key)
  end

  def thread_variable_get(key)
    @thread_locals[key]
  end

  def thread_variable_set(key, value)
    @thread_locals[key] = value
    nil
  end

  def thread_variables
    @thread_locals.keys.map(&:to_sym)
  end

  def to_s
    "#<Thread:#{name}:#{object_id} #{status}>"
  end

  def value
    Kernel.raise @value if @terminated_with_exception

    @value
  end

  def wakeup
    # Because mruby Thread instances are run synchronously on initialize, this
    # method is a noop.
    self
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

# Spawn the special "root" thread that never terminates.
Thread.new(true)
