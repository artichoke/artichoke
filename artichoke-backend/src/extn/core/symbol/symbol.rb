# frozen_string_literal: true

class Symbol
  include Comparable

  # Implemented in native code.
  # def self.all_symbols; end

  def <=>(other)
    return nil unless other.is_a?(Symbol)

    to_s <=> other.to_s
  end

  # Implemented in native code.
  # def ==; end

  def ===(other)
    self == other
  end

  def =~(other)
    to_s =~ other
  end

  def [](idx, len = (not_set = true))
    return to_s[idx] if not_set

    to_s[idx, len]
  end

  def capitalize(options = (not_set = true))
    return to_s.capitalize.intern if not_set

    to_s.capitalize(options).intern
  end

  def casecmp(other)
    return nil unless other.is_a?(Symbol)

    # Case-insensitive version of Symbol#<=>. Currently, case-insensitivity only
    # works on characters A-Z/a-z, not all of Unicode. This is different from
    # Symbol#casecmp?.
    lhs = to_s.tr('a-z', 'A-Z')
    rhs = other.to_s.tr('a-z', 'A-Z')
    lhs <=> rhs
  end

  def casecmp?(other)
    # Returns true if `self` and `other` are equal after Unicode case folding,
    # false if they are not equal.
    #
    # Delegate to String#casecmp? which is also Unicode case folding-aware.
    to_s.casecmp?(other.to_s)
  end

  def downcase(options = (not_set = true))
    return to_s.downcase.intern if not_set

    to_s.downcase(options).intern
  end

  # Implemented in native code.
  # def empty?; end

  def encoding
    raise NotImplementedError, 'Artichoke does not have Encoding support'
  end

  # Implemented in native code.
  # def length; end

  def match(*args)
    to_s.match(*args)
  end

  def match?(*args)
    to_s.match?(*args)
  end

  def succ
    to_s.succ.intern
  end

  def swapcase
    to_s.swapcase.intern
  end

  def to_proc
    ->(obj, *args, &block) { obj.__send__(self, *args, &block) }
  end

  # Implemented in native code.
  # def to_s; end

  def to_sym
    self
  end

  def upcase(options = (not_set = true))
    return to_s.upcase.intern if not_set

    to_s.upcase(options).intern
  end

  alias id2name to_s
  alias intern to_sym
  alias next succ
  alias size length
  alias slice []
end
