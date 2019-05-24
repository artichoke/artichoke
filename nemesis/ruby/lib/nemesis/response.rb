# frozen_string_literal: true

module Nemesis
  class Response
    attr_accessor :length, :status, :body
    attr_reader :header
    alias headers header

    def initialize(status = 200, header = {}, body = [])
      @status = status.to_i
      @header = header.each_with_object({}) do |(key, value), memo|
        memo[key.to_s] = value.to_s
      end

      @writer  = ->(x) { @body << x }
      @block   = nil
      @length  = 0

      @body = []

      if body.respond_to? :to_str
        write body.to_str
      elsif body.respond_to?(:each)
        body.each do |part|
          write part.to_s
        end
      else
        raise ArgumentError, 'stringable or iterable required'
      end
    end

    # Append to body and update Content-Length.
    #
    # NOTE: Do not mix #write and direct #body access!
    #
    def write(str)
      s = str.to_s
      # TODO: implement String#bytesize
      @length += s.size
      @writer.call s

      set_header(Rack::CONTENT_LENGTH, @length.to_s)
      str
    end

    def close
      body.close if body.respond_to?(:close)
    end

    def set_header(key, value)
      headers[key] = value
    end

    def body_bytes
      @body.join
    end
  end
end
