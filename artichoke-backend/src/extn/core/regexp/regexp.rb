# frozen_string_literal: true

class Regexp
  def self.last_match(*args)
    # rubocop:disable Style/SpecialGlobalVars
    return nil if $~.nil?
    return $~ if args.empty?

    $~[*args]
    # rubocop:enable Style/SpecialGlobalVars
  end

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
