# frozen_string_literal: true

class EnvClass
  def assoc(name)
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
    return nil if value.nil?

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

  def each
    return to_enum(:each) unless block_given?

    to_h.each { |key, value| yield key, value }
  end

  def each_key
    return to_enum(:each_key) unless block_given?

    to_h.each_key { |key| yield key }
  end

  def each_pair
    return to_enum(:each) unless block_given?

    to_h.each_pair { |key, value| yield key, value }
  end

  def each_value
    return to_enum(:each_value) unless block_given?

    to_h.each_value { |value| yield value }
  end

  def empty?
    to_h.empty?
  end

  def fetch(name, default = (not_set = true))
    value = self[name]

    return value unless value.nil?

    return yield name if block_given?

    return default if not_set.nil?

    raise KeyError, "key not found: #{name}"
  end

  def filter
    return to_enum(:filter) unless block_given?

    to_h.select { |key, value| yield key, value }
  end

  def filter!
    return to_enum(:filter!) unless block_given?

    select! { |key, value| yield key, value }
  end

  def has_key?(name) # rubocop:disable PredicateName
    to_h.key?(name)
  end

  def has_value?(value) # rubocop:disable PredicateName
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

  def rassoc(value)
    to_h.each do |k, v|
      return [k, v] if v == value
    end

    nil
  end

  def rehash
    nil
  end

  def reject
    return to_enum(:reject) unless block_given?

    to_h.delete_if { |key, value| yield key, value }
  end

  def reject!
    return to_enum(:reject!) unless block_given?

    env = to_h

    # collect all the keys where the block evaluates to true
    to_remove = env.reject do |key, value|
      yield key, value
    end

    # returns nil if no changes were made
    return nil if to_remove.empty?

    to_remove.each do |key, _|
      delete(key)
    end

    self
  end

  def replace(hash)
    hash.each do |k, v|
      self[k] = v
    end

    select! { |k, _| hash.key?(k) }
  end

  def select
    return to_enum(:select) unless block_given?

    to_h.select { |key, value| yield key, value }
  end

  def select!
    return to_enum(:select!) unless block_given?

    env = to_h

    # collect all the keys where the block evaluates to false
    to_remove = env.reject do |key, value|
      yield key, value
    end

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

  def to_hash
    to_h
  end

  def to_s
    'ENV'
  end

  def update(hash)
    hash.each do |key, value|
      self[key] = value
    end

    to_h
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
