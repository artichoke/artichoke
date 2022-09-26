# frozen_string_literal: true

def spec
  time_strftime_utf8
  time_strftime_binary
end

def time_strftime_utf8
  t = Time.at(1000, 0, in: 'Z')

  raise unless t.strftime('%c') == 'Thu Jan  1 00:16:40 1970'
  raise unless "%c \xEF".valid_encoding? == false
  raise unless t.strftime("%c \xEF") == "Thu Jan  1 00:16:40 1970 \xEF"
  raise unless t.strftime("%c \xEF").valid_encoding? == false
  raise unless t.strftime("%c \u{1F600}") == 'Thu Jan  1 00:16:40 1970 😀'
  raise unless t.strftime("%c \u{1F600}").length == 26
  raise unless t.strftime('%c 😀') == 'Thu Jan  1 00:16:40 1970 😀'
  raise unless t.strftime('%c 😀').length == 26
end

def time_strftime_binary
  t = Time.at(1000, 0, in: 'Z')

  raise unless t.strftime('%c'.b) == 'Thu Jan  1 00:16:40 1970'
  raise unless "%c \xEF".b.valid_encoding?
  raise unless t.strftime("%c \xEF".b) == "Thu Jan  1 00:16:40 1970 \xEF".b
  raise unless t.strftime("%c \xEF".b).valid_encoding?
  raise unless t.strftime("%c \u{1F600}".b) == "Thu Jan  1 00:16:40 1970 \xF0\x9F\x98\x80".b
  raise unless t.strftime("%c \u{1F600}".b).length == 29
  raise unless t.strftime('%c 😀'.b) == "Thu Jan  1 00:16:40 1970 \xF0\x9F\x98\x80".b
  raise unless t.strftime('%c 😀'.b).length == 29
end
