# frozen_string_literal: true

require 'sinatra/base'

class Ditty < Sinatra::Base
  # set :sessions, true
  set :foo, 'bar'

  get '/' do
    'Hello world!'
  end
end

run Ditty
