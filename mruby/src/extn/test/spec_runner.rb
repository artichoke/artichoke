# frozen_string_literal: true

require 'mspec'

class MSpecErrors
  attr_accessor :errors

  def initialize
    @errors = []
  end

  def exception(state)
    @errors << state
  end
end


def run_specs(*files)
  errors = MSpecErrors.new
  files = files.flatten
  MSpec.register_files(files)
  MSpec.register :exception, errors
  MSpec.process

  failures = errors.errors.map do |state|
    message = "#{state.exception.message} in #{state.it}\n\nBacktrace:\n"
    state.exception.backtrace.each do |frame|
      message << "\n#{frame}"
    end
    message
  end

  raise failures.join("\n\n") if failures.length.positive?
end
