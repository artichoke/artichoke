# frozen_string_literal: true

class ScanError < StandardError; end

class StringScanner
  def self.must_C_version # rubocop:disable Naming/MethodName
    self
  end

  attr_reader :charpos
  attr_reader :string

  def string=(str)
    @string = String.try_convert(str)
  end

  def initialize(string)
    @string = String.try_convert(string)
    @charpos = 0
    @previous_charpos = nil
    @last_match = nil
    @last_match_charpos = nil
  end

  def <<(str)
    raise TypeError if (str = String.try_convert(str)).nil?

    @string << str
    self
  end
  alias concat <<

  def [](group)
    return nil if @last_match.nil?
    raise IndexError unless @last_match.is_a?(MatchData)
    raise TypeError if group.nil?
    raise TypeError if group.is_a?(Range)

    case group
    when Integer, Float then
      group = group.to_int
      return nil unless group < @last_match.captures.length + 1
    when String then raise IndexError unless @last_match.named_captures.key?(group)
    when Symbol then raise IndexError unless @last_match.named_captures.key?(group.to_s)
    end
    @last_match[group]
  end

  def beginning_of_line?
    return true if @charpos.zero?

    @string[@charpos - 1] == "\n"
  end
  alias bol? beginning_of_line?

  def captures
    @last_match&.captures
  end

  def charpos=(pointer)
    raise RangeError unless pointer.abs < @string.length

    @charpos =
      if pointer.negative?
        @string.length + pointer
      else
        pointer
      end
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

  def empty?
    warn 'empty? is obsolete use eos? instead' if $VERBOSE

    eos?
  end

  def exist?(pattern)
    match = @string[@charpos..-1].match(pattern)
    return nil if match.nil?

    match.end(0)
  end

  def get_byte # rubocop:disable Naming/AccessorMethodName
    return nil if eos?

    byte, *_bytes = @string[@charpos..-1].bytes
    @charpos += 1
    @last_match_charpos = @charpos
    @last_match = [byte].pack('c*')
  end

  def getbyte
    warn 'getbyte is obsolete use get_byte instead' if $VERBOSE

    get_byte
  end

  def getch
    scan(/./)
  end

  def inspect
    return "#<#{self.class.name} fin>" if eos?

    before = @string.reverse[@string.length - @charpos, 5].reverse
    if before.length.positive? && before.length < 5
      before = " \"#{before}\""
    elsif !before.empty?
      before = " \"...#{before}\""
    end
    after = @string[@charpos, 5]
    after = "\"#{after}...\"" unless after&.empty?
    "#<#{self.class.name} #{charpos}/#{@string.length}#{before} @ #{after}>"
  end

  def match?(pattern)
    match = pattern.match(@string[@charpos..-1])
    if match.nil? || match.begin(0).positive?
      @last_match = nil
      @last_match_charpos = nil
      return nil
    end

    @last_match = match
    @last_match_charpos ||= 0
    @last_match_charpos += match.end(0)
    match.end(0) - match.begin(0)
  end

  def matched
    return nil if @last_match.nil?

    @last_match[0]
  end

  def matched?
    !matched.nil?
  end

  def matched_size
    matched&.length
  end

  def peek(len)
    raise RangeError unless len.is_a?(Integer)
    raise ArgumentError if len.negative?

    @string.bytes[pos, len].pack('c*')
  end

  def peep(len)
    warn 'peep is obsolet use peek instead' if $VERBOSE
    peek(len)
  end

  def pos
    @string[0...@charpos].bytes.length
  end
  alias pointer pos

  def pos=(pointer)
    raise RangeError unless pointer.abs < @string.bytesize

    @charpos =
      if pointer.negative?
        @string.bytes[0..pointer - 1].pack('c*').length
      else
        @string.bytes[0, pointer].pack('c*').length
      end
    pointer # rubocop:disable Lint/Void
  end
  alias pointer= pos=

  def post_match
    return nil if @last_match.nil?

    @string[@last_match_charpos..-1]
  end

  def pre_match
    return nil if @last_match.nil?

    match_len =
      if @last_match.is_a?(MatchData)
        @last_match.end(0) - @last_match.begin(0)
      else
        @last_match.length
      end
    @string[0...@last_match_charpos - match_len]
  end

  def reset
    @charpos = 0
    @previous_charpos = nil
    @last_match = nil
    @last_match_charpos = nil
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

  def restsize
    warn 'restsize is obsolete use rest_size instead' if $VERBOSE

    rest_size
  end

  def scan(pattern)
    scan_full(pattern, true, true)
  end

  def scan_full(pattern, advance_pointer_p, return_string_p)
    raise TypeError, "wrong argument type #{pattern.class} (expected Regexp)" unless pattern.is_a?(Regexp)

    previous_charpos = @charpos
    match = pattern.match(@string[@charpos..-1])
    if match.nil? || match.begin(0).positive?
      @last_match = nil
      @last_match_charpos = nil
      @previous_charpos = nil
      return nil
    end

    @charpos += match.end(0) if advance_pointer_p
    @previous_charpos = previous_charpos
    @last_match = match
    @last_match_charpos = @charpos

    if return_string_p
      @string[previous_charpos, match.end(0)]
    else
      match.end(0)
    end
  end

  def scan_until(pattern)
    previous_charpos = @charpos
    match = pattern.match(@string[@charpos..-1])
    return nil if match.nil?

    @charpos += match.end(0)
    @previous_charpos = previous_charpos
    @last_match = match
    @last_match_charpos = @charpos

    @string[previous_charpos, match.end(0)]
  end

  def search_full(pattern, advance_pointer_p, return_string_p)
    previous_charpos = @charpos
    match = pattern.match(@string[@charpos..-1])
    return nil if match.nil?

    @charpos += match.end(0) if advance_pointer_p
    @previous_charpos = previous_charpos
    if return_string_p
      @string[previous_charpos, match.end(0)]
    else
      match.end(0)
    end
  end

  def size
    @last_match.size
  end

  def skip(pattern)
    previous_charpos = @charpos
    match = pattern.match(@string[@charpos..-1])
    if match.nil? || match.begin(0).positive?
      @last_match = nil
      @last_match_charpos = nil
      @previous_charpos = nil
      return nil
    end

    @charpos += match.end(0)
    @previous_charpos = previous_charpos
    @last_match = match
    @last_match_charpos = @charpos
    match.end(0)
  end

  def skip_until(pattern)
    previous_charpos = @charpos
    match = pattern.match(@string[@charpos..-1])
    if match.nil?
      @last_match = nil
      @last_match_charpos = nil
      @previous_charpos = nil
      return nil
    end

    @charpos += match.end(0)
    @previous_charpos = previous_charpos
    @last_match = match
    @last_match_charpos = @charpos
    match.end(0)
  end

  def unscan
    raise ScanError, 'unscan failed: previous match record not exist' if @previous_charpos.nil?

    @charpos = @previous_charpos
    @previous_charpos = nil
    @last_match = nil
    @last_match_charpos = nil
    nil
  end

  def terminate
    @charpos = @string.length
    @last_match = nil
    @last_match_charpos = nil
    self
  end

  def clear
    warn 'clear is obsolete use terminate instead' if $VERBOSE

    terminate
  end

  def values_at(*args)
    return nil if @last_match.nil?

    args.map { |index| @last_match[index] }
  end
end
