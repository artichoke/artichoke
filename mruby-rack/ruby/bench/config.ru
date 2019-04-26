require 'rubygems'
require 'bundler/setup'
require 'rack/builder'
require 'thin'
require 'fools-gold'

rackup = File.read(File.join(File.dirname(__FILE__), '..', 'config.ru'))
builder = Rack::Builder.new_from_string(rackup)
app = FoolsGold::Adapter::InMemory.new(builder)

map '/' do
  run app
end

map '/img' do
  run Rack::Directory.new(File.join(File.dirname(__FILE__), '..', '..', 'static'))
end
