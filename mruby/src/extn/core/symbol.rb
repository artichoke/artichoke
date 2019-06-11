# frozen_string_literal: true

Symbol.class_eval do
  # https://ruby-doc.org/core-2.6.3/Symbol.html#method-i-5B-5D
  def [](*args)
    to_s[*args]
  end
end
