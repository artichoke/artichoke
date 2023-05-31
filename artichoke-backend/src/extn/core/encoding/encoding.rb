# frozen_string_literal: true

# TODO: Properly implement this class now that Artichoke has encoding support.
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

  def self.default_external
    UTF_8
  end

  def self.default_external=(_enc)
    UTF_8
  end

  def self.default_internal
    UTF_8
  end

  def self.default_internal=(_enc)
    UTF_8
  end

  def self.find(string)
    new(string)
  end

  attr_reader :name

  def ascii_compatible?
    true
  end

  def dummy?
    true
  end

  def inspect
    "#<#{self.class}:#{@name}>"
  end

  def names
    [name]
  end

  def replicate(name)
    new(name)
  end

  def to_s
    name
  end
end
