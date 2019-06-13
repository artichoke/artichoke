# frozen_string_literal: true

class ScanError < StandardError; end

class StringScanner
  def self.must_C_version # rubocop:disable Naming/MethodName
    self
  end

  attr_accessor :string
  attr_accessor :charpos

  def initialize(string)
    @string = string
    @charpos = 0
    @last_match = nil
  end

  def <<(str)
    @string << str
  end
  alias concat <<

  def [](group)
    return nil if @last_match.nil?

    @last_match[group]
  end

  def beginning_of_line?
    return true if @pos.zero?

    @string[@pos - 1] == "\n"
  end
  alias bol? beginning_of_line?

  def captures
    @last_match.captures
  end

  def check(pattern)
    scan_full(pattern, false, true)
  end

  def check_until(pattern)
    old = @charpos
    result = scan_until(pattern)
    @charpos = old
    result
  end

  def eos?
    @charpos == @string.length
  end
  alias empty? eos?

  def exist?(pattern)
    !@string[@charpos..-1].match(pattern).nil?
  end

  def get_byte # rubocop:disable Naming/AccessorMethodName
    raise NotImplementedError, 'byte math'
  end
  alias getbyte get_byte

  def getch
    scan(/./)
  end

  def inspect
    before = @string.reverse[@string.length - @charpos, 5].reverse
    before = "\"...#{before}\"" unless before&.empty?
    after = @string[@charpos, 5]
    after = "\"#{after}...\"" unless after&.empty?
    "#<#{self.class.name} #{charpos}/#{@string.length} #{before} @ #{after} >"
  end

  def match?(pattern)
    match = pattern.match(@string[@pos..-1])
    return nil if match.nil?
    return nil if match.begin(0).positive?

    @last_match = match
    match.end(0) - match.end(0)
  end

  def matched
    @last_match[0]
  end

  def matched?
    !matched.nil?
  end

  def matched_size
    !matched&.length
  end

  def peek(len)
    @string[pos, len]
  end
  alias peep peek

  def pos
    @string[0..@charpos].bytes.length
  end
  alias pointer pos

  def pos=(_pointer)
    raise NotImplementedError, 'byte math'
  end

  def post_match
    return nil if @last_match.nil?

    @string[@charpos..-1]
  end

  def pre_match
    return nil if @last_match.nil?

    match_len = @last_match.end(0) - @last_match.begin(0)
    @string[0..@charpos - 1 - match_len]
  end

  def reset
    @charpos = 0
  end

  def rest
    @string[@charpos..-1]
  end

  def rest?
    !eos?
  end

  def rest_size
    rest.size
  end
  alias restsize rest_size

  def scan(pattern)
    scan_full(pattern, true, true)
  end

  def scan_full(pattern, advance_pointer_p, return_string_p)
    previous_charpos = @charpos
    match = pattern.match(@string[@charpos..-1])
    if match.nil? || match.begin(0).positive?
      @last_match = nil
      @previous_charpos = nil
      return nil
    end

    @charpos += match.end(0) if advance_pointer_p
    @previous_charpos = previous_charpos
    @last_match = match

    @string[previous_charpos, match.end(0)] if return_string_p
  end

  def scan_until(pattern)
    previous_charpos = @charpos
    match = pattern.match(@string[@charpos..-1])
    return nil if match.nil?

    @charpos += match.end(0)
    @previous_charpos = previous_charpos
    @last_match = match

    @string[previous_charpos, match.end(0)]
  end

  def search_full(pattern, advance_pointer_p, return_string_p)
    previous_charpos = @charpos
    match = pattern.match(@string[@charpos..-1])
    return nil if match.nil?

    @charpos += match.end(0) if advance_pointer_p
    @previous_charpos = previous_charpos

    @string[previous_charpos, match.end(0)] if return_string_p
  end

  def size
    @last_match.size
  end

  def skip(pattern)
    previous_charpos = @charpos
    match = pattern.match(@string[@charpos..-1])
    if match.nil? || match.begin(0).positive?
      @last_match = nil
      @previous_charpos = nil
      return nil
    end

    @charpos += match.end(0)
    @previous_charpos = previous_charpos
    @last_match = match
    match.end(0)
  end

  def skip_until(pattern)
    previous_charpos = @charpos
    match = pattern.match(@string[@charpos..-1])
    if match.nil?
      @last_match = nil
      @previous_charpos = nil
      return nil
    end

    @charpos += match.end(0)
    @previous_charpos = previous_charpos
    @last_match = match
    match.end(0)
  end

  def unscan
    raise ScanError, 'unscan failed: previous match record not exist' if @previous_charpos.nil?

    @charpos = @previous_charpos
    @previous_charpos = nil
    nil
  end

  def terminate
    @pos = @string.length
    @last_match = nil
  end
  alias clear terminate

  def values_at(*args)
    return nil if @last_match.nil?

    args.map { |index| @last_match[index] }
  end
end
