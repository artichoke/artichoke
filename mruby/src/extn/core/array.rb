# frozen_string_literal: true

class Array
  undef _inspect

  def inspect
    items = map do |item|
      if object_id == item.object_id
        '[...]'
      else
        item.inspect
      end
    end
    "[#{items.join(', ')}]"
  end

  def to_ary
    self
  end
end
