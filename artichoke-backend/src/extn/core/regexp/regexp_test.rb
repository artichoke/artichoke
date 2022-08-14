# frozen_string_literal: true

# Tests from Kernel core docs in Ruby 2.6.3
# https://ruby-doc.org/core-2.6.3/Kernel.html
def spec
  regexp_initialize_already_init_literal
  regexp_initialize_already_init_compiled

  true
end

def regexp_initialize_already_init_literal
  r = /abc/in
  begin
    r.send(:initialize, 'xyz')
    raise 'expected SecurityError'
  rescue SecurityError => e
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
end

spec if $PROGRAM_NAME == __FILE__
