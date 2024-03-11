# frozen_string_literal: true

class Encoding
  class CompatibilityError < StandardError; end

  class << self
    attr_reader :default_external, :default_internal
  end

  def self.default_external=(encoding)
    encoding = find(encoding) unless encoding.instance_of?(Encoding)

    @default_external = encoding
  end

  def self.default_internal=(encoding)
    encoding = find(encoding) unless encoding.instance_of?(Encoding)

    @default_external = encoding
  end
end
