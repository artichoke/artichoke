use crate::extn::prelude::*;

pub fn init(interp: &mut Artichoke) -> InitializeResult<()> {
    let spec = class::Spec::new("IPSocket", None, None)?;
    interp.def_class::<IpSocket>(spec)?;

    let spec = class::Spec::new("IPAddr", None, None)?;
    interp.def_class::<IpAddr>(spec)?;

    let spec = module::Spec::new(interp, "URI", None)?;
    interp.def_module::<Uri>(spec)?;

    interp.def_rb_source_file("uri.rb", &include_bytes!("vendor/uri.rb")[..])?;
    interp.def_rb_source_file("uri/common.rb", &include_bytes!("vendor/uri/common.rb")[..])?;
    interp.def_rb_source_file("uri/file.rb", &include_bytes!("vendor/uri/file.rb")[..])?;
    interp.def_rb_source_file("uri/ftp.rb", &include_bytes!("vendor/uri/ftp.rb")[..])?;
    interp.def_rb_source_file(
        "uri/generic.rb",
        &include_bytes!("vendor/uri/generic.rb")[..],
    )?;
    interp.def_rb_source_file("uri/http.rb", &include_bytes!("vendor/uri/http.rb")[..])?;
    interp.def_rb_source_file("uri/https.rb", &include_bytes!("vendor/uri/https.rb")[..])?;
    interp.def_rb_source_file("uri/ldap.rb", &include_bytes!("vendor/uri/ldap.rb")[..])?;
    interp.def_rb_source_file("uri/ldaps.rb", &include_bytes!("vendor/uri/ldaps.rb")[..])?;
    interp.def_rb_source_file("uri/mailto.rb", &include_bytes!("vendor/uri/mailto.rb")[..])?;
    interp.def_rb_source_file(
        "uri/rfc2396_parser.rb",
        &include_bytes!("vendor/uri/rfc2396_parser.rb")[..],
    )?;
    interp.def_rb_source_file(
        "uri/rfc3986_parser.rb",
        &include_bytes!("vendor/uri/rfc3986_parser.rb")[..],
    )?;

    Ok(())
}

#[derive(Debug)]
pub struct IpSocket;

#[derive(Debug)]
pub struct IpAddr;

#[derive(Debug)]
pub struct Uri;

#[cfg(test)]
mod tests {
    use crate::test::prelude::*;
    use bstr::ByteSlice;

    const SUBJECT: &str = "URI";
    const FUNCTIONAL_TEST: &[u8] = include_bytes!("uri_test.rb");

    #[test]
    fn functional() {
        let mut interp = crate::interpreter().unwrap();
        let _ = interp.eval(FUNCTIONAL_TEST).unwrap();
        let result = interp.eval(b"spec");
        if let Err(exc) = result {
            let backtrace = exc.vm_backtrace(&mut interp);
            let backtrace = bstr::join("\n", backtrace.unwrap_or_default());
            panic!(
                "{} tests failed with message: {:?} and backtrace:\n{:?}",
                SUBJECT,
                exc.message().as_bstr(),
                backtrace.as_bstr()
            );
        }
    }
}
