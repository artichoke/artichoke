# frozen_string_literal: true

require 'securerandom'
require 'foolsgold/metrics'

module FoolsGold
  class RequestContext
    attr_accessor :trace_id

    def initialize
      @trace_id = SecureRandom.uuid
    end

    def metrics
      FoolsGold::Metrics
    end
  end
end
