# frozen_string_literal: true

# mruby requires a "default" build. This is the base build from
# vendor/mruby/build_config.rb
MRuby::Build.new do |conf|
  # Gets set by the VS command prompts.
  if ENV['VisualStudioVersion'] || ENV['VSINSTALLDIR']
    toolchain :visualcpp
  else
    toolchain :gcc
  end

  # include the default GEMs
  conf.gembox 'default'
end

# A minimal build of mruby for the mruby-sys crate. This build config does the
# following:
#
# - Set `-fPIC` CFLAG which is expected by static libs in Rust sys crates.
# - Disable <stdio.h> dependent code in mruby.
# - Do not build mruby binaries.
# - Enable mruby debug hooks.
# - Compile a custom set  of gems. This gembox removes these gems from the
#   default gembox: mruby-io, mruby-print, mruby-objectspace.
MRuby::CrossBuild.new('sys') do |conf|
  # Gets set by the VS command prompts.
  if ENV['VisualStudioVersion'] || ENV['VSINSTALLDIR']
    toolchain :visualcpp
  else
    toolchain :gcc
    conf.cc.flags << '-fPIC'
  end

  enable_debug

  # C compiler settings
  # https://github.com/mruby/mruby/blob/master/doc/guides/mrbconf.md#other-configuration
  conf.cc.defines += %w[MRB_DISABLE_STDIO MRB_ENABLE_DEBUG_HOOK MRB_UTF8_STRING]

  conf.bins = []

  # gemset for mruby-sys
  # NOTE: Disable some gems from `default.gembox` because they violate our
  # expectations around sandboxing (e.g. no filesystem access).
  conf.gembox File.join(File.dirname(File.absolute_path(__FILE__)), 'sys')
end
