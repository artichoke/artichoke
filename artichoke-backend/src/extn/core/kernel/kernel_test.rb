# frozen_string_literal: true

# Tests from Kernel core docs in Ruby 2.6.3
# https://ruby-doc.org/core-2.6.3/Kernel.html
def spec
  throw_catch
  kernel_integer_implicit_conversion
  kernel_integer_float
  kernel_integer_float_infinity
  kernel_integer_float_neg_infinity
  kernel_integer_float_nan
  kernel_integer_integer
  kernel_integer_nil
  kernel_integer_maybe_to_int
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

class A
  def to_int
    16
  end
end

class AA
  def to_int
    '16'
  end
end

class B
  def to_str
    '55'
  end
end

class C
  def to_int
    raise 'to int err'
  end
end

class D
  def to_str
    raise 'to str err'
  end
end

class AAA
  def to_int
    Object.new
  end
end

class S < String; end

class AAAA
  def to_int
    S.new
  end
end

class R
  def to_int
    'xys'
  end

  def to_i
    nil
  end
end

class W
  def to_int
    'nine'
  end
end

class X
  def to_int
    '9'
  end
end

class Y
  def to_i
    'nine'
  end
end

class Z
  def to_i
    '9'
  end
end

def kernel_integer_implicit_conversion
  begin
    Integer(10, A.new)
    raise 'expected ArgumentError'
  rescue ArgumentError => e
    raise unless e.message == 'base specified for non string value'
  end

  begin
    Integer(10.9, A.new)
    raise 'expected ArgumentError'
  rescue ArgumentError => e
    raise "got message: #{e.message}" unless e.message == 'base specified for non string value'
  end

  raise unless Integer('10', A.new) == 16
  raise unless Integer(10, AA.new) == 10
  raise unless Integer(10.9, AA.new) == 10
  raise unless Integer('10', AA.new) == 10
  raise unless Integer(10, AAA.new) == 10
  raise unless Integer(10.9, AAA.new) == 10
  raise unless Integer('10', AAA.new) == 10
  raise unless Integer(10, AAAA.new) == 10
  raise unless Integer(10.9, AAAA.new) == 10
  raise unless Integer('10', AAAA.new) == 10
  raise unless Integer(10, B.new) == 10
  raise unless Integer(10.9, B.new) == 10
  raise unless Integer('10', B.new) == 10

  begin
    Integer(10, C.new)
    raise 'expected RuntimeError'
  rescue RuntimeError => e
    raise "got message: #{e.message}" unless e.message == 'to int err'
  end

  begin
    Integer(10.9, C.new)
    raise 'expected RuntimeError'
  rescue RuntimeError => e
    raise "got message: #{e.message}" unless e.message == 'to int err'
  end

  begin
    Integer('10', C.new)
    raise 'expected RuntimeError'
  rescue RuntimeError => e
    raise "got message: #{e.message}" unless e.message == 'to int err'
  end

  raise unless Integer(10, D.new) == 10
  raise unless Integer(10.9, D.new) == 10
  raise unless Integer('10', D.new) == 10
  raise unless Integer(10, [1, 2, 3]) == 10
  raise unless Integer(10.9, [1, 2, 3]) == 10
  raise unless Integer('10', [1, 2, 3]) == 10

  raise unless Integer('555', '55') == 555
  raise unless Integer('555', '10') == 555
  raise unless Integer(A.new) == 16

  begin
    Integer(B.new)
    raise 'expected TypeError'
  rescue TypeError => e
    raise "got message: #{e.message}" unless e.message == "can't convert B into Integer"
  end

  begin
    Integer(C.new)
    raise 'expected TypeError'
  rescue TypeError => e
    raise "got message: #{e.message}" unless e.message == "can't convert C into Integer"
  end

  begin
    Integer(D.new)
    raise 'expected TypeError'
  rescue TypeError => e
    raise "got message: #{e.message}" unless e.message == "can't convert D into Integer"
  end

  begin
    Integer(A.new, 10)
    raise 'expected ArgumentError'
  rescue ArgumentError => e
    raise "got message: #{e.message}" unless e.message == 'base specified for non string value'
  end

  begin
    Integer(A.new, A.new)
    raise 'expected ArgumentError'
  rescue ArgumentError => e
    raise "got message: #{e.message}" unless e.message == 'base specified for non string value'
  end

  raise unless Integer(A.new, B.new) == 16

  begin
    Integer(A.new, C.new)
    raise 'expected RuntimeError'
  rescue RuntimeError => e
    raise "got message: #{e.message}" unless e.message == 'to int err'
  end

  raise unless Integer(A.new, D.new) == 16
  raise unless Integer(A.new, Object.new) == 16
  raise unless Integer(A.new, BasicObject.new) == 16
  raise unless Integer(A.new, [1, 2, 3]) == 16
  raise unless Integer(A.new, AA.new) == 16
  raise unless Integer(B.new, 10) == 55

  begin
    Integer(B.new, '10')
    raise 'expected TypeError'
  rescue TypeError => e
    raise "got message: #{e.message}" unless e.message == "can't convert B into Integer"
  end

  raise unless Integer(B.new, A.new) == 85

  begin
    Integer(B.new, AA.new)
    raise 'expected TypeError'
  rescue TypeError => e
    raise "got message: #{e.message}" unless e.message == "can't convert B into Integer"
  end

  begin
    Integer(B.new, B.new)
    raise 'expected TypeError'
  rescue TypeError => e
    raise "got message: #{e.message}" unless e.message == "can't convert B into Integer"
  end

  begin
    Integer(B.new, C.new)
    raise 'expected RuntimeError'
  rescue RuntimeError => e
    raise "got message: #{e.message}" unless e.message == 'to int err'
  end

  begin
    Integer(B.new, D.new)
    raise 'expected TypeError'
  rescue TypeError => e
    raise "got message: #{e.message}" unless e.message == "can't convert B into Integer"
  end

  begin
    Integer(C.new, 10)
    raise 'expected ArgumentError'
  rescue ArgumentError => e
    raise "got message: #{e.message}" unless e.message == 'base specified for non string value'
  end

  begin
    Integer(C.new, A.new)
    raise 'expected ArgumentError'
  rescue ArgumentError => e
    raise "got message: #{e.message}" unless e.message == 'base specified for non string value'
  end

  begin
    Integer(C.new, AA.new)
    raise 'expected ArgumentError'
  rescue TypeError => e
    raise "got message: #{e.message}" unless e.message == "can't convert C into Integer"
  end

  begin
    Integer(C.new, B.new)
    raise 'expected ArgumentError'
  rescue TypeError => e
    raise "got message: #{e.message}" unless e.message == "can't convert C into Integer"
  end

  begin
    Integer(C.new, C.new)
    raise 'expected RuntimeError'
  rescue RuntimeError => e
    raise "got message: #{e.message}" unless e.message == 'to int err'
  end

  begin
    Integer(C.new, D.new)
    raise 'expected ArgumentError'
  rescue TypeError => e
    raise "got message: #{e.message}" unless e.message == "can't convert C into Integer"
  end

  begin
    Integer(D.new, 10)
    raise 'expected RuntimeError'
  rescue RuntimeError => e
    raise "got message: #{e.message}" unless e.message == 'to str err'
  end

  begin
    Integer(D.new, A.new)
    raise 'expected RuntimeError'
  rescue RuntimeError => e
    raise "got message: #{e.message}" unless e.message == 'to str err'
  end

  begin
    Integer(D.new, AA.new)
    raise 'expected ArgumentError'
  rescue TypeError => e
    raise "got message: #{e.message}" unless e.message == "can't convert D into Integer"
  end

  begin
    Integer(D.new, B.new)
    raise 'expected ArgumentError'
  rescue TypeError => e
    raise "got message: #{e.message}" unless e.message == "can't convert D into Integer"
  end

  begin
    Integer(D.new, C.new)
    raise 'expected RuntimeError'
  rescue RuntimeError => e
    raise "got message: #{e.message}" unless e.message == 'to int err'
  end

  begin
    Integer(D.new, D.new)
    raise 'expected ArgumentError'
  rescue TypeError => e
    raise "got message: #{e.message}" unless e.message == "can't convert D into Integer"
  end
