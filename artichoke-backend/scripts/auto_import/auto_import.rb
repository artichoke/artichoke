#!/usr/bin/env ruby
# frozen_string_literal: true

require 'erb'

raise 'must provide a library base path' if ARGV[0].nil?
raise 'must provide a library to import' if ARGV[1].nil?
raise 'must provide an output directory' if ARGV[2].nil?

base = ARGV[0]
package = ARGV[1]
out_file = ARGV[2]
sources = ARGV[3].to_s.split(',').map { |source| source.gsub(%r{^.*#{base}/?}, '').gsub(/\.rb$/, '') }
auto_import_dir = File.dirname(__FILE__)
# Import the Ruby 2.6.3 sources.
constants = `ruby --disable-did_you_mean --disable-gems #{auto_import_dir}/get_constants_loaded.rb "#{base}" "#{package}"`.split("\n").map { |const| const.split(',') }

# Add Rust glue, like this example for ostruct. Make a commit here.
template = File.read("#{auto_import_dir}/rust_glue.rs.erb")
renderer = ERB.new(template)
output = renderer.result(binding)
File.write(out_file.to_s, output)

# Add test for spec compliance. Make a commit here.
# Run spec compliance tests with yarn spec.
# If the tests pass, great. If they do not, there is likely a bug in Artichoke
# or an upstream bug in mruby. We can discuss in this issue how to proceed.
