# frozen_string_literal: true

class BasicObject
  # rubocop:disable Style/RedundantConditional
  # rubocop:disable Style/IfWithBooleanLiteralBranches
  def !=(other)
    if self == other
      false
    else
      true
    end
  end
  # rubocop:enable Style/IfWithBooleanLiteralBranches
  # rubocop:enable Style/RedundantConditional
end
