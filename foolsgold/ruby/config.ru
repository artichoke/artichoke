# frozen_string_literal: true

require 'foolsgold'

# Monkeypatch String to add String#strip_heredoc_indent
class String
  # mruby doesn't support Regexp natively, so use a fixed width
  # strip technique.
  def strip_indent(indent)
    each_line.map { |line| line[indent..-1] }.join
  end
end

use FoolsGold::Middleware::Request

class App
  def self.body(trace_id, req_counter)
    <<-HTML.strip_indent(6)
      <!DOCTYPE html>
      <html>
        <head>
          <title>
            FoolsGold Ruby Rack in Rust Server Extravaganza
          </title>
          <link
            rel="stylesheet"
            href="https://stackpath.bootstrapcdn.com/bootstrap/4.3.1/css/bootstrap.min.css"
          />
        </head>
        <body>
          <div class="container my-2">
            <h1>FoolsGold Ruby Rack in Rust Server Extravaganza</h1>
            <p align="center">
              <a href="/fools-gold">
                <img class="mw-100" src="/img/pyrite.jpg" alt="FoolsGold" />
              </a>
            </p>
            <h2>Request ID</h2>
            <p>Request IDs are generated in Rust with the uuid crate.</p>
            <p>Trace: <code>#{trace_id}</code></p>
            <h2>Request Count</h2>
            <p>
              Request count tracks the total number of seen requests across all
              threads and all mruby interpreters. Request count is tracked in a static
              <code>AtomicI64</code> in Rust.
            </p>
            <p>Counter: <code>#{req_counter}</code></p>
          </div>
        </body>
      </html>
    HTML
  end

  def self.call(env)
    context = env[FoolsGold::CONTEXT]
    trace_id = context.trace_id
    req_counter = context.metrics.total_requests.get
    [200, { 'Content-Type' => 'text/html' }, [body(trace_id, req_counter)]]
  ensure
    context.metrics.total_requests.inc
  end
end

run App
