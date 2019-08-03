#!ruby -w
old_constants = Module.constants
require ARGV[0]
new_constants = Module.constants - old_constants
puts new_constants