# frozen_string_literal: true

class Symbol
  include Comparable

  def <=>(other)
    return nil unless other.is_a?(Symbol)

    to_s <=> other.to_s
  end

  def =~(other)
    to_s =~ other
  end

  def [](*args)
    to_s[*args]
  end

  def capitalize
    to_s.capitalize.intern
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

  def downcase
    to_s.downcase.intern
  end

  def empty?
    self == :''
  end

  def encoding
    raise NotImplementedError, 'Artichoke does not have Encoding support'
  end

  def length
    to_s.length
  end

  def upcase
    to_s.upcase.intern
  end

  alias size length
  alias intern to_sym
end
