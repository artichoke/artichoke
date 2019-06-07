# frozen_string_literal: true

RUBY_ROOT = File.join(File.dirname(__FILE__), '..', 'ruby')
$LOAD_PATH.unshift(File.join(RUBY_ROOT, 'lib'))

require 'rack/builder'
require 'thin'
require 'foolsgold'
require 'foolsgold/stats'
require 'foolsgold/metrics'
require 'foolsgold/counter'
require 'foolsgold/middleware/request'

# preload counter
FoolsGold::RequestContext.new.metrics.total_requests

map '/' do
  root = File.join(File.dirname(__FILE__), '..', 'static')
  use Rack::Static, urls: [''], root: root, index: 'index.html'
  run ->(_env) { [404, {}, 'Not Found'] }
end

map '/fools-gold/shared-nothing' do
  rackup = File.read(File.join(RUBY_ROOT, 'config.ru'))
  run Rack::Builder.new_from_string(rackup)
end

map '/fools-gold/prefork' do
  rackup = File.read(File.join(RUBY_ROOT, 'config.ru'))
  run Rack::Builder.new_from_string(rackup)
end
