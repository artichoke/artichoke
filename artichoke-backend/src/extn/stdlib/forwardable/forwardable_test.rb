# frozen_string_literal: true

require 'forwardable'

class RecordCollection
  attr_accessor :records
  extend Forwardable
  def_delegator :@records, :[], :record_number
end

# specs taken from stdlib documentation
# https://ruby-doc.org/stdlib-2.6.3/libdoc/forwardable/rdoc/Forwardable.html
def spec
  lookup
  reopen
  # reqires STDOUT impl
  # object_extend
  another_example

  true
end

def lookup
  r = RecordCollection.new
  r.records = [4,5,6]
  raise unless r.record_number(0) == 4
end

class RecordCollection # re-open RecordCollection class
  def_delegators :@records, :size, :<<, :map
end

def reopen
  r = RecordCollection.new
  r.records = [1,2,3]
  raise unless r.record_number(0) == 1
  raise unless r.size == 3
  raise unless (r << 4) == [1, 2, 3, 4]
  raise unless r.map { |x| x * 2 }  == [2, 4, 6, 8]
end

def object_extend
  my_hash = Hash.new
  my_hash.extend Forwardable              # prepare object for delegation
  my_hash.def_delegator "STDOUT", "puts"  # add delegation for STDOUT.puts()
  my_hash.puts "Howdy!"
end

class Queue
  extend Forwardable

  def initialize
    @q = [ ]    # prepare delegate object
  end

  # setup preferred interface, enq() and deq()...
  def_delegator :@q, :push, :enq
  def_delegator :@q, :shift, :deq

  # support some general Array methods that fit Queues well
  def_delegators :@q, :clear, :first, :push, :shift, :size
end

def another_example
  q = Queue.new
  q.enq 1, 2, 3, 4, 5
  q.push 6

  raise unless q.shift == 1
  while q.size > 0
    result = q.deq
    expected = 6 - q.size
    raise unless result == expected
  end

  q.enq "Ruby", "Perl", "Python"
  raise unless q.first == "Ruby"
  q.clear
  raise unless q.first.nil?
end

spec if $PROGRAM_NAME == __FILE__
