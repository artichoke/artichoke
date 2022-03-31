# frozen_string_literal: true

ENV = Object.new

class << ENV
  def [](name)
    @backend ||= ::Artichoke::Environ.new
    @backend[name]
  end

  def []=(name, value)
    @backend ||= ::Artichoke::Environ.new
    @backend[name] = value
  end

  def assoc(name)
    name = name.to_str unless name.is_a?(String)
    value = self[name]
    return nil if value.nil?

    [name, value]
  end

  def clear
    to_h.each do |var_name, var_value|
      self[var_name] = nil unless var_value.nil?
    end

    to_h
  end

  def delete(name)
    value = self[name]
    if value.nil?
      yield name if block_given?
      return nil
    end

    self[name] = nil
    yield name if block_given?

    value
  end

  def delete_if
    return to_enum(:delete_if) unless block_given?

    to_h.each do |key, value|
      delete(key) if yield key, value
    end
    to_h
  end

  def each(&blk)
    return to_enum(:each) unless block_given?

    to_h.each(&blk)
  end

  def each_key(&blk)
    return to_enum(:each_key) unless block_given?

    to_h.each_key(&blk)
  end

  def each_pair(&blk)
    return to_enum(:each_pair) unless block_given?

    to_h.each_pair(&blk)
  end

  def each_value(&blk)
    return to_enum(:each_value) unless block_given?

    to_h.each_value(&blk)
  end

  def empty?
    to_h.empty?
  end

  def fetch(name, default = (not_set = true))
    warn 'warning: block supersedes default value argument' if !not_set && block_given?

    name =
      if name.is_a?(String)
        name
      elsif name.respond_to?(:to_str)
        converted = name.to_str
        unless converted.is_a?(String)
          raise TypeError, "can't convert #{name.class} to String (#{name.class}#to_str gives #{converted.class})"
        end

        converted
      else
        cls = name.class
        cls = 'nil' if name.nil?

        raise TypeError, "no implicit conversion of #{cls} into String"
      end

    value = self[name]
    return value unless value.nil?
    return yield name if block_given?
    return default unless not_set

    raise KeyError.new("key not found: #{name.inspect}", receiver: self, key: name)
  end

  def filter(&blk)
    return to_enum(:filter) unless block_given?

    to_h.select(&blk)
  end

  def filter!(&blk)
    return to_enum(:filter!) unless block_given?

    select!(&blk)
  end

  def has_key?(name) # rubocop:disable Naming/PredicateName
    to_h.key?(name)
  end

  def has_value?(value) # rubocop:disable Naming/PredicateName
    to_h.value?(value)
  end

  def include?(name)
    to_h.key?(name)
  end

  def index(value)
    to_h.key(value)
  end

  def inspect
    to_h.to_s
  end

  def invert
    to_h.invert
  end

  def keep_if
    return to_enum(:keep_if) unless block_given?

    to_h.each do |key, value|
      delete(key) unless yield key, value
    end
    to_h
  end

  def key(value)
    to_h.key(value)
  end

  def key?(name)
    !self[name].nil?
  end

  def keys
    to_h.keys
  end

  def length
    to_h.length
  end

  def member?(name)
    !self[name].nil?
  end

  def merge!(hash)
    hash.each do |key, value|
      value = yield(key, self[key], value) if block_given? && key?(key)
      self[key] = value
    end

    to_h
  end
  alias update merge!

  def rassoc(value)
    value = value.to_str unless value.is_a?(String)
    to_h.each do |k, v|
      return [k, v] if v == value
    end
    nil
  end

  def rehash
    nil
  end

  def reject(&blk)
    return to_enum(:reject) unless block_given?

    to_h.delete_if(&blk)
  end

  def reject!
    return to_enum(:reject!) unless block_given?

    modified = false
    to_h.each do |key, value|
      if yield key, value
        delete(key)
        modified = true
      end
    end

    return self if modified

    nil
  end

  def replace(hash)
    hash.each do |k, v|
      self[k] = v
    end
    select! { |k, _| hash.key?(k) }
  end

  def select(&blk)
    return to_enum(:select) unless block_given?

    to_h.select(&blk)
  end

  def select!(&blk)
    return to_enum(:select!) unless block_given?

    env = to_h
    # collect all the keys where the block evaluates to false
    to_remove = env.reject(&blk)

    # returns nil if no changes were made
    return nil if to_remove.empty?

    to_remove.each do |key, _|
      delete(key)
    end
    self
  end

  def shift
    envs = to_h
    return nil if envs.nil? || envs.empty?

    name, value = envs.shift
    self[name] = nil
    [name, value]
  end

  def size
    to_h.size
  end

  def slice(*keys)
    to_h.slice(*keys)
  end

  def store(name, value)
    self[name] = value
  end

  def to_a
    to_h.to_a
  end

  def to_h
    @backend ||= ::Artichoke::Environ.new
    h = @backend.to_h
    return h unless block_given?

    pairs = h.each_pair.map do |name, value|
      tx = yield(name, value)
      if tx.is_a?(Array)
        raise ArgumentError, "element has wrong array length (expected 2, was #{tx.length})" if tx.length != 2

        tx
      elsif tx.respond_to?(:to_ary)
        pair = tx.to_ary
        unless pair.is_a?(Array)
          raise TypeError, "can't convert #{tx.class} to Array (#{tx.class}#to_ary gives #{pair.class})"
        end
        raise ArgumentError, "element has wrong array length (expected 2, was #{pair.length})" if pair.length != 2

        pair
      else
        raise TypeError, "wrong element type #{tx.class} (expected array)"
      end
    end
    pairs.to_h
  end

  def to_hash
    to_h
  end

  def to_s
    'ENV'
  end

  def value?(name)
    to_h.value?(name)
  end

  def values
    to_h.values
  end

  def values_at(*names)
    to_h.values_at(*names)
  end
end
