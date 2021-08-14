#!/usr/bin/env ruby
# frozen_string_literal: true

if $PROGRAM_NAME == __FILE__
  mspec = File.join(File.dirname(__FILE__), '..', 'vendor', 'mspec', 'lib')
  $LOAD_PATH.unshift(mspec)
end

class StubIO
  def initialize(is_stderr: false)
    @is_stderr = is_stderr
  end

  def puts(*args)
    if @is_stderr
      Kernel.warn(*args)
      return
    end
    Kernel.puts(*args)
  end

  def print(*args)
    Kernel.print(*args)
  end

  def flush
    Kernel.puts ''
  end

  def method_missing(method, *args, &block)
    super
  rescue NoMethodError
    nil
  end

  def respond_to_missing?(method, include_private)
    true || super
  end
end

STDOUT = StubIO.new unless Object.const_defined?(:STDOUT)
STDERR = StubIO.new(is_stderr: true) unless Object.const_defined?(:STDERR)
$stdout ||= STDOUT # rubocop:disable Style/GlobalStdStream
$stderr ||= STDERR # rubocop:disable Style/GlobalStdStream

RUBY_EXE = '/usr/bin/true'

require 'mspec'
require 'mspec/runner/actions'
require 'mspec/utils/script'

module Artichoke
  module Spec
    module Formatter
      class Artichoke
        RED = "\e[31m"
        GREEN = "\e[32m"
        YELLOW = "\e[33m"
        PLAIN = "\e[0m"

        def self.run_specs(*specs)
          specs = specs.flatten
          MSpec.register_files(specs)

          collector = new

          MSpec.register(:start, collector)
          MSpec.register(:enter, collector)
          MSpec.register(:before, collector)
          MSpec.register(:after, collector)
          MSpec.register(:exception, collector)
          MSpec.register(:finish, collector)

          MSpec.process

          collector.success?
        end

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
          case state.exception
          when ArgumentError
            skipped = true if state.message =~ /Oniguruma.*UTF-8/
          when NoMethodError
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
            skipped = true if state.message =~ /undefined method 'Rational'/
          when NameError
            skipped = true if state.message =~ /uninitialized constant Bignum/
          when SpecExpectationNotMetError
            skipped = true if state.it =~ /encoding/
            skipped = true if state.it =~ /ASCII/
            skipped = true if state.it =~ /is too big/ # mruby does not have Bignum
            skipped = true if state.it =~ /hexadecimal digits/
          when SyntaxError
            skipped = true if state.it =~ /ASCII/
            skipped = true if state.it =~ /hexadecimal digits/
            skipped = true if state.message =~ /Regexp pattern/
          when TypeError
            skipped = true if state.it =~ /encoding/
          when NotImplementedError
            @not_implemented += 1
            @spec_state = "\b#{YELLOW}N#{PLAIN}"
            return
          when RuntimeError
            skipped = true if state.message =~ /invalid UTF-8/
          end
          skipped = true if state.it == 'does not add a URI method to Object instances'
          skipped = true if state.it == 'is multi-byte character sensitive'
          skipped = true if state.it =~ /UTF-8/
          skipped = true if state.it =~ /\\u/

          skipped = true if state.describe == 'Regexp#initialize'

          skipped = true if state.it =~ /Bignum/

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

      class Summary
        def self.run_specs(*specs)
          specs = specs.flatten
          MSpec.register_files(specs)

          MSpecScript.set(:backtrace_filter, %r{/lib/mspec/})

          formatter = SummaryFormatter.new
          formatter.register

          MSpec.process

          return false unless formatter.tally.counter.failures.zero?
          return false unless formatter.tally.counter.errors.zero?

          true
        end
      end

      class Yaml
        def self.run_specs(*specs)
          specs = specs.flatten
          MSpec.register_files(specs)

          MSpecScript.set(:backtrace_filter, %r{/lib/mspec/})

          formatter = YamlFormatter.new
          formatter.register

          MSpec.process

          return false unless formatter.tally.counter.failures.zero?
          return false unless formatter.tally.counter.errors.zero?

          true
        end
      end

      class Tagger
        def self.run_specs(*specs)
          specs = specs.flatten
          MSpec.register_files(specs)

          MSpecScript.set(:backtrace_filter, %r{/lib/mspec/})

          tagger = :add
          tag = 'spec-runner-tagger:'

          case tagger
          when :add, :del
            tag = SpecTag.new(tag)
            tag_action = new(
              tag.tag,
              tag.comment
            )
          else
            raise ArgumentError, 'No recognized tagger action given'
          end
          tag_action.register

          MSpec.process

          true
        end

        def initialize(tag, comment, _tags = nil, _descs = nil)
          @tag = tag
          @comment = comment
          @report = []
          @exception = false
          @spec_group = nil
        end

        def enter(_description)
          state = MSpec.current
          parent = state.parent
          until parent.nil?
            state = parent
            parent = state.parent
          end
          @spec = state.to_s
          @spec = @spec[1..-1] if @spec.start_with?('#')
          @spec = 'Enumerable#slice_after' if @spec == 'when an iterator method yields more than one value'
          @spec = @spec.split[0]
          @spec_group = @spec.split(/[.#]/)[0]
        end

        def before(_state)
          @exception = false
        end

        def exception(_state)
          @exception = true
        end

        def after(state)
          tag = SpecTag.new
          tag.tag = @tag
          tag.comment = @comment
          tag.description = state.description

          outcome = :pass
          outcome = :fail if @exception
          @report << { outcome: outcome, tag: tag, spec: @spec, group: @spec_group }
        end

        def finish
          puts '---'
          puts 'tags:'
          @report.sort_by { |item| [item[:group], item[:spec], item[:tag].to_s] }.each do |item|
            puts "- tag: #{item[:tag].to_s.inspect}"
            puts "  group: #{item[:group].to_s.inspect}"
            puts "  spec: #{item[:spec].to_s.inspect}"
            puts "  outcome: #{item[:outcome]}"
          end
        end

        def register
          MSpec.register :enter,     self
          MSpec.register :before,    self
          MSpec.register :exception, self
          MSpec.register :after,     self
          MSpec.register :finish,    self
        end
      end
    end
  end
end

if $PROGRAM_NAME == __FILE__
  ENV['MSPEC_RUNNER'] = '1'
  specs = ARGV.reject do |file|
    next true if file.include?('/fixtures/')
    next true if file.include?('/shared/')

    false
  end
  Artichoke::Spec::Formatter::Artichoke.run_specs(*specs)
end
