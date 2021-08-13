#!/usr/bin/env ruby
# frozen_string_literal: true

require 'json'
require 'yaml'

report = ARGF.readlines

# trim garbage printed to stdout during spec eval
report = report.drop_while { |line| line != "---\n" }
report = report.join

# manually escape some garbled YAML output by hand from `MSpec`
report = report.split('self\#@-').join('self#@-')
report = report.gsub(
  /String#unpack with format 'M' decodes pre-encoded byte values (.*) FAILED\\nExpected.*?artichoke/,
  "String#unpack with format 'M' decodes pre-encoded byte values \\1 FAILED\\nExpected [...]./artichoke"
)

parsed = YAML.safe_load(report)
converted = JSON.pretty_generate(parsed)

puts converted
