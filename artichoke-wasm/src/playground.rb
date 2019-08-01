# frozen_string_literal: true

require 'forwardable'
require 'json'

class Properties
  extend Forwardable
  def_delegators :@properties, :[], :[]=, :to_json

  def initialize(name)
    @name = name
    @properties = {}
  end

  def inspect
    @name
  end
end

artichoke = Properties.new('Artichoke Ruby')
artichoke[:language] = 'Ruby'
artichoke[:implementation] = 'Artichoke'
artichoke[:target] = 'wasm'
artichoke[:emoji] = '💎'

serialized = JSON.pretty_generate(artichoke)
puts serialized if serialized =~ /💎/

artichoke
