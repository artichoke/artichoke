use crate::load::MrbLoadSources;
use crate::Mrb;
use crate::MrbError;

pub fn init(interp: &Mrb) -> Result<(), MrbError> {
    interp.def_rb_source_file("uri.rb", include_str!("uri.rb"))?;
    interp.def_rb_source_file("uri/common.rb", include_str!("uri/common.rb"))?;
    interp.def_rb_source_file("uri/file.rb", include_str!("uri/file.rb"))?;
    interp.def_rb_source_file("uri/ftp.rb", include_str!("uri/ftp.rb"))?;
    interp.def_rb_source_file("uri/generic.rb", include_str!("uri/generic.rb"))?;
    interp.def_rb_source_file("uri/http.rb", include_str!("uri/http.rb"))?;
    interp.def_rb_source_file("uri/https.rb", include_str!("uri/https.rb"))?;
    interp.def_rb_source_file("uri/ldap.rb", include_str!("uri/ldap.rb"))?;
    interp.def_rb_source_file("uri/ldaps.rb", include_str!("uri/ldaps.rb"))?;
    interp.def_rb_source_file("uri/mailto.rb", include_str!("uri/mailto.rb"))?;
    interp.def_rb_source_file(
        "uri/rfc2396_parser.rb",
        include_str!("uri/rfc2396_parser.rb"),
    )?;
    interp.def_rb_source_file(
        "uri/rfc3986_parser.rb",
        include_str!("uri/rfc3986_parser.rb"),
    )?;
    Ok(())
}
