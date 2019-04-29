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

  conf.gembox 'default'
end
