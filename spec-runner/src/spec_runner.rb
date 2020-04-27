#!/usr/bin/env ruby
# frozen_string_literal: true

if $PROGRAM_NAME == __FILE__
  mspec = File.join(File.dirname(__FILE__), '..', 'vendor', 'mspec', 'lib')
  $LOAD_PATH.unshift(mspec)
end

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

STDOUT = StubIO.new unless Object.const_defined?(:STDOUT)
STDERR = StubIO.new unless Object.const_defined?(:STDERR)
RUBY_EXE = '/usr/bin/true'

require 'mspec'
require 'mspec/utils/script'

class SpecCollector
  RED = "\e[31m"
  GREEN = "\e[32m"
  YELLOW = "\e[33m"
  PLAIN = "\e[0m"

  def initialize
    @errors = []
    @total = 0
    @successes = 0
    @failures = 0
    @skipped = 0
    @not_implemented = 0
    @current_description = nil
    @spec_state = nil
  end

  def success?
    @errors.empty?
  end

  def start
    MSpecScript.set(:backtrace_filter, %r{/lib/mspec/})
  end

  def enter(description)
    print "\n", description, ': '
    @description = description
  end

  def before(_state)
    @total += 1
    @spec_state = nil
    print '.'
  end

  def after(_state)
    print @spec_state if @spec_state
  end

  def exception(state)
    skipped = false
    if state.exception.is_a?(ArgumentError)
      skipped = true if state.message =~ /Oniguruma.*UTF-8/
    elsif state.exception.is_a?(NoMethodError)
      skipped = true if state.message =~ /'allocate'/
      skipped = true if state.message =~ /'encoding'/
      skipped = true if state.message =~ /'private_instance_methods'/
      if state.message =~ /'size'/
        # Enumerable#size is not implemented on mruby
        skipped = true
      end
      skipped = true if state.message =~ /'taint'/
      skipped = true if state.message =~ /'tainted\?'/
      skipped = true if state.message =~ /'untrust'/
      skipped = true if state.message =~ /'untrusted\?'/
    elsif state.exception.is_a?(SpecExpectationNotMetError)
      skipped = true if state.it =~ /encoding/
      skipped = true if state.it =~ /ASCII/
      skipped = true if state.it =~ /is too big/ # mruby does not have Bignum
      skipped = true if state.it =~ /hexadecimal digits/
    elsif state.exception.is_a?(SyntaxError)
      skipped = true if state.it =~ /ASCII/
      skipped = true if state.it =~ /hexadecimal digits/
      skipped = true if state.message =~ /Regexp pattern/
    elsif state.exception.is_a?(TypeError)
      skipped = true if state.it =~ /encoding/
    elsif state.exception.is_a?(NotImplementedError)
      @not_implemented += 1
      @spec_state = "\b#{YELLOW}N#{PLAIN}"
      return
    elsif state.exception.is_a?(RuntimeError)
      skipped = true if state.message =~ /invalid UTF-8/
    end
    if state.it == 'does not add a URI method to Object instances'
      skipped = true
    end
    skipped = true if state.it == 'is multi-byte character sensitive'
    skipped = true if state.it =~ /UTF-8/
    skipped = true if state.it =~ /\\u/

    skipped = true if state.describe == 'Regexp#initialize'

    if skipped
      @skipped += 1
      @spec_state = "\b#{YELLOW}S#{PLAIN}"
    else
      @errors << state
      @spec_state = "\b#{RED}X#{PLAIN}"
    end
    nil
  end

  def finish
    failures = @errors.length
    successes = @total - @skipped - @not_implemented - failures
    successes = 0 if successes.negative?
    puts "\n"

    if failures.zero?
      report(
        color: GREEN,
        successes: successes,
        skipped: @skipped,
        not_implemented: @not_implemented,
        failed: failures
      )
      return
    end

    report(
      color: RED,
      successes: successes,
      skipped: @skipped,
      not_implemented: @not_implemented,
      failed: failures
    )
    @errors.each do |state|
      puts '',
           "#{RED}#{state.description}#{PLAIN}",
           "#{RED}#{state.exception.class}: #{state.exception}#{PLAIN}"
      puts '', state.backtrace unless state.exception.is_a?(SystemStackError)
    end
    puts ''
    report(
      color: RED,
      successes: successes,
      skipped: @skipped,
      not_implemented: @not_implemented,
      failed: failures
    )
  end

  def report(color:, successes:, skipped:, not_implemented:, failed:)
    print color
    print "Passed #{successes}, "
    print "skipped #{skipped}, "
    print "not implemented #{not_implemented}, "
    print "failed #{failed} specs."
    print PLAIN, "\n"
  end
end

def run_specs(*specs)
  specs = specs.flatten
  MSpec.register_files(specs)

  collector = SpecCollector.new

  MSpec.register(:start, collector)
  MSpec.register(:enter, collector)
  MSpec.register(:before, collector)
  MSpec.register(:after, collector)
  MSpec.register(:exception, collector)
  MSpec.register(:finish, collector)

  MSpec.process

  collector.success?
end

if $PROGRAM_NAME == __FILE__
  ENV['MSPEC_RUNNER'] = '1'
  specs = ARGV.reject do |file|
    next true if file.include?('/fixtures/')
    next true if file.include?('/shared/')

    false
  end
  run_specs(*specs)
end
