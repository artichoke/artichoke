# frozen_string_literal: true

# Tests from String core docs in Ruby 2.6.3
# https://ruby-doc.org/core-2.6.3/String.html
def spec
  string_match_operator
  string_element_reference_regexp
  string_byteslice
  string_byteindex
  string_scan
  string_unary_minus
  string_reverse
  string_tr
  string_end_with
  string_to_i
  string_eq

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
  s = 'abcdefghijk' # bytesize == 11
  # scalar
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
  raise unless s.byteslice(0..0) == 'a'
  raise unless s.byteslice(0..1) == 'ab'
  raise unless s.byteslice(0..10) == 'abcdefghijk'
  raise unless s.byteslice(1..9) == 'bcdefghij'
  raise unless s.byteslice(9..10) == 'jk'
  raise unless s.byteslice(9..11) == 'jk'
  raise unless s.byteslice(10..10) == 'k'
  raise unless s.byteslice(10..11) == 'k'
  raise unless s.byteslice(11..11) == ''
  raise unless s.byteslice(11..12) == ''
  raise unless s.byteslice(1..0) == ''
  raise unless s.byteslice(10..0) == ''
  raise unless s.byteslice(9..1) == ''
  raise unless s.byteslice(10..9) == ''
  raise unless s.byteslice(11..9) == ''
  raise unless s.byteslice(11..10) == ''
  raise unless s.byteslice(12..11).nil?
  raise unless s.byteslice(-12..0).nil?
  raise unless s.byteslice(-12..1).nil?
  raise unless s.byteslice(-11..0) == 'a'
  raise unless s.byteslice(-11..1) == 'ab'
  raise unless s.byteslice(-11..10) == 'abcdefghijk'
  raise unless s.byteslice(-11..11) == 'abcdefghijk'
  raise unless s.byteslice(-10..9) == 'bcdefghij'
  raise unless s.byteslice(-2..10) == 'jk'
  raise unless s.byteslice(-1..10) == 'k'
  raise unless s.byteslice(0..-11) == 'a'
  raise unless s.byteslice(0..-10) == 'ab'
  raise unless s.byteslice(0..-1) == 'abcdefghijk'
  raise unless s.byteslice(1..-2) == 'bcdefghij'
  raise unless s.byteslice(9..-1) == 'jk'
  raise unless s.byteslice(10..-1) == 'k'
  raise unless s.byteslice(0..-12) == ''
  raise unless s.byteslice(1..-12) == ''
  raise unless s.byteslice(1..-11) == ''
  raise unless s.byteslice(10..-2) == ''
  raise unless s.byteslice(11..-2) == ''
  raise unless s.byteslice(11..-1) == ''
  raise unless s.byteslice(-13..-12).nil?
  raise unless s.byteslice(-12..-12).nil?
  raise unless s.byteslice(-12..-11).nil?
  raise unless s.byteslice(-12..-10).nil?
  raise unless s.byteslice(-11..-11) == 'a'
  raise unless s.byteslice(-11..-10) == 'ab'
  raise unless s.byteslice(-11..-1) == 'abcdefghijk'
  raise unless s.byteslice(-2..-1) == 'jk'
  raise unless s.byteslice(-1..-1) == 'k'
  raise unless s.byteslice(-12..-13).nil?
  raise unless s.byteslice(-11..-12) == ''
  raise unless s.byteslice(-10..-12) == ''
  raise unless s.byteslice(-10..-11) == ''
  raise unless s.byteslice(-1..-11) == ''
  raise unless s.byteslice(-1..-2) == ''

  # non-ascii range test
  s = '太贵了!!' # bytesize == 11
  raise unless s.byteslice(0..0) == "\xE5"
  raise unless s.byteslice(0..1) == "\xE5\xA4"
  raise unless s.byteslice(0..10) == '太贵了!!'
  raise unless s.byteslice(1..9) == "\xA4\xAA贵了!"
  raise unless s.byteslice(9..10) == '!!'
  raise unless s.byteslice(9..11) == '!!'
  raise unless s.byteslice(10..10) == '!'
  raise unless s.byteslice(10..11) == '!'
  raise unless s.byteslice(11..11) == ''
  raise unless s.byteslice(11..12) == ''
  raise unless s.byteslice(1..0) == ''
  raise unless s.byteslice(10..0) == ''
  raise unless s.byteslice(9..1) == ''
  raise unless s.byteslice(10..9) == ''
  raise unless s.byteslice(11..9) == ''
  raise unless s.byteslice(11..10) == ''
  raise unless s.byteslice(12..11).nil?
  raise unless s.byteslice(-12..0).nil?
  raise unless s.byteslice(-12..1).nil?
  raise unless s.byteslice(-11..0) == "\xE5"
  raise unless s.byteslice(-11..1) == "\xE5\xA4"
  raise unless s.byteslice(-11..10) == '太贵了!!'
  raise unless s.byteslice(-11..11) == '太贵了!!'
  raise unless s.byteslice(-10..9) == "\xA4\xAA贵了!"
  raise unless s.byteslice(-2..10) == '!!'
  raise unless s.byteslice(-1..10) == '!'
  raise unless s.byteslice(0..-11) == "\xE5"
  raise unless s.byteslice(0..-10) == "\xE5\xA4"
  raise unless s.byteslice(0..-1) == '太贵了!!'
  raise unless s.byteslice(1..-2) == "\xA4\xAA贵了!"
  raise unless s.byteslice(9..-1) == '!!'
  raise unless s.byteslice(10..-1) == '!'
  raise unless s.byteslice(0..-12) == ''
  raise unless s.byteslice(1..-12) == ''
  raise unless s.byteslice(1..-11) == ''
  raise unless s.byteslice(10..-2) == ''
  raise unless s.byteslice(11..-2) == ''
  raise unless s.byteslice(11..-1) == ''
  raise unless s.byteslice(-13..-12).nil?
  raise unless s.byteslice(-12..-12).nil?
  raise unless s.byteslice(-12..-11).nil?
  raise unless s.byteslice(-12..-10).nil?
  raise unless s.byteslice(-11..-11) == "\xE5"
  raise unless s.byteslice(-11..-10) == "\xE5\xA4"
  raise unless s.byteslice(-11..-1) == '太贵了!!'
  raise unless s.byteslice(-2..-1) == '!!'
  raise unless s.byteslice(-1..-1) == '!'
  raise unless s.byteslice(-12..-13).nil?
  raise unless s.byteslice(-11..-12) == ''
  raise unless s.byteslice(-10..-12) == ''
  raise unless s.byteslice(-10..-11) == ''
  raise unless s.byteslice(-1..-11) == ''
  raise unless s.byteslice(-1..-2) == ''
