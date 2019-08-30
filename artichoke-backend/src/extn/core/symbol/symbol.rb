class Symbol
  include Comparable

  alias intern to_sym

  def capitalize
    (self.to_s.capitalize! || self).to_sym
  end

  def downcase
    (self.to_s.downcase! || self).to_sym
  end

  def upcase
    (self.to_s.upcase! || self).to_sym
  end

  def casecmp(other)
    return nil unless other.kind_of?(Symbol)
    lhs =  self.to_s; lhs.upcase!
    rhs = other.to_s.upcase
    lhs <=> rhs
  end

  def casecmp?(sym)
    c = self.casecmp(sym)
    return nil if c.nil?
    return c == 0
  end

  def empty?
    self.length == 0
  end
end
