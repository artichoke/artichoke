#!/usr/bin/env ruby
# frozen_string_literal: true

# The purpose of this script is to open a fresh interpreter, pull the constants,
# require a library and figure out what constants were added.
old_constants = Module.constants
require ARGV[0]
new_constants = Module.constants - old_constants
puts new_constants
