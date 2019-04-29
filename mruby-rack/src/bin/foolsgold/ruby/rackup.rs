const RACKUP_SOURCE: &str = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/ruby/config.ru"));

pub fn rack_adapter() -> String {
    format!(
        r#"
require 'rack/builder'
require 'foolsgold'

FoolsGold::Adapter::Memory.new(Rack::Builder.new {{ {} }}).call({{}})
    "#,
        RACKUP_SOURCE
    )
}
