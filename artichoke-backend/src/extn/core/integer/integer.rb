# frozen_string_literal: true

class Integer
  include Comparable
  # mruby hack to get Integer#<=>
  include Integral

  def allbits?(mask)
    if mask.respond_to?(:to_int)
      mask = mask.to_int
      return self & mask == mask
    end

    classname = mask.class
    if mask.nil? || mask.equal?(false) || mask.equal?(true)
      classname = mask.inspect
    end
    raise TypeError, "no implicit conversion of #{classname} into #{self.class}"
  end

  def anybits?(mask)
    if mask.respond_to?(:to_int)
      mask = mask.to_int
      return !(self & mask).zero?
    end

    classname = mask.class
    if mask.nil? || mask.equal?(false) || mask.equal?(true)
      classname = mask.inspect
    end
    raise TypeError, "no implicit conversion of #{classname} into #{self.class}"
  end

  def ceil
    self
  end

  def floor
    self
  end

  def nobits?(mask)
    if mask.respond_to?(:to_int)
      mask = mask.to_int
      return (self & mask).zero?
    end

    classname = mask.class
    if mask.nil? || mask.equal?(false) || mask.equal?(true)
      classname = mask.inspect
    end
    raise TypeError, "no implicit conversion of #{classname} into #{self.class}"
  end

  alias round floor
  alias truncate floor
end
