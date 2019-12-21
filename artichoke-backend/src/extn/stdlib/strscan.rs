use artichoke_core::load::LoadSources;

use crate::class;
use crate::{Artichoke, ArtichokeError};

pub fn init(interp: &Artichoke) -> Result<(), ArtichokeError> {
    let spec = class::Spec::new("StringScanner", None, None);
    interp.0.borrow_mut().def_class::<StringScanner>(spec);
    interp.def_rb_source_file(b"strscan.rb", &include_bytes!("strscan.rb")[..])?;
    Ok(())
}

pub struct StringScanner;

// StringScanner tests from Ruby stdlib docs
// https://ruby-doc.org/stdlib-2.6.3/libdoc/strscan/rdoc/StringScanner.html
#[cfg(test)]
mod tests {
    use artichoke_core::eval::Eval;
    use artichoke_core::load::LoadSources;

    #[test]
    fn strscan_spec() {
        let interp = crate::interpreter().expect("init");
        interp
            .def_rb_source_file(
                b"/src/test/strscan_test.rb",
                &include_bytes!("strscan_test.rb")[..],
            )
            .unwrap();
        let _ = interp
            .eval(&b"require '/src/test/strscan_test.rb'"[..])
            .unwrap();
        if let Err(err) = interp.eval(b"spec") {
            panic!("{}", err);
        }
    }
}
