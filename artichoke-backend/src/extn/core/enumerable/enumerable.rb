# frozen_string_literal: true

##
# Enumerable
#
module Enumerable
  def all?(pat = NONE, &block)
    if pat != NONE
      each do |*val|
        return false unless pat === val.__svalue # rubocop:disable Style/CaseEquality
      end
    elsif block
      each { |*val| return false unless block.call(*val) }
    else
      each { |*val| return false unless val.__svalue }
    end
    true
  end

  def any?(pat = NONE, &block)
    if pat != NONE
      each do |*val|
        return true if pat === val.__svalue # rubocop:disable Style/CaseEquality
      end
    elsif block
      each { |*val| return true if block.call(*val) }
    else
      each { |*val| return true if val.__svalue }
    end
    false
  end

  def count(elem = NONE, &block)
    count = 0
    if block
      each do |*val|
        count += 1 if block.call(*val)
      end
    elsif elem == NONE
      each { count += 1 }
    else
      each do |*val|
        count += 1 if val.__svalue == elem
      end
    end
    count
  end

  def cycle(nval = nil, &block)
    return to_enum(:cycle, nval) unless block

    n = nil

    if nval.nil?
      n = -1
    else
      n = nval.__to_int
      return nil if n <= 0
    end

    ary = []
    each do |*i|
      ary.push(i)
      yield(*i)
    end
    return nil if ary.empty?

    while n.negative? || (n -= 1).positive?
      ary.each do |i|
        yield(*i)
      end
    end

    nil
  end

  def drop(size)
    size = size.__to_int
    raise ArgumentError, 'attempt to drop negative size' if size.negative?

    ary = []
    each { |*val| size.zero? ? ary << val.__svalue : size -= 1 }
    ary
  end

  def drop_while(&block)
    return to_enum :drop_while unless block

    ary = []
    state = false
    each do |*val|
      state = true if !state && !block.call(*val)
      ary << val.__svalue if state
    end
    ary
  end

  def each_cons(size, &block)
    size = size.__to_int
    raise ArgumentError, 'invalid size' if size <= 0

    return to_enum(:each_cons, size) unless block

    ary = []
    size = size.to_i
    each do |*val|
      ary.shift if ary.size == size
      ary << val.__svalue
      block.call(ary.dup) if ary.size == size
    end
    nil
  end

  def each_slice(size, &block)
    size = size.__to_int
    raise ArgumentError, 'invalid slice size' if size <= 0

    return to_enum(:each_slice, size) unless block

    ary = []
    size = size.to_i
    each do |*val|
      ary << val.__svalue
      if ary.size == size
        block.call(ary)
        ary = []
      end
    end
    block.call(ary) unless ary.empty?
    nil
  end

  def each_with_object(obj, &block)
    return to_enum(:each_with_object, obj) unless block

    each { |*val| block.call(val.__svalue, obj) }
    obj
  end

  def find_index(val = NONE, &block)
    return to_enum(:find_index, val) if !block && val == NONE

    idx = 0
    if block
      each do |*e|
        return idx if block.call(*e)

        idx += 1
      end
    else
      each do |*e|
        return idx if e.__svalue == val

        idx += 1
      end
    end
    nil
  end

  def first(*args)
    case args.length
    when 0
      each do |*val|
        return val.__svalue
      end
      nil
    when 1
      i = args[0].__to_int
      raise ArgumentError, 'attempt to take negative size' if i.negative?

      ary = []
      return ary if i.zero?

      each do |*val|
        ary << val.__svalue
        i -= 1
        break if i.zero?
      end
      ary
    else
      raise ArgumentError, "wrong number of arguments (given #{args.length}, expected 0..1)"
    end
  end

  def flat_map(&block)
    return to_enum :flat_map unless block

    ary = []
    each do |*e|
      e2 = block.call(*e)
      if e2.respond_to? :each
        e2.each { |e3| ary.push(e3) }
      else
        ary.push(e2)
      end
    end
    ary
  end
  alias collect_concat flat_map

  def group_by(&block)
    return to_enum :group_by unless block

    h = {}
    each do |*val|
      key = block.call(*val)
      sv = val.__svalue
      h.key?(key) ? (h[key] << sv) : (h[key] = [sv])
    end
    h
  end

  def max_by(&block)
    return to_enum :max_by unless block

    first = true
    max = nil
    max_cmp = nil

    each do |*val|
      if first
        max = val.__svalue
        max_cmp = block.call(*val)
        first = false
      elsif (cmp = block.call(*val)) > max_cmp
        max = val.__svalue
        max_cmp = cmp
      end
    end
    max
  end

  def min_by(&block)
    return to_enum :min_by unless block

    first = true
    min = nil
    min_cmp = nil

    each do |*val|
      if first
        min = val.__svalue
        min_cmp = block.call(*val)
        first = false
      elsif (cmp = block.call(*val)) < min_cmp
        min = val.__svalue
        min_cmp = cmp
      end
    end
    min
  end

  def minmax(&block)
    max = nil
    min = nil
    first = true

    each do |*val|
      val = val.__svalue
      if first
        max = val
        min = val
        first = false
      elsif block
        max = val if block.call(val, max).positive?
        min = val if block.call(val, min).negative?
      else
        max = val if (val <=> max).positive?
        min = val if (val <=> min).negative?
      end
    end
    [min, max]
  end

  def minmax_by(&block)
    return to_enum :minmax_by unless block

    max = nil
    max_cmp = nil
    min = nil
    min_cmp = nil
    first = true

    each do |*val|
      if first
        max = min = val.__svalue
        max_cmp = min_cmp = block.call(*val)
        first = false
      elsif (cmp = block.call(*val)) > max_cmp
        max = val.__svalue
        max_cmp = cmp
      elsif (cmp = block.call(*val)) < min_cmp
        min = val.__svalue
        min_cmp = cmp
      end
    end
    [min, max]
  end

  def none?(pat = NONE, &block)
    if pat != NONE
      each do |*val|
        return false if pat === val.__svalue # rubocop:disable Style/CaseEquality # rubocop:disable Style/CaseEquality
      end
    elsif block
      each do |*val|
        return false if block.call(*val)
      end
    else
      each do |*val|
        return false if val.__svalue
      end
    end
    true
  end

  def one?(pat = NONE, &block)
    count = 0
    if pat != NONE
      each do |*val|
        count += 1 if pat === val.__svalue # rubocop:disable Style/CaseEquality
        return false if count > 1
      end
    elsif block
      each do |*val|
        count += 1 if block.call(*val)
        return false if count > 1
      end
    else
      each do |*val|
        count += 1 if val.__svalue
        return false if count > 1
      end
    end

    count == 1
  end

  def reverse_each(&block)
    return to_enum :reverse_each unless block

    ary = to_a
    i = ary.size - 1
    while i >= 0
      block.call(ary[i])
      i -= 1
    end
    self
  end

  def sort_by(&block)
    return to_enum :sort_by unless block

    ary = []
    orig = []
    each_with_index do |e, i|
      orig.push(e)
      ary.push([block.call(e), i])
    end
    ary.sort! if ary.size > 1
    ary.collect { |_e, i| orig[i] }
  end

  def take(size)
    size = size.__to_int
    i = size.to_i
    raise ArgumentError, 'attempt to take negative size' if i.negative?

    ary = []
    return ary if i.zero?

    each do |*val|
      ary << val.__svalue
      i -= 1
      break if i.zero?
    end
    ary
  end

  def take_while(&block)
    return to_enum :take_while unless block

    ary = []
    each do |*val|
      return ary unless block.call(*val)

      ary << val.__svalue
    end
    ary
  end

  def to_h(&blk)
    h = {}
    if blk
      each do |v|
        v = blk.call(v)
        raise TypeError, "wrong element type #{v.class} (expected Array)" unless v.is_a? Array
        raise ArgumentError, "element has wrong array length (expected 2, was #{v.size})" if v.size != 2

        h[v[0]] = v[1]
      end
    else
      each do |*v|
        v = v.__svalue
        raise TypeError, "wrong element type #{v.class} (expected Array)" unless v.is_a? Array
        raise ArgumentError, "element has wrong array length (expected 2, was #{v.size})" if v.size != 2

        h[v[0]] = v[1]
      end
    end
    h
  end

  def nil.to_h
    {}
  end

  def uniq(&block)
    hash = {}
    if block
      each do |*v|
        v = v.__svalue
        hash[block.call(v)] ||= v
      end
    else
      each do |*v|
        v = v.__svalue
        hash[v] ||= v
      end
    end
    hash.values
  end

  def zip(*arg, &block)
    result = block ? nil : []
    arg = arg.map do |a|
      raise TypeError, "wrong argument type #{a.class} (must respond to :to_a)" unless a.respond_to?(:to_a)

      a.to_a
    end

    i = 0
    each do |*val|
      a = []
      a.push(val.__svalue)
      idx = 0
      while idx < arg.size
        a.push(arg[idx][i])
        idx += 1
      end
      i += 1
      if result.nil?
        block.call(a)
      else
        result.push(a)
      end
    end
    result
  end
end
