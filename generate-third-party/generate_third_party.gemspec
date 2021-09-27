# frozen_string_literal: true

Gem::Specification.new do |s|
  s.name                  = 'generate_third_party'
  s.version               = '0.0.0'
  s.required_ruby_version = '>= 2.6.3'
  s.summary               = "Generate Artichoke's third party dependencies"
  s.description           = 'Generate lists of third party dependencies and their licenses'
  s.authors               = ['Ryan Lopopolo']
  s.email                 = 'rj@hyperbo.la'
  s.files                 = Dir['lib/**/*'].keep_if { |file| File.file?(file) } + Dir['bin/*']
  s.bindir                = 'bin'
  s.homepage              = 'https://github.com/artichoke/artichoke'
  s.license               = 'MIT'

  s.executables << 'generate-third-party-html'
  s.executables << 'generate-third-party-text-file'
  s.executables << 'generate-third-party-text-file-single-target'
end
