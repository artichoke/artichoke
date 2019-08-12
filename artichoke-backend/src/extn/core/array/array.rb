# frozen_string_literal: true

class Array
  def &(other)
    raise TypeError, "can't convert #{other.class} into Array" unless other.is_a?(Array)

    hash = {}
    array = []
    idx = 0
    len = other.size
    while idx < len
      hash[other[idx]] = true
      idx += 1
    end
    idx = 0
    len = size
    while idx < len
      v = self[idx]
      if hash[v]
        array << v
        hash.delete v
      end
      idx += 1
    end
    array
  end

  def -(other)
    raise TypeError, "can't convert #{other.class} into Array" unless other.is_a?(Array)

    hash = {}
    array = []
    idx = 0
    len = other.size
    while idx < len
      hash[other[idx]] = true
      idx += 1
    end
    idx = 0
    len = size
    while idx < len
      v = self[idx]
      array << v unless hash[v]
      idx += 1
    end
    array
  end

  def bsearch(&block)
    return to_enum :bsearch unless block

    idx = bsearch_index(&block)
    return self[idx] unless index.nil?

    nil
  end

  def bsearch_index(&block)
    return to_enum :bsearch_index unless block

    low = 0
    high = size
    satisfied = false

    while low < high
      mid = ((low + high) / 2).truncate
      res = block.call self[mid]

      case res
      when 0 # find-any mode: Found!
        return mid
      when Numeric # find-any mode: Continue...
        in_lower_half = res.negative?
      when true # find-min mode
        in_lower_half = true
        satisfied = true
      when false, nil # find-min mode
        in_lower_half = false
      else
        raise TypeError, 'invalid block result (must be numeric, true, false or nil)'
      end

      if in_lower_half
        high = mid
      else
        low = mid + 1
      end
    end

    satisfied ? low : nil
  end

  def combination(kcombinations, &block)
    size = self.size
    return to_enum(:combination, kcombinations) unless block
    return if n > size

    if kcombinations.zero?
      yield []
    elsif kcombinations == 1
      i = 0
      while i < size
        yield [self[i]]
        i += 1
      end
    else
      i = 0
      while i < size
        result = [self[i]]
        self[i + 1..-1].combination(n - 1) do |c|
          yield result + c
        end
        i += 1
      end
    end
  end

  def compact
    result = dup
    result.compact!
    result
  end

  def compact!
    result = reject(&:nil?)
    if result.size == size
      nil
    else
      replace(result)
    end
  end

  def delete_if(&block)
    return to_enum :delete_if unless block

    idx = 0
    while idx < size
      if block.call(self[idx])
        delete_at(idx)
      else
        idx += 1
      end
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

  def fetch(index, ifnone = NONE, &block)
    warn 'block supersedes default value argument' if !index.nil? && ifnone != NONE && block

    idx = index
    idx += size if idx.negative?
    if idx.negative? || size <= idx
      return block.call(index) if block
      raise IndexError, "index #{n} outside of array bounds: #{-size}...#{size}" if ifnone == NONE

      return ifnone
    end
    self[idx]
  end

  def fill(arg0 = nil, arg1 = nil, arg2 = nil, &block)
    raise ArgumentError, 'wrong number of arguments (0 for 1..3)' if arg0.nil? && arg1.nil? && arg2.nil? && !block

    beg = len = 0
    if block
      if arg0.nil? && arg1.nil? && arg2.nil?
        # ary.fill { |index| block }                    -> ary
        beg = 0
        len = size
      elsif !arg0.nil? && arg0.is_a?(Range)
        # ary.fill(range) { |index| block }             -> ary
        beg = arg0.begin
        beg += size if beg.negative?
        len = arg0.end
        len += size if len.negative?
        len += 1 unless arg0.exclude_end?
      elsif !arg0.nil?
        # ary.fill(start [, length] ) { |index| block } -> ary
        beg = arg0
        beg += size if beg.negative?
        len =
          if arg1.nil?
            size
          else
            arg0 + arg1
          end
      end
    elsif !arg0.nil? && arg1.nil? && arg2.nil?
      # ary.fill(obj)                                 -> ary
      beg = 0
      len = size
    elsif !arg0.nil? && !arg1.nil? && arg1.is_a?(Range)
      # ary.fill(obj, range )                         -> ary
      beg = arg1.begin
      beg += size if beg.negative?
      len = arg1.end
      len += size if len.negative?
      len += 1 unless arg1.exclude_end?
    elsif !arg0.nil? && !arg1.nil?
      # ary.fill(obj, start [, length])               -> ary
      beg = arg1
      beg += size if beg.negative?
      len =
        if arg2.nil?
          size
        else
          beg + arg2
        end
    end

    i = beg
    if block
      while i < len
        self[i] = block.call(i)
        i += 1
      end
    else
      while i < len
        self[i] = arg0
        i += 1
      end
    end
    self
  end

  def flatten(depth = nil)
    res = dup
    res.flatten! depth
    res
  end

  def flatten!(depth = nil)
    modified = false
    ar = []
    idx = 0
    len = size
    while idx < len
      e = self[idx]
      if e.is_a?(Array) && (depth.nil? || depth.positive?)
        ar += e.flatten(depth.nil? ? nil : depth - 1)
        modified = true
      else
        ar << e
      end
      idx += 1
    end
    replace(ar) if modified
  end

  def index(val = NONE, &block)
    return to_enum(:find_index, val) if !block && val == NONE

    if block
      each_with_index do |obj, idx|
        return idx if block.call(obj)
      end
    else
      each_with_index do |obj, idx|
        return idx if obj == val
      end
    end
    nil
  end

  def insert(idx, *args)
    idx += size + 1 if idx.negative?

    self[idx, 0] = args
    self
  end

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

  def keep_if(&block)
    return to_enum :keep_if unless block

    idx = 0
    while idx < size
      if block.call(self[idx])
        idx += 1
      else
        delete_at(idx)
      end
    end
    self
  end

  def permutation(kcombinations = size, &block)
    size = self.size
    return to_enum(:permutation, n) unless block
    return if n > size

    if kcombinations.zero?
      yield []
    else
      i = 0
      while i < size
        result = [self[i]]
        if (kcombinations - 1).positive?
          ary = self[0...i] + self[i + 1..-1]
          ary.permutation(kcombinations - 1) do |c|
            yield result + c
          end
        else
          yield result
        end
        i += 1
      end
    end
  end

  def reject!(&block)
    return to_enum :reject! unless block

    len = size
    idx = 0
    while idx < size
      if block.call(self[idx])
        delete_at(idx)
      else
        idx += 1
      end
    end
    if size == len
      nil
    else
      self
    end
  end

  def reverse_each(&block)
    return to_enum :reverse_each unless block

    i = size - 1
    while i >= 0
      block.call(self[i])
      i -= 1
    end
    self
  end

  def rotate(count = 1)
    ary = []
    len = length

    return ary unless len.positive?

    # rotate count
    idx =
      if count.negative?
        (len - (~count % len) - 1)
      else
        (count % len)
      end
    len.times do
      ary << self[idx]
      idx += 1
      idx = 0 if idx > len - 1
    end
  end

  def rotate!(count = 1)
    replace(rotate(count))
  end

  def select!(&block)
    return to_enum :select! unless block

    result = []
    idx = 0
    len = size
    while idx < len
      elem = self[idx]
      result << elem if block.call(elem)
      idx += 1
    end
    return nil if len == result.size

    replace(result)
  end

  def to_a
    self
  end

  def to_ary
    self
  end

  def to_h(&blk)
    h = {}
    each do |v|
      v = blk.call(v) if blk
      raise TypeError, "wrong element type #{v.class}" unless v.is_a?(Array)
      raise ArgumentError, "wrong array length (expected 2, was #{v.length})" unless v.length == 2

      h[v[0]] = v[1]
    end
    h
  end

  def transpose
    return [] if empty?

    column_count = nil
    each do |row|
      raise TypeError unless row.is_a?(Array)

      column_count ||= row.count
      raise IndexError, 'element size differs' unless column_count == row.count
    end

    Array.new(column_count) do |column_index|
      map { |row| row[column_index] }
    end
  end

  def union(*args)
    ary = dup
    args.each do |x|
      ary.concat(x)
      ary.uniq!
    end
    ary
  end

  def uniq(&block)
    ary = dup
    ary.uniq!(&block)
    ary
  end

  def uniq!(&block)
    hash = {}
    if block
      each do |val|
        key = block.call(val)
        hash[key] = val unless hash.key?(key)
      end
      result = hash.values
    else
      hash = {}
      each do |val|
        hash[val] = val
      end
      result = hash.keys
    end
    if result.size == size
      nil
    else
      replace(result)
    end
  end

  def |(other)
    raise TypeError, "can't convert #{other.class} into Array" unless other.is_a?(Array)

    ary = self + other
    ary.uniq! || ary
  end

  alias append push
  alias prepend unshift
end
