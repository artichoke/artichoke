module FoolsGold
  REQ_STATS = 'FOOLS_GOLD_REQ_STATS'.freeze

  module Adapter
    # In memory adapter that directly calls the rack app supplied in the block.
    class InMemory
      def initialize(app)
        @app = app
      end

      def call(env)
        @env = env.merge(REQ_STATS => FoolsGold::RequestStats.new)

        @app.call(@env)
      end
    end
  end
end
