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
    (to_s.capitalize! || self).to_sym
  end

  def downcase
    (to_s.downcase! || self).to_sym
  end

  def upcase
    (to_s.upcase! || self).to_sym
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

  def casecmp?(sym)
    c = casecmp(sym)
    return nil if c.nil?

    c.zero?
  end

  def empty?
    to_s.empty?
  end

  def length
    to_s.length
  end

  alias size length
  alias intern to_sym
end
