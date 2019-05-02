# frozen_string_literal: true

MRuby::Build.new do |conf|
  # load specific toolchain settings

  # Gets set by the VS command prompts.
  if ENV['VisualStudioVersion'] || ENV['VSINSTALLDIR']
    toolchain :visualcpp
  else
    toolchain :gcc
    conf.cc.flags << '-fPIC'
  end

  enable_debug

  # gemset for mruby-sys
  # NOTE: Disable some gems from `default.gembox` because they violate our
  # expectations around sandboxing (e.g. no filesystem access).
  conf.gembox File.join(File.dirname(__FILE__), 'sys')

  # C compiler settings
  conf.cc.defines = %w[MRB_ENABLE_DEBUG_HOOK]

  # Generate mirb command
  conf.gem core: 'mruby-bin-mirb'

  # Generate mruby command
  conf.gem core: 'mruby-bin-mruby'

  # Generate mruby debugger command (require mruby-eval)
  conf.gem core: 'mruby-bin-debugger'

  # bintest
  # conf.enable_bintest
end
