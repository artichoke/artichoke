use crate::extn::prelude::*;

pub fn init(interp: &mut Artichoke) -> InitializeResult<()> {
    
    
    let spec = crate::module::Spec::new(interp, "Exception2MessageMapper", None)?;
    interp.0.borrow_mut().def_module::<Exception2MessageMapper>(spec);
    
    
    
    let spec = crate::module::Spec::new(interp, "RubyToken", None)?;
    interp.0.borrow_mut().def_module::<RubyToken>(spec);
    
    
    
    let spec = crate::module::Spec::new(interp, "IRB", None)?;
    interp.0.borrow_mut().def_module::<IRB>(spec);
    
    
    
    let spec = crate::class::Spec::new("RubyLex", None, None)?;
    interp.0.borrow_mut().def_class::<RubyLex>(spec);
    
    
    
    let spec = crate::module::Spec::new(interp, "Readline", None)?;
    interp.0.borrow_mut().def_module::<Readline>(spec);
    
    
    
    interp.def_rb_source_file(
        b"irb.rb",
        &include_bytes!(concat!(env!("OUT_DIR"), "/src/generated/irb.rb"))[..]
    )?;
    
    interp.def_rb_source_file(
        b"irb/context.rb",
        &include_bytes!(concat!(env!("OUT_DIR"), "/src/generated/irb/context.rb"))[..]
    )?;
    
    interp.def_rb_source_file(
        b"irb/extend-command.rb",
        &include_bytes!(concat!(env!("OUT_DIR"), "/src/generated/irb/extend-command.rb"))[..]
    )?;
    
    interp.def_rb_source_file(
        b"irb/init.rb",
        &include_bytes!(concat!(env!("OUT_DIR"), "/src/generated/irb/init.rb"))[..]
    )?;
    
    interp.def_rb_source_file(
        b"irb/input-method.rb",
        &include_bytes!(concat!(env!("OUT_DIR"), "/src/generated/irb/input-method.rb"))[..]
    )?;
    
    interp.def_rb_source_file(
        b"irb/inspector.rb",
        &include_bytes!(concat!(env!("OUT_DIR"), "/src/generated/irb/inspector.rb"))[..]
    )?;
    
    interp.def_rb_source_file(
        b"irb/locale.rb",
        &include_bytes!(concat!(env!("OUT_DIR"), "/src/generated/irb/locale.rb"))[..]
    )?;
    
    interp.def_rb_source_file(
        b"irb/magic-file.rb",
        &include_bytes!(concat!(env!("OUT_DIR"), "/src/generated/irb/magic-file.rb"))[..]
    )?;
    
    interp.def_rb_source_file(
        b"irb/notifier.rb",
        &include_bytes!(concat!(env!("OUT_DIR"), "/src/generated/irb/notifier.rb"))[..]
    )?;
    
    interp.def_rb_source_file(
        b"irb/output-method.rb",
        &include_bytes!(concat!(env!("OUT_DIR"), "/src/generated/irb/output-method.rb"))[..]
    )?;
    
    interp.def_rb_source_file(
        b"irb/ruby-lex.rb",
        &include_bytes!(concat!(env!("OUT_DIR"), "/src/generated/irb/ruby-lex.rb"))[..]
    )?;
    
    interp.def_rb_source_file(
        b"irb/ruby-token.rb",
        &include_bytes!(concat!(env!("OUT_DIR"), "/src/generated/irb/ruby-token.rb"))[..]
    )?;
    
    interp.def_rb_source_file(
        b"irb/slex.rb",
        &include_bytes!(concat!(env!("OUT_DIR"), "/src/generated/irb/slex.rb"))[..]
    )?;
    
    interp.def_rb_source_file(
        b"irb/src_encoding.rb",
        &include_bytes!(concat!(env!("OUT_DIR"), "/src/generated/irb/src_encoding.rb"))[..]
    )?;
    
    interp.def_rb_source_file(
        b"irb/version.rb",
        &include_bytes!(concat!(env!("OUT_DIR"), "/src/generated/irb/version.rb"))[..]
    )?;
    
    interp.def_rb_source_file(
        b"irb/workspace.rb",
        &include_bytes!(concat!(env!("OUT_DIR"), "/src/generated/irb/workspace.rb"))[..]
    )?;
    
    Ok(())
}

#[derive(Debug)]
pub struct Exception2MessageMapper;


#[derive(Debug)]
pub struct RubyToken;


#[derive(Debug)]
pub struct IRB;


#[derive(Debug)]
pub struct RubyLex;


#[derive(Debug)]
pub struct Readline;


