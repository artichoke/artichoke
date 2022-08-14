# frozen_string_literal: true

# Tests from Kernel core docs in Ruby 2.6.3
# https://ruby-doc.org/core-2.6.3/Kernel.html
def spec
  regexp_initialize_already_init_literal
  regexp_initialize_already_init_compiled
  regexp_initialize_only_literals_frozen_by_default

  true
end

def regexp_initialize_already_init_literal
  r = /abc/in
  begin
    r.send(:initialize, 'xyz')
    raise 'expected FrozenError'
  rescue FrozenError => e
    raise "got message: #{e.message}" unless e.message == "can't modify literal regexp"
  end

  r = /abc/in
  begin
    r.send(:initialize, '/xyz/')
    raise 'expected FrozenError'
  rescue FrozenError => e
    raise "got message: #{e.message}" unless e.message == "can't modify literal regexp"
  end

  r = /abc/in
  begin
    r.send(:initialize, Regexp.compile('xyz'))
    raise 'expected FrozenError'
  rescue FrozenError => e
    raise "got message: #{e.message}" unless e.message == "can't modify literal regexp"
  end
end

def regexp_initialize_already_init_compiled
  r = Regexp.compile('abc', 'i', 'n')
  begin
    r.send(:initialize, 'xyz')
    raise 'expected TypeError'
  rescue TypeError => e
    raise "got message: #{e.message}" unless e.message == 'already initialized regexp'
  end

  r = Regexp.compile('abc', 'i', 'n')
  begin
    r.send(:initialize, '/xyz/')
    raise 'expected TypeError'
  rescue TypeError => e
    raise "got message: #{e.message}" unless e.message == 'already initialized regexp'
  end

  r = Regexp.compile('abc', 'i', 'n')
  begin
    r.send(:initialize, Regexp.compile('xyz'))
    raise 'expected TypeError'
  rescue TypeError => e
    raise "got message: #{e.message}" unless e.message == 'already initialized regexp'
  end
end

# Since Ruby 3.0, Regexp literals are frozen by default.
# https://github.com/ruby/ruby/pull/2705
# https://github.com/ruby/ruby/pull/3676
# Non-literal Regexp objects are still unfrozen by default.
def regexp_initialize_only_literals_frozen_by_default
  raise unless /abc/.frozen?

  s = 'abc'
  raise unless /#{s}/.frozen?

  raise if Regexp.compile('abc').frozen?
end

spec if $PROGRAM_NAME == __FILE__
