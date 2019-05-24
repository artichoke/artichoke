# frozen_string_literal: true

require 'rack/builder'
require 'thin'
$LOAD_PATH.unshift(File.join(File.dirname(__FILE__), '..', 'lib'))
require 'foolsgold'
require 'foolsgold/adapter/memory'
require 'foolsgold/stats'

# preload counter
FoolsGold::RequestContext.new.metrics.total_requests

rackup = File.read(File.join(File.dirname(__FILE__), '..', 'config.ru'))
builder = Rack::Builder.new_from_string(rackup)
app = FoolsGold::Adapter::Memory.new(builder)

map '/fools-gold' do
  run app
end

map '/img' do
  run Rack::Directory.new(File.join(File.dirname(__FILE__), '..', '..', 'static'))
end