end

def kernel_integer_float
  raise unless Integer(10.2) == 10
  raise unless Integer(10.5) == 10
  raise unless Integer(10.9) == 10
  raise unless Integer(-10.2) == -10
  raise unless Integer(-10.5) == -10
  raise unless Integer(-10.9) == -10
  raise unless Integer(10.2, nil) == 10
  raise unless Integer(10.5, nil) == 10
  raise unless Integer(10.9, nil) == 10
  raise unless Integer(-10.2, nil) == -10
  raise unless Integer(-10.5, nil) == -10
  raise unless Integer(-10.9, nil) == -10

  begin
    Integer(10.2, 10)
    raise 'expected ArgumentError'
  rescue ArgumentError => e
    raise "got message: #{e.message}" unless e.message == 'base specified for non string value'
  end

  begin
    Integer(10.2, 10)
    raise 'expected ArgumentError'
  rescue ArgumentError => e
    raise "got message: #{e.message}" unless e.message == 'base specified for non string value'
  end

  begin
    Integer(10.2, 10)
    raise 'expected ArgumentError'
  rescue ArgumentError => e
    raise "got message: #{e.message}" unless e.message == 'base specified for non string value'
  end
end

def kernel_integer_float_infinity
  begin
    Integer(Float::INFINITY)
    raise 'expected FloatDomainError'
  rescue FloatDomainError => e
    raise "got message: #{e.message}" unless e.message == 'Infinity'
  end

  begin
    Integer(Float::INFINITY, 10)
    raise 'expected ArgumentError'
  rescue ArgumentError => e
    raise "got message: #{e.message}" unless e.message == 'base specified for non string value'
  end

  begin
    Integer(Float::INFINITY, '10')
    raise 'expected FloatDomainError'
  rescue FloatDomainError => e
    raise "got message: #{e.message}" unless e.message == 'Infinity'
  end
