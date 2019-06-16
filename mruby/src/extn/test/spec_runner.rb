# frozen_string_literal: true

require 'mspec'

class ErrorCollector
  attr_reader :errors

  def initialize
    @errors = []
  end

  def exception(state)
    @errors << state
  end
end

def run_specs(*specs)
  specs = specs.flatten
  error_collector = ErrorCollector.new
  MSpec.register_files(specs)
  MSpec.register(:exception, error_collector)

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
