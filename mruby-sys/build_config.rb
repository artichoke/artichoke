# frozen_string_literal: true

# mruby requires a "default" build. This default build bootstraps the
# compilation of the "sys" build.
#
# The "bootstrap" build compiles the mrbc bytecode compiler which is required
# by some gems in the "sys" build.
#
# This build can be nulled out once the Artichoke runtime is complete.
MRuby::Build.new do |conf|
  # Gets set by the VS command prompts.
  if ENV['VisualStudioVersion'] || ENV['VSINSTALLDIR']
    toolchain :visualcpp
  else
    toolchain :clang
  end
  conf.gperf.command = 'true'

  conf.bins = ['mrbc']
  conf.gembox File.join(File.dirname(File.absolute_path(__FILE__)), 'bootstrap')
end

# This cross-build generates C sources so `build.rs` can compile them into a
# static lib.
MRuby::CrossBuild.new('sys') do |conf|
  conf.cc.command = 'true'
  conf.cxx.command = 'true'
  conf.objc.command = 'true'
  conf.asm.command = 'true'
  conf.gperf.command = 'true'
  conf.linker.command = 'true'
  conf.archiver.command = 'true'

  # C compiler settings
  # https://github.com/mruby/mruby/blob/master/doc/guides/mrbconf.md#other-configuration
  conf.cc.defines += %w[MRB_DISABLE_STDIO MRB_UTF8_STRING]

  conf.bins = []

  # gemset for mruby-sys
  conf.gembox File.join(File.dirname(File.absolute_path(__FILE__)), 'sys')
end
