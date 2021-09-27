# frozen_string_literal: true

require 'stringio'
require 'yaml'

module Artichoke
  module Generate
    module ThirdParty
      module Deps
        # Parse the output of `cargo about` and return a list of `Dependency` objects
        # alphabetized by dependency name.
        def self.parse(cargo_about_output)
          # Psych won't parse a document delimiter in a quoted string, so munge it
          tx = cargo_about_output.gsub(
            '--- LLVM Exceptions to the Apache 2.0 License ----',
            'aaa LLVM Exceptions to the Apache 2.0 License zzzz'
          )

          # turn license text blocks into pipe-delimited literal multi-line strings
          is_text = false
          tx = tx.each_line.map do |line|
            if is_text && line == "@@@@text-end@@@@\n"
              is_text = false
              next nil
            end
            next "      #{line}" if is_text

            if line == "@@@@text-start@@@@\n"
              is_text = true
              # The `|2` indentation specifier is necessary because some licenses like
              # Apache-2.0 have initial lines that begin with whitespace.
              next "    text: |2\n"
            end
            line
          end

          yaml_output = tx.compact.join

          deps = YAML.safe_load(yaml_output)
          deps = deps['deps'].map do |hash|
            Dependency.from_hash(hash)
          end

          deps.sort_by!(&:name)
        end
      end

      class Dependency
        attr_reader :name, :version, :url, :license, :license_id

        def initialize(name, version, url, license, license_id, text)
          @name = name
          @version = version
          @url = url
          @license = license
          @license_id = license_id
          @text = text
        end

        def self.from_hash(hash)
          new(
            hash.fetch('name'),
            hash.fetch('version'),
            hash.fetch('url'),
            hash.fetch('license'),
            hash.fetch('id'),
            hash.fetch('text')
          )
        end

        def license_full_text
          @text.gsub(
            'aaa LLVM Exceptions to the Apache 2.0 License zzzz',
            '--- LLVM Exceptions to the Apache 2.0 License ----'
          )
        end

        def to_yaml
          s = StringIO.new
          s.puts <<~YAML
            - name: #{name}
              version: "#{version}"
              url: "#{url}"
              license: #{license}
              license_id: #{license_id}
          YAML

          # The `|2` indentation specifier is necessary because some licenses like
          # Apache-2.0 have initial lines that begin with whitespace.
          s.puts '  text: |2'

          license_full_text.each_line do |line|
            s.print "    #{line}"
          end
          s.string
        end
      end
    end
  end
end
