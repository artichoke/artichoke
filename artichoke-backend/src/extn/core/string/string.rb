# frozen_string_literal: true

module Artichoke
  class String
    def self.tr_expand_str(str)
      arr = []
      str_a = str.chars
      i = 0
      while i < str_a.length
        if str_a[i + 1] == '-' && !str_a[i + 2].nil?
          range = (str_a[i]..str_a[i + 2]).to_a
          raise ArgumentError if range.empty?

          range[1] = range[0] if range.length == 1
          range.each do |c|
            arr.push(c)
          end
          i += 3
        else
          arr.push(str_a[i])
          i += 1
        end
      end
      arr
    end

    def self.tr_compose_str(src, container, from, to, skip_double, index_check, retrieve_value)
      previous_char = ''
      src.chars.map do |c|
        index = from.index(c)
        if index_check.call(index)
          proposed_replacement_char = retrieve_value.call(to, index)
          replacement_char = proposed_replacement_char
          replacement_char = '' if skip_double && proposed_replacement_char == previous_char
          previous_char = proposed_replacement_char
          container << replacement_char
        else
          previous_char = ''
          container << c
        end
      end.join
    end

    def self.tr(src, from_str, to_str, skip_double)
      from_str = from_str.to_str
      to_str = to_str.to_str

      result = src.class.new

      if from_str.start_with?('^') && from_str.length > 1
        from_str = from_str[1..-1]
        from = tr_expand_str(from_str)
        to = tr_expand_str(to_str)
        tr_compose_str(src, result, from, to, skip_double, ->(index) { index.nil? }, ->(lookup, _) { lookup.last })
      else
        from = tr_expand_str(from_str)
        to = tr_expand_str(to_str)
        tr_compose_str(src, result, from, to, skip_double, ->(index) { !index.nil? }, ->(lookup, index) { lookup[index] || lookup.last || '' })
      end
      result
    end
  end
end

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

