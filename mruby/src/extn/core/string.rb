# frozen_string_literal: true

class Encoding
  ASCII_8BIT = new('ASCII-8BIT')
  US_ASCII = new('US-ASCII')
  UTF_8 = new('UTF-8')

  def self.find(string)
    new(string)
  end

  attr_reader :name

  def initialize(name)
    @name = name
  end

  def ascii_compatible?
    true
  end

  def dummy?
    true
  end

  def inspect
    "#<#{self.class}:#{@name}"
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

class String
  def self.try_convert(obj = nil)
    raise ArgumentError if obj.nil?

    obj.to_str
  rescue StandardError
    nil
  end

  def +@
    return dup if frozen?

    self
  end

  def -@
    return self if frozen?

    dup.freeze
  end

  def <<(obj)
    obj = obj.chr if obj.is_a?(Integer)

    self[0..-1] = "#{self}#{obj}"
    self
  end
  alias concat <<

  def =~(other)
    return other.match(self)&.begin(0) if other.is_a?(Regexp)
    raise TypeError, "type mismatch: #{other.class} given" if other.is_a?(String)
    return other =~ self if other.respond_to?(:=~)

    nil
  end

  alias __old_element_reference []
  def [](*args)
    return __old_element_reference(*args) unless args[0].is_a?(Regexp)

    regexp = args[0]
    capture = args.fetch(1, 0)
    regexp.match(self)&.[](capture)
  end

  alias __old_element_assignment []=
  def []=(*args)
    return __old_element_assignment(*args) unless args[0].is_a?(Regexp) # rubocop:disable Lint/ReturnInVoidContext

    *args, replace = *args
    regexp = args[0]
    capture = args.fetch(1, 0)
    match = regexp.match(self)
    return if match.nil?

    self[match.begin(capture)...match.end(capture)] = replace
  end

  def ascii_only?
    bytes.length == length
  end

  def b
    # mruby has no Encoding, so there is no difference between an ASCII_8BIT
    # String and a UTF-8 String.
    dup
  end

  def byteslice(*args)
    if args[0].is_a?(Integer)
      position, len = *args
      len = 1 if len.nil?
      position = length + position if position.negative?

      slice = bytes[position...position + len]
      slice.pack('c*')
    elsif args.length == 1 && args[0].is_a?(Range)
      range, = *args
      position = range.begin
      len = range.size

      slice = bytes[position...position + len]
      slice.pack('c*')
    else
      raise ArgumentError
    end
  end

  def casecmp(str)
    return nil unless String.try_convert(str)

    downcase <=> str.downcase
  end

  def casecmp?(str)
    casecmp(str)&.zero? == true
  end

  def center(width, padstr = ' ')
    return self if length >= width

    left_pad = (width - length) / 2
    left_pad = (padstr * left_pad)[0...left_pad]
    right_pad = (width - length) / 2 + (width - length) % 2
    right_pad = (padstr * right_pad)[0...right_pad]
    "#{left_pad}#{self}#{right_pad}"
  end

  def chars
    if block_given?
      split('').each do |char|
        yield char
      end
      self
    else
      split('')
    end
  end

  def chr
    dup[0]
  end

  def clear
    self[0..-1] = ''
  end

  def codepoints
    each_codepoint.to_a
  end

  def count
    raise NotImplementedError
  end

  def crypt(_salt)
    raise NotImplementedError
  end

  def delete(*args)
    args.inject(self) { |string, pattern| string.tr(pattern, '') }
  end

  def delete!(*args)
    replaced = delete(*args)
    self[0..-1] = replaced unless self == replaced
  end

  def delete_prefix(prefix)
    self[0...prefix.length] = '' if start_with?(prefix)

    self
  end

  def delete_prefix!(prefix)
    replaced = delete_prefix(prefix)
    self[0..-1] = replaced unless self == replaced
  end

  def delete_suffix(suffix)
    self[-suffix.length..-1] = '' if end_with?(suffix)

    self
  end

  def delete_suffix!(prefix)
    replaced = delete_suffix(prefix)
    self[0..-1] = replaced unless self == replaced
  end

  def dump
    raise NotImplementedError
  end

  def each_codepoint
    return to_enum(:each_codepoint) unless block_given?

    split('').each do |c|
      yield c.ord
    end
  end

  def each_grapheme_cluster
    raise NotImplementedError
  end

  def encode(*_args)
    # mruby does not support encoding, all Strings are UTF-8. This method is a
    # NOOP and is here for compatibility.
    dup
  end

  def encode!(*_args)
    # mruby does not support encoding, all Strings are UTF-8. This method is a
    # NOOP and is here for compatibility.
    self
  end

  def encoding
    # mruby does not support encoding, all Strings are UTF-8. This method is a
    # NOOP and is here for compatibility.
    nil
  end

  def end_with?(*suffixes)
    suffixes.each do |suffix|
      return true if self[-suffix.length..-1] == suffix
    end
    false
  end

  def force_encoding(*_args)
    # mruby does not support encoding, all Strings are UTF-8. This method is a
    # NOOP and is here for compatibility.
    self
  end

  def getbyte(index)
    bytes[index]
  end

  def grapheme_clusters
    each_grapheme_cluster.to_a
  end

  # TODO: Support backrefs
  #
  #   "hello".gsub(/([aeiou])/, '<\1>')             #=> "h<e>ll<o>"
  #   "hello".gsub(/(?<foo>[aeiou])/, '{\k<foo>}')  #=> "h{e}ll{o}"
  def gsub(pattern, replacement = nil)
    return to_enum(:gsub, pattern) if replacement.nil? && !block_given?

    replace =
      if replacement.nil?
        ->(old) { (yield old).to_s }
      elsif replacement.is_a?(Hash)
        ->(old) { replacement[old].to_s }
      else
        ->(_old) { replacement.to_s }
      end
    pattern = Regexp.compile(Regexp.escape(pattern)) if pattern.is_a?(String)
    match = pattern.match(self)
    return dup if match.nil?

    buf = ''
    remainder = dup
    until match.nil? || remainder.empty?
      buf << remainder[0..match.begin(0) - 1] if match.begin(0).positive?
      buf << replace.call(match[0])
      remainder = remainder[match.end(0)..-1]
      remainder = remainder[1..-1] if match.begin(0) == match.end(0)
      match = pattern.match(remainder)
    end
    buf << remainder
  end

  def gsub!(pattern, replacement = nil, &blk)
    replaced = gsub(pattern, replacement, &blk)
    self[0..-1] = replaced unless self == replaced
    self
  end

  def hex
    raise NotImplementedError
  end

  def insert(index, other_str)
    return self << other_str if index == -1

    index += 1 if index.negative?

    self[index, 0] = other_str
    self
  end

  def lines(*args)
    each_line(*args).to_a
  end

  def ljust(integer, padstr = ' ')
    raise ArgumentError, 'zero width padding' if padstr == ''

    return self if integer <= length

    pad_repetitions = (integer / padstr.length).ceil
    padding = (padstr * pad_repetitions)[0...(integer - length)]
    "#{self}#{padding}"
  end

  def lstrip
    strip_pointer = 0
    string_end = length - 1
    strip_pointer += 1 while strip_pointer <= string_end && " \f\n\r\t\v".include?(self[strip_pointer])
    return '' if string_end.zero?

    dup[strip_pointer..string_end]
  end

  def lstrip!
    replaced = lstrip
    self[0..-1] = replaced unless self == replaced
  end

  def match(pattern, pos = 0)
    pattern = Regexp.compile(Regexp.escape(pattern)) if pattern.is_a?(String)

    pattern.match(self[pos..-1])
  end

  def match?(pattern, pos = 0)
    pattern = Regexp.compile(Regexp.escape(pattern)) if pattern.is_a?(String)

    # TODO: Don't set $~ and other Regexp globals
    pattern.match?(self[pos..-1])
  end

  def next
    raise NotImplementedError
  end
  alias succ next

  def next!
    raise NotImplementedError
  end
  alias succ! next!

  def oct
    raise NotImplementedError
  end

  def partition(pattern)
    pattern = Regexp.compile(Regexp.escape(pattern)) if pattern.is_a?(String)

    match = pattern.match(self)
    [match.pre_match, match[0], match.post_match]
  end

  def prepend(*args)
    insert(0, args.join(''))
  end

  def rjust(integer, padstr = ' ')
    raise ArgumentError, 'zero width padding' if padstr == ''

    return self if integer <= length

    pad_repetitions = (integer / padstr.length).ceil
    padding = (padstr * pad_repetitions)[0...(integer - length)]
    "#{padding}#{self}"
  end

  def rpartition(pattern)
    pattern = Regexp.compile(Regexp.escape(pattern)) if pattern.is_a?(String)

    _ = pattern
    raise NotImplementedError
  end

  def rstrip
    strip_pointer = length - 1
    string_start = 0
    strip_pointer -= 1 while strip_pointer >= string_start && " \f\n\r\t\v".include?(self[strip_pointer])
    return '' if strip_pointer.zero?

    dup[string_start..strip_pointer]
  end

  def rstrip!
    replaced = rstrip
    self[0..-1] = replaced unless self == replaced
  end

  def scan(pattern)
    pattern = Regexp.compile(Regexp.escape(pattern)) if pattern.is_a?(String)
    raise TypeError, "wrong argument type #{pattern.inspect} (expected Regexp)" if pattern.nil?
    raise TypeError, "wrong argument type #{pattern.class.name} (expected Regexp)" unless pattern.is_a?(Regexp)

    remainder = dup
    match = pattern.match(remainder)
    collect = []
    until remainder.empty? || match.nil?
      collect <<
        if block_given?
          yield match[0]
        else
          match[0]
        end
      remainder = remainder[match.end(0)..-1]
      remainder = remainder[1..-1] if match.begin(0) == match.end(0)
      match = nil
      match = pattern.match(remainder) unless remainder.nil?
    end
    collect
  end

  def scrub
    # TODO: This is a stub. Implement scrub correctly.
    self
  end

  def scrub!
    # TODO: This is a stub. Implement scrub! correctly.
    self
  end

  def setbyte(index, integer)
    slice = bytes
    slice[index] = integer
    self[0..-1] = slice.pack('c*')
  end

  def split(pattern, limit = nil)
    parts = []
    return parts if self == ''

    pattern = Regexp.compile(Regexp.escape(pattern)) if pattern.is_a?(String)
    return length.times.map { |i| self[i].dup } if pattern.to_s == ''

    remainder = dup
    match = pattern.match(remainder)
    if limit&.positive?
      until match.nil? || remainder.nil? || parts.length >= limit - 1
        parts << remainder[0...match.begin(0)]
        remainder = remainder[match.end(0)..-1]
        remainder = remainder[1..-1] if match.begin(0) == match.end(0)
        match = nil
        match = pattern.match(remainder) unless remainder.nil?
      end
      parts << remainder unless remainder.nil?
    else
      until match.nil? || remainder.nil?
        parts << remainder[0...match.begin(0)]
        remainder = remainder[match.end(0)..-1]
        remainder = remainder[1..-1] if match.begin(0) == match.end(0)
        match = nil
        match = pattern.match(remainder) unless remainder.nil?
      end
      parts << remainder unless remainder.nil?
      if limit&.negative? && -limit > parts.length
        (-limit - parts.length).times do
          parts << ''
        end
      end
    end
    parts.each { |part| yield part } if block_given?

    parts
  end

  def squeeze(*_args)
    raise NotImplementedError
  end

  def start_with?(*prefixes)
    prefixes.each do |prefix|
      return true if self[0...prefix.length] == prefix
    end
    false
  end

  def strip
    result = lstrip
    result = self if result.nil?
    result.rstrip
  end

  def strip!
    replaced = strip
    self[0..-1] = replaced unless self == replaced
  end

  def sub(pattern, replacement = nil)
    return to_enum(:sub, pattern) if replacement.nil? && !block_given?

    replace =
      if replacement.nil?
        ->(old) { (yield old).to_s }
      elsif replacement.is_a?(Hash)
        ->(old) { replacement[old].to_s }
      else
        ->(_old) { replacement.to_s }
      end
    pattern = Regexp.compile(Regexp.escape(pattern)) if pattern.is_a?(String)
    match = pattern.match(self)
    return dup if match.nil?

    buf = ''
    remainder = dup
    buf << remainder[0..match.begin(0) - 1] if match.begin(0).positive?
    buf << replace.call(match[0])
    remainder = remainder[match.end(0)..-1]
    remainder = remainder[1..-1] if match.begin(0) == match.end(0)
    buf << remainder
    buf
  end

  def sub!(pattern, replacement = nil, &blk)
    replaced = sub(pattern, replacement, &blk)
    self[0..-1] = replaced unless self == replaced
  end

  def sum
    raise NotImplementedError
  end

  def swapcase(*_args)
    raise NotImplementedError
  end

  def swapcase!(*_args)
    raise NotImplementedError
  end

  def to_c
    raise NotImplementedError
  end

  def to_r
    raise NotImplementedError
  end

  def to_str
    dup
  end

  def tr(from_str, to_str)
    # TODO: Support character ranges c1-c2
    # TODO: Support backslash escapes
    to_str = to_str.rjust(from_str.length, to_str[-1]) if to_str.length.positive?

    gsub(Regexp.compile("[#{from_str}]")) do |char|
      to_str[from_str.index(char)] || ''
    end
  end

  def tr!(from_str, to_str)
    replaced = tr(from_str, to_str)
    self[0..-1] = replaced unless self == replaced
  end

  def tr_s(_from_str, _to_str)
    # TODO: Support character ranges c1-c2
    # TODO: Support backslash escapes
    raise NotImplementedError
  end

  def tr_s!(_from_str, _to_str)
    # TODO: Support character ranges c1-c2
    # TODO: Support backslash escapes
    raise NotImplementedError
  end

  def undump
    raise NotImplementedError
  end

  def unicode_normalize(_form = :nfc)
    raise NotImplementedError
  end

  def unicode_normalize!(_form = :nfc)
    raise NotImplementedError
  end

  def unicode_normalized?(_form = :nfc)
    raise NotImplementedError
  end

  def upto(_other_str, _exclusive = false)
    raise NotImplementedError
  end

  def valid_encoding
    # mruby does not support encoding, all Strings are UTF-8. This method is a
    # NOOP and is here for compatibility.
    true
  end
end
