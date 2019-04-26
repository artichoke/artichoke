require 'securerandom'
require 'foolsgold/counter'

COUNTER = FoolsGold::Counter.new

module FoolsGold
  class RequestStats
    def initialize
      @id = SecureRandom.uuid
    end

    def req_start
      @id
    end

    def seen_count
      COUNTER.get
    end

    def req_finalize
      COUNTER.inc
    end
  end
end
