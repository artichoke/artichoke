# frozen_string_literal: false

class Symbol
  # Returns a hash, that will be turned into a JSON object and represent this
  # object.
  def as_json(*)
    {
      JSON.create_id => self.class.name,
      's' => to_s
    }
  end

  # Stores class name (Symbol) with String representation of Symbol as a JSON string.
  def to_json(*args)
    as_json.to_json(*args)
  end

  # Deserializes JSON string by converting the <tt>string</tt> value stored in the object to a Symbol
  def self.json_create(obj)
    obj['s'].to_sym
  end
end
