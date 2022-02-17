# frozen_string_literal: true

module Artichoke
  class Array
    # rubocop:disable Lint/HashCompareByIdentity
    def self.inspect(ary, recur_list)
      size = ary.size
      return '[]' if size.zero?
      return '[...]' if recur_list[ary.object_id]

      recur_list[ary.object_id] = true
      out = []
      i = 0
      while i < size
        elem = ary[i]
        out <<
          case elem
          when ::Array
            ::Artichoke::Array.inspect(elem, recur_list)
          when ::Hash
            ::Artichoke::Hash.inspect(elem, recur_list)
          else
            elem.inspect
          end

        i += 1
      end
      "[#{out.join(', ')}]"
    end
    # rubocop:enable Lint/HashCompareByIdentity
  end
end

class Array
  # include depends on Array#reverse which hasn't been defined yet so inline the
  # `Module` include.
  #
  # include Enumerable
  Enumerable.append_features(self)
  Enumerable.included(self)

  def self.try_convert(other)
    ary = other.to_ary
    return nil if ary.nil?
    unless ary.is_a?(Array)
      raise TypeError, "can't convert #{other.class} to Array (#{other.class}#to_ary gives #{ary.class})"
    end

    ary
  rescue NoMethodError
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

  def -(other)
    ary = other.to_ary if other.respond_to?(:to_ary)
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
    return nil unless other.is_a?(Array)

    len = length
    return len <=> other.length unless len == other.length

    idx = 0
    while idx < len
      if self[idx].equal?(other[idx])
        idx += 1
        next
      end
      cmp = self[idx] <=> other[idx]
      return false if cmp.nil?

      unless cmp.is_a?(Numeric)
        classname = other.class
        classname = other.inspect if other.nil? || other.equal?(false) || other.equal?(true) || other.is_a?(Numeric)
        raise ArgumentError, "Comparison of #{self.class} with #{classname} failed"
      end

      return cmp unless cmp.zero?

      idx += 1
    end
    0
  end

  def ==(other)
    return false unless other.is_a?(Array)
    return false unless length == other.length

    len = length
    idx = 0
    while idx < len
      left = self[idx]
      right = other[idx]
      idx += 1
      next if left.equal?(right)

      if left.is_a?(Comparable)
        cmp = left <=> right
        return false if cmp.nil?
        raise ArgumentError unless cmp.is_a?(Numeric)
        return false unless cmp.zero?
      else
        return false unless left == right
      end
    end
    true
  rescue NoMethodError
    false
  end

  def all?(pattern = (not_set = true), &block)
    if not_set
      idx = 0
      if block
        while idx < length
          return false unless block.call(self[idx])

          idx += 1
        end
      else
        len = length
        while idx < len
          return false unless self[idx]

          idx += 1
        end
      end
    else
      warn('warning: given block not used') if block

      len = length
      idx = 0
      while idx < len
        return false unless pattern === self[idx] # rubocop:disable Style/CaseEquality

        idx += 1
      end
    end
    true
  end

  def any?(pattern = (not_set = true), &block)
    if not_set
      idx = 0
      if block
        while idx < length
          return true if block.call(self[idx])

          idx += 1
        end
      else
        len = length
        while idx < len
          return true if self[idx]

          idx += 1
        end
      end
    else
      warn('warning: given block not used') if block

      len = length
      idx = 0
      while idx < len
        return true if pattern === self[idx] # rubocop:disable Style/CaseEquality

        idx += 1
      end
    end
    false
  end

  def assoc(obj)
    idx = 0
    len = length
    while idx < len
      ary = self[idx]
      idx += 1
      next unless ary.is_a?(Array)
      next unless ary.length.positive?

      return ary if ary.first == obj
    end
    nil
  end

  def at(index)
    raise TypeError, 'no implicit conversion from nil to integer' if index.nil?

    idx =
      if index.is_a?(Integer)
        index
      elsif index.respond_to?(:to_int)
        classname = index.class
        classname = index.inspect if index.equal?(false) || index.equal?(true)
        idx = index.to_int
        unless idx.is_a?(Integer)
          raise TypeError, "can't convert #{classname} to Integer (#{classname}#to_int gives #{idx.class})"
        end

        idx
      else
        classname = index.class
        classname = index.inspect if index.equal?(false) || index.equal?(true)
        raise TypeError, "no implicit conversion of #{classname} into Integer"
      end

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

  def collect(&block)
    return to_enum :collect unless block

    ary = []
    idx = 0
    while idx < length
      ary << block.call(self[idx])
      idx += 1
    end
    ary
  end

  def collect!(&block)
    return to_enum :collect! unless block
    raise FrozenError, "can't modify frozen Array" if frozen?

    idx = 0
    while idx < length
      self[idx] = block.call(self[idx])
      idx += 1
    end
    self
  end

  def combination(k, &block) # rubocop:disable Naming/MethodParameterName
    k =
      if k.is_a?(Integer)
        k
      elsif k.nil?
        raise TypeError, 'no implicit conversion from nil to integer'
      elsif k.respond_to?(:to_int)
        classname = k.class
        classname = k.inspect if k.equal?(false) || k.equal?(true)
        k = k.to_int
        unless k.is_a?(Integer)
          raise TypeError, "can't convert #{classname} to Integer (#{classname}#to_int gives #{k.class})"
        end

        k
      else
        classname = k.class
        classname = k.inspect if k.equal?(false) || k.equal?(true)
        raise TypeError, "no implicit conversion of #{classname} into Integer"
      end

    return to_enum(:combination, k) unless block
    return self if k > length
    return self if k.negative?

    if k.zero?
      block.call([])
    elsif k == 1
      ary = dup
      len = length
      idx = 0
      while idx < len
        block.call([ary[idx]])
        idx += 1
      end
    elsif k == length
      block.call(dup)
    else
      ary = dup
      len = length
      indexes = (0...k).to_a
      incr = k - 1
      loop do
        while indexes[incr] < len
          block.call(indexes.map { |i| ary[i] })
          indexes[incr] += 1
        end

        reset = incr
        until reset.negative?
          reset -= 1
          prev = indexes[reset]
          break unless prev + 1 >= len
        end
        base = indexes[reset] + 1
        replace = k - reset
        indexes[reset, replace] = (base...(base + replace)).to_a
        break if indexes[0] + k > len

        incr = k - 1
      end
    end
    self
  end

  def compact
    reject(&:nil?)
  end

  def compact!
    raise FrozenError, "can't modify frozen Array" if frozen?

    reject!(&:nil?)
  end

  def count(obj = (not_set = true), &block)
    count = 0
    idx = 0
    len = length
    if not_set
      return len unless block

      while idx < len
        item = self[idx]
        count += 1 if block.call(item)
        idx += 1
      end
    else
      warn('warning: given block not used') if block

      while idx < len
        item = self[idx]
        count += 1 if obj == item
        idx += 1
      end
    end
    count
  end

  def cycle(num = nil, &block)
    return to_enum(:cycle, num) unless block

    if num.nil?
      return nil if empty?

      while true
        idx = 0
        len = length
        while idx < len
          block.call(self[idx])
          idx += 1
        end
      end
    else
      count =
        if num.is_a?(Integer)
          num
        elsif num.respond_to?(:to_int)
          classname = num.class
          classname = num.inspect if num.equal?(false) || num.equal?(true)
          num = num.to_int
          unless num.is_a?(Integer)
            raise TypeError, "can't convert #{classname} to Integer (#{classname}#to_int gives #{num.class})"
          end

          num
        else
          classname = num.class
          classname = num.inspect if num.equal?(false) || num.equal?(true)
          raise TypeError, "no implicit conversion of #{classname} into Integer"
        end
      return nil unless count.positive?

      iteration = 0
      while iteration < count
        idx = 0
        len = length
        while idx < len
          block.call(self[idx])
          idx += 1
        end
        iteration += 1
      end
    end
  end

  def delete(key, &block)
    sentinel = Object.new
    ret = sentinel
    while (i = index(key))
      ret = delete_at(i)
    end

    return block.call if ret.equal?(sentinel) && block
    return nil if ret.equal?(sentinel)

    ret
  end

  def delete_at(index)
    raise FrozenError, "can't modify frozen Array" if frozen?

    index =
      if index.is_a?(Integer)
        index
      elsif index.nil?
        raise TypeError, 'no implicit conversion from nil to integer'
      elsif index.respond_to?(:to_int)
        classname = index.class
        classname = index.inspect if index.equal?(false) || index.equal?(true)
        index = index.to_int
        unless index.is_a?(Integer)
          raise TypeError, "can't convert #{classname} to Integer (#{classname}#to_int gives #{index.class})"
        end

        index
      else
        classname = index.class
        classname = index.inspect if index.equal?(false) || index.equal?(true)
        raise TypeError, "no implicit conversion of #{classname} into Integer"
      end
    index += length if index.negative?
    return nil if index >= length
    return nil if index.negative?

    ret = self[index]
    self[index, 1] = []
    ret
  end

  def delete_if(&block)
    return to_enum :delete_if unless block
    raise FrozenError, "can't modify frozen Array" if frozen?

    idx = 0
    delete_indexes = []
    while idx < size
      delete_indexes << idx if block.call(self[idx])

      idx += 1
    end
    delete_indexes.reverse_each { |index| delete_at(index) }
    self
  end

  def dig(idx, *args)
    item = self[idx]
    if args.empty?
      item
    else
      item&.dig(*args)
    end
  end

  def drop(num)
    count =
      if num.is_a?(Integer)
        num
      elsif num.nil?
        raise TypeError, 'no implicit conversion from nil to integer'
      elsif num.respond_to?(:to_int)
        classname = num.class
        classname = num.inspect if index.equal?(false) || index.equal?(true)
        num = num.to_int
        unless num.is_a?(Integer)
          raise TypeError, "can't convert #{classname} to Integer (#{classname}#to_int gives #{num.class})"
        end

        num
      else
        classname = num.class
        classname = num.inspect if index.equal?(false) || index.equal?(true)
        raise TypeError, "no implicit conversion of #{classname} into Integer"
      end
    raise ArgumentError, 'attempt to drop negative size' if count.negative?
    return self if count.zero?

    self[0, count] = []
    self
  end

  def drop_while(&block)
    return to_enum(:drop_while) unless block

    drop_until = 0
    idx = 0
    while idx < length
      drop = block.call(self[idx])
      drop_until += 1 if drop
      break unless drop

      idx += 1
    end
    self[0, drop_until] = []
    self
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

  def empty?
    length.zero?
  end

  def eql?(other)
    return true if equal?(other)
    return false unless other.is_a?(Array)
    return false if length != other.length

    len = length
    i = 0
    while i < len
      s = self[i]
      o = other[i]
      i += 1
      return false unless s.eql?(o)
    end
    true
  end

  def fetch(index, default = (not_set = true), &block)
    warn 'block supersedes default value argument' if !index.nil? && !not_set && block

    idx = index
    idx += size if idx.negative?
    if idx.negative? || size <= idx
      return block.call(index) if block
      raise IndexError, "index #{idx} outside of array bounds: #{-size}...#{size}" if not_set

      return default
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

    raise ArgumentError, 'argument too big' if len > (2**(0.size * 8)) - 1

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

  def filter(&block)
    return to_enum(:filter) unless block

    res = []
    idx = 0
    len = length
    while idx < len
      item = self[idx]
      res << item if block.call(item).equal?(true)
      idx += 1
    end
    res
  end

  def filter!(&block)
    return to_enum(:filter!) unless block

    res = filter(&block)
    return nil if length == res.length

    self[0, length] = res
  end

  def find_index(obj = (not_set = true), &block)
    return to_enum(:find_index, obj) if !block && not_set

    idx = 0
    len = length
    if not_set
      while idx < len
        item = self[idx]
        return idx if block.call(item).equal?(true)

        idx += 1
      end
    else
      warn('warning: given block not used') if block

      while idx < len
        item = self[idx]
        return idx if obj == item

        idx += 1
      end
    end
    nil
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
    len = length
    while idx < len
      e = self[idx]
      if e.is_a?(Array) && (depth.nil? || depth.positive?)
        ar.concat(e.flatten(depth.nil? ? nil : depth - 1))
        modified = true
      else
        ar << e
      end
      idx += 1
    end
    self[0, len] = ar if modified
  end

  def include?(object)
    idx = 0
    len = length
    while idx < len
      return true if self[idx] == object

      idx += 1
    end
    false
  end

  def index(val = (not_set = true), &block)
    return to_enum(:index) if !block && not_set # rubocop:disable Lint/ToEnumArguments

    idx = 0
    if not_set
      while idx < length
        return idx if block.call(self[idx])

        idx += 1
      end
    else
      warn('warning: given block not used') if block

      len = length
      while idx < len
        return idx if self[idx] == val

        idx += 1
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
    ::Artichoke::Array.inspect(self, {})
  end

  def join(separator = $,) # rubocop:disable Style/SpecialGlobalVars
    classname = separator.class
    classname = separator.inspect if separator.equal?(true) || separator.equal?(false)

    separator = '' if separator.nil?
    sep = String.try_convert(separator)
    raise "No implicit conversion of #{classname} into String" if sep.nil?

    s = +''
    idx = 0
    len = size
    while idx < len
      s << self[idx].to_s
      s << sep if idx < len - 1
      idx += 1
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

  def max(*)
    raise NotImplementedError
  end

  def min(*)
    raise NotImplementedError
  end

  def none?(pattern = (not_set = true), &block)
    if not_set
      idx = 0
      if block
        while idx < length
          return false if block.call(self[idx]).equal?(true)

          idx += 1
        end
      else
        len = length
        while idx < len
          return false if self[idx].equal?(true)

          idx += 1
        end
      end
    else
      warn('warning: given block not used') if block

      len = length
      idx = 0
      while idx < len
        return false if pattern === self[idx] # rubocop:disable Style/CaseEquality

        idx += 1
      end
    end
    true
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

  def product(*_args)
    raise NotImplementedError
  end

  def push(*args)
    raise FrozenError, "can't modify frozen Array" if frozen?

    concat(args)
    self
  end

  def rassoc(obj)
    idx = 0
    len = length
    while idx < len
      ary = self[idx]
      idx += 1
      next unless ary.is_a?(Array)
      next unless ary.length.positive?

      return ary if ary[1] == obj
    end
    nil
  end

  def reject(&block)
    return to_enum :reject unless block

    ary = []
    idx = 0
    while idx < length
      item = self[idx]
      ary << item unless block.call(item)
      idx += 1
    end
    ary
  end

  def reject!(&block)
    return to_enum :reject! unless block

    ary = []
    idx = 0
    modified = false
    while idx < length
      item = self[idx]
      if block.call(item)
        modified = true
      else
        ary << item
      end
      idx += 1
    end
    return nil unless modified

    self[0, length] = ary
    self
  end

  def repeated_combination(_num)
    raise NotImplementedError
  end

  def repeated_permutation(_num)
    raise NotImplementedError
  end

  def replace(other)
    classname = other.class
    classname = other.inspect if other.equal?(true) || other.equal?(false) || other.nil?
    ary =
      if other.is_a?(Array)
        other
      elsif other.respond_to?(:to_ary)
        other = other.to_ary
        unless other.is_a?(Array)
          raise TypeError, "can't convert #{classname} to Array (#{classname}#to_ary gives #{other.class})"
        end

        other
      else
        raise TypeError, "no implicit conversion of #{classname} into Array" unless other.is_a?(Array)
      end
    self[0, length] = ary
    self
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

  def rindex(val = (not_set = true), &block)
    return to_enum(:rindex) if !block && not_set # rubocop:disable Lint/ToEnumArguments

    if not_set
      reverse.index(&block)
    else
      reverse.index(val, &block)
    end
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

  def sample(*_args)
    raise NotImplementedError, 'TODO implement in Rust'
  end

  def select(&block)
    return to_enum :select unless block

    dup.tap { |ary| ary.select!(&block) }
  end

  def select!(&block)
    return to_enum :select! unless block
    raise FrozenError, "can't modify frozen Array" if frozen?

    result = []
    idx = 0
    skipped = false
    while idx < length
      elem = self[idx]
      if block.call(elem)
        result << elem
      else
        skipped = true
      end
      idx += 1
    end
    return nil unless skipped

    replace(result)
  end

  def shuffle(random: (not_set = true))
    random = Random::DEFAULT if not_set
    shuffled_orders = (0...size).map { |idx| [random.rand, idx] }.sort { |a, b| a[0] <=> b[0] }
    shuffled_orders.map { |_n, idx| idx }.map do |idx|
      self[idx]
    end
  end

  def shuffle!(random: (not_set = true))
    raise FrozenError, "can't modify frozen Array" if frozen?
    return self if length <= 1

    self[0, length] =
      if not_set
        shuffle
      else
        shuffle(random: random)
      end
    self
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
    return dup if length <= 1

    block ||= ->(a, b) { a <=> b }
    if length == 2
      l = self[0]
      r = self[1]
      begin
        cmp = block.call(l, r)
        return dup if cmp <= 0
        return reverse if cmp > 0 # rubocop:disable Style/NumericPredicate
      rescue StandardError
        raise ArgumentError, "comparison of #{l.class} with #{r.class} failed"
      end
      raise ArgumentError, "comparison of #{l.class} with #{r.class} failed"
    end

    ary = dup
    middle = (ary.length / 2).to_i
    left = ary[0...middle].sort(&block)
    right = ary[middle..-1].sort(&block)

    # merge
    result = []
    until left.empty? || right.empty?
      # change the direction of this comparison to change the direction of the sort
      l = left[0]
      r = right[0]

      begin
        cmp = block.call(l, r)
        result <<
          if cmp <= 0
            left.shift
          elsif cmp > 0 # rubocop:disable Style/NumericPredicate
            right.shift
          else
            raise ArgumentError, "comparison of #{l.class} with #{r.class} failed"
          end
      rescue StandardError
        raise ArgumentError, "comparison of #{l.class} with #{r.class} failed"
      end
    end
    result + left + right
  end

  def sort!(&block)
    raise FrozenError, "can't modify frozen Array" if frozen?
    return self if length <= 1

    self[0, length] = sort(&block)
    self
  end

  def sort_by!(&block)
    raise FrozenError, "can't modify frozen Array" if frozen?
    return to_enum(:sort_by!) unless block
    return self if length <= 1

    sort! { |left, right| block.call(left) <=> block.call(right) }
  end

  def sum(init = 0, &block)
    idx = 0
    sum = init
    while idx < length
      item = self[idx]
      item = block.call(item) if block

      classname = item.class
      classname = item.inspect if item.equal?(true) || item.equal?(false) || item.nil?
      raise TypeError, "#{classname} can't be coerced into Integer" unless item.respond_to?(:to_i)

      item = item.to_i
      raise TypeError, "#{classname} can't be coerced into Integer" unless item.is_a?(Integer)

      sum += item
      idx += 1
    end
    sum
  end

  def to_a
    return self if instance_of?(Array)

    [].concat(self)
  end

  def to_ary
    self
  end

  def to_h(&blk)
    h = {}
    idx = 0
    len = length
    while idx < len
      v = self[idx]
      v = blk.call(v) if blk
      v =
        if v.is_a?(Array)
          v
        elsif v.respond_to?(:to_ary)
          val = v.to_ary
          unless v.is_a?(Array)
            raise TypeError, "can't convert #{v.class} to Array (#{v.class}#to_ary gives #{val.class})"
          end

          val
        else
          raise TypeError, "wrong element type #{v.class} at #{idx} (expected array)"
        end

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

  def unshift(*args)
    self[0, 0] = args
    self
  end

  def values_at(*selectors)
    ary = []
    idx = 0
    len = selectors.length
    while idx < len
      selector = selectors[idx]
      case selector
      when Integer
        ary << self[selector]
      when Range
        ary.concat(self[selector])
      else
        classname = selector.class
        classname = selector.inspect if selector.equal?(true) || selector.equal?(false) || selector.nil?
        raise TypeError, "No implicit conversion from #{classname} to Integer" unless selector.respond_to?(:to_int)

        selector = selector.to_int
        unless selector.is_a?(Integer)
          raise TypeError, "can't convert #{classname} to Integer (#{classname}#to_int gives #{selector.class})"
        end

        ary << self[selector]
      end
      idx += 1
    end
    ary
  end

  def zip(*_args)
    raise NotImplementedError
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
  alias slice []
  alias take drop
  alias take_while drop_while
  alias to_s inspect
end
