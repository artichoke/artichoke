# frozen_string_literal: true

# mruby requires a "default" build. This is the base build from
# vendor/mruby/build_config.rb
MRuby::Build.new do |conf|
  # Gets set by the VS command prompts.
  if ENV['VisualStudioVersion'] || ENV['VSINSTALLDIR']
    toolchain :visualcpp
  else
    toolchain :clang
  end

  # include the default GEMs
  conf.gembox 'default'
end

MRuby::CrossBuild.new('sys-emscripten') do |conf|
  toolchain :clang
  conf.cc.command = 'emcc'
  conf.cc.flags << '-Os'
  conf.cc.flags << '-fPIC'
  conf.linker.command = 'emcc'
  conf.linker.flags = %w[-s WASM=1 -s ENVIRONMENT=web -s LINKABLE=1 -s EXPORT_ALL=1]
  conf.archiver.command = 'emar'

  # C compiler settings
  # https://github.com/mruby/mruby/blob/master/doc/guides/mrbconf.md#other-configuration
  conf.cc.defines += %w[MRB_DISABLE_STDIO MRB_UTF8_STRING]

  conf.bins = []

  # gemset for mruby-sys
  # NOTE: Disable some gems from `default.gembox` because they violate our
  # expectations around sandboxing (e.g. no filesystem access).
  conf.gembox File.join(File.dirname(File.absolute_path(__FILE__)), 'sys')
end

# MRuby::CrossBuild.new('sys-wasm') do |conf|
#   toolchain :clang
#   conf.cc.flags << '-emit-llvm'
#   conf.cc.flags << '--target=wasm32'
#   conf.cc.flags << '-Os'
#   conf.cc.flags << '-fPIC'
#   conf.linker.flags << '-asm-verbose=false'
#
#   # C compiler settings
#   # https://github.com/mruby/mruby/blob/master/doc/guides/mrbconf.md#other-configuration
#   conf.cc.defines += %w[MRB_DISABLE_STDIO MRB_ENABLE_DEBUG_HOOK MRB_UTF8_STRING]
#
#   conf.bins = []
#
#   # gemset for mruby-sys
#   # NOTE: Disable some gems from `default.gembox` because they violate our
#   # expectations around sandboxing (e.g. no filesystem access).
#   conf.gembox File.join(File.dirname(File.absolute_path(__FILE__)), 'sys')
# end
