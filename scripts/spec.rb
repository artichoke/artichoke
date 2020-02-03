#!/usr/bin/env ruby
# frozen_string_literal: true

require 'optparse'
require 'shellwords'
require 'yaml'

WORKSPACE_ROOT = File.absolute_path(File.join(__dir__, '..'))
SPEC_CONFIG = File.join(WORKSPACE_ROOT, 'spec-runner', 'enforced-specs.yaml')
SPEC_ROOT = File.join(WORKSPACE_ROOT, 'spec-runner', 'vendor', 'spec')

USAGE = <<~USAGE.strip
  spec.rb runs ruby/specs against Artichoke and MRI.

  Usage: #{$PROGRAM_NAME} artichoke [ --timed ITERATIONS | --profile ] [ passing | family [ component [ spec ] ] ]
  Usage: #{$PROGRAM_NAME} ruby [ --timed ITERATIONS ] family [ component [ spec ] ]

  Examples:
      $ #{$PROGRAM_NAME} artichoke passing
      $ #{$PROGRAM_NAME} artichoke core
      $ #{$PROGRAM_NAME} artichoke core string
      $ #{$PROGRAM_NAME} ruby core string scan
      $ #{$PROGRAM_NAME} artichoke --timed 30 core string scan
      $ #{$PROGRAM_NAME} artichoke --profile passing
USAGE

class Runner
  attr_accessor :timing_iterations, :profile, :test
  attr_reader :specs, :all_core_specs, :all_lib_specs

  def validate!
    if [timing_iterations, profile].select(&:itself).length > 1
      puts USAGE
      exit 1
    elsif !timing_iterations && !profile
      @test = true
    end
  end

  def register(spec)
    @specs ||= []
    @specs << spec
  end

  def do_test
    warn 'Harness does not support profiling'
    exit 1
  end

  def do_timing
    warn 'Harness does not support profiling'
    exit 1
  end

  def do_profile
    warn 'Harness does not support profiling'
    exit 1
  end

  def run!
    if test
      do_test
    elsif timing_iterations
      do_timing
    elsif profile
      do_profile
    end
  end
end

class Artichoke < Runner
  def do_test
    exit 1 unless system('cargo build')
    binary = File.join(WORKSPACE_ROOT, 'target', 'debug', 'spec-runner')
    Dir.chdir(SPEC_ROOT)
    spec_sources = Spec.fixtures + specs.flat_map(&:files)
    spec_sources = spec_sources.map { |spec| '.' + spec.delete_prefix(SPEC_ROOT) }
    command = spec_sources.unshift(binary)
    exec(command.shelljoin)
  end

  def do_timing
    exit 1 unless system('cargo build --release')
    binary = File.join(WORKSPACE_ROOT, 'target', 'release', 'spec-runner')
    Dir.chdir(SPEC_ROOT)
    spec_sources = Spec.fixtures + specs.flat_map(&:files)
    spec_sources = spec_sources.map { |spec| '.' + spec.delete_prefix(SPEC_ROOT) }
    command = ['precise-time', timing_iterations, binary].concat(spec_sources)
    puts command.shelljoin
    exec(command.shelljoin)
  end

  def do_profile
    Dir.chdir(SPEC_ROOT)
    spec_sources = Spec.fixtures + specs.flat_map(&:files)
    spec_sources = spec_sources.map { |spec| spec.delete_prefix(SPEC_ROOT) }
    command = ['cargo', 'flamegraph', '-o', "#{WORKSPACE_ROOT}/flamegraph.svg", '--bin', 'spec-runner'].concat(spec_sources)
    exec(command.shelljoin)
  end
end

class Ruby < Runner
  def do_test
    runner = File.join(WORKSPACE_ROOT, 'spec-runner', 'src', 'spec_runner.rb')
    Dir.chdir(SPEC_ROOT)
    spec_sources = Spec.fixtures + specs.flat_map(&:files)
    spec_sources = spec_sources.map { |spec| '.' + spec.delete_prefix(SPEC_ROOT) }
    command = spec_sources.unshift(runner)
    exec(command.shelljoin)
  end

  def do_timing
    runner = File.join(WORKSPACE_ROOT, 'spec-runner', 'src', 'spec_runner.rb')
    Dir.chdir(SPEC_ROOT)
    spec_sources = Spec.fixtures + specs.flat_map(&:files)
    spec_sources = spec_sources.map { |spec| '.' + spec.delete_prefix(SPEC_ROOT) }
    command = ['precise-time', timing_iterations, runner].concat(spec_sources)
    puts command.shelljoin
    exec(command.shelljoin)
  end
end

class Spec
  def self.fixtures
    Dir.glob(File.join(SPEC_ROOT, '**', 'shared', '**', '*.rb')) +
      Dir.glob(File.join(SPEC_ROOT, '**', 'fixtures', '**', '*.rb'))
  end

  def initialize(group, component = nil, method = nil)
    @group = group
    @component = component
    @method = method
  end

  def files
    if @component.nil?
      Dir.glob(File.join(SPEC_ROOT, @group, '*', '*_spec.rb'))
    elsif @method.nil?
      Dir.glob(File.join(SPEC_ROOT, @group, @component, '*_spec.rb'))
    else
      Dir.glob(File.join(SPEC_ROOT, @group, @component, "#{@method}_spec.rb"))
    end
  end

  def inspect
    if @method
      "Spec<@group=#{@group} @component=#{@component} @method=#{@method}>"
    else
      "Spec<@group=#{@group} @component=#{@component}>"
    end
  end
  alias to_s inspect
end

runner =
  case ARGV.shift
  when 'artichoke' then Artichoke.new
  when 'ruby' then Ruby.new
  else
    puts USAGE
    exit 1
  end

ARGV.options do |opts|
  opts.on('--timed iterations', Integer) do |iterations|
    runner.timing_iterations = iterations
  end
  opts.on('--profile') do
    runner.profile = true
  end
end.parse!

runner.validate!

if ARGV.empty?
  puts USAGE
  exit 1
elsif ARGV.first == 'passing'
  conf = YAML.load_file(SPEC_CONFIG)
  conf.fetch('specs').each_pair do |category, suites|
    suites.map do |suite|
      name = suite.fetch('suite')
      if suite.key?('specs')
        suite.fetch('specs').each do |spec|
          runner.register(Spec.new(category, name, spec))
        end
      else
        runner.register(Spec.new(category, name))
      end
    end
  end
else
  runner.register(Spec.new(*ARGV))
end

runner.run!