class String
  include Comparable

  def self.try_convert(obj = nil)
    raise ArgumentError if obj.nil?
    return obj if obj.is_a?(String)

    str = obj.to_str
    return nil if str.nil?
    raise TypeError unless str.is_a?(String)

    str
  rescue NoMethodError
    nil
  end

  def %(other)
    if other.is_a?(Array)
      sprintf(self, *other) # rubocop:disable Style/FormatString
    else
      sprintf(self, other) # rubocop:disable Style/FormatString
    end
  end

  def +@
    return dup if frozen?

    self
  end

  def -@
    return self if frozen?

    dup.freeze
  end

  def =~(other)
    return other.match(self)&.begin(0) if other.is_a?(Regexp)
    raise TypeError, "type mismatch: #{other.class} given" if other.is_a?(String)
    return other =~ self if other.respond_to?(:=~)

    nil
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
    raise TypeError, "no implicit conversion of #{prefix.class} into String" unless prefix.is_a?(String)

    return self[prefix.length..-1] if start_with?(prefix)

    dup
  end

  def delete_prefix!(prefix)
    replaced = delete_prefix(prefix)
    self[0..-1] = replaced unless self == replaced
  end

  def delete_suffix(suffix)
    raise TypeError, "no implicit conversion of #{suffix.class} into String" unless suffix.is_a?(String)

    return self[0..-suffix.length] if end_with?(suffix)

    dup
  end

  def delete_suffix!(prefix)
    replaced = delete_suffix(prefix)
    self[0..-1] = replaced unless self == replaced
  end

  def dump
    raise NotImplementedError
  end

  def each_byte(&block)
    return to_enum(:each_byte, &block) unless block

    bytes = self.bytes
    pos = 0
    while pos < bytes.size
      block.call(bytes[pos])
      pos += 1
    end
    self
  end

  def each_char(&block)
    return to_enum(:each_char, &block) unless block

    chars = self.chars
    pos = 0
    while pos < chars.size
      block.call(chars[pos])
      pos += 1
    end
    self
  end

  def each_codepoint
    return to_enum(:each_codepoint) unless block_given?

    codepoints = self.codepoints
    pos = 0
    while pos < codepoints.size
      block.call(codepoints[pos])
      pos += 1
    end
    self
  end

  def each_grapheme_cluster
    raise NotImplementedError
  end

  def each_line(separator = $/, getline_args = nil) # rubocop:disable Style/SpecialGlobalVars
    return to_enum(:each_line, separator, getline_args) unless block_given?

    if separator.nil?
      yield self
      return self
    end
    raise TypeError if separator.is_a?(Symbol)
    raise TypeError if (separator = String.try_convert(separator)).nil?

    paragraph_mode = false
    if separator.empty?
      paragraph_mode = true
      separator = "\n\n"
    end
    start = 0
    string = dup
    self_len = length
    sep_len = separator.length
    should_yield_subclass_instances = self.class != String

    while (pointer = string.index(separator, start))
      pointer += sep_len
      pointer += 1 while paragraph_mode && string[pointer] == "\n"
      if should_yield_subclass_instances
        yield self.class.new(string[start, pointer - start])
      else
        yield string[start, pointer - start]
      end
      start = pointer
    end
    return self if start == self_len

    if should_yield_subclass_instances
      yield self.class.new(string[start, self_len - start])
    else
      yield string[start, self_len - start]
    end
    self
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
    # stub and is here for compatibility.
    Encoding::UTF_8
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

  def grapheme_clusters
    each_grapheme_cluster.to_a
  end

  # TODO: Support backrefs
  #
  #   "hello".gsub(/([aeiou])/, '<\1>')             #=> "h<e>ll<o>"
  #   "hello".gsub(/(?<foo>[aeiou])/, '{\k<foo>}')  #=> "h{e}ll{o}"
  def gsub(pattern, replacement = nil)
    return to_enum(:gsub, pattern, replacement) if replacement.nil? && !block_given?

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
    insert(0, args.join)
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

  def scrub
    # TODO: This is a stub. Implement scrub correctly.
    self
  end

  def scrub!
    # TODO: This is a stub. Implement scrub! correctly.
    self
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
    return to_enum(:sub, pattern, replacement) if replacement.nil? && !block_given?

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
    Artichoke::String.tr(self, from_str, to_str, false)
  end

  def tr!(from_str, to_str)
    raise FrozenError if frozen?

    replaced = tr(from_str, to_str)
    self[0..-1] = replaced unless self == replaced
  end

  def tr_s(from_str, to_str)
    Artichoke::String.tr(self, from_str, to_str, true)
  end

  def tr_s!(from_str, to_str)
    raise FrozenError if frozen?

    replaced = tr_s(from_str, to_str)
    self[0..-1] = replaced unless self == replaced
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

  def upto(max, exclusive = false, &block) # rubocop:disable Style/OptionalBooleanParameter
    return to_enum(:upto, max, exclusive) unless block
    raise TypeError, "no implicit conversion of #{max.class} into String" unless max.is_a?(String)

    len = length
    maxlen = max.length
    # single character
    if len == 1 && maxlen == 1
      c = ord
      e = max.ord
      while c <= e
        break if exclusive && c == e

        yield c.chr
        c += 1
      end
      return self
    end
    # both edges are all digits
    bi = to_i(10)
    ei = max.to_i(10)
    if (bi.positive? || bi == '0' * len) && (ei.positive? || ei == '0' * maxlen)
      while bi <= ei
        break if exclusive && bi == ei

        s = bi.to_s
        s = s.rjust(len, '0') if s.length < len

        yield s
        bi += 1
      end
      return self
    end
    bs = self
    loop do
      n = (bs <=> max)
      break if n.positive?
      break if exclusive && n.zero?

      yield bs
      break if n.zero?

      bs = bs.succ
    end
    self
  end
end
