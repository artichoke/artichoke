module FoolsGold
  ENV_KEY = 'FOOLS_GOLD_REQ_STATS'.freeze

  module Adapter
    # In memory adapter that directly calls the rack app supplied in the block.
    class InMemory
      def initialize(&blk)
        @app = blk
      end

      def run
        @env = {
          ENV_KEY => FoolsGold::RequestStats.new
        }

        # status, headers, body = @app.call(@env)
        # Response.new(status: status, headers: headers, body: body)
        @app.call(@env)
      end
    end

    # Response wrapper for InMemory adapter
    class Response
      attr_reader :status
      attr_reader :body

      def initialize(status:, headers:, body:)
        @status = status
        @headers = headers
        @body = body.map(&:to_s).join
      end
    end
  end
end
