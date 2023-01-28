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
  alias __raise__ raise

  attr_accessor :abort_on_exception, :name, :report_on_exception, :__unwind_with_exception

  def self.__mark_unwind(exc)
    @thread_stack.each do |thread|
      thread.__unwind_with_exception = exc
    end
    nil
  end

  def self.__gen_thread_name(id)
    "thread-#{@thread_stack.length}-#{id}"
  end

  @abort_on_exception = false
  class << self
    attr_accessor :abort_on_exception
  end

  # To simulate concurrent execution, Thread maintains a stack of Threads.
  @thread_stack = []

  def self.__push_stack(thread)
    @thread_stack.push(thread)
    nil
  end

  def self.__pop_stack
    @thread_stack.pop
  end

  def self.current
    @thread_stack.last
  end

  def self.exclusive
    yield
  end

  def self.exit
    # TODO: not implemented
  end

  def self.fork(*args, &blk)
    # TODO: handle subclassing behavior correctly.
    new(*args, blk)
  end

  def self.handle_interrupt(_hash)
    # https://ruby-doc.org/core-2.6.3/Thread.html#method-c-handle_interrupt
    # Implemented as an immediate yield because interrupts are not a thing in
    # the mruby interpreter. `Thread::exit` is not implemented.
    yield
  end

  def self.list
    # make sure to clone the list
    @thread_stack.map(&:itself)
  end

  def self.main
    @thread_stack.first
  end

  def self.pass
    # noop since there is no scheduler
    nil
  end

  def self.pending_interrupt?(_error = nil)
    # See `Thread::handle_interrupt`.
    false
  end

  @report_on_exception = false

  def self.stop
    # noop since there is no scheduler
    nil
  end

  class << self
    attr_accessor :report_on_exception

    alias kill exit
    alias start fork
  end

  def initialize(root: false, &blk)
    __raise__ ThreadError, 'must be called with a block' unless block_given?

    @priority = 0
    @priority = self.class.current.priority unless self.class.current.nil?

    self.class.__push_stack(self)
    @fiber_locals = {}
    @thread_locals = {}
    @abort_on_exception = false
    @name = self.class.__gen_thread_name(object_id)
    @report_on_exception = false
    @terminated_with_exception = nil
    # mruby is not multi-threaded. Threads are executed synchronously.
    @alive = true
    @value = blk.call
  rescue StandardError => e
    if @__unwind_with_exception.nil?
      @terminated_with_exception = true
      @value = e
      self.class.__mark_unwind(e) if self.class.abort_on_exception || abort_on_exception
    end
  ensure
    @alive = false unless root
    self.class.__pop_stack unless root
    __raise__ @__unwind_with_exception if self.class.current == self.class.main && !@__unwind_with_exception.nil?
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
    __raise__ ArgumentError, 'Thread#fetch requires 1 or 2 arguments' unless [1, 2].include?(args.length)

    key = args[0]
    if @fiber_locals.key?(key)
      @fiber_locals[key]
    elsif block_given?
      # block supersedes default argument
      yield key
    elsif args.length == 2
      args[1]
    else
      __raise__ "key '#{key}' not found in Thread locals"
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
    when 0 then __raise__
    when 1 then __raise__(exc)
    when 2 then __raise__(exc, message)
    when 3 then __raise__(exc, message, array)
    else __raise__ ArgumentError
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
    __raise__ @value if @terminated_with_exception

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

Thread::Mutex = Mutex

# Spawn the special "root" thread that never terminates.
# rubocop:disable Lint/EmptyBlock
Thread.new(root: true) {}
# rubocop:enable Lint/EmptyBlock
