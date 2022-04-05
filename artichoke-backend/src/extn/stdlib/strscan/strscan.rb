# frozen_string_literal: true

class ScanError < StandardError; end

class StringScanner
  def self.must_C_version # rubocop:disable Naming/MethodName
    self
  end

  def initialize(string)
    self.string = string
  end

  def <<(str)
    raise TypeError if (str = String.try_convert(str)).nil?
    raise FrozenError, 'can\'t modify frozen String' if @string.frozen?

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
    when Integer, Float
      group = group.to_int
      return nil unless group.abs < @last_match.captures.length + 1
    when String
      raise IndexError unless @last_match.named_captures.key?(group)
    when Symbol
      raise IndexError unless @last_match.named_captures.key?(group.to_s)
    end
    @last_match[group]
  end

  def beginning_of_line?
    return true if @pos.zero?

    @string.byteslice(@pos - 1) == "\n"
  end
  alias bol? beginning_of_line?

  def captures
    @last_match&.captures
  end

  def charpos
    @string.byteslice(0, @pos).length
  end

  def check(pattern)
    scan_full(pattern, false, true)
  end

  def check_until(pattern)
    old = @pos
    result = scan_until(pattern)
    @pos = old
    result
  end

  def clear
    warn 'clear is obsolete use terminate instead' if $VERBOSE

    terminate
  end

  def empty?
    warn 'empty? is obsolete use eos? instead' if $VERBOSE

    eos?
  end

  def eos?
    @pos == @string.bytesize
  end

  def exist?(pattern)
    match = @string.byteslice(@pos, @string.bytesize - @pos).match(pattern)
    return nil if match.nil?

    match.end(0)
  end

  def fixed_anchor?
    raise NotImplementedError, 'StringScanner#fixed_anchor? is not yet implemented'
  end

  def get_byte # rubocop:disable Naming/AccessorMethodName
    return nil if eos?

    byte = @string.byteslice(@pos)
    @pos += 1
    @last_match_pos = @pos
    @last_match = byte
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

    result = +"#<#{self.class.name}"
    result << " #{@pos}/#{@string.bytesize}"
    result << " "

    slice_begin = @pos - 5
    slice_begin = 0 if slice_begin.negative?
    if slice_begin < @pos
      len = @pos - slice_begin
      previous = @string.byteslice(slice_begin, len)
      previous = "...#{previous}" if @pos > 5
      result << previous.b.inspect
      result << " "
    end

    result << "@ "

    slice_end = @pos + 5
    slice_end = @string.bytesize if slice_end > @string.bytesize
    if @pos < slice_end
      len = slice_end - @pos
      following = @string.byteslice(@pos, len)
      following = "#{following}..." if @string.bytesize - @pos > 5
      result << following.b.inspect
    end

    result << ">"

    result
  end

  def match?(pattern)
    haystack = @string.byteslice(@pos, @string.bytesize - @pos)
    match = pattern.match(haystack)
    if match.nil? || match.begin(0).positive?
      @last_match = nil
      @last_match_pos = nil
      return nil
    end

    @last_match = match
    @last_match_pos ||= 0

    @last_match_pos += haystack[0, match.end(0)].bytesize

    match.end(0) - match.begin(0)
  end

  def matched
    return nil if @last_match.nil?

    ret = @last_match[0]
    ret = String.new(ret) unless ret.instance_of?(String)
    ret
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

    @string.byteslice(pos, len)
  end

  def peep(len)
    warn 'peep is obsolete use peek instead' if $VERBOSE
    peek(len)
  end

  attr_reader :pos # rubocop:disable Style/AccessorGrouping
  alias pointer pos

  # rubocop:disable Lint/Void
  def pos=(pointer)
    raise RangeError unless pointer.abs < @string.bytesize

    pointer = @string.bytesize + pointer if pointer.negative?
    @pos = @string.byteslice(0, pointer).bytesize
    pointer
  end
  alias pointer= pos=
  # rubocop:enable Lint/Void

  def post_match
    return nil if @last_match.nil?

    ret = @string.byteslice(@last_match_pos, @string.bytesize - @last_match_pos) || ''
    ret = String.new(ret) unless ret.instance_of?(String)
    ret
  end

  def pre_match
    return nil if @last_match.nil?

    match_byte_offset =
      if @last_match.is_a?(MatchData)
        match_char_len = @last_match.end(0) - @last_match.begin(0)
        @last_match.string[@last_match.begin(0), match_char_len].bytesize
      else
        @last_match.bytesize
      end
    ret = @string.byteslice(0, @last_match_pos - match_byte_offset) || ''
    ret = String.new(ret) unless ret.instance_of?(String)
    ret
  end

  def reset
    @pos = 0
    @previous_pos = nil
    @last_match = nil
    @last_match_pos = nil
  end

  def rest
    ret = @string.byteslice(@pos, @string.bytesize - @pos)
    ret = String.new(ret) unless ret.instance_of?(String)
    ret
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

    previous_pos = @pos
    haystack = @string.byteslice(@pos, @string.bytesize - @pos)
    match = pattern.match(haystack)

    if match.nil? || match.begin(0).positive?
      @last_match = nil
      @last_match_pos = nil
      @previous_pos = nil
      return nil
    end

    match_end_byte_pos = haystack[0, match.end(0)].bytesize
    @pos += match_end_byte_pos if advance_pointer_p
    @previous_pos = previous_pos
    @last_match = match
    @last_match_pos = @pos

    if return_string_p
      ret = @string.byteslice(previous_pos, match_end_byte_pos)
      ret = String.new(ret) unless ret.instance_of?(String)
      ret
    else
      match.end(0)
    end
  end

  def scan_until(pattern)
    previous_pos = @pos
    haystack = @string.byteslice(@pos, @string.bytesize - @pos)
    match = pattern.match(haystack)
    return nil if match.nil?

    match_end_byte_pos = haystack[0, match.end(0)].bytesize
    @pos += match_end_byte_pos
    @previous_pos = previous_pos
    @last_match = match
    @last_match_pos = @pos

    @string.byteslice(previous_pos, match_end_byte_pos)
  end

  def search_full(pattern, advance_pointer_p, return_string_p)
    previous_pos = @pos
    haystack = @string.byteslice(@pos, @string.bytesize - @pos)
    match = pattern.match(haystack)
    return nil if match.nil?

    match_end_byte_pos = haystack[0, match.end(0)].bytesize
    @pos += match_end_byte_pos if advance_pointer_p
    @previous_pos = previous_pos
    if return_string_p
      @string.byteslice(previous_pos, match_end_byte_pos)
    else
      match.end(0)
    end
  end

  def size
    @last_match.size
  end

  def skip(pattern)
    previous_pos = @pos
    haystack = @string.byteslice(@pos, @string.bytesize - @pos)
    match = pattern.match(haystack)
    if match.nil? || match.begin(0).positive?
      @last_match = nil
      @last_match_pos = nil
      @previous_pos = nil
      return nil
    end

    match_end_byte_pos = haystack[0, match.end(0)].bytesize
    @pos += match_end_byte_pos
    @previous_pos = previous_pos
    @last_match = match
    @last_match_pos = @pos
    match.end(0)
  end

  def skip_until(pattern)
    previous_pos = @pos
    haystack = @string.byteslice(@pos, @string.bytesize - @pos)
    match = pattern.match(haystack)
    if match.nil?
      @last_match = nil
      @last_match_pos = nil
      @previous_pos = nil
      return nil
    end

    match_end_byte_pos = haystack[0, match.end(0)].bytesize
    @pos += match_end_byte_pos
    @previous_pos = previous_pos
    @last_match = match
    @last_match_pos = @pos
    match.end(0)
  end

  attr_reader :string # rubocop:disable Style/AccessorGrouping

  # rubocop:disable Lint/Void
  def string=(str)
    s = str
    s = String.try_convert(str) unless str.is_a?(String)

    @string = s
    reset

    str
  end
  # rubocop:enable Lint/Void

  def terminate
    @pos = @string.bytesize
    @last_match = nil
    @last_match_pos = nil
    self
  end

  def unscan
    raise ScanError, 'unscan failed: previous match record not exist' if @previous_pos.nil?

    @pos = @previous_pos
    @previous_pos = nil
    @last_match = nil
    @last_match_pos = nil
    nil
  end

  def values_at(*args)
    return nil if @last_match.nil?

    args.map { |index| @last_match[index] }
  end
end
