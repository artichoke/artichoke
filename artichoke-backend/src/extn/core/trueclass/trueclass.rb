# frozen_string_literal: true

class TrueClass
  def <=>(other)
    return nil unless other.equal?(true) || other.equal?(false)
    return 0 if self == other
  
    1
  end
  
  def dup
    self
  end
end
