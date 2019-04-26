module FoolsGold
  module Adapter
    # In memory adapter that directly calls the rack app supplied in the block.
    class Memory
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
