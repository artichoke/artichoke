# frozen_string_literal: true

def spec
  thread_required_by_default?
  current_thread_main?
  thread_join_value
  thread_main_running?
  thread_spawn
  thread_locals
  thread_abort_on_exception

  true
end

def thread_required_by_default?
  raise unless Object.const_defined?(:Thread)
end

def current_thread_main?
  raise unless Thread.current == Thread.main
  raise unless Thread.new { Thread.current == Thread.main }.join.value == false
end

def thread_join_value
  raise unless Thread.new { 2 + 3 }.join.value == 5
  raise unless Thread.new { Thread.new { 2 }.join.value + 3 }.join.value == 5
end

def thread_main_running?
  raise unless Thread.current.status == 'run'
  raise unless Thread.current.alive?
end

def thread_spawn
  raise if Thread.new { Thread.current }.join.value == Thread.current
  raise if Thread.new { Thread.current.name }.join.value == Thread.current.name
  raise if Thread.new { Thread.current }.join.value.alive?
  raise if Thread.new { Thread.current }.join.value.status
end

def thread_locals
  Thread.current[:local] = 42
  raise unless Thread.new { Thread.current.keys.empty? }.join.value

  Thread.current[:local] = 42
  Thread.new { Thread.current[:local] = 96 }.join
  raise unless Thread.current[:local] == 42

  Thread.current.thread_variable_set(:local, 42)
  raise unless Thread.new { Thread.current.thread_variables.empty? }.join.value

  Thread.current.thread_variable_set(:local, 42)
  Thread.new { Thread.current.thread_variable_set(:local, 96) }.join
  raise unless Thread.current.thread_variable_get(:local) == 42
end

def thread_abort_on_exception
  Thread.abort_on_exception = true
  raised = false
  begin
    Thread.new { raise 'failboat' }.join
  rescue StandardError
    raised = true
  ensure
    raise unless raised
  end

  Thread.abort_on_exception = true
  raised = false
  begin
    Thread.new do
      Thread.new { raise 'failboat' }.join
    rescue StandardError
      # swallow errors
      nil
    end.join
  rescue StandardError
    raised = true
  ensure
    raise unless raised
  end

  Thread.abort_on_exception = false
  raised = false
  begin
    Thread.new do
      Thread.new do
        Thread.current.abort_on_exception = true
        raise 'failboat'
      end.join
    rescue StandardError
      # swallow errors
      nil
    end.join
  rescue StandardError
    raised = true
  ensure
    raise unless raised
  end

  Thread.abort_on_exception = false
  raised = false
  begin
    Thread.new do
      Thread.new do
        Thread.new do
          Thread.current.abort_on_exception = true
          raise 'inner'
        end.join
        raise 'outer'
      rescue StandardError
        # swallow errors
        nil
      end.join
      raise 'failboat'
    rescue StandardError
      # swallow errors
      nil
    end.join
  rescue StandardError => e
    raised = true
    raise unless e.message == 'inner'
  ensure
    raise unless raised
  end
end

spec if $PROGRAM_NAME == __FILE__
