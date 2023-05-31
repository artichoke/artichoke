# frozen_string_literal: true

def spec
  offset_returns_utf8_character_index

  true
end

def offset_returns_utf8_character_index
  raise unless 'тест'.match('с').offset(0) == [2, 3]
end
