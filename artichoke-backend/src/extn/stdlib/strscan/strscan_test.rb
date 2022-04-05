# frozen_string_literal: true

require 'strscan'

def spec
  test_strscan
  test_shift
  test_shift_frozen
  test_index
  test_beginning_of_line?
  test_captures
  test_charpos
  test_check
  test_check_until
  test_eos
  test_exist
  test_get_byte
  test_getch
  test_inspect
  test_match?
  test_matched
  test_matched?
  test_matched_size
  test_peek
  test_pos_setter
  test_pos
  test_post_match
  test_pre_match
  test_rest
  test_rest_size
  test_scan
  test_scan_until
  test_size
  test_skip
  test_skip_until
  test_unscan
  test_values_at
  test_gh_1630_functional
  test_inspect_emoji_partial

  true
end

def test_strscan
  s = StringScanner.new('This is an example string')
  raise if s.eos?
  raise unless s.scan(/\w+/) == 'This'
  raise unless s.scan(/\w+/).nil?
  raise unless s.scan(/\s+/) == ' '
  raise unless s.scan(/\s+/).nil?
  raise unless s.scan(/\w+/) == 'is'
  raise if s.eos?
  raise unless s.scan(/\s+/) == ' '
  raise unless s.scan(/\w+/) == 'an'
  raise unless s.scan(/\s+/) == ' '
  raise unless s.scan(/\w+/) == 'example'
  raise unless s.scan(/\s+/) == ' '
  raise unless s.scan(/\w+/) == 'string'
  raise unless s.eos?
  raise unless s.scan(/\s+/).nil?
  raise unless s.scan(/\w+/).nil?
end

def test_shift
  s = StringScanner.new(+'Fri Dec 12 1975 14:39')
  s.scan(/Fri /)
  s << ' +1000 GMT'
  raise unless s.string == 'Fri Dec 12 1975 14:39 +1000 GMT'
  raise unless s.scan(/Dec/) == 'Dec'
end

def test_shift_frozen
  s = 'abc'
  s.freeze
  scn = StringScanner.new(s)
  scn << 'abc'
  raise 'should not be reached'
rescue FrozenError
  nil
end

def test_index
  s = StringScanner.new('Fri Dec 12 1975 14:39')
  raise unless s.scan(/(\w+) (\w+) (\d+) /) == 'Fri Dec 12 '
  raise unless s[0] == 'Fri Dec 12 '
  raise unless s[1] == 'Fri'
  raise unless s[2] == 'Dec'
  raise unless s[3] == '12'
  raise unless s.post_match == '1975 14:39'
  raise unless s.pre_match == ''

  s.reset
  raise unless s.scan(/(?<wday>\w+) (?<month>\w+) (?<day>\d+) /) == 'Fri Dec 12 '
  raise unless s[0] == 'Fri Dec 12 '
  raise unless s[1] == 'Fri'
  raise unless s[2] == 'Dec'
  raise unless s[3] == '12'
  raise unless s[:wday] == 'Fri'
  raise unless s[:month] == 'Dec'
  raise unless s[:day] == '12'
  raise unless s.post_match == '1975 14:39'
  raise unless s.pre_match == ''
end

def test_beginning_of_line?
  s = StringScanner.new("test\ntest\n")
  raise unless s.bol?

  s.scan(/te/)
  raise if s.bol?

  s.scan(/st\n/)
  raise unless s.bol?

  s.terminate
  raise unless s.bol?
end

def test_captures
  s = StringScanner.new('Fri Dec 12 1975 14:39')
  raise unless s.scan(/(\w+) (\w+) (\d+) /) == 'Fri Dec 12 '
  raise unless s.captures == %w[Fri Dec 12]
  raise unless s.scan(/(\w+) (\w+) (\d+) /).nil?
  raise unless s.captures.nil?
end

def test_charpos
  s = StringScanner.new('abcädeföghi')
  raise unless s.charpos.zero?
  raise unless s.scan_until(/ä/) == 'abcä'
  raise unless s.pos == 5
  raise unless s.charpos == 4
end

def test_check
  s = StringScanner.new('Fri Dec 12 1975 14:39')
  raise unless s.check(/Fri/) == 'Fri'
  raise unless s.pos.zero?
  raise unless s.matched == 'Fri'
  raise unless s.check(/12/).nil?
  raise unless s.matched.nil?
end

