require 'rack/builder'

# rubocop:disable Metrics/BlockLength
app = Rack::Builder.new do
  # mruby cannot resolve the parser ambiguity without the parens around the
  # lambda expression.
  run (lambda do |env|
    begin
      request_id = env[FoolsGold::ENV_KEY].req_start
      seen = env[FoolsGold::ENV_KEY].seen_count
      body = <<-HTML
        <!doctype html>
        <html>
          <head>
            <title>
              FoolsGold Rack in Rust Server Extravaganza
            </title>
          </head>
          <body>
            <h1>FoolsGold Rack in Rust Server Extravaganza</h1>
            <p align="center"><img src="https://geology.com/gold/fools-gold/pyrite.jpg" alt="Fool's Gold"></img></p>
            <h2>Request ID</h2>
            <p>Generated in Rust</p>
            <p>#{request_id}</p>
            <h2>Total Seen Requests Across All Threads and All mruby Interpreters</h2>
            <p>Tracked in a static AtomicI64 in Rust</p>
            <p>#{seen}</p>
          </body>
        </html>
      HTML
      [200, { 'Content-Type'.freeze => 'text/html'.freeze }, [body]]
    ensure
      env[FoolsGold::ENV_KEY].req_finalize
    end
  end)
end
# rubocop:enable Metrics/BlockLength

adapter = FoolsGold::Adapter::InMemory.new do |env|
  app.call(env)
end

adapter.run
