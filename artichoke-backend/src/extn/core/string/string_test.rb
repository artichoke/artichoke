# frozen_string_literal: true

# Tests from String core docs in Ruby 2.6.3
# https://ruby-doc.org/core-2.6.3/String.html
def spec
  string_match_operator
  string_element_reference_regexp
  string_byteslice
  string_scan
  string_unary_minus
  string_reverse
  string_tr

  true
end

def string_match_operator
  match = "cat o' 9 tails" =~ /\d/
  raise unless match == 7

  match = "cat o' 9 tails" =~ 9
  raise unless match.nil?
end

def string_element_reference_regexp
  raise unless 'hello there'[/[aeiou](.)\1/] == 'ell'
  raise unless 'hello there'[/[aeiou](.)\1/, 0] == 'ell'
  raise unless 'hello there'[/[aeiou](.)\1/, 1] == 'l'
  raise unless 'hello there'[/[aeiou](.)\1/, 2].nil?
  raise unless 'hello there'[/(?<vowel>[aeiou])(?<non_vowel>[^aeiou])/, 'non_vowel'] == 'l'
  raise unless 'hello there'[/(?<vowel>[aeiou])(?<non_vowel>[^aeiou])/, 'vowel'] == 'e'
end

def string_byteslice
  s = 'abcdefghijk'
  #scalar
  raise unless s.byteslice(0, 1000) == 'abcdefghijk'
  raise unless s.byteslice(5, 1000) == 'fghijk'
  raise unless s.byteslice(20, 1000).nil?
  raise unless s.byteslice(-5, 1000) == 'ghijk'
  raise unless s.byteslice(-25, 1000).nil?
  raise unless s.byteslice(-25).nil?
  raise unless s.byteslice(-5) == 'g'
  raise unless s.byteslice(-5, 10) == 'ghijk'
  raise unless s.byteslice(0) == 'a'
  raise unless s.byteslice(2) == 'c'
  raise unless s.byteslice(0, 5) == 'abcde'
  raise unless s.byteslice(5, 3) == 'fgh'
  raise unless s.byteslice(5, -10).nil?
  raise unless s.byteslice(5, -2).nil?
  # range
  raise unless s.byteslice(1..4) == 'bcde'
  raise unless s.byteslice(1..-1) == 'bcdefghijk'
  raise unless s.byteslice(10..-1) == 'k'
  raise unless s.byteslice(20..-1).nil?
  raise unless s.byteslice(20..-20).nil?
  raise unless s.byteslice(2..-20) == ''
  raise unless s.byteslice(-1..20) == 'k'
  raise unless s.byteslice(-20..20).nil?
  raise unless s.byteslice(-5..-1) == 'ghijk'
  raise unless s.byteslice(-5..1) == ''
  raise unless s.byteslice(-5..8) == 'ghi'
end

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

def string_reverse
  raise unless '再见'.reverse == '见再'
end

def string_tr
  raise unless 'abcd'.tr('a-z', 'xxx') == 'xxxx'
end

spec if $PROGRAM_NAME == __FILE__
