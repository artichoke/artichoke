# frozen_string_literal: true

class Hash
  def self.[](*object)
    length = object.length
    if length == 1
      o = object[0]
      if o.is_a?(Hash)
        h = new
        o.each { |k, v| h[k] = v }
        return h
      elsif o.respond_to?(:to_a)
        h = new
        o.to_a.each do |i|
          raise ArgumentError, "wrong element type #{i.class} (expected array)" unless i.respond_to?(:to_a)

          k, v = nil
          case i.size
          when 2
            k = i[0]
            v = i[1]
          when 1
            k = i[0]
          else
            raise ArgumentError, "invalid number of elements (#{i.size} for 1..2)"
          end
          h[k] = v
        end
        return h
      end
    end
    raise ArgumentError, 'odd number of arguments for Hash' unless length.even?

    h = new
    0.step(length - 2, 2) do |i|
      h[object[i]] = object[i + 1]
    end
    h
  end

  def <(other)
    raise TypeError, "can't convert #{other.class} to Hash" unless other.is_a?(Hash)

    (size < other.size) && all? do |key, val|
      other.key?(key) && (other[key] == val)
    end
  end

  def <=(other)
    raise TypeError, "can't convert #{other.class} to Hash" unless other.is_a?(Hash)

    (size <= other.size) && all? do |key, val|
      other.key?(key) && (other[key] == val)
    end
  end

  def >(other)
    raise TypeError, "can't convert #{other.class} to Hash" unless other.is_a?(Hash)

    (size > other.size) && other.all? do |key, val|
      key?(key) && (self[key] == val)
    end
  end

  def >=(other)
    raise TypeError, "can't convert #{other.class} to Hash" unless other.is_a?(Hash)

    (size >= other.size) && other.all? do |key, val|
      key?(key) && (self[key] == val)
    end
  end

  def compact
    non_nil_valued_keys = keys.reject do |k|
      self[k].nil?
    end
    non_nil_valued_keys.each_with_object({}) do |key, memo|
      memo[key] = self[key]
    end
  end

  def compact!
    non_nil_valued_keys = keys.reject do |k|
      self[k].nil?
    end
    return nil if keys.size == nk.size

    hash = non_nil_valued_keys.each_with_object({}) do |key, memo|
      memo[key] = self[key]
    end
    replace(hash)
  end

  def delete_if(&block)
    return to_enum :delete_if unless block

    each do |k, v|
      delete(k) if block.call(k, v)
    end
    self
  end

  def dig(idx, *args)
    n = self[idx]
    if !args.empty?
      n&.dig(*args)
    else
      n
    end
  end

  def fetch(key, none = NONE, &block)
    if key?(key)
      self[key]
    elsif block
      block.call(key)
    elsif none != NONE
      none
    else
      raise KeyError, "Key not found: #{key.inspect}"
    end
  end

  def fetch_values(*keys, &block)
    keys.map do |k|
      fetch(k, &block)
    end
  end

  def flatten(level = 1)
    to_a.flatten(level)
  end

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

  def invert
    h = self.class.new
    each { |k, v| h[v] = k }
    h
  end

  def keep_if(&block)
    return to_enum :keep_if unless block

    each do |k, v|
      delete(k) unless block.call([k, v])
    end
    self
  end

  def key(val)
    each do |k, v|
      return k if v == val
    end
    nil
  end

  def merge!(other, &block)
    raise TypeError, "Hash required (#{other.class} given)" unless other.is_a?(Hash)

    if block
      other.each_key do |k|
        self[k] = key?(k) ? block.call(k, self[k], other[k]) : other[k]
      end
    else
      other.each_key { |k| self[k] = other[k] }
    end
    self
  end

  def to_h
    self
  end

  def to_hash
    self
  end

  def to_proc
    ->(key) { self[key] }
  end

  def transform_keys(&block)
    return to_enum :transform_keys unless block

    hash = {}
    keys.each do |k|
      new_key = block.call(k)
      hash[new_key] = self[k]
    end
    hash
  end

  def transform_keys!(&block)
    return to_enum :transform_keys! unless block

    keys.each do |k|
      value = self[k]
      __delete(k)
      k = block.call(k) if block
      self[k] = value
    end
    self
  end

  def transform_values
    return to_enum :transform_values unless block_given?

    hash = {}
    keys.each do |k|
      hash[k] = yield(self[k])
    end
    hash
  end

  def transform_values!
    return to_enum :transform_values! unless block_given?

    keys.each do |k|
      self[k] = yield(self[k])
    end
    self
  end

  alias to_s inspect
  alias each_pair each
  alias update merge!
end
