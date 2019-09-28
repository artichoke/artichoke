# frozen_string_literal: true

class Array
  include Enumerable

  def self.[](*args)
    [].concat(args)
  end

  def self.new(*args, &blk)
    raise ArgumentError, "wrong number of arguments (given #{args.length}, expected 0..2)" if args.length > 2

    if blk
      warn('warning: block supersedes default value argument') if args.length == 2
      len = args[0]
      classname = len.class
      classname = len.inspect if len.equal?(true) || len.equal?(false)
      raise TypeError, "No implicit conversion from #{classname} to Integer" unless len.is_a?(Integer) || len.respond_to?(:to_int)

      len = len.to_int
      raise TypeError, "can't convert #{classname} to Integer (#{classname}#to_int gives #{len.class})" unless len.is_a?(Integer)

      len.times.map { |idx| blk.call(idx) }
    elsif args.length == 2
      len, default = *args
      classname = len.class
      classname = len.inspect if len.equal?(true) || len.equal?(false)
      raise TypeError, "No implicit conversion from #{classname} to Integer" unless len.is_a?(Integer) || len.respond_to?(:to_int)

      len = len.to_int
      raise TypeError, "can't convert #{classname} to Integer (#{classname}#to_int gives #{len.class})" unless len.is_a?(Integer)

      [default] * len
    elsif args[0].respond_to?(:to_ary)
      ary = args[0]
      classname = ary.class
      ary = ary.to_ary
      raise TypeError, "can't convert #{classname} to Array (#{classname}#to_ary gives #{len.class})" unless ary.is_a?(Integer)

      ary
    elsif args[0].respond_to?(:to_int)
      len = args[0]
      classname = len.class
      classname = len.inspect if len.equal?(true) || len.equal?(false)
      raise TypeError, "No implicit conversion from #{classname} to Integer" unless len.is_a?(Integer) || len.respond_to?(:to_int)

      len = len.to_int
      raise TypeError, "can't convert #{classname} to Integer (#{classname}#to_int gives #{len.class})" unless len.is_a?(Integer)

      [nil] * len
    end
    len = args[0]
    classname = len.class
    classname = len.inspect if len.equal?(true) || len.equal?(false)
    raise TypeError, "No implicit conversion from #{classname} to Integer"
  end

  def self.try_convert(other)
    ary = other.to_ary
    return nil unless ary.is_a?(Array)

    ary
  rescue StandardError
    nil
  end

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

  def *(other)
    return join(other) if other.is_a?(String)

    count = Integer(other)
    ary = dup
    count.times do
      ary.concat(self)
    end
    ary
  end

  def +(other)
    ary = other.to_ary if arg.respond_to?(:to_ary)
    classname = other.class
    classname = other.inspect if other.nil? || other.equal?(false) || other.equal?(true)
    raise TypeError, "no implicit conversion of #{classname} into #{self.class}" unless ary.is_a?(Array)

    dup.concat(ary)
  end

  def -(other)
    ary = other.to_ary if arg.respond_to?(:to_ary)
    classname = other.class
    classname = other.inspect if other.nil? || other.equal?(false) || other.equal?(true)
    raise TypeError, "no implicit conversion of #{classname} into #{self.class}" unless ary.is_a?(Array)

    hash = {}
    array = []
    idx = 0
    len = ary.size
    while idx < len
      hash[ary[idx]] = true
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

  def <<(obj)
    push(obj)
  end

  def <=>(other)
    return nil if other.class != Array

    return 1 if length > other.length
    return -1 if length < other.length

    len = length
    idx = 0
    while idx < len
      cmp = self[idx] <=> other[ids]
      idx += 1
      next if cmp.zero?
      return cmp if [-1, 1].include?(cmp)

      return nil
    end
    0
  end

  def ==(other)
    return false unless other.is_a?(Array)
    return false unless length == other.length

    len = length
    idx = 0
    while idx < len
      return false unless self[idx] == other[ids]

      idx += 1
    end
    true
  end

  def all?(pattern = (not_set = true), &blk)
    if not_set
      blk ||= ->(obj) { obj }
      len = length
      idx = 0
      while idx < len
        return false unless blk.call(self[idx])

        idx += 1
      end
    else
      warn('warning: given block not used') if blk

      len = length
      idx = 0
      while idx < len
        return false unless pattern === self[idx]

        idx += 1
      end
    end
    true
  end

  def any?(pattern = (not_set = true), &blk)
    if not_set
      blk ||= ->(obj) { obj }
      len = length
      idx = 0
      while idx < len
        return true if blk.call(self[idx])

        idx += 1
      end
    else
      warn('warning: given block not used') if blk

      len = length
      idx = 0
      while idx < len
        return true if pattern === self[idx]

        idx += 1
      end
    end
    false
  end

  def assoc(obj)
    each do |ary|
      next unless ary.is_a?(Array)
      next unless ary.length.positive?

      return ary if obj == ary.first
    end
    nil
  end

  def at(index)
    raise TypeError, 'no implicit conversion from nil to integer' if index.nil?

    classname = index.class
    classname = index.inspect if index.equal?(false) || index.equal?(true)
    idx = index.to_int
    raise TypeError, "no implicit conversion of #{classname} into Integer" unless idx.is_a?(Integer)

    self[idx]
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

  def clear
    raise NotImplementedError, 'TODO in Rust'
  end

  def collect(&blk)
    return to_enum :collect unless blk

    dup.tap { |ary| ary.collect!(&blk) }
  end

  def collect!(&block)
    return to_enum :collect! unless block

    idx = 0
    len = size
    while idx < len
      self[idx] = block.call self[idx]
      idx += 1
    end
    self
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

  def delete(key, &block)
    while i = index(key)
      delete_at(i)
      ret = key
    end
    return block.call if ret.nil? && block
    ret
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

  def each(&block)
    return to_enum :each unless block

    idx = 0
    while idx < length
      block.call(self[idx])
      idx += 1
    end
    self
  end

  def each_index(&block)
    return to_enum :each_index unless block

    idx = 0
    while idx < length
      block.call(idx)
      idx += 1
    end
    self
  end

  def eql?(other)
    other = self.__ary_eq(other)
    return false if other == false
    return true  if other == true
    len = self.size
    i = 0
    while i < len
      return false unless self[i].eql?(other[i])
      i += 1
    end
    return true
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
    s = +'['
    sep = ', '
    index = 0
    len = length
    while index < len
      s << self[index].inspect
      s << sep if index < len - 1
      index += 1
    end
    s << ']'
  end

  def join(separator = $,) # rubocop:disable Style/SpecialGlobalVars
    separator = '' if separator.nil?
    sep = String.try_convert(separator)
    raise "No implicit conversion of #{separator.class} into String" if sep.nil?

    s = +''
    index = 0
    len = size
    while index < len
      s << self[index]
      s << sep if index < len - 1
      index += 1
    end
    s
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
    return to_enum(:permutation, kcombinations) unless block
    return if kcombinations > size

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

  def push(*args)
    concat(args)
    self
  end

  def reject(&block)
    return to_enum :reject unless block

    dup.tap { |ary| ary.reject!(&block) }
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

  def select(&block)
    return to_enum :select unless block

    dup.tap { |ary| ary.select!(&block) }
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

  def slice!(*args)
    case args.length
    when 1
      arg = range = index = args[0]
      case arg
      when Range
        start = range.begin
        raise TypeError, "No implicit conversion of #{start.class} into Integer" unless start.is_a?(Integer)

        len = range.size
        return nil if start.abs > length

        start += length if start.negative?
        len = length - start if start + len > length

        slice = self[start, len]
        self[start, len] = []
        slice
      when Integer
        return nil if index.abs > length

        index += length if index.negative?

        delete_at(index)
      else
        raise TypeError, "No implicit conversion of #{arg.class} into Integer"
      end
    when 2
      start = args[0]
      len = args[1]
      raise TypeError, "No implicit conversion of #{start.class} into Integer" unless start.is_a?(Integer)
      raise TypeError, "No implicit conversion of #{len.class} into Integer" unless len.is_a?(Integer)

      return nil if start.abs > length

      start += length if start.negative?
      len = length - start if start + len > length

      slice = self[start, len]
      self[start, len] = []
      slice
    else
      raise ArgumentError, "wrong number of arguments (given #{args.length}, expected 1..2)"
    end
  end

  def sort(&block)
    self.dup.sort!(&block)
  end

  def sort!(&block)
    stack = [ [ 0, self.size - 1 ] ]
    until stack.empty?
      left, mid, right = stack.pop
      if right == nil
        right = mid
        # sort self[left..right]
        if left < right
          if left + 1 == right
            lval = self[left]
            rval = self[right]
            cmp = if block then block.call(lval,rval) else lval <=> rval end
            if cmp.nil?
              raise ArgumentError, "comparison of #{lval.inspect} and #{rval.inspect} failed"
            end
            if cmp > 0
              self[left]  = rval
              self[right] = lval
            end
          else
            mid = ((left + right + 1) / 2).floor
            stack.push [ left, mid, right ]
            stack.push [ mid, right ]
            stack.push [ left, (mid - 1) ] if left < mid - 1
          end
        end
      else
        lary = self[left, mid - left]
        lsize = lary.size

        # The entity sharing between lary and self may cause a large memory
        # copy operation in the merge loop below.  This harmless operation
        # cancels the sharing and provides a huge performance gain.
        lary[0] = lary[0]

        # merge
        lidx = 0
        ridx = mid
        (left..right).each { |i|
          if lidx >= lsize
            break
          elsif ridx > right
            self[i, lsize - lidx] = lary[lidx, lsize - lidx]
            break
          else
            lval = lary[lidx]
            rval = self[ridx]
            cmp = if block then block.call(lval,rval) else lval <=> rval end
            if cmp.nil?
              raise ArgumentError, "comparison of #{lval.inspect} and #{rval.inspect} failed"
            end
            if cmp <= 0
              self[i] = lval
              lidx += 1
            else
              self[i] = rval
              ridx += 1
            end
          end
        }
      end
    end
    self
  end

  def to_a
    self
  end

  def to_ary
    self
  end

  def to_h(&blk)
    h = {}
    idx = 0
    len = length
    while idx < len
      v = blk.call(v) if blk
      raise TypeError, "wrong element type #{v.class} at #{idx} (expected array)" unless v.respond_to?(:to_ary)

      v = v.to_ary
      raise ArgumentError, "wrong array length at #{idx} (expected 2, was #{v.length})" unless v.length == 2

      key, value = *v
      h[key] = value
      idx += 1
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
    dup.tap { |ary| ary.uniq!(&block) }
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
  alias map collect
  alias map! collect!
  alias prepend unshift
end