def test_check_until
  s = StringScanner.new('Fri Dec 12 1975 14:39')
  raise unless s.check_until(/12/) == 'Fri Dec 12'
  raise unless s.pos.zero?
  raise unless s.matched == '12'
end

def test_eos
  s = StringScanner.new('test string')
  raise if s.eos?

  s.scan(/test/)
  raise if s.eos?

  s.terminate
  raise unless s.eos?
end

def test_exist
  s = StringScanner.new('test string')
  raise unless s.exist?(/s/) == 3
  raise unless s.scan(/test/) == 'test'
  raise unless s.exist?(/s/) == 2
  raise unless s.exist?(/e/).nil?
end

def test_get_byte
  s = StringScanner.new('ab')
  raise unless s.get_byte == 'a'
  raise unless s.get_byte == 'b'
  raise unless s.get_byte.nil?

  # Encoding not supported by mruby
  #
  # $KCODE = 'EUC'
  # s = StringScanner.new("\244\242")
  # s.get_byte         # => "\244"
  # s.get_byte         # => "\242"
  # s.get_byte         # => nil
end

def test_getch
  s = StringScanner.new('ab')
  raise unless s.getch == 'a'
  raise unless s.getch == 'b'
  raise unless s.getch.nil?

  # Encoding not supported by mruby
  #
  # $KCODE = 'EUC'
  # s = StringScanner.new("\244\242")
  # s.getch           # => "\244\242"   # Japanese hira-kana "A" in EUC-JP
  # s.getch           # => nil
end

def test_inspect
  s = StringScanner.new('Fri Dec 12 1975 14:39')
  raise unless s.inspect == '#<StringScanner 0/21 @ "Fri D...">'
  raise unless s.scan_until(/12/) == 'Fri Dec 12'
  raise unless s.inspect == '#<StringScanner 10/21 "...ec 12" @ " 1975...">'
  raise unless s.terminate.inspect == '#<StringScanner fin>'
end

def test_match?
  s = StringScanner.new('test string')
  raise unless s.match?(/\w+/) == 4
  raise unless s.match?(/\w+/) == 4
  raise unless s.match?(/\s+/).nil?
end

def test_matched
  s = StringScanner.new('test string')
  raise unless s.match?(/\w+/) == 4
  raise unless s.matched == 'test'
end

def test_matched?
  s = StringScanner.new('test string')
  raise unless s.match?(/\w+/) == 4
  raise unless s.matched?
  raise unless s.match?(/\d+/).nil?
  raise if s.matched?
end

def test_matched_size
  s = StringScanner.new('test string')
  raise unless s.check(/\w+/) == 'test'
  raise unless s.matched_size == 4
  raise unless s.check(/\d+/).nil?
  raise unless s.matched_size.nil?
end

def test_peek
  s = StringScanner.new('test string')
  raise unless s.peek(7) == 'test st'
  raise unless s.peek(7) == 'test st'
end

def test_pos_setter
  s = StringScanner.new('test string')
  raise unless (s.pos = 7) == 7
  raise unless s.rest == 'ring'
end

def test_pos
  s = StringScanner.new('test string')
  raise unless s.pos.zero?
  raise unless s.scan_until(/str/) == 'test str'
  raise unless s.pos == 8

  s.terminate
  raise unless s.pos == 11
end

def test_post_match
  s = StringScanner.new('test string')
  raise unless s.scan(/\w+/) == 'test'
  raise unless s.scan(/\s+/) == ' '
  raise unless s.pre_match == 'test'
  raise unless s.post_match == 'string'
end

def test_pre_match
  s = StringScanner.new('test string')
  raise unless s.scan(/\w+/) == 'test'
  raise unless s.scan(/\s+/) == ' '
  raise unless s.pre_match == 'test'
  raise unless s.post_match == 'string'
end

def test_rest
  s = StringScanner.new('test string')
  raise unless s.rest == 'test string'
  raise unless s.scan(/\w+/) == 'test'
  raise unless s.scan(/\s+/) == ' '
  raise unless s.rest == 'string'
  raise unless s.scan(/\w+/) == 'string'
  raise unless s.rest == ''
end

def test_rest_size
  s = StringScanner.new('test string')
  raise unless s.rest_size == s.rest.size
  raise unless s.scan(/\w+/) == 'test'
  raise unless s.rest_size == s.rest.size
  raise unless s.scan(/\s+/) == ' '
  raise unless s.rest_size == s.rest.size
  raise unless s.scan(/\w+/) == 'string'
  raise unless s.rest == ''
