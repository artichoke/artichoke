# frozen_string_literal: true

class Hash
  undef _inspect

  def inspect
    return '{}' if size.zero?

    items = keys.map do |key|
      val = self[key]
      key =
        if object_id == key.object_id
          '{...}'
        else
          key.inspect
        end
      val =
        if object_id == val.object_id
          '{...}'
        else
          val.inspect
        end
      "#{key}=>#{val}"
    end
    "{#{items.join(', ')}}"
  end

  alias to_s inspect
end
