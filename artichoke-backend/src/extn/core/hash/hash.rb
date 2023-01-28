# frozen_string_literal: true

module Artichoke
  class Hash
    # rubocop:disable Lint/HashCompareByIdentity
    def self.inspect(hash, recur_list)
      keys = hash.keys
      size = keys.size
      return '{}' if size.zero?
      return '{...}' if recur_list[hash.object_id]

      recur_list[hash.object_id] = true
      out = []
      i = 0
      while i < size
        pair = []

        key = keys[i]
        pair <<
          case key
          when ::Array
            ::Artichoke::Array.inspect(key, recur_list)
          when ::Hash
            ::Artichoke::Hash.inspect(key, recur_list)
          else
            key.inspect
          end

        value = hash[key]
        pair <<
          case value
          when ::Array
            ::Artichoke::Array.inspect(value, recur_list)
          when ::Hash
            ::Artichoke::Hash.inspect(value, recur_list)
          else
            value.inspect
          end

        out << pair.join('=>')
        i += 1
      end
      "{#{out.join(', ')}}"
    end
    # rubocop:enable Lint/HashCompareByIdentity
  end
end

class Hash
  include Enumerable

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

  def ==(other)
    return true if equal?(other)
    return false unless other.is_a?(Hash)
    return false unless size == other.size

    each do |k, _|
      return false unless other.key?(k)
      return false unless self[k] == other[k]
    end
    true
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

  def delete(key, &block)
    return block.call(key) if block && !key?(key)

    # TODO: mruby internal method call
    __delete(key)
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
    if args.empty?
      n
    else
      n&.dig(*args)
    end
  end

  def each(&block)
    return to_enum :each unless block

    len = size
    idx = 0
    while idx < len
      key = keys[idx]
      block.call([key, self[key]])
      idx += 1
      len = size
    end
    self
  end

  def each_key(&block)
    return to_enum :each_key unless block

    keys.each { |k| block.call(k) }
    self
  end

  def each_value(&block)
    return to_enum :each_value unless block

    values.each { |v| block.call(v) }
    self
  end

  def eql?(other)
    return true if equal?(other)
    return false unless other.is_a?(Hash)
    return false if size != other.size

    each do |k, _|
      return false unless other.key?(k)
      return false unless self[k].eql?(other[k])
    end
    true
  end

  def fetch(key, default = (not_set = true), &block)
    if key?(key)
      self[key]
    elsif block
      block.call(key)
    elsif !not_set
      default
    else
      raise KeyError, "key not found: #{key.inspect}"
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
    ::Artichoke::Hash.inspect(self, {})
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

  def merge(other, &block)
    raise TypeError, "Hash required (#{other.class} given)" unless other.is_a?(Hash)

    h = dup
    if block
      other.each_key do |k|
        if key?(k)
          block.call(k, self[k], other[k])
        else
          other[k]
        end
      end
    else
      other.each_key { |k| h[k] = other[k] }
    end
    h
  end

  def merge!(*others, &block)
    i = 0
    len = others.size
    return __merge(*others) unless block

    while i < len
      other = others[i]
      i += 1
      raise TypeError, "Hash required (#{other.class} given)" unless other.is_a?(Hash)

      if block
        other.each_key do |k|
          self[k] = key?(k) ? block.call(k, self[k], other[k]) : other[k]
        end
      else
        other.each_key { |k| self[k] = other[k] }
      end
    end
    self
  end

  def reject(&block)
    return to_enum :reject unless block

    h = {}
    each do |k, v|
      h[k] = v unless block.call([k, v])
    end
    h
  end

  def reject!(&block)
    return to_enum :reject! unless block

    keys = []
    each do |k, v|
      keys.push(k) if block.call([k, v])
    end
    return nil if keys.empty?

    keys.each { |k| delete(k) }
    self
  end

  def replace(hash)
    raise TypeError, "Hash required (#{hash.class} given)" unless hash.is_a?(Hash)

    clear
    hash.each_key { |k| self[k] = hash[k] }
    if hash.default_proc
      self.default_proc = hash.default_proc
    else
      self.default = hash.default
    end
    self
  end

  def select(&block)
    return to_enum :select unless block

    h = {}
    each do |k, v|
      h[k] = v if block.call([k, v])
    end
    h
  end

  def select!(&block)
    return to_enum :select! unless block

    keys = []
    each do |k, v|
      keys.push(k) unless block.call([k, v])
    end

    return nil if keys.empty?

    keys.each do |k|
      delete(k)
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

  alias each_pair each
  alias initialize_copy replace
  alias to_s inspect
  alias update merge!
end
