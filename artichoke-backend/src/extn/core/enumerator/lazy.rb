# frozen_string_literal: true

module Enumerable
  # = Enumerable#lazy implementation
  #
  # Enumerable#lazy returns an instance of Enumerator::Lazy.
  # You can use it just like as normal Enumerable object,
  # except these methods act as 'lazy':
  #
  #   - map       collect
  #   - select    find_all
  #   - reject
  #   - grep
  #   - drop
  #   - drop_while
  #   - take_while
  #   - flat_map  collect_concat
  #   - zip
  def lazy
    Enumerator::Lazy.new(self)
  end
end

class Enumerator
  # == Acknowledgements
  #
  #   Based on https://github.com/yhara/enumerable-lazy
  #   Inspired by https://github.com/antimon2/enumerable_lz
  #   http://jp.rubyist.net/magazine/?0034-Enumerable_lz (ja)
  class Lazy < Enumerator
    def initialize(obj, &block)
      super() do |yielder|
        obj.each do |x|
          if block
            block.call(yielder, x)
          else
            yielder << x
          end
        end
      rescue StopIteration
        nil
      end
    end

    def to_enum(meth = :each, *args, &block)
      raise ArgumentError, "undefined method #{meth}" unless respond_to?(meth)

      lz = Lazy.new(self, &block)
      lz.obj = self
      lz.meth = meth
      lz.args = args
      lz
    end
    alias enum_for to_enum

    def map(&block)
      Lazy.new(self) do |yielder, val|
        yielder << block.call(val)
      end
    end
    alias collect map

    def select(&block)
      Lazy.new(self) do |yielder, val|
        yielder << val if block.call(val)
      end
    end
    alias find_all select

    def reject(&block)
      Lazy.new(self) do |yielder, val|
        yielder << val unless block.call(val)
      end
    end

    def grep(pattern)
      Lazy.new(self) do |yielder, val|
        yielder << val if pattern === val # rubocop:disable Style/CaseEquality
      end
    end

    def drop(num)
      dropped = 0
      Lazy.new(self) do |yielder, val|
        if dropped < num
          dropped += 1
        else
          yielder << val
        end
      end
    end

    def drop_while(&block)
      dropping = true
      Lazy.new(self) do |yielder, val|
        if dropping
          unless block.call(val)
            yielder << val
            dropping = false
          end
        else
          yielder << val
        end
      end
    end

    def take(num)
      return Lazy.new(self) { raise StopIteration } if num.zero?

      taken = 0
      Lazy.new(self) do |yielder, val|
        yielder << val
        taken += 1
        raise StopIteration if taken >= num
      end
    end

    def take_while(&block)
      Lazy.new(self) do |yielder, val|
        raise StopIteration unless block.call(val)

        yielder << val
      end
    end

    def flat_map(&block)
      Lazy.new(self) do |yielder, val|
        ary = block.call(val)
        # TODO: check ary is an Array
        ary.each  do |x|
          yielder << x
        end
      end
    end
    alias collect_concat flat_map

    def zip(*args, &block)
      enums = [self] + args
      Lazy.new(self) do |yielder, _val|
        ary = enums.map(&:next)
        yielder << if block
                     block.call(ary)
                   else
                     ary
                   end
      end
    end

    def uniq(&block)
      hash = {}
      Lazy.new(self) do |yielder, val|
        v = if block
              block.call(val)
            else
              val
            end
        unless hash.include?(v)
          yielder << val
          hash[v] = val
        end
      end
    end

    alias force to_a
  end
end
