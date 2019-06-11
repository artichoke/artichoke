# frozen_string_literal: true

String.class_eval do
  alias_method :__old_idx, :[]

  # https://ruby-doc.org/core-2.6.3/String.html#method-i-5B-5D
  def [](*args)
    if (regexp = args[0]).is_a?(Regexp)
      capture = args[1] || 0
      return regexp.match(self)&.[](capture)
    end
    __old_idx(*args)
  end

  # https://ruby-doc.org/core-2.6.3/String.html#method-i-3D-7E
  def =~(other)
    return other.match(self).begin(0) if other.is_a?(Regexp)
    return other =~ self if other.respond_to?(:=~)

    nil
  end
end
