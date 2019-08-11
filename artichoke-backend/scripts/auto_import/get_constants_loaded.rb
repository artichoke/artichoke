#!/usr/bin/env ruby
# frozen_string_literal: true

# The purpose of this script is to open a fresh interpreter, pull the constants,
# require a library and figure out what constants were added.
base = ARGV[0]
package = ARGV[1]
$LOAD_PATH.unshift(base)
old_constants = Module.constants
require package
new_constants = Module.constants - old_constants
new_constants = new_constants.map { |const| "#{const},#{Object.const_get(const).class.name}" }
puts new_constants
