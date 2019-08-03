#!ruby -w
require 'erb'

raise 'must provide a library to import' if ARGV[0].nil?
LIB = ARGV[0]
RUBY = `which ruby`.strip
source_file = `gem which #{LIB}`.strip # used in erb
filename = File.basename(source_file)
rust_filename = filename.gsub(/.rb$/, '.rs')
# Import the Ruby 2.6.3 sources.
`cp #{source_file} ../../artichoke-backend/src/extn/stdlib/`
constants = `#{RUBY} ./get_constants_loaded.rb "#{LIB}"`.split("\n")

# Add Rust glue, like this example for ostruct. Make a commit here.
template = File.read('rust_glue.rs.erb')
renderer = ERB.new(template)
output = renderer.result(binding)
File.write("../../artichoke-backend/src/extn/stdlib/#{rust_filename}", output)



# Add test for spec compliance. Make a commit here.
# Run spec compliance tests with yarn spec.
# If the tests pass, 🎉. If they do not, there is likely a bug in Artichoke or an upstream bug in mruby. We can discuss in this issue how to proceed.