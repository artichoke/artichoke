use std::ffi::CStr;

use crate::extn::prelude::*;

const JSON_CSTR: &CStr = cstr::cstr!("JSON");
static JSON_RUBY_SOURCE: &[u8] = include_bytes!("vendor/json.rb");
static JSON_COMMON_RUBY_SOURCE: &[u8] = include_bytes!("vendor/json/common.rb");
static JSON_GENERIC_OBJECT_RUBY_SOURCE: &[u8] = include_bytes!("vendor/json/generic_object.rb");
static JSON_VERSION_RUBY_SOURCE: &[u8] = include_bytes!("vendor/json/version.rb");
static JSON_PURE_RUBY_SOURCE: &[u8] = include_bytes!("vendor/json/pure.rb");
static JSON_PURE_GENERATOR_RUBY_SOURCE: &[u8] = include_bytes!("vendor/json/pure/generator.rb");
static JSON_PURE_PARSER_RUBY_SOURCE: &[u8] = include_bytes!("vendor/json/pure/parser.rb");

pub fn init(interp: &mut Artichoke) -> InitializeResult<()> {
    let spec = module::Spec::new(interp, "JSON", JSON_CSTR, None)?;
    interp.def_module::<Json>(spec)?;
    // NOTE(lopopolo): This setup of the JSON gem in the virtual file system does not include
    // any of the `json/add` sources for serializing "extra" types like `Time`
    // and `BigDecimal`, not all of which Artichoke supports.
    interp.def_rb_source_file("json.rb", JSON_RUBY_SOURCE)?;
    interp.def_rb_source_file("json/common.rb", JSON_COMMON_RUBY_SOURCE)?;
    interp.def_rb_source_file("json/generic_object.rb", JSON_GENERIC_OBJECT_RUBY_SOURCE)?;
    interp.def_rb_source_file("json/version.rb", JSON_VERSION_RUBY_SOURCE)?;
    interp.def_rb_source_file("json/pure.rb", JSON_PURE_RUBY_SOURCE)?;
    interp.def_rb_source_file("json/pure/generator.rb", JSON_PURE_GENERATOR_RUBY_SOURCE)?;
    interp.def_rb_source_file("json/pure/parser.rb", JSON_PURE_PARSER_RUBY_SOURCE)?;

    Ok(())
}

#[derive(Debug, Clone, Copy)]
pub struct Json;

#[cfg(test)]
mod tests {
    use crate::test::prelude::*;

    const SUBJECT: &str = "JSON";
    const FUNCTIONAL_TEST: &[u8] = include_bytes!("json_test.rb");

    #[test]
    fn functional() {
        let mut interp = interpreter().unwrap();
        let result = interp.eval(FUNCTIONAL_TEST);
        unwrap_or_panic_with_backtrace(&mut interp, SUBJECT, result);
        let result = interp.eval(b"spec");
        unwrap_or_panic_with_backtrace(&mut interp, SUBJECT, result);
    }
}