end

def kernel_integer_float_neg_infinity
  begin
    Integer(-Float::INFINITY)
    raise 'expected FloatDomainError'
  rescue FloatDomainError => e
    raise "got message: #{e.message}" unless e.message == '-Infinity'
  end

  begin
    Integer(-Float::INFINITY, 10)
    raise 'expected ArgumentError'
  rescue ArgumentError => e
    raise "got message: #{e.message}" unless e.message == 'base specified for non string value'
  end

  begin
    Integer(-Float::INFINITY, '10')
    raise 'expected FloatDomainError'
  rescue FloatDomainError => e
    raise "got message: #{e.message}" unless e.message == '-Infinity'
  end
end

def kernel_integer_float_nan
  begin
    Integer(Float::NAN)
    raise 'expected FloatDomainError'
  rescue FloatDomainError => e
    raise "got message: #{e.message}" unless e.message == 'NaN'
  end

  begin
    Integer(Float::NAN, 10)
    raise 'expected ArgumentError'
  rescue ArgumentError => e
    raise "got message: #{e.message}" unless e.message == 'base specified for non string value'
  end

  begin
    Integer(Float::NAN, '10')
    raise 'expected FloatDomainError'
  rescue FloatDomainError => e
    raise "got message: #{e.message}" unless e.message == 'NaN'
  end
end

def kernel_integer_integer
  raise unless Integer(16) == 16
  raise unless Integer(-16) == -16

  begin
    Integer(16, 10)
    raise 'expected ArgumentError'
  rescue ArgumentError => e
    raise "got message: #{e.message}" unless e.message == 'base specified for non string value'
  end

  raise unless Integer(16, '10') == 16
  raise unless Integer(-16, '10') == -16
end

def kernel_integer_nil
  begin
    Integer(nil)
    raise 'expected TypeError'
  rescue TypeError => e
    raise "got message: #{e.message}" unless e.message == "can't convert nil into Integer"
  end

  begin
    Integer(nil, nil)
    raise 'expected TypeError'
  rescue TypeError => e
    raise "got message: #{e.message}" unless e.message == "can't convert nil into Integer"
  end

  begin
    Integer(nil, '10')
    raise 'expected TypeError'
  rescue TypeError => e
    raise "got message: #{e.message}" unless e.message == "can't convert nil into Integer"
  end

  begin
    Integer(nil, [1, 2, 3])
    raise 'expected TypeError'
  rescue TypeError => e
    raise "got message: #{e.message}" unless e.message == "can't convert nil into Integer"
  end

  begin
    Integer(nil, 10)
    raise 'expected ArgumentError'
  rescue ArgumentError => e
    raise "got message: #{e.message}" unless e.message == 'base specified for non string value'
  end

  begin
    Integer(nil, A.new)
    raise 'expected ArgumentError'
  rescue ArgumentError => e
    raise "got message: #{e.message}" unless e.message == 'base specified for non string value'
  end

  begin
    Integer(nil, AA.new)
    raise 'expected TypeError'
  rescue TypeError => e
    raise "got message: #{e.message}" unless e.message == "can't convert nil into Integer"
  end

  begin
    Integer(nil, B.new)
    raise 'expected TypeError'
  rescue TypeError => e
    raise "got message: #{e.message}" unless e.message == "can't convert nil into Integer"
  end

  begin
    Integer(nil, C.new)
    raise 'expected RuntimeError'
  rescue RuntimeError => e
    raise "got message: #{e.message}" unless e.message == 'to int err'
  end

  begin
    Integer(nil, D.new)
    raise 'expected TypeError'
  rescue TypeError => e
    raise "got message: #{e.message}" unless e.message == "can't convert nil into Integer"
  end
end

def kernel_integer_maybe_to_int
  begin
    Integer(Y.new)
    raise 'expected TypeError'
  rescue TypeError => e
    raise "got message: #{e.message}" unless e.message == "can't convert Y to Integer (Y#to_i gives String)"
  end

  begin
    Integer(Z.new)
    raise 'expected TypeError'
  rescue TypeError => e
    raise "got message: #{e.message}" unless e.message == "can't convert Z to Integer (Z#to_i gives String)"
  end

  begin
    Integer(W.new)
    raise 'expected TypeError'
  rescue TypeError => e
    raise "got message: #{e.message}" unless e.message == "can't convert W into Integer"
  end

  begin
    Integer(X.new)
    raise 'expected TypeError'
  rescue TypeError => e
    raise "got message: #{e.message}" unless e.message == "can't convert X into Integer"
  end

  begin
    Integer(R.new)
    raise 'expected TypeError'
  rescue TypeError => e
    raise "got message: #{e.message}" unless e.message == "can't convert R to Integer (R#to_i gives NilClass)"
  end

  begin
    Integer('abc', R.new)
    raise 'expected ArgumentError'
  rescue ArgumentError => e
    raise "got message: #{e.message}" unless e.message == 'invalid value for Integer(): "abc"'
  end
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
