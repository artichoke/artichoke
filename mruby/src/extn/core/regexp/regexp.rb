# frozen_string_literal: true

class Regexp
  def self.try_convert(obj = nil)
    raise ArgumentError if obj.nil?
    return obj if obj.is_a?(Regexp)

    obj.to_regexp
  rescue StandardError
    nil
  end

  def ~
    self =~ $_ # rubocop:disable Style/SpecialGlobalVars
  end
end