end

def string_byteindex
  s = 'foo'
  raise unless s.byteindex('f').zero?
  raise unless s.byteindex('o') == 1
  raise unless s.byteindex('oo') == 1
  raise unless s.byteindex('ooo').nil?

  raise unless s.byteindex(/z/).nil?
  raise unless s.byteindex(/f/).zero?
  raise unless s.byteindex(/o/) == 1
  raise unless s.byteindex(/oo/) == 1
  raise unless s.byteindex(/ooo/).nil?

  raise unless s.byteindex(/f/, 2).nil?
  raise unless s.byteindex(/o/, 1) == 1
  raise unless s.byteindex(/o/, 2) == 2

  raise unless 'abcdef'.byteindex(/(c).*(f)/, 2) == 2
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

def assert_raise_type_error
  yield
rescue TypeError
  nil
else
  raise
end

def string_end_with
  assert_raise_type_error { 'abc'.end_with?(/c/) }
  assert_raise_type_error { 'abc'.end_with?('e', 'xyz', /c/) }

  'abc'.end_with?('e', 'bc', /c/)
end

def string_to_i
  # All POSIX whitespace should get trimmed
  raise unless "\x0B\n\r\t\x0C 123".to_i == 123

  # Only single underscores are valid
  raise unless '1__23'.to_i == 1
end

def string_eq
  raise unless '太贵了!!' != '太贵了!!'.b # rubocop:disable Style/YodaCondition
  raise unless '太贵了!!'.b != '太贵了!!'

  a = "\x7f"
  b = a.b
  raise unless a == b
  raise unless b == a

  c = "\x80"
  d = c.b
  raise if c == d
  raise if d == c
end

spec if $PROGRAM_NAME == __FILE__
