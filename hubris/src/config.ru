# frozen_string_literal: true

require 'sinatra/base'
require 'json'
require 'digest/sha1'
require 'securerandom'

# Sinatra app that echoes Rack environment as JSON.
class Echo < Sinatra::Base
  def self.all_methods(path, opts = {}, &block)
    get(path, opts, &block)
    post(path, opts, &block)
    put(path, opts, &block)
    delete(path, opts, &block)
    patch(path, opts, &block)
    options(path, opts, &block)
    head(path, opts, &block)
  end

  def headers
    env.select { |k, _v| k.start_with? 'HTTP_' }
  end

  def echo_response
    # TODO: nemesis does not pass RACK_INPUT
    body = request.body&.read || ''
    hash = Digest::SHA1.base64digest(body)

    headers.each do |(header, value)|
      response_header = header
                        .sub(/^HTTP_/, '')
                        .split('_')
                        .collect(&:capitalize)
                        .join('-')

      headers[response_header] = value
    end

    response_args = {
      method: request.request_method,
      path: request.path,
      args: request.query_string,
      body: body,
      headers: headers,
      uuid: SecureRandom.uuid
    }
    unless body.empty?
      response_args[:bodySha1] = hash
      response_args[:bodyLength] = body.length
    end

    # Prefer JSON if possible, including cases of unrecognised/erroneous types.
    content_type 'application/json'
    JSON.pretty_generate(response_args)
  end

  get '/status/:code' do |code|
    [code.to_i, echo_response]
  end

  get '/favicon.ico' do # Avoid bumping counter on favicon
  end

  all_methods '/*' do
    echo_response
  end
end

run Echo
