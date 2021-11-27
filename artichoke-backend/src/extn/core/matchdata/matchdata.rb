# frozen_string_literal: true

class MatchData
  def ==(other)
    return false unless other.is_a?(MatchData)
    return false unless string == other.string
    return false unless regexp == other.regexp
    return false unless offset(0) == other.offset(0)

    true
  end

  def eql?(other)
    self == other
  end

  def inspect
    s = %(#<MatchData "#{self[0]}")
    if names.empty?
      puts captures.inspect
      captures.each_with_index do |capture, index|
        s << %( #{index + 1}:"#{capture || nil.inspect}")
      end
    else
      names.each do |name|
        s << %( #{name}:#{self[name].inspect})
      end
    end
    s << '>'
  end

  def values_at(*indexes)
    indexes.map { |index| self[index] }.flatten
  end
end
