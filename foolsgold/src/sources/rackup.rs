const RACKUP_SOURCE: &str = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/ruby/config.ru"));

pub fn rack_adapter() -> String {
    RACKUP_SOURCE.to_owned()
}
