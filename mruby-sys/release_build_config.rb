# frozen_string_literal: true

MRuby::Build.new do |conf|
  # Gets set by the VS command prompts.
  if ENV['VisualStudioVersion'] || ENV['VSINSTALLDIR']
    toolchain :visualcpp
  else
    toolchain :gcc
    conf.cc.flags << '-O3'
    conf.cc.flags << '-fPIC'
  end

  # gemset for mruby-sys
  # NOTE: Disable some gems from `default.gembox` because they violate our
  # expectations around sandboxing (e.g. no filesystem access).
  conf.gembox File.join(File.dirname(__FILE__), 'sys')
end
