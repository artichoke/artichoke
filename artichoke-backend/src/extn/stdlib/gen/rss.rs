use crate::extn::prelude::*;

pub fn init(interp: &mut Artichoke) -> InitializeResult<()> {
    
    
    let spec = crate::module::Spec::new(interp, "Forwardable", None)?;
    interp.0.borrow_mut().def_module::<Forwardable>(spec);
    
    
    
    let spec = crate::class::Spec::new("Set", None, None)?;
    interp.0.borrow_mut().def_class::<Set>(spec);
    
    
    
    let spec = crate::class::Spec::new("StringScanner", None, None)?;
    interp.0.borrow_mut().def_class::<StringScanner>(spec);
    
    
    
    let spec = crate::class::Spec::new("Date", None, None)?;
    interp.0.borrow_mut().def_class::<Date>(spec);
    
    
    
    let spec = crate::class::Spec::new("SortedSet", None, None)?;
    interp.0.borrow_mut().def_class::<SortedSet>(spec);
    
    
    
    let spec = crate::class::Spec::new("ScanError", None, None)?;
    interp.0.borrow_mut().def_class::<ScanError>(spec);
    
    
    
    let spec = crate::class::Spec::new("DateTime", None, None)?;
    interp.0.borrow_mut().def_class::<DateTime>(spec);
    
    
    
    let spec = crate::module::Spec::new(interp, "SingleForwardable", None)?;
    interp.0.borrow_mut().def_module::<SingleForwardable>(spec);
    
    
    
    let spec = crate::module::Spec::new(interp, "OpenURI", None)?;
    interp.0.borrow_mut().def_module::<OpenURI>(spec);
    
    
    
    let spec = crate::class::Spec::new("IPSocket", None, None)?;
    interp.0.borrow_mut().def_class::<IPSocket>(spec);
    
    
    
    let spec = crate::class::Spec::new("IPAddr", None, None)?;
    interp.0.borrow_mut().def_class::<IPAddr>(spec);
    
    
    
    let spec = crate::module::Spec::new(interp, "RSS", None)?;
    interp.0.borrow_mut().def_module::<RSS>(spec);
    
    
    
    let spec = crate::module::Spec::new(interp, "URI", None)?;
    interp.0.borrow_mut().def_module::<URI>(spec);
    
    
    
    let spec = crate::module::Spec::new(interp, "REXML", None)?;
    interp.0.borrow_mut().def_module::<REXML>(spec);
    
    
    
    let spec = crate::class::Spec::new("StringIO", None, None)?;
    interp.0.borrow_mut().def_class::<StringIO>(spec);
    
    
    
    interp.def_rb_source_file(
        b"rss.rb",
        &include_bytes!(concat!(env!("OUT_DIR"), "/src/generated/rss.rb"))[..]
    )?;
    
    interp.def_rb_source_file(
        b"rss/0.9.rb",
        &include_bytes!(concat!(env!("OUT_DIR"), "/src/generated/rss/0.9.rb"))[..]
    )?;
    
    interp.def_rb_source_file(
        b"rss/1.0.rb",
        &include_bytes!(concat!(env!("OUT_DIR"), "/src/generated/rss/1.0.rb"))[..]
    )?;
    
    interp.def_rb_source_file(
        b"rss/2.0.rb",
        &include_bytes!(concat!(env!("OUT_DIR"), "/src/generated/rss/2.0.rb"))[..]
    )?;
    
    interp.def_rb_source_file(
        b"rss/atom.rb",
        &include_bytes!(concat!(env!("OUT_DIR"), "/src/generated/rss/atom.rb"))[..]
    )?;
    
    interp.def_rb_source_file(
        b"rss/content.rb",
        &include_bytes!(concat!(env!("OUT_DIR"), "/src/generated/rss/content.rb"))[..]
    )?;
    
    interp.def_rb_source_file(
        b"rss/content/1.0.rb",
        &include_bytes!(concat!(env!("OUT_DIR"), "/src/generated/rss/content/1.0.rb"))[..]
    )?;
    
    interp.def_rb_source_file(
        b"rss/content/2.0.rb",
        &include_bytes!(concat!(env!("OUT_DIR"), "/src/generated/rss/content/2.0.rb"))[..]
    )?;
    
    interp.def_rb_source_file(
        b"rss/converter.rb",
        &include_bytes!(concat!(env!("OUT_DIR"), "/src/generated/rss/converter.rb"))[..]
    )?;
    
    interp.def_rb_source_file(
        b"rss/dublincore.rb",
        &include_bytes!(concat!(env!("OUT_DIR"), "/src/generated/rss/dublincore.rb"))[..]
    )?;
    
    interp.def_rb_source_file(
        b"rss/dublincore/1.0.rb",
        &include_bytes!(concat!(env!("OUT_DIR"), "/src/generated/rss/dublincore/1.0.rb"))[..]
    )?;
    
    interp.def_rb_source_file(
        b"rss/dublincore/2.0.rb",
        &include_bytes!(concat!(env!("OUT_DIR"), "/src/generated/rss/dublincore/2.0.rb"))[..]
    )?;
    
    interp.def_rb_source_file(
        b"rss/dublincore/atom.rb",
        &include_bytes!(concat!(env!("OUT_DIR"), "/src/generated/rss/dublincore/atom.rb"))[..]
    )?;
    
    interp.def_rb_source_file(
        b"rss/image.rb",
        &include_bytes!(concat!(env!("OUT_DIR"), "/src/generated/rss/image.rb"))[..]
    )?;
    
    interp.def_rb_source_file(
        b"rss/itunes.rb",
        &include_bytes!(concat!(env!("OUT_DIR"), "/src/generated/rss/itunes.rb"))[..]
    )?;
    
    interp.def_rb_source_file(
        b"rss/maker.rb",
        &include_bytes!(concat!(env!("OUT_DIR"), "/src/generated/rss/maker.rb"))[..]
    )?;
    
    interp.def_rb_source_file(
        b"rss/maker/0.9.rb",
        &include_bytes!(concat!(env!("OUT_DIR"), "/src/generated/rss/maker/0.9.rb"))[..]
    )?;
    
    interp.def_rb_source_file(
        b"rss/maker/1.0.rb",
        &include_bytes!(concat!(env!("OUT_DIR"), "/src/generated/rss/maker/1.0.rb"))[..]
    )?;
    
    interp.def_rb_source_file(
        b"rss/maker/2.0.rb",
        &include_bytes!(concat!(env!("OUT_DIR"), "/src/generated/rss/maker/2.0.rb"))[..]
    )?;
    
    interp.def_rb_source_file(
        b"rss/maker/atom.rb",
        &include_bytes!(concat!(env!("OUT_DIR"), "/src/generated/rss/maker/atom.rb"))[..]
    )?;
    
    interp.def_rb_source_file(
        b"rss/maker/base.rb",
        &include_bytes!(concat!(env!("OUT_DIR"), "/src/generated/rss/maker/base.rb"))[..]
    )?;
    
    interp.def_rb_source_file(
        b"rss/maker/content.rb",
        &include_bytes!(concat!(env!("OUT_DIR"), "/src/generated/rss/maker/content.rb"))[..]
    )?;
    
    interp.def_rb_source_file(
        b"rss/maker/dublincore.rb",
        &include_bytes!(concat!(env!("OUT_DIR"), "/src/generated/rss/maker/dublincore.rb"))[..]
    )?;
    
    interp.def_rb_source_file(
        b"rss/maker/entry.rb",
        &include_bytes!(concat!(env!("OUT_DIR"), "/src/generated/rss/maker/entry.rb"))[..]
    )?;
    
    interp.def_rb_source_file(
        b"rss/maker/feed.rb",
        &include_bytes!(concat!(env!("OUT_DIR"), "/src/generated/rss/maker/feed.rb"))[..]
    )?;
    
    interp.def_rb_source_file(
        b"rss/maker/image.rb",
        &include_bytes!(concat!(env!("OUT_DIR"), "/src/generated/rss/maker/image.rb"))[..]
    )?;
    
    interp.def_rb_source_file(
        b"rss/maker/itunes.rb",
        &include_bytes!(concat!(env!("OUT_DIR"), "/src/generated/rss/maker/itunes.rb"))[..]
    )?;
    
    interp.def_rb_source_file(
        b"rss/maker/slash.rb",
        &include_bytes!(concat!(env!("OUT_DIR"), "/src/generated/rss/maker/slash.rb"))[..]
    )?;
    
    interp.def_rb_source_file(
        b"rss/maker/syndication.rb",
        &include_bytes!(concat!(env!("OUT_DIR"), "/src/generated/rss/maker/syndication.rb"))[..]
    )?;
    
    interp.def_rb_source_file(
        b"rss/maker/taxonomy.rb",
        &include_bytes!(concat!(env!("OUT_DIR"), "/src/generated/rss/maker/taxonomy.rb"))[..]
    )?;
    
    interp.def_rb_source_file(
        b"rss/maker/trackback.rb",
        &include_bytes!(concat!(env!("OUT_DIR"), "/src/generated/rss/maker/trackback.rb"))[..]
    )?;
    
    interp.def_rb_source_file(
        b"rss/parser.rb",
        &include_bytes!(concat!(env!("OUT_DIR"), "/src/generated/rss/parser.rb"))[..]
    )?;
    
    interp.def_rb_source_file(
        b"rss/rexmlparser.rb",
        &include_bytes!(concat!(env!("OUT_DIR"), "/src/generated/rss/rexmlparser.rb"))[..]
    )?;
    
    interp.def_rb_source_file(
        b"rss/rss.rb",
        &include_bytes!(concat!(env!("OUT_DIR"), "/src/generated/rss/rss.rb"))[..]
    )?;
    
    interp.def_rb_source_file(
        b"rss/slash.rb",
        &include_bytes!(concat!(env!("OUT_DIR"), "/src/generated/rss/slash.rb"))[..]
    )?;
    
    interp.def_rb_source_file(
        b"rss/syndication.rb",
        &include_bytes!(concat!(env!("OUT_DIR"), "/src/generated/rss/syndication.rb"))[..]
    )?;
    
    interp.def_rb_source_file(
        b"rss/taxonomy.rb",
        &include_bytes!(concat!(env!("OUT_DIR"), "/src/generated/rss/taxonomy.rb"))[..]
    )?;
    
    interp.def_rb_source_file(
        b"rss/trackback.rb",
        &include_bytes!(concat!(env!("OUT_DIR"), "/src/generated/rss/trackback.rb"))[..]
    )?;
    
    interp.def_rb_source_file(
        b"rss/utils.rb",
        &include_bytes!(concat!(env!("OUT_DIR"), "/src/generated/rss/utils.rb"))[..]
    )?;
    
    interp.def_rb_source_file(
        b"rss/xml-stylesheet.rb",
        &include_bytes!(concat!(env!("OUT_DIR"), "/src/generated/rss/xml-stylesheet.rb"))[..]
    )?;
    
    interp.def_rb_source_file(
        b"rss/xml.rb",
        &include_bytes!(concat!(env!("OUT_DIR"), "/src/generated/rss/xml.rb"))[..]
    )?;
    
    Ok(())
}

#[derive(Debug)]
pub struct Forwardable;


#[derive(Debug)]
pub struct Set;


#[derive(Debug)]
pub struct StringScanner;


#[derive(Debug)]
pub struct Date;


#[derive(Debug)]
pub struct SortedSet;


#[derive(Debug)]
pub struct ScanError;


#[derive(Debug)]
pub struct DateTime;


#[derive(Debug)]
pub struct SingleForwardable;


#[derive(Debug)]
pub struct OpenURI;


#[derive(Debug)]
pub struct IPSocket;


#[derive(Debug)]
pub struct IPAddr;


#[derive(Debug)]
pub struct RSS;


#[derive(Debug)]
pub struct URI;


#[derive(Debug)]
pub struct REXML;


#[derive(Debug)]
pub struct StringIO;