end

def test_scan
  s = StringScanner.new('test string')
  raise unless s.scan(/\w+/) == 'test'
  raise unless s.scan(/\w+/).nil?
  raise unless s.scan(/\s+/) == ' '
  raise unless s.scan(/\w+/) == 'string'
  raise unless s.scan(/./).nil?
end

def test_scan_until
  s = StringScanner.new('Fri Dec 12 1975 14:39')
  raise unless s.scan_until(/1/) == 'Fri Dec 1'
  raise unless s.pre_match == 'Fri Dec '
  raise unless s.scan_until(/XYZ/).nil?
end

def test_size
  s = StringScanner.new('Fri Dec 12 1975 14:39')
  raise unless s.scan(/(\w+) (\w+) (\d+) /) == 'Fri Dec 12 '
  raise unless s.size == 4
end

def test_skip
  s = StringScanner.new('test string')
  raise unless s.skip(/\w+/) == 4
  raise unless s.skip(/\w+/).nil?
  raise unless s.skip(/\s+/) == 1
  raise unless s.skip(/\w+/) == 6
  raise unless s.skip(/./).nil?
end

def test_skip_until
  s = StringScanner.new('Fri Dec 12 1975 14:39')
  raise unless s.skip_until(/12/) == 10
end

def test_unscan
  s = StringScanner.new('test string')
  raise unless s.scan(/\w+/) == 'test'

  s.unscan
  raise unless s.scan(/../) == 'te'
  raise unless s.scan(/\d/).nil?

  s.unscan
  raise 'Expected ScanError'
rescue ScanError
  nil
end

def test_values_at
  s = StringScanner.new('Fri Dec 12 1975 14:39')
  raise unless s.scan(/(\w+) (\w+) (\d+) /) == 'Fri Dec 12 '
  raise unless s.values_at(0, -1, 5, 2) == ['Fri Dec 12 ', '12', nil, 'Dec']
  raise unless s.scan(/(\w+) (\w+) (\d+) /).nil?
  raise unless s.values_at(0, -1, 5, 2).nil?
end

# https://github.com/artichoke/artichoke/pull/1630#issuecomment-1004481741
def test_gh_1630_functional
  s = StringScanner.new('💎')
  raise unless s.inspect == '#<StringScanner 0/4 @ "\xF0\x9F\x92\x8E">'
  raise unless s.get_byte == "\xF0"
  raise unless s.rest == "\x9F\x92\x8E"
  raise unless s.inspect == '#<StringScanner 1/4 "\xF0" @ "\x9F\x92\x8E">'
end

def test_inspect_emoji_partial
  s = StringScanner.new('abc💎xyz')
  raise unless s.inspect == '#<StringScanner 0/10 @ "abc\xF0\x9F...">'

  s.get_byte
  raise unless s.inspect == '#<StringScanner 1/10 "a" @ "bc\xF0\x9F\x92...">'

  s.get_byte
  raise unless s.inspect == '#<StringScanner 2/10 "ab" @ "c\xF0\x9F\x92\x8E...">'

  s.get_byte
  raise unless s.inspect == '#<StringScanner 3/10 "abc" @ "\xF0\x9F\x92\x8Ex...">'

  s.get_byte
  raise unless s.inspect == '#<StringScanner 4/10 "abc\xF0" @ "\x9F\x92\x8Exy...">'

  s.get_byte
  raise unless s.inspect == '#<StringScanner 5/10 "abc\xF0\x9F" @ "\x92\x8Exyz">'

  s.get_byte
  raise unless s.inspect == '#<StringScanner 6/10 "...bc\xF0\x9F\x92" @ "\x8Exyz">'

  s.get_byte
  raise unless s.inspect == '#<StringScanner 7/10 "...c\xF0\x9F\x92\x8E" @ "xyz">'

  s.get_byte
  raise unless s.inspect == '#<StringScanner 8/10 "...\xF0\x9F\x92\x8Ex" @ "yz">'

  s.get_byte
  raise unless s.inspect == '#<StringScanner 9/10 "...\x9F\x92\x8Exy" @ "z">'

  s.get_byte
  raise unless s.inspect == '#<StringScanner fin>'

  s.get_byte
  raise unless s.inspect == '#<StringScanner fin>'
end

spec if $PROGRAM_NAME == __FILE__
