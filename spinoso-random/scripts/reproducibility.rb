#!/usr/bin/env ruby
# frozen_string_literal: true

Random.srand 33
bytes_a = Random.bytes(64)

Random.srand 33
bytes_b = Random.bytes(64)

raise 'not reproducible' unless bytes_a == bytes_b

puts 'Random bytes:', bytes_a.bytes, ''

Random.srand 33
floats_a = 20.times.map { Random.rand }

Random.srand 33
floats_b = 20.times.map { Random.rand }

raise 'not reproducible' unless floats_a == floats_b

puts 'Random floats:', floats_a
