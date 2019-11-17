# frozen_string_literal: true

require 'fileutils'
require 'rbconfig'

def windows?
  /mswin|msys|mingw|cygwin|bccwin|wince|emc/.match?(RbConfig::CONFIG['host_os'])
end

# mruby requires a "default" build. This default build bootstraps the
# compilation of the "sys" build.
#
# The "bootstrap" build compiles the mrbc bytecode compiler which is required
# by some gems in the "sys" build.
#
# This build can be nulled out once the Artichoke runtime is complete.
MRuby::Build.new do |conf|
  def build_mrbc_exec; end

  conf.cc.command = 'true'
  conf.cxx.command = 'true'
  conf.objc.command = 'true'
  conf.asm.command = 'true'
  conf.gperf.command = 'true'
  conf.gperf.compile_options = ''
  conf.linker.command = 'true'
  conf.archiver.command = 'true'
  conf.mrbc.command = 'true'

  conf.yacc.command = 'win_bison' if windows?

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

  conf.cc.command = 'true'
  conf.cxx.command = 'true'
  conf.objc.command = 'true'
  conf.asm.command = 'true'
  conf.gperf.command = 'true'
  conf.gperf.compile_options = ''
  conf.linker.command = 'true'
  conf.archiver.command = 'true'
  conf.mrbc.command = 'true'

  conf.yacc.command = 'win_bison' if windows?

  # C compiler settings
  # https://github.com/mruby/mruby/blob/master/doc/guides/mrbconf.md#other-configuration
  conf.cc.defines += %w[MRB_DISABLE_STDIO MRB_UTF8_STRING]

  conf.bins = []

  # gemset for mruby artichoke static lib
  conf.gembox File.join(File.dirname(File.absolute_path(__FILE__)), 'sys')

  FileUtils.mkdir_p("#{build_dir}/bin")
  FileUtils.touch("#{build_dir}/bin/mrbc")
  FileUtils.touch("#{build_dir}/bin/mrbc.exe")
end
