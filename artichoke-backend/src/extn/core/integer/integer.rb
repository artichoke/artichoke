# frozen_string_literal: true

class Integer
  def allbits?(mask)
    if mask.respond_to?(:to_int)
      mask = mask.to_int
      return self & mask == mask
    end

    classname = mask.class
    classname = mask.inspect if mask.nil? || mask.equal?(false) || mask.equal?(true)
    raise TypeError, "no implicit conversion of #{classname} into #{self.class}"
  end

  def anybits?(mask)
    if mask.respond_to?(:to_int)
      mask = mask.to_int
      return !(self & mask).zero?
    end

    classname = mask.class
    classname = mask.inspect if mask.nil? || mask.equal?(false) || mask.equal?(true)
    raise TypeError, "no implicit conversion of #{classname} into #{self.class}"
  end

  def nobits?(mask)
    if mask.respond_to?(:to_int)
      mask = mask.to_int
      return (self & mask).zero?
    end

    classname = mask.class
    classname = mask.inspect if mask.nil? || mask.equal?(false) || mask.equal?(true)
    raise TypeError, "no implicit conversion of #{classname} into #{self.class}"
  end
end
