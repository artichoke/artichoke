# frozen_string_literal: true

require 'stringio'

module Artichoke
  module Generate
    module ThirdParty
      module AllTargets
        def self.third_party_flatfile
          cmd = CargoAbout.new(
            config: File.join(__dir__, 'all_targets', 'about.toml')
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

        def self.third_party_html
          cmd = CargoAbout.new(
            config: File.join(__dir__, 'all_targets', 'about.toml')
          )

          deps = cmd.invoke

          s = StringIO.new
          s.write <<~HEADER
            <!DOCTYPE html>
            <html lang="en">
              <head>
                <meta charset="utf-8" />
                <meta
                  name="viewport"
                  content="width=device-width, initial-scale=1, shrink-to-fit=no"
                />
                <meta
                  name="description"
                  content="Artichoke Ruby third party acknowledgements and copyright notices."
                />
                <meta name="author" content="Ryan Lopopolo" />

                <title>Artichoke Ruby Third Party Licenses</title>

                <link rel="canonical" href="https://www.artichokeruby.org/thirdparty/" />

                <!-- Favicons -->
                <%= require("./partials/favicons.html").default %>

                <!-- Twitter -->
                <meta name="twitter:card" content="summary_large_image" />
                <meta name="twitter:site" content="@artichokeruby" />
                <meta name="twitter:creator" content="@artichokeruby" />
                <meta name="twitter:title" content="Artichoke Ruby Third Party Licenses" />
                <meta
                  name="twitter:description"
                  content="Artichoke Ruby third party acknowledgements and copyright notices."
                />
                <meta
                  name="twitter:image"
                  content="https://www.artichokeruby.org/artichoke-social-logo.png"
                />

                <!-- Facebook Open Graph metadata -->
                <meta
                  property="og:url"
                  content="https://www.artichokeruby.org/thirdparty/"
                />
                <meta property="og:site_name" content="artichokeruby.org" />
                <meta property="og:title" content="Artichoke Ruby Third Party Licenses" />
                <meta
                  property="og:description"
                  content="Artichoke Ruby third party acknowledgements and copyright notices."
                />
                <meta property="og:type" content="website" />
                <meta
                  property="og:image"
                  content="https://www.artichokeruby.org/artichoke-social-logo.png"
                />
                <meta property="og:image:type" content="image/png" />
                <meta property="og:image:width" content="1600" />
                <meta property="og:image:height" content="800" />

                <%= require("./partials/google-analytics.html").default %>
              </head>
              <body>
                <%= require("./partials/google-analytics-noscript.html").default %>
                <a class="visually-hidden visually-hidden-focusable" href="#content">
                  Skip to main content
                </a>
                <%= require("./partials/nav/thirdparty.html").default %>

                <main id="content">
                  <div class="container mb-3 mb-md-5 mt-3 mt-md-5">
                    <div class="row">
                      <div class="thirdparty col-sm-12 col-md-8 offset-md-2">
                        <div class="intro">
                          <h1>Third Party Licenses</h1>
                          <p class="lead">
                            Artichoke is made possible by the Artichoke open source project
                            and other open source software.
                          </p>
                        </div>

                        <h2>Overview of Licenses</h2>
                        <ul class="licenses-overview">
          HEADER

          counts = Hash.new(0)
          deps.each do |dep|
            counts[dep.license] += 1
          end
          counts.each_pair.to_a.sort_by { |_k, v| -v }.each do |license, count|
            s.puts "              <li>#{license} (#{count})</li>"
          end

          s.puts '            </ul>'
          s.puts '            <h2>All License Text</h2>'
          deps.each do |dep|
            section = <<~LICENSE
              <section class="license" id="#{dep.name}-#{dep.version}-#{dep.license_id}">
                <h3>
                  <a href="#{dep.url}">#{dep.name} #{dep.version}</a>
                  <small class="text-muted"
                    >[<a href="##{dep.name}-#{dep.version}-#{dep.license_id}">&sect;</a>]</small
                  >
                </h3>
                <pre class="license-text">
                @@@
                </pre>
              </section>
            LICENSE
            section = section.lines.map do |line|
              next dep.license_full_text.gsub('<', '&lt;').gsub('>', '&gt;') if line == "  @@@\n"

              "            #{line}"
            end
            s.write section.join
          end

          s.puts '          </div>'
          s.puts '        </div>'
          s.puts '      </div>'
          s.puts '    </main>'
          s.puts
          s.puts '    <%= require("./partials/footer.html").default %>'
          s.puts '  </body>'
          s.puts '</html>'

          s.string
        end
      end
    end
  end
end
