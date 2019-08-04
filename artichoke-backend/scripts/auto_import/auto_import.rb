#!/usr/bin/env ruby
# frozen_string_literal: true

require 'erb'

raise 'must provide a library to import' if ARGV[0].nil?
raise 'must provide an output directory' if ARGV[1].nil?

LIB = ARGV[0]
OUT_DIR = ARGV[1]
auto_import_dir = File.dirname(__FILE__)
source_file = `gem which #{LIB}`.strip # used in erb
filename = File.basename(source_file)
rust_filename = filename.gsub(/.rb$/, '.rs')
# Import the Ruby 2.6.3 sources.
`cp #{source_file} #{OUT_DIR}`
constants = `#{auto_import_dir}/get_constants_loaded.rb "#{LIB}"`.split("\n")

# Add Rust glue, like this example for ostruct. Make a commit here.
template = File.read("#{auto_import_dir}/rust_glue.rs.erb")
renderer = ERB.new(template)
output = renderer.result(binding)
File.write("#{OUT_DIR}#{rust_filename}", output)

# Add test for spec compliance. Make a commit here.
# Run spec compliance tests with yarn spec.
# If the tests pass, great. If they do not, there is likely a bug in Artichoke
# or an upstream bug in mruby. We can discuss in this issue how to proceed.
