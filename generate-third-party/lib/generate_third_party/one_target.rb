# frozen_string_literal: true

require 'stringio'

module Artichoke
  module Generate
    module ThirdParty
      module OneTarget
        def self.third_party_flatfile(target)
          raise ArgumentError if target.nil?
          raise ArgumentError unless target.is_a?(String)

          cmd = CargoAbout.new(
            config: File.join(__dir__, 'one_target', target, 'about.toml')
          )

          deps = cmd.invoke

          s = StringIO.new
          needs_separator = false
          deps.each do |dep|
            if needs_separator
              s.puts
              s.puts '---'
              s.puts
            end

            s.puts "#{dep.name} #{dep.version}"
            s.puts ''
            s.puts dep.url
            s.puts
            s.puts dep.license_full_text

            needs_separator = true
          end

          s.string
        end
      end
    end
  end
end
