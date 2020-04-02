# frozen_string_literal: true

require 'abbrev'

# specs taken from stdlib documentation
# https://ruby-doc.org/stdlib-2.6.3/libdoc/abbrev/rdoc/Abbrev.html
def spec
  abbrev
  abbrev_multiple
  abbrev_array
  abbrev_abbrev
  abbrev_abbrev_pattern

  true
end

def abbrev
  expect = {
    'ruby' => 'ruby',
    'rub' => 'ruby',
    'ru' => 'ruby',
    'r' => 'ruby'
  }
  result = Abbrev.abbrev(['ruby'])
  raise unless expect == result
end

def abbrev_multiple
  expect = {
    'ruby' => 'ruby',
    'rub' => 'ruby',
    'rules' => 'rules',
    'rule' => 'rules',
    'rul' => 'rules'
  }
  result = Abbrev.abbrev(%w[ruby rules])
  raise unless expect == result
end

def abbrev_array
  expect = {
    'summer' => 'summer',
    'summe' => 'summer',
    'summ' => 'summer',
    'sum' => 'summer',
    'su' => 'summer',
    's' => 'summer',
    'winter' => 'winter',
    'winte' => 'winter',
    'wint' => 'winter',
    'win' => 'winter',
    'wi' => 'winter',
    'w' => 'winter'
  }
  result = %w[summer winter].abbrev
  raise unless expect == result
end

def abbrev_abbrev
  expect = { 'ca' => 'car', 'con' => 'cone', 'co' => 'cone', 'car' => 'car', 'cone' => 'cone' }
  result = Abbrev.abbrev(%w[car cone])
  raise unless expect == result
end

def abbrev_abbrev_pattern
  expect = { 'box' => 'box', 'bo' => 'box', 'b' => 'box', 'crab' => 'crab' }
  result = Abbrev.abbrev(%w[car box cone crab], /b/)
  raise unless expect == result

  expect = { 'car' => 'car', 'ca' => 'car' }
  result = Abbrev.abbrev(%w[car box cone], 'ca')
  raise unless expect == result
end

spec if $PROGRAM_NAME == __FILE__
