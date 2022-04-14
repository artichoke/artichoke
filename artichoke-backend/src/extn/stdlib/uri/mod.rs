use std::ffi::CStr;

use crate::extn::prelude::*;

const IP_SOCKET_CSTR: &CStr = qed::const_cstr_from_str!("IPSocket\0");
const IP_ADDR_CSTR: &CStr = qed::const_cstr_from_str!("IPAddr\0");
const URI_CSTR: &CStr = qed::const_cstr_from_str!("URI\0");

static URI_RUBY_SOURCE: &[u8] = include_bytes!("vendor/uri.rb");
static URI_COMMON_RUBY_SOURCE: &[u8] = include_bytes!("vendor/uri/common.rb");
static URI_FILE_RUBY_SOURCE: &[u8] = include_bytes!("vendor/uri/file.rb");
static URI_FTP_RUBY_SOURCE: &[u8] = include_bytes!("vendor/uri/ftp.rb");
static URI_GENERIC_RUBY_SOURCE: &[u8] = include_bytes!("vendor/uri/generic.rb");
static URI_HTTP_RUBY_SOURCE: &[u8] = include_bytes!("vendor/uri/http.rb");
static URI_HTTPS_RUBY_SOURCE: &[u8] = include_bytes!("vendor/uri/https.rb");
static URI_LDAP_RUBY_SOURCE: &[u8] = include_bytes!("vendor/uri/ldap.rb");
static URI_LDAPS_RUBY_SOURCE: &[u8] = include_bytes!("vendor/uri/ldaps.rb");
static URI_MAILTO_RUBY_SOURCE: &[u8] = include_bytes!("vendor/uri/mailto.rb");
static URI_RFC2396_PARSER_RUBY_SOURCE: &[u8] = include_bytes!("vendor/uri/rfc2396_parser.rb");
static URI_RFC3986_PARSER_RUBY_SOURCE: &[u8] = include_bytes!("vendor/uri/rfc3986_parser.rb");

pub fn init(interp: &mut Artichoke) -> InitializeResult<()> {
    let spec = class::Spec::new("IPSocket", IP_SOCKET_CSTR, None, None)?;
    interp.def_class::<IpSocket>(spec)?;

    let spec = class::Spec::new("IPAddr", IP_ADDR_CSTR, None, None)?;
    interp.def_class::<IpAddr>(spec)?;

    let spec = module::Spec::new(interp, "URI", URI_CSTR, None)?;
    interp.def_module::<Uri>(spec)?;

    interp.def_rb_source_file("uri.rb", URI_RUBY_SOURCE)?;
    interp.def_rb_source_file("uri/common.rb", URI_COMMON_RUBY_SOURCE)?;
    interp.def_rb_source_file("uri/file.rb", URI_FILE_RUBY_SOURCE)?;
    interp.def_rb_source_file("uri/ftp.rb", URI_FTP_RUBY_SOURCE)?;
    interp.def_rb_source_file("uri/generic.rb", URI_GENERIC_RUBY_SOURCE)?;
    interp.def_rb_source_file("uri/http.rb", URI_HTTP_RUBY_SOURCE)?;
    interp.def_rb_source_file("uri/https.rb", URI_HTTPS_RUBY_SOURCE)?;
    interp.def_rb_source_file("uri/ldap.rb", URI_LDAP_RUBY_SOURCE)?;
    interp.def_rb_source_file("uri/ldaps.rb", URI_LDAPS_RUBY_SOURCE)?;
    interp.def_rb_source_file("uri/mailto.rb", URI_MAILTO_RUBY_SOURCE)?;
    interp.def_rb_source_file("uri/rfc2396_parser.rb", URI_RFC2396_PARSER_RUBY_SOURCE)?;
    interp.def_rb_source_file("uri/rfc3986_parser.rb", URI_RFC3986_PARSER_RUBY_SOURCE)?;

    Ok(())
}

#[derive(Debug, Clone, Copy)]
pub struct IpSocket;

#[derive(Debug, Clone, Copy)]
pub struct IpAddr;

#[derive(Debug, Clone, Copy)]
pub struct Uri;

#[cfg(test)]
mod tests {
    use crate::test::prelude::*;

    const SUBJECT: &str = "URI";
    const FUNCTIONAL_TEST: &[u8] = include_bytes!("uri_test.rb");

    #[test]
    fn functional() {
        let mut interp = interpreter();
        let result = interp.eval(FUNCTIONAL_TEST);
        unwrap_or_panic_with_backtrace(&mut interp, SUBJECT, result);
        let result = interp.eval(b"spec");
        unwrap_or_panic_with_backtrace(&mut interp, SUBJECT, result);
    }
}
