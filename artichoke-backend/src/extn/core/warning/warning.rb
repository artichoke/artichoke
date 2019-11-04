# frozen_string_literal: true

module Warning
  # TODO: This method should be defined, but due to method visibility
  # limitations of the mruby VM, we cannot shadow the warn method in `Kernel`.
  # def warn(message)
  #   $stderr.print(message)
  # end
end
