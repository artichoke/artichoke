# frozen_string_literal: true

module FoolsGold
  module Metrics
    attr_accessor :total_requests

    def self.total_requests
      @total_requests ||= FoolsGold::Counter.new
    end
  end
end
