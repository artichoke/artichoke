# frozen_string_literal: true

module FoolsGold
  module Middleware
    # Add a RequestContext to the Rack ENV
    class Request
      def initialize(app)
        @app = app
      end

      def call(env)
        @env = env.merge(CONTEXT => FoolsGold::RequestContext.new)
        @app.call(@env)
      end
    end
  end
end
