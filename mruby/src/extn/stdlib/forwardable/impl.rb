# frozen_string_literal: true

# :stopdoc:
module Forwardable
  def self._valid_method?(method)
    catch do |tag|
      eval("BEGIN{throw tag}; ().#{method}", binding, __FILE__, __LINE__) # rubocop:disable Security/Eval
    end
  rescue SyntaxError
    false
  else
    true
  end

  def self._compile_method(src, file, line)
    eval(src, nil, file, line) # rubocop:disable Security/Eval
  end
end
