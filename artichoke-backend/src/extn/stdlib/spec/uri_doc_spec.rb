# frozen_string_literal: true

require 'uri'

module URI
  class RSYNC < Generic
    DEFAULT_PORT = 873
  end
  @@schemes['RSYNC'] = RSYNC
end

describe URI do
  it 'completes a basic example' do
    uri = URI('http://foo.com/posts?id=30&limit=5#time=1305298413')
    uri.inspect.should eql('#<URI::HTTP http://foo.com/posts?id=30&limit=5#time=1305298413>')
    uri.scheme.should eql('http')
    uri.host.should eql('foo.com')
    uri.path.should eql('/posts')
    uri.query.should eql('id=30&limit=5')
    uri.fragment.should eql('time=1305298413')
    uri.to_s.should eql('http://foo.com/posts?id=30&limit=5#time=1305298413')
  end

  it 'adds custom urls' do
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
    URI.scheme_list.should eql(expected_scheme_list)
    uri = URI('rsync://rsync.foo.com')
    uri.inspect.should eql('#<URI::RSYNC rsync://rsync.foo.com>')
  end

  it 'URI#decode_www_form' do
    ary = URI.decode_www_form('a=1&a=2&b=3')
    ary.should eql([%w[a 1], %w[a 2], %w[b 3]])
    ary.assoc('a').last.should eql('1')
    ary.assoc('b').last.should eql('3')
    # this line fails on YARV
    # ary.rassoc('a').last.should eql('2')
    Hash[ary].should eql('a' => '2', 'b' => '3')
  end

  it 'URI#encode_www_form' do
    URI.encode_www_form([%w[q ruby], %w[lang en]]).should eql('q=ruby&lang=en')
    URI.encode_www_form('q' => 'ruby', 'lang' => 'en').should eql('q=ruby&lang=en')
    URI.encode_www_form('q' => %w[ruby perl], 'lang' => 'en').should eql('q=ruby&q=perl&lang=en')
    URI.encode_www_form([%w[q ruby], %w[q perl], %w[lang en]]).should eql('q=ruby&q=perl&lang=en')
  end

  it 'URI#extract' do
    uris = URI.extract('text here http://foo.example.org/bla and here mailto:test@example.com and here also.')
    uris.should eql(['http://foo.example.org/bla', 'mailto:test@example.com'])
  end

  it 'URI#join' do
    URI.join('http://example.com/', 'main.rbx').inspect.should eql('#<URI::HTTP http://example.com/main.rbx>')
    URI.join('http://example.com/', 'foo').inspect.should eql('#<URI::HTTP http://example.com/foo>')
    URI.join('http://example.com/', '/foo', '/bar').inspect.should eql('#<URI::HTTP http://example.com/bar>')
    URI.join('http://example.com/', '/foo', 'bar').inspect.should eql('#<URI::HTTP http://example.com/bar>')
    URI.join('http://example.com/', '/foo/', 'bar').inspect.should eql('#<URI::HTTP http://example.com/foo/bar>')
  end

  it 'URI#parse' do
    uri = URI.parse('http://www.ruby-lang.org/')
    uri.inspect.should eql('#<URI::HTTP http://www.ruby-lang.org/>')
    uri.scheme.should eql('http')
    uri.host.should eql('www.ruby-lang.org')
  end

  # it 'URI#regexp' do
  #   # # extract first URI from html_string
  #   # html_string.slice(URI.regexp)
  #   #
  #   # # remove ftp URIs
  #   # html_string.sub(URI.regexp(['ftp']), '')
  #   #
  #   # # You should not rely on the number of parentheses
  #   # html_string.scan(URI.regexp) do |*matches|
  #   #   p $&
  #   # end
  # end

  it 'URI#split' do
    expected_split = ['http', nil, 'www.ruby-lang.org', nil, nil, '/', nil, nil, nil]
    URI.split('http://www.ruby-lang.org/').should eql(expected_split)
  end
end
