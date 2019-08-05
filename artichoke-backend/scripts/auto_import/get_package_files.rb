# frozen_string_literal: true

# The purpose of this script is to open a fresh interpreter, pull the constants,
# require a library and figure out what constants were added.
base = ARGV[0]
package = ARGV[1]
require package
lib_sources = $LOADED_FEATURES.select { |f| f.start_with?(base) }
package_sources = lib_sources.select { |f| f =~ %r{/#{package}} }
puts package_sources.sort
