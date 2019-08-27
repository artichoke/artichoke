# frozen_string_literal: true

class Integer
  def allbits?(mask)
    mask = rb_to_int(mask)
    self & mask == mask
  end

  def anybits?(mask)
    mask = rb_to_int(mask)
    !(self & mask).zero?
  end

  def nobits?(mask)
    mask = rb_to_int(mask)
    (self & mask).zero?
  end

  private

  def rb_to_int(value)
    unless value.respond_to?(:to_int)
      value = case value
              when true, false
                value
              else
                value.class
              end
      raise TypeError, "no implicit conversion of #{value} into Integer"
    end
    value.to_int
  end
end
