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

class SpecCollector
  def initialize
    @errors = []
    @total = 0
    @successes = 0
    @failures = 0
    @skipped = 0
    @not_implemented = 0
  end

  def success?
    @errors.empty?
  end

  def start
    MSpecScript.set(:backtrace_filter, %r{/lib/mspec/})
  end

  def enter(description)
    collector = self
    MSpec.current.before(:each) { collector.begin }
    puts "\n", "In #{description}:", ''
  end

  def begin
    @total += 1
    print '.'
  end

  def exception(state)
    skipped = false
    if state.exception.is_a?(NoMethodError)
      skipped = true if state.message =~ /'allocate'/
      skipped = true if state.message =~ /'encoding'/
      skipped = true if state.message =~ /'private_instance_methods'/
      skipped = true if state.message =~ /'taint'/
      skipped = true if state.message =~ /'tainted\?'/
      skipped = true if state.message =~ /'untrust'/
    elsif state.exception.is_a?(SpecExpectationNotMetError)
      skipped = true if state.it =~ /encoding/
      skipped = true if state.it =~ /ASCII/
      skipped = true if state.it =~ /is too big/ # mruby does not have Bignum
    elsif state.exception.is_a?(SyntaxError)
      skipped = true if state.it =~ /encoding/
      skipped = true if state.it =~ /ASCII/
    elsif state.exception.is_a?(NotImplementedError)
      @not_implemented += 1
      return
    end
    skipped = true if state.it == 'is multi-byte character sensitive'
    skipped = true if state.it =~ /UTF-8/
    if skipped
      @skipped += 1
      print "\b\e[33mS\e[0m"
    else
      @errors << state
      print "\b\e[31mX\e[0m"
    end
    nil
  end

  def finish
    @failures = @errors.length
    @successes = @total - @failures - @skipped

    puts "\n"
    if @errors.length.zero?
      puts "\e[32mPassed #{@successes} specs. Skipped #{@skipped} spec. Not implemented #{@not_implemented}.\e[0m"
      return
    end

    puts "\e[31mPassed #{@successes}, skipped #{@skipped}, not implemented #{@not_implemented}, failed #{@errors.length} specs.\e[0m"
    @errors.each do |state|
      puts '', "\e[31m#{state.message} in #{state.it}\e[0m", '', state.backtrace
    end
    puts "\e[31mPassed #{@successes}, skipped #{@skipped}, not implemented #{@not_implemented}, failed #{@errors.length} specs.\e[0m"
  end
end

def run_specs(*specs)
  specs = specs.flatten
  collector = SpecCollector.new

  MSpec.register(:start, collector)
  MSpec.register(:enter, collector)
  MSpec.register(:exception, collector)
  MSpec.register(:finish, collector)
  MSpec.register_files(specs)

  MSpec.process

  collector.success?
end
