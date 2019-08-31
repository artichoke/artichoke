use crate::load::LoadSources;
use crate::Artichoke;
use crate::ArtichokeError;

pub fn init(interp: &Artichoke) -> Result<(), ArtichokeError> {
    interp
        .0
        .borrow_mut()
        .def_module::<StringScanner>("StringScanner", None);
    interp.def_rb_source_file("strscan.rb", include_str!("strscan.rb"))?;
    Ok(())
}

pub struct StringScanner;

// StringScanner tests from Ruby stdlib docs
// https://ruby-doc.org/stdlib-2.6.3/libdoc/strscan/rdoc/StringScanner.html
#[cfg(test)]
mod tests {
    use crate::eval::Eval;
    use crate::load::LoadSources;

    #[test]
    fn strscan_spec() {
        let interp = crate::interpreter().expect("init");
        interp
            .def_rb_source_file("/src/test/strscan_test.rb", include_str!("strscan_test.rb"))
            .unwrap();
        interp.eval("require '/src/test/strscan_test.rb'").unwrap();
        if let Err(err) = interp.eval("spec") {
            panic!("{}", err);
        }
    }
}
