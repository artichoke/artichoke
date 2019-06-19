# frozen_string_literal: true

class Regexp
  def ~
    self =~ $_ # rubocop:disable Style/SpecialGlobalVars
  end
end
