# frozen_string_literal: true

String.class_eval do
  # https://ruby-doc.org/core-2.6.3/String.html#method-i-3D-7E
  def =~(other)
    return other.match(self).begin(0) if other.is_a?(Regexp)
    return other =~ self if other.respond_to?(:=~)

    nil
  end
end
