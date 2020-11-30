# frozen_string_literal: true

require 'fileutils'
require 'rbconfig'
require 'shellwords'

def windows?
  /mswin|msys|mingw|cygwin|bccwin|wince|emc/.match?(RbConfig::CONFIG['host_os'])
end

def noop_for(build:, command:)
  [
    'ruby',
    File.join(File.dirname(File.absolute_path(__FILE__)), 'noop.rb'),
    "build=#{build}",
    "command=#{command}"
  ].shelljoin
end

module MRuby
  class Build
    def build_mrbc_exec; end
  end
end

module MRuby
  class Command
    class Mrbc
      def run(out, infiles, funcname, cdump = true); end # rubocop:disable Style/OptionalBooleanParameter
    end
  end
end

# mruby requires a "default" build. This default build bootstraps the
# compilation of the "sys" build.
#
# The "bootstrap" build compiles the mrbc bytecode compiler which is required
# by some gems in the "sys" build.
#
# This build can be nulled out once the Artichoke runtime is complete.
MRuby::Build.new do |conf|
  conf.cc.command = noop_for(build: 'host', command: 'cc')
  conf.cxx.command = noop_for(build: 'host', command: 'cxx')
  conf.objc.command = noop_for(build: 'host', command: 'objc')
  conf.asm.command = noop_for(build: 'host', command: 'asm')
  conf.gperf.command = noop_for(build: 'host', command: 'gperf')
  conf.gperf.compile_options = ''
  conf.linker.command = noop_for(build: 'host', command: 'linker')
  conf.archiver.command = noop_for(build: 'host', command: 'archiver')
  conf.mrbc.command = noop_for(build: 'host', command: 'mrbc')

  # use embedded `y.tab.c`.
  #
  # See artichoke/mruby@a3ec6ede76bba29616ff4df33cd19ae59ebca07f.
  conf.yacc.command = noop_for(build: 'host', command: 'yacc')

  conf.bins = []
  conf.gembox File.join(File.dirname(File.absolute_path(__FILE__)), 'bootstrap')

  FileUtils.mkdir_p("#{build_dir}/bin")
  FileUtils.touch("#{build_dir}/bin/mrbc")
  FileUtils.touch("#{build_dir}/bin/mrbc.exe")
end

# This cross-build generates C sources so `build.rs` can compile them into a
# static lib.
MRuby::CrossBuild.new('sys') do |conf|
  def build_mrbc_exec; end

  conf.cc.command = noop_for(build: 'sys', command: 'cc')
  conf.cxx.command = noop_for(build: 'sys', command: 'cxx')
  conf.objc.command = noop_for(build: 'sys', command: 'objc')
  conf.asm.command = noop_for(build: 'sys', command: 'asm')
  conf.gperf.command = noop_for(build: 'sys', command: 'gperf')
  conf.gperf.compile_options = ''
  conf.linker.command = noop_for(build: 'sys', command: 'linker')
  conf.archiver.command = noop_for(build: 'sys', command: 'archiver')
  conf.mrbc.command = noop_for(build: 'sys', command: 'mrbc')

  # use embedded `y.tab.c`.
  #
  # See artichoke/mruby@a3ec6ede76bba29616ff4df33cd19ae59ebca07f.
  conf.yacc.command = noop_for(build: 'sys', command: 'yacc')

  # C compiler settings
  # https://github.com/mruby/mruby/blob/master/doc/guides/mrbconf.md#other-configuration
  conf.cc.defines += %w[MRB_DISABLE_STDIO MRB_UTF8_STRING MRB_ARY_NO_EMBED MRB_NO_BOXING]

  conf.bins = []

  # gemset for mruby artichoke static lib
  conf.gembox File.join(File.dirname(File.absolute_path(__FILE__)), 'sys')

  FileUtils.mkdir_p("#{build_dir}/bin")
  FileUtils.touch("#{build_dir}/bin/mrbc")
  FileUtils.touch("#{build_dir}/bin/mrbc.exe")
end
