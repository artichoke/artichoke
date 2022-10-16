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

    def self.implicit_conversion(target)
      return target if target.is_a?(::String)
      raise TypeError, 'no implicit conversion of nil into String' if target.nil?
      raise TypeError, 'no implicit conversion of Symbol into String' if target.is_a?(Symbol)

      converted = target.to_str
      return converted if converted.is_a?(::String)

      inspect_name = target.class.name
      inspect_name = target.inspect if target.is_a?(TrueClass) || target.is_a?(FalseClass) || target.nil?
      message = "can't convert #{inspect_name} to String (#{inspect_name}#to_str gives #{converted.class})"
      raise TypeError, message
    rescue NoMethodError
      inspect_name = target.class.name
      inspect_name = target.inspect if target.is_a?(TrueClass) || target.is_a?(FalseClass) || target.nil?
      message = "no implicit conversion of #{inspect_name} into String"
      raise TypeError, message
    end
  end
end

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

# https://ruby-doc.org/core-3.0.2/String.html
class String
  include Comparable

  # https://ruby-doc.org/core-3.0.2/String.html#method-c-new
  #
  # NOTE: Implemented in native code.
  #
  # def self.new(string, **kwargs); end

  # https://ruby-doc.org/core-3.0.2/String.html#method-c-try_convert
  def self.try_convert(obj)
    return nil if obj.nil?
    return obj if obj.is_a?(String)

    str = obj.to_str
    return nil if str.nil?
    return str if str.is_a?(String)

    raise TypeError, "can't convert #{obj.class} to String (#{obj.class}#to_str gives #{str.class})"
  rescue NoMethodError
    nil
  end

  # https://ruby-doc.org/core-3.0.2/String.html#method-i-25
  def %(other)
    if other.is_a?(Array)
      sprintf(self, *other) # rubocop:disable Style/FormatString
    else
      sprintf(self, other) # rubocop:disable Style/FormatString
    end
  end

  # https://ruby-doc.org/core-3.0.2/String.html#method-i-2A
  #
  # NOTE: Implemented in native code.
  #
  # def *(integer); end

  # https://ruby-doc.org/core-3.0.2/String.html#method-i-2B
  #
  # NOTE: Implemented in native code.
  #
  # def +(other_string); end

  # https://ruby-doc.org/core-3.0.2/String.html#method-i-2B-40
  def +@
    return dup if frozen?

    self
  end

  # https://ruby-doc.org/core-3.0.2/String.html#method-i-2D-40
  def -@
    # TODO: check to see if the string does not have any ivars defined on it.
    return self if frozen?

    dup.freeze
  end

  # https://ruby-doc.org/core-3.0.2/String.html#method-i-2F
  alias / split

  # https://ruby-doc.org/core-3.0.2/String.html#method-i-3C-3C
  #
  # NOTE: Implemented in native code.
  #
  # def <<(object); end

  # https://ruby-doc.org/core-3.0.2/String.html#method-i-3C-3D-3E
  #
  # NOTE: Implemented in native code.
  #
  # def <=>(other_string); end

  # https://ruby-doc.org/core-3.0.2/String.html#method-i-3D-3D
  #
  # NOTE: Implemented in native code.
  #
  # def ==(other_string); end

  # https://ruby-doc.org/core-3.0.2/String.html#method-i-3D-3D-3D
  alias === ==

  # https://ruby-doc.org/core-3.0.2/String.html#method-i-3D-7E
  def =~(other)
    # TODO: This implementation does not "also updates Regexp-related global
    # variables" like MRI does.
    return other.match(self)&.begin(0) if other.is_a?(Regexp)
    raise TypeError, "type mismatch: #{other.class} given" if other.is_a?(String)
    return other =~ self if other.respond_to?(:=~)

    nil
  end

  # https://ruby-doc.org/core-3.0.2/String.html#method-i-5B-5D
  #
  # NOTE: Implemented in native code.
  #
  # def [](*args); end

  # https://ruby-doc.org/core-3.0.2/String.html#method-i-5B-5D-3D
  #
  # NOTE: Implemented in native code.
  #
  # def []=(*args); end

  # https://ruby-doc.org/core-3.0.2/String.html#method-i-ascii_only-3F
  #
  # NOTE: Implemented in native code.
  #
  # def ascii_only?; end

  # https://ruby-doc.org/core-3.0.2/String.html#method-i-b
  #
  # NOTE: Implemented in native code.
  #
  # def b; end

  # https://ruby-doc.org/core-3.0.2/String.html#method-i-bytes
  #
  # NOTE: Implemented in native code.
  #
  # def bytes; end

  # https://ruby-doc.org/core-3.0.2/String.html#method-i-bytesize
  #
  # NOTE: Implemented in native code.
  #
  # def bytesize; end

  # https://ruby-doc.org/core-3.0.2/String.html#method-i-byteslice
  #
  # NOTE: Implemented in native code.
  #
  # def bytesize(integer, *args); end

  # https://ruby-doc.org/core-3.0.2/String.html#method-i-capitalize
  #
  # NOTE: Implemented in native code.
  #
  # def capitalize; end

  # https://ruby-doc.org/core-3.0.2/String.html#method-i-capitalize-21
  #
  # NOTE: Implemented in native code.
  #
  # def capitalize!; end

  # https://ruby-doc.org/core-3.0.2/String.html#method-i-casecmp
  #
  # NOTE: Implemented in native code.
  #
  # def casecmp(other_str); end

  # https://ruby-doc.org/core-3.0.2/String.html#method-i-casecmp-3F
  #
  # NOTE: Implemented in native code.
  #
  # def casecmp?(other_string); end

  # https://ruby-doc.org/core-3.0.2/String.html#method-i-center
  #
  # NOTE: Implemented in native code.
  #
  # def center(width, padstr=' '); end

  # https://ruby-doc.org/core-3.0.2/String.html#method-i-chars
  #
  # NOTE: Implemented in native code.
  #
  # def chars; end

  # https://ruby-doc.org/core-3.0.2/String.html#method-i-chomp
  #
  # NOTE: Implemented in native code.
  #
  # def chomp(separator=$/); end

  # https://ruby-doc.org/core-3.0.2/String.html#method-i-chomp-21
  #
  # NOTE: Implemented in native code.
  #
  # def chomp!(separator=$/); end

  # https://ruby-doc.org/core-3.0.2/String.html#method-i-chop
  #
  # NOTE: Implemented in native code.
  #
  # def chop; end

  # https://ruby-doc.org/core-3.0.2/String.html#method-i-chop-21
  #
  # NOTE: Implemented in native code.
  #
  # def chop!; end

  # https://ruby-doc.org/core-3.0.2/String.html#method-i-chr
  #
  # NOTE: Implemented in native code.
  #
  # def chr; end

  # https://ruby-doc.org/core-3.0.2/String.html#method-i-clear
  #
  # NOTE: Implemented in native code.
  #
  # def clear; end

  # https://ruby-doc.org/core-3.0.2/String.html#method-i-codepoints
  #
  # NOTE: Implemented in native code.
  #
  # def codepoints; end

  # https://ruby-doc.org/core-3.0.2/String.html#method-i-concat
  #
  # NOTE: Implemented in native code.
  #
  # def concat(*objects); end

  # https://ruby-doc.org/core-3.0.2/String.html#method-i-count
  def count(other_str, *rest)
    raise NotImplementedError
  end

  # https://ruby-doc.org/core-3.0.2/String.html#method-i-crypt
  def crypt(_salt_str)
    raise NotImplementedError, 'String#crypt uses an insecure algorithm and is deprecated'
  end

  # https://ruby-doc.org/core-3.0.2/String.html#method-i-delete
  def delete(*args)
    args.inject(self) { |string, pattern| Artichoke::String.implicit_conversion(string).tr(pattern, '') }
  end

  # https://ruby-doc.org/core-3.0.2/String.html#method-i-delete-21
  def delete!(*args)
    raise FrozenError, "can't modify frozen String: #{inspect}" if frozen?

    replaced = delete(*args)
    replace(replaced) unless self == replaced
  end

  # https://ruby-doc.org/core-3.0.2/String.html#method-i-delete_prefix
  def delete_prefix(prefix)
    prefix = Artichoke::String.implicit_conversion(prefix)
    return dup if prefix.empty?

    if start_with?(prefix)
      slice = self[prefix.length..-1]
      return slice if instance_of?(String)

      return self.class.new(slice)
    end
    dup
  end

  # https://ruby-doc.org/core-3.0.2/String.html#method-i-delete_prefix-21
  def delete_prefix!(prefix)
    raise FrozenError, "can't modify frozen String: #{inspect}" if frozen?

    replaced = delete_prefix(prefix)
    replace(replaced) unless self == replaced
  end

  # https://ruby-doc.org/core-3.0.2/String.html#method-i-delete_suffix
  def delete_suffix(suffix)
    suffix = Artichoke::String.implicit_conversion(suffix)
    return dup if suffix.empty?

    if end_with?(suffix)
      slice = self[0...-suffix.length]
      return slice if instance_of?(String)

      return self.class.new(slice)
    end
    dup
  end

  # https://ruby-doc.org/core-3.0.2/String.html#method-i-delete_suffix-21
  def delete_suffix!(prefix)
    raise FrozenError, "can't modify frozen String: #{inspect}" if frozen?

    replaced = delete_suffix(prefix)
    replace(replaced) unless self == replaced
  end

  # https://ruby-doc.org/core-3.0.2/String.html#method-i-downcase
  #
  # NOTE: Implemented in native code.
  #
  # def downcase; end

  # https://ruby-doc.org/core-3.0.2/String.html#method-i-downcase-21
  #
  # NOTE: Implemented in native code.
  #
  # def downcase!; end

  # https://ruby-doc.org/core-3.0.2/String.html#method-i-dump
  def dump
    raise NotImplementedError
  end

  # https://ruby-doc.org/core-3.0.2/String.html#method-i-each_byte
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

  # https://ruby-doc.org/core-3.0.2/String.html#method-i-each_char
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

  # https://ruby-doc.org/core-3.0.2/String.html#method-i-each_codepoint
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

  # https://ruby-doc.org/core-3.0.2/String.html#method-i-each_grapheme_cluster
  def each_grapheme_cluster
    raise NotImplementedError
  end

  # https://ruby-doc.org/core-3.0.2/String.html#method-i-each_line
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

  # https://ruby-doc.org/core-3.0.2/String.html#method-i-empty-3F
  #
  # NOTE: Implemented in native code.
  #
  # def empty?; end

  # https://ruby-doc.org/core-3.0.2/String.html#method-i-encode
  #
  # TODO: Properly implement this method now that Artichoke has encoding support.
  def encode(*_args)
    # mruby does not support encoding, all Strings are UTF-8. This method is a
    # NOOP and is here for compatibility.
    dup
  end

  # https://ruby-doc.org/core-3.0.2/String.html#method-i-encode-21
  #
  # TODO: Properly implement this method now that Artichoke has encoding support.
  def encode!(*_args)
    # mruby does not support encoding, all Strings are UTF-8. This method is a
    # NOOP and is here for compatibility.
    self
  end

  # https://ruby-doc.org/core-3.0.2/String.html#method-i-encoding
  #
  # TODO: Properly implement this method now that Artichoke has encoding support.
  def encoding
    # mruby does not support encoding, all Strings are UTF-8. This method is a
    # stub and is here for compatibility.
    Encoding::UTF_8
  end

  # https://ruby-doc.org/core-3.0.2/String.html#method-i-end_with-3F
  #
  # NOTE: Implemented in native code.
  #
  # def end_with?; end

  # https://ruby-doc.org/core-3.0.2/String.html#method-i-eql-3F
  #
  # NOTE: Implemented in native code.
  #
  # def eql?(object); end

  # https://ruby-doc.org/core-3.0.2/String.html#method-i-force_encoding
  #
  # TODO: Properly implement this method now that Artichoke has encoding support.
  def force_encoding(_encoding)
    # mruby does not support encoding, all Strings are UTF-8. This method is a
    # NOOP and is here for compatibility.
    self
  end

  # https://ruby-doc.org/core-3.0.2/String.html#method-i-freeze
  #
  # NOTE: Implemented in native code.
  #
  # def freeze(); end

  # https://ruby-doc.org/core-3.0.2/String.html#method-i-getbyte
  #
  # NOTE: Implemented in native code.
  #
  # def getbyte(index); end

  # https://ruby-doc.org/core-3.0.2/String.html#method-i-grapheme_clusters
  def grapheme_clusters
    each_grapheme_cluster.to_a
  end

  # https://ruby-doc.org/core-3.0.2/String.html#method-i-gsub
  #
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

  # https://ruby-doc.org/core-3.0.2/String.html#method-i-gsub-21
  def gsub!(pattern, replacement = nil, &blk)
    raise FrozenError, "can't modify frozen String: #{inspect}" if frozen?

    replaced = gsub(pattern, replacement, &blk)
    replace(replaced) unless self == replaced
  end

  # https://ruby-doc.org/core-3.0.2/String.html#method-i-hash
  #
  # NOTE: Implemented in native code.
  #
  # def hash; end

  # https://ruby-doc.org/core-3.0.2/String.html#method-i-hex
  def hex
    raise NotImplementedError
  end

  # https://ruby-doc.org/core-3.0.2/String.html#method-i-include-3F
  #
  # NOTE: Implemented in native code.
  #
  # def include? other_str; end

  # https://ruby-doc.org/core-3.0.2/String.html#method-i-index
  #
  # NOTE: Implemented in native code.
  #
  # def index(*args); end

  # https://ruby-doc.org/core-3.0.2/String.html#method-i-initialize_copy
  #
  # NOTE: Implemented in native code.
  #
  # def replace(other_str); end

  # https://ruby-doc.org/core-3.0.2/String.html#method-i-insert
  def insert(index, other_str)
    return self << other_str if index == -1

    index += 1 if index.negative?

    self[index, 0] = other_str
    self
  end

  # https://ruby-doc.org/core-3.0.2/String.html#method-i-inspect
  #
  # NOTE: Implemented in native code.
  #
  # def inspect; end

  # https://ruby-doc.org/core-3.0.2/String.html#method-i-intern
  #
  # NOTE: Implemented in native code.
  #
  # def intern; end

  # https://ruby-doc.org/core-3.0.2/String.html#method-i-length
  #
  # NOTE: Implemented in native code.
  #
  # def length; end

  # https://ruby-doc.org/core-3.0.2/String.html#method-i-lines
  def lines(*args)
    each_line(*args).to_a
  end

  # https://ruby-doc.org/core-3.0.2/String.html#method-i-ljust
  def ljust(integer, padstr = ' ')
    raise ArgumentError, 'zero width padding' if padstr == ''

    return self if integer <= length

    pad_repetitions = (integer / padstr.length).ceil
    padding = (padstr * pad_repetitions)[0...(integer - length)]
    "#{self}#{padding}"
  end

  # https://ruby-doc.org/core-3.0.2/String.html#method-i-lstrip
  def lstrip
    strip_pointer = 0
    string_end = length - 1

    # Whitespace is defined as any of the following characters:
    #
    # - null
    # - horizontal tab
    # - line feed
    # - vertical tab
    # - form feed
    # - carriage return
    # - space
    strip_pointer += 1 while strip_pointer <= string_end && "\x00\t\n\v\f\r ".include?(self[strip_pointer])
    return '' if string_end.zero?

    dup[strip_pointer..string_end]
  end

  # https://ruby-doc.org/core-3.0.2/String.html#method-i-lstrip-21
  def lstrip!
    raise FrozenError, "can't modify frozen String: #{inspect}" if frozen?

    replaced = lstrip
    replace(replaced) unless self == replaced
  end

  # https://ruby-doc.org/core-3.0.2/String.html#method-i-match
  def match(pattern, pos = 0)
    pattern = Regexp.compile(Regexp.escape(pattern)) if pattern.is_a?(String)

    pattern.match(self[pos..-1])
  end

  # https://ruby-doc.org/core-3.0.2/String.html#method-i-match-3F
  def match?(pattern, pos = 0)
    pattern = Regexp.compile(Regexp.escape(pattern)) if pattern.is_a?(String)

    # TODO: Don't set $~ and other Regexp globals
    pattern.match?(self[pos..-1])
  end

  # https://ruby-doc.org/core-3.0.2/String.html#method-i-next
  def next
    raise NotImplementedError
  end
  alias succ next

  # https://ruby-doc.org/core-3.0.2/String.html#method-i-next-21
  def next!
    raise FrozenError, "can't modify frozen String: #{inspect}" if frozen?

    raise NotImplementedError
  end
  alias succ! next!

  # https://ruby-doc.org/core-3.0.2/String.html#method-i-oct
  def oct
    raise NotImplementedError
  end

  # https://ruby-doc.org/core-3.0.2/String.html#method-i-ord
  #
  # NOTE: Implemented in native code.
  #
  # def ord; end

  # https://ruby-doc.org/core-3.0.2/String.html#method-i-partition
  def partition(pattern)
    pattern = Regexp.compile(Regexp.escape(pattern)) if pattern.is_a?(String)

    match = pattern.match(self)
    [match.pre_match, match[0], match.post_match]
  end

  # https://ruby-doc.org/core-3.0.2/String.html#method-i-prepend
  def prepend(*args)
    insert(0, args.join)
  end

  # https://ruby-doc.org/core-3.0.2/String.html#method-i-replace
  #
  # NOTE: Implemented in native code.
  #
  # def replace(other_str); end

  # https://ruby-doc.org/core-3.0.2/String.html#method-i-reverse
  #
  # NOTE: Implemented in native code.
  #
  # def reverse; end

  # https://ruby-doc.org/core-3.0.2/String.html#method-i-reverse-21
  #
  # NOTE: Implemented in native code.
  #
  # def reverse!; end

  # https://ruby-doc.org/core-3.0.2/String.html#method-i-rindex
  #
  # NOTE: Implemented in native code.
  #
  # def rindex(*args); end

  # https://ruby-doc.org/core-3.0.2/String.html#method-i-rjust
  def rjust(integer, padstr = ' ')
    raise ArgumentError, 'zero width padding' if padstr == ''

    return self if integer <= length

    pad_repetitions = (integer / padstr.length).ceil
    padding = (padstr * pad_repetitions)[0...(integer - length)]
    "#{padding}#{self}"
  end

  # https://ruby-doc.org/core-3.0.2/String.html#method-i-rpartition
  def rpartition(pattern)
    pattern = Regexp.compile(Regexp.escape(pattern)) if pattern.is_a?(String)

    _ = pattern
    raise NotImplementedError
  end

  # https://ruby-doc.org/core-3.0.2/String.html#method-i-rstrip
  def rstrip
    strip_pointer = length - 1
    string_start = 0

    # Whitespace is defined as any of the following characters:
    #
    # - null
    # - horizontal tab
    # - line feed
    # - vertical tab
    # - form feed
    # - carriage return
    # - space
    strip_pointer -= 1 while strip_pointer >= string_start && "\x00\t\n\v\f\r ".include?(self[strip_pointer])
    return '' if strip_pointer.zero?

    dup[string_start..strip_pointer]
  end

  # https://ruby-doc.org/core-3.0.2/String.html#method-i-rstrip-21
  def rstrip!
    raise FrozenError, "can't modify frozen String: #{inspect}" if frozen?

    replaced = rstrip
    replace(replaced) unless self == replaced
  end

  # https://ruby-doc.org/core-3.0.2/String.html#method-i-scan
  #
  # NOTE: Implemented in native code.
  #
  # def scan(pattern, &block); end

  # https://ruby-doc.org/core-3.0.2/String.html#method-i-scrub
  def scrub
    # TODO: This is a stub. Implement scrub correctly.
    self
  end

  # https://ruby-doc.org/core-3.0.2/String.html#method-i-scrub-21
  def scrub!
    raise FrozenError, "can't modify frozen String: #{inspect}" if frozen?

    # TODO: This is a stub. Implement scrub! correctly.
    self
  end

  # https://ruby-doc.org/core-3.0.2/String.html#method-i-setbyte
  #
  # NOTE: Implemented in native code.
  #
  # def setbyte(index, integer); end

  # https://ruby-doc.org/core-3.0.2/String.html#method-i-size
  #
  # NOTE: Implemented in native code.
  #
  # def length; end

  # https://ruby-doc.org/core-3.0.2/String.html#method-i-slice
  #
  # NOTE: Implemented in native code.
  #
  # def [](*args); end

  # https://ruby-doc.org/core-3.0.2/String.html#method-i-slice-21
  #
  # NOTE: Implemented in native code.
  #
  # def slice!(*args); end

  # https://ruby-doc.org/core-3.0.2/String.html#method-i-split
  #
  # XXX: This should probably be implemented in native code.
  # TODO: Lots of branches are not implemented.
  # TODO: when implemented in native code remove `#[cfg(feature = "regexp")]`
  #       from  `value::tests::funcall_string_split_regexp`.
  def split(pattern = nil, limit = (limit_not_set = true), &block)
    return [] if empty?
    return [dup] if limit == 1

    raise NotImplementedError, 'String#split with block is not supported' unless block.nil?

    limit = -1 if limit_not_set

    if pattern.is_a?(Regexp)
      s = dup
      chunks = []
      until s.empty?
        if limit.positive? && chunks.length == limit - 1
          chunks << s
          return chunks
        end

        match = pattern.match(s)
        if match.nil?
          chunks << s
          return chunks
        end
        chunks << s[0, match.begin(0)]
        advance_to = match.end(0)
        advance_to += 1 if (match.end(0) - match.begin(0)).zero?

        s = s[advance_to..-1]

        return chunks if s.nil?
      end
      return chunks
    end

    pattern = $; if pattern.nil? # rubocop:disable Style/SpecialGlobalVars
    pattern = ' ' if pattern.nil?

    unless pattern.is_a?(String)
      converted = pattern.to_str
      unless converted.is_a?(String)
        raise TypeError, "can't convert #{pattern.class} to String (#{pattern.class}#to_str gives #{converted.class})"
      end

      pattern = converted
    end
    return chars if pattern.empty?

    s = dup
    chunks = []
    until s.empty?
      if limit.positive? && chunks.length == limit - 1
        chunks << s
        return chunks
      end

      index = s.index(pattern)
      if index.nil?
        chunks << s
        return chunks
      end
      chunks << s[0, index]
      s = s[index + pattern.length..-1]
    end
    chunks << ''
    chunks
  end

  # https://ruby-doc.org/core-3.0.2/String.html#method-i-squeeze
  def squeeze(*other_str)
    return '' if empty?
    raise NotImplementedError, 'String#squeeze with arguments is not implemented' unless other_str.empty?

    iter = chars
    head, *tail = iter
    runs = [head]
    last_seen = head

    tail.each do |ch|
      next if ch == last_seen

      last_seen = ch
      runs << ch
    end
    runs.join
  end

  # https://ruby-doc.org/core-3.0.2/String.html#method-i-squeeze-21
  def squeeze!(*other_str)
    raise FrozenError, "can't modify frozen String: #{inspect}" if frozen?

    replaced = squeeze(*other_str)
    replace(replaced) unless self == replaced
  end

  # https://ruby-doc.org/core-3.0.2/String.html#method-i-start_with-3F
  #
  # NOTE: Implemented in native code.
  #
  # def start_with?; end

  # https://ruby-doc.org/core-3.0.2/String.html#method-i-strip
  def strip
    result = lstrip
    result = self if result.nil?
    result.rstrip
  end

  # https://ruby-doc.org/core-3.0.2/String.html#method-i-strip-21
  def strip!
    raise FrozenError, "can't modify frozen String: #{inspect}" if frozen?

    replaced = strip
    replace(replaced) unless self == replaced
  end

  # https://ruby-doc.org/core-3.0.2/String.html#method-i-sub
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

  # https://ruby-doc.org/core-3.0.2/String.html#method-i-sub-21
  def sub!(pattern, replacement = nil, &blk)
    raise FrozenError, "can't modify frozen String: #{inspect}" if frozen?

    replaced = sub(pattern, replacement, &blk)
    replace(replaced) unless self == replaced
  end

  # https://ruby-doc.org/core-3.0.2/String.html#method-i-sum
  def sum(n_bits = 16)
    raise NotImplementedError
  end

  # https://ruby-doc.org/core-3.0.2/String.html#method-i-swapcase
  def swapcase(*_args)
    raise NotImplementedError
  end

  # https://ruby-doc.org/core-3.0.2/String.html#method-i-swapcase-21
  def swapcase!(*_args)
    raise FrozenError, "can't modify frozen String: #{inspect}" if frozen?

    raise NotImplementedError
  end

  # https://ruby-doc.org/core-3.0.2/String.html#method-i-to_a
  def to_a
    chars
  end

  # https://ruby-doc.org/core-3.0.2/String.html#method-i-to_c
  def to_c
    raise NotImplementedError
  end

  # https://ruby-doc.org/core-3.0.2/String.html#method-i-to_f
  #
  # NOTE: Implemented in native code.
  #
  # def to_f; end

  # https://ruby-doc.org/core-3.0.2/String.html#method-i-to_i
  #
  # NOTE: Implemented in native code.
  #
  # def to_i; end

  # https://ruby-doc.org/core-3.0.2/String.html#method-i-to_r
  def to_r
    raise NotImplementedError
  end

  # https://ruby-doc.org/core-3.0.2/String.html#method-i-to_s
  #
  # NOTE: Implemented in native code.
  #
  # def to_s; end

  # https://ruby-doc.org/core-3.0.2/String.html#method-i-to_str
  def to_str
    return self if instance_of?(String)

    String.new(self)
  end

  # https://ruby-doc.org/core-3.0.2/String.html#method-i-to_sym
  #
  # NOTE: Implemented in native code.
  #
  # def to_sym; end

  # https://ruby-doc.org/core-3.0.2/String.html#method-i-tr
  def tr(from_str, to_str)
    Artichoke::String.tr(self, from_str, to_str, false)
  end

  # https://ruby-doc.org/core-3.0.2/String.html#method-i-tr-21
  def tr!(from_str, to_str)
    raise FrozenError, "can't modify frozen String: #{inspect}" if frozen?

    replaced = tr(from_str, to_str)
    replace(replaced) unless self == replaced
  end

  # https://ruby-doc.org/core-3.0.2/String.html#method-i-tr_s
  def tr_s(from_str, to_str)
    Artichoke::String.tr(self, from_str, to_str, true)
  end

  # https://ruby-doc.org/core-3.0.2/String.html#method-i-tr_s-21
  def tr_s!(from_str, to_str)
    raise FrozenError, "can't modify frozen String: #{inspect}" if frozen?

    replaced = tr_s(from_str, to_str)
    replace(replaced) unless self == replaced
  end

  # https://ruby-doc.org/core-3.0.2/String.html#method-i-undump
  def undump
    raise NotImplementedError
  end

  # https://ruby-doc.org/core-3.0.2/String.html#method-i-unicode_normalize
  def unicode_normalize(_form = :nfc)
    raise NotImplementedError
  end

  # https://ruby-doc.org/core-3.0.2/String.html#method-i-unicode_normalize-21
  def unicode_normalize!(_form = :nfc)
    raise FrozenError, "can't modify frozen String: #{inspect}" if frozen?

    raise NotImplementedError
  end

  # https://ruby-doc.org/core-3.0.2/String.html#method-i-unicode_normalized-3F
  def unicode_normalized?(_form = :nfc)
    raise NotImplementedError
  end

  # https://ruby-doc.org/core-3.0.2/String.html#method-i-unpack
  #
  # NOTE: Implemented in native code.
  #
  # def unpack; end

  # https://ruby-doc.org/core-3.0.2/String.html#method-i-unpack1
  #
  # NOTE: Implemented in native code in `mruby-pack` mrbgem.
  #
  # def unpack1; end

  # https://ruby-doc.org/core-3.0.2/String.html#method-i-upcase
  #
  # NOTE: Implemented in native code.
  #
  # def upcase; end

  # https://ruby-doc.org/core-3.0.2/String.html#method-i-upcase-21
  #
  # NOTE: Implemented in native code.
  #
  # def upcase!; end

  # https://ruby-doc.org/core-3.0.2/String.html#method-i-upto
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

  # https://ruby-doc.org/core-3.0.2/String.html#method-i-valid_encoding-3F
  #
  # NOTE: Implemented in native code.
  #
  # def valid_encoding?; end
end
