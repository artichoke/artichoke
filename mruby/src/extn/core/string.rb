# frozen_string_literal: true

class String
  alias __old_idx []

  # https://ruby-doc.org/core-2.6.3/String.html#method-i-5B-5D
  def [](*args)
    if (regexp = args[0]).is_a?(Regexp)
      capture = args[1] || 0
      return regexp.match(self)&.[](capture)
    end
    __old_idx(*args)
  end

  # https://ruby-doc.org/core-2.6.3/String.html#method-i-3D-7E
  def =~(other)
    return other.match(self)&.begin(0) if other.is_a?(Regexp)
    return other =~ self if other.respond_to?(:=~)

    nil
  end

  # TODO: handle block
  def split(pattern, limit = nil)
    if pattern == ''
      parts = []
      length.times do |i|
        parts << self[i]
      end
      return parts
    end

    pattern = Regexp.compile(Regexp.escape(pattern)) if pattern.is_a?(String)

    parts = []
    remainder = self
    match = pattern.match(remainder)
    until match.nil? || (limit && parts.length >= limit)
      parts <<
        if match.begin(0).positive?
          remainder[0..match.begin(0) - 1]
        else
          ''
        end
      remainder = remainder[match.end(0)..-1]
      remainder = remainder[1..-1] if match.begin(0) == match.end(0)
      match = nil
      pattern.match(remainder) unless remainder.nil?
    end
    return parts if remainder.nil? || remainder.empty?

    parts << remainder if limit.nil? || parts.length < limit
    parts
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
    remainder = self
    buf << remainder[0..match.begin(0) - 1] if match.begin(0).positive?
    buf << replace.call(match[0])
    remainder = remainder[match.end(0)..-1]
    remainder = remainder[1..-1] if match.begin(0) == match.end(0)
    buf << remainder
    buf
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
    remainder = self
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
    self[0..-1] = replaced
  end
end
