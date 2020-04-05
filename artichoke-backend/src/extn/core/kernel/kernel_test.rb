# frozen_string_literal: true

# Tests from Kernel core docs in Ruby 2.6.3
# https://ruby-doc.org/core-2.6.3/Kernel.html
def spec
  throw_catch

  true
end

# https://ruby-doc.org/core-2.6.3/Kernel.html#method-i-catch
def throw_catch
  raise unless catch(1) { 123 } == 123

  raise unless catch(1) { throw(1, 456) } == 456
  raise unless catch(1) { throw(1) }.nil?

  raise unless catch(1) { |x| x + 2 } == 3

  result = nil
  result = catch do |obj_A|
    catch do |obj_B|
      throw(obj_B, 123)
      puts "This puts is not reached"
    end

    puts "This puts is displayed"
    456
  end
  raise unless result == 456

  result = nil
  result = catch do |obj_A|
    catch do |obj_B|
      throw(obj_A, 123)
      puts "This puts is still not reached"
    end

    puts "Now this puts is also not reached"
    456
  end
  raise unless result == 123
end

spec if $PROGRAM_NAME == __FILE__
