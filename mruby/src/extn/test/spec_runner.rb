# frozen_string_literal: true

class StubIO
  def method_missing(method, *args, &block)
    super
  rescue NoMethodError
    nil
  end

  def respond_to_missing?(method, include_private = false)
    true || super
  end
end

STDOUT = StubIO.new
STDERR = StubIO.new
RUBY_EXE = '/usr/bin/true'

require 'mspec'
require 'mspec/utils/script'

class ErrorCollector
  attr_reader :errors

  def initialize
    @errors = []
  end

  def exception(state)
    if state.exception.is_a?(NoMethodError)
      return if state.message =~ /'private_instance_methods'/
      return if state.message =~ /'taint'/
      return if state.message =~ /'tainted\?'/
      return if state.message =~ /'warn'/
    end
    @errors << state
  end
end

def run_specs(*specs)
  specs = specs.flatten
  error_collector = ErrorCollector.new
  MSpec.register_files(specs)
  MSpec.register(:exception, error_collector)
  MSpecScript.set(:backtrace_filter, %r{/lib/mspec/})

  MSpec.process

  return true if error_collector.errors.length.zero?

  message = "\n\n\e[31m#{error_collector.errors.length} spec failures:\e[0m"
  error_collector.errors.each do |state|
    message << "\n\n\e[31m#{state.message} in #{state.it}\e[0m\n\n"
    message << state.backtrace
  end
  message << "\n\n\e[31m#{error_collector.errors.length} spec failures.\e[0m\n"

  raise message
end
