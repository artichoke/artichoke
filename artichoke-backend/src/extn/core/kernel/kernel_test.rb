# frozen_string_literal: true

# Tests from Kernel core docs in Ruby 2.6.3
# https://ruby-doc.org/core-2.6.3/Kernel.html
def spec
  throw_catch
  kernel_p_no_args
  kernel_p_one_arg
  kernel_p_array_args

  true
end

# https://ruby-doc.org/core-2.6.3/Kernel.html#method-i-catch
def throw_catch
  raise unless catch(1) { 123 } == 123

  raise unless catch(1) { throw(1, 456) } == 456
  raise unless catch(1) { throw(1) }.nil?

  raise unless catch(1) { |x| x + 2 } == 3

  result = catch do |_obj_a|
    catch do |obj_b|
      throw(obj_b, 123)
      puts 'This puts is not reached' # rubocop:disable Lint/UnreachableCode
    end

    puts 'This puts is displayed'
    456
  end
  raise unless result == 456

  result = catch do |obj_a|
    catch do |_obj_b|
      throw(obj_a, 123)
      puts 'This puts is still not reached' # rubocop:disable Lint/UnreachableCode
    end

    puts 'Now this puts is also not reached'
    456
  end
  raise unless result == 123
end

class Foo
  attr_accessor :bar, :baz
  def initialize(bar, baz)
    @bar = bar
    @baz = baz
  end
end

# https://ruby-doc.org/core-2.6.3/Kernel.html#method-i-p
def kernel_p_no_args
  result = p
  raise unless result.nil?
end

def kernel_p_one_arg
  f = Foo.new(1, 2)
  result = p(f)
  raise unless result.equal?(f)
end

def kernel_p_array_args
  f = Foo.new(1, 2)
  g = Foo.new(3, 4)
  result = p(f, g)
  raise unless result == [f, g]
  raise unless result.is_a?(Array)
  raise unless result.length == 2
end

spec if $PROGRAM_NAME == __FILE__
