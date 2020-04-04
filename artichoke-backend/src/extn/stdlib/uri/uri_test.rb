# frozen_string_literal: true

require 'uri'

module URI
  class RSYNC < Generic
    DEFAULT_PORT = 873
  end
  @@schemes['RSYNC'] = RSYNC
end

def spec
  basic_example
  add_custom_urls
  uri_decode_www_form
  uri_encode_www_form
  uri_extract
  uri_join
  uri_parse
  uri_regexp
  uri_split

  true
end

def basic_example
  uri = URI('http://foo.com/posts?id=30&limit=5#time=1305298413')
  raise unless uri.inspect == '#<URI::HTTP http://foo.com/posts?id=30&limit=5#time=1305298413>'
  raise unless uri.scheme == 'http'
  raise unless uri.host == 'foo.com'
  raise unless uri.path == '/posts'
  raise unless uri.query == 'id=30&limit=5'
  raise unless uri.fragment == 'time=1305298413'
  raise unless uri.to_s == 'http://foo.com/posts?id=30&limit=5#time=1305298413'
end

def add_custom_urls
  expected_scheme_list = {
    'FILE' => URI::File,
    'FTP' => URI::FTP,
    'HTTP' => URI::HTTP,
    'HTTPS' => URI::HTTPS,
    'LDAP' => URI::LDAP,
    'LDAPS' => URI::LDAPS,
    'MAILTO' => URI::MailTo,
    'RSYNC' => URI::RSYNC
  }
  raise unless URI.scheme_list == expected_scheme_list

  uri = URI('rsync://rsync.foo.com')
  raise unless uri.inspect == '#<URI::RSYNC rsync://rsync.foo.com>'
end

def uri_decode_www_form
  ary = URI.decode_www_form('a=1&a=2&b=3')
  raise unless ary == [%w[a 1], %w[a 2], %w[b 3]]
  raise unless ary.assoc('a').last == '1'
  raise unless ary.assoc('b').last == '3'
  # this line fails on YARV
  # raise unless ary.rassoc('a').last == '2'
  raise unless Hash[ary] == { 'a' => '2', 'b' => '3' }
end

def uri_encode_www_form
  raise unless URI.encode_www_form([%w[q ruby], %w[lang en]]) == 'q=ruby&lang=en'
  raise unless URI.encode_www_form('q' => 'ruby', 'lang' => 'en') == 'q=ruby&lang=en'
  raise unless URI.encode_www_form('q' => %w[ruby perl], 'lang' => 'en') == 'q=ruby&q=perl&lang=en'
  raise unless URI.encode_www_form([%w[q ruby], %w[q perl], %w[lang en]]) == 'q=ruby&q=perl&lang=en'
end

def uri_extract
  uris = URI.extract('text here http://foo.example.org/bla and here mailto:test@example.com and here also.')
  raise unless uris == ['http://foo.example.org/bla', 'mailto:test@example.com']
end

def uri_join
  raise unless URI.join('http://example.com/', 'main.rbx').inspect == '#<URI::HTTP http://example.com/main.rbx>'
  raise unless URI.join('http://example.com/', 'foo').inspect == '#<URI::HTTP http://example.com/foo>'
  raise unless URI.join('http://example.com/', '/foo', '/bar').inspect == '#<URI::HTTP http://example.com/bar>'
  raise unless URI.join('http://example.com/', '/foo', 'bar').inspect == '#<URI::HTTP http://example.com/bar>'
  raise unless URI.join('http://example.com/', '/foo/', 'bar').inspect == '#<URI::HTTP http://example.com/foo/bar>'
end

def uri_parse
  uri = URI.parse('http://www.ruby-lang.org/')
  raise unless uri.inspect == '#<URI::HTTP http://www.ruby-lang.org/>'
  raise unless uri.scheme == 'http'
  raise unless uri.host == 'www.ruby-lang.org'
end

def uri_regexp(skipped = true)
  return if skipped

  # extract first URI from html_string
  html_string.slice(URI::DEFAULT_PARSER.make_regexp)

  # remove ftp URIs
  html_string.sub(URI.regexp(['ftp']), '')

  # You should not rely on the number of parentheses
  html_string.scan(URI::DEFAULT_PARSER.make_regexp) do |*_matches|
    p $&
  end
end

def uri_split
  expected_split = ['http', nil, 'www.ruby-lang.org', nil, nil, '/', nil, nil, nil]
  raise unless URI.split('http://www.ruby-lang.org/') == expected_split
end

spec if $PROGRAM_NAME == __FILE__
