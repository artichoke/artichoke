# frozen_string_literal: true

# The purpose of this script is to open a fresh interpreter, pull the constants,
# require a library and figure out what constants were added.
BASE = ARGV[0]
PACKAGE = ARGV[1]

$LOAD_PATH.unshift(BASE)

require PACKAGE

lib_sources = $LOADED_FEATURES.select { |f| f.include?(BASE) }
lib_sources +=
  $LOADED_FEATURES
    .map { |f| f.gsub('/', '\\') }
    .select { |f| f.include?(BASE) }
package_sources = lib_sources.grep(/#{PACKAGE}/)

puts package_sources.sort.uniq
