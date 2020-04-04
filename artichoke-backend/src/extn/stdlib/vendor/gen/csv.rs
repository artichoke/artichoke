use crate::extn::prelude::*;

pub fn init(interp: &mut Artichoke) -> InitializeResult<()> {
    
    
    let spec = crate::module::Spec::new(interp, "SingleForwardable", None)?;
    interp.0.borrow_mut().def_module::<SingleForwardable>(spec);
    
    
    
    let spec = crate::class::Spec::new("ScanError", None, None)?;
    interp.0.borrow_mut().def_class::<ScanError>(spec);
    
    
    
    let spec = crate::class::Spec::new("CSV", None, None)?;
    interp.0.borrow_mut().def_class::<CSV>(spec);
    
    
    
    let spec = crate::class::Spec::new("Date", None, None)?;
    interp.0.borrow_mut().def_class::<Date>(spec);
    
    
    
    let spec = crate::class::Spec::new("StringScanner", None, None)?;
    interp.0.borrow_mut().def_class::<StringScanner>(spec);
    
    
    
    let spec = crate::class::Spec::new("DateTime", None, None)?;
    interp.0.borrow_mut().def_class::<DateTime>(spec);
    
    
    
    let spec = crate::class::Spec::new("StringIO", None, None)?;
    interp.0.borrow_mut().def_class::<StringIO>(spec);
    
    
    
    let spec = crate::module::Spec::new(interp, "Forwardable", None)?;
    interp.0.borrow_mut().def_module::<Forwardable>(spec);
    
    
    
    interp.def_rb_source_file(
        b"csv.rb",
        &include_bytes!(concat!(env!("OUT_DIR"), "/src/generated/csv.rb"))[..]
    )?;
    
    interp.def_rb_source_file(
        b"csv/core_ext/array.rb",
        &include_bytes!(concat!(env!("OUT_DIR"), "/src/generated/csv/core_ext/array.rb"))[..]
    )?;
    
    interp.def_rb_source_file(
        b"csv/core_ext/string.rb",
        &include_bytes!(concat!(env!("OUT_DIR"), "/src/generated/csv/core_ext/string.rb"))[..]
    )?;
    
    interp.def_rb_source_file(
        b"csv/delete_suffix.rb",
        &include_bytes!(concat!(env!("OUT_DIR"), "/src/generated/csv/delete_suffix.rb"))[..]
    )?;
    
    interp.def_rb_source_file(
        b"csv/fields_converter.rb",
        &include_bytes!(concat!(env!("OUT_DIR"), "/src/generated/csv/fields_converter.rb"))[..]
    )?;
    
    interp.def_rb_source_file(
        b"csv/match_p.rb",
        &include_bytes!(concat!(env!("OUT_DIR"), "/src/generated/csv/match_p.rb"))[..]
    )?;
    
    interp.def_rb_source_file(
        b"csv/parser.rb",
        &include_bytes!(concat!(env!("OUT_DIR"), "/src/generated/csv/parser.rb"))[..]
    )?;
    
    interp.def_rb_source_file(
        b"csv/row.rb",
        &include_bytes!(concat!(env!("OUT_DIR"), "/src/generated/csv/row.rb"))[..]
    )?;
    
    interp.def_rb_source_file(
        b"csv/table.rb",
        &include_bytes!(concat!(env!("OUT_DIR"), "/src/generated/csv/table.rb"))[..]
    )?;
    
    interp.def_rb_source_file(
        b"csv/version.rb",
        &include_bytes!(concat!(env!("OUT_DIR"), "/src/generated/csv/version.rb"))[..]
    )?;
    
    interp.def_rb_source_file(
        b"csv/writer.rb",
        &include_bytes!(concat!(env!("OUT_DIR"), "/src/generated/csv/writer.rb"))[..]
    )?;
    
    Ok(())
}

#[derive(Debug)]
pub struct SingleForwardable;


#[derive(Debug)]
pub struct ScanError;


#[derive(Debug)]
pub struct CSV;


#[derive(Debug)]
pub struct Date;


#[derive(Debug)]
pub struct StringScanner;


#[derive(Debug)]
pub struct DateTime;


#[derive(Debug)]
pub struct StringIO;


#[derive(Debug)]
pub struct Forwardable;


