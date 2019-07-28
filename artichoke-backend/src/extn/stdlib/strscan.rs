use crate::load::MrbLoadSources;
use crate::ArtichokeError;
use crate::Mrb;

pub fn init(interp: &Mrb) -> Result<(), ArtichokeError> {
    interp
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
    use crate::eval::MrbEval;
    use crate::load::MrbLoadSources;

    #[test]
    fn strscan_spec() {
        let interp = crate::interpreter().expect("mrb init");
        interp
            .def_rb_source_file("/src/test/strscan_test.rb", include_str!("strscan_test.rb"))
            .unwrap();
        interp.eval("require '/src/test/strscan_test.rb'").unwrap();
        if let Err(err) = interp.eval("spec") {
            panic!("{}", err);
        }
    }
}
