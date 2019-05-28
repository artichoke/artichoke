use mruby::eval::MrbEval;
use mruby::interpreter::Interpreter;
use mruby::MrbError;
use mruby_gems::rubygems;

const APP: &str = r#"
require 'sinatra/base'

class MyApp < Sinatra::Base
  set :sessions, true
  set :foo, 'bar'

  get '/' do
    'Hello world!'
  end
end
"#;

fn main() -> Result<(), MrbError> {
    let interp = Interpreter::create()?;
    rubygems::mustermann::init(&interp)?;
    rubygems::rack::init(&interp)?;
    rubygems::rack_protection::init(&interp)?;
    rubygems::sinatra::init(&interp)?;
    rubygems::tilt::init(&interp)?;

    println!("{}", interp.eval(APP)?.to_s_debug());
    Ok(())
}
