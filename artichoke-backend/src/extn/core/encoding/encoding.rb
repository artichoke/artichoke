# frozen_string_literal: true

class Encoding
  class CompatibilityError < StandardError; end

  def initialize(name)
    @name = name
  end

  ASCII_8BIT = new('ASCII-8BIT')
  BINARY = ASCII_8BIT
  US_ASCII = new('US-ASCII')
  ASCII = US_ASCII
  EUC_JP = new('EUC-JP')
  IBM437 = new('IBM437')
  ISO_8859_1 = new('ISO-8859-1')
  Shift_JIS = new('Shift_JIS')
  SHIFT_JIS = Shift_JIS
  UTF_8 = new('UTF-8')
  UTF_16LE = new('UTF-16LE')
  UTF_32BE = new('UTF-32BE')

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
