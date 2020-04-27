# frozen_string_literal: true

# Tests from String core docs in Ruby 2.6.3
# https://ruby-doc.org/core-2.6.3/String.html
def spec
  string_match_operator
  string_element_reference_regexp
  string_scan
  string_unary_minus

  true
end

def string_match_operator
  match = "cat o' 9 tails" =~ /\d/
  raise unless match == 7

  match = "cat o' 9 tails" =~ 9
  raise unless match.nil?
end

# rubocop:disable Style/GuardClause
def string_element_reference_regexp
  raise unless 'hello there'[/[aeiou](.)\1/] == 'ell'
  raise unless 'hello there'[/[aeiou](.)\1/, 0] == 'ell'
  raise unless 'hello there'[/[aeiou](.)\1/, 1] == 'l'
  raise unless 'hello there'[/[aeiou](.)\1/, 2].nil?
  unless 'hello there'[/(?<vowel>[aeiou])(?<non_vowel>[^aeiou])/, 'non_vowel'] == 'l'
    raise
  end
  unless 'hello there'[/(?<vowel>[aeiou])(?<non_vowel>[^aeiou])/, 'vowel'] == 'e'
    raise
  end
end
# rubocop:enable Style/GuardClause

def string_scan
  s = 'abababa'
  raise unless s.scan(/./) == %w[a b a b a b a]
  raise unless s.scan(/../) == %w[ab ab ab]
  raise unless s.scan('aba') == %w[aba aba]
  raise unless s.scan('no no no') == []
end

def string_unary_minus
  s = -'abababa'
  raise unless s.frozen?
  raise unless s.itself == 'abababa'
end

spec if $PROGRAM_NAME == __FILE__
