# frozen_string_literal: true

# Tests from Encoding core docs in Ruby 3.1.2
# https://ruby-doc.org/3.1.2/Encoding.html
def spec
  encoding_name
  encoding_simple_ex

  true
end

def encoding_name
  raise unless Encoding::ISO_8859_1.name == 'ISO-8859-1'

  # TODO: Enable full `names` support
  # raise unless Encoding::ISO_8859_1.names == %w[ISO-8859-1 ISO8859-1]
end

def encoding_simple_ex
  encoding = 'some string'.encoding
  raise unless encoding == Encoding::UTF_8

  string = 'some string'.encode(Encoding::ISO_8859_1)
  raise unless string == 'some string'

  # TODO: Enable String#encoding
  # raise unless string.encoding == Encoding::ISO_8859_1

  string = 'some string'.encode 'ISO-8859-1'
  raise unless string == 'some string'

  # TODO: Enable String#encoding
  # raise unless string.encoding == Encoding::ISO_8859_1
end
