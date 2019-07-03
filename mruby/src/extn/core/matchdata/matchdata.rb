# frozen_string_literal: true

class MatchData
  def values_at(*indexes)
    indexes.map { |index| self[index] }.flatten
  end
end
