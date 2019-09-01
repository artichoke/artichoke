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

  def empty?
    to_h.empty?
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

  def key(value)
    to_h.key(value)
  end

  def keys
    to_h.keys
  end

  def length
    to_h.length
  end

  def rehash
    nil
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
