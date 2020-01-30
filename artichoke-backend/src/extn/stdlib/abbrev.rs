use crate::extn::prelude::*;

pub fn init(interp: &Artichoke) -> InitializeResult<()> {
    let spec = module::Spec::new(interp, "Abbrev", None)?;
    interp.0.borrow_mut().def_module::<Abbrev>(spec);
    interp.def_rb_source_file(b"abbrev.rb", &include_bytes!("abbrev.rb")[..])?;
    Ok(())
}

pub struct Abbrev;

// Abbrev tests from Ruby stdlib docs
// https://ruby-doc.org/stdlib-2.6.3/libdoc/abbrev/rdoc/Abbrev.html
#[cfg(test)]
mod tests {
    use crate::test::prelude::*;

    #[test]
    fn abbrev() {
        let spec = br#"
expect = {
    "ruby"=>"ruby",
    "rub"=>"ruby",
    "ru"=>"ruby",
    "r"=>"ruby"
}
result = Abbrev.abbrev(['ruby'])
expect == result
"#;
        let interp = crate::interpreter().expect("init");
        let _ = interp.eval(b"require 'abbrev'").expect("require");
        let result = interp.eval(spec).expect("spec");
        assert!(result.try_into::<bool>().expect("convert"));
    }

    #[test]
    fn abbrev_multiple() {
        let spec = br#"
expect = {
    "ruby"  =>  "ruby",
    "rub"   =>  "ruby",
    "rules" =>  "rules",
    "rule"  =>  "rules",
    "rul"   =>  "rules"
 }
result = Abbrev.abbrev(%w{ ruby rules })
expect == result
"#;
        let interp = crate::interpreter().expect("init");
        let _ = interp.eval(b"require 'abbrev'").expect("require");
        let result = interp.eval(spec).expect("spec");
        assert!(result.try_into::<bool>().expect("convert"));
    }

    #[test]
    fn abbrev_array() {
        let spec = br#"
expect = {
    "summer"  => "summer",
    "summe"   => "summer",
    "summ"    => "summer",
    "sum"     => "summer",
    "su"      => "summer",
    "s"       => "summer",
    "winter"  => "winter",
    "winte"   => "winter",
    "wint"    => "winter",
    "win"     => "winter",
    "wi"      => "winter",
    "w"       => "winter"
 }
result = %w{ summer winter }.abbrev
expect == result
"#;
        let interp = crate::interpreter().expect("init");
        let _ = interp.eval(b"require 'abbrev'").expect("require");
        let result = interp.eval(spec).expect("spec");
        assert!(result.try_into::<bool>().expect("convert"));
    }
}
