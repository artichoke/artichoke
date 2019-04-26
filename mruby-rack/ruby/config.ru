# rubocop:disable Metrics/BlockLength
# mruby cannot resolve the parser ambiguity without the parens around the
# lambda expression.
run (lambda do |env|
  begin
    request_id = env[FoolsGold::REQ_STATS].req_start
    seen = env[FoolsGold::REQ_STATS].seen_count
    body = <<-HTML
      <!doctype html>
      <html>
        <head>
          <title>
            FoolsGold Ruby Rack in Rust Server Extravaganza
          </title>
          <link rel="stylesheet" href="https://stackpath.bootstrapcdn.com/bootstrap/4.3.1/css/bootstrap.min.css"/>
        </head>
        <body>
          <div class="container my-2">
            <h1>FoolsGold Ruby Rack in Rust Server Extravaganza</h1>
            <p align="center">
              <a href="/fools-gold">
                <img class="mw-100" src="/img/pyrite.jpg" alt="FoolsGold"></img>
              </a>
            </p>
            <h2>Request ID</h2>
            <p>Request IDs are generated in Rust with the uuid crate.</p>
            <p>Trace: <code>#{request_id}</code></p>
            <h2>Request Count</h2>
            <p>
              Request count tracks the total number of seen requests across all
              threads and all mruby interpreters. Request count is tracked in a
              static <code>AtomicI64</code> in Rust.
            </p>
            <p>Counter: <code>#{seen}</code></p>
          </div>
        </body>
      </html>
    HTML
    [200, { 'Content-Type'.freeze => 'text/html'.freeze }, [body]]
  ensure
    env[FoolsGold::REQ_STATS].req_finalize
  end
end)
# rubocop:enable Metrics/BlockLength
